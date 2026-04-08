## Встановлення

1. Клонувати репозиторій:
   git clone https://github.com/USERNAME/cpu_temp_reader.git

2. Перейти в каталог проєкту:
   cd cpu_temp_reader

3. Скомпілювати програму локально:
   cargo build --release

## Запуск

Програма потребує прав адміністратора для доступу до температурних WMI/ACPI-даних.
Після компіляції запускайте файл:

target\release\cpu_temp_reader.exe

Windows автоматично покаже запит UAC на підвищення прав.
