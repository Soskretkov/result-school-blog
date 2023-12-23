mod role;
pub use role::RoleName;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role: RoleName,
    pub session_id: String,
}
