//! The system's surfaces and buttons on them: glass buttons and bars, the
//! window's own materials, the toolbar icon button and the alert.

use super::*;

/// A Liquid Glass button: `NSBezelStyleGlass` on macOS 26 and later, the
/// glass button configuration on iOS 26 and later (`prominent`: tinted
/// with the accent). Older systems draw their plain bezel.
pub fn native_glass_button(
    title: &'static str,
    symbol: Option<&'static str>,
    prominent: bool,
    on_press: impl Fn() + 'static,
) -> AnyElement {
    if !(on_macos() || on_ios()) {
        let variant = if prominent {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outline
        };
        return button_with(title, variant, ButtonSize::Medium, false, None, on_press)
            .into_element();
    }
    nsh_core::element::native(GLASS_BUTTON)
        .prop("title", JsonValue::from(title))
        .prop("symbol", JsonValue::from(symbol.unwrap_or_default()))
        .prop(
            "style",
            JsonValue::from(if prominent { "prominent" } else { "regular" }),
        )
        .prop("tint", accent)
        .on_native_event(move |e: &NativeEvent| {
            if e.data["index"].is_number() {
                on_press();
            }
        })
        .self_start()
        .into_element()
}

/// A toolbar icon button: an image-only system button drawing SF Symbol
/// `symbol` (Liquid Glass on macOS 26 and iOS 26, borderless before),
/// named `label` for VoiceOver and its tooltip. Elsewhere the app's icon
/// button drawing `svg`.
pub fn native_icon_button(
    symbol: &'static str,
    svg: &'static str,
    label: &'static str,
    on_press: impl Fn() + 'static,
) -> AnyElement {
    if !(on_macos() || on_ios()) {
        return crate::ui::button::icon_svg_button(svg, label, on_press).into_element();
    }
    nsh_core::element::native(GLASS_BUTTON)
        .prop("title", JsonValue::from(""))
        .prop("symbol", JsonValue::from(symbol))
        .prop("text", JsonValue::from(label))
        .prop("style", JsonValue::from("regular"))
        .on_native_event(move |e: &NativeEvent| {
            if e.data["index"].is_number() {
                on_press();
            }
        })
        .into_element()
}

/// The system's window material behind a bar (`material`: "titlebar",
/// "header", "sidebar" or "window"), blending with what is behind the
/// window as AppKit's own bars do; an absolutely placed layer filling its
/// parent. `None` where AppKit does not draw it: the bar keeps its fill.
pub fn native_material(material: &'static str) -> Option<AnyElement> {
    if !on_macos() {
        return None;
    }
    Some(
        nsh_core::element::native(MATERIAL)
            .prop("style", JsonValue::from(material))
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .into_element(),
    )
}

/// A floating toolbar of SF Symbol buttons on one Liquid Glass pill:
/// `NSGlassEffectView`, `UIGlassEffect` (`clear`: the clear glass, for
/// busy backgrounds). Elsewhere the app's ghost buttons, named `labels`.
pub fn native_glass_bar(
    symbols: &'static [&'static str],
    labels: &'static [&'static str],
    clear: bool,
    on_press: impl Fn(usize) + 'static,
) -> AnyElement {
    if !(on_macos() || on_ios()) {
        let on_press = std::rc::Rc::new(on_press);
        return view()
            .flex_row()
            .self_start()
            .gap_1()
            .p_1()
            .rounded_full()
            // No system glass control here: NSH's glass (its blur where
            // the platform has no Liquid Glass) under a translucent fill.
            .glass_effect(nsh_core::GlassEffect::Clear)
            .backdrop_blur(px(30.0))
            .bg(move || theme().card.opacity(0.45))
            .border_1()
            .border_color(move || theme().border.opacity(0.5))
            .children(labels.iter().enumerate().map(|(at, label)| {
                let on_press = on_press.clone();
                button_with(
                    *label,
                    ButtonVariant::Ghost,
                    ButtonSize::Small,
                    false,
                    None,
                    move || on_press(at),
                )
            }))
            .into_element();
    }
    nsh_core::element::native(GLASS_BAR)
        .prop("items", JsonValue::from(symbols.to_vec()))
        .prop("bodies", JsonValue::from(labels.to_vec()))
        .prop(
            "style",
            JsonValue::from(if clear { "clear" } else { "regular" }),
        )
        .on_native_event(move |e: &NativeEvent| {
            if let Some(at) = e.data["index"].as_u64() {
                on_press(at as usize);
            }
        })
        .self_start()
        .into_element()
}

/// The system's alert (`sheet`: its action sheet on iOS; macOS shows both
/// as a sheet on the window). `on_choice` gets the chosen action's label,
/// `None` on cancel.
pub fn native_alert(
    title: &'static str,
    message: &'static str,
    actions: &'static [&'static str],
    destructive: Option<&'static str>,
    sheet: bool,
    on_choice: impl Fn(Option<&'static str>) + 'static,
) {
    spawn(async move {
        let mut dialog = nsh_base::alert_dialog()
            .title(title)
            .description(message)
            .cancel("cancel", "Cancel");
        for action in actions {
            dialog = dialog.action(*action, *action);
        }
        if let Some(label) = destructive {
            dialog = dialog.destructive(label, label);
        }
        if sheet {
            dialog = dialog.action_sheet();
        }
        let answer = dialog.show().await.ok().and_then(|r| r.action);
        let chosen = answer.and_then(|id| {
            actions
                .iter()
                .chain(destructive.iter())
                .find(|a| **a == id.as_ref())
                .copied()
        });
        on_choice(chosen);
    });
}
