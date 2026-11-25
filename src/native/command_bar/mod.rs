//! The macOS command bar: the palette as a window of its own, Raycast
//! style. A borderless, non-activating panel on Liquid Glass
//! (`NSGlassEffectView`; the HUD material before macOS 26) with a search
//! field and the command list, opened by Cmd+K in the app, by Option+Space
//! from any app, and from the menu bar item, which works with the main
//! window hidden.
//!
//! It was built on NSH 0.4.0, which had no second window, status item or
//! global shortcut, so AppKit draws all of it. 0.5.0 added all three
//! (`Window`, `Tray`, global shortcuts) and 0.6.0 keeps them. The panel stays on AppKit: its glass material and non-activating panel
//! have no `Window` option, and it already serves the menu bar item and Option+Space. A hidden native component in the shell
//! (`shell::command_bar::bridge`) hands the panel its commands and carries the chosen one back
//! into the app, where the shell's palette runs it. Elsewhere the in-app
//! palette stays.
//!
//! This module knows nothing of the app's pages: the shell hands it the
//! commands as data and gets the chosen id back.

mod rank;

pub use rank::{rank, score};

/// The bridge's name among the native components.
pub const BRIDGE: &str = "starter.command-bar";

/// The global shortcut, as the menu bar item shows it.
pub const HOTKEY_LABEL: &str = "⌥Space";

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::register as register_macos;
