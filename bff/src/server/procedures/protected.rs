use super::super::utils;
use crate::server::types::export::{RoleType, Session, User};
use crate::store;

pub async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleType,
) -> Result<(), String> {
    let check_perm = |user: &User| user.role_id.can_update_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    let path_suffix = format!("users/{user_id}");
    store::update_field(&path_suffix, "role_id", &role_name).await?;
    Ok(())
}

pub async fn remove_user(session: &Session, id_to_delete: &str) -> Result<(), String> {
    // нужны привилегии чтобы удалять других
    if id_to_delete != session.user_id {
        let check_perm = |user: &User| user.role_id.can_remove_users();
        utils::get_user_with_permission(session, check_perm).await?;
    }

    let path_suffix = format!("users/{}", id_to_delete);
    store::delete(&path_suffix).await
}
