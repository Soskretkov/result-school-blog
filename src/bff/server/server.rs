use super::{Authentic, CurrentSession};
use crate::bff::db_utils;
use crate::bff::shared::{Sessions, User};
use crate::utils;
use chrono::{TimeZone, Utc};
use leptos::*;

#[derive(Clone)]
pub struct Server {
}

impl Server {
    pub async fn authorize(login: &str, password: &str) -> Authentic {
        let wrapped_user = db_utils::get_user(login).await;

        match wrapped_user {
            Some(user) if user.password != password => {
                logging::log!("Пароль не верен");
                return Authentic {
                    error: Some("Пароль не верен".to_string()),
                    res: None,
                };
            }
            Some(user) => {                
                let session = CurrentSession::new(user);

                logging::log!("Сервер дал успешный ответ");
                return Authentic {
                    error: None,
                    res: Some(session),
                };
            }
            None => {
                logging::log!("Такой пользователь не найден");
                return Authentic {
                    error: Some("Такой пользователь не найден".to_string()),
                    res: None,
                };
            }
        }
    }

    pub async fn register(login: String, password: String) -> Authentic {
        let wrapped_user = db_utils::get_user(&login).await;

        if wrapped_user.is_some() {
            logging::log!("Такой логин уже занят");
            return Authentic {
                error: Some("Такой логин уже занят".to_string()),
                res: None,
            };
        }

        let sessions = Sessions::new().add_rnd_session();

        let new_user = User {
            id: utils::create_rnd_float64().to_string(),
            login,
            password,
            registed_at: get_rnd_date(),
            role_id: 2,
            sessions: sessions
        };

        db_utils::add_user(&new_user);

        let session = CurrentSession::new(new_user);

        logging::log!("Сервер дал успешный ответ");
        Authentic {
            error: None,
            res: Some(session),
        }
    }
    pub async fn logout(login: &str, session_id: &str) {
        // заменить на запрос по ручке
        let user = db_utils::get_user(&login).await.unwrap();
        let _new_sessions = Sessions::del_session(user.sessions, session_id);
        unimplemented!("отправить измененные сессии на хранение в бд")
    }
}

fn get_rnd_date() -> String {
    let random_float: f64 = utils::create_rnd_float64();

    let msecs = (random_float * 1000000000000.0 + 1999999999999.0) as i64;

    let secs = msecs / 1000;
    let nsecs = (msecs % 1000) * 1000000;

    Utc.timestamp_opt(secs, nsecs as u32);

    let date_time = Utc.timestamp_opt(secs, nsecs as u32).unwrap();

    let iso_str = date_time.to_rfc3339();

    iso_str
        .chars()
        .take(16)
        .collect::<String>()
        .replace("T", " ")
}
