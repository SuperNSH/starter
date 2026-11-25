//! AppKit's controls.

use std::{
    ffi::{CStr, CString, c_char},
    rc::Rc,
};

use nsh_core::JsonValue;
use nsh_platform_macos::{
    ComponentContext, Components, NativeComponent,
    objc2::{
        DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
        rc::Retained,
        runtime::{AnyClass, AnyObject, NSObject},
        sel,
    },
    objc2_app_kit::NSView,
    objc2_foundation::{NSPoint, NSRect, NSSize},
};

use super::{KINDS, Kind, Props};

mod make;

use make::make;

/// What the target reports for: its control's kind, and the component.
struct Ivars {
    kind: Kind,
    context: ComponentContext,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements; the target only
    // reads its control and forwards to the component's context.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "NshStarterKitTarget"]
    #[ivars = Ivars]
    struct Target;

    impl Target {
        #[unsafe(method(act:))]
        fn act(&self, sender: &AnyObject) {
            let ivars = self.ivars();
            // SAFETY: each read is the documented getter of the control
            // `kind` made, `sender` being that control.
            let data = unsafe {
                match ivars.kind {
                    Kind::Slider | Kind::Stepper => {
                        let v: f64 = msg_send![sender, doubleValue];
                        serde_json::json!({ "value": v })
                    }
                    Kind::Checkbox => {
                        let state: isize = msg_send![sender, state];
                        serde_json::json!({ "on": state == 1 })
                    }
                    Kind::Radio | Kind::GlassButton | Kind::GlassBar => {
                        let tag: isize = msg_send![sender, tag];
                        serde_json::json!({ "index": tag })
                    }
                    Kind::Popup => {
                        let at: isize = msg_send![sender, indexOfSelectedItem];
                        serde_json::json!({ "index": at })
                    }
                    Kind::Date => {
                        let date: Option<Retained<AnyObject>> = msg_send![sender, dateValue];
                        let Some(date) = date else { return };
                        let v: f64 = msg_send![&*date, timeIntervalSince1970];
                        serde_json::json!({ "value": v })
                    }
                    Kind::Color => {
                        let color: Retained<AnyObject> = msg_send![sender, color];
                        let space: Retained<AnyObject> =
                            msg_send![class(c"NSColorSpace"), sRGBColorSpace];
                        let srgb: Option<Retained<AnyObject>> =
                            msg_send![&*color, colorUsingColorSpace: &*space];
                        let Some(c) = srgb else { return };
                        let r: f64 = msg_send![&*c, redComponent];
                        let g: f64 = msg_send![&*c, greenComponent];
                        let b: f64 = msg_send![&*c, blueComponent];
                        serde_json::json!({ "rgb": [r, g, b] })
                    }
                    Kind::Text | Kind::Search => {
                        serde_json::json!({ "text": string_value(sender) })
                    }
                    Kind::Tabs => {
                        let at: isize = msg_send![sender, selectedSegment];
                        serde_json::json!({ "index": at })
                    }
                    Kind::Progress | Kind::Spinner | Kind::Material => return,
                }
            };
            ivars.context.emit("change", data);
        }

        #[unsafe(method(controlTextDidChange:))]
        fn text_changed(&self, note: &AnyObject) {
            // SAFETY: NSNotification's `object`, here the text field.
            let field: Option<Retained<AnyObject>> = unsafe { msg_send![note, object] };
            if let Some(field) = field {
                let text = unsafe { string_value(&field) };
                self.ivars()
                    .context
                    .emit("change", serde_json::json!({ "text": text }));
            }
        }
    }
);

impl Target {
    fn new(mtm: MainThreadMarker, kind: Kind, context: ComponentContext) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(Ivars { kind, context });
        // SAFETY: NSObject's designated initializer.
        unsafe { msg_send![super(this), init] }
    }
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

/// An NSControl's `stringValue` as Rust text.
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

