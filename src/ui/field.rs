//! Text fields: the app's input wrapper, with the frame, focus ring,
//! caret and placeholder colors, plus the labeled field layout.
//!
//! Two consumption paths: the `*_field` wrappers own a plain
//! `Signal<String>`; a form field from `nsh-form` binds over the `*_frame`
//! constructors instead — `form_field.bind(ui::field::text_field_frame("Email"))`
//! — and the `BindField` supplies the value, the edits and the blur while
//! the frame supplies the look.

use nsh_base::{input, theme};
use nsh_core::prelude::*;

/// One field of a form: its label above, the control, then the hint or
/// validation message under it.
pub fn field(
    label: impl IntoReactive<SharedString>,
    control: impl IntoElement,
) -> impl IntoElement {
    view()
        .flex_col()
        .gap_1p5()
        .child(crate::ui::text::label(label))
        .child(control)
}

/// The input frame: min height by target type, the input border, the
/// focus ring, the caret and placeholder from the theme.
fn frame(input: Input) -> Input {
    let touch = !is_desktop();
    input
        .min_h(move || px(if touch { 44.0 } else { 36.0 }))
        .px_3()
        .py_2()
        .text_sm()
        .text_color(move || theme().foreground)
        .rounded(move || theme().radius)
        .border_1()
        .border_color(move || theme().input)
        .bg(move || theme().background)
        .transition_colors()
        .focus(|s| {
            s.border_color(move || theme().ring)
                .outline(px(2.0))
                .outline_offset(px(2.0))
                .outline_color(move || theme().ring)
        })
        .placeholder_color(move || color(theme().muted_foreground))
        .caret_color(move || color(theme().foreground))
}

/// A single-line field bound to `value`; typing writes the signal. The
/// placeholder is the field's accessibility label too — an unnamed field
/// is a blank line to a screen reader (the platform pages' convention).
pub fn text_field(value: Signal<String>, placeholder: impl Into<String>) -> Input {
    let placeholder = placeholder.into();
    text_field_labeled(value, placeholder.clone(), placeholder.as_str().to_owned())
}

/// A single-line field whose placeholder and accessibility label are set
/// apart: the inline edit holds the task's title, so its name says what
/// it edits while its placeholder stays quiet; a localized field reads
/// both from the locale as bindings.
pub fn text_field_labeled(
    value: Signal<String>,
    placeholder: impl IntoReactive<String>,
    label: impl IntoReactive<SharedString>,
) -> Input {
    frame(
        input()
            .value(move || value.get())
            .on_change(move |text: &str| value.set(text.to_owned()))
            .placeholder(placeholder)
            .label(label),
    )
}

/// A password field: masked, no autocapitalization or correction.
pub fn password_field(value: Signal<String>, placeholder: impl Into<String>) -> Input {
    frame(
        input()
            .value(move || value.get())
            .on_change(move |text: &str| value.set(text.to_owned()))
            .placeholder(placeholder.into())
            .masked(true)
            .auto_capitalize(AutoCapitalize::None)
            .auto_correct(false),
    )
}

/// A multi-line field: it grows with its content from a 4-line floor.
pub fn multiline_field(value: Signal<String>, placeholder: impl Into<String>) -> Input {
    frame(
        input()
            .value(move || value.get())
            .on_change(move |text: &str| value.set(text.to_owned()))
            .placeholder(placeholder.into())
            // 4 lines of 14pt text: the floor a note needs to read as one.
            .min_h(px(96.0))
            .multiline(true)
            .auto_capitalize(AutoCapitalize::Sentences),
    )
}

/// A numeric field: the digits keyboard, values kept as text.
pub fn numeric_field(value: Signal<String>, placeholder: impl Into<String>) -> Input {
    frame(
        input()
            .value(move || value.get())
            .on_change(move |text: &str| value.set(text.to_owned()))
            .placeholder(placeholder.into())
            .keyboard_type(KeyboardType::Number),
    )
}

/// The text field's frame without value wiring, for `nsh-form`: bind a
/// `BindField` over it and it supplies the value, the edits and the blur.
pub fn text_field_frame(placeholder: impl Into<String>) -> Input {
    frame(input().placeholder(placeholder.into()))
}

/// The masked frame for a password `BindField`: bind over it as over
/// [`text_field_frame`]; masking is not `bind`'s to set.
pub fn password_field_frame(placeholder: impl Into<String>) -> Input {
    frame(
        input()
            .placeholder(placeholder.into())
            .masked(true)
            .auto_capitalize(AutoCapitalize::None)
            .auto_correct(false),
    )
}

/// The multi-line frame, four lines tall at the floor: bind a
/// `BindField` over it as over [`text_field_frame`].
pub fn multiline_field_frame(placeholder: impl Into<String>) -> Input {
    frame(
        input()
            .placeholder(placeholder.into())
            // 4 lines of 14pt text: the floor a note needs to read as one.
            .min_h(px(96.0))
            .multiline(true)
            .auto_capitalize(AutoCapitalize::Sentences),
    )
}
