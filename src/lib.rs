use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupyterError {
    #[error("Failed to read notebook file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Invalid notebook format: {0}")]
    InvalidNotebookFormat(String),

    #[error("Notebook cell has invalid type: {0}")]
    InvalidCellType(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub cells: Vec<Cell>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub nbformat: u32,
    pub nbformat_minor: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: String,
    pub source: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub outputs: Option<Vec<Output>>,
    pub execution_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub output_type: String,
    pub text: Option<Vec<String>>,
    pub data: Option<HashMap<String, serde_json::Value>>,
    pub execution_count: Option<u32>,
}

pub struct JupyterConverter {
    include_outputs: bool,
    include_metadata: bool,
}

impl Default for JupyterConverter {
    fn default() -> Self {
        Self {
            include_outputs: false,
            include_metadata: false,
        }
    }
}

impl JupyterConverter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_outputs(mut self, include: bool) -> Self {
        self.include_outputs = include;
        self
    }

    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    pub fn convert_file<P: AsRef<Path>>(&self, path: P) -> Result<String, JupyterError> {
        let content = std::fs::read_to_string(path)?;
        self.convert_str(&content)
    }

    pub fn convert_str(&self, content: &str) -> Result<String, JupyterError> {
        let notebook: Notebook = serde_json::from_str(content)?;
        self.convert_notebook(&notebook)
    }

    pub fn convert_notebook(&self, notebook: &Notebook) -> Result<String, JupyterError> {
        let mut output = String::new();

        // Add notebook metadata if requested
        if self.include_metadata {
            output.push_str(&self.format_metadata(notebook));
            output.push_str("\n\n");
        }

        // Process each cell
        for (index, cell) in notebook.cells.iter().enumerate() {
            output.push_str(&self.format_cell(cell, index + 1)?);
            output.push_str("\n\n");
        }

        Ok(output)
    }

    fn format_metadata(&self, notebook: &Notebook) -> String {
        let mut metadata_str = String::from("# Jupyter Notebook\n\n");

        if let Some(kernelspec) = notebook.metadata.get("kernelspec") {
            if let Some(name) = kernelspec.get("name").and_then(|v| v.as_str()) {
                metadata_str.push_str(&format!("**Kernel**: {}\n", name));
            }
            if let Some(display_name) = kernelspec.get("display_name").and_then(|v| v.as_str()) {
                metadata_str.push_str(&format!("**Display Name**: {}\n", display_name));
            }
        }

        if let Some(language_info) = notebook.metadata.get("language_info") {
            if let Some(name) = language_info.get("name").and_then(|v| v.as_str()) {
                metadata_str.push_str(&format!("**Language**: {}\n", name));
            }
            if let Some(version) = language_info.get("version").and_then(|v| v.as_str()) {
                metadata_str.push_str(&format!("**Version**: {}\n", version));
            }
        }

        metadata_str.push_str(&format!(
            "**Format**: nbformat {}.{}\n",
            notebook.nbformat, notebook.nbformat_minor
        ));
        metadata_str.push_str(&format!("**Total Cells**: {}\n", notebook.cells.len()));

        metadata_str
    }

    fn format_cell(&self, cell: &Cell, cell_number: usize) -> Result<String, JupyterError> {
        let mut cell_str = String::new();

        match cell.cell_type.as_str() {
            "markdown" => {
                cell_str.push_str(&format!("## Cell {}: Markdown\n", cell_number));
                cell_str.push_str("```markdown\n");
                for line in &cell.source {
                    cell_str.push_str(line);
                    cell_str.push('\n');
                }
                cell_str.push_str("```\n");
            }
            "code" => {
                cell_str.push_str(&format!("## Cell {}: Code\n", cell_number));

                // Add execution count if present
                if let Some(exec_count) = cell.execution_count {
                    cell_str.push_str(&format!("*Execution Count: {}*\n", exec_count));
                }

                // Add source code
                cell_str.push_str("```python\n");
                for line in &cell.source {
                    cell_str.push_str(line);
                    cell_str.push('\n');
                }
                cell_str.push_str("```\n");

                // Add outputs if requested and present
                if self.include_outputs {
                    if let Some(outputs) = &cell.outputs {
                        if !outputs.is_empty() {
                            cell_str.push_str("### Outputs\n");
                            for output in outputs {
                                cell_str.push_str(&self.format_output(output));
                            }
                        }
                    }
                }
            }
            "raw" => {
                cell_str.push_str(&format!("## Cell {}: Raw\n", cell_number));
                cell_str.push_str("```\n");
                for line in &cell.source {
                    cell_str.push_str(line);
                    cell_str.push('\n');
                }
                cell_str.push_str("```\n");
            }
            _ => {
                return Err(JupyterError::InvalidCellType(cell.cell_type.clone()));
            }
        }

        Ok(cell_str)
    }

    fn format_output(&self, output: &Output) -> String {
        let mut output_str = String::new();

        match output.output_type.as_str() {
            "stream" => {
                if let Some(text) = &output.text {
                    output_str.push_str("**Stream Output**:\n");
                    output_str.push_str("```\n");
                    for line in text {
                        output_str.push_str(line);
                        output_str.push('\n');
                    }
                    output_str.push_str("```\n");
                }
            }
            "execute_result" | "display_data" => {
                if let Some(data) = &output.data {
                    if let Some(text_plain) = data.get("text/plain") {
                        if let Some(text) = text_plain.as_str() {
                            output_str.push_str("**Result**:\n");
                            output_str.push_str("```\n");
                            output_str.push_str(text);
                            output_str.push_str("\n```\n");
                        } else if let Some(text_array) = text_plain.as_array() {
                            output_str.push_str("**Result**:\n");
                            output_str.push_str("```\n");
                            for item in text_array {
                                if let Some(s) = item.as_str() {
                                    output_str.push_str(s);
                                    output_str.push('\n');
                                }
                            }
                            output_str.push_str("```\n");
                        }
                    }
                }
            }
            "error" => {
                if let Some(data) = &output.data {
                    if let Some(traceback) = data.get("traceback") {
                        output_str.push_str("**Error**:\n");
                        output_str.push_str("```\n");
                        if let Some(tb_array) = traceback.as_array() {
                            for item in tb_array {
                                if let Some(s) = item.as_str() {
                                    output_str.push_str(s);
                                    output_str.push('\n');
                                }
                            }
                        }
                        output_str.push_str("```\n");
                    }
                }
            }
            _ => {
                output_str.push_str(&format!("**Output Type: {}**\n", output.output_type));
            }
        }

        output_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_creation() {
        let converter = JupyterConverter::new();
        assert!(!converter.include_outputs);
        assert!(!converter.include_metadata);
    }

    #[test]
    fn test_converter_with_options() {
        let converter = JupyterConverter::new()
            .with_outputs(true)
            .with_metadata(true);
        assert!(converter.include_outputs);
        assert!(converter.include_metadata);
    }
}
