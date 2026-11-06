---
name: rust-i18n
description: Use when adding, wiring, or debugging localization with the Longbridge rust-i18n crate in a Rust app, especially GPUI / gpui-component desktop apps, including init, locale files, runtime locale switching, component-string overrides, and cargo i18n extraction; not for date, number, or plural formatting.
---

# rust-i18n

Reference for the Longbridge `rust-i18n` crate in Rust applications, with GPUI / gpui-component specifics: translations compiled into the binary from YAML/JSON/TOML files, a global `t!` macro, locale fallback chains, and gpui-component string overrides. Everything below is drawn from the official README, docs.rs, crates.io, the v4.2.4 source, GPUI Kit's i18n documentation, and local compile-and-run checks (see Grounded in). Scope is message selection: `set_locale` does not format dates, numbers, currencies, units, or plurals (GPUI Kit i18n docs).

## When to Use

- Adding UI localization with `rust-i18n` to a Rust app, especially a desktop app built with GPUI or gpui-component (GPUI Kit).
- Debugging locale files that do not load, wrong fallback order, or keys that render as-is.
- Overriding gpui-component's built-in component strings from the application.
- Extracting untranslated text with `cargo i18n`.
- Choosing between `_version: 1` split locale files and `_version: 2` single-file layouts.

## Identity and Version

- Crate name: `rust-i18n` (Rust identifier `rust_i18n`), maintained by Longbridge; repository https://github.com/longbridge/rust-i18n. Current release as of 2026-11-05: **4.2.4**, published 2026-09-29 (crates.io API); docs.rs builds 4.2.4, dated 29 September 2026. License: MIT.
- Do not confuse it with "rust-intl": an unrelated, small `rust_intl` crate exists on crates.io (v0.2.2, repository github.com/voctal/rust-intl, ~375 Rust code lines, "A Rust internationalization library"). That is a different project with a different API — the crate this skill documents is `rust-i18n`.

## Install and Initialization

Add the dependency (README shows `rust-i18n = "4"`; component overrides require 4.2+, so use `rust-i18n = "4.2"` there):

```toml
[dependencies]
rust-i18n = "4"
```

Initialize once at the crate root of the crate that owns the locale files (`main.rs` or `lib.rs`); alternatively import per file with `use rust_i18n::t;`. Every crate that calls `t!` needs its own `i18n!`, because `t!` expands to `crate::_rust_i18n_t!`.

```rust
#[macro_use]
extern crate rust_i18n;

// Alternatives: keep exactly one `i18n!` per crate; a second one is a compile error.
// Loads [package.metadata.i18n] config if present, or use arguments to override.
i18n!("locales", fallback = "en");             // single fallback
i18n!("locales", fallback = ["en", "es"]);     // priority chain
i18n!("locales", minify_key = true);           // short hashed keys for long literals
i18n!("locales", backend = RemoteI18n::new()); // custom backend (since v2.0.0); takes priority over local files
```

- `minify_key = true` also accepts `minify_key_len` (default 24), `minify_key_prefix` (default empty; README examples set `t_`), and `minify_key_thresh` (default 127; only longer messages are hashed).
- Cargo.toml `[package.metadata.i18n]` (documented defaults): `available-locales = ["en"]`, `default-locale = "en"`, `load-path = "locales"`. The extractor docs describe this section as primarily for the `cargo i18n` command; `i18n!` reads it when present and arguments override it. Keep `load-path` identical to the path passed to `i18n!`. `default-locale` does not set the runtime locale (it stays `en` until `set_locale`), and `available_locales!()` reports locales found in loaded files, not the `available-locales` list.
- Feature flags: `load-path` enables runtime locale loading via `try_load_locales(path, ignore_if, report_lookup_errors)`, which returns the parsed map and does not feed `t!` (otherwise YAML/TOML parsing deps stay compile-time only); `log-miss-tr` logs missing translations at warning level (add `log` to your own dependencies, or `t!` fails to compile).

## Locale Files

