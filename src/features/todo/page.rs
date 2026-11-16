//! The to-do page: the add row, the filter chips with the remaining
//! count, and the list itself — one column at phone widths, with the
//! summary rail beside it once a window is wide enough to hold both. One
//! screen, no stack of its own; the shell mounts it under `Route::Home`'s
//! header.

use nsh_base::theme;
use nsh_core::{
    element::{AnyElement, KeyboardDismissMode, KeyboardTaps},
    prelude::*,
};

use crate::{
    features::todo::{
        model::{self, TodoFilter, TodoItem},
        store,
    },
    platform::{localized, localized_text},
    ui::{
        button::{ButtonSize, ButtonVariant, button_with, choice_button, icon_button},
        controls::checkbox_control,
        field::text_field_labeled,
        layout::{card, divider, page_insets},
        text::{muted, section_title},
    },
};

/// The window width the summary rail joins the column at: past it the
/// pane holds the list and the rail with room to read both; below it the
/// column stands alone (the phone shell's single column, and mid-size
/// windows on their way between the two).
const RAIL_WIDTH: f32 = 1024.0;

/// Whether the window is wide enough for the summary rail beside the
/// list. Read in a `when`, so the rail joins and leaves as the window is
/// resized, not only when the page remounts.
fn rail_shows() -> bool {
    window().width >= px(RAIL_WIDTH)
}

/// The page `Route::Home` mounts. Re-reads the stored list, so a fresh
/// engine — a restart — starts from what the last change saved.
pub fn todo_page() -> AnyElement {
    store::load();
    let items = store::items();
    let filter = store::filter();
    let draft = signal(String::new());

    scroll()
        .flex_1()
        .content_inset(page_insets(0.0))
        // The software keyboard leaves the way the form pages put it
        // away: a drag scrolls it off, and a tap nothing else handles
        // closes it (a tap on a row still reaches the row).
        .keyboard_dismiss_mode(KeyboardDismissMode::OnDrag)
        .keyboard_taps(KeyboardTaps::Handled)
        .child(
            view()
                .flex_row()
                .items_start()
                .gap_6()
                .p_4()
                .pb_safe_area_plus(px(16.0))
                .child(
                    view()
                        .flex_col()
                        .gap_4()
                        .flex_1()
                        .max_w(px(640.0))
                        .child(add_row(draft))
                        .child(filters_row(filter, items))
                        .child(list_area(items, filter)),
                )
                .child(when(rail_shows, move || summary_rail(items))),
        )
        .into_element()
}

/// Adds what `draft` holds (trimmed; whitespace adds nothing) and clears
/// the field either way: the submit said the entry was finished.
fn submit_draft(draft: Signal<String>) {
    store::add(&draft.get_untracked());
    draft.set(String::new());
}

/// The add row: the field, Enter confirming, and a button for the same
/// commit (the touch target Stage 3 keeps). On the desktops the field is
/// focused on arrival — typing is the first thing that works; off them an
/// auto-focused field would raise the software keyboard over the list
/// before any intent to type. The placeholder is the field's accessible
/// name, so what a screen reader hears is what the page asks for.
fn add_row(draft: Signal<String>) -> impl IntoElement {
    view()
        .flex_row()
        .gap_2()
        .child(
            view().test_id("todo-add").flex_1().child(
                text_field_labeled(
                    draft,
                    localized("todo.add.placeholder"),
                    localized("todo.add.placeholder"),
                )
                .return_key_type(ReturnKeyType::Done)
                .on_submit(move |_: &str| submit_draft(draft))
                .auto_focus(is_desktop()),
            ),
        )
        .child(
            view()
                .test_id("todo-add-go")
                .child(icon_button("+", "Add task", move || submit_draft(draft))),
        )
}

/// The remaining count as the page words it — none, one, many — read live
/// so both a change in the list and a switch of the locale re-run it.
fn remaining_label(items: Signal<Vec<TodoItem>>) -> impl Fn() -> String {
    move || {
        crate::platform::locale::app_locale().get();
        match model::remaining(&items.get()) {
            0 => t!("todo.remaining.none").to_string(),
            1 => t!("todo.remaining.one").to_string(),
            count => t!("todo.remaining.many", count = count).to_string(),
        }
    }
}

/// The chips and the count: which tasks show, and how many are left. The
/// chips' visible labels follow the locale; their names for screen readers
/// stay the literal filter names (NSH 0.4.0 sets an accessibility label
/// once, at build).
fn filters_row(filter: Signal<TodoFilter>, items: Signal<Vec<TodoItem>>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .flex_wrap()
        .gap_2()
        .children(TodoFilter::ALL.map(|choice| {
            view()
                .test_id(&format!("todo-filter-{}", choice.key()))
                .child(choice_button(
                    localized(choice.label_key()),
                    choice.label(),
                    move || filter.get() == choice,
                    false,
                    move || filter.set(choice),
                ))
        }))
        .child(view().flex_1())
        .child(
            view()
                .test_id("todo-remaining")
                .child(muted(remaining_label(items))),
        )
        // The finished tasks leave through one action wherever the width
        // puts it: beside the count here while the rail is away (the row
        // wraps under the chips at phone widths), and in the rail once it
        // shows. Hidden only when there is nothing to clear.
        .child(when(
            move || model::completed(&items.get()) > 0,
            move || {
                view().test_id("todo-clear").child(button_with(
                    localized("todo.clear.label"),
                    ButtonVariant::Ghost,
                    ButtonSize::Small,
                    false,
                    Some("Clear completed tasks".to_owned()),
                    store::clear_completed,
                ))
            },
        ))
}

