pub use super::db_types::{Role, User as DbUser};
use crate::api_utils;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role_id: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    pub async fn is_exist(&self) -> bool {
        match api_utils::find_user_by_kv::<DbUser>("id", &self.user_id).await {
            None => false,
            Some(user) => user.sessions.is_exist(&self.id),
        }
    }
}
