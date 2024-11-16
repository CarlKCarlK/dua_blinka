use crate::shared_const::Schedule;
use defmt::info;
use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;

/// Type representing the physical LED and its "display" mode.
pub struct Led {
    sender: &'static Signal<CriticalSectionRawMutex, Schedule>,
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
        signal: &'static Signal<CriticalSectionRawMutex, Schedule>,
        schedule: Schedule,
    ) -> Result<Self, SpawnError> {
        let led = Self { sender: signal };
        spawner.spawn(led_driver(pin, signal, schedule))?;
        Ok(led)
    }

    /// Send a new schedule to the `led_driver` task.
    pub fn schedule(&mut self, schedule: Schedule) {
        self.sender.signal(schedule);
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
#[expect(
    clippy::indexing_slicing,
    reason = "Safe because `index` is always less than `schedule.len()"
)]
#[expect(clippy::len_zero, reason = "This is always safe.")]
#[expect(clippy::arithmetic_side_effects, reason = "schedule.len() is not zero.")]
async fn led_driver(
    pin: AnyPin,
    receiver: &'static Signal<CriticalSectionRawMutex, Schedule>,
    mut schedule: Schedule,
) -> ! {
    // Define `led_pin` as an `Output` pin (meaning the microcontroller will supply 3.3V when its
    // value is set to `Level::High`.
    let mut led_pin = Output::new(pin, Level::Low);
    // Drive the LED's behavior forever.
    let mut index = 0;
    loop {
        info!("led_driver: index: {}", index);
        // if schedule is empty or 1 item, turn off the LED and wait for a new schedule
        if schedule.len() <= 1 {
            led_pin.set_low();
            schedule = receiver.wait().await;
            continue;
        }
        debug_assert!(index < schedule.len() && 0 < schedule.len(), "real assert");
        led_pin.set_level((index % 2 == 1).into());
        if let Either::Second(new_schedule) =
            select(Timer::after(schedule[index]), receiver.wait()).await
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
