//! The command bar on AppKit: the controller class, its state, the
//! helpers its parts share, the global shortcut, and the bridge component
//! the shell places. `panel.rs`, `list.rs` and `main_window.rs` hold the
//! controller's methods.

use std::{
    cell::{OnceCell, RefCell},
    ffi::{CStr, CString, c_char, c_void},
    rc::Rc,
};

use nsh_core::JsonValue;
use nsh_platform_macos::{
    ComponentContext, Components, NativeComponent,
    objc2::{
        DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
        rc::Retained,
        runtime::{AnyClass, AnyObject, Bool, ClassBuilder, NSObject, Sel},
        sel,
    },
    objc2_app_kit::NSView,
    objc2_foundation::{NSPoint, NSRect, NSSize},
};

use super::{BRIDGE, HOTKEY_LABEL, rank};

mod list;
mod main_window;
mod panel;

const WIDTH: f64 = 680.0;
const HEIGHT: f64 = 430.0;
const TOP: f64 = 64.0;
const FOOTER: f64 = 36.0;
const ROW: f64 = 44.0;
const RADIUS: f64 = 18.0;

/// One command, as the bridge sent it.
#[derive(Clone)]
struct Item {
    id: String,
    title: String,
    group: String,
    symbol: String,
    shortcut: String,
    front: bool,
}

/// The panel's views, made on first use.
struct Views {
    panel: Retained<AnyObject>,
    field: Retained<AnyObject>,
    list: Retained<AnyObject>,
}

#[derive(Default)]
struct State {
    items: Vec<Item>,
    /// The listed items, best match first.
    shown: Vec<usize>,
    selected: usize,
    /// The row backgrounds, for the highlight.
    rows: Vec<Retained<AnyObject>>,
    views: Option<Views>,
    /// The bridge: its context reaches the app, its view's window is
    /// the main window.
    context: Option<ComponentContext>,
    bridge: Option<Retained<NSView>>,
    /// The menu bar item (kept: the status bar does not retain it).
    status: Option<Retained<AnyObject>>,
    main_patched: bool,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements; the controller
    // only receives AppKit's actions, delegate calls and timers.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "NshStarterCommandBar"]
    #[ivars = RefCell<State>]
    struct Controller;

    impl Controller {
        #[unsafe(method(controlTextDidChange:))]
        fn text_changed(&self, _note: &AnyObject) {
            self.refilter();
        }

        #[unsafe(method(control:textView:doCommandBySelector:))]
        fn command(&self, _control: &AnyObject, _view: &AnyObject, command: Sel) -> Bool {
            if command == sel!(moveDown:) {
                self.step(1);
            } else if command == sel!(moveUp:) {
                self.step(-1);
            } else if command == sel!(insertNewline:) {
                let at = self.ivars().borrow().selected;
                self.run(at);
            } else if command == sel!(cancelOperation:) {
                self.hide();
            } else {
                return Bool::NO;
            }
            Bool::YES
        }

        #[unsafe(method(windowDidResignKey:))]
        fn resigned(&self, _note: &AnyObject) {
            self.hide();
        }

        #[unsafe(method(pick:))]
        fn pick(&self, sender: &AnyObject) {
            // SAFETY: the row buttons' `tag`.
            let tag: isize = unsafe { msg_send![sender, tag] };
            self.run(tag.max(0) as usize);
        }

        #[unsafe(method(openBar:))]
        fn open_bar(&self, _sender: Option<&AnyObject>) {
            self.show();
        }

        #[unsafe(method(showMain:))]
        fn show_main_action(&self, _sender: Option<&AnyObject>) {
            self.show_main();
        }

        #[unsafe(method(hideMain:))]
        fn hide_main(&self, _sender: Option<&AnyObject>) {
            if let Some(window) = self.main_window() {
                let none: Option<&AnyObject> = None;
                // SAFETY: NSWindow's `orderOut:`.
                let _: () = unsafe { msg_send![&*window, orderOut: none] };
            }
        }

        #[unsafe(method(patchMain:))]
        fn patch_main_action(&self, _sender: Option<&AnyObject>) {
            self.patch_main();
        }
    }
);

