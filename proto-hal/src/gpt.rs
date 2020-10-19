use crate::ral;
use core::time::Duration;

/// General purpose timers (GPT)
pub struct GPT {
    /// Registers for this GPT instance
    registers: ral::gpt::Instance,
}

/// GPT clock divider
///
/// This crystal oscillator is very sensitive. Not all values
/// seem to work. 5 is one of them that does. So is 3. 10 does
/// not work. The field is supposed to support values up to 0xF.
///
/// The seL4 project also notes issues with this divider value.
/// Can't find anything in the errata...
const DIVIDER: u32 = 5;

/// GPT effective frequency
const CLOCK_HZ: u32 = crate::ccm::PERCLOCK_FREQUENCY_HZ / DIVIDER;
const _CLOCK_PERIOD_US: u32 = 1_000_000u32 / CLOCK_HZ;
const _STATIC_ASSERT: [u32; 1] = [0; (_CLOCK_PERIOD_US == 5) as usize];

/// An output compare register (OCR)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputCompareRegister {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Possible modes of the GPT
pub enum Mode {
    /// Reset mode
    ///
    /// A comparions event on channel 1 will reset the GPT counter.
    /// Comparison events on channels 2 and 3 do not reset the counter.
    Reset,
    /// Free running mode
    ///
    /// Comparisons on channel 1 are treated like comparions on channels
    /// 2 and 3. The counter continues to increment on comparison.
    FreeRunning,
}

impl GPT {
    /// Create a new `GPT` from a RAL GPT instance
    pub fn new(gpt: ral::gpt::Instance, _: &crate::ccm::PerClock<ral::gpt::Instance>) -> Self {
        ral::write_reg!(
            ral::gpt,
            gpt,
            CR,
            EN_24M: 1, // Enable crystal oscillator
            CLKSRC: 0b101 // Crystal Oscillator
        );
        ral::write_reg!(ral::gpt, gpt, PR, PRESCALER24M: DIVIDER - 1);

        // Clear all statuses
        ral::write_reg!(ral::gpt, gpt, SR, 0b11_1111);
        ral::modify_reg!(
            ral::gpt, gpt, IR,
            ROVIE: 0 // Rollover interrupt disabled
        );
        ral::modify_reg!(
            ral::gpt, gpt, CR,
            FRR: 1, // Free-running mode, no matter the output compare channel
            WAITEN: 1, // Run in wait mode
            ENMOD: 0, // Counter maintains value when disabled
            EN: 1 // Start the timer
        );

        GPT { registers: gpt }
    }

    /// Returns the current mode of the GPT
    pub fn mode(&self) -> Mode {
        if ral::read_reg!(ral::gpt, self.registers, CR, FRR == 0) {
            Mode::Reset
        } else {
            Mode::FreeRunning
        }
    }

    /// Set the GPT mode
    ///
    /// Refer to the module level documentation for more information on the GPT modes.
    pub fn set_mode(&mut self, mode: Mode) {
        ral::modify_reg!(ral::gpt, self.registers, CR, FRR: (mode as u32))
    }

    /// Set the reset on enable behavior
    ///
    /// See the module level docs for more information.
    pub fn set_reset_on_enable(&mut self, reset_on_enable: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, ENMOD: (reset_on_enable as u32));
    }

    /// Returns `true` if the GPT counter will reset the next time it is enabled
    pub fn reset_on_enable(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, ENMOD == 1)
    }

    /// Enable or disable the GPT
    ///
    /// When enabled, the counter starts counting. When disabled, the counter will
    /// stop counting.
    pub fn set_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, EN: (enable as u32));
    }

    /// Indicates if the GPT is enabled (`true`) or disabled (`false`).
    pub fn enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, EN == 1)
    }

    /// Allow the GPT to run in wait mode; or, prevent the GPT from running
    /// in wait mode.
    pub fn set_wait_mode_enable(&mut self, wait: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, WAITEN: (wait as u32));
    }

    /// Indicates if the GPT runs while in wait mode
    pub fn wait_mode_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, WAITEN == 1)
    }

    /// Returns the current count of the GPT
    pub fn count(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.registers, CNT)
    }

    /// Set an output compare register to trigger on the next `count` value of the
    /// counter.
    pub fn set_output_compare_count(&mut self, output: OutputCompareRegister, count: u32) {
        match output {
            OutputCompareRegister::One => ral::write_reg!(ral::gpt, self.registers, OCR1, count),
            OutputCompareRegister::Two => ral::write_reg!(ral::gpt, self.registers, OCR2, count),
            OutputCompareRegister::Three => ral::write_reg!(ral::gpt, self.registers, OCR3, count),
        }
    }

    /// Returns the current output compare count for the specified register
    pub fn output_compare_count(&self, output: OutputCompareRegister) -> u32 {
        match output {
            OutputCompareRegister::One => ral::read_reg!(ral::gpt, self.registers, OCR1),
            OutputCompareRegister::Two => ral::read_reg!(ral::gpt, self.registers, OCR2),
            OutputCompareRegister::Three => ral::read_reg!(ral::gpt, self.registers, OCR3),
        }
    }

    /// Returns a handle that can query and modify the output compare status for the provided output
    pub fn output_compare_status(&mut self, output: OutputCompareRegister) -> OutputCompareStatus {
        OutputCompareStatus { gpt: self, output }
    }

    /// Returns the clock period as a duration
    ///
    /// This represents the resolution of the clock. The maximum measurement
    /// interval is `clock_period() * u32::max_value()`.
    pub fn clock_period(&self) -> Duration {
        Duration::from_nanos((1_000_000_000u32 / CLOCK_HZ).into())
    }
}

/// A handle to evaluate and modify the output compare status
pub struct OutputCompareStatus<'a> {
    gpt: &'a mut GPT,
    output: OutputCompareRegister,
}

impl<'a> OutputCompareStatus<'a> {
    /// Returns true if this output compare has triggered
    pub fn is_set(&self) -> bool {
        let sr = ral::read_reg!(ral::gpt, self.gpt.registers, SR);
        sr & (1 << (self.output as u32)) != 0
    }

    /// Clear the output compare status flag
    ///
    /// It's necessary to clear the flag when the comparison has triggered
    /// an interrupt.
    pub fn clear(&mut self) {
        ral::write_reg!(ral::gpt, self.gpt.registers, SR, 1 << (self.output as u32));
    }
}
