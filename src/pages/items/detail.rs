//! One item's screen: its title, author, body and counts, read through its
//! own query key, with the same loading and error states as the list.

use nsh_core::prelude::*;

use super::queries::use_item;
use super::view::{error_state, reload_on_tick, skeletons};
use crate::services::fake_api::Post;
use crate::store::locale;
use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::layout::{self, BadgeTone};
use crate::ui::text;

/// The item's screen, as the shell pushes it; `id` comes from the route.
pub fn item_detail_page(id: u64) -> impl IntoElement {
    item_detail(id, None::<fn()>)
}

/// The item's content. `on_back` adds the bar the list's own presentation
/// closes it with (the shell's header has a back button of its own).
pub(super) fn item_detail(id: u64, on_back: Option<impl Fn() + 'static>) -> impl IntoElement {
    let item = use_item(id);
    let reloads = reload_on_tick(item);
    let loaded = move || item.with(|data| data.is_some());
    let field = move |read: fn(&Post) -> String| {
        move || item.with(|data| data.map(read).unwrap_or_default())
    };
    let body = layout::page_body()
        .when_some(on_back, |page, on_back| {
            page.child(view().flex_row().child(button_with(
                locale::text("items.back"),
                ButtonVariant::Ghost,
                ButtonSize::Small,
                false,
                None,
                on_back,
            )))
        })
        .child(when(
            move || !loaded() && !item.is_failure(),
            || skeletons(3),
        ))
        .child(when(
            move || !loaded() && item.is_failure(),
            move || error_state(move || item.error(), reloads),
        ))
        .child(when(loaded, move || {
            view()
                .test_id("items_detail-body")
                .gap_3()
                .child(text::title(field(|post| post.title.clone())))
                .child(text::muted(move || {
                    let by = locale::text("items.by")();
                    let author = field(|post| post.author.clone())();
                    SharedString::from(format!("{by} {author}"))
                }))
                .child(text::body(field(|post| post.body.clone())))
                .child(
                    view()
                        .flex_row()
                        .gap_2()
                        .child(layout::badge(
                            count(field(|post| post.likes.to_string()), "items.likes"),
                            BadgeTone::Neutral,
                        ))
                        .child(layout::badge(
                            count(field(|post| post.comments.to_string()), "items.comments"),
                            BadgeTone::Neutral,
                        )),
                )
        }));
    layout::page_scroll(0.0)
        .test_id("items_detail-scroll")
        .child(body)
}

/// "Likes 12": a translated label and its number.
fn count(
    number: impl Fn() -> String + 'static,
    unit: &'static str,
) -> impl Fn() -> SharedString + 'static {
    let unit = locale::text(unit);
    move || SharedString::from(format!("{} {}", unit(), number()))
}
