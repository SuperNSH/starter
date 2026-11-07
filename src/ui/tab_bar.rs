//! The system tab bar (`nsh/mobile`): iOS's UITabBar and Android's
//! navigation bar, in the system look. It takes no room on desktops and
//! the web, where the shell shows its tabs with `nsh/base`.

use nsh_core::mobile::{TabBarItem, tab_bar};
use nsh_core::prelude::*;

/// One tab of the mobile bar: a title and, per platform, an icon name (an
/// SF Symbol on iOS, a drawable on Android).
pub fn tab_item(title: &'static str, icon: Option<&'static str>) -> TabBarItem {
    let item = TabBarItem::new(title);
    match icon {
        Some(icon) => item.icon(icon),
        None => item,
    }
}

/// The mobile tab bar over `items`, the chosen one in `selected`; the
/// user's choice is reported and the caller shows that tab's page.
pub fn mobile_tab_bar(items: Vec<TabBarItem>, selected: Signal<u32>) -> impl IntoElement {
    tab_bar()
        .items(items)
        .selected_index(selected)
        .on_change(move |index: &u32| selected.set(*index))
}
