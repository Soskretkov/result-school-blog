use super::Auth;
use crate::server::{Role};
use leptos::*;

#[derive(Clone)]
pub struct GlobContext {
    pub auth: ReadSignal<Option<Auth>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}
