//! The Settings page's tests, through the real app on a manual clock.
//!
//! Seen to fail: `name_card`'s save skipping `config::save` failed at the
//! stored name; `locale::text` not reading the locale signal failed at
//! the language switch.

use std::sync::Arc;
use std::time::SystemTime;

use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::{Headless, MemoryStorage};

fn open_settings(storage: Arc<MemoryStorage>) -> Headless {
    let config = EngineConfig {
        clock: Clock::manual(SystemTime::UNIX_EPOCH),
        storage: Some(storage),
        ..EngineConfig::new(Root::of::<nsh_starter::App>())
    };
    let mut app = Headless::start(config).unwrap();
    app.wait_for("text:Home").unwrap();
    app.press("id:page-settings").unwrap();
    app.wait_for("text:Language").unwrap();
    app
}

/// One test, in order: the language is process-global in `rust-i18n`, so
/// two tests switching it in parallel would read each other's language.
#[test]
fn the_name_saves_and_the_language_switches() {
    let storage = MemoryStorage::new([]);
    let mut app = open_settings(storage.clone());
    app.type_text("id:settings-name", "Ada").unwrap();
    app.press("role:button[Save]").unwrap();
    app.wait_for("text:Saved").unwrap();
    let stored = storage
        .entries()
        .get("app.config")
        .cloned()
        .unwrap_or_default();
    assert!(
        stored.contains(r#""display_name":"Ada""#),
        "stored: {stored}"
    );

    app.press("role:button[中文]").unwrap();
    app.wait_for("text:语言").unwrap();
    app.press("role:button[English]").unwrap();
    app.wait_for("text:Language").unwrap();
}
