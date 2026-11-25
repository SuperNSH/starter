//! The Items page's tests, through the real app on a manual clock, against
//! the local fake API. The harness puts the fake API's switches in a strip
//! under the app, so a test turns them as a person would.
//!
//! Seen to fail, one break per test (log:
//! `docs/evidence/project/template-screens/seen-to-fail.log`): `items/view.rs`
//! opening a row with `presented.set` only, never `nav::push_child`;
//! `items/view.rs` never showing the presented item; `items/view.rs`
//! never showing the error state; `fake_api::posts` ignoring the empty
//! switch; `queries::error_key` mapping `NotFound` to the generic failure.

use std::time::{Duration, SystemTime};

use nsh_core::platform::{Idiom, Os};
use nsh_core::prelude::*;
use nsh_engine::{Clock, EngineConfig, Environment, Points, Root, Size};
use nsh_starter::services::fake_api;
use nsh_starter::ui::button::{ButtonSize, ButtonVariant, button_with};
use nsh_testing::Headless;

/// The app under a strip of the fake API's switches.
#[component]
fn ItemsHarness() -> impl IntoElement {
    let switch = |label: &'static str, run: fn()| {
        button_with(
            label,
            ButtonVariant::Outline,
            ButtonSize::Small,
            false,
            None,
            run,
        )
    };
    // Under the app, not over it: the desktop shell lays its title bar
    // out from the window's top.
    view()
        .flex_1()
        .child(view().flex_1().child(nsh_starter::app()))
        .child(
            view()
                .flex_row()
                .gap_2()
                .p_2()
                .child(switch("Fail all", || fake_api::set_fail_every(1)))
                .child(switch("Fail none", || fake_api::set_fail_every(0)))
                .child(switch("Empty", || fake_api::set_empty(true))),
        )
}

/// A phone by default (a headless engine reports its build's mobile OS);
/// `desktop` is a macOS window at 1280 × 800, with the sidebar shell.
fn start(desktop: bool) -> Headless {
    let mut config = EngineConfig::new(Root::of::<ItemsHarness>().url_schemes(["nsh-starter"]));
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    if desktop {
        config.viewport = Size::new(Points(1280.0), Points(800.0));
        config.environment = Environment {
            os: Os::MacOs,
            idiom: Idiom::Desktop,
            ..Environment::default()
        };
    }
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app
}

/// A phone: the shell pushes the item as a screen of its own.
#[test]
fn a_row_opens_its_item_and_back_returns_to_the_list() {
    let mut app = start(false);
    app.press("id:page-items").unwrap();
    // The fake API answers after 100 ms of the manual clock: skeletons first.
    app.wait_for("id:items-loading").unwrap();
    app.wait_for("id:items-row-3").unwrap();
    assert!(app.shows("Signals over snapshots"));

    app.press("id:items-row-3").unwrap();
    app.wait_until("item 3 is on top", Duration::from_secs(5), |screen| {
        screen.count("text:Item 3") >= Ok(1)
    })
    .unwrap();
    app.wait_for("id:items_detail-body").unwrap();
    app.wait_for("text:By Alan Kay").unwrap();

    app.back().unwrap();
    app.wait_for_gone("id:items_detail-body", Duration::from_secs(5))
        .unwrap();
    app.wait_for("id:items-row-3").unwrap();
}

/// A desktop width: the list presents the item over itself, with its own
/// way back.
#[test]
fn on_a_wide_window_the_list_presents_the_item_itself() {
    let mut app = start(true);
    app.press("id:page-items").unwrap();
    app.wait_for("id:items-row-2").unwrap();
    app.press("id:items-row-2").unwrap();
    app.wait_for("id:items_detail-body").unwrap();
    app.wait_for("role:button[Back to items]").unwrap();
    app.press("role:button[Back to items]").unwrap();
    app.wait_for_gone("id:items_detail-body", Duration::from_secs(5))
        .unwrap();
    app.wait_for("id:items-row-2").unwrap();
}

#[test]
fn a_failed_load_shows_the_error_and_retry_loads_the_list() {
    let mut app = start(false);
    app.press("role:button[Fail all]").unwrap();
    app.press("id:page-items").unwrap();
    app.wait_for("text:Couldn't load the items").unwrap();
    assert_eq!(app.screen().count("id:items-row-1"), Ok(0));

    app.press("role:button[Fail none]").unwrap();
    app.press("role:button[Retry]").unwrap();
    app.wait_for("id:items-row-1").unwrap();
    app.wait_for_gone("text:Couldn't load the items", Duration::from_secs(1))
        .unwrap();
}

#[test]
fn an_empty_answer_shows_the_empty_state() {
    let mut app = start(false);
    app.press("role:button[Empty]").unwrap();
    app.press("id:page-items").unwrap();
    app.wait_for("id:items-empty").unwrap();
    assert!(app.shows("Nothing here yet"));
    assert_eq!(app.screen().count("id:items-row-1"), Ok(0));
}

#[test]
fn a_link_to_an_item_that_does_not_exist_says_so() {
    let mut app = start(false);
    app.open_with_url("nsh-starter://items/999").unwrap();
    app.wait_for("text:Nothing has that id.").unwrap();
}
