//! The app's identity from `nsh.json`, read at compile time: the window
//! title, the log directory and the diagnostics page all take it from here,
//! so none of them can drift from the manifest.

use std::sync::LazyLock;

use nsh_core::element::SharedString;
use serde_json::Value;

/// The app's name, id and version, as `nsh.json` declares them.
pub fn info() -> (SharedString, SharedString, SharedString) {
    static INFO: LazyLock<(SharedString, SharedString, SharedString)> = LazyLock::new(|| {
        let json: Value = serde_json::from_str(include_str!("../../nsh.json")).unwrap_or_default();
        let field = |key: &str| {
            json.get(key)
                .and_then(Value::as_str)
                .unwrap_or("?")
                .to_owned()
                .into()
        };
        (field("name"), field("id"), field("version"))
    });
    INFO.clone()
}
