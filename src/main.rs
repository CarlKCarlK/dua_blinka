#![allow(dead_code, reason = "cmk")]
#![no_std]
#![no_main]
#![allow(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]

// cmk is cargo embassy now published?

mod signal;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::Duration;
use lib::shared_const::Vec;
use lib::{Button, Led, Never, PressDuration};
use panic_probe as _;

use lib::error::Result;
use signal::SIGNAL0;
use signal::SIGNAL1;

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
    let led_pin0 = peripherals.PIN_2.degrade();
    let led_pin1 = peripherals.PIN_3.degrade();

    // Construct the `Led` type.  `led_pin` is explained above.  `spawner` is the type which knows
    // how to create new tasks on the `embassy_executor` async runtime (analogous to spawning a new
    // thread in an OS).  Lastly, `SIGNAL` is a "hotline" allowing `Led` to communicate with other
    // contexts (in our scenario, `task`s).
    let mut led0 = Led::new(led_pin0, spawner, &SIGNAL0, slow_even())?;
    let mut led1 = Led::new(led_pin1, spawner, &SIGNAL1, slow_odd())?;
    // cmk understand how we can give away spawner under the ownership rules. Also, what more can we do with spawner?
    // cmk understand SIGNAL

    let button_pin = peripherals.PIN_13.degrade();

    // Even though we are `loop`ing forever, the loop will spend most of its time paused, waiting
    // for the user to press a button.  This saves huge amounts of power over "busy-waiting" and
    // makes an embedded device energy-efficient (suitable to be battery-powered, for example).
    // cmk could I do my state machine stuff here?
    let mut button = Button::new(button_pin);
    let mut state = State::First;
    loop {
        defmt::info!("State: {:?}", state);
        state = match state {
            State::First => State::FastAlternating,
            State::FastAlternating => {
                fast_alternating_state(&mut button, &mut led0, &mut led1).await
            },
            State::FastTogether => fast_together_state(&mut button, &mut led0, &mut led1).await,
            State::SlowAlternating => {
                slow_alternating_state(&mut button, &mut led0, &mut led1).await
            },
            State::AlwaysOn => always_on_state(&mut button, &mut led0, &mut led1).await,
            State::AlwaysOff => always_off_state(&mut button, &mut led0, &mut led1).await,
            State::Last => State::First,
        };
    }
}

#[derive(Debug, defmt::Format)]
enum State {
    First,
    FastAlternating,
    FastTogether,
    SlowAlternating,
    AlwaysOn,
    AlwaysOff,
    Last,
}

async fn fast_alternating_state(button: &mut Button<'_>, led0: &mut Led, led1: &mut Led) -> State {
    led0.schedule(fast_even());
    led1.schedule(fast_odd());
    match button.wait_for_press().await {
        PressDuration::Short => State::FastTogether,
        PressDuration::Long => State::First,
    }
}

async fn fast_together_state(button: &mut Button<'_>, led0: &mut Led, led1: &mut Led) -> State {
    led0.schedule(fast_even());
    led1.schedule(fast_even());
    match button.wait_for_press().await {
        PressDuration::Short => State::SlowAlternating,
        PressDuration::Long => State::First,
    }
}

async fn slow_alternating_state(button: &mut Button<'_>, led0: &mut Led, led1: &mut Led) -> State {
    led0.schedule(slow_even());
    led1.schedule(slow_odd());
    match button.wait_for_press().await {
        PressDuration::Short => State::AlwaysOn,
        PressDuration::Long => State::First,
    }
}

async fn always_on_state(button: &mut Button<'_>, led0: &mut Led, led1: &mut Led) -> State {
    led0.schedule(on());
    led1.schedule(on());
    match button.wait_for_press().await {
        PressDuration::Short => State::AlwaysOff,
        PressDuration::Long => State::First,
    }
}

async fn always_off_state(button: &mut Button<'_>, led0: &mut Led, led1: &mut Led) -> State {
    led0.schedule(off());
    led1.schedule(off());
    match button.wait_for_press().await {
        PressDuration::Short => State::FastAlternating,
        PressDuration::Long => State::First,
    }
}

fn fast_even() -> Vec {
    Vec::from_slice(&[
        Duration::from_millis(200),
        Duration::from_millis(200),
        Duration::from_millis(200),
    ])
    .expect("Vec::from_slice failed")
}

fn fast_odd() -> Vec {
    Vec::from_slice(&[Duration::MIN, Duration::from_millis(200), Duration::from_millis(200)])
        .expect("Vec::from_slice failed")
}

fn slow_even() -> Vec {
    Vec::from_slice(&[
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
    ])
    .expect("Vec::from_slice failed")
}

fn slow_odd() -> Vec {
    Vec::from_slice(&[Duration::MIN, Duration::from_millis(500), Duration::from_millis(500)])
        .expect("Vec::from_slice failed")
}

const fn off() -> Vec {
    Vec::new()
}

fn on() -> Vec {
    Vec::from_slice(&[Duration::MIN, Duration::from_secs(60 * 60 * 24)])
        .expect("Vec::from_slice failed")
}
