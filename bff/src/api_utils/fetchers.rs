use reqwest;
use serde::de::DeserializeOwned;
use super::URL;
use leptos::*;

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

pub async fn find_user_by_kv<T>(key: &str, value: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let url = format!("{URL}/users/?{key}={value}");
    let relevant_users = fetch_by_url(&url).await;
    relevant_users.into_iter().next()
}

async fn fetch_by_url<T>(url: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    logging::log!("{}", url);

    reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<T>>()
        .await
        .unwrap()
}
