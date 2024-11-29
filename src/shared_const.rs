//! Constants defined for use throughout the program

use embassy_time::Duration;

/// Debounce delay for button inputs.
pub const BUTTON_DEBOUNCE_DELAY: Duration = Duration::from_millis(10);

/// Duration to recognize a long button press.
pub const LONG_PRESS_DURATION: Duration = Duration::from_millis(500);

/// Delay between flashes for fast blinking.
pub const FAST_FLASH_DELAY: Duration = Duration::from_millis(250);

/// Delay between flashes for slow blinking.
pub const SLOW_FLASH_DELAY: Duration = Duration::from_millis(750);

/// Zero duration, representing no delay.
pub const ZERO_DELAY: Duration = Duration::from_millis(0);

/// Duration representing one day.
pub const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

/// Duration of one millisecond.
pub const ONE_MILLIS: Duration = Duration::from_millis(1);

/// Duration of three milliseconds.
pub const THREE_MILLIS: Duration = Duration::from_millis(3);

/// Morse code representation for 'S' with interleaved delays.
pub const MORSE_S_MILLIS: [Duration; 5] = pad([ONE_MILLIS, ONE_MILLIS, ONE_MILLIS], ONE_MILLIS);

/// Morse code representation for 'O' with interleaved delays.
pub const MORSE_O_MILLIS: [Duration; 5] =
    pad([THREE_MILLIS, THREE_MILLIS, THREE_MILLIS], ONE_MILLIS);

/// Pads an array of durations with a specified padding duration.
///
/// # Parameters
///
/// - `input`: Array of durations to be padded.
/// - `padding`: Duration to interleave between input durations.
///
/// # Returns
///
/// An array where each input duration is followed by the padding duration.
const fn pad<const N: usize, const M: usize>(
    input: [Duration; N],
    padding: Duration,
) -> [Duration; M] {
    let mut result = [Duration::MIN; M];
    let mut i = 0;
    while i < N {
        result[i * 2] = input[i];
        if i * 2 + 1 < M {
            result[i * 2 + 1] = padding;
        }
        i += 1;
    }
    result
}