- Default directory: `locales/` at the project root (resolved against the crate's `CARGO_MANIFEST_DIR`). YAML is the default format; JSON and TOML work by renaming (`en.json`, `en.toml`). Multiple files are merged. `_version` selects the layout and defaults to `1`.
- `_version: 1` — split per locale (`locales/en.yml`, `locales/zh-CN.yml`):

```yaml
_version: 1
hello: "Hello world"
messages.hello: "Hello, %{name}"
```

- `_version: 2` — one file per module, each key mapping locale to text:

```yaml
_version: 2
hello:
  en: Hello world
  zh-CN: 你好世界
messages.hello:
  en: Hello, %{name}
  zh-CN: 你好，%{name}
```

- Placeholders use `%{name}`; dotted keys (`messages.hello`) form nested message keys. With `minify_key`, literals longer than `minify_key_thresh` are replaced by generated keys (`t_4Cct6Q289b12SkvF47dXIx` for `"Hello, %{name}"` with `minify_key_prefix = "t_"`, `minify_key_thresh = 10`), and locale files must contain those generated keys; `tkv!("...")` returns `(key, message)`.

## Runtime API

```rust
use rust_i18n::t;

t!("hello");                             // "Hello world"
t!("hello", locale = "zh-CN");           // explicit locale
t!("messages.hello", name = "world");    // %{name} parameter
t!("messages.hello", "name" => "world"); // key => value form
t!("Hello, %{name}, you serial number is: %{sn}", name = "Jason", sn = 123 : {:08});
                                         // std::fmt formatting: "... 00000123"
```

- `rust_i18n::set_locale("zh-CN")` sets the global locale, `rust_i18n::locale()` reads it back as `impl Deref<Target = str>` (not `Display`; write `&*rust_i18n::locale()`), and `rust_i18n::available_locales!()` lists available locales.
- The `fallback` argument accepts one locale or a priority list. Automatic territory fallback: if `zh-CN` is unavailable it falls back to `zh` (since v2.4.0), before the `fallback` chain is tried.

## Component Overrides (gpui-component / GPUI Kit)

GPUI Component ships its own component strings in `crates/component/locales/ui.yml` (gpui-component 0.9.0: en, pt-BR, ru, zh-CN, zh-HK with 122 keys each; zh-TW 85; it 46). Overrides require rust-i18n 4.2 or later in the application crate (`rust-i18n = "4.2"` beside `gpui-kit`).

1. Application `locales/ui.yml`, `_version: 2`, component keys under the `gpui_component` namespace (the Rust crate name `gpui-component` with hyphens converted to underscores):

```yaml
_version: 2
app:
  save:
    en: Save
    fr: Enregistrer
gpui_component:
  Calendar:
    month.January:
      fr: Janvier
```

2. Initialize app locales and register the override before initializing the component:

```rust
rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    let app = gpui_kit::application();
    app.run(move |cx| {
        use gpui_kit::component as gpui_component;
        rust_i18n::extend!(gpui_component); // once, at startup; a second call panics
        gpui_kit::init(cx);
    });
}
```

- Merge priority: the application's value first, then the component's built-in value; keys the app omits keep the built-in value, so a partial locale such as `fr` is valid.
- The namespace applies to component lookups only: in app code `t!("gpui_component.Calendar.month.February")` returns the key unless you define it. Read component strings by rendering components, and use your own keys (for example `t!("app.save")`) for application copy.
- gpui-component re-exports the locale accessors: `use gpui_kit::component::{locale, set_locale};`.

## Extraction Tooling

- `cargo install rust-i18n-cli` provides the `cargo i18n` command; it scans Rust sources for untranslated `t!` literals and merges entries into YAML. It is experimental, outputs YAML only in the `_version: 2` format, writes new entries to a single `locales/TODO.yml` (the README's `TODO.en.yml` is stale), and exits 1 when it finds new text, so it can gate CI. `i18n!` also compiles `TODO.yml`, so its placeholder text (`open` for `menu.file.open`) reaches the UI until you translate or delete it.
- `-t "text"` or `-t "text => translation"` adds translations for non-literal `t!` keys; `[package.metadata.i18n]` (`available-locales`, `default-locale`, `load-path`) tells the extractor which locales to check. CLI 4.2.4 has no `--locale` option, although the README still shows one; passing it exits 2.

## GPUI Notes

- Look strings up at render time so a later `set_locale` takes effect. The active locale is global state outside GPUI's change tracking: changing it alone does not repaint — call `cx.notify()` on the owning view or `cx.refresh_windows()`; rebuild cached menus and labels explicitly.
- Keep application copy in complete message keys so each language can reorder words and punctuation; do not assemble sentences from separately translated fragments.
- Tests: `set_locale` is process-global. Restore the previous locale after any test that changes it, or isolate such tests, so parallel tests do not leak locale state.

## Minimal Complete Example

Cargo.toml:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"

[dependencies]
rust-i18n = "4"

[package.metadata.i18n]
available-locales = ["en", "zh-CN"]
default-locale = "en"
load-path = "locales"
```

locales/en.yml:

```yaml
_version: 1
app.title: "My App"
messages.greeting: "Hello, %{name}!"
```

main.rs:

```rust
#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

fn main() {
    println!("{}", t!("app.title")); // My App
    println!("{}", t!("messages.greeting", name = "Ada")); // Hello, Ada!

    rust_i18n::set_locale("zh-CN");
    println!("{}", &*rust_i18n::locale()); // zh-CN
}
```

## Pitfalls

- Wrong crate: "rust-intl" / `rust_intl` is unrelated; depend on `rust-i18n`.
- Locale-only edits do not rebuild: Cargo does not track the application's locale files, so the binary keeps the old text until the crate recompiles. Add a `build.rs` containing `println!("cargo:rerun-if-changed=locales");`. A `rustc-wrapper` compile cache can still replay old text (observed with kache, even after `touch`); rebuild with `RUSTC_WRAPPER=""`.
- A wrong `i18n!` path compiles silently: nothing loads, `available_locales!()` is empty, and every `t!` returns its key. `RUST_I18N_DEBUG=1 cargo build` prints the load path and loaded files, or the path error; touch a source file first, because the variable alone does not trigger a rebuild.
- A missing key returns the key text itself (GPUI Kit's mistakes table: "Application t! returns a key") — define the key or fix the namespace.
- Overrides that silently do nothing: key outside `gpui_component`, misspelled key or locale code, or `extend!` called after `gpui_kit::init` (it must be registered first).
- Expecting app `t!` to read component built-ins; it reads only your application locale files.
- Changed locale without a repaint: the UI keeps the old language until you notify the view or refresh windows.
- Locale leakage between tests through the global `set_locale`.
- Territory fallback can mask a missing regional file: `zh-CN` silently shows `zh` strings when `zh-CN` is missing.
- `package.metadata.i18n` mismatch: `load-path` must match the path passed to `i18n!`.
- `minify_key` rewrites long literal keys to hashes; locale files that lack the generated hash stop matching.
- Dropping `%{name}` placeholders in a translation loses runtime values.
- RTL: setting an Arabic or Hebrew locale selects strings but does not mirror layout.

## Grounded in

- `rust-i18n` README — https://github.com/longbridge/rust-i18n (features, install, `i18n!` options, locale file versions, `t!` / `set_locale` / `locale` / `available_locales!`, backend, `extend!` and namespace, extractor, `package.metadata.i18n`, use cases). Its `--locale`, `TODO.<locale>.yml`, and `{:08}` output examples are stale against 4.2.4.
- `rust-i18n` source at v4.2.4 — https://github.com/longbridge/rust-i18n/tree/v4.2.4 (minify defaults, `locale()` return type, lookup order, `extend!` single-call assert, CLI options, `TODO.yml` output, `RUST_I18N_DEBUG`).
- docs.rs `rust_i18n` 4.2.4 — https://docs.rs/rust-i18n (release date, macros/functions/traits list, `extend!` since v4.2.0, territory fallback since v2.4.0, `minify_key` since v3.1.0, feature flags).
- crates.io `rust-i18n` — https://crates.io/crates/rust-i18n (current version 4.2.4 published 2026-09-29, MIT, repository).
- crates.io `rust_intl` — https://crates.io/crates/rust_intl (unrelated crate: v0.2.2, repository github.com/voctal/rust-intl, ~375 Rust code lines).
- crates.io `gpui-component` — https://crates.io/crates/gpui-component (0.9.0 published 2026-11-02; shipped `locales/ui.yml` per-locale key counts, read with `try_load_locales`).
- GPUI Kit i18n docs — https://gpui-kit.com/docs/i18n (component locale file, `gpui_component` namespace, `extend!` ordering, merge priority, locale accessors, notify/refresh requirement, formatting scope, RTL note, common-mistakes table).
- gpui-component repository — https://github.com/longbridge/gpui-component (redirects to github.com/longbridge/gpui-kit; gpui-component is the styled component layer of GPUI Kit).
- Local verification, 2026-11-05 — rustc 1.99.1 on editions 2021 and 2024 with rust-i18n 4.2.4, rust-i18n-cli 4.2.4, and gpui-kit 0.9.0: samples compiled and run, including `extend!` layering against the real gpui-component without opening a window. Live repaint behavior (`cx.notify()`, `cx.refresh_windows()`) was type-checked only and otherwise rests on the GPUI Kit docs.
