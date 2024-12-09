use crate::{
    button::{Button, PressDuration},
    error::Result,
    led::Led,
    Schedule,
};

/// Represents the different states the LEDs can operate in.
///
/// For example, an `Led` in `Sos` state sends the Morse code distress signal.
#[expect(missing_docs, reason = "We don't need to document the variants of this enum.")]
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

    #[inline]
    async fn run_and_next_fast_alternate(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::fast_with_delay()?);
        led1.schedule(Schedule::fast_no_delay()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastTogether),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    #[inline]
    async fn run_and_next_fast_together(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::fast_with_delay()?);
        led1.schedule(Schedule::fast_with_delay()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::SlowAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    #[inline]
    async fn run_and_next_slow_alternate(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::slow_even()?);
        led1.schedule(Schedule::slow_no_delay()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::AlwaysOn),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    #[inline]
    async fn run_and_next_sos(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::sos_slow()?);
        led1.schedule(Schedule::sos_fast()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }

    #[inline]
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

    #[inline]
    async fn run_and_next_always_off(
        led0: &mut Led<'_>,
        led1: &mut Led<'_>,
        button: &mut Button<'_>,
    ) -> Result<Self> {
        led0.schedule(Schedule::off()?);
        led1.schedule(Schedule::off()?);
        match button.press_duration().await {
            PressDuration::Short => Ok(Self::FastAlternate),
            PressDuration::Long => Ok(Self::Sos),
        }
    }
}
