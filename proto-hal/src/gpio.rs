use crate::iomuxc::{consts::Unsigned, gpio::Pin};
use crate::ral::{self, gpio::RegisterBlock};
use core::marker::PhantomData;

/// Indicates that a pin is configured as an input
pub enum Input {}
/// Indicates that a pin is configured as an output
pub enum Output {}
pub struct GPIO<P, D> {
    pin: P,
    dir: PhantomData<D>,
}

impl<P, D> GPIO<P, D>
where
    P: Pin,
{
    fn register_block(&self) -> *const RegisterBlock {
        #[allow(unreachable_patterns)]
        match self.module() {
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            1 => ral::gpio::GPIO1,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            2 => ral::gpio::GPIO2,
            #[cfg(any(feature = "imxrt1060"))]
            3 => ral::gpio::GPIO3,
            #[cfg(any(feature = "imxrt1060"))]
            4 => ral::gpio::GPIO4,
            #[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
            5 => ral::gpio::GPIO5,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn offset(&self) -> u32 {
        1u32 << <P as Pin>::Offset::USIZE
    }

    /// The return is a non-zero number, since the GPIO identifiers
    /// start with '1.'
    #[inline(always)]
    fn module(&self) -> usize {
        <P as Pin>::Module::USIZE
    }
}

impl<P> GPIO<P, Input>
where
    P: Pin,
{
    /// Create a GPIO from a pad that supports a GPIO configuration
    ///
    /// All pads may be used as a GPIO, so `new` should work with every `iomuxc` pad.
    ///
    /// ```no_run
    /// use imxrt_async_hal as hal;
    /// use hal::gpio::GPIO;
    ///
    /// let pads = hal::iomuxc::new(hal::ral::iomuxc::IOMUXC::take().unwrap());
    /// let input_pin = GPIO::new(pads.b0.p03);
    /// ```
    pub fn new(mut pin: P) -> Self {
        crate::iomuxc::gpio::prepare(&mut pin);
        Self {
            pin,
            dir: PhantomData,
        }
    }

    /// Transition the GPIO from an input to an output
    pub fn output(self) -> GPIO<P, Output> {
        // Safety: critical section ensures consistency
        cortex_m::interrupt::free(|_| unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                | self.offset());
        });
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Returns `true` if this input pin is high
    pub fn is_set(&self) -> bool {
        // Safety: read is atomic
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), PSR) & self.offset() != 0 }
    }
}

impl<P> GPIO<P, Output>
where
    P: Pin,
{
    /// Transition the pin from an output to an input
    pub fn input(self) -> GPIO<P, Input> {
        // Safety: critical section ensures consistency
        cortex_m::interrupt::free(|_| unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                & !self.offset());
        });
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Drive the GPIO high
    pub fn set(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_SET, self.offset()) };
    }

    /// Drive the GPIO low
    pub fn clear(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_CLEAR, self.offset()) };
    }

    /// Returns `true` if the pin is driving high
    pub fn is_set(&self) -> bool {
        // Safety: atomic read
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), DR) & self.offset() != 0u32 }
    }

    /// Alternate the state of the pin
    ///
    /// Using `toggle` will be more efficient than checking [`is_set`](#method.is_set)
    /// and then selecting the opposite state.
    pub fn toggle(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_TOGGLE, self.offset()) }
    }
}
