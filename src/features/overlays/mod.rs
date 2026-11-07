//! Overlays and desktop surfaces: `Dialog`, `AlertDialog`, `Sheet`,
//! `Popover`, `Menu`, `ContextMenu`, `HoverCard`, `Tooltip`, toasts,
//! `DockArea`, resizable panels, `Editor`, `DiffView`, `TextView`. Owned
//! by the overlays area. The shell mounts `overlays_page`; keep its name
//! and signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the overlays area replaces this file.
pub fn overlays_page() -> impl IntoElement {
    page_placeholder("Overlays")
}
