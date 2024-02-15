mod protected;
use crate::store_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    store_utils::find_users_by_kv("login", login).await
}
