//! What the native side reads from an element's props, shared by AppKit's
//! and UIKit's controls.

use nsh_core::JsonValue;

use super::*;

/// The props a component keeps, to rebuild itself as they change.
#[derive(Default)]
pub(super) struct Props {
    pub(super) title: String,
    pub(super) items: Vec<String>,
    pub(super) bodies: Vec<String>,
    pub(super) value: f64,
    pub(super) min: f64,
    pub(super) max: f64,
    pub(super) step: f64,
    pub(super) selected: isize,
    pub(super) on: bool,
    pub(super) text: String,
    pub(super) placeholder: String,
    pub(super) style: String,
    pub(super) symbol: String,
    pub(super) tint: Option<[f64; 3]>,
    pub(super) color: Option<[f64; 3]>,
    /// A date value was given (`value` holds its epoch seconds).
    pub(super) has_date: bool,
}

pub(super) fn rgb(value: Option<&JsonValue>) -> Option<[f64; 3]> {
    let list = value?.as_array()?;
    Some([
        list.first()?.as_f64()?,
        list.get(1)?.as_f64()?,
        list.get(2)?.as_f64()?,
    ])
}

pub(super) fn strings(value: Option<&JsonValue>) -> Vec<String> {
    value
        .and_then(JsonValue::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|i| i.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

impl Props {
    pub(super) fn new() -> Self {
        Props {
            max: 1.0,
            step: 1.0,
            selected: -1,
            ..Props::default()
        }
    }

    /// Takes prop `name`; false for a prop it does not know.
    pub(super) fn take(&mut self, name: &str, value: Option<&JsonValue>) -> bool {
        let text = || {
            value
                .and_then(JsonValue::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        let num = || value.and_then(JsonValue::as_f64);
        match name {
            "title" => self.title = text(),
            "text" => self.text = text(),
            "placeholder" => self.placeholder = text(),
            "style" => self.style = text(),
            "symbol" => self.symbol = text(),
            "items" => self.items = strings(value),
            "bodies" => self.bodies = strings(value),
            "value" => {
                self.has_date = num().is_some();
                self.value = num().unwrap_or_default();
            }
            "min" => self.min = num().unwrap_or_default(),
            "max" => self.max = num().unwrap_or(1.0),
            "step" => self.step = num().unwrap_or(1.0),
            "selected" => self.selected = value.and_then(JsonValue::as_i64).unwrap_or(-1) as isize,
            "on" => self.on = value.and_then(JsonValue::as_bool).unwrap_or_default(),
            "tint" => self.tint = rgb(value),
            "color" => self.color = rgb(value),
            _ => return false,
        }
        true
    }
}

/// Which control a component is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Kind {
    Slider,
    Checkbox,
    Radio,
    Popup,
    Stepper,
    Progress,
    Spinner,
    Date,
    Color,
    Text,
    Search,
    Tabs,
    GlassButton,
    GlassBar,
    Material,
}

pub(super) const KINDS: [(&str, Kind); 15] = [
    (SLIDER, Kind::Slider),
    (CHECKBOX, Kind::Checkbox),
    (RADIO, Kind::Radio),
    (POPUP, Kind::Popup),
    (STEPPER, Kind::Stepper),
    (PROGRESS, Kind::Progress),
    (SPINNER, Kind::Spinner),
    (DATE, Kind::Date),
    (COLOR, Kind::Color),
    (TEXT, Kind::Text),
    (SEARCH, Kind::Search),
    (TABS, Kind::Tabs),
    (GLASS_BUTTON, Kind::GlassButton),
    (GLASS_BAR, Kind::GlassBar),
    (MATERIAL, Kind::Material),
];
