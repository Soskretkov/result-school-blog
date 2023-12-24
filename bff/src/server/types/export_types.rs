use serde::{Deserialize, Serialize};
pub use super::db_types::Role;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role_id: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub sess_id: String,
    pub user_id: String,
}