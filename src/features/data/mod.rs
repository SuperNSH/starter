//! The data feature: the feed, its detail page and the query devtools on
//! `nsh-query`, backed by the local fake API in `src/services/fake_api.rs`
//! (owned by this area). Owned by the data area. The shell mounts
//! `data_page`, `data_detail_page` and `query_devtools_page`; keep their
//! names and signatures.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the data area replaces this file.
pub fn data_page() -> impl IntoElement {
    page_placeholder("Data")
}

/// Placeholder detail page; `id` comes from the shell's route.
pub fn data_detail_page(id: u64) -> impl IntoElement {
    let _ = id;
    page_placeholder("Detail")
}

/// Placeholder devtools page; the data area replaces this file.
pub fn query_devtools_page() -> impl IntoElement {
    page_placeholder("Query devtools")
}
