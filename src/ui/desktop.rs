//! Desktop window helpers (`nsh/desktop`): the title bar of the app's
//! own and the application menu, so the shell draws its chrome from here.

use nsh_core::desktop::TitleBarStyle;
use nsh_core::element::desktop::set_title_bar;
use nsh_core::element::desktop::{MenuItem, set_application_menu, title_bar};

/// Hide the system title bar: the app fills the window and draws its own
/// bar in the area [`title_bar_inset`] reports. Desktops only.
pub fn use_custom_title_bar() {
    set_title_bar(TitleBarStyle::Hidden, None);
}

/// The desktop's title-bar insets, so the app's bar keeps clear of the
/// window controls; zero on platforms without one.
pub fn title_bar_inset() -> nsh_core::Edges<nsh_core::Points> {
    title_bar()
}

/// The application menu, macOS's menu bar and the window's menu bar
/// elsewhere; `None` restores the platform's default.
pub fn set_app_menu(items: Option<Vec<MenuItem>>) {
    set_application_menu(items);
}

/// A menu item that runs `on_select`, labeled `label`.
pub fn menu_action(label: &'static str, on_select: impl Fn() + 'static) -> MenuItem {
    MenuItem::new(label).on_select(on_select)
}