thread_local! {
    static CONTROLLER: OnceCell<Retained<Controller>> = const { OnceCell::new() };
}

fn controller() -> Retained<Controller> {
    CONTROLLER.with(|cell| {
        cell.get_or_init(|| {
            let mtm = MainThreadMarker::new().expect("the command bar lives on the main thread");
            let this = Controller::alloc(mtm).set_ivars(RefCell::new(State::default()));
            // SAFETY: NSObject's designated initializer.
            let this: Retained<Controller> = unsafe { msg_send![super(this), init] };
            this.install();
            this
        })
        .clone()
    })
}

fn class(name: &CStr) -> &'static AnyClass {
    AnyClass::get(name).expect("an AppKit class")
}

fn ns_string(text: &str) -> Retained<AnyObject> {
    let text = CString::new(text.replace('\0', "")).unwrap_or_default();
    // SAFETY: NSString's documented factory, with a NUL-terminated
    // UTF-8 buffer that outlives the call.
    unsafe { msg_send![class(c"NSString"), stringWithUTF8String: text.as_ptr()] }
}

/// The field's text.
///
/// # Safety
/// `control` answers `stringValue`.
unsafe fn string_value(control: &AnyObject) -> String {
    // SAFETY: the caller's promise, then NSString's `UTF8String`.
    unsafe {
        let s: Option<Retained<AnyObject>> = msg_send![control, stringValue];
        let Some(s) = s else {
            return String::new();
        };
        let p: *const c_char = msg_send![&*s, UTF8String];
        if p.is_null() {
            String::new()
        } else {
            CStr::from_ptr(p).to_string_lossy().into_owned()
        }
    }
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> NSRect {
    NSRect::new(NSPoint::new(x, y), NSSize::new(w, h))
}

fn symbol(name: &str) -> Option<Retained<AnyObject>> {
    let none: Option<&AnyObject> = None;
    // SAFETY: NSImage's SF Symbols factory (macOS 11).
    unsafe {
        msg_send![
            class(c"NSImage"),
            imageWithSystemSymbolName: &*ns_string(name),
            accessibilityDescription: none
        ]
    }
}

/// A one-line label in `size` points, `secondary` in the quieter color.
///
/// # Safety
/// On the main thread.
unsafe fn label(text: &str, frame: NSRect, size: f64, secondary: bool) -> Retained<AnyObject> {
    // SAFETY: the caller's promise; NSTextField, NSFont and NSColor's
    // documented factories.
    unsafe {
        let label: Retained<AnyObject> =
            msg_send![class(c"NSTextField"), labelWithString: &*ns_string(text)];
        let _: () = msg_send![&*label, setFrame: frame];
        // NSFontWeightMedium for titles, regular for the rest.
        let weight = if secondary { 0.0_f64 } else { 0.23_f64 };
        let font: Retained<AnyObject> =
            msg_send![class(c"NSFont"), systemFontOfSize: size, weight: weight];
        let _: () = msg_send![&*label, setFont: &*font];
        let color: Retained<AnyObject> = if secondary {
            msg_send![class(c"NSColor"), secondaryLabelColor]
        } else {
            msg_send![class(c"NSColor"), labelColor]
        };
        let _: () = msg_send![&*label, setTextColor: &*color];
        let _: () = msg_send![&*label, setLineBreakMode: 4_usize];
        label
    }
}

/// A subclass of `superclass` answering `selector` with YES: a panel
/// that can take the keyboard while borderless, a list laid out top
/// down.
fn yes_subclass(name: &CStr, superclass: &CStr, selector: Sel) -> &'static AnyClass {
    if let Some(made) = AnyClass::get(name) {
        return made;
    }
    extern "C-unwind" fn yes(_this: &AnyObject, _cmd: Sel) -> Bool {
        Bool::YES
    }
    let mut builder =
        ClassBuilder::new(name, class(superclass)).expect("a new class name for the bar");
    // SAFETY: the method's signature is the getter's (`-(BOOL)x`).
    unsafe {
        builder.add_method(selector, yes as extern "C-unwind" fn(_, _) -> _);
    }
    builder.register()
}

