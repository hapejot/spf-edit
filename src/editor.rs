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
use std::time::Duration;

use crossterm::{cursor::MoveTo, event, event::MouseButton, event::MouseEventKind, queue};
use tracing::{debug, info, trace, warn};

use crate::buffer::FileBuffer;
use crate::command::{self, PrimaryCommand};
use crate::input::{EditorAction, InputHandler, SpfEvent, SpfMouseEvent};
use crate::line::{Line, LineFlags};
use crate::line_cmd;
use crate::line_store::LineStore;
use crate::panel::PanelManager;
use crate::prefix::{ParsedLineCmd, PrefixParseResult, parse_prefix_command};
use crate::rust_analyzer::{CompletionItem, LspEvent, RustAnalyzerClient};
use crate::screen::{CompletionPopup, Screen, Selection, SelectionArea};
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerTarget {
    CommandLine { col: usize },
    ScrollField { col: usize },
    PrefixArea { line_index: usize, screen_row: u16 },
    DataArea {
        line_index: usize,
        screen_row: u16,
        col: usize,
    },
}

#[derive(Debug, Clone)]
struct CompletionMenu {
    anchor_row: u16,
    anchor_col: u16,
    selected: usize,
    items: Vec<CompletionItem>,
}

#[derive(Debug, Clone)]
struct PendingCompletion {
    request_id: u64,
    version: i32,
    anchor_row: u16,
    anchor_col: u16,
    req_line: u32,
    req_char_utf16: u32,
    req_prefix: String,
    req_token: String,
}

pub struct Editor {
    pub buffer: FileBuffer,
    pub screen: Screen,
    pub input: InputHandler,
    pub panel_manager: Option<PanelManager>,
    pub running: bool,
    pub last_find: Option<String>,
    pub command_history: VecDeque<String>,
    pub history_index: Option<usize>,
    mouse_drag_anchor: Option<PointerTarget>,
    lsp: Option<RustAnalyzerClient>,
    lsp_startup_error: Option<String>,
    completion_menu: Option<CompletionMenu>,
    pending_completion: Option<PendingCompletion>,

    // Cursor tracking
    pub cursor_line_index: usize, // Buffer line index cursor is on
    pub cursor_col: usize,
    needs_full_redraw: bool,       // Column within the current field
    pending_panel: Option<String>, // Panel to display after command processing
}

impl Editor {
    pub fn new(buffer: FileBuffer) -> io::Result<Self> {
        let screen = Screen::new()?;

        // Try to load panel manager from panels/ directory
        let panels_dir = std::path::Path::new("panels");
        let panel_manager = if panels_dir.is_dir() {
            PanelManager::new(panels_dir).ok()
        } else {
            None
        };

        // Restore the user's Enter-key preference from the SPFSETS profile.
        let enter_mode = panel_manager
            .as_ref()
            .and_then(|pm| pm.vars().profile_get("SPFSETS", "ZENTRKEY"))
            .map(EnterMode::from_profile)
            .unwrap_or(EnterMode::Legacy);

        let mut input = InputHandler::new();
        input.enter_mode = enter_mode;

        let mut lsp = None;
        let mut lsp_startup_error = None;
        if buffer
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("rs"))
            .unwrap_or(false)
        {
            match RustAnalyzerClient::start(&buffer.file_path) {
                Ok(client) => {
                    lsp = Some(client);
                }
                Err(err) => {
                    warn!("rust-analyzer unavailable: {err}");
                    lsp_startup_error = Some(err.to_string());
                }
            }
        }

        let mut editor = Editor {
            buffer,
            screen,
            input,
            panel_manager,
            running: true,
            last_find: None,
            command_history: VecDeque::with_capacity(COMMAND_HISTORY_SIZE),
            history_index: None,
            mouse_drag_anchor: None,
            lsp,
            lsp_startup_error,
            completion_menu: None,
            pending_completion: None,
            needs_full_redraw: true,
            cursor_line_index: 1, // First data line (after TopOfData)
            cursor_col: 0,
            pending_panel: None,
        };

        editor.send_did_open_if_available();
        if let Some(err) = editor.lsp_startup_error.clone() {
            editor.screen.message = Some(Message {
                text: format!("rust-analyzer unavailable: {err}"),
                msg_type: MessageType::Error,
            });
        }

        Ok(editor)
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
            self.drain_lsp_events();
            if self.needs_full_redraw {
                self.redraw(stdout)?;
            }

            if !event::poll(Duration::from_millis(40))? {
                continue;
            }