/// The summary rail the wide window adds beside the list: the counts the
/// filter chips stand for, and the clear action's other home. The page's
/// `when` owns its coming and going; it holds no state of its own.
fn summary_rail(items: Signal<Vec<TodoItem>>) -> impl IntoElement {
    view().test_id("todo-rail").w(px(280.0)).child(
        card()
            .gap_2()
            .child(section_title(localized("todo.rail.title")))
            .child(count_row(localized("todo.filter.all"), move || {
                items.get().len()
            }))
            .child(count_row(localized("todo.filter.active"), move || {
                model::remaining(&items.get())
            }))
            .child(count_row(localized("todo.filter.completed"), move || {
                model::completed(&items.get())
            }))
            .child(divider())
            .child(view().test_id("todo-rail-clear").child(button_with(
                localized("todo.clear.label"),
                ButtonVariant::Outline,
                ButtonSize::Small,
                // Present but idle while there is nothing to
                // clear: a card that changes shape reads worse
                // than a button that says not-now.
                move || model::completed(&items.get()) == 0,
                Some("Clear completed tasks".to_owned()),
                store::clear_completed,
            ))),
    )
}

/// One label-and-count row of the rail: the filter's name, its number
/// right-aligned so the column reads as one figure.
fn count_row(
    name: impl Fn() -> String + 'static,
    count: impl Fn() -> usize + 'static,
) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .child(
            text(name)
                .flex_1()
                .text_sm()
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(move || count().to_string())
                .text_sm()
                .tabular_nums()
                .text_color(move || theme().foreground),
        )
}

/// The list, or the empty state that says why there is nothing to show.
fn list_area(items: Signal<Vec<TodoItem>>, filter: Signal<TodoFilter>) -> AnyElement {
    let rows = each(
        move || {
            model::visible(&items.get(), filter.get())
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        },
        |item: &TodoItem| item.id,
        |item, _index| todo_row(item).into_element(),
    );
    view()
        .flex_col()
        .gap_1()
        .child(rows)
        .child(when(
            move || model::visible(&items.get(), filter.get()).is_empty(),
            move || {
                view()
                    .test_id("todo-empty")
                    .child(card().items_center().child(muted(move || {
                        localized_text(empty_key(&items.get(), filter.get()))
                    })))
            },
        ))
        .into_element()
}

/// What the empty state says, as its i18n key: a different line for a list
/// with nothing in it and for a filter that hides everything it has.
fn empty_key(items: &[TodoItem], filter: TodoFilter) -> &'static str {
    if items.is_empty() {
        "todo.empty.none"
    } else {
        match filter {
            TodoFilter::Active => "todo.empty.active",
            TodoFilter::Completed => "todo.empty.completed",
            // A non-empty list under All always has a row to show.
            TodoFilter::All => "todo.empty.all",
        }
    }
}

/// One task row: the completion checkbox, the title, the edit, and the
/// delete. The Edit button swaps the title for the field; Enter renames
/// (a field emptied to whitespace removes the task). Each control's name
/// for screen readers carries the task's title, so a list of rows reads
/// as tasks, not as a repeated "Toggle task" — set once, at the row's
/// build (NSH 0.4.0 knows no live label), so a rename keeps the name it
/// began with until the row rebuilds.
fn todo_row(item: ReadSignal<TodoItem>) -> AnyElement {
    let id = item.get_untracked().id;
    let task_title = item.get_untracked().title;
    // Built once, moved each to its one control: a name a closure can own.
    let toggle_name = format!("Toggle task: {task_title}");
    let edit_name = format!("Edit task: {task_title}");
    let delete_name = format!("Delete task: {task_title}");
    let edit_field_label = edit_name.clone();
    let editing = signal(false);

    let begin_draft = signal(String::new());
    let begin_edit = move || {
        begin_draft.set(item.get_untracked().title);
        editing.set(true);
    };

    let field = move || {
        view().test_id(&format!("todo-edit-{id}")).flex_1().child(
            text_field_labeled(begin_draft, "", edit_field_label.clone())
                .return_key_type(ReturnKeyType::Done)
                .on_submit(move |_: &str| {
                    store::rename(id, &begin_draft.get_untracked());
                    editing.set(false);
                }),
        )
    };

    let middle = when(move || editing.get(), field).otherwise(move || {
        let title = view()
            .flex_1()
            .test_id(&format!("todo-label-{id}"))
            .style_when(
                move || item.get().completed,
                |s| {
                    s.line_through()
                        .text_decoration_color(theme().muted_foreground.opacity(0.6))
                },
            )
            .child(
                text(move || item.get().title)
                    .text_sm()
                    .number_of_lines(1)
                    .text_color(move || theme().foreground),
            );
        view()
            .flex_1()
            .flex_row()
            .items_center()
            .gap_1()
            .child(title)
            .child(
                button_with(
                    localized("todo.edit.label"),
                    ButtonVariant::Ghost,
                    ButtonSize::Small,
                    false,
                    Some(edit_name.clone()),
                    begin_edit,
                )
                .opacity(move || {
                    if item.get().completed {
                        0.6_f32
                    } else {
                        1.0_f32
                    }
                }),
            )
    });

    view()
        .test_id(&format!("todo-item-{id}"))
        .flex_row()
        .items_center()
        .gap_2()
        .py_1()
        .child(
            view()
                .test_id(&format!("todo-toggle-{id}"))
                .child(checkbox_control(
                    move || item.get().completed,
                    move |next| store::set_completed(id, next),
                    toggle_name,
                )),
        )
        .child(middle)
        .child(
            view()
                .test_id(&format!("todo-delete-{id}"))
                .child(icon_button("✕", delete_name, move || store::remove(id))),
        )
        .into_element()
}
