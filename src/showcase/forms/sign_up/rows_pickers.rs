//! The sign-up rows that pick a value: phone, birth date, meeting time,
//! region, interests and plan.

use super::*;

pub(super) fn phone_row(phone: Field<SignUp, String>) -> View {
    text_field_row(
        "Phone",
        phone,
        input_shell(move || phone.error().is_some())
            .placeholder("+1 555 0100")
            .keyboard_type(KeyboardType::Phone)
            .auto_capitalize(AutoCapitalize::None)
            .auto_fill(AutoFill::Tel)
            .return_key_type(ReturnKeyType::Done)
            .test_id("signup-phone"),
    )
}

pub(super) fn date_row(birth_date: Field<SignUp, DateTime>) -> View {
    field_row(
        "Birth date",
        view()
            .test_id("signup-birth-date")
            .child(styled_date_picker(
                "Birth date",
                false,
                move || birth_date.value(),
                move |picked: &PickedDate| birth_date.set(picked.value),
                move || birth_text(birth_date.value()).to_string(),
            )),
        move || birth_date.error(),
    )
}

/// The picked date as the trigger shows it, or the prompt.
pub(super) fn birth_text(date: DateTime) -> SharedString {
    date.start()
        .map(|day| day.format("%Y/%m/%d").to_string())
        .unwrap_or_else(|| "Pick a date".to_owned())
        .into()
}

/// The time field wears the input frame itself (border, background and
/// segment states), as the ui pickers' own wrapper does.
pub(super) fn time_row(meeting_time: Field<SignUp, Option<TimeOfDay>>) -> View {
    field_row(
        "Meeting time",
        view().test_id("signup-time").child(
            time_field()
                .value(move || meeting_time.value())
                .on_change(move |time: &TimeOfDay| {
                    meeting_time.set(Some(*time));
                    meeting_time.blur();
                })
                .accessibility_label("Meeting time")
                .flex_row()
                .items_center()
                .min_h(px(44.0))
                .px_2()
                .gap_1()
                .rounded(move || theme().radius)
                .border_1()
                .border_color(move || theme().input)
                .bg(move || theme().background)
                .segment_class(
                    "min-w-7 px-1 py-1.5 rounded-sm text-sm text-popover-foreground text-center \
                     hover:bg-accent focus:bg-accent",
                )
                .separator_style(|s| s.text_sm().text_color(move || theme().muted_foreground)),
        ),
        move || meeting_time.error(),
    )
}

pub(super) fn region_row(region: Field<SignUp, Option<String>>) -> View {
    const REGIONS: [(&str, &str); 5] = [
        ("eu", "Europe"),
        ("na", "North America"),
        ("apac", "Asia-Pacific"),
        ("sa", "South America"),
        ("amea", "Africa & Middle East"),
    ];
    let title = |value: Option<String>| {
        value
            .and_then(|value| {
                REGIONS
                    .iter()
                    .find(|(key, _)| *key == value)
                    .map(|(_, title)| *title)
            })
            .unwrap_or("Pick a region")
    };
    field_row(
        "Region",
        view().test_id("signup-region").child(
            select()
                .items(
                    REGIONS
                        .iter()
                        .map(|(value, title)| SelectItem::new(*value, *title))
                        .collect::<Vec<SelectItem>>(),
                )
                .value(move || region.value().map(SharedString::from))
                .on_change(move |value: &Option<SharedString>| {
                    region.set(value.as_deref().map(str::to_owned));
                    region.blur();
                })
                .accessibility_label("Region")
                .offset(px(4.0))
                .popup_style(popup_surface)
                .backdrop_style(popup_backdrop)
                .item_class("rounded-md focus:bg-accent selected:font-medium disabled:opacity-50")
                .render_item(|item: &SelectEntry| {
                    view().px_2().py_1p5().child(
                        text(item.title.clone())
                            .text_sm()
                            .text_color(move || theme().popover_foreground),
                    )
                })
                .trigger(popup_trigger_frame(
                    move || title(region.value()).to_string(),
                    true,
                )),
        ),
        move || region.error(),
    )
}

pub(super) fn interests_row(interests: Field<SignUp, Vec<String>>) -> View {
    field_row(
        "Interests",
        view().test_id("signup-interests").child(
            combobox()
                .items(vec![
                    SelectItem::new("cycling", "Cycling"),
                    SelectItem::new("photography", "Photography"),
                    SelectItem::new("cooking", "Cooking"),
                    SelectItem::new("climbing", "Climbing"),
                    SelectItem::new("board-games", "Board games"),
                    SelectItem::new("astronomy", "Astronomy"),
                ])
                .multiple(true)
                .value(move || {
                    Some(
                        interests
                            .value()
                            .into_iter()
                            .map(SharedString::from)
                            .collect::<Vec<_>>(),
                    )
                })
                .on_change(move |values: &Vec<SharedString>| {
                    interests.set(values.iter().map(|value| value.to_string()).collect());
                    interests.blur();
                })
                .placeholder("Type to filter")
                .accessibility_label("Interests")
                .popup_style(|s| {
                    s.p_1()
                        .rounded_lg()
                        .border_1()
                        .border_color(move || theme().border)
                        .bg(move || theme().popover)
                        .shadow_md()
                })
                .item_class("rounded-md focus:bg-accent selected:font-medium disabled:opacity-50")
                .input_style(|s| {
                    s.h(px(44.0))
                        .px(px(12.0))
                        .text_size(px(15.0))
                        .rounded(move || theme().radius)
                        .border_1()
                        .border_color(move || theme().input)
                        .bg(move || theme().background)
                }),
        ),
        move || interests.error(),
    )
}

pub(super) fn plan_row(plan: Field<SignUp, Option<String>>) -> View {
    let option = |value: &'static str, label: &'static str| {
        let selected = move || plan.value().as_deref() == Some(value);
        radio()
            .checked(selected)
            .on_change(move |_: &bool| {
                plan.set(Some(value.to_owned()));
                plan.blur();
            })
            .accessibility_label(label)
            .flex_row()
            .items_center()
            .gap_3()
            .min_h(px(44.0))
            .child(
                view()
                    .size(px(20.0))
                    .rounded_full()
                    .border_2()
                    .border_color(move || {
                        if selected() {
                            theme().primary
                        } else {
                            theme().border
                        }
                    })
                    .items_center()
                    .justify_center()
                    .child(
                        view()
                            .size(px(8.0))
                            .rounded_full()
                            .style_when(selected, |s| s.bg(move || theme().primary)),
                    ),
            )
            .child(text(label).text_size(px(15.0)))
    };
    field_row(
        "Plan",
        radio_group()
            .flex_row()
            .gap_1()
            .child(option("free", "Free"))
            .child(option("pro", "Pro"))
            .child(option("team", "Team")),
        move || plan.error(),
    )
}
