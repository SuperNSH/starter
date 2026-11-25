//! Who is signed in: the user the auth seam (`services::auth`) returned,
//! kept as a signal so every screen that shows it redraws on a change.
//! The app starts signed out; nothing is gated on a session.

use nsh_core::prelude::*;

use crate::error::AppError;
use crate::services::auth::{self, AuthUser};

/// The signed-in user, if any.
#[derive(Clone, Default, PartialEq)]
struct Session(Option<AuthUser>);

/// The signed-in user, or `None`. Tracked.
pub fn user() -> Option<AuthUser> {
    global::<Session>().get().0
}

/// Signs in through the provider and keeps the user it returns. The error
/// is the provider's (`"invalid"` input, `"denied"` password).
pub async fn sign_in(email: &str, password: &str) -> Result<AuthUser, AppError> {
    let provider = auth::provider();
    let user = provider.sign_in(email, password).await?;
    global::<Session>().set(Session(Some(user.clone())));
    Ok(user)
}

/// Signs out of the provider and forgets the user.
pub fn sign_out() {
    auth::provider().sign_out();
    global::<Session>().set(Session(None));
}
