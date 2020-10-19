//! i.MX RT Clock Control Module (CCM)

mod perclock;
mod uart;

pub use perclock::{
    clock_gate_gpt, configure as configure_perclock, CLOCK_FREQUENCY_HZ as PERCLOCK_FREQUENCY_HZ,
    GPT,
};
pub use uart::{
    clock_gate as clock_gate_uart, configure as configure_uart,
    CLOCK_FREQUENCY_HZ as UART_CLOCK_FREQUENCY_HZ, UART,
};

use crate::ral;
use core::marker::PhantomData;

/// A peripheral instance whose clock can be gated
///
/// # Safety
///
/// You should only implement `Instance` on a true i.MX RT peripheral instance.
/// `Instance`s are only used when you have both a mutable reference to the instance,
/// and a mutable reference to the CCM [`Handle`](struct.Handle.html). If you incorrectly
/// implement `Instance`, you can violate the safety associted with accessing global,
/// mutable state.
pub unsafe trait Instance {
    /// An identifier that describes the instance
    type Inst: Copy + PartialEq;
    /// Returns the identifier that describes this peripheral instance
    fn instance(&self) -> Self::Inst;
    /// Returns `true` if this instance is valid for a particular
    /// implementation.
    fn is_valid(inst: Self::Inst) -> bool;
}

/// Returns `Some(inst)` if `inst` is valid for this peripheral, or
/// `None` if `inst` is not valid.
#[inline(always)]
fn check_instance<I: Instance>(inst: I::Inst) -> Option<I::Inst> {
    Some(inst).filter(|inst| I::is_valid(*inst))
}

/// Handle to the CCM register block
///
/// `Handle` also supports clock gating for peripherals that
/// don't have an obvious clock root, like DMA.
pub struct Handle(());

/// The root clocks and CCM handle
///
/// Most root clocks are disabled. Call `enable`, and supply the
/// `handle`, to enable them.
#[non_exhaustive]
pub struct CCM {
    /// The handle to the CCM register block
    ///
    /// `Handle` is used throughout the HAL
    pub handle: Handle,
    /// The periodic clock handle
    ///
    /// `perclock` is used for timers, including GPT and PIT timers
    pub perclock: Disabled<PerClock<ral::gpt::Instance>>,
    /// The UART clock
    ///
    /// `uart_clock` is for UART peripherals.
    pub uart_clock: Disabled<UARTClock<ral::lpuart::Instance>>,
}

impl CCM {
    /// Construct a new CCM peripheral
    ///
    /// # Safety
    ///
    /// This should only be called once. Ideally, it's encapsulated behind another
    /// constructor that takes ownership of CCM peripheral memory. Calling this more
    /// than once will let you access global, mutable memory that's assumed to not
    /// be aliased.
    pub const unsafe fn new() -> Self {
        CCM {
            handle: Handle(()),
            perclock: Disabled(PerClock::assume_enabled()),
            uart_clock: Disabled(UARTClock::assume_enabled()),
        }
    }
}

/// Describes a clock gate setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockGate {
    /// Clock is off during all modes
    ///
    /// Stop enter hardware handshake is disabled.
    Off = 0b00,
    /// Clock is on in run mode, but off in wait and stop modes
    OnlyRun = 0b01,
    /// Clock is on in all modes, except stop mode
    On = 0b11,
}

/// Crystal oscillator frequency
const OSCILLATOR_FREQUENCY_HZ: u32 = 24_000_000;

/// A disabled clock of type `Clock`
///
/// Call `enable` on your instance to enable the clock.
pub struct Disabled<Clock>(Clock);

/// The periodic clock root
///
/// `PerClock` is the input clock for GPT and PIT. It runs at
/// 1MHz.
pub struct PerClock<G>(PhantomData<G>);

impl<G> PerClock<G> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

/// The UART clock
pub struct UARTClock<C>(PhantomData<C>);

impl<C> UARTClock<C> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

/// Starting address of the clock control gate registers
const CCGR_BASE: *mut u32 = 0x400F_C068 as *mut u32;

/// # Safety
///
/// Should only be used when you have a mutable reference to an enabled clock.
/// Should only be used on a valid clock gate register.
#[inline(always)]
unsafe fn set_clock_gate(ccgr: *mut u32, gates: &[usize], value: u8) {
    const MASK: u32 = 0b11;
    let mut register = core::ptr::read_volatile(ccgr);

    for gate in gates {
        let shift: usize = gate * 2;
        register &= !(MASK << shift);
        register |= (MASK & (value as u32)) << shift;
    }

    core::ptr::write_volatile(ccgr, register);
}

#[cfg(test)]
mod tests {
    use super::set_clock_gate;

    #[test]
    fn test_set_clock_gate() {
        let mut reg = 0;

        unsafe {
            set_clock_gate(&mut reg, &[3, 7], 0b11);
        }
        assert_eq!(reg, (0b11 << 14) | (0b11 << 6));

        unsafe {
            set_clock_gate(&mut reg, &[3], 0b1);
        }
        assert_eq!(reg, (0b11 << 14) | (0b01 << 6));
    }
}
