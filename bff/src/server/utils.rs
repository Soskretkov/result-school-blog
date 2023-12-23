use rand::{Rng, thread_rng};
use chrono::{TimeZone, Utc};

pub fn create_rnd_float64() -> f64 {
    let mut rng = thread_rng(); // Получаем генератор случайных чисел
    let random_float: f64 = rng.gen();
    random_float
}


pub fn get_rnd_date() -> String {
    let random_float: f64 = create_rnd_float64();

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

pub fn get_current_date() -> String {
    let now = Utc::now(); // Получаем текущее время
    now.format("%Y-%m-%d %H:%M").to_string() // Форматируем дату и время
}

