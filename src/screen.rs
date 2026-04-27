//! Screen rendering: draws the TUI to the terminal.
//!
//! ## Screen Layout (top to bottom)
//!
//!   Row 0:  Title line  — mode (EDIT/BROWSE), filename, messages, column range.
//!   Row 1:  Command line — "Command ===> ...   Scroll ===> PAGE"
//!   Row 2+: Data lines   — each row is: [prefix 6 chars] [sep 1 char] [data area]
//!                          prefix shows line numbers, pending cmds, or sentinel markers.
//!
//! The data area scrolls both vertically (`top_line_index`) and horizontally
//! (`horizontal_offset`).  UTF-8 / wide-char text is handled via `unicode_width`.
//!
//! ## Known Issues
//!
//! - `draw_full` is called on every keystroke (`Editor::redraw`).  This is
//!   simple but wasteful — a full terminal repaint on every key.  For large
//!   files or slow terminals this will flicker.  A differential / dirty-line
//!   approach would be better.
//!   TODO: track dirty lines and only redraw changed rows.
//! - The title line padding uses `.len()` (byte length) to compute remaining
//!   space, which is wrong if the filename or message contains non-ASCII.
//!   Should use `UnicodeWidthStr::width()` there too.
//!   TODO: fix title line width calculation for Unicode.
//! - `horizontal_offset` counts in characters but `cursor_col` is also in
//!   character units — works for monospace ASCII but the relationship gets
//!   complex with CJK wide chars.  The current approach treats every char
//!   as 1 cursor position which is only correct for single-width chars.
//!   TODO: unify column model to display-column widths.

use std::io::{self, Write};
use tracing::{error, trace};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use unicode_width::UnicodeWidthStr;

use crate::buffer::FileBuffer;
use crate::line::{Line, LineFlags, LineType};
use crate::line_store::LineStore;
use crate::prefix::{cols_ruler_text, format_prefix, sentinel_text};
use crate::types::*;

pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub top_line_index: usize,
    pub horizontal_offset: usize,
    pub scroll_amount: ScrollAmount,
    pub command_line: String,
    pub command_cursor_pos: usize,
    pub scroll_field_text: String,
    pub message: Option<Message>,
    pub prefix_width: usize,
    pub cols_visible: bool,
    pub needs_full_redraw: bool,
}

