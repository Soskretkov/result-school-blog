use super::session::Session;
use super::shared::User;
use super::{db_utils, sessions::Sessions};
use chrono::{TimeZone, Utc};
use leptos::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
// клиентская сторона не конфигурирует Authorize, это ответ сервера
pub struct Authentic {
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

    pub async fn authorize(&mut self, login: &str, password: &str) -> Authentic {
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
                let session_id = self.sessions.create();
                let session = Session::new(user, session_id);

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

    pub async fn register(&mut self, login: String, password: String) -> Authentic {
        let wrapped_user = db_utils::get_user(&login).await;

        if wrapped_user.is_some() {
            logging::log!("Такой логин уже занят");
            return Authentic {
                error: Some("Такой логин уже занят".to_string()),
                res: None,
            };
        }

        let new_user = User {
            id: rand::thread_rng().gen::<f64>().to_string(),
            login,
            password,
            registed_at: get_rnd_date(),
            role_id: 2,
        };

        db_utils::add_user(&new_user);

        let session_id = self.sessions.create();
        let session = Session::new(new_user, session_id);

        logging::log!("Сервер дал успешный ответ");
        Authentic {
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
