//! The to-do list's headless tests: add by Enter, toggle, rename inline,
//! delete, the filters with their empty states, and the restart, through
//! the real page on a manual clock.
//!
//! Seen-to-fail record: every test below was seen failing once by
//! breaking the code on purpose, then restored. Breaks, one row per test
//! (a break may fail several tests):
//!
//! | Test | Break applied |
//! |---|---|
//! | `enter_adds_a_task_and_the_count_follows`, `whitespace_alone_adds_nothing` | `store.rs`: `add` answered `None` for every title |
//! | `toggling_completes_and_uncompletes`, `filters_show_each_view_with_its_own_empty_state` | `store.rs`: `set_completed` wrote nothing |
//! | `editing_renames_via_enter_and_an_emptied_field_removes` | `store.rs`: `rename` wrote nothing |
//! | `deleting_removes_the_task`, `the_list_survives_a_restart` | `store.rs`: `remove` retained every id |
//! | `a_seeded_list_renders_its_tasks`, `the_list_survives_a_restart` | `store.rs`: `load` set an empty list over the document |
//! | `an_empty_list_shows_its_empty_state` | `model.rs`: `TodoFilter::All` matched nothing (the empty state's `visible` under All) |

use std::sync::Arc;
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

fn start(storage: Arc<MemoryStorage>) -> Headless {
    let mut app = Headless::start(EngineConfig {
        clock: Clock::manual(SystemTime::UNIX_EPOCH),
        storage: Some(storage),
        ..EngineConfig::new(Root::of::<Harness>())
    })
    .unwrap();
    app.first_render().unwrap();
    app
}

/// A storage that already holds `titles`, done-ness paired with each.
fn seeded(tasks: &[(&str, bool)]) -> Arc<MemoryStorage> {
    let items: Vec<String> = tasks
        .iter()
        .enumerate()
        .map(|(at, (title, completed))| {
            format!(
                r#"{{"id":{},"title":"{title}","completed":{}}}"#,
                at + 1,
                completed
            )
        })
        .collect();
    MemoryStorage::new(vec![(
        "todo.items".to_owned(),
        format!("[{}]", items.join(",")),
    )])
}

/// Types `title` into the add field and presses Enter, as the page adds.
fn add(app: &mut Headless, title: &str) -> Result<(), String> {
    app.type_text("id:todo-add", &format!("{title}\n"))
}

#[test]
fn an_empty_list_shows_its_empty_state() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    app.wait_for("No tasks yet — add the first one above.")?;
    assert!(app.shows("No tasks left"));
    Ok(())
}

#[test]
fn enter_adds_a_task_and_the_count_follows() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "Wire the store")?;
    app.wait_for("text:Wire the store")?;
    assert!(app.shows("1 task left"));

    // The button commits the same way Enter does.
    app.type_text("id:todo-add", "Name the pages")?;
    app.press("id:todo-add-go >> label:Add task")?;
    app.wait_for("text:Name the pages")?;
    assert!(app.shows("2 tasks left"));
    Ok(())
}

#[test]
fn whitespace_alone_adds_nothing() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "   ")?;
    app.wait_for("No tasks yet — add the first one above.")?;
    assert!(app.shows("No tasks left"));
    Ok(())
}

#[test]
fn toggling_completes_and_uncompletes() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "First")?;
    add(&mut app, "Second")?;
    app.wait_for("text:Second")?;

    app.press("id:todo-toggle-1 >> role:checkbox")?;
    app.wait_for("1 task left")?;
    app.press("id:todo-toggle-2 >> role:checkbox")?;
    app.wait_for("No tasks left")?;
    // Uncomplete: the task comes back to the count.
    app.press("id:todo-toggle-1 >> role:checkbox")?;
    app.wait_for("1 task left")?;
    Ok(())
}

