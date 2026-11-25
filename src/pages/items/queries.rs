//! The items screens' server reads: their query keys and fetchers, over the
//! fake API. A real backend replaces the two `fake_api` calls.

use std::time::Duration;

use nsh_query::{Query, QueryKey, QueryOptions, RetryPolicy, use_query_with};

use crate::services::fake_api::{self, ApiError, Post};

/// The list's key.
pub fn list_key() -> QueryKey {
    QueryKey::from("items")
}

/// One item's key. Not under `items`, so invalidating the list leaves the
/// items alone.
pub fn item_key(id: u64) -> QueryKey {
    let id = id.to_string();
    QueryKey::new(["item", id.as_str()])
}

/// Fresh for 30 seconds; one retry before the error shows.
fn options() -> QueryOptions {
    QueryOptions::new()
        .ttl(Duration::from_secs(30))
        .retry(RetryPolicy::new(1).with_delay(200))
}

/// The list: every item, in id order.
pub fn use_items() -> Query<Vec<Post>, ApiError> {
    use_query_with(list_key(), options(), |_context| fake_api::posts())
}

/// One item.
pub fn use_item(id: u64) -> Query<Post, ApiError> {
    use_query_with(item_key(id), options(), move |_context| fake_api::post(id))
}

/// The string that says why a read failed (`items.error.*`).
pub fn error_key(error: Option<&ApiError>) -> &'static str {
    match error {
        Some(ApiError::Offline) => "items.error.offline",
        Some(ApiError::NotFound) => "items.error.not_found",
        Some(ApiError::Failed) | None => "items.error.failed",
    }
}
