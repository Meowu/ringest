use anyhow::{anyhow, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::path::{Path, PathBuf};
use std::{error, fs, path};

pub fn walk_dir(
    root: &str,
    include_patterns: &[&str],
    exclude_patterns: &[&str],
    respect_gitignore: bool,
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut files = Vec::new();

    let mut overrides_builder = OverrideBuilder::new(root);

    // Override 规则中的 ! 含义与标准 .gitignore 文件中的含义是相反的。
    // overrides.add("include_pattern") => 列入白名单 (Whitelist)
    // overrides.add("!exclude_pattern") => 列入黑名单/排除 (Ignore/Exclude within overrides)
    for pattern in include_patterns {
        overrides_builder.add(&format!("{}", pattern))?;
    }

    for pattern in exclude_patterns {
        overrides_builder.add(&format!("!{}", pattern))?;
    }

    let overrides = overrides_builder.build()?;

    let mut builder = WalkBuilder::new(root);
    builder.standard_filters(respect_gitignore);

    builder.overrides(overrides);

    // builder.follow_links(true).hidden(false);

    for result in builder.build() {
        match result {
            Ok(entry) => {
                // skip directory
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    // println!("Directory: {}", entry.path().display());
                    continue;
                }

                if let Some(path_str) = entry.path().to_str() {
                    files.push(path_str.to_string());
                }
            }
            Err(err) => {
                eprint!("遍历错误: {}", err);
            }
        }
    }

    Ok(files)
}

fn is_text_file(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "rs" | "py" | "js" | "ts" | "tsx" | "jsx" | "mjs" | "cjs" | "mts" | "java" | "c"
            | "cpp" | "h" | "hpp" | "go" | "php" | "rb" | "swift" | "kt" | "scala" | "sql"
            | "css" | "html" | "htm" | "xml" | "json" | "yaml" | "yml" | "toml" | "sh" | "bash"
            | "dockerfile" | "makefile" | "txt" | "md" | "markdown" | "text" | "log" | "conf" => {
                true
            }
            _ => false,
        }
    } else {
        false
    }
}

async fn read_file(file_path: &Path) -> Result<String> {
    fs::read_to_string(file_path)
        .map_err(|e| anyhow!("Failed to read file: {}, {}", file_path.display(), e))
}

async fn process_path(repo_path: &Path, path: &str, content: &mut String) -> Result<()> {
    let full_path = repo_path.join(path);

    if full_path.is_file() {
        if is_text_file(&full_path) {
            let text = read_file(&full_path).await?;
            content.push_str(&format!("\n===== Start of file: {} =====\n", path));
            content.push_str(&text);
            content.push_str("\n===== End of file =====\n");
        } else {
            println!("Skipping non-text file: {}", full_path.display());
        }
    } else if full_path.is_dir() {
        let entries = fs::read_dir(&full_path)
            .map_err(|e| anyhow!("Failed to read directory: {}, {}", full_path.display(), e))?;

        for entry in entries {
            let entry = entry?;
            // let file_name = entry.file_name().to_string_lossy();
            // let sub_path = Path::new(&path)
            //     .join(file_name)
            //     .to_str()
            //     .expect("Invalid path");

            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_str().ok_or(anyhow!("Invalid name"))?;

            let mut sub_path = PathBuf::from(path);
            sub_path.push(file_name);

            let sub_path_str = sub_path.to_str().ok_or(anyhow!("Invalid file path"))?;

            Box::pin(process_path(repo_path, sub_path_str, content)).await?;
        }
    } else {
        return Err(anyhow!("Invalid path: {}", full_path.display()));
    }

    Ok(())
}

pub async fn read_and_concat_files(repo_path: &str, files: Vec<String>) -> Result<String> {
    let repo_path = Path::new(repo_path);

    let mut content = String::new();

    for path in files {
        if path.is_empty() {
            continue;
        }
        process_path(repo_path, &path, &mut content).await?;
    }
    Ok(content)
}
