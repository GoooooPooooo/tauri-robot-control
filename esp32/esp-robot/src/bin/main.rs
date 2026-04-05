#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::timer::timg::TimerGroup;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

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
        match s.trim() {
            "forward" => Command::Forward,
            "backward" => Command::Backward,
            "left" => Command::Left,
            "right" => Command::Right,
            _ => Command::Stop,
        }
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE");
    let (_wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let output_config = OutputConfig::default();

    let mut in1 = Output::new(peripherals.GPIO26, Level::Low, output_config.clone());
    let mut in2 = Output::new(peripherals.GPIO27, Level::Low, output_config.clone());
    let mut in3 = Output::new(peripherals.GPIO14, Level::Low, output_config.clone());
    let mut in4 = Output::new(peripherals.GPIO12, Level::Low, output_config.clone());
    let _ena = Output::new(peripherals.GPIO25, Level::High, output_config.clone());
    let _enb = Output::new(peripherals.GPIO33, Level::High, output_config);

    let mut current_command = Command::Stop;

    loop {
        match current_command {
            Command::Stop => {
                in1.set_low();
                in2.set_low();
                in3.set_low();
                in4.set_low();
            }
            Command::Forward => {
                in1.set_high();
                in2.set_low();
                in3.set_high();
                in4.set_low();
            }
            Command::Backward => {
                in1.set_low();
                in2.set_high();
                in3.set_low();
                in4.set_high();
            }
            Command::Left => {
                in1.set_low();
                in2.set_high();
                in3.set_high();
                in4.set_low();
            }
            Command::Right => {
                in1.set_high();
                in2.set_low();
                in3.set_low();
                in4.set_high();
            }
        }

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }
}
