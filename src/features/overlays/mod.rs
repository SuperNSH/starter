//! Overlays and desktop surfaces: `Dialog`, `AlertDialog`, `Sheet`,
//! `Popover`, `Menu`, `ContextMenu`, `HoverCard`, `Tooltip`, toasts,
//! `DockArea`, resizable panels, `Editor`, `DiffView`, `TextView`. Owned
//! by the overlays area. The shell mounts `overlays_page`; keep its name
//! and signature.

mod code_sections;
mod dialog_section;
mod dock_section;
mod hovering;
mod menus;
mod panels;
mod sheet_section;
mod toasts;

use crate::ui::button as ui_button;
use crate::ui::controls as ui_controls;
use crate::ui::field as ui_field;
use crate::ui::popups as ui_popups;
use nsh_base::ButtonElement;
use nsh_base::theme;
use nsh_core::{element::StyleRefinement, prelude::*};

/// A section's button, forwarded to `crate::ui`'s with this area's
/// variant names for the destructive commitment.
pub(crate) fn overlay_button(
    label: &'static str,
    variant: ButtonVariant,
    on_press: impl Fn() + 'static,
) -> ButtonElement {
    ui_button::button_with(
        label,
        match variant {
            ButtonVariant::Primary => ui_button::ButtonVariant::Primary,
            ButtonVariant::Outline => ui_button::ButtonVariant::Outline,
            ButtonVariant::Ghost => ui_button::ButtonVariant::Ghost,
            ButtonVariant::Destructive => ui_button::ButtonVariant::Danger,
        },
        ui_button::ButtonSize::Medium,
        false,
        None,
        on_press,
    )
}

/// The variants [`overlay_button`] forwards.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ButtonVariant {
    Primary,
    Outline,
    Ghost,
    Destructive,
}

/// A section's switch row, forwarded to `crate::ui`.
pub(crate) fn overlay_switch(label: &'static str, on: Signal<bool>) -> impl IntoElement {
    ui_controls::switch_row(label, on)
}

/// A section's single-line field, `crate::ui`'s.
pub(crate) fn text_field(value: Signal<String>, placeholder: impl Into<String>) -> nsh_base::Input {
    ui_field::text_field(value, placeholder)
}

/// The popup family's one surface, `crate::ui`'s.
pub(crate) fn popup_surface(s: StyleRefinement) -> StyleRefinement {
    ui_popups::popup_surface(s)
}

/// A scrim over what a dialog or sheet covers, `crate::ui`'s token-backed
/// backdrop.
pub(crate) fn scrim(s: StyleRefinement) -> StyleRefinement {
    ui_popups::popup_backdrop(s)
}

/// Secondary reading inside a section, static or following a signal.
pub(crate) fn caption(note: impl IntoReactive<SharedString> + 'static) -> impl IntoElement {
    text(note)
        .text_xs()
        .text_color(move || theme().muted_foreground)
}

/// A status line a flow writes to, so tests and people see the outcome.
pub(crate) fn status_line(value: impl Fn() -> String + 'static) -> impl IntoElement {
    text(value)
        .text_sm()
        .font_family("monospace")
        .text_color(move || theme().muted_foreground)
}

/// A trailing shortcut hint of a menu row.
pub(crate) fn shortcut(keys: &'static str) -> impl IntoElement {
    text(keys)
        .text_xs()
        .font_family("monospace")
        .text_color(move || theme().muted_foreground)
}

/// The overlays page: every overlay and desktop surface, each in a
/// section that runs a real flow. Toasts are hosted once, at the end.
pub fn overlays_page() -> impl IntoElement {
    view().flex_1().bg(move || theme().background).child(
        scroll().flex_1().child(
            view().w_full().flex_col().items_center().child(
                view()
                    .w_full()
                    .max_w(px(720.0))
                    .flex_col()
                    .gap_6()
                    .p_safe_area()
                    .p_4()
                    .child(
                        text("Overlays")
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(move || theme().foreground),
                    )
                    .child(caption(
                        "Dialogs, popups, menus, toasts and desktop surfaces, each in a flow.",
                    ))
                    .child(dialog_section::dialog_section())
                    .child(dialog_section::alert_section())
                    .child(sheet_section::sheet_section())
                    .child(menus::popover_section())
                    .child(menus::menu_section())
                    .child(menus::context_menu_section())
                    .child(hovering::hover_card_section())
                    .child(hovering::tooltip_section())
                    .child(toasts::toasts_area())
                    .child(dock_section::dock_section())
                    .child(panels::panels_section())
                    .child(code_sections::editor_section())
                    .child(code_sections::diff_section())
                    .child(code_sections::text_section()),
            ),
        ),
    )
}

/// A root that mounts just this page, so `tests/overlays.rs` runs it
/// without the shell.
#[component]
pub fn OverlaysTestRoot() -> impl IntoElement {
    overlays_page()
}
