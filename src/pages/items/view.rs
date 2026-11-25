//! The list: skeletons while it loads, the error with Retry, the empty
//! case, then a row per item. Where the shell pushes no screens (desktop
//! widths) the list presents the opened item over itself.

use nsh_base::theme;
use nsh_core::prelude::*;
use nsh_query::Query;

use super::detail::item_detail;
use super::queries::{error_key, use_items};
use crate::routes::Route;
use crate::services::fake_api::{ApiError, Post};
use crate::store::{locale, nav};
use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::{layout, nav::list_row, text};

/// The list page.
pub fn items_page() -> impl IntoElement {
    let items = use_items();
    // The item open over the list, where the shell did not push it.
    let presented = signal(None::<u64>);
    let reloads = reload_on_tick(items);
    let open = move |id: u64| {
        if !nav::push_child(Route::ItemsDetail(id)) {
            presented.set(Some(id));
        }
    };
    let list = layout::page_scroll(0.0).test_id("items-scroll").child(
        layout::page_body()
            .child(text::page_intro(
                locale::text("items.title"),
                locale::text("items.summary"),
            ))
            .child(list_body(items, reloads, open)),
    );
    view().flex_1().child(list).child(when(
        move || presented.get().is_some(),
        move || {
            // Tracked: another item presented while one is up rebuilds it.
            dynamic(move || {
                let id = presented.get().unwrap_or_default();
                untracked(|| {
                    view()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        // Below a system header (iOS at desktop widths): the
                        // safe area holds it, and is zero on the desktops.
                        .pt_safe_area()
                        .bg(move || theme().background)
                        .child(item_detail(id, Some(move || presented.set(None))))
                        .into_element()
                })
            })
        },
    ))
}

/// A counter whose every step refetches `query`, under the page: Retry's
/// button unmounts as the status flips, so it must not own the load.
pub(super) fn reload_on_tick<T: 'static>(query: Query<T, ApiError>) -> Signal<u32> {
    let reloads = signal(0_u32);
    effect(move || {
        if reloads.get() > 0 {
            untracked(|| query.refetch());
        }
    });
    reloads
}

/// The list under the intro, in whichever state the query is.
fn list_body(
    items: Query<Vec<Post>, ApiError>,
    reloads: Signal<u32>,
    open: impl Fn(u64) + Copy + 'static,
) -> impl IntoElement {
    let rows = computed(move || items.data().unwrap_or_default());
    let loaded = move || items.with(|data| data.is_some());
    view()
        .gap_2()
        .child(when(
            move || !loaded() && !items.is_failure(),
            || skeletons(4),
        ))
        .child(when(
            move || !loaded() && items.is_failure(),
            move || error_state(move || items.error(), reloads),
        ))
        .child(when(
            move || loaded() && rows.with(Vec::is_empty),
            || {
                view()
                    .test_id("items-empty")
                    .gap_1()
                    .child(text::section_title(locale::text("items.empty.title")))
                    .child(text::muted(locale::text("items.empty.hint")))
            },
        ))
        .child(each(
            rows,
            |post: &Post| post.id,
            move |post, _| {
                let post = post.get_untracked();
                let (id, author, likes) = (post.id, post.author, post.likes);
                list_row(
                    post.title,
                    move || {
                        let likes_label = locale::text("items.likes")();
                        SharedString::from(format!("{author} · {likes_label} {likes}"))
                    },
                    &format!("items-row-{id}"),
                    move || open(id),
                )
            },
        ))
}

/// Placeholder rows in the shape of what loads.
pub(super) fn skeletons(count: usize) -> impl IntoElement {
    view()
        .test_id("items-loading")
        .gap_2()
        .children((0..count).map(|_| layout::skeleton_of(56.0)))
}

/// What failed, and a way back. `error` is read in a binding: the query's
/// error lands a write after its status turns to failure.
pub(super) fn error_state(
    error: impl Fn() -> Option<ApiError> + 'static,
    reloads: Signal<u32>,
) -> impl IntoElement {
    view()
        .gap_2()
        .items_start()
        .child(text::section_title(locale::text("items.error.title")))
        .child(text::muted(move || {
            locale::text(error_key(error().as_ref()))()
        }))
        .child(button_with(
            locale::text("items.retry"),
            ButtonVariant::Primary,
            ButtonSize::Medium,
            false,
            None,
            move || reloads.update(|n| *n += 1),
        ))
}
