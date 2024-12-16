use defmt::info;
use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::Output;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;

use crate::Schedule;

/// Type representing the physical LED and its "display" mode.
pub struct Led<'a> {
    notifier: &'a LedNotifier,
}
/// Type alias for notifier that sends messages an `Led`.
#[expect(
    clippy::module_name_repetitions,
    reason = "We use the prefix because other structs may need their own notifier type."
)]
pub type LedNotifier = Signal<CriticalSectionRawMutex, Schedule>;

impl<'a> Led<'a> {
    /// Create a new `Led`, which entails starting an Embassy task.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin that controls the `Led`.
    /// * `notifier` - The static notifier that sends messages to the `Led`.
    ///          This notifier is created with the `Led::notifier()` method.
    /// * `spawner` - The spawner that will spawn the task that controls the `Led`.
    ///
    /// # Errors
    ///
    /// Returns a `SpawnError` if the task cannot be spawned.
    pub fn new(
        pin: Output<'static>,
        notifier: &'static LedNotifier,
        spawner: Spawner,
    ) -> Result<Self, SpawnError> {
        spawner.spawn(device_loop(pin, notifier))?;
        Ok(Self { notifier })
    }

    /// Creates a new `LedNotifier` instance.
    ///
    /// This notifier is used to send messages to the `Led`.
    ///
    /// The  `LedNotifier` instance should be assigned to a static variable
    /// and passed to the `Led::new()` method.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[expect(clippy::items_after_statements, reason = "Keeps related code together and avoids name conflicts")]
    /// static CLOCK_NOTIFIER: ClockNotifier = Clock::notifier();
    /// let mut clock = Clock::new(hardware.cells, hardware.segments, &CLOCK_NOTIFIER, spawner)?;
    /// ```
    #[must_use]
    pub const fn notifier() -> LedNotifier {
        Signal::new()
    }

    /// Send a new schedule to the `led_driver` task.
    pub fn schedule(&mut self, schedule: Schedule) {
        self.notifier.signal(schedule);
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
async fn device_loop(mut pin: Output<'static>, notifier: &'static LedNotifier) -> ! {
    let mut schedule = Schedule::default();
    // Drive the LED's behavior forever.
    loop {
        // Keep the LED off the the initial delay.
        pin.set_low(); // Turn off the LED.
        if let Either::Second(new_schedule) =
            select(Timer::after(schedule.initial_delay), notifier.wait()).await
        {
            info!("new schedule");
            schedule = new_schedule;
            continue;
        }

        // If the schedule is empty, wait for a new schedule with the LED off.
        if schedule.on_off_durations.is_empty() {
            info!("new schedule");
            schedule = notifier.wait().await;
            continue;
        }

        // Cycle forever through the schedule, toggling the LED on and off.
        // until a new schedule is received.
        for duration in schedule.on_off_durations.iter().cycle() {
            pin.toggle();
            if let Either::Second(new_schedule) =
                select(Timer::after(*duration), notifier.wait()).await
            {
                info!("new schedule");
                schedule = new_schedule;
                break;
            }
        }
    }
}
