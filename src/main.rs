use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use anyhow::{anyhow, Result};
use git::{git_exists, is_valid_git_url};

mod file;
mod git;
mod llm;
mod prompts;


#[tokio::main]
async fn main() -> Result<()> {

    let string_to_path = |seg: &str| { format!("repos/{}", seg) };

    let otter = "https://github.com/tchayen/red-otter.git";

    // let valid = is_valid_git_url(url).await?;
    // if valid {
    //     println!("Git repo exists.");
    // } else {
    //     println!("Invalid git url.");
    // }

    let repo_name = Path::new(otter)
        .file_stem()
        .ok_or(anyhow!("Invalid repo url format"))?
        .to_str()
        .ok_or(anyhow!("Invalid repo url format"))?;
    let clone_dest = string_to_path(repo_name);
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

    let api_url = "https://api.openai.com/v1/chat/completions";
    let file_name = "bore/llm.txt";
    let dest = string_to_path(file_name);
    let code = fs::read_to_string(&dest)?;
    let lines: Vec<&str> = code.lines().collect();
    let len = lines.iter().count();
    println!("Total {} lines under {}", len, dest);


    let model = "gpt-4o";
    let content = prompts::prompts::RUST_PROMPT.replace("{{file_content}}", &code);
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY not exists");

    let llm_client = llm::OpenAIClient::new(api_url);

    println!("Prompting...");
    let answer = llm::send_to_llm(&llm_client, &api_key, model, &content).await?;
    let output = format!("LLM Response:\n{}", answer);
    fs::write(string_to_path("bore/llm_output.md"), output)?;
    println!("Done!");

    Ok(())
}
