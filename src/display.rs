use crate::models::{DiskInfo, SystemStatus};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn get_unix_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

fn format_f32_opt(value: Option<f32>, suffix: &str) -> String {
    match value {
        Some(v) => format!("{:.1} {}", v, suffix),
        None => "Н/Д".to_string(),
    }
}

fn format_u32_opt(value: Option<u32>, suffix: &str) -> String {
    match value {
        Some(v) => format!("{} {}", v, suffix),
        None => "Н/Д".to_string(),
    }
}

fn format_temp_list(status: &SystemStatus) -> String {
    if status.temperatures.readings.is_empty() {
        return "Н/Д".to_string();
    }

    status
        .temperatures
        .readings
        .iter()
        .map(|r| format!("[{}] {:.1} °C", r.index, r.celsius))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_disks(disks: &[DiskInfo]) {
    if disks.is_empty() {
        println!("Диски                   : Н/Д");
        return;
    }

    println!("Диски:");
    for disk in disks {
        println!(
            "  {}  Загалом: {} | Вільно: {} | Використано: {} | Завантаження: {}",
            disk.device_id,
            format_f32_opt(disk.total_gb, "GB"),
            format_f32_opt(disk.free_gb, "GB"),
            format_f32_opt(disk.used_gb, "GB"),
            format_f32_opt(disk.used_percent, "%"),
        );
    }
}

pub fn print_status(status: &SystemStatus) {
    println!("======================================================================");
    println!("              CPU TEMPERATURE READER (Rust / Windows)");
    println!("======================================================================");
    println!("Час UNIX                  : {}", get_unix_time());
    println!();

    println!("Процесор                  : {}", status.cpu.name);
    println!(
        "Завантаження CPU          : {}",
        format_f32_opt(status.cpu.load_percent, "%")
    );
    println!(
        "Поточна частота CPU       : {}",
        format_u32_opt(status.cpu.current_clock_mhz, "MHz")
    );
    println!(
        "Максимальна частота CPU   : {}",
        format_u32_opt(status.cpu.max_clock_mhz, "MHz")
    );
    println!(
        "Ядра CPU                  : {}",
        match status.cpu.cores {
            Some(v) => v.to_string(),
            None => "Н/Д".to_string(),
        }
    );
    println!(
        "Логічні процесори         : {}",
        match status.cpu.logical_processors {
            Some(v) => v.to_string(),
            None => "Н/Д".to_string(),
        }
    );
    println!();

    println!(
        "Поточна температура       : {}",
        format_f32_opt(status.temperatures.current, "°C")
    );
    println!(
        "Мін. температура (сесія)  : {}",
        format_f32_opt(status.session_temperatures.min, "°C")
    );
    println!(
        "Макс. температура (сесія) : {}",
        format_f32_opt(status.session_temperatures.max, "°C")
    );
    println!(
        "Сер. температура (сесія)  : {}",
        format_f32_opt(status.session_temperatures.average, "°C")
    );
    println!(
        "К-сть вимірів (сесія)     : {}",
        status.session_temperatures.samples
    );
    println!("Кількість сенсорів        : {}", status.temperatures.count);
    println!("Усі температурні значення : {}", format_temp_list(status));
    println!();

    println!(
        "RAM загалом               : {}",
        format_f32_opt(status.memory.total_mb, "MB")
    );
    println!(
        "RAM вільно                : {}",
        format_f32_opt(status.memory.free_mb, "MB")
    );
    println!(
        "RAM використано           : {}",
        format_f32_opt(status.memory.used_mb, "MB")
    );
    println!(
        "RAM завантаження          : {}",
        format_f32_opt(status.memory.used_percent, "%")
    );
    println!();

    print_disks(&status.disks);
    println!();

    println!(
        "Батарея                   : {}",
        match status.battery.charge_percent {
            Some(v) => format!("{} %", v),
            None => "Н/Д".to_string(),
        }
    );
    println!("Стан батареї              : {}", status.battery.status_text);
    println!();
    println!("Ctrl + C — вихід");
    println!("======================================================================");
}