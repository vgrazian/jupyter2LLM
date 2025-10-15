# jupyter2llm

A Rust CLI tool that converts Jupyter notebooks to LLM-optimized text format.

## Features

- **Cell Type Detection**: Automatically detects markdown, code, and raw cells
- **Cell-by-Cell Extraction**: Processes each cell individually with clear separation
- **LLM-Optimized Output**: Formats content for easy ingestion by language models
- **Flexible Output**: Can include or exclude cell outputs and metadata
- **Robust Error Handling**: Comprehensive error handling for malformed notebooks

## Installation

### Prerequisites

#### macOS Dependencies

1. **Install Rust**:

   ```bash
   # Using the official Rust installer (recommended)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

   Or using Homebrew:

   ```bash
   brew install rustup
   rustup-init
   ```

2. **Install Xcode Command Line Tools** (required for Rust compilation):

   ```bash
   xcode-select --install
   ```

   If you encounter issues, you may need to install the full Xcode from the App Store and then install command line tools:

   ```bash
   sudo xcodebuild -license accept
   ```

### Building from Source

```bash
git clone https://github.com/your-username/jupyter2llm
cd jupyter2llm

# Build in release mode (recommended)
cargo build --release

# Install globally
cargo install --path .
```

### Verification

Verify the installation:

```bash
jupyter2llm --help
```

## Usage

### Basic Conversion

```bash
jupyter2llm notebook.ipynb
```

### Save to File

```bash
jupyter2llm notebook.ipynb -o output.txt
```

### Include Cell Outputs

```bash
jupyter2llm notebook.ipynb --include-outputs
```

### Include Notebook Metadata

```bash
jupyter2llm notebook.ipynb --include-metadata
```

### All Options

```bash
jupyter2llm notebook.ipynb -o output.txt --include-outputs --include-metadata
```

## Output Format

The tool converts notebooks to a structured text format:

```text
# Jupyter Notebook
**Kernel**: python3
**Language**: python
**Format**: nbformat 4.4
**Total Cells**: 5

## Cell 1: Markdown
```markdown
# Sample Notebook
This is a sample notebook.
```

## Cell 2: Code

*Execution Count: 1*

```python
print("Hello, World!")
```

### Outputs

**Stream Output**:

```
Hello, World!
```

## Cell 3: Raw

```
Raw content goes here.
```

```

## Building and Testing

### Building the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Testing with Sample Notebooks

```bash
# Test with the sample notebook
cargo run -- tests/test_data/sample_notebook.ipynb

# Test with outputs included
cargo run -- tests/test_data/sample_notebook.ipynb --include-outputs
```

### macOS-Specific Build Notes

- The first build might take longer as Rust downloads and compiles dependencies
- If you encounter linker errors, ensure Xcode Command Line Tools are properly installed
- For M1/M2 Mac users, the build process is fully supported and optimized

## Troubleshooting

### Common macOS Issues

1. **"xcrun: error: invalid active developer path"**

   ```bash
   xcode-select --install
   ```

2. **Linker errors**

   ```bash
   # Reinstall command line tools
   sudo rm -rf /Library/Developer/CommandLineTools
   xcode-select --install
   ```

3. **Permission issues**

   ```bash
   # Fix cargo permissions
   sudo chown -R $(whoami) ~/.cargo
   ```

## Key Features

- **Robust Error Handling**: Comprehensive error types for file I/O, JSON parsing, and invalid notebook formats
- **Flexible Configuration**: Options to include/exclude outputs and metadata
- **LLM-Optimized Formatting**: Clear section headers, code blocks, and structured output
- **Comprehensive Testing**: Unit tests and integration tests with sample data
- **Command-line Interface**: User-friendly CLI with helpful error messages

The tool provides a clean, structured text output that's perfect for feeding into LLMs while preserving the notebook's structure and content.

## License

MIT License
