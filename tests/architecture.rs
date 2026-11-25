//! The architecture's rules, checked. `AGENTS.md` states them in prose;
//! this file is what keeps them true. Each failure says which rule broke,
//! where, and how to fix it. The checks read the source as text, so they
//! run in well under a second: `just rules`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// The longest a source file may be, in lines.
const MAX_LINES: usize = 500;

/// Which of the crate's top-level modules a folder's code may name with
/// `crate::`. Imports point one way: `shell -> pages -> store -> services`,
/// with `ui` and `native` below the screens.
const MAY_IMPORT: &[(&str, &[&str])] = &[
    ("src/error.rs", &["services"]),
    ("src/routes.rs", &["showcase"]),
    ("src/services/", &["services", "error"]),
    ("src/ui/", &["ui"]),
    ("src/native/", &["native", "ui"]),
    ("src/store/", &["store", "services", "error", "routes"]),
    (
        "src/showcase/",
        &[
            "showcase", "store", "services", "ui", "native", "routes", "error",
        ],
    ),
    (
        "src/pages/",
        &[
            "pages", "store", "services", "ui", "native", "routes", "error",
        ],
    ),
    (
        "src/shell/",
        &[
            "shell", "pages", "store", "services", "ui", "native", "routes", "error",
        ],
    ),
];

fn root() -> &'static Path {
    Path::new(ROOT)
}

/// Every `.rs` file under `dir` (relative to the repo), sorted.
fn rust_files(dir: &str) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&root().join(dir), &mut out);
    out.sort();
    out
}

/// A path as the repo names it: `src/pages/home/view.rs`.
fn rel(path: &Path) -> String {
    path.strip_prefix(root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("{}: {err}", rel(path)))
}

