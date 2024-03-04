mod protected;
use super::types::db_interaction::{Comment, CommentPayload, RoleType, User, UserPayload};
use super::types::{Session, SessionsStore};
use super::utils;
use crate::server::error::Error;
use crate::store;
pub use protected::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, Error> {
    let db_user = utils::get_user(user_id).await?;

    if db_user.payload.password != password {
        return Err(Error::InvalidPassword);
    }

    let mut new_sessions = db_user.payload.sessions;
    let session_id = new_sessions.add_rnd_session();
    let path_suffix = format!("users/{}", db_user.id);
    store::update_field(&path_suffix, "sessions", &new_sessions)
        .await
        .map_err(Error::Reqwest)?;
    Ok(session_id)
}

pub async fn register(login: String, password: String) -> Result<String, Error> {
    let path_suffix = format!("users/?login={}", &login);
    if store::fetch::<Vec<User>>(&path_suffix)
        .await
        .map_err(Error::Reqwest)?
        .into_iter()
        .next()
        .is_some()
    {
        return Err(Error::DbEntryNotUnique);
    }

    let user_payload = UserPayload {
        login,
        password,
        created_at: utils::get_current_date(),
        role_id: RoleType::Reader,
        sessions: SessionsStore::new(),
    };
    let added_user: User = store::add("users", &user_payload)
        .await
        .map_err(Error::Reqwest)?;
    Ok(added_user.id.to_string())
}

pub async fn logout(session: &Session) -> Result<(), Error> {
    let db_user = utils::get_user(&session.user_id).await?;
    let sessions = db_user.payload.sessions;

    // Удалить нужную сессию и образовать обновленное хранилище сессий
    let new_sessions = sessions.del_session(&session.id);

    // Записать обновленые сессии через утилиту для json-server
    let path_suffix = format!("users/{}", db_user.id);
    store::update_field(&path_suffix, "sessions", &new_sessions)
        .await
        .map_err(Error::Reqwest)?;
    Ok(())
}

// fn не размещается в protected.rs пока права пользователя не важны
pub async fn add_comment(
    session: &Session,
    post_id: String,
    content: String,
) -> Result<Comment, Error> {
    let db_user = utils::verify_user_session(session).await?;

    let comment_payload = CommentPayload {
        post_id,
        user_id: db_user.id,
        login_snapshot: db_user.payload.login,
        content,
        created_at: utils::get_current_date(),
    };

    store::add("comments", &comment_payload)
        .await
        .map_err(Error::Reqwest)
}
