use super::URL;
use reqwest::Error;

pub async fn delete(path_suffix: &str) -> Result<(), Error> {
    let url = format!("{URL}/{path_suffix}");
    let client = reqwest::Client::new();

    client.delete(url).send().await.map(|_| ())
}
