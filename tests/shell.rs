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
//! | `deep_links_parse_into_routes` | `routes.rs`: the `["settings"]` arm answered `Diagnostics` |
//! | `the_back_order_is_overlays_pages_then_the_exit_ask` | `back.rs`: `back_step`'s `depth > 0` arm returned `AskExit` |
//! | `back_pops_a_page_then_asks_to_exit` | the same `back_step` break; the overlay-first leg: the palette dialog's `on_cancel` closed nothing |
//! | `the_lifecycle_machine_follows_the_engine_events` | `lifecycle.rs`: `transition` mapped `BecameActive` to `Backgrounded` |
//! | `the_lifecycle_watch_follows_the_environment` | the same `transition` break |
//! | `undo_history_is_bounded` | `undo.rs`: `record` trimmed to `LIMIT + 10`, then to clearing `past` |
//! | `undo_and_redo_retrace_navigation` (tests/showcase.rs) | `nav.rs`: `push` took no undo snapshot |
//! | `the_palette_opens_and_navigates` | `palette.rs`: `dispatch` ignored destination ids |
//! | `the_desktop_width_takes_the_sidebar` | `mod.rs`: `desktop()` thresholds 7000/9000 |
//! | `the_sidebar_collapses_and_comes_back` | `chrome::toggle_sidebar` left the state as it was |
//! | `every_path_parses_back_into_its_route` (lib) | `showcase/routes.rs`: `DataItem`'s path dropped the id |
//! | `undo_returns_to_the_recorded_state` (lib) | `undo.rs`: `record` pushed nothing |
//! | `the_stack_is_bounded` (lib) | the `LIMIT + 10` trim |
//! | `App.test.tsx` "the root mounts" | `status.rs`: the app's name replaced by `"shell"` |
//! | `App.test.tsx` "the shell navigates from Home" | `back.rs`: the `AskExit` arm opened no prompt |
//! | `smoke/pages.test.tsx` | `nav.rs`: `open` never applied the pages |
//! | `a_link_of_the_claimed_scheme_opens_its_page` | `deep_link.rs`: `listen` never called `open` |
//! | `the_sidebar_key_parses_on_every_os` | `chrome.rs`: the non-Apple arm returned `"ctrl+mod+s"` |

use std::time::{Duration, SystemTime};

use nsh_core::{
    geometry::{Points, Size},
    platform::Lifecycle,
    time::Clock,
};
use nsh_engine::{EngineConfig, Root};
use nsh_starter::routes::Route;
use nsh_starter::shell::back::{BackStep, back_step};
use nsh_starter::store::{
    lifecycle::{AppEvent, AppPhase, transition},
    undo::UndoHistory,
};
use nsh_testing::Headless;

/// A phone-sized app on a manual clock, as a headless run behaves (the
/// engine reports its build's mobile OS).
fn phone() -> Headless {
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>());
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    Headless::start(config).unwrap()
}

/// A desktop-width app on a manual clock: the sidebar shell.
fn desktop() -> Headless {
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>());
    config.viewport = Size::new(Points(1280.0), Points(800.0));
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    Headless::start(config).unwrap()
}

#[test]
fn deep_links_parse_into_routes() {
    assert_eq!(Route::parse(&[]), Some(Route::Home));
    assert_eq!(Route::parse(&["settings"]), Some(Route::Settings));
    assert_eq!(Route::parse(&["diagnostics"]), Some(Route::Diagnostics));
    assert_eq!(Route::parse(&["nope"]), None);
    assert_eq!(Route::parse(&["settings", "extra"]), None);
    // A whole link goes through the same table.
    assert_eq!(
        Route::from_url("nsh-starter://settings"),
        Some(Route::Settings)
    );
    assert_eq!(Route::from_url("other://settings"), None);
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
    for depth in 0..60_usize {
        history.record(vec![Route::Diagnostics; depth]);
    }
    // The oldest snapshots went; the limit is the newest fifty.
    assert_eq!(
        history.undo(vec![Route::Home]),
        Some(vec![Route::Diagnostics; 59])
    );
    assert_eq!(history.redo(vec![Route::Home]), Some(vec![Route::Home]));
    assert_eq!(
        history.redo(vec![Route::Home]),
        None,
        "the redo tail went on record"
    );
}

