//! The app's appearance: light or dark. NSH's own `dark()` is the reactive
//! read and follows the system; this is the one place that overrides it.

use nsh_core::prelude::*;

/// Flips the appearance between light and dark.
pub fn toggle() {
    if dark() {
        set_color_scheme(Appearance::Light);
    } else {
        set_color_scheme(Appearance::Dark);
    }
}
