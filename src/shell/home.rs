//! The shell's index: every page reachable from here, as the smoke suite
//! walks it. Rows carry test ids (`page-<key>`) so the suite can press
//! them apart from the sidebar's same-named rows on desktop widths; a
//! component element takes no test id of its own, so the wrapper carries
//! it and a press reaches the row's button.

use nsh_base::{button, theme};
use nsh_core::{element::AnyElement, prelude::*};

use crate::shell::{
    nav,
    routes::{Route, route_from_deep_link},
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
                    demo_row("page-data-item", "Open data item 3".into(), move || {
                        nav::push(Route::DataDetail(3))
                    }),
                    demo_row("page-deep-link", "Open a deep link".into(), move || {
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
    demo_row(&format!("page-{}", route.key()), route.label(), move || {
        nav::open(route)
    })
}

fn demo_row(id: &str, label: String, open: impl Fn() + 'static) -> AnyElement {
    button()
        .on_press(open)
        .accessibility_label(label.clone())
        .h_11()
        .w_full()
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .rounded(move || theme().radius)
        .hover(|s| s.bg(move || theme().accent))
        .active(|s| s.opacity(0.7))
        .focus(|s| s.outline(px(2.0)).outline_color(move || theme().ring))
        .child(
            // The row's test id rides this view, inside the button: a
            // press on it reaches the button, as a tap on a label does.
            view()
                .test_id(id)
                .flex_1()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    text(label)
                        .flex_1()
                        .text_sm()
                        .number_of_lines(1)
                        .text_color(move || theme().foreground),
                )
                .child(text("›").text_color(move || theme().muted_foreground)),
        )
        .into_element()
}
