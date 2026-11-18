title:	nsh v0.6.0
tag:	v0.6.0
draft:	false
prerelease:	false
immutable:	false
author:	github-actions[bot]
created:	2026-11-18T01:05:58Z
published:	2026-11-18T03:40:12Z
url:	https://github.com/longbridge/nsh/releases/tag/v0.6.0
asset:	install.ps1
asset:	install.sh
asset:	nsh-android-symbols.tar.gz
asset:	nsh-android-symbols.tar.gz.sha256
asset:	nsh-android.tar.gz
asset:	nsh-android.tar.gz.sha256
asset:	nsh-embed-android.zip
asset:	nsh-embed-android.zip.sha256
asset:	nsh-embed-ios.zip
asset:	nsh-embed-ios.zip.sha256
asset:	nsh-gpui-linux-arm64.tar.gz
asset:	nsh-gpui-linux-arm64.tar.gz.sha256
asset:	nsh-gpui-linux-x86_64.tar.gz
asset:	nsh-gpui-linux-x86_64.tar.gz.sha256
asset:	nsh-gpui-macos-arm64.tar.gz
asset:	nsh-gpui-macos-arm64.tar.gz.sha256
asset:	nsh-gpui-macos-x86_64.tar.gz
asset:	nsh-gpui-macos-x86_64.tar.gz.sha256
asset:	nsh-gpui-windows-arm64.tar.gz
asset:	nsh-gpui-windows-arm64.tar.gz.sha256
asset:	nsh-gpui-windows-x86_64.tar.gz
asset:	nsh-gpui-windows-x86_64.tar.gz.sha256
asset:	nsh-ios-macos-arm64.tar.gz
asset:	nsh-ios-macos-arm64.tar.gz.sha256
asset:	nsh-linux-arm64.tar.gz
asset:	nsh-linux-arm64.tar.gz.sha256
asset:	nsh-linux-x86_64.tar.gz
asset:	nsh-linux-x86_64.tar.gz.sha256
asset:	nsh-macos-arm64.tar.gz
asset:	nsh-macos-arm64.tar.gz.sha256
asset:	nsh-macos-x86_64.tar.gz
asset:	nsh-macos-x86_64.tar.gz.sha256
asset:	nsh-tui-linux-arm64.tar.gz
asset:	nsh-tui-linux-arm64.tar.gz.sha256
asset:	nsh-tui-linux-x86_64.tar.gz
asset:	nsh-tui-linux-x86_64.tar.gz.sha256
asset:	nsh-tui-macos-arm64.tar.gz
asset:	nsh-tui-macos-arm64.tar.gz.sha256
asset:	nsh-tui-macos-x86_64.tar.gz
asset:	nsh-tui-macos-x86_64.tar.gz.sha256
asset:	nsh-tui-windows-arm64.tar.gz
asset:	nsh-tui-windows-arm64.tar.gz.sha256
asset:	nsh-tui-windows-x86_64.tar.gz
asset:	nsh-tui-windows-x86_64.tar.gz.sha256
asset:	nsh-web.tar.gz
asset:	nsh-web.tar.gz.sha256
asset:	nsh-windows-arm64.zip
asset:	nsh-windows-arm64.zip.sha256
asset:	nsh-windows-x86_64.zip
asset:	nsh-windows-x86_64.zip.sha256
asset:	nsh.json
--
A large release: 457 commits since v0.5.1 from 60+ contributors. Highlights are native mobile controls (Switch, DateTimePicker, system context menus and share sheet, iOS sheet detents), spring-based style and layout animation, multilingual and channel-aware updates, a capability gate for the application and home directory, and a long list of Windows fidelity fixes.

## Breaking Changes

