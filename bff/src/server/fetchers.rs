mod protected;
use super::types::db_interaction_types::User as DbUser;
use crate::store_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    store_utils::find_user_by_kv::<DbUser>("login", login)
        .await
        .map(|users_vec| users_vec.into_iter().next().map(|user| user.id))
}
