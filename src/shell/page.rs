//! Every page's frame: a `nav_page` with the route's key (pages below
//! keep their state), its title for the system header, and the shell's
//! header slot for the desktops and the web, which carry the back button
//! and the palette. What differs from page to page is read from the
//! page's record (`Page::frame`, `Page::action`), never from its name.

use nsh_base::{nav_page, theme};
use nsh_core::{element::Screen, prelude::*};

use crate::{
    native::{kit::native_icon_button, soft_edges},
    pages,
    routes::{Content, Frame, Route},
    shell::{chrome, desktop, palette},
    store::nav,
    ui::{icons, layout, nav::themed_header},
};

/// The widest the page content grows, in points: past it, label and value
/// columns drift too far apart to read as one row.
const CONTENT_MAX: f32 = 1040.0;

/// The route's page as the stack shows it.
pub fn page_screen(route: Route) -> Screen {
    let page = route.page();
    let phone = !is_desktop();
    let ios = os() == nsh_core::platform::Os::Ios;
    let own_stack = page.frame == Frame::OwnStack;
    let pushed = route.parent().is_some();
    let mut screen = themed_header(nav_page().key(route.stack_key()).title(route.title()));
    // A pushed screen over the tab bar's pages keeps the bar out while it
    // is up.
    if pushed {
        screen = screen.hides_bottom_bar_when_pushed(true);
    }
    // A tab is a root on phones: the tab bar, not a back button, leaves it
    // (Android's system back still returns Home, the shell's back order).
    if phone && pages::tabs().contains(&route) {
        screen = screen.header_back_visible(false).pop_on_swipe(false);
    }
    // One system header per screen: a page with its own stack draws it.
    if phone && own_stack {
        screen = screen.header_visible(false);
    }
    // A fixed page's top is not a scroll view: it keeps the same clear
    // header as every page, but starts below it and stops above the
    // floating tab bar, since nothing scrolls under either.
    let fixed = phone && page.frame == Frame::Fixed;
    // The page's own action as a native header button: UIKit draws the SF
    // Symbol, and a press on it presses this view's button.
    if let Some(action) = &page.action
        && ios
        && phone
    {
        screen = screen
            .header_right(
                view().test_id(action.test_id).child(
                    nsh_base::button()
                        .on_press(action.run)
                        .child(text(action.label)),
                ),
            )
            .header_right_symbol(action.symbol);
    }
    // Under the transparent iOS header, UIKit's soft edge: content fades
    // and blurs lightly as it slides under. A page with its own stack
    // places its own (`soft_edges::page_edges`).
    let top_fade = ios && phone && !own_stack;
    // nav_page paints the platform's window color (macOS dark: #1e1e1e),
    // so the content area takes the theme's own background on every page;
    // wide windows center the content at a readable measure.
    screen.header(page_header(route)).child(
        view()
            .flex_1()
            .items_center()
            .when(fixed, |page| {
                // The screen's own top inset, which takes in its header.
                page.pt_safe_area()
                    .pb(move || px(layout::tab_bar_room() + safe_area().bottom.0))
            })
            .bg(move || theme().background)
            .child(
                view()
                    .flex_1()
                    .w_full()
                    .max_w(px(CONTENT_MAX))
                    .child(content(route)),
            )
            .child(when(move || top_fade, soft_edges::soft_edges))
            // ...and behind the floating tab bar, the bottom one.
            .child(when(
                move || top_fade && !fixed && !pushed,
                || soft_edges::soft_bottom_edge(|| layout::tab_bar_room() + safe_area().bottom.0),
            )),
    )
}

/// What the route's page draws, given the route's id when it shows one
/// item.
fn content(route: Route) -> AnyElement {
    match (&route.page().content, route.id()) {
        (Content::Plain(draw), _) => draw(),
        (Content::WithId(draw), Some(id)) => draw(id),
        // The page table pairs every `WithId` record with a route that
        // carries an id (`pages::tests`), so this draws nothing in practice.
        (Content::WithId(_), None) => view().into_element(),
    }
}

/// The header slot (desktops and the web only; iOS and Android draw their
/// system header from `title`), drawn as a unified toolbar: draggable, as
/// tall as the title bar, clear of the window controls while the sidebar
/// is hidden. A page is born pushed or root, so the back button's
/// presence is decided here.
fn page_header(route: Route) -> impl IntoElement {
    let lead = move || {
        if chrome::sidebar_open() || !desktop() {
            px(12.0)
        } else {
            chrome::controls_inset() + px(12.0)
        }
    };
    view()
        .h(px(chrome::TITLE_BAR))
        .flex_row()
        .items_center()
        .gap_1()
        .pl(lead)
        .pr_3()
        .border_b_1()
        .border_color(move || theme().border)
        .bg(move || theme().background)
        .window_drag(true)
        .child(when(
            // The narrow layout has no sidebar to show.
            move || desktop() && !chrome::sidebar_open(),
            || {
                native_icon_button(
                    "sidebar.left",
                    icons::SIDEBAR,
                    "Show sidebar",
                    chrome::toggle_sidebar,
                )
            },
        ))
        .when(route != Route::Home, |header| {
            header.child(native_icon_button(
                "chevron.left",
                icons::BACK,
                "Back",
                nav::pop,
            ))
        })
        .child(
            text(route.title())
                .flex_1()
                .px_1()
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD)
                .number_of_lines(1),
        )
        .when(is_desktop(), |header| {
            header.child(native_icon_button(
                "magnifyingglass",
                icons::SEARCH,
                "Commands",
                palette::toggle,
            ))
        })
}
