//! The app's UI library: theme helpers and one wrapper per `nsh_base`
//! component in use, so screens import from `ui` only. Owned by the ui
//! area, which replaces and extends every file here.

pub mod gallery;

use nsh_base::theme;
use nsh_core::prelude::*;

/// Placeholder gallery page; the ui area replaces `gallery.rs`.
pub fn gallery_placeholder() -> impl IntoElement {
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .bg(move || theme().background)
        .child(
            text("UI gallery")
                .text_size(px(22.0))
                .text_color(move || theme().muted_foreground),
        )
}
