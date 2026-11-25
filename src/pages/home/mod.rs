//! Home: a card for every page the app has.
//!
//! Test: `tests/home.rs`.

mod view;

pub use view::home_page;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "home",
    title: "Home",
    summary: "Every page of the boilerplate.",
    icon: lucide!(
        r#"<path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>"#
    ),
    symbol: "house",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(home_page),
};
