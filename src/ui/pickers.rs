//! Dates and times: the calendar, the date picker over it, and the time
//! field. On desktops the picker is a popup, on phones a sheet.

use nsh_base::{
    CalendarEntry, CalendarItemKind, CalendarValue, Date, DatePickerValue, DateTime, HourCycle,
    PickedDate, TimeFieldSegmentEntry, TimeOfDay, TimePrecision, calendar, date_picker, theme,
    time_field,
};
use nsh_core::prelude::*;

use crate::ui::popups::{popup_backdrop, popup_surface, popup_trigger_frame};

/// The day cells and month grid, drawn once per item, states by class.
const CALENDAR_ITEM_CLASS: &str = "size-8 items-center justify-center rounded-md text-xs \
     text-popover-foreground hover:bg-accent focus:bg-accent selected:bg-primary \
     selected:text-primary-foreground disabled:opacity-40";

fn cell_label(item: &CalendarEntry) -> String {
    match item.kind {
        // Previous and Next: the base draws its own chevron glyphs in the
        // header; the rendered item only needs the arrow.
        CalendarItemKind::Previous => "‹".to_owned(),
        CalendarItemKind::Next => "›".to_owned(),
        // `value` is a number for these (0 is Sunday, 1 is January).
        CalendarItemKind::Weekday => WEEKDAYS[item.value.rem_euclid(7) as usize].to_owned(),
        CalendarItemKind::MonthToggle | CalendarItemKind::Month => {
            MONTHS[(item.value - 1).rem_euclid(12) as usize].to_owned()
        }
        _ => item.value.to_string(),
    }
}

const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// A calendar cell: days, weekdays and arrows are squares, the toggles and
/// month and year cells as wide as their text.
fn cell_view(item: &CalendarEntry) -> View {
    let square = matches!(
        item.kind,
        CalendarItemKind::Day
            | CalendarItemKind::Weekday
            | CalendarItemKind::Previous
            | CalendarItemKind::Next
    );
    let cell = view().items_center().justify_center();
    let cell = if square {
        cell.size(px(32.0))
    } else {
        cell.h(px(32.0)).min_w(px(56.0)).px_2()
    };
    cell.child(text(cell_label(item)).text_xs())
}

/// A month calendar: its chosen date in `value` (`None`: nothing yet),
/// styled down to its day cells.
pub fn calendar_of(value: Signal<Option<Date>>) -> impl IntoElement {
    calendar()
        .value(move || CalendarValue::Controlled(value.get().unwrap_or_default()))
        .on_change(move |next: &Date| value.set(next.is_complete().then_some(*next)))
        .header_style(|s| {
            s.flex_row()
                .items_center()
                .justify_between()
                .px_2()
                .py_1p5()
                .gap_2()
        })
        .body_style(|s| s.p_2().gap_2())
        .month_style(|s| {
            s.text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || theme().muted_foreground)
        })
        .row_style(|s| s.flex_row().gap(px(2.0)))
        .item_class(CALENDAR_ITEM_CLASS)
        .render_item(cell_view)
}

/// A date picker: a trigger showing the picked date, a popup (a sheet on
/// phones) with the calendar over it, and a clear button.
pub fn date_picker_of(
    name: &'static str,
    value: Signal<Option<Date>>,
    placeholder: impl Into<String>,
) -> impl IntoElement {
    let placeholder = SharedString::from(placeholder.into());
    styled_date_picker(
        name,
        true,
        move || match value.get().and_then(|date| date.start()) {
            Some(date) => DateTime::from(date),
            None => DateTime::default(),
        },
        move |picked: &PickedDate| value.set(Some(picked.value.date())),
        move || match value.get().and_then(|date| date.start()) {
            Some(date) => format!("{date}"),
            None => placeholder.to_string(),
        },
    )
}

/// The app's date picker styling over any value: `value` is read by the
/// picker, `on_change` receives the pick, `label` is the trigger's text.
/// Forms use it so their popup (a sheet on phones) matches the gallery's.
pub fn styled_date_picker(
    name: &'static str,
    cleanable: bool,
    value: impl Fn() -> DateTime + 'static,
    on_change: impl Fn(&PickedDate) + 'static,
    label: impl Fn() -> String + 'static,
) -> impl IntoElement {
    date_picker()
        .accessibility_label(name)
        .cleanable(cleanable)
        .value(move || DatePickerValue::Controlled(value()))
        .on_change(on_change)
        .offset(px(4.0))
        .popup_style(|s| {
            // The base shows a sheet on touch screens, docked at the bottom
            // and as wide as its style says: full width there, not 320.
            if is_desktop() {
                popup_surface(s).w(px(320.0)).p_2()
            } else {
                popup_surface(s)
                    .w_full()
                    .rounded_t(move || theme().radius)
                    .rounded_b(px(0.0))
                    .p_2()
                    .pb_6()
            }
        })
        .backdrop_style(popup_backdrop)
        .content_style(|s| s.flex_col().gap_2())
        .header_style(|s| {
            s.flex_row()
                .items_center()
                .justify_between()
                .px_2()
                .py_1p5()
                .gap_2()
        })
        .body_style(|s| s.p_2().gap_2())
        .month_style(|s| {
            s.text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || theme().muted_foreground)
        })
        .row_style(|s| s.flex_row().gap(px(2.0)))
        .item_class(CALENDAR_ITEM_CLASS)
        .render_item(cell_view)
        .clear_button_class("rounded-md px-2 py-1.5 text-xs hover:bg-accent disabled:opacity-50")
        .presets_style(|s| s.flex_col().gap_1().px_2().pt_2())
        .preset_class(
            "rounded-md px-2 py-1.5 text-xs text-popover-foreground hover:bg-accent \
             selected:font-medium",
        )
        .trigger(popup_trigger_frame(label, true))
}

/// A time field at minutes precision, 24-hour: its segments are styled
/// here; the entered time is reported as `"HH:MM"` text in `value`.
pub fn time_field_of(name: &'static str, value: Signal<SharedString>) -> impl IntoElement {
    time_field()
        .accessibility_label(name)
        .precision(TimePrecision::Minute)
        .hour_cycle(HourCycle::H23)
        .flex_row()
        .items_center()
        .min_h(px(36.0))
        .rounded(move || theme().radius)
        .border_1()
        .border_color(move || theme().input)
        .bg(move || theme().background)
        .px_2()
        .gap_1()
        .on_change(move |time: &TimeOfDay| value.set(SharedString::from(format!("{}", time.0))))
        .segment_class(
            "min-w-7 px-1 py-1.5 rounded-sm text-sm text-popover-foreground text-center \
             hover:bg-accent focus:bg-accent selected:bg-primary selected:text-primary-foreground",
        )
        .separator_style(|s| s.text_sm().text_color(move || theme().muted_foreground))
        .render_segment(|segment: &TimeFieldSegmentEntry| {
            view().px_1().child(text(segment.label.clone()).text_sm())
        })
}
