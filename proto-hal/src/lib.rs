//! Prototype HAL for the i.MX RT processor family
//!
//! This HAL compiles with no feature flags. It compiles with one or
//! more feature flags. It compiles with *all* feature flags! This
//! crate represents an ideal for a common HAL crate: it has a driver
//! that works across all chips, and builds without required feature
//! flags.
//!
//! The goal is to explore how a unified HAL, or a split HAL with a
//! common crate, might work. Do not use this; it will not be maintained.
//!
//! # Structure
//!
//! - A custom, RAL-like interface, called `proto-ral`. This crate is
//!   designed to build without feature flags. As you add in feature
//!   flags, concrete instance modules and constants are included.
//!   this works because the UART register block and driver is the same
//!   across all i.MX RT procesors (1010, 1015, 1020, 1050, 1060, 1064).
//!
//! - The `imxrt-iomuxc` crate. We already know how this works.
//!
//! - A prototype driver for the CCM. I've been working on this driver on
//!   the side. I took the important parts and moved them into the CCM module
//!   in this crate.
//!
//! - The HAL, `proto-hal`. The HAL uses the RAL. It's lifted from the async HAL
//!   prototype, without the async parts.
//!
//! # Usage
//!
//! Library implementers will never need to compile this crate with feature flags.
//! End users who are creating the final program will select a feature flag. Or, we
//! provide a thin, chip-specific HAL crate that enables the feature flag on their
//! behalf.
//!
//! The chip-specific HAL crate should also include drivers that are specific for that
//! chip. However, we should not block users from using the common HAL to design common,
//! higher-level drivers.
//!
//! # Next Steps
//!
//! - Explore an implementation for other peripheral drivers. I picked UART because it
//!   conveniently works across all chips. I know that DMA works as well. We should pick
//!   other peripherals to make sure this common HAL can have some real structure to it.

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