// Carbon's global hot keys: the one API that sees a shortcut while
// another app is active, without the accessibility permission.
#[repr(C)]
struct EventTypeSpec {
    event_class: u32,
    event_kind: u32,
}

#[repr(C)]
struct EventHotKeyId {
    signature: u32,
    id: u32,
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn GetApplicationEventTarget() -> *mut c_void;
    fn InstallEventHandler(
        target: *mut c_void,
        handler: extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32,
        count: u32,
        types: *const EventTypeSpec,
        user_data: *mut c_void,
        out: *mut *mut c_void,
    ) -> i32;
    fn RegisterEventHotKey(
        key_code: u32,
        modifiers: u32,
        id: EventHotKeyId,
        target: *mut c_void,
        options: u32,
        out: *mut *mut c_void,
    ) -> i32;
}

extern "C" fn hot_key(_next: *mut c_void, _event: *mut c_void, _data: *mut c_void) -> i32 {
    controller().toggle();
    0
}

fn register_hot_key() {
    // kEventClassKeyboard ('keyb'), kEventHotKeyPressed; kVK_Space
    // with optionKey.
    let pressed = EventTypeSpec {
        event_class: u32::from_be_bytes(*b"keyb"),
        event_kind: 5,
    };
    let id = EventHotKeyId {
        signature: u32::from_be_bytes(*b"nshs"),
        id: 1,
    };
    let mut handler = std::ptr::null_mut();
    let mut key = std::ptr::null_mut();
    // SAFETY: Carbon's documented calls, on the main thread, with the
    // application's own event target; the handler lives for the app.
    unsafe {
        let target = GetApplicationEventTarget();
        let installed = InstallEventHandler(
            target,
            hot_key,
            1,
            &pressed,
            std::ptr::null_mut(),
            &mut handler,
        );
        let registered = RegisterEventHotKey(49, 0x0800, id, target, 0, &mut key);
        if installed != 0 || registered != 0 {
            log::warn!(
                "command bar: the {HOTKEY_LABEL} shortcut is unavailable \
                 (install {installed}, register {registered})"
            );
        }
    }
}

/// The bridge: an empty view in the main window.
struct Bridge {
    view: Retained<NSView>,
    /// The last open request seen.
    opened: u64,
}

impl NativeComponent for Bridge {
    fn view(&self) -> Retained<NSView> {
        self.view.clone()
    }

    fn set_prop(&mut self, name: &str, value: Option<&JsonValue>) {
        let bar = controller();
        match name {
            "items" => {
                let items = value
                    .and_then(JsonValue::as_array)
                    .map(|list| {
                        list.iter()
                            .map(|c| {
                                let text = |k: &str| c[k].as_str().unwrap_or_default().to_owned();
                                Item {
                                    id: text("id"),
                                    title: text("title"),
                                    group: text("group"),
                                    symbol: text("symbol"),
                                    shortcut: text("shortcut"),
                                    front: c["front"].as_bool().unwrap_or_default(),
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                bar.ivars().borrow_mut().items = items;
            }
            "open" => {
                let seen = value.and_then(JsonValue::as_u64).unwrap_or_default();
                if seen > self.opened {
                    self.opened = seen;
                    bar.toggle();
                }
            }
            _ => {}
        }
    }
}

fn make(context: ComponentContext) -> Box<dyn NativeComponent> {
    let bar = controller();
    // SAFETY: NSView's designated initializer.
    let view: Retained<NSView> = unsafe {
        msg_send![
            msg_send![class(c"NSView"), alloc],
            initWithFrame: rect(0.0, 0.0, 0.0, 0.0)
        ]
    };
    {
        let mut state = bar.ivars().borrow_mut();
        state.context = Some(context);
        state.bridge = Some(view.clone());
    }
    bar.patch_main();
    Box::new(Bridge { view, opened: 0 })
}

/// Registers the bridge; the bar, the menu bar item and the shortcut
/// come with the first one made.
pub fn register(components: &Components) {
    components.register(BRIDGE, Rc::new(make));
}