fn color([r, g, b]: [f64; 3]) -> Retained<AnyObject> {
    // SAFETY: NSColor's documented factory.
    unsafe {
        msg_send![
            class(c"NSColor"),
            colorWithSRGBRed: r,
            green: g,
            blue: b,
            alpha: 1.0_f64
        ]
    }
}

/// Liquid Glass exists here (macOS 26 and later).
fn glass() -> bool {
    AnyClass::get(c"NSGlassEffectView").is_some()
}

fn rect(w: f64, h: f64) -> NSRect {
    NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(w, h))
}

struct Control {
    kind: Kind,
    view: Retained<NSView>,
    target: Retained<Target>,
    context: ComponentContext,
    props: Props,
    /// The items the control was last built from (radio, pop-up, tabs).
    built: Vec<String>,
    /// Tabs: the segmented control heading the box, and the box's line.
    tabs: Option<(Retained<AnyObject>, Retained<AnyObject>)>,
}

impl Control {
    /// Rebuilds the radio buttons, pop-up items or tabs when the item
    /// list changed.
    ///
    /// # Safety
    /// The view is the control `kind` made.
    unsafe fn rebuild_items(&mut self) {
        // Tabs read their bodies too, which may arrive after the items.
        let wanted: Vec<String> = self
            .props
            .items
            .iter()
            .chain(&self.props.bodies)
            .cloned()
            .collect();
        if self.built == wanted {
            return;
        }
        self.built = wanted;
        let view: &AnyObject = &self.view;
        // SAFETY: the caller's promise; each call is that control's
        // documented AppKit API.
        unsafe {
            match self.kind {
                Kind::Radio => {
                    let old: Retained<AnyObject> = msg_send![view, arrangedSubviews];
                    let count: usize = msg_send![&*old, count];
                    for at in 0..count {
                        let radio: Retained<AnyObject> = msg_send![&*old, objectAtIndex: at];
                        // A stack view drops an arranged view that
                        // leaves it.
                        let _: () = msg_send![&*radio, removeFromSuperview];
                    }
                    for (at, item) in self.props.items.iter().enumerate() {
                        let radio: Retained<AnyObject> = msg_send![
                            class(c"NSButton"),
                            radioButtonWithTitle: &*ns_string(item),
                            target: &*self.target,
                            action: sel!(act:)
                        ];
                        let _: () = msg_send![&*radio, setTag: at as isize];
                        let _: () = msg_send![view, addArrangedSubview: &*radio];
                    }
                }
                Kind::GlassBar => {
                    let stack: Retained<AnyObject> = if glass() {
                        msg_send![view, contentView]
                    } else {
                        let subviews: Retained<AnyObject> = msg_send![view, subviews];
                        msg_send![&*subviews, firstObject]
                    };
                    let old: Retained<AnyObject> = msg_send![&*stack, arrangedSubviews];
                    let count: usize = msg_send![&*old, count];
                    for at in 0..count {
                        let b: Retained<AnyObject> = msg_send![&*old, objectAtIndex: at];
                        let _: () = msg_send![&*b, removeFromSuperview];
                    }
                    for (at, symbol) in self.props.items.iter().enumerate() {
                        let name = self.props.bodies.get(at).cloned().unwrap_or_default();
                        let image: Option<Retained<AnyObject>> = msg_send![
                            class(c"NSImage"),
                            imageWithSystemSymbolName: &*ns_string(symbol),
                            accessibilityDescription: &*ns_string(&name)
                        ];
                        let Some(image) = image else { continue };
                        let b: Retained<AnyObject> = msg_send![
                            class(c"NSButton"),
                            buttonWithImage: &*image,
                            target: &*self.target,
                            action: sel!(act:)
                        ];
                        let _: () = msg_send![&*b, setBordered: false];
                        let _: () = msg_send![&*b, setTag: at as isize];
                        let _: () = msg_send![&*b, setToolTip: &*ns_string(&name)];
                        let _: () = msg_send![&*stack, addArrangedSubview: &*b];
                    }
                }
                Kind::Popup => {
                    let _: () = msg_send![view, removeAllItems];
                    for item in &self.props.items {
                        let _: () = msg_send![view, addItemWithTitle: &*ns_string(item)];
                    }
                }
                Kind::Tabs => {
                    if let Some((segmented, _)) = &self.tabs {
                        let n = self.props.items.len() as isize;
                        let _: () = msg_send![&**segmented, setSegmentCount: n];
                        for (at, item) in self.props.items.iter().enumerate() {
                            let _: () = msg_send![
                                &**segmented,
                                setLabel: &*ns_string(item),
                                forSegment: at as isize
                            ];
                        }
                        let _: () = msg_send![&**segmented, sizeToFit];
                    }
                }
                _ => {}
            }
        }
    }

