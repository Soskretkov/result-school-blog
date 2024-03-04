use super::URL;
use reqwest::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn delete<T>(path_suffix: &str) -> Result<T, Error>
where
    T: DeserializeOwned + Serialize + Debug,
{
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    client
        .delete(url)
        .send()
        .await?
        .json::<T>()
        .await
}
