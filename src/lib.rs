//! Share the types and modules defined below across the crate.
#![no_std]
#![no_main]


mod button;
pub mod error;
mod led;
mod never;
pub mod shared_const;

pub use button::{Button, PressDuration};
pub use led::{Led, LedMode};
pub use never::Never;