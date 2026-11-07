# Coverage

Every `nsh_core` and `nsh_base` element export on NSH 0.4.0 (e626a8d), and where this app uses it. The list comes from the generated declarations in `.nsh/types/nsh/` (regenerate them with `nsh check` at the repo root if `.nsh/` is missing) cross-checked against the 0.4.0 crate sources. The "Used in" column is a grep over `src/` and `packages/remote-screen/`; rows marked *planned* name the page an area is building — the audit pass replaces those with real paths, and anything still unused gets a reason.

The Rust constructors are what screens call (`use nsh_core::prelude::*`; `nsh_base` exports the same components as lowercase factory functions — `button()`, `tree()`, `virtual_list()` — next to their PascalCase types); the TSX names are the `nsh` package's exports, which mirror the same crates. The tables use the export names from the generated declarations; both surfaces are listed because the app ships both (Rust root plus the TSX remote-screen package).

Status at this audit (mid-build snapshot): the shell routing, the data feed, the lists screens and the remote-screen package have landed while other areas are still in flight — rows below name real paths where the grep found them, and *planned* where their area has not landed yet. The final audit pass re-runs the greps and leaves nothing marked planned.

## nsh_core elements

| Rust (prelude) | TSX (`nsh`) | Used in | Notes |
|---|---|---|---|
| `view()` | `<View>` | everywhere (every screen); TSX: the remote-screen package | The base layout element. |
| `text()` | `<Text>` | everywhere; TSX: the remote-screen package | The only way to render a string. |
| `image()` | `<Image>` | planned: canvas (image loading with placeholders and blur) | `Image` also carries module members (cache/size helpers: `image_size`, `prefetch_image`). |
| `canvas()` | `<Canvas>` | planned: canvas (drawing app, animated canvas) | Painter runs when signals it reads change. |
| `svg()` | — (Rust only) | planned: canvas (SVG) | |
| `video()` | `<Video>` | planned: canvas (video) | `Video` is a component with a `VideoRef`. |
| `web_view()`, `web_view_html()` | `<WebView>` | planned: canvas (`web_view` with messaging) | Needs the `webview` capability. |
| `scroll()` | `<Scroll>` | TSX: the remote-screen package's root; planned: lists, overlays | |
| `overlay()` | `<Overlay>` | planned: overlays | |
| `virtual_list()` | `<List>` | real: `src/features/data/feed.rs` (feed rows), `src/features/lists/orders.rs` (infinite orders) | TSX `List` recycles rows; keep per-item state in the data. |
| `shader()` | `<Shader>` | planned: canvas (WGSL `shader`) | Reading `nsh.time` animates it. |
| `shader_effect()` | `<ShaderEffect>` | planned: canvas (animated canvas) | Draws children through a WGSL `shade`. |
| `shader_transition()` | `<ShaderTransition>` | planned: motion (transition demo) | |
| `screen_effect()` | `<ScreenEffect>` | planned: platform (device page) | Blur/material behind children. |
| `screen_transition()` | `<ScreenTransition>` | planned: motion | |
| `input_accessory_view()` | `nsh/mobile` `InputAccessoryView` | planned: forms (keyboard flow) | See the `nsh/mobile` table below. |
| — | `<ErrorBoundary>` | planned: shell (root) | Rust side handles component errors with `catch_error`. |
| `each()` | `<Each>` (global) | real: throughout `src/` | A global in TSX — importing it is a type error. |
| — | `<If>` (global) | TSX: the remote-screen package; planned: overlays, data | Same: a global, never imported. |

Module and framework surfaces of `nsh_core` — not elements, but the rest of the `nsh` export list, which the same accounting covers. Rust screens call most of these through the prelude (lowercase names); the TSX names are the `nsh` package's exports.

