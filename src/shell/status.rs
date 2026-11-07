//! The status bar at the bottom of the root: the app, the route on top
//! and the lifecycle phase, on every platform.

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::shell::{lifecycle, nav};

pub fn status_bar() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .justify_between()
        .px_3()
        .py_1()
        .pb_safe_area()
        .border_t_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .child(
            text(t!("app.name").to_string())
                .text_xs()
                .text_color(move || theme().muted_foreground),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    text(move || nav::current().label())
                        .text_xs()
                        .number_of_lines(1)
                        .text_color(move || theme().muted_foreground),
                )
                .child(
                    text(move || lifecycle::phase().get().name())
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                ),
        )
}
