use crate::server::types::db_interaction::{Comment, RoleType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub role_id: RoleType,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub content: String,
    pub created_at: String,
    pub comments: Vec<Comment>,
}
