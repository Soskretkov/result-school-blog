use super::shared::User;
use super::shared::constants::URL;
use leptos::logging;
use reqwest;

pub async fn get_users() -> Vec<User> {
    let url = format!("{URL}/users");
    let users: Vec<User> = reqwest::get(url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    users
}

pub async fn get_user(login_to_find: &str) -> Option<User> {
    let url = format!("{URL}/users?login={login_to_find}");
    let relevant_users: Vec<User> = reqwest::get(url).await.unwrap().json().await.unwrap();

    relevant_users.into_iter().next()
}

pub async fn add_user(new_user: &User) {
    logging::log!("БД: добавлен пользователь {}", new_user.login);
    let url = format!("{URL}/users");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_user) // Установка тела запроса
        .send()
        .await
        .unwrap();
}
