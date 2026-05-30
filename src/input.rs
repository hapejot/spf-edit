//! Input handling: translate every crossterm event into configurable SPF events.
//!
//! The translation layer is intentionally decoupled from editor state mutation.
//! It turns terminal input into internal SPF events, which may in turn dispatch
//! editor actions, execute commands directly, or expand into ordered sequences.

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseEvent,
    MouseEventKind,
};
use tracing::trace;

use crate::{
    command::PrimaryCommand,
    types::{EnterMode, FieldFocus, InputMode},
};

/// Actions the editor should take in response to input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorAction {
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
    /// Function key actions.
    FnScrollUp,    // F7
    FnScrollDown,  // F8
    FnScrollLeft,  // F10
    FnScrollRight, // F11
    FnRetrieve,    // F12
    /// Trigger rust-analyzer completion request.
    TriggerCompletion,
    /// Move selection to next completion candidate.
    CompletionNext,
    /// Move selection to previous completion candidate.
    CompletionPrev,
    /// Accept current completion candidate.
    CompletionAccept,
    /// Dismiss completion menu.
    CompletionCancel,
    /// Terminal resized.
    Resize(u16, u16),
    /// Quit (Ctrl+Q emergency exit).
    ForceQuit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpfKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
    pub state: KeyEventState,
}

