//! Editor: the main coordinator that ties together buffer, screen, and input.
//!
//! Owns the event loop (`run`).  Each iteration:
//!   1. `event::read()` — blocks until a key/resize event.
//!   2. `InputHandler::handle_event` → `EditorAction`.
//!   3. `handle_action` dispatches to the appropriate handler.
//!   4. Data edits are applied directly to the buffer.
//!   5. On Enter: execute line cmds → execute primary command → renumber → redraw.
//!
//! ## Cursor Model
//!
//! The cursor's position is tracked by:
//!   - `input.focus`: which field (CommandLine, ScrollField, PrefixArea, DataArea)
//!   - `screen_row` (in the focus enum): which terminal row
//!   - `cursor_col`: column offset within the focused field
//!   - `cursor_line_index`: buffer index corresponding to current screen row
//!
//! For the command line, `screen.command_cursor_pos` is used instead of
//! `cursor_col` — this is a design inconsistency.
//! TODO: unify cursor column tracking.
//!
//! ## Known Issues
//!
//! - `move_cursor_left` decrements both `cursor_col` AND `command_cursor_pos`
//!   when focus is CommandLine — double-decrement bug.  The first `if` block
//!   decrements `cursor_col` (which is unused for command line), and then the
//!   second `if` block also decrements `command_cursor_pos`.  This makes Left
//!   arrow appear to work but `cursor_col` drifts out of sync.
//!   FIXME: the method should `return` or skip the second block for CommandLine.
//! - `handle_char` for DataArea uses `line.data.len()` (byte length) to
//!   extend with spaces, and does char-vec indexing.  This assumes ASCII.
//!   Multi-byte chars will cause misalignment.
//!   TODO: switch to char-based column model.
//! - Full screen redraw on every action (`redraw` calls `draw_full`).
//!   See `screen.rs` TODOs.
//! - CANCEL confirmation (Y/N prompt) is not implemented — it just exits.

use std::collections::VecDeque;
use std::io::{self, Write};

use crossterm::{cursor::MoveTo, event, queue};
use tracing::{debug, info, trace, warn};

use crate::buffer::FileBuffer;
use crate::command::{self, PrimaryCommand};
use crate::input::{EditorAction, InputHandler};
use crate::line::{Line, LineFlags};
use crate::line_cmd;
use crate::line_store::LineStore;
use crate::screen::Screen;
use crate::types::*;

pub struct Editor {
    pub buffer: FileBuffer,
    pub screen: Screen,
    pub input: InputHandler,
    pub running: bool,
    pub last_find: Option<String>,
    pub command_history: VecDeque<String>,
    pub history_index: Option<usize>,

    // Cursor tracking
    pub cursor_line_index: usize, // Buffer line index cursor is on
    pub cursor_col: usize,        // Column within the current field
}

impl Editor {
    pub fn new(buffer: FileBuffer) -> io::Result<Self> {
        let screen = Screen::new()?;
        Ok(Editor {
            buffer,
            screen,
            input: InputHandler::new(),
            running: true,
            last_find: None,
            command_history: VecDeque::with_capacity(COMMAND_HISTORY_SIZE),
            history_index: None,

            cursor_line_index: 1, // First data line (after TopOfData)
            cursor_col: 0,
        })
    }

    /// Main loop: read event → process → draw.
    pub fn run<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        info!(
            "Editor::run — entering event loop, {} lines in buffer",
            self.buffer.line_count()
        );
        // Initial draw
        self.screen.draw_full(stdout, &self.buffer)?;
        self.position_cursor(stdout)?;

        while self.running {
            let event = event::read()?;
            let action = self.input.handle_event(event);
            self.handle_action(action, stdout)?;
        }

