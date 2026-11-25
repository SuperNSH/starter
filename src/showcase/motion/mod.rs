//! Motion and gestures: motions, transitions, layout animations, springs,
//! swipe rows, drag and drop, pinch and rotate, the scroll-linked header
//! on `diff_clamp`, `pan_activation`, reduced-motion paths.
//!
//! Every section is one demo of a tool from the animation and gesture
//! guides, run natively (no signal is written per frame to animate). The
//! look is `crate::ui`'s; only the gesture surfaces draw their own views.
//!
//! Test: `tests/motion.rs`.

mod composite;
mod drag_drop;
mod header;
mod reduced;
mod reorder;
mod scrub;
mod springs;
mod swipe;
mod transform;
mod transitions;
mod view;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "motion",
    title: "Motion",
    summary: "Springs, transitions, swipe rows and gestures.",
    icon: lucide!(
        r#"<path d="M12.8 19.6A2 2 0 1 0 14 16H2m15.5-8a2.5 2.5 0 1 1 2 4H2m7.8-7.6A2 2 0 1 1 11 8H2"/>"#
    ),
    symbol: "wind",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| motion_page().into_element()),
};
