use crate::display::print_status;
use crate::errors::AppError;
use crate::models::SessionTemperatureStats;
use crate::sensor::SensorService;

pub fn show_menu() {
    println!("==============================================================");
    println!("        СИСТЕМНИЙ МОНІТОР ПРОЦЕСОРА ТА РЕСУРСІВ ПК");
    println!("==============================================================");
    println!("1  - Інформація про процесор");
    println!("2  - Завантаження процесора");
    println!("3  - Частота процесора");
    println!("4  - Температура процесора");
    println!("5  - Оперативна пам’ять");
    println!("6  - Логічні диски");
    println!("7  - Стан батареї");
    println!("8  - Повна інформація про систему");
    println!("9  - Усі температурні значення");
    println!("0  - Вихід");
    println!("==============================================================");
}

pub fn handle_choice(choice: &str) -> Result<(), AppError> {
    let service = SensorService::new();
    let mut status = service.collect_status()?;
    status.session_temperatures = SessionTemperatureStats::new();

    match choice {
        "1" => {
            println!("================ ІНФОРМАЦІЯ ПРО ПРОЦЕСОР ================");
            println!("Процесор                : {}", status.cpu.name);
            println!(
                "Ядра CPU                : {}",
                status
                    .cpu
                    .cores
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "Логічні процесори       : {}",
                status
                    .cpu
                    .logical_processors
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!("=========================================================");
        }

        "2" => {
            println!("================ ЗАВАНТАЖЕННЯ ПРОЦЕСОРА =================");
            println!(
                "Завантаження CPU        : {}",
                status
                    .cpu
                    .load_percent
                    .map(|v| format!("{:.1} %", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!("=========================================================");
        }

        "3" => {
            println!("================= ЧАСТОТА ПРОЦЕСОРА =====================");
            println!(
                "Поточна частота CPU     : {}",
                status
                    .cpu
                    .current_clock_mhz
                    .map(|v| format!("{} MHz", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "Максимальна частота CPU : {}",
                status
                    .cpu
                    .max_clock_mhz
                    .map(|v| format!("{} MHz", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!("=========================================================");
        }

        "4" => {
            println!("=============== ТЕМПЕРАТУРА ПРОЦЕСОРА ===================");
            println!(
                "Поточна температура     : {}",
                status
                    .temperatures
                    .current
                    .map(|v| format!("{:.1} °C", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "Кількість сенсорів      : {}",
                status.temperatures.count
            );
            println!("=========================================================");
        }

        "5" => {
            println!("================ ОПЕРАТИВНА ПАМ’ЯТЬ =====================");
            println!(
                "RAM загалом             : {}",
                status
                    .memory
                    .total_mb
                    .map(|v| format!("{:.1} MB", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "RAM вільно              : {}",
                status
                    .memory
                    .free_mb
                    .map(|v| format!("{:.1} MB", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "RAM використано         : {}",
                status
                    .memory
                    .used_mb
                    .map(|v| format!("{:.1} MB", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!(
                "RAM завантаження        : {}",
                status
                    .memory
                    .used_percent
                    .map(|v| format!("{:.1} %", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!("=========================================================");
        }

        "6" => {
            println!("=================== ЛОГІЧНІ ДИСКИ =======================");
            if status.disks.is_empty() {
                println!("Дані про диски недоступні.");
            } else {
                for disk in &status.disks {
                    println!("Диск                   : {}", disk.device_id);
                    println!(
                        "  Загальний обсяг      : {}",
                        disk.total_gb
                            .map(|v| format!("{:.1} GB", v))
                            .unwrap_or_else(|| "Н/Д".to_string())
                    );
                    println!(
                        "  Вільне місце         : {}",
                        disk.free_gb
                            .map(|v| format!("{:.1} GB", v))
                            .unwrap_or_else(|| "Н/Д".to_string())
                    );
                    println!(
                        "  Використано          : {}",
                        disk.used_gb
                            .map(|v| format!("{:.1} GB", v))
                            .unwrap_or_else(|| "Н/Д".to_string())
                    );
                    println!(
                        "  Завантаження         : {}",
                        disk.used_percent
                            .map(|v| format!("{:.1} %", v))
                            .unwrap_or_else(|| "Н/Д".to_string())
                    );
                    println!("---------------------------------------------------------");
                }
            }
            println!("=========================================================");
        }

        "7" => {
            println!("==================== СТАН БАТАРЕЇ =======================");
            println!(
                "Рівень заряду          : {}",
                status
                    .battery
                    .charge_percent
                    .map(|v| format!("{} %", v))
                    .unwrap_or_else(|| "Н/Д".to_string())
            );
            println!("Стан батареї           : {}", status.battery.status_text);
            println!("=========================================================");
        }

        "8" => {
            print_status(&status);
        }

        "9" => {
            println!("============== УСІ ТЕМПЕРАТУРНІ ЗНАЧЕННЯ ================");
            if status.temperatures.readings.is_empty() {
                println!("Температурні значення недоступні.");
            } else {
                for reading in &status.temperatures.readings {
                    println!("Сенсор {:>2}           : {:.1} °C", reading.index, reading.celsius);
                }
            }
            println!("=========================================================");
        }

        _ => {
            println!("Помилка: обрано невідомий пункт меню.");
        }
    }

    Ok(())
}