impl Screen {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Screen {
            width,
            height,
            top_line_index: 0,
            horizontal_offset: 0,
            scroll_amount: ScrollAmount::Page,
            command_line: String::new(),
            command_cursor_pos: 0,
            scroll_field_text: "PAGE".to_string(),
            message: None,
            prefix_width: PREFIX_WIDTH,
            cols_visible: false,
            needs_full_redraw: true,
        })
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.needs_full_redraw = true;
    }

    /// Number of data lines that fit on screen (total height minus header rows).
    pub fn data_rows(&self) -> usize {
        self.height.saturating_sub(HEADER_ROWS) as usize
    }

    /// Width available for the data area.
    pub fn data_width(&self) -> usize {
        (self.width as usize).saturating_sub(self.prefix_width + SEPARATOR_WIDTH)
    }

    /// Column position where data area starts.
    pub fn data_start_col(&self) -> u16 {
        (self.prefix_width + SEPARATOR_WIDTH) as u16
    }

    pub fn draw_char<W: Write>(
        &mut self,
        stdout: &mut W,
        x: u16,
        y: u16,
        c: char,
    ) -> io::Result<()> {
        queue!(stdout, MoveTo(x, y), Print(c))?;
        stdout.flush()?;
        Ok(())
    }

    /// Draw the full screen.
    pub fn draw_full<W: Write>(&mut self, stdout: &mut W, buffer: &FileBuffer) -> io::Result<()> {
        trace!(
            "draw_full: top_line={} h_offset={} size={}x{}",
            self.top_line_index, self.horizontal_offset, self.width, self.height
        );
        queue!(stdout, Clear(ClearType::All))?;
        self.draw_title_line(stdout, buffer)?;
        self.draw_command_line(stdout)?;
        self.draw_data_lines(stdout, buffer)?;
        stdout.flush()?;
        self.needs_full_redraw = false;
        Ok(())
    }

    pub fn draw_line<W: Write>(
        &mut self,
        stdout: &mut W,
        line_index: usize,
        buffer: &FileBuffer,
    ) -> io::Result<()> {
        match line_index {
            0 => self.draw_title_line(stdout, buffer),
            1 => self.draw_command_line(stdout),
            terminal_row if terminal_row >= HEADER_ROWS as usize => {
                let screen_row = terminal_row - HEADER_ROWS as usize;
                let line_index = self.top_line_index + screen_row;

                queue!(stdout, MoveTo(0, terminal_row as u16))?;
                self.draw_data_line(stdout, line_index, buffer)
            }
            n => {
                error!("drawing line {n}");
                Ok(())
            }
        }
    }

    /// Draw the title line (row 0).
    fn draw_title_line<W: Write>(&self, stdout: &mut W, buffer: &FileBuffer) -> io::Result<()> {
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Colors::TITLE_FG),
            SetBackgroundColor(Colors::TITLE_BG),
        )?;

        let mode = if buffer.browse_mode { "BROWSE" } else { "EDIT" };
        let name = buffer.display_name();
        let modified_indicator = if buffer.modified { " - Modified" } else { "" };

        let col_start = self.horizontal_offset + 1;
        let col_end = self.horizontal_offset + self.data_width();
        let col_info = format!("Columns {:05} {:05}", col_start, col_end);

        let left = format!(" {:<8}{}{}", mode, name, modified_indicator);

        // Message on the right side of title
        let right = if let Some(ref msg) = self.message {
            format!("{}  {}", msg.text, col_info)
        } else {
            format!("  {}", col_info)
        };

        let padding = (self.width as usize).saturating_sub(left.len() + right.len());
        let title = format!("{}{}{}", left, " ".repeat(padding), right);

        // Truncate to screen width
        let display: String = title.chars().take(self.width as usize).collect();
        queue!(stdout, Print(&display))?;

        // Fill remaining width
        let remaining = (self.width as usize).saturating_sub(display.len());
        if remaining > 0 {
            queue!(stdout, Print(" ".repeat(remaining)))?;
        }

        queue!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw the command line (row 1).
    fn draw_command_line<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        queue!(stdout, MoveTo(0, 1))?;

        // "Command ===> "
        let prompt = "Command ===> ";
        queue!(
            stdout,
            SetForegroundColor(Colors::CMD_PROMPT_FG),
            SetBackgroundColor(Colors::CMD_PROMPT_BG),
            Print(prompt),
        )?;

        // Command input area
        let scroll_label = "  Scroll ===> ";
        let scroll_value = &self.scroll_field_text;
        let scroll_section_width = scroll_label.len() + scroll_value.len() + 1;
        let cmd_input_width =
            (self.width as usize).saturating_sub(prompt.len() + scroll_section_width);

        let cmd_display: String = self.command_line.chars().take(cmd_input_width).collect();
        let cmd_padding = cmd_input_width.saturating_sub(cmd_display.len());

        queue!(
            stdout,
            SetForegroundColor(Colors::CMD_INPUT_FG),
            SetBackgroundColor(Colors::CMD_INPUT_BG),
            Print(&cmd_display),
            Print(" ".repeat(cmd_padding)),
        )?;

        // Scroll indicator
        queue!(
            stdout,
            SetForegroundColor(Colors::SCROLL_FG),
            SetBackgroundColor(Colors::SCROLL_BG),
            Print(scroll_label),
            Print(scroll_value),
            Print(" "),
        )?;

        queue!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw all data lines.
    fn draw_data_lines<W: Write>(&self, stdout: &mut W, buffer: &FileBuffer) -> io::Result<()> {
        let data_rows = self.data_rows();

        for screen_row in 0..data_rows {
            let line_index = self.top_line_index + screen_row;
            let terminal_row = (HEADER_ROWS as usize + screen_row) as u16;

            queue!(stdout, MoveTo(0, terminal_row))?;

            self.draw_data_line(stdout, line_index, buffer)?;
        }

        Ok(())
    }

    /// Draw a single line (prefix + separator + data).
    fn draw_data_line<W: Write>(
        &self,
        stdout: &mut W,
        line_index: usize,
        buffer: &FileBuffer,
    ) -> io::Result<()> {
        if line_index < buffer.lines.len() {
            if let Some(line) = buffer.lines.get(line_index) {
                // --- Prefix area ---
                let prefix_text = format_prefix(line, buffer.number_mode);
                let (prefix_fg, prefix_bg) = self.prefix_colors(line);

                queue!(
                    stdout,
                    SetForegroundColor(prefix_fg),
                    SetBackgroundColor(prefix_bg),
                    Print(&prefix_text),
                )?;

                // --- Separator ---
                queue!(
                    stdout,
                    SetForegroundColor(Colors::DATA_FG),
                    SetBackgroundColor(Colors::DATA_BG),
                    Print(" "),
                )?;

                // --- Data area ---
                let data_width = self.data_width();
                let (data_fg, data_bg) = self.data_colors(line);

                queue!(
                    stdout,
                    SetForegroundColor(data_fg),
                    SetBackgroundColor(data_bg),
                )?;

                let display_text = self.get_display_text(line, data_width);
                queue!(stdout, Print(&display_text))?;

                // Pad to fill data area
                let padding =
                    data_width.saturating_sub(UnicodeWidthStr::width(display_text.as_str()));
                if padding > 0 {
                    queue!(stdout, Print(" ".repeat(padding)))?;
                }

                return (queue!(stdout, ResetColor));
            }
        }
        // Empty row (past end of buffer)
        queue!(
            stdout,
            SetForegroundColor(Colors::DATA_FG),
            SetBackgroundColor(Colors::DATA_BG),
            Print(" ".repeat(self.width as usize)),
            ResetColor,
        )
    }
    /// Get the text to display in the data area for a line.
    fn get_display_text(&self, line: &Line, data_width: usize) -> String {
        match line.line_type {
            LineType::TopOfData | LineType::BottomOfData => {
                sentinel_text(line.line_type, data_width)
            }
            LineType::ColsRuler => {
                let ruler = cols_ruler_text(data_width + self.horizontal_offset);
                if self.horizontal_offset < ruler.len() {
                    let end = (self.horizontal_offset + data_width).min(ruler.len());
                    ruler[self.horizontal_offset..end].to_string()
                } else {
                    String::new()
                }
            }
            LineType::Message => truncate_to_width(&line.data, data_width),
            LineType::Data => {
                let data = &line.data;
                let skipped = skip_chars(data, self.horizontal_offset);
                truncate_to_width(skipped, data_width)
            }
        }
    }

    /// Get colors for the prefix area based on line state.
    fn prefix_colors(&self, line: &Line) -> (Color, Color) {
        if line.flags.contains(LineFlags::CMD_ERROR) {
            (Colors::PREFIX_ERROR_FG, Colors::PREFIX_ERROR_BG)
        } else if line.flags.contains(LineFlags::PENDING_CMD) {
            (Colors::PREFIX_PENDING_FG, Colors::PREFIX_PENDING_BG)
        } else if line.is_sentinel() {
            (Colors::SENTINEL_FG, Colors::SENTINEL_BG)
        } else {
            (Colors::PREFIX_FG, Colors::PREFIX_BG)
        }
    }

    /// Get colors for the data area based on line type.
    fn data_colors(&self, line: &Line) -> (Color, Color) {
        match line.line_type {
            LineType::TopOfData | LineType::BottomOfData => {
                (Colors::SENTINEL_FG, Colors::SENTINEL_BG)
            }
            LineType::ColsRuler => (Colors::RULER_FG, Colors::RULER_BG),
            LineType::Message => (Colors::ERROR_FG, Colors::ERROR_BG),
            LineType::Data => (Colors::DATA_FG, Colors::DATA_BG),
        }
    }

    // --- Scrolling ---

    pub fn scroll_up(&mut self, lines: usize) {
        let old = self.top_line_index;
        self.top_line_index = self.top_line_index.saturating_sub(lines);
        trace!("scroll_up: {} -> {} (by {lines})", old, self.top_line_index);
        self.needs_full_redraw = true;
    }

    pub fn scroll_down(&mut self, lines: usize, max_index: usize) {
        let old = self.top_line_index;
        self.top_line_index = (self.top_line_index + lines).min(max_index);
        self.needs_full_redraw = true;
        trace!(
            "scroll_down: {} -> {} (by {lines}, max={max_index})",
            old, self.top_line_index
        );
    }

    pub fn scroll_left(&mut self, cols: usize) {
        self.horizontal_offset = self.horizontal_offset.saturating_sub(cols);
        self.needs_full_redraw = true;
    }

    pub fn scroll_right(&mut self, cols: usize) {
        self.horizontal_offset += cols;
        self.needs_full_redraw = true;
    }

    pub fn scroll_to_line(&mut self, line_index: usize, max_index: usize) {
        self.top_line_index = line_index.min(max_index);
        self.needs_full_redraw = true;
    }

    /// Ensure a line index is visible on screen. Returns true if scrolling occurred.
    pub fn ensure_visible(&mut self, line_index: usize, max_index: usize) -> bool {
        if line_index < self.top_line_index {
            self.top_line_index = line_index;
            self.needs_full_redraw = true;
            return true;
        }
        let bottom = self.top_line_index + self.data_rows();
        if line_index >= bottom {
            self.top_line_index = line_index.saturating_sub(self.data_rows() / 2);
            self.top_line_index = self.top_line_index.min(max_index);
            self.needs_full_redraw = true;
            return true;
        }
        false
    }

    /// Convert a buffer line index to a screen row (if visible).
    pub fn line_to_screen_row(&self, line_index: usize) -> Option<u16> {
        if line_index >= self.top_line_index && line_index < self.top_line_index + self.data_rows()
        {
            Some((HEADER_ROWS as usize + line_index - self.top_line_index) as u16)
        } else {
            None
        }
    }

    /// Convert a screen row to a buffer line index.
    pub fn screen_row_to_line(&self, screen_row: u16) -> usize {
        self.top_line_index + (screen_row as usize).saturating_sub(HEADER_ROWS as usize)
    }

    /// Get the command line prompt width (position where input starts).
    pub fn command_input_col(&self) -> u16 {
        13 // "Command ===> " is 13 chars
    }

    /// Parse scroll field text into ScrollAmount.
    pub fn parse_scroll_field(&mut self) {
        let upper = self.scroll_field_text.trim().to_uppercase();
        self.scroll_amount = match upper.as_str() {
            "PAGE" | "P" => ScrollAmount::Page,
            "HALF" | "H" => ScrollAmount::Half,
            "CSR" | "C" => ScrollAmount::Csr,
            "DATA" | "D" => ScrollAmount::Data,
            "MAX" | "M" => ScrollAmount::Max,
            _ => {
                if let Ok(n) = upper.parse::<usize>() {
                    ScrollAmount::Lines(n)
                } else {
                    ScrollAmount::Page
                }
            }
        };
        self.scroll_field_text = format!("{}", self.scroll_amount);
    }
}

/// Skip `n` characters from the start of a string, returning the remainder.
fn skip_chars(s: &str, n: usize) -> &str {
    let mut chars = s.chars();
    for _ in 0..n {
        if chars.next().is_none() {
            return "";
        }
    }
    chars.as_str()
}

/// Truncate a string to fit within `max_width` display columns.
fn truncate_to_width(s: &str, max_width: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    let mut result = String::new();
    let mut width = 0;
    for ch in s.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width {
            break;
        }
        result.push(ch);
        width += ch_width;
    }
    result
}
