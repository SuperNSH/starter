//! The diagnostics page: the app's config, its capability registry, the
//! background task list, the event bus's recent traffic and the log tail,
//! with the actions that prove each wiring works.

use std::time::Duration;

use nsh_base::theme;
use nsh_core::element::{AccessibilityRole, SharedString};
use nsh_core::prelude::*;

use crate::error::AppError;
use crate::services::{capabilities, logging, manifest, time};
use crate::store::event_bus::{self, BusEvent};
use crate::store::tasks::{self, TaskInfo};
use crate::store::{config, errors};
use crate::ui::button::{ButtonElement, ButtonSize, ButtonVariant, button_with};

/// How many log lines the page shows.
const TAIL_LINES: usize = 6;

/// The page's body without its `nav_page`, for a stack that frames its
/// pages itself (the shell): a page nested in a page draws nothing.
pub fn diagnostics_page() -> impl IntoElement {
    crate::ui::layout::page_scroll(0.0).child(
        crate::ui::layout::page_body()
            .child(crate::ui::text::page_intro(
                "Diagnostics",
                "Config, capabilities, tasks, events and the log.",
            ))
            .child(app_card())
            .child(config_card())
            .child(capabilities_card())
            .child(tasks_card())
            .child(events_card())
            .child(log_card()),
    )
}

/// One titled section of the page.
fn section(title: &'static str) -> impl IntoElement {
    text(title)
        .role(AccessibilityRole::Header)
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .letter_spacing(px(0.6))
        .text_color(move || theme().muted_foreground)
}

/// The card a section's rows sit in (`ui`'s surface).
fn card(children: impl IntoIterator<Item = impl IntoElement>) -> impl IntoElement {
    crate::ui::layout::card().children(children)
}

/// A label-on-the-left, value-on-the-right row.
fn row(
    label: impl Into<SharedString>,
    value: impl Fn() -> SharedString + 'static,
) -> impl IntoElement {
    view()
        .w_full()
        .flex_row()
        .items_start()
        .gap_3()
        .child(
            text(label.into())
                .w(px(120.0))
                .flex_shrink_0()
                .text_sm()
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(value)
                .flex_1()
                .text_sm()
                .number_of_lines(3)
                .text_color(move || theme().foreground),
        )
}

/// The app card: identity a person can quote in a bug report.
fn app_card() -> impl IntoElement {
    let (name, id, version) = manifest::info();
    card([
        section("App").into_element(),
        row("Name", move || name.clone()).into_element(),
        row("Id", move || id.clone()).into_element(),
        row("Version", move || version.clone()).into_element(),
    ])
}

/// The config card: what the typed config holds as the page opened.
fn config_card() -> impl IntoElement {
    let app_config = config::load();
    let schema = app_config.schema_version;
    let display_name = SharedString::from(app_config.display_name);
    let theme_setting = SharedString::from(app_config.theme);
    let telemetry = app_config.telemetry_enabled;
    card([
        section("Config").into_element(),
        row("Schema", move || format!("v{schema}").into()).into_element(),
        row("Display name", move || {
            if display_name.is_empty() {
                SharedString::from("(not set)")
            } else {
                display_name.clone()
            }
        })
        .into_element(),
        row("Theme", move || theme_setting.clone()).into_element(),
        row("Telemetry", move || {
            if telemetry {
                SharedString::from("Enabled")
            } else {
                SharedString::from("Disabled")
            }
        })
        .into_element(),
    ])
}

/// The capabilities card: what `nsh.json` declares.
fn capabilities_card() -> impl IntoElement {
    let caps = capabilities::registry();
    card(
        std::iter::once(section("Capabilities").into_element()).chain(caps.into_iter().map(
            |cap| {
                let name = cap.name.clone();
                let declared = cap.declared;
                view()
                    .w_full()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        text(move || name.clone())
                            .flex_1()
                            .text_sm()
                            .text_color(move || theme().foreground),
                    )
                    .child(
                        text(if declared { "Declared" } else { "Not declared" })
                            .text_xs()
                            // Undeclared is the safe default, not a fault:
                            // only a declared one draws the eye.
                            .text_color(move || {
                                if declared {
                                    crate::ui::tokens::success()
                                } else {
                                    theme().muted_foreground
                                }
                            }),
                    )
                    .into_element()
            },
        )),
    )
}

/// The tasks card: the live list and the actions that exercise it.
fn tasks_card() -> impl IntoElement {
    let tasks = tasks::tasks();
    card([
        section("Tasks").into_element(),
        each(
            move || tasks.get().0,
            |info: &TaskInfo| info.id,
            |info, _| {
                view()
                    .w_full()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        text(move || info.with(|t| t.name.clone()))
                            .flex_1()
                            .text_sm()
                            .number_of_lines(1)
                            .text_color(move || theme().foreground),
                    )
                    .child(
                        text(move || {
                            format!(
                                "#{} {}",
                                info.with(|t| t.id),
                                info.with(|t| t.state.as_str())
                            )
                        })
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                    )
                    .child(when(
                        move || info.with(|t| t.state == tasks::TaskState::Running),
                        move || {
                            let id = info.with(|t| t.id);
                            action("Cancel", move || tasks::cancel(id))
                        },
                    ))
                    .into_element()
            },
        )
        .into_element(),
        empty_tasks_note().into_element(),
        view()
            .flex_row()
            .gap_2()
            .child(action("Start sample task", start_sample_task))
            .child(action("Cancel all", || {
                tasks::cancel_all();
            }))
            .into_element(),
    ])
}

