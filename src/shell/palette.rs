//! The command palette: the ui area's `command_palette` in a dialog,
//! opened with Cmd/Ctrl+K. Destinations first (the palette highlights its
//! first row), then the shell's actions. One owner per keystroke: `mod+k`,
//! `mod+z` and `mod+shift+z` are keymap bindings, so the entries show them
//! without binding them again.

use nsh_base::{dialog, theme};
use nsh_core::{element::AnyElement, prelude::*};

use crate::{
    shell::{nav, routes::Route, undo},
    ui::command::{command_entry, command_palette},
};

/// The palette is up.
#[derive(Default, Clone, PartialEq)]
pub struct Open(pub bool);

fn open_state() -> Signal<Open> {
    global::<Open>()
}

/// Shows or hides the palette (the shortcut, the header's button).
pub fn toggle() {
    open_state().update(|open| open.0 = !open.0);
}

fn close() {
    open_state().set(Open(false));
}

/// Runs the command `id` names and closes the palette. A destination's id
/// is its route key; an action's is `action:<name>`.
fn dispatch(id: &str) {
    if let Some(route) = Route::ALL.iter().copied().find(|r| r.key_str() == id) {
        nav::open(route);
    } else {
        match id {
            "action:undo" => undo::undo(),
            "action:redo" => undo::redo(),
            "action:appearance" => crate::shell::toggle_appearance(),
            _ => {}
        }
    }
    close();
}

/// Every command the palette lists.
pub fn items() -> Vec<nsh_base::CommandItem> {
    let mut items: Vec<nsh_base::CommandItem> = Route::ALL
        .iter()
        .filter(|route| **route != Route::Home)
        .map(|route| command_entry(route.key_str(), route.label_str()).group("Go to"))
        .collect();
    items.push(
        command_entry("action:undo", "Undo")
            .group("Actions")
            .shortcut("mod+z"),
    );
    items.push(
        command_entry("action:redo", "Redo")
            .group("Actions")
            .shortcut("mod+shift+z"),
    );
    items.push(command_entry("action:appearance", "Toggle appearance").group("Actions"));
    items
}

/// The palette over everything, while it is open. Escape and back close it
/// as the modal's own (the wrapper owns the query's Escape); either way
/// the dialog reports `on_cancel`.
pub fn palette_dialog() -> AnyElement {
    let open = open_state();
    when(
        move || open.with(|open| open.0),
        || {
            // Wrapper gap: ui::dialog_of lays out a full page, so a
            // floating card over a scrim keeps nsh_base::dialog here.
            dialog()
                .on_cancel(|| {})
                .flex_1()
                .flex_col()
                .items_center()
                .bg(move || theme().foreground.opacity(0.25))
                .child(
                    view()
                        .w(px(560.0))
                        .max_h(px(420.0))
                        .flex_col()
                        .overflow_hidden()
                        .rounded_lg()
                        .border_1()
                        .border_color(move || theme().border)
                        .bg(move || theme().popover)
                        .shadow_lg()
                        .child(command_palette(
                            items(),
                            "Type a command…",
                            move |id: SharedString| dispatch(&id),
                        )),
                )
                .into_element()
        },
    )
    .into_element()
}
