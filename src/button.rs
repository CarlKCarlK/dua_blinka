#![allow(dead_code)]
mod press_duration;

use crate::shared_const::BUTTON_DEBOUNCE_DELAY;
use embassy_rp::gpio::{AnyPin, Input, Pull};
use embassy_time::{Instant, Timer};

pub use press_duration::PressDuration;

/// Type representing the hardware button and its state.
pub struct Button<'a> {
    pin: Input<'a>,
}

impl Button<'_> {
    /// Constructor.  Inject the GPIO pin where the hardware button can be found.
    pub fn new(pin: AnyPin) -> Self {
        Button {
            // Define the pin as an input with a "default" of `Level::Low`, or no voltage
            pin: Input::new(pin, Pull::Down),
        }
    }

    pub async fn wait_for_press(&mut self) -> PressDuration {
        // Wait for the voltage level on this pin to go high (button has been pressed)
        self.wait_for_high().await;

        // Start the "stopwatch" to determine how long the button was held down
        let start_of_press = Instant::now();

        // Sometimes the start (and end) of a press can be "noisy" (fluctuations between
        // "pressed" and "unpressed" states on the microsecond time scale as the physical
        // contactors changing from "not touching" through "almost touching" to "touching" (or
        // vice-versa)).  We're going to ignore the button's state during the noisy, fluctuating
        // "almost touching" state.  This is called "debouncing".
        self.debounce_delay().await;

        // The button is now fully depressed.  Wait for the button to be released, and then
        // we'll know how long the user pressed it for.
        self.wait_for_falling_edge().await;
        // Stop the "stopwatch" at the first sign of button release.
        let duration_of_press = start_of_press.elapsed();
        // Debounce the button's release.
        self.debounce_delay().await;

        //defmt::info!("Button press duration: {}ms", press_duration.as_millis());

        // Determine if the duration of the button press was a `Short` or `Long`
        duration_of_press.into()
    }

    /// Pause for a predetermined time to let the button's state become consistent.
    async fn debounce_delay(&mut self) -> &mut Self {
        Timer::after(BUTTON_DEBOUNCE_DELAY).await;
        self
    }

    /// Pause until voltage is present on the input pin.
    async fn wait_for_high(&mut self) -> &mut Self {
        self.pin.wait_for_high().await;
        self
    }

    /// Pause until voltage is absent on the input pin.
    async fn wait_for_low(&mut self) -> &mut Self {
        self.pin.wait_for_low().await;
        self
    }

    /// Pause until voltage on the input pin begins to go away.
    async fn wait_for_falling_edge(&mut self) -> &mut Self {
        self.pin.wait_for_falling_edge().await;
        self
    }

    /// Pause until voltage on the input pin begins to appear.
    async fn wait_for_rising_edge(&mut self) -> &mut Self {
        self.pin.wait_for_rising_edge().await;
        self
    }
}
