use anyhow::{anyhow, Ok, Result};
use reqwest::Client;
use tokio::process::Command;

pub async fn git_exists(url: &str) -> Result<bool> {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg(url)
        .output()
        .await?;

    if output.status.success() {
        Ok(true)
    } else {
        let err_message = String::from_utf8_lossy(&output.stderr);

        if (err_message.contains("not found") || err_message.contains("does not exist")) {
            return Ok(false);
        }

        // such as auth err.
        Err(anyhow!("Failed to check git URL: {}", err_message))
    }
}

pub async fn is_valid_git_url(url: &str) -> Result<bool> {
    let client = Client::new();
    let response = client.head(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(true),
        reqwest::StatusCode::NOT_FOUND => Ok(false),
        _ => {
            let err_message = format!("Unexpected status code: {}", response.status());
            Err(anyhow!("Failed to check git URL: {}", err_message))
        }
    }
}