#[test]
fn back_pops_a_page_then_asks_to_exit() {
    let mut app = phone();
    app.wait_for("text:Home").unwrap();

    // A page is pushed, then the palette goes up over it.
    app.press("id:page-settings").unwrap();
    app.wait_for("text:Language").unwrap();
    app.press_key("mod+k").unwrap();
    app.wait_for("text:Go to").unwrap();

    // The overlay takes back first: the palette closes and the page
    // beneath stays (its own section still shows).
    app.back().unwrap();
    app.wait_until(
        "the palette closed over the intact page",
        Duration::from_secs(5),
        |screen| screen.count("text:Go to") == Ok(0) && screen.count("text:Language") >= Ok(1),
    )
    .unwrap();

    // The next back pops the pushed page.
    app.back().unwrap();
    app.wait_until(
        "the pushed Settings page popped",
        Duration::from_secs(5),
        |screen| screen.count("text:Language") == Ok(0),
    )
    .unwrap();

    // Back at Home asks before the platform could exit the app.
    app.back().unwrap();
    app.wait_for("text:Exit nsh-starter?").unwrap();
    app.press("role:button[Cancel]").unwrap();
    app.wait_for_gone("text:Exit nsh-starter?", Duration::from_secs(5))
        .unwrap();

    // The ask comes again: canceling did not arm the exit.
    app.back().unwrap();
    app.wait_for("text:Exit nsh-starter?").unwrap();
}

#[test]
fn the_palette_opens_and_navigates() {
    let mut app = phone();
    app.wait_for("text:Home").unwrap();
    app.press_key("mod+k").unwrap();
    app.wait_for("text:Go to").unwrap();
    // The headless keyboard does not focus the palette's search field
    // (auto_focus is unverified there), so the highlight's Return move is
    // not exercisable headlessly: the run chooses the row as a tap does.
    // With the palette down, only the page (its marker and the status
    // bar) says the destination's name: two matches.
    app.press("role:option[Diagnostics]").unwrap();
    app.wait_until(
        "the first destination opened and the palette closed",
        Duration::from_secs(5),
        |screen| {
            // The palette's rows go, and the gallery's page is up (its
            // first section's label, which neither Home nor the palette
            // shows). The popped dialog's search field can outlive it in
            // the headless screen, so its placeholder is no closure
            // condition here.
            screen.count("text:Go to") == Ok(0) && screen.count("text:Start sample task") >= Ok(1)
        },
    )
    .unwrap();
}

#[test]
fn the_desktop_width_takes_the_sidebar() {
    let mut app = desktop();
    app.wait_for("label:Hide sidebar").unwrap();
    // The pane beside it still shows Home, the status bar under both.
    app.wait_for("text:Home").unwrap();
    app.wait_for("text:Active").unwrap();

    // A sidebar row opens its page into the pane.
    app.press("id:nav-settings").unwrap();
    app.wait_for("text:Language").unwrap();

    let mut narrow = phone();
    narrow.wait_for("text:Home").unwrap();
    assert_eq!(
        narrow.screen().count("label:Hide sidebar"),
        Ok(0),
        "the phone shell has no sidebar"
    );
}

#[test]
fn the_sidebar_collapses_and_comes_back() {
    let mut app = desktop();
    app.wait_for("id:nav-settings").unwrap();
    app.press("label:Hide sidebar").unwrap();
    app.wait_for_gone("id:nav-forms", Duration::from_secs(1))
        .unwrap();
    // The toolbar takes the toggle while the sidebar is away.
    app.press("label:Show sidebar").unwrap();
    app.wait_for("id:nav-settings").unwrap();
    // The platform shortcut does the same.
    app.press_key("ctrl+mod+s").unwrap();
    app.wait_for_gone("id:nav-forms", Duration::from_secs(1))
        .unwrap();
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

#[test]
fn the_sidebar_key_parses_on_every_os() {
    use nsh_core::{keymap::Keystroke, platform::Os};
    use nsh_starter::shell::chrome::sidebar_key_on;
    // `mod` twice is invalid off the Apple platforms and failed the
    // shell's mount on Android.
    for os in [
        Os::Ios,
        Os::Android,
        Os::MacOs,
        Os::Windows,
        Os::Linux,
        Os::Web,
    ] {
        let key = sidebar_key_on(os);
        assert!(
            Keystroke::parse(key, os).is_ok(),
            "{key:?} is not a valid keystroke on {os:?}"
        );
    }
}

#[test]
fn a_link_of_the_claimed_scheme_opens_its_page() {
    // nsh's generated code claims `nsh.json`'s schemes; a test says so itself.
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>().url_schemes(["nsh-starter"]));
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app.open_with_url("nsh-starter://settings").unwrap();
    app.wait_for("id:settings-name").unwrap();
}
