use serde::{Deserialize, Serialize};
use crate::server::RoleType;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub role_id: RoleType,
    pub created_at: String,
}