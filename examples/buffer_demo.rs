use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use spf_edit::line::LineType;
use tracing::info;

// #[path = "../src/buffer.rs"]
// mod buffer;
// #[path = "../src/file_io.rs"]
// mod file_io;
// #[path = "../src/line.rs"]
// mod line;
// #[path = "../src/line_store.rs"]
// mod line_store;
// #[path = "../src/prefix.rs"]
// mod prefix;
// #[path = "../src/types.rs"]
// mod types;

use spf_edit::buffer::*;
use spf_edit::types::*;

use spf_edit::line::Line;
use spf_edit::line_store::VecLineStore;

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn print_lines(lines: &VecLineStore) {
    for l in lines.iter() {
        print_line(l);
    }
}

fn print_line(l: &Line) {
    let prefix = if let Some(ll) = &l.label {
        format!("{:<6} ", ll)
    } else {
        format!("{:06} ", l.current_number)
    };
    match l.line_type {        
        LineType::Data => println!("{}{}", prefix, String::from_iter(l.data.iter())),
        LineType::TopOfData => println!("--- Top of Data ---"),
        LineType::BottomOfData => println!("--- Bottom of Data ---"),
        LineType::ColsRuler => println!("--- Columns Ruler ---"),
        LineType::Message => println!("--- Message ---"),
        LineType::Insert => println!("--- Insert ---"),
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    info!("starting buffer demo");

    let path = Path::new(&args.file);

    let mut b = FileBuffer::open(&path, RecordFormat::Variable, false).unwrap();

    b.set_label(".TEST".into(), 2);
    print_lines(&b.lines);
    info!("exit");
    Ok(())
}