#[test]
fn editing_renames_via_enter_and_an_emptied_field_removes() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "First draft")?;
    app.wait_for("text:First draft")?;

    app.press("id:todo-item-1 >> label:Edit task")?;
    app.wait_for("id:todo-edit-1")?;
    app.clear("id:todo-edit-1")?;
    app.type_text("id:todo-edit-1", "Renamed\n")?;
    app.wait_for("text:Renamed")?;
    app.wait_for_gone("text:First draft", std::time::Duration::from_secs(1))?;

    // An edit that empties the field removes the task (TodoMVC's rule).
    app.press("id:todo-item-1 >> label:Edit task")?;
    app.wait_for("id:todo-edit-1")?;
    app.clear("id:todo-edit-1")?;
    app.type_text("id:todo-edit-1", "\n")?;
    app.wait_for("No tasks yet — add the first one above.")?;
    Ok(())
}

#[test]
fn deleting_removes_the_task() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "Keep me")?;
    add(&mut app, "Remove me")?;
    app.wait_for("text:Remove me")?;

    app.press("id:todo-delete-2 >> label:Delete task")?;
    app.wait_for_gone("text:Remove me", std::time::Duration::from_secs(1))?;
    app.wait_for("text:Keep me")?;
    assert!(app.shows("1 task left"));
    Ok(())
}

#[test]
fn filters_show_each_view_with_its_own_empty_state() -> Result<(), String> {
    let mut app = start(MemoryStorage::new(vec![]));
    add(&mut app, "Active one")?;
    add(&mut app, "Done one")?;
    app.wait_for("text:Done one")?;
    app.press("id:todo-toggle-2 >> role:checkbox")?;
    app.wait_for("1 task left")?;

    app.press("id:todo-filter-active >> label:Active")?;
    app.wait_for("text:Active one")?;
    app.wait_for_gone("text:Done one", std::time::Duration::from_secs(1))?;

    app.press("id:todo-filter-completed >> label:Completed")?;
    app.wait_for("text:Done one")?;
    app.wait_for_gone("text:Active one", std::time::Duration::from_secs(1))?;

    // Everything done, so Active has nothing of its own to say so.
    app.press("id:todo-filter-all >> label:All")?;
    app.press("id:todo-toggle-1 >> role:checkbox")?;
    app.press("id:todo-filter-active >> label:Active")?;
    app.wait_for("No active tasks — everything is done.")?;
    app.press("id:todo-filter-completed >> label:Completed")?;
    app.wait_for("text:Done one")?;
    app.press("id:todo-filter-all >> label:All")?;
    app.wait_for("text:Active one")?;
    Ok(())
}

#[test]
fn a_seeded_list_renders_its_tasks() -> Result<(), String> {
    let mut app = start(seeded(&[("Seeded task", false), ("Finished seed", true)]));
    app.wait_for("text:Seeded task")?;
    app.wait_for("text:Finished seed")?;
    assert!(app.shows("1 task left"));
    Ok(())
}

#[test]
fn the_list_survives_a_restart() -> Result<(), String> {
    let storage = MemoryStorage::new(vec![]);

    // One session: two tasks, one finished.
    let mut first = start(storage.clone());
    add(&mut first, "Kept active")?;
    add(&mut first, "Kept done")?;
    first.wait_for("text:Kept done")?;
    first.press("id:todo-toggle-2 >> role:checkbox")?;
    first.wait_for("1 task left")?;
    drop(first);

    // The next session starts from what the last one saved.
    let mut second = start(storage.clone());
    second.wait_for("text:Kept active")?;
    second.wait_for("text:Kept done")?;
    assert!(second.shows("1 task left"));

    // A change in the new session reaches the one after it.
    second.press("id:todo-delete-1 >> label:Delete task")?;
    second.wait_for_gone("text:Kept active", std::time::Duration::from_secs(1))?;
    drop(second);

    let mut third = start(storage);
    third.wait_for("text:Kept done")?;
    third.wait_for_gone("text:Kept active", std::time::Duration::from_secs(1))?;
    assert!(third.shows("No tasks left"));
    Ok(())
}
