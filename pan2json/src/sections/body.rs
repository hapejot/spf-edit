use std::collections::HashMap;
use regex::Regex;
use crate::json_model::*;
use crate::parser::is_box_horizontal;

/// Parse the )BODY section into a Title and list of BodyRows.
pub fn parse_body(
    text: &str,
    attrs: &HashMap<char, AttributeDef>,
) -> (Option<Title>, Vec<BodyRow>, Vec<String>) {
    let mut rows = Vec::new();
    let mut title = None;
    let mut warnings = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    // Detect asterisk box regions
    let mut in_asterisk_box = false;
    let mut box_rows: Vec<BodyRow> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // First line is typically the title
        if i == 0 && line.starts_with('%') {
            title = parse_title_line(line);
            i += 1;
            continue;
        }

        // Detect full-width asterisk box border: %*****...* or  *-----...-*
        if is_asterisk_border(line) {
            if !in_asterisk_box {
                in_asterisk_box = true;
                box_rows.clear();
            } else {
                // Closing border
                in_asterisk_box = false;
                rows.push(BodyRow::Box {
                    style: BoxStyle::Asterisk,
                    rows: std::mem::take(&mut box_rows),
                });
            }
            i += 1;
            continue;
        }

        // Detect announcement box: +  *---...---*
        if is_announcement_border(line) {
            if !in_asterisk_box {
                // Start collecting announcement box
                let mut ann_rows = Vec::new();
                i += 1;
                while i < lines.len() && !is_announcement_border(lines[i]) {
                    if let Some(content) = extract_announcement_content(lines[i]) {
                        ann_rows.push(BodyRow::Text {
                            content,
                            style: None,
                        });
                    }
                    i += 1;
                }
                rows.push(BodyRow::Box {
                    style: BoxStyle::Announcement,
                    rows: ann_rows,
                });
                i += 1; // skip closing border
                continue;
            }
        }

        // Inside asterisk box: parse content between %* + and %*
        if in_asterisk_box {
            if let Some(content) = extract_asterisk_box_content(line) {
                let parsed = parse_content_line(&content, attrs, &mut warnings);
                box_rows.extend(parsed);
            } else {
                box_rows.push(BodyRow::Raw { content: line.to_string() });
            }
            i += 1;
            continue;
        }

        // Blank line
        let stripped = strip_line_marker(line);
        if stripped.trim().is_empty() {
            rows.push(BodyRow::Blank);
            i += 1;
            continue;
        }

        // Column header divider: %/─/% or similar
        if is_column_divider(line) {
            let style = if line.contains('\u{2550}') {
                DividerStyle::Double
            } else {
                DividerStyle::Single
            };
            rows.push(BodyRow::Divider { style });
            i += 1;
            continue;
        }

        // Command line: %COMMAND ═══$ZCMD ... or %OPTION ═══$ZCMD ...
        if let Some(cmd_row) = try_parse_command_line(line) {
            rows.push(cmd_row);
            i += 1;
            continue;
        }

        // Regular content lines
        let parsed = parse_content_line(line, attrs, &mut warnings);
        rows.extend(parsed);
        i += 1;
    }

    // If we were in an unclosed box, emit what we have
    if in_asterisk_box && !box_rows.is_empty() {
        rows.push(BodyRow::Box {
            style: BoxStyle::Asterisk,
            rows: box_rows,
        });
        warnings.push("Unclosed asterisk box".to_string());
    }

    (title, rows, warnings)
}

