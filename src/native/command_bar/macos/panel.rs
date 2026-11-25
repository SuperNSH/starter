//! The panel itself: the menu bar item and the global shortcut, the
//! panel's views, and showing and hiding it.

use super::*;

impl Controller {
    /// The menu bar item and the global shortcut, once per app.
    pub(super) fn install(&self) {
        register_hot_key();
        // SAFETY: NSStatusBar, NSStatusItem and NSMenu's documented
        // API, on the main thread.
        let status = unsafe {
            let bar: Retained<AnyObject> = msg_send![class(c"NSStatusBar"), systemStatusBar];
            // NSVariableStatusItemLength.
            let item: Retained<AnyObject> = msg_send![&*bar, statusItemWithLength: -1.0_f64];
            let button: Option<Retained<AnyObject>> = msg_send![&*item, button];
            if let (Some(button), Some(image)) = (button, symbol("command.square")) {
                let _: () = msg_send![&*image, setTemplate: true];
                let _: () = msg_send![&*button, setImage: &*image];
                let _: () = msg_send![&*button, setToolTip: &*ns_string("nsh-starter")];
            }
            let menu: Retained<AnyObject> = msg_send![
                msg_send![class(c"NSMenu"), alloc],
                initWithTitle: &*ns_string("nsh-starter")
            ];
            let add = |title: &str, action: Sel, key: &str| -> Retained<AnyObject> {
                let entry: Retained<AnyObject> = msg_send![
                    &*menu,
                    addItemWithTitle: &*ns_string(title),
                    action: action,
                    keyEquivalent: &*ns_string(key)
                ];
                let _: () = msg_send![&*entry, setTarget: self];
                entry
            };
            let open = add("Open Command Bar", sel!(openBar:), " ");
            // NSEventModifierFlagOption: shown beside the entry.
            let _: () = msg_send![&*open, setKeyEquivalentModifierMask: 1_usize << 19];
            add("Show nsh-starter", sel!(showMain:), "");
            let separator: Retained<AnyObject> = msg_send![class(c"NSMenuItem"), separatorItem];
            let _: () = msg_send![&*menu, addItem: &*separator];
            // No target: `terminate:` goes up the responder chain to NSApp.
            let _: Retained<AnyObject> = msg_send![
                &*menu,
                addItemWithTitle: &*ns_string("Quit nsh-starter"),
                action: sel!(terminate:),
                keyEquivalent: &*ns_string("q")
            ];
            let _: () = msg_send![&*item, setMenu: &*menu];
            item
        };
        self.ivars().borrow_mut().status = Some(status);
    }

