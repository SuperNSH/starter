//! The shell's side of the macOS command bar (`native::command_bar`): the
//! commands it lists, built from the page records, and the hidden bridge
//! element that hands them over and runs the one chosen through the
//! palette. Elsewhere the in-app palette stays.

use nsh_core::{JsonValue, element::NativeEvent, prelude::*};

use crate::native::command_bar::BRIDGE;
use crate::pages;
use crate::shell::palette;

/// The command bar replaces the in-app palette here.
pub fn available() -> bool {
    cfg!(target_os = "macos") && os() == nsh_core::platform::Os::MacOs
}

/// How many times the app asked for the bar: the bridge opens it on each
/// new value.
#[derive(Default, Clone, PartialEq)]
struct Requests(u64);

/// Opens the bar (Cmd+K, the header's button).
pub fn open() {
    global::<Requests>().update(|r| r.0 += 1);
}

/// One command the bar lists.
struct Command {
    id: &'static str,
    title: &'static str,
    group: &'static str,
    symbol: &'static str,
    shortcut: &'static str,
    /// Brings the main window forward first (a destination).
    front: bool,
}

fn commands() -> Vec<Command> {
    let mut list: Vec<Command> = pages::sidebar()
        .iter()
        .map(|route| route.page())
        .map(|page| Command {
            id: page.key,
            title: page.title,
            group: "Go to",
            symbol: page.symbol,
            shortcut: "",
            front: true,
        })
        .collect();
    list.extend([
        Command {
            id: "action:undo",
            title: "Undo",
            group: "Action",
            symbol: "arrow.uturn.backward",
            shortcut: "⌘Z",
            front: false,
        },
        Command {
            id: "action:redo",
            title: "Redo",
            group: "Action",
            symbol: "arrow.uturn.forward",
            shortcut: "⇧⌘Z",
            front: false,
        },
        Command {
            id: "action:appearance",
            title: "Toggle appearance",
            group: "Action",
            symbol: "circle.lefthalf.filled",
            shortcut: "",
            front: false,
        },
    ]);
    list
}

fn commands_json() -> JsonValue {
    JsonValue::from(
        commands()
            .into_iter()
            .map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "title": c.title,
                    "group": c.group,
                    "symbol": c.symbol,
                    "shortcut": c.shortcut,
                    "front": c.front,
                })
            })
            .collect::<Vec<_>>(),
    )
}

/// The hidden element joining the shell to the bar; nothing elsewhere.
pub fn bridge() -> AnyElement {
    if !available() {
        return view().into_element();
    }
    let requests = global::<Requests>();
    nsh_core::element::native(BRIDGE)
        .prop("items", commands_json())
        .prop("open", move || JsonValue::from(requests.get().0))
        .on_native_event(move |e: &NativeEvent| {
            if &*e.name == "run"
                && let Some(id) = e.data["id"].as_str()
            {
                palette::dispatch(id);
            }
        })
        .absolute()
        .w(px(0.0))
        .h(px(0.0))
        .into_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_route_and_action_is_a_command() {
        let ids: Vec<&str> = commands().iter().map(|c| c.id).collect();
        for route in pages::sidebar() {
            assert!(ids.contains(&route.page().key), "{route:?} missing");
        }
        assert!(ids.contains(&"action:appearance"));
    }
}
