#[derive(Debug, Clone)]
pub struct TemperatureReading {
    pub index: usize,
    pub celsius: f32,
}

#[derive(Debug, Clone)]
pub struct TemperatureStats {
    pub current: Option<f32>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub average: Option<f32>,
    pub count: usize,
    pub readings: Vec<TemperatureReading>,
}

impl TemperatureStats {
    pub fn empty() -> Self {
        Self {
            current: None,
            min: None,
            max: None,
            average: None,
            count: 0,
            readings: Vec::new(),
        }
    }

    pub fn from_values(values: Vec<f32>) -> Self {
        if values.is_empty() {
            return Self::empty();
        }

        let min = values.iter().copied().reduce(f32::min);
        let max = values.iter().copied().reduce(f32::max);
        let sum: f32 = values.iter().sum();
        let average = Some(sum / values.len() as f32);

        let readings = values
            .iter()
            .enumerate()
            .map(|(i, value)| TemperatureReading {
                index: i + 1,
                celsius: *value,
            })
            .collect::<Vec<_>>();

        Self {
            current: values.first().copied(),
            min,
            max,
            average,
            count: values.len(),
            readings,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionTemperatureStats {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub average: Option<f32>,
    pub samples: usize,
    pub sum: f32,
}

impl SessionTemperatureStats {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            average: None,
            samples: 0,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, value: Option<f32>) {
        if let Some(v) = value {
            self.min = Some(match self.min {
                Some(current) => current.min(v),
                None => v,
            });

            self.max = Some(match self.max {
                Some(current) => current.max(v),
                None => v,
            });

            self.samples += 1;
            self.sum += v;
            self.average = Some(self.sum / self.samples as f32);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub load_percent: Option<f32>,
    pub current_clock_mhz: Option<u32>,
    pub max_clock_mhz: Option<u32>,
    pub cores: Option<u32>,
    pub logical_processors: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_mb: Option<f32>,
    pub free_mb: Option<f32>,
    pub used_mb: Option<f32>,
    pub used_percent: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device_id: String,
    pub total_gb: Option<f32>,
    pub free_gb: Option<f32>,
    pub used_gb: Option<f32>,
    pub used_percent: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub charge_percent: Option<u32>,
    pub status_text: String,
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub cpu: CpuInfo,
    pub temperatures: TemperatureStats,
    pub session_temperatures: SessionTemperatureStats,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub battery: BatteryInfo,
}