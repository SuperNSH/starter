//! The Diagnostics page's tests, on a manual clock, through the real app:
//! the config migration, the event bus, the task manager and the central
//! error surface, each driven from the page's own buttons.
//!
//! Each test was seen to fail once by breaking the code on purpose; the
//! break used is named in the test's doc comment.

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use nsh_engine::{EngineConfig, Root};
use nsh_testing::{Headless, MemoryStorage};

/// The wall time every test's clock starts at: the Unix epoch, so ids and
/// timestamps are reproducible.
const START: SystemTime = SystemTime::UNIX_EPOCH;

/// Starts the app with `entries` preloaded into its storage and the clock
/// standing at [`START`].
fn app_with(entries: Arc<MemoryStorage>) -> Headless {
    let config = EngineConfig {
        clock: nsh_core::time::Clock::manual(START),
        storage: Some(entries.clone()),
        ..EngineConfig::new(Root::of::<nsh_starter::App>())
    };
    Headless::start(config).unwrap()
}

/// An app with nothing stored.
fn app() -> Headless {
    app_with(MemoryStorage::new([]))
}

/// Opens Diagnostics from its card on Home.
fn open_diagnostics(app: &mut Headless) {
    app.wait_for("text:Home").unwrap();
    app.press("id:page-diagnostics").unwrap();
    app.wait_for("text:/Capabilities/").unwrap();
}

