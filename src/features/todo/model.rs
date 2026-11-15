//! The to-do model: a task, the three filters, and the pure logic over a
//! task list. Nothing here touches signals or storage.

use serde::{Deserialize, Serialize};

/// One task: a stable id, its title, and whether it is done.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    /// Unique and stable across restarts: one more than the highest id
    /// the stored list already holds.
    pub id: u64,
    /// The task, trimmed: never empty in the store.
    pub title: String,
    /// Whether the task is done.
    pub completed: bool,
}

/// Which tasks the list shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TodoFilter {
    /// Every task.
    #[default]
    All,
    /// Only the tasks left to do.
    Active,
    /// Only the finished tasks.
    Completed,
}

impl TodoFilter {
    /// The filters in the order the page offers them.
    pub const ALL: [TodoFilter; 3] = [TodoFilter::All, TodoFilter::Active, TodoFilter::Completed];

    /// The filter's chip label.
    pub fn label(self) -> &'static str {
        match self {
            TodoFilter::All => "All",
            TodoFilter::Active => "Active",
            TodoFilter::Completed => "Completed",
        }
    }

    /// The filter's stable key, for test ids.
    pub fn key(self) -> &'static str {
        match self {
            TodoFilter::All => "all",
            TodoFilter::Active => "active",
            TodoFilter::Completed => "completed",
        }
    }

    /// Whether `item` shows under this filter.
    pub fn matches(self, item: &TodoItem) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => !item.completed,
            TodoFilter::Completed => item.completed,
        }
    }
}

/// How many tasks are left to do.
pub fn remaining(items: &[TodoItem]) -> usize {
    items.iter().filter(|item| !item.completed).count()
}

/// How many tasks are done.
pub fn completed(items: &[TodoItem]) -> usize {
    items.iter().filter(|item| item.completed).count()
}

/// The tasks `filter` shows, in list order.
pub fn visible(items: &[TodoItem], filter: TodoFilter) -> Vec<&TodoItem> {
    items.iter().filter(|item| filter.matches(item)).collect()
}

/// The count as the page words it: "No tasks left", "1 task left",
/// "3 tasks left".
pub fn remaining_label(items: &[TodoItem]) -> String {
    match remaining(items) {
        0 => "No tasks left".to_owned(),
        1 => "1 task left".to_owned(),
        count => format!("{count} tasks left"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(id: u64, title: &str, completed: bool) -> TodoItem {
        TodoItem {
            id,
            title: title.to_owned(),
            completed,
        }
    }

    #[test]
    fn filters_match_by_completion() {
        let active = task(1, "Wire the store", false);
        let done = task(2, "Name the app", true);
        assert!(TodoFilter::All.matches(&active) && TodoFilter::All.matches(&done));
        assert!(TodoFilter::Active.matches(&active));
        assert!(!TodoFilter::Active.matches(&done));
        assert!(!TodoFilter::Completed.matches(&active));
        assert!(TodoFilter::Completed.matches(&done));
    }

    #[test]
    fn visible_keeps_list_order() {
        let items = vec![task(1, "a", false), task(2, "b", true), task(3, "c", false)];
        let ids = |shown: Vec<&TodoItem>| shown.iter().map(|item| item.id).collect::<Vec<_>>();
        assert_eq!(ids(visible(&items, TodoFilter::All)), vec![1, 2, 3]);
        assert_eq!(ids(visible(&items, TodoFilter::Active)), vec![1, 3]);
        assert_eq!(ids(visible(&items, TodoFilter::Completed)), vec![2]);
    }

    #[test]
    fn remaining_counts_the_undone() {
        assert_eq!(remaining(&[]), 0);
        assert_eq!(remaining(&[task(1, "a", false)]), 1);
        assert_eq!(
            remaining(&[task(1, "a", false), task(2, "b", true), task(3, "c", false)]),
            2
        );
    }

    #[test]
    fn completed_counts_the_done() {
        assert_eq!(completed(&[]), 0);
        assert_eq!(completed(&[task(1, "a", true)]), 1);
        assert_eq!(completed(&[task(1, "a", false), task(2, "b", true)]), 1);
        // The two counts add up to the list.
        let items = vec![task(1, "a", false), task(2, "b", true), task(3, "c", false)];
        assert_eq!(remaining(&items) + completed(&items), items.len());
    }

    #[test]
    fn the_remaining_label_words_its_counts() {
        assert_eq!(remaining_label(&[]), "No tasks left");
        assert_eq!(remaining_label(&[task(1, "a", false)]), "1 task left");
        assert_eq!(
            remaining_label(&[task(1, "a", true), task(2, "b", false), task(3, "c", false)]),
            "2 tasks left"
        );
    }
}
