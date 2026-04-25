//! File I/O: read and write files in text (variable-length) or fixed-record format.
//!
//! On read, tabs are expanded to spaces and sentinel lines (TopOfData / BottomOfData)
//! are prepended / appended.  On write, only `Data` lines are emitted; sentinels are
//! stripped.  Writes are atomic: data goes to a `.spf-edit.tmp` file first, then
//! renamed over the original.
//!
//! ## Known Issues
//!
//! - `write_fixed_file` uses byte-slicing (`data[..lrecl]`) which will panic
//!   on multi-byte UTF-8 chars if `lrecl` falls inside a char.  Should use
//!   char-boundary-aware truncation (same pattern as `screen::truncate_to_width`).
//!   TODO: fix for UTF-8 safety.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use tracing::{info, debug};

use crate::line::Line;
use crate::line_store::VecLineStore;
use crate::types::{LineEnding, RecordFormat, TAB_STOP, LINE_NUMBER_INCREMENT};

/// Read a file and return a VecLineStore with sentinel lines.
pub fn read_file(
    path: &Path,
    record_format: RecordFormat,
) -> io::Result<(VecLineStore, LineEnding)> {
    info!("read_file: {:?} format={:?}", path, record_format);
    let content = fs::read(path)?;
    debug!("  file size: {} bytes", content.len());

    match record_format {
        RecordFormat::Variable => read_text_file(&content),
        RecordFormat::Fixed(lrecl) => read_fixed_file(&content, lrecl),
    }
}

fn read_text_file(content: &[u8]) -> io::Result<(VecLineStore, LineEnding)> {
    let line_ending = detect_line_ending(content);

    let text = String::from_utf8_lossy(content);

    // Split into lines, handling all line ending styles
    let raw_lines: Vec<&str> = match line_ending {
        LineEnding::CrLf => text.split("\r\n").collect(),
        LineEnding::Lf => text.split('\n').collect(),
        LineEnding::Cr => text.split('\r').collect(),
    };

    let mut lines = Vec::with_capacity(raw_lines.len() + 2);

    // TopOfData sentinel
    lines.push(Line::top_of_data());

    // Data lines
    for (i, raw) in raw_lines.iter().enumerate() {
        // Skip trailing empty line from final newline
        if i == raw_lines.len() - 1 && raw.is_empty() {
            break;
        }
        let expanded = expand_tabs(raw);
        let number = (i + 1) * LINE_NUMBER_INCREMENT;
        lines.push(Line::new_data(expanded, number));
    }

    // BottomOfData sentinel
    lines.push(Line::bottom_of_data());

    Ok((VecLineStore::from_lines(lines), line_ending))
}

fn read_fixed_file(content: &[u8], lrecl: usize) -> io::Result<(VecLineStore, LineEnding)> {
    let mut lines = Vec::new();

    // TopOfData sentinel
    lines.push(Line::top_of_data());

    let mut offset = 0;
    let mut line_num = 0;
    while offset + lrecl <= content.len() {
        let chunk = &content[offset..offset + lrecl];
        let text = String::from_utf8_lossy(chunk).into_owned();
        line_num += 1;
        let number = line_num * LINE_NUMBER_INCREMENT;
        lines.push(Line::new_data(text, number));
        offset += lrecl;
    }

    // Handle partial last record if present
    if offset < content.len() {
        let chunk = &content[offset..];
        let mut text = String::from_utf8_lossy(chunk).into_owned();
        // Pad to lrecl
        while text.len() < lrecl {
            text.push(' ');
        }
        line_num += 1;
        let number = line_num * LINE_NUMBER_INCREMENT;
        lines.push(Line::new_data(text, number));
    }

    // BottomOfData sentinel
    lines.push(Line::bottom_of_data());

    // Fixed files don't have line endings per se; default to Lf for internal use
    Ok((VecLineStore::from_lines(lines), LineEnding::Lf))
}

fn detect_line_ending(content: &[u8]) -> LineEnding {
    for i in 0..content.len() {
        if content[i] == b'\r' {
            if i + 1 < content.len() && content[i + 1] == b'\n' {
                return LineEnding::CrLf;
            }
            return LineEnding::Cr;
        }
        if content[i] == b'\n' {
            return LineEnding::Lf;
        }
    }
    // Default
    LineEnding::Lf
}

/// Expand tab characters to spaces using standard tab stops.
fn expand_tabs(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut col = 0;
    for ch in line.chars() {
        if ch == '\t' {
            let spaces = TAB_STOP - (col % TAB_STOP);
            for _ in 0..spaces {
                result.push(' ');
            }
            col += spaces;
        } else {
            result.push(ch);
            col += 1;
        }
    }
    result
}

/// Write buffer lines to file.
pub fn write_file(
    path: &Path,
    lines: &VecLineStore,
    record_format: RecordFormat,
    line_ending: LineEnding,
    nulls_mode: bool,
) -> io::Result<()> {
    info!("write_file: {:?} format={:?} nulls={}", path, record_format, nulls_mode);
    // Write to a temporary file, then rename for atomicity
    let tmp_path = path.with_extension("spf-edit.tmp");

    {
        let mut file = fs::File::create(&tmp_path)?;

        match record_format {
            RecordFormat::Variable => {
                write_text_file(&mut file, lines, line_ending, nulls_mode)?;
            }
            RecordFormat::Fixed(lrecl) => {
                write_fixed_file(&mut file, lines, lrecl)?;
            }
        }

        file.flush()?;
    }

    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn write_text_file(
    file: &mut fs::File,
    lines: &VecLineStore,
    line_ending: LineEnding,
    nulls_mode: bool,
) -> io::Result<()> {
    let ending = line_ending.as_str();
    let mut first = true;

    for line in lines.iter() {
        if !line.is_data() {
            continue;
        }

        if !first {
            file.write_all(ending.as_bytes())?;
        }

        let data = if nulls_mode {
            line.data.trim_end().to_string()
        } else {
            line.data.clone()
        };

        file.write_all(data.as_bytes())?;
        first = false;
    }

    // Write final line ending
    if !first {
        file.write_all(ending.as_bytes())?;
    }

    Ok(())
}

fn write_fixed_file(
    file: &mut fs::File,
    lines: &VecLineStore,
    lrecl: usize,
) -> io::Result<()> {
    for line in lines.iter() {
        if !line.is_data() {
            continue;
        }

        let data = &line.data;
        if data.len() >= lrecl {
            // Truncate to record length
            file.write_all(data[..lrecl].as_bytes())?;
        } else {
            // Pad with spaces
            file.write_all(data.as_bytes())?;
            let padding = lrecl - data.len();
            for _ in 0..padding {
                file.write_all(b" ")?;
            }
        }
    }

    Ok(())
}

/// Create an empty buffer (for new files).
pub fn create_empty_buffer() -> VecLineStore {
    VecLineStore::from_lines(vec![
        Line::top_of_data(),
        Line::new_blank(LINE_NUMBER_INCREMENT),
        Line::bottom_of_data(),
    ])
}