    /// The panel and its views, made on first use.
    pub(super) fn views(&self) -> (Retained<AnyObject>, Retained<AnyObject>) {
        if let Some(views) = &self.ivars().borrow().views {
            return (views.panel.clone(), views.field.clone());
        }
        // SAFETY: NSPanel, NSGlassEffectView (or NSVisualEffectView),
        // NSTextField, NSScrollView and NSBox's documented API, on the
        // main thread; frames are in each superview's coordinates.
        let views = unsafe {
            let panel_class = yes_subclass(
                c"NshStarterCommandPanel",
                c"NSPanel",
                sel!(canBecomeKeyWindow),
            );
            // NSWindowStyleMaskBorderless | NonactivatingPanel; buffered.
            let panel: Retained<AnyObject> = msg_send![
                msg_send![panel_class, alloc],
                initWithContentRect: rect(0.0, 0.0, WIDTH, HEIGHT),
                styleMask: 1_usize << 7,
                backing: 2_usize,
                defer: false
            ];
            let clear: Retained<AnyObject> = msg_send![class(c"NSColor"), clearColor];
            let _: () = msg_send![&*panel, setOpaque: false];
            let _: () = msg_send![&*panel, setBackgroundColor: &*clear];
            let _: () = msg_send![&*panel, setHasShadow: true];
            let _: () = msg_send![&*panel, setFloatingPanel: true];
            let _: () = msg_send![&*panel, setHidesOnDeactivate: false];
            let _: () = msg_send![&*panel, setReleasedWhenClosed: false];
            let _: () = msg_send![&*panel, setMovableByWindowBackground: true];
            // NSStatusWindowLevel: over full-screen apps too.
            let _: () = msg_send![&*panel, setLevel: 25_isize];
            // CanJoinAllSpaces | FullScreenAuxiliary.
            let _: () = msg_send![&*panel, setCollectionBehavior: 1_usize | (1_usize << 8)];
            let _: () = msg_send![&*panel, setDelegate: self];

            // The window's content: the backdrop at the bottom, the
            // bar's views over it as a sibling. Inside the glass's own
            // content view they would take its tint and vibrancy.
            let frame = rect(0.0, 0.0, WIDTH, HEIGHT);
            let root: Retained<NSView> =
                msg_send![msg_send![class(c"NSView"), alloc], initWithFrame: frame];
            let _: () = msg_send![&*root, setWantsLayer: true];
            let _: () = msg_send![&*panel, setContentView: &*root];
            // The glass draws its own edge; the system shadow framed it
            // as a hard box.
            let _: () = msg_send![&*panel, setHasShadow: false];
            let backdrop: Retained<AnyObject> = match AnyClass::get(c"NSGlassEffectView") {
                Some(glass) => {
                    let backdrop: Retained<AnyObject> =
                        msg_send![msg_send![glass, alloc], initWithFrame: frame];
                    let _: () = msg_send![&*backdrop, setCornerRadius: RADIUS];
                    // NSGlassEffectViewStyleClear: the raw glass, not the
                    // appearance's dark or light panel.
                    let _: () = msg_send![&*backdrop, setStyle: 1_isize];
                    // The heavily frosted variant (private, so asked for
                    // only where it answers; gpui-starter's launcher).
                    if backdrop.class().responds_to(sel!(set_variant:)) {
                        let _: () = msg_send![&*backdrop, set_variant: 3_isize];
                    }
                    // A breath of tint keeps text readable over a bright
                    // desktop without turning the glass into a fill.
                    let tint: Retained<AnyObject> = msg_send![
                        class(c"NSColor"),
                        colorWithRed: 0.0_f64,
                        green: 0.0_f64,
                        blue: 0.0_f64,
                        alpha: 0.06_f64
                    ];
                    let _: () = msg_send![&*backdrop, setTintColor: &*tint];
                    // Glass renders only around a layer-backed content
                    // view.
                    let inner: Retained<NSView> =
                        msg_send![msg_send![class(c"NSView"), alloc], initWithFrame: frame];
                    let _: () = msg_send![&*inner, setWantsLayer: true];
                    let _: () = msg_send![&*inner, setAutoresizingMask: 2_usize | 16_usize];
                    let _: () = msg_send![&*backdrop, setContentView: &*inner];
                    backdrop
                }
                None => {
                    // Before macOS 26: the HUD material, behind the
                    // window, always active, rounded by its layer.
                    let backdrop: Retained<AnyObject> = msg_send![
                        msg_send![class(c"NSVisualEffectView"), alloc],
                        initWithFrame: frame
                    ];
                    let _: () = msg_send![&*backdrop, setMaterial: 13_isize];
                    let _: () = msg_send![&*backdrop, setBlendingMode: 0_isize];
                    let _: () = msg_send![&*backdrop, setState: 1_isize];
                    let _: () = msg_send![&*backdrop, setWantsLayer: true];
                    let layer: Retained<AnyObject> = msg_send![&*backdrop, layer];
                    let _: () = msg_send![&*layer, setCornerRadius: RADIUS];
                    let _: () = msg_send![&*layer, setMasksToBounds: true];
                    backdrop
                }
            };
            let _: () = msg_send![&*backdrop, setAutoresizingMask: 2_usize | 16_usize];
            let _: () = msg_send![&*root, addSubview: &*backdrop];
            let content: Retained<NSView> =
                msg_send![msg_send![class(c"NSView"), alloc], initWithFrame: frame];
            let _: () = msg_send![&*root, addSubview: &*content];

            // The search row.
            if let Some(image) = symbol("magnifyingglass") {
                let icon: Retained<AnyObject> =
                    msg_send![class(c"NSImageView"), imageViewWithImage: &*image];
                let _: () = msg_send![&*icon, setFrame: rect(24.0, HEIGHT - 42.0, 20.0, 20.0)];
                let gray: Retained<AnyObject> = msg_send![class(c"NSColor"), secondaryLabelColor];
                let _: () = msg_send![&*icon, setContentTintColor: &*gray];
                let _: () = msg_send![&*content, addSubview: &*icon];
            }
            let field: Retained<AnyObject> = msg_send![
                msg_send![class(c"NSTextField"), alloc],
                initWithFrame: rect(56.0, HEIGHT - 47.0, WIDTH - 80.0, 30.0)
            ];
            let _: () = msg_send![&*field, setBezeled: false];
            let _: () = msg_send![&*field, setBordered: false];
            let _: () = msg_send![&*field, setDrawsBackground: false];
            // NSFocusRingTypeNone.
            let _: () = msg_send![&*field, setFocusRingType: 1_usize];
            let font: Retained<AnyObject> =
                msg_send![class(c"NSFont"), systemFontOfSize: 21.0_f64, weight: 0.0_f64];
            let _: () = msg_send![&*field, setFont: &*font];
            let _: () = msg_send![
                &*field,
                setPlaceholderString: &*ns_string("Search pages and actions…")
            ];
            let _: () = msg_send![&*field, setDelegate: self];
            let _: () = msg_send![&*content, addSubview: &*field];

            // NSBoxSeparator lines under the search row and over the
            // footer.
            for y in [HEIGHT - TOP, FOOTER] {
                let line: Retained<AnyObject> = msg_send![
                    msg_send![class(c"NSBox"), alloc],
                    initWithFrame: rect(0.0, y, WIDTH, 1.0)
                ];
                let _: () = msg_send![&*line, setBoxType: 2_usize];
                let _: () = msg_send![&*content, addSubview: &*line];
            }

            // The list, top down in a scroll view.
            let list_class = yes_subclass(c"NshStarterCommandList", c"NSView", sel!(isFlipped));
            let list_height = HEIGHT - TOP - FOOTER - 2.0;
            let list: Retained<AnyObject> = msg_send![
                msg_send![list_class, alloc],
                initWithFrame: rect(0.0, 0.0, WIDTH, list_height)
            ];
            let scroll: Retained<AnyObject> = msg_send![
                msg_send![class(c"NSScrollView"), alloc],
                initWithFrame: rect(0.0, FOOTER + 1.0, WIDTH, list_height)
            ];
            let _: () = msg_send![&*scroll, setDrawsBackground: false];
            let _: () = msg_send![&*scroll, setHasVerticalScroller: true];
            let _: () = msg_send![&*scroll, setAutohidesScrollers: true];
            let clip: Retained<AnyObject> = msg_send![&*scroll, contentView];
            let _: () = msg_send![&*clip, setDrawsBackground: false];
            let _: () = msg_send![&*scroll, setDocumentView: &*list];
            let _: () = msg_send![&*content, addSubview: &*scroll];

            // The footer: the keys, then the global shortcut.
            let keys = label(
                "↵ Open     ↑↓ Move     esc Close",
                rect(24.0, 10.0, 360.0, 16.0),
                12.0,
                true,
            );
            let _: () = msg_send![&*content, addSubview: &*keys];
            let anywhere = label(
                &format!("{HOTKEY_LABEL} from anywhere"),
                rect(WIDTH - 224.0, 10.0, 200.0, 16.0),
                12.0,
                true,
            );
            // NSTextAlignmentRight.
            let _: () = msg_send![&*anywhere, setAlignment: 1_isize];
            let _: () = msg_send![&*content, addSubview: &*anywhere];

            Views { panel, field, list }
        };
        let handles = (views.panel.clone(), views.field.clone());
        self.ivars().borrow_mut().views = Some(views);
        handles
    }

