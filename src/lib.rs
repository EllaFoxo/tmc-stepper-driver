#![no_std]

#[cfg(feature = "tmc2209")]
pub mod tmc2209;

pub struct Config {}

pub trait Driver {

    /// Initializes this stepper driver for use
    fn init();

    /// Configures the stepper driver with a default configuration
    fn defaults() -> Config;

    /// Returns the current stepper driver configuration
    fn get_config() -> Config;

    /// Configures the speed for the SPI link
    ///
    /// * `speed` - SPI frequency in Hz
    fn set_spi_speed(speed: u32);

    /// TMC drivers provide an internal clock generator. When precision or fine-tuning the clock
    /// frequency is required, this allows the use of an external clock signal, and disables the
    /// internal clock.
    ///
    /// When enabled, the CS pin is pulled high. If no clock is detected after a set period,
    /// the driver will automatically switch back to the internal clock generator.
    ///
    /// * `state` - enabled if true
    fn external_clock_enable(state: bool);

    /// Check if the motor is enabled
    fn is_enabled() -> bool;

    /// Push the current command stack to the driver
    fn push();

    /// Whether to enable analog scaling of the motor current
    ///
    /// * `state` - enabled if true
    fn analog_scaling_enable(state: bool);

    /// Check if analog current scaling is enabled
    fn is_analog_scaling_enabled() -> bool;

    /// Whether to use the stepper driver's internal sense resistor
    ///
    /// * `state` - enabled if true
    fn sense_resistor_enable(state: bool);

    /// Check if the internal sense resistor is used
    fn is_sense_resistor_enabled() -> bool;

    /// Whether to enable stealthChop PWM mode
    ///
    /// * `state` - enabled if true
    fn stealthchop_pwm_mode_enable(state: bool);

    // todo: unknown meaning, dig into it. Some kind of bitmask
    fn enc_commutation(state: bool);

    /// Invert the motor direction
    ///
    /// * `state` - inverted if true
    fn shaft(state: bool);

    /// Check if the DIAG signal is in the error state.
    ///
    /// A motor stall or sudden change in velocity can trigger a state in which the motor cannot
    /// recover. In this case, the error state here will be true.
    fn has_diag_error() -> bool;

    /// Resets the error state for the DIAG signal, clearing any previous errors.
    ///
    /// It is advised after calling this function, the motor is restarted, and
    /// zero velocity is assumed for a safe recovery.
    fn diag_error_reset();

    /// Check the over-temperature pre-warning (OTPW) state of the driver.
    ///
    /// The driver shall report OTPW when the MOSFET of the stepper driver is experiencing high
    /// temperatures. If not resolved, the driver will enter a thermal shutdown.
    fn has_overtemp_prewarning() -> bool;

    /// Configure the motor off time.
    ///
    /// The off time setting is a duration of the slow decay phase `CLK = 24 + 32*TOFF`.
    /// %0000 - Driver disable, all bridges off
    /// %0001 - Use only with TBL >= 2
    /// %0010...%1111 - 2-15
    ///
    /// For more information, please see the Trinamic datasheet for your given driver.
    fn toff(off_time: u8);
}
