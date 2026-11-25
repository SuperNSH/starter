//! The app's screens, one folder each, and the one table that lists them.
//!
//! A screen's folder has the same shape everywhere:
//!
//! - `mod.rs`: what the screen is, and its [`Page`] record;
//! - `view.rs`: its layout;
//! - `state.rs`: its own signals and the functions that change them, when
//!   it has any;
//! - `queries.rs`: its server reads and writes, when it has any.
//!
//! Its tests are `tests/<folder>.rs`. Adding a screen:
//! `docs/recipes/add-a-page.md`.
//!
//! A screen may import: `store`, `services`, `ui`, `native`, `routes`,
//! `error`. Never `shell`, and never another screen's folder: what two
//! screens share lives in `store`.

pub mod diagnostics;
pub mod home;
pub mod items;
pub mod settings;
pub mod sign_in;

use std::sync::LazyLock;

use crate::routes::{Page, Route};
use crate::showcase;
use crate::store::nav;

impl Route {
    /// The record of the screen this route shows. A new screen adds its
    /// arm here; the compiler asks for it.
    pub fn page(self) -> &'static Page {
        match self {
            Route::Home => &home::PAGE,
            Route::Items => &items::PAGE,
            Route::ItemsDetail(_) => &items::DETAIL,
            Route::SignIn => &sign_in::PAGE,
            Route::Settings => &settings::PAGE,
            Route::Diagnostics => &diagnostics::PAGE,
            Route::Showcase(route) => route.page(),
        }
    }

    /// The screen's title, with the id when the route carries one
    /// ("Data item 3").
    pub fn title(self) -> String {
        match self.id() {
            Some(id) => format!("{} {id}", self.page().title),
            None => self.page().title.to_owned(),
        }
    }
}

/// Every screen the sidebar, Home and the palette list, in their order.
/// `SignIn` is not listed: Settings and its link open it.
pub fn sidebar() -> &'static [Route] {
    static LIST: LazyLock<Vec<Route>> = LazyLock::new(|| {
        let mut list = vec![Route::Home];
        list.extend(showcase::SIDEBAR.iter().copied().map(Route::Showcase));
        list.push(Route::Items);
        list.push(Route::Settings);
        list.push(Route::Diagnostics);
        list
    });
    &LIST
}

/// The screens on the phone's tab bar, in their order: five at most, or
/// iOS folds the rest into a More tab. A tab is a root on phones: it shows
/// no back button.
pub fn tabs() -> &'static [Route] {
    static LIST: LazyLock<Vec<Route>> = LazyLock::new(|| {
        let mut list = vec![Route::Home];
        list.extend(showcase::TABS.iter().copied().map(Route::Showcase));
        list.push(Route::Items);
        list
    });
    &LIST
}

/// A row Home shows under its cards: a test id, a label and what a press
/// does.
pub type HomeRow = (&'static str, &'static str, fn());

/// Rows Home shows under its cards: the screens no card opens (a pushed
/// item, the sign-in), so every screen is one press from Home.
pub fn home_rows() -> &'static [HomeRow] {
    static LIST: LazyLock<Vec<HomeRow>> = LazyLock::new(|| {
        let mut list: Vec<HomeRow> = vec![("page-items_detail", "Open item 3", || {
            nav::push(Route::ItemsDetail(3))
        })];
        list.push(("page-sign_in", "Sign in", || nav::push(Route::SignIn)));
        list.extend_from_slice(showcase::HOME_ROWS);
        list
    });
    &LIST
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A screen's key is the snake_case of its route's name, so one word
    /// finds its variant, its folder, its test file and its ids.
    #[test]
    fn every_key_is_its_routes_name_in_snake_case() {
        for route in sidebar() {
            // `Showcase(DataDevtools)` and `Settings` both name their screen.
            let debug = format!("{route:?}");
            let name = debug.trim_start_matches("Showcase(").trim_end_matches(')');
            let mut snake = String::new();
            for (at, letter) in name.chars().enumerate() {
                if letter.is_uppercase() && at > 0 {
                    snake.push('_');
                }
                snake.extend(letter.to_lowercase());
            }
            assert_eq!(route.page().key, snake, "{route:?}");
        }
    }

    /// Every listed screen, and the ones Home's rows open, answers to its
    /// own link.
    #[test]
    fn every_listed_screen_opens_from_its_link() {
        let unlisted = [Route::ItemsDetail(3), Route::SignIn];
        for route in sidebar().iter().chain(&unlisted) {
            let path = route.path();
            let segments: Vec<&str> = if path.is_empty() {
                Vec::new()
            } else {
                path.split('/').collect()
            };
            assert_eq!(Route::parse(&segments), Some(*route), "{path}");
        }
    }
}
