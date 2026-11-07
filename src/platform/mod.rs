//! Platform and system: safe areas, keyboard, the device and environment
//! page, clipboard, picker, speech, file dialogs, `nsh/mobile` and
//! `nsh/desktop`, i18n, accessibility, capabilities in `nsh.json` (owned
//! by this area), icons and app metadata. Owned by the platform area. The
//! shell mounts `platform_page` and `device_page`; keep their names and
//! signatures.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the platform area replaces this file.
pub fn platform_page() -> impl IntoElement {
    page_placeholder("Platform")
}

/// Placeholder device page; the platform area replaces this file.
pub fn device_page() -> impl IntoElement {
    page_placeholder("Device")
}
