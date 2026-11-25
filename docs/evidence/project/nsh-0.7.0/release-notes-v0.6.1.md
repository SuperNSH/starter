## SVG assets

Native apps using SVG files through `asset!` or `assets!` now get a build error explaining which platform feature to enable. SVG remains optional. Runtime URLs are checked when loaded.

## Runtime and controls

Disposed reactive memos can no longer overwrite another node. Keyed rows use their current index, text blocks retain their content, and disposing a carousel clears its indicator target. Resizable panels respect their maximum size and fixed table columns leave room for the scrollbar.

Stopping or replacing a composite animation cancels queued work so an old animation cannot overwrite its replacement. Native updater calls report when updater support is disabled. Missing shader, Windows video, highlighting and math support produce diagnostics.

## Platform fixes

On Windows 11, a view in a transparent window can show the system backdrop using `windows={{ backdrop: { material: "blur" } }}`.

Video stays paused or ended when unrelated properties change on Apple, Android, Linux, Windows and the web. Desktop fixes preserve hidden window state and reduce macOS screenshot memory use. Mobile fixes address Android callbacks after view destruction, system bars, RTL headers and badges, and iOS keyboard, scrolling and safe-area handling.

## Building and embedding

Rust app packages respect their minimum Apple deployment target, embedded iOS apps link AuthenticationServices, and macOS text tests compile again.

<details><summary>Contributors</summary>

@labcoder @cosformula @Lemenxzy @aiiven @alexwan02 @burdygin @chaxus @daoleno @huacnlee @jsiu93 @maanaskarwa @madcodelife @richerfu @sunli829 @ttizze

</details>

<details><summary>What's Changed</summary>

- android: Keep scroll calls from reaching native code after the view is destroyed by @alexwan02 in https://github.com/longbridge/nsh/pull/2449
- android: Skip scroll callbacks after the view is destroyed by @huacnlee in https://github.com/longbridge/nsh/pull/2442
- android: Unbox the dialog command before moving its fields out by @madcodelife in https://github.com/longbridge/nsh/pull/2498
- list: Read a keyed row at its new index whichever memo runs first by @madcodelife in https://github.com/longbridge/nsh/pull/2370
- core: Keep a text block's only raw text run without a node by @huacnlee in https://github.com/longbridge/nsh/pull/2366
- core: Refuse a Windows path as a URL to open by @sunli829 in https://github.com/longbridge/nsh/pull/2388
- macos: Avoid retaining CoreGraphics frame cache across screenshots by @burdygin in https://github.com/longbridge/nsh/pull/2577
- macos: Keep a hidden window hidden when it is asked for full screen by @sunli829 in https://github.com/longbridge/nsh/pull/2487
- windows: Keep a hidden window hidden when its mode or bounds are set by @sunli829 in https://github.com/longbridge/nsh/pull/2419
- windows: Keep the app's size while its main window is minimized by @sunli829 in https://github.com/longbridge/nsh/pull/2428
- ios: Keep the keyboard up when UIKit reports no next focused view by @maanaskarwa in https://github.com/longbridge/nsh/pull/2486
- ios: Preserve the latest content height when narrowing a scroll view by @ttizze in https://github.com/longbridge/nsh/pull/2491
- video: Act on `paused` only when it changes on Apple and Android by @sunli829 in https://github.com/longbridge/nsh/pull/2495
- web: Act on a video's `paused` only when it changes by @sunli829 in https://github.com/longbridge/nsh/pull/2515
- windows: Leave an ended video ended when another prop changes by @sunli829 in https://github.com/longbridge/nsh/pull/2427
- carousel: Clear the indicator target when its carousel is disposed by @richerfu in https://github.com/longbridge/nsh/pull/2527
- ios: Keep a stack's safe area when a transformed container lays it out by @madcodelife in https://github.com/longbridge/nsh/pull/2601
- android: Keep header and tab bar items clear of a side navigation bar by @madcodelife in https://github.com/longbridge/nsh/pull/2608
- android: Keep a scroll view's edge pull when nothing around takes the drag by @madcodelife in https://github.com/longbridge/nsh/pull/2584
- android: Keep the default system bars in contrast with the app's appearance by @madcodelife in https://github.com/longbridge/nsh/pull/2602
- data_table: Keep the horizontal scroll bar between the fixed columns by @Lemenxzy in https://github.com/longbridge/nsh/pull/2430
- resizable: Stop a drag where the panel after the handle reaches its maxSize by @aiiven in https://github.com/longbridge/nsh/pull/2415
- android: Keep a page's headerRight off the screen edge in RTL by @chaxus in https://github.com/longbridge/nsh/pull/2534
- android: Keep a TabBar badge's count in its order in RTL by @chaxus in https://github.com/longbridge/nsh/pull/2561
- core: Make svg optional again and fail the build when SVG assets need it by @huacnlee in https://github.com/longbridge/nsh/pull/2615
- ios: Link AuthenticationServices in apps that embed the runtime by @daoleno in https://github.com/longbridge/nsh/pull/2374
- cli: Build an app's Rust code for its packages' minimum Apple system by @madcodelife in https://github.com/longbridge/nsh/pull/2368
- macos: Build the text tests again with a qualified Arc by @jsiu93 in https://github.com/longbridge/nsh/pull/2412
- linux: Leave an ended or control-paused video so when another prop changes by @huacnlee in https://github.com/longbridge/nsh/pull/2443
- core: Keep a memo let go during its own run from reading or writing another node by @madcodelife in https://github.com/longbridge/nsh/pull/2402
- Windows backdrop support by @labcoder in https://github.com/longbridge/nsh/pull/2471
- Missing-feature diagnostics by @huacnlee in https://github.com/longbridge/nsh/pull/2613
- Cancel pending composite animations by @cosformula in https://github.com/longbridge/nsh/pull/2507

</details>

<details><summary>New Contributors</summary>

- @aiiven contributed the resizable panel fix in https://github.com/longbridge/nsh/pull/2415.
- @maanaskarwa contributed the iOS keyboard fix in https://github.com/longbridge/nsh/pull/2486.

</details>

🤖 [Noop](https://huacnlee.github.io/noop) (release-manager) · AI agent acting for @huacnlee

