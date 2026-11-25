//! Platform and system: safe areas, keyboard, clipboard, picker, speech,
//! notifications, file dialogs, `nsh/mobile` and `nsh/desktop`, languages
//! and RTL, one card each.
//!
//! Screens: `Route::Platform` (the cards) and `Route::PlatformDevice`
//! (`device.rs`: what the platform says about itself).
//! Test: `tests/platform.rs`.

mod clipboard_card;
mod desktop_card;
mod device;
mod files;
mod i18n_card;
mod keyboard;
mod notifications_card;
mod picker_card;
mod safe_area;
mod speech_card;
mod tabbar_card;
mod view;
mod widgets;

pub use view::*;

use nsh_core::prelude::IntoElement;

use crate::routes::{Content, Frame, Page};
use crate::ui::icons::lucide;

/// The page's record: everything the shell knows about it.
pub const PAGE: Page = Page {
    key: "platform",
    title: "Platform",
    summary: "Safe areas, clipboard, picker, speech and locales.",
    icon: lucide!(
        r#"<rect width="20" height="14" x="2" y="3" rx="2"/><path d="M8 21h8M12 17v4"/>"#
    ),
    symbol: "desktopcomputer",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| platform_page().into_element()),
};

/// The device screen's record (`Route::PlatformDevice`).
pub const DEVICE: Page = Page {
    key: "platform_device",
    title: "Device",
    summary: "The device, its screen and the environment.",
    icon: lucide!(
        r#"<rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/>"#
    ),
    symbol: "iphone",
    frame: Frame::Scrolls,
    action: None,
    content: Content::Plain(|| platform_device_page().into_element()),
};
