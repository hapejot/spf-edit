use std::collections::HashMap;
use regex::Regex;
use crate::json_model::{AttributeDef, FieldType, Intensity, Justification};

/// Parse the )ATTR section into a map of marker char → AttributeDef.
pub fn parse_attr(text: &str) -> (HashMap<char, AttributeDef>, Vec<String>) {
    let mut attrs = HashMap::new();
    let mut warnings = Vec::new();

    // Pattern: 'char' TYPE(xxx) [modifiers...]
    // Also handles: char TYPE(xxx) ... (without quotes in some files)
    let line_re = Regex::new(
        r#"(?i)^\s*'?(.)'?\s+TYPE\((\w+)\)(.*)"#
    ).unwrap();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("/*") {
            continue;
        }

        if let Some(caps) = line_re.captures(trimmed) {
            let ch = caps[1].chars().next().unwrap();
            let type_str = caps[2].to_uppercase();
            let modifiers_str = &caps[3];

            let field_type = match type_str.as_str() {
                "PROT" => FieldType::Prot,
                "INPUT" => FieldType::Input,
                "OUTPUT" => FieldType::Output,
                "SEL" => FieldType::Sel,
                other => {
                    warnings.push(format!("Unknown field type: {}", other));
                    FieldType::Prot
                }
            };

            let mods_upper = modifiers_str.to_uppercase();

            let intensity = extract_paren_value(&mods_upper, "INTENS").map(|v| match v.as_str() {
                "HIGH" => Intensity::High,
                "LOW" => Intensity::Low,
                "NON" => Intensity::Non,
                _ => Intensity::High,
            });

            let caps_val = extract_paren_value(&mods_upper, "CAPS").map(|v| v == "ON");
            let scroll = extract_paren_value(&mods_upper, "SCROLL").map(|v| v == "ON");
            let mouse = extract_paren_value(&mods_upper, "MOUSE").map(|v| v == "ON");
            let attn = extract_paren_value(&mods_upper, "ATTN").map(|v| v == "ON");

            let justification = extract_paren_value(&mods_upper, "JUST").map(|v| match v.as_str() {
                "LEFT" => Justification::Left,
                "RIGHT" => Justification::Right,
                "ASIS" => Justification::Asis,
                _ => Justification::Left,
            });

            let picture = extract_paren_value_preserve(modifiers_str, "PICT")
                .map(|v| v.trim_matches('"').trim_matches('\'').to_string());

            attrs.insert(ch, AttributeDef {
                field_type,
                intensity,
                caps: caps_val,
                scroll,
                justification,
                mouse,
                attn,
                picture,
            });
        } else if !trimmed.is_empty() {
            // Try simpler format:  char Type(xxx) ...  (without TYPE keyword prefix check)
            // Some files use just: ~ TYPE(OUTPUT) INTENS(LOW) without quotes
            let simple_re = Regex::new(r#"(?i)^\s*(\S)\s+Type\((\w+)\)(.*)"#).unwrap();
            if let Some(caps) = simple_re.captures(trimmed) {
                let ch = caps[1].chars().next().unwrap();
                let type_str = caps[2].to_uppercase();
                let modifiers_str = &caps[3];

                let field_type = match type_str.as_str() {
                    "PROT" => FieldType::Prot,
                    "INPUT" => FieldType::Input,
                    "OUTPUT" => FieldType::Output,
                    "SEL" => FieldType::Sel,
                    _ => FieldType::Prot,
                };

                let mods_upper = modifiers_str.to_uppercase();
                let intensity = extract_paren_value(&mods_upper, "INTENS").map(|v| match v.as_str() {
                    "HIGH" => Intensity::High,
                    "LOW" => Intensity::Low,
                    "NON" => Intensity::Non,
                    _ => Intensity::High,
                });

                attrs.insert(ch, AttributeDef {
                    field_type,
                    intensity,
                    caps: extract_paren_value(&mods_upper, "CAPS").map(|v| v == "ON"),
                    scroll: extract_paren_value(&mods_upper, "SCROLL").map(|v| v == "ON"),
                    justification: extract_paren_value(&mods_upper, "JUST").map(|v| match v.as_str() {
                        "LEFT" => Justification::Left,
                        "RIGHT" => Justification::Right,
                        "ASIS" => Justification::Asis,
                        _ => Justification::Left,
                    }),
                    mouse: extract_paren_value(&mods_upper, "MOUSE").map(|v| v == "ON"),
                    attn: extract_paren_value(&mods_upper, "ATTN").map(|v| v == "ON"),
                    picture: None,
                });
            } else {
                warnings.push(format!("Unparsed ATTR line: {}", trimmed));
            }
        }
    }

    (attrs, warnings)
}

/// Extract value from KEYWORD(VALUE) pattern (case-insensitive, returns uppercase)
fn extract_paren_value(text: &str, keyword: &str) -> Option<String> {
    let pattern = format!(r"(?i){}\(([^)]+)\)", regex::escape(keyword));
    let re = Regex::new(&pattern).unwrap();
    re.captures(text).map(|c| c[1].trim().to_uppercase())
}

/// Extract value preserving original case (for PICT patterns)
fn extract_paren_value_preserve(text: &str, keyword: &str) -> Option<String> {
    let pattern = format!(r"(?i){}\(([^)]+)\)", regex::escape(keyword));
    let re = Regex::new(&pattern).unwrap();
    re.captures(text).map(|c| c[1].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attr_basic() {
        let input = "  '^' TYPE(INPUT) SCROLL(ON) CAPS(OFF) JUST(LEFT) INTENS(HIGH)";
        let (attrs, warnings) = parse_attr(input);
        assert!(warnings.is_empty());
        let a = attrs.get(&'^').unwrap();
        assert_eq!(a.field_type, FieldType::Input);
        assert_eq!(a.scroll, Some(true));
        assert_eq!(a.caps, Some(false));
    }

    #[test]
    fn test_parse_attr_multiple() {
        let input = "  '~' TYPE(OUTPUT) INTENS(LOW)\n  '^' TYPE(PROT) ATTN(ON) INTENS(HIGH)";
        let (attrs, _) = parse_attr(input);
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs[&'~'].field_type, FieldType::Output);
        assert_eq!(attrs[&'^'].field_type, FieldType::Prot);
    }

    #[test]
    fn test_parse_attr_no_quotes() {
        let input = " ~ TYPE(OUTPUT) INTENS(LOW)";
        let (attrs, _) = parse_attr(input);
        assert!(attrs.contains_key(&'~'));
    }
}
