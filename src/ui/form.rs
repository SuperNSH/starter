//! Form glue for `nsh-form`: a field's label and error line, the input
//! frame that reads destructive while its field fails, the form-level
//! banner, and the one validation timing the app's forms share. A screen
//! binds a field over [`input_shell`] (`.bind(field)`).

use nsh_base::{Input, input, theme};
use nsh_core::prelude::*;

use crate::ui::text;

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

/// The forms' one validation timing (`ValidationLogic::revalidate`): a rule
/// stays quiet until the first submit, then follows every edit. Blur alone
/// never shows an error.
pub trait SubmitRule<H, T> {
    fn check(self, rule: impl Fn(&T, &H) -> Option<String> + 'static) -> Self;
}

impl<H: Clone + PartialEq + 'static, T: Clone + PartialEq + 'static> SubmitRule<H, T>
    for nsh_form::Field<H, T>
{
    fn check(self, rule: impl Fn(&T, &H) -> Option<String> + 'static) -> Self {
        self.validators(nsh_form::FieldValidators::new().on_dynamic(rule))
    }
}

/// A form whose rules run on submit, then on change (see [`SubmitRule`]).
pub fn submit_first_form<H: Clone + PartialEq + 'static>(initial: H) -> nsh_form::Form<H> {
    nsh_form::use_form_with(
        nsh_form::FormOptions::new(initial)
            .validation_logic(nsh_form::ValidationLogic::revalidate()),
    )
}
