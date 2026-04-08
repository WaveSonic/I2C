use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub interval_secs: u64,
    pub run_once: bool,
    pub log_enabled: bool,
    pub clear_screen: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            interval_secs: 2,
            run_once: false,
            log_enabled: true,
            clear_screen: true,
        }
    }
}

impl AppConfig {
    pub fn from_args() -> Self {
        let mut config = Self::default();
        let args: Vec<String> = env::args().collect();

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--once" => config.run_once = true,
                "--no-log" => config.log_enabled = false,
                "--no-clear" => config.clear_screen = false,
                "--interval" => {
                    if i + 1 < args.len() {
                        if let Ok(value) = args[i + 1].parse::<u64>() {
                            if value > 0 {
                                config.interval_secs = value;
                            }
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        config
    }
}