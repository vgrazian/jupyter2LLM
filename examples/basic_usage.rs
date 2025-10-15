// examples/basic_usage.rs
use jupyter2llm::JupyterConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example notebook content - using a function to avoid raw string issues
    let notebook_content = create_example_notebook();

    println!("Converting notebook to LLM-optimized text...\n");

    // Demonstrate different conversion options
    println!("=== Basic Conversion ===");
    let converter_basic = JupyterConverter::new();
    let result_basic = converter_basic.convert_str(&notebook_content)?;
    println!("{}", result_basic);

    println!("\n=== LLM-Ready Conversion (with outputs and metadata) ===");
    let converter_llm = JupyterConverter::new()
        .with_outputs(true)
        .with_metadata(true);
    let result_llm = converter_llm.convert_str(&notebook_content)?;
    println!("{}", result_llm);

    Ok(())
}

fn create_example_notebook() -> String {
    r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": [
    "# Example Notebook",
    "",
    "This is a simple example."
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
      "Hello from example!"
     ]
    }
   ],
   "source": [
    "print(\"Hello from example!\")"
   ]
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "name": "python3"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 4
}"##
    .to_string()
}
