use crate::server::error::Error;
use crate::server::types::db_interaction::User;
use crate::server::types::Session;
use crate::store;
use chrono::{DateTime, TimeZone, Utc};
use rand::{thread_rng, Rng};

pub async fn get_user(id: &str) -> Result<User, Error> {
    let path_suffix = format!("users/?id={}", id);
    store::fetch::<Vec<User>>(&path_suffix)
        .await
        .map_err(Error::Reqwest)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::DbEntryNotFound)
}

// не делаю на session, иначе метод будет и у клиента (раскроет пароль)
pub async fn verify_session(session: &Session) -> Result<User, Error> {
    let db_user = get_user(&session.user_id).await.map_err(|_| Error::InvalidSession)?;

    if !db_user.payload.sessions.exists(&session.id) {
        return Err(Error::InvalidSession);
    }

    Ok(db_user)
}

pub async fn verify_session_with_permissions<F>(
    session: &Session,
    check_perm: F,
) -> Result<User, Error>
where
    F: FnOnce(&User) -> bool,
{
    let db_user = verify_session(session).await?;
    // выход по ошибке если fn передана проверка, которая провалилась
    if !check_perm(&db_user) {
        return Err(Error::UserPermission);
    }

    Ok(db_user)
}

pub fn _create_rnd_float64() -> f64 {
    let mut rng = thread_rng(); // Получаем генератор случайных чисел
    let random_float: f64 = rng.gen();
    random_float
}

pub fn _get_rnd_date() -> String {
    let random_float: f64 = _create_rnd_float64();

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

pub fn get_current_date() -> DateTime<Utc> {
    Utc::now() // Получаем текущее время
    //  в соответствии с RFC 3339, профилем стандарта ISO 8601 для интернета
    // now.format("%+").to_string()
    // now.format("%Y-%m-%d").to_string()
}
