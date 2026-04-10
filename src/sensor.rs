use crate::errors::AppError;
use crate::models::{
    BatteryInfo, CpuInfo, DiskInfo, MemoryInfo, SystemStatus, TemperatureStats,
};
use crate::powershell::run_powershell;

pub struct SensorService;

impl SensorService {
    pub fn new() -> Self {
        Self
    }

    pub fn collect_status(&self) -> Result<SystemStatus, AppError> {
    let cpu = self.get_cpu_info()?;
    let temperatures = self.get_all_temperatures()?;
    let memory = self.get_memory_info()?;
    let disks = self.get_disk_info()?;
    let battery = self.get_battery_info()?;

    Ok(SystemStatus {
        cpu,
        temperatures,
        session_temperatures: crate::models::SessionTemperatureStats::new(),
        memory,
        disks,
        battery,
    })
}

    fn get_cpu_info(&self) -> Result<CpuInfo, AppError> {
        let name_script =
            "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name";
        let load_script = "Get-CimInstance Win32_Processor | Measure-Object -Property LoadPercentage -Average | Select-Object -ExpandProperty Average";
        let current_clock_script =
            "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty CurrentClockSpeed";
        let max_clock_script =
            "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty MaxClockSpeed";
        let cores_script =
            "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty NumberOfCores";
        let logical_script = "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty NumberOfLogicalProcessors";

        let name = run_powershell(name_script)?;
        if name.trim().is_empty() {
            return Err(AppError::EmptyData(
                "Не вдалося отримати назву процесора".to_string(),
            ));
        }

        let load_percent = self.parse_f32_opt(&run_powershell(load_script)?);
        let current_clock_mhz = self.parse_u32_opt(&run_powershell(current_clock_script)?);
        let max_clock_mhz = self.parse_u32_opt(&run_powershell(max_clock_script)?);
        let cores = self.parse_u32_opt(&run_powershell(cores_script)?);
        let logical_processors = self.parse_u32_opt(&run_powershell(logical_script)?);

        Ok(CpuInfo {
            name,
            load_percent,
            current_clock_mhz,
            max_clock_mhz,
            cores,
            logical_processors,
        })
    }

    fn get_all_temperatures(&self) -> Result<TemperatureStats, AppError> {
        let script = "(Get-CimInstance -Namespace root/wmi -ClassName MSAcpi_ThermalZoneTemperature | Select-Object -ExpandProperty CurrentTemperature)";
        let output = run_powershell(script)?;

        if output.trim().is_empty() {
            return Ok(TemperatureStats::empty());
        }

        let mut values_celsius = Vec::new();

        for line in output.lines() {
            let value = line.trim();
            if value.is_empty() {
                continue;
            }

            let normalized = value.replace(',', ".");
            if let Ok(raw) = normalized.parse::<f32>() {
                let celsius = (raw / 10.0) - 273.15;
                if (-50.0..=150.0).contains(&celsius) {
                    values_celsius.push(celsius);
                }
            }
        }

        Ok(TemperatureStats::from_values(values_celsius))
    }

    fn get_memory_info(&self) -> Result<MemoryInfo, AppError> {
        let total_script =
            "Get-CimInstance Win32_OperatingSystem | Select-Object -ExpandProperty TotalVisibleMemorySize";
        let free_script =
            "Get-CimInstance Win32_OperatingSystem | Select-Object -ExpandProperty FreePhysicalMemory";

        let total_kb = self.parse_f32_opt(&run_powershell(total_script)?);
        let free_kb = self.parse_f32_opt(&run_powershell(free_script)?);

        match (total_kb, free_kb) {
            (Some(total_kb), Some(free_kb)) => {
                let total_mb = total_kb / 1024.0;
                let free_mb = free_kb / 1024.0;
                let used_mb = total_mb - free_mb;
                let used_percent = if total_mb > 0.0 {
                    Some((used_mb / total_mb) * 100.0)
                } else {
                    None
                };

                Ok(MemoryInfo {
                    total_mb: Some(total_mb),
                    free_mb: Some(free_mb),
                    used_mb: Some(used_mb),
                    used_percent,
                })
            }
            _ => Ok(MemoryInfo {
                total_mb: None,
                free_mb: None,
                used_mb: None,
                used_percent: None,
            }),
        }
    }

    fn get_disk_info(&self) -> Result<Vec<DiskInfo>, AppError> {
        let script = "Get-CimInstance Win32_LogicalDisk -Filter \"DriveType=3\" | Select-Object DeviceID,Size,FreeSpace | ForEach-Object { \"$($_.DeviceID)|$($_.Size)|$($_.FreeSpace)\" }";
        let output = run_powershell(script)?;

        if output.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut disks = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 3 {
                continue;
            }

            let device_id = parts[0].trim().to_string();
            let size_bytes = self.parse_f32_opt(parts[1]);
            let free_bytes = self.parse_f32_opt(parts[2]);

            let (total_gb, free_gb, used_gb, used_percent) = match (size_bytes, free_bytes) {
                (Some(total), Some(free)) => {
                    let total_gb = total / 1024.0 / 1024.0 / 1024.0;
                    let free_gb = free / 1024.0 / 1024.0 / 1024.0;
                    let used_gb = total_gb - free_gb;
                    let used_percent = if total_gb > 0.0 {
                        Some((used_gb / total_gb) * 100.0)
                    } else {
                        None
                    };
                    (Some(total_gb), Some(free_gb), Some(used_gb), used_percent)
                }
                _ => (None, None, None, None),
            };

            disks.push(DiskInfo {
                device_id,
                total_gb,
                free_gb,
                used_gb,
                used_percent,
            });
        }

        Ok(disks)
    }

    fn get_battery_info(&self) -> Result<BatteryInfo, AppError> {
        let charge_script =
            "Get-CimInstance Win32_Battery | Select-Object -First 1 -ExpandProperty EstimatedChargeRemaining";
        let status_script =
            "Get-CimInstance Win32_Battery | Select-Object -First 1 -ExpandProperty BatteryStatus";

        let charge_raw = run_powershell(charge_script).unwrap_or_default();
        let status_raw = run_powershell(status_script).unwrap_or_default();

        let charge_percent = self.parse_u32_opt(&charge_raw);
        let status_code = self.parse_u32_opt(&status_raw);

        let status_text = match status_code {
            Some(1) => "Розряджається".to_string(),
            Some(2) => "Підключено до живлення".to_string(),
            Some(3) => "Повністю заряджена".to_string(),
            Some(4) => "Низький заряд".to_string(),
            Some(5) => "Критичний заряд".to_string(),
            Some(6) => "Заряджається".to_string(),
            Some(7) => "Заряджається і високий рівень".to_string(),
            Some(8) => "Заряджається і низький рівень".to_string(),
            Some(9) => "Заряджається і критичний рівень".to_string(),
            Some(10) => "Невизначено".to_string(),
            Some(11) => "Частково заряджена".to_string(),
            _ => "Батарея відсутня або дані недоступні".to_string(),
        };

        Ok(BatteryInfo {
            charge_percent,
            status_text,
        })
    }

    fn parse_f32_opt(&self, text: &str) -> Option<f32> {
        let cleaned = text.trim().replace(',', ".");
        if cleaned.is_empty() {
            return None;
        }
        cleaned.parse::<f32>().ok()
    }

    fn parse_u32_opt(&self, text: &str) -> Option<u32> {
        let cleaned = text.trim();
        if cleaned.is_empty() {
            return None;
        }
        cleaned.parse::<u32>().ok()
    }

}