mod protected;
use crate::api_utils;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    api_utils::find_users_by_kv("login", login).await
}
