//! The shell's headless tests: the deep-link table, the back order and
//! the palette on a manual clock, undo and redo, the two layouts and the
//! lifecycle watch. Pure tables first, then the real engine.
//!
//! Seen-to-fail record: every test below was seen failing once by
//! breaking the code on purpose, then restored. Breaks, one row per test
//! (a break may fail two tests):
//!
//! | Test | Break applied |
//! |---|---|
//! | `deep_links_parse_into_routes` | `routes.rs`: the `[]` arm answered `None` |
//! | `the_back_order_is_overlays_pages_then_the_exit_ask` | `back.rs`: `back_step`'s `depth > 0` arm returned `AskExit` |
//! | `back_closes_the_overlay_then_asks_to_exit` | the same `back_step` break; the overlay-first leg: the palette dialog's `on_cancel` closed nothing |
//! | `the_lifecycle_machine_follows_the_engine_events` | `lifecycle.rs`: `transition` mapped `BecameActive` to `Backgrounded` |
//! | `the_lifecycle_watch_follows_the_environment` | the same `transition` break |
//! | `undo_history_is_bounded` | `undo.rs`: `record` trimmed to `LIMIT + 10`, then to clearing `past` |
//! | `the_palette_opens_and_dispatches` | `palette.rs`: `dispatch` ignored action ids |
//! | `the_desktop_width_takes_the_sidebar` | `mod.rs`: `desktop()` thresholds 7000/9000 |
//! | `undo_history_is_bounded`, `the_stack_is_bounded` (lib) | `undo.rs`: `record` trimmed to `LIMIT + 10` (stack lengths, the one page left) |
//! | `the_home_key_is_stable` (lib) | `routes.rs`: `key` answered `"home "` |
//! | `undo_returns_to_the_recorded_state` (lib) | `undo.rs`: `record` pushed nothing |
//! | `the_stack_is_bounded` (lib) | the `LIMIT + 10` trim |
//! | `App.test.tsx` "the root mounts" | `status.rs`: the app's name replaced by `"shell"` |
//! | `App.test.tsx` "the shell navigates from Home" | `back.rs`: the `AskExit` arm opened no prompt |
//! | `smoke/pages.test.tsx` | `nav.rs`: `open` never applied the pages |

use std::time::{Duration, SystemTime};

use nsh_core::{
    geometry::{Points, Size},
    platform::Lifecycle,
    time::Clock,
};
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;
use todo_sample::shell::{
    back::{BackStep, back_step},
    lifecycle::{AppEvent, AppPhase, transition},
    routes::{Route, route_from_deep_link},
    undo::UndoHistory,
};

/// A phone-sized app on a manual clock, as a headless run behaves (the
/// engine reports its build's mobile OS).
fn phone() -> Headless {
    let mut config = EngineConfig::new(Root::of::<todo_sample::App>());
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    Headless::start(config).unwrap()
}

/// A desktop-width app on a manual clock: the sidebar shell.
fn desktop() -> Headless {
    let mut config = EngineConfig::new(Root::of::<todo_sample::App>());
    config.viewport = Size::new(Points(1280.0), Points(800.0));
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    Headless::start(config).unwrap()
}

#[test]
fn deep_links_parse_into_routes() {
    assert_eq!(route_from_deep_link(&[]), Some(Route::Home));
    assert_eq!(route_from_deep_link(&["home"]), Some(Route::Home));
    // The demo pages the app once had are the app's no-longer's.
    assert_eq!(route_from_deep_link(&["forms"]), None);
    assert_eq!(route_from_deep_link(&["data"]), None);
    assert_eq!(route_from_deep_link(&["nope"]), None);
}

#[test]
fn the_back_order_is_overlays_pages_then_the_exit_ask() {
    // The engine asks the newest modal before this listener runs; what
    // this table fixes is what the listener itself decides.
    assert_eq!(back_step(2, false), BackStep::PopPage);
    assert_eq!(back_step(1, false), BackStep::PopPage);
    assert_eq!(back_step(0, false), BackStep::AskExit);
    assert_eq!(back_step(0, true), BackStep::Exit);
    // A page popped disarms the exit ask: it is asked for again.
    assert_eq!(back_step(1, true), BackStep::PopPage);
}

