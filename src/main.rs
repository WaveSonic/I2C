mod admin;
mod app;
mod config;
mod display;
mod errors;
mod logger;
mod menu;
mod models;
mod powershell;
mod sensor;

use app::run_app;

fn main() {
    if !admin::is_running_as_admin() {
        eprintln!("Помилка: програму потрібно запускати від імені адміністратора.");
        eprintln!("Причина: без підвищених прав Windows не дає доступ до потрібних WMI/ACPI-даних.");
        std::process::exit(1);
    }

    if let Err(err) = run_app() {
        eprintln!("Критична помилка: {}", err);
    }
}