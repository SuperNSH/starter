//! The remote screen: the signed TSX package's delivery status, the
//! revocation control that models a host applying a revocation list, and the
//! embedding contract an NSH 0.7.0 host must provide to run the package.
//!
//! Test: `tests/remote_screen.rs`.

mod delivery;
mod view;

pub use delivery::state_path;
pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "remote_screen",
    title: "Remote screen",
    summary: "A signed TSX package, verified and revocable.",
    icon: lucide!(
        r#"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><path d="M3.29 7 12 12l8.71-5M12 22V12"/>"#
    ),
    symbol: "shippingbox",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| remote_screen_page().into_element()),
};
