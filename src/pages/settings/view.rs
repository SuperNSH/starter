//! The settings page's layout: one card per setting.

use nsh_core::prelude::*;

use crate::store::{appearance, config, errors, locale};
use crate::ui::{
    button::{ButtonSize, ButtonVariant, button_with, choice_button},
    controls, field, layout, text,
};

/// The settings page.
pub fn settings_page() -> impl IntoElement {
    scroll()
        .test_id("settings-scroll")
        .flex_1()
        .content_inset(layout::page_insets(0.0))
        .child(
            layout::page_body()
                .child(text::page_intro(
                    locale::text("settings.title"),
                    locale::text("settings.summary"),
                ))
                .child(name_card())
                .child(language_card())
                .child(appearance_card())
                .child(telemetry_card()),
        )
}

/// A titled card.
fn card(title: &'static str) -> View {
    layout::card().child(text::section_title(locale::text(title)))
}

/// The name the app calls the user, saved on the button.
fn name_card() -> impl IntoElement {
    let name = signal(config::load().display_name);
    let saved = signal(false);
    let save = move || {
        let mut app_config = config::load();
        app_config.display_name = name.get_untracked().trim().to_owned();
        match config::save(&app_config) {
            Ok(()) => saved.set(true),
            Err(error) => errors::report_error(error),
        }
    };
    // An edit after a save is unsaved again.
    effect(move || {
        name.with(|_| ());
        untracked(|| saved.set(false));
    });
    card("settings.name.title")
        .child(field::text_field(name, "").test_id("settings-name"))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(view().test_id("settings-save").child(button_with(
                    locale::text("settings.name.save"),
                    ButtonVariant::Primary,
                    ButtonSize::Medium,
                    false,
                    None,
                    save,
                )))
                .child(when(
                    move || saved.get(),
                    || text::muted(locale::text("settings.name.saved")).into_element(),
                )),
        )
}

/// The UI language: each choice named in its own language.
fn language_card() -> impl IntoElement {
    card("settings.language.title").child(view().flex_row().flex_wrap().gap_2().children(
        locale::CHOICES.map(|(code, name)| {
            choice_button(
                name,
                name,
                move || {
                    let current = locale::app_locale();
                    current == code || (current.is_empty() && code == "en")
                },
                false,
                move || locale::set_app_locale(code),
            )
        }),
    ))
}

/// Light or dark, flipped at once.
fn appearance_card() -> impl IntoElement {
    card("settings.appearance.title")
        .child(text::muted(move || {
            if dark() {
                locale::text("settings.appearance.dark")()
            } else {
                locale::text("settings.appearance.light")()
            }
        }))
        .child(
            view()
                .flex_row()
                .child(view().test_id("settings-appearance").child(button_with(
                    locale::text("settings.appearance.toggle"),
                    ButtonVariant::Outline,
                    ButtonSize::Medium,
                    false,
                    None,
                    appearance::toggle,
                ))),
        )
}

/// Whether analytics and monitoring may send, saved as it flips.
fn telemetry_card() -> impl IntoElement {
    let stored = config::load().telemetry_enabled;
    let enabled = signal(stored);
    effect(move || {
        let on = enabled.get();
        untracked(|| {
            let mut app_config = config::load();
            if app_config.telemetry_enabled != on {
                app_config.telemetry_enabled = on;
                if let Err(error) = config::save(&app_config) {
                    errors::report_error(error);
                }
            }
        });
    });
    card("settings.telemetry.title")
        .child(controls::switch_row_with(
            locale::text("settings.telemetry.label"),
            enabled,
            move |next| enabled.set(next),
            "Telemetry",
        ))
        .child(text::muted(locale::text("settings.telemetry.note")))
}
