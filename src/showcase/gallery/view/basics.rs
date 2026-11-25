//! The gallery's first sections: choices, the system's own controls,
//! buttons and text.

use super::*;

pub(super) fn choices_section() -> impl IntoElement {
    let fruit = signal(None::<SharedString>);
    let tags = signal(Vec::<SharedString>::new());
    layout::section(
        "Choices from a list",
        view()
            .flex_col()
            .gap_4()
            .child(field::field(
                "Fruit",
                select::select_field(
                    "Fruit",
                    vec![
                        select::choice("apple", "Apple"),
                        select::choice("kiwi", "Kiwi"),
                        select::choice("pear", "Pear"),
                    ],
                    fruit,
                    "Pick a fruit",
                ),
            ))
            .child(field::field(
                "Tags",
                select::combobox_field(
                    "Tags",
                    vec![
                        select::choice("rust", "Rust"),
                        select::choice("swift", "Swift"),
                        select::choice("kotlin", "Kotlin"),
                    ],
                    tags,
                    "Type to tag",
                ),
            ))
            .child(text::muted(move || {
                format!(
                    "Fruit {} tags {}",
                    fruit.get().unwrap_or_default(),
                    tags.get().join(", ")
                )
            })),
    )
}

/// The platform's own controls (`native::controls`): UIKit's on iOS, AppKit's
/// on macOS, the app's styled ones elsewhere. The app's accent tints them.
pub(super) fn native_section() -> impl IntoElement {
    use crate::native::controls::{
        NativeButtonStyle, is_native, native_button, native_segmented, native_switch,
    };
    let presses = signal(0_u32);
    let press = move || presses.update(|n| *n += 1);
    let airplane = signal(false);
    let range = signal(1_usize);
    layout::section(
        "Native controls",
        view()
            .flex_col()
            .gap_4()
            .child(text::muted(if is_native() {
                "Drawn by the system: its buttons, switch and segmented control, in the app's accent."
            } else {
                "The system draws these on iOS and macOS; here they are the app's own controls."
            }))
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .child(native_button(
                        "Continue",
                        NativeButtonStyle::Filled,
                        None,
                        press,
                    ))
                    .child(native_button(
                        "Send",
                        NativeButtonStyle::Tinted,
                        Some("square.and.arrow.up"),
                        press,
                    ))
                    .child(native_button(
                        "Later",
                        NativeButtonStyle::Gray,
                        None,
                        press,
                    ))
                    .child(native_button(
                        "Glass",
                        NativeButtonStyle::Glass,
                        Some("sparkles"),
                        press,
                    ))
                    .child(native_button(
                        "Details",
                        NativeButtonStyle::Plain,
                        Some("info.circle"),
                        press,
                    )),
            )
            .child(text::muted(move || {
                format!("Native presses: {}", presses.get())
            }))
            .child(native_switch("Airplane mode", airplane))
            .child(native_segmented(&["Day", "Week", "Month"], range))
            .child(text::muted(move || {
                let ranges = ["Day", "Week", "Month"];
                format!(
                    "Airplane mode {} · range {}",
                    if airplane.get() { "on" } else { "off" },
                    ranges.get(range.get()).unwrap_or(&"none")
                )
            })),
    )
}

pub(super) fn buttons_section() -> impl IntoElement {
    let pressed = signal(0_u32);
    let disabled = signal(true);
    layout::section(
        "Buttons",
        view()
            .flex_col()
            .gap_4()
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .child(button::button("Primary", || ()))
                    .child(button::button_with(
                        "Secondary",
                        button::ButtonVariant::Secondary,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        || (),
                    ))
                    .child(button::button_with(
                        "Outline",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        || (),
                    ))
                    .child(button::button_with(
                        "Ghost",
                        button::ButtonVariant::Ghost,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        || (),
                    ))
                    .child(button::button_with(
                        "Danger",
                        button::ButtonVariant::Danger,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        || (),
                    )),
            )
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_2()
                    .child(button::button_with(
                        "Small",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        || (),
                    ))
                    .child(button::button_with(
                        "Large",
                        button::ButtonVariant::Primary,
                        button::ButtonSize::Large,
                        false,
                        None,
                        || (),
                    ))
                    .child(button::icon_button("✚", "Add", || ()))
                    .child(button::icon_button("⋯", "More actions", || ()))
                    .child(button::icon_button("＋", "Disabled add", || ())),
            )
            .child(button::button_with(
                "Disabled",
                button::ButtonVariant::Primary,
                button::ButtonSize::Medium,
                true,
                None,
                || (),
            ))
            .child(
                view()
                    .flex_col()
                    .gap_2()
                    .child(button::button_with(
                        "Enable the delete below",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || disabled.set(false),
                    ))
                    .child(button::button_with(
                        "Delete account",
                        button::ButtonVariant::Danger,
                        button::ButtonSize::Medium,
                        move || disabled.get(),
                        None,
                        || (),
                    )),
            )
            .child(text::muted(move || {
                format!("Primary pressed {} times", pressed.get())
            })),
    )
}

pub(super) fn no_picture() -> Option<String> {
    None
}

pub(super) fn text_section() -> impl IntoElement {
    layout::section(
        "Text and marks",
        view()
            .flex_col()
            .gap_2()
            .child(text::title("Title"))
            .child(text::section_title("Section title"))
            .child(text::body("Body copy carries the work."))
            .child(text::muted("Secondary text: hints, captions, metadata."))
            .child(text::label("Field label"))
            .child(
                // Wraps: the code and five badges are wider than a phone.
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .items_center()
                    .child(text::code("nsh_base::button"))
                    .child(layout::badge("Default", layout::BadgeTone::Neutral))
                    .child(layout::badge("New", layout::BadgeTone::Success))
                    .child(layout::badge("Beta", layout::BadgeTone::Warning))
                    .child(layout::badge("Info", layout::BadgeTone::Info))
                    .child(layout::badge("Broken", layout::BadgeTone::Danger)),
            )
            .child(layout::divider())
            .child(layout::skeleton())
            .child(layout::skeleton_of(56.0)),
    )
}