/// Parse the title line: %&ZPRODTSK /─/ TITLE TEXT /─/ V &ZSHRTVER
/// Or: %&ZPRODTSK CHANGES /─/ TITLE /─/ V &ZSHRTVER
/// Or: %&ZPRODTSK HELP /─/ TITLE /─/ V &ZSHRTVER
/// Or: %&ZPRODTSK EDIT &ZSPFHEAD /─/
fn parse_title_line(line: &str) -> Option<Title> {
    let line = line.trim_start_matches('%');

    // Find box-drawing separators
    let sep_positions: Vec<usize> = line.char_indices()
        .filter(|(_, c)| is_box_horizontal(*c))
        .map(|(i, _)| i)
        .collect();

    if sep_positions.is_empty() {
        // No separators — simple title
        return Some(Title {
            product_var: extract_first_var(line),
            text: line.trim().to_string(),
            version_var: None,
            prefix: None,
        });
    }

    // Find slash-delimited separator regions: /─/ or /═/
    let mut separator_regions = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut ci = 0;
    while ci < chars.len() {
        if chars[ci] == '/' && ci + 2 < chars.len() && is_box_horizontal(chars[ci + 1]) && chars[ci + 2] == '/' {
            separator_regions.push((ci, ci + 2));
            ci += 3;
        } else {
            ci += 1;
        }
    }

    let product_var = extract_first_var(line);

    if separator_regions.len() >= 2 {
        // %&ZPRODTSK [PREFIX] /─/ TITLE /─/ [V &ZSHRTVER]
        let before_first_sep: String = chars[..separator_regions[0].0].iter().collect();
        let between_seps: String = chars[separator_regions[0].1 + 1..separator_regions[1].0].iter().collect();
        let after_second_sep: String = chars[separator_regions[1].1 + 1..].iter().collect();

        let prefix = extract_prefix_text(&before_first_sep);
        let title_text = between_seps.trim().to_string();
        let version_var = extract_version_var(&after_second_sep);

        Some(Title {
            product_var,
            text: title_text,
            version_var,
            prefix,
        })
    } else if separator_regions.len() == 1 {
        // %&ZPRODTSK EDIT &ZSPFHEAD /─/
        let before: String = chars[..separator_regions[0].0].iter().collect();
        let title_text = before.trim().to_string();
        // Remove the variable prefix
        let text = remove_var_prefix(&title_text);

        Some(Title {
            product_var,
            text,
            version_var: None,
            prefix: None,
        })
    } else {
        Some(Title {
            product_var,
            text: line.trim().to_string(),
            version_var: None,
            prefix: None,
        })
    }
}

/// Extract &VARNAME at the start of text
fn extract_first_var(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.starts_with('&') {
        let var: String = trimmed[1..].chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !var.is_empty() {
            return Some(var);
        }
    }
    None
}

/// Remove leading &VAR from text to get the remaining title
fn remove_var_prefix(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.starts_with('&') {
        let rest: String = trimmed[1..].chars()
            .skip_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        rest.trim().to_string()
    } else {
        trimmed.to_string()
    }
}

