use super::super::utils;
use crate::api_utils;
use crate::server::types::export_types::{RoleName, Session, User};
use gloo_timers::future::TimeoutFuture;

async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleName,
) -> Result<(), String> {
    TimeoutFuture::new(500).await;
    let check_perm = |user: &User| user.role_id.can_update_roles();
    utils::get_user_with_permission(session, check_perm).await?;
    api_utils::update_user_field(user_id, "role_id", &role_name).await?;
    Ok(())
}