impl From<KeyEvent> for SpfKeyEvent {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
            kind: value.kind,
            state: value.state,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpfMouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

impl From<MouseEvent> for SpfMouseEvent {
    fn from(value: MouseEvent) -> Self {
        Self {
            kind: value.kind,
            column: value.column,
            row: value.row,
            modifiers: value.modifiers,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpfInputEvent {
    Key(SpfKeyEvent),
    Resize { width: u16, height: u16 },
    FocusGained,
    FocusLost,
    Mouse(SpfMouseEvent),
    Paste(String),
}

impl From<Event> for SpfInputEvent {
    fn from(value: Event) -> Self {
        match value {
            Event::Key(key) => SpfInputEvent::Key(key.into()),
            Event::Resize(width, height) => SpfInputEvent::Resize { width, height },
            Event::FocusGained => SpfInputEvent::FocusGained,
            Event::FocusLost => SpfInputEvent::FocusLost,
            Event::Mouse(mouse) => SpfInputEvent::Mouse(mouse.into()),
            Event::Paste(text) => SpfInputEvent::Paste(text),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpfEvent {
    Action(EditorAction),
    Command(PrimaryCommand),
    FocusChanged { focused: bool },
    Mouse(SpfMouseEvent),
    Sequence(Vec<SpfEvent>),
    Input(SpfInputEvent),
    Ignore,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCodePattern {
    Any,
    Exact(KeyCode),
    AnyChar,
}

impl KeyCodePattern {
    fn matches(&self, code: &KeyCode) -> bool {
        match self {
            KeyCodePattern::Any => true,
            KeyCodePattern::Exact(expected) => expected == code,
            KeyCodePattern::AnyChar => matches!(code, KeyCode::Char(_)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifiersPattern {
    Any,
    Exact(KeyModifiers),
    Contains(KeyModifiers),
}

impl ModifiersPattern {
    fn matches(&self, modifiers: KeyModifiers) -> bool {
        match self {
            ModifiersPattern::Any => true,
            ModifiersPattern::Exact(expected) => *expected == modifiers,
            ModifiersPattern::Contains(expected) => modifiers.contains(*expected),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyKindPattern {
    Any,
    Exact(KeyEventKind),
}

impl KeyKindPattern {
    fn matches(&self, kind: KeyEventKind) -> bool {
        match self {
            KeyKindPattern::Any => true,
            KeyKindPattern::Exact(expected) => *expected == kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPattern {
    pub code: KeyCodePattern,
    pub modifiers: ModifiersPattern,
    pub kind: KeyKindPattern,
}

impl KeyPattern {
    pub fn press(code: KeyCode) -> Self {
        Self {
            code: KeyCodePattern::Exact(code),
            modifiers: ModifiersPattern::Any,
            kind: KeyKindPattern::Exact(KeyEventKind::Press),
        }
    }

    pub fn any_char_press() -> Self {
        Self {
            code: KeyCodePattern::AnyChar,
            modifiers: ModifiersPattern::Any,
            kind: KeyKindPattern::Exact(KeyEventKind::Press),
        }
    }

    pub fn with_modifiers(mut self, modifiers: ModifiersPattern) -> Self {
        self.modifiers = modifiers;
        self
    }

    fn matches(&self, event: &SpfKeyEvent) -> bool {
        self.code.matches(&event.code)
            && self.modifiers.matches(event.modifiers)
            && self.kind.matches(event.kind)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventMatcher {
    Key(KeyPattern),
    ResizeAny,
    FocusGained,
    FocusLost,
    MouseAny,
    PasteAny,
    Any,
}

impl EventMatcher {
    fn matches(&self, event: &SpfInputEvent) -> bool {
        match (self, event) {
            (EventMatcher::Key(pattern), SpfInputEvent::Key(key)) => pattern.matches(key),
            (EventMatcher::ResizeAny, SpfInputEvent::Resize { .. }) => true,
            (EventMatcher::FocusGained, SpfInputEvent::FocusGained) => true,
            (EventMatcher::FocusLost, SpfInputEvent::FocusLost) => true,
            (EventMatcher::MouseAny, SpfInputEvent::Mouse(_)) => true,
            (EventMatcher::PasteAny, SpfInputEvent::Paste(_)) => true,
            (EventMatcher::Any, _) => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventBindingOutput {
    Event(SpfEvent),
    InsertChar,
    SubmitOrNewline,
    Resize,
    PasteAsSequence,
    FocusChanged(bool),
    Mouse,
    RawInput,
    Ignore,
}

impl EventBindingOutput {
    fn emit(&self, event: &SpfInputEvent, _enter_mode: EnterMode) -> SpfEvent {
        match self {
            EventBindingOutput::Event(spf_event) => spf_event.clone(),
            EventBindingOutput::InsertChar => match event {
                SpfInputEvent::Key(SpfKeyEvent {
                    code: KeyCode::Char(ch),
                    ..
                }) => SpfEvent::Action(EditorAction::InsertChar(*ch)),
                _ => SpfEvent::Input(event.clone()),
            },
            EventBindingOutput::SubmitOrNewline => match event {
                SpfInputEvent::Key(key) => {
                    let submit = key.modifiers.contains(KeyModifiers::SHIFT)
                        || key.modifiers.contains(KeyModifiers::CONTROL);
                    if submit {
                        SpfEvent::Action(EditorAction::Enter)
                    } else {
                        SpfEvent::Action(EditorAction::Newline)
                    }
                }
                _ => SpfEvent::Input(event.clone()),
            },
            EventBindingOutput::Resize => match event {
                SpfInputEvent::Resize { width, height } => {
                    SpfEvent::Action(EditorAction::Resize(*width, *height))
                }
                _ => SpfEvent::Input(event.clone()),
            },
            EventBindingOutput::PasteAsSequence => match event {
                SpfInputEvent::Paste(text) => SpfEvent::Sequence(
                    text.chars()
                        .map(|ch| SpfEvent::Action(EditorAction::InsertChar(ch)))
                        .collect(),
                ),
                _ => SpfEvent::Input(event.clone()),
            },
            EventBindingOutput::FocusChanged(focused) => SpfEvent::FocusChanged { focused: *focused },
            EventBindingOutput::Mouse => match event {
                SpfInputEvent::Mouse(mouse) => SpfEvent::Mouse(mouse.clone()),
                _ => SpfEvent::Input(event.clone()),
            },
            EventBindingOutput::RawInput => SpfEvent::Input(event.clone()),
            EventBindingOutput::Ignore => SpfEvent::Ignore,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBinding {
    pub matcher: EventMatcher,
    pub output: EventBindingOutput,
}

impl EventBinding {
    pub fn new(matcher: EventMatcher, output: EventBindingOutput) -> Self {
        Self { matcher, output }
    }

    fn matches(&self, event: &SpfInputEvent) -> bool {
        self.matcher.matches(event)
    }

    fn emit(&self, event: &SpfInputEvent, enter_mode: EnterMode) -> SpfEvent {
        self.output.emit(event, enter_mode)
    }
}

#[derive(Debug, Clone)]
pub struct EventTranslator {
    bindings: Vec<EventBinding>,
}

impl Default for EventTranslator {
    fn default() -> Self {
        Self {
            bindings: Self::default_bindings(),
        }
    }
}

#[allow(dead_code)]
impl EventTranslator {
    pub fn with_bindings(bindings: Vec<EventBinding>) -> Self {
        Self { bindings }
    }

    pub fn set_bindings(&mut self, bindings: Vec<EventBinding>) {
        self.bindings = bindings;
    }

    pub fn push_binding(&mut self, binding: EventBinding) {
        self.bindings.push(binding);
    }

    pub fn translate(&self, event: SpfInputEvent, enter_mode: EnterMode) -> SpfEvent {
        for binding in &self.bindings {
            if binding.matches(&event) {
                return binding.emit(&event, enter_mode);
            }
        }

        SpfEvent::Input(event)
    }

    pub fn default_bindings() -> Vec<EventBinding> {
        vec![
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char('q'))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::ForceQuit)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Enter)),
                EventBindingOutput::SubmitOrNewline,
            ),
            EventBinding::new(EventMatcher::ResizeAny, EventBindingOutput::Resize),
            EventBinding::new(
                EventMatcher::FocusGained,
                EventBindingOutput::FocusChanged(true),
            ),
            EventBinding::new(
                EventMatcher::FocusLost,
                EventBindingOutput::FocusChanged(false),
            ),
            EventBinding::new(EventMatcher::MouseAny, EventBindingOutput::Mouse),
            EventBinding::new(EventMatcher::PasteAny, EventBindingOutput::PasteAsSequence),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Backspace)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::Backspace)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Delete)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::DeleteChar)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Up)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorUp)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Down)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorDown)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Left)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorLeft)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Right)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorRight)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Home)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorHome)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::End)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CursorEnd)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Tab)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::Tab)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::BackTab)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::BackTab)),
            ),
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char(' '))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::TriggerCompletion)),
            ),
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char('n'))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CompletionNext)),
            ),
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char('p'))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CompletionPrev)),
            ),
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char('y'))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CompletionAccept)),
            ),
            EventBinding::new(
                EventMatcher::Key(
                    KeyPattern::press(KeyCode::Char('e'))
                        .with_modifiers(ModifiersPattern::Contains(KeyModifiers::CONTROL)),
                ),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::CompletionCancel)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Insert)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::ToggleInsertMode)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::PageDown)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollDown)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::PageUp)),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollUp)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(3))),
                EventBindingOutput::Event(SpfEvent::Command(PrimaryCommand::End)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(5))),
                EventBindingOutput::Event(SpfEvent::Command(PrimaryCommand::RFind)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(7))),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollUp)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(8))),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollDown)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(10))),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollLeft)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(11))),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnScrollRight)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::F(12))),
                EventBindingOutput::Event(SpfEvent::Action(EditorAction::FnRetrieve)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::press(KeyCode::Esc)),
                EventBindingOutput::Event(SpfEvent::Command(PrimaryCommand::End)),
            ),
            EventBinding::new(
                EventMatcher::Key(KeyPattern::any_char_press()),
                EventBindingOutput::InsertChar,
            ),
        ]
    }
}

