use bff::server::{Session, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auth {
    pub session: Session,
    pub user: Option<User>,
}

impl Auth {
    pub fn new(session: Session, user: Option<User>) -> Self {
        Self { session, user }
    }
}
