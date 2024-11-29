use crate::{
    error::Result,
    shared_const::{
        FAST_FLASH_DELAY, MORSE_O_MILLIS, MORSE_S_MILLIS, ONE_DAY, SLOW_FLASH_DELAY, THREE_MILLIS,
        ZERO_DELAY,
    },
};
use embassy_time::Duration;
use heapless::Vec;

/// The schedule consists of an initial delay followed by a cycle of durations specifying the LED's on and off states.
///
/// - **Initial Delay:** The `initial_delay` field represents the time the LED remains off before starting its on/off cycle.
/// - **Cycle Durations:** The `cycle` vector contains durations that alternate the LED's state. It must have
///   an even number of elements. If the number of elements is zero, the LED remains off after the initial delay.
///
/// # Example
///
/// This schedule introduces a 1-second initial delay, then alternates the LED on for 500 milliseconds and off for 300 milliseconds:
///
/// ```rust
/// use core::time::Duration;
/// use heapless::Vec;
///
/// let schedule = Schedule::new(Duration::from_secs(1),
///    &[Duration::from_millis(500),
///      Duration::from_millis(300)]);
/// ```
#[derive(Debug, Default)]
pub struct Schedule {
    /// The time the LED remains off before starting its on/off cycle.
    pub initial_delay: Duration,
    /// A vector of durations that alternate the LED's state.
    pub pattern: Vec<Duration, 20>, // const
}

impl Schedule {
    pub fn new(initial_delay: Duration, pattern: Vec<Duration, 20>) -> Result<Self> {
        if pattern.len() % 2 != 0 {
            return Err(crate::error::Error::ScheduleCycleLengthMustBeEven);
        };

        Ok(Self {
            initial_delay,
            pattern,
        })
    }

    /// Creates a new `Schedule` instance.
    ///
    /// # Arguments
    ///
    /// * `initial_delay` - The time the LED remains off before starting its on/off cycle.
    /// * `slice` - A slice of durations that alternate the LED's state. It must have an even number of elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice length is not even or if the slice capacity is exceeded.
    ///
    /// # Example
    ///
    /// This schedule introduces a 1-second initial delay, then alternates the LED on for 500 milliseconds and off for 300 milliseconds:
    ///
    /// ```rust,ignore
    /// use core::time::Duration;
    /// use heapless::Vec;
    ///
    /// let schedule = Schedule::from_slice(Duration::from_secs(1),
    ///   &[Duration::from_millis(500),
    ///    Duration::from_millis(300)]);
    /// ```
    pub fn from_slice(initial_delay: Duration, slice: &[Duration]) -> Result<Self> {
        let pattern =
            Vec::from_slice(slice).map_err(|()| crate::error::Error::ScheduleCapacityExceeded)?;
        Self::new(initial_delay, pattern)
    }

    // TODO: We could instead create these once statically.

    /// cmk
    ///
    /// # Errors
    pub fn fast_even() -> Result<Self> {
        Self::from_slice(FAST_FLASH_DELAY, &[FAST_FLASH_DELAY, FAST_FLASH_DELAY])
    }

    /// cmk
    ///
    /// # Errors
    pub fn fast_odd() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[FAST_FLASH_DELAY, FAST_FLASH_DELAY])
    }

    /// cmk
    ///
    /// # Errors
    pub fn slow_even() -> Result<Self> {
        Self::from_slice(SLOW_FLASH_DELAY, &[SLOW_FLASH_DELAY, SLOW_FLASH_DELAY])
    }

    /// cmk
    ///
    /// # Errors
    pub fn slow_odd() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[SLOW_FLASH_DELAY, SLOW_FLASH_DELAY])
    }

    /// cmk
    ///
    /// # Errors
    pub fn on() -> Result<Self> {
        Self::from_slice(ZERO_DELAY, &[ONE_DAY, ZERO_DELAY])
        // cmk const
    }

    fn sos(dot_delay: u64, dot_after: u64, millis_per_dot: u32) -> Result<Self> {
        let mut sos = Vec::default();
        sos.extend_from_slice(&MORSE_S_MILLIS)
            .map_err(|()| crate::error::Error::ScheduleCapacityExceeded)?;
        sos.push(THREE_MILLIS).map_err(|_| crate::error::Error::ScheduleCapacityExceeded)?;
        sos.extend_from_slice(&MORSE_O_MILLIS)
            .map_err(|()| crate::error::Error::ScheduleCapacityExceeded)?;
        sos.push(THREE_MILLIS).map_err(|_| crate::error::Error::ScheduleCapacityExceeded)?;
        sos.extend_from_slice(&MORSE_S_MILLIS)
            .map_err(|()| crate::error::Error::ScheduleCapacityExceeded)?;
        sos.push(Duration::from_millis(dot_after))
            .map_err(|_| crate::error::Error::ScheduleCapacityExceeded)?;

        sos.iter_mut().for_each(|x| *x *= millis_per_dot);
        Self::new(Duration::from_millis(dot_delay * u64::from(millis_per_dot)), sos)
    }

    /// cmk
    ///
    /// # Errors
    pub fn sos0() -> Result<Self> {
        Self::sos(5, 50, 120)
    }

    /// cmk
    ///
    /// # Errors
    pub fn sos1() -> Result<Self> {
        Self::sos(100, 10, 60)
    }
}
