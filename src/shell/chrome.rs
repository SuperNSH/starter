//! Desktop chrome: the app's own title bar and the application menu
//! (`nsh/desktop`'s `Window` and `Menu`). Everything here no-ops off the
//! desktops, so it is installed unconditionally. One owner per keystroke:
//! the shortcuts are keymap bindings (`shell::install`); the menu items
//! show them and carry no `shortcut` of their own.

use nsh_base::theme;
use nsh_core::{
    desktop::{MenuRole, TitleBarStyle},
    element::desktop::{
        MenuItem, close_window, set_application_menu, set_title_bar, title_bar as title_bar_insets,
    },
    prelude::*,
};

use crate::shell::{nav, palette, routes::Route, undo};

/// The title bar's height: the platform's window controls center in it
/// (`set_title_bar` caps at 200 points; a platform boundary, not a scale
/// step).
const TITLE_BAR: f32 = 48.0;

/// The desktop shell's wiring: the hidden system title bar under the one
/// the shell draws, and the application menu. Undone when the desktop
/// shell goes (a window narrowed to the phone shell).
pub fn install() {
    effect(move || {
        set_title_bar(TitleBarStyle::Hidden, Some(px(TITLE_BAR)));
        cleanup(|| set_title_bar(TitleBarStyle::Default, None));
    });
    effect(move || {
        // Read so the menu (its appearance item's check mark) follows.
        let dark = dark();
        set_application_menu(Some(menus(dark)));
    });
}

/// Closes the window as the platform does (the exit ask's command on the
/// desktops).
pub fn close() {
    close_window();
}

/// The application menu: the shell's commands, every route, the window.
fn menus(is_dark: bool) -> Vec<MenuItem> {
    let go = |route: Route| MenuItem::new(route.label()).on_select(move || nav::open(route));
    vec![
        MenuItem::submenu(
            t!("app.name"),
            [
                MenuItem::role(MenuRole::About),
                MenuItem::separator(),
                MenuItem::new(format!("Command Palette  {}", key_label("mod+k")))
                    .on_select(palette::toggle),
                MenuItem::new(format!("Undo  {}", key_label("mod+z"))).on_select(undo::undo),
                MenuItem::new(format!("Redo  {}", key_label("mod+shift+z"))).on_select(undo::redo),
                MenuItem::separator(),
                MenuItem::new("Toggle Appearance")
                    .checked(is_dark)
                    .on_select(crate::shell::toggle_appearance),
                MenuItem::separator(),
                MenuItem::role(MenuRole::Quit),
            ],
        ),
        MenuItem::submenu("Go", Route::ALL.iter().copied().map(go).collect::<Vec<_>>()),
        MenuItem::submenu(
            "Window",
            [
                MenuItem::role(MenuRole::Minimize),
                MenuItem::role(MenuRole::Zoom),
                MenuItem::role(MenuRole::ToggleFullScreen),
            ],
        ),
    ]
}

/// The bar the shell draws under the hidden system one: draggable, with
/// the native window controls' lane kept free at its end.
pub fn title_bar() -> impl IntoElement {
    // The native controls' lanes (macOS: left, Windows: right); reading
    // them subscribes, so entering full screen gives the lane back.
    view()
        .h(px(TITLE_BAR))
        .flex_row()
        .items_center()
        .gap_3()
        .pl(move || title_bar_insets().left + px(12.0))
        .pr(move || title_bar_insets().right + px(12.0))
        .border_b_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .window_drag(true)
        .child(
            text(t!("app.name").to_string())
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || theme().card_foreground),
        )
        .child(
            text("Boilerplate for Native Shell")
                .flex_1()
                .text_xs()
                .number_of_lines(1)
                .text_color(move || theme().muted_foreground),
        )
}