    /// Applies every prop, then reports the natural size.
    fn apply(&mut self) {
        // SAFETY: the view is the control `kind` made.
        unsafe { self.rebuild_items() };
        let view: &AnyObject = &self.view;
        let props = &self.props;
        let tint = props.tint.map(color);
        // SAFETY: each call is the documented AppKit API (macOS 11 and
        // later) of the control `kind` made.
        unsafe {
            match self.kind {
                Kind::Slider => {
                    let _: () = msg_send![view, setMinValue: props.min];
                    let _: () = msg_send![view, setMaxValue: props.max];
                    let _: () = msg_send![view, setDoubleValue: props.value];
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setTrackFillColor: &**tint];
                    }
                }
                Kind::Stepper => {
                    let _: () = msg_send![view, setMinValue: props.min];
                    let _: () = msg_send![view, setMaxValue: props.max];
                    let _: () = msg_send![view, setIncrement: props.step];
                    let _: () = msg_send![view, setDoubleValue: props.value];
                }
                Kind::Checkbox => {
                    let _: () = msg_send![view, setTitle: &*ns_string(&props.title)];
                    let _: () = msg_send![view, setState: isize::from(props.on)];
                }
                Kind::Radio => {
                    let radios: Retained<AnyObject> = msg_send![view, arrangedSubviews];
                    let count: usize = msg_send![&*radios, count];
                    for at in 0..count {
                        let radio: Retained<AnyObject> = msg_send![&*radios, objectAtIndex: at];
                        let on = at as isize == props.selected;
                        let _: () = msg_send![&*radio, setState: isize::from(on)];
                    }
                }
                Kind::Popup => {
                    let _: () = msg_send![view, selectItemAtIndex: props.selected.max(0)];
                }
                Kind::Progress => {
                    let _: () = msg_send![view, setDoubleValue: props.value];
                }
                Kind::Spinner => {}
                Kind::GlassButton => {
                    let _: () = msg_send![view, setTitle: &*ns_string(&props.title)];
                    if glass() {
                        // NSBezelStyleGlass (macOS 26).
                        let _: () = msg_send![view, setBezelStyle: 16_usize];
                    }
                    let none: Option<&AnyObject> = None;
                    let bezel = if props.style == "prominent" {
                        tint.as_deref()
                    } else {
                        none
                    };
                    let _: () = msg_send![view, setBezelColor: bezel];
                    if !props.symbol.is_empty() {
                        let image: Option<Retained<AnyObject>> = msg_send![
                            class(c"NSImage"),
                            imageWithSystemSymbolName: &*ns_string(&props.symbol),
                            accessibilityDescription: none
                        ];
                        let _: () = msg_send![view, setImage: image.as_deref()];
                        // NSImageOnly for an icon button, else
                        // NSImageLeading.
                        let position: usize = if props.title.is_empty() { 1 } else { 7 };
                        let _: () = msg_send![view, setImagePosition: position];
                    }
                    if props.title.is_empty() {
                        // Before glass, a toolbar icon has no bezel.
                        let _: () = msg_send![view, setBordered: glass()];
                        if !props.text.is_empty() {
                            let name = ns_string(&props.text);
                            let _: () = msg_send![view, setToolTip: &*name];
                            let _: () = msg_send![view, setAccessibilityLabel: &*name];
                        }
                    }
                }
                Kind::Material => {
                    // NSVisualEffectMaterial: Titlebar, HeaderView,
                    // Sidebar, WindowBackground.
                    let material: isize = match props.style.as_str() {
                        "header" => 10,
                        "sidebar" => 7,
                        "window" => 12,
                        _ => 3,
                    };
                    let _: () = msg_send![view, setMaterial: material];
                    // Behind the window; following its active state.
                    let _: () = msg_send![view, setBlendingMode: 0_isize];
                    let _: () = msg_send![view, setState: 0_isize];
                }
                Kind::GlassBar => {
                    if glass() {
                        // NSGlassEffectViewStyleRegular, Clear.
                        let style: isize = isize::from(props.style == "clear");
                        let _: () = msg_send![view, setStyle: style];
                    }
                }
                Kind::Date => {
                    // NSDatePickerStyleTextFieldAndStepper, ClockAndCalendar.
                    let style: usize = if props.style == "calendar" { 1 } else { 0 };
                    let _: () = msg_send![view, setDatePickerStyle: style];
                    if props.has_date {
                        let date: Retained<AnyObject> = msg_send![
                            class(c"NSDate"),
                            dateWithTimeIntervalSince1970: props.value
                        ];
                        let _: () = msg_send![view, setDateValue: &*date];
                    }
                }
                Kind::Color => {
                    if let Some(c) = props.color {
                        let _: () = msg_send![view, setColor: &*color(c)];
                    }
                }
                Kind::Text | Kind::Search => {
                    let _: () = msg_send![
                        view,
                        setPlaceholderString: &*ns_string(&props.placeholder)
                    ];
                    if string_value(view) != props.text {
                        let _: () = msg_send![view, setStringValue: &*ns_string(&props.text)];
                    }
                }
                Kind::Tabs => {
                    if let Some((segmented, line)) = &self.tabs {
                        let _: () = msg_send![&**segmented, setSelectedSegment: props.selected];
                        if let Some(tint) = &tint {
                            let _: () =
                                msg_send![&**segmented, setSelectedSegmentBezelColor: &**tint];
                        }
                        let body = usize::try_from(props.selected)
                            .ok()
                            .and_then(|at| props.bodies.get(at))
                            .cloned()
                            .unwrap_or_default();
                        let _: () = msg_send![&**line, setStringValue: &*ns_string(&body)];
                        // Centered over the box's top edge, as AppKit's
                        // tab view draws its tabs.
                        let bounds: NSRect = msg_send![view, bounds];
                        let frame: NSRect = msg_send![&**segmented, frame];
                        let x = ((bounds.size.width - frame.size.width) / 2.0).max(0.0);
                        let y = bounds.size.height - frame.size.height;
                        let _: () = msg_send![
                            &**segmented,
                            setFrameOrigin: NSPoint::new(x, y)
                        ];
                    }
                }
            }
            let size: NSSize = msg_send![view, fittingSize];
            let (w, h) = match self.kind {
                // The width comes from the layout; AppKit's bar and
                // field heights.
                Kind::Progress => (0.0, size.height.max(20.0)),
                Kind::Spinner => (32.0, 32.0),
                // Each symbol button gets a 36 pt square on the pill.
                Kind::GlassBar => (self.props.items.len() as f64 * 36.0 + 16.0, 44.0),
                Kind::Color => (size.width.max(44.0), size.height.max(24.0)),
                _ => (size.width.max(0.0), size.height.max(0.0)),
            };
            self.context.set_size(w as f32, h as f32);
        }
    }
}

impl NativeComponent for Control {
    fn view(&self) -> Retained<NSView> {
        self.view.clone()
    }

    fn set_prop(&mut self, name: &str, value: Option<&JsonValue>) {
        if self.props.take(name, value) {
            self.apply();
        }
    }
}

/// Registers the controls.
pub fn register(components: &Components) {
    for (name, kind) in KINDS {
        components.register(
            name,
            Rc::new(move |context: ComponentContext| make(kind, context)),
        );
    }
}
