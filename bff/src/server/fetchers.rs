mod protected;
use super::types::export_types::User;
use crate::api_utils;
use crate::server::types::export_types::Session;
pub use protected::*;


pub async fn fetch_id_by_login(login: &str) -> Option<String> {
    api_utils::find_user_by_kv("login", login).await.map(|user: User| user.id)
}

pub async fn fetch_self_user_info(session: &Session) -> Option<User> {
    api_utils::find_user_by_kv("id", &session.user_id).await
}