//! Lists and data views: the `Table` parts over an order book, a sectioned
//! list with sticky headers, `DataTable` over three thousand quotes with
//! sorting and a fixed column, a `Tree`, a `Calendar`, a grid of cards with
//! viewability, an inverted chat that loads at both ends, a `Carousel` and
//! `Pagination`. One file per view; `view.rs` switches between them.
//!
//! Test: `tests/lists.rs`.

pub mod calendar_view;
pub mod carousel_view;
pub mod chat;
pub mod grid;
pub mod orders;
pub mod pager;
pub mod quotes;
pub mod tree;
mod view;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "lists",
    title: "Lists",
    summary: "Tables, a tree, a calendar, a chat and a carousel.",
    icon: lucide!(r#"<path d="M3 5h.01M3 12h.01M3 19h.01M8 5h13M8 12h13M8 19h13"/>"#),
    symbol: "list.bullet",
    frame: Frame::Fixed,
    action: None,
    content: Content::Plain(|| lists_page().into_element()),
};
