mod protected;
use super::types::db_interaction_types::{RoleType, User as DbUser};
use super::types::export_types::Session;
use super::types::SessionsStore;
use super::utils;
use crate::store_utils;
pub use protected::*;
use uuid::Uuid;

// исключительно для случая когда нет куки
// почему подход выше работает: при смене пароля массив сессий обнуляется
// почему id: при наличии учетки клиент так и так проясняет id чтобы образовать сессию
pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    match store_utils::user::<DbUser>(user_id).await? {
        None => Err("Пользователь не найден".into()),
        Some(user) if user.password != password => Err("Пароль не верен".into()),
        Some(user) => {
            let mut new_sessions = user.sessions;
            let session_id = new_sessions.add_rnd_session();
            store_utils::update_user_field(&user.id, "sessions", &new_sessions).await?;
            Ok(session_id)
        }
    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    if store_utils::find_user_by_kv::<DbUser>("login", &login)
        .await?
        .into_iter()
        .next()
        .is_some()
    {
        return Err("Логин уже занят".to_string());
    }

    let user_id = Uuid::new_v4()
        .to_string()
        .chars()
        .take(8)
        .collect::<String>();

    if user_id.len() < 8 {
        panic!("UUID короче чем 8 символов");
    }

    let new_user = DbUser {
        id: user_id,
        login,
        password,
        registered_at: utils::get_current_date(),
        role_id: RoleType::Reader,
        sessions: SessionsStore::new(),
    };

    store_utils::add_user(&new_user).await?;

    Ok(new_user.id)
}

pub async fn logout(session: &Session) -> Result<(), String> {
    let user: DbUser = store_utils::user(&session.user_id)
        .await
        .map(|users_vec| users_vec.into_iter().next())?
        .ok_or_else(|| "Пользователь не существует".to_string())?;
    let sessions = user.sessions;

    // Удалить нужную сессию и образовать обновленное хранилище сессий
    let new_sessions = sessions.del_session(&session.id);

    // Записать обновленые сессии через утилиту для json-server
    store_utils::update_user_field(&user.id, "sessions", &new_sessions).await?;
    Ok(())
}
