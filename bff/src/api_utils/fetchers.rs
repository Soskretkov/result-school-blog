use reqwest;
use serde::de::DeserializeOwned;
use super::URL;

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

pub async fn user_by_id<T>(id_to_find: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/id?id={id_to_find}");
    let relevant_users = fetch_by_url(&url).await;
    relevant_users.into_iter().next()
}

pub async fn user_by_login<T>(id_to_find: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/id?login={id_to_find}");
    let relevant_users = fetch_by_url(&url).await;
    relevant_users.into_iter().next()
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
