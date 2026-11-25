//! The platform page's layout.

use super::{
    clipboard_card, desktop_card, device, files, i18n_card, keyboard, notifications_card,
    picker_card, safe_area, speech_card, tabbar_card,
};

use nsh_base::theme;
use nsh_core::prelude::*;

use super::widgets::localized;

/// One page of this section: its content in a scroll view that sits
/// inside the safe area on every edge, and lifts above the keyboard. The
/// keyboard and the safe area both own the bottom edge, so one computed
/// padding decides it (the keyboard covers the bottom inset while it shows).
/// The scroll carries `scroll_id` so captures and tests can scroll the page.
fn page(title: &'static str, scroll_id: &'static str, cards: Vec<AnyElement>) -> AnyElement {
    crate::ui::layout::page_scroll(0.0)
        .test_id(scroll_id)
        .child(
            view()
                // The gallery's frame (`ui::layout::page_body`), with the
                // sides kept clear of a landscape notch. UIKit insets the
                // scroll under the header already, so the top is no inset.
                .w_full()
                .flex_col()
                .pt(px(crate::ui::layout::page_top()))
                .pr_safe_area_plus(px(16.0))
                .pl_safe_area_plus(px(16.0))
                .pb(move || {
                    let bars = crate::ui::layout::page_bottom(0.0).0;
                    px(keyboard_height().0.max(bars) + 24.0)
                })
                .gap_6()
                .text_color(move || theme().foreground)
                .child(crate::ui::text::page_heading(localized(title)))
                .children(cards),
        )
        .into_element()
}

/// The platform page: safe areas, keyboard, clipboard, picker, speech, file
/// dialogs, language and RTL, the desktop's windows and menus, the system tab
/// bar.
pub fn platform_page() -> impl IntoElement {
    page(
        "platform.title",
        "platform-page-scroll",
        vec![
            safe_area::card(),
            clipboard_card::card(),
            picker_card::card(),
            speech_card::card(),
            notifications_card::card(),
            files::card(),
            i18n_card::card(),
            desktop_card::card(),
            tabbar_card::card(),
            // The keyboard card last: its field sits at the bottom, where the
            // keyboard reaches.
            keyboard::card(),
        ],
    )
}

/// The device and environment page: what the platform exposes about itself.
pub fn platform_device_page() -> impl IntoElement {
    page(
        "platform.device.page_title",
        "device-page-scroll",
        device::cards(),
    )
}
