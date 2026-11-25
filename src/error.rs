//! The app's one error type. Services return it, pages raise it with
//! `store::errors::report_error`, and the shell's error surface shows it.

use nsh_core::element::SharedString;

/// The app's error: a stable `code` callers branch on, a `message` the
/// surface shows, and an optional `detail` with what only diagnostics
/// needs.
#[derive(Clone, Debug, PartialEq)]
pub struct AppError {
    /// Stable machine name: `"network"`, `"storage"`, `"invalid"`,
    /// `"unavailable"`, `"internal"`, or a feature's `"feed.offline"`.
    pub code: SharedString,
    /// One sentence on what happened and, when useful, how to recover.
    pub message: SharedString,
    /// The technical part, for logs and diagnostics.
    pub detail: Option<SharedString>,
    /// When it was raised (`services::time::now_ms`).
    pub at_ms: f64,
    /// Ids it within the queue (`store::errors::dismiss_error`).
    pub id: u64,
}

impl AppError {
    /// An error of `code` and `message`, raised now.
    pub fn new(code: impl Into<SharedString>, message: impl Into<SharedString>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            detail: None,
            at_ms: crate::services::time::now_ms(),
            id: crate::services::id::next_id(),
        }
    }

    /// Attaches the technical `detail`.
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        Self::new("internal", message)
    }
}

impl From<nsh_core::storage::StorageFull> for AppError {
    fn from(full: nsh_core::storage::StorageFull) -> Self {
        Self::new("storage", "The app's storage is full").detail(full.to_string())
    }
}
