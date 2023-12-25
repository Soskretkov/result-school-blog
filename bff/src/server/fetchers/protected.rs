use crate::server::types::export_types::{User, Role, Session};
use crate::api_utils;


pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, String> {
    Ok(api_utils::all_users().await)
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, String> {
    Ok(api_utils::all_roles().await)
}

pub async fn fetch_user_by_id(session: &Session, id_to_find: &str) -> Option<User> {
    api_utils::user_by_id(id_to_find).await
}