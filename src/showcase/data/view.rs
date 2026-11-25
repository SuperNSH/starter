//! The data section's three screens, as the shell mounts them.

use nsh_core::prelude::*;

use super::{devtools, feed, item, presented_post};

/// The feed page: an infinite, virtualized, pull-to-refresh feed of posts.
/// A post opened where the shell does not push screens is presented over
/// the feed until it is closed, so the feed's pages and scroll position
/// survive the round trip.
pub fn data_page() -> impl IntoElement {
    view().flex_1().child(feed::feed_page()).child(when(
        move || presented_post().is_some(),
        move || {
            // Tracked: a different id presented while the overlay is up
            // rebuilds the post for that id instead of showing the old one.
            dynamic(move || {
                let id = presented_post().unwrap_or(0);
                untracked(|| {
                    view()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .child(item::post_detail(id, true))
                        .into_element()
                })
            })
            .into_element()
        },
    ))
}

/// The screen of one post; `id` comes from the route.
pub fn data_item_page(id: u64) -> impl IntoElement {
    item::post_detail(id, false)
}

/// The query devtools: cache entries and the fake API's switches.
pub fn data_devtools_page() -> impl IntoElement {
    devtools::devtools_page()
}
