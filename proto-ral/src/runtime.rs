#[cfg(feature = "imxrt1010")]
mod imxrt1010;
#[cfg(feature = "imxrt1060")]
mod imxrt1060;

#[cfg(feature = "imxrt1010")]
pub use imxrt1010::*;
#[cfg(feature = "imxrt1060")]
pub use imxrt1060::*;

// If we're in here, there must be a feature flag enabled.
// So, we know that we can do this...
pub use Interrupt as interrupt;