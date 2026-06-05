#[path = "../src/rust_analyzer.rs"]
mod rust_analyzer;

use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use rust_analyzer::{LspEvent, RustAnalyzerClient};
use clap::Parser;
use tracing::info;


#[derive(Debug,Parser)]
struct Args  {
    file: String,
    line: u32,
    pos: u32,

    #[arg(long)]
    trace: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    info!("starting rust analyzer");
    let text = fs::read_to_string(&args.file)?;

    let path = Path::new(&args.file);
    let mut client = RustAnalyzerClient::start_with_trace(&path, args.trace)?;
    client.did_open(&text)?;
    let _project_ready = client.wait_for_project_ready(Duration::from_secs(60))?;
    info!("project ready");
    let line = args.line;
    let character_utf16 = args.pos;
    let request_id = client.request_hover(line as u32, character_utf16 as u32)?;
    let hover = wait_for_hover(&client, request_id, Duration::from_secs(60))?;
    info!("hover returned");
    match hover {
        Some(text) => println!("{}", text),
            None => println!("<no hover information>"),
    }
    info!("exit");
    Ok(())
}

fn wait_for_hover(
    client: &RustAnalyzerClient,
    request_id: u64,
    timeout: Duration,
) -> io::Result<Option<String>> {
    loop {
        match client.recv_timeout(timeout) {
            Ok(LspEvent::HoverResponse {
                request_id: response_id,
                contents,
            }) if response_id == request_id => return Ok(contents),
            Ok(LspEvent::CompletionResponse { .. })
            | Ok(LspEvent::HoverResponse { .. })
            | Ok(LspEvent::InitializeComplete) => continue,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "timed out waiting for hover response",
                ))
            }
        }
    }
}