/// The config migration roundtrip, through the Diagnostics page: a schema-0
/// document ("Ada", dark) migrates to the current schema, shows migrated on
/// screen, and is saved back so the stored form converges.
///
/// Break seen: `config::migrate` mapped `v0.dark` to `"system"` always —
/// the test failed at `wait_for("text:dark")`.
#[test]
fn config_migration_roundtrip() {
    let storage = MemoryStorage::new([(
        "app.config".to_owned(),
        r#"{"name":"Ada","dark":true}"#.to_owned(),
    )]);
    let mut app = app_with(storage.clone());
    app.wait_for("text:Home").unwrap();
    // The logging ring is process-global and parallel tests share it.
    // Clearing now bounds the read to this test's window, and the line
    // itself names this engine's saved config ("Ada") — no other test's
    // engine saves a config with that name, so only this engine's
    // subscription (`startup::install`) can produce the asserted string.
    nsh_starter::services::logging::clear_log();
    open_diagnostics(&mut app);

    // The migrated fields the config card shows.
    app.wait_for("text:Ada").unwrap();
    app.wait_for("text:dark").unwrap();
    app.wait_for("text:v1").unwrap();

    // The save-back travelled the bus: the events card shows the
    // `app.config` publish (its effect runs after the page's build, so the
    // log card's snapshot cannot hold the line).
    app.wait_for("text:app.config").unwrap();
    assert!(
        nsh_starter::services::logging::log_tail(200)
            .iter()
            .any(|line| line.message == "config saved (app.config) for Ada"),
        "the subscription did not log: {:?}",
        nsh_starter::services::logging::log_tail(200)
    );

    // The migration was saved back in the current schema.
    let stored = storage
        .entries()
        .get("app.config")
        .expect("config saved")
        .clone();
    assert!(stored.contains(r#""schema_version":1"#), "stored: {stored}");
    assert!(
        stored.contains(r#""display_name":"Ada""#),
        "stored: {stored}"
    );
    assert!(stored.contains(r#""theme":"dark""#), "stored: {stored}");
}

/// The schema-0 migrator is pure: every field lands where the v1 config
/// keeps it. (No UI, no clock; the UI roundtrip above proves load and
/// save.)
///
/// Break seen: `config::migrate` mapped `v0.dark` to `"dark"` always (the
/// opposite break of the roundtrip's) — the test failed on the
/// `theme == "system"` assertion.
#[test]
fn migrate_maps_schema_zero_fields() {
    let migrated = nsh_starter::store::config::migrate(r#"{"name":"Bo","dark":false}"#).unwrap();
    assert_eq!(migrated.schema_version, 1);
    assert_eq!(migrated.display_name, "Bo");
    assert_eq!(migrated.theme, "system");
    assert!(migrated.telemetry_enabled);
}

/// The event bus keeps publish order: three events published in one batch
/// appear on the Diagnostics page first, second, third.
///
/// Break seen: `publish_test_events` published `"third"`, `"second"`,
/// `"first"` — the test failed on the order assertion at `first < second`.
#[test]
fn event_bus_ordering() {
    let mut app = app();
    open_diagnostics(&mut app);

    app.press("role:button[Publish test events]").unwrap();
    app.wait_for("text:first").unwrap();

    let texts = app.screen().texts();
    let at = |needle: &str| {
        texts
            .iter()
            .position(|text| *text == needle)
            .unwrap_or_else(|| panic!("{needle} not on screen: {texts:?}"))
    };
    let (first, second, third) = (at("first"), at("second"), at("third"));
    assert!(first < second && second < third, "order: {texts:?}");
}

/// The task manager cancels a running task: its future stops, the state
/// reads Cancelled, and moving the clock afterwards brings no "done".
///
/// Break seen: `tasks::cancel` recorded Cancelled but
/// `std::mem::forget(spawned)` kept the future alive — the task ran to the
/// end, and the test failed on "task finished despite cancel".
#[test]
fn task_manager_cancel() {
    let mut app = app();
    open_diagnostics(&mut app);

    app.press("role:button[Start sample task]").unwrap();
    app.wait_for("text:/running/").unwrap();

    // Cancel while the first tick (one second out) is still pending.
    app.press("role:button[Cancel]").unwrap();
    app.wait_for("text:/cancelled/").unwrap();

    // The cancelled task stays cancelled no matter how far time moves.
    app.run_for(Duration::from_secs(6)).unwrap();
    // Task rows read "#<id> <state>"; event rows are history and are
    // allowed to mention anything.
    let done_rows = app.screen().count("text:/#[0-9]+ (done|running)/").unwrap();
    assert_eq!(done_rows, 0, "task finished despite cancel");
    let cancelled_rows = app.screen().count("text:/#[0-9]+ cancelled/").unwrap();
    assert!(cancelled_rows >= 1, "no cancelled row on screen");
}

/// A task that runs to its end reads Done on the bus's task events, and
/// Cancel all empties the running set (the drain-on-quit path).
///
/// Break seen: `tasks::cancel_all` took the live handles but
/// `std::mem::forget(spawned)` kept them running — the test failed on
/// "drain did not cancel both: 0".
#[test]
fn task_manager_runs_to_done_and_drains() {
    let mut app = app();
    open_diagnostics(&mut app);

    app.press("role:button[Start sample task]").unwrap();
    // Five ticks, one second apart, then the task ends by itself.
    app.run_for(Duration::from_secs(6)).unwrap();
    app.wait_for("text:done sample").unwrap();

    // Two samples running; Cancel all (drain on quit) ends both.
    app.press("role:button[Start sample task]").unwrap();
    app.press("role:button[Start sample task]").unwrap();
    app.press("role:button[Cancel all]").unwrap();
    app.run_for(Duration::from_secs(6)).unwrap();
    let running = app.screen().count("text:/#[0-9]+ running/").unwrap();
    assert_eq!(running, 0, "drain left a task running");
    let cancelled = app.screen().count("text:/#[0-9]+ cancelled/").unwrap();
    assert!(cancelled >= 2, "drain did not cancel both: {cancelled}");
}

/// The central error surface: a raised error shows with its detail, and
/// Dismiss clears it.
///
/// Break seen: `error_surface`'s `when` condition was hardcoded to
/// `false` — the banner never appeared, and the test failed at
/// `wait_for` of the message.
#[test]
fn error_surface_shows_and_dismisses() {
    let mut app = app();
    open_diagnostics(&mut app);

    app.press("role:button[Send test error]").unwrap();
    app.wait_for("text:Test error from Diagnostics. Dismiss it when read.")
        .unwrap();
    // The detail line rides along.
    app.wait_for("text:/raised to verify the central surface/")
        .unwrap();

    app.press("role:button[Dismiss]").unwrap();
    // The banner's detail line is unique to it: the events card keeps the
    // payload as history, so it cannot say whether the banner went.
    app.wait_for_gone(
        "text:/raised to verify the central surface/",
        Duration::from_secs(2),
    )
    .unwrap();

    // The same raise landed in the monitoring sink, so the real sink
    // receives everything a page raises.
    let recorded = nsh_starter::services::monitoring::sink().errors();
    assert_eq!(recorded.len(), 1, "monitoring saw the raise: {recorded:?}");
    assert_eq!(recorded[0].error.code, "diagnostic");
}
