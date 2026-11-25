//! UIKit's controls. Checkbox, radio group and tabs have no UIKit control;
//! the Rust side never asks for them on iOS.

use std::{
    ffi::{CStr, CString, c_char},
    rc::Rc,
};

use nsh_core::JsonValue;
use nsh_platform_ios::{ComponentContext, Components, NativeComponent};
use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, NSObject},
    sel,
};
use objc2_core_foundation::CGSize;
use objc2_ui_kit::UIView;

use super::{KINDS, Kind, Props};

/// `UIControlEventValueChanged`, `UIControlEventEditingChanged`.
const VALUE_CHANGED: usize = 1 << 12;
const EDITING_CHANGED: usize = 1 << 17;
/// `UIControlEventPrimaryActionTriggered`.
const PRIMARY_ACTION: usize = 1 << 13;

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
                    Kind::Slider => {
                        let v: f32 = msg_send![sender, value];
                        serde_json::json!({ "value": v })
                    }
                    Kind::Stepper => {
                        let v: f64 = msg_send![sender, value];
                        serde_json::json!({ "value": v })
                    }
                    Kind::Date => {
                        let date: Retained<AnyObject> = msg_send![sender, date];
                        let v: f64 = msg_send![&*date, timeIntervalSince1970];
                        serde_json::json!({ "value": v })
                    }
                    Kind::Color => {
                        let c: Option<Retained<AnyObject>> = msg_send![sender, selectedColor];
                        let Some(c) = c else { return };
                        let (mut r, mut g, mut b, mut a) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
                        let _: bool = msg_send![
                            &*c,
                            getRed: &mut r,
                            green: &mut g,
                            blue: &mut b,
                            alpha: &mut a
                        ];
                        serde_json::json!({ "rgb": [r, g, b] })
                    }
                    Kind::GlassButton | Kind::GlassBar => {
                        let tag: isize = msg_send![sender, tag];
                        serde_json::json!({ "index": tag })
                    }
                    Kind::Text | Kind::Search => {
                        let s: Option<Retained<AnyObject>> = msg_send![sender, text];
                        serde_json::json!({ "text": rust_string(s.as_deref()) })
                    }
                    _ => return,
                }
            };
            ivars.context.emit("change", data);
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
    AnyClass::get(name).expect("a UIKit class")
}

fn ns_string(text: &str) -> Retained<AnyObject> {
    let text = CString::new(text.replace('\0', "")).unwrap_or_default();
    // SAFETY: NSString's documented factory, with a NUL-terminated
    // UTF-8 buffer that outlives the call.
    unsafe { msg_send![class(c"NSString"), stringWithUTF8String: text.as_ptr()] }
}

fn rust_string(s: Option<&AnyObject>) -> String {
    let Some(s) = s else {
        return String::new();
    };
    // SAFETY: NSString's `UTF8String`, valid while `s` lives.
    unsafe {
        let p: *const c_char = msg_send![s, UTF8String];
        if p.is_null() {
            String::new()
        } else {
            CStr::from_ptr(p).to_string_lossy().into_owned()
        }
    }
}

fn color([r, g, b]: [f64; 3]) -> Retained<AnyObject> {
    // SAFETY: UIColor's documented factory.
    unsafe { msg_send![class(c"UIColor"), colorWithRed: r, green: g, blue: b, alpha: 1.0_f64] }
}

struct Control {
    kind: Kind,
    view: Retained<UIView>,
    target: Retained<Target>,
    context: ComponentContext,
    props: Props,
    built: Vec<String>,
    built_selected: isize,
}

