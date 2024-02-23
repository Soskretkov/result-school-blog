use super::super::utils;
use crate::server::types::export_types::{Role, Session, User};
use crate::store_utils;

pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, String> {
    let check_perm = |user: &User| user.role_id.can_view_users();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::all_users().await
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, String> {
    let check_perm = |user: &User| user.role_id.can_view_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::all_roles().await
}

pub async fn fetch_user(session: &Session, id_to_find: &str) -> Result<Option<User>, String> {
    // нужны привилегии чтобы запрашивать других
    if id_to_find != session.user_id {
        let check_perm = |user: &User| user.role_id.can_view_users();
        utils::get_user_with_permission(session, check_perm).await?;
    }

    store_utils::user(id_to_find).await
}
