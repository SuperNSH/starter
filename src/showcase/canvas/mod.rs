//! Canvas and graphics: the drawing app on `canvas` with responder input
//! and undo, a signal-driven frame loop, `Plot` charts, `shader` with
//! WGSL, SVG, images with placeholder and blur-up, video and `web_view`
//! with messaging, plus the shape editor, the bézier editor, the node
//! graph, the particle field, the text and image specimens, and the shared
//! board with demo peers. One file per section; `view.rs` lays them out.
//!
//! Test: `tests/canvas.rs`.

mod bezier;
mod canvas_image;
mod canvas_text;
mod charts;
mod collab;
mod media;
mod node_graph;
mod particles;
mod shader;
mod shape_edit;
mod simulation;
mod sketch;
mod svg;
mod view;
mod web;
mod world;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "canvas",
    title: "Canvas",
    summary: "Drawing, charts, shaders, SVG, media and the web view.",
    icon: lucide!(
        r#"<path d="M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z"/><circle cx="13.5" cy="6.5" r=".5"/><circle cx="17.5" cy="10.5" r=".5"/><circle cx="6.5" cy="12.5" r=".5"/><circle cx="8.5" cy="7.5" r=".5"/>"#
    ),
    symbol: "paintpalette",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| canvas_page().into_element()),
};