| Surface | Exports | Used in |
|---|---|---|
| Animation values | `Animated`, `AnimatedNode`, `AnimatedValue`, `createAnimatedValue`, `createMotion`/`Motion`, `Easing` | planned: motion (springs, driven values) |
| Layout animation | `LayoutAnimation` | planned: motion (layout animations) |
| Gestures | `PanResponder` | planned: motion (drag, pinch, rotate) |
| Motion settings | `reduceMotion`, `motions` | planned: motion (reduced-motion paths) |
| Canvas types | `CanvasGradient`, `CanvasRenderingContext2D`, `Path2D` | planned: canvas (the drawing app's paint and paths) |
| Accessibility | `announce`, `setAccessibilityFocus`, `screen_reader` (TSX `screenReaderEnabled`) | planned: platform (accessibility page) |
| Native bridging | `nativeComponent`, `nativeModule`, `split` | planned: the TSX entry's import of the crate's `#[native_module]` export; `split` in the ui wrappers that pass props through |
| Capability modules | `Clipboard`, `Keyboard`, `Picker`, `Speech` (+ `SpeechSession`), `Storage`, `host`, `keymap`, `lifecycle`, `openUrl` | planned: platform (device page), foundation (tasks, file logging), shell (keymap, lifecycle), overlays (host surfaces) |
| Environment accessors | `safeArea`/`safeAreaInsets()`, `keyboardFrame`, `colorScheme`/`setColorScheme`, `locale`, `pixelRatio`, `fontScale`, `platform`, `windowSize`, `is_desktop`, `is_catalyst` | planned: platform (device page), shell (density and navigation adaptation) |
| Framework functions | `signal`, `computed`, `effect`, `batch`, `untracked`, `cleanup`, `resource`, `context`, `token`, `mount`, `spawn`, `timer`, and Rust's `native_module` macro | real: signals, effects and `each` throughout `src/`; `mount` real: the remote-screen package's `App.tsx`; the rest land with their feature pages |

The capability modules' per-target behavior is accounted by the platform area's `tests/platform.rs` (planned until that area lands it), and `docs/STATUS.md` records the per-target gaps.

## nsh_base elements

Grouped rows name every export of a family; parts with nothing to say of their own share the row.

| Export(s) | Used in |
|---|---|
| `Accordion`, `AccordionHeader`, `AccordionItem`, `AccordionPanel`, `AccordionTrigger` | planned: ui gallery |
| `AlertDialog` | planned: overlays |
| `Avatar`, `AvatarGroup` | planned: data (feed authors) |
| `Button` | real: the `button()` factory in `src/features/data/feed.rs`, `src/features/lists/`, `src/shell/back.rs`; TSX: the remote-screen package wraps `nsh/base`'s `Button` in its own `ui.tsx` |
| `Calendar` | real: the lists page's calendar demo (`src/features/lists/`); planned: forms (date) |
| `Carousel`, `CarouselContent`, `CarouselItem`, `CarouselNext`, `CarouselPagination`, `CarouselPaginationItem`, `CarouselPrevious` | real: the lists page's carousel demo (`src/features/lists/`) |
| `Checkbox` | planned: forms (sign-up), ui gallery |
| `Collapsible` | planned: overlays, data devtools |
| `ColorArea`, `ColorEyeDropper`, `ColorField`, `ColorPicker`, `ColorSlider`, `ColorSwatch` | planned: forms (color picker). `ColorEyeDropper`: reason at audit if it stays off non-desktop targets. |
| `Combobox` | planned: forms (sign-up) |
| `Command` | planned: shell (command palette) |
| `ContextMenu` | planned: overlays |
| `DataTable` | real: the quotes table (`src/features/lists/quotes.rs`) |
| `DatePicker` | planned: forms (sign-up) |
| `Dialog` | real: the `dialog()` factory in `src/shell/back.rs` (the back-guard confirm); planned: more surfaces in overlays |
| `DiffView` | planned: overlays |
| `DockArea` | planned: overlays (desktop) |
| `Editor` | planned: overlays (desktop) |
| `HoverCard` | planned: overlays |
| `Input` | TSX: the remote-screen package's register form (`nsh/base` `Input`); planned: forms, ui gallery |
| `Link` | planned: data (detail), platform |
| `Menu`, `MenuItem`, `MenuSeparator`, `Submenu` | planned: overlays; shell desktop menus |
| `NavStack`, `NavPage` (+ `createNavStack`) | planned: shell (every page mounts through them) |
| `NumberInput` | planned: forms (number input) |
| `OtpInput` | planned: forms (OTP) |
| `Pagination` | planned: lists |
| `Popover` | planned: overlays |
| `Progress` | planned: data (skeletons/loading), foundation (tasks) |
| `Radio`, `RadioGroup` | planned: forms (sign-up) |
| `ResizablePanel`, `ResizablePanelGroup` | planned: overlays (desktop) |
| `RichEditor` | planned: forms (the rich editor screen) |
| Rich editor session, schema and extension builders: `createRichEditor`, `defineSchema`, `starterSchema`, `defineExtension`, `richExtension` | planned: forms — the session the screen drives and the schema it edits with |
| Rich editor types: `RichEditorPlugin`, `RichEditorSession`, `RichImage`, `RichLink`, `RichSchema`, `RichTable`, `RichTableCell`, `RichTableHeader`, `RichTableRow`, `RichTextStyle` | planned: forms — the types the screen's session and schema are built from |
| Kits: `StarterKit`, `RichExtension`, `TableKit` | planned: forms — the kits the editor screen composes |
| StarterKit extension components: `Blockquote`, `Bold`, `BulletList`, `Code`, `CodeBlock`, `Document`, `HardBreak`, `Heading`, `Highlight`, `History`, `HorizontalRule`, `Italic`, `ListKeymap`, `OrderedList`, `Paragraph`, `Strike`, `Subscript`, `Superscript`, `TaskItem`, `TaskList`, `TextAlign`, `TrailingNode`, `Typography`, `Underline` | planned: forms — the marks, nodes and behaviors StarterKit brings; any the screen never exercises gets a reason at the final audit |
| Rich editor commands: `addColumnAfter`, `addColumnBefore`, `addRowAfter`, `addRowBefore`, `chain`, `characterCount`, `clearNodes`, `command`, `cut`, `deleteColumn`, `deleteCurrentNode`, `deleteNode`, `deleteRange`, `deleteRow`, `deleteTable`, `exitSuggestion`, `extendMarkRange`, `first`, `fixTables`, `focus`, `goToNextCell`, `goToPreviousCell`, `insertContent`, `insertContentAt`, `insertMention`, `insertTable`, `insertText`, `joinItemBackward`, `joinItemForward`, `lift`, `liftListItem`, `mergeCells`, `mergeOrSplit`, `moveTableColumn`, `moveTableRow`, `redo`, `replaceSuggestion`, `resetAttributes`, `scrollIntoView`, `selectAll`, `setBlockType`, `setCellAttribute`, `setCellSelection`, `setCharacterLimit`, `setColumnWidth`, `setLink`, `setMark`, `setMeta`, `setNode`, `setNodeAttribute`, `setNodeSelection`, `setRowHeight`, `setTextSelection`, `sinkListItem`, `splitCell`, `splitListItem`, `toggleHeaderCell`, `toggleHeaderColumn`, `toggleHeaderRow`, `toggleLink`, `toggleList`, `toggleMark`, `toggleNode`, `toggleWrap`, `undo`, `unsetAllMarks`, `unsetLink`, `unsetMark`, `updateAttributes`, `wrapIn`, `wrapInList` | planned: forms — the editor screen exercises these through its session; any command the screen never touches is listed here with a reason at the final audit |
| Rich editor plugins and input rules: `baseKeymap`, `history`, `inputRules`, `keymap`, `link`, `markRule`, `markdown`, `markdownRules`, `starterKeymap`, `starterKit`, `suggestion`, `textRule`, `textblockRule`, `typographyRules`, `wrappingRule`, `withMention` | planned: forms — the plugin set the editor screen composes; unused plugins get a reason at the final audit |
| `Scrollbar` | planned: lists, overlays (desktop) |
| `Select` | planned: forms (sign-up) |
| `Sheet` | planned: overlays |
| `Slider` | planned: forms (sign-up) |
| `Switch` | planned: forms, settings |
| `Tab`, `Tabs` | planned: ui gallery, platform |
| `Table`, `TableBody`, `TableCaption`, `TableCell`, `TableFooter`, `TableHead`, `TableHeader`, `TableRow` | planned: lists |
| `TextView` | planned: overlays |
| `TimeField` | planned: forms (sign-up) |
| `Toast`, `ToastStack` | planned: foundation (central error surface), overlays |
| `ToastManager` | planned: foundation — the headless toast store (auto-hide, limits, swipe-away) the stack renders from |
| `Toggle`, `ToggleGroup` | planned: ui gallery |
| `Toolbar`, `ToolbarGroup`, `ToolbarSeparator` | planned: canvas (drawing tools) |
| `Tooltip` | planned: overlays, shell (desktop) |
| `Tree` | real: the workspace tree demo (`src/features/lists/tree.rs`) |
| Plot family: `Arc`, `Area`, `Bar`, `Cell`, `Dot`, `Grid`, `Line`, `Pie`, `Plot`, `PlotAppearScope`, `PlotAxis`, `PlotLabel`, `RadialGrid`, `RadialLine`, `Rule`, `Sankey`, `Treemap` | planned: canvas (Plot charts). Any chart type with no screen gets its reason at audit. |

## nsh/mobile

| Export | Used in |
|---|---|
| `TabBar` | planned: shell (mobile navigation) |
| `InputAccessoryView` | planned: forms (keyboard flow) |
| `BackHandler` | planned: shell (Android back order) |

## nsh/desktop

| Export | Used in |
|---|---|
| `Window` | planned: shell (window assembly) |
| `Menu` | planned: shell (application menus) |
| `titleBar` | planned: shell (title bar layout) |
| `isAvailable` | planned: shell, platform |

## Unused elements

Nothing is excluded yet: the areas are still landing, so every not-yet-referenced element above is marked planned rather than unused. The final audit pass moves anything the finished screens never touch into this section, each with its reason — an element is only "unused" here with a sentence saying why, never silently.
