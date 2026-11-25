//! What the app talks to: everything outside its own memory.
//!
//! - Seams, each one small trait with one local implementation, so the real
//!   library replaces it by editing that one file: `http` (over NSH's
//!   `fetch`), `auth` (a mock provider), `analytics` and `monitoring`
//!   (in-memory sinks). They do not fake the real libraries' depth (no
//!   retry policy, no token refresh, no batching): only the calls the app
//!   makes.
//! - `fake_api`: the local backend the data screens read, with latency,
//!   failure and offline switches.
//! - `logging`, `manifest`, `capabilities`, `time`, `id` and `validation`:
//!   the log, what `nsh.json` declares, the engine's clock, ids and input
//!   checks.
//!
//! May import: `error`. Never anything above it.

pub mod analytics;
pub mod auth;
pub mod capabilities;
pub mod fake_api;
pub mod http;
pub mod id;
pub mod logging;
pub mod manifest;
pub mod monitoring;
pub mod time;
pub mod validation;
