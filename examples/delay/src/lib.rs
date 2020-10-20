//! A dummy library that implements a GPT delay
//!
//! This library designs to the common HAL. Note that
//! it does not need to enable a HAL feature to provide
//! this implementation

#![no_std]

use proto_hal as hal;

/// Wait a bit
pub fn delay(timer: &mut hal::GPT) {
    const FIVE_HUNDRED_MILLISECONDS_IN_TICKS: u32 = 100000;
    const OCR: hal::OutputCompareRegister = hal::OutputCompareRegister::One;

    timer.set_enable(false);
    let count = timer.output_compare_count(OCR);
    let count = count.wrapping_add(FIVE_HUNDRED_MILLISECONDS_IN_TICKS);
    timer.set_output_compare_count(OCR, count);
    timer.set_enable(true);
    let mut status = timer.output_compare_status(OCR);
    while !status.is_set() {}
    status.clear();
    timer.set_enable(false);
}
