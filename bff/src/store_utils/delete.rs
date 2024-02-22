use super::URL;
use reqwest;

pub async fn delete_users_by_kv(key: &str, value: &str) -> Result<(), String> {
    let url = format!("{URL}/users/?{key}={value}");
    let client = reqwest::Client::new();

    match client.delete(url).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