/// The line under an empty task list, so the section never reads as broken.
fn empty_tasks_note() -> impl IntoElement {
    let tasks = tasks::tasks();
    when(
        move || tasks.with(|tasks| tasks.0.is_empty()),
        || {
            text("No tasks yet")
                .text_sm()
                .text_color(move || theme().muted_foreground)
        },
    )
}

/// The sample task the card starts: five ticks a second apart, so a test
/// can cancel it mid-run and a person can watch it finish.
fn start_sample_task() {
    tasks::spawn_task("sample", async {
        for _ in 0..5 {
            timer(Duration::from_secs(1)).await;
            event_bus::publish(event_bus::TOPIC_TASK, "sample tick");
        }
    });
}

/// The events card: the bus's recent traffic, oldest first.
fn events_card() -> impl IntoElement {
    let events = event_bus::events();
    card([
        section("Events").into_element(),
        each(
            move || events.get().0,
            |event: &BusEvent| event.id,
            |event, _| {
                view()
                    .w_full()
                    .flex_row()
                    .items_start()
                    .gap_2()
                    .child(
                        text(move || event.with(|e| e.topic.clone()))
                            .text_sm()
                            .text_color(move || theme().foreground),
                    )
                    .child(
                        text(move || event.with(|e| e.payload.clone()))
                            .flex_1()
                            .text_sm()
                            .number_of_lines(1)
                            .text_color(move || theme().muted_foreground),
                    )
                    .child(
                        text(move || time::format_time(event.with(|e| e.at_ms)))
                            .text_xs()
                            .text_color(move || theme().muted_foreground),
                    )
                    .into_element()
            },
        )
        .into_element(),
        empty_events_note().into_element(),
        view()
            .flex_row()
            .gap_2()
            .child(action("Publish test events", publish_test_events))
            .child(action("Send test error", send_test_error))
            .into_element(),
    ])
}

/// Three events in one batch, so the ordering on screen is checkable.
fn publish_test_events() {
    for word in ["first", "second", "third"] {
        event_bus::publish("diagnostic.batch", word);
    }
}

/// Raises a real error through the central surface, so the wiring from
/// this page to whatever the root mounted is provable in the field.
fn send_test_error() {
    errors::report_error(
        AppError::new(
            "diagnostic",
            "Test error from Diagnostics. Dismiss it when read.",
        )
        .detail("raised to verify the central surface"),
    );
}

/// The note under an empty event list.
fn empty_events_note() -> impl IntoElement {
    let events = event_bus::events();
    when(
        move || events.with(|events| events.0.is_empty()),
        || {
            text("Nothing published yet")
                .text_sm()
                .text_color(move || theme().muted_foreground)
        },
    )
}

/// The log card: the tail, and clearing it.
fn log_card() -> impl IntoElement {
    // The tail is a snapshot the card refreshes when it builds and when
    // its own action runs; live-tailing the file is the log's own job.
    let tail = signal(Vec::<SharedString>::new());
    let refresh = move || {
        tail.set(
            logging::log_tail(TAIL_LINES)
                .iter()
                .map(|line| {
                    SharedString::from(format!("[{}] {}", line.level.as_str(), line.message))
                })
                .collect(),
        );
    };
    refresh();
    card([
        section("Log").into_element(),
        each(
            move || {
                tail.get()
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, SharedString)>>()
            },
            |at: &(usize, SharedString)| at.0,
            |line, _| {
                view().child(
                    text(move || line.with(|(_, text)| text.clone()))
                        .w_full()
                        .text_xs()
                        .number_of_lines(1)
                        .text_color(move || theme().muted_foreground),
                )
            },
        )
        .into_element(),
        when(
            move || tail.with(Vec::is_empty),
            || {
                text("The log is empty")
                    .text_sm()
                    .text_color(move || theme().muted_foreground)
            },
        )
        .into_element(),
        action("Clear log", move || {
            logging::clear_log();
            refresh();
        })
        .into_element(),
    ])
}

/// The page's quiet action button (`ui`'s outline, small): `name` pins the
/// accessible name so tests can tell "Cancel" from "Cancel all".
fn action(label: &'static str, on_press: impl Fn() + 'static) -> ButtonElement {
    button_with(
        label,
        ButtonVariant::Outline,
        ButtonSize::Small,
        false,
        Some(label),
        on_press,
    )
}
