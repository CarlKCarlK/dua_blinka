//! Share the types and modules defined below across the crate.
#![no_std]
#![no_main]

mod button;
mod error;
mod hardware;
mod led;
mod led_state;
mod never;
mod press_duration;
mod schedule;
pub mod shared_const;

pub use button::{Button, PressDuration};
pub use error::Result;
pub use hardware::Hardware;
pub use led::{Led, LedNotifier};
pub use led_state::LedState;
pub use never::Never;
pub use schedule::Schedule;
