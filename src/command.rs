//! Primary command parser and executor.
//!
//! Primary commands are typed on the command line (row 1) and executed on Enter.
//! Minimum abbreviations are supported (e.g. "SA" for SAVE, "F" for FIND).
//!
//! ## Supported Commands
//!
//!   SAVE (SA)        — write buffer to disk
//!   END              — save if modified, then exit
//!   CANCEL (CAN)     — exit without saving (prompts if modified)
//!   FIND (F) str     — search for text (supports quoted/delimited strings)
//!   RFIND (RF)       — repeat last FIND
//!   TOP (T)          — scroll to top
//!   BOTTOM (BOT)     — scroll to bottom
//!   UP [n|PAGE|HALF] — scroll up
//!   DOWN (DO) [n]    — scroll down
//!   LEFT (LE) [n]    — scroll left n columns
//!   RIGHT (RI) [n]   — scroll right n columns
//!   LOCATE (L) n/.lbl — scroll to line number or label
//!   RESET (RES) [CMD|LAB] — clear pending commands and/or labels
//!   COLS             — toggle column ruler
//!   NUMBER (NUM) ON|OFF — toggle line numbers
//!   NULLS (NUL) ON|OFF  — trailing-space stripping on save
//!   CAPS ON|OFF      — auto-uppercase typed characters
//!
//! ## Known Issues
//!
//! - `FIND ALL` returns the first match but shows a count.  The cursor only
//!   goes to the first hit.  Real ISPF would exclude all non-matching lines.
//! - `RFIND` always searches forward (Direction::Next).  It should remember
//!   the direction from the original FIND.
//! - `CANCEL` when `modified == true` sets `needs_save_prompt` but the editor
//!   just exits.  A Y/N confirmation prompt is not implemented yet.
//!   TODO: implement CANCEL confirmation dialog.

use tracing::{info, debug, warn};

use crate::buffer::FileBuffer;
use crate::line_store::LineStore;
use crate::types::{
    Direction, LocateTarget, Message, MessageType, NumberMode, OnOff, ResetScope, ScrollAmount,
};

/// Parsed primary command.
#[derive(Debug, Clone)]
pub enum PrimaryCommand {
    Save,
    End,
    Cancel,
    Find {
        text: String,
        direction: Direction,
    },
    RFind,
    Top,
    Bottom,
    Up(ScrollAmount),
    Down(ScrollAmount),
    Left(usize),
    Right(usize),
    Locate(LocateTarget),
    Reset(ResetScope),
    Cols,
    Number(OnOff),
    Nulls(OnOff),
    Caps(OnOff),
    /// Display a panel by ID.
    Panel(String),
}

/// Result of executing a command.
#[derive(Default)]
pub struct CommandResult {
    pub message: Option<Message>,
    pub should_exit: bool,
    pub needs_save_prompt: bool, // For CANCEL when modified
    pub scroll_to: Option<usize>,
    pub cursor_to: Option<(usize, usize)>, // (line_index, col)
    pub scroll_up: Option<usize>,
    pub scroll_down: Option<usize>,
    pub scroll_left: Option<usize>,
    pub scroll_right: Option<usize>,
    pub toggle_cols: bool,
    /// Panel to display (if set, editor should invoke panel manager).
    pub show_panel: Option<String>,
}

impl CommandResult {
    fn none() -> Self {
        Self::default()
    }

    fn with_message(text: &str, msg_type: MessageType) -> Self {
        Self {
            message: Some(Message {
                text: text.to_string(),
                msg_type,
            }),
            ..Self::default()
        }
    }

    fn info(text: &str) -> Self {
        Self::with_message(text, MessageType::Info)
    }

    fn error(text: &str) -> Self {
        Self::with_message(text, MessageType::Error)
    }

    fn exit() -> Self {
        Self {
            should_exit: true,
            ..Self::default()
        }
    }
}

