# ESP32 Robot Car Controller (Rust)

## Текущий статус

Проект настроен для сборки с esp-idf. Для полной сборки требуется:
1. ESP-IDF framework
2. Правильная настройка toolchain

## Быстрый старт с Arduino

Если установка ESP-IDF вызывает сложности, используйте Arduino IDE:

### 1. Установка Arduino IDE
```bash
# Скачайте с https://www.arduino.cc/en/software
```

### 2. Добавление поддержки ESP32
1. Arduino IDE → Файл → Настройки
2. Добавьте URL:
   ```
   https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
   ```
3. Инструменты → Плата → Менеджер плат → установите "ESP32"

### 3. Загрузите код
Скопируйте содержимое папки `arduino/` в Arduino IDE и загрузите.

## Установка ESP-IDF (для сборки на Rust)

### Автоматическая установка
```powershell
# Установите espup
cargo install espup

# Установите esp-idf
espup install

# Перезапустите терминал

# Проверьте
$env:ESP_IDF_PATH = "$env:USERPROFILE\.espressif\esp-idf\v5.3"
```

### Ручная установка
1. Скачайте: https://dl.espressif.com/dl/esp-idf-installer/esp-idf-installer-latest.exe
2. Установите в `C:\Users\igore\esp-idf`
3. Обновите `C:\Users\igore\export-esp.ps1`:
   ```powershell
   $Env:ESP_IDF_PATH = "C:\Users\igore\esp-idf\esp-idf"
   ```

## Структура проекта

```
esp32/
├── src/
│   └── main.rs        # Основной код (Rust)
├── Cargo.toml          # Зависимости
├── build.rs           # Скрипт сборки
├── .cargo/
│   └── config.toml    # Конфигурация
├── arduino/           # Arduino версия (для сравнения)
│   └── robot_car/
│       └── robot_car.ino
└── README.md
```

## Пины подключения

```
ESP32          L298N
-------        ------
GPIO 5   -->  ENA
GPIO 17  -->  IN1
GPIO 18  -->  IN2
GPIO 19  -->  IN3
GPIO 21  -->  IN4
GPIO 22  -->  ENB
GND      -->  GND (общий с батареей)
```

## Конфигурация WiFi

В `src/main.rs` измените:
```rust
const WIFI_SSID: &str = "ваша_сеть";
const WIFI_PASSWORD: &str = "ваш_пароль";
const SERVER_IP: &str = "192.168.50.19";
```

## Сборка

После установки ESP-IDF:
```powershell
cd C:\projects\tauri-app\esp32
. C:\Users\igore\export-esp.ps1
cargo build
```
