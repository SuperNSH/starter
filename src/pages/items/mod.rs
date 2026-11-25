//! Items: a list read from the fake API (`services::fake_api`) through
//! `nsh-query`, and one item's own screen. The template for a list with a
//! detail: loading, error with Retry, empty, and a row that opens the item.
//! Every string is translated (`items.*`).
//!
//! Screens: `Route::Items` (`view.rs`) and `Route::ItemsDetail` (`detail.rs`).
//! Test: `tests/items.rs`.

mod detail;
mod queries;
mod view;

pub use detail::item_detail_page;
pub use view::items_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The list's icon, shared by the detail.
const ICON: &str = lucide!(r#"<path d="M3 5h.01M3 12h.01M3 19h.01M8 5h13M8 12h13M8 19h13"/>"#);

/// The list's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "items",
    title: "Items",
    summary: "A list from the fake API; a row opens its item.",
    icon: ICON,
    symbol: "list.bullet",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| items_page().into_element()),
};

/// One item's record (`Route::ItemsDetail`).
pub const DETAIL: Page = Page {
    key: "items_detail",
    title: "Item",
    summary: "One item, read by its id.",
    icon: ICON,
    symbol: "doc.text",
    frame: Frame::Scrolls,
    action: None,
    content: Content::WithId(|id| item_detail_page(id).into_element()),
};
