use super::{UserInfo};
use crate::server::Session;
use leptos::*;

#[derive(Debug, Clone)]
pub struct GlobContext {
    pub location: ReadSignal<String>,
    pub session: ReadSignal<Option<Session>>,
    pub user_info: UserInfo,
}




