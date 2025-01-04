use std::path::Path;

use anyhow::{anyhow, Result};
use git::{git_exists, is_valid_git_url};

mod git;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://github.com/zasper-io/zasper";
    let bore = "https://github.com/ekzhang/bore.git";
    let valid = is_valid_git_url(url).await?;
    if valid {
        println!("Git repo exists.");
    } else {
        println!("Invalid git url.");
    }

    let repo_name = Path::new(bore)
        .file_stem()
        .ok_or(anyhow!("Invalid repo url format"))?
        .to_str()
        .ok_or(anyhow!("Invalid repo url format"))?;
    let clone_dest = format!("repos/{}", repo_name);
    println!("Ready to clone into: {}", clone_dest);

    git::clone_repo(bore, &clone_dest).await?;
    println!("Successfully cloned repository to: {:?}", clone_dest);
    Ok(())
}
