use super::super::utils;
use crate::store_utils;
use crate::server::types::export_types::{Role, Session, User};
use gloo_timers::future::TimeoutFuture;

pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, String> {
    TimeoutFuture::new(500).await;
    let check_perm = |user: &User| user.role_id.can_view_users();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::all_users().await
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, String> {
    TimeoutFuture::new(1000).await;
    let check_perm = |user: &User| user.role_id.can_view_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::all_roles().await
}

pub async fn fetch_user(session: &Session, id_to_find: &str) -> Result<Option<User>, String> {
    TimeoutFuture::new(500).await;
    // не нужны привилегии чтобы запрашивать по себе
    if session.user_id == id_to_find {
        return store_utils::find_users_by_kv::<User>("id", id_to_find).await;
    }

    let check_perm = |user: &User| user.role_id.can_view_users();
    let user = utils::get_user_with_permission(session, check_perm).await?;

    if &user.id == id_to_find {
        return Ok(Some(user));
    };
    
    store_utils::find_users_by_kv("id", id_to_find).await
}
