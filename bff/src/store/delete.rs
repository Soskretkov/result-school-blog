use super::URL;
use crate::server::error::Error;
use reqwest::{StatusCode};
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
        .await
        .map_err(|reqw_err| match reqw_err.status() {
            Some(StatusCode::NOT_FOUND) => Error::DbEntryNotFound,
            _ => Error::Reqwest(reqw_err),
        })?
        .json::<T>()
        .await
        .map_err(Error::Reqwest)
}