/// Parse a command line string into a PrimaryCommand.
pub fn parse_command(input: &str) -> Result<PrimaryCommand, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(String::new()); // Empty command is not an error
    }

    let mut parts = trimmed.splitn(2, ' ');
    let verb = parts.next().unwrap_or("").to_uppercase();
    let args = parts.next().unwrap_or("").trim();

    // Verbs with no arguments and a fixed minimum-abbreviation length.
    if matches_cmd(&verb, "SAVE", 2) {
        return Ok(PrimaryCommand::Save);
    }
    if verb == "END" {
        return Ok(PrimaryCommand::End);
    }
    if matches_cmd(&verb, "CANCEL", 3) {
        return Ok(PrimaryCommand::Cancel);
    }
    if matches_cmd(&verb, "RFIND", 2) {
        return Ok(PrimaryCommand::RFind);
    }
    if matches_cmd(&verb, "TOP", 1) {
        return Ok(PrimaryCommand::Top);
    }
    if matches_cmd(&verb, "BOTTOM", 3) {
        return Ok(PrimaryCommand::Bottom);
    }
    if verb == "COLS" {
        return Ok(PrimaryCommand::Cols);
    }

    // Verbs with arguments.
    if matches_cmd(&verb, "FIND", 1) {
        return parse_find_command(args);
    }
    if verb == "UP" {
        return parse_scroll_command(args, true);
    }
    if matches_cmd(&verb, "DOWN", 2) {
        return parse_scroll_command(args, false);
    }
    if matches_cmd(&verb, "LEFT", 2) {
        return parse_count_arg(args).map(PrimaryCommand::Left);
    }
    if matches_cmd(&verb, "RIGHT", 2) {
        return parse_count_arg(args).map(PrimaryCommand::Right);
    }
    if matches_cmd(&verb, "LOCATE", 1) {
        return parse_locate_command(args);
    }
    if matches_cmd(&verb, "RESET", 3) {
        return parse_reset_command(args);
    }
    if matches_cmd(&verb, "NUMBER", 3) {
        return parse_onoff_command(args, "NUMBER").map(PrimaryCommand::Number);
    }
    if matches_cmd(&verb, "NULLS", 3) {
        return parse_onoff_command(args, "NULLS").map(PrimaryCommand::Nulls);
    }
    if verb == "CAPS" {
        return parse_onoff_command(args, "CAPS").map(PrimaryCommand::Caps);
    }
    if matches_cmd(&verb, "PANEL", 3) {
        if args.is_empty() {
            return Err("PANEL requires a panel name".to_string());
        }
        return Ok(PrimaryCommand::Panel(args.to_uppercase()));
    }

    Err(format!("Unknown command: {verb}"))
}

/// Parse an optional integer scroll-count argument (defaults to 1).
fn parse_count_arg(args: &str) -> Result<usize, String> {
    if args.is_empty() {
        return Ok(1);
    }
    args.parse::<usize>()
        .map_err(|_| format!("Invalid number: {args}"))
}

/// Check if `input` matches a command name with minimum abbreviation length.
fn matches_cmd(input: &str, full_name: &str, min_len: usize) -> bool {
    let len = input.len();
    if len < min_len || len > full_name.len() {
        return false;
    }
    full_name.starts_with(input)
}

fn parse_find_command(args: &str) -> Result<PrimaryCommand, String> {
    if args.is_empty() {
        return Err("FIND requires a search string".to_string());
    }

    let (text, rest) = extract_string_arg(args)?;
    let direction = parse_direction(rest.trim());

    Ok(PrimaryCommand::Find { text, direction })
}

/// Extract a string argument, handling quoted strings.
fn extract_string_arg(input: &str) -> Result<(String, &str), String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Expected a string argument".to_string());
    }

    // Check for quoted string
    if input.starts_with('\'') || input.starts_with('"') {
        let quote = input.chars().next().unwrap();
        if let Some(end_pos) = input[1..].find(quote) {
            let text = &input[1..1 + end_pos];
            let rest = &input[2 + end_pos..];
            return Ok((text.to_string(), rest));
        }
        return Err("Unterminated string".to_string());
    }

    // Check for delimiter-style: /string/ or $string$ etc.
    let first_char = input.chars().next().unwrap();
    if !first_char.is_alphanumeric() && !first_char.is_whitespace() {
        if let Some(end_pos) = input[1..].find(first_char) {
            let text = &input[1..1 + end_pos];
            let rest = &input[2 + end_pos..];
            return Ok((text.to_string(), rest));
        }
    }

    // Unquoted word (no spaces)
    let end = input.find(' ').unwrap_or(input.len());
    let text = &input[..end];
    let rest = &input[end..];
    Ok((text.to_string(), rest))
}

fn parse_direction(input: &str) -> Direction {
    let upper = input.to_uppercase();
    match upper.as_str() {
        "NEXT" | "N" => Direction::Next,
        "PREV" | "P" => Direction::Prev,
        "FIRST" => Direction::First,
        "LAST" => Direction::Last,
        "ALL" => Direction::All,
        _ => Direction::Next,
    }
}

fn parse_scroll_command(args: &str, is_up: bool) -> Result<PrimaryCommand, String> {
    let amount = if args.is_empty() {
        ScrollAmount::Page
    } else {
        parse_scroll_amount(args)?
    };

    if is_up {
        Ok(PrimaryCommand::Up(amount))
    } else {
        Ok(PrimaryCommand::Down(amount))
    }
}

