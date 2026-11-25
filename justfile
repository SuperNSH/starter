# Development commands. `just` with no argument lists them.
#
# Gates (no check may be weakened to get a pass):
#   Rust  just check && just test
#   TSX   just test-tsx            (packages/remote-screen)
#   All   just smoke

default:
    @just --list

# Rust gate: formatting check and clippy with warnings denied.
check:
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings

# Rust tests (also run by `nsh test` before its TSX tests). `just test` runs
# all of them; `just test forms` runs one screen's file, tests/forms.rs.
test section='':
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "{{section}}" ]; then cargo test; else cargo test --test "{{section}}"; fi

# The architecture's rules alone (AGENTS.md): what may import what, file
# sizes, the shape of a screen folder. Each failure says how to fix it.
rules:
    cargo test --test architecture

# TSX package gate in packages/remote-screen: type check, a build with no
# warnings, headless tests. Needs the node_modules/{nsh-query,nsh-form}
# symlinks that point at the sibling library checkouts.
test-tsx:
    cd packages/remote-screen && nsh check && nsh build && nsh test

# Headless smoke gate: the root's app test plus the per-page smoke suite,
# and the remote-screen package gates. tests/vision/ is excluded on purpose: its
# tests drive screenshots, which need a real target (`--target`) and a
# reviewing model, so they fail a headless run by design.
smoke:
    nsh test tests/smoke/app.test.tsx
    nsh test tests/smoke/pages.test.tsx
    just test-tsx

# Desktop run in the AppKit development host, with hot reload.
run-macos:
    nsh run --target macos

# iOS simulator run. Uses a booted simulator, else the newest iPhone.
run-ios:
    nsh run --target ios

# Android run on the first device `adb` lists. Uses Android Studio's JDK when
# JAVA_HOME is unset: /usr/bin/java is a stub with no runtime behind it.
run-android:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "${JAVA_HOME:-}" ] && [ -d "/Applications/Android Studio.app/Contents/jbr/Contents/Home" ]; then
        export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    fi
    nsh run --target android

# Web run in the browser (the dev server serves the page).
run-web:
    nsh run --target web

# Package all four shipped targets into .nsh/out: macOS .app, iOS simulator
# .app, Android .apk, and the web static site.
package:
    nsh package --target macos
    nsh package --target ios-simulator
    nsh package --target android
    nsh package --target web

# Make a new app from this starter: a copy without the demo screens, renamed.
#   just new-app ../acme acme com.acme.app
new-app dir name id:
    scripts/new-app.sh {{dir}} {{name}} {{id}}

# Rename the app everywhere it lives, or prove the rename with a roundtrip:
#   just rename <new-name> <new-id>
#   just rename --roundtrip-test
rename *args='':
    scripts/rename.sh {{args}}

# Print docs/STATUS.md: what is done, how it was verified, what is unverified.
status:
    cat docs/STATUS.md
