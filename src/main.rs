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

    let repo = otter;
    let repo_name = Path::new(repo)
        .file_stem()
        .ok_or(anyhow!("Invalid repo url format"))?
        .to_str()
        .ok_or(anyhow!("Invalid repo url format"))?;
    let clone_dest = string_to_path(repo_name);

    if !Path::new(&clone_dest).exists() {
        println!("Ready to clone into: {}", clone_dest);
        git::clone_repo(repo, &clone_dest).await?;
        println!("Successfully cloned repository to: {:?}", clone_dest);
    } else {
        println!("Repo exists, ready to concat files.");
    }

    // concat files and write to local.
    let mut code_path = PathBuf::from(&clone_dest);
    code_path.push("llm.txt");

    let mut code_text = String::new();
    if !code_path.exists() {
        let code = file::read_and_concat_files(&clone_dest, "src").await?;
        fs::write(&code_path, &code)?;
        code_text.push_str(&code);
    } else {
        let code = fs::read_to_string(&code_path)?;
        code_text.push_str(&code);
    }

    let lines: Vec<&str> = code_text.lines().collect();
    let len = lines.iter().count();
    println!("Successfully written concat content to {}, {} lines in total.", code_path.display(), len);

    let api_url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-4o";
    let content = prompts::prompts::COMMON_PROMPT.replace("{{file_content}}", &code_text);
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY not exists");

    let llm_client = llm::OpenAIClient::new(api_url);

    println!("Prompting...");
    let answer = llm::send_to_llm(&llm_client, &api_key, model, &content).await?;
    let output = format!("LLM Response:\n{}", answer);
    fs::write(format!("{}/llm_output_common.md", clone_dest), output)?;
    println!("Done!");

    Ok(())
}
