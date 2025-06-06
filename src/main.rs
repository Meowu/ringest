use anyhow::{anyhow, Result};
use arboard::Clipboard;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

mod file;
mod git;
mod llm;
mod prompts;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    address: String,

    #[arg(short, long, default_value = "src")]
    src: String,

    #[arg(short, long)]
    copy: bool,

    #[arg(short, long, default_value = "./llms.txt")]
    output: PathBuf,

    #[arg(short, long)]
    include: Option<String>,

    #[arg(short, long)]
    exclude: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let address = cli.address;
    let is_remote = address.starts_with("http");
    let output_path = cli.output;

    let mut code_text = String::new();

    let src_dir = if is_remote {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir
            .path()
            .to_str()
            .ok_or_else(|| anyhow!("Failed to convert temp path to string"))?;
        println!("temp dir: {:?}", dir_path);
        git::clone_repo(&address, dir_path).await?;

        // let output = Command::new("ls")
        //     .arg("-al")
        //     .current_dir(dir_path)
        //     .output()?;
        // println!(
        //     "Directory listing:\n{}",
        //     String::from_utf8_lossy(&output.stdout)
        // );
        dir_path.to_string()
    } else {
        String::from(".")
    };
    let include_patterns: Vec<&str> = if let Some(pattern) = &cli.include {
        vec![pattern]
    } else {
        vec![]
    };
    let exclude_patterns = cli
        .exclude
        .as_deref()
        .map(|pattern| vec![pattern])
        .unwrap_or_default();

    let files = file::walk_dir(&src_dir, &include_patterns, &exclude_patterns, true).unwrap();

    let code = file::read_and_concat_files(&src_dir, files).await?;
    code_text.push_str(&code);

    let lines: Vec<&str> = code_text.lines().collect();
    let len = lines.iter().count();

    // todo: calculate token cost.
    if cli.copy {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(code_text).unwrap();
        println!(
            "✓ Code has been copied to clipboard, {} lines in total.",
            len
        );
    } else {
        fs::write(&output_path, &code_text)?;
        println!(
            "✓ Code has been written to {}, {} lines in total.",
            output_path.display(),
            len
        );
    }

    Ok(())
}
