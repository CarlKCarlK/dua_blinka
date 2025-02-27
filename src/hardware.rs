use embassy_rp::{
    gpio::{self, Level},
    peripherals::CORE1,
    Peripherals,
};

/// Represents the hardware components of the clock.
pub struct Hardware<'a> {
    /// An LED
    pub led0: gpio::Output<'a>,
    /// Another LED
    pub led1: gpio::Output<'a>,
    /// The button that controls the clock.
    pub button: gpio::Input<'a>,
    /// The second core of the RP2040 (not currently used).
    pub core1: CORE1,
}

impl Default for Hardware<'_> {
    fn default() -> Self {
        let peripherals: Peripherals = embassy_rp::init(embassy_rp::config::Config::default());

        let led0 = gpio::Output::new(peripherals.PIN_2, Level::Low);
        let led1 = gpio::Output::new(peripherals.PIN_3, Level::Low);
        let button = gpio::Input::new(peripherals.PIN_13, gpio::Pull::Down);
        let core1 = peripherals.CORE1;

        Self {
            led0,
            led1,
            button,
            core1,
        }
    }
}
