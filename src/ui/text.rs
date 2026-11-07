//! The app's text styles: one function per role of the type scale (12-20
//! points around a 16-point body), so screens never set sizes directly.

use nsh_base::theme;
use nsh_core::prelude::*;

/// A page or screen title: 20pt semibold.
pub fn title(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_xl()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || theme().foreground)
}

/// A section header inside a page: 16pt semibold.
pub fn section_title(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_base()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || theme().foreground)
}

/// Body copy: 14pt, the reading size for lists and cards.
pub fn body(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_sm()
        .text_color(move || theme().foreground)
}

/// Secondary text: hints, captions, metadata.
pub fn muted(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_sm()
        .text_color(move || theme().muted_foreground)
}

/// The small label above a form field: 12pt medium.
pub fn label(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(move || theme().foreground)
}

/// Inline code and identifiers: monospaced, in an inset surface.
pub fn code(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label.into_reactive())
        .text_xs()
        .font_family(move || {
            theme()
                .font_mono
                .clone()
                .unwrap_or_else(|| "monospace".into())
        })
        .px_1()
        .rounded(move || theme().radius)
        .bg(move || theme().muted)
        .text_color(move || theme().foreground)
}
