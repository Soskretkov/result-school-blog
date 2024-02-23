use super::URL;
use reqwest;
use serde::de::DeserializeOwned;

pub async fn all_users<T>() -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/users");
    fetch_to_vec_by_url(&url)
        .await
        .map_err(|err| err.to_string())
}

pub async fn all_roles<T>() -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/roles");
    fetch_to_vec_by_url(&url)
        .await
        .map_err(|err| err.to_string())
}

pub async fn user<T>(id: &str) -> Result<Option<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/users/{id}");

    reqwest::get(url)
        .await
        .map_err(|err| err.to_string())?
        .json::<Option<T>>()
        .await
        .map_err(|err| err.to_string())
}

pub async fn find_user_by_kv<T>(key: &str, value: &str) -> Result<Vec<T>, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/users/?{key}={value}");

    fetch_to_vec_by_url(&url)
        .await
        .map_err(|err| err.to_string())
        .map(|x| x)
}

async fn fetch_to_vec_by_url<T>(url: &str) -> Result<Vec<T>, String>
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
