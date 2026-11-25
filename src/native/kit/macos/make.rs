//! Building each AppKit control: the view for a kind, wired to its
//! target, and the parts of a tab view.

use super::*;

pub(super) fn make(kind: Kind, context: ComponentContext) -> Box<dyn NativeComponent> {
    let mtm = MainThreadMarker::new().expect("components are made on the main thread");
    let target = Target::new(mtm, kind, context.clone());
    // SAFETY: each control's documented initializer, then NSControl's
    // target and action (or the delegate the kind reports through).
    let view: Retained<NSView> = unsafe {
        let name: &CStr = match kind {
            Kind::Slider => c"NSSlider",
            Kind::Checkbox => c"NSButton",
            Kind::Radio => c"NSStackView",
            Kind::Popup => c"NSPopUpButton",
            Kind::Stepper => c"NSStepper",
            Kind::Progress | Kind::Spinner => c"NSProgressIndicator",
            Kind::Date => c"NSDatePicker",
            Kind::Color => c"NSColorWell",
            Kind::Text => c"NSTextField",
            Kind::Search => c"NSSearchField",
            Kind::Tabs => c"NSView",
            Kind::GlassButton => c"NSButton",
            // An older macOS gets the vibrant material in its place.
            Kind::GlassBar if glass() => c"NSGlassEffectView",
            Kind::GlassBar | Kind::Material => c"NSVisualEffectView",
        };
        let view: Retained<NSView> =
            msg_send![msg_send![class(name), alloc], initWithFrame: rect(160.0, 24.0)];
        let v: &AnyObject = &view;
        match kind {
            Kind::Radio => {
                // NSUserInterfaceLayoutOrientationVertical, leading.
                let _: () = msg_send![v, setOrientation: 1_isize];
                let _: () = msg_send![v, setAlignment: 5_isize];
                let _: () = msg_send![v, setSpacing: 6.0_f64];
            }
            // A backdrop: no target, no action.
            Kind::Tabs | Kind::Material => {}
            Kind::GlassBar => {
                let stack: Retained<NSView> = msg_send![
                    msg_send![class(c"NSStackView"), alloc],
                    initWithFrame: rect(160.0, 44.0)
                ];
                // Horizontal, buttons spread evenly over the pill.
                let _: () = msg_send![&*stack, setOrientation: 0_isize];
                let _: () = msg_send![&*stack, setDistribution: 1_isize];
                let _: () = msg_send![&*stack, setSpacing: 0.0_f64];
                if glass() {
                    let _: () = msg_send![v, setCornerRadius: 22.0_f64];
                    let _: () = msg_send![v, setContentView: &*stack];
                } else {
                    let _: () = msg_send![v, setWantsLayer: true];
                    let layer: Retained<AnyObject> = msg_send![v, layer];
                    let _: () = msg_send![&*layer, setCornerRadius: 22.0_f64];
                    let _: () = msg_send![&*stack, setAutoresizingMask: 2_usize | 16_usize];
                    let _: () = msg_send![v, addSubview: &*stack];
                }
            }
            Kind::Text | Kind::Search => {
                let _: () = msg_send![v, setDelegate: &*target];
                if kind == Kind::Text {
                    let _: () = msg_send![v, setBezeled: true];
                    // NSTextFieldRoundedBezel.
                    let _: () = msg_send![v, setBezelStyle: 1_usize];
                }
            }
            Kind::Progress | Kind::Spinner => {
                let spinning = kind == Kind::Spinner;
                // NSProgressIndicatorStyleBar, Spinning.
                let _: () = msg_send![v, setStyle: usize::from(spinning)];
                let _: () = msg_send![v, setIndeterminate: spinning];
                let _: () = msg_send![v, setMinValue: 0.0_f64];
                let _: () = msg_send![v, setMaxValue: 1.0_f64];
                if spinning {
                    let none: Option<&AnyObject> = None;
                    let _: () = msg_send![v, startAnimation: none];
                }
            }
            _ => {
                let _: () = msg_send![v, setTarget: &*target];
                let _: () = msg_send![v, setAction: sel!(act:)];
            }
        }
        match kind {
            // NSButtonTypeSwitch.
            Kind::Checkbox => {
                let _: () = msg_send![v, setButtonType: 3_usize];
            }
            Kind::Slider => {
                let _: () = msg_send![v, setContinuous: true];
            }
            Kind::Stepper => {
                let _: () = msg_send![v, setValueWraps: false];
            }
            Kind::Date => {
                // NSDatePickerElementFlagYearMonthDay.
                let _: () = msg_send![v, setDatePickerElements: 0x00e0_usize];
                let _: () = msg_send![v, setBezeled: true];
            }
            // NSColorWellStyleExpanded (macOS 13): the swatch with the
            // panel's button beside it.
            Kind::Color if v.class().responds_to(sel!(setColorWellStyle:)) => {
                let _: () = msg_send![v, setColorWellStyle: 2_isize];
            }
            _ => {}
        }
        view
    };
    // SAFETY: as above: NSBox, NSSegmentedControl and NSTextField's
    // documented API.
    let tabs = (kind == Kind::Tabs).then(|| unsafe { tab_parts(&view, &target) });
    let mut control = Control {
        kind,
        view,
        target,
        context,
        props: Props::new(),
        built: Vec::new(),
        tabs,
    };
    control.apply();
    Box::new(control)
}

