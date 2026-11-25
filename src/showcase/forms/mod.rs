//! Forms on `nsh-form`: the sign-up screen (every `nsh-form` behavior) and
//! the settings form that loads with `form.load`.
//!
//! The section runs a navigation stack of its own (`Frame::OwnStack`): its
//! screens' `pop_on_back` guards and their confirm dialogs pop by
//! themselves, so nothing wraps it in another stack.
//!
//! Test: `tests/forms.rs`.

pub mod fields;
pub mod settings;
pub mod sign_up;
mod view;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "forms",
    title: "Forms",
    summary: "A full sign-up and a settings form on nsh-form.",
    icon: lucide!(
        r#"<path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z"/>"#
    ),
    symbol: "square.and.pencil",
    frame: Frame::OwnStack,
    action: None,
    content: Content::Plain(|| forms_page().into_element()),
};
