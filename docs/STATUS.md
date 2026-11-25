# Status

What is done, what was verified how (command + evidence path under
`docs/evidence/<area>/`), and what is NOT verified. A claim with no evidence
is unverified. Every area updates its own section as it lands work.

## Template screens: Items and Sign in (2026-11-25)

Added `src/pages/items/` (list + detail on `nsh-query` over the fake API), `src/pages/sign_in/` with `src/store/session.rs` (`nsh-form` over the mock auth seam), an Account card in Settings, `ui::form` and `ui::nav::list_row`. `nsh-query`/`nsh-form` now come from crates.io 0.1.0 (same source as the sibling checkouts, `diff -r` clean). `scripts/new-app.sh` keeps both and the fake API; the new app's tabs are Home, Items, Settings. Logs: `docs/evidence/project/template-screens/`.

### Verified, and how

| Command (repo) | Result | Log |
|---|---|---|
| `cargo fmt --all -- --check` | exit 0 | `fmt.log` |
| `cargo clippy --all-targets -- -D warnings` | exit 0 | `clippy.log` |
| `cargo test` (all, incl. `--test architecture`) | exit 0, 162 passed (items 5, sign_in 4 new) | `cargo-test.log` |
| `cargo clippy --lib --target aarch64-apple-ios-sim -- -D warnings` | exit 0 | `clippy-ios-sim.log` |
| `nsh test tests/smoke/app.test.tsx` | 2 passed | `nsh-test-app.log` |
| `nsh test tests/smoke/pages.test.tsx` | 1 passed (17 pages, incl. items, items_detail, sign_in) | `nsh-test-pages.log` |
| Seen to fail: one break per new test (9) and the new gallery assertion | each failed as intended, code restored | `seen-to-fail.log` |
| `scripts/new-app.sh /private/tmp/claude-501/newapp-template/acme acme com.acme.app` | exit 0 | `new-app.log` |
| In the copy (`CARGO_TARGET_DIR` in scratch): fmt check, clippy `-D warnings`, `cargo test`, both `nsh test` smoke files | all exit 0; `cargo test` 59 passed | `generated-app/*.log` |

The generated copy was made before two comment-only edits (`items/view.rs`, `store/session.rs`); the repo gates above ran after them.

### Not verified

- Nothing was looked at on screen: no target run, no screenshot. The desktop presentation of an item is tested headless only (macOS environment, 1280 × 800); its `pt_safe_area` under an iPad's system header is unverified.
- `tests/smoke/pages.test.tsx` per target (`--target ios|android|macos|web`): not run; its phone scroll offsets for the new Home rows are computed, not seen.
- CI (`.github/workflows/ci.yml`) still fetches the sibling checkouts; the Rust build no longer needs them. Not run.

## Restructure for navigation (2026-11-25)

The source moved to the layout in `AGENTS.md`. No product behavior was meant to change except what is listed under "Behavior changes".

### Verified, and how

Evidence: `docs/evidence/project/restructure/` (`summary.txt` lists every exit code).

