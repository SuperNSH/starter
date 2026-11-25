//! The forms section's field layout: sections, labelled rows that scroll
//! into view on a failed submit, and the page header, over the generic form
//! glue in `ui::form` (re-exported here for the section's screens).

use nsh_base::theme;
use nsh_core::prelude::*;

pub use crate::ui::form::{
    SubmitRule, error_line, field_label, hint_line, input_shell, message_banner, submit_first_form,
};
use crate::ui::text;

/// A section of fields: the ui card's box (rebuilt here: `ui::layout::card`
/// returns `impl IntoElement`, which takes no children) and its title.
pub fn field_section(title: &'static str, fields: impl IntoIterator<Item = View>) -> View {
    view()
        .w_full()
        .flex_col()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_color(move || theme().border)
        .bg(move || theme().card)
        .p_4()
        .text_color(move || theme().card_foreground)
        .child(text::section_title(title))
        .children(fields)
}

/// A labelled field: label, the control, its error.
pub fn field_row(
    label: &'static str,
    control: impl IntoElement,
    error: impl Fn() -> Option<String> + Copy + 'static,
) -> View {
    revealed_row(
        view()
            .flex_col()
            .gap_1p5()
            .child(field_label(label))
            .child(control)
            .child(error_line(error)),
        move || error().is_some(),
    )
}

/// Room kept above a row the failed submit scrolls to: the platform's own
/// reveal margin, so the label sits where the input alone would have.
const ROW_REVEAL_MARGIN: f32 = 16.0;

/// Scrolls the first failing row, label and all, into view after a failed
/// submit. nsh-form focuses the first invalid input there, and the
/// platform's focus reveal brings only that input into view, which leaves
/// the label above it under the page header. A screen makes one, wraps
/// its scroll and content with it, and `provide`s it; every row made by
/// [`field_row`] (or passed to [`revealed_row`]) under it then registers.
#[derive(Clone)]
pub struct RowReveal {
    scroll: nsh_core::element::ElementRef,
    content: nsh_core::element::ElementRef,
    /// The scroll's top content inset, which `scroll_to` adds back.
    inset_top: f32,
    rows: std::rc::Rc<std::cell::RefCell<Vec<RevealEntry>>>,
}

type RevealEntry = (nsh_core::element::ElementRef, std::rc::Rc<dyn Fn() -> bool>);

impl RowReveal {
    /// One per screen, for a scroll with `inset_top` points of top inset;
    /// later rows find it through the context.
    pub fn provide<H: Clone + PartialEq + 'static>(
        form: nsh_form::Form<H>,
        inset_top: f32,
    ) -> Self {
        let reveal = Self {
            scroll: element_ref(),
            content: element_ref(),
            inset_top,
            rows: Default::default(),
        };
        provide_context(reveal.clone());
        let hook = reveal.clone();
        form.on_submit_invalid(move |_: &H| hook.reveal_first_failing());
        reveal
    }

    /// The page's scroll view.
    pub fn scroll(&self, scroll: nsh_core::element::Scroll) -> nsh_core::element::Scroll {
        scroll.element_ref(&self.scroll)
    }

    /// The scroll's one content child.
    pub fn content(&self, content: View) -> View {
        content.element_ref(&self.content)
    }

    /// Scrolls to the first row (in page order) whose field fails. The
    /// form's focus went out before this runs, so this scroll lands after
    /// the platform's reveal and replaces it.
    fn reveal_first_failing(&self) {
        let failing = self
            .rows
            .borrow()
            .iter()
            .find(|(_, failed)| failed())
            .map(|(row, _)| *row);
        let Some(row) = failing else { return };
        let (scroll, content, inset_top) = (self.scroll, self.content, self.inset_top);
        spawn(async move {
            let (Some(row), Some(content)) = (row.measure().await, content.measure().await) else {
                return;
            };
            let top = row.page.y.0 - content.page.y.0 - ROW_REVEAL_MARGIN - inset_top;
            scroll.scroll_to(Point::new(px(0.0), px(top.max(-inset_top))), true);
        });
    }
}

/// Registers a field row with the screen's [`RowReveal`], when it has one.
pub fn revealed_row(row: View, failed: impl Fn() -> bool + 'static) -> View {
    let Some(reveal) = use_context::<RowReveal>() else {
        return row;
    };
    let target = element_ref();
    reveal
        .rows
        .borrow_mut()
        .push((target, std::rc::Rc::new(failed)));
    row.element_ref(&target)
}

/// The app's header for the desktops and the web (iOS and Android draw the
/// system one from `title`): its back button pops the feature's own stack,
/// so only a page the stack can pop passes `on_back`.
pub fn page_header(title: &'static str, on_back: Option<impl Fn() + 'static>) -> View {
    view()
        .min_h(px(40.0))
        .flex_row()
        .items_center()
        .gap_1()
        .px_3()
        .border_b_1()
        .border_color(move || theme().border)
        .bg(move || theme().background)
        .children(on_back.map(|on_back| {
            crate::ui::button::icon_svg_button(crate::ui::icons::BACK, "Back", on_back)
        }))
        .child(text::label(title))
}
