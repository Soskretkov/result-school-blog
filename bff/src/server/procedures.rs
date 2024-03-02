mod protected;
use super::types::db_interaction_types::{RoleType, User, UserPayload};
use super::types::export_types::Session;
use super::types::SessionsStore;
use super::utils;
use crate::store;
pub use protected::*;

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    let path_suffix = format!("users/{user_id}");
    match store::fetch::<Option<User>>(&path_suffix).await? {
        None => Err("Пользователь не найден".into()),
        Some(user) if user.payload.password != password => Err("Пароль не верен".into()),
        Some(user) => {
            let mut new_sessions = user.payload.sessions;
            let session_id = new_sessions.add_rnd_session();
            let path_suffix = format!("users/{}", user.id);
            store::update_field(&path_suffix, "sessions", &new_sessions).await?;
            Ok(session_id)
        }
    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    let path_suffix = format!("users/?login={}", &login);
    if store::fetch::<Vec<UserPayload>>(&path_suffix)
        .await?
        .into_iter()
        .next()
        .is_some()
    {
        return Err("Логин уже занят".to_string());
    }

    let user_payload = UserPayload {
        login,
        password,
        created_at: utils::get_current_date(),
        role_id: RoleType::Reader,
        sessions: SessionsStore::new(),
    };

    let resp = store::add("users", &user_payload).await?;
    let added_user: User = resp.json::<User>().await.map_err(|e| e.to_string())?;

    Ok(added_user.id.to_string())
}

pub async fn logout(session: &Session) -> Result<(), String> {
    let path_suffix = format!("users/{}", session.user_id);
    let user: User = store::fetch::<Option<User>>(&path_suffix)
        .await
        .map(|users_vec| users_vec.into_iter().next())?
        .ok_or_else(|| "Пользователь не существует".to_string())?;
    let sessions = user.payload.sessions;

    // Удалить нужную сессию и образовать обновленное хранилище сессий
    let new_sessions = sessions.del_session(&session.id);

    // Записать обновленые сессии через утилиту для json-server
    let path_suffix = format!("users/{}", user.id);
    store::update_field(&path_suffix, "sessions", &new_sessions).await?;
    Ok(())
}

pub async fn add_comment(session: &Session) -> Result<(), String> {
    let path_suffix = format!("users/{}", session.user_id);

    Ok(())
}
