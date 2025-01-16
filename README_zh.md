# Ringest

Rust Ingest, 一个基于 Rust 的代码提取工具，用于准备代码库以供大型语言模型（LLMs）分析。

## 概述

Ringest 通过以下方式简化了为 LLM 分析准备代码的过程：
- 克隆远程代码仓库或读取本地代码目录
- 基于包含/排除模式过滤文件
- 处理并使用适当的标记格式化文本文件
- 将处理后的内容输出到文件或剪贴板
- 提供专门的代码分析提示

该工具弥合了代码库与语言模型之间的差距，使用 AI 助手分析和理解代码变得更加容易。

## 主要特性

- **仓库处理**：克隆远程 Git 仓库或处理本地目录
- **选择性文件处理**：使用自定义模式包含或排除文件
- **文本文件识别**：自动识别并处理常见的文本文件格式
- **文件标记**：添加清晰的开始/结束标记以区分不同文件
- **灵活输出**：可选择文件输出或剪贴板复制
- **分析提示**：内置 Rust 专用和通用代码分析提示
- **Gitignore 支持**：处理文件时尊重 `.gitignore` 规则

## 安装

### 前提条件
- Rust 工具链（rustc, cargo）
- Git（用于克隆仓库）

### 从源代码安装
```bash
# 克隆仓库
git clone https://github.com/yourusername/ringest.git
cd ringest

# 构建项目
cargo build --release

# 运行可执行文件
./target/release/ringest
```

## 使用方法

```bash
# 使用本地目录的基本用法
ringest -s src

# 克隆并处理远程仓库
ringest -a https://github.com/username/repo.git

# 仅包含特定文件类型
ringest -a . -i "*.rs" "*.toml"

# 排除特定文件或目录
ringest -a . -e "target/*" "*.json"

# 输出到特定文件
ringest -a . -o code_analysis.txt

# 将输出复制到剪贴板
ringest -a . -c

# 组合多个选项
ringest -a https://github.com/username/repo.git -i "*.rs" -e "tests/*" -o analysis.txt
```

### 命令行参数

| 参数 | 长格式 | 描述 | 默认值 |
|----------|-----------|-------------|---------|
| `-a` | `--address` | 仓库 URL 或本地目录路径 | `.`（当前目录） |
| `-s` | `--src` | 仓库内的源代码目录 | `src` |
| `-c` | `--copy` | 将输出复制到剪贴板 | `false` |
| `-o` | `--output` | 输出文件路径 | `./llms.txt` |
| `-i` | `--include` | 包含的模式（逗号分隔） | 无（包含所有） |
| `-e` | `--exclude` | 排除的模式（逗号分隔） | 无（不排除任何内容） |

## 项目架构

Ringest 由多个协同工作的模块组成，用于处理和准备代码：

- **main.rs**：入口点和 CLI 参数处理
- **git.rs**：仓库克隆和 URL 验证
- **file.rs**：文件遍历、过滤和内容处理
- **llm.rs**：LLM 客户端接口（用于未来集成）
- **prompts.rs**：预定义的分析提示

工具遵循以下工作流程：
1. 解析命令行参数
2. 克隆仓库（如果是远程）或使用本地目录
3. 遍历文件应用包含/排除过滤器
4. 使用适当的格式处理每个文件
5. 使用文件标记连接所有内容
6. 将结果输出到文件或剪贴板

## 依赖

- `reqwest`：用于 API 请求的 HTTP 客户端
- `tokio`：异步运行时
- `serde`/`serde_json`：JSON 序列化/反序列化
- `anyhow`：错误处理
- `clap`：命令行参数解析
- `git2`：Git 仓库操作
- `async-trait`：异步 trait 支持
- `tempfile`：临时文件/目录处理
- `arboard`：剪贴板访问
- `ignore`：文件模式匹配和 .gitignore 支持

## 许可证

MIT

## 贡献

欢迎贡献！请随时提交 Pull Request。
