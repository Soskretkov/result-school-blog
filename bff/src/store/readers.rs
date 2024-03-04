use super::URL;
use reqwest::Error;
use serde::de::DeserializeOwned;

pub async fn fetch<T>(path_suffix: &str) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/{path_suffix}");
    reqwest::get(url)
        .await?
        .json::<T>()
        .await
}
