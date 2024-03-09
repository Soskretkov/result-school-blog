use bff::server::{Session, User};

#[derive(Debug, Clone, PartialEq)]
pub struct Auth {
    pub session: Session,
    pub user: User,
}

impl Auth {
    pub fn new(session: Session, user: User) -> Self {
        Self { session, user }
    }
}
