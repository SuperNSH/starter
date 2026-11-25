//! More of the platform's own controls, beside `native::controls`'s button,
//! switch and segmented control: slider, checkbox, radio group, pop-up
//! button, stepper, progress, spinner, date picker, color well, text and
//! search fields, and tabs. AppKit draws them on macOS, UIKit on iOS where
//! it has the control; everywhere else (and for the iOS gaps: checkbox,
//! radio group, tabs) each function falls back to the app's own control,
//! bound to the same signal, so a screen can swap one for the other.
//!
//! Alerts and action sheets need no component: `nsh_base::alert_dialog`
//! already shows an `NSAlert` sheet on macOS and a `UIAlertController` on
//! iOS (see [`native_alert`]).

use nsh_base::{Date, theme};
use nsh_core::{JsonValue, color::Rgba, element::NativeEvent, prelude::*};

use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::{controls, field, pickers, select, tabs, text};

/// The components' names among the native components.
pub const SLIDER: &str = "starter.kit-slider";
pub const CHECKBOX: &str = "starter.kit-checkbox";
pub const RADIO: &str = "starter.kit-radio";
pub const POPUP: &str = "starter.kit-popup";
pub const STEPPER: &str = "starter.kit-stepper";
pub const PROGRESS: &str = "starter.kit-progress";
pub const SPINNER: &str = "starter.kit-spinner";
pub const DATE: &str = "starter.kit-date";
pub const COLOR: &str = "starter.kit-color";
pub const TEXT: &str = "starter.kit-text";
pub const SEARCH: &str = "starter.kit-search";
pub const TABS: &str = "starter.kit-tabs";
pub const GLASS_BUTTON: &str = "starter.kit-glass-button";
pub const GLASS_BAR: &str = "starter.kit-glass-bar";
pub const MATERIAL: &str = "starter.kit-material";

/// AppKit draws these controls here.
pub fn on_macos() -> bool {
    cfg!(target_os = "macos") && os() == nsh_core::platform::Os::MacOs
}

/// UIKit draws these controls here.
pub fn on_ios() -> bool {
    cfg!(target_os = "ios") && os() == nsh_core::platform::Os::Ios
}

fn rgb_of(color: Hsla) -> JsonValue {
    let c = Rgba::from(color);
    JsonValue::from(vec![f64::from(c.r), f64::from(c.g), f64::from(c.b)])
}

fn accent() -> JsonValue {
    let primary = theme().primary;
    rgb_of(if dark() { primary.dark } else { primary.light })
}

fn number(value: f64) -> JsonValue {
    serde_json::Number::from_f64(value).map_or(JsonValue::Null, JsonValue::Number)
}

/// A date at noon UTC, as seconds since 1970: noon keeps the same day in
/// every time zone the native picker might show it in.
fn epoch_of(date: Option<Date>) -> JsonValue {
    date.and_then(|d| d.start())
        .and_then(|d| d.and_hms_opt(12, 0, 0))
        .map_or(JsonValue::Null, |d| number(d.and_utc().timestamp() as f64))
}

fn date_of(epoch: f64) -> Option<Date> {
    chrono::DateTime::from_timestamp(epoch as i64, 0).map(|d| Date::from(d.date_naive()))
}

/// A system slider from `min` to `max`, writing `value`.
pub fn native_slider(value: Signal<f64>, min: f64, max: f64) -> AnyElement {
    if !(on_macos() || on_ios()) {
        return view()
            .w_full()
            .child(controls::slider(value, min, max))
            .into_element();
    }
    nsh_core::element::native(SLIDER)
        .prop("min", number(min))
        .prop("max", number(max))
        .prop("value", move || number(value.get()))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if let Some(v) = e.data["value"].as_f64() {
                value.set(v);
            }
        })
        .w_full()
        .into_element()
}

/// A labelled checkbox (AppKit's; iOS has none, so the app's draws there).
pub fn native_checkbox(label: &'static str, on: Signal<bool>) -> AnyElement {
    if !on_macos() {
        return controls::checkbox_row(label, on).into_element();
    }
    nsh_core::element::native(CHECKBOX)
        .prop("title", JsonValue::from(label))
        .prop("on", move || JsonValue::from(on.get()))
        .on_native_event(move |e: &NativeEvent| {
            if let Some(next) = e.data["on"].as_bool() {
                on.set(next);
            }
        })
        .self_start()
        .into_element()
}

