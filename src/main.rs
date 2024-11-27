#![no_std]
#![no_main]
#![allow(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::Duration;
use lib::error::Result;
use lib::shared_const::Schedule;
use lib::{Button, Led, LedNotifier, Never, PressDuration};
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
async fn inner_main(spawner: Spawner) -> Result<Never> {
    // Receive a definition of all the peripherals inside the `RP2040` processor.
    let peripherals: embassy_rp::Peripherals =
        embassy_rp::init(embassy_rp::config::Config::default());

    // We have the LED wired to GPIO pin 2.  `degrade()` converts the `PIN_2` type (too specific for
    // the `Led` type we're about to construct) to an `AnyPin` type with a value of 2.  This allows
    // us to avoid hard-coding the GPIO pin number inside the `Led` type.
    let led_pin0 = peripherals.PIN_2.degrade();
    let led_pin1 = peripherals.PIN_3.degrade();

    // Construct the `Led` type.  `led_pin` is explained above.  `spawner` is the type which knows
    // how to create new tasks on the `embassy_executor` async runtime (analogous to spawning a new
    // thread in an OS).  Lastly, `SIGNAL` is a "hotline" allowing `Led` to communicate with other
    // contexts (in our scenario, `task`s).
    static LED_NOTIFIER0: LedNotifier = Led::notifier();
    let mut led0 = Led::new(led_pin0, spawner, &LED_NOTIFIER0, slow_even()?)?;
    static LED_NOTIFIER1: LedNotifier = Led::notifier();
    let mut led1 = Led::new(led_pin1, spawner, &LED_NOTIFIER1, slow_odd()?)?;

    let button_pin = peripherals.PIN_13.degrade();

    // Even though we are `loop`ing forever, the loop will spend most of its time paused, waiting
    // for the user to press a button.  This saves huge amounts of power over "busy-waiting" and
    let mut button = Button::new(button_pin);
    let mut state = State::First;
    loop {
        defmt::info!("State: {:?}", state);
        state = match state {
            State::First => State::FastAlternate,
            State::FastAlternate => fast_alternate_state(&mut button, &mut led0, &mut led1).await?,
            State::FastTogether => fast_together_state(&mut button, &mut led0, &mut led1).await?,
            State::SlowAlternate => slow_alternate_state(&mut button, &mut led0, &mut led1).await?,
            State::Sos => sos_state(&mut button, &mut led0, &mut led1).await?,
            State::AlwaysOn => always_on_state(&mut button, &mut led0, &mut led1).await?,
            State::AlwaysOff => always_off_state(&mut button, &mut led0, &mut led1).await?,
            State::Last => State::First,
        };
    }
}

#[derive(Debug, defmt::Format)]
enum State {
    First,
    FastAlternate,
    FastTogether,
    SlowAlternate,
    Sos,
    AlwaysOn,
    AlwaysOff,
    Last,
}

async fn fast_alternate_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(fast_even()?);
    led1.schedule(fast_odd()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::FastTogether),
        PressDuration::Long => Ok(State::Sos),
    }
}

async fn fast_together_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(fast_even()?);
    led1.schedule(fast_even()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::SlowAlternate),
        PressDuration::Long => Ok(State::Sos),
    }
}

async fn slow_alternate_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(slow_even()?);
    led1.schedule(slow_odd()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::AlwaysOn),
        PressDuration::Long => Ok(State::Sos),
    }
}

async fn sos_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(sos_even()?);
    led1.schedule(sos_odd()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::First),
        PressDuration::Long => Ok(State::Sos),
    }
}

async fn always_on_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(on()?);
    led1.schedule(on()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::AlwaysOff),
        PressDuration::Long => Ok(State::Sos),
    }
}

async fn always_off_state(
    button: &mut Button<'_>,
    led0: &mut Led<'_>,
    led1: &mut Led<'_>,
) -> Result<State> {
    led0.schedule(off()?);
    led1.schedule(off()?);
    match button.press_duration().await {
        PressDuration::Short => Ok(State::Last),
        PressDuration::Long => Ok(State::Sos),
    }
}

// TODO: We could instead create these once statically.
fn fast_even() -> Result<Schedule> {
    Ok(Schedule::from_slice(&[200, 200, 200].map(Duration::from_millis))?)
}

fn fast_odd() -> Result<Schedule> {
    Ok(Schedule::from_slice(&[
        Duration::MIN,
        Duration::from_millis(200),
        Duration::from_millis(200),
    ])?)
}

fn slow_even() -> Result<Schedule> {
    Ok(Schedule::from_slice(&[500, 500, 500].map(Duration::from_millis))?)
}

fn slow_odd() -> Result<Schedule> {
    Ok(Schedule::from_slice(&[
        Duration::MIN,
        Duration::from_millis(500),
        Duration::from_millis(500),
    ])?)
}

const fn off() -> Result<Schedule> {
    Ok(Schedule::new())
}

fn on() -> Result<Schedule> {
    Ok(Schedule::from_slice(&[Duration::MIN, Duration::from_secs(60 * 60 * 24)])?)
}

#[expect(clippy::arithmetic_side_effects, reason = "multiplication is in bounds")]
fn sos_even() -> Result<Schedule> {
    Ok(Schedule::from_slice(
        &[1, 1, 1, 1, 1, 1, 1, 3, 2, 3, 2, 3, 2, 1, 1, 1, 1, 1, 50]
            .map(|x| Duration::from_millis(x * 120)),
    )?)
}

#[expect(clippy::arithmetic_side_effects, reason = "multiplication is in bounds")]
fn sos_odd() -> Result<Schedule> {
    Ok(Schedule::from_slice(
        &[50, 1, 1, 1, 1, 1, 1, 3, 2, 3, 2, 3, 2, 1, 1, 1, 1, 1, 10]
            .map(|x| Duration::from_millis(x * 60)),
    )?)
}
