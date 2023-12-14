use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub password: String,
    pub registed_at: String,
    pub role_id: u8,
}
