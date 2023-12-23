use super::types::db_types::User;
use super::types::Sessions;
use super::utils;
use crate::api_utils;

// это не действия пользователя, а возможности приложения влиять на сервер
// регистрация на клиенте это комбинация register + authorize возможностей сервера
pub async fn authorize(login: &str, password: &str) -> Result<String, String> {
    let wrapped_user: Option<User> = api_utils::user_by_login(login).await;

    match wrapped_user {
        None => {
            return Err("Пользователь не найден".to_string());
        }
        Some(user) if user.password != password => {
            return Err("Пароль не верен".to_string());
        }
        Some(user) => {
            let session = user.sessions.data.into_iter().next().unwrap();

            return Ok(session);
        }

    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    let wrapped_user: Option<User> = api_utils::user_by_login(&login).await;

    if wrapped_user.is_some() {
        return Err("Логин уже занят".to_string());
    }

    let id = utils::create_rnd_float64().to_string();
    let new_user = User {
        id: id.clone(),
        login,
        password,
        registered_at: utils::get_rnd_date(),
        role_id: 2,
        sessions: Sessions::new().add_rnd_session(),
    };

    api_utils::add_user(&new_user);

    Ok(id)
}

pub async fn logout(user_id: &str, session_id: &str) {
    // заменить на запрос по ручке
    let user: User = api_utils::user_by_id(&user_id).await.unwrap();
    let _new_sessions = Sessions::del_session(user.sessions, session_id);
    unimplemented!("отправить измененные сессии на хранение в бд")
}

pub async fn _is_valid_session(user_id: &str, session_id: &str) -> bool {
    api_utils::user_by_id(&user_id)
        .await
        .filter(|user: &User| user.sessions.is_exist(session_id))
        .is_some()
}
