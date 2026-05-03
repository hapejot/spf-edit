mod json_model;
mod parser;
mod sections;
mod categorizer;
mod emitter;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser as ClapParser;
use glob::glob;

use json_model::*;
use parser::*;
use sections::attr::parse_attr;
use sections::body::parse_body;
use sections::model::parse_model;
use sections::init::{parse_init, parse_reinit};
use sections::proc_section::parse_proc;
use categorizer::categorize;
use emitter::*;

#[derive(ClapParser, Debug)]
#[command(name = "pan2json", about = "Convert SPFPC .PAN panel files to JSON")]
struct Cli {
    /// Input directory containing .PAN files
    #[arg(short, long, default_value = "SPFPC")]
    input: PathBuf,

    /// Output directory for JSON files
    #[arg(short, long, default_value = "panels")]
    output: PathBuf,

    /// Only convert files matching this glob pattern (e.g., "S2D*")
    #[arg(short, long)]
    filter: Option<String>,

    /// Print detailed parse information
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let input_dir = &cli.input;
    if !input_dir.is_dir() {
        eprintln!("Error: Input directory '{}' not found", input_dir.display());
        std::process::exit(1);
    }

    // Create output directory
    if let Err(e) = fs::create_dir_all(&cli.output) {
        eprintln!("Error creating output directory: {}", e);
        std::process::exit(1);
    }

    // Collect PAN files
    let pattern = input_dir.join(
        cli.filter.as_deref().unwrap_or("*.PAN")
    );
    let pattern_str = pattern.to_string_lossy().to_string();

    let mut pan_files: Vec<PathBuf> = Vec::new();
    match glob(&pattern_str) {
        Ok(paths) => {
            for entry in paths.flatten() {
                pan_files.push(entry);
            }
        }
        Err(e) => {
            eprintln!("Error reading glob pattern: {}", e);
            std::process::exit(1);
        }
    }

    // Also try lowercase .pan extension
    let pattern_lower = input_dir.join(
        cli.filter.as_deref().unwrap_or("*.pan")
    );
    if let Ok(paths) = glob(&pattern_lower.to_string_lossy()) {
        for entry in paths.flatten() {
            if !pan_files.contains(&entry) {
                pan_files.push(entry);
            }
        }
    }

    pan_files.sort();

    if pan_files.is_empty() {
        eprintln!("No .PAN files found in '{}'", input_dir.display());
        std::process::exit(1);
    }

    println!("Found {} PAN files in {}", pan_files.len(), input_dir.display());

    let mut stats = ConversionStats::new();
    let mut manifest = Manifest::new();

    for path in &pan_files {
        stats.total_files += 1;
        let filename = path.file_name().unwrap().to_string_lossy().to_string();

        match convert_pan_file(path, &cli.output, cli.verbose) {
            Ok(panel) => {
                let has_warnings = !panel.metadata.parse_warnings.is_empty();
                let category = panel.metadata.category.clone();

                manifest.add_panel(&panel);
                *stats.by_category.entry(category).or_insert(0) += 1;

                if has_warnings {
                    stats.with_warnings += 1;
                    if cli.verbose {
                        for w in &panel.metadata.parse_warnings {
                            eprintln!("  WARN [{}]: {}", filename, w);
                        }
                    }
                }
                stats.successful += 1;
            }
            Err(e) => {
                stats.failed += 1;
                eprintln!("FAIL [{}]: {}", filename, e);
            }
        }
    }

    // Write manifest
    if let Err(e) = manifest.write(&cli.output) {
        eprintln!("Error writing manifest: {}", e);
    }

    stats.report();
}

/// Convert a single PAN file to JSON and write to output.
fn convert_pan_file(path: &Path, output_dir: &Path, verbose: bool) -> Result<Panel, String> {
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    let id = filename.trim_end_matches(".PAN").trim_end_matches(".pan").to_string();

    if verbose {
        println!("Processing: {}", filename);
    }

    // Read and decode file
    let bytes = fs::read(path).map_err(|e| format!("Read error: {}", e))?;
    let text = decode_pan_bytes(&bytes);

    // Split into sections
    let sections = split_sections(&text);

    // Parse attributes
    let (attrs, mut all_warnings) = if let Some(ref attr_text) = sections.attr {
        parse_attr(attr_text)
    } else {
        (HashMap::new(), Vec::new())
    };

    // Parse body
    let (title, body_rows, body_warnings) = if let Some(ref body_text) = sections.body {
        parse_body(body_text, &attrs)
    } else {
        (None, Vec::new(), vec!["No BODY section found".to_string()])
    };
    all_warnings.extend(body_warnings);

    // Parse model
    let (model, model_warnings) = if let Some(ref model_text) = sections.model {
        parse_model(model_text, &attrs)
    } else {
        (None, Vec::new())
    };
    all_warnings.extend(model_warnings);

    // Parse init
    let (init, init_warnings) = if let Some(ref init_text) = sections.init {
        parse_init(init_text)
    } else {
        (None, Vec::new())
    };
    all_warnings.extend(init_warnings);

    // Parse reinit
    let (reinit, reinit_warnings) = if let Some(ref reinit_text) = sections.reinit {
        parse_reinit(reinit_text)
    } else {
        (None, Vec::new())
    };
    all_warnings.extend(reinit_warnings);

    // Parse proc
    let (proc_section, proc_warnings) = if let Some(ref proc_text) = sections.proc_section {
        parse_proc(proc_text)
    } else {
        (None, Vec::new())
    };
    all_warnings.extend(proc_warnings);

    // Categorize
    let (panel_type, category) = categorize(&filename, &sections);

    // Extract copyright
    let copyright = extract_copyright(&sections.trailing);

    let panel = Panel {
        id: id.clone(),
        title,
        panel_type,
        attributes: attrs,
        body: Body { rows: body_rows },
        model,
        init,
        reinit,
        proc_section,
        pfkeys: std::collections::HashMap::new(),
        metadata: Metadata {
            source_file: filename.clone(),
            category: category.to_string(),
            copyright,
            parse_warnings: all_warnings,
        },
    };

    // Write JSON
    write_panel(&panel, output_dir)
        .map_err(|e| format!("Write error: {}", e))?;

    Ok(panel)
}
