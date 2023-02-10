use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::{self, gpio, prelude::Peripherals, uart, units::Hertz};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use core::fmt::Write;
use std::prelude::rust_2021::*;

// use embedded_hal::serial
// use embedded_hal_nb::serial;

// use std::io::prelude::*;
// use std::io::Write;

fn test_esp_idf_hal_uart() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = uart::config::Config {
        baudrate: Hertz(115_200),
        ..Default::default()
    };

    // peripherals.uart1

    let mut uart = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio0,
        pins.gpio1,
        None::<gpio::Gpio0>,
        None::<gpio::Gpio0>,
        &config,
    )
    .unwrap();

    // uart.write(&[0xaa])?;
    // uart.write_char('x')?;
    uart.write_str("Hello, world!")?;


    let mut buf = [0_u8; 13];
    uart.read(&mut buf, BLOCK)?;

    print!("Read: {}", String::from_utf8_lossy(&buf));

    Ok(())
}

fn test_embedded_hal_nb_uart() {}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    test_esp_idf_hal_uart()?;
    test_embedded_hal_nb_uart();

    Ok(())
}
