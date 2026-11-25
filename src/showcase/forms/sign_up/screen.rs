//! The sign-up screen: the form's fields in order, its draft autosave and
//! its header.

use super::*;

pub(super) fn sign_up_screen(routes: Signal<Vec<FormsRoute>>) -> Screen {
    let form = submit_first_form(SignUp::default());
    // The draft restores before anything reads the form; `load` makes the
    // restored values the baseline too, so a restored draft is not dirty.
    if let Some(draft) = Storage::get_item(DRAFT_KEY)
        .and_then(|json| serde_json::from_str::<SignUpDraft>(&json).ok())
    {
        form.load(SignUp::from_draft(draft));
    }

    let email = field!(form, email)
        .check(rules::required("Enter your email"))
        .check(rules::email("That is not an email"));
    let password = field!(form, password)
        .check(rules::required("Choose a password"))
        .check(rules::min_length(8, "Use at least 8 characters"));
    let confirm_password = field!(form, confirm_password).check(rules::matches(
        |values: &SignUp| &values.password,
        "Passwords do not match",
    ));
    let username = field!(form, username)
        .check(rules::required("Pick a username"))
        .check(rules::min_length(3, "Use at least 3 characters"))
        .validators(
            FieldValidators::new()
                .on_dynamic_async(check_username)
                .on_dynamic_async_debounce(USERNAME_DEBOUNCE),
        );
    let phone = field!(form, phone).check(phone_rule());
    let referral =
        field!(form, referral).check(rules::unsigned_number::<u32, SignUp>("Use digits only"));
    let birth_date = field!(form, birth_date).check(|date: &DateTime, _: &SignUp| {
        (!date.start().is_some()).then(|| "Pick your birth date".to_owned())
    });
    let meeting_time = field!(form, meeting_time);
    let region = field!(form, region);
    let interests = field!(form, interests);
    let plan = field!(form, plan).check(rules::required("Choose a plan"));
    let newsletter = field!(form, newsletter);
    let age = field!(form, age);
    let favorite_color = field!(form, favorite_color);
    let budget = field!(form, budget).check(budget_rule());
    let verification_code = field!(form, verification_code)
        .check(rules::required("Enter the 6 digits"))
        .check(otp_rule());
    let bio = field!(form, bio).check(|bio: &String, _: &SignUp| {
        (!doc_has_text(bio)).then(|| "Write a line or two about you".to_owned())
    });
    let terms = field!(form, terms).check(|terms: &bool, _: &SignUp| {
        (!terms).then(|| "Accept the terms to continue".to_owned())
    });

    // What the fake code check said about the code as it was completed.
    let code_check = signal(String::new());
    // What the availability check answered for the name as it stands —
    // raw, unlike the field's error, which waits for the first submit.
    let username_answer = signal(String::new());
    let username_seen = stored(0_u64);
    effect(move || {
        let name = username.value();
        let seen = username_seen.with(|seen| {
            *seen += 1;
            *seen
        });
        // A new edit clears the last answer and drops an older one's write.
        username_answer.set(String::new());
        if name.chars().count() < 3 {
            return;
        }
        let answer = username_answer;
        spawn(async move {
            timer(USERNAME_DEBOUNCE + USERNAME_CHECK_LATENCY).await;
            if username_seen.with(|seen| *seen) == seen {
                answer.set(if username_taken(&name) {
                    "That username is taken".to_owned()
                } else {
                    "Available".to_owned()
                });
            }
        });
    });
    // The email the fake server accepted, for the success banner.
    let created = signal(None::<String>);

    // Draft autosave: an actual edit lands in `Storage` as it happens, and
    // a clean form (fresh page, Discard) writes nothing.
    effect(move || {
        if !form.is_dirty() {
            return;
        }
        let draft = SignUpDraft::of(&form.values());
        if let Ok(json) = serde_json::to_string(&draft) {
            let _ = Storage::set_item(DRAFT_KEY, json);
        }
    });

    // A failed submit scrolls the first failing row's label into view.
    let reveal = RowReveal::provide(form, SCROLL_TOP_INSET);

    let submit = move || form.submit(move |values: SignUp| fake_server(values, created));

    // The bio editor reads its first document once, before the body ends.
    let bio_api = component_ref::<RichEditor>();
    let initial_bio = serde_json::from_str::<Value>(&bio.value_untracked()).ok();

    let account = field_section(
        "Account",
        [
            text_field_row(
                "Email",
                email,
                input_shell(move || email.error().is_some())
                    .placeholder("ada@example.com")
                    .keyboard_type(KeyboardType::Email)
                    .auto_capitalize(AutoCapitalize::None)
                    .auto_correct(false)
                    .auto_fill(AutoFill::Email)
                    .return_key_type(ReturnKeyType::Done)
                    .on_submit(move |_: &str| submit())
                    .test_id("signup-email"),
            ),
            text_field_row(
                "Password",
                password,
                input_shell(move || password.error().is_some())
                    .placeholder("At least 8 characters")
                    .masked(true)
                    .auto_fill(AutoFill::NewPassword)
                    .return_key_type(ReturnKeyType::Done)
                    .on_submit(move |_: &str| submit())
                    .test_id("signup-password"),
            ),
            text_field_row(
                "Confirm password",
                confirm_password,
                input_shell(move || confirm_password.error().is_some())
                    .placeholder("Type the password again")
                    .masked(true)
                    .auto_fill(AutoFill::NewPassword)
                    .return_key_type(ReturnKeyType::Done)
                    .on_submit(move |_: &str| submit())
                    .test_id("signup-confirm"),
            ),
            username_row(
                username,
                username_answer,
                input_shell(move || username.error().is_some())
                    .placeholder("Lowercase letters and digits")
                    .auto_capitalize(AutoCapitalize::None)
                    .auto_correct(false)
                    .auto_fill(AutoFill::Username)
                    .return_key_type(ReturnKeyType::Done)
                    .on_submit(move |_: &str| submit())
                    .test_id("signup-username"),
            ),
            phone_row(phone),
            text_field_row(
                "Referral code",
                referral,
                input_shell(move || referral.error().is_some())
                    .placeholder("Who sent you here")
                    .keyboard_type(KeyboardType::Number)
                    .auto_fill(AutoFill::Off)
                    .return_key_type(ReturnKeyType::Done)
                    .test_id("signup-referral"),
            ),
        ],
    );

    let profile = field_section(
        "Profile",
        [
            date_row(birth_date),
            time_row(meeting_time),
            region_row(region),
            interests_row(interests),
            plan_row(plan),
            age_row(age),
        ],
    );

    let preferences = field_section(
        "Preferences",
        [
            switch_row(newsletter),
            color_row(favorite_color),
            budget_row(budget),
        ],
    );

    let verify = field_section(
        "Verify",
        [
            code_row(verification_code, code_check),
            bio_row(bio, bio_api, initial_bio),
        ],
    );

    let consent = view()
        .gap_3()
        .child(terms_row(terms))
        .child(submit_button(form, created))
        .child(when(
            move || form.is_submitting(),
            move || {
                view()
                    .gap_2()
                    .child(progress_indeterminate())
                    .child(text::muted("Contacting the fake server…"))
            },
        ))
        .child(when(
            move || form.error().is_some(),
            move || {
                message_banner(
                    move || form.error().unwrap_or_default().into(),
                    move || theme().destructive,
                )
            },
        ))
        .child(when(
            move || created.get().is_some(),
            move || {
                let email = created.get().unwrap_or_default();
                message_banner(
                    move || format!("Account created for {email}").into(),
                    move || theme().primary,
                )
            },
        ))
        .child(button_with(
            "App settings",
            ButtonVariant::Ghost,
            ButtonSize::Medium,
            false,
            None,
            move || open_settings(routes),
        ));

    sign_up_nav(form)
        .child(
            reveal
                .scroll(scroll())
                .flex_1()
                // nav_page paints the platform's color; the page takes the theme's.
                .bg(move || theme().background)
                // The focused field's label keeps clear of the page header when
                // the reveal scrolls it to the top.
                .content_inset(crate::ui::layout::page_insets(SCROLL_TOP_INSET))
                .keyboard_dismiss_mode(KeyboardDismissMode::OnDrag)
                .keyboard_taps(KeyboardTaps::Handled)
                .test_id("forms-scroll")
                .child(
                    reveal.content(
                        view()
                            .p_4()
                            .pb_safe_area_plus(px(24.0))
                            .gap_4()
                            .child(account)
                            .child(profile)
                            .child(preferences)
                            .child(verify)
                            .child(consent),
                    ),
                ),
        )
        // The same soft header and tab-bar edges as the shell's pages.
        .child(crate::native::soft_edges::page_edges())
}

/// The screen's page furniture: title, drawn header, the dirty guard.
pub(super) fn sign_up_nav(form: Form<SignUp>) -> Screen {
    crate::ui::nav::themed_header(nav_page())
        .title("Sign up")
        .header(page_header("Sign up", None::<fn()>))
        .pop_on_back(move || !form.is_dirty())
        .on_back_press(move || {
            // Discard clears the draft; the next back goes through, the
            // form now clean.
            if form.is_dirty() {
                confirm_discard(form);
            }
        })
}
