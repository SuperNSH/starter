//! UIKit's button, switch and segmented control.

use std::{ffi::CString, rc::Rc};

use nsh_core::JsonValue;
use nsh_platform_ios::{ComponentContext, Components, NativeComponent};
use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, NSObject, Sel},
    sel,
};
use objc2_core_foundation::CGSize;
use objc2_ui_kit::UIView;

use super::{BUTTON, Kind, Props, SEGMENTED, SWITCH};

/// `UIControlEventValueChanged` and `UIControlEventPrimaryActionTriggered`.
const VALUE_CHANGED: usize = 1 << 12;
const PRIMARY_ACTION: usize = 1 << 13;

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
            // SAFETY: UISwitch's `isOn`.
            let on: bool = unsafe { msg_send![sender, isOn] };
            self.ivars().emit("change", serde_json::json!({ "on": on }));
        }

        #[unsafe(method(segmented:))]
        fn segmented(&self, sender: &AnyObject) {
            // SAFETY: UISegmentedControl's `selectedSegmentIndex`.
            let index: isize = unsafe { msg_send![sender, selectedSegmentIndex] };
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
    AnyClass::get(name).expect("a UIKit class")
}

fn ns_string(text: &str) -> Retained<AnyObject> {
    let text = CString::new(text.replace('\0', "")).unwrap_or_default();
    // SAFETY: NSString's documented factory, with a NUL-terminated
    // UTF-8 buffer that outlives the call.
    unsafe { msg_send![class(c"NSString"), stringWithUTF8String: text.as_ptr()] }
}

fn color([r, g, b]: [f64; 3]) -> Retained<AnyObject> {
    // SAFETY: UIColor's documented factory.
    unsafe { msg_send![class(c"UIColor"), colorWithRed: r, green: g, blue: b, alpha: 1.0_f64] }
}

struct Control {
    kind: Kind,
    view: Retained<UIView>,
    // Kept alive while the control targets it (controls do not retain
    // their targets).
    _target: Retained<Target>,
    context: ComponentContext,
    props: Props,
}

impl Control {
    /// Applies the props to the view, then reports its natural size.
    fn apply(&self) {
        let view: &AnyObject = &self.view;
        let props = &self.props;
        // SAFETY: each call below is the documented UIKit API of the
        // control `kind` made (checked with respondsToSelector where it
        // is newer than iOS 15).
        unsafe {
            match self.kind {
                Kind::Button => {
                    let configs = class(c"UIButtonConfiguration");
                    let glass = sel!(glassButtonConfiguration);
                    let config: Retained<AnyObject> = match props.style.as_str() {
                        "filled" => msg_send![configs, filledButtonConfiguration],
                        "tinted" => msg_send![configs, tintedButtonConfiguration],
                        "plain" => msg_send![configs, plainButtonConfiguration],
                        "glass" if configs.responds_to(glass) => {
                            msg_send![configs, glassButtonConfiguration]
                        }
                        _ => msg_send![configs, grayButtonConfiguration],
                    };
                    let _: () = msg_send![&*config, setTitle: &*ns_string(&props.title)];
                    if let Some(symbol) = &props.symbol {
                        let image: Option<Retained<AnyObject>> = msg_send![
                            class(c"UIImage"),
                            systemImageNamed: &*ns_string(symbol)
                        ];
                        let _: () = msg_send![&*config, setImage: image.as_deref()];
                        let _: () = msg_send![&*config, setImagePadding: 6.0_f64];
                    }
                    // UIButtonConfigurationCornerStyleCapsule.
                    let _: () = msg_send![&*config, setCornerStyle: 4_isize];
                    let _: () = msg_send![view, setConfiguration: &*config];
                    if let Some(tint) = props.tint {
                        let _: () = msg_send![view, setTintColor: &*color(tint)];
                    }
                }
                Kind::Switch => {
                    if let Some(tint) = props.tint {
                        let _: () = msg_send![view, setOnTintColor: &*color(tint)];
                    }
                }
                Kind::Segmented => {
                    let _: () = msg_send![view, removeAllSegments];
                    for (at, item) in props.items.iter().enumerate() {
                        let _: () = msg_send![
                            view,
                            insertSegmentWithTitle: &*ns_string(item),
                            atIndex: at,
                            animated: false
                        ];
                    }
                    let _: () = msg_send![view, setSelectedSegmentIndex: props.selected];
                    if let Some(tint) = props.tint {
                        let _: () = msg_send![view, setSelectedSegmentTintColor: &*color(tint)];
                    }
                }
            }
            let size: CGSize = msg_send![view, intrinsicContentSize];
            self.context
                .set_size(size.width.max(0.0) as f32, size.height.max(0.0) as f32);
        }
    }
}

impl NativeComponent for Control {
    fn view(&self) -> &UIView {
        &self.view
    }

    fn set_prop(&mut self, name: &str, value: Option<&JsonValue>) {
        let view: &AnyObject = &self.view;
        match (self.kind, name) {
            (Kind::Switch, "on") => {
                let on = value.and_then(JsonValue::as_bool).unwrap_or_default();
                // SAFETY: UISwitch's `setOn:animated:`.
                unsafe { msg_send![view, setOn: on, animated: true] }
            }
            (Kind::Segmented, "selected") => {
                self.props.take(name, value);
                // SAFETY: UISegmentedControl's `setSelectedSegmentIndex:`.
                unsafe { msg_send![view, setSelectedSegmentIndex: self.props.selected] }
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
    let (name, action, events): (&std::ffi::CStr, Sel, usize) = match kind {
        Kind::Button => (c"UIButton", sel!(pressed:), PRIMARY_ACTION),
        Kind::Switch => (c"UISwitch", sel!(switched:), VALUE_CHANGED),
        Kind::Segmented => (c"UISegmentedControl", sel!(segmented:), VALUE_CHANGED),
    };
    // SAFETY: the control's designated initializer, then UIControl's
    // documented target-action registration.
    let control: Retained<UIView> = unsafe {
        let control: Retained<UIView> = msg_send![msg_send![class(name), alloc], init];
        let _: () = msg_send![
            &*control,
            addTarget: &*target,
            action: action,
            forControlEvents: events
        ];
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
