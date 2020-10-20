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
    led.set();
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}