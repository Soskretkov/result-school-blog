use super::URL;
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn add<T>(path_suffix: &str, new_data: &T) -> Result<Response, String>
where
    T: DeserializeOwned + Serialize + Debug,
{
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_data) // установка тела запроса
        .send()
        .await
        .map_err(|err| err.to_string())
}