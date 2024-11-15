mod mode;

use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;
use enum_iterator::{cardinality, Sequence};

pub use mode::LedMode;

use crate::shared_const::{FAST_FLASH_DELAY, SLOW_FLASH_DELAY};

/// Type representing the physical LED and its "display" mode.
pub struct Led {
    mode: LedMode,
    sender: &'static Signal<CriticalSectionRawMutex, LedMode>,
}

impl Led {
    /// Constructor.  Inject:
    ///     * the GPIO pin where the LED is connected.
    ///     * `embassy_executor`'s task spawner, which enables creating new cooperative tasks,
    ///       running them on the async `Executor`.
    ///     * `Signal`, which is like a `Channel` or a "hotline" to communicate from one task to
    ///       another.  In this case, `Led` will use the `Signal` to tell the `led_driver` task
    ///       when to change operating modes.
    pub fn new(
        pin: AnyPin,
        spawner: Spawner,
        signal: &'static Signal<CriticalSectionRawMutex, LedMode>,
    ) -> Result<Self, SpawnError> {
        let led = Self {
            mode: LedMode::default(),
            sender: signal,
        };
        spawner.spawn(led_driver(pin, signal, led.mode))?;
        Ok(led)
    }

    /// Advances `state` from `Off` -> `FastFlash` -> `SlowFlash` -> `On` -> `Off` -> ..., returning
    /// the state `Led` was in prior to advancement.
    pub fn advance_mode(&mut self) -> LedMode {
        // cmk not async
        // Invariant: `LedState` `enum` must have at least 1 variant
        debug_assert!(cardinality::<LedMode>() > 0);

        let old_mode = self.mode;
        self.mode =
            old_mode.next().into_iter().chain(LedMode::first()).next().unwrap_or_else(|| {
                unreachable!("Internal error: Non-empty iterator failed to provide an element (!)")
            });
        self.sender.signal(self.mode);
        old_mode
    }

    /// Force the LED into the provided `LedMode`, returning the state `Led` was in prior to the
    /// `set_mode()` call.
    pub fn set_mode(&mut self, mode: LedMode) -> LedMode {
        // cmk not async
        let old_mode = self.mode;

        self.mode = mode;
        self.sender.signal(self.mode);

        old_mode
    }
}

/// Define an `embassy_executor::task` to control the behavior (flashing pattern) of the hardware
/// LED.  A `task` is a bit like an operating system (OS) thread, but differs in important ways.  A
/// `task`:
/// i) isn't controlled by an OS--there is no OS, remember since we are doing bare-metal development
/// ii) is co-operatively scheduled (not preemptively scheduled by an OS)
/// iii) must never "block", but "yield" instead (via the `await` keyword) or all `task`s will be
///      blocked (!)
/// iv) does not consume any computing cycles when "yield"ing.  Important for battery-powered and
///     limited-compute-capability devices.
#[embassy_executor::task]
async fn led_driver(
    pin: AnyPin,
    receiver: &'static Signal<CriticalSectionRawMutex, LedMode>,
    initial_mode: LedMode,
) -> ! {
    // Define `led_pin` as an `Output` pin (meaning the microcontroller will supply 3.3V when its
    // value is set to `Level::High`.
    let mut led_pin = Output::new(pin, Level::Low);
    let mut led_mode = initial_mode;
    // Drive the LED's behavior forever.
    loop {
        // Check the `Signal` (like a `Channel` or "hotline"; a `Signal`'s messages do not
        // accumulate in a queue or block the sender; instead, the `Signal` will provide only the
        // latest "message" sent to the receiver (if any).  New messages overwrite previously sent
        // unread messages.
        use LedMode as Lm;
        let res = match led_mode {
            // Flash the LED quickly and wait for the next message.
            Lm::FastFlash => {
                led_pin.toggle();
                select(Timer::after(FAST_FLASH_DELAY), receiver.wait()).await
            },
            // Flash the LED slowly and wait for the next message.
            Lm::SlowFlash => {
                led_pin.toggle();
                select(Timer::after(SLOW_FLASH_DELAY), receiver.wait()).await
            },
            // Leave the LED on continuously and wait for the next message.
            Lm::On => {
                led_pin.set_high();
                Either::Second(receiver.wait().await)
            },
            // Turn the LED off and wait for the next message.
            Lm::Off => {
                led_pin.set_low();
                Either::Second(receiver.wait().await)
            },
        };
        if let Either::Second(new_mode) = res {
            led_mode = new_mode
        }
    }
}