- **Capabilities** — apps that declare `capabilities` must now list `application` (main window, app menu, tray, global shortcuts, quit/restart, launch at login) and `homeDir` (`os.homedir()`). Apps without a declaration are unchanged. Closes a sandbox escape for embedded apps. (#1499)
- **Input `maxLength`** counts characters, not UTF-16 units; `nsh_core::text_edit::clip_utf16` is replaced by `clip_chars`. On Android, an app's `value` longer than `maxLength` is no longer cut. (#2266)
- **WebView downloads** use event handles and a shared lifecycle (`cancelled` state, `source`, `code`, `reason`); native adapters move from `AcceptDownload` to `StartDownload`. Linux's `webview` feature needs WebKitGTK 2.42. (#1666, #1986)
- **iOS `Clipboard.readText()`** rejects with `ERR_ACCESS_DENIED` when the person refuses the paste, instead of returning `""`. (#2004)
- **`AlertDialogOptions::show`** returns a named type with `closer()`; awaiting it still works. (#2294)
- **`ContextMenu`** listeners (Rust) take `ContextMenuEvents` instead of `MenuEvent`. (#2202)
- **`Transition`** is `#[non_exhaustive]` and gains `spring`. (#2259)
- **`ModalConfig` / `ModalProp`** (platform-facing) are no longer `Copy`. (#2243)
- **`RichEditorEvent`** gains `CompositionChange`. (#2072)
- **DataTable `selectOnModifierPress={false}`** now leaves cell and title selection alone too. (#2059)
- **Dev server** answers 403 to hosts that reach it by a rebound name; connect by the printed address. (#2153)
- **GPUI embedding** needs GPUI Fast 0.1.14+ / GPUI Kit 0.9.1+. (#2093)
- Platform-facing additions for out-of-tree platforms: `Command::Share` / `Environment::share`, `clipboard_images: Option<bool>`, `TextBackground` vertical padding fields, macOS `AppOptions::handoff`. (#2198, #2164, #2215, #2150)

## New Features

- **Mobile controls**: `Switch` and `DateTimePicker` use the system controls (#2284, #2278); system context menus on iOS (#2202) and system menus / segmented controls (#1736); `Share` opens the system share sheet (#2198).
- **Sheets and dialogs**: iOS sheets rest at detents with a grabber and an inner stack (#2243); bottom sheets get detents (#2194); the sheet is a named modal dialog everywhere (#2028); apps can close an alert they showed (#2294).
- **Animation**: style transitions on a spring (#2259); layout springs take stiffness, damping and mass (#2227); `ViewRef.animateLayout` and Bezier layout curves; Collapsible reveal spring (#2233).
- **Updater**: per-language release notes, one file per language (#2286); named channels (#2249); `installOnQuit` (#2263); shell-only updates reported (#2222).
- **Auth and desktop**: `open_auth_session` for web sign-in (#2285, #2304); launch at login (#1884); app-named data directory (#2232); Handoff on iOS and macOS (#2150); Linux session-end save (#2213); macOS quit/close confirmation (#2050).
- **Input and events**: pen pressure, tilt and pointer type (#2142); desktop hardware key events for custom views (#1590); mouse and context-menu events on any component (#1881); reactive size classes (#2081); MacBook notch as `cutouts` (#1981).
- **Text and rich editor**: annotated code and custom inline spans in TextView (#1741); task-item presses (#2045); `onContentSizeChange` (#2071); TS plugins in paste (#2154); span padding (#2141, #2215).
- **Layout**: SplitView adaptive columns with independent navigation stacks (#2096); `overlays: "viewport"` on Web (#2257); `scrollsToTop` (#2012).
- **Packaging**: iOS launch screen color/image from `nsh.json` (#2293); strict Content-Security-Policy for packaged Web apps (#2155); install script progress (#2161).
- **Size**: WebSocket, updater, SVG and HTTP become optional features (#1709); WebView2 and media optional on Windows (#2135); GPUI embedders can leave optional parts out (#1886).

## Improvements

- **Performance**: cheap updates of `each` for rows that stay (#2268); restyle only moved siblings (#1596); per-batch redraw decisions on macOS (#1924); startup/process-tree/size sampler for baselines (#2210).
- **Windows**: TSF composition reported live (#1994), single caption-button set and system icon font (#2282, #2133), correct frame/placement and minimize restore (#2270, #2225), Windows 11 dark backdrop (#2238), shader frames shared with Direct2D (#2126), and many rendering fixes.
- **macOS / iOS**: editor focus restored after dialogs (#2267), composition keys kept in hosted inputs (#2288), sharp SVG on canvas (#2272), hardware shortcuts after keyboard dismissal (#2115), accessibility roles restored (#1874), windows move from `windowDrag` views with a hidden title bar (#2295), new windows open at the app's default position (#2325), iOS auth session presentation anchor registered (#2310).
- **Accessibility**: native components keep accessibility nodes under GPUI (#2093); menu item descriptions read by screen readers (#2043); `describedBy`-style element links (#1910).
- **Linux**: touchpad fling velocity no longer underestimated on release (#2328).
- **Components**: DataTable hovers views drawn by `renderCell` (#2223); List shows each row in a single cell, including spare cells (#2312).
- **Docs**: minimum browsers for published web apps (#2296).
- **Testing**: iOS tests can press bar buttons and choose from system menus (#2313); headless hosts run without stealing focus (#1813), Tab reaches focusable views on Windows (#2006), tests can fail opened URLs (#1869).

<details><summary>Contributors</summary>

@0xbentang @ahonn @akongG @Amamiya23 @antoniel @ArnoChenFx @BeADre @Blackman99 @blueberrycongee @chaxus @ClarkThan @Cooooooker @cosformula @crizant @daoleno @davo20019 @daydoing @duo @dXnSix @hexiang1993 @hexuria @hmziqrs @honsunrise @huacnlee @jcdiv47 @jesseXu @jjyr @Joy-jinyu @jsiu93 @KeepNoob @kim0 @labcoder @LeeDashan @Liguoqiang-code @linyisu @lnnduy @lurenjia534 @LVtomatoJ @madcodelife @mengyinghu-sys @mliemm @obito-t @RebelBIrd @richerfu @sekzerd @subframe7536 @sunfuze @sunli829 @t4t5 @tacshi @teng-lin @ttizze @Tzyito @villatothesea @wenlingang @williamjakfar @WillisXue @xhofe @YougLin-dev @yuezk 

</details>

<details><summary>What's Changed</summary>

* testing: Break the line on Shift+Return in a multiline input that submits by @madcodelife in https://github.com/longbridge/nsh/pull/1679
* windows: Close overlays on a click on a windowDrag view by @WillisXue in https://github.com/longbridge/nsh/pull/1658
* core: Let a focused control take Return and Space before other shortcuts by @madcodelife in https://github.com/longbridge/nsh/pull/1678
* desktop: Follow the app's language in native menus by @ttizze in https://github.com/longbridge/nsh/pull/1510
* access: Open media files under the host's narrower grant by @huacnlee in https://github.com/longbridge/nsh/pull/1677
* macos: Grow multiline inputs with what an input method composes by @sunfuze in https://github.com/longbridge/nsh/pull/1689
* radio: Let a `RadioGroup` take a name screen readers say by @madcodelife in https://github.com/longbridge/nsh/pull/1692
* rich_editor: Size an editor without a height to its content by @Cooooooker in https://github.com/longbridge/nsh/pull/1690
* web: Remove an open dialog with the subtree that held it by @Cooooooker in https://github.com/longbridge/nsh/pull/1693
* core: Check that a focused control takes Return before the keymap by @madcodelife in https://github.com/longbridge/nsh/pull/1701
* rich_editor: Put a task item's text where it is drawn by @Tzyito in https://github.com/longbridge/nsh/pull/1706
* rich_editor: Declare an extension's plugins as a list by @Tzyito in https://github.com/longbridge/nsh/pull/1707
* testing: Keep Tab inside the newest dialog, as the desktops do by @madcodelife in https://github.com/longbridge/nsh/pull/1696
* macos: Read an alert as it appears and as its content changes by @madcodelife in https://github.com/longbridge/nsh/pull/1699
* list: Keep the first visible row in place as rows above it are measured by @madcodelife in https://github.com/longbridge/nsh/pull/1704
* ci: Refresh the nightly from stable releases and skip it on release day by @huacnlee in https://github.com/longbridge/nsh/pull/1708
* core: Keep one wheel scroll on the view it went to by @LeeDashan in https://github.com/longbridge/nsh/pull/1651
* macos: Clear stale text undo actions before editor changes by @mliemm in https://github.com/longbridge/nsh/pull/1694
* core: Mount a view's first children without rescanning siblings by @huacnlee in https://github.com/longbridge/nsh/pull/1680
* cli: Draw the spinner line in one write so it stops flickering by @huacnlee in https://github.com/longbridge/nsh/pull/1733
* rich_editor: Inset the sink with root padding and fill the viewport by @richerfu in https://github.com/longbridge/nsh/pull/1702
* cli: Build nsh-core and nsh-base once across a Rust app's programs by @huacnlee in https://github.com/longbridge/nsh/pull/1687
* text_view: Keep a bare URL in a link's text as text by @madcodelife in https://github.com/longbridge/nsh/pull/1715
* engine: Check a Rust root's calls against its nsh.json capabilities by @madcodelife in https://github.com/longbridge/nsh/pull/1718
* windows: Activate the window when its frame is pressed by @YougLin-dev in https://github.com/longbridge/nsh/pull/1717
* core: Count the system's wakes from sleep in `wakeCount()` by @madcodelife in https://github.com/longbridge/nsh/pull/1723
* menu: Open `ContextMenu` below its trigger when the app opens it by @madcodelife in https://github.com/longbridge/nsh/pull/1725
* testing: Name each test file's entry by its path in the app by @Cooooooker in https://github.com/longbridge/nsh/pull/1734
* android: Keep the app alive when a web view's page process ends by @madcodelife in https://github.com/longbridge/nsh/pull/1763
* windows: Let apps save their state when the session ends by @madcodelife in https://github.com/longbridge/nsh/pull/1744
* desktop: Leave an app's own logger in place in the Windows and Linux runners by @LeeDashan in https://github.com/longbridge/nsh/pull/1703
* macos: Redraw only the recolored lines of a text by @madcodelife in https://github.com/longbridge/nsh/pull/1728
* core: Let a focused element name its active item for screen readers by @madcodelife in https://github.com/longbridge/nsh/pull/1727
* desktop: Let the app decide when the user asks to quit by @madcodelife in https://github.com/longbridge/nsh/pull/1768
* android: Pan a zoomed Scroll in a pager before the pager pages by @Cooooooker in https://github.com/longbridge/nsh/pull/1732
* testing: Let a test fail an image's load by @madcodelife in https://github.com/longbridge/nsh/pull/1775
* gpui: Override a running app's theme tokens with set_theme_overrides by @Cooooooker in https://github.com/longbridge/nsh/pull/1724
* docs: Define the scope of features NSH accepts by @huacnlee in https://github.com/longbridge/nsh/pull/1791
* testing: Give the focus back when a dialog closes, as the desktops do by @madcodelife in https://github.com/longbridge/nsh/pull/1771
* cli: Share a Rust app's build only when this machine's hosts follow by @madcodelife in https://github.com/longbridge/nsh/pull/1776
* core: Drop a cached image when its redirect is refused by @ClarkThan in https://github.com/longbridge/nsh/pull/1726
* terminal: Send Command's line editing keys as macOS terminals do by @Blackman99 in https://github.com/longbridge/nsh/pull/1633
* gallery: Add a block editor in the manner of Notion by @Tzyito in https://github.com/longbridge/nsh/pull/1731
* android: Keep the touches inside a scroll view through scroll commands by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/1714
* text_view: Size an app's own images against the text's width by @madcodelife in https://github.com/longbridge/nsh/pull/1782
* core: Align texts by their first baseline in items-baseline rows by @williamjakfar in https://github.com/longbridge/nsh/pull/1474
* picker: Report picked files too large to read instead of dropping them by @madcodelife in https://github.com/longbridge/nsh/pull/1790
* testing: Act on views at a scroll view's end without dragging past it by @Cooooooker in https://github.com/longbridge/nsh/pull/1749
* testing: Keep removed views from hearing hover and keys in headless runs by @Cooooooker in https://github.com/longbridge/nsh/pull/1750
* macos: Preserve actions and selection inside drawn text by @ttizze in https://github.com/longbridge/nsh/pull/1743
* core: Let a page say which orientations the screen may take by @daoleno in https://github.com/longbridge/nsh/pull/1766
* macos: Zoom scroll views with pinches, Control + wheel and zoomTo by @LVtomatoJ in https://github.com/longbridge/nsh/pull/1491
* data_table: Keep scaled web scrollbars stable at rest by @crizant in https://github.com/longbridge/nsh/pull/1721
* core: Match mobile classes and accessories by OS and include OHOS budgets by @richerfu in https://github.com/longbridge/nsh/pull/1710
* testing: Let a headless test forget the commands sent so far by @madcodelife in https://github.com/longbridge/nsh/pull/1793
* core: Slide a list's rows under a layout animation by @Cooooooker in https://github.com/longbridge/nsh/pull/1740
* desktop: Log packaged apps to a rolling file and honor RUST_LOG by @madcodelife in https://github.com/longbridge/nsh/pull/1748
* testing: Drive a page's browser history in headless tests by @madcodelife in https://github.com/longbridge/nsh/pull/1781
* text: Place inline views on their wrapped line in headless layout by @madcodelife in https://github.com/longbridge/nsh/pull/1787
* rich_editor: Start the task Enter makes from a checked one unchecked by @Tzyito in https://github.com/longbridge/nsh/pull/1797
* rich_editor: Keep a task checkbox's press when the pointer moves as it lands by @Tzyito in https://github.com/longbridge/nsh/pull/1798
* core: Mount the pointer-events of a view made after they were worked out by @Tzyito in https://github.com/longbridge/nsh/pull/1812
* core: Keep a dialog's `supportedOrientations` from being read as a page's by @madcodelife in https://github.com/longbridge/nsh/pull/1845
* rich_editor: Give apps a layer over the document that scrolls with it by @Tzyito in https://github.com/longbridge/nsh/pull/1746
* gallery: Keep the block editor's handle with its block, add toggles and nesting by drag by @Tzyito in https://github.com/longbridge/nsh/pull/1753
* web: Give input fields their accessible name and description by @jsiu93 in https://github.com/longbridge/nsh/pull/1844
* macos: Keep the keyboard focus out of hidden views by @0xbentang in https://github.com/longbridge/nsh/pull/1832
* macos: Build again with one NshView::surface by @ahonn in https://github.com/longbridge/nsh/pull/1819
* ios: Show large titles expanded, with the content below them by @ahonn in https://github.com/longbridge/nsh/pull/1811
* ios: Build the iOS runtime without the dev feature again by @daoleno in https://github.com/longbridge/nsh/pull/1810
* editor: Measure in the user's text size, so rows fit their text by @daoleno in https://github.com/longbridge/nsh/pull/1808
* macos: Run embedded scroll checks only on macOS by @ttizze in https://github.com/longbridge/nsh/pull/1807
* ios: Keep a long-press selection when the touch ends by @ttizze in https://github.com/longbridge/nsh/pull/1806
* core: Give a paced view in a list cell its frame when made by @huacnlee in https://github.com/longbridge/nsh/pull/1800
* desktop: Keep removed views from hearing hover leaves and blurs by @Cooooooker in https://github.com/longbridge/nsh/pull/1795
* core: Give a culled scroll view its content size before its first scroll by @obito-t in https://github.com/longbridge/nsh/pull/1794
* web: Do not run a page shortcut for a key a field's own handler already took by @cosformula in https://github.com/longbridge/nsh/pull/1784
* mobile: Stop only the tap in a scroll view with the keyboard up by @Cooooooker in https://github.com/longbridge/nsh/pull/1792
* core: Keep a padded label on one line at fractional pixel ratios by @cosformula in https://github.com/longbridge/nsh/pull/1705
* core: Give inputs and scroll views made late their content box and extent by @Blackman99 in https://github.com/longbridge/nsh/pull/1682
* core: Show the next tooltip at once after one the pointer showed by @BeADre in https://github.com/longbridge/nsh/pull/1683
* mobile: Show a badge on a TabBar item by @daoleno in https://github.com/longbridge/nsh/pull/1765
* js: Bound the array items the runtime reads natively by @ClarkThan in https://github.com/longbridge/nsh/pull/1685
* android: Keep a pan view's drags from the scroll view under it by @Cooooooker in https://github.com/longbridge/nsh/pull/1745
* core: Let apps bind a focused view's own Return and Space by @sunfuze in https://github.com/longbridge/nsh/pull/1789
* testing: Run every desktop's test hosts without taking the user's focus by @madcodelife in https://github.com/longbridge/nsh/pull/1813
* web: Keep the typing field's focus when moving the screen reader's by @madcodelife in https://github.com/longbridge/nsh/pull/1848
* core: Make the rows that show in a list cell that waited at once by @LeeDashan in https://github.com/longbridge/nsh/pull/1823
* linux: Move the screen reader to views that take no keyboard focus by @madcodelife in https://github.com/longbridge/nsh/pull/1851
* menu: Line a submenu's first item up with its trigger by @madcodelife in https://github.com/longbridge/nsh/pull/1825
* cli: Keep WebKitGTK out of a Rust app's headless test builds by @madcodelife in https://github.com/longbridge/nsh/pull/1836
* core: Resolve a header's and a list content's classes again when design tokens change by @dXnSix in https://github.com/longbridge/nsh/pull/1769
* android: Let embedding apps add safe-area insets for their own UI by @hexiang1993 in https://github.com/longbridge/nsh/pull/1843
* docs: Keep the background test hosts' pages to their direction by @madcodelife in https://github.com/longbridge/nsh/pull/1850
* core: Let a child's hit slop take touches outside its parent by @madcodelife in https://github.com/longbridge/nsh/pull/1751
* gpui: Place an app's windows on the display their position is on by @madcodelife in https://github.com/longbridge/nsh/pull/1829
* notifications: Close all of an app's notifications by @madcodelife in https://github.com/longbridge/nsh/pull/1774
* gpui: Let a hide or resize follow a show, and hidden windows lose focus by @madcodelife in https://github.com/longbridge/nsh/pull/1852
* macos: Leave full screen before hiding a window by @madcodelife in https://github.com/longbridge/nsh/pull/1772
* web: Do not submit a field on a Return that a shortcut takes by @sunfuze in https://github.com/longbridge/nsh/pull/1855
* core: Stop a multiline input's parent at the input's maximum height by @sunfuze in https://github.com/longbridge/nsh/pull/1788
* rich_editor: Show the selection in code blocks by @huacnlee in https://github.com/longbridge/nsh/pull/1837
* windows: Draw input text at the target's scale by @subframe7536 in https://github.com/longbridge/nsh/pull/1816
* sheet: End editing when the user starts dragging a sheet by @Cooooooker in https://github.com/longbridge/nsh/pull/1856
* core: Keep the layout when a restyle leaves the layout style as it was by @dXnSix in https://github.com/longbridge/nsh/pull/1770
* gpui: Turn images and icons with their rotated views by @Cooooooker in https://github.com/longbridge/nsh/pull/1864
* ios: Show the system's menus and segmented controls from apps by @huacnlee in https://github.com/longbridge/nsh/pull/1736
* web: Load side modules on demand and retry failed loads by @huacnlee in https://github.com/longbridge/nsh/pull/1588
* core: Apply hover: and active: styles to a nested Text under the pointer by @Cooooooker in https://github.com/longbridge/nsh/pull/1859
* js: Hand files dropped on Rust components such as RichEditor to the app by @Cooooooker in https://github.com/longbridge/nsh/pull/1739
* rich_editor: Walk the history's items with iterators by @madcodelife in https://github.com/longbridge/nsh/pull/1868
* engine: Apply what JS queued before binding a list's overscan by @madcodelife in https://github.com/longbridge/nsh/pull/1861
* testing: Settle a headless step once the engine has handled it by @madcodelife in https://github.com/longbridge/nsh/pull/1862
* ci: Run the headless checks on a self-hosted runner by @madcodelife in https://github.com/longbridge/nsh/pull/1827
* core: Drop a cancelled task's timer at once by @madcodelife in https://github.com/longbridge/nsh/pull/1866
* testing: Let tests fail the URLs an app opens by @madcodelife in https://github.com/longbridge/nsh/pull/1869
* desktop: Write panics of packaged apps to their log file by @madcodelife in https://github.com/longbridge/nsh/pull/1865
* macos: Keep text and canvases sharp under scaled ancestors by @YougLin-dev in https://github.com/longbridge/nsh/pull/1847
* menu: Take keys typed ahead in order by @huacnlee in https://github.com/longbridge/nsh/pull/1876
* windows: Release a destroyed window's focused field before thread exit by @huacnlee in https://github.com/longbridge/nsh/pull/1801
* core: Report the mouse and context menus over any component by @huacnlee in https://github.com/longbridge/nsh/pull/1881
* gpui: Let embedding apps leave the runtime's optional parts out by @obito-t in https://github.com/longbridge/nsh/pull/1886
* web: Turn a pager's pages from a zoomable Scroll under the finger by @Cooooooker in https://github.com/longbridge/nsh/pull/1887
* web: Turn the keyboard's autocorrection off with autoCorrect={false} by @daoleno in https://github.com/longbridge/nsh/pull/1883
* desktop: Add launch at login by @huacnlee in https://github.com/longbridge/nsh/pull/1884
* web: Title the page with the main window's title by @madcodelife in https://github.com/longbridge/nsh/pull/1873
* ci: Say the scheduled Core and Examples runs stay on GitHub's runners by @madcodelife in https://github.com/longbridge/nsh/pull/1877
* tabs: Name a tab list for screen readers with `accessibilityLabel` by @madcodelife in https://github.com/longbridge/nsh/pull/1860
* select: Let dropdown popups come in and go with the app's transitions by @madcodelife in https://github.com/longbridge/nsh/pull/1857
* windows: Put the app in the background while the screen cannot be seen by @madcodelife in https://github.com/longbridge/nsh/pull/1867
* gallery: Show the WebView demo full width in two tabs by @huacnlee in https://github.com/longbridge/nsh/pull/1915
* gallery: Center inline views on the text line by @huacnlee in https://github.com/longbridge/nsh/pull/1913
* web: Lay out only what an edit of a long text changed by @huacnlee in https://github.com/longbridge/nsh/pull/1914
* ci: Run the short Linux checks on the self-hosted runner instead of Core by @madcodelife in https://github.com/longbridge/nsh/pull/1917
* gpui: Cut a clamped text's last line at its box, not after a word by @obito-t in https://github.com/longbridge/nsh/pull/1918
* core: Render a canvas drawing offscreen into an image (`OffscreenCanvas`) by @wenlingang in https://github.com/longbridge/nsh/pull/1849
* macos: Keep a multiline input's caret in view as it grows by @sunfuze in https://github.com/longbridge/nsh/pull/1921
* macos: Decide which redrawn views show once per batch by @huacnlee in https://github.com/longbridge/nsh/pull/1924
* core: Keep a list's sticky rows below its built-in scroll bar by @obito-t in https://github.com/longbridge/nsh/pull/1909
* rich_editor: Place the caret and presses by an inline view's width by @Tzyito in https://github.com/longbridge/nsh/pull/1894
* android: Keep the development error screen clear of side insets by @daoleno in https://github.com/longbridge/nsh/pull/1898
* sheet: Keep bottom sheets and menus above the software keyboard by @madcodelife in https://github.com/longbridge/nsh/pull/1872
* macos: Preserve traffic light spacing with a custom offset by @huacnlee in https://github.com/longbridge/nsh/pull/1907
* macos: Collapse the performance monitor on a click, and let apps set how it shows by @sunfuze in https://github.com/longbridge/nsh/pull/1933
* android: Keep input text centered and carets natural on Android 15 by @linyisu in https://github.com/longbridge/nsh/pull/1929
* gpui: Zoom scroll views with touchpad pinches and zoomTo by @Cooooooker in https://github.com/longbridge/nsh/pull/1888
* engine: Test that an undefined pan handler turns the pan off by @Cooooooker in https://github.com/longbridge/nsh/pull/1885
* ios: Keep a page below its header after a headerless page is popped by @tacshi in https://github.com/longbridge/nsh/pull/1890
* text: Line up texts by their first baselines on every platform by @Tzyito in https://github.com/longbridge/nsh/pull/1841
* android: Keep short text on one line after an Input lays out on Android 15 by @daoleno in https://github.com/longbridge/nsh/pull/1871
* macos: List open windows in the Window menu by @huacnlee in https://github.com/longbridge/nsh/pull/1560
* ci: Run the Windows checks of Rust application and GPUI on a self-hosted runner by @madcodelife in https://github.com/longbridge/nsh/pull/1937
* terminal: Report the first size even when it is the starting 80 × 24 by @Blackman99 in https://github.com/longbridge/nsh/pull/1638
* diff_view: Keep unwrapped lines on one row at a larger text size by @daoleno in https://github.com/longbridge/nsh/pull/1804
* gpui: Batch canvas animation argument updates by @lurenjia534 in https://github.com/longbridge/nsh/pull/1815
* terminal: Keep each character on its cells by @Blackman99 in https://github.com/longbridge/nsh/pull/1649
* core: Let apps read what the performance monitor measures by @sunfuze in https://github.com/longbridge/nsh/pull/1923
* macos: Avoid 1px seams across partial-border cells on fractional scroll by @antoniel in https://github.com/longbridge/nsh/pull/1761
* core: Add desktop hardware key events for custom views by @honsunrise in https://github.com/longbridge/nsh/pull/1590
* ios: Forward a page's orientations to an embedding host's controller by @daoleno in https://github.com/longbridge/nsh/pull/1818
* core: List image::density as unitless where OffscreenCanvas moved it by @huacnlee in https://github.com/longbridge/nsh/pull/1944
* macos: Redraw text when a layout animation grows its height by @blueberrycongee in https://github.com/longbridge/nsh/pull/1758
* terminal: Draw an input method's composition at the cursor by @Blackman99 in https://github.com/longbridge/nsh/pull/1648
* core: Register only nsh/base's core part by default for Rust apps by @huacnlee in https://github.com/longbridge/nsh/pull/1730
* gallery: Keep the perf stats test's buttons clear of the monitor's panel by @sunfuze in https://github.com/longbridge/nsh/pull/1947
* core: Keep a component's own onContextMenu over its host's by @Cooooooker in https://github.com/longbridge/nsh/pull/1925
* text: Cut a clamped text's last line at its box on the web and terminal by @obito-t in https://github.com/longbridge/nsh/pull/1928
* ios: Show several trailing bar buttons with native menus in a NavPage's header by @ahonn in https://github.com/longbridge/nsh/pull/1336
* rich_editor: Let a closed suggestion list pass its captured keys on by @Cooooooker in https://github.com/longbridge/nsh/pull/1752
* core: Check a cached image's redirects again before serving it by @ClarkThan in https://github.com/longbridge/nsh/pull/1838
* js: Keep a removed dialog's dismiss handler outside cycle collection by @huacnlee in https://github.com/longbridge/nsh/pull/1698
* core: Document perfStats' dropped alike in Rust and TS, and hide its constructor by @sunfuze in https://github.com/longbridge/nsh/pull/1956
* gallery: Center the sidebar search's icon on its field by @madcodelife in https://github.com/longbridge/nsh/pull/1957
* docs: Keep the skills from piping scripts or obeying fetched pages by @huacnlee in https://github.com/longbridge/nsh/pull/1962
* web: Start a published site under a strict script-src by @huacnlee in https://github.com/longbridge/nsh/pull/1700
* gpui: Expect a popover's pressable trigger as a Tab stop in the tests by @huacnlee in https://github.com/longbridge/nsh/pull/1961
* windows: Walk an offscreen canvas's pixels in fixed four-byte chunks by @huacnlee in https://github.com/longbridge/nsh/pull/1960
* web: Route OffscreenCanvas's clear op to the canvas calls in the browser by @huacnlee in https://github.com/longbridge/nsh/pull/1959
* platforms: Make WebSocket, updater, SVG and HTTP optional features by @huacnlee in https://github.com/longbridge/nsh/pull/1709
* canvas: Clear offscreen canvases on the web, keep scaled images' edges by @madcodelife in https://github.com/longbridge/nsh/pull/1963
* ios: Keep a blocking responder's drag from swiping its page back by @hmziqrs in https://github.com/longbridge/nsh/pull/1757
* select: Test that the trigger keeps the app's accessibility states by @madcodelife in https://github.com/longbridge/nsh/pull/1905
* linux: Find Flatpak and Snap browsers when launched without XDG_DATA_DIRS by @madcodelife in https://github.com/longbridge/nsh/pull/1908
* core: Let an element name the element whose text describes it by @madcodelife in https://github.com/longbridge/nsh/pull/1910
* macos: Keep the test agent's presses where it aims on a locked screen by @LVtomatoJ in https://github.com/longbridge/nsh/pull/1729
* android: Color a stack's screens in the app's own appearance by @daoleno in https://github.com/longbridge/nsh/pull/1950
* testing: Avoid false cover errors for wholly clipped targets by @kim0 in https://github.com/longbridge/nsh/pull/1735
* js: Match platform.name first in platform.select by @richerfu in https://github.com/longbridge/nsh/pull/1713
* core: Reveal views nested in culled inline attachments by @ttizze in https://github.com/longbridge/nsh/pull/1742
* linux: Build without GStreamer when an app plays no sounds by @huacnlee in https://github.com/longbridge/nsh/pull/1964
* rich_editor: Copy the selection as HTML, and paste it back as it was by @Tzyito in https://github.com/longbridge/nsh/pull/1935
* core: Decide WebView downloads through event handles by @honsunrise in https://github.com/longbridge/nsh/pull/1666
* core: Show what views inside text hold when in a scroll view by @jcdiv47 in https://github.com/longbridge/nsh/pull/1948
* macos: Show alerts in front while the app's window is out of sight by @BeADre in https://github.com/longbridge/nsh/pull/1697
* text_view: Support annotated code and custom inline spans by @ttizze in https://github.com/longbridge/nsh/pull/1741
* macos: Restore native text-field accessibility roles by @0xbentang in https://github.com/longbridge/nsh/pull/1874
* hover_card: Follow the pointer only, not the keyboard focus by @BeADre in https://github.com/longbridge/nsh/pull/1695
* android: Let a tap through a scroll view once the keyboard is down by @daoleno in https://github.com/longbridge/nsh/pull/1755
* android: Pass the declared environment to Rust apps under nsh run by @madcodelife in https://github.com/longbridge/nsh/pull/1958
* core: Gate the application and os.homedir() behind capabilities by @hexuria in https://github.com/longbridge/nsh/pull/1499
* ci: Fix main's Linux, Windows and Rust application checks by @madcodelife in https://github.com/longbridge/nsh/pull/1971
* gpui: Blur the focused field once the mouse starts selecting text by @obito-t in https://github.com/longbridge/nsh/pull/1968
* windows: Leave views that cannot paint out of the frame by @villatothesea in https://github.com/longbridge/nsh/pull/1973
* testing: Keep the native-only engine features out of the web runtime by @madcodelife in https://github.com/longbridge/nsh/pull/1977
* ci: Fix the compat build and the application-refused case by @huacnlee in https://github.com/longbridge/nsh/pull/1969
* core: Lay out anchored overlays within the room they are placed in by @madcodelife in https://github.com/longbridge/nsh/pull/1938
* web: Ask the app as the page leaves while it listens to quit requests by @madcodelife in https://github.com/longbridge/nsh/pull/1875
* web: Test the links a clamped line cuts and hides by what it draws by @madcodelife in https://github.com/longbridge/nsh/pull/1982
* test: Grant homeDir in the process/os parity test by @huacnlee in https://github.com/longbridge/nsh/pull/1979
* windows: Scroll a multiline input's text with the wheel by @huacnlee in https://github.com/longbridge/nsh/pull/1976
* core: Report the focus leaving an element removed with the focus inside by @madcodelife in https://github.com/longbridge/nsh/pull/1980
* gpui: Show Mac shortcut symbols and translated role labels in menus by @obito-t in https://github.com/longbridge/nsh/pull/1967
* input: Preserve password input behavior when revealing passwords by @daoleno in https://github.com/longbridge/nsh/pull/1828
* macos: Host an input's editor above scaled ancestors by @YougLin-dev in https://github.com/longbridge/nsh/pull/1965
* core: Let image sources declare their natural size by @lurenjia534 in https://github.com/longbridge/nsh/pull/1831
* macos: Keep typed quotes and dashes when autoCorrect is off by @wenlingang in https://github.com/longbridge/nsh/pull/1978
* windows: Stop partial frames leaving behind what was drawn before by @Blackman99 in https://github.com/longbridge/nsh/pull/1993
* list: Dispose the empty cell with its list by @daoleno in https://github.com/longbridge/nsh/pull/1990
* windows: Report what an input method writes through TSF as it writes it by @Blackman99 in https://github.com/longbridge/nsh/pull/1994
* core: Share WebView download lifecycle and support iOS by @honsunrise in https://github.com/longbridge/nsh/pull/1986
* android: Dismiss the keyboard of an input in a dialog by @akongG in https://github.com/longbridge/nsh/pull/2001
* ios: Reject Clipboard.readText when the person refuses the paste by @daoleno in https://github.com/longbridge/nsh/pull/2004
* core: Report a MacBook's notch as a display's `cutouts` by @madcodelife in https://github.com/longbridge/nsh/pull/1981
* core: Keep a root from taking its freed owner's reused slot for its owner by @daoleno in https://github.com/longbridge/nsh/pull/1991
* testing: Let Tab reach focusable views on Windows by @madcodelife in https://github.com/longbridge/nsh/pull/2006
* testing: Keep the keyboard focus off what pages and dialogs cover by @madcodelife in https://github.com/longbridge/nsh/pull/2013
* dev: Catch up a reconnecting host with the hot updates it missed by @huacnlee in https://github.com/longbridge/nsh/pull/1983
* engine: Name external platforms neutrally in the platform tests by @huacnlee in https://github.com/longbridge/nsh/pull/1966
* windows: Exit a closed app without tearing down its UI thread by @Amamiya23 in https://github.com/longbridge/nsh/pull/1992
* scroll: Add scrollsToTop to let a scroll view opt out of iOS's scroll-to-top by @daoleno in https://github.com/longbridge/nsh/pull/2012
* windows: Clear stale pixels in transparent partial redraws by @Amamiya23 in https://github.com/longbridge/nsh/pull/1995
* animation: Keep the graph's storage to the nodes that live by @madcodelife in https://github.com/longbridge/nsh/pull/2005
* select: Let apps choose whether the list opens over the trigger by @madcodelife in https://github.com/longbridge/nsh/pull/1989
* tui: Deliver terminal key transitions to custom views by @honsunrise in https://github.com/longbridge/nsh/pull/1919
* engine: Keep calls on the page's host when the page passes parameters by @Liguoqiang-code in https://github.com/longbridge/nsh/pull/1840
* js: Bound what the decoders read of the values apps pass by @ClarkThan in https://github.com/longbridge/nsh/pull/1954
* cli: Build an app's release runner with only the native features it reaches by @huacnlee in https://github.com/longbridge/nsh/pull/2014
* core: Lay out unstyled component hosts through to their one node by @huacnlee in https://github.com/longbridge/nsh/pull/2016
* ios: Report hardware keys from focused custom views by @honsunrise in https://github.com/longbridge/nsh/pull/1930
* cli: Name a cross-built Windows package's update archive for Windows by @huacnlee in https://github.com/longbridge/nsh/pull/2021
* windows: Preserve alpha from the first composed window frame by @Amamiya23 in https://github.com/longbridge/nsh/pull/1975
* js: Keep a component's children apart when shown beside its own branch by @daoleno in https://github.com/longbridge/nsh/pull/1941
* macos: Keep scaled inputs aligned inside clipped content by @0xbentang in https://github.com/longbridge/nsh/pull/2023
* bundler: Let an If's else expression follow what it reads by @daoleno in https://github.com/longbridge/nsh/pull/1943
* windows: Clear a partial frame's areas again under an opaque background by @madcodelife in https://github.com/longbridge/nsh/pull/2020
* cli: Build a TSX app's release runner with only the native features it reaches by @huacnlee in https://github.com/longbridge/nsh/pull/2022
* text: Preserve leading BOMs in Apple text measurement by @huacnlee in https://github.com/longbridge/nsh/pull/2026
* gpui: Load the next build from scratch after a hot update throws by @huacnlee in https://github.com/longbridge/nsh/pull/2030
* web: Keep focus when zIndex restacks a view by @huacnlee in https://github.com/longbridge/nsh/pull/2015
* testing: Press a Select's trigger by its testID, and code unsupported key holds by @huacnlee in https://github.com/longbridge/nsh/pull/2034
* docs: Show how to rebuild a branch per value without a keyed If by @huacnlee in https://github.com/longbridge/nsh/pull/2031
* touch: Report the modifier keys held at the press on responder events by @huacnlee in https://github.com/longbridge/nsh/pull/2018
* web: Keep the nodes of the lines a growing text did not change by @sunfuze in https://github.com/longbridge/nsh/pull/2038
* web: Stop writing animated values into views that left the page by @sunfuze in https://github.com/longbridge/nsh/pull/2037
* gpui: Keep the host's focus and pass unhandled scrolls on when embedded by @huacnlee in https://github.com/longbridge/nsh/pull/2032
* android: Report hardware keys and preserve native input ownership by @honsunrise in https://github.com/longbridge/nsh/pull/1931
* list: Keep the rows on screen when a keyed update changes both ends by @daoleno in https://github.com/longbridge/nsh/pull/2042
* xtask: Enable WebSockets in compatibility checks by @0xbentang in https://github.com/longbridge/nsh/pull/2047
* animation: Accept animated bounds in diffClamp by @huacnlee in https://github.com/longbridge/nsh/pull/2040
* macos: Confirm system quit and main-window close requests by @0xbentang in https://github.com/longbridge/nsh/pull/2050
* android: Keep the keyboard's timing when a back swipe dismisses it by @lurenjia534 in https://github.com/longbridge/nsh/pull/2053
* navigation: Fit a native header's title view between the bar's items by @huacnlee in https://github.com/longbridge/nsh/pull/2052
* input: Let a wheel listener with a pan axis take only wheels along it by @huacnlee in https://github.com/longbridge/nsh/pull/2049
* animation: Add ViewRef.animateLayout and Bezier layout curves, keep list updates out of scoped animations by @huacnlee in https://github.com/longbridge/nsh/pull/2051
* windows: Copy selected text while an input keeps focus by @ArnoChenFx in https://github.com/longbridge/nsh/pull/2058
* ios: Set input insets before the first draw by @daydoing in https://github.com/longbridge/nsh/pull/2062
* menu: Let the app own whether a `Submenu` is open by @madcodelife in https://github.com/longbridge/nsh/pull/2044
* ios: Leave the presses inside a focusable view to their views by @dXnSix in https://github.com/longbridge/nsh/pull/2063
* core: Keep telling a failed memo's readers of the changes after it by @daoleno in https://github.com/longbridge/nsh/pull/2065
* macos: Show the app's icon in the Dock under nsh run by @Joy-jinyu in https://github.com/longbridge/nsh/pull/2068
* macos: Require a whole canvas paint before partial slot redraws by @blueberrycongee in https://github.com/longbridge/nsh/pull/2069
* cli: Build an Android app's libnsh.so with only the native features it reaches by @huacnlee in https://github.com/longbridge/nsh/pull/2060
* text: Accept em and percentage font sizes, warn on span padding by @huacnlee in https://github.com/longbridge/nsh/pull/2061
* testing: Read a view's background and opacity with app.viewStyle by @huacnlee in https://github.com/longbridge/nsh/pull/2039
* windows: Clear partial frames' areas in opaque windows again by @huacnlee in https://github.com/longbridge/nsh/pull/2027
* ci: Update actions/checkout to v6 by @huacnlee in https://github.com/longbridge/nsh/pull/2048
* menu: Leave the focus where a chosen item's handler moved it by @madcodelife in https://github.com/longbridge/nsh/pull/2024
* rich_editor: Keep inline node views' position and selection current by @huacnlee in https://github.com/longbridge/nsh/pull/2077
* menu: Let screen readers read a menu item's description by @madcodelife in https://github.com/longbridge/nsh/pull/2043
* sheet: Make the sheet a named modal dialog on every platform by @madcodelife in https://github.com/longbridge/nsh/pull/2028
* core: Leave Return to a dialog over a focused control it covers by @madcodelife in https://github.com/longbridge/nsh/pull/2029
* data_table: Let the app select several cells and columns by @xhofe in https://github.com/longbridge/nsh/pull/2059
* core: Restyle only siblings whose position changed by @lurenjia534 in https://github.com/longbridge/nsh/pull/1596
* engine: Open the shader device without reserving memory it never uses by @labcoder in https://github.com/longbridge/nsh/pull/2083
* rich_editor: Render an extension's atoms with the app's component (addNodeView) by @huacnlee in https://github.com/longbridge/nsh/pull/2079
* ios: Keep video playing in the background with playback audio sessions by @huacnlee in https://github.com/longbridge/nsh/pull/2074
* ios: Report selectable view bounds as the selection moves by @huacnlee in https://github.com/longbridge/nsh/pull/2075
* cli: Build a TSX app's iOS runner with only the features it uses by @huacnlee in https://github.com/longbridge/nsh/pull/2076
* selection: Add the app's items to a selectable view's selection menu by @huacnlee in https://github.com/longbridge/nsh/pull/2080
* core: Leave the focus where any popup's choice moved it by @madcodelife in https://github.com/longbridge/nsh/pull/2082
* core: Report reactive horizontal and vertical size classes by @huacnlee in https://github.com/longbridge/nsh/pull/2081
* engine: Open the shader device on Direct3D 12 first on Windows by @labcoder in https://github.com/longbridge/nsh/pull/2084
* core: Batch reconciliation cleanup after node releases by @KeepNoob in https://github.com/longbridge/nsh/pull/2070
* terminal: Apply resizes once the size settles during a drag by @huacnlee in https://github.com/longbridge/nsh/pull/2086
* rich_editor: Give node views the RichEditor's context under QuickJS by @huacnlee in https://github.com/longbridge/nsh/pull/2087
* linux: Take the keyboard focus away with a removed view by @t4t5 in https://github.com/longbridge/nsh/pull/2092
* windows: Place the IME's candidates by the window a field shows in by @labcoder in https://github.com/longbridge/nsh/pull/2094
* linux: Report an Input's onPress by @t4t5 in https://github.com/longbridge/nsh/pull/1988
* js: Charge a call the time its thread runs, not the time the system kept it frozen by @daoleno in https://github.com/longbridge/nsh/pull/2099
* split_view: Add adaptive columns with independent navigation stacks by @huacnlee in https://github.com/longbridge/nsh/pull/2096
* text_view: Scan code spans once in streaming repair by @lurenjia534 in https://github.com/longbridge/nsh/pull/2100
* gpui: Keep multiline input wheel gestures on their field by @huacnlee in https://github.com/longbridge/nsh/pull/2090
* terminal: Send input method text only after composition ends by @huacnlee in https://github.com/longbridge/nsh/pull/2095
* android: Defer focusing an input until it is shown in a focused window by @huacnlee in https://github.com/longbridge/nsh/pull/2097
* testing: Focus an input that asks for it when it mounts by @madcodelife in https://github.com/longbridge/nsh/pull/2102
* list: Keep the cells of keyed rows that move, and of the rows they pass by @madcodelife in https://github.com/longbridge/nsh/pull/2078
* gpui: Keep native components' accessibility nodes in the tree by @lurenjia534 in https://github.com/longbridge/nsh/pull/2093
* core: Send a window's attributes before the commit's requests to it by @labcoder in https://github.com/longbridge/nsh/pull/2111
* ios: Press the views a page puts in its header's slots by @madcodelife in https://github.com/longbridge/nsh/pull/2117
* ios: Act on the first tap while the software keyboard is up by @madcodelife in https://github.com/longbridge/nsh/pull/2118
* core: Wrap a padded text at its content box when its width is known by @madcodelife in https://github.com/longbridge/nsh/pull/2119
* core: Leave a header slot that shows nothing out of the native bar by @madcodelife in https://github.com/longbridge/nsh/pull/2120
* core: Send window attributes only to surfaces that are in a window by @madcodelife in https://github.com/longbridge/nsh/pull/2123
* ios: Keep a page's scroll position when a view in its header is pressed by @madcodelife in https://github.com/longbridge/nsh/pull/2124
* ios: Scroll views into view in the test agent without flinging past them by @madcodelife in https://github.com/longbridge/nsh/pull/2125
* input: Leave taps to the view around one that listens only to onLongPress by @huacnlee in https://github.com/longbridge/nsh/pull/2105
* list: Test that rows re-measure when narrowed inside animate_layout by @huacnlee in https://github.com/longbridge/nsh/pull/2114
* webview: Build Windows web views outside the app update by @huacnlee in https://github.com/longbridge/nsh/pull/2116
* menu: Move to an item by typing its first letters by @madcodelife in https://github.com/longbridge/nsh/pull/2109
* macos: Refresh effects when nested shader frames arrive by @huacnlee in https://github.com/longbridge/nsh/pull/2108
* windows: Break overlong words instead of overflowing the line by @huacnlee in https://github.com/longbridge/nsh/pull/2104
* ios: Build release runtimes with the iOS 26 SDK or newer by @huacnlee in https://github.com/longbridge/nsh/pull/2089
* macos: Keep glass dialogs live while fading out by @huacnlee in https://github.com/longbridge/nsh/pull/2107
* android: Scroll nested Scrolls in a pager and blur late backdrops by @huacnlee in https://github.com/longbridge/nsh/pull/2106
* text: Let span horizontal padding take room in its line by @huacnlee in https://github.com/longbridge/nsh/pull/2141
* cli: Check the nsh version an app's nsh.json requires by @huacnlee in https://github.com/longbridge/nsh/pull/2139
* windows: Press the focused control with Return and Space by @huacnlee in https://github.com/longbridge/nsh/pull/2132
* windows: Draw caption buttons with the system icon font by @huacnlee in https://github.com/longbridge/nsh/pull/2133
* rich_editor: Answer nodeRect for inline nodes by @huacnlee in https://github.com/longbridge/nsh/pull/2131
* android: Verify WebSocket TLS with the system trust store by @huacnlee in https://github.com/longbridge/nsh/pull/2134
* core: Store a node's view and text styles only when set by @huacnlee in https://github.com/longbridge/nsh/pull/2144
* bundler: Compile TypeScript enums instead of panicking by @Tzyito in https://github.com/longbridge/nsh/pull/2147
* rich_editor: Report composition state transitions by @KeepNoob in https://github.com/longbridge/nsh/pull/2072
* runtime: Hand the app's address off to the system on iOS and macOS by @chaxus in https://github.com/longbridge/nsh/pull/2150
* dev: Refuse file requests under names rebound to this machine by @ClarkThan in https://github.com/longbridge/nsh/pull/2153
* notifications: Play an app's own sound with a notification by @huacnlee in https://github.com/longbridge/nsh/pull/2140
* core: Report pen pressure, tilt and pointer type on touches and mouse events by @huacnlee in https://github.com/longbridge/nsh/pull/2142
* core: Bind a component's keys in the window JS places it in by @madcodelife in https://github.com/longbridge/nsh/pull/2127
* android: Build the runtime again after notification sounds by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/2158
* android: Keep a scrollTo the content does not reach yet until it does by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/2159
* ios: Restore hardware shortcuts after keyboard dismissal by @huacnlee in https://github.com/longbridge/nsh/pull/2115
* rich_editor: Add onContentSizeChange by @KeepNoob in https://github.com/longbridge/nsh/pull/2071
* core: Stop assuming built-in backends' features on external platforms by @richerfu in https://github.com/longbridge/nsh/pull/1676
* image: Show one frame of multi-size ICO, ICNS and TIFF images on Apple platforms by @madcodelife in https://github.com/longbridge/nsh/pull/2151
* package: Include iOS toolchain metadata and arm64 capability by @huacnlee in https://github.com/longbridge/nsh/pull/2137
* macos: Keep incomplete IME text from crashing an app by @ttizze in https://github.com/longbridge/nsh/pull/1821
* text_view: Read deeply nested Markdown without running the stack out by @ClarkThan in https://github.com/longbridge/nsh/pull/2157
* mobile: Return local copies from pick_paths by @huacnlee in https://github.com/longbridge/nsh/pull/2138
* install: Show download progress and step spinners by @huacnlee in https://github.com/longbridge/nsh/pull/2161
* macos: Share opaque RGBA pixels with CoreGraphics by @huacnlee in https://github.com/longbridge/nsh/pull/2162
* text: Stand a view holding text on its line by that text's baseline by @Tzyito in https://github.com/longbridge/nsh/pull/2163
* rich_editor: Let plugins made in TS take part in a paste by @Tzyito in https://github.com/longbridge/nsh/pull/2154
* windows: Draw shader frames from textures shared with Direct2D's device by @labcoder in https://github.com/longbridge/nsh/pull/2126
* ci: Read nsh component list from stderr in the Homebrew test by @huacnlee in https://github.com/longbridge/nsh/pull/2129
* core: Place a follower made after layout on its target by @richerfu in https://github.com/longbridge/nsh/pull/1834
* text_view: Report presses on task items' boxes by @ttizze in https://github.com/longbridge/nsh/pull/2045
* text_view: Keep formulas in a selection's Markdown and HTML by @huacnlee in https://github.com/longbridge/nsh/pull/2130
* windows: Make WebView2 and media optional features by @huacnlee in https://github.com/longbridge/nsh/pull/2135
* rich_editor: Select an inline node view's node when it is pressed by @Tzyito in https://github.com/longbridge/nsh/pull/2149
* ios: Keep the safe area clear of an iPad window's controls by @tacshi in https://github.com/longbridge/nsh/pull/2160
* tabs: Accept an accessibility description on `tab()` by @madcodelife in https://github.com/longbridge/nsh/pull/2156
* web: Run packaged apps under a strict Content-Security-Policy by @madcodelife in https://github.com/longbridge/nsh/pull/2155
* core: Take clipboard images from the OS unless a platform says otherwise by @madcodelife in https://github.com/longbridge/nsh/pull/2164
* ios: Accept a nil position or range from UIKit's text input calls by @madcodelife in https://github.com/longbridge/nsh/pull/2165
* text: Take a view's baseline only from text in its flow by @Tzyito in https://github.com/longbridge/nsh/pull/2173
* text_view: Read a streamed reply from where it can still change by @lurenjia534 in https://github.com/longbridge/nsh/pull/2136
* menu: Open anchored on an iPad in a regular width by @tacshi in https://github.com/longbridge/nsh/pull/2172
* rich_editor: Paste HTML and text from JS as a paste would by @Tzyito in https://github.com/longbridge/nsh/pull/2176
* macos: Keep a masked box shadow's corners from showing a dark arc by @huacnlee in https://github.com/longbridge/nsh/pull/2174
* windows: Show a transition's own start picture until its first frame by @labcoder in https://github.com/longbridge/nsh/pull/2184
* core: Restore scroll behavior when views mount by @richerfu in https://github.com/longbridge/nsh/pull/2188
* core: Keep restored modal focus from reopening tooltips by @yuezk in https://github.com/longbridge/nsh/pull/2190
* text: Keep fontFamily fallback lists in CanvasFont and measure_text by @jjyr in https://github.com/longbridge/nsh/pull/2169
* ios: Serve all of nsh/base in the iOS SDK by default by @Cooooooker in https://github.com/longbridge/nsh/pull/2193
* testing: Type into a focused input without pressing it first by @huacnlee in https://github.com/longbridge/nsh/pull/2186
* windows: Compose the development host's window so a later backdrop shows by @huacnlee in https://github.com/longbridge/nsh/pull/2189
* scrollbar: Pass pointer events through when content fits by @jjyr in https://github.com/longbridge/nsh/pull/2195
* editor: Keep built-in scrollbars off the caret sink by @jjyr in https://github.com/longbridge/nsh/pull/2197
* docs: Keep agent instructions focused and move lookup references by @huacnlee in https://github.com/longbridge/nsh/pull/2204
* windows: Draw a shader fill's new uniforms only when their frame lands by @labcoder in https://github.com/longbridge/nsh/pull/2203
* core: Take a list to its end with one press of End by @duo in https://github.com/longbridge/nsh/pull/2192
* windows: Let a topmost level take under the foreground window by @labcoder in https://github.com/longbridge/nsh/pull/2207
* linux: Let apps save their state when the session ends by @madcodelife in https://github.com/longbridge/nsh/pull/2213
* selection: Keep a hostname, URL or path whole when a long press selects a word by @daoleno in https://github.com/longbridge/nsh/pull/2209
* sheet: Rest bottom sheets at detents with an optional grabber by @madcodelife in https://github.com/longbridge/nsh/pull/2194
* style: Add direct Rust gradient mask methods by @madcodelife in https://github.com/longbridge/nsh/pull/2191
* testing: Read Editor document values and reject covered input edits by @huacnlee in https://github.com/longbridge/nsh/pull/2205
* web: Place text runs by the DOM's baseline, not canvas's rounded font box by @davo20019 in https://github.com/longbridge/nsh/pull/2216
* text: Let a span's vertical padding extend its background by @davo20019 in https://github.com/longbridge/nsh/pull/2215
* runtime: Add `Share` to show the system's share sheet by @madcodelife in https://github.com/longbridge/nsh/pull/2198
* windows: Let a native component bring a composition visual by @labcoder in https://github.com/longbridge/nsh/pull/2200
* context_menu: Show the system's context menu on iOS for `items` by @madcodelife in https://github.com/longbridge/nsh/pull/2202
* web: Keep a touch responder's touches from the browser's scrolling by @davo20019 in https://github.com/longbridge/nsh/pull/2220
* updater: Report shell-only updates from checkForUpdate by @huacnlee in https://github.com/longbridge/nsh/pull/2222
* gpui: Build on gpui-fast 0.1.16 by @huacnlee in https://github.com/longbridge/nsh/pull/2221
* core: Skip a waiting view whose parent is gone instead of panicking by @sunfuze in https://github.com/longbridge/nsh/pull/2229
* core: Let layout animation springs take stiffness, damping and mass by @RebelBIrd in https://github.com/longbridge/nsh/pull/2227
* windows: Restore a minimized window at the size it had by @sekzerd in https://github.com/longbridge/nsh/pull/2225
* base: Use the default cursor on controls, keep the pointing hand for links by @huacnlee in https://github.com/longbridge/nsh/pull/2224
* collapsible: Let the app give the reveal's spring by @RebelBIrd in https://github.com/longbridge/nsh/pull/2233
* gallery: Read perf demos from the index and add --log by @huacnlee in https://github.com/longbridge/nsh/pull/2211
* perf: Add a startup, process-tree and size sampler for baselines by @huacnlee in https://github.com/longbridge/nsh/pull/2210
* core: Play motions on a spring that keeps velocity when interrupted by @RebelBIrd in https://github.com/longbridge/nsh/pull/2234
* windows: Open the focus' context menu on the Menu key and Shift+F10 by @sunli829 in https://github.com/longbridge/nsh/pull/2237
* macos: Keep the main thread free when a page shows new content by @huacnlee in https://github.com/longbridge/nsh/pull/2235
* windows: Report an Input's onPress so a click opens a Combobox by @sunli829 in https://github.com/longbridge/nsh/pull/2236
* windows: Draw a dark page's backdrop in Windows 11's window color by @sunli829 in https://github.com/longbridge/nsh/pull/2238
* windows: Draw a dashed or dotted border only on the sides that have one by @sunli829 in https://github.com/longbridge/nsh/pull/2239
* switch: Style the track and thumb as pressed while the switch is held by @RebelBIrd in https://github.com/longbridge/nsh/pull/2240
* updater: Let checkForUpdate pick a named channel the package declares by @sunfuze in https://github.com/longbridge/nsh/pull/2249
* desktop: Let an app name its own data directory by @jesseXu in https://github.com/longbridge/nsh/pull/2232
* core: Defer native updates until a view is created by @richerfu in https://github.com/longbridge/nsh/pull/2252
* rich_editor: Place the bubble menu and the input by a selected block's box by @Tzyito in https://github.com/longbridge/nsh/pull/2246
* windows: Shrink large animations to fit instead of dropping to one frame by @huacnlee in https://github.com/longbridge/nsh/pull/2258
* list: Keep the place while a row near the end reveals under follow_end by @huacnlee in https://github.com/longbridge/nsh/pull/2260
* testing: Place a system header's items side by side in headless runs by @madcodelife in https://github.com/longbridge/nsh/pull/2265
* collapsible: Check revealSpring with margins a loaded machine keeps by @RebelBIrd in https://github.com/longbridge/nsh/pull/2269
* web: Keep each app's realm frame in the document when the page moves its player by @Liguoqiang-code in https://github.com/longbridge/nsh/pull/2262
* macos: Keep dataDir in a package of the prebuilt runner by @jesseXu in https://github.com/longbridge/nsh/pull/2261
* web: Show anchored overlays over the window with overlays: "viewport" by @Tzyito in https://github.com/longbridge/nsh/pull/2257
* macos: Restore editor focus after a dialog closes by @jjyr in https://github.com/longbridge/nsh/pull/2267
* updater: Add installOnQuit to install the downloaded update as the app quits by @sunfuze in https://github.com/longbridge/nsh/pull/2263
* dialog: Rest iOS sheets at detents with a grabber and a stack inside by @madcodelife in https://github.com/longbridge/nsh/pull/2243
* resizable: Restore a panel's size when a drag brings it back to its size at the press by @huacnlee in https://github.com/longbridge/nsh/pull/2255
* cli: Refresh runtime components left from another release by @huacnlee in https://github.com/longbridge/nsh/pull/2254
* core: Make an update of an each cheap for the rows that stay by @lurenjia534 in https://github.com/longbridge/nsh/pull/2268
* dialog: Make an iOS sheet's glass follow the app's appearance by @huacnlee in https://github.com/longbridge/nsh/pull/2253
* windows: Report and place a window by the frame that shows by @sunli829 in https://github.com/longbridge/nsh/pull/2270
* input: Count `maxLength` in characters, as its docs say by @madcodelife in https://github.com/longbridge/nsh/pull/2266
* gallery: Show shortcuts in the platform's own keys by @sunli829 in https://github.com/longbridge/nsh/pull/2271
* canvas: Draw SVG images sharp at the screen's scale by @sunli829 in https://github.com/longbridge/nsh/pull/2272
* gallery: Fix small copy and layout slips in the demos by @sunli829 in https://github.com/longbridge/nsh/pull/2273
* input: Keep macOS text aligned when focusing by @huacnlee in https://github.com/longbridge/nsh/pull/2264
* android: Go back without recording the revealed page again by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/2280
* mobile: Add `DateTimePicker`, the system's date and time picker by @madcodelife in https://github.com/longbridge/nsh/pull/2278
* core: Run style transitions on a spring by @RebelBIrd in https://github.com/longbridge/nsh/pull/2259
* core: Store rare element attributes out of line by @blueberrycongee in https://github.com/longbridge/nsh/pull/2277
* macos: Preserve active feedback for trackpad taps by @huacnlee in https://github.com/longbridge/nsh/pull/2275
* rich_editor: Update only the blocks an edit or selection touches by @blueberrycongee in https://github.com/longbridge/nsh/pull/2276
* windows: Draw one set of caption buttons on a maximized window with a backdrop by @Blackman99 in https://github.com/longbridge/nsh/pull/2282
* windows: Apply an input's text alpha to what RichEdit draws by @hexuria in https://github.com/longbridge/nsh/pull/2283
* runtime: Add `open_auth_session` for web sign-in sessions by @madcodelife in https://github.com/longbridge/nsh/pull/2285
* data_table: Skip unstyled edge views and idle cell bindings by @blueberrycongee in https://github.com/longbridge/nsh/pull/2279
* mobile: Add `Switch`, the system's switch by @madcodelife in https://github.com/longbridge/nsh/pull/2284
* macos: Keep composition keys in hosted input clients by @jjyr in https://github.com/longbridge/nsh/pull/2288
* macos: Size canvas content layers to their painted bounds by @lnnduy in https://github.com/longbridge/nsh/pull/2250
* package: Give iOS packages a launch screen color and image from nsh.json by @madcodelife in https://github.com/longbridge/nsh/pull/2293
* input: Focus the field when a padding click can't reach its editor by @huacnlee in https://github.com/longbridge/nsh/pull/2287
* updater: Carry release notes in more than one language by @sunfuze in https://github.com/longbridge/nsh/pull/2286
* alert_dialog: Let an app close an alert it showed by @madcodelife in https://github.com/longbridge/nsh/pull/2294
* mobile: Name `DateTimePicker` for screen readers with `accessibilityLabel` by @madcodelife in https://github.com/longbridge/nsh/pull/2300
* context_menu: Lift an app-made preview beside the iOS system menu by @madcodelife in https://github.com/longbridge/nsh/pull/2297
* text_view: Measure words fading into a long paragraph by @madcodelife in https://github.com/longbridge/nsh/pull/2303
* auth_session: Let an app close its sign-in session by @madcodelife in https://github.com/longbridge/nsh/pull/2304
* linux: Keep installed desktop entries when registering URL schemes by @madcodelife in https://github.com/longbridge/nsh/pull/2305
* list: Lay cells out at their new width when only the columns change by @madcodelife in https://github.com/longbridge/nsh/pull/2306
* windows: Report multiline input overflow to scrollbars by @yuezk in https://github.com/longbridge/nsh/pull/2302
* web: Let a swipe across a one-axis scroll view scroll the view around it by @davo20019 in https://github.com/longbridge/nsh/pull/2309
* auth_session: Register the iOS presentation anchor with its protocol by @madcodelife in https://github.com/longbridge/nsh/pull/2310
* list: Show each row in one cell, spare cells included by @madcodelife in https://github.com/longbridge/nsh/pull/2312
* ios: Let tests press bar buttons and choose from system menus by @madcodelife in https://github.com/longbridge/nsh/pull/2313
* macos: Open a window at the default position the app gives it by @teng-lin in https://github.com/longbridge/nsh/pull/2325
* linux: Leave the stop event out of the touchpad release velocity by @huacnlee in https://github.com/longbridge/nsh/pull/2328
* docs: Document the minimum browsers for published web apps by @huacnlee in https://github.com/longbridge/nsh/pull/2296
* macos: Move the window from windowDrag views and a hidden title bar by @huacnlee in https://github.com/longbridge/nsh/pull/2295
* data_table: Hover the views renderCell draws as the pointer moves over them by @RebelBIrd in https://github.com/longbridge/nsh/pull/2223
* android: Let Android Lint pass on the Java runtime a host compiles by @teng-lin in https://github.com/longbridge/nsh/pull/2329
* release: Prepare version 0.6.0 by @huacnlee in https://github.com/longbridge/nsh/pull/2311

</details>

<details><summary>New Contributors</summary>

* @mliemm made their first contribution in https://github.com/longbridge/nsh/pull/1694
* @crizant made their first contribution in https://github.com/longbridge/nsh/pull/1721
* @jsiu93 made their first contribution in https://github.com/longbridge/nsh/pull/1844
* @hexiang1993 made their first contribution in https://github.com/longbridge/nsh/pull/1843
* @subframe7536 made their first contribution in https://github.com/longbridge/nsh/pull/1816
* @wenlingang made their first contribution in https://github.com/longbridge/nsh/pull/1849
* @linyisu made their first contribution in https://github.com/longbridge/nsh/pull/1929
* @tacshi made their first contribution in https://github.com/longbridge/nsh/pull/1890
* @blueberrycongee made their first contribution in https://github.com/longbridge/nsh/pull/1758
* @jcdiv47 made their first contribution in https://github.com/longbridge/nsh/pull/1948
* @villatothesea made their first contribution in https://github.com/longbridge/nsh/pull/1973
* @Amamiya23 made their first contribution in https://github.com/longbridge/nsh/pull/1992
* @Liguoqiang-code made their first contribution in https://github.com/longbridge/nsh/pull/1840
* @ArnoChenFx made their first contribution in https://github.com/longbridge/nsh/pull/2058
* @xhofe made their first contribution in https://github.com/longbridge/nsh/pull/2059
* @labcoder made their first contribution in https://github.com/longbridge/nsh/pull/2083
* @KeepNoob made their first contribution in https://github.com/longbridge/nsh/pull/2070
* @jjyr made their first contribution in https://github.com/longbridge/nsh/pull/2169
* @davo20019 made their first contribution in https://github.com/longbridge/nsh/pull/2216
* @sekzerd made their first contribution in https://github.com/longbridge/nsh/pull/2225
* @jesseXu made their first contribution in https://github.com/longbridge/nsh/pull/2232
* @lnnduy made their first contribution in https://github.com/longbridge/nsh/pull/2250

</details>

**Full Changelog**: https://github.com/longbridge/nsh/compare/v0.5.0...v0.6.0
