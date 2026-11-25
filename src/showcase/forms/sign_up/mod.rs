//! The sign-up page: rules, an async username check with debounce, server
//! errors landing on fields by path, draft autosave to `Storage`, the
//! dirty guard on `pop_on_back`, the keyboard flow and autofill hints, and
//! one of every control. Everything a server would do here is fake and
//! in-process; nothing touches the network.

use std::time::Duration;

use nsh_base::rich_editor::view::{RichEditor, RichEditorApi, RichEditorChange, rich_editor};
use nsh_base::{
    ColorChannel, ColorSpace, DateTime, PickedDate, SelectEntry, SelectItem, SliderValue,
    TimeOfDay, checkbox, color_area, color_picker, color_slider, color_swatch, combobox, nav_page,
    nav_stack, number_input, otp_input, radio, radio_group, select, slider, switch, theme,
    time_field,
};
use nsh_core::{
    element::{
        AutoCapitalize, AutoFill, KeyboardDismissMode, KeyboardTaps, KeyboardType, ReturnKeyType,
        Screen,
    },
    prelude::*,
    storage::Storage,
};
use nsh_form::{BindField, Field, FieldValidators, Form, SubmitError, field, rules};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::showcase::forms::fields::{
    RowReveal, SubmitRule, error_line, field_label, field_row, field_section, hint_line,
    input_shell, message_banner, page_header, revealed_row, submit_first_form,
};
use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::controls::progress_indeterminate;
use crate::ui::dialog::confirm_destructive;
use crate::ui::pickers::styled_date_picker;
use crate::ui::popups::{popup_backdrop, popup_surface, popup_trigger_frame};
use crate::ui::text;

/// The page scroll's top content inset, in points.
const SCROLL_TOP_INSET: f32 = 8.0;

/// The usernames the fake availability check reports as taken.
const TAKEN_USERNAMES: [&str; 4] = ["ada", "admin", "grace", "lovelace"];

/// The pause after the last keystroke before the check runs.
const USERNAME_DEBOUNCE: Duration = Duration::from_millis(400);

/// How long the fake availability check thinks.
const USERNAME_CHECK_LATENCY: Duration = Duration::from_millis(300);

/// How long the fake sign-up server thinks.
pub(crate) const SERVER_LATENCY: Duration = Duration::from_millis(800);

/// How long the fake code check thinks.
const CODE_CHECK_LATENCY: Duration = Duration::from_millis(500);

/// The draft's key in `Storage`; a fresh app restores it on mount.
const DRAFT_KEY: &str = "forms.sign_up.draft";

/// The sign-up's values.
#[derive(Clone, PartialEq)]
struct SignUp {
    email: String,
    password: String,
    confirm_password: String,
    username: String,
    phone: String,
    referral: String,
    birth_date: DateTime,
    meeting_time: Option<TimeOfDay>,
    region: Option<String>,
    interests: Vec<String>,
    plan: Option<String>,
    terms: bool,
    newsletter: bool,
    age: f64,
    favorite_color: String,
    verification_code: String,
    budget: Option<f64>,
    bio: String,
}

impl Default for SignUp {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            username: String::new(),
            phone: String::new(),
            referral: String::new(),
            birth_date: DateTime::default(),
            meeting_time: None,
            region: None,
            interests: Vec::new(),
            plan: None,
            terms: false,
            newsletter: false,
            age: 21.0,
            favorite_color: String::new(),
            verification_code: String::new(),
            budget: None,
            bio: String::new(),
        }
    }
}

/// The draft as `Storage` keeps it: every non-secret value, the pickers as
/// ISO text. `password`, `confirm_password` and `verification_code` are
/// deliberately left out — a template does not save credentials.
#[derive(Default, Serialize, Deserialize)]
struct SignUpDraft {
    email: String,
    username: String,
    phone: String,
    referral: String,
    /// `"YYYY-MM-DD"`, as the date picker's value round-trips.
    birth_date: Option<String>,
    /// `"HH:MM"`, as the time field's value round-trips.
    meeting_time: Option<String>,
    region: Option<String>,
    interests: Vec<String>,
    plan: Option<String>,
    terms: bool,
    newsletter: bool,
    age: f64,
    favorite_color: String,
    budget: Option<f64>,
    bio: String,
}

