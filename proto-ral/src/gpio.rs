mod fields;
pub use fields::*;

#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
mod instances;
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub use instances::*;

use crate::{RORegister, RWRegister, WORegister};
use core::marker::PhantomData;

#[repr(C)]
pub struct RegisterBlock {
    /// GPIO data register
    pub DR: RWRegister<u32>,

    /// GPIO direction register
    pub GDIR: RWRegister<u32>,

    /// GPIO pad status register
    pub PSR: RORegister<u32>,

    /// GPIO interrupt configuration register1
    pub ICR1: RWRegister<u32>,

    /// GPIO interrupt configuration register2
    pub ICR2: RWRegister<u32>,

    /// GPIO interrupt mask register
    pub IMR: RWRegister<u32>,

    /// GPIO interrupt status register
    pub ISR: RWRegister<u32>,

    /// GPIO edge select register
    pub EDGE_SEL: RWRegister<u32>,

    _reserved1: [u32; 25],

    /// GPIO data register SET
    pub DR_SET: WORegister<u32>,

    /// GPIO data register CLEAR
    pub DR_CLEAR: WORegister<u32>,

    /// GPIO data register TOGGLE
    pub DR_TOGGLE: WORegister<u32>,
}

pub struct ResetValues {
    pub DR: u32,
    pub GDIR: u32,
    pub PSR: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub IMR: u32,
    pub ISR: u32,
    pub EDGE_SEL: u32,
    pub DR_SET: u32,
    pub DR_CLEAR: u32,
    pub DR_TOGGLE: u32,
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