| Check | Command | Result |
|---|---|---|
| Format | `cargo fmt --all -- --check` | exit 0 (`fmt.log`) |
| Lint | `cargo clippy --all-targets -- -D warnings` | exit 0 (`clippy.log`) |
| Rust tests | `cargo test` | 152 passed, 0 failed (`test.log`); 132 before, on the old layout |
| iOS compile | `cargo clippy --lib --target aarch64-apple-ios-sim` and `aarch64-apple-ios`, `-D warnings` | exit 0 |
| Web compile | `cargo clippy --lib --target wasm32-unknown-unknown -- -D warnings` | exit 0 |
| Headless TSX | `nsh test tests/smoke/app.test.tsx`, `tests/smoke/pages.test.tsx`, `tests/trace/data.test.tsx` | 2, 1 and 1 passed |
| remote-screen | `nsh check`, `nsh build`, `nsh test` in `packages/remote-screen` | exit 0, 9 tests passed |
| Rules | `cargo test --test architecture` | 9 passed; each rule seen to fail with a throwaway violating file |
| New app | `scripts/new-app.sh <tmp>/acme-notes acme-notes com.acme.notes`, then in the copy `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `nsh test` on both smoke files | exit 0; 49 Rust tests, 2 and 1 TSX tests passed (run in a scratch directory; no log kept in the repo) |

New tests seen to fail once: `tests/home.rs` (Home skipping a page; `nav_tile` ignoring its press), `tests/native_vs_app.rs` (the fallback switch bound to its own signal).

### Behavior changes

- Links and test ids of six screens follow their new names (see `CHANGELOG.md`).
- The error banner is mounted in the real app, so an error raised anywhere shows; before, only the test root mounted it.
- A config save writes a log line in the real app.
- Home lists a Settings page.

### Not verified

- Nothing was run on a device, a simulator, an emulator or a desktop window after the move: `tests/vision/` and the per-target smoke runs were not repeated. Their files were moved and their ids and `open` paths rewritten by script.
- The Settings page has not been looked at on any target; only its headless test ran.
- `tests/smoke/controls.test.tsx` fails headless, as it did before the move (it scrolls Home, which headless cannot); it is a per-target test.
- `cargo clippy --lib --target aarch64-linux-android` fails on this Mac building `onig_sys` (a C dependency; the NDK toolchain is not set up in this shell). CI runs it on Linux.
- The moved macOS and iOS native code compiles; the command bar, the system controls and the soft edges were not exercised on screen.
- Not rechecked against the new layout, and still describing what was true when written: the sections below, `docs/coverage.md` beyond its paths, and the evidence paths `docs/evidence/<area>/` that name the old areas.

### Open decisions

- The first-run welcome flow works and is tested but the app does not show it (`docs/architecture.md`, "What is built but not switched on").
- `docs/evidence/` holds 931 tracked files (about 120 MB) under the old area names.

## Baseline (bootstrap, pre-workflow)

- Scaffolded the Rust app at the repo root with `nsh init --template rust`
  shape: `nsh.json` (`"rust"`, `"root"`, theme at `themes/theme.json`),
  `Cargo.toml` with `nsh-query`/`nsh-form` path deps, `rust-i18n`, and
  `nsh-engine`/`nsh-testing` 0.4.0 dev-deps; module tree in `src/` with
  placeholder pages for every feature area; `App.test.tsx` + `tsconfig.json`.
- Verified on the baseline only: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, `nsh check`,
  `nsh build` (no warnings), `nsh test` (1 passing test). No feature work
  exists yet; every satisfaction criterion is unverified.

## Project docs and tooling

- Done: The project area's full file set — `justfile` (12 recipes: check,
  test, test-tsx, smoke, run-macos/ios/android/web, package, rename, status),
  `scripts/rename.sh` (in-place rename of every name/id token form plus a
  `--roundtrip-test` mode that copies to a sibling, renames, runs the
  criterion-1 gate there, and always cleans the copy up), and the docs:
  README (clean clone to running app, page map aligned with the shell's
  `Route` enum), CONTRIBUTING (gates, manual-clock and seen-to-fail test
  rules, doc standards), AGENTS.md (PROMPT pointer kept, plus gates,
  ownership map, evidence conventions), CHANGELOG 0.1.0,
  `docs/architecture.md`, `docs/coverage.md` (every nsh_core, nsh_base,
  plot, nsh/mobile and nsh/desktop export as a row, named from
  `.nsh/types/nsh/`), `docs/gpui-starter-parity.md`, and
  `docs/upstream-issues.md`.
- Verified: scoped gate `bash -n scripts/rename.sh` pass and
  `test -x scripts/rename.sh` pass (re-run after every fix round);
  in-place rename functional test in a throwaway /tmp copy — all three
  token forms (`nsh-starter`, `nsh_starter`, `com.example.nshstarter`)
  renamed across 9 files, Cargo.lock correctly left to cargo, zero
  leftovers outside it, original untouched; roundtrip trap-fix repro in
  /tmp with the gate stubbed — pass path exits 0 with the copy deleted,
  FAIL path exits 1 with the copy deleted; 3 audit rounds (round 1: 5
  findings incl. 1 high EXIT-trap unbound-variable bug, round 2: 4/5
  confirmed fixed + 2 residual, round 3 fixes self-verified by checking
  every coverage row name against `.nsh/types` and confirmed by the
  orchestrator's spot-check); `just -n` parses all 12 recipes; every
  documented nsh flag checked against the installed nsh 0.4.0 help
  (`nsh --help`, `nsh test/run/package/keygen/publish/verify/revoke
  --help`); coverage rows and cross-references verified against
  `.nsh/types/nsh/*.d.ts`, the nsh-base/nsh-core 0.4.0 crate sources, and
  the `~/fo/nsh-query` / `~/fo/nsh-form` sources (`form.load` at
  nsh-form `form.rs:157`, nsh-query exports, `tracked.rs`).
- Not verified: the `--roundtrip-test` full criterion-1 gate inside a
  renamed copy (fmt + clippy + cargo test + nsh test compile the whole
  graph; it runs at the whole-tree smoke gate); docs/architecture.md and
  README factual claims about code other areas are still building (shell
  header/menus, overlays pages, platform surfaces, `smoke/`,
  `tests/<area>.rs`, `trace/`, `scripts/delivery.sh` in the remote-screen
  package — re-verify against the tree at the join);
  `docs/coverage.md` unused-export accounting (the *planned* rows need a
  final grep pass at the join, and anything still unused needs its
  stated reason); the `brew install nsh` formula name was inferred from
  the brief's `$(brew --prefix nsh)` and never run.

## Lists area audit notes (round 2)

- Wrapper gap kept local: `src/features/lists/calendar_view.rs` does NOT
  use `ui::pickers::calendar_of`, because that wrapper hardcodes its own
  `render_item` (it labels weekdays and months with bare numbers, so the
  month header would read "11" instead of "November 2026"), takes no
  `disabled_matcher` (the demo's weekends-off behavior is asserted by
  `tests/lists.rs`), and returns `impl IntoElement`, so neither
  `render_item` nor `disabled_matcher` can be chained by the caller. To
  adopt it, `ui::pickers::calendar_of` needs `disabled_matcher` and
  `render_item` passthrough parameters (or a `_with` variant).
- Retracted an earlier engine-quirk claim: "a row prepended by
  `on_load_more_at_start` never mounts, even after `scroll_to_index(0)`"
  was an artifact of the demo's old loader bookkeeping (a broken window
  fed duplicate ids into the list's keys). With the fixed window, the
  start-loader test asserts the arrival mounts after the Latest jump and
  passes; no upstream issue.
- Verified engine fact worth keeping: on an inverted list whose array is
  newest-first, a loader that prepends (array head) is a layout-tail
  append, so dropping `.keys(…)` produces no visible difference in these
  flows; keys stay in place per the coding guides, and the chat's loader
  test breaks on loader bookkeeping instead.

## Lists and data views

- Done: All nine deliverables in `src/features/lists/` — `Table` semantics
  parts over an order book plus a sectioned list with sticky headers
  (orders.rs), `DataTable` over 3,000 deterministic quotes with app-side
  sorting, a fixed symbol column and stripes on `ui::table::data_table_of`
  (quotes.rs), a controlled `Tree` with depth-indented, expansion-aware
  rows (tree.rs), a controlled `Calendar` with a weekends-off matcher and
  named month/weekday items (calendar_view.rs), a 2,000-card grid with
  viewability logging, jump controls and data-keyed stars (grid.rs), an
  inverted chat opened mid-history whose loaders produce gapless pages at
  both ends, with Receive/Latest controls and a `ui::field::text_field`
  draft (chat.rs), a `ui::carousel_of` cover carousel (carousel_view.rs),
  and `ui::pagination_of` over 500 generated entries (pager.rs); the
  audit round moved text, table, pagination and carousel surfaces onto the
  ui wrappers and landed the chat loader redesign (window 240..=280 of
  room 1..=300, page math in pure `older_page`/`newer_page` helpers).
- Verified: `cargo test --test lists` — 7/7 pass (contiguity unit test,
  sort orders, grid scroll keeps stars, chat start loader + no-jump, chat
  older loader + mounted-run span check, calendar selection + refused
  weekend, pager rows) on the manual clock, final run after the audit
  fixes; scoped `cargo clippy --all-targets` — no findings in
  `src/features/lists/**` or `tests/lists.rs`; scoped `cargo fmt --check`
  clean for the area's files; `tsc -p tsconfig.json` exit 0 covering
  `vision/lists.test.tsx`; two audit rounds — r1 six findings (the
  chat-loader HIGH window bug, missing loader tests, calendar/input
  wrapper swaps, in-repo break records, the Latest-mount re-check), r2
  clean with two comment nits fixed (the older page is 225..=239; the
  pager break fails at the Archive-21 wait, not the caption); seen-to-fail
  re-verified per test with the records in the `tests/lists.rs` header;
  the calendar wrapper-gap record and the retracted Latest-mount claim
  are in this file's "Lists area audit notes (round 2)" section.
- Not verified: pixels and real-platform runs — the vision phase has not
  run yet; `vision/lists.test.tsx` (8 views × light/dark × phone/desktop)
  is written and type-checks but has never executed, and it presses the
  shell's "Lists" nav entry, so it needs the shell and a real target;
  whole-tree gates (`cargo clippy/test --all-targets` over all areas,
  `nsh build`, the smoke suite) are left to the join/smoke phases —
  my scoped runs were blocked at times by other areas' in-flight files
  and re-run until green; two suspected engine quirks remain
  observations only, repro notes for `docs/upstream-issues.md`: (a) on a
  manual clock, an animated `scroll_to_index` can leave cell rebinds
  lagging the geometry (viewability reported rows 0–17 while the cells at
  those rows still showed their pre-jump texts, repro: grid demo, jump to
  card 1500 and back, read `screen().texts()`), and (b) on an inverted
  list, a row prepended by `on_load_more_at_start` while the viewport was
  anchored away from the start did not appear in the mounted pool after a
  later jump to the start in early probes — not reproducible after the
  loader fix (the Latest-mount assertion passes), so it is recorded as a
  first-half observation, not a claim.

## UI pass on macOS, iOS and Android (2026-11-08)

- Done: every page captured in light and dark on all three targets, reviewed, fixed, then captured again. Fixes:
  - Diagnostics rendered blank inside the shell: its `nav_page` sat inside the shell's `nav_page`. The shell now mounts `foundation::diagnostics_content`.
  - On macOS dark, pages switched between the window gray (#1e1e1e) and the theme background. `shell/page.rs` now paints the theme background on every page and caps content at 1040 pt.
  - The title bar reserved space on the right, but macOS puts the window controls on the left. It now pads by `title_bar().left` and `.right`.
  - Home duplicated the sidebar. It is now a grid of page cards, each with a one-line summary (`Route::summary`).
  - Phones: dropped the status strip above the tab bar. Added tab icons (SF Symbols on iOS, system drawables on Android). Tab roots no longer show a back button. Forms shows one header instead of two.
  - Android: the system header follows the app's appearance (`ui::nav::themed_header`), and the Forms stack paints the theme background.
  - Controls: Lists chips and Platform buttons had labels stuck to the top-left; they are now centered. Off-state switch tracks are visible in light and dark. Badges and pills hug their text. Primary buttons no longer stretch full width on desktop. The gallery's popup triggers sit in one row at one size. The motion drawer hides when shut. Overlays flows sit in titled cards. Diagnostics shows undeclared capabilities in a neutral color instead of red.
- Verified: `nsh test --target macos|ios|android .audit/capture2.test.tsx` (a throwaway capture script kept out of git) passed on each target. Its 52 screenshots per target are in `docs/evidence/shell/ui-pass/{macos,ios,android}/`, and the key ones were looked at after the fixes. `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, `cargo test` (all suites), `nsh check`, `nsh test App.test.tsx` and `nsh test smoke/pages.test.tsx` all pass. Test updates: `tests/shell.rs` (the lifecycle test now runs at desktop width, where the status bar lives; the back-order test uses the "Sign up" marker) and `App.test.tsx` (the root check uses Home's own text). Each failed before its update.
- Not verified: the web target; widths between phone and desktop (tablet); `vision/*.test.tsx` was not re-run, and its saved screenshots predate this pass.

## 0.6.0 update (2026-11-18)

NSH 0.6.0 (5541637, released 2026-11-18). Release notes: `docs/evidence/project/nsh-0.6.0/release-notes-v0.6.0.md`; gate logs: `docs/evidence/project/nsh-0.6.0/final/` (below, `E/`).

- Changed: `Cargo.toml` pins 0.5.1 → 0.6.0, `cargo update -p` for the NSH crates only; every NSH crate resolves to 0.6.0, one version each (`cargo tree -i nsh-core`). `nsh.json` adds `"application": true` (0.6.0 breaking change #1499: the shell's `main_window()`, `set_application_menu` and `close()` need it once `capabilities` is declared). `src/foundation/capabilities.rs` lists 0.6.0's sixteen keys from `KNOWN` in `nsh-core/src/access.rs`. CI `NSH_VERSION`, README, `CONTRIBUTING.md`, `docs/architecture.md`, `docs/coverage.md` say 0.6.0. The CLI is 0.6.0 in `~/.nsh` (release `install.sh`), components `android`, `ios`, `web` re-added.
- The other breaking changes in the release notes were grepped for across `src/`, `tests/`, `packages/`, `smoke/`, `trace/`: no use of `clip_utf16`, `AlertDialogOptions`, `MenuEvent`, `Transition::`, `ModalConfig`/`ModalProp`, `RichEditorEvent`, WebView downloads, `selectOnModifierPress`, `readText` or the home directory.
- Gates on 0.6.0 (`E/summary.txt`): fmt 0, clippy 0, `cargo test --no-fail-fast` 127 passed / 0 failed (same as 0.5.1 after the fixes), `nsh test App.test.tsx` 2/2, `nsh test smoke/pages.test.tsx` 1/1, `packages/remote-screen` `nsh check` 0, `nsh build` 0 with no warning, `nsh test` 9/9. Re-run after the registry change: fmt 0, clippy 0, 127/0.
- Trace (`E/trace.log`): `open()` 527 mount items, like tap 1, scroll 42 per step while the pool fills, `nodes` flat at 479: identical to 0.5.1.
- The headless runner's build prints one linker warning (`ld: __eh_frame section too large (max 16MB)`) for the debug test binary; it is the linker on binary size, not app code, and the root `nsh build` is not affected.
- Siblings: every `~/fo/nsh-*` repo pinned to 0.5.1 moved to 0.6.0, uncommitted, each with its own fmt/clippy/test and TSX gates; none needed a source change. `nsh-query` (Rust all-features 122, TSX 74/74) and `nsh-form` (Rust 113, TSX 100/100) all 0.
- Not run at the time (the section below runs the device ones): the vision pass, cross-target `cargo check`. 0.6.0 changes mobile `Switch`/`DateTimePicker` to system controls and iOS sheets to detents, which no screen here has been looked at with yet.

## 0.6.0 finish pass (2026-11-19)

Evidence: `docs/evidence/project/nsh-0.6.0/finish/` (below, `E/`).

- Old-version references: 40 hits of `(NSH|nsh|Native Shell) 0.[345]` outside evidence, CHANGELOG and lockfiles (a count at the start of this pass): 26 history (the 0.4.0 to 0.5.0 migration plan, the `docs/PROMPT.md` original goal and measured environment, the `.agents/skills` and `.zcode` snapshots, older STATUS sections, `docs/upstream-issues.md` 0.4.0 entries), 9 limitation claims (the four i18n strings, the Remote screen text, `STATUS.md` 172, and the comments in `command_bar.rs`, `deep_link.rs`, `native.rs`), 5 stale facts (`README.md`, two `PROMPT.md` facts, the Remote screen and Device module comments). The history hits stay. Limitation claims, checked against the 0.6.0 sources: device model not exposed (`nsh-core/src/platform.rs`, `Environment` has no model field): still true, the string now says 0.6.0; reading text aloud (`nsh-core/src/speech.rs`, recognition only): still true; a verified package cannot be embedded in a page tree (`nsh-engine/src/lib.rs`, `load_package`; `nsh-core/src/element/` has no embed element): still true, string says 0.6.0; native components not registered on Android (`nsh-macros/src/native_components.rs`: windows, linux, macos, ios only): still true; `GestureEvent` carries no centroid (`nsh-core/src/element/events.rs`, unchanged from 0.5.0): still true. URL schemes and notifications were claimed as follow-ups and exist in 0.6.0: implemented below.
- Implemented: URL-scheme deep links (`nsh.json` `schemes: ["nsh-starter"]`, `shell::deep_link::listen` over `next_url_scheme_open`; test `a_link_of_the_claimed_scheme_opens_its_page`, `tests/shell.rs`), and a Notifications card on the Platform page (`src/platform/notifications_card.rs`, `notifications` capability, en and zh-CN strings; test `notifications_report_a_platform_that_shows_none`, `tests/platform.rs`: a headless run reports the permission unavailable and the send refused). Not adopted, with reasons in `docs/gpui-starter-parity.md`: `Tray` and `register_global_shortcut` (the AppKit command bar already has the menu bar item and Option+Space), single instance (the headless driver cannot deliver a second launch, so it could not be tested), the other exports in `docs/coverage.md` "New in 0.6.0, not yet used". The earlier docs said the system notification sat behind a trait with an in-app inbox; neither exists in the tree, and the docs now say what is there.
- Sign-up birth date on Android: `nsh-base`'s date picker shows a bottom sheet on touch screens and applies `popup_style` to it; this app gave it a 320 pt width and all-round corners, so it hung at the lower left instead of spanning the screen. Ours; `styled_date_picker` now uses full width, top corners and bottom padding on touch. Test `the_birth_date_sheet_spans_the_phone_screen` (`tests/forms.rs`), seen to fail with the desktop style kept. Not re-run on the Android emulator, so the new look is unverified on a device.
- `nsh test --target web` reached 8 of 13 pages. A probe (every step timed) showed the cost was one step: `homeRowInView` scrolled Home, which does not scroll at desktop width, and the failure took 25 s a page (the other steps took 1 s in all). The smoke test now scrolls only where the size was rejected (phones). All 13 pages in 26.7 s (`E/test-web.log`, PNGs `E/web/`, every one looked at). Looking at them found the Remote screen's "Loading this package" paragraphs drawn over each other on the web (`flex_1` on text in a column); fixed, re-shot clean (`E/web/page-remote.png`). A headless test could not reproduce that layout, so it has none.
- `chrome::sidebar_key` now takes the OS (`sidebar_key_on`); `the_sidebar_key_parses_on_every_os` parses it for every `Os` and was seen failing with the non-Apple arm returning `ctrl+mod+s`.
- Gates (`E/`): `just check` 0, `just test` 132 passed / 0 failed (128 before, plus four new tests), `nsh test App.test.tsx` 2/2, `nsh test smoke/pages.test.tsx` 1/1 (headless), `just test-tsx` 0 (`nsh check`, `nsh build` with no warning, 9/9). Not run: the vision pass, iOS and Android device runs after these changes, packaging. Trace not re-run: no change touches the feed or its list.

## 0.6.0 device runs (2026-11-19)

Evidence: `docs/evidence/project/nsh-0.6.0/device/` (below, `E/`). Own devices, deleted at the end: simulator `nsh-starter-0.6` (iPhone 17, iOS 27.0), AVD `nsh-starter-0.6` (Pixel 8, API 37, 20 GB data: the debug dev host APK is 3.6 GB with four ABIs and does not install on a 6 GB data partition). Android builds ran with `ANDROID_NDK_HOME` at NDK 28 (NDK 29's `llvm-ar` loops, and `nsh` builds four ABIs).

- Smoke, every page opened and screenshotted (`smoke/pages.test.tsx`): macOS 1/1 (`E/test-macos.log`, PNGs `E/macos/`), iOS simulator 1/1 (`E/test-ios.log`, `E/ios/`), Android emulator 1/1 (`E/test-android.log`, `E/android/`), web headless Chrome 8 of 13 pages (`E/test-web.log`, `E/web/`): the run reached 8 pages and hit the 240 s test limit at about 30 s a page, with no page failing (cause and fix in the finish pass above); the web site itself was opened separately (below). The web target lacks the features `SKIPPED_ON_WEB` names. Every PNG was looked at. No page is clipped, empty or broken on any target. The remote-screen page says "Not built" on Android (the delivery record is machine-local) and "Verified" on macOS and iOS.
- The smoke test was not testing what it claimed, fixed: its markers were page titles, which also match the Home row and sidebar entry, so the wait passed before the page was up (the iOS shots showed Home with the exit prompt over it); on macOS `back()` does nothing, so run 1 failed on page 2. Markers are now text only that page shows, Home is scrolled so a row is not under the title or tab bar (`home-scroll` test id on Home's scroll in `src/shell/home.rs`), the macOS sidebar goes Home by its first row, the per-test timeout is 240 s, and the screenshot waits 500 ms for the push. Seen to fail: the old markers (iOS shots) and the Canvas marker headless before it was changed to `Sketch`.
- Packaging (`E/package-*.log`): `--target macos` 0 (9 m 25 s), `ios-simulator` 0 (6 m 53 s), `web` 0 (14.1 MB, `E/package-web.log`), `android` 0 (arm64-v8a, armeabi-v7a, 80 MB). Launched: the macOS app (process up, 1200 x 800 window in the window list; `screencapture` is refused on this Mac, so no screenshot, as on 0.5.0: `E/macos-packaged-launch.txt`), the iOS app (`E/ios-packaged-launch.png`, Home), the Android APK (`E/android-packaged-launch.png`, Home), the web site served from `.nsh/out/web` on a local port (`E/web-packaged-launch.jpg`, Home with the sidebar).
- Forms switch, date picker and overlays sheet on iOS and Android, light and dark (`smoke/controls.test.tsx`, PNGs `E/ios-controls/`, `E/android-controls/`): the newsletter and the sheet's switches are visible and in place in both schemes on both targets (I could not tell from the PNGs whether iOS draws them as system controls). The sheet opens at its half detent with its Done button, switches and text, unclipped in both schemes. The sheet is the app's own `sheet()` with its own detent signal; it does not use 0.6.0's grabber or detent API, so no grabber shows (adopting it is a feature, not done).
- Bugs found on a device and fixed: (1) Android failed at mount, `on_key: invalid keystroke "ctrl+mod+s": "mod" twice` (`mod` is Ctrl off the Apple platforms); `shell::chrome::sidebar_key()` gives `ctrl+alt+s` there (`E/test-android-before-sidebar-key-fix.log` is the failing run; the headless test keeps `ctrl+mod+s` and passes). (2) The sign-up birth date calendar was unstyled, its weekday cells showed 0 to 6 and its month 11; `ui::pickers::styled_date_picker` now carries the gallery's styling, weekdays are `Su`..`Sa` and months `Jan`..`Dec`, and the month and year toggles no longer wrap (`tests/forms.rs::the_birth_date_calendar_names_its_weekdays`, seen to fail with a weekday renamed). (3) The budget field's typed text now takes the theme foreground.
- Not fixed: the budget field's placeholder `0` is near invisible in dark mode on Android (nsh's Rust style API has no placeholder color; only the TypeScript style has); the iOS sheet's date picker is still the popup calendar (this app's `date_picker()`, not the system `DateTimePicker`); the two stale strings (fixed in the finish pass below); the build prints `error Fake map` 44 times in the Android test run (from the host, not `src/`).
- Gates after these changes (`E/gates/`): fmt 0, clippy 0, `cargo test --no-fail-fast` 128 passed / 0 failed, `nsh test App.test.tsx` 0, `nsh test smoke/pages.test.tsx` 0 (headless), `packages/remote-screen` check 0, build 0 with no warning, test 9/9. Trace not re-run: no change touches the feed or its list.

## 0.5.1 update (2026-11-17)

NSH 0.5.1 (224d9a7, released 2026-11-14) is bug-fix only, with no API changes. Evidence: `docs/evidence/project/nsh-0.5.1/` (below, `E/`); `E/gate-summary.txt` lists every gate's exit code, baseline and after.

- Changed: `Cargo.toml` pins 0.5.0 → 0.5.1; `cargo update -p` for each NSH crate only (`E/nsh-starter/cargo-update.log`; the lock diff is the 14 NSH crates' versions and checksums, nothing else); CI `NSH_VERSION` v0.5.1; README install line, `CONTRIBUTING.md`, `docs/PROMPT.md` environment, `docs/architecture.md` and `docs/coverage.md` say 0.5.1. No source or test change. `cargo tree` shows every NSH crate at 0.5.1, one version each (`E/nsh-starter/final-0.5.1/cargo-tree-nsh.log`). The `pub` lists in `nsh-core` and `nsh-base` `src/lib.rs` are the same in 0.5.0 and 0.5.1.
- Baseline on 0.5.0 (`E/nsh-starter/baseline-0.5.0/`), run with the 0.5.1 CLI: fmt 0, clippy 0, `cargo test --no-fail-fast` 125 passed / 2 failed (the known `data::retry_recovers_a_failed_feed` and `remote_screen::the_signed_package_verifies_runs_and_then_refuses_after_revocation`), `nsh check` 1 (23 errors, all in `packages/remote-screen`), `nsh build` 0 with no warning, `nsh test App.test.tsx` 2/2, `nsh test smoke/pages.test.tsx` 1/1, `packages/remote-screen` check/build/test 1.
- On 0.5.1 (`E/nsh-starter/final-0.5.1/`): the same exit codes, the same 125/2 split with the same two failures, and the same 23 type errors, line for line. Nothing regressed, so nothing was fixed.
- Trace, `nsh test trace/feed.test.tsx --trace`: mount items per step identical on 0.5.0 and 0.5.1 (`open()` 527, like tap 1, scroll 42 per step while the pool fills), `nodes` flat at 479 on both.
- Siblings, same method (`E/<repo>/`): `nsh-query` and `nsh-form` gates match their baselines (TSX 74/74 and 100/100). The `feat/tanstack` worktrees moved 0.4.0 → 0.5.1; `nsh-form`'s r3/r5 failed on 0.5.1 exactly as on master at 0.5.0, and took master's 0.5.0 change (the debounce is measured from the blur); seen to fail with the debounces broken (`E/nsh-form-tanstack/final-0.5.1/rows2-seen-to-fail.log`).
- Not run: device and packaging runs, the vision pass, cross-target `cargo check`. 0.5.1 changed lists, focus and overlays on devices; none of that was looked at on a screen.
- Fixed after the update (`E/fixes/`, `E/fixes/summary.txt`): both old failures came from the siblings' master moving to TanStack (`nsh-query` f6c0db8/e17723f, `nsh-form` 67fcf0c), not from NSH. `packages/remote-screen` is ported to master's TypeScript API (`QueryClientProvider`, `createQuery(() => ({ queryKey, retry: false, ... }))`, `createMutation` with `invalidateQueries` in `onSuccess`, `createForm(() => ({ defaultValues, validators: { onSubmitAsync } }))` for the server errors, `form.Field` with an `onBlur` rule). The Rust crate now keeps infinite lists in the client cache, so the data test's "Remount feed" got the cached pages and never failed; it now calls `query_client().remove(feed_key())` before remounting, as the crate's README says a first load needs. The app itself is unchanged: a remounted feed shows its cached pages, and the error state shows on a load with no pages. Gates: fmt 0, clippy 0, `cargo test` 127 passed / 0 failed, root `nsh check` 0, `nsh build` 0 with no warning, `nsh test App.test.tsx` 0, `nsh test smoke/pages.test.tsx` 0, remote-screen check/build/test 0 (9/9, build with no warning). Seen to fail: the retry test with Retry wired to nothing (`E/fixes/data-retry-seen-to-fail.log`), the signed-package test with a section title changed, and four package tests with invalidation, the server-error mapping, Retry and the name rule each broken (`E/fixes/remote-screen-*`). `scripts/delivery.sh` ran again with fresh throwaway keys in the git-ignored `.nsh/keys` (`docs/evidence/remote-screen/delivery.log`), so the Remote screen page has a delivery record again.

## 0.5.0 migration (2026-11-11)

Plan: `docs/nsh-0.5.0-migration.md`. Evidence: `docs/evidence/project/nsh-0.5.0/` (below, `E/`). Load on this Mac reached 200+ during the run (other agents' builds); nothing was lowered for it.

### What changed

- CLI: nsh 0.5.0 (f4d68f7) installed with the release's `install.sh` into `~/.nsh`, linked at `~/.local/bin/nsh`, which comes first on `PATH`, so no `justfile` override was needed (`E/phase1-which-a-nsh.log`). Homebrew's 0.4.0 is untouched. Components `android`, `ios`, `web` added (`E/phase1-component-add.log`). Help of every subcommand saved for both versions (`E/help/`); no flag the repo uses was removed or renamed (`E/phase1-help-diff-0.4.0-vs-0.5.0.txt`).
- Sibling libraries, uncommitted: `~/fo/nsh-query` (Cargo pins, `Cargo.lock`, CI `NSH_VERSION`, README "Requires NSH 0.5.0") and `~/fo/nsh-form` (the same, plus `packages/nsh-form/App.tsx` and `tests/rows2.test.tsx`: r3/r5 now measure the form-level async debounce from the blur, because a 0.5.0 test step can move the manual clock by a display frame; both were seen to fail with the debounce broken, `E/phase2-0.5.0-nsh-form-rows2-seen-to-fail.log`). Their `node_modules` were missing, so `bun install --frozen-lockfile` ran in both packages first.
- Starter: `Cargo.toml` pins 0.5.0 and `cargo update` (the old lock is kept at `E/Cargo.lock.before-0.5.0`); every NSH crate resolves to 0.5.0 (`E/phase3-nsh-versions-in-lock.txt`). Code: `LayoutCurve::FittedSpring { damping_ratio: 0.7 }` (reorder), `frame_rate: None` on the `Spring` const, `ColumnFixed` from `bool` (quotes), the window calls ported to `main_window()` / `WindowAttributes` / `set_mode` / `set_bounds` / `close()` (`src/ui/desktop.rs`, `src/shell/chrome.rs`, `src/platform/desktop_card.rs`), `use rust_i18n::t;` in ten modules (0.5.0's prelude has its own `t!`), `Idiom::Terminal` on the device page (+ `en`/`zh-CN` strings), and the capability registry lists 0.5.0's thirteen keys. Two overlay tests changed to press what a user can reach (see below).

### Verified, and how

- Baseline on 0.4.0 before any edit (`E/baseline-0.4.0-*.log`): fmt 0, clippy 0, `cargo test --no-fail-fast` 125 passed / 2 failed (`data::retry_recovers_a_failed_feed`, `remote_screen::the_signed_package_verifies_runs_and_then_refuses_after_revocation`), `nsh test App.test.tsx` 2/2, `nsh test smoke/pages.test.tsx` 1/1, `just test-tsx` failing on remote-screen type errors.
- On 0.5.0 (`E/final-0.5.0-*.log`): fmt 0, clippy 0, `cargo test --no-fail-fast` 125 passed / the same 2 failed, `nsh test App.test.tsx` 2/2, `nsh test smoke/pages.test.tsx` 1/1. `nsh check` at the root fails only on `packages/remote-screen` files (23 errors), the same errors the package shows on 0.4.0 (`E/baseline-0.4.0-withdeps-remote-screen-tsx-check.log` vs `E/phase5-0.5.0-remote-screen-tsx-check.log`); `vision/`, `trace/` and `smoke/` type-check on 0.5.0.
- New 0.5.0 failures fixed, not reported: `overlays::the_dialog_closes_on_a_scrim_press` and `the_sheet_opens_as_a_bottom_sheet_on_phone_width`. 0.5.0's headless `press` lands where a view is drawn; the scrim's center is under the dialog, and the page's detent buttons are under the open sheet. The tests now press the scrim at an uncovered corner (`press_at`) and pick each detent while the sheet is closed. Both seen to fail with the documented breaks (`E/phase6-overlays-seen-to-fail.log`).
- Cross-target `cargo check --lib`: `aarch64-apple-ios-sim` 0, `wasm32-unknown-unknown` 0, `aarch64-linux-android` 0 (`E/phase3-check-*.log`; Android needed `CC`/`AR` for `ring`, and NDK 28's `llvm-ar` because NDK 29's is a self-exec loop on this Mac).
- Siblings on 0.5.0: nsh-query fmt/clippy (default and `--all-features`)/test (both) all 0, TSX check/build/test 0 with 74/74 (`E/phase2-0.5.0-nsh-query-*`); nsh-form the same, TSX 100/100 (`E/phase2-0.5.0-nsh-form-*`). Their 0.4.0 TSX baselines with deps installed also pass (`E/baseline-0.4.0-withdeps-*`).
- Signed delivery flags on 0.5.0: keygen → certify → publish → verify → revoke → verify refuses, with the flags `tests/remote_screen.rs` and `scripts/delivery.sh` use, on a throwaway app in the scratchpad (`E/phase5-signed-delivery-probe-0.5.0.log`). The remote-screen package itself does not build on either version (below).
- Trace (criterion 7), `nsh test trace/feed.test.tsx --trace` on 0.5.0 (`E/final-0.5.0-trace.log`): like tap 1 mount item (was 1); scroll: 42 items per step while the pool fills, then 1–27, `nodes` flat at 479 after warm-up (was flat at 398); `open()` 527 items (was 363). The old numbers (`docs/evidence/data/trace-feed.txt`) predate the WIP shell redesign, so the node and open counts are not a clean version comparison.

### Behavior changes (Phase 4)

- Media grants (#1417): every image, video and SVG source in `src/` is `asset!` (an `asset:` URI) or inline SVG markup; `nsh-core-0.5.0/src/access.rs` always allows `asset:` and `data:`, so no `fs.read` grant was added. The picker shows a file name only. `packages/remote-screen` has no media. Inferred from source; no device run looked at the canvas page.
- `focus-visible:` keyboard-only (#1519): the app's wrappers use `focus:`, so they are unaffected; `nsh_base` components' own rings now show for keyboard focus only. Unverified on screen.
- Lists (#1132, #1057): `tests/lists.rs` (chat start loader, no-jump) and `tests/data.rs` pass as before. Not looked at on a device.
- `scrollbar(false)` (#937): the gallery rail now also hides its horizontal indicator on desktop. Unverified on screen.
- Back in an unselected tab (#996): `tests/shell.rs` back-order tests pass. Unverified on Android.
- Popover Tab order (#1616): `tests/overlays.rs` keyboard paths pass headless.
- macOS file drops (#1092): no drop handler exists in `src/`; nothing to change.
- macOS test hosts in the background (#907): see "Not verified".
- `nsh::perf` → `nsh::fps` (#1322): no use in `src/`, `trace/` or docs.
- Reorder spring: same damping ratio and duration; the feel was not looked at.

### Not verified, and why

- Packaging and device runs, partial (`E/phase6-package-*.log`): `nsh package --target macos` exit 0, and the packaged app started and stayed up 15 s (`E/phase6-macos-app-launch.log`; a window capture was refused by macOS, so no screenshot). `nsh package --target ios-simulator` exit 0. `nsh package --target web` and `nsh test --target web` fail because `nsh` sets `AR_wasm32_unknown_unknown=llvm-ar` and no `llvm-ar` is on this Mac's `PATH` (the same code is in 0.4.0's CLI); with Homebrew's `llvm-ar`/`clang` passed in, the web runtime built (`E/phase6-package-web-with-llvm.log`), then `cargo metadata` waited an hour on the shared cargo package lock held by another agent's processes and I stopped it, so the web site was neither packaged nor launched. `nsh test --target macos smoke/pages.test.tsx` reached 12 of 15 pages and then hit the test's 30 s limit at a load average near 200 (`E/phase6-package-test-macos.log`); the 10 screenshots it took are in `E/macos-smoke/`, all looked at, and show no breakage (the Data shots catch the loading state, the Forms shot catches the push mid-transition, and `page-ui.png` shows Home because the marker "UI gallery" already matches the sidebar; that last one is a test issue that predates 0.5.0). Not run: `nsh test --target ios|android`, `nsh package --target android`, the vision pass, and `scripts/rename.sh --roundtrip-test` (its gate cannot pass while the two old `cargo test` failures remain). The Android toolchain on this Mac also has the NDK `llvm-ar` loop described above.
- `packages/remote-screen` does not type-check or build on 0.4.0 or 0.5.0: it calls `nsh-query`/`nsh-form` APIs (`noRetries`, `invalidates`, `initial`, `form.field`, ...) that the siblings' current TanStack-based packages do not have. Pre-existing, not caused by 0.5.0; this also fails `just test-tsx` and the signed-package test. Fixing it is a port of the package, left to the owner.
- `packages/remote-screen/.nsh/` was deleted to regenerate types, as the plan says; it also held the git-ignored delivery record (`.nsh/delivery/state.json`) and throwaway keys. `scripts/delivery.sh` cannot rebuild them until the package builds, so the Remote screen page shows its "no delivery yet" state.
- 0.5.0's `nsh test` logs `warn the status bar is not granted` for apps that declare `capabilities` without `statusBar` (84 times in nsh-query's run); the tests pass. Not investigated beyond the source line that prints it.

### Follow-ups (out of scope)

Tray and global shortcuts (and moving `src/shell/command_bar.rs` onto `Window`/`Tray`); single instance; updater/OTA; `EmojiPicker`; `Terminal`; multi-window beyond the main window; haptics. See `docs/coverage.md` "New in 0.5.0, not yet used" and "New in 0.6.0, not yet used".