/// A radio group over `options` writing the chosen label (AppKit's radio
/// buttons; iOS has none).
pub fn native_radio_group(
    options: &'static [&'static str],
    value: Signal<SharedString>,
) -> AnyElement {
    if !on_macos() {
        return controls::radio_group_control(options, value).into_element();
    }
    nsh_core::element::native(RADIO)
        .prop("items", JsonValue::from(options.to_vec()))
        .prop("selected", move || {
            let picked = value.get();
            JsonValue::from(
                options
                    .iter()
                    .position(|o| *o == picked.as_ref())
                    .map_or(-1, |at| at as i64),
            )
        })
        .on_native_event(move |e: &NativeEvent| {
            if let Some(o) = e.data["index"]
                .as_u64()
                .and_then(|at| options.get(at as usize))
            {
                value.set(SharedString::from(*o));
            }
        })
        .self_start()
        .into_element()
}

/// A pop-up button over `options`: `NSPopUpButton` on macOS, a `UIButton`
/// whose menu changes its selection on iOS.
pub fn native_popup(
    name: &'static str,
    options: &'static [&'static str],
    value: Signal<Option<SharedString>>,
) -> AnyElement {
    if !(on_macos() || on_ios()) {
        let items = options.iter().map(|o| select::choice(o, o)).collect();
        return select::select_field(name, items, value, "Choose").into_element();
    }
    nsh_core::element::native(POPUP)
        .prop("items", JsonValue::from(options.to_vec()))
        .prop("selected", move || {
            let picked = value.get();
            JsonValue::from(
                options
                    .iter()
                    .position(|o| Some(*o) == picked.as_deref())
                    .map_or(0, |at| at as i64),
            )
        })
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if let Some(o) = e.data["index"]
                .as_u64()
                .and_then(|at| options.get(at as usize))
            {
                value.set(Some(SharedString::from(*o)));
            }
        })
        .self_start()
        .into_element()
}

/// A stepper nudging `value` by `step` within `min..=max`.
pub fn native_stepper(value: Signal<f64>, min: f64, max: f64, step: f64) -> AnyElement {
    if !(on_macos() || on_ios()) {
        let nudge = move |by: f64| value.set((value.get() + by).clamp(min, max));
        return view()
            .flex_row()
            .gap_1()
            .child(button_with(
                "−",
                ButtonVariant::Outline,
                ButtonSize::Small,
                false,
                Some("Decrease"),
                move || nudge(-step),
            ))
            .child(button_with(
                "+",
                ButtonVariant::Outline,
                ButtonSize::Small,
                false,
                Some("Increase"),
                move || nudge(step),
            ))
            .into_element();
    }
    nsh_core::element::native(STEPPER)
        .prop("min", number(min))
        .prop("max", number(max))
        .prop("step", number(step))
        .prop("value", move || number(value.get()))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if let Some(v) = e.data["value"].as_f64() {
                value.set(v);
            }
        })
        .self_start()
        .into_element()
}

/// A determinate progress bar at `value` (0 to 1).
pub fn native_progress(value: impl Fn() -> f64 + Copy + 'static) -> AnyElement {
    if !(on_macos() || on_ios()) {
        // The app's bar runs from 0 to 100.
        return controls::progress(move || (value() * 100.0) as f32).into_element();
    }
    nsh_core::element::native(PROGRESS)
        .prop("value", move || number(value()))
        .prop("tint", accent)
        .w_full()
        .into_element()
}

/// The system's activity indicator.
pub fn native_spinner() -> AnyElement {
    if !(on_macos() || on_ios()) {
        return controls::progress_indeterminate().into_element();
    }
    nsh_core::element::native(SPINNER)
        .self_start()
        .into_element()
}

