use crate::server::{Role, Session, User};
use leptos::*;

#[derive(Clone)]
pub struct GlobContext {
    pub session: ReadSignal<Option<Session>>,
    pub user_resource: Resource<(), Option<User>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}
