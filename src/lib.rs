//! Share the types and modules defined below across the crate.
#![warn(
    clippy::pedantic,
    clippy::nursery,
    //  clippy::use_self,
    //  unused_lifetimes,
    // missing_docs,
    //  single_use_lifetimes,
    //  unreachable_pub,
    // // TODO: clippy::cargo,
    // clippy::perf,
    // clippy::style,
    // clippy::complexity,
    // clippy::correctness,
    // clippy::must_use_candidate,
    // // TODO: clippy::cargo_common_metadata
    // clippy::unwrap_used, clippy::unwrap_used, // Warns if you're using .unwrap() or .expect(), which can be a sign of inadequate error handling.
    // clippy::panic_in_result_fn, // Ensures functions that return Result do not contain panic!, which could be inappropriate in production code.
)]
#![no_std]
#![no_main]

mod button;
pub mod error;
mod hardware;
mod led;
mod led_state;
mod never;
mod press_duration;
mod schedule;
pub mod shared_const;

pub use button::Button;
pub use hardware::Hardware;
pub use led::{Led, LedNotifier};
pub use led_state::LedState;
pub use never::Never;
pub use press_duration::PressDuration;
pub use schedule::Schedule;
