mod protected;
use super::types::db_interaction_types::User as DbUser;
use crate::store_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    store_utils::find_first_user_by_kv::<DbUser>("login", login)
        .await
        .map(|opt_user| opt_user.map(|user| user.id))
}
