//! The Native vs app page's tests, on a manual clock. Headless has no
//! system controls, so each "system" side falls back to the app's own
//! control: what these tests fix is that both sides share one value.
//!
//! Seen to fail: `native_switch`'s fallback bound to a signal of its own
//! failed the second test at `on: false`.

use std::time::SystemTime;

use nsh_core::prelude::*;
use nsh_engine::{Clock, EngineConfig, Root};
use nsh_starter::showcase::native_vs_app::native_vs_app_page;
use nsh_testing::Headless;

/// The page as a root: what the shell mounts, alone in a view.
#[component]
fn PageRoot() -> impl IntoElement {
    view().flex_1().child(native_vs_app_page())
}

fn start() -> Headless {
    let mut app = Headless::start(EngineConfig {
        clock: Clock::manual(SystemTime::UNIX_EPOCH),
        ..EngineConfig::new(Root::of::<PageRoot>())
    })
    .unwrap();
    app.wait_for("text:Liquid Glass").unwrap();
    app
}

#[test]
fn every_comparison_shows_both_sides() {
    let app = start();
    for title in ["Switch", "Checkbox", "Radio group", "Stepper", "Calendar"] {
        assert!(app.screen().shows(title), "{title}");
    }
    // One "App" and one "System" column per comparison.
    let apps = app.screen().count("text:App").unwrap();
    let systems = app.screen().count("text:System").unwrap();
    assert!(apps >= 10, "App columns: {apps}");
    assert_eq!(apps, systems);
}

#[test]
fn both_sides_share_one_value() {
    let mut app = start();
    app.wait_for("text:on: true").unwrap();
    // Two switches named Wi-Fi: the app's, and the system side's fallback.
    assert_eq!(app.screen().count("role:switch[Wi-Fi]").unwrap(), 2);
    // The second is the system side: pressing it moves the shared value.
    app.press("role:switch[Wi-Fi] >> nth=1").unwrap();
    app.wait_for("text:on: false").unwrap();
    app.press("role:switch[Wi-Fi] >> nth=0").unwrap();
    app.wait_for("text:on: true").unwrap();
}
