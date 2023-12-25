mod protected;
use super::types::export_types::User;
use crate::api_utils;
pub use api_utils::test;
use crate::server::types::export_types::Session;
pub use protected::*;


pub async fn fetch_user_id (login: &str) -> Option<String> {
    api_utils::user_by_login::<User>(login).await.map(|user| user.id)
}

pub async fn fetch_self_user_info(session: &Session) -> Option<User> {
    api_utils::user_by_id(&session.id).await
}