//! The app's UI language: `rust-i18n`'s global locale mirrored into a
//! signal, because a component runs once and only bindings that read a
//! changed signal run again. A binding that shows translated text reads
//! [`app_locale`] and then looks `t!` up, so a language switch redraws it.

use nsh_core::prelude::*;
// nsh_core's prelude has a `t!` of its own; the app translates with rust-i18n.
use rust_i18n::t;

/// The locale code the pages show.
#[derive(Clone, Default, PartialEq)]
struct AppLocale(String);

/// The locale code the pages show (`en`, `zh-CN`), empty until first set
/// (`t!` then answers in the `i18n!` default, `en`). Tracked.
pub fn app_locale() -> String {
    global::<AppLocale>().get().0
}

/// Switches the UI language: `rust-i18n`'s global locale and the signal the
/// string bindings read move together.
pub fn set_app_locale(code: impl Into<String>) {
    let code = code.into();
    rust_i18n::set_locale(&code);
    global::<AppLocale>().set(AppLocale(code));
}

/// The locales the app offers, with their names as read in that language
/// (a language's own name does not translate).
pub const CHOICES: [(&str, &str); 2] = [("en", "English"), ("zh-CN", "中文")];

/// The string under `key`, looked up on every run: a binding given this
/// redraws when the language changes.
pub fn text(key: &'static str) -> impl Fn() -> String {
    move || {
        app_locale();
        t!(key).to_string()
    }
}
