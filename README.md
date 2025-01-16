# Ringest

Rust Ingest, a Rust-based code ingestion tool for preparing codebases for analysis with Large Language Models (LLMs).

## Overview

Ringest simplifies the process of preparing code for LLM analysis by:
- Cloning remote code repositories or reading local code directories
- Filtering files based on include/exclude patterns
- Processing and formatting text files with appropriate markers
- Outputting the processed content to a file or clipboard
- Providing specialized code analysis prompts

This tool bridges the gap between codebases and language models, making it easier to analyze and understand code using AI assistants.

## Key Features

- **Repository Processing**: Clone remote Git repositories or process local directories
- **Selective File Processing**: Include or exclude files using custom patterns
- **Text File Recognition**: Automatically identify and process common text file formats
- **File Marking**: Add clear start/end markers to distinguish between files
- **Flexible Output**: Choose between file output or clipboard copying
- **Analysis Prompts**: Built-in prompts for Rust-specific and general code analysis
- **Gitignore Support**: Respect `.gitignore` rules when processing files

## Installation

### Prerequisites
- Rust toolchain (rustc, cargo)
- Git (for cloning repositories)

### Installing from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ringest.git
cd ringest

# Build the project
cargo build --release

# Run the executable
./target/release/ringest
```

## Usage

```bash
# Basic usage with local directory
ringest -s src

# Clone and process a remote repository
ringest -a https://github.com/username/repo.git

# Include only specific file types
ringest -a . -i "*.rs" "*.toml"

# Exclude specific files or directories
ringest -a . -e "target/*" "*.json"

# Output to a specific file
ringest -a . -o code_analysis.txt

# Copy output to clipboard
ringest -a . -c

# Combine multiple options
ringest -a https://github.com/username/repo.git -i "*.rs" -e "tests/*" -o analysis.txt
```

### Command Line Arguments

| Argument | Long form | Description | Default |
|----------|-----------|-------------|---------|
| `-a` | `--address` | Repository URL or local directory path | `.` (current directory) |
| `-s` | `--src` | Source directory within the repository | `src` |
| `-c` | `--copy` | Copy output to clipboard | `false` |
| `-o` | `--output` | Output file path | `./llms.txt` |
| `-i` | `--include` | Patterns to include (comma-separated) | None (include all) |
| `-e` | `--exclude` | Patterns to exclude (comma-separated) | None (exclude nothing) |

## Project Architecture

Ringest consists of several modules working together to process and prepare code:

- **main.rs**: Entry point and CLI argument handling
- **git.rs**: Repository cloning and URL validation
- **file.rs**: File traversal, filtering, and content processing
- **llm.rs**: LLM client interface (for future integration)
- **prompts.rs**: Predefined analysis prompts

The tool follows this workflow:
1. Parse command line arguments
2. Clone repository (if remote) or use local directory
3. Walk through files applying include/exclude filters
4. Process each file with appropriate formatting
5. Concatenate all content with file markers
6. Output the result to a file or clipboard

## Dependencies

This project relies on the following Rust crates:
- `reqwest`: HTTP client for API requests
- `tokio`: Asynchronous runtime
- `serde`/`serde_json`: JSON serialization/deserialization
- `anyhow`: Error handling
- `clap`: Command line argument parsing
- `git2`: Git repository operations
- `async-trait`: Async trait support
- `tempfile`: Temporary file/directory handling
- `arboard`: Clipboard access
- `ignore`: File pattern matching and .gitignore support

## License

MIT

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.
