//! The first-run flow's test, on a manual clock. The app does not show
//! the flow yet (`src/shell/first_run.rs` says how to turn it on), so the
//! test mounts it behind the same gate a root would use.

use std::sync::Arc;
use std::time::SystemTime;

use nsh_base::{nav_page, nav_stack};
use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_starter::shell::first_run;
use nsh_testing::{Headless, MemoryStorage};

/// What the gate shows.
#[derive(Clone, Debug, PartialEq)]
enum Step {
    Welcome,
    Past,
}

/// The gate: the welcome page until the flow completes, then what is past
/// it. A `nav_page` lays out inside a `nav_stack`, so each is a route.
#[component]
fn Gate() -> impl IntoElement {
    let steps = signal(vec![if first_run::is_done() {
        Step::Past
    } else {
        Step::Welcome
    }]);
    nav_stack().flex_1().child(each(
        move || steps.get(),
        |step: &Step| format!("{step:?}"),
        move |step, _| match step.get_untracked() {
            Step::Welcome => first_run::first_run_page(move || steps.set(vec![Step::Past])),
            Step::Past => nav_page()
                .title("Past")
                .child(view().flex_1().child(text("Past the welcome"))),
        },
    ))
}

fn app_with(entries: Arc<MemoryStorage>) -> Headless {
    let config = EngineConfig {
        clock: nsh_core::time::Clock::manual(SystemTime::UNIX_EPOCH),
        storage: Some(entries.clone()),
        ..EngineConfig::new(Root::of::<Gate>())
    };
    Headless::start(config).unwrap()
}

/// The first-run flow shows once, persists completion, and never shows
/// again on the next start.
///
/// Break seen: `first_run::complete` published the bus event but skipped
/// `Storage::set_item` (`let _ = marker`) — the restarted app showed the
/// flow again, and the test failed at the marker assertion.
#[test]
fn first_run_once_only() {
    let storage = MemoryStorage::new([]);
    let mut app = app_with(storage.clone());
    app.wait_for("text:Welcome to nsh-starter").unwrap();

    // "Bo", not the roundtrip test's "Ada": each test's engine logs its
    // own saved name, so the process-global ring never cross-matches.
    app.type_text("id:first-run-name", "Bo").unwrap();
    app.press("role:button[Get started]").unwrap();
    app.wait_for("text:Past the welcome").unwrap();

    let stored = storage.entries();
    // The completion marker is an id (`services::id::new_id`): on the
    // manual clock its timestamp part is the clock's, so it reads back
    // exactly.
    let marker = stored
        .get("app.first_run_done")
        .expect("first-run marker saved");
    assert!(marker.starts_with("first-run-"), "marker: {marker}");
    let config = stored.get("app.config").expect("config saved").clone();
    assert!(
        config.contains(r#""display_name":"Bo""#),
        "stored: {config}"
    );

    // The next start skips the flow.
    let mut again = app_with(storage.clone());
    again.wait_for("text:Past the welcome").unwrap();
    assert!(
        !again.screen().shows("Welcome to nsh-starter"),
        "first-run came back"
    );
}
