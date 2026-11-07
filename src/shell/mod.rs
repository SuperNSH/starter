//! The shell: lifecycle, the typed route stack, deep links, the sidebar on
//! desktop widths and tabs on phones, the command palette, shortcuts,
//! undo and redo, the desktop chrome, the status bar and the Android back
//! order. The rest of the crate hangs off the route enum here.

pub mod back;
pub mod chrome;
pub mod deep_link;
pub mod home;
pub mod lifecycle;
pub mod nav;
pub mod page;
pub mod palette;
pub mod routes;
pub mod status;
pub mod undo;

use nsh_base::{button, nav_stack, theme};
use nsh_core::prelude::*;

use crate::{
    shell::routes::Route,
    ui::{
        tab_bar::{mobile_tab_bar, tab_item},
        toast,
    },
};

/// The width the desktop shell needs: the sidebar and the pane beside it.
/// Desktops take it from 700 pt, other platforms from 900 pt (a wide
/// tablet window); below it is the phone shell. Read in `when`, so the
/// shell remakes once when the window crosses the threshold.
pub fn desktop() -> bool {
    window().width >= if is_desktop() { px(700.0) } else { px(900.0) }
}

/// Flips the system's appearance between light and dark.
pub fn toggle_appearance() {
    if dark() {
        set_color_scheme(crate::Appearance::Light);
    } else {
        set_color_scheme(crate::Appearance::Dark);
    }
}

/// The shell-wide wiring: shortcuts, the back order, the lifecycle watch
/// and the launch deep link. Once, from the root's body.
pub fn install() {
    on_key("mod+k", move |_| palette::toggle());
    on_key("mod+z", move |_| undo::undo());
    on_key("mod+shift+z", move |_| undo::redo());
    back::install();
    lifecycle::watch();
    deep_link::open_pending();
}

/// The shell: the layout of the window's width, with the palette, the
/// exit ask and the toast host over it all.
pub fn shell() -> impl IntoElement {
    view()
        .flex_1()
        .bg(move || theme().background)
        .child(when(desktop, desktop_shell).otherwise(phone_shell))
        .child(palette::palette_dialog())
        .child(back::exit_prompt_dialog())
        // App level, so a toast from any page or task reaches the host.
        .child(toast::toast_host())
}

/// Phones: the stack of Home and the pushed pages, the status bar and the
/// system tab bar for the primary pages.
fn phone_shell() -> AnyElement {
    let tabs = [
        Route::Home,
        Route::Forms,
        Route::Data,
        Route::Lists,
        Route::Motion,
    ];
    // One past the tabs selects none (a route the bar does not show).
    let selected = signal(tabs.len() as u32);
    // The stack drives the tabs...
    effect({
        let selected = selected;
        move || {
            let at = tabs
                .iter()
                .position(|tab| *tab == nav::current())
                .map_or(tabs.len() as u32, |at| at as u32);
            selected.set(at);
        }
    });
    // ...and the tabs drive the stack (a no-op when this effect itself
    // moved the selection, so the two never loop).
    effect(move || {
        if let Some(route) = tabs.get(selected.get() as usize)
            && nav::current() != *route
        {
            nav::open(*route);
        }
    });
    view()
        .flex_1()
        .flex_col()
        .child(
            nav_stack()
                .flex_1()
                .child(page::page_screen(Route::Home))
                .child(pushed()),
        )
        .child(status::status_bar())
        .child(
            // Off the detail pages; the bar itself shows nothing off
            // iOS and Android.
            view()
                .style_when(move || nav::current().is_detail(), |s| s.hidden())
                .child(mobile_tab_bar(
                    tabs.iter()
                        .map(|route| tab_item(route.label_str(), None))
                        .collect::<Vec<_>>(),
                    selected,
                )),
        )
        .into_element()
}

/// Desktop widths: the sidebar and the pane beside it, under the app's own
/// title bar.
fn desktop_shell() -> AnyElement {
    chrome::install();
    view()
        .flex_1()
        .flex_col()
        .child(chrome::title_bar())
        .child(
            view().flex_1().flex_row().child(sidebar()).child(
                nav_stack()
                    .flex_1()
                    .child(page::page_screen(Route::Home))
                    .child(pushed()),
            ),
        )
        .child(status::status_bar())
        .into_element()
}

/// The pushed pages, bottom to top, each dropping its route when the user
/// popped it natively.
fn pushed() -> AnyElement {
    let routes = nav::stack();
    each(
        move || routes.with(|routes| routes.0.clone()),
        |route: &Route| route.key(),
        |route, depth| {
            let route = route.get_untracked();
            page::page_screen(route).on_dismiss(move || nav::truncate(depth.get_untracked()))
        },
    )
    .into_element()
}

/// The desktop layout's sidebar: every page, the one on top selected.
fn sidebar() -> impl IntoElement {
    view()
        .w_64()
        .flex_col()
        .bg(move || theme().card)
        .border_r_1()
        .border_color(move || theme().border)
        .child(
            view()
                .px_4()
                .pt_4()
                .pb_2()
                .gap_1()
                .child(
                    text(t!("app.name").to_string())
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(move || theme().card_foreground),
                )
                .child(
                    text("Boilerplate for Native Shell")
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                ),
        )
        .child(
            scroll().flex_1().child(
                view()
                    .px_2()
                    .pb_4()
                    .gap_1()
                    .child(
                        text("Pages")
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(move || theme().muted_foreground)
                            .px_2()
                            .pt_2()
                            .pb_1(),
                    )
                    .children(Route::ALL.iter().copied().map(sidebar_row)),
            ),
        )
}

/// A sidebar row: a raised fill while its page is on top, an accent fill
/// under the pointer otherwise. The test id rides a wrapper, as a
/// component element takes none of its own.
fn sidebar_row(route: Route) -> AnyElement {
    let selected = move || nav::current() == route;
    button()
        .on_press(move || nav::open(route))
        .selected(selected)
        .selected_style(|s| s.bg(move || theme().secondary))
        .hover(|s| s.bg(move || theme().accent))
        .focus(|s| s.outline(px(2.0)).outline_color(move || theme().ring))
        .h_9()
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .rounded(move || theme().radius)
        .child(
            // The row's test id rides this view, inside the button: a
            // press on it reaches the button, as a tap on a label does. A
            // style keeps the view mounted: an unstyled one collapses
            // into its parent and the id never mounts.
            view()
                .test_id(&format!("nav-{}", route.key()))
                .flex_1()
                .child(
                    text(route.label())
                        .text_sm()
                        .number_of_lines(1)
                        .text_color(move || {
                            if selected() {
                                theme().card_foreground
                            } else {
                                theme().muted_foreground
                            }
                        }),
                ),
        )
        .into_element()
}
