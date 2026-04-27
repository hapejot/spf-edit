/// CP437 byte values for box-drawing characters
pub const CP437_SINGLE_HORIZONTAL: u8 = 0xC4; // ─
pub const CP437_DOUBLE_HORIZONTAL: u8 = 0xCD; // ═
pub const CP437_DLE: u8 = 0x10; // field-start marker

/// Known section keywords (case-insensitive)
const SECTION_KEYWORDS: &[&str] = &["ATTR", "BODY", "MODEL", "INIT", "REINIT", "PROC", "PDF", "END"];

/// Raw sections extracted from a PAN file
#[derive(Debug, Default)]
pub struct RawSections {
    pub attr: Option<String>,
    pub body: Option<String>,
    pub model: Option<String>,
    pub init: Option<String>,
    pub reinit: Option<String>,
    pub proc_section: Option<String>,
    pub trailing: Option<String>, // copyright etc. after )END
}

/// Decode raw file bytes. Tries UTF-8 first, falls back to CP437 byte-by-byte.
pub fn decode_pan_bytes(bytes: &[u8]) -> String {
    // Try UTF-8 first
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }
    // Fall back to CP437 mapping
    bytes.iter().map(|&b| cp437_to_char(b)).collect()
}

/// Map a single CP437 byte to a Unicode char.
/// We only need the handful of non-ASCII chars actually used in PAN files.
fn cp437_to_char(b: u8) -> char {
    match b {
        0x10 => '\u{0010}', // DLE - field-start marker, kept as control
        0x00..=0x7F => b as char,
        0xB0 => '\u{2591}', // ░ light shade
        0xB1 => '\u{2592}', // ▒ medium shade
        0xB2 => '\u{2593}', // ▓ dark shade
        0xB3 => '\u{2502}', // │ single vertical
        0xBA => '\u{2551}', // ║ double vertical
        0xBB => '\u{2557}', // ╗
        0xBC => '\u{255D}', // ╝
        0xBF => '\u{2510}', // ┐
        0xC0 => '\u{2514}', // └
        0xC4 => '\u{2500}', // ─ single horizontal
        0xC8 => '\u{2558}', // ╘  (close enough to ╚)
        0xC9 => '\u{2554}', // ╔
        0xCD => '\u{2550}', // ═ double horizontal
        0xD9 => '\u{2518}', // ┘
        0xDA => '\u{250C}', // ┌
        0xDB => '\u{2588}', // █ full block
        _ => '\u{FFFD}',    // replacement char for unmapped
    }
}

/// Check if a character is a box-drawing horizontal line
pub fn is_single_horizontal(ch: char) -> bool {
    ch == '\u{2500}' || ch == '\u{C4}' // ─
}

pub fn is_double_horizontal(ch: char) -> bool {
    ch == '\u{2550}' || ch == '\u{CD}' // ═
}

pub fn is_box_horizontal(ch: char) -> bool {
    is_single_horizontal(ch) || is_double_horizontal(ch)
}

/// Split decoded PAN text into raw sections by `)KEYWORD` markers.
pub fn split_sections(text: &str) -> RawSections {
    let mut sections = RawSections::default();
    let mut current_section: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();
    let mut past_end = false;
    let mut trailing_lines: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        if past_end {
            if !trimmed.is_empty() {
                trailing_lines.push(line.to_string());
            }
            continue;
        }

        // Check for section start: )KEYWORD at start of trimmed line
        if trimmed.starts_with(')') {
            let keyword = trimmed[1..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_uppercase();

            if SECTION_KEYWORDS.contains(&keyword.as_str()) {
                // Flush previous section
                if let Some(ref sect) = current_section {
                    store_section(&mut sections, sect, &current_lines);
                }
                current_lines.clear();

                if keyword == "END" {
                    current_section = None;
                    past_end = true;
                } else if keyword == "PDF" {
                    // )PDF is ignored (print definition), skip
                    current_section = None;
                } else {
                    current_section = Some(keyword);
                }
                continue;
            }
        }

        if current_section.is_some() {
            current_lines.push(line.to_string());
        }
    }

    // Flush last section if no )END was found
    if let Some(ref sect) = current_section {
        store_section(&mut sections, sect, &current_lines);
    }

    if !trailing_lines.is_empty() {
        sections.trailing = Some(trailing_lines.join("\n"));
    }

    sections
}

fn store_section(sections: &mut RawSections, keyword: &str, lines: &[String]) {
    let content = lines.join("\n");
    if content.trim().is_empty() {
        return;
    }
    match keyword {
        "ATTR" => sections.attr = Some(content),
        "BODY" => sections.body = Some(content),
        "MODEL" => sections.model = Some(content),
        "INIT" => sections.init = Some(content),
        "REINIT" => sections.reinit = Some(content),
        "PROC" => sections.proc_section = Some(content),
        _ => {}
    }
}

/// Extract copyright text from trailing content after )END
pub fn extract_copyright(trailing: &Option<String>) -> Option<String> {
    trailing.as_ref().and_then(|t| {
        let text = t.trim();
        if text.contains("Copyright") || text.contains("copyright") {
            Some(text.to_string())
        } else if !text.is_empty() {
            Some(text.to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_simple_panel() {
        let text = ")ATTR\n  '~' TYPE(OUTPUT)\n)BODY\n%Title line\n+text\n)INIT\n.HELP = \"FOO\"\n)END\n(C) Copyright";
        let sections = split_sections(text);
        assert!(sections.attr.is_some());
        assert!(sections.body.is_some());
        assert!(sections.init.is_some());
        assert!(sections.trailing.is_some());
        assert!(sections.attr.unwrap().contains("TYPE(OUTPUT)"));
    }

    #[test]
    fn test_split_case_insensitive() {
        let text = ")attr\n  '~' TYPE(OUTPUT)\n)body\n%Title\n)init\n)end\n";
        let sections = split_sections(text);
        assert!(sections.attr.is_some());
        assert!(sections.body.is_some());
    }

    #[test]
    fn test_cp437_decode() {
        let bytes = &[0x2F, 0xC4, 0x2F]; // /─/
        let decoded = decode_pan_bytes(bytes);
        assert_eq!(decoded, "/\u{2500}/");
        assert!(is_single_horizontal(decoded.chars().nth(1).unwrap()));
    }
}
