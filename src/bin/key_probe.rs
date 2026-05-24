use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

fn main() -> io::Result<()> {
    println!("Terminal Key Probe");
    println!("Press keys to inspect what arrives over the terminal connection.");
    println!("Exit with Ctrl+Q or Ctrl+C.\\n");

    enable_raw_mode()?;
    let _raw_mode_guard = RawModeGuard;

    let mut stdout = io::stdout();
    loop {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if is_exit_chord(key) {
                    println!("exit");
                    break;
                }

                let formatted = to_vscode_key_string(key);
                println!("raw={key:?} => key=\"{formatted}\"");
                stdout.flush()?;
            }
            Event::Resize(w, h) => {
                println!("resize: {w}x{h}");
                stdout.flush()?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn is_exit_chord(key: KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('c') | KeyCode::Char('C'))
}

fn to_vscode_key_string(key: KeyEvent) -> String {
    let mut mods = key.modifiers;

    let (base_key, inferred_shift) = map_key_code(key.code);
    if inferred_shift {
        mods.insert(KeyModifiers::SHIFT);
    }

    let mut parts: Vec<&str> = Vec::new();
    if mods.contains(KeyModifiers::CONTROL) {
        parts.push("ctrl");
    }
    if mods.contains(KeyModifiers::SHIFT) {
        parts.push("shift");
    }
    if mods.contains(KeyModifiers::ALT) {
        parts.push("alt");
    }
    if mods.contains(KeyModifiers::SUPER) {
        parts.push("meta");
    }
    parts.push(base_key);

    parts.join("+")
}

fn map_key_code(code: KeyCode) -> (&'static str, bool) {
    match code {
        KeyCode::Backspace => ("backspace", false),
        KeyCode::Enter => ("enter", false),
        KeyCode::Left => ("left", false),
        KeyCode::Right => ("right", false),
        KeyCode::Up => ("up", false),
        KeyCode::Down => ("down", false),
        KeyCode::Home => ("home", false),
        KeyCode::End => ("end", false),
        KeyCode::PageUp => ("pageup", false),
        KeyCode::PageDown => ("pagedown", false),
        KeyCode::Tab => ("tab", false),
        KeyCode::BackTab => ("tab", true),
        KeyCode::Delete => ("delete", false),
        KeyCode::Insert => ("insert", false),
        KeyCode::Esc => ("escape", false),
        KeyCode::F(n) => function_key_name(n),
        KeyCode::Char(ch) => map_char_key(ch),
        KeyCode::Null => ("null", false),
        KeyCode::CapsLock => ("capslock", false),
        KeyCode::ScrollLock => ("scrolllock", false),
        KeyCode::NumLock => ("numlock", false),
        KeyCode::PrintScreen => ("printscreen", false),
        KeyCode::Pause => ("pause", false),
        KeyCode::Menu => ("contextmenu", false),
        KeyCode::KeypadBegin => ("numpad5", false),
        _ => ("unknown", false),
    }
}

fn function_key_name(n: u8) -> (&'static str, bool) {
    match n {
        1 => ("f1", false),
        2 => ("f2", false),
        3 => ("f3", false),
        4 => ("f4", false),
        5 => ("f5", false),
        6 => ("f6", false),
        7 => ("f7", false),
        8 => ("f8", false),
        9 => ("f9", false),
        10 => ("f10", false),
        11 => ("f11", false),
        12 => ("f12", false),
        13 => ("f13", false),
        14 => ("f14", false),
        15 => ("f15", false),
        16 => ("f16", false),
        17 => ("f17", false),
        18 => ("f18", false),
        19 => ("f19", false),
        20 => ("f20", false),
        21 => ("f21", false),
        22 => ("f22", false),
        23 => ("f23", false),
        24 => ("f24", false),
        _ => ("f", false),
    }
}

fn map_char_key(ch: char) -> (&'static str, bool) {
    match ch {
        'a' | 'A' => ("a", ch.is_ascii_uppercase()),
        'b' | 'B' => ("b", ch.is_ascii_uppercase()),
        'c' | 'C' => ("c", ch.is_ascii_uppercase()),
        'd' | 'D' => ("d", ch.is_ascii_uppercase()),
        'e' | 'E' => ("e", ch.is_ascii_uppercase()),
        'f' | 'F' => ("f", ch.is_ascii_uppercase()),
        'g' | 'G' => ("g", ch.is_ascii_uppercase()),
        'h' | 'H' => ("h", ch.is_ascii_uppercase()),
        'i' | 'I' => ("i", ch.is_ascii_uppercase()),
        'j' | 'J' => ("j", ch.is_ascii_uppercase()),
        'k' | 'K' => ("k", ch.is_ascii_uppercase()),
        'l' | 'L' => ("l", ch.is_ascii_uppercase()),
        'm' | 'M' => ("m", ch.is_ascii_uppercase()),
        'n' | 'N' => ("n", ch.is_ascii_uppercase()),
        'o' | 'O' => ("o", ch.is_ascii_uppercase()),
        'p' | 'P' => ("p", ch.is_ascii_uppercase()),
        'q' | 'Q' => ("q", ch.is_ascii_uppercase()),
        'r' | 'R' => ("r", ch.is_ascii_uppercase()),
        's' | 'S' => ("s", ch.is_ascii_uppercase()),
        't' | 'T' => ("t", ch.is_ascii_uppercase()),
        'u' | 'U' => ("u", ch.is_ascii_uppercase()),
        'v' | 'V' => ("v", ch.is_ascii_uppercase()),
        'w' | 'W' => ("w", ch.is_ascii_uppercase()),
        'x' | 'X' => ("x", ch.is_ascii_uppercase()),
        'y' | 'Y' => ("y", ch.is_ascii_uppercase()),
        'z' | 'Z' => ("z", ch.is_ascii_uppercase()),

        '1' | '!' => ("1", ch == '!'),
        '2' | '@' => ("2", ch == '@'),
        '3' | '#' => ("3", ch == '#'),
        '4' | '$' => ("4", ch == '$'),
        '5' | '%' => ("5", ch == '%'),
        '6' | '^' => ("6", ch == '^'),
        '7' | '&' => ("7", ch == '&'),
        '8' | '*' => ("8", ch == '*'),
        '9' | '(' => ("9", ch == '('),
        '0' | ')' => ("0", ch == ')'),

        '-' | '_' => ("oem_minus", ch == '_'),
        '=' | '+' => ("oem_plus", ch == '+'),
        '[' | '{' => ("oem_4", ch == '{'),
        ']' | '}' => ("oem_6", ch == '}'),
        '\\' | '|' => ("oem_5", ch == '|'),
        ';' | ':' => ("oem_1", ch == ':'),
        '\'' | '"' => ("oem_7", ch == '"'),
        ',' | '<' => ("oem_comma", ch == '<'),
        '.' | '>' => ("oem_period", ch == '>'),
        '/' | '?' => ("oem_2", ch == '?'),
        '`' | '~' => ("oem_3", ch == '~'),

        ' ' => ("space", false),
        _ => ("unknown", false),
    }
}