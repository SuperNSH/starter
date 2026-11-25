//! The crate root: the module list, the native registration and `App`,
//! the root component `nsh run`, `nsh package` and `cargo run` mount
//! (`nsh.json` `"root"`). Keep the `i18n!` block: every `t!` call in the
//! crate depends on it.
//!
//! Start reading at `AGENTS.md`, then `routes.rs`, then `pages/mod.rs`.

#[macro_use]
extern crate rust_i18n;

use nsh_base::theme;
use nsh_core::{element::desktop::window as app_window, prelude::*};

i18n!("i18n", fallback = "en");

// `unsafe` lives in `native` and nowhere else: the compiler refuses it in
// every other module.
#[forbid(unsafe_code)]
pub mod error;
pub mod native;
#[forbid(unsafe_code)]
pub mod pages;
#[forbid(unsafe_code)]
pub mod routes;
#[forbid(unsafe_code)]
pub mod services;
#[forbid(unsafe_code)]
pub mod shell;
#[forbid(unsafe_code)]
pub mod showcase;
#[forbid(unsafe_code)]
pub mod startup;
#[forbid(unsafe_code)]
pub mod store;
#[forbid(unsafe_code)]
pub mod ui;

/// The app's UI set (`nsh.json` `"rust"`): what a TSX entry imports from
/// `nsh-starter/native`.
#[native_module("nsh-starter/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    // The crate's shipped files under assets/: the canvas media's images
    // and clip, so nsh run serves them and nsh package copies them.
    registry.assets(nsh_core::element::assets!());
}

/// The iOS views the crate draws itself (`src/native`).
#[native_components(ios)]
fn ios_components(components: &nsh_platform_ios::Components) {
    native::soft_edges::register(components);
    native::controls::register_ios(components);
    native::kit::register_ios(components);
}

/// The macOS views the crate draws itself (`src/native`, and the command bar).
#[native_components(macos)]
fn macos_components(components: &nsh_platform_macos::Components) {
    native::controls::register_macos(components);
    native::kit::register_macos(components);
    native::command_bar::register_macos(components);
}

/// The application's root: the launch wiring, then the shell on the
/// surface's background and text color. The safe area is left to the
/// shell's pages and bars, which know where their headers and tab bars sit.
/// The app's root: its one window. Off the desktops `window()` shows its
/// content in the app's one surface.
#[component]
pub fn App() -> impl IntoElement {
    app_window()
        .id("main")
        .title(services::manifest::info().0.to_string())
        // Wide enough for the desktop shell (`shell::desktop`).
        .default_size(Size::new(px(1280.0), px(800.0)))
        .content(|| {
            // Inside the window: keystrokes and the title bar are its own.
            startup::install();
            view()
                .flex_1()
                .bg(move || theme().background)
                .text_color(move || theme().foreground)
                .child(shell::shell())
        })
}
