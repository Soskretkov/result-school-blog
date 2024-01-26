use super::GlobContext;
use bff::server;
pub use bff::server::{Role, RoleName, Session, User};
use leptos::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    server::authorize(user_id, password).await
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    server::register(login, password).await
}

pub async fn logout() -> Result<(), ()> {
    server::logout(&get_session()).await
}

pub async fn fetch_id_by_login(login: &str) -> Option<String> {
    server::fetch_id_by_login(login).await
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    server::fetch_all_roles(&get_session()).await
}

pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    server::fetch_all_users(&get_session()).await
}

pub async fn fetch_user(id_to_find: &str) -> Result<Option<User>, String> {
    server::fetch_user(&get_session(), id_to_find).await
}

fn get_session() -> Session {
    use_context::<GlobContext>()
    .unwrap()
    .session
    .get_untracked()
    .unwrap()
}
