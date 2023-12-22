// содержит методы, которых нет у json-server
// то есть json-server предоставляется в ведение клиента,
// и только в особых случаях клиент вызывает этот модуль

use super::api_utils;
mod types;
mod utils;
pub use types::{Sessions, User};

pub async fn authorize(login: &str, password: &str) -> Result<String, String> {
    let wrapped_user: Option<User> = api_utils::user_info(login).await;

    match wrapped_user {
        Some(user) if user.password != password => {
            return Err("Пароль не верен".to_string());
        }
        Some(user) => {
            let session = user.sessions.data.into_iter().next().unwrap();

            return Ok(session);
        }
        None => {
            return Err("Пользователь не найден".to_string());
        }
    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    let wrapped_user: Option<User> = api_utils::user_info(&login).await;

    if wrapped_user.is_some() {
        return Err("Логин уже занят".to_string());
    }

    let sessions = Sessions::new().add_rnd_session();

    let new_user = User {
        id: utils::create_rnd_float64().to_string(),
        login,
        password,
        registered_at: utils::get_rnd_date(),
        role_id: 2,
        sessions: sessions,
    };

    api_utils::add_user(&new_user);

    let session = new_user.sessions.data.into_iter().next().unwrap();

    Ok(session)
}

pub async fn logout(user_id: &str, session_id: &str) {
    // заменить на запрос по ручке
    let user: User = api_utils::user_info(&user_id).await.unwrap();
    let _new_sessions = Sessions::del_session(user.sessions, session_id);
    unimplemented!("отправить измененные сессии на хранение в бд")
}

pub async fn is_valid_session(user_id: &str, session_id: &str) -> bool {
    api_utils::user_info(&user_id)
        .await
        .filter(|user: &User| user.sessions.is_exist(session_id))
        .is_some()
}
