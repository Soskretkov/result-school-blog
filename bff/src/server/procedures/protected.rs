use super::super::utils;
use crate::server::types::export_types::{RoleType, Session, User};
use crate::store_utils;

pub async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleType,
) -> Result<(), String> {
    let check_perm = |user: &User| user.role_id.can_update_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::update_user_field(user_id, "role_id", &role_name).await?;
    Ok(())
}

pub async fn remove_user(session: &Session, id_to_delete: &str) -> Result<(), String> {
    // нужны привилегии чтобы удалять других
    if id_to_delete != session.user_id {
        let check_perm = |user: &User| user.role_id.can_remove_users();
        utils::get_user_with_permission(session, check_perm).await?;
    }

    store_utils::delete_user(id_to_delete).await
}
