//! The foundation: errors, ids, time, validation, typed config with schema
//! migrations, the event bus, the background task manager, the capability
//! registry, the first-run flow, file logging, the diagnostics page and the
//! central error surface. Owned by the foundation area, which also owns
//! `src/state` and the service seams in `src/services` (except
//! `fake_api.rs`, owned by the data area).

use nsh_base::theme;
use nsh_core::prelude::*;

/// Placeholder diagnostics page; the foundation area replaces this file.
pub fn diagnostics_page() -> impl IntoElement {
    page_placeholder("Diagnostics")
}

/// The shared placeholder the feature stubs render until their area lands
/// the real page. `pub` so it never reads as dead code while stubs remain.
pub fn page_placeholder(title: &'static str) -> impl IntoElement {
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .bg(move || theme().background)
        .child(
            text(title)
                .text_size(px(22.0))
                .text_color(move || theme().muted_foreground),
        )
}
