use std::collections::HashMap;
use regex::Regex;
use crate::json_model::*;

/// Parse the )PROC section into validations, navigation, and assignments.
pub fn parse_proc(text: &str) -> (Option<ProcSection>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut validations = Vec::new();
    let mut navigation = None;
    let mut assignments = HashMap::new();

    // Join continuation lines (lines that are indented continuations of TRANS calls)
    let joined = join_continuation_lines(text);
    let lines: Vec<&str> = joined.lines().collect();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("/*") {
            continue;
        }

        let upper = trimmed.to_uppercase();

        // VER ( &field, rules... )
        if upper.starts_with("VER") {
            if let Some(v) = parse_ver(trimmed) {
                validations.push(v);
            } else {
                warnings.push(format!("Unparsed VER: {}", trimmed));
            }
            continue;
        }

        // &ZSEL = TRANS( ... ) — navigation
        if upper.contains("TRANS(") || upper.contains("TRANS (") {
            match parse_trans(trimmed) {
                Ok(nav) => navigation = Some(nav),
                Err(e) => warnings.push(format!("TRANS parse error: {}", e)),
            }
            continue;
        }

        // Variable assignment: &VAR = "value"
        if trimmed.starts_with('&') && trimmed.contains('=') && !trimmed.contains("TRANS") {
            if let Some((var, val)) = parse_assignment(trimmed) {
                assignments.insert(var, val);
            }
            continue;
        }

        // .MSG=XXXX (standalone)
        if upper.starts_with(".MSG") {
            continue; // handled within TRANS
        }

        if !trimmed.is_empty() {
            warnings.push(format!("Unparsed PROC line: {}", trimmed));
        }
    }

    if validations.is_empty() && navigation.is_none() && assignments.is_empty() {
        return (None, warnings);
    }

    (Some(ProcSection {
        validations,
        navigation,
        assignments,
    }), warnings)
}

