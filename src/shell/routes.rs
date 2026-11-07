//! The shell's typed routes: one page each, their keys and labels, and the
//! deep-link paths they answer to. Every page is mounted through
//! [`Route::page`], so a new route is one variant plus its arm here.

use nsh_core::{element::AnyElement, prelude::*};

use crate::shell::home;

/// The routes the shell answers to, `Home` the stack's root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    UiGallery,
    Forms,
    Data,
    DataDetail(u64),
    QueryDevtools,
    Lists,
    Canvas,
    Motion,
    Overlays,
    Platform,
    Device,
    RemoteScreen,
    Diagnostics,
}

impl Route {
    /// Every root route, in the order Home lists them. Detail pages are
    /// pushed, never destinations.
    pub const ALL: &'static [Route] = &[
        Route::Home,
        Route::UiGallery,
        Route::Forms,
        Route::Data,
        Route::QueryDevtools,
        Route::Lists,
        Route::Canvas,
        Route::Motion,
        Route::Overlays,
        Route::Platform,
        Route::Device,
        Route::RemoteScreen,
        Route::Diagnostics,
    ];

    /// The stack key: the same route is the same page, so the pages below
    /// keep their state.
    pub fn key(self) -> String {
        match self {
            Route::Home => "home".into(),
            Route::UiGallery => "ui".into(),
            Route::Forms => "forms".into(),
            Route::Data => "data".into(),
            Route::DataDetail(id) => format!("data-item:{id}"),
            Route::QueryDevtools => "query-devtools".into(),
            Route::Lists => "lists".into(),
            Route::Canvas => "canvas".into(),
            Route::Motion => "motion".into(),
            Route::Overlays => "overlays".into(),
            Route::Platform => "platform".into(),
            Route::Device => "device".into(),
            Route::RemoteScreen => "remote".into(),
            Route::Diagnostics => "diagnostics".into(),
        }
    }

    /// What the sidebar, the tabs, the palette and the tests call it.
    pub fn label(self) -> String {
        match self {
            Route::DataDetail(id) => format!("Data item {id}"),
            route => route.label_str().into(),
        }
    }

    /// The label of a root route, as a literal (static contexts: the
    /// palette's entries, the smoke suite's markers).
    pub fn label_str(self) -> &'static str {
        match self {
            Route::Home => "Home",
            Route::UiGallery => "UI gallery",
            Route::Forms => "Forms",
            Route::Data => "Data",
            Route::QueryDevtools => "Query devtools",
            Route::Lists => "Lists",
            Route::Canvas => "Canvas",
            Route::Motion => "Motion",
            Route::Overlays => "Overlays",
            Route::Platform => "Platform",
            Route::Device => "Device",
            Route::RemoteScreen => "Remote screen",
            Route::Diagnostics => "Diagnostics",
            Route::DataDetail(_) => "Data item",
        }
    }

    /// The key of a root route, as a literal (static contexts).
    /// The tab bar's icon: an SF Symbol on iOS, a system drawable on
    /// Android (`nsh package` cannot add app drawables); none elsewhere.
    pub fn tab_icon(self) -> Option<&'static str> {
        use nsh_core::platform::Os;
        let (ios, android) = match self {
            Route::Home => ("house", "android:ic_menu_compass"),
            Route::Forms => ("square.and.pencil", "android:ic_menu_edit"),
            Route::Data => ("tray.full", "android:ic_menu_agenda"),
            Route::Lists => ("list.bullet", "android:ic_menu_sort_by_size"),
            Route::Motion => ("wind", "android:ic_menu_rotate"),
            _ => return None,
        };
        match os() {
            Os::Ios => Some(ios),
            Os::Android => Some(android),
            _ => None,
        }
    }

    /// One line on what the page shows, for Home's cards.
    pub fn summary(self) -> &'static str {
        match self {
            Route::Home => "Every page of the boilerplate.",
            Route::UiGallery => "Every control of the app's own library, with its states.",
            Route::Forms => "A full sign-up and a settings form on nsh-form.",
            Route::Data => "An infinite feed on nsh-query with optimistic likes.",
            Route::DataDetail(_) => "One post, read through a tracked query key.",
            Route::QueryDevtools => "The query cache: keys, states and timings.",
            Route::Lists => "Tables, a tree, a calendar, a chat and a carousel.",
            Route::Canvas => "Drawing, charts, shaders, SVG, media and the web view.",
            Route::Motion => "Springs, transitions, swipe rows and gestures.",
            Route::Overlays => "Dialogs, sheets, menus, toasts and desktop panes.",
            Route::Platform => "Safe areas, clipboard, picker, speech and locales.",
            Route::Device => "The device, its screen and the environment.",
            Route::RemoteScreen => "A signed TSX package, verified and revocable.",
            Route::Diagnostics => "Config, capabilities, tasks, events and the log.",
        }
    }

    pub fn key_str(self) -> &'static str {
        match self {
            Route::Home => "home",
            Route::UiGallery => "ui",
            Route::Forms => "forms",
            Route::Data => "data",
            Route::QueryDevtools => "query-devtools",
            Route::Lists => "lists",
            Route::Canvas => "canvas",
            Route::Motion => "motion",
            Route::Overlays => "overlays",
            Route::Platform => "platform",
            Route::Device => "device",
            Route::RemoteScreen => "remote",
            Route::Diagnostics => "diagnostics",
            Route::DataDetail(_) => "data-item",
        }
    }

    /// The Home row's test id (`page-<key>`), as a literal: the ui row
    /// wrapper carries test ids on `&'static str`.
    pub fn page_row_id(self) -> &'static str {
        match self {
            Route::Home => "page-home",
            Route::UiGallery => "page-ui",
            Route::Forms => "page-forms",
            Route::Data => "page-data",
            Route::QueryDevtools => "page-query-devtools",
            Route::Lists => "page-lists",
            Route::Canvas => "page-canvas",
            Route::Motion => "page-motion",
            Route::Overlays => "page-overlays",
            Route::Platform => "page-platform",
            Route::Device => "page-device",
            Route::RemoteScreen => "page-remote",
            Route::Diagnostics => "page-diagnostics",
            Route::DataDetail(_) => "page-data-item",
        }
    }

    /// The sidebar row's test id (`nav-<key>`), as a literal.
    pub fn nav_row_id(self) -> &'static str {
        match self {
            Route::Home => "nav-home",
            Route::UiGallery => "nav-ui",
            Route::Forms => "nav-forms",
            Route::Data => "nav-data",
            Route::QueryDevtools => "nav-query-devtools",
            Route::Lists => "nav-lists",
            Route::Canvas => "nav-canvas",
            Route::Motion => "nav-motion",
            Route::Overlays => "nav-overlays",
            Route::Platform => "nav-platform",
            Route::Device => "nav-device",
            Route::RemoteScreen => "nav-remote",
            Route::Diagnostics => "nav-diagnostics",
            Route::DataDetail(_) => "nav-data-item",
        }
    }

    /// The system header's title.
    pub fn title(self) -> SharedString {
        self.label().into()
    }

    /// The pages the phone's tab bar holds, in its order.
    pub const TABS: [Route; 5] = [
        Route::Home,
        Route::Forms,
        Route::Data,
        Route::Lists,
        Route::Motion,
    ];

    /// One of the phone's tabs: a root there, so it shows no back button.
    pub fn is_tab(self) -> bool {
        Route::TABS.contains(&self)
    }

    /// A page that runs a `nav_stack` of its own (Forms: sign-up, then
    /// settings), whose pages draw the phone's system header themselves.
    pub fn has_own_stack(self) -> bool {
        matches!(self, Route::Forms)
    }

    /// A page pushed over its parent rather than opened in its place.
    pub fn is_detail(self) -> bool {
        matches!(self, Route::DataDetail(_))
    }

    /// The route a detail sits on.
    pub fn parent(self) -> Option<Route> {
        match self {
            Route::DataDetail(_) => Some(Route::Data),
            _ => None,
        }
    }

    /// The page's content, made by the feature areas.
    pub fn page(self) -> AnyElement {
        match self {
            Route::Home => home::content().into_element(),
            Route::UiGallery => crate::ui::gallery::ui_gallery_page().into_element(),
            Route::Forms => crate::features::forms::forms_page().into_element(),
            Route::Data => crate::features::data::data_page().into_element(),
            Route::DataDetail(id) => crate::features::data::data_detail_page(id).into_element(),
            Route::QueryDevtools => crate::features::data::query_devtools_page().into_element(),
            Route::Lists => crate::features::lists::lists_page().into_element(),
            Route::Canvas => crate::features::canvas::canvas_page().into_element(),
            Route::Motion => crate::features::motion::motion_page().into_element(),
            Route::Overlays => crate::features::overlays::overlays_page().into_element(),
            Route::Platform => crate::platform::platform_page().into_element(),
            Route::Device => crate::platform::device_page().into_element(),
            Route::RemoteScreen => {
                crate::features::remote_screen::remote_screen_page().into_element()
            }
            Route::Diagnostics => crate::foundation::diagnostics_content().into_element(),
        }
    }
}

/// The route a deep-link path names: the segments after
/// `nsh-starter://`, in order. `None` when the path is not the app's.
pub fn route_from_deep_link(seg: &[&str]) -> Option<Route> {
    match seg {
        [] => Some(Route::Home),
        ["forms"] => Some(Route::Forms),
        ["ui"] => Some(Route::UiGallery),
        ["data"] => Some(Route::Data),
        ["data", "devtools"] => Some(Route::QueryDevtools),
        ["data", "item", id] => Some(Route::DataDetail(id.parse().ok()?)),
        ["lists"] => Some(Route::Lists),
        ["canvas"] => Some(Route::Canvas),
        ["motion"] => Some(Route::Motion),
        ["overlays"] => Some(Route::Overlays),
        ["platform"] => Some(Route::Platform),
        ["device"] => Some(Route::Device),
        ["remote"] => Some(Route::RemoteScreen),
        ["diagnostics"] => Some(Route::Diagnostics),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_stable_per_route() {
        assert_eq!(Route::DataDetail(3).key(), "data-item:3");
        assert_eq!(Route::DataDetail(3), Route::DataDetail(3));
        assert_ne!(Route::DataDetail(3), Route::DataDetail(4));
    }
}
