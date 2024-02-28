use super::URL;
use reqwest;

pub async fn delete(path_suffix: &str) -> Result<(), String> {
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    match client.delete(url).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
