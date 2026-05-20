//! Prefix area: parse user input in the 6-char prefix (line-number) area into
//! line commands, and format prefix text for display.
//!
//! In SPF/ISPF, users overtype the line number with short commands:
//!   I/I5  — insert 1 or 5 blank lines after this line
//!   D/D3  — delete 1 or 3 lines starting here
//!   R/R2  — repeat (duplicate) line 1 or 2 times
//!   C / M — mark line as copy / move source (needs A/B destination)
//!   CC/MM — block start+end markers for copy / move range
//!   DD    — block delete start+end
//!   RR    — block repeat start+end
//!   A / B — destination: insert After / Before this line
//!   .name — set a label on this line (for LOCATE .name)
//!
//! ## Known Issues
//!
//! - Block-end variants (CopyBlockEnd, MoveBlockEnd, DeleteBlockEnd,
//!   RepeatBlockEnd) are defined in `ParsedLineCmd` but never produced by
//!   `parse_prefix_command`.  The parser currently treats a second CC/DD/MM/RR
//!   identically to the start variant.  Pairing is handled in `line_cmd.rs`
//!   by tracking block_starts, but the parse step should ideally know about
//!   pending state so it can label a second CC as CopyBlockEnd.
//!   TODO: consider removing unused *BlockEnd variants or wiring them up.

use tracing::debug;

use crate::line::{Line, LineType};
use crate::types::{NumberMode, PREFIX_WIDTH};

/// Parsed line command from prefix area input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedLineCmd {
    Insert(usize),
    Delete(usize),
    Repeat(usize),
    CopySingle,
    MoveSingle,
    CopyBlock,
    MoveBlock,
    DeleteBlock,
    RepeatBlock(Option<usize>),
    After,
    Before,
    Label(String),
}

/// Result of parsing a prefix area.
#[derive(Debug)]
pub enum PrefixParseResult {
    None,
    Command(ParsedLineCmd),
    Error(String),
}

/// Parse the text typed in a prefix area into a line command.
pub fn parse_prefix_command(input: &str) -> PrefixParseResult {
    let trimmed = input.trim().to_uppercase();
    debug!("parse_prefix_command: {:?} -> trimmed={:?}", input, trimmed);

    if trimmed.is_empty() {
        return PrefixParseResult::None;
    }

    // Label: starts with '.'
    if let Some(label) = trimmed.strip_prefix('.') {
        if label.is_empty() {
            return PrefixParseResult::Error("Empty label".to_string());
        }
        return PrefixParseResult::Command(ParsedLineCmd::Label(label.to_string()));
    }

    // Single letter commands: A, B
    if trimmed == "A" {
        return PrefixParseResult::Command(ParsedLineCmd::After);
    }
    if trimmed == "B" {
        return PrefixParseResult::Command(ParsedLineCmd::Before);
    }

    // Block commands (double letter): CC, MM, DD, RR[n]
    if trimmed.starts_with("CC") && trimmed.len() == 2 {
        return PrefixParseResult::Command(ParsedLineCmd::CopyBlock);
    }
    if trimmed.starts_with("MM") && trimmed.len() == 2 {
        return PrefixParseResult::Command(ParsedLineCmd::MoveBlock);
    }
    if trimmed.starts_with("DD") && trimmed.len() == 2 {
        return PrefixParseResult::Command(ParsedLineCmd::DeleteBlock);
    }
    if trimmed.starts_with("RR") {
        if trimmed.len() == 2 {
            return PrefixParseResult::Command(ParsedLineCmd::RepeatBlock(None));
        }
        if let Ok(n) = trimmed[2..].parse::<usize>() {
            if n > 0 {
                return PrefixParseResult::Command(ParsedLineCmd::RepeatBlock(Some(n)));
            }
        }
        return PrefixParseResult::Error(format!("Invalid repeat count: {trimmed}"));
    }

    // Single letter + optional count: I[n], D[n], R[n], C, M
    if let Some(rest) = trimmed.strip_prefix('I') {
        match parse_count(rest) {
            Ok(count) => return PrefixParseResult::Command(ParsedLineCmd::Insert(count)),
            Err(e) => return e,
        }
    }
    if let Some(rest) = trimmed.strip_prefix('D') {
        match parse_count(rest) {
            Ok(count) => return PrefixParseResult::Command(ParsedLineCmd::Delete(count)),
            Err(e) => return e,
        }
    }
    if let Some(rest) = trimmed.strip_prefix('R') {
        match parse_count(rest) {
            Ok(count) => return PrefixParseResult::Command(ParsedLineCmd::Repeat(count)),
            Err(e) => return e,
        }
    }
    if trimmed == "C" {
        return PrefixParseResult::Command(ParsedLineCmd::CopySingle);
    }
    if trimmed == "M" {
        return PrefixParseResult::Command(ParsedLineCmd::MoveSingle);
    }

    PrefixParseResult::Error(format!("Unknown line command: {trimmed}"))
}

