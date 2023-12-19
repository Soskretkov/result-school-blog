mod role;
use crate::bff::server::CurrentSession;
pub use role::Role;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub role: Role,
    pub session_id: String,
}

impl User {
    fn new(session: CurrentSession) -> Self {
        let role = Role::from_id(session.user_role).unwrap();
        Self {
            id: session.user_id,
            login: session.user_login,
            role,
            session_id: session.id,
        }
    }
}
