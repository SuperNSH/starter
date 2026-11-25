//! What the shell does with the demo screens, through the real app on a
//! manual clock: a pushed screen, undo over it, a link that carries an id,
//! and a post opened from the feed on a phone. A new app deletes this file
//! with `src/showcase`.
//!
//! Seen to fail: `nav.rs` `push` taking no undo snapshot failed the first
//! test; `showcase/routes.rs` dropping the id from `DataItem`'s arm failed
//! the second; `nav::push_child` always answering `false` failed the third.

use std::time::{Duration, SystemTime};

use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

/// A phone-sized app on a manual clock.
fn phone() -> Headless {
    let mut config = EngineConfig::new(Root::of::<nsh_starter::App>().url_schemes(["nsh-starter"]));
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app
}

#[test]
fn undo_and_redo_retrace_navigation() {
    let mut app = phone();

    // Push a detail: its name is on screen (the page's own copy aside,
    // the data area owns that).
    app.press("id:page-data_item").unwrap();
    app.wait_until("the detail is on top", Duration::from_secs(5), |screen| {
        screen.count("text:Data item 3") >= Ok(1)
    })
    .unwrap();

    app.press_key("mod+z").unwrap();
    app.wait_until("undo popped the detail", Duration::from_secs(5), |screen| {
        screen.count("text:Data item 3") == Ok(0)
    })
    .unwrap();

    app.press_key("mod+shift+z").unwrap();
    app.wait_until(
        "redo pushed the detail again",
        Duration::from_secs(5),
        |screen| screen.count("text:Data item 3") >= Ok(1),
    )
    .unwrap();
}

#[test]
fn a_link_with_an_id_opens_that_item_over_its_parent() {
    let mut app = phone();
    app.open_with_url("nsh-starter://data/item/7").unwrap();
    app.wait_until("item 7 is on top", Duration::from_secs(5), |screen| {
        screen.count("text:Data item 7") >= Ok(1)
    })
    .unwrap();
    // Back lands on the parent the link brought with it, not on Home.
    app.back().unwrap();
    app.wait_for("id:feed-list").unwrap();
}

#[test]
fn a_post_opened_on_a_phone_is_pushed_as_its_own_screen() {
    let mut app = phone();
    app.press("id:page-data").unwrap();
    app.wait_for("text:Shipping a Rust shell").unwrap();
    app.press("text:Shipping a Rust shell").unwrap();
    app.wait_until(
        "the post's screen is on top",
        Duration::from_secs(5),
        |screen| screen.count("text:Data item 1") >= Ok(1),
    )
    .unwrap();
}
