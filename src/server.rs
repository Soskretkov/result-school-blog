use super::GlobContext;
use bff::server as bff_server;
pub use bff::server::{Comment, Error, Post, Role, RoleType, Session, User};

use gloo_timers::future::TimeoutFuture;
use leptos::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<Session, String> {
    logging::log!("server.rs: authorize ({})", user_id);
    bff_server::authorize(user_id, password)
        .await
        .map_err(|e| e.to_string())
}

pub async fn register(login: String, password: String) -> Result<(), String> {
    logging::log!("server.rs: register (логин: {})", login);
    bff_server::register(login, password)
        .await
        .map_err(|e| e.to_string())
}

pub async fn logout() -> Result<(), String> {
    logging::log!("server.rs: logout");
    bff_server::logout(&get_session())
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_post(id: &str) -> Result<Post, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_post (id: {})", id);
    bff_server::fetch_post(id).await.map_err(|e| e.to_string())
}

pub async fn fetch_user(id: &str) -> Result<User, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_user (id: {})", id);
    bff_server::fetch_user(&get_session(), id)
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn fetch_all_users() -> Result<Vec<User>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_all_users");
    bff_server::fetch_all_users(&get_session())
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn remove_user(id_to_delete: &str) -> Result<(), String> {
    logging::log!("server.rs: remove_user");
    bff_server::remove_user(&get_session(), id_to_delete)
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn add_comment(post_id: String, content: String) -> Result<Comment, String> {
    logging::log!("server.rs: add_comment");
    bff_server::add_comment(&get_session(), post_id, content)
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn remove_comment(id_to_delete: &str) -> Result<(), String> {
    logging::log!("server.rs: remove_comment");
    bff_server::remove_comment(&get_session(), id_to_delete)
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn fetch_all_roles() -> Result<Vec<Role>, String> {
    TimeoutFuture::new(1000).await;
    logging::log!("server.rs: fetch_all_roles");
    bff_server::fetch_all_roles(&get_session())
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}

pub async fn update_user_role(user_id: &str, role_name: RoleType) -> Result<(), String> {
    TimeoutFuture::new(1000).await;
    logging::log!(
        r#"server.rs: update_user_role (id {} назначена роль "{}")"#,
        user_id,
        role_name.as_str()
    );
    bff_server::update_user_role(&get_session(), user_id, role_name)
        .await
        .map_err(|e| {
            if let Error::InvalidSession = e {
                set_none_for_session();
            }
            e.to_string()
        })
}



fn get_session() -> Session {
    use_context::<GlobContext>()
        .unwrap()
        .session
        .get_untracked()
        .unwrap()
}

fn set_none_for_session() {
    use_context::<GlobContext>().unwrap().set_session.set(None);
}
