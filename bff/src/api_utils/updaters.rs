use super::URL;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

pub async fn add_user<T>(new_user: &T)
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users");
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_user) // установка тела запроса
        .send()
        .await
        .unwrap();
}

pub async fn update_user_sessions<T>(user_id: &str, sessions: &T)
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users/{}", user_id);
    let client = reqwest::Client::new();

    client
        .patch(url)
        .json(&json!({ "sessions": sessions }))
        .send()
        .await
        .unwrap();
}
