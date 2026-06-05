use std::io;
use std::path::Path;
use clap::Parser;
use tracing::info;
use spf_edit::buffer::*;
use spf_edit::types::*;

/// Structure to hold command line arguments
#[derive(Debug, Parser)]
struct Args {
    /// File path to read from
    file: String,
}

/// Main entry point of the application
fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    // Log start of buffer demo
    info!("starting buffer demo");

    // Create a new path object from file path
    let path = Path::new(&args.file);

    // Open the file as a mutable FileBuffer
    let mut b = FileBuffer::open(&path, RecordFormat::Variable, false).unwrap();

    // Set labels for lines in the buffer
    b.set_label(".TEST".into(), 2);
    b.set_label(".A".into(), 20);
    b.set_label(".B".into(), 25);

    // Exclude a range of characters from the buffer
    b.exclude(10..12);

    // Get and print lines in the buffer
    for l in b.get_lines(5..39) {
        println!("{} {}", String::from_iter(l.prefix.iter()), String::from_iter(l.display.iter()));
    }

    // Log exit message
    info!("exit");

    Ok(())
}