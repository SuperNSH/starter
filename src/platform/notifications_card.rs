//! The notifications card: the system's permission state and a test
//! notification, through `nsh-core`'s `Notifications` module (`notifications`
//! capability). Where the platform shows none, the card says so.

use nsh_base::theme;
use nsh_core::{
    notifications::{Notification, NotificationPermission},
    prelude::*,
};

// nsh_core's prelude has a `t!` of its own; the app translates with rust-i18n.
use rust_i18n::t;

use super::{
    locale,
    widgets::{action_button, hint, panel, quiet_button},
};

fn permission_key(permission: Option<NotificationPermission>) -> &'static str {
    match permission {
        None => "platform.notifications.checking",
        Some(NotificationPermission::Default) => "platform.notifications.default",
        Some(NotificationPermission::Granted) => "platform.notifications.granted",
        Some(NotificationPermission::Denied) => "platform.notifications.denied",
        Some(NotificationPermission::Unavailable) => "platform.notifications.unavailable",
    }
}

#[component]
fn NotificationsCard() -> impl IntoElement {
    let permission = signal(None::<NotificationPermission>);
    let sent = signal(None::<bool>);
    let owner = Owner::current();
    spawn(async move {
        let answer = notification_permission().await;
        if owner.is_live() {
            permission.set(Some(answer));
        }
    });
    panel(
        "platform.notifications.title",
        [
            text(move || {
                locale::app_locale().get();
                t!(permission_key(permission.get())).to_string()
            })
            .text_sm()
            .test_id("platform-notifications-status")
            .text_color(move || {
                if permission.get() == Some(NotificationPermission::Granted) {
                    theme().foreground
                } else {
                    theme().muted_foreground
                }
            })
            .into_element(),
            quiet_button(
                "platform-notifications-ask",
                move || {
                    locale::app_locale().get();
                    t!("platform.notifications.ask").to_string()
                },
                move || {
                    let owner = Owner::current();
                    spawn(async move {
                        let answer = request_notification_permission().await;
                        if owner.is_live() {
                            permission.set(Some(answer));
                        }
                    });
                },
            )
            .into_element(),
            action_button(
                "platform-notifications-send",
                move || {
                    locale::app_locale().get();
                    t!("platform.notifications.send").to_string()
                },
                move || {
                    let owner = Owner::current();
                    spawn(async move {
                        let shown = show_notification(Notification::new(
                            "starter-demo",
                            t!("platform.notifications.demo_title").to_string(),
                            t!("platform.notifications.demo_body").to_string(),
                        ))
                        .await;
                        if owner.is_live() {
                            sent.set(Some(shown));
                        }
                    });
                },
            )
            .into_element(),
            text(move || {
                locale::app_locale().get();
                match sent.get() {
                    None => String::new(),
                    Some(true) => t!("platform.notifications.sent").to_string(),
                    Some(false) => t!("platform.notifications.refused").to_string(),
                }
            })
            .text_sm()
            .test_id("platform-notifications-result")
            .into_element(),
            hint("platform.notifications.note"),
        ],
    )
}

/// The notifications card, as the platform page places it.
pub(crate) fn card() -> AnyElement {
    view().child(notifications_card()).into_element()
}
