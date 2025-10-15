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
            Arg::new("llm-ready")
                .short('l')
                .long("llm-ready")
                .help(
                    "Create LLM-ready output (equivalent to --include-outputs --include-metadata)",
                )
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("copy-clipboard")
                .short('c')
                .long("copy-clipboard")
                .help("Copy output to clipboard (macOS only)")
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
    let llm_ready = matches.get_flag("llm-ready");
    let copy_clipboard = matches.get_flag("copy-clipboard");
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

    // Handle llm-ready flag (overrides individual flags)
    let final_include_outputs = llm_ready || include_outputs;
    let final_include_metadata = llm_ready || include_metadata;

    // Create converter with specified options
    let converter = JupyterConverter::new()
        .with_outputs(final_include_outputs)
        .with_metadata(final_include_metadata);

    // Convert the notebook
    let result = converter.convert_file(&input_path)?;

    // Handle clipboard copying (macOS only)
    if copy_clipboard {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let mut cmd = Command::new("pbcopy");
            let mut child = cmd.stdin(std::process::Stdio::piped()).spawn()?;
            {
                let stdin = child.stdin.as_mut().expect("Failed to open stdin");
                use std::io::Write;
                stdin.write_all(result.as_bytes())?;
            }
            child.wait()?;
            if !quiet {
                eprintln!("Output copied to clipboard!");
            }
            return Ok(());
        }
        #[cfg(not(target_os = "macos"))]
        {
            eprintln!("Warning: Clipboard copying is only supported on macOS");
        }
    }

    // Write output
    match output_path {
        Some(output_path) => {
            std::fs::write(output_path, &result)?;
            if !quiet {
                if llm_ready {
                    eprintln!("LLM-ready output written to: {}", output_path);
                } else {
                    eprintln!("Output written to: {}", output_path);
                }
            }
        }
        None => {
            println!("{}", result);
        }
    }

    Ok(())
}
