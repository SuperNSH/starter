//! One row per control: the app's own on one side, the system's on the
//! other, bound to one signal.

use super::*;

pub(super) fn glass_row() -> impl IntoElement {
    let last = signal(SharedString::from("none"));
    let press = move |name: &'static str| move || last.set(name.into());
    let tool = move |at: usize| last.set((*BAR_LABELS.get(at).unwrap_or(&"?")).into());
    let app = over_photo(
        view()
            .flex_col()
            .gap_3()
            .child(
                view()
                    .flex_row()
                    .gap_2()
                    .child(button_with(
                        "Share",
                        ButtonVariant::Primary,
                        ButtonSize::Medium,
                        false,
                        None,
                        press("Share"),
                    ))
                    .child(button_with(
                        "Edit",
                        ButtonVariant::Secondary,
                        ButtonSize::Medium,
                        false,
                        None,
                        press("Edit"),
                    )),
            )
            .child(
                view()
                    .flex_row()
                    .self_start()
                    .gap_1()
                    .p_1()
                    .rounded_full()
                    .bg(move || theme().card)
                    .border_1()
                    .border_color(move || theme().border)
                    .children(BAR_LABELS.iter().enumerate().map(move |(at, label)| {
                        button_with(
                            *label,
                            ButtonVariant::Ghost,
                            ButtonSize::Small,
                            false,
                            None,
                            move || tool(at),
                        )
                    })),
            ),
    );
    let native = over_photo(
        view()
            .flex_col()
            .gap_3()
            .child(
                view()
                    .flex_row()
                    .gap_2()
                    .child(native_glass_button(
                        "Share",
                        Some("square.and.arrow.up"),
                        true,
                        press("Share"),
                    ))
                    .child(native_glass_button("Edit", None, false, press("Edit"))),
            )
            .child(native_glass_bar(BAR_SYMBOLS, BAR_LABELS, false, tool))
            .child(native_glass_bar(BAR_SYMBOLS, BAR_LABELS, true, tool)),
    );
    compare(
        "Liquid Glass",
        "Glass is for controls floating over content: toolbars, the tab bar, a play button. \
         Keep it off cards and lists. Regular glass first, clear (the second bar) over media.",
        app,
        native,
        move || format!("last pressed: {}", last.get()),
    )
}

pub(super) fn buttons_row() -> impl IntoElement {
    let last = signal(SharedString::from("none"));
    let press = move |name: &'static str| move || last.set(name.into());
    let app = view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(button_with(
            "Continue",
            ButtonVariant::Primary,
            ButtonSize::Medium,
            false,
            None,
            press("Continue"),
        ))
        .child(button_with(
            "Send",
            ButtonVariant::Secondary,
            ButtonSize::Medium,
            false,
            None,
            press("Send"),
        ))
        .child(button_with(
            "Later",
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            press("Later"),
        ))
        .child(button_with(
            "Details",
            ButtonVariant::Ghost,
            ButtonSize::Medium,
            false,
            None,
            press("Details"),
        ));
    let native = view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(native_button(
            "Continue",
            NativeButtonStyle::Filled,
            None,
            press("Continue"),
        ))
        .child(native_button(
            "Send",
            NativeButtonStyle::Tinted,
            Some("square.and.arrow.up"),
            press("Send"),
        ))
        .child(native_button(
            "Later",
            NativeButtonStyle::Gray,
            None,
            press("Later"),
        ))
        .child(native_button(
            "Details",
            NativeButtonStyle::Plain,
            Some("info.circle"),
            press("Details"),
        ));
    compare(
        "Buttons",
        "System buttons for dialogs, toolbars and settings; the app's for branded calls to action.",
        app,
        native,
        move || format!("last pressed: {}", last.get()),
    )
}

pub(super) fn switch_row() -> impl IntoElement {
    let on = signal(true);
    compare(
        "Switch",
        "An on/off setting that applies at once. On macOS the system prefers a checkbox for most of these.",
        controls::switch_row("Wi-Fi", on),
        native_switch("Wi-Fi", on),
        move || format!("on: {}", on.get()),
    )
}

pub(super) fn checkbox_row() -> impl IntoElement {
    let on = signal(false);
    compare(
        "Checkbox",
        "Options in a form or a settings pane, applied on save.",
        controls::checkbox_row("Show hidden files", on),
        native_checkbox("Show hidden files", on),
        move || format!("checked: {}", on.get()),
    )
}

pub(super) fn radio_row() -> impl IntoElement {
    let size = signal(SharedString::from("Medium"));
    compare(
        "Radio group",
        "One of a few visible options. Past five, use a pop-up.",
        controls::radio_group_control(SIZES, size),
        native_radio_group(SIZES, size),
        move || format!("size: {}", size.get()),
    )
}

pub(super) fn segmented_row() -> impl IntoElement {
    let at = signal(0_usize);
    compare(
        "Segmented control",
        "Switching the view of the same content.",
        tabs::tab_bar(move || at.get(), VIEWS, move |next| at.set(next)),
        native_segmented(VIEWS, at),
        move || format!("view: {}", VIEWS.get(at.get()).unwrap_or(&"none")),
    )
}

pub(super) fn tabs_row() -> impl IntoElement {
    let at = signal(0_usize);
    let app = view()
        .flex_col()
        .gap_2()
        .child(tabs::tab_bar(
            move || at.get(),
            PANES,
            move |next| at.set(next),
        ))
        .child(view().p_3().child(text::body(move || {
            SharedString::from(*PANE_BODIES.get(at.get()).unwrap_or(&""))
        })));
    compare(
        "Tabs",
        "Separate panes of one window: tabs over a box on macOS. iOS keeps tabs to the tab bar.",
        app,
        native_tabs(PANES, PANE_BODIES, at),
        move || format!("pane: {}", PANES.get(at.get()).unwrap_or(&"none")),
    )
}

