use super::session::Session;
use super::shared::User;
use super::{db_utils, sessions::Sessions};
use chrono::{TimeZone, Utc};
use leptos::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
// клиентская сторона не конфигурирует Authorize, это ответ сервера
pub struct Authorize {
    pub error: Option<String>,
    pub res: Option<Session>,
}

#[derive(Clone)]
pub struct Server {
    sessions: Sessions,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: Sessions::new(),
        }
    }

    pub async fn authorize(&mut self, login: &str, password: &str) -> Authorize {
        let wrapped_user = db_utils::get_user(login).await;

        if let Some(user) = wrapped_user {
            if user.password != password {
                logging::log!("Пароль не верен");

                return Authorize {
                    error: Some("Пароль не верен".to_string()),
                    res: None,
                };
            }

            logging::log!("Сервер дал успешный ответ");
            let session_id = self.sessions.create();
            let session = Session::new(user, session_id);

            return Authorize {
                error: None,
                res: Some(session),
            };
        } else {
            logging::log!("Такой пользователь не найден");
            return Authorize {
                error: Some("Такой пользователь не найден".to_string()),
                res: None,
            };
        };
    }

    pub async fn register(&mut self, login: String, password: String) -> Authorize {
        let wrapped_user = db_utils::get_user(&login).await;

        if wrapped_user.is_some() {
            return Authorize {
                error: Some("Такой логин уже занят".to_string()),
                res: None,
            };
        }

        let registed_at = get_rnd_date();

        let new_user = User {
            id: "002".to_string(),
            login,
            password,
            registed_at,
            role_id: 2,
        };

        db_utils::add_user(&new_user).await;

        let session_id = self.sessions.create();
        let session = Session::new(new_user, session_id);

        Authorize {
            error: None,
            res: Some(session),
        }
    }
    pub async fn logout(&mut self, session_id: &str) -> bool {
        self.sessions.del(session_id)
    }
}

fn get_rnd_date() -> String {
    let mut rng = rand::thread_rng(); // Получаем генератор случайных чисел
    let random_float: f64 = rng.gen();

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