/// Parse the optional numeric count after a command letter.
/// Empty string returns 1 (default). Non-numeric returns error.
fn parse_count(s: &str) -> Result<usize, PrefixParseResult> {
    if s.is_empty() {
        return Ok(1);
    }
    match s.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err(PrefixParseResult::Error(
            "Count must be greater than 0".to_string(),
        )),
        Err(_) => Err(PrefixParseResult::Error(format!(
            "Invalid count: {s}"
        ))),
    }
}

/// Format a line number for display in the prefix area.
pub fn format_prefix(line: &Line, number_mode: NumberMode) -> String {
    match line.line_type {
        LineType::TopOfData | LineType::BottomOfData => {
            "******".to_string()
        }
        LineType::ColsRuler => {
            "=COLS>".to_string()
        }
        LineType::Message => {
            "==MSG>".to_string()
        }
        LineType::Data => {
            // If there's a pending command, show it instead
            if let Some(ref cmd) = line.prefix_cmd {
                return format!("{:<width$}", cmd, width = PREFIX_WIDTH);
            }

            match number_mode {
                NumberMode::On => {
                    format!("{:06}", line.current_number)
                }
                NumberMode::Off => {
                    "======".to_string()
                }
            }
        }
    }
}

/// Generate the column ruler text for the data area.
pub fn cols_ruler_text(width: usize) -> String {
    let mut ruler = String::with_capacity(width);
    for col in 1..=width {
        if col % 10 == 0 {
            // Show the tens digit
            ruler.push(char::from_digit((col / 10 % 10) as u32, 10).unwrap_or('0'));
        } else if col % 5 == 0 {
            ruler.push('+');
        } else {
            ruler.push('-');
        }
    }
    ruler
}

/// Generate the sentinel text for Top/Bottom of Data.
pub fn sentinel_text(line_type: LineType, data_width: usize) -> String {
    let msg = match line_type {
        LineType::TopOfData => " Top of Data ",
        LineType::BottomOfData => " Bottom of Data ",
        _ => return String::new(),
    };

    let total = data_width;
    if total <= msg.len() {
        return msg[..total].to_string();
    }

    let stars_total = total - msg.len();
    let stars_left = stars_total / 2;
    let stars_right = stars_total - stars_left;

    format!(
        "{}{}{}",
        "*".repeat(stars_left),
        msg,
        "*".repeat(stars_right)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ParsedLineCmd, PrefixParseResult, cols_ruler_text, format_prefix, parse_prefix_command,
        sentinel_text,
    };
    use crate::line::{Line, LineType};
    use crate::types::NumberMode;

    #[test]
    fn format_prefix_for_each_line_type() {
        assert_eq!(format_prefix(&Line::top_of_data(), NumberMode::On), "******");
        assert_eq!(format_prefix(&Line::bottom_of_data(), NumberMode::On), "******");
        assert_eq!(format_prefix(&Line::cols_ruler(), NumberMode::On), "=COLS>");
        assert_eq!(format_prefix(&Line::message("m".to_string()), NumberMode::On), "==MSG>");
    }

    #[test]
    fn format_prefix_for_data_line_numbers_and_pending_command() {
        let mut line = Line::new_data("abc".to_string(), 42);
        assert_eq!(format_prefix(&line, NumberMode::On), "000042");
        assert_eq!(format_prefix(&line, NumberMode::Off), "======");

        line.prefix_cmd = Some("D3".to_string());
        assert_eq!(format_prefix(&line, NumberMode::On), "D3    ");
    }

    #[test]
    fn parse_prefix_commands_basic_examples() {
        assert!(matches!(
            parse_prefix_command("i2"),
            PrefixParseResult::Command(ParsedLineCmd::Insert(2))
        ));
        assert!(matches!(
            parse_prefix_command("RR3"),
            PrefixParseResult::Command(ParsedLineCmd::RepeatBlock(Some(3)))
        ));
        assert!(matches!(
            parse_prefix_command(".mark"),
            PrefixParseResult::Command(ParsedLineCmd::Label(s)) if s == "MARK"
        ));
    }

    #[test]
    fn cols_ruler_and_sentinel_text_examples() {
        assert_eq!(cols_ruler_text(12), "----+----1--");

        let top = sentinel_text(LineType::TopOfData, 24);
        let bottom = sentinel_text(LineType::BottomOfData, 24);
        assert_eq!(top.len(), 24);
        assert_eq!(bottom.len(), 24);
        assert!(top.contains(" Top of Data "));
        assert!(bottom.contains(" Bottom of Data "));

        assert_eq!(sentinel_text(LineType::Data, 24), "");
    }
}