impl Control {
    /// The pop-up's menu: one action per item, the selected one on.
    ///
    /// # Safety
    /// The view is a UIButton.
    unsafe fn rebuild_menu(&mut self) {
        if self.built == self.props.items && self.built_selected == self.props.selected {
            return;
        }
        self.built = self.props.items.clone();
        self.built_selected = self.props.selected;
        // SAFETY: the caller's promise; UIAction, UIMenu and NSArray's
        // documented factories (iOS 14 and later).
        unsafe {
            let mut actions: Vec<Retained<AnyObject>> = Vec::new();
            for (at, item) in self.props.items.iter().enumerate() {
                let context = self.context.clone();
                let handler = block2::RcBlock::new(move |_: *mut AnyObject| {
                    context.emit("change", serde_json::json!({ "index": at }));
                });
                let none: Option<&AnyObject> = None;
                let action: Retained<AnyObject> = msg_send![
                    class(c"UIAction"),
                    actionWithTitle: &*ns_string(item),
                    image: none,
                    identifier: none,
                    handler: &*handler
                ];
                // UIMenuElementStateOn.
                if at as isize == self.props.selected.max(0) {
                    let _: () = msg_send![&*action, setState: 1_isize];
                }
                actions.push(action);
            }
            let ptrs: Vec<*const AnyObject> = actions.iter().map(Retained::as_ptr).collect();
            let list: Retained<AnyObject> = msg_send![
                class(c"NSArray"),
                arrayWithObjects: ptrs.as_ptr(),
                count: ptrs.len()
            ];
            let menu: Retained<AnyObject> = msg_send![class(c"UIMenu"), menuWithChildren: &*list];
            let view: &AnyObject = &self.view;
            let _: () = msg_send![view, setMenu: &*menu];
        }
    }

    /// The glass bar's buttons, one per SF Symbol in `items`.
    ///
    /// # Safety
    /// The view is the bar's UIVisualEffectView, holding its stack.
    unsafe fn rebuild_bar(&mut self) {
        if self.built == self.props.items {
            return;
        }
        self.built = self.props.items.clone();
        // SAFETY: the caller's promise; UIStackView, UIButton and
        // UIImage's documented API.
        unsafe {
            let content: Retained<AnyObject> = msg_send![&*self.view, contentView];
            let subviews: Retained<AnyObject> = msg_send![&*content, subviews];
            let stack: Retained<AnyObject> = msg_send![&*subviews, firstObject];
            let old: Retained<AnyObject> = msg_send![&*stack, arrangedSubviews];
            let count: usize = msg_send![&*old, count];
            for at in 0..count {
                let b: Retained<AnyObject> = msg_send![&*old, objectAtIndex: at];
                let _: () = msg_send![&*b, removeFromSuperview];
            }
            for (at, symbol) in self.props.items.iter().enumerate() {
                let b: Retained<AnyObject> = msg_send![class(c"UIButton"), buttonWithType: 1_isize];
                let image: Option<Retained<AnyObject>> =
                    msg_send![class(c"UIImage"), systemImageNamed: &*ns_string(symbol)];
                // UIControlStateNormal.
                let _: () = msg_send![&*b, setImage: image.as_deref(), forState: 0_usize];
                if let Some(name) = self.props.bodies.get(at) {
                    let _: () = msg_send![&*b, setAccessibilityLabel: &*ns_string(name)];
                }
                let _: () = msg_send![&*b, setTag: at as isize];
                let _: () = msg_send![
                    &*b,
                    addTarget: &*self.target,
                    action: sel!(act:),
                    forControlEvents: PRIMARY_ACTION
                ];
                let _: () = msg_send![&*stack, addArrangedSubview: &*b];
            }
        }
    }

