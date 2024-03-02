use serde::{Deserialize, Serialize};

// реэкспорт клиентскому коду общих типов для бекенда и фронтенда
pub use crate::server::types::db_interaction::{Comment, Post, Role, RoleType};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub role_id: RoleType,
    pub created_at: String,
}
