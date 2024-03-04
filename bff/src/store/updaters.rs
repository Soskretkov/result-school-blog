use super::URL;
use reqwest::{Error, Response};
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
}
