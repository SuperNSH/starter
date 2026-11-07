//! Every page's frame: a `nav_page` with the route's key (pages below
//! keep their state), its title for the system header, and the shell's
//! header slot for the desktops and the web, which carry the back button
//! and the palette.

use nsh_base::{nav_page, theme};
use nsh_core::{element::Screen, prelude::*};

use crate::{
    shell::{nav, palette, routes::Route},
    ui::button::{ButtonSize, ButtonVariant, button_with},
};

/// The route's page as the stack shows it.
pub fn page_screen(route: Route) -> Screen {
    let mut screen = nav_page().key(route.key()).title(route.title());
    // A detail over the tab bar's pages keeps the bar out while it is up.
    if route.is_detail() {
        screen = screen.hides_bottom_bar_when_pushed(true);
    }
    screen.header(page_header(route)).child(route.page())
}

/// The header slot (desktops and the web only; iOS and Android draw their
/// system header from `title`). Made once per page: a page is born pushed
/// or root, so the back button's presence is decided here.
fn page_header(route: Route) -> impl IntoElement {
    view()
        .h_12()
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .border_b_1()
        .border_color(move || theme().border)
        .when(route != Route::Home, |header| {
            header.child(button_with(
                "Back",
                ButtonVariant::Ghost,
                ButtonSize::Small,
                false,
                None,
                nav::pop,
            ))
        })
        .child(
            text(route.title())
                .flex_1()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .number_of_lines(1),
        )
        .when(is_desktop(), |header| {
            header.child(button_with(
                format!("Commands  {}", key_label("mod+k")),
                ButtonVariant::Ghost,
                ButtonSize::Small,
                false,
                None,
                palette::toggle,
            ))
        })
}
