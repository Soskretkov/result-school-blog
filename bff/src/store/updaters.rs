use super::URL;
use crate::server::error::Error;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::fmt::Debug;

pub async fn update_field<T>(
    path_suffix: &str,
    field_name: &str,
    field_value: &T,
) -> Result<Response, Error>
where
    T: DeserializeOwned + Serialize + Debug,
{
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    client
        .patch(url)
        .json(&json!({ field_name: field_value }))
        .send()
        .await
        .map_err(|reqw_err| match reqw_err.status() {
            Some(StatusCode::NOT_FOUND) => Error::DbEntryNotFound,
            _ => Error::Reqwest(reqw_err),
        })
}
