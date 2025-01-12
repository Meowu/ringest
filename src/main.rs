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

    #[arg(short, long)]
    output: Option<String>,

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

    let mut code_text = String::new();
    let output_path = if let Some(output) = cli.output {
        PathBuf::from(&output)
    } else {
        PathBuf::from(".")
    };
    let output_path = output_path.join("llms.txt");

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
    let code = file::read_and_concat_files(&src_dir, &cli.src).await?;
    code_text.push_str(&code);

    let lines: Vec<&str> = code_text.lines().collect();
    let len = lines.iter().count();
    // println!(
    //     "Successfully written concat content to {}, {} lines in total.",
    //     output_path.display(),
    //     len
    // );

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