fn parse_scroll_amount(input: &str) -> Result<ScrollAmount, String> {
    let upper = input.to_uppercase();
    match upper.as_str() {
        "PAGE" | "P" => Ok(ScrollAmount::Page),
        "HALF" | "H" => Ok(ScrollAmount::Half),
        "CSR" | "C" => Ok(ScrollAmount::Csr),
        "DATA" | "D" => Ok(ScrollAmount::Data),
        "MAX" | "M" => Ok(ScrollAmount::Max),
        _ => {
            let n = input
                .parse::<usize>()
                .map_err(|_| format!("Invalid scroll amount: {input}"))?;
            Ok(ScrollAmount::Lines(n))
        }
    }
}

fn parse_locate_command(args: &str) -> Result<PrimaryCommand, String> {
    if args.is_empty() {
        return Err("LOCATE requires a line number or label".to_string());
    }

    let target = args.trim();

    if target.starts_with('.') {
        Ok(PrimaryCommand::Locate(LocateTarget::Label(
            target.to_uppercase(),
        )))
    } else if let Ok(n) = target.parse::<usize>() {
        Ok(PrimaryCommand::Locate(LocateTarget::LineNumber(n)))
    } else {
        Err(format!("Invalid LOCATE target: {target}"))
    }
}

fn parse_reset_command(args: &str) -> Result<PrimaryCommand, String> {
    if args.is_empty() {
        return Ok(PrimaryCommand::Reset(ResetScope::All));
    }

    let upper = args.to_uppercase();
    match upper.as_str() {
        "COMMAND" | "CMD" => Ok(PrimaryCommand::Reset(ResetScope::Command)),
        "LABEL" | "LAB" => Ok(PrimaryCommand::Reset(ResetScope::Label)),
        _ => Err(format!("Invalid RESET option: {args}")),
    }
}

fn parse_onoff_command(args: &str, cmd_name: &str) -> Result<OnOff, String> {
    let upper = args.to_uppercase();
    match upper.as_str() {
        "ON" => Ok(OnOff::On),
        "OFF" => Ok(OnOff::Off),
        "" => Err(format!("{cmd_name} requires ON or OFF")),
        _ => Err(format!("Invalid {cmd_name} option: {args}")),
    }
}

/// Execute a primary command against the buffer.
pub fn execute_command(
    cmd: &PrimaryCommand,
    buffer: &mut FileBuffer,
    last_find: &mut Option<String>,
    current_line: usize,
    current_col: usize,
    page_size: usize,
    cursor_row: usize,
    scroll_amount: &ScrollAmount,
) -> CommandResult {
    match cmd {
        PrimaryCommand::Save => exec_save(buffer),
        PrimaryCommand::End => exec_end(buffer),
        PrimaryCommand::Cancel => exec_cancel(buffer),
        PrimaryCommand::Find { text, direction } => {
            exec_find(buffer, last_find, text, *direction, current_line, current_col)
        }
        PrimaryCommand::RFind => exec_rfind(
            buffer,
            last_find,
            current_line,
            current_col,
            page_size,
            cursor_row,
            scroll_amount,
        ),
        PrimaryCommand::Top => CommandResult {
            scroll_to: Some(0),
            ..CommandResult::default()
        },
        PrimaryCommand::Bottom => CommandResult {
            scroll_to: Some(buffer.line_count().saturating_sub(page_size)),
            ..CommandResult::default()
        },
        PrimaryCommand::Up(amount) => CommandResult {
            scroll_up: Some(amount.resolve(page_size, cursor_row)),
            ..CommandResult::default()
        },
        PrimaryCommand::Down(amount) => CommandResult {
            scroll_down: Some(amount.resolve(page_size, cursor_row)),
            ..CommandResult::default()
        },
        PrimaryCommand::Left(n) => CommandResult {
            scroll_left: Some(*n),
            ..CommandResult::default()
        },
        PrimaryCommand::Right(n) => CommandResult {
            scroll_right: Some(*n),
            ..CommandResult::default()
        },
        PrimaryCommand::Locate(target) => exec_locate(buffer, target),
        PrimaryCommand::Reset(scope) => exec_reset(buffer, scope),
        PrimaryCommand::Cols => CommandResult {
            toggle_cols: true,
            ..CommandResult::default()
        },
        PrimaryCommand::Number(onoff) => {
            buffer.number_mode = match onoff {
                OnOff::On => NumberMode::On,
                OnOff::Off => NumberMode::Off,
            };
            CommandResult::none()
        }
        PrimaryCommand::Nulls(onoff) => {
            buffer.nulls_mode = matches!(onoff, OnOff::On);
            CommandResult::none()
        }
        PrimaryCommand::Caps(onoff) => {
            buffer.caps_mode = matches!(onoff, OnOff::On);
            CommandResult::info(if buffer.caps_mode { "CAPS ON" } else { "CAPS OFF" })
        }
        PrimaryCommand::Panel(panel_id) => CommandResult {
            show_panel: Some(panel_id.clone()),
            ..CommandResult::default()
        },
    }
}

