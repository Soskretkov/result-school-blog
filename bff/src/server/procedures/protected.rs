use super::super::utils;
use crate::server::types::db_interaction::User as DbUser;
use crate::server::types::export::{RoleType, Session};
use crate::store;

pub async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleType,
) -> Result<(), String> {
    let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_update_roles();
    utils::verify_session_with_permissions(session, check_perm).await?;
    let path_suffix = format!("users/{user_id}");
    store::update_field(&path_suffix, "role_id", &role_name).await?;
    Ok(())
}

pub async fn remove_user(session: &Session, id_to_delete: &str) -> Result<(), String> {
    // нужны привилегии чтобы удалять других
    if id_to_delete != session.user_id {
        let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_remove_users();
        utils::verify_session_with_permissions(session, check_perm).await?;
    }

    let path_suffix = format!("users/{}", id_to_delete);
    store::delete(&path_suffix).await
}
