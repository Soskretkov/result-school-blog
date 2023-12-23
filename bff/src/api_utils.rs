use reqwest;
use serde::{de::DeserializeOwned, Serialize};

pub const URL: &'static str = "http://localhost:3005";

pub async fn test() -> String {
    "Успешный тест async fn".to_string()
}

pub async fn all_users<T>() -> Vec<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/users");
    fetch_by_url(&url).await
}

pub async fn all_roles<T>() -> Vec<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/roles");
    fetch_by_url(&url).await
}

pub async fn user<T>(user_id_to_find: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/id?login={user_id_to_find}");
    let relevant_users = fetch_by_url(&url).await;
    relevant_users.into_iter().next()
}

pub async fn add_user<T>(new_user: &T)
where
    T: DeserializeOwned + Serialize,
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

async fn fetch_by_url<T>(url: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<T>>()
        .await
        .unwrap()
}
