use crate::api_utils;
use serde::de::DeserializeOwned;
pub use api_utils::test;

pub async fn fetch_all_users<T>() -> Result<Vec<T>, String>
where
    T: DeserializeOwned,
{
    Ok(api_utils::all_users().await)
}

pub async fn fetch_all_roles<T>() -> Vec<T>
where
    T: DeserializeOwned,
{
    // Ok(api_utils::all_roles().await)
    todo!()
}
