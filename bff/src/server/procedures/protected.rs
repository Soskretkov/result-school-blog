use super::super::utils;
use crate::server::error::Error;
use crate::server::types::db_interaction::{Comment, CommentPayload, RoleType, User as DbUser};
use crate::server::types::Session;
use crate::store;

pub async fn add_comment(
    session: &Session,
    post_id: String,
    content: String,
) -> Result<Comment, Error> {
    let db_user = utils::verify_session(session).await?;

    let comment_payload = CommentPayload {
        post_id,
        user_id: db_user.id,
        user_name_snapshot: db_user.payload.name,
        content,
        created_at: utils::get_current_date(),
    };

    store::add("comments", &comment_payload)
        .await
        .map_err(Error::Reqwest)
}

pub async fn update_user_role(
    session: &Session,
    user_id: &str,
    role_name: RoleType,
) -> Result<(), Error> {
    let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_update_roles();
    utils::verify_session_with_permissions(session, check_perm).await?;
    let path_suffix = format!("users/{user_id}");
    store::update_field(&path_suffix, "role_id", &role_name)
        .await
        .map_err(Error::Reqwest)?;
    Ok(())
}

pub async fn remove_user(session: &Session, id_to_delete: &str) -> Result<(), Error> {
    // нужны привилегии чтобы удалять других
    if id_to_delete != session.user_id {
        let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_remove_users();
        utils::verify_session_with_permissions(session, check_perm).await?;
    }

    let path_suffix = format!("users/{}", id_to_delete);
    store::delete(&path_suffix).await.map_err(Error::Reqwest)
}

pub async fn remove_comment(session: &Session, id_to_delete: &str) -> Result<(), Error> {
    let check_perm = |db_user: &DbUser| db_user.payload.role_id.can_remove_comment();
    utils::verify_session_with_permissions(session, check_perm).await?;

    let path_suffix = format!("comments/{}", id_to_delete);
    store::delete(&path_suffix).await.map_err(Error::Reqwest)
}