/// Join continuation lines: indented lines that continue a TRANS( call
fn join_continuation_lines(text: &str) -> String {
    let mut result = String::new();
    let mut in_trans = false;
    let mut paren_depth: i32 = 0;

    for line in text.lines() {
        let trimmed = line.trim();

        if in_trans {
            // Continue appending to current line
            result.push(' ');
            result.push_str(trimmed);
            paren_depth += trimmed.chars().filter(|&c| c == '(').count() as i32;
            paren_depth -= trimmed.chars().filter(|&c| c == ')').count() as i32;
            if paren_depth <= 0 {
                in_trans = false;
                result.push('\n');
            }
            continue;
        }

        let upper = trimmed.to_uppercase();
        if upper.contains("TRANS(") || upper.contains("TRANS (") {
            in_trans = true;
            paren_depth = trimmed.chars().filter(|&c| c == '(').count() as i32;
            paren_depth -= trimmed.chars().filter(|&c| c == ')').count() as i32;
            result.push_str(trimmed);
            if paren_depth <= 0 {
                in_trans = false;
                result.push('\n');
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

/// Parse VER ( &field, NB, BOOL ) etc.
fn parse_ver(line: &str) -> Option<Validation> {
    // Extract content inside outer parens
    let re = Regex::new(r"(?i)VER\s*\(\s*(.+)\s*\)").unwrap();
    let caps = re.captures(line)?;
    let inner = caps[1].trim();

    // Split by comma
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return None;
    }

    // First part is the field name: &FIELDNAME
    let field = parts[0].trim_start_matches('&').to_string();
    let mut rules = Vec::new();

    let mut i = 1;
    while i < parts.len() {
        let part = parts[i].to_uppercase();
        match part.as_str() {
            "NB" => rules.push(ValidationRule::NonBlank),
            "BOOL" => rules.push(ValidationRule::Boolean),
            "NUM" => {
                // Check for RANGE
                if i + 1 < parts.len() && parts[i + 1].trim().to_uppercase() == "RANGE" {
                    if i + 3 < parts.len() {
                        let min = parse_range_value(parts[i + 2].trim());
                        let max = parse_range_value(parts[i + 3].trim());
                        rules.push(ValidationRule::Numeric {
                            range: Some(NumericRange { min, max }),
                        });
                        i += 3;
                    } else {
                        rules.push(ValidationRule::Numeric { range: None });
                    }
                } else {
                    rules.push(ValidationRule::Numeric { range: None });
                }
            }
            "ALPHA" => rules.push(ValidationRule::Alpha),
            "HEX" => rules.push(ValidationRule::Hex),
            "PICT" => {
                // Next part is the format string
                if i + 1 < parts.len() {
                    let fmt = parts[i + 1].trim().trim_matches('"').trim_matches('\'').to_string();
                    rules.push(ValidationRule::Picture { format: fmt });
                    i += 1;
                }
            }
            "LIST" => {
                // Remaining parts are the list values
                let values: Vec<String> = parts[i + 1..]
                    .iter()
                    .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                    .collect();
                rules.push(ValidationRule::List { values });
                break;
            }
            "RANGE" => {
                // RANGE without preceding NUM
                if i + 2 < parts.len() {
                    let min = parse_range_value(parts[i + 1].trim());
                    let max = parse_range_value(parts[i + 2].trim());
                    rules.push(ValidationRule::Numeric {
                        range: Some(NumericRange { min, max }),
                    });
                    i += 2;
                }
            }
            _ => {
                // Unknown validation type — skip
            }
        }
        i += 1;
    }

    Some(Validation { field, rules })
}

fn parse_range_value(s: &str) -> RangeValue {
    let s = s.trim_start_matches('&');
    if let Ok(n) = s.parse::<i64>() {
        RangeValue::Literal(n)
    } else {
        RangeValue::Variable(s.to_string())
    }
}

/// Parse TRANS( &ZCMD, 1, 'PANEL(...)' 2, 'PANEL(...)' ... .MSG=XXX)
fn parse_trans(line: &str) -> Result<Navigation, String> {
    // Extract source variable
    let source_re = Regex::new(r"(?i)TRANS\s*\(\s*(?:TRUNC\s*\(\s*)?&?(\w+)").unwrap();
    let source_var = source_re.captures(line)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| "ZCMD".to_string());

    // Extract default error: .MSG=XXXX
    let msg_re = Regex::new(r"\.MSG\s*=\s*(\w+)").unwrap();
    let default_error = msg_re.captures(line).map(|c| c[1].to_string());

    // Extract routes: value, 'ACTION' pairs
    let mut routes = Vec::new();
    let route_re = Regex::new(r"(\S+)\s*,\s*'([^']*)'").unwrap();

    for caps in route_re.captures_iter(line) {
        let value = caps[1].trim().trim_matches(',').to_string();
        let action_str = caps[2].to_string();

        // Skip if value looks like part of the TRANS header
        if value.to_uppercase().contains("TRANS") || value.to_uppercase().contains("TRUNC") {
            continue;
        }

        let action = parse_nav_action(&action_str);
        routes.push(NavRoute { value, action });
    }

    Ok(Navigation {
        source_variable: source_var,
        routes,
        default_error,
    })
}

/// Parse a navigation action string like 'PANEL(S2CHNG01)' or 'LIST=X/Y/Z' or 'UP'
fn parse_nav_action(action: &str) -> NavAction {
    let upper = action.to_uppercase();

    if upper.starts_with("PANEL(") {
        let re = Regex::new(r"(?i)PANEL\((\w+)\)").unwrap();
        if let Some(caps) = re.captures(action) {
            return NavAction::Panel { target: caps[1].to_string() };
        }
    }

    if upper.starts_with("LIST=") {
        let list_str = &action[5..];
        let targets: Vec<String> = list_str.split('/').map(|s| s.trim().to_string()).collect();
        return NavAction::List { targets };
    }

    if upper == "UP" {
        return NavAction::Up;
    }

    if upper.starts_with("CTC(") {
        let re = Regex::new(r"(?i)CTC\((.+)\)").unwrap();
        if let Some(caps) = re.captures(action) {
            return NavAction::Ctc { command: caps[1].to_string() };
        }
    }

    if action.trim().is_empty() || action.trim() == " " {
        return NavAction::Blank;
    }

    // Fallback: treat as panel reference
    NavAction::Panel { target: action.to_string() }
}

fn parse_assignment(line: &str) -> Option<(String, String)> {
    let re = Regex::new(r"&(\w+)\s*=\s*(.+)").unwrap();
    re.captures(line).map(|c| {
        let var = c[1].to_string();
        let val = c[2].trim().trim_matches('"').to_string();
        (var, val)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ver_nb_bool() {
        let v = parse_ver("VER ( &ZDOSKEY, NB, BOOL )").unwrap();
        assert_eq!(v.field, "ZDOSKEY");
        assert_eq!(v.rules.len(), 2);
    }

    #[test]
    fn test_parse_ver_pict() {
        let v = parse_ver("VER ( &PHONEW, PICT, \"999-999-9999\" )").unwrap();
        assert_eq!(v.field, "PHONEW");
        assert_eq!(v.rules.len(), 1);
        match &v.rules[0] {
            ValidationRule::Picture { format } => assert_eq!(format, "999-999-9999"),
            _ => panic!("Expected Picture rule"),
        }
    }

    #[test]
    fn test_parse_ver_num_range() {
        let v = parse_ver("VER( &PLR, NUM, RANGE, 1, &ZMAXLREC )").unwrap();
        assert_eq!(v.field, "PLR");
        match &v.rules[0] {
            ValidationRule::Numeric { range: Some(r) } => {
                match &r.min {
                    RangeValue::Literal(n) => assert_eq!(*n, 1),
                    _ => panic!("Expected literal min"),
                }
            }
            _ => panic!("Expected Numeric with range"),
        }
    }

    #[test]
    fn test_parse_trans_simple() {
        let input = "&ZSEL = TRANS( &ZCMD, 1, 'PANEL(S2CHNG01)' 2, 'PANEL(S2CHNG02)' ' ', ' ' .MSG=ISPF102)";
        let (proc, warnings) = parse_proc(input);
        assert!(proc.is_some(), "warnings: {:?}", warnings);
        let p = proc.unwrap();
        let nav = p.navigation.unwrap();
        assert_eq!(nav.source_variable, "ZCMD");
        assert!(!nav.routes.is_empty());
        assert_eq!(nav.default_error, Some("ISPF102".to_string()));
    }

    #[test]
    fn test_parse_proc_assignment() {
        let input = "&ZSEL = \"UP\"";
        let (proc, _) = parse_proc(input);
        let p = proc.unwrap();
        assert_eq!(p.assignments.get("ZSEL"), Some(&"UP".to_string()));
    }
}