pub(super) fn popup_row() -> impl IntoElement {
    let plan = signal(Some(SharedString::from("Pro")));
    let items = PLANS.iter().map(|p| select::choice(p, p)).collect();
    compare(
        "Pop-up / select",
        "One of many, closed until asked. The system menu matches every other app's.",
        select::select_field("Plan", items, plan, "Choose a plan"),
        native_popup("Plan", PLANS, plan),
        move || format!("plan: {}", plan.get().unwrap_or_default()),
    )
}

pub(super) fn text_row() -> impl IntoElement {
    let name = signal(String::new());
    let query = signal(String::new());
    let app = view()
        .flex_col()
        .gap_3()
        .child(field::text_field(name, "Display name"))
        .child(field::text_field(query, "Search"));
    let native = view()
        .flex_col()
        .gap_3()
        .child(native_text_field(name, "Display name", false))
        .child(native_text_field(query, "Search", true));
    compare(
        "Text and search fields",
        "The system field brings the platform's editing menu, autofill and spell check.",
        app,
        native,
        move || format!("name: {:?} · search: {:?}", name.get(), query.get()),
    )
}

pub(super) fn slider_row() -> impl IntoElement {
    let volume = signal(40.0_f64);
    let level = move || volume.get() / 100.0;
    let app = view()
        .flex_col()
        .gap_3()
        .child(controls::slider(volume, 0.0, 100.0))
        .child(controls::progress(move || volume.get() as f32))
        .child(controls::progress_indeterminate());
    let native = view()
        .flex_col()
        .gap_3()
        .child(native_slider(volume, 0.0, 100.0))
        .child(native_progress(level))
        .child(native_spinner());
    compare(
        "Slider, progress and activity",
        "The slider drives both progress bars below it.",
        app,
        native,
        move || format!("volume: {:.0}", volume.get()),
    )
}

pub(super) fn stepper_row() -> impl IntoElement {
    let count = signal(Some(2.0_f64));
    let native_count = signal(2.0_f64);
    // The two controls hold different shapes (the field may be empty); the
    // guards stop the pair echoing.
    effect(move || {
        if let Some(n) = count.get()
            && n != native_count.get_untracked()
        {
            native_count.set(n);
        }
    });
    effect(move || {
        let n = native_count.get();
        if count.get_untracked() != Some(n) {
            count.set(Some(n));
        }
    });
    let native = view()
        .flex_row()
        .items_center()
        .gap_3()
        .child(text::body(move || {
            SharedString::from(format!("{} seats", native_count.get()))
        }))
        .child(native_stepper(native_count, 0.0, 20.0, 1.0));
    compare(
        "Stepper",
        "Small counts nudged by one. macOS pairs the stepper with a field or label.",
        number::number_field("Seats", count, "Seats"),
        native,
        move || format!("seats: {:?}", count.get()),
    )
}

/// Today in UTC: a picker with no date shows AppKit's reference date
/// (1 January 2001), so both sides start on a real one.
pub(super) fn today() -> Option<Date> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?;
    chrono::DateTime::from_timestamp(now.as_secs() as i64, 0).map(|d| Date::from(d.date_naive()))
}

pub(super) fn date_row() -> impl IntoElement {
    let day = signal(today());
    compare(
        "Date picker",
        "The system picker follows the user's calendar, locale and first weekday.",
        pickers::date_picker_of("Due date", day, "Pick a date"),
        native_date(day, false),
        move || format!("date: {:?}", day.get().and_then(|d| d.start())),
    )
}

pub(super) fn calendar_row() -> impl IntoElement {
    let day = signal(today());
    compare(
        "Calendar",
        "A month always open, for picking among nearby days.",
        pickers::calendar_of(day),
        native_date(day, true),
        move || format!("date: {:?}", day.get().and_then(|d| d.start())),
    )
}

pub(super) fn color_row() -> impl IntoElement {
    let tint = signal(theme().primary.light);
    compare(
        "Color",
        "The system well opens the platform's color panel, with the user's saved colors.",
        color::color_picker_of(tint, |_| {}),
        native_color_well(tint),
        move || {
            let c = tint.get();
            format!(
                "hsla: {:.0}° {:.0}% {:.0}%",
                c.h * 360.0,
                c.s * 100.0,
                c.l * 100.0
            )
        },
    )
}

pub(super) fn alerts_row(open_dialog: Signal<bool>, open_sheet: Signal<bool>) -> impl IntoElement {
    let answer = signal(SharedString::from("none"));
    let answered = move |chosen: Option<&'static str>| {
        answer.set(chosen.unwrap_or("Cancel").into());
    };
    let app = view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(button_with(
            "Dialog",
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            move || open_dialog.set(true),
        ))
        .child(button_with(
            "Sheet",
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            move || open_sheet.set(true),
        ));
    let native = view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(button_with(
            "Alert",
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            move || {
                native_alert(
                    "Discard changes?",
                    "Your edits since the last save are lost.",
                    &["Save"],
                    Some("Discard"),
                    false,
                    answered,
                )
            },
        ))
        .child(button_with(
            "Action sheet",
            ButtonVariant::Outline,
            ButtonSize::Medium,
            false,
            None,
            move || {
                native_alert(
                    "Share",
                    "Send the draft to:",
                    &["Mail", "Messages", "Copy link"],
                    None,
                    true,
                    answered,
                )
            },
        ));
    compare(
        "Dialogs and sheets",
        "System alerts for short decisions (a sheet on the macOS window); the app's for any content.",
        app,
        native,
        move || format!("system answer: {}", answer.get()),
    )
}
