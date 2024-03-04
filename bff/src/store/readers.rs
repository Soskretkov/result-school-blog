use super::URL;
use crate::server::error::Error;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub async fn fetch<T>(path_suffix: &str) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let url = format!("{URL}/{path_suffix}");
    reqwest::get(url)
        .await
        .map_err(|reqw_err| match reqw_err.status() {
            Some(StatusCode::NOT_FOUND) => Error::DbEntryNotFound,
            _ => Error::Reqwest(reqw_err),
        })?
        .json::<T>()
        .await
        .map_err(Error::Reqwest)
}
