use super::UserInfo;
use bff::server::Session;
use leptos::*;

#[derive(Debug, Clone)]
pub struct GlobContext {
    pub session: ReadSignal<Option<Session>>,
    pub user_info: UserInfo,
}
