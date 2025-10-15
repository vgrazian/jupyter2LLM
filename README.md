# jupyter2llm

A Rust CLI tool that converts Jupyter notebooks to LLM-optimized text format.

## Features

- **Cell Type Detection**: Automatically detects markdown, code, and raw cells
- **Cell-by-Cell Extraction**: Processes each cell individually with clear separation
- **LLM-Optimized Output**: Formats content for easy ingestion by language models
- **Flexible Output**: Can include or exclude cell outputs and metadata
- **Robust Error Handling**: Comprehensive error handling for malformed notebooks
- **LLM-Ready Text Files**: Creates formatted text files perfect for copying into LLM interfaces

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

### Create LLM-Ready Text File

```bash
jupyter2llm notebook.ipynb -o llm_input.txt --include-outputs --include-metadata
```

### All Options

```bash
jupyter2llm notebook.ipynb -o output.txt --include-outputs --include-metadata
```

### Copy to Clipboard (macOS)

You can also pipe the output directly to the clipboard on macOS:

```bash
jupyter2llm notebook.ipynb --include-outputs --include-metadata | pbcopy
```

## Output Format

The tool converts notebooks to a structured text format perfect for LLM input:

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

## LLM Integration

### Using the Generated Text File with LLMs

1. **Create the LLM-ready file**:
   ```bash
   jupyter2llm my_notebook.ipynb -o llm_context.txt --include-outputs --include-metadata
   ```

2. **Copy and paste into your LLM interface**:
   - Open the generated `llm_context.txt` file
   - Copy the entire content
   - Paste it into your preferred LLM (ChatGPT, Claude, Gemini, etc.)
   - Add your prompt at the end

### Example LLM Prompts

After pasting the notebook content, here are practical prompts you can use:

#### #1 - Notebook Analysis & Explanation

```
[Pasted notebook content here]

Based on this Jupyter notebook, please:

1. Explain the main purpose and functionality of this notebook
2. Summarize what each major section does
3. Identify the key libraries and techniques used
4. Describe the expected outputs or results
```

#### #2 - Debugging Help

```
[Pasted notebook content here]

I'm getting an error in Cell #4. The error message is:
"NameError: name 'df' is not defined"

Please:
1. Analyze the notebook and identify why this error is occurring
2. Suggest specific fixes for Cell #4
3. Explain what changes need to be made in previous cells to prevent this error
4. Provide corrected code for the problematic cell
```

#### #3 - Code Improvement & Optimization

```
[Pasted notebook content here]

Please review this notebook and suggest improvements:

1. Identify any inefficient code or potential bottlenecks
2. Suggest better Python practices or more efficient libraries
3. Recommend ways to make the code more readable and maintainable
4. Point out any potential bugs or edge cases not handled
```

#### #4 - Educational Explanation

```
[Pasted notebook content here]

I'm learning data science and want to understand this notebook better:

1. Explain the data science concepts used in this notebook
2. Break down complex code sections into simpler terms
3. What prerequisites should I know to understand this notebook?
4. Suggest related topics I should study next
```

#### #5 - Error Analysis with Traceback

```
[Pasted notebook content here]

I'm getting this error traceback in Cell #7:

```

ValueError: could not convert string to float: 'N/A'
  File "<ipython-input-7-abc123>", line 5, in <module>
    data = pd.to_numeric(data['column'])

```

Please:
1. Explain what causes this specific error
2. Show how to fix the data preprocessing in earlier cells
3. Provide robust error handling for this scenario
4. Suggest data validation steps to prevent similar issues
```

#### #6 - Extension & Enhancement

```
[Pasted notebook content here]

This notebook analyzes sales data. Please suggest:

1. Additional analyses that could provide more insights
2. Visualizations that would complement the existing ones
3. How to extend this analysis to handle more data sources
4. Potential machine learning models that could be applied
```

#### #7 - Code Translation

```
[Pasted notebook content here]

Please convert this Python notebook to:

1. Equivalent R code for data analysis sections
2. SQL queries for the data manipulation steps
3. PySpark code for large-scale data processing
4. JavaScript equivalent for the visualization parts
```

### Pro Tips for Better LLM Results

1. **Include outputs**: Use `--include-outputs` so the LLM can see actual results and errors
2. **Include metadata**: Use `--include-metadata` for context about the environment
3. **Be specific**: Reference cell numbers in your prompts (e.g., "Cell #5")
4. **Provide error details**: Include exact error messages and tracebacks
5. **State your goal**: Clearly explain what you want to achieve with the notebook

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

# Create an LLM-ready test file
cargo run -- tests/test_data/sample_notebook.ipynb -o test_llm.txt --include-outputs --include-metadata
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
- **LLM Integration Ready**: Creates text files specifically formatted for easy copying into LLM interfaces

The tool provides a clean, structured text output that's perfect for feeding into LLMs while preserving the notebook's structure and content.

## License

MIT License
