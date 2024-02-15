mod protected;
use crate::db_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    db_utils::find_users_by_kv("login", login).await
}
