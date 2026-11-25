//! The data section's query keys, and the log of what each key did, which
//! the devtools screen lists.

use nsh_core::prelude::*;
use nsh_query::QueryKey;

use crate::services::fake_api;

/// The feed's infinite-query key.
pub fn feed_key() -> QueryKey {
    QueryKey::from("feed")
}

/// The detail query's key for one post.
pub fn post_key(id: u64) -> QueryKey {
    let id = id.to_string();
    QueryKey::new(["post", id.as_str()])
}

/// The comments query's key for one post.
pub fn comments_key(id: u64) -> QueryKey {
    let id = id.to_string();
    QueryKey::new(["comments", id.as_str()])
}

/// How long a detail query's answer stays fresh before it serves stale and
/// revalidates.
pub const DETAIL_TTL_MS: u64 = fake_api::DETAIL_TTL_MS;

/// What the devtools know about one key this feature loaded. nsh-query's
/// client cannot list its entries, so the fetchers record their own keys;
/// the cached data itself is read through `query_client`.
#[derive(Clone, PartialEq)]
pub struct QueryInfo {
    pub key: QueryKey,
    /// "loading", "loaded" or "error" at the last transition.
    pub status: &'static str,
    pub note: String,
    pub updated_ms: u64,
}

/// The query log's entries, one per key.
#[derive(Clone, Default, PartialEq)]
pub struct QueryLog(pub Vec<QueryInfo>);

/// The surface's query log.
pub fn query_log() -> Signal<QueryLog> {
    global::<QueryLog>()
}

/// Records one transition of `key` for the devtools page.
pub fn record_query(key: &QueryKey, status: &'static str, note: String) {
    let info = QueryInfo {
        key: key.clone(),
        status,
        note,
        // The engine's clock: what the queries' own timestamps and the
        // manual test clock use, so the devtools' ages agree with them.
        updated_ms: nsh_core::time::unix_ms() as u64,
    };
    query_log().update(
        |log| match log.0.iter_mut().find(|entry| entry.key == info.key) {
            Some(entry) => *entry = info,
            None => log.0.push(info),
        },
    );
}
