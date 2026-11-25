//! The demo screens: what the starter shows off, one folder per section,
//! each built like a page of the app (`src/pages`). A new app deletes this
//! folder and the lines that name it in `src/routes.rs` and
//! `src/pages/mod.rs`.
//!
//! May import: `store`, `services`, `ui`, `native`, `routes`, `error`.
//! Never `shell`, `pages`, or another section's folder.

pub mod canvas;
pub mod data;
pub mod forms;
pub mod gallery;
pub mod lists;
pub mod motion;
pub mod native_vs_app;
pub mod overlays;
pub mod platform;
pub mod remote_screen;
mod routes;

pub use routes::Route;

use crate::routes::Page;
use crate::store::nav;

impl Route {
    /// The record of the screen this route shows. A new demo screen adds
    /// its arm here.
    pub fn page(self) -> &'static Page {
        match self {
            Route::Gallery => &gallery::PAGE,
            Route::NativeVsApp => &native_vs_app::PAGE,
            Route::Forms => &forms::PAGE,
            Route::Data => &data::PAGE,
            Route::DataItem(_) => &data::ITEM,
            Route::DataDevtools => &data::DEVTOOLS,
            Route::Lists => &lists::PAGE,
            Route::Canvas => &canvas::PAGE,
            Route::Motion => &motion::PAGE,
            Route::Overlays => &overlays::PAGE,
            Route::Platform => &platform::PAGE,
            Route::PlatformDevice => &platform::DEVICE,
            Route::RemoteScreen => &remote_screen::PAGE,
        }
    }
}

/// The demo screens the sidebar, Home and the palette list, in order.
pub const SIDEBAR: &[Route] = &[
    Route::Gallery,
    Route::NativeVsApp,
    Route::Forms,
    Route::Data,
    Route::DataDevtools,
    Route::Lists,
    Route::Canvas,
    Route::Motion,
    Route::Overlays,
    Route::Platform,
    Route::PlatformDevice,
    Route::RemoteScreen,
];

/// The demo screens on the phone's tab bar, after Home and before the
/// app's Items (`pages::tabs`).
pub const TABS: &[Route] = &[Route::Forms, Route::Data, Route::Motion];

/// Rows Home shows under its cards: a test id, a label and what a press
/// does. They demonstrate a pushed screen and a deep link.
pub const HOME_ROWS: &[(&str, &str, fn())] = &[
    ("page-data_item", "Open data item 3", || {
        nav::push(Route::DataItem(3).into())
    }),
    ("page-deep-link", "Open a deep link", || {
        // The same parse a link from outside the app goes through.
        if let Some(route) = crate::routes::Route::from_url("nsh-starter://data/item/7") {
            nav::open(route);
        }
    }),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pushed_screen_has_its_own_record() {
        assert_eq!(Route::DataItem(3).page().key, "data_item");
        assert_eq!(Route::Data.page().key, "data");
    }
}
