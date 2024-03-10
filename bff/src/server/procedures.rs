mod protected;
use super::types::db_interaction::{RoleType, User, UserPayload};
use super::types::{Session, SessionsStore};
use super::utils;
use crate::server::error::Error;
use crate::store;
pub use protected::*;

pub async fn authorize(login: &str, password: &str) -> Result<Session, Error> {
    let path_suffix = format!("users/?login={}", &login);
    let db_user = store::fetch::<Vec<User>>(&path_suffix)
        .await
        .map_err(Error::Reqwest)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::DbEntryNotFound)?;

    if db_user.payload.password != password {
        return Err(Error::InvalidPassword);
    }

    let mut new_sessions = db_user.payload.sessions;
    let session_id = new_sessions.add_rnd_session();
    let path_suffix = format!("users/{}", db_user.id);
    store::update_field(&path_suffix, "sessions", &new_sessions)
        .await
        .map_err(Error::Reqwest)?;
    Ok(Session {
        id: session_id,
        user_id: db_user.id,
    })
}

pub async fn register(login: String, password: String) -> Result<(), Error> {
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
        login: login.clone(),
        name: login,
        password,
        created_at: utils::get_current_date(),
        role_id: RoleType::Reader,
        sessions: SessionsStore::new(),
    };

    let _added_user: User = store::add("users", &user_payload)
        .await
        .map_err(Error::Reqwest)?;

    Ok(())
}

// fn не размещается в protected.rs т.к. валидность сессии роли не играет
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
