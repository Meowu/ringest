use anyhow::{anyhow, Result};
use git::{git_exists, is_valid_git_url};

mod git;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://github.com/zasper-io/zasper";
    let valid = is_valid_git_url(url).await?;
    if valid {
        println!("Git exists.");
    } else {
        println!("Invalid git url.");
    }

    let repo = git::clone_repo(url, "repos/zion").await?;
    println!("Successfully cloned repository to: {:?}", repo.path());
    Ok(())
}
