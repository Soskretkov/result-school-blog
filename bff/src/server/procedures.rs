use super::types::db_types::User as DbUser;
use super::types::export_types::Session;
use super::types::SessionsStore;
use super::utils;
use crate::api_utils;
use uuid::Uuid;

// исключительно для случая когда нет куки
// почему подход выше работает: при смене пароля массив сессий обнуляется
// почему id: при наличии учетки клиент так и так проясняет id чтобы образовать сессию
pub async fn authorize(user_id: &str, password: &str) -> Result<String, String> {
    let wrapped_user: Option<DbUser> = api_utils::find_user_by_kv("id", user_id).await;

    match wrapped_user {
        None => Err("Пользователь не найден".into()),
        Some(user) if user.password != password => Err("Пароль не верен".into()),
        Some(user) => {
            let mut new_sessions = user.sessions;
            let session_id = new_sessions.add_rnd_session();
            api_utils::update_user_sessions(&user.id, &new_sessions).await;
            Ok(session_id)
        }
    }
}

pub async fn register(login: String, password: String) -> Result<String, String> {
    let wrapped_user: Option<DbUser> = api_utils::find_user_by_kv("login", &login).await;

    if wrapped_user.is_some() {
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
        role_id: 2,
        sessions: SessionsStore::new(),
    };

    api_utils::add_user(&new_user).await;

    Ok(new_user.id)
}

pub async fn logout(session: &Session) -> Result<(), ()> {
    let user: DbUser = api_utils::find_user_by_kv("id", &session.user_id)
        .await
        .unwrap();
    let sessions = user.sessions;

    // Удалить нужную сессию и образовать обновленное хранилище сессий
    let new_sessions = sessions.del_session(&session.id);

    // Записать обновленые сессии через утилиту для json-server
    api_utils::update_user_sessions(&user.id, &new_sessions).await;
    Ok(())
}

// async fn _is_valid_session(session: &Session) -> bool {
//     api_utils::find_user_by_kv("id", &session.user_id)
//         .await
//         .filter(|user: &DbUser| user.sessions.is_exist(&session.id))
//         .is_some()
// }
