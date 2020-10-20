//! UART serial driver

use crate::{iomuxc, ral};

/// UART Serial driver
pub struct UART<TX, RX> {
    uart: ral::lpuart::Instance,
    tx: TX,
    rx: RX,
}

impl<TX, RX, M> UART<TX, RX>
where
    TX: iomuxc::uart::Pin<Direction = iomuxc::uart::TX, Module = M>,
    RX: iomuxc::uart::Pin<Direction = iomuxc::uart::RX, Module = M>,
    M: iomuxc::consts::Unsigned,
{
    /// Create a new `UART` from a UART instance, TX and RX pins, and a DMA channel
    ///
    /// The baud rate of the returned `UART` is unspecified. Make sure you use [`set_baud`](#method.set_baud)
    /// to properly configure the driver.
    pub fn new(
        uart: ral::lpuart::Instance,
        mut tx: TX,
        mut rx: RX,
        _: &crate::ccm::UARTClock<ral::lpuart::Instance>,
    ) -> UART<TX, RX> {
        crate::iomuxc::uart::prepare(&mut tx);
        crate::iomuxc::uart::prepare(&mut rx);

        let mut uart = UART { uart, tx, rx };
        let _ = uart.set_baud(9600);
        ral::modify_reg!(ral::lpuart, uart.uart, CTRL, TE: TE_1, RE: RE_1);
        uart
    }
}

impl<TX, RX> UART<TX, RX> {
    /// Set the serial baud rate
    ///
    /// If there is an error, the error is [`Error::Clock`](enum.UARTError.html#variant.Clock).
    pub fn set_baud(&mut self, baud: u32) -> Result<(), Error> {
        let timings = timings(UART_CLOCK, baud)?;
        self.while_disabled(|this| {
            ral::modify_reg!(
                ral::lpuart,
                this.uart,
                BAUD,
                OSR: u32::from(timings.osr),
                SBR: u32::from(timings.sbr),
                BOTHEDGE: u32::from(timings.both_edge)
            );
        });
        Ok(())
    }

    fn while_disabled<F: FnMut(&mut Self) -> R, R>(&mut self, mut act: F) -> R {
        ral::modify_reg!(
            ral::lpuart,
            self.uart,
            FIFO,
            TXFLUSH: TXFLUSH_1,
            RXFLUSH: RXFLUSH_1
        );
        let (te, re) = ral::read_reg!(ral::lpuart, self.uart, CTRL, TE, RE);
        ral::modify_reg!(ral::lpuart, self.uart, CTRL, TE: TE_0, RE: RE_0);
        let res = act(self);
        ral::modify_reg!(ral::lpuart, self.uart, CTRL, TE: te, RE: re);
        res
    }

    /// Return the pins, RAL instance, and DMA channel that comprise the UART driver
    pub fn release(self) -> (TX, RX, ral::lpuart::Instance) {
        (self.tx, self.rx, self.uart)
    }

    fn clear_status(&mut self) {
        ral::modify_reg!(
            ral::lpuart,
            self.uart,
            STAT,
            IDLE: IDLE_1,
            OR: OR_1,
            NF: NF_1,
            FE: FE_1,
            PF: PF_1
        );
    }

    /// Read a byte from the UART receiver
    pub async fn read(&mut self) -> Result<u8, ReadError> {
        use ral::lpuart::DATA::*;
        let data = ral::read_reg!(ral::lpuart, self.uart, DATA);
        if data & RXEMPT::mask != 0 {
            Err(ReadError {
                flags: ReadErrorFlags::WOULDBLOCK,
                raw: 0,
            })
        } else {
            let mut flags = ReadErrorFlags::empty();
            flags.set(
                ReadErrorFlags::OVERRUN,
                ral::read_reg!(ral::lpuart, self.uart, STAT, OR == OR_1),
            );
            flags.set(ReadErrorFlags::PARITY, data & PARITYE::mask != 0);
            flags.set(ReadErrorFlags::FRAME_ERROR, data & FRETSC::mask != 0);
            flags.set(ReadErrorFlags::NOISY, data & NOISY::mask != 0);

            let raw = (data & 0xFF) as u8;
            self.clear_status();

            if flags.is_empty() {
                Ok(raw)
            } else {
                Err(ReadError { flags, raw })
            }
        }
    }

