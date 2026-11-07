//! Motion and gestures: motions, transitions, layout animations, springs,
//! swipe rows, drag and drop, pinch and rotate, the scroll-linked header
//! on `diff_clamp`, `pan_activation`, reduced-motion paths. Owned by the
//! motion area. The shell mounts `motion_page`; keep its name and
//! signature.

use crate::foundation::page_placeholder;
use nsh_core::prelude::*;

/// Placeholder page; the motion area replaces this file.
pub fn motion_page() -> impl IntoElement {
    page_placeholder("Motion")
}
