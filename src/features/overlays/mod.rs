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

use nsh_base::{ButtonElement, button, sheet, switch, theme};
use nsh_core::{element::StyleRefinement, prelude::*};

/// A local pressable with the app's tokens, until `crate::ui` lands its
/// `Button` wrapper (finding for the ui area).
pub(crate) fn overlay_button(
    label: &'static str,
    variant: ButtonVariant,
    on_press: impl Fn() + 'static,
) -> ButtonElement {
    button()
        .on_press(on_press)
        .min_h_9()
        .px_4()
        .py_2()
        .items_center()
        .justify_center()
        .rounded(move || theme().radius)
        .transition_colors()
        .when(variant == ButtonVariant::Primary, |b| {
            b.bg(move || theme().primary)
                .active(|s| s.bg(move || theme().primary.opacity(0.85)))
        })
        .when(variant == ButtonVariant::Outline, |b| {
            b.border_1()
                .border_color(move || theme().border)
                .bg(move || theme().background)
                .active(|s| s.bg(move || theme().accent))
        })
        .when(variant == ButtonVariant::Ghost, |b| {
            b.active(|s| s.bg(move || theme().accent))
        })
        .when(variant == ButtonVariant::Destructive, |b| {
            b.bg(move || theme().destructive)
                .active(|s| s.bg(move || theme().destructive.opacity(0.85)))
        })
        .focus(|s| s.outline(px(2.0)).outline_color(move || theme().ring))
        .disabled_style(|s| s.opacity(0.5))
        .child(
            text(label)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || match variant {
                    ButtonVariant::Primary => theme().primary_foreground,
                    ButtonVariant::Destructive => theme()
                        .color("destructive-foreground")
                        .unwrap_or(theme().primary_foreground),
                    _ => theme().foreground,
                }),
        )
}

/// The variants [`overlay_button`] draws.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ButtonVariant {
    Primary,
    Outline,
    Ghost,
    Destructive,
}

/// A labelled `Switch` with the app's tokens: the track and thumb draw
/// nothing until styled, so the look lives here until `crate::ui` lands
/// its `Switch` wrapper.
pub(crate) fn overlay_switch(label: &'static str, on: Signal<bool>) -> nsh_base::SwitchElement {
    switch()
        .checked(on)
        .on_change(move |next: &bool| on.set(*next))
        .flex_row()
        .items_center()
        .gap_3()
        .disabled_style(|s| s.opacity(0.5))
        .track_style(|s| {
            s.w(px(44.0))
                .p(px(2.0))
                .rounded_full()
                .bg(move || theme().muted)
                .transition_colors()
                .checked(|s| s.bg(move || theme().primary))
        })
        .thumb_style(|s| {
            s.size(px(20.0))
                .rounded_full()
                .bg(move || theme().background)
                .shadow_sm()
                .transition_transform()
                .checked(|s| s.translate_x(px(20.0)))
        })
        .child(text(label).text_sm().text_color(move || theme().foreground))
}

/// The popup family's one surface treatment — background, hairline,
/// radius, shadow — shared by menus, popovers and hover cards so they do
/// not drift apart.
pub(crate) fn popup_surface(s: StyleRefinement) -> StyleRefinement {
    s.min_w(px(192.0))
        .p_1()
        .rounded(move || theme().radius)
        .border_1()
        .border_color(move || theme().border)
        .bg(move || theme().popover)
        .shadow_lg()
}

/// A scrim over what a dialog or sheet covers, mixed from a token rather
/// than a literal (finding for the ui area: a `backdrop` token).
pub(crate) fn scrim(s: StyleRefinement) -> StyleRefinement {
    s.bg(move || theme().foreground.opacity(0.4))
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

/// A scratch root for probing the sheet's mount; removed with the probe
/// that uses it.
#[component]
pub fn OverlaysScrollProbe() -> impl IntoElement {
    let open = signal(false);
    let side = if std::env::var("PROBE_SIDE").is_ok() {
        Placement::Right
    } else {
        Placement::Bottom
    };
    view()
        .flex_1()
        .bg(move || theme().background)
        .flex_col()
        .child(
            view()
                .min_h_9()
                .px_4()
                .flex_row()
                .items_center()
                .bg(move || theme().primary)
                .test_id("probe-open-sheet")
                .role(AccessibilityRole::Button)
                .on_press(move || open.set(true))
                .child(text("Open sheet").text_sm()),
        )
        .child(when(open, {
            let open = open;
            move || {
                sheet()
                    .side(side)
                    .popup_style(|s| s.bg(move || theme().background).flex_col().gap_3().p_5())
                    .on_open_change({
                        let open = open;
                        move |e: &nsh_base::SheetEvent| {
                            if let nsh_base::SheetEvent::OpenChange(false, _) = e {
                                open.set(false);
                            }
                        }
                    })
                    .h(px(300.0))
                    .child(
                        text("Probe sheet body")
                            .text_sm()
                            .test_id("probe-sheet-body"),
                    )
            }
        }))
}
