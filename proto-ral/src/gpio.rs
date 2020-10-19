mod fields;
pub use fields::*;

#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
mod instances;
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub use instances::*;

use crate::{RORegister, RWRegister, WORegister};
use core::marker::PhantomData;

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
