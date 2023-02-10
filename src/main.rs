use esp_idf_hal::{self, prelude::Peripherals, uart, units::Hertz};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn test_uart() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = uart::config::Config {
        baudrate: Hertz(115_200),
        ..Default::default()
    };

    let uart = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio0,
        pins.gpio1,
    )
}

fn main() {
    esp_idf_sys::link_patches();

    test_uart();
}