/// Extract prefix text between product var and first separator (e.g., "CHANGES", "HELP")
fn extract_prefix_text(before_sep: &str) -> Option<String> {
    let text = remove_var_prefix(before_sep.trim());
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Extract version var from text after last separator (e.g., "V &ZSHRTVER")
fn extract_version_var(text: &str) -> Option<String> {
    let re = Regex::new(r"&(\w+)").unwrap();
    re.captures(text.trim()).map(|c| c[1].to_string())
}

/// Try to parse a command line: %COMMAND ═══$ZCMD / /% or %OPTION ═══$ZCMD / /%SCROLL ═══_Z
fn try_parse_command_line(line: &str) -> Option<BodyRow> {
    let text = line.trim();
    if !text.starts_with('%') {
        return None;
    }
    let text = &text[1..]; // strip leading %

    let upper = text.to_uppercase();
    if !upper.starts_with("COMMAND") && !upper.starts_with("OPTION") {
        return None;
    }

    // Find the command variable: $ZCMD or ^ZCMD after connectors
    let cmd_var = extract_command_var(text).unwrap_or_else(|| "ZCMD".to_string());

    // Check for SCROLL field
    let scroll = if upper.contains("SCROLL") {
        Some(ScrollField {
            variable: "SCROLL".to_string(),
        })
    } else {
        None
    };

    Some(BodyRow::Command {
        variable: cmd_var,
        scroll,
    })
}

/// Extract command variable from command line text
fn extract_command_var(text: &str) -> Option<String> {
    // Look for $VARNAME or &VARNAME after box-drawing chars or DLE
    let re = Regex::new(r"[\$&\x{0010}]([A-Za-z]\w*)").unwrap();
    re.captures(text).map(|c| c[1].to_string())
}

/// Check if line is a full-width asterisk border: %*****...* or just *****...*
fn is_asterisk_border(line: &str) -> bool {
    let trimmed = line.trim().trim_start_matches('%');
    trimmed.len() > 10 && trimmed.chars().all(|c| c == '*')
}

/// Check if line is an announcement border: +  *---...---*
fn is_announcement_border(line: &str) -> bool {
    let trimmed = line.trim().trim_start_matches('+').trim();
    if !trimmed.starts_with('*') || !trimmed.ends_with('*') {
        return false;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    inner.len() > 5 && inner.chars().all(|c| c == '-')
}

/// Extract content from inside asterisk box line: %* +CONTENT%*
fn extract_asterisk_box_content(line: &str) -> Option<String> {
    let trimmed = line.trim();
    // Pattern: %* +CONTENT ...%*  or  %* +CONTENT%*
    if trimmed.starts_with("%*") && trimmed.ends_with("%*") {
        let inner = &trimmed[2..trimmed.len() - 2];
        let content = inner.trim_start_matches(' ').trim_start_matches('+');
        Some(content.trim_end().to_string())
    } else if trimmed.starts_with("%*") {
        let inner = &trimmed[2..];
        let content = inner.trim_start_matches(' ').trim_start_matches('+');
        Some(content.trim_end().to_string())
    } else {
        None
    }
}

/// Extract content from inside announcement box line: +  * CONTENT *
fn extract_announcement_content(line: &str) -> Option<String> {
    let trimmed = line.trim().trim_start_matches('+').trim();
    if trimmed.starts_with('*') && trimmed.ends_with('*') {
        let inner = &trimmed[1..trimmed.len() - 1].trim();
        Some(inner.to_string())
    } else {
        None
    }
}

/// Check if line is a column header divider: %/─/% or %/═/%
fn is_column_divider(line: &str) -> bool {
    let trimmed = line.trim();
    // Pattern: %/X/% where X is a box-drawing char
    if trimmed.len() >= 5 && trimmed.starts_with('%') {
        let inner = trimmed.trim_start_matches('%').trim_end_matches('%');
        let chars: Vec<char> = inner.chars().collect();
        if chars.len() == 3 && chars[0] == '/' && is_box_horizontal(chars[1]) && chars[2] == '/' {
            return true;
        }
    }
    false
}

/// Strip leading line marker (+, %, etc.) and return content
fn strip_line_marker(line: &str) -> &str {
    let trimmed = line.trim_end();
    if trimmed.starts_with('+') || trimmed.starts_with('%') {
        &trimmed[1..]
    } else {
        trimmed
    }
}

/// Parse a regular content line into one or more BodyRows.
fn parse_content_line(
    line: &str,
    attrs: &HashMap<char, AttributeDef>,
    _warnings: &mut Vec<String>,
) -> Vec<BodyRow> {
    let mut results = Vec::new();
    let trimmed = line.trim_end();

    if trimmed.is_empty() {
        results.push(BodyRow::Blank);
        return results;
    }

    // Check for inline group: / /.../ / pattern
    if trimmed.contains("/ /") {
        let parts = split_inline_groups(trimmed);
        if parts.len() > 1 {
            let fields: Vec<Field> = parts.iter()
                .filter_map(|p| {
                    let p = p.trim().trim_start_matches('%').trim_end_matches('+');
                    if p.is_empty() { return None; }
                    Some(Field::Text {
                        content: clean_text_content(p),
                        style: None,
                    })
                })
                .collect();
            if !fields.is_empty() {
                results.push(BodyRow::InlineGroup { fields });
                return results;
            }
        }
    }

    // Check for column header: %  NAME  DESCRIPTION  % (all text, styled)
    // We'll use a heuristic: line starts with % and has multiple uppercase words with large gaps

    // Parse fields from the line
    let fields = extract_fields_from_line(trimmed, attrs);

    match fields.len() {
        0 => {
            // Pure text line
            let content = clean_body_line(trimmed);
            if !content.is_empty() {
                results.push(BodyRow::Text { content, style: None });
            } else {
                results.push(BodyRow::Blank);
            }
        }
        1 => {
            // Single field — might be text, input, or output
            match fields.into_iter().next().unwrap() {
                Field::Text { content, style } => {
                    results.push(BodyRow::Text { content, style });
                }
                Field::Input { variable, attribute, width, field_connector } => {
                    results.push(BodyRow::Input {
                        variable,
                        attribute,
                        width,
                        field_connector,
                    });
                }
                Field::Output { variable, attribute: _ } => {
                    results.push(BodyRow::Output {
                        variable,
                        style: None,
                        indent: None,
                    });
                }
            }
        }
        _ => {
            // Multiple fields in a row
            results.push(BodyRow::FieldRow { fields });
        }
    }

    results
}

/// Extract fields from a body line, handling attribute markers, variables, and text.
fn extract_fields_from_line(
    line: &str,
    attrs: &HashMap<char, AttributeDef>,
) -> Vec<Field> {
    let mut fields = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut text_buf = String::new();
    let mut has_field_connector = false;

    while i < len {
        let ch = chars[i];

        // Skip leading/trailing line markers
        if (i == 0 || i == len - 1) && (ch == '+' || ch == '%') {
            i += 1;
            continue;
        }

        // Box-drawing chars → field connector marker
        if is_box_horizontal(ch) {
            has_field_connector = true;
            i += 1;
            // Skip consecutive box chars
            while i < len && is_box_horizontal(chars[i]) {
                i += 1;
            }
            continue;
        }

        // DLE (0x10) field-start marker — skip
        if ch == '\u{0010}' {
            i += 1;
            continue;
        }

        // % attribute marker — toggle highlight context
        if ch == '%' {
            // Flush text buffer
            flush_text_buf(&mut text_buf, &mut fields);
            i += 1;
            continue;
        }

        // Variable reference: &VAR or $VAR followed by identifier
        if (ch == '&' || ch == '$') && i + 1 < len && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_') {
            flush_text_buf(&mut text_buf, &mut fields);
            i += 1;
            let var_name = collect_identifier(&chars, &mut i);

            // Determine if this is input or output based on context
            // $-prefixed vars in command area are typically input fields
            // &-prefixed vars are typically output/display
            if ch == '&' {
                fields.push(Field::Output {
                    variable: var_name,
                    attribute: None,
                });
            } else {
                // $ typically used for scrollable/input fields
                fields.push(Field::Input {
                    variable: var_name,
                    attribute: None,
                    width: None,
                    field_connector: has_field_connector,
                });
                has_field_connector = false;
            }
            continue;
        }

        // Attr-defined marker char followed by variable name
        if attrs.contains_key(&ch) && i + 1 < len && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_') {
            flush_text_buf(&mut text_buf, &mut fields);
            let attr_ch = ch;
            i += 1;
            let var_name = collect_identifier(&chars, &mut i);

            let attr_def = &attrs[&attr_ch];
            match attr_def.field_type {
                FieldType::Input => {
                    fields.push(Field::Input {
                        variable: var_name,
                        attribute: Some(attr_ch),
                        width: None,
                        field_connector: has_field_connector,
                    });
                }
                FieldType::Output | FieldType::Prot => {
                    fields.push(Field::Output {
                        variable: var_name,
                        attribute: Some(attr_ch),
                    });
                }
                FieldType::Sel => {
                    fields.push(Field::Input {
                        variable: var_name,
                        attribute: Some(attr_ch),
                        width: Some(1),
                        field_connector: false,
                    });
                }
            }
            has_field_connector = false;
            continue;
        }

        // _ followed by identifier → input field (default INPUT attr)
        if ch == '_' && i + 1 < len && chars[i + 1].is_alphabetic() {
            flush_text_buf(&mut text_buf, &mut fields);
            i += 1;
            let var_name = collect_identifier(&chars, &mut i);
            // Count trailing underscores/spaces for width
            let mut width = var_name.len();
            while i < len && (chars[i] == ' ' || chars[i] == '_') && i + 1 < len {
                width += 1;
                i += 1;
            }
            fields.push(Field::Input {
                variable: var_name,
                attribute: None,
                width: Some(width),
                field_connector: has_field_connector,
            });
            has_field_connector = false;
            continue;
        }

        // / / delimiter — skip (handled by inline group detection above)
        if ch == '/' && i + 1 < len && chars[i + 1] == ' ' && i + 2 < len && chars[i + 2] == '/' {
            i += 3;
            continue;
        }

        // Regular text character
        text_buf.push(ch);
        i += 1;
    }

    flush_text_buf(&mut text_buf, &mut fields);
    fields
}

