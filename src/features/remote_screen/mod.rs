//! The remote screen: the signed TSX package in `packages/remote-screen`
//! and the Rust page that shows its status, loading and revocation. Owned
//! by the remote-screen area. The shell mounts `remote_screen_page`; keep
//! its name and signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the remote-screen area replaces this file.
pub fn remote_screen_page() -> impl IntoElement {
    page_placeholder("Remote screen")
}