/// A tab view as AppKit draws one: a box with a segmented control over
/// its top edge. `NSTabView` itself draws its tab strip blank inside the
/// nsh host (both appearances), so its parts are composed here.
///
/// # Safety
/// `container` is a plain NSView.
pub(super) unsafe fn tab_parts(
    container: &NSView,
    target: &Target,
) -> (Retained<AnyObject>, Retained<AnyObject>) {
    let (w, h) = (400.0, 150.0);
    // SAFETY: the caller's promise; NSView, NSBox, NSSegmentedControl
    // and NSTextField's documented API.
    unsafe {
        let _: () = msg_send![container, setFrame: rect(w, h)];
        let boxed: Retained<NSView> =
            msg_send![msg_send![class(c"NSBox"), alloc], initWithFrame: rect(w, h - 11.0)];
        // NSNoTitle; width and height follow the container.
        let _: () = msg_send![&*boxed, setTitlePosition: 0_usize];
        let _: () = msg_send![&*boxed, setAutoresizingMask: 2_usize | 16_usize];
        let _: () = msg_send![container, addSubview: &*boxed];
        let line: Retained<AnyObject> = msg_send![
            class(c"NSTextField"),
            wrappingLabelWithString: &*ns_string("")
        ];
        let content: Retained<NSView> = msg_send![&*boxed, contentView];
        let inner: NSRect = msg_send![&*content, bounds];
        let _: () = msg_send![
            &*line,
            setFrame: NSRect::new(
                NSPoint::new(12.0, inner.size.height - 52.0),
                NSSize::new(inner.size.width - 24.0, 36.0)
            )
        ];
        // Width sizable, pinned to the top.
        let _: () = msg_send![&*line, setAutoresizingMask: 2_usize | 8_usize];
        let _: () = msg_send![&*content, addSubview: &*line];
        let segmented: Retained<AnyObject> = msg_send![
            msg_send![class(c"NSSegmentedControl"), alloc],
            initWithFrame: rect(240.0, 24.0)
        ];
        // NSSegmentSwitchTrackingSelectOne.
        let _: () = msg_send![&*segmented, setTrackingMode: 0_usize];
        let _: () = msg_send![&*segmented, setTarget: target];
        let _: () = msg_send![&*segmented, setAction: sel!(act:)];
        // Centered, at the top, as the container resizes.
        let _: () = msg_send![&*segmented, setAutoresizingMask: 1_usize | 4_usize | 8_usize];
        let _: () = msg_send![container, addSubview: &*segmented];
        (segmented, line)
    }
}
