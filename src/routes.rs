//! Every screen's address, and the record each screen declares.
//!
//! [`Route`] is where a screen lives: what navigation pushes, what a deep
//! link parses into, what undo snapshots. [`Page`] is what a screen says
//! about itself: one `const PAGE` in its folder's `mod.rs`. `pages/mod.rs`
//! joins the two (`Route::page`).
//!
//! Adding a screen: `docs/recipes/add-a-page.md`.
//!
//! May import: nothing of the app, except the showcase's own route enum.

use nsh_core::element::AnyElement;

use crate::showcase;

/// The scheme the app's links start with (`nsh.json` `"schemes"`).
pub const SCHEME: &str = "nsh-starter://";

/// Where a screen lives. `Home` is the bottom of the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Settings,
    Diagnostics,
    /// One of the demo screens (`src/showcase`). A new app deletes this
    /// variant together with that folder.
    Showcase(showcase::Route),
}

impl Route {
    /// The id the route carries, for a screen that shows one item.
    pub fn id(self) -> Option<u64> {
        match self {
            Route::Showcase(route) => route.id(),
            _ => None,
        }
    }

    /// The route a pushed screen sits on. Opening a screen that has a
    /// parent opens the parent under it.
    pub fn parent(self) -> Option<Route> {
        match self {
            Route::Showcase(route) => route.parent().map(Route::Showcase),
            _ => None,
        }
    }

    /// The route's key in the stack: the same route is the same page, so
    /// the pages under it keep their state.
    pub fn stack_key(self) -> String {
        format!("/{}", self.path())
    }

    /// The route's link path, without the scheme: `data/item/3`.
    pub fn path(self) -> String {
        match self {
            Route::Home => String::new(),
            Route::Settings => "settings".into(),
            Route::Diagnostics => "diagnostics".into(),
            Route::Showcase(route) => route.path(),
        }
    }

    /// The route a link path names, segment by segment. `None` when the
    /// path is not one of the app's.
    pub fn parse(segments: &[&str]) -> Option<Route> {
        match segments {
            [] => Some(Route::Home),
            ["settings"] => Some(Route::Settings),
            ["diagnostics"] => Some(Route::Diagnostics),
            other => showcase::Route::parse(other).map(Route::Showcase),
        }
    }

    /// The route a whole link names (`nsh-starter://data/item/3`).
    pub fn from_url(url: &str) -> Option<Route> {
        let path = url.strip_prefix(SCHEME)?;
        Route::parse(&path.split('/').collect::<Vec<_>>())
    }
}

impl From<showcase::Route> for Route {
    fn from(route: showcase::Route) -> Self {
        Route::Showcase(route)
    }
}

/// What a screen declares about itself. The shell reads nothing else about
/// a screen: its row in the sidebar, its tab, its header, its card on Home
/// and its entry in the palette all come from this record.
pub struct Page {
    /// The screen's one name: its folder, its test file, the prefix of its
    /// strings and test ids. The snake_case of its `Route` variant.
    pub key: &'static str,
    /// What the header, the sidebar, the tabs and the palette call it.
    pub title: &'static str,
    /// One line on what it shows, for its card on Home.
    pub summary: &'static str,
    /// Its icon in the sidebar and the Android tab bar: Lucide markup
    /// (`ui::icons::lucide!`), drawn in the current text color.
    pub icon: &'static str,
    /// Its SF Symbol, for the iOS tab bar and the macOS command bar.
    pub symbol: &'static str,
    /// How the shell frames it.
    pub frame: Frame,
    /// A button in the iOS system header, when the screen has one.
    pub action: Option<HeaderAction>,
    /// What it draws.
    pub content: Content,
}

/// What a screen draws, under the shell's header.
pub enum Content {
    /// A screen with nothing to be told.
    Plain(fn() -> AnyElement),
    /// A screen of one item, given the route's id.
    WithId(fn(u64) -> AnyElement),
}

/// How the shell frames a screen.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Frame {
    /// The usual: the content scrolls under the header and the tab bar.
    Scrolls,
    /// The content does not scroll as a whole: on phones it starts below
    /// the header and stops above the tab bar.
    Fixed,
    /// The screen runs a navigation stack of its own, whose pages draw
    /// the phone's system header themselves.
    OwnStack,
}

/// A button in the iOS system header, beside the back button.
pub struct HeaderAction {
    /// The test id of the button.
    pub test_id: &'static str,
    /// Its accessibility label.
    pub label: &'static str,
    /// The SF Symbol UIKit draws for it.
    pub symbol: &'static str,
    /// What a press does.
    pub run: fn(),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_link_names_its_route() {
        assert_eq!(
            Route::from_url("nsh-starter://diagnostics"),
            Some(Route::Diagnostics)
        );
        assert_eq!(Route::from_url("other://diagnostics"), None);
        assert_eq!(Route::parse(&[]), Some(Route::Home));
        assert_eq!(Route::parse(&["nowhere"]), None);
    }
}
