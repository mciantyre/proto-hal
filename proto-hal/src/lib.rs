//! Prototype HAL for the i.MX RT processor family

#![no_std]

pub mod ccm;
mod gpio;
mod gpt;
mod uart;
pub use proto_ral as ral;

pub use gpio::{Input, Output, GPIO};
pub use gpt::{GPT, OutputCompareRegister};
pub use uart::UART;

pub mod iomuxc {
    pub use imxrt_iomuxc::*;
}
