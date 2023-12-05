#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_println as _;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, Uart, IO};
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio12.into_push_pull_output();

    let mut uart0 = Uart::new(peripherals.UART0, &clocks);
    loop {
        defmt::println!("on!");
        write!(uart0, "on!").ok();
        led.set_high().unwrap();
        delay.delay_ms(500u16);
        defmt::println!("off!");
        write!(uart0, "off!").ok();
        led.set_low().unwrap();
    }
}
