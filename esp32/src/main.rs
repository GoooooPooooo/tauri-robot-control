//! ESP32 Robot Car Controller (Rust - no_std)
//!
//! Минимальная прошивка для управления роботом.

#![no_std]
#![no_main]

use esp-hal::{delay::FreeRtosDelay, prelude::*};
use esp-hal::gpio::IO;

#[entry]
fn main() -> ! {
    // Инициализация периферии
    let peripherals = esp-hal::peripherals::Peripherals::take().unwrap();
    let io = IO::new(peripherals.GPIO);
    
    // Тестовый пин (встроенный LED на ESP32)
    let led = io.pins.gpio2.into_push_pull_output();
    
    let delay = FreeRtosDelay {};

    loop {
        // Тестовая заглушка - мигание светодиодом
        led.set_high().ok();
        delay.delay_ms(500u32);
        led.set_low().ok();
        delay.delay_ms(500u32);
    }
}