impl SignUpDraft {
    fn of(values: &SignUp) -> Self {
        Self {
            email: values.email.clone(),
            username: values.username.clone(),
            phone: values.phone.clone(),
            referral: values.referral.clone(),
            birth_date: values
                .birth_date
                .start()
                .map(|day| day.format("%Y-%m-%d").to_string()),
            meeting_time: values
                .meeting_time
                .map(|time| time.0.format("%H:%M").to_string()),
            region: values.region.clone(),
            interests: values.interests.clone(),
            plan: values.plan.clone(),
            terms: values.terms,
            newsletter: values.newsletter,
            age: values.age,
            favorite_color: values.favorite_color.clone(),
            budget: values.budget,
            bio: values.bio.clone(),
        }
    }
}

impl SignUp {
    fn from_draft(draft: SignUpDraft) -> Self {
        Self {
            email: draft.email,
            username: draft.username,
            phone: draft.phone,
            referral: draft.referral,
            birth_date: draft
                .birth_date
                .and_then(|text| chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d").ok())
                .and_then(|day| day.and_hms_opt(0, 0, 0))
                .map(|day| DateTime::Single(Some(day)))
                .unwrap_or_default(),
            meeting_time: draft
                .meeting_time
                .and_then(|text| chrono::NaiveTime::parse_from_str(&text, "%H:%M").ok())
                .map(TimeOfDay),
            region: draft.region,
            interests: draft.interests,
            plan: draft.plan,
            terms: draft.terms,
            newsletter: draft.newsletter,
            age: draft.age,
            favorite_color: draft.favorite_color,
            budget: draft.budget,
            bio: draft.bio,
            ..Self::default()
        }
    }
}

/// The feature's inner routes; the stack is `sign_up_page`'s.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FormsRoute {
    SignUp,
    Settings,
}

/// The feature's stack: the sign-up at the bottom, settings pushed.
pub(crate) fn sign_up_page() -> impl IntoElement {
    let routes = signal(vec![FormsRoute::SignUp]);
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(nav_stack().flex_1().child(each(
            move || routes.get(),
            |route: &FormsRoute| format!("{route:?}"),
            move |route, index| {
                let page = match route.get_untracked() {
                    FormsRoute::SignUp => sign_up_screen(routes),
                    FormsRoute::Settings => {
                        crate::showcase::forms::settings::settings_screen(routes)
                    }
                };
                page.on_dismiss(move || {
                    routes.update(|r| r.truncate(index.get_untracked()));
                })
            },
        )))
}

/// A bound text field row: the app's look, the field's binding and error.
fn text_field_row(
    label: &'static str,
    field: Field<SignUp, String>,
    input: nsh_base::Input,
) -> View {
    field_row(label, field.bind(input), move || field.error())
}

/// The username row. The hint mirrors the live check's answer as the user
/// types; the field's error waits for a submit like every other rule.
fn username_row(
    username: Field<SignUp, String>,
    answer: Signal<String>,
    input: nsh_base::Input,
) -> View {
    let hint = move || {
        if username.error().is_some() {
            // The field's error line says it, in red, below the input.
            SharedString::from("")
        } else if !answer.get().is_empty() {
            answer.get().into()
        } else if username.value().chars().count() < 3 {
            SharedString::from("")
        } else {
            // The live check is between the debounce and its answer.
            SharedString::from("Checking…")
        }
    };
    revealed_row(
        view()
            .gap(px(6.0))
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(field_label("Username"))
                    .child(hint_line(hint)),
            )
            .child(username.bind(input))
            .child(error_line(move || username.error())),
        move || username.error().is_some(),
    )
}

mod rows_inputs;
mod rows_pickers;
mod screen;
mod submit;

use rows_inputs::*;
use rows_pickers::*;
use screen::*;
use submit::*;
