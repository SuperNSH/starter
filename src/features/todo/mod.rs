//! The to-do list: the app's one screen. `todo_page()` is the shell's
//! mounting contract (`Route::Home` mounts it); keep its name and
//! signature.
//!
//! - `model`: the task and filter types, and the pure list logic.
//! - `store`: the task list as the engine's global signal, persisted to
//!   `Storage` on every change (the settings pattern: one JSON document
//!   under one key).
//! - `page`: the screen — add, toggle, edit, delete, filter; one column
//!   at phone widths, the summary rail beside it on wide windows.

pub mod model;
pub mod page;
pub mod store;
