use crate::{button::Button, button::PressDuration, error::Result, led::Led, Schedule};

/// Represents the different states an LED can operate in.
///
/// For example, an `Led` in `Sos` state sends the Morse code distress signal.
#[allow(missing_docs)] // We don't need to document the variants of this enum.
#[derive(Debug, defmt::Format, Default)]
pub enum LedState {
    #[default]
    FastAlternate,
    FastTogether,
    SlowAlternate,
    Sos,
    AlwaysOn,
    AlwaysOff,
}

impl LedState {
    /// Runs the current LED state and returns the next state.
    ///
    /// # Errors
    ///
    /// This function will return an error if scheduling the LED state fails.
    pub async fn run_and_next(
        self,
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        match self {
            Self::FastAlternate => Self::run_and_next_fast_alternate(led0, led1, button).await,
            Self::FastTogether => Self::run_and_next_fast_together(led0, led1, button).await,
            Self::SlowAlternate => Self::run_and_next_slow_alternate(led0, led1, button).await,
            Self::Sos => Self::run_and_next_sos(led0, led1, button).await,
            Self::AlwaysOn => Self::run_and_next_always_on(led0, led1, button).await,
            Self::AlwaysOff => Self::run_and_next_always_off(led0, led1, button).await,
        }
    }

    async fn run_and_next_fast_alternate(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::fast_even()?);
        led1.schedule(Schedule::fast_odd()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastTogether),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    async fn run_and_next_fast_together(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::fast_even()?);
        led1.schedule(Schedule::fast_even()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::SlowAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    async fn run_and_next_slow_alternate(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::slow_even()?);
        led1.schedule(Schedule::slow_odd()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::AlwaysOn),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    async fn run_and_next_sos(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::sos0()?);
        led1.schedule(Schedule::sos1()?); // cmk switch to sos1
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    async fn run_and_next_always_on(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::on()?);
        led1.schedule(Schedule::on()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::AlwaysOff),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    async fn run_and_next_always_off(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::default());
        led1.schedule(Schedule::default());
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }
}
