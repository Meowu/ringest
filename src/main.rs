use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use git::{git_exists, is_valid_git_url};

mod file;
mod git;
mod llm;

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

    // git::clone_repo(bore, &clone_dest).await?;
    // println!("Successfully cloned repository to: {:?}", clone_dest);

    // concat files and write to local.
    // let contents = file::read_and_concat_files(&clone_dest, "src").await?;
    //
    // let mut output_path = PathBuf::from(&clone_dest);
    // output_path.push("llm.txt");
    // std::fs::write(&output_path, contents)?;
    // println!("Successfully written content to {}", output_path.display());

    let api_url = "https://api.openai.com/v1/chat/completions"
    let model = "gpt-4o";
    let content = "What's the sky color?";
    let api_key = "";

    let llm_client = llm::OpenAIClient::new(api_url);

    let answer = llm::send_to_llm(&llm_client, api_key, model, content).await?;
    println!("\n{}\nLLM Response:\n{}", content, answer);
    Ok(())
}