pub struct InputHandler {
    pub focus: FieldFocus,
    pub mode: InputMode,
    pub enter_mode: EnterMode,
    translator: EventTranslator,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            focus: FieldFocus::CommandLine,
            mode: InputMode::Overtype,
            enter_mode: EnterMode::Legacy,
            translator: EventTranslator::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_bindings(&mut self, bindings: Vec<EventBinding>) {
        self.translator.set_bindings(bindings);
    }

    // Translate a crossterm event into an SPF event.
    pub fn translate_event(&self, event: Event) -> SpfEvent {
        let input_event = SpfInputEvent::from(event);
        let spf_event = self.translator.translate(input_event.clone(), self.enter_mode);
        trace!("input={:?} -> {:?}", input_event, spf_event);
        spf_event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_map_chars_to_insert_actions() {
        let handler = InputHandler::new();
        let event = Event::Key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));

        assert_eq!(
            handler.translate_event(event),
            SpfEvent::Action(EditorAction::InsertChar('x'))
        );
    }

    #[test]
    fn default_bindings_map_resize_to_action() {
        let handler = InputHandler::new();

        assert_eq!(
            handler.translate_event(Event::Resize(132, 43)),
            SpfEvent::Action(EditorAction::Resize(132, 43))
        );
    }

    #[test]
    fn default_bindings_can_execute_commands() {
        let handler = InputHandler::new();
        let event = Event::Key(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE));

        assert_eq!(handler.translate_event(event), SpfEvent::Command(PrimaryCommand::End));
    }

    #[test]
    fn custom_binding_can_emit_event_sequence() {
        let mut handler = InputHandler::new();
        handler.set_bindings(vec![EventBinding::new(
            EventMatcher::Key(
                KeyPattern::press(KeyCode::F(2))
                    .with_modifiers(ModifiersPattern::Exact(KeyModifiers::NONE)),
            ),
            EventBindingOutput::Event(SpfEvent::Sequence(vec![
                SpfEvent::Command(PrimaryCommand::Save),
                SpfEvent::Action(EditorAction::FnRetrieve),
            ])),
        )]);

        assert_eq!(
            handler.translate_event(Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE))),
            SpfEvent::Sequence(vec![
                SpfEvent::Command(PrimaryCommand::Save),
                SpfEvent::Action(EditorAction::FnRetrieve),
            ])
        );
    }

    #[test]
    fn default_bindings_map_focus_changes() {
        let handler = InputHandler::new();

        assert_eq!(
            handler.translate_event(Event::FocusGained),
            SpfEvent::FocusChanged { focused: true }
        );
        assert_eq!(
            handler.translate_event(Event::FocusLost),
            SpfEvent::FocusChanged { focused: false }
        );
    }

    #[test]
    fn default_bindings_map_mouse_events() {
        let handler = InputHandler::new();

        assert_eq!(
            handler.translate_event(Event::Mouse(MouseEvent {
                kind: MouseEventKind::ScrollDown,
                column: 12,
                row: 7,
                modifiers: KeyModifiers::NONE,
            })),
            SpfEvent::Mouse(SpfMouseEvent {
                kind: MouseEventKind::ScrollDown,
                column: 12,
                row: 7,
                modifiers: KeyModifiers::NONE,
            })
        );
    }

    #[test]
    fn unmatched_events_are_still_translated_to_spf_input() {
        let handler = InputHandler::new();

        assert_eq!(
            handler.translate_event(Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Release,
                state: KeyEventState::empty(),
            })),
            SpfEvent::Input(SpfInputEvent::Key(SpfKeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Release,
                state: KeyEventState::empty(),
            }))
        );
    }
}
