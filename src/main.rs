use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod file;
mod git;
mod llm;
mod prompts;

#[tokio::main]
async fn main() -> Result<()> {
    let string_to_path = |seg: &str| format!("repos/{}", seg);

    let otter = "https://github.com/tchayen/red-otter.git";
    let pratt = "https://github.com/matklad/minipratt";

    // let valid = is_valid_git_url(url).await?;
    // if valid {
    //     println!("Git repo exists.");
    // } else {
    //     println!("Invalid git url.");
    // }

    let repo = pratt;
    let repo_name = Path::new(repo)
        .file_stem()
        .ok_or(anyhow!("Invalid repo url format"))?
        .to_str()
        .ok_or(anyhow!("Invalid repo url format"))?;
    let clone_dest = string_to_path(repo_name);

    println!("Ready to clone into: {}", clone_dest);

    git::clone_repo(repo, &clone_dest).await?;
    println!("Successfully cloned repository to: {:?}", clone_dest);

    // concat files and write to local.
    let code_text = file::read_and_concat_files(&clone_dest, "src").await?;
    let mut output_txt = PathBuf::from(&clone_dest);
    output_txt.push("llm.txt");
    fs::write(&output_txt, &code_text)?;

    let lines: Vec<&str> = code_text.lines().collect();
    let len = lines.iter().count();
    println!("Successfully written concat content to {}, {} lines in total.", output_txt.display(), len);

    let api_url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-4o";
    let content = prompts::prompts::RUST_PROMPT.replace("{{file_content}}", &code_text);
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY not exists");

    let llm_client = llm::OpenAIClient::new(api_url);

    println!("Prompting...");
    let answer = llm::send_to_llm(&llm_client, &api_key, model, &content).await?;
    let output = format!("LLM Response:\n{}", answer);
    fs::write(format!("{}/llm_output.md", clone_dest), output)?;
    println!("Done!");

    Ok(())
}
