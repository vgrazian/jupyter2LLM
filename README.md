# jupyter2llm

A Rust CLI tool that converts Jupyter notebooks to LLM-optimized text format.

## Features

- **Cell Type Detection**: Automatically detects markdown, code, and raw cells
- **Cell-by-Cell Extraction**: Processes each cell individually with clear separation
- **LLM-Optimized Output**: Formats content for easy ingestion by language models
- **Flexible Output**: Can include or exclude cell outputs and metadata
- **Robust Error Handling**: Comprehensive error handling for malformed notebooks

## Installation

```bash
git clone https://github.com/your-username/jupyter2llm
cd jupyter2llm
cargo install --path .
```

##  Usage

Basic Conversion

```bash
jupyter2llm notebook.ipynb
```

## Save to File

```bash
jupyter2llm notebook.ipynb -o output.txt
```

## Include Cell Outputs

```bash
jupyter2llm notebook.ipynb --include-outputs
```

## Include Notebook Metadata

```bash
jupyter2llm notebook.ipynb --include-metadata
```

## All Options

``` bash
jupyter2llm notebook.ipynb -o output.txt --include-outputs --include-metadata
```

## Output Format

The tool converts notebooks to a structured text format:

``` text
# Jupyter Notebook
**Kernel**: python3
**Language**: python
**Format**: nbformat 4.4
**Total Cells**: 5
```

## Cell 1: Markdown

```markdown
# Sample Notebook
This is a sample notebook.
Cell 2: Code
Execution Count: 1

python
print("Hello, World!")
Outputs
Stream Output:

text
Hello, World!
Cell 3: Raw
text
Raw content goes here.
text
```

## Testing

Run the test suite:

```bash
cargo test
The test suite includes a sample notebook and comprehensive integration tests.

License
MIT License

text

## Building and Testing

1. **Build the project**:
```bash
cargo build --release

## Run tests:

```bash
cargo test
```

## Test with the sample notebook

```bash
cargo run -- tests/test_data/sample_notebook.ipynb
```

## Test with outputs included

```bash
cargo run -- tests/test_data/sample_notebook.ipynb --include-outputs
```

##  Key Features
Robust Error Handling: Comprehensive error types for file I/O, JSON parsing, and invalid notebook formats

Flexible Configuration: Options to include/exclude outputs and metadata

LLM-Optimized Formatting: Clear section headers, code blocks, and structured output

Comprehensive Testing: Unit tests and integration tests with sample data

Command-line Interface: User-friendly CLI with helpful error messages

The tool provides a clean, structured text output that's perfect for feeding into LLMs while preserving the notebook's structure and content.