            let event = event::read()?;
            let evt_str = format!("{event:?}");
            self.screen.status_info = evt_str;
            let spf_event = self.input.translate_event(event);
            self.handle_spf_event(spf_event, stdout)?;
        }

        info!(
            "Editor::run — event loop ended, modified={}",
            self.buffer.modified
        );
        Ok(())
    }

    fn handle_action<W: Write>(&mut self, action: EditorAction, stdout: &mut W) -> io::Result<()> {
        match action {
            EditorAction::InsertChar(c) => {
                trace!(
                    "Action: InsertChar({:?}) focus={:?} col={}",
                    c, self.input.focus, self.cursor_col
                );
                self.handle_char(c);
            }
            EditorAction::DeleteChar => {
                trace!("Action: DeleteChar focus={:?}", self.input.focus);
                self.handle_delete();
            }
            EditorAction::Backspace => {
                trace!("Action: Backspace focus={:?}", self.input.focus);
                self.handle_backspace();
            }

            EditorAction::CursorUp => self.move_cursor_up(),
            EditorAction::CursorDown => self.move_cursor_down(),
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

            EditorAction::Tab => self.cycle_focus_forward(),
            EditorAction::BackTab => self.cycle_focus_backward(),

            EditorAction::Newline => {
                debug!("Action: Newline (regular Enter)");
                self.handle_newline();
            }
            EditorAction::Enter => {
                debug!("Action: Enter — processing commands");
                self.handle_enter();
                self.maybe_display_pending_panel(stdout)?;
            }

            EditorAction::ToggleInsertMode => {
                self.input.mode = match self.input.mode {
                    InputMode::Overtype => InputMode::Insert,
                    InputMode::Insert => InputMode::Overtype,
                };
                debug!("Mode toggled to {:?}", self.input.mode);
            }

            EditorAction::FnScrollUp => self.scroll_page_up(),
            EditorAction::FnScrollDown => self.scroll_page_down(),
            EditorAction::FnScrollLeft => {
                self.screen.scroll_left(self.screen.data_width());
                self.needs_full_redraw = true;
            }
            EditorAction::FnScrollRight => {
                self.screen.scroll_right(self.screen.data_width());
                self.needs_full_redraw = true;
            }

            EditorAction::FnRetrieve => self.retrieve_command(),
            EditorAction::TriggerCompletion => {
                self.trigger_completion();
                self.needs_full_redraw = true;
            }
            EditorAction::CompletionNext => {
                self.completion_next();
                self.needs_full_redraw = true;
            }
            EditorAction::CompletionPrev => {
                self.completion_prev();
                self.needs_full_redraw = true;
            }
            EditorAction::CompletionAccept => {
                self.accept_completion();
                self.needs_full_redraw = true;
            }
            EditorAction::CompletionCancel => {
                self.cancel_completion();
                self.needs_full_redraw = true;
            }

            EditorAction::Resize(w, h) => {
                info!("Terminal resized to {w}x{h}");
                self.screen.resize(w, h);
                self.clamp_cursor();
                self.needs_full_redraw = true;
                self.screen.draw_full(stdout, &self.buffer)?;
                self.position_cursor(stdout)?;
            }

            EditorAction::ForceQuit => {
                warn!("ForceQuit (Ctrl+Q) — exiting without save");
                self.running = false;
            }
        }

        self.redraw(stdout)
    }

    fn handle_spf_event<W: Write>(&mut self, event: SpfEvent, stdout: &mut W) -> io::Result<()> {
        match event {
            SpfEvent::Action(action) => self.handle_action(action, stdout),
            SpfEvent::Command(command) => self.handle_primary_command_direct(command, stdout),
            SpfEvent::FocusChanged { focused } => self.handle_focus_changed(focused, stdout),
            SpfEvent::Mouse(mouse) => self.handle_mouse_event(mouse, stdout),
            SpfEvent::Sequence(events) => {
                for event in events {
                    self.handle_spf_event(event, stdout)?;
                    if !self.running {
                        break;
                    }
                }
                Ok(())
            }
            SpfEvent::Input(input) => {
                trace!("Unhandled SPF input event: {:?}", input);
                Ok(())
            }
            SpfEvent::Ignore => Ok(()),
        }
    }

    fn handle_focus_changed<W: Write>(&mut self, focused: bool, stdout: &mut W) -> io::Result<()> {
        self.screen.status_info = if focused {
            " focus gained".to_string()
        } else {
            " focus lost".to_string()
        };
        self.redraw(stdout)
    }

    fn handle_mouse_event<W: Write>(
        &mut self,
        mouse: SpfMouseEvent,
        stdout: &mut W,
    ) -> io::Result<()> {
        self.screen.status_info = format!(" mouse {:?} @ {},{}", mouse.kind, mouse.column, mouse.row);

        match mouse.kind {
            MouseEventKind::ScrollUp => {
                self.scroll_page_up();
                self.redraw(stdout)
            }
            MouseEventKind::ScrollDown => {
                self.scroll_page_down();
                self.redraw(stdout)
            }
            MouseEventKind::ScrollLeft => {
                self.screen.scroll_left(self.screen.data_width());
                self.needs_full_redraw = true;
                self.redraw(stdout)
            }
            MouseEventKind::ScrollRight => {
                self.screen.scroll_right(self.screen.data_width());
                self.needs_full_redraw = true;
                self.redraw(stdout)
            }
            MouseEventKind::Down(MouseButton::Left) => {
                self.mouse_drag_anchor = self.pointer_target(mouse.column, mouse.row);
                self.screen.clear_selection();
                if let Some(target) = self.mouse_drag_anchor {
                    self.apply_pointer_target(target);
                    self.redraw(stdout)
                } else {
                    Ok(())
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let Some(anchor) = self.mouse_drag_anchor else {
                    return Ok(());
                };
                let Some(target) = self.pointer_target(mouse.column, mouse.row) else {
                    return Ok(());
                };
                self.apply_pointer_target(target);
                self.screen.set_selection(self.selection_from_drag(anchor, target));
                self.redraw(stdout)
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if let Some(target) = self.pointer_target(mouse.column, mouse.row) {
                    self.apply_pointer_target(target);
                    if let Some(anchor) = self.mouse_drag_anchor.take() {
                        if self.screen.selection.is_some() {
                            self.screen.set_selection(self.selection_from_drag(anchor, target));
                        }
                    }
                    self.redraw(stdout)
                } else {
                    self.mouse_drag_anchor = None;
                    Ok(())
                }
            }
            MouseEventKind::Down(_) | MouseEventKind::Drag(_) | MouseEventKind::Up(_) | MouseEventKind::Moved => Ok(()),
        }
    }

    fn pointer_target(&self, column: u16, row: u16) -> Option<PointerTarget> {
        if row == 1 {
            let prompt_len = self.screen.command_input_col() as usize;
            let scroll_label = "  Scroll ===> ";
            let cmd_input_width = (self.screen.width as usize)
                .saturating_sub(prompt_len + scroll_label.len() + 5);
            let scroll_start_col = (prompt_len + cmd_input_width + scroll_label.len()) as u16;

            return Some(if column >= scroll_start_col {
                PointerTarget::ScrollField {
                    col: (column - scroll_start_col) as usize,
                }
            } else {
                PointerTarget::CommandLine {
                    col: column.saturating_sub(self.screen.command_input_col()) as usize,
                }
            });
        }

        let data_start_row = HEADER_ROWS;
        let data_end_row = HEADER_ROWS + self.screen.data_rows() as u16;
        if row < data_start_row || row >= data_end_row {
            return None;
        }

        let line_index = self.screen.screen_row_to_line(row);
        if line_index >= self.buffer.line_count() {
            return None;
        }

        if column < PREFIX_WIDTH as u16 {
            Some(PointerTarget::PrefixArea {
                line_index,
                screen_row: row,
            })
        } else {
            Some(PointerTarget::DataArea {
                line_index,
                screen_row: row,
                col: self.data_col_from_pointer(column, line_index),
            })
        }
    }

    fn apply_pointer_target(&mut self, target: PointerTarget) {
        match target {
            PointerTarget::CommandLine { col } => {
                self.input.focus = FieldFocus::CommandLine;
                self.screen.command_cursor_pos = col.min(self.screen.command_line.len());
                self.cursor_col = 0;
            }
            PointerTarget::ScrollField { col } => {
                self.input.focus = FieldFocus::ScrollField;
                self.cursor_col = col.min(self.screen.scroll_field_text.len());
            }
            PointerTarget::PrefixArea {
                line_index: _,
                screen_row,
            } => {
                self.input.focus = FieldFocus::PrefixArea { screen_row };
                self.cursor_col = 0;
                self.update_cursor_line_index();
            }
            PointerTarget::DataArea {
                line_index: _,
                screen_row,
                col,
            } => {
                self.input.focus = FieldFocus::DataArea { screen_row };
                self.cursor_col = col.saturating_sub(self.screen.horizontal_offset);
                self.update_cursor_line_index();
            }
        }
    }

    fn selection_from_drag(&self, anchor: PointerTarget, target: PointerTarget) -> Option<Selection> {
        match (anchor, target) {
            (PointerTarget::PrefixArea { line_index: start, .. }, PointerTarget::PrefixArea { line_index: end, .. })
            | (PointerTarget::PrefixArea { line_index: start, .. }, PointerTarget::DataArea { line_index: end, .. }) => Some(Selection::Lines {
                start_line: start,
                end_line: end,
            }),
            (
                PointerTarget::DataArea {
                    line_index: start_line,
                    col: start_col,
                    ..
                },
                PointerTarget::DataArea {
                    line_index: end_line,
                    col: end_col,
                    ..
                },
            ) => Some(Selection::Text {
                area: SelectionArea::Data,
                start_line,
                start_col,
                end_line,
                end_col,
            }),
            (
                PointerTarget::DataArea {
                    line_index: start_line,
                    col: start_col,
                    ..
                },
                PointerTarget::PrefixArea { line_index: end_line, .. },
            ) => Some(Selection::Text {
                area: SelectionArea::Data,
                start_line,
                start_col,
                end_line,
                end_col: 0,
            }),
            (PointerTarget::CommandLine { col: start_col }, PointerTarget::CommandLine { col: end_col }) => {
                Some(Selection::Text {
                    area: SelectionArea::CommandLine,
                    start_line: 0,
                    start_col,
                    end_line: 0,
                    end_col,
                })
            }
            (PointerTarget::ScrollField { col: start_col }, PointerTarget::ScrollField { col: end_col }) => {
                Some(Selection::Text {
                    area: SelectionArea::ScrollField,
                    start_line: 0,
                    start_col,
                    end_line: 0,
                    end_col,
                })
            }
            _ => None,
        }
    }

    fn data_col_from_pointer(&self, column: u16, line_index: usize) -> usize {
        let data_start_col = self.screen.data_start_col();
        let requested = column.saturating_sub(data_start_col) as usize;
        let max_visible_col = self.screen.data_width().saturating_sub(1);
        let line_len = self
            .buffer
            .lines
            .get(line_index)
            .map(|line| line.data.len().saturating_sub(self.screen.horizontal_offset))
            .unwrap_or(0);
        self.screen.horizontal_offset + requested.min(max_visible_col).min(line_len)
    }

    /// Scroll the data area up by the configured scroll amount.
    fn scroll_page_up(&mut self) {
        let amount = self.screen.scroll_amount.clone();
        let lines = amount.resolve(self.screen.data_rows(), self.cursor_screen_row());
        debug!("Action: ScrollUp by {lines}");
        self.screen.scroll_up(lines);
        self.clamp_cursor();
        self.needs_full_redraw = true;
    }

    /// Scroll the data area down by the configured scroll amount.
    fn scroll_page_down(&mut self) {
        let amount = self.screen.scroll_amount.clone();
        let lines = amount.resolve(self.screen.data_rows(), self.cursor_screen_row());
        let max = self.buffer.line_count().saturating_sub(1);
        debug!("Action: ScrollDown by {lines}");
        self.needs_full_redraw = true;
        self.screen.scroll_down(lines, max);
        self.clamp_cursor();
    }

    /// If a panel display was queued by command processing, show it now.
    fn maybe_display_pending_panel<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        let Some(panel_id) = self.pending_panel.take() else {
            return Ok(());
        };
        let Some(ref mut pm) = self.panel_manager else {
            self.screen.message = Some(Message {
                text: "Panel system not available (no panels/ directory)".to_string(),
                msg_type: MessageType::Error,
            });
            return Ok(());
        };
        if !pm.has_panel(&panel_id) {
            self.screen.message = Some(Message {
                text: format!("Panel not found: {panel_id}"),
                msg_type: MessageType::Error,
            });
            return Ok(());
        }
        match pm.display(stdout, &panel_id) {
            Ok(true) => {
                self.running = false;
                return Ok(());
            }
            Ok(false) => {}
            Err(e) => {
                warn!("Panel display error: {e}");
                self.screen.message = Some(Message {
                    text: format!("Panel error: {e}"),
                    msg_type: MessageType::Error,
                });
            }
        }
        // Force full redraw after returning from panel
        self.needs_full_redraw = true;
        // Pick up any updated EDITOR OPTIONS settings.
        self.refresh_settings();
        Ok(())
    }

    // --- Character handling ---

    fn handle_char(&mut self, c: char) {
        self.cancel_completion();
        let c = if self.buffer.caps_mode {
            c.to_uppercase().next().unwrap_or(c)
        } else {
            c
        };
        let insert_mode = self.input.mode == InputMode::Insert;
        match self.input.focus {
            FieldFocus::CommandLine => self.insert_in_command_line(c, insert_mode),
            FieldFocus::ScrollField => self.insert_in_scroll_field(c, insert_mode),
            FieldFocus::PrefixArea { screen_row } => {
                self.insert_in_prefix(c, screen_row, insert_mode)
            }
            FieldFocus::DataArea { screen_row } => self.insert_in_data(c, screen_row, insert_mode),
        }
    }

    /// Insert/overtype `c` into a `String` at `pos`. When `insert` is true or
    /// `pos` is at/past the end, the character is inserted; otherwise the
    /// existing character at `pos` is replaced.
    fn insert_or_overtype(s: &mut String, pos: usize, c: char, insert: bool) {
        if insert || pos >= s.len() {
            s.insert(pos.min(s.len()), c);
            return;
        }
        let mut chars: Vec<char> = s.chars().collect();
        if pos < chars.len() {
            chars[pos] = c;
        } else {
            chars.push(c);
        }
        *s = chars.into_iter().collect();
    }

    fn insert_in_command_line(&mut self, c: char, insert_mode: bool) {
        Self::insert_or_overtype(
            &mut self.screen.command_line,
            self.screen.command_cursor_pos,
            c,
            insert_mode,
        );
        self.screen.command_cursor_pos += 1;
    }

    fn insert_in_scroll_field(&mut self, c: char, insert_mode: bool) {
        Self::insert_or_overtype(
            &mut self.screen.scroll_field_text,
            self.cursor_col,
            c,
            insert_mode,
        );
        self.cursor_col += 1;
    }

    fn insert_in_prefix(&mut self, c: char, screen_row: u16, insert_mode: bool) {
        if self.cursor_col >= PREFIX_WIDTH {
            return;
        }
        let line_index = self.screen.screen_row_to_line(screen_row);
        if let Some(line) = self.buffer.lines.get_mut(line_index) {
            let mut cmd = line.prefix_cmd.clone().unwrap_or_default();
            if self.cursor_col < cmd.len() {
                let mut chars: Vec<char> = cmd.chars().collect();
                if insert_mode {
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

    fn insert_in_data(&mut self, c: char, screen_row: u16, insert_mode: bool) {
        let line_index = self.screen.screen_row_to_line(screen_row);
        if self.buffer.browse_mode {
            return;
        }
        match self.buffer.lines.get(line_index) {
            Some(line) if line.is_writable() => {}
            _ => return,
        }
        let actual_col = self.screen.horizontal_offset + self.cursor_col;
        let mut data = self
            .buffer
            .lines
            .get(line_index)
            .map(|l| l.data.clone())
            .unwrap_or_default();
        while data.len() <= actual_col {
            data.push(' ');
        }
        let mut chars: Vec<char> = data.clone();
        if insert_mode {
            chars.insert(actual_col, c);
        } else {
            chars[actual_col] = c;
        }
        self.buffer
            .update_line_data(line_index, chars.into_iter().collect());
        self.cursor_col += 1;
    }

    fn handle_delete(&mut self) {
        self.cancel_completion();
        match self.input.focus {
            FieldFocus::CommandLine => {
                let pos = self.screen.command_cursor_pos;
                if pos < self.screen.command_line.len() {
                    self.screen.command_line.remove(pos);
                }
            }
            FieldFocus::ScrollField => {
                if self.cursor_col < self.screen.scroll_field_text.len() {
                    self.screen.scroll_field_text.remove(self.cursor_col);
                }
            }
            FieldFocus::PrefixArea { screen_row } => {
                self.delete_in_prefix(screen_row, self.cursor_col);
            }
            FieldFocus::DataArea { screen_row } => {
                let line_index = self.screen.screen_row_to_line(screen_row);
                let actual_col = self.screen.horizontal_offset + self.cursor_col;
                self.delete_in_data(line_index, actual_col);
            }
        }
    }

    fn delete_in_prefix(&mut self, screen_row: u16, pos: usize) {
        let line_index = self.screen.screen_row_to_line(screen_row);
        let Some(line) = self.buffer.lines.get_mut(line_index) else {
            return;
        };
        let Some(ref mut cmd) = line.prefix_cmd else {
            return;
        };
        if pos < cmd.len() {
            cmd.remove(pos);
        }
        if cmd.is_empty() {
            line.prefix_cmd = None;
            line.flags.clear(LineFlags::PENDING_CMD);
        }
    }

    fn delete_in_data(&mut self, line_index: usize, actual_col: usize) {
        let data = self
            .buffer
            .lines
            .get(line_index)
            .map(|l| l.data.clone())
            .unwrap_or_default();
        if actual_col >= data.len() {
            return;
        }
        let mut chars: Vec<char> = data.clone();
        chars.remove(actual_col);
        self.buffer
            .update_line_data(line_index, chars.into_iter().collect());
    }

    fn handle_backspace(&mut self) {
        self.cancel_completion();
        match self.input.focus {
            FieldFocus::CommandLine => {
                if self.screen.command_cursor_pos == 0 {
                    return;
                }
                self.screen.command_cursor_pos -= 1;
                let pos = self.screen.command_cursor_pos;
                self.screen.command_line.remove(pos);
            }
            FieldFocus::ScrollField => {
                if self.cursor_col == 0 {
                    return;
                }
                self.cursor_col -= 1;
                self.screen.scroll_field_text.remove(self.cursor_col);
            }
            FieldFocus::PrefixArea { screen_row } => {
                if self.cursor_col == 0 {
                    return;
                }
                self.cursor_col -= 1;
                self.delete_in_prefix(screen_row, self.cursor_col);
            }
            FieldFocus::DataArea { screen_row } => {
                if self.cursor_col == 0 {
                    return;
                }
                self.cursor_col -= 1;
                let line_index = self.screen.screen_row_to_line(screen_row);
                let actual_col = self.screen.horizontal_offset + self.cursor_col;
                self.delete_in_data(line_index, actual_col);
            }
        }
    }

    // --- Cursor movement ---

    fn move_cursor_up(&mut self) {
        if self.completion_menu.is_some() {
            self.completion_prev();
            return;
        }
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
                    if self.screen.top_line_index() > 0 {
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
        if self.completion_menu.is_some() {
            self.completion_next();
            return;
        }
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
        self.cancel_completion();
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else {
            if self.screen.horizontal_offset > 0 {
                self.screen.scroll_left(1);
                self.needs_full_redraw = true;
            }
        }

        if matches!(self.input.focus, FieldFocus::CommandLine) && self.screen.command_cursor_pos > 0
        {
            self.screen.command_cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        self.cancel_completion();
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
                } else {
                    self.screen.scroll_right(1);
                    self.needs_full_redraw = true;
                }
            }
        }
    }

    fn handle_home(&mut self) {
        self.cancel_completion();
        self.input.focus = FieldFocus::CommandLine;
        self.screen.command_cursor_pos = 0;
    }

    fn handle_end(&mut self) {
        self.cancel_completion();
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
        if self.completion_menu.is_some() {
            self.accept_completion();
            return;
        }
        match self.input.focus {
            FieldFocus::CommandLine => {
                self.input.focus = FieldFocus::ScrollField;
                self.cursor_col = 0;
                // self.input.focus = FieldFocus::CommandLine;
                // self.cursor_col = 0;
                // self.screen.command_cursor_pos = 0;
            }
            FieldFocus::ScrollField => {
                self.input.focus = FieldFocus::DataArea {
                    screen_row: HEADER_ROWS,
                };
                self.cursor_col = 0;
                self.update_cursor_line_index();
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
        self.cancel_completion();
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
        match self.input.focus {
            FieldFocus::PrefixArea { screen_row } | FieldFocus::DataArea { screen_row } => {
                self.cursor_line_index = self.screen.screen_row_to_line(screen_row);
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

    // --- Newline processing (regular Enter key) ---

    /// Re-read editor settings (e.g. Enter-key mode) from the SPFSETS
    /// profile after a panel display might have changed them.
    fn refresh_settings(&mut self) {
        if let Some(ref pm) = self.panel_manager {
            if let Some(val) = pm.vars().profile_get("SPFSETS", "ZENTRKEY") {
                self.input.enter_mode = EnterMode::from_profile(val);
            }
        }
    }

    /// Handle the regular Enter key: insert a blank line below the
    /// cursor (when in the data area) and move the cursor to it.
    /// Outside the data area this is a no-op.
    fn handle_newline(&mut self) {
        self.cancel_completion();
        self.advance_cursor_after_enter();
    }

    // --- Enter processing ---

    fn handle_enter(&mut self) {
        self.cancel_completion();
        let line_cmd_insert_origin = self.pending_insert_origin_from_prefix();
        let enter_on_insert_line = self.is_cursor_on_insert_line();

        // 1. Normalise prefix commands (uppercase / clear empties).
        self.normalize_prefix_commands();

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
        // Line cmd results may change line types and prefixes, so do a full redraw.
        self.needs_full_redraw = true;

        // 3. Parse and execute primary command
        let cmd_text = self.screen.command_line.trim().to_string();
        if !cmd_text.is_empty() {
            self.process_primary_command(cmd_text);
        }

        // 4. Clear command line
        self.screen.command_line.clear();
        self.screen.command_cursor_pos = 0;
        self.history_index = None;

        // 5. Parse scroll field
        self.screen.parse_scroll_field();

        // 6. Special insert-line behavior.
        if enter_on_insert_line {
            self.handle_insert_line_enter();
            return;
        }

        if let Some(origin) = line_cmd_insert_origin {
            if self.focus_insert_line_after(origin) {
                return;
            }
        }

        // 7. If on data area, advance cursor to next line
        self.advance_cursor_after_enter();
    }

    fn pending_insert_origin_from_prefix(&self) -> Option<usize> {
        let FieldFocus::PrefixArea { screen_row } = self.input.focus else {
            return None;
        };
        let line_index = self.screen.screen_row_to_line(screen_row);
        let line = self.buffer.lines.get(line_index)?;
        let cmd = line.prefix_cmd.as_deref()?;
        match parse_prefix_command(cmd) {
            PrefixParseResult::Command(ParsedLineCmd::Insert(_)) => Some(line_index),
            _ => None,
        }
    }

    fn is_cursor_on_insert_line(&self) -> bool {
        let FieldFocus::DataArea { screen_row } = self.input.focus else {
            return false;
        };
        let line_index = self.screen.screen_row_to_line(screen_row);
        matches!(
            self.buffer.lines.get(line_index).map(|l| l.line_type),
            Some(crate::line::LineType::Insert)
        )
    }

    fn focus_insert_line_after(&mut self, origin_line_index: usize) -> bool {
        let target = origin_line_index + 1;
        let Some(line) = self.buffer.lines.get(target) else {
            return false;
        };
        if line.line_type != crate::line::LineType::Insert {
            return false;
        }
        self.focus_data_line(target, 0);
        true
    }

    fn handle_insert_line_enter(&mut self) {
        let FieldFocus::DataArea { screen_row } = self.input.focus else {
            return;
        };
        let line_index = self.screen.screen_row_to_line(screen_row);
        if !self.buffer.promote_insert_line(line_index) {
            return;
        }
        self.buffer.insert_insert_marker_after(line_index);
        self.buffer.renumber();
        self.focus_data_line(line_index + 1, 0);
        self.needs_full_redraw = true;
    }

    fn focus_data_line(&mut self, line_index: usize, col: usize) {
        let max = self.buffer.line_count().saturating_sub(1);
        self.screen.ensure_visible(line_index, max);
        if let Some(row) = self.screen.line_to_screen_row(line_index) {
            self.input.focus = FieldFocus::DataArea { screen_row: row };
            self.cursor_line_index = line_index;
            self.cursor_col = col;
        }
    }

    /// Uppercase pending prefix commands; clear empty ones to restore the
    /// line-number display.
    fn normalize_prefix_commands(&mut self) {
        for line in self.buffer.lines.iter_mut() {
            let Some(ref cmd) = line.prefix_cmd else {
                continue;
            };
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

    /// Parse and execute a primary command, recording it in history and
    /// applying its result.
    fn process_primary_command(&mut self, cmd_text: String) {
        info!("Primary command: {:?}", cmd_text);
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
            Err(msg) if !msg.is_empty() => {
                warn!("Command parse error: {msg}");
                self.screen.message = Some(Message {
                    text: msg,
                    msg_type: MessageType::Error,
                });
            }
            Err(_) => {}
        }
    }

    /// After an Enter on a data/prefix row, move the cursor down one line
    /// (or transition prefix→data) where appropriate.
    fn advance_cursor_after_enter(&mut self) {
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
            // CommandLine: stay on command line after executing command.
            FieldFocus::CommandLine | FieldFocus::ScrollField => {}
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
            self.needs_full_redraw = true;
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

        // Schedule panel display if requested
        if result.show_panel.is_some() {
            self.pending_panel = result.show_panel;
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
        debug!("redrawing screen (full redraw: {})", self.needs_full_redraw);
        self.screen.input_mode = self.input.mode;
        self.sync_completion_popup();
        // self.screen.draw_full(stdout, &self.buffer)?;
        let (col, row) = self.calculate_cursor_position();
        trace!("Positioning cursor for focus: {col} {row}");
        if self.needs_full_redraw || self.screen.needs_full_redraw {
            self.screen.draw_full(stdout, &self.buffer)?;
            self.needs_full_redraw = false;
        } else {
            self.redraw_line(stdout, row as usize)?;
        }
        queue!(stdout, MoveTo(col, row))?;
        stdout.flush()?;
        Ok(())
    }

    fn redraw_line<W: Write>(&mut self, stdout: &mut W, line_index: usize) -> io::Result<()> {
        self.screen.draw_line(stdout, line_index, &self.buffer)
    }

    fn position_cursor<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        let (col, row) = self.calculate_cursor_position();
        trace!("Positioning cursor for focus: {col} {row}");
        queue!(stdout, MoveTo(col, row))?;
        stdout.flush()?;
        Ok(())
    }

    fn calculate_cursor_position(&self) -> (u16, u16) {
        let f = self.input.focus;
        let (col, row) = match f {
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
        (col, row)
    }

    fn send_did_open_if_available(&mut self) {
        let text = self.document_text();
        if let Some(lsp) = self.lsp.as_ref() {
            if let Err(err) = lsp.did_open(&text) {
                warn!("didOpen failed: {err}");
            }
        }
    }

    fn document_text(&self) -> String {
        self.buffer
            .lines
            .iter()
            .filter(|line| line.is_data())
            .map(|line| line.data.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn trigger_completion(&mut self) {
        let text = self.document_text();
        let position = self.current_lsp_position();
        let req_prefix = self.current_prefix_sample();
        let trigger_character = Self::completion_trigger_from_prefix(&req_prefix);
        let req_token = Self::completion_token_from_prefix(&req_prefix);
        let mut req_doc_uri = String::new();
        let mut req_root_uri = String::new();
        if let Some(client) = self.lsp.as_ref() {
            req_doc_uri = Self::shorten_debug(client.document_uri(), 64);
            req_root_uri = Self::shorten_debug(client.workspace_root_uri(), 64);
        }

        let Some(lsp) = self.lsp.as_mut() else {
            self.screen.message = Some(Message {
                text: self
                    .lsp_startup_error
                    .clone()
                    .unwrap_or_else(|| "rust-analyzer not available".to_string()),
                msg_type: MessageType::Error,
            });
            return;
        };

        let FieldFocus::DataArea { .. } = self.input.focus else {
            self.screen.message = Some(Message {
                text: "Completion works in data area only".to_string(),
                msg_type: MessageType::Error,
            });
            return;
        };

        let Some((line, character)) = position else {
            self.screen.message = Some(Message {
                text: "Completion is available on editable data lines only".to_string(),
                msg_type: MessageType::Error,
            });
            return;
        };

        let version = match lsp.did_change(&text) {
            Ok(v) => v,
            Err(err) => {
                self.screen.message = Some(Message {
                    text: format!("LSP sync failed: {err}"),
                    msg_type: MessageType::Error,
                });
                return;
            }
        };

        let project_ready = match lsp.wait_for_project_ready(Duration::from_secs(3)) {
            Ok(ready) => ready,
            Err(err) => {
                self.screen.message = Some(Message {
                    text: format!("LSP readiness check failed: {err}"),
                    msg_type: MessageType::Error,
                });
                return;
            }
        };

        match lsp.request_completion(line, character, trigger_character) {
            Ok(request_id) => {
                let (anchor_col, anchor_row) = self.calculate_cursor_position();
                self.pending_completion = Some(PendingCompletion {
                    request_id,
                    version,
                    anchor_row: anchor_row.saturating_add(1),
                    anchor_col,
                    req_line: line,
                    req_char_utf16: character,
                    req_prefix: req_prefix.clone(),
                    req_token: req_token.clone(),
                });
                self.screen.status_info = format!(
                    "LSP req id={request_id} ready={project_ready} l={line} c16={character} tok='{}' pfx='{}' doc={} root={}",
                    Self::shorten_debug(&req_token, 20),
                    Self::shorten_debug(&req_prefix, 24),
                    req_doc_uri,
                    req_root_uri,
                );
                self.screen.message = Some(Message {
                    text: format!(
                        "Completion requested l={line} c16={character} tok='{}'",
                        Self::shorten_debug(&req_token, 20)
                    ),
                    msg_type: MessageType::Info,
                });
            }
            Err(err) => {
                self.screen.message = Some(Message {
                    text: format!("Completion request failed: {err}"),
                    msg_type: MessageType::Error,
                });
            }
        }
    }

    fn current_lsp_position(&self) -> Option<(u32, u32)> {
        let FieldFocus::DataArea { screen_row } = self.input.focus else {
            return None;
        };

        let line_index = self.screen.screen_row_to_line(screen_row);
        let line = self.buffer.lines.get(line_index)?;
        if !line.is_data() {
            return None;
        }

        let doc_line = self.buffer_line_to_document_line(line_index);
        let char_col = (self.screen.horizontal_offset + self.cursor_col).min(line.data.len());
        let utf16_col = Self::utf16_column_from_chars(&line.data, char_col);
        Some((doc_line as u32, utf16_col as u32))
    }

    fn utf16_column_from_chars(chars: &[char], char_col: usize) -> usize {
        chars
            .iter()
            .take(char_col)
            .map(|ch| ch.len_utf16())
            .sum()
    }

    fn buffer_line_to_document_line(&self, line_index: usize) -> usize {
        self.buffer
            .lines
            .iter()
            .enumerate()
            .filter(|(idx, line)| *idx < line_index && line.is_data())
            .count()
    }

    fn drain_lsp_events(&mut self) {
        loop {
            let event = match self.lsp.as_ref() {
                Some(lsp) => match lsp.try_recv() {
                    Ok(ev) => ev,
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        self.lsp = None;
                        break;
                    }
                },
                None => break,
            };

            match event {
                LspEvent::InitializeComplete => {}
                LspEvent::HoverResponse { request_id, contents } => todo!(),
                LspEvent::CompletionResponse { request_id, items } => {
                    self.handle_completion_response(request_id, items);
                }
            }
        }
    }

    fn handle_completion_response(&mut self, request_id: u64, items: Vec<CompletionItem>) {
        let Some(pending) = self.pending_completion.take() else {
            return;
        };
        if pending.request_id != request_id {
            return;
        }
        if let Some(lsp) = self.lsp.as_ref() {
            if lsp.version() != pending.version {
                return;
            }
        }
        let items = Self::filter_completion_items(items, &pending.req_token);
        if items.is_empty() {
            self.screen.status_info = format!(
                "LSP rsp id={} l={} c16={} tok='{}' pfx='{}' count=0",
                request_id,
                pending.req_line,
                pending.req_char_utf16,
                Self::shorten_debug(&pending.req_token, 20),
                Self::shorten_debug(&pending.req_prefix, 24),
            );
            self.screen.message = Some(Message {
                text: "No completions".to_string(),
                msg_type: MessageType::Info,
            });
            self.completion_menu = None;
            return;
        }

        let sample = items
            .iter()
            .take(3)
            .map(|it| it.label.as_str())
            .collect::<Vec<_>>()
            .join(",");
        self.screen.status_info = format!(
            "LSP rsp id={} l={} c16={} tok='{}' pfx='{}' count={} top=[{}]",
            request_id,
            pending.req_line,
            pending.req_char_utf16,
            Self::shorten_debug(&pending.req_token, 20),
            Self::shorten_debug(&pending.req_prefix, 24),
            items.len(),
            Self::shorten_debug(&sample, 36),
        );

        self.completion_menu = Some(CompletionMenu {
            anchor_row: pending.anchor_row,
            anchor_col: pending.anchor_col,
            selected: 0,
            items,
        });
        self.needs_full_redraw = true;
    }

    fn current_prefix_sample(&self) -> String {
        let FieldFocus::DataArea { screen_row } = self.input.focus else {
            return String::new();
        };
        let line_index = self.screen.screen_row_to_line(screen_row);
        let Some(line) = self.buffer.lines.get(line_index) else {
            return String::new();
        };
        let char_col = (self.screen.horizontal_offset + self.cursor_col).min(line.data.len());
        let start = char_col.saturating_sub(20);
        line.data[start..char_col].iter().collect::<String>()
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

    fn filter_completion_items(items: Vec<CompletionItem>, token: &str) -> Vec<CompletionItem> {
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

    fn shorten_debug(input: &str, max_chars: usize) -> String {
        let mut out = input.chars().take(max_chars).collect::<String>();
        if input.chars().count() > max_chars {
            out.push_str("...");
        }
        out
    }

    fn completion_next(&mut self) {
        let Some(menu) = self.completion_menu.as_mut() else {
            return;
        };
        if menu.items.is_empty() {
            return;
        }
        menu.selected = (menu.selected + 1) % menu.items.len();
    }

    fn completion_prev(&mut self) {
        let Some(menu) = self.completion_menu.as_mut() else {
            return;
        };
        if menu.items.is_empty() {
            return;
        }
        if menu.selected == 0 {
            menu.selected = menu.items.len() - 1;
        } else {
            menu.selected -= 1;
        }
    }

    fn accept_completion(&mut self) {
        let Some(menu) = self.completion_menu.as_ref() else {
            return;
        };
        let Some(item) = menu.items.get(menu.selected) else {
            return;
        };

        let text = item.insert_text.clone();
        self.apply_completion_text(&text);
        self.completion_menu = None;
    }

    fn cancel_completion(&mut self) {
        self.completion_menu = None;
        self.pending_completion = None;
    }

    fn apply_completion_text(&mut self, insert_text: &str) {
        let FieldFocus::DataArea { screen_row } = self.input.focus else {
            return;
        };
        let line_index = self.screen.screen_row_to_line(screen_row);
        let Some(line) = self.buffer.lines.get(line_index) else {
            return;
        };
        if !line.is_data() {
            return;
        }

        let actual_col = self.screen.horizontal_offset + self.cursor_col;
        let line_chars = line.data.clone();
        let mut start = actual_col.min(line_chars.len());
        while start > 0 {
            let ch = line_chars[start - 1];
            if ch.is_ascii_alphanumeric() || ch == '_' {
                start -= 1;
            } else {
                break;
            }
        }

        let mut updated = Vec::with_capacity(line_chars.len() + insert_text.len());
        updated.extend_from_slice(&line_chars[..start]);
        updated.extend(insert_text.chars());
        updated.extend_from_slice(&line_chars[actual_col.min(line_chars.len())..]);

        self.buffer
            .update_line_data(line_index, updated.into_iter().collect::<String>());

        let new_col = start + insert_text.chars().count();
        self.cursor_col = new_col.saturating_sub(self.screen.horizontal_offset);
        self.screen.message = Some(Message {
            text: "Completion inserted".to_string(),
            msg_type: MessageType::Info,
        });
    }

    fn sync_completion_popup(&mut self) {
        let popup = self.completion_menu.as_ref().map(|menu| CompletionPopup {
            row: menu.anchor_row,
            col: menu.anchor_col,
            selected: menu.selected,
            items: menu.items.iter().map(|item| item.label.clone()).collect(),
        });
        self.screen.set_completion_popup(popup);
    }
}
