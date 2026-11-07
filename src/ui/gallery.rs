//! The control gallery: every wrapper in `src/ui`, shown with its states,
//! in both appearances. Owned by the ui area; `ui_gallery_page` is mounted
//! by the shell; keep its name and signature.

use crate::ui::gallery_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the ui area replaces this file.
pub fn ui_gallery_page() -> impl IntoElement {
    gallery_placeholder()
}
