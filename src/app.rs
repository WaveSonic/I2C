use crate::config::AppConfig;
use crate::display::{clear_screen, print_status};
use crate::errors::AppError;
use crate::logger::append_log;
use crate::models::SessionTemperatureStats;
use crate::sensor::SensorService;
use std::thread;
use std::time::Duration;

pub fn run_app() -> Result<(), AppError> {
    let config = AppConfig::from_args();
    let service = SensorService::new();
    let mut session_stats = SessionTemperatureStats::new();

    loop {
        let mut status = service.collect_status()?;

        session_stats.update(status.temperatures.current);
        status.session_temperatures = session_stats.clone();

        if config.clear_screen {
            clear_screen();
        }

        print_status(&status);

        if config.log_enabled {
            if let Err(err) = append_log(&status) {
                eprintln!("Не вдалося записати лог: {}", err);
            }
        }

        if config.run_once {
            break;
        }

        thread::sleep(Duration::from_secs(config.interval_secs));
    }

    Ok(())
}