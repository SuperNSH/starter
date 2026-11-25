//! The sign-in page's tests, through the real app on a manual clock: field
//! errors from validation, a refusal on the form, the signed-in state, and
//! the Account card in Settings. The provider is the mock (`services::auth`).
//!
//! Seen to fail, one break per test (log:
//! `docs/evidence/project/template-screens/seen-to-fail.log`): the email
//! rule never failing; `store::session::sign_in` not keeping the user; the
//! mock provider accepting `REJECTED_PASSWORD`; Settings' Account card
//! never showing the signed-in branch.

use std::time::SystemTime;

use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

fn app() -> Headless {
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>());
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app
}

/// Opens the page the way a person does: Settings, then its Sign in.
fn open_sign_in(app: &mut Headless) {
    app.press("id:page-settings").unwrap();
    app.wait_for("text:You are not signed in.").unwrap();
    app.press("role:button[Sign in]").unwrap();
    app.wait_for("id:sign_in-email").unwrap();
}

#[test]
fn a_bad_email_shows_its_field_error_and_signs_no_one_in() {
    let mut app = app();
    open_sign_in(&mut app);
    // Rules stay quiet until the first submit.
    app.type_text("id:sign_in-email", "not-an-email").unwrap();
    assert!(!app.shows("Enter a valid email address"));
    app.press("role:button[Sign in]").unwrap();
    app.wait_for("text:Enter a valid email address").unwrap();
    // An empty password is a field error of its own.
    app.wait_for("text:Enter your password").unwrap();
    assert_eq!(app.screen().count("id:sign_in-account"), Ok(0));
    // After the submit, an edit clears the error it fixes.
    app.type_text("id:sign_in-password", "secret").unwrap();
    app.wait_for_gone(
        "text:Enter your password",
        std::time::Duration::from_secs(1),
    )
    .unwrap();
    assert!(app.shows("Enter a valid email address"));
}

#[test]
fn a_good_sign_in_shows_the_name_and_sign_out_returns_the_form() {
    let mut app = app();
    open_sign_in(&mut app);
    app.type_text("id:sign_in-email", "ada@example.com")
        .unwrap();
    app.type_text("id:sign_in-password", "secret").unwrap();
    app.press("role:button[Sign in]").unwrap();
    app.wait_for("id:sign_in-account").unwrap();
    assert!(app.shows("Mock User"));
    assert!(app.shows("ada@example.com"));

    app.press("role:button[Sign out]").unwrap();
    app.wait_for("id:sign_in-email").unwrap();
    assert_eq!(app.screen().count("id:sign_in-account"), Ok(0));
    assert_eq!(app.screen().input_text("id:sign_in-password"), Ok(Some("")));
}

#[test]
fn a_refused_password_shows_on_the_form() {
    let mut app = app();
    open_sign_in(&mut app);
    app.type_text("id:sign_in-email", "ada@example.com")
        .unwrap();
    app.type_text("id:sign_in-password", "wrong-password")
        .unwrap();
    app.press("role:button[Sign in]").unwrap();
    app.wait_for("text:Wrong email or password").unwrap();
    assert_eq!(app.screen().count("id:sign_in-account"), Ok(0));
}

#[test]
fn settings_shows_who_is_signed_in_and_signs_out() {
    let mut app = app();
    open_sign_in(&mut app);
    app.type_text("id:sign_in-email", "grace@example.com")
        .unwrap();
    app.type_text("id:sign_in-password", "secret").unwrap();
    app.press("role:button[Sign in]").unwrap();
    app.wait_for("id:sign_in-account").unwrap();

    // Back on Settings, the Account card names the user.
    app.back().unwrap();
    app.wait_for("text:Account").unwrap();
    app.wait_for("text:Mock User").unwrap();
    app.press("role:button[Sign out]").unwrap();
    app.wait_for("text:You are not signed in.").unwrap();
}
