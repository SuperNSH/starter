//! The errors the app has raised and not yet dismissed. Anything calls
//! [`report_error`]; the shell's error surface shows the newest.

use nsh_core::element::Signal;
use nsh_core::prelude::*;

use crate::error::AppError;
use crate::store::event_bus;

/// The most queued errors; older ones drop as new ones arrive.
pub const ERROR_QUEUE_MAX: usize = 5;

/// The queued errors, oldest first.
#[derive(Clone, Default, PartialEq)]
pub struct ErrorQueue(pub Vec<AppError>);

/// The app's error queue. One per surface: `global` keeps one value per
/// type, which is why the type is this module's own.
fn queue() -> Signal<ErrorQueue> {
    global::<ErrorQueue>()
}

/// Raises `error` into the central surface (the shell's error surface
/// shows it), into the monitoring sink, and onto the bus (`app.error`).
pub fn report_error(error: impl Into<AppError>) {
    let error = error.into();
    crate::services::logging::log_error(&format!("{error}"));
    crate::services::monitoring::sink().record_error(&error);
    event_bus::publish(
        event_bus::TOPIC_APP_ERROR,
        format!("{} ({})", error.message, error.code),
    );
    queue().update(|queue| {
        queue.0.push(error);
        if queue.0.len() > ERROR_QUEUE_MAX {
            queue.0.remove(0);
        }
    });
}

/// Removes the queued error with this id (the surface's Dismiss).
pub fn dismiss_error(id: u64) {
    queue().update(|queue| queue.0.retain(|error| error.id != id));
}

/// Removes every queued error.
pub fn clear_errors() {
    queue().update(|queue| queue.0.clear());
}

/// The queued errors, oldest first (tests and diagnostics read this).
pub fn errors() -> Signal<ErrorQueue> {
    queue()
}
