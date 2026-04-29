//! Shared types, enums, color constants, and layout constants used across modules.
//!
//! This module is imported via `use crate::types::*;` in most other modules.

use crossterm::style::Color;

// --- Record format ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordFormat {
    Variable,
    Fixed(usize),
}

// --- Line endings ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
    Cr,
}

impl LineEnding {
    pub fn as_str(self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
            LineEnding::Cr => "\r",
        }
    }
}

// --- Number mode ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberMode {
    On,
    Off,
}

// --- Scroll amount ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollAmount {
    Page,
    Half,
    Csr,
    Data,
    Max,
    Lines(usize),
}

impl ScrollAmount {
    pub fn resolve(&self, page_size: usize, cursor_row: usize) -> usize {
        match self {
            ScrollAmount::Page => page_size,
            ScrollAmount::Half => page_size / 2,
            ScrollAmount::Csr => cursor_row.max(1),
            ScrollAmount::Data => page_size.saturating_sub(1),
            ScrollAmount::Max => usize::MAX,
            ScrollAmount::Lines(n) => *n,
        }
    }
}

impl std::fmt::Display for ScrollAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollAmount::Page => write!(f, "PAGE"),
            ScrollAmount::Half => write!(f, "HALF"),
            ScrollAmount::Csr => write!(f, "CSR"),
            ScrollAmount::Data => write!(f, "DATA"),
            ScrollAmount::Max => write!(f, "MAX"),
            ScrollAmount::Lines(n) => write!(f, "{n}"),
        }
    }
}

// --- Field focus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldFocus {
    CommandLine,
    ScrollField,
    PrefixArea { screen_row: u16 },
    DataArea { screen_row: u16 },
}

// --- Input mode ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Overtype,
    Insert,
}

// --- Direction for FIND ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Prev,
    First,
    Last,
    All,
}

// --- LOCATE target ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocateTarget {
    LineNumber(usize),
    Label(String),
}

// --- RESET scope ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetScope {
    All,
    Command,
    Label,
}

// --- ON/OFF toggle ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnOff {
    On,
    Off,
}

// --- Message type ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub text: String,
    pub msg_type: MessageType,
}

// --- Color scheme constants ---

pub struct Colors;

impl Colors {
    // Title line
    pub const TITLE_FG: Color = Color::White;
    pub const TITLE_BG: Color = Color::Blue;

    // Command line prompt
    pub const CMD_PROMPT_FG: Color = Color::Green;
    pub const CMD_PROMPT_BG: Color = Color::Black;

    // Command line input
    pub const CMD_INPUT_FG: Color = Color::White;
    pub const CMD_INPUT_BG: Color = Color::Black;

    // Scroll indicator
    pub const SCROLL_FG: Color = Color::Green;
    pub const SCROLL_BG: Color = Color::Black;

    // Prefix area (line numbers)
    pub const PREFIX_FG: Color = Color::Cyan;
    pub const PREFIX_BG: Color = Color::Black;

    // Prefix area (pending command)
    pub const PREFIX_PENDING_FG: Color = Color::Yellow;
    pub const PREFIX_PENDING_BG: Color = Color::Black;

    // Prefix area (error)
    pub const PREFIX_ERROR_FG: Color = Color::Red;
    pub const PREFIX_ERROR_BG: Color = Color::Black;

    // Data area
    pub const DATA_FG: Color = Color::White;
    pub const DATA_BG: Color = Color::Black;

    // Sentinel lines (Top/Bottom of Data)
    pub const SENTINEL_FG: Color = Color::Blue;
    pub const SENTINEL_BG: Color = Color::Black;

    // Column ruler
    pub const RULER_FG: Color = Color::Blue;
    pub const RULER_BG: Color = Color::Black;

    // Found text highlight
    pub const FOUND_FG: Color = Color::Black;
    pub const FOUND_BG: Color = Color::Yellow;

    // Error messages
    pub const ERROR_FG: Color = Color::Red;
    pub const ERROR_BG: Color = Color::Black;

    // Status bar
    pub const STATUS_FG: Color = Color::Black;
    pub const STATUS_BG: Color = Color::Green;
}

// --- Layout constants ---

pub const PREFIX_WIDTH: usize = 6;
pub const SEPARATOR_WIDTH: usize = 1;
pub const HEADER_ROWS: u16 = 2; // title + command line
pub const FOOTER_ROWS: u16 = 1; // status bar
pub const TAB_STOP: usize = 8;
pub const LINE_NUMBER_INCREMENT: usize = 1;
pub const COMMAND_HISTORY_SIZE: usize = 50;
