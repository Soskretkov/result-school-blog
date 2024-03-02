mod protected;
use super::types::db_interaction_types::{RoleType, User as DbUser};
use super::types::export_types::Session;
use super::types::SessionsStore;
use super::utils;
use crate::store;
use leptos::leptos_dom::logging;
pub use protected::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewUser {
    pub login: String,
    pub password: String,
    pub registered_at: String,
    pub role_id: RoleType,
    pub sessions: SessionsStore,
}

pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    let path_suffix = format!("users/{user_id}");
    match store::fetch::<Option<DbUser>>(&path_suffix).await? {
        None => Err("Пользователь не найден".into()),
        Some(user) if user.password != password => Err("Пароль не верен".into()),
        Some(user) => {
            let mut new_sessions = user.sessions;
            let session_id = new_sessions.add_rnd_session();
            let path_suffix = format!("users/{}", user.id);
            store::update_field(&path_suffix, "sessions", &new_sessions).await?;
            Ok(session_id)
        }
    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    let path_suffix = format!("users/?login={}", &login);
    if store::fetch::<Vec<DbUser>>(&path_suffix)
        .await?
        .into_iter()
        .next()
        .is_some()
    {
        return Err("Логин уже занят".to_string());
    }

    let new_user = NewUser {
        login,
        password,
        registered_at: utils::get_current_date(),
        role_id: RoleType::Reader,
        sessions: SessionsStore::new(),
    };

    let resp = store::add("users", &new_user).await?;

    leptos::logging::log!("{:?}", resp);
    let added_db_user: DbUser = resp.json::<DbUser>().await.map_err(|e| e.to_string())?;

    Ok(added_db_user.id.to_string())
}

pub async fn logout(session: &Session) -> Result<(), String> {
    let path_suffix = format!("users/{}", session.user_id);
    let user: DbUser = store::fetch::<Option<DbUser>>(&path_suffix)
        .await
        .map(|users_vec| users_vec.into_iter().next())?
        .ok_or_else(|| "Пользователь не существует".to_string())?;
    let sessions = user.sessions;

    // Удалить нужную сессию и образовать обновленное хранилище сессий
    let new_sessions = sessions.del_session(&session.id);

    // Записать обновленые сессии через утилиту для json-server
    let path_suffix = format!("users/{}", user.id);
    store::update_field(&path_suffix, "sessions", &new_sessions).await?;
    Ok(())
}
