use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub password: String,
    pub registed_at: String,
    pub role_id: u8,
}

pub async fn get_users() -> Vec<User> {
    let users: Vec<User> = reqwest::get("http://localhost:3005/users")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    users
}

pub async fn get_user(login: &str) -> Option<User> {
    let users: Vec<User> = get_users().await;
    let user = users.into_iter().find(|item| item.login == login);
    user
}

pub async fn add_user(new_user: &User) {
    let client = reqwest::Client::new();

    client
        .post("http://localhost:3005/users")
        .json(new_user) // Установка тела запроса
        .send()
        .await
        .unwrap();
}
