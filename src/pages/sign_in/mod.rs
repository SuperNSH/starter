//! Sign in: an email and password form on `nsh-form`, signing in through
//! `store::session` (the mock auth seam, no real account). Field errors
//! come from validation, a refused sign-in shows on the form, and a
//! signed-in user sees their name and Sign out. Nothing is gated on it.
//! Every string is translated (`sign_in.*`).
//!
//! Test: `tests/sign_in.rs`.

mod view;

pub use view::sign_in_page;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "sign_in",
    title: "Sign in",
    summary: "Your account: sign in with an email and a password.",
    icon: lucide!(
        r#"<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><path d="m10 17 5-5-5-5"/><path d="M15 12H3"/>"#
    ),
    symbol: "person.crop.circle",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| sign_in_page().into_element()),
};
