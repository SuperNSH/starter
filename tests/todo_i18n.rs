//! The to-do page's i18n: every string the page renders reads from the
//! catalogs, proven by rendering the page under `zh-CN` — a key that fell
//! back to English would fail one of the waits. One test in its own file:
//! the locale is process-global, and this file's binary shares a process
//! with nothing that asserts English.
//!
//! Seen-to-fail record: `localized("todo.add.placeholder")` on the page
//! replaced with the literal "What needs doing?" — the first wait timed
//! out against the Chinese string.

use std::time::SystemTime;

use nsh_core::prelude::*;
use nsh_engine::{Clock, EngineConfig, Root};
use nsh_testing::{Headless, MemoryStorage};
use todo_sample::features::todo::page::todo_page;

/// The page as a root: what `Route::Home` mounts, alone in a view.
#[component]
fn Harness() -> impl IntoElement {
    view().flex_1().child(todo_page())
}

#[test]
fn the_page_renders_from_the_zh_cn_catalog() -> Result<(), String> {
    rust_i18n::set_locale("zh-CN");
    let mut app = Headless::start(EngineConfig {
        clock: Clock::manual(SystemTime::UNIX_EPOCH),
        storage: Some(MemoryStorage::new(vec![])),
        ..EngineConfig::new(Root::of::<Harness>())
    })
    .unwrap();
    app.first_render().unwrap();

    // The add field's placeholder and label, the empty list's state, the
    // count at zero — all from the zh-CN catalog.
    app.wait_for("text:要做点什么？")?;
    app.wait_for("还没有任务，在上方添加第一条吧。")?;
    app.wait_for("没有待办任务")?;

    // Adding through the Chinese page still works; the count pluralizes
    // in the catalog's own wording.
    app.type_text("id:todo-add", "买牛奶\n")?;
    app.wait_for("text:买牛奶")?;
    app.wait_for("1 个待办任务")?;

    // Everything done: the none count, and the Active filter's own empty
    // state (the chips' names for screen readers stay the literal ones).
    app.press("id:todo-toggle-1 >> role:checkbox")?;
    app.wait_for("没有待办任务")?;
    app.press("id:todo-filter-active >> label:Active")?;
    app.wait_for("没有未完成的任务，全部完成了。")?;

    // `rust_i18n::set_locale` is process-global: put it back.
    rust_i18n::set_locale("en");
    Ok(())
}
