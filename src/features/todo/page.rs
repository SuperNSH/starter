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
    ui::{
        button::{ButtonSize, ButtonVariant, button_with, choice_button, icon_button},
        controls::checkbox_control,
        field::text_field,
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
/// before any intent to type.
fn add_row(draft: Signal<String>) -> impl IntoElement {
    view()
        .flex_row()
        .gap_2()
        .child(
            view().test_id("todo-add").flex_1().child(
                text_field(draft, "What needs doing?")
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

/// The chips and the count: which tasks show, and how many are left.
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
                    choice.label(),
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
                .child(muted(move || model::remaining_label(&items.get()))),
        )
        // The finished tasks leave through one action wherever the width
        // puts it: beside the count here while the rail is away (the row
        // wraps under the chips at phone widths), and in the rail once it
        // shows. Hidden only when there is nothing to clear.
        .child(when(
            move || model::completed(&items.get()) > 0,
            move || {
                view().test_id("todo-clear").child(button_with(
                    "Clear completed",
                    ButtonVariant::Ghost,
                    ButtonSize::Small,
                    false,
                    Some("Clear completed tasks"),
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
            .child(section_title("Summary"))
            .child(count_row("All", move || items.get().len()))
            .child(count_row("Active", move || model::remaining(&items.get())))
            .child(count_row("Completed", move || {
                model::completed(&items.get())
            }))
            .child(divider())
            .child(view().test_id("todo-rail-clear").child(button_with(
                "Clear completed",
                ButtonVariant::Outline,
                ButtonSize::Small,
                // Present but idle while there is nothing to
                // clear: a card that changes shape reads worse
                // than a button that says not-now.
                move || model::completed(&items.get()) == 0,
                Some("Clear completed tasks"),
                store::clear_completed,
            ))),
    )
}

/// One label-and-count row of the rail: the filter's name, its number
/// right-aligned so the column reads as one figure.
fn count_row(name: &'static str, count: impl Fn() -> usize + 'static) -> impl IntoElement {
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
                view().test_id("todo-empty").child(
                    card()
                        .items_center()
                        .child(muted(move || empty_message(&items.get(), filter.get()))),
                )
            },
        ))
        .into_element()
}

/// What the empty state says: a different line for a list with nothing in
/// it and for a filter that hides everything it has.
fn empty_message(items: &[TodoItem], filter: TodoFilter) -> String {
    if items.is_empty() {
        "No tasks yet — add the first one above.".to_owned()
    } else {
        match filter {
            TodoFilter::Active => "No active tasks — everything is done.".to_owned(),
            TodoFilter::Completed => "Nothing completed yet.".to_owned(),
            // A non-empty list under All always has a row to show.
            TodoFilter::All => "No tasks here.".to_owned(),
        }
    }
}

/// One task row: the completion checkbox, the title, the edit, and the
/// delete. The Edit button swaps the title for the field; Enter renames
/// (a field emptied to whitespace removes the task).
fn todo_row(item: ReadSignal<TodoItem>) -> AnyElement {
    let id = item.get_untracked().id;
    let editing = signal(false);

    let begin_draft = signal(String::new());
    let begin_edit = move || {
        begin_draft.set(item.get_untracked().title);
        editing.set(true);
    };

    let field = move || {
        view().test_id(&format!("todo-edit-{id}")).flex_1().child(
            text_field(begin_draft, "")
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
                    "Edit",
                    ButtonVariant::Ghost,
                    ButtonSize::Small,
                    false,
                    Some("Edit task"),
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
                    "Toggle task",
                )),
        )
        .child(middle)
        .child(
            view()
                .test_id(&format!("todo-delete-{id}"))
                .child(icon_button("✕", "Delete task", move || store::remove(id))),
        )
        .into_element()
}
