//! The app's Rust UI: its root component, which `nsh run`, `nsh package`
//! and `cargo run` mount. Owned by the shell area; keep the `i18n!` block,
//! which every `t!` call in the crate depends on.

#[macro_use]
extern crate rust_i18n;

use nsh_base::theme;
use nsh_core::prelude::*;

i18n!("i18n", fallback = "en");

pub mod app;
pub mod features;
pub mod foundation;
pub mod platform;
pub mod services;
pub mod shell;
pub mod state;
pub mod ui;

/// The app's UI set (`nsh.json` `"rust"`): what a TSX entry imports from
/// `nsh-starter/native`. The shell area keeps this block current.
#[native_module("nsh-starter/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
}

/// The application's root component. Placeholder until the shell area
/// lands the real root; the module tree holds the feature pages.
#[component]
pub fn App() -> impl IntoElement {
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .flex_1()
                .p_safe_area()
                .items_center()
                .justify_center()
                .child(text(t!("app.name").to_string()).text_size(px(24.0))),
        )
}
