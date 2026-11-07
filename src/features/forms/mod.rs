//! The forms feature: the full sign-up and the settings form on
//! `nsh-form`. Owned by the forms area. The shell mounts `forms_page`;
//! keep its name and signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the forms area replaces this file.
pub fn forms_page() -> impl IntoElement {
    page_placeholder("Forms")
}
