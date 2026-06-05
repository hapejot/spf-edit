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
use spf_edit::buffer::DisplayLine;
use tracing::{error, trace};

use chrono::Local;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self},
};
use unicode_width::UnicodeWidthStr;

use crate::buffer::FileBuffer;
use crate::line::{Line, LineFlags, LineType};
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionArea {
    CommandLine,
    ScrollField,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    Text {
        area: SelectionArea,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    },
    Lines {
        start_line: usize,
        end_line: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionPopup {
    pub row: u16,
    pub col: u16,
    pub selected: usize,
    pub items: Vec<String>,
}

impl Selection {
    fn command_range(&self) -> Option<(usize, usize)> {
        match self {
            Selection::Text {
                area: SelectionArea::CommandLine,
                start_col,
                end_col,
                ..
            } => Some(normalize_columns(*start_col, *end_col)),
            _ => None,
        }
    }

    fn scroll_range(&self) -> Option<(usize, usize)> {
        match self {
            Selection::Text {
                area: SelectionArea::ScrollField,
                start_col,
                end_col,
                ..
            } => Some(normalize_columns(*start_col, *end_col)),
            _ => None,
        }
    }

    fn line_range(&self) -> Option<(usize, usize)> {
        match self {
            Selection::Lines {
                start_line,
                end_line,
            } => Some(normalize_columns(*start_line, *end_line)),
            _ => None,
        }
    }

    fn data_range_for_line(&self, line_index: usize) -> Option<(usize, usize)> {
        match self {
            Selection::Text {
                area: SelectionArea::Data,
                start_line,
                start_col,
                end_line,
                end_col,
            } => {
                let ((start_line, start_col), (end_line, end_col)) =
                    normalize_points((*start_line, *start_col), (*end_line, *end_col));
                if line_index < start_line || line_index > end_line {
                    return None;
                }
                if start_line == end_line {
                    return Some((start_col, end_col.saturating_add(1)));
                }
                if line_index == start_line {
                    return Some((start_col, usize::MAX));
                }
                if line_index == end_line {
                    return Some((0, end_col.saturating_add(1)));
                }
                Some((0, usize::MAX))
            }
            _ => None,
        }
    }
}

fn normalize_columns(start: usize, end: usize) -> (usize, usize) {
    if start <= end {
        (start, end.saturating_add(1))
    } else {
        (end, start.saturating_add(1))
    }
}

fn normalize_points(
    start: (usize, usize),
    end: (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    if start <= end {
        (start, end)
    } else {
        (end, start)
    }
}

pub struct Screen {
    pub width: u16,
    pub height: u16,
    top_line_index: usize,
    pub horizontal_offset: usize,
    pub scroll_amount: ScrollAmount,
    pub command_line: String,
    pub command_cursor_pos: usize,
    pub scroll_field_text: String,
    pub message: Option<Message>,
    pub prefix_width: usize,
    pub cols_visible: bool,
    pub needs_full_redraw: bool,
    pub input_mode: InputMode,
    pub status_info: String,
    pub selection: Option<Selection>,
    pub completion_popup: Option<CompletionPopup>,
    pub data_lines: Vec<DisplayLine>,
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
            input_mode: InputMode::Overtype,
            status_info: String::new(),
            selection: None,
            completion_popup: None,
            data_lines: Vec::new(),
        })
    }

    pub fn set_selection(&mut self, selection: Option<Selection>) {
        self.selection = selection;
        self.needs_full_redraw = true;
    }

    pub fn clear_selection(&mut self) {
        self.set_selection(None);
    }

    pub fn set_completion_popup(&mut self, popup: Option<CompletionPopup>) {
        if self.completion_popup != popup {
            self.completion_popup = popup;
            self.needs_full_redraw = true;
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.needs_full_redraw = true;
    }

    /// Number of data lines that fit on screen (total height minus header and footer rows).
    pub fn data_rows(&self) -> usize {
        self.height.saturating_sub(HEADER_ROWS + FOOTER_ROWS) as usize
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
//        queue!(stdout, Clear(ClearType::All))?;
        self.draw_title_line(stdout, buffer)?;
        self.draw_command_line(stdout)?;
        self.draw_data_lines(stdout, buffer)?;
        self.draw_completion_popup(stdout)?;
        self.draw_status_bar(stdout, buffer)?;
        stdout.flush()?;
        self.needs_full_redraw = false;
        Ok(())
    }

    fn draw_completion_popup<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        let Some(popup) = &self.completion_popup else {
            return Ok(());
        };
        if popup.items.is_empty() {
            return Ok(());
        }

        let visible_rows = popup.items.len().min(8);
        let width = popup
            .items
            .iter()
            .take(visible_rows)
            .map(|s| s.chars().count())
            .max()
            .unwrap_or(0)
            .saturating_add(2)
            .min(self.width as usize);

        let max_row = self.height.saturating_sub(2);
        let start_row = popup.row.min(max_row);
        let max_col = self.width.saturating_sub(width as u16);
        let start_col = popup.col.min(max_col);

        for idx in 0..visible_rows {
            let row = start_row + idx as u16;
            if row >= self.height.saturating_sub(1) {
                break;
            }
            let item = popup.items.get(idx).cloned().unwrap_or_default();
            let selected = idx == popup.selected;
            let fg = if selected {
                Colors::COMPLETION_SELECTED_FG
            } else {
                Colors::COMPLETION_FG
            };
            let bg = if selected {
                Colors::COMPLETION_SELECTED_BG
            } else {
                Colors::COMPLETION_BG
            };
            let mut text = item.chars().take(width.saturating_sub(1)).collect::<String>();
            let fill = width.saturating_sub(text.chars().count());
            if fill > 0 {
                text.push_str(&" ".repeat(fill));
            }
            queue!(
                stdout,
                MoveTo(start_col, row),
                SetForegroundColor(fg),
                SetBackgroundColor(bg),
                Print(text),
                ResetColor,
            )?;
        }

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
                self.draw_data_line(stdout, line_index, buffer)?;
                self.draw_status_bar(stdout, buffer)
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
        self.draw_text_with_selection(
            stdout,
            &cmd_display,
            cmd_input_width,
            Colors::CMD_INPUT_FG,
            Colors::CMD_INPUT_BG,
            self.selection
                .as_ref()
                .and_then(|selection| selection.command_range()),
        )?;

        // Scroll indicator
        let scroll_selection = self
            .selection
            .as_ref()
            .and_then(|selection| selection.scroll_range());
        queue!(
            stdout,
            SetForegroundColor(Colors::SCROLL_FG),
            SetBackgroundColor(Colors::SCROLL_BG),
            Print(scroll_label),
        )?;
        self.draw_text_with_selection(
            stdout,
            scroll_value,
            scroll_value.chars().count() + 1,
            Colors::SCROLL_FG,
            Colors::SCROLL_BG,
            scroll_selection,
        )?;

        queue!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw the status bar (last row).
    fn draw_status_bar<W: Write>(&self, stdout: &mut W, buffer: &FileBuffer) -> io::Result<()> {
        let status_row = self.height.saturating_sub(1);
        let w = self.width as usize;

        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();

        let mode_str = match self.input_mode {
            InputMode::Insert => "INS",
            InputMode::Overtype => "OVR",
        };

        let line_count = buffer.data_line_count();
        let col_pos = self.horizontal_offset + 1;

        let caps = if buffer.caps_mode { "CAPS" } else { "    " };

        let left = format!(
            " {} | {} | Lines: {} | Col: {} | {} |{}",
            time_str, mode_str, line_count, col_pos, caps, self.status_info
        );
        let right = format!("EDIT ");
        let padding = w.saturating_sub(left.len() + right.len());
        let bar = format!("{}{}{}", left, " ".repeat(padding), right);

        // Truncate to screen width
        let display: String = bar.chars().take(w).collect();

        queue!(
            stdout,
            MoveTo(0, status_row),
            SetForegroundColor(Colors::STATUS_FG),
            SetBackgroundColor(Colors::STATUS_BG),
            Print(&display),
        )?;

        // // Fill remaining width
        // let remaining = w.saturating_sub(display.len());
        // if remaining > 0 {
        //     queue!(stdout, Print(" ".repeat(remaining)))?;
        // }

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
        if line_index < buffer.line_count() {
            if let Some(display_line) =
                buffer.get_lines(line_index..line_index+1).get(0)
            {
                let line_selected = self.line_is_selected(line_index);
                let data_selection = if line_selected {
                    Some((0, self.data_width()))
                } else {
                    self.visible_data_selection_range(line_index, display_line.display.len())
                };
                // --- Prefix area ---
                self.draw_text_with_selection(
                    stdout,
                    &display_line.prefix.iter().cloned().collect::<String>(),
                    self.prefix_width,
                    Colors::PREFIX_FG,
                    Colors::PREFIX_BG,
                    if line_selected {
                        Some((0, self.prefix_width))
                    } else {
                        None
                    },
                )?;

                // --- Separator ---
                if line_selected {
                    queue!(
                        stdout,
                        SetForegroundColor(Colors::SELECT_FG),
                        SetBackgroundColor(Colors::SELECT_BG),
                        Print(" "),
                    )?;
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(Colors::DATA_FG),
                        SetBackgroundColor(Colors::DATA_BG),
                        Print(" "),
                    )?;
                }

                // --- Data area ---
                let data_width = self.data_width();
                self.draw_text_with_selection(
                    stdout,
                    &display_line.display[1..80].iter().cloned().collect::<String>(),
                    data_width,
                    Colors::DATA_FG,
                    Colors::DATA_BG,
                    data_selection,
                )?;

                return queue!(stdout, ResetColor);
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
            LineType::Insert => (Colors::DATA_FG, Colors::DATA_BG),
            _ => todo!()
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

    fn line_is_selected(&self, line_index: usize) -> bool {
        self.selection
            .as_ref()
            .and_then(|selection| selection.line_range())
            .map(|(start, end)| line_index >= start && line_index < end)
            .unwrap_or(false)
    }

    fn visible_data_selection_range(
        &self,
        line_index: usize,
        visible_len: usize,
    ) -> Option<(usize, usize)> {
        let (start, end) = self
            .selection
            .as_ref()
            .and_then(|selection| selection.data_range_for_line(line_index))?;
        let visible_start = start.saturating_sub(self.horizontal_offset);
        let visible_end = if end == usize::MAX {
            self.data_width()
        } else {
            end.saturating_sub(self.horizontal_offset)
        };
        let clamped_start = visible_start.min(self.data_width()).min(visible_len.max(visible_start));
        let clamped_end = visible_end.min(self.data_width()).max(clamped_start);
        Some((clamped_start, clamped_end))
    }

    fn draw_text_with_selection<W: Write>(
        &self,
        stdout: &mut W,
        text: &str,
        total_width: usize,
        fg: Color,
        bg: Color,
        selected_range: Option<(usize, usize)>,
    ) -> io::Result<()> {
        let chars: Vec<char> = text.chars().collect();
        for index in 0..total_width {
            let selected = selected_range
                .map(|(start, end)| index >= start && index < end)
                .unwrap_or(false);
            let (cell_fg, cell_bg) = if selected {
                (Colors::SELECT_FG, Colors::SELECT_BG)
            } else {
                (fg, bg)
            };
            let ch = chars.get(index).copied().unwrap_or(' ');
            queue!(
                stdout,
                SetForegroundColor(cell_fg),
                SetBackgroundColor(cell_bg),
                Print(ch),
            )?;
        }
        Ok(())
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
    
    pub fn top_line_index(&self) -> usize {
        self.top_line_index
    }
}

#[cfg(test)]
mod tests {
    use super::{Screen, Selection, SelectionArea};

    #[test]
    fn line_selection_is_normalized() {
        let mut screen = Screen::new().expect("screen");
        screen.set_selection(Some(Selection::Lines {
            start_line: 8,
            end_line: 3,
        }));

        assert!(screen.line_is_selected(3));
        assert!(screen.line_is_selected(8));
        assert!(!screen.line_is_selected(2));
        assert!(!screen.line_is_selected(9));
    }

    #[test]
    fn data_selection_spans_multiple_lines() {
        let mut screen = Screen::new().expect("screen");
        screen.horizontal_offset = 0;
        screen.set_selection(Some(Selection::Text {
            area: SelectionArea::Data,
            start_line: 4,
            start_col: 5,
            end_line: 6,
            end_col: 2,
        }));

        assert_eq!(screen.visible_data_selection_range(4, 12), Some((5, screen.data_width())));
        assert_eq!(screen.visible_data_selection_range(5, 12), Some((0, screen.data_width())));
        assert_eq!(screen.visible_data_selection_range(6, 12), Some((0, 3)));
        assert_eq!(screen.visible_data_selection_range(3, 12), None);
    }

    #[test]
    fn command_selection_uses_character_range() {
        let selection = Selection::Text {
            area: SelectionArea::CommandLine,
            start_line: 0,
            start_col: 7,
            end_line: 0,
            end_col: 2,
        };

        assert_eq!(selection.command_range(), Some((2, 8)));
    }
}
