//! A two-led set up that can be controlled by a button.
//!
//! Runs on a Raspberry Pi Pico RP2040. See the `README.md` for more information.
#![no_std]
#![no_main]
#![allow(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]

use defmt_rtt as _;
use embassy_executor::Spawner;
use lib::{Button, Led, LedNotifier, LedState, Never, Result};
use panic_probe as _;

// In bare-metal development, your application is launched by the processor's boot loader (from ROM).
// The boot loader typically jumps (doesn't make a function call) to your application's entry point.
// This is because there's nothing more for the boot loader to do.  By jumping instead of making a
// function call, the boot loader ensures there's nothing on the stack for your program to return to.
#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    // If it returns, something went wrong.
    let err = inner_main(spawner).await.unwrap_err();
    panic!("{err}");
}

// This application defines `inner_main` because some of the app initialization is fallible and, as
// per the above comment, the entry point must never return.
// Rust's `!` is also not yet stable for use as anything other than a naked function return type.
// That is why `inner_main()` uses a locally-defined "never" type called `Never` which serves
// exactly the same purpose as `!`, inside a `Result`.
#[expect(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]
#[expect(clippy::items_after_statements, reason = "Keeps related code together")]
async fn inner_main(spawner: Spawner) -> Result<Never> {
    // Initialize the hardware.
    let hardware: lib::Hardware<'_> = lib::Hardware::default();

    // Start virtual peripherals.
    static LED_NOTIFIER0: LedNotifier = Led::notifier();
    let mut led0 = Led::new(hardware.led0, &LED_NOTIFIER0, spawner)?;
    static LED_NOTIFIER1: LedNotifier = Led::notifier();
    let mut led1 = Led::new(hardware.led1, &LED_NOTIFIER1, spawner)?;
    let mut button = Button::new(hardware.button);

    // Run the state machine.
    let mut state = LedState::default();
    loop {
        defmt::info!("State: {:?}", state);
        state = state.run_and_next(&mut led0, &mut led1, &mut button).await?;
    }
}

// TODO: at least do enough CI to compile
