//! ESP32 Robot Car Controller (Rust)
//!
//! Подключается к WiFi и опрашивает сервер за командами.

#![no_std]
#![no_main]

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const SERVER_IP: &str = env!("SERVER_IP");

// Пины управления моторами (L298N)
const ENA_PIN: i32 = 5;
const IN1_PIN: i32 = 17;
const IN2_PIN: i32 = 18;
const IN3_PIN: i32 = 19;
const IN4_PIN: i32 = 21;
const ENB_PIN: i32 = 22;

#[entry]
fn main() -> ! {
    EspLogger::initialize_default();

    log::info!("========================================");
    log::info!("  ESP32 Robot Car Controller (Rust)");
    log::info!("========================================");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Настройка пинов как выходы
    let _in1 = pins.gpio17.into_output().unwrap();
    let _in2 = pins.gpio18.into_output().unwrap();
    let _in3 = pins.gpio19.into_output().unwrap();
    let _in4 = pins.gpio21.into_output().unwrap();

    // Настройка GPIO для ШИМ (упрощённо - просто вкл/выкл)
    let _ena = pins.gpio5.into_output().unwrap();
    let _enb = pins.gpio22.into_output().unwrap();

    log::info!("GPIO pins configured");

    // Подключение к WiFi
    match connect_wifi() {
        Ok(_) => log::info!("WiFi connected!"),
        Err(e) => {
            log::error!("WiFi error: {:?}", e);
            loop {
                FreeRtos::delay_ms(1000);
            }
        }
    }

    log::info!("========================================");
    log::info!("Robot car ready!");
    log::info!("Server: {}:8080", SERVER_IP);
    log::info!("========================================");

    // Главный цикл управления
    loop {
        log::info!("Polling server for commands...");

        // Здесь будет HTTP запрос к серверу
        // Пока заглушка - команда STOP
        let command = "STOP";

        log::info!("Command: {}", command);
        execute_command(command);

        FreeRtos::delay_ms(100); // 100ms
    }
}

fn execute_command(command: &str) {
    // Заглушка для команд
    match command {
        "FORWARD" => log::info!("Moving FORWARD"),
        "BACKWARD" => log::info!("Moving BACKWARD"),
        "LEFT" => log::info!("Turning LEFT"),
        "RIGHT" => log::info!("Turning RIGHT"),
        "STOP" => log::info!("STOPPED"),
        _ => log::info!("Unknown command: {}", command),
    }
}

fn connect_wifi() -> Result<(), esp_idf_svc::wifi::WifiError> {
    let netif_stack = EspNetifStack::new()?;
    let sys_loop_stack = EspSysLoopStack::new()?;
    let default_nvs = EspDefaultNvs::new()?;

    let mut wifi = EspWifi::new(netif_stack, sys_loop_stack, Some(default_nvs))?;

    log::info!("Connecting to WiFi: {}", WIFI_SSID);

    wifi.start(WifiMode::Sta)?;

    let config = StaConfiguration {
        ssid: WIFI_SSID.into(),
        password: WIFI_PASSWORD.into(),
        ..Default::default()
    };

    wifi.connect(&config.into())?;

    // Ждём подключения
    let timeout_ms = 30000;
    let start = esp_idf_hal::time::now().uptime_ms() as u64;

    while !wifi.is_connected().unwrap() {
        if esp_idf_hal::time::now().uptime_ms() as u64 - start > timeout_ms {
            return Err(esp_idf_svc::wifi::WifiError::ConnectTimeout);
        }
        FreeRtos::delay_ms(500);
    }

    log::info!("WiFi connection established");

    let ip_info = wifi.sta_netif().get_ip_info().unwrap();
    log::info!("IP: {:?}", ip_info.ip);

    Ok(())
}
