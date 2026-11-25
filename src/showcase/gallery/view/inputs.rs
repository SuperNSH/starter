//! The gallery's input sections: fields, controls, selection, navigation
//! rows and pickers.

use super::*;

pub(super) fn fields_section() -> impl IntoElement {
    let name = signal(String::new());
    let password = signal(String::new());
    let bio = signal(String::new());
    let amount = signal(String::new());
    layout::section(
        "Fields",
        view()
            .flex_col()
            .gap_4()
            .child(field::field(
                "Display name",
                field::text_field(name, "Ada Lovelace"),
            ))
            .child(field::field(
                "Password",
                field::password_field(password, "••••••••"),
            ))
            .child(field::field(
                "About you",
                field::multiline_field(bio, "A few lines about you"),
            ))
            .child(field::field("Amount", field::numeric_field(amount, "0")))
            .child(text::muted(move || {
                format!(
                    "{} {} {} {}",
                    name.get(),
                    if password.get().is_empty() {
                        ""
                    } else {
                        "•••"
                    },
                    bio.get(),
                    amount.get()
                )
            })),
    )
}

pub(super) fn controls_section() -> impl IntoElement {
    let wifi = signal(true);
    let subscribe = signal(false);
    let all = signal(2_usize);
    let plan = signal(SharedString::from("Standard"));
    let level = signal(60.0_f64);
    layout::section(
        "Controls",
        view()
            .flex_col()
            .gap_4()
            .child(controls::switch_row("Wi-Fi", wifi))
            .child(text::muted(move || {
                if wifi.get() {
                    "Wi-Fi is on".to_owned()
                } else {
                    "Wi-Fi is off".to_owned()
                }
            }))
            .child(controls::checkbox_row(
                "Subscribe to the changelog",
                subscribe,
            ))
            .child(controls::checkbox_tri_state("Select all", all, 2))
            .child(controls::radio_group_control(
                &["Basic", "Standard", "Pro"],
                plan,
            ))
            .child(controls::slider(level, 0.0, 100.0))
            .child(controls::progress(move || level.get() as f32))
            .child(controls::progress_indeterminate())
            .child(text::muted(move || {
                format!("Level {} of 100", level.get().round() as u32)
            }))
            .child(text::muted(move || {
                format!("{} of 2 selected, plan {}", all.get(), plan.get())
            })),
    )
}

pub(super) fn selection_section() -> impl IntoElement {
    let tab = signal(0_usize);
    let bold = signal(true);
    let italic = signal(false);
    layout::section(
        "Selection and toggles",
        view()
            .flex_col()
            .gap_4()
            .child(tabs::tab_bar(
                move || tab.get(),
                &["Overview", "Activity", "Logs"],
                move |at| tab.set(at),
            ))
            .child(text::muted(move || {
                format!("Showing {}", ["Overview", "Activity", "Logs"][tab.get()])
            }))
            .child(tabs::toggle_bar(vec![
                tabs::toggle("Bold", bold, move |next| bold.set(next)).into_element(),
                tabs::toggle("Italic", italic, move |next| italic.set(next)).into_element(),
            ]))
            .child(tabs::toolbar_bar(vec![
                button::icon_button("↺", "Undo", || ()).into_element(),
                button::icon_button("↻", "Redo", || ()).into_element(),
                tabs::toolbar_divider().into_element(),
                button::button_with(
                    "Share",
                    button::ButtonVariant::Ghost,
                    button::ButtonSize::Small,
                    false,
                    None,
                    || (),
                )
                .into_element(),
            ]))
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(avatar::avatar_image(no_picture, "Ada Lovelace", 40.0))
                    .child(avatar::avatar_group_of(
                        [
                            avatar::avatar_image(no_picture, "Grace Hopper", 32.0),
                            avatar::avatar_image(no_picture, "Alan Turing", 32.0),
                            avatar::avatar_image(no_picture, "Edsger Dijkstra", 32.0),
                        ],
                        2,
                    )),
            ),
    )
}

pub(super) fn nav_rows_section() -> impl IntoElement {
    let which = signal(SharedString::from("Home"));
    layout::section(
        "Navigation rows",
        view()
            .flex_col()
            .gap_2()
            .children([
                crate::ui::nav::nav_row(
                    "Home",
                    None,
                    computed(move || which.get().as_ref() == "Home"),
                    "nav-home",
                    move || which.set(SharedString::from("Home")),
                )
                .into_element(),
                crate::ui::nav::nav_row(
                    "Inbox",
                    Some(view().child(layout::badge("4", layout::BadgeTone::Primary))),
                    computed(move || which.get().as_ref() == "Inbox"),
                    "nav-inbox",
                    move || which.set(SharedString::from("Inbox")),
                )
                .into_element(),
                crate::ui::nav::nav_row(
                    "Reports",
                    Some(crate::ui::nav::chevron()),
                    computed(move || which.get().as_ref() == "Reports"),
                    "nav-reports",
                    move || which.set(SharedString::from("Reports")),
                )
                .into_element(),
                crate::ui::nav::nav_row(
                    "Archive",
                    Some(crate::ui::nav::chevron()),
                    false,
                    "nav-archive",
                    || (),
                )
                .disabled(true)
                .into_element(),
            ])
            .child(text::muted(move || format!("{} row selected", which.get())))
            .child(crate::ui::nav::list_row(
                "Quarterly report",
                "Ada Lovelace · 3 comments",
                "nav-record",
                move || which.set(SharedString::from("Record")),
            )),
    )
}

pub(super) fn pickers_section() -> impl IntoElement {
    let chosen = signal(Some(nsh_base::Date::Single(None)));
    let time = signal(SharedString::from("--:--"));
    let count = signal(Some(3.0_f64));
    let code = signal(SharedString::from(""));
    // The picker starts at the theme primary, resolved for the current
    // appearance: a picked color is data, not a style.
    let accent = signal(color(theme().primary));
    layout::section(
        "Pickers",
        view()
            .flex_col()
            .gap_4()
            .child(pickers::calendar_of(chosen))
            .child(pickers::date_picker_of("Start date", chosen, "Pick a date"))
            .child(field::field(
                "Start time",
                pickers::time_field_of("Start time", time),
            ))
            .child(field::field(
                "Seats",
                number::number_field("Seats", count, "1"),
            ))
            .child(field::field("Code", number::otp_field("Code", code, 6, 2)))
            .child(color::color_picker_of(accent, |_| ()))
            .child(
                view()
                    .flex_row()
                    .gap_2()
                    .child(color::color_swatch_of(color(theme().primary), true)),
            )
            .child(text::muted(move || {
                format!(
                    "Date {} time {} code {}",
                    chosen
                        .get()
                        .and_then(|date| date.start())
                        .map(|date| format!("{date}"))
                        .unwrap_or_else(|| "none".to_owned()),
                    time.get(),
                    code.get()
                )
            })),
    )
}
