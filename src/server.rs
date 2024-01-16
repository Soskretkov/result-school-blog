use super::GlobContext;
use bff::server;
pub use bff::server::*;
use leptos::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    server::authorize(user_id, password).await
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    server::register(login, password).await
}

pub async fn logout() -> Result<(), ()> {
    let sess = use_context::<GlobContext>().unwrap().session.get().unwrap();
    server::logout(&sess).await
}

pub async fn fetch_id_by_login(login: &str) -> Option<String> {
    server::fetch_id_by_login(login).await
}

pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    let sess = use_context::<GlobContext>().unwrap().session.get().unwrap();
    server::fetch_all_users(&sess).await
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    let sess = use_context::<GlobContext>().unwrap().session.get().unwrap();
    server::fetch_all_roles(&sess).await
}

pub async fn fetch_user(id_to_find: &str) -> Result<Option<User>, String> {
    let sess = use_context::<GlobContext>().unwrap().session.get().unwrap();
    server::fetch_user(&sess, id_to_find).await
}
