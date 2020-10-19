//! Periodic clock implementations

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, PerClock, CCGR_BASE};
use crate::ral;

/// Peripheral instance identifier for GPT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPT {
    GPT1,
    GPT2,
}

/// Periodic clock frequency (Hz)
///
/// This may be further divided by internal GPT dividers.
pub const CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ / PERIODIC_CLOCK_DIVIDER;
const PERIODIC_CLOCK_DIVIDER: u32 = 24;

impl<G> PerClock<G> {
    /// Set the clock gate for the GPT
    #[inline(always)]
    pub fn clock_gate_gpt(&mut self, gpt: &mut G, gate: ClockGate)
    where
        G: Instance<Inst = GPT>,
    {
        unsafe { clock_gate_gpt::<G>(gpt.instance(), gate) };
    }
}

impl<G> Disabled<PerClock<G>> {
    /// Enable the periodic clock root
    ///
    /// When `enable` returns, all GPT and PIT clock gates will be set to off. To
    /// re-enable clock gates, use the clock gate methods on [`PerClock`](struct.PerClock.html).
    #[inline(always)]
    pub fn enable(self, _: &mut Handle) -> PerClock<G>
    where
        G: Instance<Inst = GPT>,
    {
        unsafe {
            clock_gate_gpt::<G>(GPT::GPT1, ClockGate::Off);
            clock_gate_gpt::<G>(GPT::GPT2, ClockGate::Off);
            configure();
        };
        self.0
    }
}

/// Set the GPT clock gate
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate_gpt<G: Instance<Inst = GPT>>(gpt: GPT, gate: ClockGate) {
    let value = gate as u8;
    match super::check_instance::<G>(gpt) {
        Some(GPT::GPT1) => set_clock_gate(CCGR_BASE.add(1), &[10, 11], value),
        Some(GPT::GPT2) => set_clock_gate(CCGR_BASE.add(0), &[12, 13], value),
        _ => (),
    }
}

/// Configure the periodic clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
    const CSCMR1: *mut u32 = 0x400F_C01C as *mut u32;
    const PERCLK_PODF_OFFSET: u32 = 0;
    const PERCLK_PODF_MASK: u32 = 0x1F << PERCLK_PODF_OFFSET;
    const PERCLK_SEL_OFFSET: u32 = 6;
    const PERCLK_SEL_MASK: u32 = 0x01 << PERCLK_SEL_OFFSET;
    const OSCILLATOR: u32 = 1;

    let mut cscmr1 = CSCMR1.read_volatile();
    cscmr1 &= !(PERCLK_PODF_MASK | PERCLK_SEL_MASK);
    cscmr1 |= PERIODIC_CLOCK_DIVIDER.saturating_sub(1) << PERCLK_PODF_OFFSET;
    cscmr1 |= OSCILLATOR << PERCLK_SEL_OFFSET;
    CSCMR1.write_volatile(cscmr1);
}

unsafe impl Instance for ral::gpt::Instance {
    type Inst = GPT;
    #[inline(always)]
    fn instance(&self) -> GPT {
        match &**self as *const _ {
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::gpt::GPT1 => GPT::GPT1,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::gpt::GPT2 => GPT::GPT2,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn is_valid(gpt: GPT) -> bool {
        match gpt {
            GPT::GPT1 | GPT::GPT2 => true,
        }
    }
}
