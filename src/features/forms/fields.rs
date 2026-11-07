//! The forms feature's field glue: the layout a `nsh-form` field needs
//! (label, control, error) over the ui library's primitives, and the input
//! look with the failed state the ui signal-based wrappers cannot express.
//! The Field-bound variants of `ui::field` are wanted upstream of this
//! module; the request is recorded in docs/STATUS.md (lands with the
//! forms area's STATUS section). No raw `nsh_base` control leaves it.

use nsh_base::{Input, input, theme};
use nsh_core::prelude::*;

use crate::ui::text;

/// A section of fields: the ui card's box (rebuilt here: `ui::layout::card`
/// returns `impl IntoElement`, which takes no children) and its title.
pub fn field_section(title: &'static str, fields: impl IntoIterator<Item = View>) -> View {
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
        .child(text::section_title(title))
        .children(fields)
}

/// A field's label.
pub fn field_label(label: &'static str) -> AnyElement {
    text::label(label).into_element()
}

/// A live hint under a label (`""` hides it): the age, "Checking…".
pub fn hint_line(hint: impl Fn() -> SharedString + Copy + 'static) -> impl IntoElement {
    when(move || !hint().is_empty(), move || text::muted(hint))
}

/// The message a field shows, once its field has one to give.
pub fn error_line(error: impl Fn() -> Option<String> + Copy + 'static) -> impl IntoElement {
    when(
        move || error().is_some(),
        move || {
            text(move || error().unwrap_or_default())
                .text_sm()
                .text_color(move || theme().destructive)
        },
    )
}

/// A labelled field: label, the control, its error.
pub fn field_row(
    label: &'static str,
    control: impl IntoElement,
    error: impl Fn() -> Option<String> + Copy + 'static,
) -> View {
    view()
        .flex_col()
        .gap_1p5()
        .child(field_label(label))
        .child(control)
        .child(error_line(error))
}

/// The app's text input look, as `ui::field`'s frame draws it, with the
/// border reading destructive while the field fails. Behavior (value,
/// autofill, the form's binding) belongs to the caller, then
/// `.bind(field)`.
pub fn input_shell(failed: impl Fn() -> bool + Copy + 'static) -> Input {
    let touch = !is_desktop();
    input()
        .min_h(move || px(if touch { 44.0 } else { 36.0 }))
        .px_3()
        .py_2()
        .text_sm()
        .text_color(move || theme().foreground)
        .rounded(move || theme().radius)
        .border_1()
        .border_color(move || {
            if failed() {
                theme().destructive
            } else {
                theme().input
            }
        })
        .bg(move || theme().background)
        .transition_colors()
        .focus(move |s| {
            s.border_color(move || {
                if failed() {
                    theme().destructive
                } else {
                    theme().ring
                }
            })
            .outline(px(2.0))
            .outline_offset(px(2.0))
            .outline_color(move || theme().ring)
        })
        .placeholder_color(move || color(theme().muted_foreground))
        .caret_color(move || color(theme().foreground))
}

/// The app's header for the desktops and the web (iOS and Android draw the
/// system one from `title`): its back button pops the feature's own stack,
/// so only a page the stack can pop passes `on_back`.
pub fn page_header(title: &'static str, on_back: Option<impl Fn() + 'static>) -> View {
    view()
        .min_h(px(48.0))
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .border_b_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .children(on_back.map(|on_back| {
            crate::ui::button::button_with(
                "Back",
                crate::ui::button::ButtonVariant::Ghost,
                crate::ui::button::ButtonSize::Small,
                false,
                None,
                on_back,
            )
        }))
        .child(text::title(title))
}

/// The banner a form-level message shows in; `tone` picks the role color.
pub fn message_banner(
    message: impl Fn() -> SharedString + 'static,
    tone: impl Fn() -> AdaptiveColor + Copy + 'static,
) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .py_2()
        .rounded(move || theme().radius)
        .bg(move || theme().muted)
        .border_1()
        .border_color(tone)
        .child(
            text(move || message().to_string())
                .text_sm()
                .text_color(tone),
        )
}
