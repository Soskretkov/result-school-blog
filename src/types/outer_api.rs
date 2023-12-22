use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role_id: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub id: u8,
    pub name: String,
}