/// The source with comments blanked, so a rule quoted in a doc comment
/// never trips its own check. Line numbers stay the same.
fn code(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.trim_start().starts_with("//") {
                ""
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every path the code names under `crate::`, as its segments, with the
/// line it is on. `use crate::{a::b, c}` gives `[a, b]` and `[c]`.
fn crate_paths(code: &str) -> Vec<(usize, Vec<String>)> {
    fn ident(bytes: &[u8], mut at: usize) -> (String, usize) {
        let start = at;
        while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
            at += 1;
        }
        (String::from_utf8_lossy(&bytes[start..at]).into_owned(), at)
    }
    fn skip_space(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && bytes[at].is_ascii_whitespace() {
            at += 1;
        }
        at
    }
    /// Parses one use-tree at `at`, pushing each complete path.
    fn tree(bytes: &[u8], at: usize, prefix: &[String], out: &mut Vec<Vec<String>>) -> usize {
        let mut at = skip_space(bytes, at);
        if at < bytes.len() && bytes[at] == b'{' {
            at += 1;
            loop {
                at = skip_space(bytes, at);
                if at >= bytes.len() {
                    return at;
                }
                if bytes[at] == b'}' {
                    return at + 1;
                }
                at = tree(bytes, at, prefix, out);
                at = skip_space(bytes, at);
                if at < bytes.len() && bytes[at] == b',' {
                    at += 1;
                }
            }
        }
        let (name, next) = ident(bytes, at);
        if name.is_empty() {
            // A glob, or the end of an expression path.
            out.push(prefix.to_vec());
            return if at < bytes.len() && bytes[at] == b'*' {
                at + 1
            } else {
                at
            };
        }
        let mut path = prefix.to_vec();
        path.push(name);
        if bytes[next..].starts_with(b"::") {
            tree(bytes, next + 2, &path, out)
        } else {
            out.push(path);
            next
        }
    }

    let bytes = code.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(hit) = code[from..].find("crate::") {
        let at = from + hit;
        // `my_crate::` is not `crate::`.
        let own_word = at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_');
        let after = at + "crate::".len();
        if own_word {
            let line = code[..at].matches('\n').count() + 1;
            let mut paths = Vec::new();
            tree(bytes, after, &[], &mut paths);
            found.extend(paths.into_iter().map(|path| (line, path)));
        }
        from = after;
    }
    found
}

/// The folders under `src/<group>` (`pages`, `showcase`): one per section.
fn sections(group: &str) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(root().join("src").join(group))
        .unwrap_or_else(|err| panic!("src/{group}: {err}"))
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// Fails with every problem at once, under the rule they break.
fn report(rule: &str, fix: &str, problems: Vec<String>) {
    assert!(
        problems.is_empty(),
        "\n\nRULE: {rule}\nFIX:  {fix}\n\n  {}\n",
        problems.join("\n  ")
    );
}

#[test]
fn imports_point_one_way() {
    let page_sections = sections("pages");
    let demo_sections = sections("showcase");
    let mut problems = Vec::new();
    for file in rust_files("src") {
        let name = rel(&file);
        let Some((_, allowed)) = MAY_IMPORT
            .iter()
            .find(|(prefix, _)| name == *prefix || name.starts_with(prefix))
        else {
            // lib.rs, main.rs and startup.rs sit on top and may name anything.
            continue;
        };
        for (line, path) in crate_paths(&code(&read(&file))) {
            let Some(top) = path.first() else { continue };
            // Names from the prelude the crate root re-exports (`crate::Appearance`).
            if top.starts_with(|first: char| first.is_uppercase()) {
                continue;
            }
            let second = path.get(1).map(String::as_str).unwrap_or_default();
            let mut why = None;
            if !allowed.contains(&top.as_str()) {
                // The one junction: the page table lists the showcase's screens.
                if !(name == "src/pages/mod.rs" && top == "showcase") {
                    why = Some(format!("names `{top}`, which this folder may not import"));
                }
            } else if top == "showcase" && name.starts_with("src/showcase/") {
                let own = name.split('/').nth(2).unwrap_or_default();
                if demo_sections.iter().any(|s| s == second) && second != own {
                    why = Some(format!("reaches into another section, `{second}`"));
                }
            } else if top == "pages" && name.starts_with("src/pages/") {
                let own = name.split('/').nth(2).unwrap_or_default();
                if page_sections.iter().any(|s| s == second) && second != own {
                    why = Some(format!("reaches into another page, `{second}`"));
                }
            } else if top == "pages"
                && name.starts_with("src/shell/")
                && page_sections.iter().any(|s| s == second)
            {
                why = Some(format!(
                    "names the page `{second}`; the shell reads pages through `Route::page` only"
                ));
            }
            if let Some(why) = why {
                problems.push(format!("{name}:{line}: `crate::{}` {why}", path.join("::")));
            }
        }
    }
    report(
        "imports point one way: shell -> pages -> store -> services; ui and native sit below the screens",
        "move what two places share down a layer (two screens share through `store`), or pass it in. The table at the top of tests/architecture.rs lists what each folder may import.",
        problems,
    );
}

#[test]
fn no_file_is_longer_than_the_limit() {
    let problems = rust_files("src")
        .into_iter()
        .filter_map(|file| {
            let lines = read(&file).lines().count();
            (lines > MAX_LINES).then(|| format!("{}: {lines} lines", rel(&file)))
        })
        .collect();
    report(
        &format!("no source file is longer than {MAX_LINES} lines"),
        "split it by responsibility: turn `x.rs` into `x/mod.rs` and move a group of functions into `x/<part>.rs` (`use super::*;` at its top).",
        problems,
    );
}

#[test]
fn every_global_has_a_type_of_its_own() {
    let mut problems = Vec::new();
    for file in rust_files("src") {
        let text = code(&read(&file));
        for (at, line) in text.lines().enumerate() {
            let mut rest = line;
            while let Some(hit) = rest.find("global::<") {
                let ty = &rest[hit + "global::<".len()..];
                let name: String = ty
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                let named = name.starts_with(|c: char| c.is_ascii_uppercase())
                    && ty[name.len()..].starts_with('>')
                    && !matches!(name.as_str(), "String" | "Vec" | "Option" | "HashMap");
                if !named {
                    problems.push(format!("{}:{}: {}", rel(&file), at + 1, line.trim()));
                }
                rest = &ty[name.len()..];
            }
        }
    }
    report(
        "every `global::<T>()` uses a type declared for that one piece of state",
        "`global` keeps one value per type, so a plain type (`bool`, `String`, `Vec<..>`) is shared by everything that asks for it. Declare `struct MyState(bool);` beside its use and ask for that.",
        problems,
    );
}

#[test]
fn unsafe_stays_in_native() {
    let mut problems = Vec::new();
    for file in rust_files("src") {
        let name = rel(&file);
        if name.starts_with("src/native/") {
            continue;
        }
        for (at, line) in code(&read(&file)).lines().enumerate() {
            let is_word = |word: &str| {
                line.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .any(|token| token == word)
            };
            if is_word("unsafe") {
                problems.push(format!("{name}:{}: {}", at + 1, line.trim()));
            }
        }
    }
    report(
        "`unsafe` appears only under src/native",
        "move the platform code into `src/native/` behind a safe function, and call that. (`src/lib.rs` also forbids `unsafe` in every other module, so this fails the build too.)",
        problems,
    );
}

#[test]
fn every_screen_folder_has_the_same_shape() {
    let mut problems = Vec::new();
    for group in ["pages", "showcase"] {
        for section in sections(group) {
            let dir = root().join("src").join(group).join(&section);
            let at = format!("src/{group}/{section}");
            let module = dir.join("mod.rs");
            if !module.is_file() {
                problems.push(format!("{at}: no mod.rs"));
                continue;
            }
            let text = read(&module);
            if !text.starts_with("//!") {
                problems.push(format!(
                    "{at}/mod.rs: no `//!` header saying what the screen is"
                ));
            }
            if !text.contains("pub const PAGE: Page") {
                problems.push(format!("{at}/mod.rs: no `pub const PAGE: Page` record"));
            }
            if !text.contains(&format!("key: \"{section}\"")) {
                problems.push(format!(
                    "{at}/mod.rs: its record's key is not \"{section}\", the folder's name"
                ));
            }
            if !dir.join("view.rs").is_file() && !dir.join("view").join("mod.rs").is_file() {
                problems.push(format!("{at}: no view.rs (or view/mod.rs) for its layout"));
            }
            if !root().join("tests").join(format!("{section}.rs")).is_file() {
                problems.push(format!("{at}: no tests/{section}.rs"));
            }
        }
    }
    report(
        "every screen folder has a mod.rs with a header and a PAGE record keyed by the folder's name, a view, and tests/<folder>.rs",
        "follow docs/recipes/add-a-page.md; copy the shape of src/pages/diagnostics.",
        problems,
    );
}

#[test]
fn the_apps_pages_draw_with_ui() {
    let mut problems = Vec::new();
    for file in rust_files("src/pages") {
        for (at, line) in code(&read(&file)).lines().enumerate() {
            let Some(hit) = line.find("nsh_base::") else {
                continue;
            };
            let rest = &line[hit + "nsh_base::".len()..];
            let only_theme = rest.starts_with("theme;")
                || rest.starts_with("theme(")
                || rest.starts_with("{theme}");
            if !only_theme {
                problems.push(format!("{}:{}: {}", rel(&file), at + 1, line.trim()));
            }
        }
    }
    report(
        "the app's pages take their controls from `ui`, never from `nsh_base` (its `theme()` excepted)",
        "use the wrapper in src/ui, or add one there when the control has none yet. The showcase is exempt: it demonstrates the base components themselves.",
        problems,
    );
}

/// The keys of an `i18n/*.yml` file (`key: "text"` lines).
fn locale_keys(file: &Path) -> BTreeSet<String> {
    read(file)
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with('_'))
        .filter_map(|line| line.split_once(':').map(|(key, _)| key.trim().to_owned()))
        .filter(|key| !key.is_empty())
        .collect()
}

