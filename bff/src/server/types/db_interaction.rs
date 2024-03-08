mod role_type;
use super::sessions_store::SessionsStore;
use chrono::{DateTime, Utc};
pub use role_type::RoleType;
use serde::{Deserialize, Serialize};

// данные, которые не генерируются на уровне бд
// (created_at будет исключен отсюда по этой причине в будущем)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPayload {
    pub login: String,
    pub password: String,
    pub role_id: RoleType,
    pub sessions: SessionsStore,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    #[serde(flatten)]
    pub payload: UserPayload, // Использование UserData как компонента
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub id: RoleType,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostPayload {
    pub title: String,
    pub image_url: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    #[serde(flatten)] // не реэкспортируем клиенту, поэтому можно и так
    pub payload: PostPayload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentPayload {
    pub post_id: String,
    pub user_id: String,
    pub login_snapshot: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub login_snapshot: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
