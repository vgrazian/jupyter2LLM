#[cfg(test)]
mod integration_tests {
    use jupyter2llm::JupyterConverter; // Add this import
    use std::fs;
    use tempfile::NamedTempFile;

    // Use a function to create the notebook JSON to avoid raw string issues
    fn create_sample_notebook() -> String {
        let notebook_json = r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": [
    "# Test Notebook"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 1,
   "metadata": {},
   "outputs": [
    {
     "name": "stdout",
     "output_type": "stream",
     "text": [
      "Hello from test"
     ]
    }
   ],
   "source": [
    "print(\"Hello from test\")"
   ]
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "language": "python",
   "name": "python3"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 4
}"##;
        notebook_json.to_string()
    }

    fn create_mixed_notebook() -> String {
        r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": ["# Header"]
  },
  {
   "cell_type": "code",
   "execution_count": 1,
   "metadata": {},
   "outputs": [],
   "source": ["x = 1"]
  },
  {
   "cell_type": "raw",
   "metadata": {},
   "source": ["Raw content"]
  }
 ],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 4
}"##
        .to_string()
    }

    fn create_empty_notebook() -> String {
        r##"{
 "cells": [],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 4
}"##
        .to_string()
    }

    fn create_invalid_cell_notebook() -> String {
        r##"{
 "cells": [
  {
   "cell_type": "invalid_type",
   "metadata": {},
   "source": ["test"]
  }
 ],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 4
}"##
        .to_string()
    }

    #[test]
    fn test_basic_conversion() {
        let converter = JupyterConverter::new();
        let result = converter.convert_str(&create_sample_notebook()).unwrap();

        assert!(result.contains("Test Notebook"));
        assert!(result.contains("print(\"Hello from test\")"));
        assert!(result.contains("Cell 1: Markdown"));
        assert!(result.contains("Cell 2: Code"));
    }

    #[test]
    fn test_conversion_with_outputs() {
        let converter = JupyterConverter::new().with_outputs(true);
        let result = converter.convert_str(&create_sample_notebook()).unwrap();

        assert!(result.contains("Hello from test"));
        assert!(result.contains("Outputs"));
    }

    #[test]
    fn test_conversion_with_metadata() {
        let converter = JupyterConverter::new().with_metadata(true);
        let result = converter.convert_str(&create_sample_notebook()).unwrap();

        assert!(result.contains("Jupyter Notebook"));
        assert!(result.contains("Kernel"));
        assert!(result.contains("Python 3"));
    }

    #[test]
    fn test_file_conversion() {
        let notebook_content = create_sample_notebook();
        let temp_file = NamedTempFile::new().unwrap(); // Removed mut
        fs::write(temp_file.path(), &notebook_content).unwrap();

        let converter = JupyterConverter::new();
        let result = converter.convert_file(temp_file.path()).unwrap();

        assert!(result.contains("Test Notebook"));
        assert!(result.contains("Cell 1: Markdown"));
    }

    #[test]
    fn test_invalid_json() {
        let converter = JupyterConverter::new();
        let result = converter.convert_str("invalid json");

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_cell_type() {
        let converter = JupyterConverter::new();
        let result = converter.convert_str(&create_invalid_cell_notebook());

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_notebook() {
        let converter = JupyterConverter::new();
        let result = converter.convert_str(&create_empty_notebook()).unwrap();

        assert!(result.contains("Total Cells: 0"));
    }

    #[test]
    fn test_mixed_cell_types() {
        let converter = JupyterConverter::new();
        let result = converter.convert_str(&create_mixed_notebook()).unwrap();

        assert!(result.contains("Header"));
        assert!(result.contains("x = 1"));
        assert!(result.contains("Raw content"));
        assert!(result.contains("Markdown"));
        assert!(result.contains("Code"));
        assert!(result.contains("Raw"));
    }

    #[test]
    fn test_real_file_conversion() {
        let converter = JupyterConverter::new();

        // This test will only run if the test data file exists
        if std::path::Path::new("tests/test_data/sample_notebook.ipynb").exists() {
            let result = converter
                .convert_file("tests/test_data/sample_notebook.ipynb")
                .unwrap();

            assert!(result.contains("Sample Jupyter Notebook"));
            assert!(result.contains("print(\"Hello, World!\")"));
            assert!(result.contains("calculate_answer()"));
            assert!(result.contains("Raw"));
        } else {
            println!("Test data file not found, skipping file-based test");
        }
    }

    #[test]
    fn test_with_include_str() {
        // Load notebook from external file to avoid raw string issues
        let notebook_content = include_str!("test_data/simple_notebook.json");

        let converter = JupyterConverter::new();
        let result = converter.convert_str(notebook_content).unwrap();

        assert!(result.contains("Simple Test Notebook"));
        assert!(result.contains("print(\"test\")"));
        assert!(result.contains("Cell 1: Markdown"));
        assert!(result.contains("Cell 2: Code"));
    }
}
