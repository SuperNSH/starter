//! The sign-in page's layout: the form while signed out, the account card
//! while signed in.

use nsh_base::theme;
use nsh_core::element::{AutoCapitalize, AutoFill, KeyboardType, ReturnKeyType};
use nsh_core::prelude::*;
use nsh_form::{BindField, Field, Form, SubmitError, field};

use crate::error::AppError;
use crate::services::validation;
use crate::store::{locale, session};
use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::form::{SubmitRule, error_line, input_shell, message_banner, submit_first_form};
use crate::ui::{layout, text};

/// What the form holds.
#[derive(Clone, Default, PartialEq)]
struct Credentials {
    email: String,
    password: String,
}

/// The sign-in page.
pub fn sign_in_page() -> impl IntoElement {
    // Made here, not in the form's branch: a sign-in swaps the branch out
    // while the submit still runs, and the form's state must outlive that.
    let form = submit_first_form(Credentials::default());
    let email = field!(form, email).check(|email: &String, _: &Credentials| {
        validation::email_error(email).map(|_| locale::text("sign_in.email_invalid")())
    });
    let password = field!(form, password).check(|password: &String, _: &Credentials| {
        password
            .is_empty()
            .then(|| locale::text("sign_in.password_required")())
    });
    layout::page_scroll(0.0).test_id("sign_in-scroll").child(
        layout::page_body()
            .child(text::page_intro(
                locale::text("sign_in.title"),
                locale::text("sign_in.summary"),
            ))
            .child(
                when(
                    || session::user().is_some(),
                    move || account_card(form).into_element(),
                )
                .otherwise(move || sign_in_form(form, email, password).into_element()),
            ),
    )
}

/// The signed-in user, and the way out.
fn account_card(form: Form<Credentials>) -> impl IntoElement {
    layout::card()
        .test_id("sign_in-account")
        .child(text::section_title(locale::text("sign_in.signed_in")))
        .child(text::body(move || {
            session::user()
                .map(|user| user.display_name)
                .unwrap_or_default()
        }))
        .child(text::muted(move || {
            session::user().map(|user| user.email).unwrap_or_default()
        }))
        .child(view().flex_row().child(button_with(
            locale::text("sign_in.sign_out"),
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            move || {
                session::sign_out();
                form.reset_with(Credentials::default());
            },
        )))
}

/// The form: two fields, the refusal banner and the submit button.
fn sign_in_form(
    form: Form<Credentials>,
    email: Field<Credentials, String>,
    password: Field<Credentials, String>,
) -> impl IntoElement {
    let submit = move || form.submit(sign_in);
    layout::card()
        .child(labelled(
            "sign_in.email",
            email,
            input_shell(move || email.error().is_some())
                .placeholder("you@example.com")
                .keyboard_type(KeyboardType::Email)
                .auto_capitalize(AutoCapitalize::None)
                .auto_correct(false)
                .auto_fill(AutoFill::Email)
                .return_key_type(ReturnKeyType::Next)
                .test_id("sign_in-email"),
        ))
        .child(labelled(
            "sign_in.password",
            password,
            input_shell(move || password.error().is_some())
                .masked(true)
                .auto_capitalize(AutoCapitalize::None)
                .auto_correct(false)
                .auto_fill(AutoFill::Password)
                .return_key_type(ReturnKeyType::Done)
                .on_submit(move |_: &str| submit())
                .test_id("sign_in-password"),
        ))
        .child(text::muted(locale::text("sign_in.hint")))
        .child(when(
            move || form.error().is_some(),
            move || {
                message_banner(
                    move || form.error().unwrap_or_default().into(),
                    || theme().destructive,
                )
            },
        ))
        .child(view().flex_row().child(button_with(
            move || {
                if form.is_submitting() {
                    locale::text("sign_in.submitting")()
                } else {
                    locale::text("sign_in.submit")()
                }
            },
            ButtonVariant::Primary,
            ButtonSize::Medium,
            move || form.is_submitting(),
            None,
            submit,
        )))
}

/// A field's label, its input bound to the field, and its error.
fn labelled(
    label: &'static str,
    field: Field<Credentials, String>,
    input: Input,
) -> impl IntoElement {
    view()
        .flex_col()
        .gap_1p5()
        .child(text::label(locale::text(label)))
        .child(field.bind(input))
        .child(error_line(move || field.error()))
}

/// The submit: the session's sign-in, its refusal shown on the form.
async fn sign_in(values: Credentials) -> Result<(), SubmitError> {
    match session::sign_in(&values.email, &values.password).await {
        Ok(_) => Ok(()),
        Err(error) => Err(SubmitError::form(refusal(&error))),
    }
}

/// What the form says when the provider refuses: its own words, except a
/// refused password, which the app words itself.
fn refusal(error: &AppError) -> String {
    match error.code.as_ref() {
        "denied" => locale::text("sign_in.denied")(),
        _ => error.message.to_string(),
    }
}
