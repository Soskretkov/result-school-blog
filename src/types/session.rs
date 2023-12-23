mod role;
pub use role::RoleName;
use leptos::*;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role: RoleName,
    pub sess_id: String,
}

impl Session {
    pub fn new(user_id: String, sess_id: String) -> Option<Self> {

        let value = create_memo(move |_| {
            
        });
        todo!("создание сессии не реализовано")
    }
    pub fn update(&mut self) -> Option<Self> {
        todo!("обновление сессии не реализовано")
    }
}
