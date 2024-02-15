use super::GlobContext;
use bff::server as bff_server;
pub use bff::server::{Role, RoleName, Session, User};
use gloo_timers::future::TimeoutFuture;
use leptos::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    bff_server::authorize(user_id, password).await
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    bff_server::register(login, password).await
}

pub async fn logout() -> Result<(), String> {
    bff_server::logout(&get_session()).await
}

pub async fn update_user_role(user_id: &str, role_name: RoleName) -> Result<(), String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: update_user_role (id: {}, role_name: {})", user_id, role_name.as_str());
    bff_server::update_user_role(&get_session(), user_id, role_name).await
}

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_id_by_login (логин: {})", login);
    bff_server::fetch_id_by_login(login).await
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_all_roles");
    bff_server::fetch_all_roles(&get_session()).await
}

pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_all_users");
    bff_server::fetch_all_users(&get_session()).await
}

pub async fn _fetch_user(id_to_find: &str) -> Result<Option<User>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_user (id: {})", id_to_find);
    bff_server::fetch_user(&get_session(), id_to_find).await
}

fn get_session() -> Session {
    use_context::<GlobContext>()
        .unwrap()
        .session
        .get_untracked()
        .unwrap()
}
