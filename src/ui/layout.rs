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
    view().px_2().py_0p5().rounded_full().bg(bg).child(
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
