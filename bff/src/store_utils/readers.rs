use super::URL;
use reqwest;
use serde::de::DeserializeOwned;

pub async fn all_users<T>() -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/users");
    fetch_by_url(&url).await.map_err(|err| err.to_string())
}

pub async fn all_roles<T>() -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/roles");
    fetch_by_url(&url).await.map_err(|err| err.to_string())
}

pub async fn find_users_by_kv<T>(key: &str, value: &str) -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/users/?{key}={value}");

    fetch_by_url(&url)
        .await
        .map_err(|err| err.to_string())
        .map(|x| x)
}

pub async fn find_first_user_by_kv<T>(key: &str, value: &str) -> Result<Option<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    find_users_by_kv(key, value)
        .await
        .map(|x| x.into_iter().next())
}

async fn fetch_by_url<T>(url: &str) -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    // leptos::logging::log!("{}", url);
    reqwest::get(url)
        .await
        .map_err(|err| err.to_string())?
        .json::<Vec<T>>()
        .await
        .map_err(|err| err.to_string())
}
