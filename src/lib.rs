//! Share the types and modules defined below across the crate.
#![no_std]
#![no_main]

mod button;
pub mod error;
mod led;
mod never;
mod press_duration;
pub mod shared_const;

pub use button::Button;
pub use led::{Led, LedNotifier};
pub use never::Never;
pub use press_duration::PressDuration;
