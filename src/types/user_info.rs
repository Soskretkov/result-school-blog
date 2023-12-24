mod role;
pub use role::RoleName;

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub login: String,
    pub registered_at: String,
    pub role: RoleName,
}


impl UserInfo {
    pub fn new(user_id: String, sess_id: String) -> Option<Self> {
        todo!("создание сессии не реализовано")
    }
    pub fn update(&mut self) -> Option<Self> {
        todo!("обновление сессии не реализовано")
    }
}
