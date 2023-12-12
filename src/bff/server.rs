use super::db_utils::{self, User};
use super::session::Session;
use crate::shared::types::Role;
use chrono::{TimeZone, Utc};
use rand::Rng;

pub struct Authorize {
    pub error: Option<&'static str>,
    pub res: Option<Session>,
}

pub struct Server;

impl Server {
    pub async fn authorize(login: &str, password: &str) -> Authorize {
        let wrapped_user = db_utils::get_user(login).await;

        if wrapped_user.is_none() {
            return Authorize {
                error: Some("Такой пользователь не найден"),
                res: None,
            };
        }
        let user = wrapped_user.unwrap();

        if user.password != password {
            return Authorize {
                error: Some("Такой пользователь не найден"),
                res: None,
            };
        }

        let role_id = user.role_id;
        let role = Role::from_id(role_id).unwrap();
        let session = Session::new(role);

        Authorize {
            error: None,
            res: Some(session),
        }
    }

    pub async fn register(login: String, password: String) -> Authorize {
        let user = db_utils::get_user(&login).await;

        if user.is_some() {
            return Authorize {
                error: Some("Такой логин уже занят"),
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

        Authorize {
            error: None,
            res: Some(Session::new(Role::Reader)),
        }
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
