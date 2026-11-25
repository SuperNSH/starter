//! The sign-up rows that take a value directly: age, newsletter, color,
//! budget, the one-time code, the bio and the terms.

use super::*;

pub(super) fn age_row(age: Field<SignUp, f64>) -> View {
    field_row(
        "Age",
        view()
            .gap_1()
            .child(hint_line(move || {
                format!("{} years", age.value().round() as u32).into()
            }))
            .child(
                slider()
                    .value(move || Some(SliderValue::Single(age.value())))
                    .min(13.0)
                    .max(99.0)
                    .on_change(move |value: &SliderValue| {
                        age.set(value.start());
                        age.blur();
                    })
                    .accessibility_label("Age")
                    .track_class("h-6 justify-center")
                    .indicator_class("h-1.5 rounded-full bg-muted")
                    .range_class("rounded-full bg-primary")
                    .thumb_class("size-5 rounded-full border-2 border-primary bg-background")
                    .flex_1(),
            ),
        move || age.error(),
    )
}

pub(super) fn switch_row(newsletter: Field<SignUp, bool>) -> View {
    view()
        .flex_row()
        .items_center()
        .justify_between()
        .min_h(px(44.0))
        .child(text("Send me the newsletter").text_size(px(15.0)))
        .child(
            switch()
                .checked(move || newsletter.value())
                .on_change(move |on: &bool| {
                    newsletter.set(*on);
                    newsletter.blur();
                })
                .accessibility_label("Newsletter")
                .track_style(|s| {
                    s.w(px(44.0))
                        .p(px(2.0))
                        .rounded_full()
                        .bg(move || theme().muted_foreground.opacity(0.3))
                        .transition_colors()
                        .checked(|s| s.bg(move || theme().primary))
                })
                .thumb_style(|s| {
                    s.size(px(20.0))
                        .rounded_full()
                        .bg(move || theme().background)
                        .shadow_sm()
                        .transition_transform()
                        .checked(|s| s.translate_x(px(20.0)))
                }),
        )
}

pub(super) fn color_row(favorite_color: Field<SignUp, String>) -> View {
    // Swatch presets are the picker's data, not the app's look.
    const SWATCHES: [&str; 5] = ["#0ea5e9", "#22c55e", "#eab308", "#ef4444", "#a855f7"];
    field_row(
        "Favorite color",
        view()
            .gap_2()
            .child(
                color_picker()
                    .value(move || Hsla::parse_hex(&favorite_color.value()).ok())
                    .on_change(move |event: &nsh_base::ColorPickerEvent| {
                        if let nsh_base::ColorPickerEvent::Change(_, formatted) = event {
                            favorite_color.set(formatted.to_string());
                            favorite_color.blur();
                        }
                    })
                    .gap_2()
                    .child(
                        color_area()
                            .flex_1()
                            .h(px(120.0))
                            .accessibility_label("Color area")
                            .panel_class("rounded-md")
                            .thumb_class("size-4 rounded-full border-2 border-ring"),
                    )
                    .child(
                        color_slider()
                            .h(px(24.0))
                            .color_space(ColorSpace::Hsb)
                            .channel(ColorChannel::Hue)
                            .accessibility_label("Hue")
                            .track_class("h-3 rounded-full")
                            .thumb_class("size-5 rounded-full border-2 border-ring"),
                    ),
            )
            .child(view().flex_row().gap_2().children(SWATCHES.map(|hex| {
                color_swatch()
                    .color(Hsla::parse_hex(hex).ok())
                    .accessibility_label(hex)
                    .selected_style(|s| s.border_2().border_color(move || theme().foreground))
                    .size(px(28.0))
                    .rounded_full()
            })))
            .child(hint_line(move || favorite_color.value().into())),
        move || favorite_color.error(),
    )
}

