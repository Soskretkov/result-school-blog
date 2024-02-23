use crate::server::types::db_interaction_types::User as DbUser;
use crate::store_utils;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    pub async fn is_exist(&self) -> bool {
        match store_utils::user::<DbUser>(&self.user_id).await {
            Ok(Some(user)) => user.sessions.is_exist(&self.id),
            _ => false,
        }
    }
}
