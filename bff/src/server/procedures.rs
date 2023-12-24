use super::types::db_types::User as DbUser;
use super::types::SessionsStore;
use super::types::export_types::Session;
use super::utils;
use crate::api_utils;
use uuid::Uuid;

pub async fn authorize(login: &str, password: &str) -> Result<String, String> {
    let wrapped_user: Option<DbUser> = api_utils::user_by_login(login).await;

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
    let wrapped_user: Option<DbUser> = api_utils::user_by_login(&login).await;

    if wrapped_user.is_some() {
        return Err("Логин уже занят".to_string());
    }

    let id = Uuid::new_v4()
        .to_string()
        .chars()
        .take(8)
        .collect::<String>();

    if id.len() < 8 {
        panic!("UUID is shorter than 8 characters!");
    }

    let new_user = DbUser {
        id: id.clone(),
        login,
        password,
        registered_at: utils::get_current_date(),
        role_id: 2,
        sessions: SessionsStore::new().add_rnd_session(),
    };

    let _ = api_utils::add_user(&new_user);

    Ok(id)
}

pub async fn logout(session: &Session) -> Result<(),()> {
    let user: DbUser = api_utils::user_by_id(&session.user_id).await.unwrap();
    let _new_sessions = user.sessions.del_session(&session.sess_id);
    unimplemented!("отправить измененные сессии на хранение в бд")
}

async fn _is_valid_session(session: &Session) -> bool {
    api_utils::user_by_id(&session.user_id)
        .await
        .filter(|user: &DbUser| user.sessions.is_exist(&session.sess_id))
        .is_some()
}