pub(super) fn budget_row(budget: Field<SignUp, Option<f64>>) -> View {
    field_row(
        "Daily budget",
        view().test_id("signup-budget").child(
            number_input()
                .value(move || budget.value())
                .min(0.0)
                .max(10_000.0)
                .step(50.0)
                .placeholder("0")
                .controls_right(true)
                .on_change(move |value: &Option<f64>| {
                    budget.set(*value);
                    budget.blur();
                })
                .on_blur(move || budget.blur())
                .accessibility_label("Daily budget")
                .input_style(|s| {
                    s.h(px(44.0))
                        .px(px(12.0))
                        .text_size(px(15.0))
                        .text_color(move || theme().foreground)
                        .border_1()
                        .border_color(move || theme().input)
                        .rounded(move || theme().radius)
                        .bg(move || theme().background)
                }),
        ),
        move || budget.error(),
    )
}

pub(super) fn code_row(code: Field<SignUp, String>, code_check: Signal<String>) -> View {
    let check = code_check;
    let field = code;
    view()
        .gap_1()
        .child(field_row(
            "Verification code",
            view().test_id("signup-code").child(
                otp_input()
                    .length(6)
                    .groups(2)
                    .value(move || Some(SharedString::from(field.value())))
                    .on_change(move |value: &SharedString| field.set(value.to_string()))
                    .on_complete(move |_: &nsh_base::OtpInputCompleteEvent| {
                        // A complete code is done being typed: put the
                        // keyboard away, or it covers the next control.
                        dismiss_keyboard();
                        check.set("Checking the code…".into());
                        let entered = field.value_untracked();
                        spawn(async move {
                            timer(CODE_CHECK_LATENCY).await;
                            check.set(if entered == "000000" {
                                "That code was rejected".to_owned()
                            } else {
                                "Code looks good".to_owned()
                            });
                        });
                    })
                    .on_blur(move || field.blur())
                    .accessibility_label("Verification code")
                    .group_class("flex-row gap-2")
                    .cell_class(
                        "size-11 items-center justify-center rounded-md border border-input \
                         text-base focus:border-ring data-filled:border-primary",
                    ),
            ),
            move || code.error(),
        ))
        .child(hint_line(move || check.get().into()))
}

pub(super) fn bio_row(
    bio: Field<SignUp, String>,
    editor_api: ComponentRef<RichEditor>,
    initial: Option<Value>,
) -> View {
    let api = editor_api;
    let field = bio;
    field_row(
        "Short bio",
        view()
            .h(px(140.0))
            .p_2()
            .rounded(move || theme().radius)
            .border_1()
            .border_color(move || {
                if bio.error().is_some() {
                    theme().destructive
                } else {
                    theme().input
                }
            })
            .bg(move || theme().background)
            .test_id("signup-bio")
            .child(
                rich_editor()
                    .component_ref(&api)
                    .content(initial)
                    .placeholder("A few words about you")
                    .on_change(move |_: &RichEditorChange| {
                        if let Some(api) = api.api::<RichEditorApi>() {
                            field.set(api.doc_json().to_string());
                            field.blur();
                        }
                    })
                    .on_blur(move || field.blur())
                    .accessibility_label("Short bio")
                    .flex_1(),
            ),
        move || bio.error(),
    )
}

pub(super) fn terms_row(terms: Field<SignUp, bool>) -> View {
    view()
        .gap_1()
        .child(
            checkbox()
                .checked(move || terms.value())
                .on_change(move |checked: &bool| {
                    terms.set(*checked);
                    terms.blur();
                })
                .accessibility_label("Accept the terms")
                .flex_row()
                .items_center()
                .gap_3()
                .indicator_style(|s| {
                    s.size(px(20.0))
                        .items_center()
                        .justify_center()
                        .rounded(move || theme().radius)
                        .border_2()
                        .border_color(move || theme().border)
                        .checked(|s| {
                            s.bg(move || theme().primary)
                                .border_color(move || theme().primary)
                        })
                })
                .indicator(
                    text("✓")
                        .text_size(px(13.0))
                        .text_color(move || theme().primary_foreground),
                )
                .child(text("I accept the terms").text_size(px(15.0))),
        )
        .child(error_line(move || terms.error()))
}
