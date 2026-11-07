//! The shell's index: every page reachable from here, as the smoke suite
//! walks it. Rows are the ui `nav_row`, whose test ids (`page-<key>`) let
//! the suite press them apart from the sidebar's same-named rows on
//! desktop widths.

use nsh_base::theme;
use nsh_core::{element::AnyElement, prelude::*};

use crate::{
    shell::{
        nav,
        routes::{Route, route_from_deep_link},
    },
    ui::nav::{chevron, nav_row},
};

/// Home's content, under the page's header.
pub fn content() -> AnyElement {
    scroll()
        .flex_1()
        .child(
            view()
                .gap_4()
                .p_4()
                .pb_safe_area_plus(px(16.0))
                .child(
                    view()
                        .gap_2()
                        .child(
                            text("Home")
                                .text_lg()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(move || theme().foreground),
                        )
                        .child(
                            text("Everything the boilerplate demonstrates, one page each.")
                                .text_sm()
                                .text_color(move || theme().muted_foreground),
                        ),
                )
                .child(
                    view().gap_1().child(section_label("Pages")).children(
                        Route::ALL
                            .iter()
                            .filter(|route| **route != Route::Home)
                            .map(|route| row(*route)),
                    ),
                )
                .child(view().gap_1().child(section_label("Pushed")).children([
                    demo_row("page-data-item", "Open data item 3", move || {
                        nav::push(Route::DataDetail(3))
                    }),
                    demo_row("page-deep-link", "Open a deep link", move || {
                        deep_link("nsh-starter://data/item/7")
                    }),
                ])),
        )
        .into_element()
}

/// Deep links from within the app: the same parse the host seam uses.
fn deep_link(url: &str) {
    if let Some(route) = url
        .strip_prefix("nsh-starter://")
        .and_then(|path| route_from_deep_link(&path.split('/').collect::<Vec<_>>()))
    {
        nav::open(route);
    }
}

fn section_label(label: &'static str) -> impl IntoElement {
    text(label)
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || theme().muted_foreground)
        .px_2()
        .pt_2()
}

/// A page's row: the label and a chevron, the whole row the target.
fn row(route: Route) -> AnyElement {
    demo_row(route.page_row_id(), route.label_str(), move || {
        nav::open(route)
    })
}

fn demo_row(id: &'static str, label: &'static str, open: impl Fn() + 'static) -> AnyElement {
    nav_row(label, Some(chevron()), false, id, open).into_element()
}
