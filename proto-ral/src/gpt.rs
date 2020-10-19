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
    /// GPT Control Register
    pub CR: RWRegister<u32>,

    /// GPT Prescaler Register
    pub PR: RWRegister<u32>,

    /// GPT Status Register
    pub SR: RWRegister<u32>,

    /// GPT Interrupt Register
    pub IR: RWRegister<u32>,

    /// GPT Output Compare Register 1
    pub OCR1: RWRegister<u32>,

    /// GPT Output Compare Register 2
    pub OCR2: RWRegister<u32>,

    /// GPT Output Compare Register 3
    pub OCR3: RWRegister<u32>,

    /// GPT Input Capture Register 1
    pub ICR1: RORegister<u32>,

    /// GPT Input Capture Register 2
    pub ICR2: RORegister<u32>,

    /// GPT Counter Register
    pub CNT: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub PR: u32,
    pub SR: u32,
    pub IR: u32,
    pub OCR1: u32,
    pub OCR2: u32,
    pub OCR3: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub CNT: u32,
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
