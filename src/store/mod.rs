//! What the app remembers: app-wide state behind functions (navigation,
//! config, locale, appearance, the session, tasks, events, errors). Pages read
//! these signals and call these functions; they never hold another page's
//! state.
//!
//! May import: `services`, `error`, `routes`. Never `ui`, `pages`,
//! `showcase` or `shell`.

pub mod appearance;
pub mod config;
pub mod errors;
pub mod event_bus;
pub mod lifecycle;
pub mod locale;
pub mod nav;
pub mod session;
pub mod tasks;
pub mod undo;
