use super::role::RoleName;
use bff::server::User;


#[derive(Debug, Clone)]
pub struct UserData {
    pub login: String,
    pub registered_at: String,
    pub role: RoleName,
}

impl UserData {
    pub fn new(user: User) -> Self {
        Self {
            login: user.login,
            registered_at: user.registered_at,
            role: RoleName::from_id(user.role_id).unwrap(),
        }
    }
}