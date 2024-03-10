use crate::server::types::db_interaction::{Comment, RoleType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub name: String,
    pub role_id: RoleType,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub comments: Vec<Comment>,
}
