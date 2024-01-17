mod protected;
use super::types::export_types::User;
use crate::api_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Option<String> {
    api_utils::find_user_by_kv("login", login)
        .await
        .map(|user: User| user.id)
}