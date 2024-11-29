use crate::{
    error::{Error, Result},
    shared_const::{
        FAST_FLASH_DELAY, MORSE_DASH_MILLIS, MORSE_O_MILLIS, MORSE_S_MILLIS, ONE_DAY,
        SCHEDULE_CAPACITY, SLOW_FLASH_DELAY, ZERO_DELAY,
    },
};
use embassy_time::Duration;
use heapless::Vec;

/// Represents a schedule for controlling an LED's on and off states.
///
/// The schedule consists of an initial delay followed by a
/// cycling pattern of durations specifying the LED's on and off states.
///
/// The pattern must have an even number of elements.
#[derive(Debug, Default)]
pub struct Schedule {
    /// The time the LED remains off before starting its on/off cycle.
    pub initial_delay: Duration,
    /// A vector of cyclic durations that alternate the LED's state.
    pub pattern: Vec<Duration, SCHEDULE_CAPACITY>,
}

impl Schedule {
    /// Creates a new `Schedule` instance.
    ///
    /// # Arguments
    ///
    /// - `initial_delay`: The time the LED remains off before starting its on/off cycle.
    /// - `pattern`: A vector of cyclic durations that alternate the LED's state. It must have an even number of elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern length is not even.
    pub fn new(initial_delay: Duration, pattern: Vec<Duration, SCHEDULE_CAPACITY>) -> Result<Self> {
        if pattern.len() & 1 != 0 {
            // detect odd length
            return Err(Error::ScheduleCycleLengthMustBeEven);
        }

        Ok(Self {
            initial_delay,
            pattern,
        })
    }

    /// Creates a new `Schedule` from an initial delay and a slice of durations.
    ///
    /// # Arguments
    ///
    /// - `initial_delay`: The time the LED remains off before starting its on/off cycle.
    /// - `slice`: A slice of cyclic durations that alternate the LED's state. It must have an even number of elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice length is not even or if the slice exceeds the capacity of the vector.
    /// ```
    pub fn from_slice(initial_delay: Duration, slice: &[Duration]) -> Result<Self> {
        let pattern = Vec::from_slice(slice).map_err(|()| Error::ScheduleCapacityExceeded)?;
        Self::new(initial_delay, pattern)
    }

    /// Creates a schedule with a fast flashing pattern with no initial delay.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn fast_no_delay() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[FAST_FLASH_DELAY, FAST_FLASH_DELAY])
    }

    /// Creates a schedule with a fast flashing pattern after a short initial delay.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn fast_with_delay() -> Result<Self> {
        Self::from_slice(FAST_FLASH_DELAY, &[FAST_FLASH_DELAY, FAST_FLASH_DELAY])
    }

    /// Creates a schedule with a slow flashing pattern with no initial delay.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn slow_no_delay() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[SLOW_FLASH_DELAY, SLOW_FLASH_DELAY])
    }

    /// Creates a schedule with a slow flashing pattern after a short initial delay.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn slow_even() -> Result<Self> {
        Self::from_slice(SLOW_FLASH_DELAY, &[SLOW_FLASH_DELAY, SLOW_FLASH_DELAY])
    }

    /// Creates a schedule with the LED always on.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn on() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[ONE_DAY, ZERO_DELAY])
    }

    /// Creates a schedule with the LED always off.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn off() -> Result<Self> {
        Ok(Self::default())
    }

    /// Creates a schedule for the "SOS" Morse code pattern.
    fn sos(dot_delay: u64, dot_after: u64, millis_per_dot: u64) -> Result<Self> {
        let mut sos = Vec::default();
        sos.extend_from_slice(&MORSE_S_MILLIS).map_err(|()| Error::ScheduleCapacityExceeded)?;
        sos.push(MORSE_DASH_MILLIS).map_err(|_| Error::ScheduleCapacityExceeded)?;
        sos.extend_from_slice(&MORSE_O_MILLIS).map_err(|()| Error::ScheduleCapacityExceeded)?;
        sos.push(MORSE_DASH_MILLIS).map_err(|_| Error::ScheduleCapacityExceeded)?;
        sos.extend_from_slice(&MORSE_S_MILLIS).map_err(|()| Error::ScheduleCapacityExceeded)?;
        sos.push(Duration::from_millis(dot_after)).map_err(|_| Error::ScheduleCapacityExceeded)?;

        // Adjust each duration by multiplying with millis_per_dot, checking for overflow
        for duration in &mut sos {
            *duration = Duration::from_ticks(
                duration.as_ticks().checked_mul(millis_per_dot).ok_or(Error::ArithmeticOverflow)?,
            );
        }

        // Calculate the initial delay, checking for overflow
        let initial_delay = Duration::from_ticks(
            dot_delay.checked_mul(millis_per_dot).ok_or(Error::ArithmeticOverflow)?,
        );

        Self::new(initial_delay, sos)
    }

    /// Creates a schedule for the "SOS" with each dot at 120 milliseconds.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn sos_slow() -> Result<Self> {
        Self::sos(5, 50, 120)
    }

    /// Creates a schedule for the "SOS" with each dot at 60 milliseconds and a long initial delay.
    #[expect(clippy::missing_errors_doc, reason = "These inputs avoid errors.")]
    pub fn sos_fast() -> Result<Self> {
        Self::sos(100, 10, 60)
    }
}
