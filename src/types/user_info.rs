mod role;
pub use role::RoleName;
use bff::server::User;

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub login: String,
    pub registered_at: String,
    pub role: RoleName,
}


impl UserInfo {
    pub fn new(user: User) -> Self {
        Self {
            login: user.login,
            registered_at: user.registered_at,
            role: RoleName::from_id(user.role_id).unwrap(),
        }
    }
    pub fn update(&mut self) -> Option<Self> {
        todo!("обновление сессии не реализовано")
    }
}
