use super::{User, Role};
use crate::api_utils;


pub async fn fetch_all_users(user_id: &str, sess_id: &str) -> Result<Vec<User>, String> {
    Ok(api_utils::all_users().await)
}

pub async fn fetch_all_roles(user_id: &str, sess_id: &str) -> Result<Vec<Role>, String> {
    Ok(api_utils::all_roles().await)
}

pub async fn fetch_user_by_id(user_id: &str, sess_id: &str, id_to_find: &str) -> Option<User> {
    api_utils::user_by_id(id_to_find).await
}