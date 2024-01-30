use super::GlobContext;
use bff::server as bff_server;
pub use bff::server::{Role, RoleName, Session, User};
use leptos::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    bff_server::authorize(user_id, password).await
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    bff_server::register(login, password).await
}

pub async fn logout() -> Result<(), ()> {
    bff_server::logout(&get_session()).await
}

pub async fn fetch_id_by_login(login: &str) -> Option<String> {
    bff_server::fetch_id_by_login(login).await
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    bff_server::fetch_all_roles(&get_session()).await
}

pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    bff_server::fetch_all_users(&get_session()).await
}

pub async fn fetch_user(id_to_find: &str) -> Result<Option<User>, String> {
    bff_server::fetch_user(&get_session(), id_to_find).await
}

fn get_session() -> Session {
    use_context::<GlobContext>()
        .unwrap()
        .session
        .get_untracked()
        .unwrap()
}