    pub(super) fn show(&self) {
        let (panel, field) = self.views();
        // SAFETY: NSScreen and NSWindow's documented API.
        unsafe {
            let screen: Option<Retained<AnyObject>> = msg_send![class(c"NSScreen"), mainScreen];
            if let Some(screen) = screen {
                let frame: NSRect = msg_send![&*screen, visibleFrame];
                let x = frame.origin.x + (frame.size.width - WIDTH) / 2.0;
                let y = frame.origin.y + frame.size.height * 0.78 - HEIGHT;
                let _: () = msg_send![&*panel, setFrameOrigin: NSPoint::new(x, y)];
            }
            let _: () = msg_send![&*field, setStringValue: &*ns_string("")];
        }
        self.refilter();
        let none: Option<&AnyObject> = None;
        // SAFETY: as above.
        unsafe {
            let _: () = msg_send![&*panel, makeKeyAndOrderFront: none];
            let _: bool = msg_send![&*panel, makeFirstResponder: &*field];
        }
    }

    pub(super) fn hide(&self) {
        let panel = self
            .ivars()
            .borrow()
            .views
            .as_ref()
            .map(|v| v.panel.clone());
        if let Some(panel) = panel {
            let none: Option<&AnyObject> = None;
            // SAFETY: NSWindow's `orderOut:`.
            let _: () = unsafe { msg_send![&*panel, orderOut: none] };
        }
    }

    pub(super) fn toggle(&self) {
        let panel = self
            .ivars()
            .borrow()
            .views
            .as_ref()
            .map(|v| v.panel.clone());
        // SAFETY: NSWindow's `isVisible`.
        let visible = panel.is_some_and(|p| unsafe { msg_send![&*p, isVisible] });
        if visible {
            self.hide();
        } else {
            self.show();
        }
    }
}
