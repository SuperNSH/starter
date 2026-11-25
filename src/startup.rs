//! Everything that runs once at launch, in order. `App` calls [`install`]
//! from its window's content, so each piece lives as long as the window.

use nsh_core::prelude::*;

use crate::{
    services::logging,
    shell::{back, chrome, deep_link, palette},
    store::{config, event_bus, lifecycle, undo},
};

/// Wires the app. To add something that must run at launch, add it here.
pub fn install() {
    // 1. Keyboard shortcuts. One owner per keystroke: the menus and the
    //    palette show these and bind none of their own.
    on_key("mod+k", move |_| palette::toggle());
    on_key("mod+z", move |_| undo::undo());
    on_key("mod+shift+z", move |_| undo::redo());
    on_key(chrome::sidebar_key(), move |_| chrome::toggle_sidebar());

    // 2. The back order: modals first, then pushed pages, then the exit ask.
    back::install();

    // 3. The window comes back when the Dock asks for it.
    chrome::show_on_reopen();

    // 4. The lifecycle phase the status bar shows.
    lifecycle::watch();

    // 5. Deep links: the one the app was launched with, then every later one.
    deep_link::open_pending();
    deep_link::listen();

    // 6. Config saves leave a line in the log.
    event_bus::subscribe(event_bus::TOPIC_CONFIG_SAVED, |event| {
        let name = config::load().display_name;
        let who = if name.is_empty() {
            "(unnamed)"
        } else {
            name.as_str()
        };
        logging::log_info(&format!("config saved ({}) for {who}", event.payload));
    });
}
