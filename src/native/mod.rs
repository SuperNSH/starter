//! Platform code: the views UIKit and AppKit draw for the app, each with
//! a fallback built from `ui` for every other platform. This is the only
//! folder that may contain `unsafe`.
//!
//! May import: `ui`. Never `store`, `services`, `pages`, `showcase` or
//! `shell`.

pub mod command_bar;
pub mod controls;
pub mod kit;
pub mod soft_edges;
