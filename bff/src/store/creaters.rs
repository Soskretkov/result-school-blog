use super::URL;
use crate::server::error::Error;
use reqwest::StatusCode;
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
        .await
        .map_err(|reqw_err| match reqw_err.status() {
            Some(StatusCode::NOT_FOUND) => Error::DbEntryNotFound,
            _ => Error::Reqwest(reqw_err),
        })?
        .json::<U>()
        .await
        .map_err(Error::Reqwest)
}
