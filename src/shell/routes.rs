//! The shell's typed routes: the to-do list, the app's one page. Every
//! page is mounted through [`Route::page`]; the boilerplate's demo pages
//! stay in the crate (their own tests mount them directly) but no route
//! reaches them, so the app a user sees is the to-do app alone.

use nsh_core::{element::AnyElement, prelude::*};

/// The routes the shell answers to, `Home` the stack's root and only
/// page: the to-do list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
}

impl Route {
    /// Every root route, in the order the shell lists them.
    pub const ALL: &'static [Route] = &[Route::Home];

    /// The stack key: the same route is the same page, so the pages below
    /// keep their state.
    pub fn key(self) -> String {
        match self {
            Route::Home => "home".into(),
        }
    }

    /// What the sidebar, the tabs, the palette and the tests call it.
    pub fn label(self) -> String {
        self.label_str().into()
    }

    /// The label of a root route, as a literal (static contexts: the
    /// palette's entries, the smoke suite's markers).
    pub fn label_str(self) -> &'static str {
        match self {
            Route::Home => "To-dos",
        }
    }

    /// The key of a root route, as a literal (static contexts).
    pub fn key_str(self) -> &'static str {
        match self {
            Route::Home => "home",
        }
    }

    /// The tab bar's icon: an SF Symbol on iOS, a system drawable on
    /// Android (`nsh package` cannot add app drawables); none elsewhere.
    pub fn tab_icon(self) -> Option<&'static str> {
        use nsh_core::platform::Os;
        let (ios, android) = match self {
            Route::Home => ("checklist", "android:ic_menu_agenda"),
        };
        match os() {
            Os::Ios => Some(ios),
            Os::Android => Some(android),
            _ => None,
        }
    }

    /// The tab's Lucide icon (ISC), drawn in `currentColor`, for the themed
    /// tab bar Android shows in place of the system's Material bar.
    pub fn tab_svg(self) -> &'static str {
        macro_rules! icon {
            ($body:literal) => {
                concat!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">"#,
                    $body,
                    "</g></svg>"
                )
            };
        }
        match self {
            Route::Home => {
                icon!(r#"<path d="M3 5h.01M3 12h.01M3 19h.01M8 5h13M8 12h13M8 19h13"/>"#)
            }
        }
    }

    /// The test id of the route's tab in the themed (Android) tab bar.
    pub fn tab_id(self) -> &'static str {
        match self {
            Route::Home => "tab-home",
        }
    }

    /// The sidebar row's test id (`nav-<key>`), as a literal.
    pub fn nav_row_id(self) -> &'static str {
        match self {
            Route::Home => "nav-home",
        }
    }

    /// The system header's title.
    pub fn title(self) -> SharedString {
        self.label().into()
    }

    /// The pages the phone's tab bar holds, in its order.
    pub const TABS: [Route; 1] = [Route::Home];

    /// One of the phone's tabs: a root there, so it shows no back button.
    pub fn is_tab(self) -> bool {
        Route::TABS.contains(&self)
    }

    /// The page's content, made by the feature areas.
    pub fn page(self) -> AnyElement {
        match self {
            Route::Home => crate::features::todo::page::todo_page().into_element(),
        }
    }
}

/// The route a deep-link path names: the segments after
/// `todo-sample://`, in order. `None` when the path is not the app's.
pub fn route_from_deep_link(seg: &[&str]) -> Option<Route> {
    match seg {
        [] | ["home"] => Some(Route::Home),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_key_is_stable() {
        assert_eq!(Route::Home.key(), "home");
        assert_eq!(Route::Home, Route::Home);
    }
}
