//! Home's layout: a card for every page, as the smoke suite walks them.
//! A card's test id is `page-<key>`, which tells it apart from the
//! sidebar's row for the same page (`nav-<key>`) on desktop widths.

use nsh_base::theme;
use nsh_core::{element::AnyElement, prelude::*};

use crate::{
    pages,
    routes::Route,
    store::nav,
    ui::{
        nav::{chevron, nav_row, nav_tile},
        text::muted,
    },
};

/// Home's content, under the page's header: a card per page, wrapping
/// into as many columns as the width holds (one on a phone).
pub fn home_page() -> AnyElement {
    crate::ui::layout::page_scroll(0.0)
        .test_id("home-scroll")
        .child(
            view()
                .gap_6()
                .p_4()
                .pb_safe_area_plus(px(16.0))
                // The header above already says "Home".
                .child(muted(
                    "Everything the boilerplate demonstrates, one page each.",
                ))
                .child(
                    view().gap_3().child(section_label("Pages")).child(
                        view().flex_row().flex_wrap().gap_3().children(
                            pages::sidebar()
                                .iter()
                                .filter(|route| **route != Route::Home)
                                .map(|route| tile(*route)),
                        ),
                    ),
                )
                // The rows under the cards, when the page table has any.
                .when(!pages::home_rows().is_empty(), |page| {
                    page.child(
                        view().gap_1().child(section_label("Pushed")).children(
                            pages::home_rows()
                                .iter()
                                .map(|(id, label, open)| demo_row(id, label, *open)),
                        ),
                    )
                }),
        )
        .into_element()
}

fn section_label(label: &'static str) -> impl IntoElement {
    text(label)
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || theme().muted_foreground)
}

/// A page's card on Home: `page-<key>` is its test id.
fn tile(route: Route) -> AnyElement {
    let page = route.page();
    nav_tile(
        page.title,
        page.summary,
        &format!("page-{}", page.key),
        move || nav::open(route),
    )
    .into_element()
}

fn demo_row(id: &'static str, label: &'static str, open: impl Fn() + 'static) -> AnyElement {
    nav_row(label, Some(chevron()), false, id, open).into_element()
}