#[test]
fn every_language_has_every_string() {
    let dir = root().join("i18n");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("the i18n folder")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "yml"))
        .collect();
    files.sort();
    let all: BTreeSet<String> = files.iter().flat_map(|file| locale_keys(file)).collect();
    let mut problems = Vec::new();
    for file in &files {
        let have = locale_keys(file);
        for key in all.difference(&have) {
            problems.push(format!("{}: missing `{key}`", rel(file)));
        }
    }
    report(
        "every language file under i18n/ has every key",
        "add the missing key to the file named, translated (docs/recipes/add-a-string.md).",
        problems,
    );
}

#[test]
fn the_map_names_every_folder() {
    let map = read(&root().join("AGENTS.md"));
    let mut problems = Vec::new();
    for entry in fs::read_dir(root().join("src")).expect("src").flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let shown = if entry.path().is_dir() {
            format!("{name}/")
        } else {
            name
        };
        if !map.contains(&shown) {
            problems.push(format!("AGENTS.md does not mention src/{shown}"));
        }
    }
    report(
        "AGENTS.md's map names every folder and file directly under src/",
        "add a line to the map in AGENTS.md saying what the new entry is for, or put the code in the folder that already answers that question.",
        problems,
    );
}

#[test]
fn the_path_reader_reads_grouped_imports() {
    let found = crate_paths(
        "use crate::{store::{self, nav}, ui::button::*};\nlet x = crate::routes::Route::Home;",
    );
    let paths: Vec<String> = found.iter().map(|(_, path)| path.join("::")).collect();
    assert_eq!(
        paths,
        [
            "store::self",
            "store::nav",
            "ui::button",
            "routes::Route::Home"
        ]
    );
    assert_eq!(found[3].0, 2, "the second line's path carries its line");
}
