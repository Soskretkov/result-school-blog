use super::AuthedUser;
use crate::server::{Role};
use leptos::*;

#[derive(Clone)]
pub struct GlobContext {
    pub authed_user: ReadSignal<Option<AuthedUser>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}
