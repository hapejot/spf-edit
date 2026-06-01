use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use tracing::info;

#[path = "../src/prefix.rs"]
mod prefix;
#[path = "../src/file_io.rs"]
mod file_io;
#[path = "../src/line.rs"]
mod line;
#[path = "../src/line_store.rs"]
mod line_store;
#[path = "../src/types.rs"]
mod types;
#[path = "../src/buffer.rs"]
mod buffer;

use types::*;
use buffer::*;

#[derive(Debug,Parser)]
struct Args  {
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    info!("starting buffer demo");

    let path = Path::new(&args.file);

    let b = FileBuffer::open(&path, RecordFormat::Variable, false).unwrap();

    for l in b.lines.iter() {
        println!("{l:?}");
    }

    info!("exit");
    Ok(())
}
