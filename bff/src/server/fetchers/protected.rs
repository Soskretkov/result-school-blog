use super::super::utils;
use crate::server::error::Error;
use crate::server::types::db_interaction::{Role, User as DbUser};
use crate::server::types::export::User;
use crate::server::types::Session;
use crate::store;

pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, Error> {
    let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_view_users();
    utils::verify_session_with_permissions(session, check_perm).await?;
    store::fetch::<Vec<User>>("users").await
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, Error> {
    let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_view_roles();
    utils::verify_session_with_permissions(session, check_perm).await?;
    store::fetch::<Vec<Role>>("roles").await
}

pub async fn fetch_user(session: &Session, id_to_find: &str) -> Result<User, Error> {
    let check_perm = |db_user: &DbUser| {
        (id_to_find == session.user_id) || db_user.payload.role_id.can_view_users()
    };

    let db_user = utils::verify_session_with_permissions(session, check_perm).await?;

    if id_to_find != session.user_id {
        let path_suffix = format!("users/{id_to_find}");
        store::fetch::<User>(&path_suffix).await
    } else {
        return Ok(User {
            id: db_user.id,
            login: db_user.payload.login,
            role_id: db_user.payload.role_id,
            created_at: db_user.payload.created_at,
        });
    }
}
