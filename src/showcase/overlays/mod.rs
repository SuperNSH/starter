//! Overlays and desktop surfaces: `Dialog`, `AlertDialog`, `Sheet`,
//! `Popover`, `Menu`, `ContextMenu`, `HoverCard`, `Tooltip`, toasts,
//! `DockArea`, resizable panels, `Editor`, `DiffView`, `TextView`.
//!
//! Test: `tests/overlays.rs`.

mod code_sections;
mod dialog_section;
mod dock_section;
mod hovering;
mod menus;
mod panels;
mod sheet_section;
mod toasts;
mod view;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "overlays",
    title: "Overlays",
    summary: "Dialogs, sheets, menus, toasts and desktop panes.",
    icon: lucide!(
        r#"<path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/><path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/>"#
    ),
    symbol: "square.stack.3d.up",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| overlays_page().into_element()),
};