// ----- Per-command handlers -----

fn exec_save(buffer: &mut FileBuffer) -> CommandResult {
    if buffer.browse_mode {
        warn!("SAVE attempted in browse mode");
        return CommandResult::error("Cannot SAVE in browse mode");
    }
    match buffer.save() {
        Ok(()) => {
            info!("SAVE successful");
            CommandResult::info("SAVED")
        }
        Err(e) => {
            warn!("SAVE failed: {e}");
            CommandResult::error(&format!("Save failed: {e}"))
        }
    }
}

fn exec_end(buffer: &mut FileBuffer) -> CommandResult {
    if buffer.browse_mode || !buffer.modified {
        info!(
            "END — exiting (browse={} modified={})",
            buffer.browse_mode, buffer.modified
        );
        return CommandResult::exit();
    }
    info!("END — saving and exiting");
    match buffer.save() {
        Ok(()) => CommandResult {
            message: Some(Message {
                text: "SAVED".to_string(),
                msg_type: MessageType::Info,
            }),
            should_exit: true,
            ..CommandResult::default()
        },
        Err(e) => CommandResult::error(&format!("Save failed: {e}")),
    }
}

fn exec_cancel(buffer: &FileBuffer) -> CommandResult {
    if buffer.modified {
        CommandResult {
            needs_save_prompt: true,
            ..CommandResult::default()
        }
    } else {
        CommandResult::exit()
    }
}

fn exec_find(
    buffer: &FileBuffer,
    last_find: &mut Option<String>,
    text: &str,
    direction: Direction,
    current_line: usize,
    current_col: usize,
) -> CommandResult {
    debug!("FIND {:?} direction={:?}", text, direction);
    *last_find = Some(text.to_string());
    let start_col = if matches!(direction, Direction::Next) {
        current_col + 1
    } else {
        current_col
    };
    match buffer.find_text(text, current_line, start_col, direction) {
        Some((line_idx, col_idx)) => {
            let mut msg_text = format!("CHARS '{text}' FOUND");
            if matches!(direction, Direction::All) {
                let count = buffer.count_occurrences(text);
                msg_text = format!("CHARS '{text}' FOUND - {count} occurrence(s)");
            }
            CommandResult {
                message: Some(Message {
                    text: msg_text,
                    msg_type: MessageType::Info,
                }),
                cursor_to: Some((line_idx, col_idx)),
                scroll_to: Some(line_idx),
                ..CommandResult::default()
            }
        }
        None => CommandResult::error(&format!("CHARS '{text}' NOT FOUND")),
    }
}

fn exec_rfind(
    buffer: &mut FileBuffer,
    last_find: &mut Option<String>,
    current_line: usize,
    current_col: usize,
    page_size: usize,
    cursor_row: usize,
    scroll_amount: &ScrollAmount,
) -> CommandResult {
    let Some(text) = last_find.clone() else {
        return CommandResult::error("No previous FIND command");
    };
    execute_command(
        &PrimaryCommand::Find {
            text,
            direction: Direction::Next,
        },
        buffer,
        last_find,
        current_line,
        current_col,
        page_size,
        cursor_row,
        scroll_amount,
    )
}

fn exec_locate(buffer: &FileBuffer, target: &LocateTarget) -> CommandResult {
    match target {
        LocateTarget::LineNumber(n) => {
            for i in 0..buffer.line_count() {
                if let Some(line) = buffer.lines.get(i) {
                    if line.is_data() && line.current_number >= *n {
                        return CommandResult {
                            scroll_to: Some(i),
                            ..CommandResult::default()
                        };
                    }
                }
            }
            CommandResult::error(&format!("Line {n} not found"))
        }
        LocateTarget::Label(name) => match buffer.get_label(name) {
            Some(idx) => CommandResult {
                scroll_to: Some(idx),
                ..CommandResult::default()
            },
            None => CommandResult::error(&format!("Label {name} not found")),
        },
    }
}

fn exec_reset(buffer: &mut FileBuffer, scope: &ResetScope) -> CommandResult {
    match scope {
        ResetScope::All => {
            buffer.reset_commands();
            buffer.reset_labels();
        }
        ResetScope::Command => buffer.reset_commands(),
        ResetScope::Label => buffer.reset_labels(),
    }
    CommandResult::none()
}
