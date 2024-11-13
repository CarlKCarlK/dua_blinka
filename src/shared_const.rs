//! Constants defined for use throughout the program

use embassy_time::Duration;

pub const BUTTON_DEBOUNCE_DELAY: Duration = Duration::from_millis(10);
pub const LONG_PRESS_DURATION: Duration = Duration::from_millis(500);
pub const FAST_FLASH_DELAY: Duration = Duration::from_millis(250);
pub const SLOW_FLASH_DELAY: Duration = Duration::from_millis(750);