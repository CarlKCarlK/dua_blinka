#![allow(dead_code, reason = "cmk")]
#![no_std]
#![no_main]
#![allow(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]

// cmk is cargo embassy now published?

mod signal;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use lib::{Button, Led, LedMode, Never};
use panic_probe as _;

use lib::error::Result;
use signal::SIGNAL;

// In bare-metal development, your application is launched by the processor's boot loader (from ROM).
// The boot loader typically jumps (doesn't make a function call) to your application's entry point.
// This is because there's nothing more for the boot loader to do.  By jumping instead of making a
// function call, the boot loader ensures there's nothing on the stack for your program to return to.
#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err(); // cmk what is unwrap_err?
                                                      // cmk get debug print and panics showing up in the console
    panic!("{err}");
}

// This application defines `inner_main` because some of the app initialization is fallible and, as
// per the above comment, the entry point must never return.
// Rust's `!` is also not yet stable for use as anything other than a naked function return type.
// That is why `inner_main()` uses a locally-defined "never" type called `Never` which serves
// exactly the same purpose as `!`, inside a `Result`.
async fn inner_main(spawner: Spawner) -> Result<Never> {
    // Receive a definition of all the peripherals inside the `RP2040` processor.
    let peripherals: embassy_rp::Peripherals =
        embassy_rp::init(embassy_rp::config::Config::default()); // cmk understand default::default vs this

    // We have the LED wired to GPIO pin 2.  `degrade()` converts the `PIN_2` type (too specific for
    // the `Led` type we're about to construct) to an `AnyPin` type with a value of 2.  This allows
    // us to avoid hard-coding the GPIO pin number inside the `Led` type.
    let led_pin = peripherals.PIN_2.degrade();
    // Construct the `Led` type.  `led_pin` is explained above.  `spawner` is the type which knows
    // how to create new tasks on the `embassy_executor` async runtime (analogous to spawning a new
    // thread in an OS).  Lastly, `SIGNAL` is a "hotline" allowing `Led` to communicate with other
    // contexts (in our scenario, `task`s).
    let mut led = Led::new(led_pin, spawner, &SIGNAL)?;
    // cmk understand how we can give away spawner under the ownership rules. Also, what more can we do with spawner?
    // cmk understand SIGNAL

    let button_pin = peripherals.PIN_13.degrade();
    let mut button = Button::new(button_pin);

    // Even though we are `loop`ing forever, the loop will spend most of its time paused, waiting
    // for the user to press a button.  This saves huge amounts of power over "busy-waiting" and
    // makes an embedded device energy-efficient (suitable to be battery-powered, for example).
    // cmk could I do my state machine stuff here?
    loop {
        // Wait for the user to press a button.  The `button` type will classify the button-press
        // as either "short" or "long".
        let press = button.wait_for_press().await;

        use lib::PressDuration as Pd;
        match press {
            // Long press: reset `Led` to its default mode.
            Pd::Long => led.set_mode(LedMode::default()),
            // Short press: advance `Led` to its next mode, wrapping back to the first mode if
            // `Led` is currently in its last-defined mode.
            Pd::Short => led.advance_mode(),
        };
    }
}