#[test]
fn the_lifecycle_machine_follows_the_engine_events() {
    assert_eq!(
        transition(AppPhase::Starting, AppEvent::BecameActive),
        AppPhase::Active
    );
    assert_eq!(
        transition(AppPhase::Active, AppEvent::ResignedActive),
        AppPhase::Inactive
    );
    assert_eq!(
        transition(AppPhase::Inactive, AppEvent::EnteredBackground),
        AppPhase::Backgrounded
    );
    assert_eq!(
        transition(AppPhase::Backgrounded, AppEvent::BecameActive),
        AppPhase::Active
    );
    // A phase reported before any other is still the truth.
    assert_eq!(
        transition(AppPhase::Starting, AppEvent::EnteredBackground),
        AppPhase::Backgrounded
    );
    // The engine phases map onto the events.
    assert_eq!(
        AppEvent::of(Lifecycle::Background),
        AppEvent::EnteredBackground
    );
}

#[test]
fn undo_history_is_bounded() {
    let mut history = UndoHistory::default();
    assert_eq!(history.undo(Vec::new()), None, "nothing recorded yet");
    for depth in 0..60 {
        // One page now; the stack's length is what varies.
        history.record((0..depth).map(|_| Route::Home).collect());
    }
    // The oldest snapshots went; the limit is the newest fifty.
    let back = history.undo(Vec::new()).expect("a recorded state");
    assert_eq!(back.len(), 59);
    // Redo replays the state undo was given: the empty one.
    assert_eq!(history.redo(vec![Route::Home]), Some(Vec::new()));
    assert_eq!(
        history.redo(vec![Route::Home]),
        None,
        "the redo tail went on record"
    );
}

#[test]
fn back_closes_the_overlay_then_asks_to_exit() {
    let mut app = phone();
    app.wait_for("text:To-dos").unwrap();

    // The palette goes up over the page.
    app.press_key("mod+k").unwrap();
    app.wait_for("text:Actions").unwrap();

    // The overlay takes back first: the palette closes and the page
    // beneath stays (its add field still shows).
    app.back().unwrap();
    app.wait_until(
        "the palette closed over the intact page",
        Duration::from_secs(5),
        |screen| {
            screen.count("text:Actions") == Ok(0) && screen.count("text:What needs doing?") >= Ok(1)
        },
    )
    .unwrap();

    // Back at the page asks before the platform could exit the app.
    app.back().unwrap();
    app.wait_for("text:Exit todo-sample?").unwrap();
    app.press("role:button[Cancel]").unwrap();
    app.wait_for_gone("text:Exit todo-sample?", Duration::from_secs(5))
        .unwrap();

    // The ask comes again: canceling did not arm the exit.
    app.back().unwrap();
    app.wait_for("text:Exit todo-sample?").unwrap();
}

#[test]
fn the_palette_opens_and_dispatches() {
    let mut app = phone();
    app.wait_for("text:To-dos").unwrap();
    app.press_key("mod+k").unwrap();
    // One page, so the palette holds the shell's actions alone.
    app.wait_for("text:Actions").unwrap();
    // The headless keyboard does not focus the palette's search field
    // (auto_focus is unverified there), so the highlight's Return move is
    // not exercisable headlessly: the run chooses the row as a tap does.
    app.press("role:option[Toggle appearance]").unwrap();
    app.wait_until(
        "the action ran and the palette closed",
        Duration::from_secs(5),
        |screen| screen.count("text:Actions") == Ok(0),
    )
    .unwrap();
    // The page beneath is untouched: the to-do page's own field.
    app.wait_for("text:What needs doing?").unwrap();
}

#[test]
fn the_desktop_width_takes_the_sidebar() {
    let mut app = desktop();
    app.wait_for("text:A to-do list on Native Shell").unwrap();
    // The pane beside it shows the to-do page; the status bar under both.
    app.wait_for("text:What needs doing?").unwrap();
    app.wait_for("text:Active").unwrap();

    // The sidebar's one row names the page it opens.
    app.wait_for("id:nav-home").unwrap();

    let mut narrow = phone();
    narrow.wait_for("text:To-dos").unwrap();
    assert!(
        !narrow.shows("A to-do list on Native Shell"),
        "the phone shell has no sidebar"
    );
}

#[test]
fn the_lifecycle_watch_follows_the_environment() {
    // The status bar that shows the phase is the desktop shell's.
    let mut app = desktop();
    app.wait_for("text:Active").unwrap();
    // The environment handed in is the platform default's: the desktop
    // width is restated so the shell (and its status bar) stays.
    app.set_environment("the app backgrounds", |environment| {
        environment.window = Size::new(Points(1280.0), Points(800.0));
        environment.lifecycle = Lifecycle::Background;
    })
    .unwrap();
    app.wait_for("text:Backgrounded").unwrap();
    app.set_environment("the app activates", |environment| {
        environment.window = Size::new(Points(1280.0), Points(800.0));
        environment.lifecycle = Lifecycle::Active;
    })
    .unwrap();
    app.wait_for("text:Active").unwrap();
}
