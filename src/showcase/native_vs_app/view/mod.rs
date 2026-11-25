//! Native vs app: each control twice, the app's own beside the system's
//! (`native::controls`, `native::kit`), bound to one signal, so a change on
//! either side shows on both and a screen can pick per control.

use nsh_base::{Date, theme};
use nsh_core::{
    element::{ImageAsset, asset},
    prelude::*,
};

use crate::native::controls::{NativeButtonStyle, native_button, native_segmented, native_switch};
use crate::native::kit::{
    native_alert, native_checkbox, native_color_well, native_date, native_glass_bar,
    native_glass_button, native_popup, native_progress, native_radio_group, native_slider,
    native_spinner, native_stepper, native_tabs, native_text_field, on_ios, on_macos,
};
use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use crate::ui::layout::{BadgeTone, badge};
use crate::ui::{
    color, controls, dialog, field, layout, number, pickers, select, sheet, tabs, text, toast,
};

/// Glass needs something behind it to read as glass: the canvas page's
/// photo.
const BACKDROP: ImageAsset = asset!("canvas/photo.png");
const BAR_SYMBOLS: &[&str] = &["bold", "italic", "underline", "link"];
const BAR_LABELS: &[&str] = &["Bold", "Italic", "Underline", "Link"];
const SIZES: &[&str] = &["Small", "Medium", "Large"];
const PLANS: &[&str] = &["Free", "Pro", "Team"];
const VIEWS: &[&str] = &["List", "Grid", "Board"];
const PANES: &[&str] = &["General", "Privacy", "Advanced"];
const PANE_BODIES: &[&str] = &[
    "Name, appearance and the default page.",
    "What the app may read, and what it shares.",
    "Logging, caches and developer switches.",
];

/// The Native vs app page.
pub fn native_vs_app_page() -> impl IntoElement {
    let open_dialog = signal(false);
    let open_sheet = signal(false);
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            scroll()
                .flex_1()
                .content_inset(layout::page_insets(0.0))
                .test_id("native-scroll")
                .child(
                    layout::page_body()
                        .max_w(px(860.0))
                        .child(text::page_intro(
                            "Native vs app",
                            "The app's control on the left, the system's on the right, \
                             sharing one value. Change either and both follow.",
                        ))
                        .child(text::muted(platform_note()))
                        .child(glass_row())
                        .child(buttons_row())
                        .child(switch_row())
                        .child(checkbox_row())
                        .child(radio_row())
                        .child(segmented_row())
                        .child(tabs_row())
                        .child(popup_row())
                        .child(text_row())
                        .child(slider_row())
                        .child(stepper_row())
                        .child(date_row())
                        .child(calendar_row())
                        .child(color_row())
                        .child(alerts_row(open_dialog, open_sheet)),
                ),
        )
        .child(dialog::dialog_card(
            open_dialog,
            "Discard changes?".into(),
            move || open_dialog.set(false),
            || text::body("Your edits since the last save are lost.").into_element(),
            move || {
                view()
                    .flex_row()
                    .justify_end()
                    .gap_2()
                    .child(button_with(
                        "Cancel",
                        ButtonVariant::Outline,
                        ButtonSize::Small,
                        false,
                        None,
                        move || open_dialog.set(false),
                    ))
                    .child(button_with(
                        "Discard",
                        ButtonVariant::Danger,
                        ButtonSize::Small,
                        false,
                        None,
                        move || open_dialog.set(false),
                    ))
                    .into_element()
            },
        ))
        .child(sheet::sheet_of(open_sheet, Placement::Bottom, || {
            view()
                .flex_col()
                .gap_3()
                .child(text::section_title("Share"))
                .child(text::muted("The app's sheet: any content, the app's look."))
                .child(controls::switch_row("Include comments", signal(true)))
                .into_element()
        }))
        .child(toast::toast_host())
}

fn platform_note() -> &'static str {
    if on_macos() {
        "On macOS AppKit draws the right column."
    } else if on_ios() {
        "On iOS UIKit draws the right column. iOS has no checkbox, radio group or \
         in-page tabs, so those show the app's control there."
    } else {
        "This platform has no system controls wired here: the right column falls \
         back to the app's own. Run on macOS or iOS to compare."
    }
}

/// One control, compared: a title, a line on when to use which, the two
/// columns, and the shared value.
fn compare(
    title: &'static str,
    advice: &'static str,
    app: impl IntoElement,
    native: impl IntoElement,
    readout: impl Fn() -> String + 'static,
) -> impl IntoElement {
    let column = |tag: &'static str, tone: BadgeTone, body: AnyElement| {
        view()
            .flex_1()
            .min_w(px(260.0))
            .flex_col()
            .gap_3()
            .rounded_lg()
            .border_1()
            .border_color(move || theme().border)
            .bg(move || theme().card)
            .p_4()
            .child(view().flex_row().child(badge(tag, tone)))
            .child(body)
    };
    view()
        .flex_col()
        .gap_2()
        .child(text::section_title(title))
        .child(text::muted(advice))
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_3()
                .child(column("App", BadgeTone::Neutral, app.into_element()))
                .child(column("System", BadgeTone::Primary, native.into_element())),
        )
        .child(text::code(move || SharedString::from(readout())))
}

/// The photo with `controls` floating over it.
fn over_photo(controls: impl IntoElement) -> impl IntoElement {
    view()
        .w_full()
        .h(px(220.0))
        .rounded_lg()
        .overflow_hidden()
        .child(
            image(BACKDROP)
                .object_fit(ResizeMode::Cover)
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .bottom_0(),
        )
        .child(
            view()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .bottom_0()
                .p_4()
                .flex_col()
                .justify_between()
                .child(controls),
        )
}

mod rows;

use rows::*;
