//! The UI gallery: every control of `ui`, with its states.
//!
//! Test: `tests/gallery.rs`.

mod view;

pub use view::gallery_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, HeaderAction, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "gallery",
    title: "UI gallery",
    summary: "Every control of the app's own library, with its states.",
    icon: lucide!(
        r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>"#
    ),
    symbol: "square.grid.2x2",
    frame: Frame::Scrolls,
    action: Some(HeaderAction {
        test_id: "gallery-appearance",
        label: "Appearance",
        symbol: "circle.lefthalf.filled",
        run: crate::store::appearance::toggle,
    }),
    content: Content::Plain(|| gallery_page().into_element()),
};
