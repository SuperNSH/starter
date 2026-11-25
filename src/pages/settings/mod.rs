//! Settings: the user's name, the language, the appearance and the
//! telemetry switch. Reads and writes `store::config`, `store::locale` and
//! `store::appearance`; every string is translated (`settings.*`).
//!
//! Test: `tests/settings.rs`.

mod view;

pub use view::settings_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "settings",
    title: "Settings",
    summary: "Your name, the language, the appearance and telemetry.",
    icon: lucide!(
        r#"<path d="M20 7h-9M14 17H5"/><circle cx="17" cy="17" r="3"/><circle cx="7" cy="7" r="3"/>"#
    ),
    symbol: "gearshape",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| settings_page().into_element()),
};
