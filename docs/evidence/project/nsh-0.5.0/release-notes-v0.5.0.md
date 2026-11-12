# NSH 0.5.0

NSH 0.5.0 brings a terminal backend, a fuller desktop application model, signed application updates, and tools for shipping to mobile stores. It also expands the component and system APIs, improves browser publishing, and addresses everyday interaction details across macOS, Windows, Linux, iOS, Android, and Web.

A large part of this release is about keeping real applications responsive: long conversations retain their scroll position, off-screen content is created incrementally, large tables avoid repeated text layout, and memory is returned when large view trees disappear. Text selection, input methods, menus, keyboard navigation, and screen-reader support receive substantial improvements alongside the new features.

## Breaking Changes

0.5.0 is a minor release because it changes public APIs and some default behavior. No version is recorded in `nsh-compat` yet, so these changes come before the permanent compatibility contract (see the [compatibility guide](https://github.com/longbridge/nsh/blob/v0.5.0/docs/dev/compatibility.md)).

### Apps (TSX and `nsh.json`)

- **Window API replaced.** Desktop windows now use `WindowRef`, the `<Window>` component and window kinds; the old window API is replaced, not aliased. Port secondary-window code to the new model. ([#1137](https://github.com/longbridge/nsh/pull/1137), [#1219](https://github.com/longbridge/nsh/pull/1219))
- **Media loading checks `capabilities` grants.** An app that declares `capabilities` and shows local files needs an `fs.read` grant for them; `new URL("./x.png", import.meta.url)` needs a `${pluginDir}` grant; relative media paths (including Markdown `![](img/a.png)`) load nothing in native apps; malformed `asset:` paths and grant paths with `.`/`..` are refused. Web view base URLs and parser-ambiguous URLs are checked the same way. Apps without `capabilities` are unchanged. ([#1417](https://github.com/longbridge/nsh/pull/1417), [#979](https://github.com/longbridge/nsh/pull/979))
- **`focus-visible:` only follows keyboard focus.** It no longer applies after a press or tap; use `focus:` for any focus. ([#1519](https://github.com/longbridge/nsh/pull/1519))
- **`macos.infoPlist` refuses `LSUIElement`.** Set `macos.activationPolicy` instead. `LSMinimumSystemVersion` is now accepted. ([#1390](https://github.com/longbridge/nsh/pull/1390), [#1654](https://github.com/longbridge/nsh/pull/1654))
- **Behavior changes.** A list at its start now shows rows inserted above its first row instead of scrolling to keep it ([#1132](https://github.com/longbridge/nsh/pull/1132)); the Tab order puts a popover's content right after its trigger ([#1616](https://github.com/longbridge/nsh/pull/1616)); macOS delivers Finder files as path `file:` URLs and lets drops on a text input reach the surrounding `onFileDrop` ([#1092](https://github.com/longbridge/nsh/pull/1092), [#1086](https://github.com/longbridge/nsh/pull/1086)); `scrollbar={false}` also hides horizontal indicators on desktop ([#937](https://github.com/longbridge/nsh/pull/937)); going back no longer pops a stack in an unselected tab ([#996](https://github.com/longbridge/nsh/pull/996)); invalid animation parameters now settle instead of producing `NaN` ([#1556](https://github.com/longbridge/nsh/pull/1556)); unreadable `nsh.json` settings are warnings, not errors ([#1190](https://github.com/longbridge/nsh/pull/1190)).

### Rust and embedders

- **Renamed or retyped APIs.** `Theme::from_json_lenient` → `Theme::from_json_with_diagnostics` ([#1571](https://github.com/longbridge/nsh/pull/1571)); `DataTableColumn::fixed` changes from `bool` to `ColumnFixed` (`Left` / `Right`) ([#1395](https://github.com/longbridge/nsh/pull/1395)); `LayoutCurve::Spring { damping }` becomes `LayoutCurve::FittedSpring { damping_ratio }` plus `LayoutCurve::Spring(Spring)` ([#1305](https://github.com/longbridge/nsh/pull/1305)).
- **Non-exhaustive types.** iOS `SurfaceOptions` and `ContextMenuEvent` are `#[non_exhaustive]`: start from `SurfaceOptions::default()` / `ContextMenuEvent::new` ([#1404](https://github.com/longbridge/nsh/pull/1404), [#978](https://github.com/longbridge/nsh/pull/978)). Platforms matching `Command` exhaustively must handle `Command::Haptic` ([#1328](https://github.com/longbridge/nsh/pull/1328)).
- **New struct fields.** Struct literals that list every field need the new ones or `..Default::default()`: `Accessibility`, `AccessibilityState`, `ViewProps`, `ResolvedTextStyle`, `LinkProps`, `BundleOptions`, `PerfSnapshot`, `nsh_platform_macos::AppOptions`, `Driver`/`KeyframeAnimation`/`Spring`/`Timing`. ([#1650](https://github.com/longbridge/nsh/pull/1650), [#1612](https://github.com/longbridge/nsh/pull/1612), [#1616](https://github.com/longbridge/nsh/pull/1616), [#1438](https://github.com/longbridge/nsh/pull/1438), [#1370](https://github.com/longbridge/nsh/pull/1370), [#1381](https://github.com/longbridge/nsh/pull/1381), [#1262](https://github.com/longbridge/nsh/pull/1262), [#1390](https://github.com/longbridge/nsh/pull/1390), [#998](https://github.com/longbridge/nsh/pull/998))
- **Theme readers.** Embedders using `Registry::tokens_from` / `Registry::tokens` follow the new `setTheme` signatures. ([#1216](https://github.com/longbridge/nsh/pull/1216))

### Tooling

- **Platform runtimes are components.** Release archives no longer bundle other platforms' runtimes or the embedding SDKs; `nsh` installs them on demand, and offline environments run `nsh component add android web` first. ([#1302](https://github.com/longbridge/nsh/pull/1302))
- **Performance monitor.** The per-second line moves from target `nsh::perf` to `nsh::fps`, and `nsh run --fps` shows it only with `-v`. ([#1322](https://github.com/longbridge/nsh/pull/1322))
- **macOS test hosts** start in the background without activating their windows. ([#907](https://github.com/longbridge/nsh/pull/907))

## Desktop windows and application lifecycle

- **Multi-window applications.** Desktop windows now have `WindowRef` identities, a declarative `<Window>` component, and window kinds. Rust applications can open and manage their own windows as well. This gives secondary windows and their content an explicit application-level model. ([#1137](https://github.com/longbridge/nsh/pull/1137), [#1219](https://github.com/longbridge/nsh/pull/1219))
- **Tray applications and global shortcuts.** Tray icons and global shortcuts are available on macOS, Windows, and Linux, including Rust applications. Tray events report icon bounds; macOS trays can show a title with configurable placement, making it possible to position a companion window beside the menu bar item. ([#988](https://github.com/longbridge/nsh/pull/988), [#1562](https://github.com/longbridge/nsh/pull/1562), [#1355](https://github.com/longbridge/nsh/pull/1355), [#1479](https://github.com/longbridge/nsh/pull/1479), [#1582](https://github.com/longbridge/nsh/pull/1582))
- **Background and single-instance apps.** Applications can opt into single-instance mode, start their main window hidden, and use the macOS menu-bar activation policy. Hidden main windows no longer prevent the app's other windows from receiving frames. ([#1563](https://github.com/longbridge/nsh/pull/1563), [#1420](https://github.com/longbridge/nsh/pull/1420), [#1390](https://github.com/longbridge/nsh/pull/1390), [#1652](https://github.com/longbridge/nsh/pull/1652))
- **Lifecycle and window behavior.** `Application.quit()` and `Application.restart()` let applications control their lifecycle, while `on_app_quit` provides Rust cleanup on exit. Window key bindings stay scoped to the correct window; Windows gains transparent windows and taskbar visibility control. ([#1025](https://github.com/longbridge/nsh/pull/1025), [#1055](https://github.com/longbridge/nsh/pull/1055), [#1558](https://github.com/longbridge/nsh/pull/1558), [#1354](https://github.com/longbridge/nsh/pull/1354), [#1391](https://github.com/longbridge/nsh/pull/1391), [#987](https://github.com/longbridge/nsh/pull/987))

## System integration and application services

- **Notifications and badges.** Applications can send system notifications and update their badge, with Windows toast/taskbar and Linux integration. Notifications can be silent, and an embedding host can hand notification delivery to the application it contains. ([#931](https://github.com/longbridge/nsh/pull/931), [#932](https://github.com/longbridge/nsh/pull/932), [#933](https://github.com/longbridge/nsh/pull/933), [#1140](https://github.com/longbridge/nsh/pull/1140), [#986](https://github.com/longbridge/nsh/pull/986))
- **Deep links, navigation, and file associations.** Custom URL schemes can launch applications; `UrlScheme.url` and `setUrl` expose the app's address, with history entries and URL-change reasons. Desktop applications can declare the file types they open. ([#1034](https://github.com/longbridge/nsh/pull/1034), [#982](https://github.com/longbridge/nsh/pull/982), [#1448](https://github.com/longbridge/nsh/pull/1448), [#1447](https://github.com/longbridge/nsh/pull/1447), [#1565](https://github.com/longbridge/nsh/pull/1565))
- **Files and application metadata.** `Picker.pickFiles` accepts general file types, `data_dir()` exposes the app's own data directory, and `appInfo()` reports the app version, build, and development flag. Drag-and-drop events gain file URLs and MIME types so apps can decide how to handle incoming files. ([#946](https://github.com/longbridge/nsh/pull/946), [#1550](https://github.com/longbridge/nsh/pull/1550), [#1381](https://github.com/longbridge/nsh/pull/1381), [#938](https://github.com/longbridge/nsh/pull/938), [#1441](https://github.com/longbridge/nsh/pull/1441))
- **Rust integration.** Module functions receiving `ModuleCx` can spawn tasks, app Rust functions can run on the platform's main thread, and mobile hosts can reuse application-owned Tokio runtimes. Platforms implemented outside the NSH repository can also run applications. ([#1372](https://github.com/longbridge/nsh/pull/1372), [#1382](https://github.com/longbridge/nsh/pull/1382), [#1404](https://github.com/longbridge/nsh/pull/1404), [#1393](https://github.com/longbridge/nsh/pull/1393))

## Application updates and distribution

- **Full application updates.** The new updater supports checking, downloading, installing, and restarting into an updated application. Download validation and the restart path are part of the flow; subsequent fixes retain an already downloaded update and restart into the newly installed app. ([#922](https://github.com/longbridge/nsh/pull/922), [#1042](https://github.com/longbridge/nsh/pull/1042), [#1269](https://github.com/longbridge/nsh/pull/1269))
- **Signed OTA packages.** Packaged applications can receive signed code-and-asset updates without replacing the installed shell when the existing runtime is sufficient. The launcher verifies packages and supports trial launches with fallback. On iOS and Android, an accepted package runs on the next cold start; runtime or native-code changes still require a new application build. ([#1189](https://github.com/longbridge/nsh/pull/1189))
- **Installed-shell visibility.** `shellInfo()` distinguishes the installed app and NSH runtime from a running OTA package. The updater avoids offering a redundant full-app update when its release is already running as a package, and the engine warns when an installed OTA package can no longer run. ([#1226](https://github.com/longbridge/nsh/pull/1226), [#1207](https://github.com/longbridge/nsh/pull/1207), [#1671](https://github.com/longbridge/nsh/pull/1671))
- **Mobile store publishing.** `nsh publish` adds TestFlight and Google Play internal-track publishing. Android packaging supports App Bundles and configurable target SDK versions; Apple packages can include additional Info.plist keys and entitlements. ([#1052](https://github.com/longbridge/nsh/pull/1052), [#1054](https://github.com/longbridge/nsh/pull/1054), [#1023](https://github.com/longbridge/nsh/pull/1023))
- **Platform packaging controls.** Configure icons separately per platform, declare Apple app languages, and raise a package's minimum system version. macOS packaging accepts `--arch`; Windows packaging gains Inno Setup installers and support for cross-building the runner off Windows. ([#1548](https://github.com/longbridge/nsh/pull/1548), [#1585](https://github.com/longbridge/nsh/pull/1585), [#1654](https://github.com/longbridge/nsh/pull/1654), [#1481](https://github.com/longbridge/nsh/pull/1481), [#1348](https://github.com/longbridge/nsh/pull/1348), [#1067](https://github.com/longbridge/nsh/pull/1067))

## Terminal applications and embedded terminals

- **A ratatui-based TUI backend.** NSH applications can run in a terminal. Follow-up work fixes initial layout, editor focus, multiline input scrolling, and startup negotiation so the terminal host behaves more reliably across launch and editing scenarios. ([#869](https://github.com/longbridge/nsh/pull/869), [#1006](https://github.com/longbridge/nsh/pull/1006), [#1083](https://github.com/longbridge/nsh/pull/1083), [#1288](https://github.com/longbridge/nsh/pull/1288), [#1340](https://github.com/longbridge/nsh/pull/1340))
- **Embedding and terminal links.** An embedding program can provide host options and stop the TUI host, including an exit requested before its run loop starts. `Link` content can render as OSC 8 hyperlinks in supporting terminals. ([#1287](https://github.com/longbridge/nsh/pull/1287), [#1307](https://github.com/longbridge/nsh/pull/1307), [#1289](https://github.com/longbridge/nsh/pull/1289))
- **The `Terminal` component.** Applications can display program output using each platform's text rendering, including the browser runtime. This is separate from running the entire application with the TUI backend. ([#1111](https://github.com/longbridge/nsh/pull/1111), [#1152](https://github.com/longbridge/nsh/pull/1152))

## Lists, scrolling, and large data sets

- **Chat-style list alignment.** Bottom-aligned lists open at the end and retain a stable position while earlier rows are measured, history is prepended, or row heights shrink. Loading history above a reader no longer incorrectly follows the end, and native bounce behavior is preserved. ([#983](https://github.com/longbridge/nsh/pull/983), [#990](https://github.com/longbridge/nsh/pull/990), [#1243](https://github.com/longbridge/nsh/pull/1243), [#1265](https://github.com/longbridge/nsh/pull/1265), [#1477](https://github.com/longbridge/nsh/pull/1477), [#1498](https://github.com/longbridge/nsh/pull/1498))
- **Stable row identity and interaction.** `keyExtractor` keys drive list reconciliation, retained rows are reused when their order is unchanged, and rows with keyboard focus or an open overlay remain rendered. These changes protect ongoing interaction while other rows enter and leave the viewport. ([#1057](https://github.com/longbridge/nsh/pull/1057), [#1407](https://github.com/longbridge/nsh/pull/1407), [#1102](https://github.com/longbridge/nsh/pull/1102))
- **Less work outside the viewport.** Scroll views create native content near the viewport, distant `Each` rows render as they approach, and off-screen creation is spread across frames. Fast scrolling binds rows farther ahead, while appending list rows avoids rebuilding every offset. ([#1304](https://github.com/longbridge/nsh/pull/1304), [#1312](https://github.com/longbridge/nsh/pull/1312), [#1600](https://github.com/longbridge/nsh/pull/1600), [#1310](https://github.com/longbridge/nsh/pull/1310), [#1371](https://github.com/longbridge/nsh/pull/1371))
- **DataTable performance and interaction.** Scrolling skips repeated cell text measurement and layout, including web wheel-inertia improvements. Buttons inside cells receive their own presses, columns can be fixed on the right, and edge anchoring reserves the native scroll room it needs. ([#1175](https://github.com/longbridge/nsh/pull/1175), [#1270](https://github.com/longbridge/nsh/pull/1270), [#1395](https://github.com/longbridge/nsh/pull/1395), [#1392](https://github.com/longbridge/nsh/pull/1392))
- **Programmatic scrolling and mobile controls.** View references can reveal a view through its enclosing scroll containers. Mobile platforms gain native scroll handles for both axes, and iOS refresh behavior improves for lists and scroll views beneath large titles. ([#1378](https://github.com/longbridge/nsh/pull/1378), [#1401](https://github.com/longbridge/nsh/pull/1401), [#1426](https://github.com/longbridge/nsh/pull/1426))

## Text, Markdown, and editors

- **Selection APIs and native menus.** Selectable views expose plain text, Markdown, and HTML, plus selection location. Selectable text uses the platform edit menu on right-click; macOS localizes that menu, Windows application context menus can act on selected text, and Android exposes installed text actions. ([#1045](https://github.com/longbridge/nsh/pull/1045), [#1504](https://github.com/longbridge/nsh/pull/1504), [#1517](https://github.com/longbridge/nsh/pull/1517), [#1579](https://github.com/longbridge/nsh/pull/1579), [#1505](https://github.com/longbridge/nsh/pull/1505), [#1516](https://github.com/longbridge/nsh/pull/1516))
- **Rich text editing.** Rich-editor documents can be imported and exported as HTML from JavaScript. Mobile selection gains touch handles and the system edit menu; local and data-URL images render with natural sizing when no size is supplied. ([#1578](https://github.com/longbridge/nsh/pull/1578), [#1368](https://github.com/longbridge/nsh/pull/1368), [#1577](https://github.com/longbridge/nsh/pull/1577), [#1121](https://github.com/longbridge/nsh/pull/1121))
- **Streamed Markdown.** Incomplete lines are held while their interpretation is still open, including reference brackets arriving over multiple chunks. CJK emphasis and URL handling are improved, and TeX preprocessing avoids quadratic searches through code spans. ([#1632](https://github.com/longbridge/nsh/pull/1632), [#1486](https://github.com/longbridge/nsh/pull/1486), [#1024](https://github.com/longbridge/nsh/pull/1024), [#1027](https://github.com/longbridge/nsh/pull/1027), [#1458](https://github.com/longbridge/nsh/pull/1458), [#1308](https://github.com/longbridge/nsh/pull/1308))
- **Input methods and native editing.** Windows text inputs use the system text box. Linux avoids moving multiline text during CJK composition, macOS leaves composition keys to the input method and commits composition on blur, and the editor keeps uncommitted text out of the document. ([#1475](https://github.com/longbridge/nsh/pull/1475), [#926](https://github.com/longbridge/nsh/pull/926), [#1387](https://github.com/longbridge/nsh/pull/1387), [#1507](https://github.com/longbridge/nsh/pull/1507), [#965](https://github.com/longbridge/nsh/pull/965))
- **Mobile input and paste.** Touch-focusing an editor on Web keeps the keyboard open; Android submission ends composition; iOS clips a paste to `maxLength` instead of rejecting the whole paste. Apps can insert an allowed paste as an undoable user edit. ([#1482](https://github.com/longbridge/nsh/pull/1482), [#1300](https://github.com/longbridge/nsh/pull/1300), [#1647](https://github.com/longbridge/nsh/pull/1647), [#1443](https://github.com/longbridge/nsh/pull/1443))
- **Text visibility and layout reuse.** macOS text remains visible inside transformed ancestors and when the first line exceeds its height, with the latter also fixed on iOS. Color-only changes and hidden-run changes preserve text layout; hidden spans are omitted from copying and screen-reader output. ([#1135](https://github.com/longbridge/nsh/pull/1135), [#1421](https://github.com/longbridge/nsh/pull/1421), [#1334](https://github.com/longbridge/nsh/pull/1334), [#1436](https://github.com/longbridge/nsh/pull/1436), [#1619](https://github.com/longbridge/nsh/pull/1619), [#1438](https://github.com/longbridge/nsh/pull/1438))

## Components, styling, and media

- **EmojiPicker and dynamic themes.** `EmojiPicker` provides searchable emoji selection. `setTheme` changes application themes at runtime, with validation for malformed themes, and applications can register fonts while running. ([#1323](https://github.com/longbridge/nsh/pull/1323), [#1216](https://github.com/longbridge/nsh/pull/1216), [#1571](https://github.com/longbridge/nsh/pull/1571), [#1001](https://github.com/longbridge/nsh/pull/1001))
- **Images, SVG, and audio.** Images accept in-memory RGBA pixel buffers; Canvas can draw SVG images, including text in native SVG rasterization. `Sound` and `AudioPlayer` add playback, and sounds can be loaded from bytes. Apple video playback can resume after seeking back from the end. ([#1561](https://github.com/longbridge/nsh/pull/1561), [#1291](https://github.com/longbridge/nsh/pull/1291), [#955](https://github.com/longbridge/nsh/pull/955), [#1471](https://github.com/longbridge/nsh/pull/1471), [#1377](https://github.com/longbridge/nsh/pull/1377))
- **Glass, corners, and shaders.** Progressive blur and squircle rendering expand across platforms, macOS gains configurable materials and glass tint compositing, and iOS backdrop effects use mask views. macOS and iOS shader frames render into IOSurfaces; shader initialization can try another backend when a device is refused. ([#930](https://github.com/longbridge/nsh/pull/930), [#1008](https://github.com/longbridge/nsh/pull/1008), [#1575](https://github.com/longbridge/nsh/pull/1575), [#1332](https://github.com/longbridge/nsh/pull/1332), [#1333](https://github.com/longbridge/nsh/pull/1333), [#1601](https://github.com/longbridge/nsh/pull/1601))
- **Input-aware styling and events.** Tailwind adds `pointer-fine:`, `pointer-coarse:`, and `terminal:` variants. `focus-visible:` follows keyboard-originated focus, and view events gain `onWheel` and `onMouseUp`. Text colors accept opacity modifiers for arbitrary and token values, and arbitrary background gradients accept Tailwind's `image` hint. ([#1539](https://github.com/longbridge/nsh/pull/1539), [#1519](https://github.com/longbridge/nsh/pull/1519), [#1567](https://github.com/longbridge/nsh/pull/1567), [#1446](https://github.com/longbridge/nsh/pull/1446), [#1669](https://github.com/longbridge/nsh/pull/1669), [#1670](https://github.com/longbridge/nsh/pull/1670))
- **Motion and layout correctness.** Layout springs use physics-based timing, retain momentum when retargeted, and keep values finite for unusual parameters. Shrink-to-fit flex rows and columns account for their content, including slot holders. ([#1305](https://github.com/longbridge/nsh/pull/1305), [#1553](https://github.com/longbridge/nsh/pull/1553), [#1556](https://github.com/longbridge/nsh/pull/1556), [#927](https://github.com/longbridge/nsh/pull/927), [#1461](https://github.com/longbridge/nsh/pull/1461), [#1625](https://github.com/longbridge/nsh/pull/1625))
- **Links and haptics.** Links show a pointing-hand cursor by default, text links reveal their URL on hover, and `nsh/mobile` gains haptic feedback on iOS and Android. ([#1311](https://github.com/longbridge/nsh/pull/1311), [#1370](https://github.com/longbridge/nsh/pull/1370), [#1328](https://github.com/longbridge/nsh/pull/1328))

## Menus, popovers, and keyboard navigation

- **Complete keyboard paths.** Context menus can open from the keyboard and screen readers. Virtualized menu lists support traversal through every row, Page Up/Page Down, and navigation beginning at the hovered item. Tab reaches virtualized list rows and scrolls the focused row into view. ([#978](https://github.com/longbridge/nsh/pull/978), [#1437](https://github.com/longbridge/nsh/pull/1437), [#1518](https://github.com/longbridge/nsh/pull/1518), [#1620](https://github.com/longbridge/nsh/pull/1620))
- **Predictable popup focus.** Popover content follows its trigger in Tab order; `onOpen` can move focus into the popup; focus returns when a popup starts closing. Escape also closes controlled popups opened by application code. ([#1616](https://github.com/longbridge/nsh/pull/1616), [#1551](https://github.com/longbridge/nsh/pull/1551), [#1618](https://github.com/longbridge/nsh/pull/1618), [#1622](https://github.com/longbridge/nsh/pull/1622))
- **Placement and dismissal.** Popups flip using their natural size, menus fit the available room beside their triggers and scroll when needed, and anchored overlays align to device pixels. Secondary clicks outside overlays dismiss them, and right-clicking a context-menu trigger again reopens the menu at that location. ([#1497](https://github.com/longbridge/nsh/pull/1497), [#1534](https://github.com/longbridge/nsh/pull/1534), [#1400](https://github.com/longbridge/nsh/pull/1400), [#1544](https://github.com/longbridge/nsh/pull/1544), [#1610](https://github.com/longbridge/nsh/pull/1610))
- **Touch and grouping behavior.** Long-press context menus apply to touch and pen and cancel the trigger's pending press. Tabs, radio buttons, and toolbar items can participate in their group through intervening plain views. ([#1555](https://github.com/longbridge/nsh/pull/1555), [#1415](https://github.com/longbridge/nsh/pull/1415), [#1521](https://github.com/longbridge/nsh/pull/1521))

## Accessibility

- **Reading order follows the screen.** Scroll-view and list content is exposed in visual order across desktop, Web, and Linux paths. Recycled list cells are removed from the accessible content they no longer represent. ([#1108](https://github.com/longbridge/nsh/pull/1108), [#1107](https://github.com/longbridge/nsh/pull/1107), [#1122](https://github.com/longbridge/nsh/pull/1122), [#1124](https://github.com/longbridge/nsh/pull/1124), [#1503](https://github.com/longbridge/nsh/pull/1503))
- **Richer semantics.** macOS exposes table rows and cells, scroll areas and positions; dialogs announce their role and title; elements can expose descriptions and an `invalid` state. Accessibility frames account for ancestor transforms, and Linux assistive technologies can activate plain buttons. ([#1192](https://github.com/longbridge/nsh/pull/1192), [#1509](https://github.com/longbridge/nsh/pull/1509), [#1624](https://github.com/longbridge/nsh/pull/1624), [#1650](https://github.com/longbridge/nsh/pull/1650), [#1612](https://github.com/longbridge/nsh/pull/1612), [#1182](https://github.com/longbridge/nsh/pull/1182), [#1255](https://github.com/longbridge/nsh/pull/1255))
- **Focus feedback.** Tooltips and focus-visible styling distinguish keyboard navigation from pointer interaction. A brief blur between focus changes no longer immediately drops `focus-within`, improving continuity when focus moves inside a control. ([#1653](https://github.com/longbridge/nsh/pull/1653), [#1519](https://github.com/longbridge/nsh/pull/1519), [#1557](https://github.com/longbridge/nsh/pull/1557))

## Web publishing and WebView

- **Published applications as web pages.** Web publishing adds document markup, links, and page-head support. Applications can configure whether navigation creates browser history entries, serve assets from a CDN with `--asset-url`, and resolve files relative to their starting page. ([#1130](https://github.com/longbridge/nsh/pull/1130), [#1158](https://github.com/longbridge/nsh/pull/1158), [#997](https://github.com/longbridge/nsh/pull/997), [#1155](https://github.com/longbridge/nsh/pull/1155))
- **Smaller application-specific runtimes.** Web linking includes only used `nsh/base` core, crypto, and URL functionality, including Rust applications. The bundler prunes statically accessed native namespace imports, reducing unnecessary runtime inclusion. The resulting size depends on the features an application imports. ([#1220](https://github.com/longbridge/nsh/pull/1220), [#1445](https://github.com/longbridge/nsh/pull/1445), [#1522](https://github.com/longbridge/nsh/pull/1522))
- **Browser startup and integration.** Import maps are installed before app modules start, including older Safari paths; startup waits for registered fonts. Browser lifecycle handling and delivery of `host.post` to the page's host improve embedding, while reference accounting does less work during application calls. ([#921](https://github.com/longbridge/nsh/pull/921), [#1118](https://github.com/longbridge/nsh/pull/1118), [#1046](https://github.com/longbridge/nsh/pull/1046), [#975](https://github.com/longbridge/nsh/pull/975), [#1668](https://github.com/longbridge/nsh/pull/1668), [#1659](https://github.com/longbridge/nsh/pull/1659))
- **WebView capabilities.** `evaluate` returns awaitable script results, WebViews can serve application-owned URL schemes, and page loads support request headers and an incognito mode. Camera and microphone access are added; applications can detect when the platform cannot show a WebView. ([#1432](https://github.com/longbridge/nsh/pull/1432), [#1233](https://github.com/longbridge/nsh/pull/1233), [#1038](https://github.com/longbridge/nsh/pull/1038), [#1028](https://github.com/longbridge/nsh/pull/1028), [#1559](https://github.com/longbridge/nsh/pull/1559), [#1439](https://github.com/longbridge/nsh/pull/1439))
- **Browser clipboard.** Image formats are converted to PNG for clipboard interoperability, while copied GIFs retain their animated representation alongside the PNG. ([#1540](https://github.com/longbridge/nsh/pull/1540), [#1564](https://github.com/longbridge/nsh/pull/1564))

## Runtime performance and platform reliability

- **Memory reclamation.** Large commits return excess operation-buffer capacity; disposed view trees return reactive graph slots and empty node-table pages. Completed spawned tasks release cleanup state, and shared styles/text-layout storage reduce duplicate allocations. ([#1259](https://github.com/longbridge/nsh/pull/1259), [#1260](https://github.com/longbridge/nsh/pull/1260), [#1261](https://github.com/longbridge/nsh/pull/1261), [#1664](https://github.com/longbridge/nsh/pull/1664), [#1250](https://github.com/longbridge/nsh/pull/1250), [#1341](https://github.com/longbridge/nsh/pull/1341))
- **Rendering efficiency.** Backdrops are recaptured when underlying content changes, Windows reduces canvas and partial-frame paint work, and macOS avoids redundant sibling scans and detaches deleted subtrees at their roots. These are targeted reductions in work rather than a single release-wide speedup claim. ([#1315](https://github.com/longbridge/nsh/pull/1315), [#1316](https://github.com/longbridge/nsh/pull/1316), [#1661](https://github.com/longbridge/nsh/pull/1661), [#1339](https://github.com/longbridge/nsh/pull/1339), [#1513](https://github.com/longbridge/nsh/pull/1513))
- **macOS and Windows windows.** Text-heavy window resizing is improved; macOS display links follow the window's screen rate and trackpad momentum stops on a mouse press. Windows fixes stretched resize frames, black resize edges, opening-content flashes, and frameless maximized state. ([#944](https://github.com/longbridge/nsh/pull/944), [#1245](https://github.com/longbridge/nsh/pull/1245), [#1511](https://github.com/longbridge/nsh/pull/1511), [#1142](https://github.com/longbridge/nsh/pull/1142), [#1402](https://github.com/longbridge/nsh/pull/1402), [#1364](https://github.com/longbridge/nsh/pull/1364), [#1422](https://github.com/longbridge/nsh/pull/1422))
- **Linux and Android reliability.** Linux keeps drawing after Wayland video closes and can bring a window forward after a tray click. Android gains API 26 support, avoids newer-API class loading on older systems, and fixes corner, GIF, and animation compatibility. ([#904](https://github.com/longbridge/nsh/pull/904), [#1615](https://github.com/longbridge/nsh/pull/1615), [#1363](https://github.com/longbridge/nsh/pull/1363), [#949](https://github.com/longbridge/nsh/pull/949), [#1047](https://github.com/longbridge/nsh/pull/1047), [#1082](https://github.com/longbridge/nsh/pull/1082), [#956](https://github.com/longbridge/nsh/pull/956))
- **Diagnostics.** The performance monitor can be enabled with `NSH_FPS` or application controls and reports JS heap and live-node information. CPU measurement uses a one-second window; packaged macOS and iOS apps write to the unified log. ([#1150](https://github.com/longbridge/nsh/pull/1150), [#1262](https://github.com/longbridge/nsh/pull/1262), [#1313](https://github.com/longbridge/nsh/pull/1313), [#1655](https://github.com/longbridge/nsh/pull/1655))

## Security and capability enforcement

- **Check resources at the point of use.** Media loaders enforce source grants, menu icons and image redirects are checked against capabilities, and native `<Image>` file loading opens files through their granted `fs.read` root. This closes gaps between an earlier permission check and the file actually opened. ([#1417](https://github.com/longbridge/nsh/pull/1417), [#1603](https://github.com/longbridge/nsh/pull/1603), [#1605](https://github.com/longbridge/nsh/pull/1605))
- **Network and WebView validation.** TLS verification uses the system trust store. WebView base URLs and ambiguous URL forms are checked against grants, and the development server restricts asset and WebSocket serving to its own pages. ([#1543](https://github.com/longbridge/nsh/pull/1543), [#979](https://github.com/longbridge/nsh/pull/979), [#1117](https://github.com/longbridge/nsh/pull/1117))
- **Packaging and update hardening.** Packaging rejects application IDs that are not a single path segment; the updater moves to zip 9 to address dependency advisories. Capability-safe file access also permits absolute symlinks whose targets remain within the granted root. ([#943](https://github.com/longbridge/nsh/pull/943), [#1164](https://github.com/longbridge/nsh/pull/1164), [#1297](https://github.com/longbridge/nsh/pull/1297))

## Developer tooling, tests, and documentation

- **CLI workflow.** `nsh update` installs the latest release or nightly, and platform runtimes are installed as components when needed. `nsh run --release` builds an optimized development host; concurrent apps choose free development ports, and mixed Rust/TSX changes trigger the required restart. ([#1273](https://github.com/longbridge/nsh/pull/1273), [#1302](https://github.com/longbridge/nsh/pull/1302), [#1317](https://github.com/longbridge/nsh/pull/1317), [#972](https://github.com/longbridge/nsh/pull/972), [#1487](https://github.com/longbridge/nsh/pull/1487))
- **Assets and configuration.** The bundler imports videos and `?url` assets as URLs, resolves subdirectory assets from the application directory, and recognizes dynamic imports using backtick strings. Unknown or unreadable configuration settings are reported as warnings so valid settings can still be used. ([#1380](https://github.com/longbridge/nsh/pull/1380), [#1592](https://github.com/longbridge/nsh/pull/1592), [#1178](https://github.com/longbridge/nsh/pull/1178), [#1190](https://github.com/longbridge/nsh/pull/1190))
- **Platform testing.** Tests can choose an iOS simulator, access layout boxes and platform text engines, invoke application-menu shortcuts, and substitute native modules. macOS/Android screenshot capture and GPUI covered-window capture receive fixes, and web test browsers close when a test times out. ([#1604](https://github.com/longbridge/nsh/pull/1604), [#1242](https://github.com/longbridge/nsh/pull/1242), [#1375](https://github.com/longbridge/nsh/pull/1375), [#1379](https://github.com/longbridge/nsh/pull/1379), [#1160](https://github.com/longbridge/nsh/pull/1160), [#1151](https://github.com/longbridge/nsh/pull/1151), [#1172](https://github.com/longbridge/nsh/pull/1172), [#1464](https://github.com/longbridge/nsh/pull/1464))
- **Design and internationalization.** App guidance now starts with a shared application UI layer and adds more complete design and API-naming rules. Runtime localization uses rust-i18n; translated documentation uses stable English heading anchors. The gallery and website share more component styling, and the website adds a community library/tool directory. ([#974](https://github.com/longbridge/nsh/pull/974), [#1529](https://github.com/longbridge/nsh/pull/1529), [#1530](https://github.com/longbridge/nsh/pull/1530), [#1234](https://github.com/longbridge/nsh/pull/1234), [#1353](https://github.com/longbridge/nsh/pull/1353), [#1267](https://github.com/longbridge/nsh/pull/1267), [#1369](https://github.com/longbridge/nsh/pull/1369))

## Upgrading from 0.4

Review the Breaking Changes above. Rebuild with the 0.5.0 toolchain and refresh generated TypeScript declarations; check desktop code against the `WindowRef` / `<Window>` model and the versioned APIs before shipping. Test keyboard, selection, and scrolling flows on the platforms your application supports, particularly if you supplied workarounds for behavior fixed here. ([#1137](https://github.com/longbridge/nsh/pull/1137), [#1219](https://github.com/longbridge/nsh/pull/1219))

For installed applications, distinguish a code-and-asset OTA update from a shell update: using a new NSH runtime or new native functionality requires shipping a new full application. `shellInfo()` and the package's minimum-runtime requirement help keep those paths explicit. See the [0.5.0 update documentation](https://github.com/longbridge/nsh/blob/v0.5.0/docs/dev/updater.md).

Version 0.5.0 is not yet a recorded compatibility baseline. The permanent backward-compatibility contract begins with the first release recorded in `nsh-compat`; the [versioned compatibility guide](https://github.com/longbridge/nsh/blob/v0.5.0/docs/dev/compatibility.md) explicitly lists 0.3.0, 0.4.0, and 0.5.0 as unrecorded.

<details>
<summary>Contributors</summary>

Thanks to everyone who contributed to this release:

@0xbentang @ahonn @akongG @antoniel @BeADre @Blackman99 @boyan01 @burdygin @caoxing9 @chaxus @ClarkThan @Cooooooker @cosformula @Cupnfish @daoleno @duo @dXnSix @griffinqiu @hexuria @hmziqrs @honsunrise @hsingjui @huacnlee @ihavecoke @itisvincent @Joy-jinyu @kim0 @kindred-7 @kosumic @Laotree @LeeDashan @Lemenxzy @linyiru @lucianosrp @lurenjia534 @LVtomatoJ @madcodelife @marcromeyn @mengyinghu-sys @mrtolkien @nohzafk @obito-t @RebelBIrd @richerfu @sunfuze @sunli829 @sureshkrishnan-ai @t4t5 @taokeqin @Teages @TexasOct @ttizze @twosugar @Tzyito @Vehicoule @williamjakfar @WillisXue @wuyunyoujiduo-jpg @yangkx1024 @YougLin-dev @yuanzhuang-svg @yuezk @zcg @ZZombiee

</details>

<details>
<summary>What's Changed</summary>

## What's Changed
* gallery: Make the Plot page's charts as they scroll near by @madcodelife in https://github.com/longbridge/nsh/pull/897
* website: Publish merged catalog apps to App Stories by @huacnlee in https://github.com/longbridge/nsh/pull/899
* ci: Trust the local tap before testing the Homebrew formula by @huacnlee in https://github.com/longbridge/nsh/pull/906
* core: Add features for shaders and the large parts of a runtime by @huacnlee in https://github.com/longbridge/nsh/pull/898
* website: Add a release notes page by @huacnlee in https://github.com/longbridge/nsh/pull/910
* bundler: Decode entities beside multibyte JSX text by @ClarkThan in https://github.com/longbridge/nsh/pull/909
* docs: Say that camera and Bluetooth need an embedded app by @huacnlee in https://github.com/longbridge/nsh/pull/918
* build: Drop the unused nsh-platform-android workspace dependency by @huacnlee in https://github.com/longbridge/nsh/pull/843
* gallery: Refine the demos' look and show one set of SpaceX photos by @huacnlee in https://github.com/longbridge/nsh/pull/902
* linux: Keep the window drawing after a video closes on Wayland by @huacnlee in https://github.com/longbridge/nsh/pull/904
* macos: Close the window without panicking when an app closes it by @ihavecoke in https://github.com/longbridge/nsh/pull/903
* gallery: Wait for the Editor demo before pressing its language menu by @huacnlee in https://github.com/longbridge/nsh/pull/920
* web: Start published apps in older Safari by registering the import map first by @twosugar in https://github.com/longbridge/nsh/pull/921
* tui: Add a terminal backend that renders apps with ratatui by @griffinqiu in https://github.com/longbridge/nsh/pull/869
* diff_view: Cover the width of a line number's column with a test by @huacnlee in https://github.com/longbridge/nsh/pull/911
* desktop: Show icons on menu items by @huacnlee in https://github.com/longbridge/nsh/pull/929
* notifications: Add system notifications and the app's badge by @huacnlee in https://github.com/longbridge/nsh/pull/931
* style: Draw progressive blurs and squircle corners on more platforms by @huacnlee in https://github.com/longbridge/nsh/pull/930
* website: Add a brand page with the new logo, icons and previews by @huacnlee in https://github.com/longbridge/nsh/pull/923
* web: Draw a screen effect over animated shaders at the display's rate by @dXnSix in https://github.com/longbridge/nsh/pull/928
* linux: Keep a multiline input's text still when CJK is typed by @huacnlee in https://github.com/longbridge/nsh/pull/926
* core: Size rows with flex items by their content when shrink-to-fit by @huacnlee in https://github.com/longbridge/nsh/pull/927
* notifications: Show toasts and the taskbar badge on Windows by @huacnlee in https://github.com/longbridge/nsh/pull/932
* notifications: Show notifications and the badge on Linux by @huacnlee in https://github.com/longbridge/nsh/pull/933
* cli: Run apps on another nsh version with this one by @huacnlee in https://github.com/longbridge/nsh/pull/919
* gpui: Let a draggable view's onContextMenu hear its right click on Linux by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/873
* linux: Draw continuous corners and maskImage by @huacnlee in https://github.com/longbridge/nsh/pull/934
* updater: Add app self-update by @huacnlee in https://github.com/longbridge/nsh/pull/922
* text: Keep a window's resize at the display's rate on long pages by @huacnlee in https://github.com/longbridge/nsh/pull/944
* website: Fix horizontal overflow on phones by @huacnlee in https://github.com/longbridge/nsh/pull/953
* macos: Run test hosts without activating their windows by @ihavecoke in https://github.com/longbridge/nsh/pull/907
* android: Load API 30 and 31 types only on the versions that have them by @huacnlee in https://github.com/longbridge/nsh/pull/949
* skills: Add NSH skills for coding agents by @huacnlee in https://github.com/longbridge/nsh/pull/950
* android: Stop a view's animations when it or its app goes away by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/956
* package: Refuse app ids that are not one path segment by @ClarkThan in https://github.com/longbridge/nsh/pull/943
* website: Add machine-translated documentation languages by @huacnlee in https://github.com/longbridge/nsh/pull/952
* ios: Launch on iOS 15 and 16 without the iOS 17 text content types by @huacnlee in https://github.com/longbridge/nsh/pull/939
* website: State that apps built with NSH may use the logo by @huacnlee in https://github.com/longbridge/nsh/pull/957
* linux: Bake cached shadows free of the gl renderer's stale textures by @huacnlee in https://github.com/longbridge/nsh/pull/951
* docs: introduce TUI on the homepage and platform lists by @huacnlee in https://github.com/longbridge/nsh/pull/967
* picker: Add Picker.pickFiles to pick files of any type by @huacnlee in https://github.com/longbridge/nsh/pull/946
* docs: establish app design systems before building NSH screens by @huacnlee in https://github.com/longbridge/nsh/pull/974
* docs: Prefer virtual lists for growing collections by @huacnlee in https://github.com/longbridge/nsh/pull/980
* ci: Check compatibility against nsh-compat's default branch again by @sunli829 in https://github.com/longbridge/nsh/pull/970
* windows: Scale tinted images for the target DPI by @Cupnfish in https://github.com/longbridge/nsh/pull/890
* core: Build for Emscripten, where Unix error names need libc by @lucianosrp in https://github.com/longbridge/nsh/pull/896
* core: Add List alignment to rest a list at its end like a chat by @huacnlee in https://github.com/longbridge/nsh/pull/983
* core: Open context menus from the keyboard and screen readers by @madcodelife in https://github.com/longbridge/nsh/pull/978
* desktop: Add Window.open for windows of the app's own (Linux) by @huacnlee in https://github.com/longbridge/nsh/pull/989
* cli: Pick a free dev server port so apps run side by side by @huacnlee in https://github.com/longbridge/nsh/pull/972
* editor: Keep an input method's uncommitted text out of the document by @huacnlee in https://github.com/longbridge/nsh/pull/965
* windows: Add transparent windows and Window.setSkipTaskbar by @huacnlee in https://github.com/longbridge/nsh/pull/987
* web: Give browser apps their lifecycle by @dXnSix in https://github.com/longbridge/nsh/pull/975
* base: Keep text typed again after Backspace or a caret move by @BeADre in https://github.com/longbridge/nsh/pull/905
* core: Keep a bottom-aligned List at its end as rows above it are measured by @huacnlee in https://github.com/longbridge/nsh/pull/990
* android: Give Picker.pickFiles its own command code by @huacnlee in https://github.com/longbridge/nsh/pull/994
* cli: Show build progress before launching TUI by @huacnlee in https://github.com/longbridge/nsh/pull/971
* website: Redesign the home page around what makes NSH different by @huacnlee in https://github.com/longbridge/nsh/pull/993
* updater: Pass clippy on iOS, Android and the web by @dXnSix in https://github.com/longbridge/nsh/pull/1002
* core: End animations on time while a window draws no frames by @sunli829 in https://github.com/longbridge/nsh/pull/964
* windows: Keep GPU drivers from starting worker threads for each device by @LeeDashan in https://github.com/longbridge/nsh/pull/947
* macos: Fold a control's content into its name, as Windows and GPUI do by @0xbentang in https://github.com/longbridge/nsh/pull/941
* website: Read translations from longbridge/nsh-website-translates by @huacnlee in https://github.com/longbridge/nsh/pull/1007
* website: Lay out right-to-left languages mirrored by @huacnlee in https://github.com/longbridge/nsh/pull/1000
* macos: Count double and triple presses in the test agent by @ihavecoke in https://github.com/longbridge/nsh/pull/999
* android: Keep the ripple's mask on the view's current shape by @Teages in https://github.com/longbridge/nsh/pull/1003
* macos: Draw what was put off when an AppView enters a window or shows again by @nohzafk in https://github.com/longbridge/nsh/pull/1019
* windows: Open windows of the app's own with Window.open by @huacnlee in https://github.com/longbridge/nsh/pull/1011
* desktop: Add Tray and GlobalShortcut, on Linux, macOS and Windows by @huacnlee in https://github.com/longbridge/nsh/pull/988
* windows: Show notifications from an app that has not shown one yet by @huacnlee in https://github.com/longbridge/nsh/pull/1009
* windows: Show the app's icon on the taskbar and in notifications by @huacnlee in https://github.com/longbridge/nsh/pull/1010
* tui: Restore editor focus and cover terminal regressions in CI by @huacnlee in https://github.com/longbridge/nsh/pull/1006
* core: Let animations ask for a lower frame rate by @madcodelife in https://github.com/longbridge/nsh/pull/998
* web: Measure a mouse as the desktop idiom at any width by @huacnlee in https://github.com/longbridge/nsh/pull/1026
* js: Keep a live memo's value when its object is collected by @sunfuze in https://github.com/longbridge/nsh/pull/1014
* desktop: Let a host hand its notifications to the app it embeds by @huacnlee in https://github.com/longbridge/nsh/pull/986
* core: Let a web view load a page with request headers by @sunfuze in https://github.com/longbridge/nsh/pull/1038
* text_view: Read emphasis beside CJK text as the CJK-friendly rule does by @madcodelife in https://github.com/longbridge/nsh/pull/1024
* gallery: Open the window from the tray menu and drop the first row's line by @huacnlee in https://github.com/longbridge/nsh/pull/1040
* platform: Share Apple speech and notifications through a crate, handle taffy calc by @huacnlee in https://github.com/longbridge/nsh/pull/1022
* menu: Open a context menu on a held finger in the desktop idiom too by @huacnlee in https://github.com/longbridge/nsh/pull/1033
* cli: Let apps add Info.plist keys and entitlements to iOS and macOS packages by @madcodelife in https://github.com/longbridge/nsh/pull/1023
* desktop: Add `Application.quit()` to quit the app from its own code by @madcodelife in https://github.com/longbridge/nsh/pull/1025
* text_view: Show tags that name no HTML element as text by @madcodelife in https://github.com/longbridge/nsh/pull/1036
* cli: Package Android App Bundles and let nsh.json set targetSdk by @huacnlee in https://github.com/longbridge/nsh/pull/1054
* cli: Add nsh publish for TestFlight and Play internal track by @huacnlee in https://github.com/longbridge/nsh/pull/1052
* runtime: Open apps with links of their own URL schemes by @huacnlee in https://github.com/longbridge/nsh/pull/1034
* text_view: End bare URLs where Chinese, Japanese or Korean text starts by @madcodelife in https://github.com/longbridge/nsh/pull/1027
* list: Diff `each` by keyExtractor keys when given by @huacnlee in https://github.com/longbridge/nsh/pull/1057
* core: Keep a web view's website data in memory with `incognito` by @madcodelife in https://github.com/longbridge/nsh/pull/1028
* selection: Read a selectable view's selection as text, Markdown or HTML by @huacnlee in https://github.com/longbridge/nsh/pull/1045
* updater: Keep the held download when a check finds the same version by @huacnlee in https://github.com/longbridge/nsh/pull/1042
* text_view: Keep escaped brackets around reference numbers as text by @madcodelife in https://github.com/longbridge/nsh/pull/1035
* menu: Report a submenu's openings and closings with `onOpenChange` by @madcodelife in https://github.com/longbridge/nsh/pull/1063
* gpui: Read the owned window test's app height after the menu bar settles by @madcodelife in https://github.com/longbridge/nsh/pull/1066
* tui: Show an app loaded before the first draw at the drawn area's layout by @madcodelife in https://github.com/longbridge/nsh/pull/1083
* cli: Name `schemes` as the owner of `CFBundleURLTypes` by @madcodelife in https://github.com/longbridge/nsh/pull/1073
* android: Keep border colors through a style transition by @Teages in https://github.com/longbridge/nsh/pull/1021
* website: Carry the new logo and banner style across the site and gallery by @huacnlee in https://github.com/longbridge/nsh/pull/1072
* core: Read the window's bounds and the displays on the desktops by @madcodelife in https://github.com/longbridge/nsh/pull/1044
* examples: Share one button look between the gallery and the website by @madcodelife in https://github.com/longbridge/nsh/pull/1085
* audio: Play sounds and audio with Sound and AudioPlayer by @LeeDashan in https://github.com/longbridge/nsh/pull/955
* examples: Share one switch look between the gallery and the website by @madcodelife in https://github.com/longbridge/nsh/pull/1088
* js: Make List slots that are an If or Each follow their conditions by @Cooooooker in https://github.com/longbridge/nsh/pull/900
* gpui: Build on gpui-fast 0.1.13 by @obito-t in https://github.com/longbridge/nsh/pull/1100
* image: Account for macOS backing stores and release evicted metadata by @huacnlee in https://github.com/longbridge/nsh/pull/1041
* testing: Fail a press on a view another is drawn over by @marcromeyn in https://github.com/longbridge/nsh/pull/881
* desktop: Distinguish background clicks from window drags by @yuezk in https://github.com/longbridge/nsh/pull/901
* gpui: Keep Window.setFrame sizes at least 1 point and leave maximized windows alone by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/908
* dev: Serve the page at the app's own paths, as hosts of single-page apps do by @dXnSix in https://github.com/longbridge/nsh/pull/981
* term: Add Terminal, a program's output drawn with each platform's text by @huacnlee in https://github.com/longbridge/nsh/pull/1111
* examples: Keep the website demos' views when the window crosses a width by @madcodelife in https://github.com/longbridge/nsh/pull/1093
* core: Mark lists in their parent's viewport to be read in screen order by @madcodelife in https://github.com/longbridge/nsh/pull/1109
* desktop: Read a scroll view's content in the order it is on screen by @madcodelife in https://github.com/longbridge/nsh/pull/1108
* examples: Sink every gallery press natively, not from JS springs by @madcodelife in https://github.com/longbridge/nsh/pull/1090
* dock: Test that an area born compact reports its docks as it widens by @madcodelife in https://github.com/longbridge/nsh/pull/1094
* examples: Share one toggle and toolbar look between the gallery and the website by @madcodelife in https://github.com/longbridge/nsh/pull/1101
* website: List Traditional Chinese right after Simplified Chinese by @huacnlee in https://github.com/longbridge/nsh/pull/1123
* macos: Release the selection's borrow before reporting a select-all by @LeeDashan in https://github.com/longbridge/nsh/pull/1112
* examples: Move the shared finishes onto the cool neutral palette by @madcodelife in https://github.com/longbridge/nsh/pull/1106
* testing: Find a scroll view's content in the order it is on screen by @madcodelife in https://github.com/longbridge/nsh/pull/1105
* web: Read a scroll view's content in the order it is on screen by @madcodelife in https://github.com/longbridge/nsh/pull/1107
* js: Keep the VM alive after a call is interrupted inside `find` by @madcodelife in https://github.com/longbridge/nsh/pull/835
* desktop: Let scrollbar={false} hide a horizontal view's indicator by @antoniel in https://github.com/longbridge/nsh/pull/937
* core: Tell apps the user's locale settings by @dXnSix in https://github.com/longbridge/nsh/pull/976
* linux: Read a scroll view's content in the order it is on screen by @madcodelife in https://github.com/longbridge/nsh/pull/1122
* list: Give a row coming back into the window the cell that still shows it by @madcodelife in https://github.com/longbridge/nsh/pull/1099
* dock: Press a compact drawer's own buttons in tests, and lock `vte` for GPUI by @madcodelife in https://github.com/longbridge/nsh/pull/1125
* list: Keep rows holding the keyboard focus or an open overlay rendered by @madcodelife in https://github.com/longbridge/nsh/pull/1102
* core: Keep a list's cells in screen order in the tracked tree order by @madcodelife in https://github.com/longbridge/nsh/pull/1124
* list: Make `virtual_list` rows under the owner that made the list by @madcodelife in https://github.com/longbridge/nsh/pull/1096
* linux: Format `NshView`'s next accessible sibling lookup by @madcodelife in https://github.com/longbridge/nsh/pull/1131
* web: Clear animated values from a view once they are bound no more by @LVtomatoJ in https://github.com/longbridge/nsh/pull/992
* core: Keep the pages of a tab not selected when going back by @chaxus in https://github.com/longbridge/nsh/pull/996
* macos: Stop test steps waiting out keyframes that repeat forever by @ihavecoke in https://github.com/longbridge/nsh/pull/1032
* cli: Add --asset-url to serve a web site's files from a CDN by @chaxus in https://github.com/longbridge/nsh/pull/997
* text: Let apps register fonts while they run by @dXnSix in https://github.com/longbridge/nsh/pull/1001
* website: Translate the home page into every machine-translated language by @huacnlee in https://github.com/longbridge/nsh/pull/1127
* notifications: Let an app show a notification without a sound by @huacnlee in https://github.com/longbridge/nsh/pull/1140
* macos: Reduce redraw work for small canvas animations by @linyiru in https://github.com/longbridge/nsh/pull/1016
* windows: Stop stretching the last frame while a window resizes by @huacnlee in https://github.com/longbridge/nsh/pull/1142
* ci: Run pull requests' jobs only while their results still matter by @huacnlee in https://github.com/longbridge/nsh/pull/1141
* website: Keep the home page's platforms on one row when the window widens by @huacnlee in https://github.com/longbridge/nsh/pull/1128
* text_view: Report the modifier keys held with a link's press by @sunfuze in https://github.com/longbridge/nsh/pull/1126
* macos: Give dropped and pasted Finder files as path URLs by @ihavecoke in https://github.com/longbridge/nsh/pull/1092
* core: Release built-in scroll bars in the reactive runtime by @ihavecoke in https://github.com/longbridge/nsh/pull/1095
* android: Give bindgen the versioned target triple NDK 30 requires by @huacnlee in https://github.com/longbridge/nsh/pull/1113
* android: Keep continuous corners from crashing Android 9 at launch by @akongG in https://github.com/longbridge/nsh/pull/1047
* android: Play GIFs with Movie below Android 11 instead of aborting by @akongG in https://github.com/longbridge/nsh/pull/1082
* windows: Pace each window's present so several windows share a UI thread by @LeeDashan in https://github.com/longbridge/nsh/pull/1081
* web: Keep the app hidden until every face registered at start has loaded by @cosformula in https://github.com/longbridge/nsh/pull/1046
* linux: Measure each child before allocating it, as GTK expects by @yangkx1024 in https://github.com/longbridge/nsh/pull/1049
* desktop: Add `Application.restart()` to restart the app from its own code by @LeeDashan in https://github.com/longbridge/nsh/pull/1055
* gallery: Give the gallery and the site a sound for each kind of moment by @huacnlee in https://github.com/longbridge/nsh/pull/1149
* cli: Cross-build the Windows runner when packaging off Windows by @huacnlee in https://github.com/longbridge/nsh/pull/1067
* linux: End WebKit's network process with an app's last web view by @yangkx1024 in https://github.com/longbridge/nsh/pull/1051
* android: open_path — one of the app's own files to the app that opens it by @Cupnfish in https://github.com/longbridge/nsh/pull/1070
* cli: packaging fixes for Windows and newer NDKs by @Cupnfish in https://github.com/longbridge/nsh/pull/1071
* macos: Open windows of the app's own with Window.open by @LeeDashan in https://github.com/longbridge/nsh/pull/1069
* terminal: Use the line memo without cloning it, as `Memo` is `Copy` by @madcodelife in https://github.com/longbridge/nsh/pull/1166
* android: Take test screenshots of the windows as the device shows them by @huacnlee in https://github.com/longbridge/nsh/pull/1151
* macos: Take test screenshots of the window as the screen shows it by @huacnlee in https://github.com/longbridge/nsh/pull/1160
* macos: Run nsh test --target macos while the screen is locked by @huacnlee in https://github.com/longbridge/nsh/pull/1161
* web: Register Terminal in the browser runtimes by @huacnlee in https://github.com/longbridge/nsh/pull/1152
* web: Show backdrop filters inside views that clip with continuous corners by @huacnlee in https://github.com/longbridge/nsh/pull/1153
* cli: Find Rust programs a cargo built in the workspace's own target by @huacnlee in https://github.com/longbridge/nsh/pull/1168
* ci: Run main's checks and publish the website again by @huacnlee in https://github.com/longbridge/nsh/pull/1170
* web: Draw nothing of a view's own while it has no size by @madcodelife in https://github.com/longbridge/nsh/pull/1148
* macos: Show the performance monitor when Info.plist asks for it by @sunfuze in https://github.com/longbridge/nsh/pull/1056
* menu: Stop panicking when a menu goes before its submenus' cleanup by @madcodelife in https://github.com/longbridge/nsh/pull/1144
* examples: Share one menu and overlay look between the gallery and the website by @madcodelife in https://github.com/longbridge/nsh/pull/1143
* macos: Let files dropped on a text input reach the drop target around it by @ihavecoke in https://github.com/longbridge/nsh/pull/1086
* ci: Publish the website even when the Gallery outgrows its size budget by @huacnlee in https://github.com/longbridge/nsh/pull/1183
* core: Read nsh.json settings NSH cannot read as warnings, not errors by @huacnlee in https://github.com/longbridge/nsh/pull/1190
* macos: Keep embedded AppView backgrounds transparent by @huacnlee in https://github.com/longbridge/nsh/pull/1084
* core: Turn the performance monitor on with NSH_FPS or from the app by @huacnlee in https://github.com/longbridge/nsh/pull/1150
* ci: Run gallery tests in GPUI windows under Xvfb by @huacnlee in https://github.com/longbridge/nsh/pull/1169
* data-table: Skip measuring and relayout of cell text while scrolling by @huacnlee in https://github.com/longbridge/nsh/pull/1175
* linux: Document screenshot fidelity and renderer limits by @huacnlee in https://github.com/longbridge/nsh/pull/1173
* gpui: Keep Linux screenshots intact under covering windows by @huacnlee in https://github.com/longbridge/nsh/pull/1172
* linux: Install NshView's reading order for screen readers by @LeeDashan in https://github.com/longbridge/nsh/pull/1176
* android: Keep transition colors in range through an overshooting easing by @Teages in https://github.com/longbridge/nsh/pull/1110
* desktop: Give every window a `WindowRef`, `<Window>` and window kinds by @madcodelife in https://github.com/longbridge/nsh/pull/1137
* updater: Add OTA updates of a packaged app's code through signed packages by @huacnlee in https://github.com/longbridge/nsh/pull/1189
* updater: Don't offer the whole app while a package of its release runs by @huacnlee in https://github.com/longbridge/nsh/pull/1207
* macos: Include ancestor transforms in accessibility frames by @0xbentang in https://github.com/longbridge/nsh/pull/1182
* macos: Expose table rows and cells to accessibility by @0xbentang in https://github.com/longbridge/nsh/pull/1192
* linux: tray: Paint an SVG icon as a template, like macOS and Windows by @huacnlee in https://github.com/longbridge/nsh/pull/1209
* windows: Run posted UI work on the thread that owns its surface by @madcodelife in https://github.com/longbridge/nsh/pull/1195
* gpui: Type Shift's character for a shifted key in nsh test by @madcodelife in https://github.com/longbridge/nsh/pull/1205
* linux: Test keyboard input through XTest on X11 displays by @huacnlee in https://github.com/longbridge/nsh/pull/1199
* engine: Compile Driver::trial only where the native engine thread reads it by @huacnlee in https://github.com/longbridge/nsh/pull/1208
* runtime: Add `setTheme` to change the app's theme while it runs by @Blackman99 in https://github.com/longbridge/nsh/pull/1216
* core: Stop the reactive graph panicking when a node is disposed mid-run by @Blackman99 in https://github.com/longbridge/nsh/pull/1214
* web: Handle desktop context-menu keyboard requests by @honsunrise in https://github.com/longbridge/nsh/pull/1098
* engine: Read the same bundled file whatever the URL's case or query by @dXnSix in https://github.com/longbridge/nsh/pull/1156
* linux: Ignore the real pointer under nsh test by @huacnlee in https://github.com/longbridge/nsh/pull/1174
* windows: Paint the window background before showing it by @YougLin-dev in https://github.com/longbridge/nsh/pull/1177
* ci: Build with huacnlee/cargo and cache its dependency builds per job by @huacnlee in https://github.com/longbridge/nsh/pull/1184
* docs: Recommend huacnlee/cargo wherever Rust apps start by @huacnlee in https://github.com/longbridge/nsh/pull/1187
* macos: Center the traffic lights in a hidden title bar without a toolbar by @LeeDashan in https://github.com/longbridge/nsh/pull/1196
* android: Keep outer box shadows through opacity animations by @Teages in https://github.com/longbridge/nsh/pull/1212
* android: Keep a split clip's wrapper from taking touches by @Teages in https://github.com/longbridge/nsh/pull/1138
* web: Start app realms after installing their complete import map by @TexasOct in https://github.com/longbridge/nsh/pull/1118
* macos: Add configurable materials and glass tint compositing by @honsunrise in https://github.com/longbridge/nsh/pull/1008
* windows: Initialize WinRT in the toast parser test by @WillisXue in https://github.com/longbridge/nsh/pull/1087
* core: Reconcile reused nodes against current bound values by @0xbentang in https://github.com/longbridge/nsh/pull/1103
* windows: Decode sounds off the main thread by @LeeDashan in https://github.com/longbridge/nsh/pull/1116
* linux: Report a sound whose format has no plug-in as not-supported by @LeeDashan in https://github.com/longbridge/nsh/pull/1115
* platform: Share Apple sound playback through nsh-platform-apple by @LeeDashan in https://github.com/longbridge/nsh/pull/1114
* desktop: Open windows from Rust and name every window a WindowRef by @huacnlee in https://github.com/longbridge/nsh/pull/1219
* base: Stop blinking the Editor's carets after ten idle seconds on Linux by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/1120
* updater: Add shellInfo() with the installed app's version and NSH runtime by @huacnlee in https://github.com/longbridge/nsh/pull/1226
* testing: Let nsh-testing's and nsh-engine's tests pass when run with -p by @chaxus in https://github.com/longbridge/nsh/pull/1222
* web: Load an app's files and images from where its page started by @dXnSix in https://github.com/longbridge/nsh/pull/1155
* website: Draw the hero's dark arches as smoky glass by @huacnlee in https://github.com/longbridge/nsh/pull/1228
* ci: Publish a nightly build of main every night by @huacnlee in https://github.com/longbridge/nsh/pull/1229
* web: Keep the spaces a text's lines broke at in its markup by @Tzyito in https://github.com/longbridge/nsh/pull/1225
* dev: Serve only the app's assets and WebSocket to its own pages by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/1117
* gallery: Open the context-menu test on the System demo by @huacnlee in https://github.com/longbridge/nsh/pull/1223
* website: Raise the home hero's arches to stand as an arch by @huacnlee in https://github.com/longbridge/nsh/pull/1232
* web: Regenerate the browser bootstrap for macos props and setTheme by @huacnlee in https://github.com/longbridge/nsh/pull/1227
* macos: Press views in tests where their transforms draw them by @ihavecoke in https://github.com/longbridge/nsh/pull/1145
* js: Bind a signal's set and update to it, so they can be passed on their own by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/1119
* core: Show a dialog that mounts in an open window over that window by @t4t5 in https://github.com/longbridge/nsh/pull/1217
* cli: Give the nsh binary an 8 MB main-thread stack on Windows by @huacnlee in https://github.com/longbridge/nsh/pull/1239
* examples: Share the look of surfaces, rows and form controls by @madcodelife in https://github.com/longbridge/nsh/pull/1146
* examples: Share the progress, disclosure, nav, plot and workspace looks by @madcodelife in https://github.com/longbridge/nsh/pull/1147
* mobile: Let an embedding host register fonts by @akongG in https://github.com/longbridge/nsh/pull/1244
* macos: Keep the display link at the rate of the window's screen by @LeeDashan in https://github.com/longbridge/nsh/pull/1245
* cli: Resolve GPUI hosts in their Cargo target directory by @huacnlee in https://github.com/longbridge/nsh/pull/1240
* core: Let web views serve URL schemes from the app by @yangkx1024 in https://github.com/longbridge/nsh/pull/1233
* ios: Animate box shadow geometry with the corner radius by @Teages in https://github.com/longbridge/nsh/pull/1246
* core: Keep an inverted list's rows in place when history and heights land together by @huacnlee in https://github.com/longbridge/nsh/pull/1243
* engine: Give back the op buffer room a large commit took by @sunfuze in https://github.com/longbridge/nsh/pull/1259
* core: Give back the reactive graph's slots when a large tree is gone by @sunfuze in https://github.com/longbridge/nsh/pull/1260
* macos: Share paragraph styles between texts with the same settings by @0xbentang in https://github.com/longbridge/nsh/pull/1252
* core: Share a style's declarations between the copies a view keeps by @0xbentang in https://github.com/longbridge/nsh/pull/1250
* macos: Keep the views nsh places out of Auto Layout by @0xbentang in https://github.com/longbridge/nsh/pull/1251
* macos: Keep a view's rare state out of every view's allocation by @0xbentang in https://github.com/longbridge/nsh/pull/1249
* base: Report when a press goes down and up on a Toggle, as a Button does by @Vehicoule in https://github.com/longbridge/nsh/pull/1248
* ios: Apply a view's first style without a transition by @Teages in https://github.com/longbridge/nsh/pull/1247
* core: Keep the records of bound nodes out of their map's slots by @0xbentang in https://github.com/longbridge/nsh/pull/1253
* macos: Pass test hosts' presses under a hidden title bar to the app by @dXnSix in https://github.com/longbridge/nsh/pull/1254
* i18n: Add translations with rust-i18n and move language data out of the runtime by @huacnlee in https://github.com/longbridge/nsh/pull/1234
* docs: Show the nightly install in the README by @huacnlee in https://github.com/longbridge/nsh/pull/1230
* web: Link nsh/base's core, crypto and url only for apps that use them by @huacnlee in https://github.com/longbridge/nsh/pull/1220
* cli: Say why a web package keeps the whole engine by @williamjakfar in https://github.com/longbridge/nsh/pull/1181
* updater: Extract update archives with zip 9, fixing its advisories by @Vehicoule in https://github.com/longbridge/nsh/pull/1164
* web: Don't throw on key events without `key` by @huacnlee in https://github.com/longbridge/nsh/pull/1264
* core: Let a list at its start show the rows inserted above its first by @BeADre in https://github.com/longbridge/nsh/pull/1132
* core: Check web view base URLs and parser-ambiguous URLs against grants by @ClarkThan in https://github.com/longbridge/nsh/pull/979
* core: Preserve native bounce in bottom-aligned lists by @huacnlee in https://github.com/longbridge/nsh/pull/1265
* testing: Expose layout boxes and platform text engines to headless tests by @LeeDashan in https://github.com/longbridge/nsh/pull/1242
* updater: Restart into the installed app after replacing it by @huacnlee in https://github.com/longbridge/nsh/pull/1269
* core: Place a cover view again when its size changes by @LeeDashan in https://github.com/longbridge/nsh/pull/1236
* examples: Finish moving the demos' looks onto the shared layer by @madcodelife in https://github.com/longbridge/nsh/pull/1267
* cli: Pass an app's declared environment to the iOS simulator by @madcodelife in https://github.com/longbridge/nsh/pull/1271
* docs: Note that in-process macOS screenshots draw rounded borders unevenly by @madcodelife in https://github.com/longbridge/nsh/pull/1272
* gallery: Stop setting the removed `edited` window attribute by @madcodelife in https://github.com/longbridge/nsh/pull/1268
* cli: Add `nsh update` to update NSH to the latest release or nightly by @huacnlee in https://github.com/longbridge/nsh/pull/1273
* website: Point desktop-only apps to GPUI Kit by @huacnlee in https://github.com/longbridge/nsh/pull/1280
* data_table: Keep huge web tables smooth during wheel inertia by @huacnlee in https://github.com/longbridge/nsh/pull/1270
* web: Drop the browser outline on alert dialog buttons by @huacnlee in https://github.com/longbridge/nsh/pull/1284
* cli: Pass an app's declared environment to the Android development host by @madcodelife in https://github.com/longbridge/nsh/pull/1283
* runtime: Allow explicit WebSocket origin credentials by @lurenjia534 in https://github.com/longbridge/nsh/pull/1134
* testing: Pull a scroll view's offset back when its content shrinks by @BeADre in https://github.com/longbridge/nsh/pull/1129
* rich_editor: Show local and data: images, at their natural size when unsized by @sureshkrishnan-ai in https://github.com/longbridge/nsh/pull/1121
* cli: Give apps under nsh test the URL schemes their nsh.json claims by @wuyunyoujiduo-jpg in https://github.com/longbridge/nsh/pull/1224
* web: Move the browser runtime's fetch out of its test module by @dXnSix in https://github.com/longbridge/nsh/pull/1157
* tui: Wait up to a second for the terminal's startup replies by @nohzafk in https://github.com/longbridge/nsh/pull/1288
* tui: Let an embedding program pass the host's options and stop it by @nohzafk in https://github.com/longbridge/nsh/pull/1287
* engine: Show the JS heap and the nodes alive on the performance monitor by @sunfuze in https://github.com/longbridge/nsh/pull/1262
* core: Work out pointer-events regions once a commit by @huacnlee in https://github.com/longbridge/nsh/pull/1292
* core: Keep the tables of nodes by id in pages freed when they empty by @sunfuze in https://github.com/longbridge/nsh/pull/1261
* editor: Show syntax colors as soon as an editor opens by @huacnlee in https://github.com/longbridge/nsh/pull/1293
* web: Regenerate the browser bootstrap after moving fetch out of tests by @huacnlee in https://github.com/longbridge/nsh/pull/1295
* testing: Report the view the focus reaches before the one it leaves by @huacnlee in https://github.com/longbridge/nsh/pull/1296
* fs: Follow absolute symlinks whose target stays in the granted root by @huacnlee in https://github.com/longbridge/nsh/pull/1297
* core: Accept absolute commands and file roots in the browser by @sunfuze in https://github.com/longbridge/nsh/pull/1298
* cli: Keep the generated crates' names short however many crates an app lists by @dXnSix in https://github.com/longbridge/nsh/pull/1263
* bundler: Read import() of a backtick string as the module it names by @williamjakfar in https://github.com/longbridge/nsh/pull/1178
* desktop: Keep a focused view taller than its scroll view where it is by @yangkx1024 in https://github.com/longbridge/nsh/pull/1241
* android: End the composition when an input submits by @huacnlee in https://github.com/longbridge/nsh/pull/1300
* cli: Install other platforms' runtimes as components when needed by @huacnlee in https://github.com/longbridge/nsh/pull/1302
* component: Forward the tooltip host prop to the component's view by @huacnlee in https://github.com/longbridge/nsh/pull/1299
* linux: Make the performance monitor concise and collapse it on a click by @huacnlee in https://github.com/longbridge/nsh/pull/1306
* xtask: Import the stripping helper only in the test that uses it by @huacnlee in https://github.com/longbridge/nsh/pull/1309
* link: Show the pointing hand by default by @huacnlee in https://github.com/longbridge/nsh/pull/1311
* core: Bind a list's rows further ahead while it scrolls fast by @huacnlee in https://github.com/longbridge/nsh/pull/1310
* core: Make scroll views' native views only near their viewport by @huacnlee in https://github.com/longbridge/nsh/pull/1304
* windows: Read window data only from the app's own windows by @dXnSix in https://github.com/longbridge/nsh/pull/1301
* text_view: Avoid quadratic code-span searches in TeX preprocessing by @lurenjia534 in https://github.com/longbridge/nsh/pull/1308
* js: Unwrap every accessor a NavStack route renders down to its screen by @huacnlee in https://github.com/longbridge/nsh/pull/1314
* core: Picture a backdrop again only when what draws under it changed by @huacnlee in https://github.com/longbridge/nsh/pull/1315
* cli: Build an optimized development host with nsh run --release by @huacnlee in https://github.com/longbridge/nsh/pull/1317
* macos: Picture a backdrop of only what draws under it, and only when that changed by @huacnlee in https://github.com/longbridge/nsh/pull/1316
* engine: Measure the performance monitor's CPU over a second by @huacnlee in https://github.com/longbridge/nsh/pull/1313
* core: Render far rows of an Each in a scroll view as they come near by @huacnlee in https://github.com/longbridge/nsh/pull/1312
* cli: Log the performance monitor's readings only with -v by @huacnlee in https://github.com/longbridge/nsh/pull/1322
* core: Let an app set how many encoded image bytes stay in memory by @huacnlee in https://github.com/longbridge/nsh/pull/1330
* macos: Render shader frames into IOSurfaces by @huacnlee in https://github.com/longbridge/nsh/pull/1332
* tui: Keep a request to exit made before the host's run starts by @nohzafk in https://github.com/longbridge/nsh/pull/1307
* text_view: Share terminal table column width calculations by @lurenjia534 in https://github.com/longbridge/nsh/pull/1318
* list: Use binary search for neighboring sticky indices by @lurenjia534 in https://github.com/longbridge/nsh/pull/1324
* testing: Press a view across a scroll view's fold on its visible half by @Teages in https://github.com/longbridge/nsh/pull/1327
* tui: Draw the text inside a Link as an OSC 8 hyperlink by @nohzafk in https://github.com/longbridge/nsh/pull/1289
* macos: Keep text fields visible while scrolling by @hsingjui in https://github.com/longbridge/nsh/pull/1326
* macos: Skip redundant sibling scans when appending children by @huacnlee in https://github.com/longbridge/nsh/pull/1339
* dev: Replace an entry's mount made after an await on hot reload by @huacnlee in https://github.com/longbridge/nsh/pull/1329
* linux: Show the CPU beside the frame rate on the collapsed performance panel by @huacnlee in https://github.com/longbridge/nsh/pull/1331
* ios: Render shader frames into IOSurfaces by @huacnlee in https://github.com/longbridge/nsh/pull/1333
* ios: Keep text visible when its first line exceeds the height by @huacnlee in https://github.com/longbridge/nsh/pull/1334
* cli: Give bindgen the NDK's clang headers when packaging Android apps by @huacnlee in https://github.com/longbridge/nsh/pull/1335
* macos: Draw dashed and dotted borders on individual sides by @huacnlee in https://github.com/longbridge/nsh/pull/1337
* core: Play haptic feedback on iOS and Android with nsh/mobile's Haptics by @ahonn in https://github.com/longbridge/nsh/pull/1328
* tui: Show a grown multiline input's first rows again by @nohzafk in https://github.com/longbridge/nsh/pull/1340
* core: Size a leaf's kept measurements to the answers it gets by @0xbentang in https://github.com/longbridge/nsh/pull/1342
* core: Keep one copy of a text block's layout style by @0xbentang in https://github.com/longbridge/nsh/pull/1341
* cli: Build a Windows installer with Inno Setup by @huacnlee in https://github.com/longbridge/nsh/pull/1348
* emoji_picker: Add EmojiPicker, a search field over every emoji by @Cooooooker in https://github.com/longbridge/nsh/pull/1323
* ci: Release builds for Intel Macs and arm64 Linux and Windows by @hmziqrs in https://github.com/longbridge/nsh/pull/1321
* desktop: Keep window attributes set while a window first opens by @huacnlee in https://github.com/longbridge/nsh/pull/1351
* core: Give an input in a display none subtree no content box by @sunfuze in https://github.com/longbridge/nsh/pull/1350
* runtime: Give apps an address with UrlScheme.url and setUrl by @chaxus in https://github.com/longbridge/nsh/pull/982
* website: Use English heading ids on translated pages by @huacnlee in https://github.com/longbridge/nsh/pull/1353
* website: Build the releases' players with the toolchain's own Cargo by @madcodelife in https://github.com/longbridge/nsh/pull/1352
* engine: Open the shader device on OpenGL first on Linux by @huacnlee in https://github.com/longbridge/nsh/pull/1357
* website: Use the English anchor for the gallery's publish link by @huacnlee in https://github.com/longbridge/nsh/pull/1360
* website: Link the Chinese gallery page to the English section id by @madcodelife in https://github.com/longbridge/nsh/pull/1361
* windows: Test tinted images at fractional display scales by @huacnlee in https://github.com/longbridge/nsh/pull/1362
* android: Run on Android 8 (API 26) by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/1363
* linux: Keep a menu of one item from being taller than the item by @yangkx1024 in https://github.com/longbridge/nsh/pull/1359
* desktop: Fire a window's key bindings only in that window by @huacnlee in https://github.com/longbridge/nsh/pull/1354
* website: Add a Directory page for community libraries and tools by @huacnlee in https://github.com/longbridge/nsh/pull/1369
* desktop: Fire a window's key bindings in that window on GPUI and in tests by @madcodelife in https://github.com/longbridge/nsh/pull/1391
* list: Append rows without rebuilding every offset by @huacnlee in https://github.com/longbridge/nsh/pull/1371
* image: Skip the environment proxy for loopback asset URLs by @huacnlee in https://github.com/longbridge/nsh/pull/1396
* linux: Let assistive technologies activate plain buttons by @kim0 in https://github.com/longbridge/nsh/pull/1255
* text: Show link URL on hover by @huacnlee in https://github.com/longbridge/nsh/pull/1370
* core: Include file URLs in desktop drag entry events by @ihavecoke in https://github.com/longbridge/nsh/pull/938
* ios: Find an input's event target when the host wraps its delegate by @yuanzhuang-svg in https://github.com/longbridge/nsh/pull/1394
* windows: Keep resizing windows free of black edges and queued frames by @sunli829 in https://github.com/longbridge/nsh/pull/1402
* core: Let platforms built outside NSH run apps by @huacnlee in https://github.com/longbridge/nsh/pull/1393
* compat: Fix the EmojiPicker and Link hover cases by @huacnlee in https://github.com/longbridge/nsh/pull/1405
* web: Make published apps sites with document markup, links and heads by @Tzyito in https://github.com/longbridge/nsh/pull/1130
* text_view: Test emphasis after a bare URL that CJK text ends by @madcodelife in https://github.com/longbridge/nsh/pull/1398
* menu: Let Down, Up, Home and End reach a menu opened by a press by @madcodelife in https://github.com/longbridge/nsh/pull/1399
* core: Place anchored overlays on whole device pixels by @madcodelife in https://github.com/longbridge/nsh/pull/1400
* cli: Let nsh.json say whether a web app's pages are browser history entries by @dXnSix in https://github.com/longbridge/nsh/pull/1158
* macos: Keep text visible inside transformed ancestors by @0xbentang in https://github.com/longbridge/nsh/pull/1135
* macos: Let focus() take the keyboard from the app in front by @duo in https://github.com/longbridge/nsh/pull/1397
* macos: Leave keys to an input method while it composes by @caoxing9 in https://github.com/longbridge/nsh/pull/1387
* bundler: Build an entry below its app with the app's nsh.json by @sunfuze in https://github.com/longbridge/nsh/pull/1409
* core: Reuse retained rows when their order is unchanged by @lurenjia534 in https://github.com/longbridge/nsh/pull/1407
* core: Hide the app address's cross-crate plumbing from the docs by @chaxus in https://github.com/longbridge/nsh/pull/1406
* menu: Cancel the trigger's press when a long press opens a ContextMenu by @huacnlee in https://github.com/longbridge/nsh/pull/1415
* mobile: Add native scroll handles for both axes by @ZZombiee in https://github.com/longbridge/nsh/pull/1401
* data_table: Let cells' own buttons take presses and fix columns at the right by @RebelBIrd in https://github.com/longbridge/nsh/pull/1395
* macos: Copy selected text while an input keeps focus by @huacnlee in https://github.com/longbridge/nsh/pull/1410
* core: Let a module function that takes ModuleCx spawn tasks by @RebelBIrd in https://github.com/longbridge/nsh/pull/1372
* notifications: Answer the permission the system holds when asking errs by @RebelBIrd in https://github.com/longbridge/nsh/pull/1373
* data_table: Reserve native scroll room when anchoring near content edges by @richerfu in https://github.com/longbridge/nsh/pull/1392
* testing: Leave out what is in a view never created from layout nodes by @LeeDashan in https://github.com/longbridge/nsh/pull/1281
* bundler: Import videos, and any file with ?url, as their asset URLs by @RebelBIrd in https://github.com/longbridge/nsh/pull/1380
* cli: Build the terminal host of an app's crates in a workspace of its own by @RebelBIrd in https://github.com/longbridge/nsh/pull/1374
* testing: Choose application menu items by their shortcuts in headless runs by @RebelBIrd in https://github.com/longbridge/nsh/pull/1375
* windows: Stop entering content from flashing as its window opens by @duo in https://github.com/longbridge/nsh/pull/1364
* image: Read the size of a blob URL's image from its bytes by @RebelBIrd in https://github.com/longbridge/nsh/pull/1376
* testing: Write a test file's entry whole so concurrent runs never hang by @sunfuze in https://github.com/longbridge/nsh/pull/1367
* rich_editor: Select text by touch with handles and the system edit menu by @Cooooooker in https://github.com/longbridge/nsh/pull/1368
* website: Fix nav wrapping and move Releases into the version menu by @huacnlee in https://github.com/longbridge/nsh/pull/1423
* ios: Keep embedded error callbacks alive until they are removed by @huacnlee in https://github.com/longbridge/nsh/pull/1429
* ios: Refresh lists and scroll views under large titles by @huacnlee in https://github.com/longbridge/nsh/pull/1426
* core: Cancel a press after other nodes were freed without panicking by @huacnlee in https://github.com/longbridge/nsh/pull/1430
* macos: Keep text visible when its first line exceeds the height by @madcodelife in https://github.com/longbridge/nsh/pull/1421
* select: Let UI tests find the search field and combobox input by id by @madcodelife in https://github.com/longbridge/nsh/pull/1433
* macos: Keep a zoomed window's normal frame from before the zoom by @madcodelife in https://github.com/longbridge/nsh/pull/1434
* core: Report the MIME type of each file a drag offers or drops by @madcodelife in https://github.com/longbridge/nsh/pull/1441
* macos: Render a shader view that only changed without the display link by @madcodelife in https://github.com/longbridge/nsh/pull/1442
* core: Keep a text's layout when only its colors change by @madcodelife in https://github.com/longbridge/nsh/pull/1436
* cli: Link only the nsh/base components a Rust app uses on the web by @madcodelife in https://github.com/longbridge/nsh/pull/1445
* core: Tell apps when the platform cannot show a web view by @madcodelife in https://github.com/longbridge/nsh/pull/1439
* macos: Let nsh.json start an app as a menu bar app by @duo in https://github.com/longbridge/nsh/pull/1390
* menu: Close a hover-opened submenu when the keys move to another item by @madcodelife in https://github.com/longbridge/nsh/pull/1444
* select: Open the list below the trigger on Windows and Linux by @madcodelife in https://github.com/longbridge/nsh/pull/1435
* core: Let apps insert a paste they allowed as the user's undoable edit by @madcodelife in https://github.com/longbridge/nsh/pull/1443
* core: Open dialogs placed in a scroll view's content by @Cooooooker in https://github.com/longbridge/nsh/pull/1389
* web: Report why the browser facade failed to load by @Laotree in https://github.com/longbridge/nsh/pull/1428
* ci: Schedule nightly releases at 02:30 Singapore time by @huacnlee in https://github.com/longbridge/nsh/pull/1452
* core: Let the animation graph be cloned by @huacnlee in https://github.com/longbridge/nsh/pull/1456
* core: Declare rust-version for published crates by @huacnlee in https://github.com/longbridge/nsh/pull/1457
* text: Hide spans in place, leaving them out of copies and screen readers by @madcodelife in https://github.com/longbridge/nsh/pull/1438
* desktop: Report the tray icon's bounds with its click by @duo in https://github.com/longbridge/nsh/pull/1355
* text-view: Keep CJK path characters in bare URLs by @huacnlee in https://github.com/longbridge/nsh/pull/1458
* js: Read the app's version, build and development flag with appInfo() by @RebelBIrd in https://github.com/longbridge/nsh/pull/1381
* android: Give native components their view's context by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/1462
* menu: Walk every row of a virtualized menu list with the keys by @madcodelife in https://github.com/longbridge/nsh/pull/1437
* docs: Fix the activation policy's Rust example and list its host flag by @duo in https://github.com/longbridge/nsh/pull/1455
* cli: Carry nsh.json window into Linux and Windows packages by @duo in https://github.com/longbridge/nsh/pull/1419
* desktop: Let nsh.json start the main window hidden, as tray apps do by @duo in https://github.com/longbridge/nsh/pull/1420
* cli: Let Android packages allow plain HTTP by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/1460
* core: Size shrink-to-fit rows with flex items in slot holders by @burdygin in https://github.com/longbridge/nsh/pull/1461
* runtime: Let an address the app sets be a history entry of its own by @dXnSix in https://github.com/longbridge/nsh/pull/1448
* macos: Restore mouse tracking in app windows by @0xbentang in https://github.com/longbridge/nsh/pull/1454
* web: Draw svg() currentColor in its text color by @huacnlee in https://github.com/longbridge/nsh/pull/1473
* list: Don't follow the end when a page is prepended above the reader by @huacnlee in https://github.com/longbridge/nsh/pull/1477
* core: Run an app's Rust functions on the platform's main thread by @RebelBIrd in https://github.com/longbridge/nsh/pull/1382
* testing: Let open stand in for the native modules the app imports by @RebelBIrd in https://github.com/longbridge/nsh/pull/1379
* macos: Hide an input's editor while its view has no area by @huacnlee in https://github.com/longbridge/nsh/pull/1478
* text: Skip hidden runs when drawing instead of drawing them transparent by @madcodelife in https://github.com/longbridge/nsh/pull/1463
* cli: Close the web test browser when a test outlives its timeout by @LVtomatoJ in https://github.com/longbridge/nsh/pull/1464
* macos: Show a dialog in a window's first render over that window by @huacnlee in https://github.com/longbridge/nsh/pull/1480
* windows: Edit text inputs with Windows' own text box by @huacnlee in https://github.com/longbridge/nsh/pull/1475
* ci: Release and deploy the website on the organization's self-hosted runners by @huacnlee in https://github.com/longbridge/nsh/pull/1501
* cli: Show the build date in nightly versions by @huacnlee in https://github.com/longbridge/nsh/pull/1514
* video: Play a video again after a seek back from its end on Apple platforms by @RebelBIrd in https://github.com/longbridge/nsh/pull/1377
* windows: Load the WebView2 loader DLL on demand outside MSVC by @huacnlee in https://github.com/longbridge/nsh/pull/1490
* gallery: Restyle the date pickers and calendar after shadcn/ui by @madcodelife in https://github.com/longbridge/nsh/pull/1492
* ci: Build the Android and web runtimes once per release, on x86_64 Linux by @huacnlee in https://github.com/longbridge/nsh/pull/1493
* svg: Draw SVG images on canvases, with text in native SVG rasters by @YougLin-dev in https://github.com/longbridge/nsh/pull/1291
* audio: Load a sound from its bytes by @madcodelife in https://github.com/longbridge/nsh/pull/1471
* cli: Let macOS packages pick their processors with --arch by @madcodelife in https://github.com/longbridge/nsh/pull/1481
* text_view: Keep reference brackets text while their closer streams in by @madcodelife in https://github.com/longbridge/nsh/pull/1486
* core: Tell apps whether the platform draws shaders by @madcodelife in https://github.com/longbridge/nsh/pull/1476
* core: Flip popups held to their room at their natural size by @madcodelife in https://github.com/longbridge/nsh/pull/1497
* core: Show `focus-visible:` styles only for focus the keyboard moved by @madcodelife in https://github.com/longbridge/nsh/pull/1519
* gallery: Let the booking calendar flip above its field on its own by @madcodelife in https://github.com/longbridge/nsh/pull/1520
* menu: Add Page Up and Page Down and start the keys at the hovered item by @madcodelife in https://github.com/longbridge/nsh/pull/1518
* bundler: Build the window test's bundles with the default options by @huacnlee in https://github.com/longbridge/nsh/pull/1523
* docs: Add API design rules for names, options and generated types by @huacnlee in https://github.com/longbridge/nsh/pull/1530
* core: Make layout springs gpui-base's Spring, timed by their physics by @Vehicoule in https://github.com/longbridge/nsh/pull/1305
* core: Let tabs, radios and toolbar items inside plain views join their group by @madcodelife in https://github.com/longbridge/nsh/pull/1521
* core: Reveal a view in the scroll views around it from its ref by @RebelBIrd in https://github.com/longbridge/nsh/pull/1378
* docs: Make the Design Guides lead to complete, refined and animated app UI by @huacnlee in https://github.com/longbridge/nsh/pull/1529
* gpui: Deliver native components' commands without waiting for a frame by @obito-t in https://github.com/longbridge/nsh/pull/1467
* bundler: Prune statically read native namespace imports by @huacnlee in https://github.com/longbridge/nsh/pull/1522
* ci: Run the checks on main daily and the release on self-hosted runners by @huacnlee in https://github.com/longbridge/nsh/pull/1535
* gallery: Draw focus rings only for focus the keyboard moved by @madcodelife in https://github.com/longbridge/nsh/pull/1528
* linux: Fix build without the `dev` feature by @huacnlee in https://github.com/longbridge/nsh/pull/1541
* web: Load the runtime again: rename the second `MODIFIER_KEYS` by @madcodelife in https://github.com/longbridge/nsh/pull/1547
* web: Keep the keyboard open when a touch focuses an editor's text by @mrtolkien in https://github.com/longbridge/nsh/pull/1482
* android: Keep the system scrim off the navigation bar under a Dialog by @yuanzhuang-svg in https://github.com/longbridge/nsh/pull/1484
* desktop: Give Rust apps the tray and global shortcuts by @huacnlee in https://github.com/longbridge/nsh/pull/1562
* core: Add data_dir() for the app's own data directory by @huacnlee in https://github.com/longbridge/nsh/pull/1550
* text: Show the platform's edit menu on a right click over selectable text by @huacnlee in https://github.com/longbridge/nsh/pull/1517
* ci: Build the release with thin LTO to cut its builds by hours by @huacnlee in https://github.com/longbridge/nsh/pull/1494
* macos: Let a web view show the local file page its app loads by @hexuria in https://github.com/longbridge/nsh/pull/1496
* web: Put images of every format on the clipboard as PNG by @Cooooooker in https://github.com/longbridge/nsh/pull/1540
* windows: Show the pointer over links in text by @huacnlee in https://github.com/longbridge/nsh/pull/1542
* ci: Check main every three hours when it moved, and trim pull requests by @huacnlee in https://github.com/longbridge/nsh/pull/1538
* packages: Skip the component check for releases without nsh component by @huacnlee in https://github.com/longbridge/nsh/pull/1572
* android: Offer installed text actions in the selection menu by @huacnlee in https://github.com/longbridge/nsh/pull/1516
* android: End the presses around a responder, so editor handles drag by @Cooooooker in https://github.com/longbridge/nsh/pull/1577
* web: Load again after two modifier key sets got the same name by @huacnlee in https://github.com/longbridge/nsh/pull/1549
* net: Verify TLS with the system trust store by @huacnlee in https://github.com/longbridge/nsh/pull/1543
* desktop: Show a title beside the tray icon in macOS's menu bar by @Joy-jinyu in https://github.com/longbridge/nsh/pull/1479
* rich_editor: Read and write documents as HTML from JS by @Tzyito in https://github.com/longbridge/nsh/pull/1578
* core: Await WebView script results with evaluate by @kosumic in https://github.com/longbridge/nsh/pull/1432
* web: Keep a copied GIF's frames beside its PNG on the clipboard by @Cooooooker in https://github.com/longbridge/nsh/pull/1564
* ios: Lay out TabBar items again when its height changes by @huacnlee in https://github.com/longbridge/nsh/pull/1574
* ios: Shape backdrop effects with a mask view by @huacnlee in https://github.com/longbridge/nsh/pull/1575
* desktop: Give Rust trays a title and name its side a placement by @huacnlee in https://github.com/longbridge/nsh/pull/1582
* desktop: Add single-instance mode by @huacnlee in https://github.com/longbridge/nsh/pull/1563
* cli: Declare the app's languages in iOS and macOS packages by @huacnlee in https://github.com/longbridge/nsh/pull/1585
* bundler: Bring two namespace-import tests in line with the pruning by @madcodelife in https://github.com/longbridge/nsh/pull/1586
* gallery: Keep the title bar uncovered by the Input demo's sheet by @huacnlee in https://github.com/longbridge/nsh/pull/1554
* core: Keep a retargeted spring moving from its last frame by @huacnlee in https://github.com/longbridge/nsh/pull/1553
* image: Add an in-memory RGBA pixel buffer source by @huacnlee in https://github.com/longbridge/nsh/pull/1561
* menu: Open ContextMenu on a long press only for touch and pen by @huacnlee in https://github.com/longbridge/nsh/pull/1555
* core: Hold a lone blur until the next frame before focus-within leaves by @huacnlee in https://github.com/longbridge/nsh/pull/1557
* gpui: Support programmatic focus on focusable views by @huacnlee in https://github.com/longbridge/nsh/pull/1545
* macos: Set the cursor only when it changes as the mouse moves by @hexuria in https://github.com/longbridge/nsh/pull/1512
* macos: Stop a trackpad flick's momentum when the mouse is pressed by @hexuria in https://github.com/longbridge/nsh/pull/1511
* macos: Detach a deleted subtree once, from its root by @hexuria in https://github.com/longbridge/nsh/pull/1513
* macos: Pass scrolls an embedded app cannot use on to its host by @hexuria in https://github.com/longbridge/nsh/pull/1515
* gallery: Add a switch that lays the demos out right to left by @huacnlee in https://github.com/longbridge/nsh/pull/1587
* macos: Commit an input method's composition when the input blurs by @ttizze in https://github.com/longbridge/nsh/pull/1507
* macos: Read a scroll view's content in the order it is on screen by @ttizze in https://github.com/longbridge/nsh/pull/1508
* macos: Expose scroll views as scroll areas with their scroll position by @ttizze in https://github.com/longbridge/nsh/pull/1509
* render: Draw borders under child views, keep rotated GPUI images upright, document repeating gradients by @huacnlee in https://github.com/longbridge/nsh/pull/1546
* core: Add onWheel view event by @huacnlee in https://github.com/longbridge/nsh/pull/1567
* core: Show `focus-visible:` on a projected cursor while the focus shows by @madcodelife in https://github.com/longbridge/nsh/pull/1532
* gallery: Make fills, hovers and placeholders read in light and dark by @madcodelife in https://github.com/longbridge/nsh/pull/1537
* package: Accept per-platform icons in nsh.json by @huacnlee in https://github.com/longbridge/nsh/pull/1548
* mobile: Reuse application-owned Tokio runtimes by @kosumic in https://github.com/longbridge/nsh/pull/1404
* core: Check media sources as the loaders load them against grants by @ClarkThan in https://github.com/longbridge/nsh/pull/1417
* runtime: Name a URL change's reason, and keep a back to the page's own entry a back by @dXnSix in https://github.com/longbridge/nsh/pull/1447
* cli: Restart nsh run for a Rust change saved right after a TSX one by @BeADre in https://github.com/longbridge/nsh/pull/1487
* macos: Show file panels in front while the app's window is out of sight by @BeADre in https://github.com/longbridge/nsh/pull/1488
* macos: Keep input method candidates at a point-wide input by @boyan01 in https://github.com/longbridge/nsh/pull/1502
* list: Keep screen readers off the cells a list left behind by @ttizze in https://github.com/longbridge/nsh/pull/1503
* selection: Report where a selectable view's selection is by @ttizze in https://github.com/longbridge/nsh/pull/1504
* gpui: Let Windows move the window from its caption's press by @zcg in https://github.com/longbridge/nsh/pull/1525
* cli: Give the mixed template's text the theme's foreground by @zcg in https://github.com/longbridge/nsh/pull/1526
* engine: Keep merging onMouseMove events during a drag by @WillisXue in https://github.com/longbridge/nsh/pull/1533
* core: Close overlays when a secondary click lands outside them by @madcodelife in https://github.com/longbridge/nsh/pull/1544
* popover: Let onOpen move the keyboard focus into the popup by @madcodelife in https://github.com/longbridge/nsh/pull/1551
* core: Keep animation values finite whatever their parameters by @ClarkThan in https://github.com/longbridge/nsh/pull/1556
* testing: Keep the window size in the environment a test changes by @madcodelife in https://github.com/longbridge/nsh/pull/1566
* base: Reject malformed themes in setTheme by @Lemenxzy in https://github.com/longbridge/nsh/pull/1571
* windows: Keep tinted images in recorded screens and dialogs as drawn by @Blackman99 in https://github.com/longbridge/nsh/pull/1581
* menu: Hold menus to the room beside their trigger and scroll past it by @madcodelife in https://github.com/longbridge/nsh/pull/1534
* core: Run an app's cleanup as it quits with on_app_quit by @huacnlee in https://github.com/longbridge/nsh/pull/1558
* gallery: Restyle the color picker and even out the demos' details by @madcodelife in https://github.com/longbridge/nsh/pull/1584
* windows: Fix a frameless window's maximized state by @kindred-7 in https://github.com/longbridge/nsh/pull/1422
* macos: Localize the text selection context menu by @huacnlee in https://github.com/longbridge/nsh/pull/1579
* tailwind: Add pointer-fine:, pointer-coarse: and terminal: variants by @huacnlee in https://github.com/longbridge/nsh/pull/1539
* windows: Test that a paint asked for while drawing comes by @itisvincent in https://github.com/longbridge/nsh/pull/1595
* android: Number the event codes as the core orders its kinds by @madcodelife in https://github.com/longbridge/nsh/pull/1598
* testing: Hit header items above a screen's content in headless runs by @huacnlee in https://github.com/longbridge/nsh/pull/1602
* engine: Open shaders on the next backend when one refuses its device by @huacnlee in https://github.com/longbridge/nsh/pull/1601
* engine: Give the drag test's touch point its pointer type by @madcodelife in https://github.com/longbridge/nsh/pull/1607
* linux: Bring windows to the front after a tray click on Wayland by @madcodelife in https://github.com/longbridge/nsh/pull/1615
* menu: Reopen `ContextMenu` where its trigger is right-clicked again by @madcodelife in https://github.com/longbridge/nsh/pull/1610
* core: Add `invalid` to the accessibility state by @madcodelife in https://github.com/longbridge/nsh/pull/1612
* popup: Give the keyboard focus back as a popup starts to go by @madcodelife in https://github.com/longbridge/nsh/pull/1618
* core: Tell where a list's viewport starts among its rows by @madcodelife in https://github.com/longbridge/nsh/pull/1617
* core: Keep a text's layout when only its hidden runs change by @madcodelife in https://github.com/longbridge/nsh/pull/1619
* webview: Let pages use the microphone and camera by @huacnlee in https://github.com/longbridge/nsh/pull/1559
* core: Let Tab reach every row of a list and scroll the focus into view by @madcodelife in https://github.com/longbridge/nsh/pull/1620
* popup: Let Escape close a controlled popup the app opened itself by @madcodelife in https://github.com/longbridge/nsh/pull/1622
* testing: Place a page's header as the desktops do in headless runs by @madcodelife in https://github.com/longbridge/nsh/pull/1623
* dialog: Let screen readers announce a dialog and its title by @madcodelife in https://github.com/longbridge/nsh/pull/1624
* cli: Choose the iOS simulator nsh test runs apps in by @Joy-jinyu in https://github.com/longbridge/nsh/pull/1604
* macos: Keep a window setViewport grows in tests on its display by @Joy-jinyu in https://github.com/longbridge/nsh/pull/1608
* testing: Cut the texts a failure shows to a few KB by @taokeqin in https://github.com/longbridge/nsh/pull/1453
* windows: Handle WM_PAINT without DefWindowProcW by @huacnlee in https://github.com/longbridge/nsh/pull/1599
* core: Make views out of sight over frames by @huacnlee in https://github.com/longbridge/nsh/pull/1600
* core: Check menu icons and image redirects against capabilities by @huacnlee in https://github.com/longbridge/nsh/pull/1603
* bundler: Root subdirectory assets at the application directory by @huacnlee in https://github.com/longbridge/nsh/pull/1592
* core: Put a popover's content right after its trigger in the Tab order by @madcodelife in https://github.com/longbridge/nsh/pull/1616
* desktop: Let apps declare the file types they open by @huacnlee in https://github.com/longbridge/nsh/pull/1565
* text_view: Hold streamed Markdown lines whose meaning is still open by @madcodelife in https://github.com/longbridge/nsh/pull/1632
* ci: Release and deploy the website on GitHub's runners by @madcodelife in https://github.com/longbridge/nsh/pull/1645
* core: Size columns with flex items by their content when shrink-to-fit by @huacnlee in https://github.com/longbridge/nsh/pull/1625
* runtime: Let the global WebSocket buffer a burst of sends by @daoleno in https://github.com/longbridge/nsh/pull/1641
* theme: Reject identifiers attached to color functions by @Lemenxzy in https://github.com/longbridge/nsh/pull/1637
* image: Read platform <Image> files through their fs.read root by @huacnlee in https://github.com/longbridge/nsh/pull/1605
* macos: Remeasure the fallback title bar area after layout by @huacnlee in https://github.com/longbridge/nsh/pull/1573
* examples: Add a short post editor with stock, member and topic tags by @Tzyito in https://github.com/longbridge/nsh/pull/1629
* release: Prepare version 0.5.0 by @huacnlee in https://github.com/longbridge/nsh/pull/1631
* core: Let screen readers read an element's description by @madcodelife in https://github.com/longbridge/nsh/pull/1650
* cli: Let apps raise the minimum system version of their packages by @BeADre in https://github.com/longbridge/nsh/pull/1654
* core: Show a tooltip for the focus only when the keyboard moved it by @BeADre in https://github.com/longbridge/nsh/pull/1653
* docs: Bring the packaging page up to date and line up Chinese pages by @BeADre in https://github.com/longbridge/nsh/pull/1656
* release: Bring the post editor example to 0.5.0 in the lockfile by @madcodelife in https://github.com/longbridge/nsh/pull/1662
* core: Drop a spawned task's cleanup once the task ends by @madcodelife in https://github.com/longbridge/nsh/pull/1664
* ci: Keep the dependency caches within the cache storage and build the website faster by @madcodelife in https://github.com/longbridge/nsh/pull/1660
* web: Keep reference accounting to a twentieth of the page's time by @sunfuze in https://github.com/longbridge/nsh/pull/1659
* ci: Test the code the macOS and iOS backends share on pull requests by @BeADre in https://github.com/longbridge/nsh/pull/1665
* platform: Send packaged macOS and iOS apps' logs to the unified log by @BeADre in https://github.com/longbridge/nsh/pull/1655
* macos: Give the app's windows frames while its main window is hidden by @BeADre in https://github.com/longbridge/nsh/pull/1652
* web: Deliver host.post to the page's host by @Cooooooker in https://github.com/longbridge/nsh/pull/1668
* windows: Let app context menus act on selectable text by @ttizze in https://github.com/longbridge/nsh/pull/1505
* list: Open at the end and preserve scrolls as rows shrink by @ttizze in https://github.com/longbridge/nsh/pull/1498
* core: Add onMouseUp for a mouse button's release by @YougLin-dev in https://github.com/longbridge/nsh/pull/1446
* ios: Clip a paste to maxLength instead of refusing it by @Cooooooker in https://github.com/longbridge/nsh/pull/1647
* windows: Cut canvas and partial-frame paint work by @YougLin-dev in https://github.com/longbridge/nsh/pull/1661
* core: Give an arbitrary or token text color its opacity modifier by @dXnSix in https://github.com/longbridge/nsh/pull/1669
* engine: Warn when the installed OTA package no longer runs by @BeADre in https://github.com/longbridge/nsh/pull/1671
* core: Take Tailwind's image hint on an arbitrary background gradient by @dXnSix in https://github.com/longbridge/nsh/pull/1670

</details>

<details>
<summary>New Contributors</summary>

## New Contributors
* @ClarkThan made their first contribution in https://github.com/longbridge/nsh/pull/909
* @griffinqiu made their first contribution in https://github.com/longbridge/nsh/pull/869
* @Cupnfish made their first contribution in https://github.com/longbridge/nsh/pull/890
* @lucianosrp made their first contribution in https://github.com/longbridge/nsh/pull/896
* @Teages made their first contribution in https://github.com/longbridge/nsh/pull/1003
* @nohzafk made their first contribution in https://github.com/longbridge/nsh/pull/1019
* @obito-t made their first contribution in https://github.com/longbridge/nsh/pull/1100
* @marcromeyn made their first contribution in https://github.com/longbridge/nsh/pull/881
* @antoniel made their first contribution in https://github.com/longbridge/nsh/pull/937
* @linyiru made their first contribution in https://github.com/longbridge/nsh/pull/1016
* @cosformula made their first contribution in https://github.com/longbridge/nsh/pull/1046
* @yangkx1024 made their first contribution in https://github.com/longbridge/nsh/pull/1049
* @Blackman99 made their first contribution in https://github.com/longbridge/nsh/pull/1216
* @YougLin-dev made their first contribution in https://github.com/longbridge/nsh/pull/1177
* @TexasOct made their first contribution in https://github.com/longbridge/nsh/pull/1118
* @Tzyito made their first contribution in https://github.com/longbridge/nsh/pull/1225
* @t4t5 made their first contribution in https://github.com/longbridge/nsh/pull/1217
* @Vehicoule made their first contribution in https://github.com/longbridge/nsh/pull/1248
* @williamjakfar made their first contribution in https://github.com/longbridge/nsh/pull/1181
* @wuyunyoujiduo-jpg made their first contribution in https://github.com/longbridge/nsh/pull/1224
* @hsingjui made their first contribution in https://github.com/longbridge/nsh/pull/1326
* @ahonn made their first contribution in https://github.com/longbridge/nsh/pull/1328
* @hmziqrs made their first contribution in https://github.com/longbridge/nsh/pull/1321
* @kim0 made their first contribution in https://github.com/longbridge/nsh/pull/1255
* @yuanzhuang-svg made their first contribution in https://github.com/longbridge/nsh/pull/1394
* @duo made their first contribution in https://github.com/longbridge/nsh/pull/1397
* @caoxing9 made their first contribution in https://github.com/longbridge/nsh/pull/1387
* @ZZombiee made their first contribution in https://github.com/longbridge/nsh/pull/1401
* @RebelBIrd made their first contribution in https://github.com/longbridge/nsh/pull/1395
* @richerfu made their first contribution in https://github.com/longbridge/nsh/pull/1392
* @Laotree made their first contribution in https://github.com/longbridge/nsh/pull/1428
* @mrtolkien made their first contribution in https://github.com/longbridge/nsh/pull/1482
* @Joy-jinyu made their first contribution in https://github.com/longbridge/nsh/pull/1479
* @kosumic made their first contribution in https://github.com/longbridge/nsh/pull/1432
* @boyan01 made their first contribution in https://github.com/longbridge/nsh/pull/1502
* @zcg made their first contribution in https://github.com/longbridge/nsh/pull/1525
* @Lemenxzy made their first contribution in https://github.com/longbridge/nsh/pull/1571
* @kindred-7 made their first contribution in https://github.com/longbridge/nsh/pull/1422
* @itisvincent made their first contribution in https://github.com/longbridge/nsh/pull/1595
* @taokeqin made their first contribution in https://github.com/longbridge/nsh/pull/1453
* @daoleno made their first contribution in https://github.com/longbridge/nsh/pull/1641

</details>

**Full Changelog**:https://github.com/longbridge/nsh/compare/v0.4.0...v0.5.0

🤖 [Noop](https://huacnlee.github.io/noop) (release-manager) · AI agent acting for @huacnlee


