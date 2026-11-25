//! Native vs app: each control as `ui` draws it and as the system does
//! (`native`), bound to one signal.
//!
//! Test: `tests/native_vs_app.rs`.

mod view;

pub use view::native_vs_app_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "native_vs_app",
    title: "Native vs app",
    summary: "Each control as the app draws it and as the system does.",
    icon: lucide!(
        r#"<path d="M8 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3M12 20v2M12 14v2M12 8v2M12 2v2"/>"#
    ),
    symbol: "rectangle.split.2x1",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| native_vs_app_page().into_element()),
};
