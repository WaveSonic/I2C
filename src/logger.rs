use crate::models::SystemStatus;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

fn opt_f32_to_text(value: Option<f32>) -> String {
    match value {
        Some(v) => format!("{:.2}", v),
        None => "N/A".to_string(),
    }
}

fn opt_u32_to_text(value: Option<u32>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "N/A".to_string(),
    }
}

pub fn append_log(status: &SystemStatus) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("cpu_monitor.log")?;

    let temp_values = if status.temperatures.readings.is_empty() {
        "N/A".to_string()
    } else {
        status
            .temperatures
            .readings
            .iter()
            .map(|r| format!("{:.2}", r.celsius))
            .collect::<Vec<_>>()
            .join("|")
    };

    let disk_values = if status.disks.is_empty() {
        "N/A".to_string()
    } else {
        status
            .disks
            .iter()
            .map(|d| {
                format!(
                    "{}:{}:{}:{}:{}",
                    d.device_id,
                    opt_f32_to_text(d.total_gb),
                    opt_f32_to_text(d.free_gb),
                    opt_f32_to_text(d.used_gb),
                    opt_f32_to_text(d.used_percent),
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    };

    let line = format!(
        "time={}; cpu=\"{}\"; load={}; current_clock={}; max_clock={}; cores={}; logical={}; temp_current={}; temp_min={}; temp_max={}; temp_avg={}; temp_count={}; temp_all={}; ram_total={}; ram_free={}; ram_used={}; ram_used_percent={}; battery_charge={}; battery_status=\"{}\"; disks={}\n",
        timestamp(),
        status.cpu.name,
        opt_f32_to_text(status.cpu.load_percent),
        opt_u32_to_text(status.cpu.current_clock_mhz),
        opt_u32_to_text(status.cpu.max_clock_mhz),
        opt_u32_to_text(status.cpu.cores),
        opt_u32_to_text(status.cpu.logical_processors),
        opt_f32_to_text(status.temperatures.current),
        opt_f32_to_text(status.temperatures.min),
        opt_f32_to_text(status.temperatures.max),
        opt_f32_to_text(status.temperatures.average),
        status.temperatures.count,
        temp_values,
        opt_f32_to_text(status.memory.total_mb),
        opt_f32_to_text(status.memory.free_mb),
        opt_f32_to_text(status.memory.used_mb),
        opt_f32_to_text(status.memory.used_percent),
        opt_u32_to_text(status.battery.charge_percent),
        status.battery.status_text,
        disk_values
    );

    file.write_all(line.as_bytes())?;
    Ok(())
}