    fn apply(&mut self) {
        if self.kind == Kind::Popup {
            // SAFETY: a pop-up is a UIButton.
            unsafe { self.rebuild_menu() };
        }
        if self.kind == Kind::GlassBar {
            // SAFETY: made as the bar in `make`.
            unsafe { self.rebuild_bar() };
        }
        let view: &AnyObject = &self.view;
        let props = &self.props;
        let tint = props.tint.map(color);
        // SAFETY: each call is the documented UIKit API (iOS 15 and
        // later) of the control `kind` made.
        unsafe {
            match self.kind {
                Kind::Slider => {
                    let _: () = msg_send![view, setMinimumValue: props.min as f32];
                    let _: () = msg_send![view, setMaximumValue: props.max as f32];
                    let _: () = msg_send![view, setValue: props.value as f32, animated: false];
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setMinimumTrackTintColor: &**tint];
                    }
                }
                Kind::Stepper => {
                    let _: () = msg_send![view, setMinimumValue: props.min];
                    let _: () = msg_send![view, setMaximumValue: props.max];
                    let _: () = msg_send![view, setStepValue: props.step];
                    let _: () = msg_send![view, setValue: props.value];
                }
                Kind::Progress => {
                    let _: () = msg_send![view, setProgress: props.value as f32, animated: false];
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setProgressTintColor: &**tint];
                    }
                }
                Kind::Date => {
                    // UIDatePickerStyleCompact, Inline.
                    let style: isize = if props.style == "calendar" { 3 } else { 2 };
                    let _: () = msg_send![view, setPreferredDatePickerStyle: style];
                    if props.has_date {
                        let date: Retained<AnyObject> = msg_send![
                            class(c"NSDate"),
                            dateWithTimeIntervalSince1970: props.value
                        ];
                        let _: () = msg_send![view, setDate: &*date];
                    }
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setTintColor: &**tint];
                    }
                }
                Kind::Color => {
                    if let Some(c) = props.color {
                        let _: () = msg_send![view, setSelectedColor: &*color(c)];
                    }
                }
                Kind::Text | Kind::Search => {
                    let _: () = msg_send![view, setPlaceholder: &*ns_string(&props.placeholder)];
                    let now: Option<Retained<AnyObject>> = msg_send![view, text];
                    if rust_string(now.as_deref()) != props.text {
                        let _: () = msg_send![view, setText: &*ns_string(&props.text)];
                    }
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setTintColor: &**tint];
                    }
                }
                Kind::Popup => {
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setTintColor: &**tint];
                    }
                }
                Kind::GlassButton => {
                    let configs = class(c"UIButtonConfiguration");
                    let prominent = props.style == "prominent";
                    // The glass configurations are iOS 26's.
                    let config: Retained<AnyObject> = if prominent
                        && configs.responds_to(sel!(prominentGlassButtonConfiguration))
                    {
                        msg_send![configs, prominentGlassButtonConfiguration]
                    } else if configs.responds_to(sel!(glassButtonConfiguration)) {
                        msg_send![configs, glassButtonConfiguration]
                    } else if prominent {
                        msg_send![configs, filledButtonConfiguration]
                    } else {
                        msg_send![configs, grayButtonConfiguration]
                    };
                    let _: () = msg_send![&*config, setTitle: &*ns_string(&props.title)];
                    if !props.symbol.is_empty() {
                        let image: Option<Retained<AnyObject>> = msg_send![
                            class(c"UIImage"),
                            systemImageNamed: &*ns_string(&props.symbol)
                        ];
                        let _: () = msg_send![&*config, setImage: image.as_deref()];
                        let _: () = msg_send![&*config, setImagePadding: 6.0_f64];
                    }
                    let _: () = msg_send![view, setConfiguration: &*config];
                    if let Some(tint) = &tint {
                        let _: () = msg_send![view, setTintColor: &**tint];
                    }
                }
                Kind::GlassBar => {
                    if let Some(glass) = AnyClass::get(c"UIGlassEffect") {
                        // UIGlassEffectStyleRegular, Clear.
                        let style = isize::from(props.style == "clear");
                        let effect: Retained<AnyObject> = msg_send![glass, effectWithStyle: style];
                        let _: () = msg_send![view, setEffect: &*effect];
                    }
                    let content: Retained<AnyObject> = msg_send![view, contentView];
                    let bounds: objc2_core_foundation::CGRect = msg_send![&*content, bounds];
                    let subviews: Retained<AnyObject> = msg_send![&*content, subviews];
                    let stack: Retained<AnyObject> = msg_send![&*subviews, firstObject];
                    let _: () = msg_send![&*stack, setFrame: bounds];
                }
                _ => {}
            }
            let size: CGSize = msg_send![view, intrinsicContentSize];
            let (w, h) = match self.kind {
                Kind::Progress => (0.0, 4.0),
                Kind::Text | Kind::Search => (0.0, size.height.max(36.0)),
                Kind::Color => (36.0, 36.0),
                Kind::GlassBar => (props.items.len() as f64 * 44.0 + 16.0, 48.0),
                _ => (size.width.max(0.0), size.height.max(0.0)),
            };
            self.context.set_size(w as f32, h as f32);
        }
    }
}

impl NativeComponent for Control {
    fn view(&self) -> &UIView {
        &self.view
    }

    fn set_prop(&mut self, name: &str, value: Option<&JsonValue>) {
        if self.props.take(name, value) {
            self.apply();
        }
    }
}

