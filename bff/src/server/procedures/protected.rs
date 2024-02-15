use super::super::utils;
use crate::store_utils;
use crate::server::types::export_types::{RoleName, Session, User};

pub async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleName,
) -> Result<(), String> {
    let check_perm = |user: &User| user.role_id.can_update_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    store_utils::update_user_field(user_id, "role_id", &role_name).await?;
    Ok(())
}
