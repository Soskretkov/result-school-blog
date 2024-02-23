use super::URL;
use reqwest;

pub async fn delete_user(id: &str) -> Result<(), String> {
    let url = format!("{URL}/users/{id}");
    let client = reqwest::Client::new();

    match client.delete(url).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
