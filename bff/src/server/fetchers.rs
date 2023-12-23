pub use super::types::export_types::{Role, User};
use crate::api_utils;
pub use api_utils::test;

// это не действия пользователя, а впринципе возможности запросить на сервере
pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    Ok(api_utils::all_users().await)
}

pub async fn fetch_user_by_id(id_to_find: &str) -> Option<User> {
    api_utils::user_by_id(id_to_find).await
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    Ok(api_utils::all_roles().await)
}
