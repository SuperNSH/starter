//! Picking from a list: the select (one of many) and the combobox (type
//! to filter). On desktops a popup at the trigger, on phones a sheet.

use nsh_base::{SelectEntry, SelectItem, combobox, select, theme};
use nsh_core::prelude::*;

use crate::ui::popups::{popup_backdrop, popup_surface, popup_trigger_frame};

/// One choice of a [`select_field`] or [`combobox_field`].
pub fn choice(value: &'static str, title: &'static str) -> SelectItem {
    SelectItem::new(value, title)
}

/// A select: one of `items`, its choice in `value` (`None`: nothing yet).
/// The closed control is the app's trigger; the popup is styled here.
pub fn select_field(
    name: &'static str,
    items: Vec<SelectItem>,
    value: Signal<Option<SharedString>>,
    placeholder: impl Into<String>,
) -> impl IntoElement {
    let placeholder = placeholder.into();
    let titles = items
        .iter()
        .map(|item| (item.value.clone(), item.title.clone()))
        .collect::<Vec<_>>();
    let chosen = move || {
        value.get().as_ref().and_then(|picked| {
            titles
                .iter()
                .find(|(picked_value, _)| picked_value == picked)
                .map(|(_, title)| title.to_string())
        })
    };
    select()
        .items(items)
        .value(move || value.get())
        .on_change(move |next: &Option<SharedString>| value.set(next.clone()))
        .accessibility_label(name)
        .offset(px(4.0))
        .popup_style(popup_surface)
        .backdrop_style(popup_backdrop)
        .item_class("rounded-md focus:bg-accent selected:font-medium disabled:opacity-50")
        .search_style(|s| {
            s.h(px(36.0))
                .px_3()
                .text_sm()
                .border_b_1()
                .border_color(move || theme().border)
        })
        .render_item(|item: &SelectEntry| {
            view().px_2().py_1p5().child(
                text(item.title.clone())
                    .text_sm()
                    .text_color(move || theme().popover_foreground),
            )
        })
        .trigger(popup_trigger_frame(
            move || chosen().unwrap_or_else(|| placeholder.to_string()),
            true,
        ))
}

/// A combobox: type to filter, possibly several choices in `value`.
pub fn combobox_field(
    name: &'static str,
    items: Vec<SelectItem>,
    value: Signal<Vec<SharedString>>,
    placeholder: impl Into<String>,
) -> impl IntoElement {
    let placeholder = SharedString::from(placeholder.into());
    let titles = items
        .iter()
        .map(|item| (item.value.clone(), item.title.clone()))
        .collect::<Vec<_>>();
    combobox()
        .items(items)
        .value(move || Some(value.get()))
        .on_change(move |next: &Vec<SharedString>| value.set(next.clone()))
        .accessibility_label(name)
        .placeholder({
            let placeholder = placeholder.clone();
            move || placeholder.clone()
        })
        .offset(px(4.0))
        .popup_style(popup_surface)
        .backdrop_style(popup_backdrop)
        .item_class("rounded-md focus:bg-accent selected:font-medium disabled:opacity-50")
        .input_style(|s| {
            s.h(px(36.0))
                .px_3()
                .text_sm()
                .text_color(move || theme().popover_foreground)
        })
        .footer_style(|s| s.border_t_1().border_color(move || theme().border))
        .trigger_button(popup_trigger_frame(
            move || {
                value
                    .get()
                    .first()
                    .and_then(|picked| {
                        titles
                            .iter()
                            .find(|(picked_value, _)| picked_value == picked)
                            .map(|(_, title)| title.to_string())
                    })
                    .unwrap_or_else(|| placeholder.to_string())
            },
            true,
        ))
}
