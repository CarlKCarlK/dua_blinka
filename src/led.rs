use defmt::info;
use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::Output;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;

use crate::Schedule;

// cmk will need to rename this project from blinky_carlk2

/// Type representing the physical LED and its "display" mode.
pub struct Led<'a> {
    notifier: &'a LedNotifier,
}
#[allow(
    clippy::module_name_repetitions,
    reason = "We use the prefix because other structs may need their own notifier type."
)]
pub type LedNotifier = Signal<CriticalSectionRawMutex, Schedule>;

impl Led<'_> {
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
        let led = Self { notifier };
        spawner.spawn(device_loop(pin, notifier))?;
        Ok(led)
    }

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
    let mut index = 0;
    loop {
        info!("led_driver: index: {}", index);
        // if the schedule is empty or contains just one duration, turn off the LED and wait for a new schedule.
        if schedule.len() <= 1 {
            pin.set_low();
            schedule = notifier.wait().await;
            continue;
        }
        debug_assert!(index < schedule.len(), "real assert");
        pin.set_level((index % 2 == 1).into());
        if let Either::Second(new_schedule) =
            select(Timer::after(schedule[index]), notifier.wait()).await
        {
            info!("new schedule");
            schedule = new_schedule;
            index = 0;
            continue;
        }

        // increment index, wrapping around to 1 if we reach the end of the schedule
        index = (index % (schedule.len() - 1)) + 1;
        debug_assert!(0 < index && index < schedule.len(), "real assert");
    }
}