    fn flush(&mut self) {
        while ral::read_reg!(ral::lpuart, self.uart, STAT, TDRE == TDRE_0) {}
    }

    /// Write a byte out of the UART peripheral
    pub fn write(&mut self, word: u8) -> Result<(), Error> {
        self.flush();
        ral::write_reg!(ral::lpuart, self.uart, DATA, word as u32);
        Ok(())
    }
}

const UART_CLOCK: u32 = crate::ccm::UART_CLOCK_FREQUENCY_HZ;

/// An opaque type that describes timing configurations
struct Timings {
    /// OSR register value. Accounts for the -1. May be written
    /// directly to the register
    osr: u8,
    /// True if we need to set BOTHEDGE given the OSR value
    both_edge: bool,
    /// SBR value;
    sbr: u16,
}

/// Errors propagated from a [`UART`](struct.UART.html) device
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// There was an error when preparing the baud rate or clocks
    Clock,
}

bitflags::bitflags! {
    /// Errors that may occur when reading data
    pub struct ReadErrorFlags : u8 {
        /// Data was received with noise
        const NOISY = 1 << 7;
        /// Parity error when receiving data
        const PARITY = 1 << 6;
        /// Framing error when receiving data
        const FRAME_ERROR = 1 << 5;
        /// Overrun occured, and we lost data in the shift register
        const OVERRUN = 1 << 4;
        /// Not a real flag
        const WOULDBLOCK = 1 << 0;
    }
}

/// Type that describes a read error
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadError {
    /// Decribes the reason for the error
    pub flags: ReadErrorFlags,
    /// The raw value read, if you'd like to consider it
    pub raw: u8,
}

/// Compute timings for a UART peripheral. Returns the timings,
/// or a string describing an error.
fn timings(effective_clock: u32, baud: u32) -> Result<Timings, Error> {
    //        effective_clock
    // baud = ---------------
    //         (OSR+1)(SBR)
    //
    // Solve for SBR:
    //
    //       effective_clock
    // SBR = ---------------
    //        (OSR+1)(baud)
    //
    // After selecting SBR, calculate effective baud.
    // Minimize the error over all OSRs.

    let base_clock: u32 = effective_clock.checked_div(baud).ok_or(Error::Clock)?;
    let mut error = u32::max_value();
    let mut best_osr = 16;
    let mut best_sbr = 1;

    for osr in 4..=32 {
        let sbr = base_clock.checked_div(osr).ok_or(Error::Clock)?;
        let sbr = sbr.max(1).min(8191);
        let effective_baud = effective_clock.checked_div(osr * sbr).ok_or(Error::Clock)?;
        let err = effective_baud.max(baud) - effective_baud.min(baud);
        if err < error {
            best_osr = osr;
            best_sbr = sbr;
            error = err
        }
    }

    use core::convert::TryFrom;
    Ok(Timings {
        osr: u8::try_from(best_osr - 1).map_err(|_| Error::Clock)?,
        sbr: u16::try_from(best_sbr).map_err(|_| Error::Clock)?,
        both_edge: best_osr < 8,
    })
}

/// ```no_run
/// use imxrt_async_hal as hal;
/// use hal::ral::{ccm::CCM, lpuart::LPUART2};
///
/// let hal::ccm::CCM {
///     mut handle,
///     uart_clock,
///     ..
/// } = CCM::take().map(hal::ccm::CCM::new).unwrap();
/// let mut uart_clock = uart_clock.enable(&mut handle);
/// let mut uart2 = LPUART2::take().unwrap();
/// uart_clock.clock_gate(&mut uart2, hal::ccm::ClockGate::On);
/// ```
#[cfg(doctest)]
struct ClockingWeakRalInstance;

/// ```no_run
/// use imxrt_async_hal as hal;
/// use hal::ral::{ccm::CCM, lpuart::LPUART2};
///
/// let hal::ccm::CCM {
///     mut handle,
///     uart_clock,
///     ..
/// } = CCM::take().map(hal::ccm::CCM::new).unwrap();
/// let mut uart_clock = uart_clock.enable(&mut handle);
/// let mut uart2: hal::instance::UART<hal::iomuxc::consts::U2> = LPUART2::take()
///     .and_then(hal::instance::uart)
///     .unwrap();
/// uart_clock.clock_gate(&mut uart2, hal::ccm::ClockGate::On);
/// ```
#[cfg(doctest)]
struct ClockingStrongHalInstance;
