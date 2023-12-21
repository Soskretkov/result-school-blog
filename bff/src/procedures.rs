// содержит методы, которых нет у json-server
// то есть json-server предоставляется в ведение клиента,
// и только в особых случаях клиент вызывает этот модуль

use super::json_sv_utils;
mod utils;
mod types;
pub use types::{Sessions, User, Authentic};

pub async fn authorize(login: &str, password: &str) -> Authentic {
    let wrapped_user: Option<User> = json_sv_utils::user_info(login).await;

    match wrapped_user {
        Some(user) if user.password != password => {
            return Authentic {
                error: Some("Пароль не верен".to_string()),
                res: None,
            };
        }
        Some(user) => {
            let session = user.sessions.data.into_iter().next().unwrap();

            return Authentic {
                error: None,
                res: Some(session),
            };
        }
        None => {
            return Authentic {
                error: Some("Пользователь не найден".to_string()),
                res: None,
            };
        }
    }
}

pub async fn register(login: String, password: String) -> Authentic {
    let wrapped_user: Option<User> = json_sv_utils::user_info(&login).await;

    if wrapped_user.is_some() {
        return Authentic {
            error: Some("Логин уже занят".to_string()),
            res: None,
        };
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

    json_sv_utils::add_user(&new_user);

    let session = new_user.sessions.data.into_iter().next().unwrap();

    Authentic {
        error: None,
        res: Some(session),
    }
}
pub async fn logout(login: &str, session_id: &str) {
    // заменить на запрос по ручке
    let user: User = json_sv_utils::user_info(&login).await.unwrap();
    let _new_sessions = Sessions::del_session(user.sessions, session_id);
    unimplemented!("отправить измененные сессии на хранение в бд")
}
