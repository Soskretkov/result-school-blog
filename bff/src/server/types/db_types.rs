mod role_name;
pub use role_name::RoleName;
use serde::{Deserialize, Serialize};
use super::sessions_store::SessionsStore;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub password: String,
    pub registered_at: String,
    pub role_id: RoleName,
    pub sessions: SessionsStore,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub id: RoleName,
    pub name: String,
}