fn collect_identifier(chars: &[char], i: &mut usize) -> String {
    let mut name = String::new();
    while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
        name.push(chars[*i]);
        *i += 1;
    }
    name
}

fn flush_text_buf(buf: &mut String, fields: &mut Vec<Field>) {
    let trimmed = buf.trim();
    if !trimmed.is_empty() {
        fields.push(Field::Text {
            content: trimmed.to_string(),
            style: None,
        });
    }
    buf.clear();
}

/// Split line by / / delimiters for inline groups
fn split_inline_groups(line: &str) -> Vec<String> {
    line.split("/ /").map(|s| s.to_string()).collect()
}

/// Clean up a body line, removing leading/trailing markers
fn clean_body_line(line: &str) -> String {
    let mut s = line.trim_end();
    if s.starts_with('+') || s.starts_with('%') {
        s = &s[1..];
    }
    if s.ends_with('+') || s.ends_with('%') {
        s = &s[..s.len() - 1];
    }
    clean_text_content(s)
}

/// Remove attribute markers and box-drawing chars from text
fn clean_text_content(text: &str) -> String {
    let mut result = String::new();
    let mut prev_space = false;
    for ch in text.chars() {
        if is_box_horizontal(ch) || ch == '\u{0010}' {
            continue;
        }
        if ch == '%' {
            continue;
        }
        if ch == ' ' && prev_space {
            result.push(ch);
            continue;
        }
        prev_space = ch == ' ';
        result.push(ch);
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_title_with_separators() {
        let line = "%&ZPRODTSK /\u{2500}/ COMMAND PROCESSING /\u{2500}/ V &ZSHRTVER";
        let title = parse_title_line(line).unwrap();
        assert_eq!(title.product_var, Some("ZPRODTSK".to_string()));
        assert_eq!(title.text, "COMMAND PROCESSING");
        assert_eq!(title.version_var, Some("ZSHRTVER".to_string()));
    }

    #[test]
    fn test_parse_title_with_prefix() {
        let line = "%&ZPRODTSK HELP /\u{2500}/ FILE SEARCH PANEL /\u{2500}/ V &ZSHRTVER";
        let title = parse_title_line(line).unwrap();
        assert_eq!(title.prefix, Some("HELP".to_string()));
        assert_eq!(title.text, "FILE SEARCH PANEL");
    }

    #[test]
    fn test_is_column_divider() {
        assert!(is_column_divider("%/\u{2500}/%"));
        assert!(!is_column_divider("%just text%"));
    }

    #[test]
    fn test_is_asterisk_border() {
        assert!(is_asterisk_border("%******************************************************************************"));
        assert!(!is_asterisk_border("%* content *"));
    }

    #[test]
    fn test_announcement_border() {
        assert!(is_announcement_border("+  *-----------------------------------------------------------*"));
        assert!(!is_announcement_border("+  * Some content *"));
    }
}