fn make(kind: Kind, context: ComponentContext) -> Box<dyn NativeComponent> {
    let mtm = MainThreadMarker::new().expect("components are made on the main thread");
    let target = Target::new(mtm, kind, context.clone());
    // SAFETY: each control's documented initializer, then UIControl's
    // target-action registration.
    let view: Retained<UIView> = unsafe {
        let view: Retained<UIView> = match kind {
            Kind::Popup => {
                // UIButtonTypeSystem.
                let b: Retained<UIView> = msg_send![class(c"UIButton"), buttonWithType: 1_isize];
                let _: () = msg_send![&*b, setShowsMenuAsPrimaryAction: true];
                let _: () = msg_send![&*b, setChangesSelectionAsPrimaryAction: true];
                b
            }
            Kind::Progress => {
                // UIProgressViewStyleDefault.
                msg_send![msg_send![class(c"UIProgressView"), alloc], initWithProgressViewStyle: 0_isize]
            }
            Kind::GlassButton => msg_send![class(c"UIButton"), buttonWithType: 1_isize],
            Kind::GlassBar => {
                // UIGlassEffectStyleRegular; iOS before 26 gets the
                // system material.
                let effect: Retained<AnyObject> = match AnyClass::get(c"UIGlassEffect") {
                    Some(glass) => msg_send![glass, effectWithStyle: 0_isize],
                    None => msg_send![class(c"UIBlurEffect"), effectWithStyle: 8_isize],
                };
                let bar: Retained<UIView> = msg_send![
                    msg_send![class(c"UIVisualEffectView"), alloc],
                    initWithEffect: &*effect
                ];
                let layer: Retained<AnyObject> = msg_send![&*bar, layer];
                let _: () = msg_send![&*layer, setCornerRadius: 24.0_f64];
                let _: () = msg_send![&*bar, setClipsToBounds: true];
                let stack: Retained<AnyObject> =
                    msg_send![msg_send![class(c"UIStackView"), alloc], init];
                // UIStackViewDistributionFillEqually; flexible width
                // and height.
                let _: () = msg_send![&*stack, setDistribution: 1_isize];
                let _: () = msg_send![&*stack, setAutoresizingMask: 2_usize | 16_usize];
                let content: Retained<AnyObject> = msg_send![&*bar, contentView];
                let _: () = msg_send![&*content, addSubview: &*stack];
                bar
            }
            Kind::Spinner => {
                // UIActivityIndicatorViewStyleMedium.
                let s: Retained<UIView> = msg_send![
                    msg_send![class(c"UIActivityIndicatorView"), alloc],
                    initWithActivityIndicatorStyle: 100_isize
                ];
                let _: () = msg_send![&*s, startAnimating];
                s
            }
            _ => {
                let name: &CStr = match kind {
                    Kind::Slider => c"UISlider",
                    Kind::Stepper => c"UIStepper",
                    Kind::Date => c"UIDatePicker",
                    Kind::Color => c"UIColorWell",
                    Kind::Text => c"UITextField",
                    Kind::Search => c"UISearchTextField",
                    // Checkbox, radio and tabs: never asked for here.
                    _ => c"UIView",
                };
                msg_send![msg_send![class(name), alloc], init]
            }
        };
        let v: &AnyObject = &view;
        if kind == Kind::Date {
            // UIDatePickerModeDate.
            let _: () = msg_send![v, setDatePickerMode: 1_isize];
        }
        if kind == Kind::Text {
            // UITextBorderStyleRoundedRect.
            let _: () = msg_send![v, setBorderStyle: 3_isize];
        }
        let events = match kind {
            Kind::GlassButton => PRIMARY_ACTION,
            Kind::Text | Kind::Search => EDITING_CHANGED,
            Kind::Slider | Kind::Stepper | Kind::Date | Kind::Color => VALUE_CHANGED,
            _ => 0,
        };
        if events != 0 {
            let _: () = msg_send![
                v,
                addTarget: &*target,
                action: sel!(act:),
                forControlEvents: events
            ];
        }
        view
    };
    let mut control = Control {
        kind,
        view,
        target,
        context,
        props: Props::new(),
        built: Vec::new(),
        built_selected: -2,
    };
    control.apply();
    Box::new(control)
}

/// Registers the controls UIKit has.
pub fn register(components: &Components) {
    for (name, kind) in KINDS {
        if matches!(
            kind,
            Kind::Checkbox | Kind::Radio | Kind::Tabs | Kind::Material
        ) {
            continue;
        }
        components.register(
            name,
            Rc::new(move |context: ComponentContext| make(kind, context)),
        );
    }
}
