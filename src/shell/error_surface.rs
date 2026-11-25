//! The central error surface: the newest error `store::errors` holds, in
//! a banner over the bottom of the app.

use nsh_base::theme;
use nsh_core::element::AccessibilityRole;
use nsh_core::prelude::*;

use crate::shell::{desktop, status};
use crate::store::{errors, nav};
use crate::ui::{
    button::{ButtonSize, ButtonVariant, button_with},
    layout,
};

/// The space under the banner: it clears what the shell keeps at the
/// window's bottom (the status bar, or the tab bar where it shows) and the
/// home indicator.
fn clearance() -> f32 {
    let bar = if desktop() {
        status::HEIGHT
    } else if nav::current().parent().is_some() {
        0.0
    } else {
        layout::tab_bar_room()
    };
    bar + safe_area().bottom.0 + 12.0
}

/// The central error surface: a bottom overlay showing the newest queued
/// error, with any older ones counted behind it. The shell mounts it once,
/// above the rest of the app; a page never draws its own error banner.
/// Every part of the banner reads the queue, so a newer error replaces it
/// without the region being rebuilt.
pub fn error_surface() -> impl IntoElement {
    let errors = errors::errors();
    // Placed above the bars, not padded over them: padding would take the
    // tab bar's presses.
    view()
        .absolute()
        .bottom(move || px(clearance()))
        .left_0()
        .right_0()
        .child(when(
            move || errors.with(|queue| !queue.0.is_empty()),
            move || {
                view().px_4().child(
                    view()
                        .role(AccessibilityRole::Alert)
                        .w_full()
                        .flex_row()
                        .items_start()
                        .gap_3()
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(move || theme().destructive)
                        .bg(move || theme().card)
                        .shadow_sm()
                        .child(
                            view()
                                .flex_1()
                                .gap_1()
                                .child(
                                    text(move || {
                                        errors.with(|queue| {
                                            queue
                                                .0
                                                .last()
                                                .map(|e| e.message.clone())
                                                .unwrap_or_default()
                                        })
                                    })
                                    .text_sm()
                                    .text_color(move || theme().foreground),
                                )
                                .child(
                                    text(move || {
                                        errors.with(|queue| match queue.0.last() {
                                            None => String::new(),
                                            Some(error) => match (&error.detail, queue.0.len() - 1)
                                            {
                                                (Some(detail), _) => {
                                                    format!("{} · {}", error.code, detail)
                                                }
                                                (None, 0) => error.code.to_string(),
                                                (None, n) => {
                                                    format!("{} · {} earlier", error.code, n)
                                                }
                                            },
                                        })
                                    })
                                    .text_xs()
                                    .text_color(move || theme().muted_foreground),
                                ),
                        )
                        .child(
                            button_with(
                                "Dismiss",
                                ButtonVariant::Secondary,
                                ButtonSize::Small,
                                false,
                                Some("Dismiss"),
                                move || {
                                    let id = errors.with_untracked(|queue| {
                                        queue.0.last().map(|error| error.id)
                                    });
                                    if let Some(id) = id {
                                        errors::dismiss_error(id);
                                    }
                                },
                            )
                            .self_center(),
                        ),
                )
            },
        ))
}
