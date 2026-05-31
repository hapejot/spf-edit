#[path = "../rust_analyzer.rs"]
mod rust_analyzer;

use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use rust_analyzer::{CompletionItem, LspEvent, RustAnalyzerClient};
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
    let project_ready = client.wait_for_project_ready(Duration::from_secs(60))?;
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

fn remove_flag(args: &mut Vec<String>, flag: &str) -> bool {
    if let Some(idx) = args.iter().position(|arg| arg == flag) {
        args.remove(idx);
        true
    } else {
        false
    }
}

enum RequestKind {
    Completion,
    Hover,
}

impl RequestKind {
    fn name(&self) -> &'static str {
        match self {
            RequestKind::Completion => "completion",
            RequestKind::Hover => "hover",
        }
    }
}

enum LocationKind {
    Prefix {
        occurrence: usize,
        prefix: String,
        byte_index: usize,
        line: usize,
        character_utf16: usize,
    },
    LineCol {
        line: usize,
        character_utf16: usize,
        byte_index: usize,
        prefix: String,
    },
}

impl LocationKind {
    fn mode_name(&self) -> &'static str {
        match self {
            LocationKind::Prefix { .. } => "prefix",
            LocationKind::LineCol { .. } => "line-col",
        }
    }
}

fn find_location(text: &str, prefix: &str, occurrence: usize) -> Option<(usize, usize, usize)> {
    let mut search_from = 0usize;
    let mut found = None;

    for _ in 0..occurrence {
        let relative = text[search_from..].find(prefix)?;
        let absolute = search_from + relative;
        found = Some(absolute);
        search_from = absolute + prefix.len();
    }

    let start = found?;
    let cursor_byte = start + prefix.len();
    let before_cursor = &text[..cursor_byte];

    let line = before_cursor.chars().filter(|&ch| ch == '\n').count();
    let line_start = before_cursor.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    let line_slice = &text[line_start..cursor_byte];
    let character_utf16 = line_slice.encode_utf16().count();

    Some((start, line, character_utf16))
}

fn parse_line_col(spec: &str) -> io::Result<(usize, usize)> {
    let Some((line_str, col_str)) = spec.split_once(':') else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "line:column must use the form <line:column>",
        ));
    };
    let line = line_str.parse::<usize>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "line must be a non-negative integer")
    })?;
    let col = col_str.parse::<usize>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "column must be a non-negative integer")
    })?;
    Ok((line, col))
}

fn byte_index_from_line_col(text: &str, line: usize, character_utf16: usize) -> io::Result<usize> {
    let mut current_line = 0usize;
    let mut line_start = 0usize;
    for (idx, ch) in text.char_indices() {
        if current_line == line {
            break;
        }
        if ch == '\n' {
            current_line += 1;
            line_start = idx + ch.len_utf8();
        }
    }
    if current_line != line {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("line out of range: {line}"),
        ));
    }

    let line_end = text[line_start..]
        .find('\n')
        .map(|offset| line_start + offset)
        .unwrap_or(text.len());
    let line_slice = &text[line_start..line_end];

    let mut utf16_seen = 0usize;
    for (offset, ch) in line_slice.char_indices() {
        if utf16_seen == character_utf16 {
            return Ok(line_start + offset);
        }
        let next = utf16_seen + ch.len_utf16();
        if next > character_utf16 {
            return Ok(line_start + offset + ch.len_utf8());
        }
        utf16_seen = next;
    }

    if utf16_seen == character_utf16 {
        Ok(line_end)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("column out of range for line {line}: {character_utf16}"),
        ))
    }
}

fn prefix_before_byte(text: &str, byte_index: usize, max_chars: usize) -> String {
    let before = &text[..byte_index.min(text.len())];
    let line_start = before.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    let line_prefix = &before[line_start..];
    trailing_context(line_prefix, max_chars)
}

fn completion_token_from_prefix(prefix: &str) -> String {
    let mut token_chars = Vec::new();
    for ch in prefix.chars().rev() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            token_chars.push(ch);
        } else {
            break;
        }
    }
    token_chars.into_iter().rev().collect()
}

fn completion_trigger_from_prefix(prefix: &str) -> Option<char> {
    prefix
        .chars()
        .last()
        .filter(|ch| matches!(ch, '.' | ':' | '(' | '\''))
}

fn trailing_context(prefix: &str, max_chars: usize) -> String {
    let char_count = prefix.chars().count();
    let start = char_count.saturating_sub(max_chars);
    prefix.chars().skip(start).collect()
}

fn wait_for_completion(
    client: &RustAnalyzerClient,
    request_id: u64,
    timeout: Duration,
) -> io::Result<Vec<CompletionItem>> {
    loop {
        match client.recv_timeout(timeout) {
            Ok(LspEvent::CompletionResponse {
                request_id: response_id,
                items,
            }) if response_id == request_id => return Ok(items),
            Ok(LspEvent::CompletionResponse { .. })
            | Ok(LspEvent::HoverResponse { .. })
            | Ok(LspEvent::InitializeComplete) => continue,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "timed out waiting for completion response",
                ))
            }
        }
    }
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

fn rank_items(items: Vec<CompletionItem>, token: &str) -> Vec<CompletionItem> {
    if token.is_empty() {
        return items;
    }

    let token_lc = token.to_ascii_lowercase();
    let mut starts_with = Vec::new();
    let mut contains = Vec::new();
    let mut rest = Vec::new();

    for item in items {
        let label_lc = item.label.to_ascii_lowercase();
        if label_lc.starts_with(&token_lc) {
            starts_with.push(item);
        } else if label_lc.contains(&token_lc) {
            contains.push(item);
        } else {
            rest.push(item);
        }
    }

    starts_with.extend(contains);
    starts_with.extend(rest);
    starts_with
}
