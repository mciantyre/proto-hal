//! LPUART

mod fields;
pub use fields::*;

#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
mod instances;
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub use instances::*;

use crate::{RORegister, RWRegister};
use core::marker::PhantomData;

#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// LPUART Global Register
    pub GLOBAL: RWRegister<u32>,

    /// LPUART Pin Configuration Register
    pub PINCFG: RWRegister<u32>,

    /// LPUART Baud Rate Register
    pub BAUD: RWRegister<u32>,

    /// LPUART Status Register
    pub STAT: RWRegister<u32>,

    /// LPUART Control Register
    pub CTRL: RWRegister<u32>,

    /// LPUART Data Register
    pub DATA: RWRegister<u32>,

    /// LPUART Match Address Register
    pub MATCH: RWRegister<u32>,

    /// LPUART Modem IrDA Register
    pub MODIR: RWRegister<u32>,

    /// LPUART FIFO Register
    pub FIFO: RWRegister<u32>,

    /// LPUART Watermark Register
    pub WATER: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub GLOBAL: u32,
    pub PINCFG: u32,
    pub BAUD: u32,
    pub STAT: u32,
    pub CTRL: u32,
    pub DATA: u32,
    pub MATCH: u32,
    pub MODIR: u32,
    pub FIFO: u32,
    pub WATER: u32,
}

pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}

impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}
