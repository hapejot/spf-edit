//! Input handling: translate crossterm key events into `EditorAction` values.
//!
//! This module is intentionally stateless relative to the buffer — it only
//! knows about the current focus and input mode.  The `Editor` interprets
//! the resulting `EditorAction`.
//!
//! ## Key Bindings
//!
//!   Ctrl+Q          — force quit (no save prompt)
//!   F3 / Esc        — END command
//!   F5              — RFIND
//!   F7 / F8         — scroll up / down by scroll amount
//!   F10 / F11       — scroll left / right by one screen width
//!   F12             — retrieve previous command from history
//!   Insert          — toggle overtype / insert mode
//!   Tab / Shift+Tab — cycle focus: command line → data → prefix → …
//!   Enter           — flush all edits and execute commands
//!
//! ## Known Issues
//!
//! - No PF-key customization yet.  All bindings are hardcoded.
//! - Esc is mapped to FnEnd which means there's no way to cancel a
//!   partially typed prefix command without pressing Enter first.
//!   TODO: consider Esc as "reset current field" instead.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use tracing::trace;

use crate::types::{EnterMode, FieldFocus, InputMode};

/// Actions the editor should take in response to input.
#[derive(Debug, Clone)]
pub enum EditorAction {
    /// No action needed.
    None,
    /// Insert/overtype a character at cursor position.
    InsertChar(char),
    /// Delete character at cursor (Delete key).
    DeleteChar,
    /// Delete character before cursor (Backspace).
    Backspace,
    /// Move cursor in a direction.
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    /// Tab — cycle field focus.
    Tab,
    /// Backtab — cycle field focus backwards.
    BackTab,
    /// Enter (Numpad Enter) — submit pending prefix and primary commands.
    Enter,
    /// Newline (regular Enter) — split the current line at the cursor
    /// (or insert a blank line below when at end of line).
    Newline,
    /// Toggle insert/overtype mode.
    ToggleInsertMode,
    /// Function key commands.
    FnEnd, // F3
    FnRFind,       // F5
    FnScrollUp,    // F7
    FnScrollDown,  // F8
    FnScrollLeft,  // F10
    FnScrollRight, // F11
    FnRetrieve,    // F12
    /// Terminal resized.
    Resize(u16, u16),
    /// Quit (Ctrl+Q emergency exit).
    ForceQuit,
}

pub struct InputHandler {
    pub focus: FieldFocus,
    pub mode: InputMode,
    pub enter_mode: EnterMode,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            focus: FieldFocus::CommandLine,
            mode: InputMode::Overtype,
            enter_mode: EnterMode::Legacy,
        }
    }

    // Translate a crossterm event into an EditorAction.
    pub fn handle_event(&self, event: Event) -> EditorAction {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                let action = self.handle_key(key);
                trace!(
                    "key={:?} mods={:?} -> {:?}",
                    key.code, key.modifiers, action
                );
                action
            }
            Event::Resize(w, h) => EditorAction::Resize(w, h),
            _ => EditorAction::None,
        }
    }

    fn handle_key(&self, key: KeyEvent) -> EditorAction {
        // Ctrl+Q force quit
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
            return EditorAction::ForceQuit;
        }

        match key.code {
            KeyCode::Char(c) => EditorAction::InsertChar(c),
            // Enter handling depends on the user's `EnterMode` choice
            // (configured in the EDITOR OPTIONS panel) plus, on Unix
            // terminals with kitty disambiguation enabled, the
            // `KeyEventState::KEYPAD` flag.
            //
            //   Legacy        : Enter = submit, no newline key.
            //   ShiftNewline  : Enter = submit, Shift+Enter = newline.
            //   AltNewline    : Enter = submit, Alt+Enter   = newline.
            //
            // On non-Windows terminals that disambiguate keypad keys,
            // Numpad Enter always submits and the main Enter inserts a
            // newline regardless of the configured mode.
            KeyCode::Enter => {
                let shift = key.modifiers.contains(KeyModifiers::SHIFT);
                let alt = key.modifiers.contains(KeyModifiers::ALT);
                let keypad = key.state.contains(KeyEventState::KEYPAD);

                if !cfg!(windows) && keypad {
                    // Disambiguating Unix terminal: numpad submits.
                    EditorAction::Enter
                } else if !cfg!(windows) && !keypad
                    && matches!(self.enter_mode, EnterMode::Legacy)
                {
                    // Disambiguating Unix terminal in legacy mode:
                    // main Enter inserts a newline (numpad submits).
                    EditorAction::Newline
                } else {
                    match self.enter_mode {
                        EnterMode::Legacy => EditorAction::Enter,
                        EnterMode::ShiftNewline => {
                            if shift {
                                EditorAction::Newline
                            } else {
                                EditorAction::Enter
                            }
                        }
                        EnterMode::AltNewline => {
                            if alt {
                                EditorAction::Newline
                            } else {
                                EditorAction::Enter
                            }
                        }
                    }
                }
            }
            KeyCode::Backspace => EditorAction::Backspace,
            KeyCode::Delete => EditorAction::DeleteChar,
            KeyCode::Up => EditorAction::CursorUp,
            KeyCode::Down => EditorAction::CursorDown,
            KeyCode::Left => EditorAction::CursorLeft,
            KeyCode::Right => EditorAction::CursorRight,
            KeyCode::Home => EditorAction::CursorHome,
            KeyCode::End => EditorAction::CursorEnd,
            KeyCode::Tab => EditorAction::Tab,
            KeyCode::BackTab => EditorAction::BackTab,
            KeyCode::Insert => EditorAction::ToggleInsertMode,
            KeyCode::PageDown => EditorAction::FnScrollDown,
            KeyCode::PageUp => EditorAction::FnScrollUp,
            KeyCode::F(3) => EditorAction::FnEnd,
            KeyCode::F(5) => EditorAction::FnRFind,
            KeyCode::F(7) => EditorAction::FnScrollUp,
            KeyCode::F(8) => EditorAction::FnScrollDown,
            KeyCode::F(10) => EditorAction::FnScrollLeft,
            KeyCode::F(11) => EditorAction::FnScrollRight,
            KeyCode::F(12) => EditorAction::FnRetrieve,
            KeyCode::Esc => EditorAction::FnEnd, // Esc also acts as END
            _ => EditorAction::None,
        }
    }
}
