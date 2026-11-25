//! The control gallery: every wrapper in `crate::ui`, shown with its
//! states, in a scrolled, sectioned layout that renders in light and dark
//! and at phone and desktop width.

use nsh_base::dock::{DockAreaLayout, DockLayout};
use nsh_base::theme;
use nsh_core::prelude::*;

use crate::ui::{
    avatar, button, carousel, collapse, color, command, controls, dialog, dock, editor, field,
    layout, link, number, pagination, pickers, popups, resizable, select, sheet, table, tabs, text,
    toast, tree,
};

/// The UI gallery page.
pub fn gallery_page() -> impl IntoElement {
    let open_dialog = signal(false);
    let open_sheet = signal(false);
    let open_form_sheet = signal(false);
    let open_full_dialog = signal(false);
    let deletable = signal(false);
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            scroll()
                .flex_1()
                .content_inset(crate::ui::layout::page_insets(0.0))
                .test_id("gallery-scroll")
                .child(
                    layout::page_body()
                        .max_w(px(720.0))
                        .child(text::page_intro(
                            "UI gallery",
                            "Every control of the app's own library, with its states. \
                         Hover, press, focus and disable each one.",
                        ))
                        .child(buttons_section())
                        .child(native_section())
                        .child(choices_section())
                        .child(text_section())
                        .child(fields_section())
                        .child(controls_section())
                        .child(selection_section())
                        .child(nav_rows_section())
                        .child(pickers_section())
                        .child(overlays_section(
                            open_dialog,
                            open_sheet,
                            open_form_sheet,
                            open_full_dialog,
                            deletable,
                        ))
                        .child(data_section())
                        .child(surfaces_section())
                        .child(link_section()),
                ),
        )
        .child(toast::toast_host())
        .child(dialog::dialog_sheet(
            open_form_sheet,
            move || open_form_sheet.set(false),
            || {
                view()
                    .flex_col()
                    .gap_3()
                    .child(text::section_title("Invite a teammate"))
                    .child(text::muted(
                        "A page sheet: the presenting page stays visible behind it.",
                    ))
                    .into_element()
            },
        ))
        .child(dialog::dialog_of(
            open_full_dialog,
            move || open_full_dialog.set(false),
            move || {
                view()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .child(text::section_title("Review required"))
                    .child(text::body(
                        "A full-screen dialog covers the page until it is dismissed.",
                    ))
                    .child(button::button_with(
                        "Done",
                        button::ButtonVariant::Primary,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || open_full_dialog.set(false),
                    ))
                    .into_element()
            },
        ))
        .child(dialog::dialog_card(
            open_dialog,
            "Delete the draft?".into(),
            move || open_dialog.set(false),
            || {
                text::body("The draft and its autosave go with it. This cannot be undone.")
                    .into_element()
            },
            move || {
                view()
                    .flex_row()
                    .justify_end()
                    .gap_2()
                    .child(button::button_with(
                        "Cancel",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || open_dialog.set(false),
                    ))
                    .child(button::button_with(
                        "Delete",
                        button::ButtonVariant::Danger,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || {
                            deletable.set(true);
                            open_dialog.set(false);
                        },
                    ))
                    .into_element()
            },
        ))
        .child(sheet::sheet_of(open_sheet, Placement::Bottom, || {
            view()
                .flex_col()
                .gap_3()
                .child(text::section_title("Filters"))
                .child(controls::switch_row("Only unread", signal(false)))
                .child(controls::switch_row("Sort by date", signal(true)))
                .into_element()
        }))
}

mod basics;
mod inputs;
mod surfaces;

use basics::*;
use inputs::*;
use surfaces::*;
