use core::ops::Index;

use crate::error::Result;
use embassy_time::Duration;

pub struct Schedule(heapless::Vec<Duration, 20>); // cmk const

impl Default for Schedule {
    fn default() -> Self {
        Self(heapless::Vec::new())
    }
}

impl Schedule {
    /// cmk
    ///
    /// # Errors
    pub fn from_slice(slice: &[Duration]) -> Result<Self> {
        Ok(Self(
            heapless::Vec::from_slice(slice)
                .map_err(|()| crate::error::Error::ScheduleCapacityExceeded)?,
        ))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // TODO: We could instead create these once statically.
    /// cmk
    ///
    /// # Errors
    pub fn fast_even() -> Result<Self> {
        Self::from_slice(&[200, 200, 200].map(Duration::from_millis)) // cmk const
    }

    /// cmk
    ///
    /// # Errors
    pub fn fast_odd() -> Result<Self> {
        Self::from_slice(&[Duration::MIN, Duration::from_millis(200), Duration::from_millis(200)])
    }

    /// cmk
    ///
    /// # Errors
    pub fn slow_even() -> Result<Self> {
        Self::from_slice(&[500, 500, 500].map(Duration::from_millis))
    }

    /// cmk
    ///
    /// # Errors
    pub fn slow_odd() -> Result<Self> {
        Self::from_slice(&[Duration::MIN, Duration::from_millis(500), Duration::from_millis(500)])
    }

    #[must_use]
    pub fn off() -> Self {
        Self::default()
    }

    /// cmk
    ///
    /// # Errors
    pub fn on() -> Result<Self> {
        Self::from_slice(&[Duration::MIN, Duration::from_secs(60 * 60 * 24)])
    }

    /// cmk
    ///
    /// # Errors
    #[expect(clippy::arithmetic_side_effects, reason = "multiplication is in bounds")]
    pub fn sos_even() -> Result<Self> {
        Self::from_slice(
            &[1, 1, 1, 1, 1, 1, 1, 3, 2, 3, 2, 3, 2, 1, 1, 1, 1, 1, 50]
                .map(|x| Duration::from_millis(x * 120)),
        )
    }

    /// cmk
    ///
    /// # Errors
    #[expect(clippy::arithmetic_side_effects, reason = "multiplication is in bounds")]
    pub fn sos_odd() -> Result<Self> {
        Self::from_slice(
            &[50, 1, 1, 1, 1, 1, 1, 3, 2, 3, 2, 3, 2, 1, 1, 1, 1, 1, 10]
                .map(|x| Duration::from_millis(x * 60)),
        )
    }
}

impl Index<usize> for Schedule {
    type Output = Duration;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
