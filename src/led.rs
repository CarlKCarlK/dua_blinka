use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};

/// Type representing the physical LED and its "display" mode.
pub struct Led {
    times: (Duration, Duration, Duration),
    sender: &'static Signal<CriticalSectionRawMutex, (Duration, Duration, Duration)>,
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
        signal: &'static Signal<CriticalSectionRawMutex, (Duration, Duration, Duration)>,
        times: (Duration, Duration, Duration),
    ) -> Result<Self, SpawnError> {
        let led = Self {
            times,
            sender: signal,
        };
        spawner.spawn(led_driver(pin, signal, led.times))?;
        Ok(led)
    }

    /// Force the LED into the provided `LedMode`, returning the state `Led` was in prior to the
    /// `set_mode()` call.
    pub fn set_mode(
        &mut self,
        times: (Duration, Duration, Duration),
    ) -> (Duration, Duration, Duration) {
        // cmk not async
        let old_times = self.times;

        self.times = times;
        self.sender.signal(self.times);

        old_times
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
#[embassy_executor::task(pool_size = 4)]
async fn led_driver(
    pin: AnyPin,
    // cmk instead of this could pass a vector (up to some fixed length) of times
    receiver: &'static Signal<CriticalSectionRawMutex, (Duration, Duration, Duration)>,
    initial_mode: (Duration, Duration, Duration),
) -> ! {
    // Define `led_pin` as an `Output` pin (meaning the microcontroller will supply 3.3V when its
    // value is set to `Level::High`.
    let mut led_pin = Output::new(pin, Level::Low);
    let mut led_mode = initial_mode;
    // Drive the LED's behavior forever.
    'outer: loop {
        // delay before starting the blinking
        led_pin.set_low(); // off
        let res = select(Timer::after(led_mode.0), receiver.wait()).await;
        if let Either::Second(new_mode) = res {
            led_mode = new_mode;
            continue 'outer;
        }

        loop {
            led_pin.set_high(); // on
            let res = select(Timer::after(led_mode.1), receiver.wait()).await;
            if let Either::Second(new_mode) = res {
                led_mode = new_mode;
                continue 'outer;
            }

            led_pin.set_low(); // off
            let res = select(Timer::after(led_mode.2), receiver.wait()).await;
            if let Either::Second(new_mode) = res {
                led_mode = new_mode;
                continue 'outer;
            }
        }
    }
}
