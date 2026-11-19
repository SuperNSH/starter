//! Deep links: [`route_from_deep_link`](crate::shell::routes) parses the
//! path. The system hands the app links of the scheme `nsh.json` claims
//! (`schemes`) through `next_url_scheme_open`, the one that launched it
//! included ([`listen`]); a desktop launch argument and an embedding host's
//! `host.on` wiring call [`open`] too.

use std::sync::OnceLock;

use nsh_core::prelude::*;

use crate::shell::{
    nav,
    routes::{Route, route_from_deep_link},
};

/// The scheme the app answers to.
pub const SCHEME: &str = "nsh-starter://";

/// A deep link from the desktop launch arguments, read once at startup.
static PENDING: OnceLock<String> = OnceLock::new();

/// Stages `url` from the launch arguments (`main`, before the engine ran).
pub fn from_launch(url: impl Into<String>) {
    let _ = PENDING.set(url.into());
}

/// Opens `url` (`nsh-starter://<segment>/<segment>…`): the route it names,
/// or `None` when it is not one of the app's. The seam an embedding host's
/// delivery wires into.
pub fn open(url: &str) -> Option<Route> {
    let path = url.strip_prefix(SCHEME)?;
    let route = route_from_deep_link(&path.split('/').collect::<Vec<_>>())?;
    nav::open(route);
    Some(route)
}

/// Opens each link the system hands the app, for as long as the shell lives.
pub fn listen() {
    let owner = Owner::current();
    spawn(async move {
        loop {
            let url = next_url_scheme_open().await;
            if !owner.is_live() {
                break;
            }
            open(&url);
        }
    });
}

/// Opens the launch argument, once, from the shell's install.
pub fn open_pending() {
    if let Some(url) = PENDING.get() {
        open(url);
    }
}
