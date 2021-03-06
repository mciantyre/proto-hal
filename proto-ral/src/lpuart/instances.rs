//! LPUART instances
//!
//! These are conditioned on chip support. However, they're **not**
//! required to build the RAL. They'll only be required when an end
//! user wants to run a program on an embedded system.

//! LPUART
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

pub use crate::lpuart::Instance;
pub use crate::lpuart::{RegisterBlock, ResetValues};
pub use crate::lpuart::{
    BAUD, CTRL, DATA, FIFO, GLOBAL, MATCH, MODIR, PARAM, PINCFG, STAT, VERID, WATER,
};

/// Access functions for the LPUART1 peripheral instance
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub mod LPUART1 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x40184000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART1_TAKEN: bool = false;

    /// Safe access to LPUART1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART1_TAKEN {
                None
            } else {
                LPUART1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART1_TAKEN && inst.addr == INSTANCE.addr {
                LPUART1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub const LPUART1: *const RegisterBlock = 0x40184000 as *const _;

/// Access functions for the LPUART2 peripheral instance
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub mod LPUART2 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x40188000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART2_TAKEN: bool = false;

    /// Safe access to LPUART2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART2_TAKEN {
                None
            } else {
                LPUART2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART2_TAKEN && inst.addr == INSTANCE.addr {
                LPUART2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub const LPUART2: *const RegisterBlock = 0x40188000 as *const _;

/// Access functions for the LPUART3 peripheral instance
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub mod LPUART3 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x4018c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART3_TAKEN: bool = false;

    /// Safe access to LPUART3
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART3_TAKEN {
                None
            } else {
                LPUART3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART3_TAKEN && inst.addr == INSTANCE.addr {
                LPUART3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub const LPUART3: *const RegisterBlock = 0x4018c000 as *const _;

/// Access functions for the LPUART4 peripheral instance
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub mod LPUART4 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x40190000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART4_TAKEN: bool = false;

    /// Safe access to LPUART4
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART4_TAKEN {
                None
            } else {
                LPUART4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART4_TAKEN && inst.addr == INSTANCE.addr {
                LPUART4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1010", feature = "imxrt1060"))]
pub const LPUART4: *const RegisterBlock = 0x40190000 as *const _;

/// Access functions for the LPUART5 peripheral instance
#[cfg(any(feature = "imxrt1060"))]
pub mod LPUART5 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x40194000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART5
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART5_TAKEN: bool = false;

    /// Safe access to LPUART5
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART5_TAKEN {
                None
            } else {
                LPUART5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART5_TAKEN && inst.addr == INSTANCE.addr {
                LPUART5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1060"))]
pub const LPUART5: *const RegisterBlock = 0x40194000 as *const _;

/// Access functions for the LPUART6 peripheral instance
#[cfg(any(feature = "imxrt1060"))]
pub mod LPUART6 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x40198000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART6
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART6_TAKEN: bool = false;

    /// Safe access to LPUART6
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART6_TAKEN {
                None
            } else {
                LPUART6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART6_TAKEN && inst.addr == INSTANCE.addr {
                LPUART6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1060"))]
pub const LPUART6: *const RegisterBlock = 0x40198000 as *const _;

/// Access functions for the LPUART7 peripheral instance
#[cfg(any(feature = "imxrt1060"))]
pub mod LPUART7 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x4019c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART7
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART7_TAKEN: bool = false;

    /// Safe access to LPUART7
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART7_TAKEN {
                None
            } else {
                LPUART7_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART7_TAKEN && inst.addr == INSTANCE.addr {
                LPUART7_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART7_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1060"))]
pub const LPUART7: *const RegisterBlock = 0x4019c000 as *const _;

/// Access functions for the LPUART8 peripheral instance
#[cfg(any(feature = "imxrt1060"))]
pub mod LPUART8 {
    use super::ResetValues;

    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0x401a0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART8
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART8_TAKEN: bool = false;

    /// Safe access to LPUART8
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART8_TAKEN {
                None
            } else {
                LPUART8_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART8
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        cortex_m::interrupt::free(|_| unsafe {
            if LPUART8_TAKEN && inst.addr == INSTANCE.addr {
                LPUART8_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART8
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART8_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART8
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
#[cfg(any(feature = "imxrt1060"))]
pub const LPUART8: *const RegisterBlock = 0x401a0000 as *const _;
