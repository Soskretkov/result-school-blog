use super::{UserInfo};
use crate::server::{Role, Session};
use leptos::*;

#[derive(Clone)]
pub struct GlobContext {
    pub location: ReadSignal<String>,
    pub session: ReadSignal<Option<Session>>,
    pub user_info: UserInfo,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}




