use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex},
    signal::Signal
};
use lib::LedMode;

// Although defined as a `static` (a form of global variable):
// i) scope is restricted to prevent use as a  global.  This way, `SIGNAL` must be injected like any
//    other variable, providing loose-coupling and improved testability.
// `SIGNAL` is synchronized (does not violate Rust's "shared XOR mutable" borrow-checker rule)
// *but* as a global, does not support re-entrancy (e.g. re-entrant testing)
pub(super) static SIGNAL: Signal<CriticalSectionRawMutex, LedMode> = Signal::new();