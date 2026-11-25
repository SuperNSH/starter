//! Submitting the sign-up form, the rules its fields share, and leaving
//! the screen.

use super::*;

pub(super) fn submit_button(
    form: Form<SignUp>,
    created: Signal<Option<String>>,
) -> impl IntoElement {
    view().test_id("signup-submit").child(button_with(
        move || {
            if form.is_submitting() {
                SharedString::from("Creating account…")
            } else {
                SharedString::from("Create account")
            }
        },
        ButtonVariant::Primary,
        ButtonSize::Large,
        move || form.is_submitting(),
        None,
        move || form.submit(move |values: SignUp| fake_server(values, created)),
    ))
}

/// The fake sign-up server: latency, then a field error, else success.
pub(super) async fn fake_server(
    values: SignUp,
    created: Signal<Option<String>>,
) -> Result<(), SubmitError> {
    timer(SERVER_LATENCY).await;
    if values.verification_code == "000000" {
        return Err(SubmitError::field(
            "verification_code",
            "That code was rejected. Any six digits work except 000000.",
        ));
    }
    if username_taken(&values.username) {
        return Err(SubmitError::field(
            "username",
            "That username was taken while you typed. Try another.",
        ));
    }
    created.set(Some(values.email));
    Ok(())
}

/// The fake availability check the async rule awaits.
pub(super) async fn check_username(name: String) -> Option<String> {
    timer(USERNAME_CHECK_LATENCY).await;
    username_taken(&name).then(|| "That username is taken".to_owned())
}

pub(super) fn username_taken(name: &str) -> bool {
    TAKEN_USERNAMES.contains(&name.trim().to_lowercase().as_str())
}

/// Optional; digits, `+`, `(`, `)`, `-`, `.`, spaces, at least 7 digits.
pub(super) fn phone_rule() -> impl Fn(&String, &SignUp) -> Option<String> {
    |phone: &String, _: &SignUp| {
        if phone.is_empty() {
            return None;
        }
        let allowed = phone
            .chars()
            .all(|c| c.is_ascii_digit() || "+()-. ".contains(c));
        let digits = phone.chars().filter(|c| c.is_ascii_digit()).count();
        (!allowed || digits < 7).then(|| "Use digits, with optional + ( ) - and spaces".to_owned())
    }
}

pub(super) fn otp_rule() -> impl Fn(&String, &SignUp) -> Option<String> {
    |code: &String, _: &SignUp| {
        let shaped = code.chars().all(|c| c.is_ascii_digit()) && code.chars().count() == 6;
        (!code.is_empty() && !shaped).then(|| "Enter the 6 digits".to_owned())
    }
}

pub(super) fn budget_rule() -> impl Fn(&Option<f64>, &SignUp) -> Option<String> {
    |budget: &Option<f64>, _: &SignUp| {
        budget
            .is_some_and(|value| !(0.0..=10_000.0).contains(&value))
            .then(|| "Between 0 and 10,000".to_owned())
    }
}

/// Whether a ProseMirror document holds any non-blank text.
pub(super) fn doc_has_text(json: &str) -> bool {
    serde_json::from_str::<Value>(json)
        .ok()
        .is_some_and(|doc| node_has_text(&doc))
}

pub(super) fn node_has_text(node: &Value) -> bool {
    if node.get("type").and_then(Value::as_str) == Some("text") {
        return node
            .get("text")
            .and_then(Value::as_str)
            .is_some_and(|text| !text.trim().is_empty());
    }
    node.get("content")
        .and_then(Value::as_array)
        .is_some_and(|nodes| nodes.iter().any(node_has_text))
}

pub(super) fn open_settings(routes: Signal<Vec<FormsRoute>>) {
    routes.update(|r| {
        if r.len() < 2 {
            r.push(FormsRoute::Settings);
        }
    });
}

/// The confirm the dirty guard raises; discard clears the draft.
pub(super) fn confirm_discard(form: Form<SignUp>) {
    confirm_destructive(
        "Discard this draft?",
        "The sign-up draft you started goes away.",
        "Discard draft",
        move |discard| {
            if discard {
                form.reset();
                Storage::remove_item(DRAFT_KEY);
            }
        },
    );
}
