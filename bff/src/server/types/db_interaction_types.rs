mod role_type;
use super::sessions_store::SessionsStore;
pub use role_type::RoleType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub password: String,
    pub registered_at: String,
    pub role_id: RoleType,
    pub sessions: SessionsStore,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub id: RoleType,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub content: String,
    pub published_at: String,
}
