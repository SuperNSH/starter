//! Navigation controls: the full-width row for lists of destinations (a
//! sidebar, a settings list), with a trailing slot and a selected state,
//! and the card for a grid of them.

use nsh_base::{ButtonElement, button, theme};
use nsh_core::prelude::*;

/// A full-width navigation row: `label` leading, `trailing` (a chevron, a
/// count) at the far edge, selected tinted like [`crate::ui::button::choice_button`].
/// `test_id` rides the styled inner view — a component element takes no
/// test id of its own, and an unstyled wrapper view collapses into its
/// parent — so callers just pass the string.
pub fn nav_row(
    label: impl Into<SharedString>,
    trailing: Option<View>,
    selected: impl IntoReactive<bool>,
    test_id: &'static str,
    on_press: impl Fn() + 'static,
) -> ButtonElement {
    let label = label.into();
    let selected_binding = selected.into_reactive();
    button()
        .on_press(on_press)
        // The row's name is its label: the trailing glyph stays out of it,
        // for screen readers and for `label:` selectors alike.
        .accessibility_label(label.clone())
        .selected(selected_binding)
        .w_full()
        .min_h(move || px(if is_desktop() { 36.0 } else { 44.0 }))
        .px_1()
        .py_0p5()
        .flex_row()
        .items_center()
        .rounded(move || theme().radius)
        .transition_colors()
        .hover(|s| s.bg(move || theme().accent))
        .active(|s| s.opacity(0.8))
        .focus(|s| {
            s.outline(px(2.0))
                .outline_offset(px(2.0))
                .outline_color(move || theme().ring)
        })
        .disabled_style(|s| s.opacity(0.5))
        .selected_style(|s| {
            s.bg(move || theme().accent)
                .border_color(move || theme().primary)
        })
        .child(
            view()
                .test_id(test_id)
                .flex_1()
                .flex_row()
                .items_center()
                .gap_2()
                .px_2()
                .py_1p5()
                .child(
                    text(label)
                        .flex_1()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .number_of_lines(1)
                        .text_color(move || theme().foreground),
                )
                .when_some(trailing, |row, trailing| row.child(trailing)),
        )
}

/// The trailing chevron most destination rows show.
pub fn chevron() -> View {
    view().child(
        text("›")
            .text_sm()
            .text_color(move || theme().muted_foreground),
    )
}

/// A destination's card: its `title` over one line of `summary`, the whole
/// card the target. Cards wrap into as many columns as the width holds.
/// `test_id` rides the inner view, as [`nav_row`]'s does.
pub fn nav_tile(
    title: &'static str,
    summary: &'static str,
    test_id: &str,
    on_press: impl Fn() + 'static,
) -> ButtonElement {
    button()
        .on_press(on_press)
        .accessibility_label(title)
        .flex_grow(1.0)
        .basis(px(240.0))
        .min_h(px(84.0))
        .rounded_lg()
        .border_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .transition_colors()
        .hover(|s| s.bg(move || theme().accent))
        .active(|s| s.opacity(0.8))
        .focus(|s| {
            s.outline(px(2.0))
                .outline_offset(px(2.0))
                .outline_color(move || theme().ring)
        })
        .child(
            view()
                .test_id(test_id)
                .flex_1()
                .p_4()
                .gap_1()
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            text(title)
                                .flex_1()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .number_of_lines(1)
                                .text_color(move || theme().card_foreground),
                        )
                        .child(chevron()),
                )
                .child(
                    text(summary)
                        .text_xs()
                        .number_of_lines(2)
                        .text_color(move || theme().muted_foreground),
                ),
        )
}

/// A system screen in the app's colors: the screen itself takes the
/// theme's background, which on iOS shows under the header and the home
/// indicator (UIKit's system color there, pure black in dark, sat as a band
/// above and below the page); on Android the native header takes the
/// theme's colors too, as it otherwise keeps the system appearance when the
/// app picks its own (iOS follows the app's appearance and keeps its
/// material).
///
/// On iOS the header is the system's large title over the page's scroll:
/// the content starts at the top of the screen and passes under the bar's
/// own glass as it scrolls, and the title shrinks into the bar, as in
/// Apple's apps. A page whose top is not a scroll view turns it off with
/// `.header_large_title(false)`.
pub fn themed_header(screen: nsh_core::element::Screen) -> nsh_core::element::Screen {
    let screen = screen.bg(move || theme().background);
    if os() == nsh_core::platform::Os::Ios {
        // A transparent header: the page draws its own fade-in blur under
        // it (`ui::layout::top_edge_fade`) instead of UIKit's hard band.
        return screen
            .header_large_title(!is_desktop())
            .header_transparent(!is_desktop());
    }
    if os() != nsh_core::platform::Os::Android {
        return screen;
    }
    screen.header_style_with(|s| {
        s.bg(move || theme().background)
            .text_color(move || theme().foreground)
            .tint_color(move || theme().foreground)
    })
}
