#!/usr/bin/env bash
# Makes a new app from this starter: a copy without the demo screens, under
# its own name and id.
#
#   scripts/new-app.sh <directory> <name> <id>
#   scripts/new-app.sh ../acme acme com.acme.app
#
# The copy keeps the app's own screens (Home, Items, Sign in, Settings,
# Diagnostics), the shell, the store, the services with the fake API the
# Items page reads, the nsh-query and nsh-form dependencies, the ui library
# and the native code. Its phone tabs are Home, Items and Settings. It
# drops src/showcase, the demo tests, the remote-screen package and this
# starter's build records. Run the gates in the copy afterwards:
# `just check && just test`.
set -euo pipefail

die() {
    echo "error: $*" >&2
    exit 1
}

[ $# -eq 3 ] || die "usage: scripts/new-app.sh <directory> <name> <id>"
dest="$1"
name="$2"
id="$3"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

[ ! -e "$dest" ] || die "$dest already exists"
mkdir -p "$dest"
dest="$(cd "$dest" && pwd)"

# 1. Copy what git tracks or would track; build output and caches stay behind.
(cd "$root" && git ls-files -co --exclude-standard -z | while IFS= read -r -d '' file; do
    [ -e "$file" ] || continue
    mkdir -p "$dest/$(dirname "$file")"
    cp -p "$file" "$dest/$file"
done)

cd "$dest"

# 2. Drop the demo screens and everything only they used.
rm -rf src/showcase packages assets/canvas tests/vision tests/trace docs/evidence \
    docs/upstream-drafts .zcode .github tools
rm -f tests/{canvas,data,forms,gallery,lists,motion,native_vs_app,overlays,platform,remote_screen,showcase}.rs \
    tests/smoke/controls.test.tsx \
    docs/STATUS.md docs/PROMPT.md docs/coverage.md docs/gpui-starter-parity.md \
    docs/nsh-0.5.0-migration.md docs/upstream-issues.md CHANGELOG.md \
    scripts/new-app.sh

# 3. Unhook them from the files that named them. Each edit must find its
#    text: the script stops if the starter drifted from what it expects.
python3 -I - <<'PY'
import re, sys

def edit(path, steps):
    text = open(path).read()
    for kind, old, new in steps:
        if kind == "text":
            if old not in text:
                sys.exit(f"new-app: {path} no longer contains:\n{old}")
            text = text.replace(old, new)
        else:
            text, count = re.subn(old, new, text, flags=re.M | re.S)
            if count == 0:
                sys.exit(f"new-app: {path} no longer matches: {old}")
    open(path, "w").write(text)

edit("src/lib.rs", [("text", "#[forbid(unsafe_code)]\npub mod showcase;\n", "")])
edit("src/routes.rs", [
    ("text", "\nuse crate::showcase;\n", "\n"),
    ("re", r"    /// One of the demo screens.*?    Showcase\(showcase::Route\),\n", ""),
    ("text", "            Route::Showcase(route) => route.id(),\n", ""),
    ("text", "            Route::Showcase(route) => route.parent().map(Route::Showcase),\n", ""),
    ("text", "            Route::Showcase(route) => route.path(),\n", ""),
    ("text", "            other => showcase::Route::parse(other).map(Route::Showcase),\n", "            _ => None,\n"),
    ("re", r"impl From<showcase::Route> for Route \{.*?\n\}\n\n", ""),
    ("text", "//! May import: nothing of the app, except the showcase's own route enum.\n", "//! May import: nothing of the app.\n"),
])
edit("src/pages/mod.rs", [
    ("text", "use crate::showcase;\n", ""),
    ("text", "            Route::Showcase(route) => route.page(),\n", ""),
    ("text", "        list.extend(showcase::SIDEBAR.iter().copied().map(Route::Showcase));\n", ""),
    ("text", "        list.extend(showcase::TABS.iter().copied().map(Route::Showcase));\n        list.push(Route::Items);\n", "        list.push(Route::Items);\n        list.push(Route::Settings);\n"),
    ("text", "        list.extend_from_slice(showcase::HOME_ROWS);\n", ""),
])
edit("src/pages/home/view.rs", [("text", "\"Everything the boilerplate demonstrates, one page each.\"", "\"Every page of the app.\"")])
edit("tests/architecture.rs", [
    ("text", "    (\"src/routes.rs\", &[\"showcase\"]),\n", "    (\"src/routes.rs\", &[]),\n"),
    ("text", "    for group in [\"pages\", \"showcase\"] {", "    for group in [\"pages\"] {"),
    ("text", "    let demo_sections = sections(\"showcase\");\n", "    let demo_sections: Vec<String> = Vec::new();\n"),
])
edit("tests/home.rs", [("text", "Everything the boilerplate demonstrates, one page each.", "Every page of the app.")])
edit("tests/smoke/app.test.tsx", [("text", "Everything the boilerplate demonstrates, one page each.", "Every page of the app.")])
edit("tests/smoke/pages.test.tsx", [
    ("re", r"^  \{ id: \"page-(?!settings|diagnostics|items|sign_in)[a-z_]+\", marker: [^\n]*\n", ""),
    ("text", ", \"page-data_item\"]", "]"),
])
edit("justfile", [
    ("re", r"^# TSX package gate in packages/remote-screen.*?nsh test\n\n", ""),
    ("text", "    just test-tsx\n", ""),
    ("re", r"^# Make a new app from this starter.*?\n\n", ""),
])
for path in ("i18n/en.yml", "i18n/zh-CN.yml"):
    lines = [line for line in open(path).read().split("\n") if not line.startswith("platform.")]
    open(path, "w").write("\n".join(lines))
# The map and the docs: lines about the showcase go.
for path in ("AGENTS.md", "docs/architecture.md", "README.md", "tests/shell.rs"):
    try:
        lines = open(path).read().split("\n")
    except FileNotFoundError:
        continue
    open(path, "w").write("\n".join(line for line in lines if "showcase" not in line.lower()))
PY

# 4. The new name and id, everywhere they live.
scripts/rename.sh "$name" "$id"

# 5. Formatting can shift with the name's length.
cargo fmt --all

echo
echo "Made $name ($id) in $dest"
echo "Next: cd $dest && git init && just check && just test"
