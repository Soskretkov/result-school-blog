use super::URL;
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};

pub async fn add_user<T>(new_user: &T) -> Result<Response, String>
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_user) // установка тела запроса
        .send()
        .await
        .map_err(|err| err.to_string())
}