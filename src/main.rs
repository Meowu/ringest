use anyhow::{anyhow, Result};
use git::is_valid_git_url;

mod git;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://github.com/tokio-rs/axum";
    let valid = is_valid_git_url(url).await.is_err_and(|e| {
        println!("invalid --> {:?}", e);
        false
    });
    if valid {
        println!("Git exists.");
    } else {
        println!("Invalid git url.");
    }
    Ok(())
}
