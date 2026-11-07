//! Lists and data views: `Table`, `DataTable`, `Tree`, `Calendar`, grids,
//! chat on an inverted list, viewability, carousel, pagination. Owned by
//! the lists area. The shell mounts `lists_page`; keep its name and
//! signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the lists area replaces this file.
pub fn lists_page() -> impl IntoElement {
    page_placeholder("Lists")
}
