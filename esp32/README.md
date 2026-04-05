# ESP32 Robot Car Controller (Rust)

Прошивка для ESP32 на языке Rust для управления роботом-машинкой.

## Схема подключения

```
ESP32          L298N Motor Driver
-------        ------------------
GPIO 5   ----> ENA (PWM Left Motor)
GPIO 17  ----> IN1
GPIO 18  ----> IN2
GPIO 19  ----> IN3
GPIO 21  ----> IN4
GPIO 22  ----> ENB (PWM Right Motor)

L298N         Motors
----         ------
OUT1      ----> Left Motor +
OUT2      ----> Left Motor -
OUT3      ----> Right Motor +
OUT4      ----> Right Motor -

L298N         Power
----         -----
12V      ----> LiPo Battery (+)
GND      ----> Battery (-), ESP32 GND
```

## Установка Rust для ESP32

### 1. Установите Rust
```powershell
irm https://sh.rustup.rs | iex
```

### 2. Добавьте поддержку ESP32
```powershell
rustup install esp
rustup default esp
```

### 3. Установите espup
```powershell
cargo install espup
espup install
```

### 4. Настройте переменные окружения (Windows)
```powershell
$env:ESPPORT = "COM3"  # Замените на ваш порт
$env:ESP_IDF_PATH = "$env:USERPROFILE\.espressif"
```

## Настройка проекта

### 1. Установите зависимости
```bash
cd esp32
cargo install espup
espup install --espressif
```

### 2. Настройте WiFi и IP сервера
Создайте файл `.env` или установите переменные окружения:

```bash
export WIFI_SSID="YourWiFiName"
export WIFI_PASSWORD="YourWiFiPassword"  
export SERVER_IP="192.168.1.100"
```

Или отредактируйте `src/main.rs` напрямую:
```rust
const WIFI_SSID: &str = "YourWiFiName";
const WIFI_PASSWORD: &str = "YourWiFiPassword";
const SERVER_IP: &str = "192.168.1.100";
```

### 3. Как узнать IP вашего ПК
```cmd
ipconfig
```
Найдите IPv4 адрес (например, 192.168.1.100)

## Сборка и загрузка

### Подключите ESP32 по USB

### Сборка
```bash
cargo build --release
```

### Загрузка
```bash
cargo run --release
```

Или с esptool:
```bash
esptool.py --chip esp32 --port COM3 --baud 921600 write_flash 0x1000 target/riscv32imc-esp-espidf/release/esp32-robot-car.bin
```

## Использование Docker (альтернатива)

Если у вас есть Docker, можете использовать контейнер с уже настроенным окружением:

```bash
docker run --rm -v $PWD:/project -w /project -it --device /dev/ttyUSB0 espressif/idf-rust:latest
```

## Структура кода

```
esp32/
├── src/
│   └── main.rs          # Основной код
├── Cargo.toml           # Зависимости
├── build.rs             # Скрипт сборки
├── .cargo/
│   └── config.toml      # Конфигурация Rust для ESP
└── README.md            # Этот файл
```

## Работа с PlatformIO

Если предпочитаете PlatformIO:

1. Установите VS Code + PlatformIO extension
2. Создайте проект: PlatformIO → New Project
3. Выберите: Board → ESP32, Framework → esp-idf
4. Скопируйте `Cargo.toml` и `src/main.rs`
5. Настройте `platformio.ini`:
```ini
[env:esp32dev]
platform = espressif32
board = esp32dev
framework = espidf
```

## Возможные проблемы

### WiFi не подключается
- Проверьте имя и пароль сети
- Убедитесь, что WiFi 2.4GHz

### Не загружается на ESP32
- Проверьте драйвер USB-UART
- Выберите правильный COM порт

### Ошибки компиляции
```bash
# Обновите espup
cargo install espup
espup install
```

## Ресурсы

- [esp-rs книга](https://esp-rs.github.io/book/)
- [esp-hal документация](https://docs.rs/esp-hal/latest/esp_hal/)
- [Rust для ESP32 учебник](https://esp-rs.github.io/no_std-training/)
