use serde::{Deserialize, Serialize};
use crate::api_utils;
use crate::server::types::db_types::{User as DbUser};



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