mod server;
pub use server::Server;
use crate::bff::shared::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentSession {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_role: u8,
}

impl CurrentSession {
    pub fn new(user: User) -> Self {
        let session_id = user.sessions.data.into_iter().next().unwrap();
        Self {
            id: session_id,
            user_id: user.id,
            user_login: user.login,
            user_role: user.role_id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
// клиентская сторона не конфигурирует Authorize, это ответ сервера
pub struct Authentic {
    pub error: Option<String>,
    pub res: Option<CurrentSession>,
}
