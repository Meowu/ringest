use anyhow::{anyhow, Ok, Result};
use git2::{self, build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};
use reqwest::Client;
use std::path::{self, Path};
use tokio::process::Command;

pub fn open_repository(path: &str) -> Result<Repository, git2::Error> {
    Repository::open(path)
}

pub async fn _clone_repo(url: &str, path_str: &str) -> Result<Repository> {
    let repo = RepoBuilder::new().clone(url, path::Path::new(path_str))?;
    Ok(repo)
}

pub async fn clone_repo(url: &str, dest: &str) -> Result<()> {
    let path = Path::new(dest);
    // if path.exists() {
    //     return Err(anyhow!("Destination path already exists: {}", dest));
    // }

    let mut builder = RepoBuilder::new();
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks());

    builder.fetch_options(fetch_options).clone(url, path)?;
    Ok(())
}

fn remote_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_url, _username_from_url, _allow_types| Cred::default());
    remote_callbacks
}

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

        if err_message.contains("not found") || err_message.contains("does not exist") {
            println!("Not found repo.");
            return Ok(false);
        }

        // such as auth err.
        println!("ERR code: {}", err_message);
        Err(anyhow!("Failed to check git URL: {}", err_message))
    }
}

pub async fn is_valid_git_url(url: &str) -> Result<bool> {
    let client = Client::new();
    let response = client.head(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(true),
        reqwest::StatusCode::NOT_FOUND => {
            println!("Repo not found.");
            Ok(false)
        }
        _ => {
            let err_message = format!("Unexpected status code: {}", response.status());
            println!("ERR code: {}", err_message);
            Err(anyhow!("Failed to check git URL: {}", err_message))
        }
    }
}
