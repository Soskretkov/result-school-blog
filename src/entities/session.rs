mod role;
// use crate::bff::server::CurrentSession;
pub use role::Role;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role: Role,
    pub session_id: String,
}

// impl User {
//     fn new(session: CurrentSession) -> Self {
//         let role = Role::from_id(session.user_role).unwrap();
//         Self {
//             id: session.user_id,
//             login: session.user_login,
//             registred_at: "".to_string(),
//             role,
//         }
//     }
// }
