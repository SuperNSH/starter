//! The first-run flow: a welcome page shown once, then never again
//! (persisted in Storage), which asks for a display name and writes it
//! into the typed config.

use nsh_base::{nav_page, theme};
use nsh_core::element::Screen;
use nsh_core::prelude::*;
use nsh_core::storage::Storage;

use crate::store::event_bus;

use crate::error::AppError;
use crate::store::config::{self, AppConfig};
use crate::store::errors;

/// The storage key marking the flow as completed.
pub const FIRST_RUN_KEY: &str = "app.first_run_done";

/// Whether the flow has completed ever (the shell checks this before
/// showing the page; the read is synchronous and cheap).
pub fn is_done() -> bool {
    Storage::get_item(FIRST_RUN_KEY).is_some()
}

/// Persists completion. Idempotent; announces `app.first-run` on the bus.
pub fn complete() -> Result<(), AppError> {
    let marker = crate::services::id::new_id("first-run");
    Storage::set_item(FIRST_RUN_KEY, marker.as_ref())?;
    event_bus::publish(event_bus::TOPIC_FIRST_RUN, "completed");
    Ok(())
}

/// The first-run page: a short welcome, a display-name field, and one
/// button. `on_complete` runs after completion persisted, so the caller
/// can swap the page out; the page itself never decides what shows.
pub fn first_run_page(on_complete: impl Fn() + 'static) -> Screen {
    let name = signal(String::new());
    let get_started = move || {
        let display_name = name.get_untracked().trim().to_owned();
        // Persist synchronously: Storage is in memory on the engine
        // thread, and the flow must not half-finish.
        let outcome = (|| -> Result<(), AppError> {
            let mut app_config = AppConfig::new();
            if !display_name.is_empty() {
                app_config.display_name = display_name;
            }
            config::save(&app_config)?;
            complete()
        })();
        match outcome {
            Ok(()) => on_complete(),
            Err(err) => errors::report_error(err),
        }
    };
    nav_page().title("Welcome").header(header("Welcome")).child(
        scroll().flex_1().child(
            view()
                .flex_1()
                .p_safe_area()
                .p_6()
                .justify_center()
                .gap_6()
                .child(
                    view()
                        .gap_2()
                        .child(
                            text("Welcome to nsh-starter")
                                .text_xl()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(move || theme().foreground),
                        )
                        .child(
                            text(
                                "A starting point for a real app: navigation, forms, data, \
                                     and the platform underneath. You can change everything.",
                            )
                            .text_sm()
                            .text_color(move || theme().muted_foreground),
                        ),
                )
                .child(
                    view()
                        .gap_2()
                        .child(
                            text("What should we call you?")
                                .text_sm()
                                .text_color(move || theme().foreground),
                        )
                        .child(name_field(name)),
                )
                .child(crate::ui::button::button("Get started", get_started)),
        ),
    )
}

/// The display-name field over `ui`'s frame; the flow's own bits are the
/// test id and the capitalized words a name wants.
fn name_field(name: Signal<String>) -> Input {
    crate::ui::field::text_field(name, "Your name")
        .test_id("first-run-name")
        .auto_capitalize(AutoCapitalize::Words)
}

/// The page's header on platforms without a system one (desktops and the
/// web); iOS and Android draw theirs from the page's title.
fn header(title: &'static str) -> impl IntoElement {
    view()
        .h(px(52.0))
        .flex_row()
        .items_center()
        .px_4()
        .border_b_1()
        .border_color(move || theme().border)
        .bg(move || theme().background)
        .child(
            text(title)
                .role(nsh_core::element::AccessibilityRole::Header)
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || theme().foreground),
        )
}
