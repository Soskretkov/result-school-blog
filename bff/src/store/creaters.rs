use super::URL;
use reqwest::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn add<T, U>(path_suffix: &str, new_data: &T) -> Result<U, Error>
where
    T: DeserializeOwned + Serialize + Debug,
    U: DeserializeOwned + Serialize + Debug,
{
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_data) // установка тела запроса
        .send()
        .await?
        .json::<U>()
        .await
}
