//! Canvas and graphics: the drawing app on `canvas`, animated canvas,
//! `Plot` charts, `shader` with WGSL, SVG, images with placeholders and
//! blur, video, `web_view` with messaging. Owned by the canvas area. The
//! shell mounts `canvas_page`; keep its name and signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the canvas area replaces this file.
pub fn canvas_page() -> impl IntoElement {
    page_placeholder("Canvas")
}
