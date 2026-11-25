//! The platform's own controls: a system button, switch and segmented
//! control drawn by UIKit on iOS and AppKit on macOS, registered from this
//! crate as native components (`#[native_components]` in `lib.rs`).
//!
//! Elsewhere (Android, Linux, Windows, the web, the headless tests) each
//! function falls back to the app's own styled control, so a screen can use
//! them everywhere. NSH 0.6.0 registers an app crate's native components on
//! iOS, macOS, Linux and Windows only (`nsh-macros/src/native_components.rs`); Android's come from the Java host.
//!
//! The native views keep the system font and metrics on purpose: they are
//! the system's controls. The app's accent tints them, so they sit with the
//! rest of the app.

use nsh_base::theme;
use nsh_core::{JsonValue, color::Rgba, element::NativeEvent, prelude::*};

use crate::ui::button::{ButtonSize, ButtonVariant, button_with, choice_button};

/// The system button's name among the native components.
pub const BUTTON: &str = "starter.native-button";
/// The system switch's name.
pub const SWITCH: &str = "starter.native-switch";
/// The system segmented control's name.
pub const SEGMENTED: &str = "starter.native-segmented";

/// A system button's look. iOS draws each as its `UIButton.Configuration`;
/// macOS maps them to its bezels (glass is the default push button there).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NativeButtonStyle {
    /// The accent fill: the one main action.
    Filled,
    /// An accent-tinted fill: a secondary action.
    Tinted,
    /// A neutral fill.
    Gray,
    /// Text only.
    Plain,
    /// iOS 26's Liquid Glass (gray on older systems and on macOS).
    Glass,
}

impl NativeButtonStyle {
    /// The style's key, as the native side reads the `style` prop.
    pub fn key(self) -> &'static str {
        match self {
            NativeButtonStyle::Filled => "filled",
            NativeButtonStyle::Tinted => "tinted",
            NativeButtonStyle::Gray => "gray",
            NativeButtonStyle::Plain => "plain",
            NativeButtonStyle::Glass => "glass",
        }
    }

    /// The app's own button variant the fallback draws.
    fn variant(self) -> ButtonVariant {
        match self {
            NativeButtonStyle::Filled => ButtonVariant::Primary,
            NativeButtonStyle::Tinted | NativeButtonStyle::Gray => ButtonVariant::Secondary,
            NativeButtonStyle::Plain => ButtonVariant::Ghost,
            NativeButtonStyle::Glass => ButtonVariant::Outline,
        }
    }
}

/// Whether this platform draws the controls natively (the crate registers
/// them on iOS and macOS).
pub fn is_native() -> bool {
    use nsh_core::platform::Os;
    // The build and the running platform both: the headless tests run on a
    // macOS host as an iOS phone, with no UIKit to draw these.
    (cfg!(target_os = "ios") && os() == Os::Ios) || (cfg!(target_os = "macos") && os() == Os::MacOs)
}

/// The theme's accent for the current appearance, as `[r, g, b]` from 0
/// to 1: the native side tints its control with it.
fn accent() -> JsonValue {
    let primary = theme().primary;
    let c = Rgba::from(if dark() { primary.dark } else { primary.light });
    JsonValue::from(vec![f64::from(c.r), f64::from(c.g), f64::from(c.b)])
}

/// A system button: `title` with an optional SF Symbol (`symbol`, iOS and
/// macOS 11+), in `style`; `on_press` runs on a press.
pub fn native_button(
    title: &'static str,
    style: NativeButtonStyle,
    symbol: Option<&'static str>,
    on_press: impl Fn() + 'static,
) -> AnyElement {
    if !is_native() {
        return button_with(
            title,
            style.variant(),
            ButtonSize::Medium,
            false,
            None,
            on_press,
        )
        .into_element();
    }
    nsh_core::element::native(BUTTON)
        .prop("title", JsonValue::from(title))
        .prop("style", JsonValue::from(style.key()))
        .prop("symbol", symbol.map_or(JsonValue::Null, JsonValue::from))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if &*e.name == "press" {
                on_press();
            }
        })
        .self_start()
        .into_element()
}

/// A labelled system switch writing `on`.
pub fn native_switch(label: &'static str, on: Signal<bool>) -> AnyElement {
    if !is_native() {
        return crate::ui::controls::switch_row(label, on).into_element();
    }
    view()
        .flex_row()
        .items_center()
        .justify_between()
        .gap_3()
        .child(crate::ui::text::body(label))
        .child(
            nsh_core::element::native(SWITCH)
                .prop("on", move || JsonValue::from(on.get()))
                .prop("tint", accent)
                .on_native_event(move |e: &NativeEvent| {
                    if &*e.name == "change" {
                        on.set(e.data["on"].as_bool().unwrap_or_default());
                    }
                }),
        )
        .into_element()
}

/// A system segmented control over `items`, writing the chosen index to
/// `selected`.
pub fn native_segmented(items: &'static [&'static str], selected: Signal<usize>) -> AnyElement {
    if !is_native() {
        return view()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .children(items.iter().enumerate().map(|(at, item)| {
                choice_button(
                    *item,
                    item,
                    move || selected.get() == at,
                    false,
                    move || selected.set(at),
                )
            }))
            .into_element();
    }
    nsh_core::element::native(SEGMENTED)
        .prop("items", JsonValue::from(items.to_vec()))
        .prop("selected", move || JsonValue::from(selected.get()))
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if &*e.name == "change"
                && let Some(at) = e.data["index"].as_u64()
            {
                selected.set(at as usize);
            }
        })
        .self_start()
        .into_element()
}

/// The `tint` prop as a color's three components.
#[cfg(any(target_os = "ios", target_os = "macos"))]
fn rgb(value: Option<&JsonValue>) -> Option<[f64; 3]> {
    let list = value?.as_array()?;
    Some([
        list.first()?.as_f64()?,
        list.get(1)?.as_f64()?,
        list.get(2)?.as_f64()?,
    ])
}

/// Which control a component is.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Button,
    Switch,
    Segmented,
}

/// The props a control keeps to rebuild itself as they change.
#[cfg(any(target_os = "ios", target_os = "macos"))]
struct Props {
    title: String,
    style: String,
    symbol: Option<String>,
    tint: Option<[f64; 3]>,
    items: Vec<String>,
    /// The segmented control's selected index (-1: none).
    selected: isize,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl Default for Props {
    fn default() -> Self {
        Props {
            title: String::new(),
            style: String::new(),
            symbol: None,
            tint: None,
            items: Vec::new(),
            selected: -1,
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl Props {
    /// Takes prop `name`; true when the control must be rebuilt.
    fn take(&mut self, name: &str, value: Option<&JsonValue>) -> bool {
        let text = || value.and_then(JsonValue::as_str).map(str::to_owned);
        match name {
            "title" => self.title = text().unwrap_or_default(),
            "style" => self.style = text().unwrap_or_default(),
            "symbol" => self.symbol = text(),
            "tint" => self.tint = rgb(value),
            "selected" => self.selected = value.and_then(JsonValue::as_i64).unwrap_or(-1) as isize,
            "items" => {
                self.items = value
                    .and_then(JsonValue::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|i| i.as_str().map(str::to_owned))
                            .collect()
                    })
                    .unwrap_or_default();
            }
            _ => return false,
        }
        true
    }
}

#[cfg(target_os = "ios")]
pub use ios::register as register_ios;

#[cfg(target_os = "macos")]
pub use macos::register as register_macos;

/// UIKit's controls.
#[cfg(target_os = "ios")]
mod ios;

/// AppKit's controls.
#[cfg(target_os = "macos")]
mod macos;
