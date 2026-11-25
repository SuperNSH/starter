//! The data section's own state: which post the feed presents, the reload
//! asked from outside it, and the edits made since the loaded pages.

use std::collections::HashMap;

use nsh_core::prelude::*;

use crate::showcase::Route;
use crate::store::nav;

/// The post the feed presents over itself, if any.
#[derive(Clone, Default, PartialEq)]
struct Presented(Option<u64>);

/// The post the feed presents over itself, if any. Tracked.
pub fn presented_post() -> Option<u64> {
    global::<Presented>().get().0
}

/// Opens a post: as a screen of its own where the shell pushes screens
/// (phones), over the feed everywhere else.
pub fn open_post(id: u64) {
    if !nav::push_child(Route::DataItem(id)) {
        global::<Presented>().set(Presented(Some(id)));
    }
}

/// Closes a post the feed presented.
pub fn close_post() {
    global::<Presented>().set(Presented(None));
}

/// How many reloads were asked for from outside the feed.
#[derive(Clone, Default, PartialEq)]
struct Reloads(u32);

/// The count of reloads asked for from outside the feed (the header's
/// button on iOS). Tracked: the feed reloads when it grows.
pub fn reloads_asked() -> u32 {
    global::<Reloads>().get().0
}

/// Reloads the feed from its first page.
pub fn refresh_feed() {
    global::<Reloads>().update(|reloads| reloads.0 += 1);
}

/// One like write: the post and what its liked state becomes.
#[derive(Clone, PartialEq, Eq)]
pub struct LikeInput {
    pub post_id: u64,
    pub to: bool,
}

/// What the app knows about a post that the loaded feed pages predate: a
/// like or a comment count set through a mutation, keyed by post id. The
/// feed merges them over its pages, so an optimistic edit survives until a
/// fresh page carries the server truth itself.
#[derive(Clone, Default, PartialEq)]
pub struct FeedEdits {
    pub likes: HashMap<u64, (bool, u64)>,
    pub comment_counts: HashMap<u64, u64>,
}

/// The surface's feed edits.
pub fn feed_edits() -> Signal<FeedEdits> {
    global::<FeedEdits>()
}

/// Applies a like (an optimistic one first, the server's after it).
pub fn apply_like(post_id: u64, liked: bool, likes: u64) {
    feed_edits().update(|edits| {
        edits.likes.insert(post_id, (liked, likes));
    });
}

/// Drops a like's edit after its write failed.
pub fn revert_like(post_id: u64) {
    feed_edits().update(|edits| {
        edits.likes.remove(&post_id);
    });
}

/// Applies a comment count the server confirmed.
pub fn apply_comment_count(post_id: u64, count: u64) {
    feed_edits().update(|edits| {
        edits.comment_counts.insert(post_id, count);
    });
}
