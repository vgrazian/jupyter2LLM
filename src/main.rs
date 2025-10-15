use clap::{Arg, Command};
use jupyter2llm::JupyterConverter;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("jupyter2llm")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Convert Jupyter notebooks to LLM-optimized text")
        .arg(
            Arg::new("input")
                .help("Path to the Jupyter notebook file (.ipynb)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path (default: stdout)")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("include-outputs")
                .short('O')
                .long("include-outputs")
                .help("Include cell outputs in the conversion")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("include-metadata")
                .short('m')
                .long("include-metadata")
                .help("Include notebook metadata in the conversion")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress informational messages")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output");
    let include_outputs = matches.get_flag("include-outputs");
    let include_metadata = matches.get_flag("include-metadata");
    let quiet = matches.get_flag("quiet");

    // Validate input file
    let input_path = PathBuf::from(input_path);
    if !input_path.exists() {
        eprintln!(
            "Error: Input file '{}' does not exist",
            input_path.display()
        );
        std::process::exit(1);
    }

    if input_path.extension().map_or(true, |ext| ext != "ipynb") {
        eprintln!("Error: Input file must have .ipynb extension");
        std::process::exit(1);
    }

    if !quiet {
        eprintln!("Converting notebook: {}", input_path.display());
    }

    // Create converter with specified options
    let converter = JupyterConverter::new()
        .with_outputs(include_outputs)
        .with_metadata(include_metadata);

    // Convert the notebook
    let result = converter.convert_file(&input_path)?;

    // Write output
    match output_path {
        Some(output_path) => {
            std::fs::write(output_path, &result)?;
            if !quiet {
                eprintln!("Output written to: {}", output_path);
            }
        }
        None => {
            println!("{}", result);
        }
    }

    Ok(())
}
