//! Surfaces and small marks: the card, the section, the badge, the
//! skeleton, the divider. Screens compose these instead of drawing boxes.

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::ui::text;

/// A card: the app's raised content surface, with its foreground set so
/// children inherit readable text.
pub fn card() -> nsh_core::element::View {
    view()
        .w_full()
        .flex_col()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .p_4()
        .text_color(move || theme().card_foreground)
}

/// A section of a page: a title, then the children, separated from the
/// next section by space alone (no nested cards).
pub fn section(
    title: impl IntoReactive<SharedString>,
    content: impl IntoElement,
) -> impl IntoElement {
    view()
        .flex_col()
        .gap_2()
        .child(text::section_title(title))
        .child(content)
}

/// A short status or count, tinted by its tone; keep most badges neutral.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeTone {
    /// No meaning: the muted surface.
    #[default]
    Neutral,
    /// The primary action's color.
    Primary,
    /// Success.
    Success,
    /// Warning.
    Warning,
    /// Information.
    Info,
    /// Danger and errors.
    Danger,
}

/// A tone's color as a plain function, so a match can hand it around.
type ToneColor = fn() -> AdaptiveColor;

fn neutral_bg() -> AdaptiveColor {
    theme().muted
}

fn neutral_fg() -> AdaptiveColor {
    theme().muted_foreground
}

fn primary_bg() -> AdaptiveColor {
    theme().primary
}

fn primary_fg() -> AdaptiveColor {
    theme().primary_foreground
}

fn danger_bg() -> AdaptiveColor {
    theme().destructive
}

/// A badge: text on a tinted pill, one row high.
pub fn badge(label: impl IntoReactive<SharedString>, tone: BadgeTone) -> impl IntoElement {
    let (bg, fg): (ToneColor, ToneColor) = match tone {
        BadgeTone::Neutral => (neutral_bg, neutral_fg),
        BadgeTone::Primary => (primary_bg, primary_fg),
        BadgeTone::Success => (
            crate::ui::tokens::success,
            crate::ui::tokens::success_foreground,
        ),
        BadgeTone::Warning => (
            crate::ui::tokens::warning,
            crate::ui::tokens::warning_foreground,
        ),
        BadgeTone::Info => (crate::ui::tokens::info, crate::ui::tokens::info_foreground),
        BadgeTone::Danger => (danger_bg, crate::ui::tokens::destructive_foreground),
    };
    view()
        .self_start()
        .px_2()
        .py_0p5()
        .rounded_full()
        .bg(bg)
        .child(
            text(label.into_reactive())
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg),
        )
}

/// The static placeholder box for content that is loading: draw it in
/// the known shape of what will replace it, muted so it reads as a gap.
pub fn skeleton() -> impl IntoElement {
    view()
        .w_full()
        .h_4()
        .rounded(move || theme().radius)
        .bg(move || theme().muted)
}

/// A skeleton of another height, in points (a physical stand-in, not
/// rhythm): match the row it replaces.
pub fn skeleton_of(height: f32) -> impl IntoElement {
    view()
        .w_full()
        .h(px(height))
        .rounded(move || theme().radius)
        .bg(move || theme().muted)
}

/// A hairline between two regions: the boundary owner draws it.
pub fn divider() -> impl IntoElement {
    view().w_full().h(px(1.0)).bg(move || theme().border)
}

/// A page's content column, the gallery's frame shared by every page: 16pt
/// around, 24pt between sections, room under the last one. Put it inside the
/// page's scroll ([`page_scroll`]) and give it `text::page_intro` first.
pub fn page_body() -> nsh_core::element::View {
    view()
        .w_full()
        .flex_col()
        .gap_6()
        .p_4()
        .pt(px(page_top()))
        .pb(move || page_bottom(24.0))
        .text_color(move || theme().foreground)
}

/// The space above a page's content: none under iOS's large title, which
/// brings its own.
pub fn page_top() -> f32 {
    if os() == nsh_core::platform::Os::Ios && !is_desktop() {
        0.0
    } else {
        16.0
    }
}

/// The height the phone tab bar floats over at the bottom of a page, above
/// the safe area: iOS's Liquid Glass bar and Android's themed one. Zero on
/// the desktops and the web, which show no tab bar.
pub fn tab_bar_room() -> f32 {
    use nsh_core::platform::Os;
    if is_desktop() {
        return 0.0;
    }
    match os() {
        Os::Ios => 64.0,
        Os::Android => 72.0,
        _ => 0.0,
    }
}

/// A page's scroll (`top` extra points above its content). The content
/// passes under the phone's bars; its own [`page_bottom`] padding lets it
/// rest clear of them. It bounces on phones only: on a desktop the bounce
/// pulls the page far off its header.
pub fn page_scroll(top: f32) -> nsh_core::element::Scroll {
    scroll()
        .flex_1()
        .content_inset(nsh_core::EdgeInsets {
            top: px(top),
            ..nsh_core::EdgeInsets::default()
        })
        .bounces(page_bounces())
}

/// The padding under a page's last row: `gap` points, over the floating
/// tab bar. Padding, not an inset: a page whose content changes while it
/// rests in the inset jumps back by it. Tracked.
pub fn page_bottom(gap: f32) -> nsh_core::Points {
    px(gap + bottom_room())
}

/// The bars at a page's bottom that its scroll must add room for. UIKit
/// adds the home indicator's itself.
fn bottom_room() -> f32 {
    use nsh_core::platform::Os;
    if os() == Os::Ios {
        tab_bar_room()
    } else {
        tab_bar_room() + safe_area().bottom.0
    }
}

/// Whether a page's own scroll bounces at its ends (see [`page_scroll`]).
pub fn page_bounces() -> bool {
    !is_desktop()
}

/// The bottom room as a content inset, for a page that is one virtual list
/// (its rows take no padding under them).
pub fn list_insets() -> impl Fn() -> nsh_core::EdgeInsets + 'static {
    || nsh_core::EdgeInsets {
        bottom: px(bottom_room()),
        ..nsh_core::EdgeInsets::default()
    }
}
