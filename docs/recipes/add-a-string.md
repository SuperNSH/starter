# Add a string or a language

Text a person reads lives in `i18n/`, one file per language, one `key: "text"` per line.

## A string

1. Add the key to **every** file in `i18n/` (`just rules` fails if one lacks it). Start it with the screen's name: `orders.empty: "No orders yet"`.
2. Read it with `store::locale::text("orders.empty")`. It returns a closure, so the text redraws when the language changes. Pass the closure where a label goes: `text::muted(locale::text("orders.empty"))`. It takes no arguments: for a count, translate the label alone and put the number beside it inside a closure (`move || format!("{} {n}", locale::text("orders.likes")())`). A module that globs `nsh_core::prelude::*` and calls rust-i18n's `t!` directly needs `use rust_i18n::t;`, as the prelude has a `t!` of its own.

## A language

1. Copy `i18n/en.yml` to `i18n/<code>.yml` and translate every value.
2. Add the code and the language's own name to `CHOICES` in `src/store/locale.rs`.
3. `just rules && just test settings`.

`rust-i18n`'s locale is one value for the whole process: keep the language switches of a test file in one test.
