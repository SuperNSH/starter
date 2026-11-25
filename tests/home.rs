//! Home's tests, through the real app on a manual clock: every page has a
//! card, a card opens its page, and the rows under the cards work.
//!
//! Seen to fail: `home/view.rs` skipping the last page of the list failed
//! the first test; `nav_tile` ignoring its press failed the second.

use std::time::SystemTime;

use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root};
use nsh_starter::pages;
use nsh_starter::routes::Route;
use nsh_testing::Headless;

fn app() -> Headless {
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>());
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app
}

#[test]
fn every_page_has_a_card_on_home() {
    let app = app();
    for route in pages::sidebar() {
        if *route == Route::Home {
            continue;
        }
        let page = route.page();
        let card = format!("id:page-{}", page.key);
        assert_eq!(app.screen().count(&card).unwrap(), 1, "{card}");
        assert!(app.screen().shows(page.summary), "{}", page.summary);
    }
    // Home has no card for itself.
    assert_eq!(app.screen().count("id:page-home").unwrap(), 0);
}

#[test]
fn a_card_opens_its_page_and_back_returns() {
    let mut app = app();
    app.press("id:page-diagnostics").unwrap();
    app.wait_for("text:/Capabilities/").unwrap();
    app.back().unwrap();
    app.wait_for("text:Everything the boilerplate demonstrates, one page each.")
        .unwrap();
}

#[test]
fn the_rows_under_the_cards_are_listed() {
    let app = app();
    for (id, label, _) in pages::HOME_ROWS {
        assert!(app.screen().shows(label), "{label}");
        assert_eq!(app.screen().count(&format!("id:{id}")).unwrap(), 1, "{id}");
    }
}
