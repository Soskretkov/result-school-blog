use super::shared::User;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_role: u8,
}

impl Session {
    pub fn new(user: User, session_id: String) -> Self {
        Self {
            id: session_id,
            user_id: user.id,
            user_login: user.login,
            user_role: user.role_id,
        }
    }
}
