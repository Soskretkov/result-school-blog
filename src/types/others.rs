use leptos::*;
use bff::server::Session;
use super::UserInfo;

#[derive(Debug, Clone)]
pub struct GlobContext {
    pub user_info: ReadSignal<Option<UserInfo>>,
    pub session: ReadSignal<Option<Session>>,
}