/// A system date picker writing `value`: compact (a field with a stepper
/// on macOS, a button opening a calendar on iOS), or the full `calendar`.
pub fn native_date(value: Signal<Option<Date>>, calendar: bool) -> AnyElement {
    if !(on_macos() || on_ios()) {
        return if calendar {
            pickers::calendar_of(value).into_element()
        } else {
            pickers::date_picker_of("Date", value, "Pick a date").into_element()
        };
    }
    nsh_core::element::native(DATE)
        .prop(
            "style",
            JsonValue::from(if calendar { "calendar" } else { "compact" }),
        )
        .prop("value", move || epoch_of(value.get()))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if let Some(epoch) = e.data["value"].as_f64() {
                value.set(date_of(epoch));
            }
        })
        .self_start()
        .into_element()
}

/// The system color well: `NSColorWell`, `UIColorWell`. Opens the
/// system's color panel or sheet.
pub fn native_color_well(value: Signal<Hsla>) -> AnyElement {
    if !(on_macos() || on_ios()) {
        return crate::ui::color::color_picker_of(value, |_| {}).into_element();
    }
    nsh_core::element::native(COLOR)
        .prop("color", move || rgb_of(value.get()))
        .on_native_event(move |e: &NativeEvent| {
            let c = &e.data["rgb"];
            if let (Some(r), Some(g), Some(b)) = (c[0].as_f64(), c[1].as_f64(), c[2].as_f64()) {
                value.set(Hsla::from(Rgba {
                    r: r as f32,
                    g: g as f32,
                    b: b as f32,
                    a: 1.0,
                }));
            }
        })
        .self_start()
        .into_element()
}

/// A system text field (`search`: the search field, with its glass and
/// clear button) writing `value` as the user types.
pub fn native_text_field(
    value: Signal<String>,
    placeholder: &'static str,
    search: bool,
) -> AnyElement {
    if !(on_macos() || on_ios()) {
        return field::text_field(value, placeholder).into_element();
    }
    nsh_core::element::native(if search { SEARCH } else { TEXT })
        .prop("placeholder", JsonValue::from(placeholder))
        .prop("text", move || JsonValue::from(value.get()))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if let Some(next) = e.data["text"].as_str()
                && next != value.get()
            {
                value.set(next.to_owned());
            }
        })
        .w_full()
        .into_element()
}

/// Tabs over `labels`, each showing its line of `bodies`: on macOS a box
/// headed by a segmented control, as `NSTabView` draws them; iOS has no tab control inside a page, so its segmented control
/// heads the body there; elsewhere the app's tab row.
pub fn native_tabs(
    labels: &'static [&'static str],
    bodies: &'static [&'static str],
    selected: Signal<usize>,
) -> AnyElement {
    let body = move || {
        view().p_3().child(text::body(move || {
            SharedString::from(*bodies.get(selected.get()).unwrap_or(&""))
        }))
    };
    if on_ios() {
        return view()
            .flex_col()
            .gap_2()
            .child(crate::native::controls::native_segmented(labels, selected))
            .child(body())
            .into_element();
    }
    if !on_macos() {
        return view()
            .flex_col()
            .gap_2()
            .child(tabs::tab_bar(
                move || selected.get(),
                labels,
                move |at| selected.set(at),
            ))
            .child(body())
            .into_element();
    }
    nsh_core::element::native(TABS)
        .prop("items", JsonValue::from(labels.to_vec()))
        .prop("bodies", JsonValue::from(bodies.to_vec()))
        .prop("selected", move || JsonValue::from(selected.get()))
        .on_native_event(move |e: &NativeEvent| {
            if let Some(at) = e.data["index"].as_u64()
                && selected.get() != at as usize
            {
                selected.set(at as usize);
            }
        })
        .w_full()
        .h(px(150.0))
        .into_element()
}

mod surfaces;

pub use surfaces::*;

#[cfg(any(target_os = "ios", target_os = "macos"))]
mod props;

#[cfg(any(target_os = "ios", target_os = "macos"))]
use props::{KINDS, Kind, Props};

#[cfg(target_os = "macos")]
pub use macos::register as register_macos;

#[cfg(target_os = "ios")]
pub use ios::register as register_ios;

/// AppKit's controls.
#[cfg(target_os = "macos")]
mod macos;

/// UIKit's controls. Checkbox, radio group and tabs have no UIKit control;
/// the Rust side never asks for them on iOS.
#[cfg(target_os = "ios")]
mod ios;
