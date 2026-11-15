//! The to-do store: the task list as the engine's global signal, one JSON
//! document under one `Storage` key, saved whole on every change (the
//! config and settings pattern). A page mount re-reads the document, so a
//! fresh engine — a restart — picks up exactly what the last change left.

use nsh_core::prelude::*;

use crate::features::todo::model::{TodoFilter, TodoItem};

/// The storage key the task list lives under.
pub const ITEMS_KEY: &str = "todo.items";

/// The task list, the engine's one global copy. Unsaved reads never
/// happen: every change through this module saves.
pub fn items() -> Signal<Vec<TodoItem>> {
    global::<Vec<TodoItem>>()
}

/// Which tasks the page shows. Session state: a restart starts on All.
pub fn filter() -> Signal<TodoFilter> {
    global::<TodoFilter>()
}

/// Re-reads the stored list into the signal. Every mount calls this, so
/// the stored document stays the truth a fresh engine starts from; a
/// document that cannot parse starts clean (and is rewritten on the next
/// change).
pub fn load() {
    let stored =
        nsh_core::storage::Storage::get_item(ITEMS_KEY).and_then(
            |raw| match serde_json::from_str::<Vec<TodoItem>>(&raw) {
                Ok(items) => Some(items),
                Err(error) => {
                    crate::foundation::logging::log_error(&format!(
                        "the task list is unreadable, starting fresh: {error}"
                    ));
                    None
                }
            },
        );
    items().set(stored.unwrap_or_default());
}

/// Writes the list whole; a write that cannot land is logged, not raised:
/// the in-memory list stays the working truth for this session either way.
fn save() {
    let json = serde_json::to_string(&items().get_untracked());
    if let Ok(json) = json
        && let Err(error) = nsh_core::storage::Storage::set_item(ITEMS_KEY, json)
    {
        crate::foundation::logging::log_error(&format!("the task list did not save: {error}"));
    }
}

/// Adds a task from `title` (trimmed; whitespace alone adds nothing) and
/// answers its id.
pub fn add(title: &str) -> Option<u64> {
    let title = title.trim();
    if title.is_empty() {
        return None;
    }
    // One past the highest stored id: stable across restarts without a
    // counter of its own.
    let id = items()
        .get_untracked()
        .iter()
        .map(|item| item.id)
        .max()
        .unwrap_or(0)
        + 1;
    items().update(|items| {
        items.push(TodoItem {
            id,
            title: title.to_owned(),
            completed: false,
        });
    });
    save();
    Some(id)
}

/// Sets whether the task is done; a missing id is a no-op (it still saves:
/// the list is the truth, not the id).
pub fn set_completed(id: u64, completed: bool) {
    items().update(|items| {
        if let Some(item) = items.iter_mut().find(|item| item.id == id) {
            item.completed = completed;
        }
    });
    save();
}

/// Renames the task. A title that trims to nothing removes the task (the
/// TodoMVC convention); a missing id is a no-op.
pub fn rename(id: u64, title: &str) {
    let title = title.trim();
    if title.is_empty() {
        remove(id);
        return;
    }
    items().update(|items| {
        if let Some(item) = items.iter_mut().find(|item| item.id == id) {
            item.title = title.to_owned();
        }
    });
    save();
}

/// Removes the task.
pub fn remove(id: u64) {
    items().update(|items| items.retain(|item| item.id != id));
    save();
}

/// Removes every finished task at once (the summary rail's and the filter
/// row's clear action).
pub fn clear_completed() {
    items().update(|items| items.retain(|item| !item.completed));
    save();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stored_document_is_a_plain_list_of_id_title_completed() {
        let items = vec![TodoItem {
            id: 4,
            title: "Verify the restart".to_owned(),
            completed: false,
        }];
        let json = serde_json::to_string(&items).unwrap();
        assert_eq!(
            json,
            r#"[{"id":4,"title":"Verify the restart","completed":false}]"#
        );
        assert_eq!(serde_json::from_str::<Vec<TodoItem>>(&json).unwrap(), items);
    }
}
