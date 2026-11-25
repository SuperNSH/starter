//! The data section: an infinite feed, a post's own screen and the query
//! devtools, all on `nsh-query`, backed by the local fake API
//! (`services::fake_api`).
//!
//! Screens: `Route::Data` (`feed.rs`), `Route::DataItem` (`item.rs`) and
//! `Route::DataDevtools` (`devtools.rs`).
//! Test: `tests/data.rs`.

mod devtools;
mod feed;
mod item;
mod queries;
mod state;
mod view;

pub use queries::*;
pub use state::*;
pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, HeaderAction, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "data",
    title: "Data",
    summary: "An infinite feed on nsh-query with optimistic likes.",
    icon: lucide!(
        r#"<path d="M22 12h-6l-2 3h-4l-2-3H2"/><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11"/>"#
    ),
    symbol: "tray.full",
    frame: Frame::Scrolls,
    action: Some(HeaderAction {
        test_id: "data-refresh",
        label: "Refresh",
        symbol: "arrow.clockwise",
        run: refresh_feed,
    }),
    content: Content::Plain(|| data_page().into_element()),
};

/// The post screen's record (`Route::DataItem`).
pub const ITEM: Page = Page {
    key: "data_item",
    title: "Data item",
    summary: "One post, read through a tracked query key.",
    icon: lucide!(
        r#"<path d="M22 12h-6l-2 3h-4l-2-3H2"/><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11"/>"#
    ),
    symbol: "tray.full",
    frame: Frame::Scrolls,
    action: None,
    content: Content::WithId(|id| data_item_page(id).into_element()),
};

/// The devtools screen's record (`Route::DataDevtools`).
pub const DEVTOOLS: Page = Page {
    key: "data_devtools",
    title: "Query devtools",
    summary: "The query cache: keys, states and timings.",
    icon: lucide!(
        r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/>"#
    ),
    symbol: "cylinder.split.1x2",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| data_devtools_page().into_element()),
};
