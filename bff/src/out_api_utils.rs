use reqwest;
use serde::{de::DeserializeOwned, Serialize};

pub const URL: &'static str = "http://localhost:3005";

pub async fn all_users<T>() -> Vec<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/users");
    let users: Vec<T> = reqwest::get(url).await.unwrap().json().await.unwrap();
    users
}

pub async fn user_info<T>(user_id_to_find: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/id?login={user_id_to_find}");
    let relevant_users: Vec<T> = reqwest::get(url).await.unwrap().json().await.unwrap();

    relevant_users.into_iter().next()
}

pub async fn add_user<T>(new_user: &T)
where T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_user) // Установка тела запроса
        .send()
        .await
        .unwrap();
}
