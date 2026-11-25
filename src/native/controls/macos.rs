//! AppKit's button, switch and segmented control.

use std::{ffi::CString, rc::Rc};

use nsh_core::JsonValue;
use nsh_platform_macos::{
    ComponentContext, Components, NativeComponent,
    objc2::{
        DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
        rc::Retained,
        runtime::{AnyClass, AnyObject, NSObject, Sel},
        sel,
    },
    objc2_app_kit::NSView,
    objc2_foundation::NSSize,
};

use super::{BUTTON, Kind, Props, SEGMENTED, SWITCH};

define_class!(
    // SAFETY: NSObject has no subclassing requirements; the target only
    // forwards its controls' actions to the component's context.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "NshStarterNativeTarget"]
    #[ivars = ComponentContext]
    struct Target;

    impl Target {
        #[unsafe(method(pressed:))]
        fn pressed(&self, _sender: &AnyObject) {
            self.ivars().emit("press", JsonValue::Null);
        }

        #[unsafe(method(switched:))]
        fn switched(&self, sender: &AnyObject) {
            // SAFETY: NSSwitch's `state` (NSControlStateValueOn is 1).
            let state: isize = unsafe { msg_send![sender, state] };
            self.ivars()
                .emit("change", serde_json::json!({ "on": state == 1 }));
        }

        #[unsafe(method(segmented:))]
        fn segmented(&self, sender: &AnyObject) {
            // SAFETY: NSSegmentedControl's `selectedSegment`.
            let index: isize = unsafe { msg_send![sender, selectedSegment] };
            self.ivars().emit("change", serde_json::json!({ "index": index }));
        }
    }
);

impl Target {
    fn new(mtm: MainThreadMarker, context: ComponentContext) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(context);
        // SAFETY: NSObject's designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

fn class(name: &std::ffi::CStr) -> &'static AnyClass {
    AnyClass::get(name).expect("an AppKit class")
}

fn ns_string(text: &str) -> Retained<AnyObject> {
    let text = CString::new(text.replace('\0', "")).unwrap_or_default();
    // SAFETY: NSString's documented factory, with a NUL-terminated
    // UTF-8 buffer that outlives the call.
    unsafe { msg_send![class(c"NSString"), stringWithUTF8String: text.as_ptr()] }
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

struct Control {
    kind: Kind,
    view: Retained<NSView>,
    _target: Retained<Target>,
    context: ComponentContext,
    props: Props,
}

impl Control {
    fn apply(&self) {
        let view: &AnyObject = &self.view;
        let props = &self.props;
        // SAFETY: each call is the documented AppKit API of the control
        // `kind` made (macOS 11 and later).
        unsafe {
            match self.kind {
                Kind::Button => {
                    let _: () = msg_send![view, setTitle: &*ns_string(&props.title)];
                    // NSBezelStylePush.
                    let _: () = msg_send![view, setBezelStyle: 1_usize];
                    let _: () = msg_send![view, setBordered: props.style != "plain"];
                    let tint = props.tint.map(color);
                    let none: Option<&AnyObject> = None;
                    match (props.style.as_str(), tint.as_deref()) {
                        ("filled", Some(tint)) => {
                            let _: () = msg_send![view, setBezelColor: tint];
                            let _: () = msg_send![view, setContentTintColor: none];
                        }
                        ("tinted" | "plain", Some(tint)) => {
                            let _: () = msg_send![view, setBezelColor: none];
                            let _: () = msg_send![view, setContentTintColor: tint];
                        }
                        _ => {
                            let _: () = msg_send![view, setBezelColor: none];
                            let _: () = msg_send![view, setContentTintColor: none];
                        }
                    }
                    if let Some(symbol) = &props.symbol {
                        let none: Option<&AnyObject> = None;
                        let image: Option<Retained<AnyObject>> = msg_send![
                            class(c"NSImage"),
                            imageWithSystemSymbolName: &*ns_string(symbol),
                            accessibilityDescription: none
                        ];
                        let _: () = msg_send![view, setImage: image.as_deref()];
                        // NSImageLeading.
                        let _: () = msg_send![view, setImagePosition: 7_usize];
                    }
                }
                Kind::Switch => {}
                Kind::Segmented => {
                    let count: isize = msg_send![view, segmentCount];
                    if count as usize != props.items.len() {
                        let _: () = msg_send![view, setSegmentCount: props.items.len() as isize];
                    }
                    for (at, item) in props.items.iter().enumerate() {
                        let _: () = msg_send![
                            view,
                            setLabel: &*ns_string(item),
                            forSegment: at as isize
                        ];
                    }
                    let _: () = msg_send![view, setSelectedSegment: props.selected];
                    if let Some(tint) = props.tint {
                        let _: () = msg_send![view, setSelectedSegmentBezelColor: &*color(tint)];
                    }
                }
            }
            let size: NSSize = msg_send![view, fittingSize];
            self.context
                .set_size(size.width.max(0.0) as f32, size.height.max(0.0) as f32);
        }
    }
}

impl NativeComponent for Control {
    fn view(&self) -> Retained<NSView> {
        self.view.clone()
    }

    fn set_prop(&mut self, name: &str, value: Option<&JsonValue>) {
        let view: &AnyObject = &self.view;
        match (self.kind, name) {
            (Kind::Switch, "on") => {
                let on = value.and_then(JsonValue::as_bool).unwrap_or_default();
                // SAFETY: NSSwitch's `setState:`.
                unsafe { msg_send![view, setState: isize::from(on)] }
            }
            (Kind::Segmented, "selected") => {
                self.props.take(name, value);
                // SAFETY: NSSegmentedControl's `setSelectedSegment:`.
                unsafe { msg_send![view, setSelectedSegment: self.props.selected] }
            }
            _ => {
                if self.props.take(name, value) {
                    self.apply();
                }
            }
        }
    }
}

fn make(kind: Kind, context: ComponentContext) -> Box<dyn NativeComponent> {
    let mtm = MainThreadMarker::new().expect("components are made on the main thread");
    let target = Target::new(mtm, context.clone());
    let (name, action): (&std::ffi::CStr, Sel) = match kind {
        Kind::Button => (c"NSButton", sel!(pressed:)),
        Kind::Switch => (c"NSSwitch", sel!(switched:)),
        Kind::Segmented => (c"NSSegmentedControl", sel!(segmented:)),
    };
    // SAFETY: the control's initializer, then NSControl's documented
    // target and action.
    let control: Retained<NSView> = unsafe {
        let control: Retained<NSView> = msg_send![msg_send![class(name), alloc], init];
        let _: () = msg_send![&*control, setTarget: &*target];
        let _: () = msg_send![&*control, setAction: action];
        if kind == Kind::Segmented {
            // NSSegmentSwitchTrackingSelectOne.
            let _: () = msg_send![&*control, setTrackingMode: 0_usize];
        }
        control
    };
    let component = Control {
        kind,
        view: control,
        _target: target,
        context,
        props: Props::default(),
    };
    component.apply();
    Box::new(component)
}

/// Registers the three controls.
pub fn register(components: &Components) {
    for (name, kind) in [
        (BUTTON, Kind::Button),
        (SWITCH, Kind::Switch),
        (SEGMENTED, Kind::Segmented),
    ] {
        components.register(
            name,
            Rc::new(move |context: ComponentContext| make(kind, context)),
        );
    }
}
