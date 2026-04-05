use anyhow::Result;
use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::net::*;
use esp_idf_svc::sysloop::EspStdNetLoop;
use esp_idf_svc::wifi::*;
use log::info;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

const WIFI_SSID: &str = "star_room";
const WIFI_PASSWORD: &str = "08121955fF";
const SERVER_IP: &str = "192.168.50.19";
const SERVER_PORT: u16 = 8080;

enum Command {
    Stop,
    Forward,
    Backward,
    Left,
    Right,
}

impl Command {
    fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "FORWARD" => Command::Forward,
            "BACKWARD" => Command::Backward,
            "LEFT" => Command::Left,
            "RIGHT" => Command::Right,
            _ => Command::Stop,
        }
    }
}

fn get_command_from_server() -> Command {
    let addr = format!("{}:{}", SERVER_IP, SERVER_PORT);
    match TcpStream::connect_timeout(
        std::net::SocketAddr::new(SERVER_IP.parse().unwrap(), SERVER_PORT),
        Duration::from_secs(2),
    ) {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(b"GET /command HTTP/1.1\r\nHost: robot\r\n\r\n") {
                info!("HTTP request failed: {:?}", e);
                return Command::Stop;
            }

            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let response = String::from_utf8_lossy(&buffer);
                    if let Some(cmd) = response
                        .lines()
                        .find(|l| !l.is_empty() && !l.starts_with("HTTP"))
                    {
                        info!("Received command: {}", cmd);
                        return Command::from_str(cmd);
                    }
                }
                Err(e) => {
                    info!("Read failed: {:?}", e);
                }
            }
            Command::Stop
        }
        Err(e) => {
            info!("Connection failed: {:?}", e);
            Command::Stop
        }
    }
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let mut in1 = PinDriver::output(peripherals.pins.gpio26)?;
    let mut in2 = PinDriver::output(peripherals.pins.gpio27)?;
    let mut in3 = PinDriver::output(peripherals.pins.gpio14)?;
    let mut in4 = PinDriver::output(peripherals.pins.gpio12)?;
    let mut ena = PinDriver::output(peripherals.pins.gpio25)?;
    let mut enb = PinDriver::output(peripherals.pins.gpio33)?;

    in1.set_low()?;
    in2.set_low()?;
    in3.set_low()?;
    in4.set_low()?;
    ena.set_high()?;
    enb.set_high()?;

    let sys_loop = EspStdNetLoop::new()?;
    let wifi = EspWifi::new(
        peripherals.modem,
        Some(esp_idf_svc::nvs::EspDefaultNvsPartition::new()?),
        None,
    )?;

    let _wifi = WifiDriver::new(wifi, sys_loop, None)?;

    info!("Connecting to WiFi: {}", WIFI_SSID);

    let _ = _wifi.wifi().sta().connect(
        WIFI_SSID,
        WIFI_PASSWORD,
        None,
        Some(Duration::from_secs(20)),
    )?;

    info!("WiFi connected!");

    let _ = _wifi.netif().wait_netif_up(Some(Duration::from_secs(30)))?;

    let ip = _wifi.netif().get_ip_info()?;
    info!("WiFi IP: {:?}", ip);

    let mut current_command = Command::Stop;

    loop {
        let new_command = get_command_from_server();

        if matches!(current_command, Command::Stop) && !matches!(new_command, Command::Stop) {
            current_command = new_command;
        } else if !matches!(new_command, Command::Stop) {
            current_command = new_command;
        }

        match current_command {
            Command::Stop => {
                in1.set_low()?;
                in2.set_low()?;
                in3.set_low()?;
                in4.set_low()?;
            }
            Command::Forward => {
                in1.set_high()?;
                in2.set_low()?;
                in3.set_high()?;
                in4.set_low()?;
            }
            Command::Backward => {
                in1.set_low()?;
                in2.set_high()?;
                in3.set_low()?;
                in4.set_high()?;
            }
            Command::Left => {
                in1.set_low()?;
                in2.set_high()?;
                in3.set_high()?;
                in4.set_low()?;
            }
            Command::Right => {
                in1.set_high()?;
                in2.set_low()?;
                in3.set_low()?;
                in4.set_high()?;
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
