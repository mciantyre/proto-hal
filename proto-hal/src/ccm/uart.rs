//! UART clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, UARTClock, CCGR_BASE};
use crate::ral;

/// UART clock frequency (Hz)
pub const CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ;

impl<U> Disabled<UARTClock<U>> {
    /// Enable the UART clocks
    ///
    /// When `enable` returns, all UART clock gates will be set to off.
    /// Use [`clock_gate`](struct.UARTClock.html#method.clock_gate)
    /// to turn on UART clock gates.
    #[inline(always)]
    pub fn enable(self, _: &mut Handle) -> UARTClock<U>
    where
        U: Instance<Inst = UART>,
    {
        unsafe {
            clock_gate::<U>(UART::UART1, ClockGate::Off);
            clock_gate::<U>(UART::UART2, ClockGate::Off);
            clock_gate::<U>(UART::UART3, ClockGate::Off);
            clock_gate::<U>(UART::UART4, ClockGate::Off);
            clock_gate::<U>(UART::UART5, ClockGate::Off);
            clock_gate::<U>(UART::UART6, ClockGate::Off);
            clock_gate::<U>(UART::UART7, ClockGate::Off);
            clock_gate::<U>(UART::UART8, ClockGate::Off);

            configure()
        };
        self.0
    }
}

/// Peripheral instance identifier for UART
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UART {
    UART1,
    UART2,
    UART3,
    UART4,
    UART5,
    UART6,
    UART7,
    UART8,
}

impl<U> UARTClock<U> {
    /// Set the clock gate for the UART instance
    #[inline(always)]
    pub fn clock_gate(&mut self, uart: &mut U, gate: ClockGate)
    where
        U: Instance<Inst = UART>,
    {
        unsafe { clock_gate::<U>(uart.instance(), gate) }
    }
}

/// Set the clock gate for a UART peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`UARTClock`](struct.UARTClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate<U: Instance<Inst = UART>>(uart: UART, gate: ClockGate) {
    let value = gate as u8;
    match super::check_instance::<U>(uart) {
        Some(UART::UART1) => set_clock_gate(CCGR_BASE.add(5), &[12], value),
        Some(UART::UART2) => set_clock_gate(CCGR_BASE.add(0), &[14], value),
        Some(UART::UART3) => set_clock_gate(CCGR_BASE.add(0), &[6], value),
        Some(UART::UART4) => set_clock_gate(CCGR_BASE.add(1), &[12], value),
        Some(UART::UART5) => set_clock_gate(CCGR_BASE.add(3), &[1], value),
        Some(UART::UART6) => set_clock_gate(CCGR_BASE.add(3), &[3], value),
        Some(UART::UART7) => set_clock_gate(CCGR_BASE.add(5), &[13], value),
        Some(UART::UART8) => set_clock_gate(CCGR_BASE.add(6), &[7], value),
        _ => (),
    }
}

/// Configure the UART clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`UARTClock`](struct.UARTClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
    const CSCDR1: *mut u32 = 0x400F_C024 as *mut u32;
    const UART_CLK_PODF_OFFSET: u32 = 0;
    const UART_CLK_PODF_MASK: u32 = 0x3F << UART_CLK_PODF_OFFSET;
    const UART_CLK_SEL_OFFSET: u32 = 6;
    const UART_CLK_SEL_MASK: u32 = 0x3 << UART_CLK_SEL_OFFSET; // Note that the mask is 1 for 1011, but the adjacent bit is reserved
    const OSCILLATOR: u32 = 1; // Same value for 1062, 1011
    const DIVIDE_1: u32 = 0;

    let mut cscdr1 = CSCDR1.read_volatile();
    cscdr1 &= !(UART_CLK_PODF_MASK | UART_CLK_SEL_MASK);
    cscdr1 |= DIVIDE_1 << UART_CLK_PODF_OFFSET;
    cscdr1 |= OSCILLATOR << UART_CLK_SEL_OFFSET;
    CSCDR1.write_volatile(cscdr1);
}

unsafe impl Instance for ral::lpuart::Instance {
    type Inst = UART;
    #[inline(always)]
    fn instance(&self) -> UART {
        match &**self as *const _ {
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::lpuart::LPUART1 => UART::UART1,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::lpuart::LPUART2 => UART::UART2,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::lpuart::LPUART3 => UART::UART3,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            ral::lpuart::LPUART4 => UART::UART4,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART5 => UART::UART5,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART6 => UART::UART6,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART7 => UART::UART7,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART8 => UART::UART8,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    fn is_valid(uart: UART) -> bool {
        #[allow(unreachable_patterns)]
        match uart {
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            UART::UART1 | UART::UART2 | UART::UART3 | UART::UART4 => true,
            #[cfg(feature = "imxrt1060")]
            UART::UART5 | UART::UART6 | UART::UART7 | UART::UART8 => true,
            _ => false,
        }
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::lpuart::LPUART4;
///
/// let CCM{ mut handle, uart_clock, .. } = ccm::CCM::take().map(CCM::from_ral_ccm).unwrap();
/// let mut uart_clock = uart_clock.enable(&mut handle);
/// uart_clock.clock_gate(&mut LPUART4::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct UARTClockGate;
