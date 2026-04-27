use std::collections::HashMap;
use regex::Regex;
use crate::json_model::{InitSection, ReinitSection, Conditional};

/// Parse the )INIT section.
pub fn parse_init(text: &str) -> (Option<InitSection>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut help_panel = None;
    let mut cursor = None;
    let mut zvars = Vec::new();
    let mut symbols = None;
    let mut assignments = HashMap::new();
    let mut conditionals = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        if trimmed.is_empty() || trimmed.starts_with("/*") {
            i += 1;
            continue;
        }

        let upper = trimmed.to_uppercase();

        // .HELP = "PANELNAME" or .HELP = PANELNAME
        if upper.starts_with(".HELP") {
            help_panel = extract_assignment_value(trimmed);
            i += 1;
            continue;
        }

        // .CURSOR = "FIELDNAME"
        if upper.starts_with(".CURSOR") {
            cursor = extract_assignment_value(trimmed);
            i += 1;
            continue;
        }

        // .ZVARS = "VAR1 VAR2 ..."
        if upper.starts_with(".ZVARS") {
            if let Some(val) = extract_assignment_value(trimmed) {
                zvars = val.split_whitespace().map(|s| s.to_string()).collect();
            }
            i += 1;
            continue;
        }

        // .SYMBOLS = ON/OFF
        if upper.starts_with(".SYMBOLS") {
            if let Some(val) = extract_assignment_value(trimmed) {
                symbols = Some(val.to_uppercase() == "ON");
            }
            i += 1;
            continue;
        }

        // IF ( condition ) ... ENDIF
        if upper.starts_with("IF") {
            let (cond, advance) = parse_conditional(&lines[i..]);
            if let Some(c) = cond {
                conditionals.push(c);
            } else {
                warnings.push(format!("Unparsed IF block at line: {}", trimmed));
            }
            i += advance;
            continue;
        }

        // Variable assignment: &VAR = value
        if trimmed.starts_with('&') {
            if let Some((var, val)) = parse_var_assignment(trimmed) {
                assignments.insert(var, val);
            }
            i += 1;
            continue;
        }

        // Unrecognized line
        if !trimmed.is_empty() {
            warnings.push(format!("Unparsed INIT line: {}", trimmed));
        }
        i += 1;
    }

    // Return None if completely empty
    if help_panel.is_none() && cursor.is_none() && zvars.is_empty()
        && symbols.is_none() && assignments.is_empty() && conditionals.is_empty()
    {
        return (None, warnings);
    }

    (Some(InitSection {
        help_panel,
        cursor,
        zvars,
        symbols,
        assignments,
        conditionals,
    }), warnings)
}

/// Parse the )REINIT section.
pub fn parse_reinit(text: &str) -> (Option<ReinitSection>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut cursor = None;
    let mut assignments = HashMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("/*") {
            continue;
        }

        let upper = trimmed.to_uppercase();

        if upper.starts_with(".CURSOR") {
            cursor = extract_assignment_value(trimmed);
            continue;
        }

        if trimmed.starts_with('&') {
            if let Some((var, val)) = parse_var_assignment(trimmed) {
                assignments.insert(var, val);
            }
            continue;
        }

        if !trimmed.is_empty() {
            warnings.push(format!("Unparsed REINIT line: {}", trimmed));
        }
    }

    if cursor.is_none() && assignments.is_empty() {
        return (None, warnings);
    }

    (Some(ReinitSection { cursor, assignments }), warnings)
}

/// Extract value from `.KEYWORD = "value"` or `.KEYWORD = value`
fn extract_assignment_value(line: &str) -> Option<String> {
    let re = Regex::new(r#"=\s*"?([^"]*)"?"#).unwrap();
    re.captures(line).map(|c| c[1].trim().to_string())
}

/// Parse &VAR = value
fn parse_var_assignment(line: &str) -> Option<(String, String)> {
    let re = Regex::new(r"&(\w+)\s*=\s*(.+)").unwrap();
    re.captures(line).map(|c| {
        let var = c[1].to_string();
        let val = c[2].trim().trim_matches('"').to_string();
        (var, val)
    })
}

/// Parse an IF...ENDIF block, returning a Conditional and the number of lines consumed.
fn parse_conditional(lines: &[&str]) -> (Option<Conditional>, usize) {
    if lines.is_empty() {
        return (None, 0);
    }

    // Extract condition from IF line
    let re = Regex::new(r"(?i)IF\s*\((.+)\)").unwrap();
    let condition = match re.captures(lines[0].trim()) {
        Some(c) => c[1].trim().to_string(),
        None => {
            // Try: IF ( cond )\n on next lines until we find content
            return (None, 1);
        }
    };

    let mut then_assignments = HashMap::new();
    let mut then_cursor = None;
    let mut i = 1;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        let upper = trimmed.to_uppercase();

        if upper.starts_with("ENDIF") || upper == ")" {
            i += 1;
            break;
        }

        if upper.starts_with(".CURSOR") {
            then_cursor = extract_assignment_value(trimmed);
        } else if trimmed.starts_with('&') {
            if let Some((var, val)) = parse_var_assignment(trimmed) {
                then_assignments.insert(var, val);
            }
        }

        i += 1;
    }

    (Some(Conditional {
        condition,
        then_assignments,
        then_cursor,
    }), i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_init_basic() {
        let text = r#".HELP   = "S2R60010"
.CURSOR = "PENVCMD"
.ZVARS  = "ZDOSKEY""#;
        let (init, warnings) = parse_init(text);
        assert!(warnings.is_empty(), "warnings: {:?}", warnings);
        let init = init.unwrap();
        assert_eq!(init.help_panel, Some("S2R60010".to_string()));
        assert_eq!(init.cursor, Some("PENVCMD".to_string()));
        assert_eq!(init.zvars, vec!["ZDOSKEY"]);
    }

    #[test]
    fn test_parse_init_with_zvars_multi() {
        let text = ".ZVARS = \"SCROLL SEL\"";
        let (init, _) = parse_init(text);
        let init = init.unwrap();
        assert_eq!(init.zvars, vec!["SCROLL", "SEL"]);
    }

    #[test]
    fn test_parse_init_with_assignment() {
        let text = "&ZTDMARK = \"        &ZDTFILES  &ZDTBYTES BYTES USED -- END\"";
        let (init, _) = parse_init(text);
        let init = init.unwrap();
        assert!(init.assignments.contains_key("ZTDMARK"));
    }

    #[test]
    fn test_parse_reinit() {
        let text = ".CURSOR = \"PENVCMD\"";
        let (reinit, _) = parse_reinit(text);
        let reinit = reinit.unwrap();
        assert_eq!(reinit.cursor, Some("PENVCMD".to_string()));
    }
}
