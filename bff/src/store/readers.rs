use super::URL;
use reqwest;
use serde::de::DeserializeOwned;

pub async fn fetch<T>(path_suffix: &str) -> Result<T, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/{path_suffix}");
    reqwest::get(url)
        .await
        .map_err(|err| err.to_string())?
        .json::<T>()
        .await
        .map_err(|err| err.to_string())
}