        info!(
            "Editor::run — event loop ended, modified={}",
            self.buffer.modified
        );
        Ok(())
    }

    fn handle_action<W: Write>(&mut self, action: EditorAction, stdout: &mut W) -> io::Result<()> {
        match action {
            EditorAction::None => {}

            EditorAction::InsertChar(c) => {
                trace!(
                    "Action: InsertChar({:?}) focus={:?} col={}",
                    c, self.input.focus, self.cursor_col
                );

                // self.screen.draw_char(stdout, self.cursor_col as u16, self.cursor_line_index as u16, c).unwrap();
                self.handle_char(c);
                self.redraw(stdout)?;
            }

            EditorAction::DeleteChar => {
                trace!("Action: DeleteChar focus={:?}", self.input.focus);
                self.handle_delete();
                self.redraw(stdout)?;
            }

            EditorAction::Backspace => {
                trace!("Action: Backspace focus={:?}", self.input.focus);
                self.handle_backspace();
                self.redraw(stdout)?;
            }

            EditorAction::CursorUp => {
                self.move_cursor_up();
                self.redraw(stdout)?;
            }

            EditorAction::CursorDown => {
                self.move_cursor_down();
                self.redraw(stdout)?;
            }

            EditorAction::CursorLeft => {
                self.move_cursor_left();
                self.position_cursor(stdout)?;
            }

            EditorAction::CursorRight => {
                self.move_cursor_right();
                self.position_cursor(stdout)?;
            }

            EditorAction::CursorHome => {
                self.handle_home();
                self.position_cursor(stdout)?;
            }

            EditorAction::CursorEnd => {
                self.handle_end();
                self.position_cursor(stdout)?;
            }

            EditorAction::Tab => {
                self.cycle_focus_forward();
                self.redraw(stdout)?;
            }

            EditorAction::BackTab => {
                self.cycle_focus_backward();
                self.redraw(stdout)?;
            }

            EditorAction::Enter => {
                debug!("Action: Enter — processing commands");
                self.handle_enter();
                self.redraw(stdout)?;
            }

            EditorAction::ToggleInsertMode => {
                self.input.mode = match self.input.mode {
                    InputMode::Overtype => InputMode::Insert,
                    InputMode::Insert => InputMode::Overtype,
                };
                debug!("Mode toggled to {:?}", self.input.mode);
            }

            EditorAction::FnEnd => {
                debug!("Action: FnEnd (F3/Esc)");
                self.handle_primary_command_direct(PrimaryCommand::End, stdout)?;
            }

            EditorAction::FnRFind => {
                debug!("Action: FnRFind (F5)");
                self.handle_primary_command_direct(PrimaryCommand::RFind, stdout)?;
            }

            EditorAction::FnScrollUp => {
                let amount = self.screen.scroll_amount.clone();
                let lines = amount.resolve(self.screen.data_rows(), self.cursor_screen_row());
                debug!("Action: ScrollUp by {lines}");
                self.screen.scroll_up(lines);
                self.clamp_cursor();
                self.redraw(stdout)?;
            }

            EditorAction::FnScrollDown => {
                let amount = self.screen.scroll_amount.clone();
                let lines = amount.resolve(self.screen.data_rows(), self.cursor_screen_row());
                let max = self.buffer.line_count().saturating_sub(1);
                debug!("Action: ScrollDown by {lines}");
                self.screen.scroll_down(lines, max);
                self.clamp_cursor();
                self.redraw(stdout)?;
            }

            EditorAction::FnScrollLeft => {
                self.screen.scroll_left(self.screen.data_width());
                self.redraw(stdout)?;
            }

            EditorAction::FnScrollRight => {
                self.screen.scroll_right(self.screen.data_width());
                self.redraw(stdout)?;
            }

            EditorAction::FnRetrieve => {
                self.retrieve_command();
                self.redraw(stdout)?;
            }

            EditorAction::Resize(w, h) => {
                info!("Terminal resized to {w}x{h}");
                self.screen.resize(w, h);
                self.clamp_cursor();
                self.screen.draw_full(stdout, &self.buffer)?;
                self.position_cursor(stdout)?;
            }

            EditorAction::ForceQuit => {
                warn!("ForceQuit (Ctrl+Q) — exiting without save");
                self.running = false;
            }
        }

        Ok(())
    }

    // --- Character handling ---

    fn handle_char(&mut self, c: char) {
        let c = if self.buffer.caps_mode {
            c.to_uppercase().next().unwrap_or(c)
        } else {
            c
        };

        match self.input.focus {
            FieldFocus::CommandLine => {
                if self.input.mode == InputMode::Insert
                    || self.screen.command_cursor_pos >= self.screen.command_line.len()
                {
                    self.screen
                        .command_line
                        .insert(self.screen.command_cursor_pos, c);
                } else {
                    // Overtype
                    let mut chars: Vec<char> = self.screen.command_line.chars().collect();
                    if self.screen.command_cursor_pos < chars.len() {
                        chars[self.screen.command_cursor_pos] = c;
                    } else {
                        chars.push(c);
                    }
                    self.screen.command_line = chars.into_iter().collect();
                }
                self.screen.command_cursor_pos += 1;
            }

            FieldFocus::ScrollField => {
                // Edit scroll field
                if self.input.mode == InputMode::Insert
                    || self.cursor_col >= self.screen.scroll_field_text.len()
                {
                    self.screen.scroll_field_text.insert(self.cursor_col, c);
                } else {
                    let mut chars: Vec<char> = self.screen.scroll_field_text.chars().collect();
                    if self.cursor_col < chars.len() {
                        chars[self.cursor_col] = c;
                    } else {
                        chars.push(c);
                    }
                    self.screen.scroll_field_text = chars.into_iter().collect();
                }
                self.cursor_col += 1;
            }

            FieldFocus::PrefixArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);

                if self.cursor_col < PREFIX_WIDTH {
                    if let Some(line) = self.buffer.lines.get_mut(line_index) {
                        let mut cmd = line.prefix_cmd.clone().unwrap_or_default();
                        if self.cursor_col < cmd.len() {
                            let mut chars: Vec<char> = cmd.chars().collect();
                            if self.input.mode == InputMode::Insert {
                                chars.insert(self.cursor_col, c);
                            } else {
                                chars[self.cursor_col] = c;
                            }
                            cmd = chars.into_iter().collect();
                        } else {
                            cmd.push(c);
                        }
                        line.prefix_cmd = Some(cmd);
                        line.flags.set(LineFlags::PENDING_CMD);
                    }
                    self.cursor_col = (self.cursor_col + 1).min(PREFIX_WIDTH - 1);
                }
            }

            FieldFocus::DataArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);

                // Check if line is editable
                if self.buffer.browse_mode {
                    return;
                }
                if let Some(line) = self.buffer.lines.get(line_index) {
                    if !line.is_writable() {
                        return;
                    }
                } else {
                    return;
                }

                let actual_col = self.screen.horizontal_offset + self.cursor_col;

                let mut data = self.buffer
                    .lines
                    .get(line_index)
                    .map(|l| l.data.clone())
                    .unwrap_or_default();

                // Extend with spaces if needed
                while data.len() <= actual_col {
                    data.push(' ');
                }

                let mut chars: Vec<char> = data.chars().collect();
                if self.input.mode == InputMode::Insert {
                    chars.insert(actual_col, c);
                } else {
                    chars[actual_col] = c;
                }
                let new_data: String = chars.into_iter().collect();
                self.buffer.update_line_data(line_index, new_data);
                self.cursor_col += 1;
            }
        }
    }

    fn handle_delete(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                if self.screen.command_cursor_pos < self.screen.command_line.len() {
                    self.screen
                        .command_line
                        .remove(self.screen.command_cursor_pos);
                }
            }
            FieldFocus::ScrollField => {
                if self.cursor_col < self.screen.scroll_field_text.len() {
                    self.screen.scroll_field_text.remove(self.cursor_col);
                }
            }
            FieldFocus::PrefixArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                if let Some(line) = self.buffer.lines.get_mut(line_index) {
                    if let Some(ref mut cmd) = line.prefix_cmd {
                        if self.cursor_col < cmd.len() {
                            cmd.remove(self.cursor_col);
                        }
                        if cmd.is_empty() {
                            line.prefix_cmd = None;
                            line.flags.clear(LineFlags::PENDING_CMD);
                        }
                    }
                }
            }
            FieldFocus::DataArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                let actual_col = self.screen.horizontal_offset + self.cursor_col;
                let mut data = self.buffer
                    .lines
                    .get(line_index)
                    .map(|l| l.data.clone())
                    .unwrap_or_default();
                if actual_col < data.len() {
                    data.remove(actual_col);
                    self.buffer.update_line_data(line_index, data);
                }
            }
        }
    }

    fn handle_backspace(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                if self.screen.command_cursor_pos > 0 {
                    self.screen.command_cursor_pos -= 1;
                    self.screen
                        .command_line
                        .remove(self.screen.command_cursor_pos);
                }
            }
            FieldFocus::ScrollField => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                    self.screen.scroll_field_text.remove(self.cursor_col);
                }
            }
            FieldFocus::PrefixArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                if let Some(line) = self.buffer.lines.get_mut(line_index) {
                    if self.cursor_col > 0 {
                        self.cursor_col -= 1;
                        if let Some(ref mut cmd) = line.prefix_cmd {
                            if self.cursor_col < cmd.len() {
                                cmd.remove(self.cursor_col);
                            }
                            if cmd.is_empty() {
                                line.prefix_cmd = None;
                                line.flags.clear(LineFlags::PENDING_CMD);
                            }
                        }
                    }
                }
            }
            FieldFocus::DataArea { screen_row } => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                    let line_index = self.screen.screen_row_to_line(screen_row);
                    let actual_col = self.screen.horizontal_offset + self.cursor_col;
                    let mut data = self.buffer
                        .lines
                        .get(line_index)
                        .map(|l| l.data.clone())
                        .unwrap_or_default();
                    if actual_col < data.len() {
                        data.remove(actual_col);
                        self.buffer.update_line_data(line_index, data);
                    }
                }
            }
        }
    }

    // --- Cursor movement ---

    fn move_cursor_up(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine | FieldFocus::ScrollField => {
                // Move to last data line's data area
                let last_row = (HEADER_ROWS as usize + self.screen.data_rows() - 1) as u16;
                self.input.focus = FieldFocus::DataArea {
                    screen_row: last_row,
                };
                self.cursor_col = 0;
                self.update_cursor_line_index();
            }
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                if screen_row > HEADER_ROWS {
                    let new_row = screen_row - 1;
                    self.input.focus = match self.input.focus {
                        FieldFocus::PrefixArea { .. } => FieldFocus::PrefixArea {
                            screen_row: new_row,
                        },
                        _ => FieldFocus::DataArea {
                            screen_row: new_row,
                        },
                    };
                    self.update_cursor_line_index();
                } else {
                    // At top of visible area — scroll up
                    if self.screen.top_line_index > 0 {
                        self.screen.scroll_up(1);
                    } else {
                        // Move to command line
                        self.input.focus = FieldFocus::CommandLine;
                        self.cursor_col = self.screen.command_cursor_pos;
                    }
                }
            }
        }
    }

    fn move_cursor_down(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine | FieldFocus::ScrollField => {
                self.input.focus = FieldFocus::DataArea {
                    screen_row: HEADER_ROWS,
                };
                self.cursor_col = 0;
                self.update_cursor_line_index();
            }
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                let max_row = (HEADER_ROWS as usize + self.screen.data_rows() - 1) as u16;
                if screen_row < max_row {
                    let line_below = self.screen.screen_row_to_line(screen_row + 1);
                    if line_below < self.buffer.line_count() {
                        let new_row = screen_row + 1;
                        self.input.focus = match self.input.focus {
                            FieldFocus::PrefixArea { .. } => FieldFocus::PrefixArea {
                                screen_row: new_row,
                            },
                            _ => FieldFocus::DataArea {
                                screen_row: new_row,
                            },
                        };
                        self.update_cursor_line_index();
                    }
                } else {
                    // At bottom of visible area — scroll down
                    let max = self.buffer.line_count().saturating_sub(1);
                    self.screen.scroll_down(1, max);
                }
            }
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else {
            match self.input.focus {
                FieldFocus::CommandLine => {
                    if self.screen.command_cursor_pos > 0 {
                        self.screen.command_cursor_pos -= 1;
                    }
                }
                _ => {}
            }
        }
        if matches!(self.input.focus, FieldFocus::CommandLine) {
            if self.screen.command_cursor_pos > 0 {
                self.screen.command_cursor_pos -= 1;
            }
        }
    }

    fn move_cursor_right(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                if self.screen.command_cursor_pos < self.screen.command_line.len() {
                    self.screen.command_cursor_pos += 1;
                }
            }
            FieldFocus::ScrollField => {
                if self.cursor_col < self.screen.scroll_field_text.len() {
                    self.cursor_col += 1;
                }
            }
            FieldFocus::PrefixArea { .. } => {
                if self.cursor_col < PREFIX_WIDTH - 1 {
                    self.cursor_col += 1;
                }
            }
            FieldFocus::DataArea { .. } => {
                if self.cursor_col < self.screen.data_width() - 1 {
                    self.cursor_col += 1;
                }
            }
        }
    }

    fn handle_home(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                self.screen.command_cursor_pos = 0;
            }
            _ => {
                self.cursor_col = 0;
            }
        }
    }

    fn handle_end(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                self.screen.command_cursor_pos = self.screen.command_line.len();
            }
            FieldFocus::DataArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                if let Some(line) = self.buffer.lines.get(line_index) {
                    let data_len = line
                        .data
                        .len()
                        .saturating_sub(self.screen.horizontal_offset);
                    self.cursor_col = data_len.min(self.screen.data_width() - 1);
                }
            }
            _ => {}
        }
    }

    // --- Focus cycling ---

    fn cycle_focus_forward(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                self.input.focus = FieldFocus::DataArea {
                    screen_row: HEADER_ROWS,
                };
                self.cursor_col = 0;
                self.update_cursor_line_index();
            }
            FieldFocus::ScrollField => {
                self.input.focus = FieldFocus::CommandLine;
                self.cursor_col = 0;
                self.screen.command_cursor_pos = 0;
            }
            FieldFocus::PrefixArea { screen_row } => {
                self.input.focus = FieldFocus::DataArea { screen_row };
                self.cursor_col = 0;
            }
            FieldFocus::DataArea { screen_row } => {
                let max_row = (HEADER_ROWS as usize + self.screen.data_rows() - 1) as u16;
                if screen_row < max_row {
                    self.input.focus = FieldFocus::PrefixArea {
                        screen_row: screen_row + 1,
                    };
                    self.cursor_col = 0;
                } else {
                    self.input.focus = FieldFocus::CommandLine;
                    self.cursor_col = 0;
                    self.screen.command_cursor_pos = 0;
                }
            }
        }
    }

    fn cycle_focus_backward(&mut self) {
        match self.input.focus {
            FieldFocus::CommandLine => {
                let max_row = (HEADER_ROWS as usize + self.screen.data_rows() - 1) as u16;
                self.input.focus = FieldFocus::DataArea {
                    screen_row: max_row,
                };
                self.cursor_col = 0;
                self.update_cursor_line_index();
            }
            FieldFocus::ScrollField => {
                self.input.focus = FieldFocus::CommandLine;
                self.cursor_col = 0;
            }
            FieldFocus::PrefixArea { screen_row } => {
                if screen_row > HEADER_ROWS {
                    self.input.focus = FieldFocus::DataArea {
                        screen_row: screen_row - 1,
                    };
                    self.cursor_col = 0;
                } else {
                    self.input.focus = FieldFocus::CommandLine;
                    self.cursor_col = 0;
                    self.screen.command_cursor_pos = 0;
                }
            }
            FieldFocus::DataArea { screen_row } => {
                self.input.focus = FieldFocus::PrefixArea { screen_row };
                self.cursor_col = 0;
            }
        }
    }

    fn update_cursor_line_index(&mut self) {
        trace!("screen row: {}", self.cursor_screen_row());
        match self.input.focus {
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                self.cursor_line_index = self.screen.screen_row_to_line(screen_row);
                trace!("cursor line index: {}", self.cursor_line_index);
            }
            _ => {}
        }
    }

    fn cursor_screen_row(&self) -> usize {
        match self.input.focus {
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                (screen_row - HEADER_ROWS) as usize
            }
            _ => 0,
        }
    }

    fn clamp_cursor(&mut self) {
        // Ensure cursor is within valid data area after scrolling
        match self.input.focus {
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                if line_index >= self.buffer.line_count() {
                    // Move cursor to last valid line
                    let last_valid = self.buffer.line_count().saturating_sub(1);
                    if let Some(row) = self.screen.line_to_screen_row(last_valid) {
                        self.input.focus = match self.input.focus {
                            FieldFocus::PrefixArea { .. } => {
                                FieldFocus::PrefixArea { screen_row: row }
                            }
                            _ => FieldFocus::DataArea { screen_row: row },
                        };
                    }
                }
                self.update_cursor_line_index();
            }
            _ => {}
        }
    }

    // --- Enter processing ---

    fn handle_enter(&mut self) {
        // 1. Prefix commands are already set on buffer lines directly.
        //    Uppercase them, and clear empty ones (restore line number display).
        for i in 0..self.buffer.lines.len() {
            if let Some(line) = self.buffer.lines.get_mut(i) {
                if let Some(ref cmd) = line.prefix_cmd {
                    let trimmed = cmd.trim().to_uppercase();
                    if trimmed.is_empty() {
                        line.prefix_cmd = None;
                        line.flags.clear(LineFlags::PENDING_CMD);
                        line.flags.clear(LineFlags::CMD_ERROR);
                    } else {
                        line.prefix_cmd = Some(trimmed);
                    }
                }
            }
        }

        // 2. Execute line commands
        debug!("Enter: executing line commands");
        let lcmd_result = line_cmd::execute_line_commands(&mut self.buffer);
        if let Some(ref error) = lcmd_result.error {
            warn!("Line command error: {error}");
            self.screen.message = Some(Message {
                text: error.clone(),
                msg_type: MessageType::Error,
            });
        }

        // 3. Parse and execute primary command
        let cmd_text = self.screen.command_line.trim().to_string();
        if !cmd_text.is_empty() {
            info!("Primary command: {:?}", cmd_text);
            // Add to history
            self.push_command_history(cmd_text.clone());

            match command::parse_command(&cmd_text) {
                Ok(cmd) => {
                    debug!("  parsed as: {:?}", cmd);
                    let page_size = self.screen.data_rows();
                    let cursor_row = self.cursor_screen_row();
                    let scroll_amount = self.screen.scroll_amount.clone();
                    let result = command::execute_command(
                        &cmd,
                        &mut self.buffer,
                        &mut self.last_find,
                        self.cursor_line_index,
                        self.cursor_col,
                        page_size,
                        cursor_row,
                        &scroll_amount,
                    );

                    self.apply_command_result(result);
                }
                Err(msg) => {
                    if !msg.is_empty() {
                        warn!("Command parse error: {msg}");
                        self.screen.message = Some(Message {
                            text: msg,
                            msg_type: MessageType::Error,
                        });
                    }
                }
            }
        }

        // 4. Clear command line
        self.screen.command_line.clear();
        self.screen.command_cursor_pos = 0;
        self.history_index = None;

        // 5. Parse scroll field
        self.screen.parse_scroll_field();

        // 6. If on data area, advance cursor to next line
        match self.input.focus {
            FieldFocus::DataArea { screen_row } => {
                let max_row = (HEADER_ROWS as usize + self.screen.data_rows() - 1) as u16;
                if screen_row < max_row {
                    let line_below = self.screen.screen_row_to_line(screen_row + 1);
                    if line_below < self.buffer.line_count() {
                        self.input.focus = FieldFocus::DataArea {
                            screen_row: screen_row + 1,
                        };
                        self.cursor_col = 0;
                        self.update_cursor_line_index();
                    }
                }
            }
            FieldFocus::PrefixArea { screen_row } => {
                // After entering prefix cmd, move to data area
                self.input.focus = FieldFocus::DataArea { screen_row };
                self.cursor_col = 0;
            }
            FieldFocus::CommandLine => {
                // Stay on command line after executing command
            }
            _ => {}
        }
    }

    fn apply_command_result(&mut self, result: command::CommandResult) {
        if let Some(ref msg) = result.message {
            debug!("Command result message: [{:?}] {}", msg.msg_type, msg.text);
        }
        if let Some(msg) = result.message {
            self.screen.message = Some(msg);
        }

        if result.should_exit {
            info!("Command requests exit");
            self.running = false;
            return;
        }

        if result.needs_save_prompt {
            info!("CANCEL with unsaved changes — exiting (prompt not implemented)");
            // For now, just exit without saving on CANCEL
            // A proper implementation would show a Y/N prompt
            self.running = false;
            return;
        }

        if let Some(line_idx) = result.scroll_to {
            let max = self.buffer.line_count().saturating_sub(1);
            self.screen.scroll_to_line(line_idx, max);
        }

        if let Some((line_idx, col)) = result.cursor_to {
            self.cursor_line_index = line_idx;
            let max = self.buffer.line_count().saturating_sub(1);
            self.screen.ensure_visible(line_idx, max);
            if let Some(row) = self.screen.line_to_screen_row(line_idx) {
                self.input.focus = FieldFocus::DataArea { screen_row: row };
                self.cursor_col = col.saturating_sub(self.screen.horizontal_offset);
            }
        }

        if let Some(lines) = result.scroll_up {
            self.screen.scroll_up(lines);
            self.clamp_cursor();
        }

        if let Some(lines) = result.scroll_down {
            let max = self.buffer.line_count().saturating_sub(1);
            self.screen.scroll_down(lines, max);
            self.clamp_cursor();
        }

        if let Some(cols) = result.scroll_left {
            self.screen.scroll_left(cols);
        }

        if let Some(cols) = result.scroll_right {
            self.screen.scroll_right(cols);
        }

        if result.toggle_cols {
            self.screen.cols_visible = !self.screen.cols_visible;
            // Insert or remove COLS ruler line after TopOfData
            if self.screen.cols_visible {
                self.buffer.lines.insert(1, Line::cols_ruler());
            } else {
                // Find and remove COLS ruler
                for i in 0..self.buffer.lines.len() {
                    if let Some(line) = self.buffer.lines.get(i) {
                        if line.line_type == crate::line::LineType::ColsRuler {
                            self.buffer.lines.remove(i);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn handle_primary_command_direct<W: Write>(
        &mut self,
        cmd: PrimaryCommand,
        stdout: &mut W,
    ) -> io::Result<()> {
        let page_size = self.screen.data_rows();
        let cursor_row = self.cursor_screen_row();
        let scroll_amount = self.screen.scroll_amount.clone();
        let result = command::execute_command(
            &cmd,
            &mut self.buffer,
            &mut self.last_find,
            self.cursor_line_index,
            self.cursor_col,
            page_size,
            cursor_row,
            &scroll_amount,
        );

        self.apply_command_result(result);
        if self.running {
            self.redraw(stdout)?;
        }
        Ok(())
    }

    // --- Command history ---

    fn push_command_history(&mut self, cmd: String) {
        if self.command_history.len() >= COMMAND_HISTORY_SIZE {
            self.command_history.pop_back();
        }
        self.command_history.push_front(cmd);
    }

    fn retrieve_command(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let idx = match self.history_index {
            Some(i) => {
                if i + 1 < self.command_history.len() {
                    i + 1
                } else {
                    0
                }
            }
            None => 0,
        };

        if let Some(cmd) = self.command_history.get(idx) {
            self.screen.command_line = cmd.clone();
            self.screen.command_cursor_pos = cmd.len();
            self.history_index = Some(idx);
            self.input.focus = FieldFocus::CommandLine;
        }
    }

    // --- Drawing ---

    fn redraw<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        self.screen.draw_full(stdout, &self.buffer)?;
        self.position_cursor(stdout)?;
        Ok(())
    }

    fn redraw_line<W: Write>(&mut self, stdout: &mut W, line_index: usize) -> io::Result<()> {
        self.screen.draw_line(stdout, line_index, &self.buffer)
    }

    fn position_cursor<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        let (col, row) = match self.input.focus {
            FieldFocus::CommandLine => (
                self.screen.command_input_col() + self.screen.command_cursor_pos as u16,
                1,
            ),
            FieldFocus::ScrollField => {
                // Scroll field starts after "  Scroll ===> " on the command line
                let prompt_len = 13; // "Command ===> "
                let scroll_label = "  Scroll ===> ";
                let cmd_input_width = (self.screen.width as usize)
                    .saturating_sub(prompt_len + scroll_label.len() + 5);
                let scroll_col = prompt_len + cmd_input_width + scroll_label.len();
                (scroll_col as u16 + self.cursor_col as u16, 1)
            }
            FieldFocus::PrefixArea { screen_row } => (self.cursor_col as u16, screen_row),
            FieldFocus::DataArea { screen_row } => (
                self.screen.data_start_col() + self.cursor_col as u16,
                screen_row,
            ),
        };

        queue!(stdout, MoveTo(col, row))?;
        stdout.flush()?;
        Ok(())
    }
}
