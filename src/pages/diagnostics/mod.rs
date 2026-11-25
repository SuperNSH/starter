//! Diagnostics: the app's config, capabilities, background tasks, events
//! and log, with the actions that prove each one works.
//!
//! Test: `tests/diagnostics.rs`.

mod view;

pub use view::diagnostics_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "diagnostics",
    title: "Diagnostics",
    summary: "Config, capabilities, tasks, events and the log.",
    icon: lucide!(r#"<path d="M22 12h-4l-3 9L9 3l-3 9H2"/>"#),
    symbol: "waveform.path.ecg",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| diagnostics_page().into_element()),
};
