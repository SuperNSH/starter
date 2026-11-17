# NSH 0.5.1

NSH 0.5.1 is a patch release of 0.5.0 with bug fixes only: no new features and no API changes, so apps built for 0.5.0 run unchanged. It collects the fixes made on `main` since 0.5.0 that are safe to take into the 0.5 line.

Most of the fixes concern details people meet every day: lists and scroll views that keep their place, text input that behaves as the platform's own, overlays and dialogs that keep the focus where it belongs, and Windows windows that redraw cleanly. The runtime also bounds what it reads from app values and returns memory it no longer needs.

## Layout, lists, and scrolling

- **Views made late get their layout at once.** Inputs, scroll views, and paced views in list cells created after layout receive their content box, extent, and frame when they are made; a culled scroll view has its content size before its first scroll; rows in a list cell that waited appear at once. ([#1682](https://github.com/longbridge/nsh/pull/1682), [#1794](https://github.com/longbridge/nsh/pull/1794), [#1800](https://github.com/longbridge/nsh/pull/1800), [#1823](https://github.com/longbridge/nsh/pull/1823), [#1812](https://github.com/longbridge/nsh/pull/1812))
- **Lists hold their place.** The first visible row stays where it is while rows above it are measured, sticky rows stay below the list's built-in scroll bar, and JS work queued before binding the overscan is applied first. ([#1704](https://github.com/longbridge/nsh/pull/1704), [#1909](https://github.com/longbridge/nsh/pull/1909), [#1861](https://github.com/longbridge/nsh/pull/1861))
- **Text and inline views.** A padded label stays on one line at fractional pixel ratios, inline views sit on their wrapped line in headless layout, views inside text show their content in a scroll view, and views nested in culled inline attachments can be revealed. ([#1705](https://github.com/longbridge/nsh/pull/1705), [#1787](https://github.com/longbridge/nsh/pull/1787), [#1948](https://github.com/longbridge/nsh/pull/1948), [#1742](https://github.com/longbridge/nsh/pull/1742))
- **Styles and hit testing.** `hover:` and `active:` styles reach a nested `Text` under the pointer, headers and list content resolve their classes again when design tokens change, a child's hit slop takes touches outside its parent, a multiline input's parent stops at the input's maximum height, and unstyled component hosts lay out through to their single node. ([#1859](https://github.com/longbridge/nsh/pull/1859), [#1769](https://github.com/longbridge/nsh/pull/1769), [#1751](https://github.com/longbridge/nsh/pull/1751), [#1788](https://github.com/longbridge/nsh/pull/1788), [#2016](https://github.com/longbridge/nsh/pull/2016))

## Overlays, focus, and accessibility

- **Overlays and dialogs.** Anchored overlays lay out within the room they are placed in, a submenu's first item lines up with its trigger, `HoverCard` follows the pointer only, bottom sheets and menus stay above the software keyboard, and dragging a sheet ends editing. On the web, an open dialog is removed with the subtree that held it; on Windows, a click on a `windowDrag` view closes overlays. ([#1938](https://github.com/longbridge/nsh/pull/1938), [#1825](https://github.com/longbridge/nsh/pull/1825), [#1695](https://github.com/longbridge/nsh/pull/1695), [#1872](https://github.com/longbridge/nsh/pull/1872), [#1856](https://github.com/longbridge/nsh/pull/1856), [#1693](https://github.com/longbridge/nsh/pull/1693), [#1658](https://github.com/longbridge/nsh/pull/1658))
- **Focus.** Removing an element with the focus inside reports the focus leaving, and macOS keeps the keyboard focus out of hidden views. ([#1980](https://github.com/longbridge/nsh/pull/1980), [#1832](https://github.com/longbridge/nsh/pull/1832))
- **Screen readers and alerts.** macOS reads an alert as it appears and as it changes, and shows alerts in front while the app's window is out of sight; Linux moves the screen reader to views that take no keyboard focus; the web keeps the typing field's focus when the screen reader moves. ([#1699](https://github.com/longbridge/nsh/pull/1699), [#1697](https://github.com/longbridge/nsh/pull/1697), [#1851](https://github.com/longbridge/nsh/pull/1851), [#1848](https://github.com/longbridge/nsh/pull/1848))

## Text input

- **Desktop.** macOS clears stale undo actions before editor changes, keeps a growing multiline input's caret in view, and keeps typed quotes and dashes when `autoCorrect` is off. Windows draws input text at the target's scale, scrolls multiline inputs with the wheel, and reports what an input method writes through TSF as it writes it. GPUI blurs the focused field once the mouse starts selecting text. ([#1694](https://github.com/longbridge/nsh/pull/1694), [#1921](https://github.com/longbridge/nsh/pull/1921), [#1978](https://github.com/longbridge/nsh/pull/1978), [#1816](https://github.com/longbridge/nsh/pull/1816), [#1976](https://github.com/longbridge/nsh/pull/1976), [#1994](https://github.com/longbridge/nsh/pull/1994), [#1968](https://github.com/longbridge/nsh/pull/1968))
- **Mobile and web.** Android 15 keeps input text centered with natural carets and short text on one line; an input in a dialog dismisses its keyboard on Android; iOS keeps a long-press selection when the touch ends; the web turns autocorrection off with `autoCorrect={false}`. ([#1929](https://github.com/longbridge/nsh/pull/1929), [#1871](https://github.com/longbridge/nsh/pull/1871), [#2001](https://github.com/longbridge/nsh/pull/2001), [#1806](https://github.com/longbridge/nsh/pull/1806), [#1883](https://github.com/longbridge/nsh/pull/1883))
- **Password inputs** keep their behavior when the password is revealed. ([#1828](https://github.com/longbridge/nsh/pull/1828))

## Components

- **`RichEditor`.** An editor without a height sizes to its content, task items' text sits where it is drawn, the sink takes the root padding and fills the viewport, Enter on a checked task starts an unchecked one, a checkbox press survives a pointer that moves as it lands, code blocks show the selection, and the caret and presses follow an inline view's width. ([#1690](https://github.com/longbridge/nsh/pull/1690), [#1706](https://github.com/longbridge/nsh/pull/1706), [#1702](https://github.com/longbridge/nsh/pull/1702), [#1797](https://github.com/longbridge/nsh/pull/1797), [#1798](https://github.com/longbridge/nsh/pull/1798), [#1837](https://github.com/longbridge/nsh/pull/1837), [#1894](https://github.com/longbridge/nsh/pull/1894))
- **`Editor`, `TextView`, and `DiffView`.** The editor measures in the user's text size so rows fit their text; `TextView` keeps a bare URL in a link's text as text and sizes the app's own images against the text's width; `DiffView` keeps unwrapped lines on one row at larger text sizes. ([#1808](https://github.com/longbridge/nsh/pull/1808), [#1715](https://github.com/longbridge/nsh/pull/1715), [#1782](https://github.com/longbridge/nsh/pull/1782), [#1804](https://github.com/longbridge/nsh/pull/1804))
- **`Terminal` and `DataTable`.** The terminal sends Command's line editing keys as macOS terminals do, reports its first size even at 80 × 24, and keeps each character on its cells; scaled web scrollbars of `DataTable` stay stable at rest. ([#1633](https://github.com/longbridge/nsh/pull/1633), [#1638](https://github.com/longbridge/nsh/pull/1638), [#1649](https://github.com/longbridge/nsh/pull/1649), [#1721](https://github.com/longbridge/nsh/pull/1721))

## Windows

- **Clean partial frames.** Partial redraws no longer leave behind what was drawn before, in transparent and opaque windows alike; the first composed frame keeps its alpha, and views that cannot paint stay out of the frame. ([#1993](https://github.com/longbridge/nsh/pull/1993), [#1995](https://github.com/longbridge/nsh/pull/1995), [#1975](https://github.com/longbridge/nsh/pull/1975), [#2020](https://github.com/longbridge/nsh/pull/2020), [#1973](https://github.com/longbridge/nsh/pull/1973))
- **Window and app lifecycle.** Pressing a window's frame activates it, a closed app exits without tearing down its UI thread, a destroyed window releases its focused field before the thread exits, apps can save their state when the session ends, and the app goes to the background while the screen cannot be seen. ([#1717](https://github.com/longbridge/nsh/pull/1717), [#1992](https://github.com/longbridge/nsh/pull/1992), [#1801](https://github.com/longbridge/nsh/pull/1801), [#1744](https://github.com/longbridge/nsh/pull/1744), [#1867](https://github.com/longbridge/nsh/pull/1867))

## Other platforms

- **macOS.** Leaving full screen comes before hiding a window, traffic lights keep their spacing with a custom offset, and text redraws when a layout animation grows its height. ([#1772](https://github.com/longbridge/nsh/pull/1772), [#1907](https://github.com/longbridge/nsh/pull/1907), [#1758](https://github.com/longbridge/nsh/pull/1758))
- **Linux and the desktop runners.** Flatpak and Snap browsers are found without `XDG_DATA_DIRS`; the Windows and Linux runners leave an app's own logger in place. ([#1908](https://github.com/longbridge/nsh/pull/1908), [#1703](https://github.com/longbridge/nsh/pull/1703))
- **GPUI.** Images and icons turn with their rotated views, and a clamped text's last line is cut at its box rather than after a word. ([#1864](https://github.com/longbridge/nsh/pull/1864), [#1918](https://github.com/longbridge/nsh/pull/1918))
- **iOS.** Large titles show expanded with the content below them, a page stays below its header after a headerless page is popped, and a blocking responder's drag no longer swipes its page back. ([#1811](https://github.com/longbridge/nsh/pull/1811), [#1890](https://github.com/longbridge/nsh/pull/1890), [#1757](https://github.com/longbridge/nsh/pull/1757))
- **Android and mobile touch.** A zoomed `Scroll` in a pager pans before the pager pages, touches inside a scroll view survive scroll commands, a pan view keeps its drags from the scroll view under it, a tap goes through a scroll view once the keyboard is down, and only the tap is stopped in a scroll view with the keyboard up. A stack's screens take the app's own appearance, the development error screen stays clear of side insets, and Rust apps get the declared environment under `nsh run`. On the web, a pager turns its pages from a zoomable `Scroll` under the finger. ([#1732](https://github.com/longbridge/nsh/pull/1732), [#1714](https://github.com/longbridge/nsh/pull/1714), [#1745](https://github.com/longbridge/nsh/pull/1745), [#1755](https://github.com/longbridge/nsh/pull/1755), [#1792](https://github.com/longbridge/nsh/pull/1792), [#1950](https://github.com/longbridge/nsh/pull/1950), [#1898](https://github.com/longbridge/nsh/pull/1898), [#1958](https://github.com/longbridge/nsh/pull/1958), [#1887](https://github.com/longbridge/nsh/pull/1887))

## Runtime reliability and memory

- **Bounded reads of app values.** The runtime bounds the array items it reads natively and what its decoders read of the values apps pass. ([#1685](https://github.com/longbridge/nsh/pull/1685), [#1954](https://github.com/longbridge/nsh/pull/1954))
- **Memory and ownership.** A cancelled task's timer is dropped at once, the animation graph keeps storage only for live nodes, a list's empty cell is disposed with its list, a root no longer takes its freed owner's reused slot, and a removed dialog's dismiss handler stays outside cycle collection. ([#1866](https://github.com/longbridge/nsh/pull/1866), [#2005](https://github.com/longbridge/nsh/pull/2005), [#1990](https://github.com/longbridge/nsh/pull/1990), [#1991](https://github.com/longbridge/nsh/pull/1991), [#1698](https://github.com/longbridge/nsh/pull/1698))
- **Compiled JSX.** A component's children stay apart when shown beside its own branch, and an `If`'s else expression follows what it reads. ([#1941](https://github.com/longbridge/nsh/pull/1941), [#1943](https://github.com/longbridge/nsh/pull/1943))
- **Hosts and grants.** Calls stay on the page's host when the page passes parameters, and media files open under the host's narrower grant. ([#1840](https://github.com/longbridge/nsh/pull/1840), [#1677](https://github.com/longbridge/nsh/pull/1677))

## Developer tooling and tests

- **CLI and development.** The spinner line is drawn in one write so it no longer flickers, a cross-built Windows package's update archive is named for Windows, and a reconnecting host catches up with the hot updates it missed. ([#1733](https://github.com/longbridge/nsh/pull/1733), [#2021](https://github.com/longbridge/nsh/pull/2021), [#1983](https://github.com/longbridge/nsh/pull/1983))
- **Tests.** Headless runs break the line on Shift+Return in a submitting multiline input, keep Tab inside the newest dialog and give the focus back when it closes (as the desktops do), settle a step once the engine has handled it, avoid false cover errors for wholly clipped targets, and keep removed views from hearing hover and keys. The macOS test agent's presses land where it aims on a locked screen. ([#1679](https://github.com/longbridge/nsh/pull/1679), [#1696](https://github.com/longbridge/nsh/pull/1696), [#1771](https://github.com/longbridge/nsh/pull/1771), [#1862](https://github.com/longbridge/nsh/pull/1862), [#1735](https://github.com/longbridge/nsh/pull/1735), [#1750](https://github.com/longbridge/nsh/pull/1750), [#1729](https://github.com/longbridge/nsh/pull/1729))

## Upgrading from 0.5.0

Update the toolchain to 0.5.1 (`nsh update`) and rebuild; crates depending on `0.5` pick up 0.5.1 with `cargo update`. No code or configuration changes are needed. If your app worked around any of the behavior fixed here, check whether the workaround is still needed.

<details>
<summary>Contributors</summary>

Thanks to everyone who contributed to this release:

@0xbentang @ahonn @akongG @Amamiya23 @BeADre @Blackman99 @blueberrycongee @ClarkThan @Cooooooker @cosformula @crizant @daoleno @dXnSix @hmziqrs @huacnlee @jcdiv47 @kim0 @LeeDashan @Liguoqiang-code @linyisu @LVtomatoJ @madcodelife @mengyinghu-sys @mliem2k @obito-t @richerfu @subframe7536 @sunfuze @tacshi @ttizze @Tzyito @villatothesea @wenlingang @WillisXue @YougLin-dev

</details>

<details>
<summary>What's Changed</summary>

## What's Changed
* testing: Break the line on Shift+Return in a multiline input that submits by @madcodelife in https://github.com/longbridge/nsh/pull/1679
* windows: Close overlays on a click on a windowDrag view by @WillisXue in https://github.com/longbridge/nsh/pull/1658
* access: Open media files under the host's narrower grant by @huacnlee in https://github.com/longbridge/nsh/pull/1677
* rich_editor: Size an editor without a height to its content by @Cooooooker in https://github.com/longbridge/nsh/pull/1690
* web: Remove an open dialog with the subtree that held it by @Cooooooker in https://github.com/longbridge/nsh/pull/1693
* rich_editor: Put a task item's text where it is drawn by @Tzyito in https://github.com/longbridge/nsh/pull/1706
* testing: Keep Tab inside the newest dialog, as the desktops do by @madcodelife in https://github.com/longbridge/nsh/pull/1696
* macos: Read an alert as it appears and as its content changes by @madcodelife in https://github.com/longbridge/nsh/pull/1699
* list: Keep the first visible row in place as rows above it are measured by @madcodelife in https://github.com/longbridge/nsh/pull/1704
* macos: Clear stale text undo actions before editor changes by @mliem2k in https://github.com/longbridge/nsh/pull/1694
* cli: Draw the spinner line in one write so it stops flickering by @huacnlee in https://github.com/longbridge/nsh/pull/1733
* rich_editor: Inset the sink with root padding and fill the viewport by @richerfu in https://github.com/longbridge/nsh/pull/1702
* text_view: Keep a bare URL in a link's text as text by @madcodelife in https://github.com/longbridge/nsh/pull/1715
* windows: Activate the window when its frame is pressed by @YougLin-dev in https://github.com/longbridge/nsh/pull/1717
* windows: Let apps save their state when the session ends by @madcodelife in https://github.com/longbridge/nsh/pull/1744
* desktop: Leave an app's own logger in place in the Windows and Linux runners by @LeeDashan in https://github.com/longbridge/nsh/pull/1703
* android: Pan a zoomed Scroll in a pager before the pager pages by @Cooooooker in https://github.com/longbridge/nsh/pull/1732
* testing: Give the focus back when a dialog closes, as the desktops do by @madcodelife in https://github.com/longbridge/nsh/pull/1771
* terminal: Send Command's line editing keys as macOS terminals do by @Blackman99 in https://github.com/longbridge/nsh/pull/1633
* android: Keep the touches inside a scroll view through scroll commands by @mengyinghu-sys in https://github.com/longbridge/nsh/pull/1714
* text_view: Size an app's own images against the text's width by @madcodelife in https://github.com/longbridge/nsh/pull/1782
* testing: Keep removed views from hearing hover and keys in headless runs by @Cooooooker in https://github.com/longbridge/nsh/pull/1750
* data_table: Keep scaled web scrollbars stable at rest by @crizant in https://github.com/longbridge/nsh/pull/1721
* text: Place inline views on their wrapped line in headless layout by @madcodelife in https://github.com/longbridge/nsh/pull/1787
* rich_editor: Start the task Enter makes from a checked one unchecked by @Tzyito in https://github.com/longbridge/nsh/pull/1797
* rich_editor: Keep a task checkbox's press when the pointer moves as it lands by @Tzyito in https://github.com/longbridge/nsh/pull/1798
* core: Mount the pointer-events of a view made after they were worked out by @Tzyito in https://github.com/longbridge/nsh/pull/1812
* macos: Keep the keyboard focus out of hidden views by @0xbentang in https://github.com/longbridge/nsh/pull/1832
* ios: Show large titles expanded, with the content below them by @ahonn in https://github.com/longbridge/nsh/pull/1811
* editor: Measure in the user's text size, so rows fit their text by @daoleno in https://github.com/longbridge/nsh/pull/1808
* ios: Keep a long-press selection when the touch ends by @ttizze in https://github.com/longbridge/nsh/pull/1806
* core: Give a paced view in a list cell its frame when made by @huacnlee in https://github.com/longbridge/nsh/pull/1800
* core: Give a culled scroll view its content size before its first scroll by @obito-t in https://github.com/longbridge/nsh/pull/1794
* mobile: Stop only the tap in a scroll view with the keyboard up by @Cooooooker in https://github.com/longbridge/nsh/pull/1792
* core: Keep a padded label on one line at fractional pixel ratios by @cosformula in https://github.com/longbridge/nsh/pull/1705
* core: Give inputs and scroll views made late their content box and extent by @Blackman99 in https://github.com/longbridge/nsh/pull/1682
* js: Bound the array items the runtime reads natively by @ClarkThan in https://github.com/longbridge/nsh/pull/1685
* android: Keep a pan view's drags from the scroll view under it by @Cooooooker in https://github.com/longbridge/nsh/pull/1745
* web: Keep the typing field's focus when moving the screen reader's by @madcodelife in https://github.com/longbridge/nsh/pull/1848
* core: Make the rows that show in a list cell that waited at once by @LeeDashan in https://github.com/longbridge/nsh/pull/1823
* linux: Move the screen reader to views that take no keyboard focus by @madcodelife in https://github.com/longbridge/nsh/pull/1851
* menu: Line a submenu's first item up with its trigger by @madcodelife in https://github.com/longbridge/nsh/pull/1825
* core: Resolve a header's and a list content's classes again when design tokens change by @dXnSix in https://github.com/longbridge/nsh/pull/1769
* core: Let a child's hit slop take touches outside its parent by @madcodelife in https://github.com/longbridge/nsh/pull/1751
* macos: Leave full screen before hiding a window by @madcodelife in https://github.com/longbridge/nsh/pull/1772
* core: Stop a multiline input's parent at the input's maximum height by @sunfuze in https://github.com/longbridge/nsh/pull/1788
* rich_editor: Show the selection in code blocks by @huacnlee in https://github.com/longbridge/nsh/pull/1837
* windows: Draw input text at the target's scale by @subframe7536 in https://github.com/longbridge/nsh/pull/1816
* sheet: End editing when the user starts dragging a sheet by @Cooooooker in https://github.com/longbridge/nsh/pull/1856
* gpui: Turn images and icons with their rotated views by @Cooooooker in https://github.com/longbridge/nsh/pull/1864
* core: Apply hover: and active: styles to a nested Text under the pointer by @Cooooooker in https://github.com/longbridge/nsh/pull/1859
* engine: Apply what JS queued before binding a list's overscan by @madcodelife in https://github.com/longbridge/nsh/pull/1861
* testing: Settle a headless step once the engine has handled it by @madcodelife in https://github.com/longbridge/nsh/pull/1862
* core: Drop a cancelled task's timer at once by @madcodelife in https://github.com/longbridge/nsh/pull/1866
* windows: Release a destroyed window's focused field before thread exit by @huacnlee in https://github.com/longbridge/nsh/pull/1801
* web: Turn a pager's pages from a zoomable Scroll under the finger by @Cooooooker in https://github.com/longbridge/nsh/pull/1887
* web: Turn the keyboard's autocorrection off with autoCorrect={false} by @daoleno in https://github.com/longbridge/nsh/pull/1883
* windows: Put the app in the background while the screen cannot be seen by @madcodelife in https://github.com/longbridge/nsh/pull/1867
* gpui: Cut a clamped text's last line at its box, not after a word by @obito-t in https://github.com/longbridge/nsh/pull/1918
* macos: Keep a multiline input's caret in view as it grows by @sunfuze in https://github.com/longbridge/nsh/pull/1921
* core: Keep a list's sticky rows below its built-in scroll bar by @obito-t in https://github.com/longbridge/nsh/pull/1909
* rich_editor: Place the caret and presses by an inline view's width by @Tzyito in https://github.com/longbridge/nsh/pull/1894
* android: Keep the development error screen clear of side insets by @daoleno in https://github.com/longbridge/nsh/pull/1898
* sheet: Keep bottom sheets and menus above the software keyboard by @madcodelife in https://github.com/longbridge/nsh/pull/1872
* macos: Preserve traffic light spacing with a custom offset by @huacnlee in https://github.com/longbridge/nsh/pull/1907
* android: Keep input text centered and carets natural on Android 15 by @linyisu in https://github.com/longbridge/nsh/pull/1929
* ios: Keep a page below its header after a headerless page is popped by @tacshi in https://github.com/longbridge/nsh/pull/1890
* android: Keep short text on one line after an Input lays out on Android 15 by @daoleno in https://github.com/longbridge/nsh/pull/1871
* terminal: Report the first size even when it is the starting 80 × 24 by @Blackman99 in https://github.com/longbridge/nsh/pull/1638
* diff_view: Keep unwrapped lines on one row at a larger text size by @daoleno in https://github.com/longbridge/nsh/pull/1804
* terminal: Keep each character on its cells by @Blackman99 in https://github.com/longbridge/nsh/pull/1649
* macos: Redraw text when a layout animation grows its height by @blueberrycongee in https://github.com/longbridge/nsh/pull/1758
* js: Keep a removed dialog's dismiss handler outside cycle collection by @huacnlee in https://github.com/longbridge/nsh/pull/1698
* ios: Keep a blocking responder's drag from swiping its page back by @hmziqrs in https://github.com/longbridge/nsh/pull/1757
* linux: Find Flatpak and Snap browsers when launched without XDG_DATA_DIRS by @madcodelife in https://github.com/longbridge/nsh/pull/1908
* macos: Keep the test agent's presses where it aims on a locked screen by @LVtomatoJ in https://github.com/longbridge/nsh/pull/1729
* android: Color a stack's screens in the app's own appearance by @daoleno in https://github.com/longbridge/nsh/pull/1950
* testing: Avoid false cover errors for wholly clipped targets by @kim0 in https://github.com/longbridge/nsh/pull/1735
* core: Reveal views nested in culled inline attachments by @ttizze in https://github.com/longbridge/nsh/pull/1742
* core: Show what views inside text hold when in a scroll view by @jcdiv47 in https://github.com/longbridge/nsh/pull/1948
* macos: Show alerts in front while the app's window is out of sight by @BeADre in https://github.com/longbridge/nsh/pull/1697
* hover_card: Follow the pointer only, not the keyboard focus by @BeADre in https://github.com/longbridge/nsh/pull/1695
* android: Let a tap through a scroll view once the keyboard is down by @daoleno in https://github.com/longbridge/nsh/pull/1755
* android: Pass the declared environment to Rust apps under nsh run by @madcodelife in https://github.com/longbridge/nsh/pull/1958
* gpui: Blur the focused field once the mouse starts selecting text by @obito-t in https://github.com/longbridge/nsh/pull/1968
* windows: Leave views that cannot paint out of the frame by @villatothesea in https://github.com/longbridge/nsh/pull/1973
* core: Lay out anchored overlays within the room they are placed in by @madcodelife in https://github.com/longbridge/nsh/pull/1938
* windows: Scroll a multiline input's text with the wheel by @huacnlee in https://github.com/longbridge/nsh/pull/1976
* core: Report the focus leaving an element removed with the focus inside by @madcodelife in https://github.com/longbridge/nsh/pull/1980
* input: Preserve password input behavior when revealing passwords by @daoleno in https://github.com/longbridge/nsh/pull/1828
* macos: Keep typed quotes and dashes when autoCorrect is off by @wenlingang in https://github.com/longbridge/nsh/pull/1978
* windows: Stop partial frames leaving behind what was drawn before by @Blackman99 in https://github.com/longbridge/nsh/pull/1993
* list: Dispose the empty cell with its list by @daoleno in https://github.com/longbridge/nsh/pull/1990
* windows: Report what an input method writes through TSF as it writes it by @Blackman99 in https://github.com/longbridge/nsh/pull/1994
* android: Dismiss the keyboard of an input in a dialog by @akongG in https://github.com/longbridge/nsh/pull/2001
* core: Keep a root from taking its freed owner's reused slot for its owner by @daoleno in https://github.com/longbridge/nsh/pull/1991
* dev: Catch up a reconnecting host with the hot updates it missed by @huacnlee in https://github.com/longbridge/nsh/pull/1983
* windows: Exit a closed app without tearing down its UI thread by @Amamiya23 in https://github.com/longbridge/nsh/pull/1992
* windows: Clear stale pixels in transparent partial redraws by @Amamiya23 in https://github.com/longbridge/nsh/pull/1995
* animation: Keep the graph's storage to the nodes that live by @madcodelife in https://github.com/longbridge/nsh/pull/2005
* engine: Keep calls on the page's host when the page passes parameters by @Liguoqiang-code in https://github.com/longbridge/nsh/pull/1840
* js: Bound what the decoders read of the values apps pass by @ClarkThan in https://github.com/longbridge/nsh/pull/1954
* core: Lay out unstyled component hosts through to their one node by @huacnlee in https://github.com/longbridge/nsh/pull/2016
* cli: Name a cross-built Windows package's update archive for Windows by @huacnlee in https://github.com/longbridge/nsh/pull/2021
* windows: Preserve alpha from the first composed window frame by @Amamiya23 in https://github.com/longbridge/nsh/pull/1975
* js: Keep a component's children apart when shown beside its own branch by @daoleno in https://github.com/longbridge/nsh/pull/1941
* bundler: Let an If's else expression follow what it reads by @daoleno in https://github.com/longbridge/nsh/pull/1943
* windows: Clear a partial frame's areas again under an opaque background by @madcodelife in https://github.com/longbridge/nsh/pull/2020

</details>

<details>
<summary>New Contributors</summary>

## New Contributors
* @mliem2k made their first contribution in https://github.com/longbridge/nsh/pull/1694
* @crizant made their first contribution in https://github.com/longbridge/nsh/pull/1721
* @blueberrycongee made their first contribution in https://github.com/longbridge/nsh/pull/1758
* @subframe7536 made their first contribution in https://github.com/longbridge/nsh/pull/1816
* @Liguoqiang-code made their first contribution in https://github.com/longbridge/nsh/pull/1840
* @wenlingang made their first contribution in https://github.com/longbridge/nsh/pull/1849
* @tacshi made their first contribution in https://github.com/longbridge/nsh/pull/1890
* @linyisu made their first contribution in https://github.com/longbridge/nsh/pull/1929
* @jcdiv47 made their first contribution in https://github.com/longbridge/nsh/pull/1948
* @villatothesea made their first contribution in https://github.com/longbridge/nsh/pull/1973
* @Amamiya23 made their first contribution in https://github.com/longbridge/nsh/pull/1992

</details>

**Full Changelog**: https://github.com/longbridge/nsh/compare/v0.5.0...v0.5.1

