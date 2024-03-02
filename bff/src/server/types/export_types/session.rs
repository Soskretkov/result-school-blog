use crate::server::types::db_interaction_types::UserPayload;
use crate::store;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    pub async fn is_exist(&self) -> bool {
        let path_suffix = format!("users/{}", self.user_id);
        match store::fetch::<Option<UserPayload>>(&path_suffix).await {
            Ok(Some(user_data)) => user_data.sessions.is_exist(&self.id),
            _ => false,
        }
    }
}
