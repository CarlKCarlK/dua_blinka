use derive_more::Display;
use embassy_time::Duration;

use crate::shared_const::LONG_PRESS_DURATION;

// Instead of having API describing a short vs a long button-press vaguely using a `bool`, we define
// an `enum` to clarify what each state represents.  The compiler will compile this down to the
// very same `boolean` that we would have coded by hand.
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PressDuration {
    #[default]
    Short,
    Long,
}

// Make `PressDuration` solely responsible for the distinction in `Duration` between a short and long
// button press.
impl From<Duration> for PressDuration {
    fn from(duration: Duration) -> Self {
        match duration >= LONG_PRESS_DURATION {
            true => PressDuration::Long,
            false => PressDuration::Short,
        }
    }
}
