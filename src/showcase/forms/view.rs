//! The forms page's layout.

use nsh_core::prelude::*;

use super::sign_up::sign_up_page;

/// The sign-up page (see the module docs for how the shell mounts it).
pub fn forms_page() -> impl IntoElement {
    sign_up_page()
}
