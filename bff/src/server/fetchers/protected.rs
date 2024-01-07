use crate::api_utils;
use crate::server::types::db_types::User as DbUser;
use crate::server::types::export_types::{Role, Session, User};
use gloo_timers::future::TimeoutFuture;

pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, String> {
    TimeoutFuture::new(1_000).await;

    match api_utils::find_user_by_kv::<DbUser>("id", &session.user_id).await {
        None => Err("Пользователь не существует".to_string()),
        Some(user) if user.role_id == 0 => Ok(api_utils::all_users().await),
        _ => Err("Недостаточно прав на операцию".to_string()),
    }
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, String> {
    TimeoutFuture::new(2_000).await;

    match api_utils::find_user_by_kv::<DbUser>("id", &session.user_id).await {
        None => Err("Пользователь не существует".to_string()),
        Some(user) if user.role_id < 2 => Ok(api_utils::all_roles().await),
        _ => Err("Недостаточно прав на операцию".to_string()),
    }
}

pub async fn fetch_user_by_id(session: &Session, id_to_find: &str) -> Result<Option<User>, String> {
    TimeoutFuture::new(1_000).await;

    match api_utils::find_user_by_kv::<DbUser>("id", &session.user_id).await {
        None => Err("Пользователь не существует".to_string()),
        Some(user) if user.role_id < 2 => Ok(api_utils::find_user_by_kv("id", id_to_find).await),
        _ => Err("Недостаточно прав на операцию".to_string()),
    }
}
