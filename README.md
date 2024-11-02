# Snapshot Context (SnapCtx)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust-based CLI tool that generates comprehensive, single-file summaries of source code projects, optimized for use as context with Large Language Models (LLMs) like Claude or ChatGPT.

## Features

- 🔍 **Automatic Project Detection**: Identifies project types (Rust, JavaScript, Python) based on key markers
- 📑 **Smart Summary Generation**: Creates concise yet informative project overviews
- 🌳 **Project Tree View**: Provides a visual representation of your project structure
- 💬 **Interactive Mode**: Append custom LLM prompts to your summary
- ⚡ **Batch Processing**: Quick, non-interactive summary generation

## Installation

```bash
cargo install snapctx
```

## Usage

Basic usage:

```bash
scx /path/to/project
```

### Command Line Options

```bash
# Run in interactive mode (default)
scx /path/to/project

# Run in batch mode (no prompts)
scx --batch /path/to/project
```

### Output

SnapCtx generates a Markdown file containing:
- Project metadata (name, timestamp, type)
- Directory structure
- Key file contents
- Git status information (when available)
- Your custom LLM prompt (in interactive mode)

Output files are saved with the format: `{project_name}_snapshot_{timestamp}.md`

## Project Detection

SnapCtx automatically detects project types based on common markers:
- **Rust**: `Cargo.toml`
- **JavaScript**: `package.json`
- **Python**: `requirements.txt` or `setup.py`

Unknown project types are still processed with default settings.

## File Filtering

By default, SnapCtx ignores:
- Hidden files and directories (starting with `.`)
- Build directories (`target`, `node_modules`, `__pycache__`)
- Compiled Python files (`.pyc`)

## Use Cases

- 🤖 Preparing context for LLM coding sessions
- 📚 Quick project documentation generation
- 🔄 Code review preparation

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building from Source

```bash
git clone https://github.com/serkanaltuntas/snapctx.git
cd snapctx
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Dependencies

- `clap`: Command-line argument parsing
- `walkdir`: Directory traversal
- `chrono`: Timestamp generation
- `anyhow` & `thiserror`: Error handling
- `serde` & `serde_yaml`: Data serialization
- `env_logger`: Logging functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Serkan Altuntas - [serkan@serkan.ai](mailto:serkan@serkan.ai)

## Acknowledgments

- Inspired by the need for better LLM context generation
- Built with Rust for performance and reliability