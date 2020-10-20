//! A `proto-hal` example that runs on the Teensy 4 (4.0, 4.1).
//! 
//! The example uses a GPT to blink the LED. Every blink, it writes an `'X'`
//! character over serial.
//!
//! - pin 14 is Teensy TX, host RX
//! - pin 15 is Teensy RX, host TX
//! - baud rate: 115200 bps
//! 
//! # Building
//! 
//! ```text
//! cd examples/teensy4
//! cargo objcopy --release --target thumbv7em-none-eabihf  -- -O ihex main.hex
//! teensy_loader_cli -w -v --mcu=TEENSY40 main.hex
//! ```


#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
extern crate panic_halt;
#[cfg(target_arch = "arm")]
extern crate t4_startup;

use proto_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Safety: don't care for this demo
    //
    // End users would use safe API.
    let pads = unsafe { hal::iomuxc::imxrt106x::Pads::new() };
    let pins = teensy4_pins::t40::into_pins(pads);
    let mut led = hal::GPIO::new(pins.p13).output();

    let hal::ccm::CCM {
        mut handle,
        perclock,
        uart_clock,
        ..
    } = unsafe { hal::ccm::CCM::new() }; // Safety: don't care for this demo

    let mut perclock = perclock.enable(&mut handle);
    let mut gpt = hal::ral::gpt::GPT2::take().unwrap();
    perclock.clock_gate_gpt(&mut gpt, hal::ccm::ClockGate::On);
    let mut timer = hal::GPT::new(gpt, &perclock);

    timer.set_wait_mode_enable(true);

    let mut uart_clock = uart_clock.enable(&mut handle);
    let mut uart = hal::ral::lpuart::LPUART2::take().unwrap();
    uart_clock.clock_gate(&mut uart, hal::ccm::ClockGate::On);

    // TODO there's no static guarantee that the UART matches with
    // the TX and RX pins. That's out of scope for this demo. We've
    // already shown that works in both the HAL and async HAL.
    let mut serial = hal::UART::new(uart, pins.p14, pins.p15, &uart_clock);
    serial.set_baud(115_200).unwrap();
    led.set();

    loop {
        delay::delay(&mut timer);
        led.toggle();
        serial.write(b'X').unwrap();
    }
}
