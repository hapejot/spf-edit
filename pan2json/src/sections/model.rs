use std::collections::HashMap;
use crate::json_model::*;
use crate::parser::is_box_horizontal;

/// Parse the )MODEL section into a ModelDef.
pub fn parse_model(
    text: &str,
    attrs: &HashMap<char, AttributeDef>,
) -> (Option<ModelDef>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut selection_field = None;
    let mut columns = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let chars: Vec<char> = trimmed.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            // Skip line markers and spaces
            if ch == '+' || ch == ' ' {
                i += 1;
                continue;
            }

            // Box-drawing chars (field connectors) — skip
            if is_box_horizontal(ch) || ch == '\u{0010}' {
                i += 1;
                continue;
            }

            // _Z or _SEL → selection field
            if ch == '_' && i + 1 < len && chars[i + 1].is_alphabetic() {
                i += 1;
                let var = collect_identifier(&chars, &mut i);
                if var == "Z" || var.to_uppercase() == "SEL" {
                    selection_field = Some(SelectionField {
                        variable: var,
                        width: 1,
                    });
                } else {
                    // Regular input column
                    columns.push(ModelColumn {
                        variable: var,
                        attribute: None,
                        width: None,
                    });
                }
                continue;
            }

            // &VAR → variable column (usually display)
            if ch == '&' && i + 1 < len && chars[i + 1].is_alphabetic() {
                i += 1;
                let var = collect_identifier(&chars, &mut i);
                // Estimate width from trailing spaces
                let start = i;
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let trailing_spaces = i - start;
                let width = if trailing_spaces > 1 { Some(var.len() + trailing_spaces) } else { None };
                columns.push(ModelColumn {
                    variable: var,
                    attribute: None,
                    width,
                });
                continue;
            }

            // Attr-char followed by VARNAME → data column
            if attrs.contains_key(&ch) && i + 1 < len && chars[i + 1].is_alphabetic() {
                let attr_ch = ch;
                i += 1;
                let var = collect_identifier(&chars, &mut i);
                // Estimate width from trailing spaces
                let start = i;
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let trailing_spaces = i - start;
                let width = if trailing_spaces > 1 { Some(var.len() + trailing_spaces) } else { None };
                columns.push(ModelColumn {
                    variable: var,
                    attribute: Some(attr_ch),
                    width,
                });
                continue;
            }

            // #VARNAME → output column (high intensity)
            if ch == '#' && i + 1 < len && chars[i + 1].is_alphabetic() {
                i += 1;
                let var = collect_identifier(&chars, &mut i);
                let start = i;
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let trailing_spaces = i - start;
                let width = if trailing_spaces > 1 { Some(var.len() + trailing_spaces) } else { None };
                columns.push(ModelColumn {
                    variable: var,
                    attribute: Some('#'),
                    width,
                });
                continue;
            }

            // Skip unrecognized
            i += 1;
        }
    }

    if columns.is_empty() && selection_field.is_none() {
        warnings.push("MODEL section found but no columns parsed".to_string());
        return (None, warnings);
    }

    (Some(ModelDef {
        selection_field,
        columns,
    }), warnings)
}

fn collect_identifier(chars: &[char], i: &mut usize) -> String {
    let mut name = String::new();
    while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
        name.push(chars[*i]);
        *i += 1;
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model_phonemnu() {
        let attrs = HashMap::from([
            ('~', AttributeDef {
                field_type: FieldType::Output,
                intensity: Some(Intensity::Low),
                caps: None, scroll: None, justification: None,
                mouse: None, attn: None, picture: None,
            }),
        ]);
        let text = "_Z+&LNAME, &FNAME   ~COMPANY          ~CITY          ~PHONEW      ~PHONEH";
        let (model, warnings) = parse_model(text, &attrs);
        assert!(model.is_some());
        let m = model.unwrap();
        assert!(m.selection_field.is_some());
        assert!(m.columns.len() >= 4, "Expected at least 4 columns, got {}", m.columns.len());
    }

    #[test]
    fn test_parse_model_color() {
        let attrs = HashMap::from([
            ('~', AttributeDef {
                field_type: FieldType::Output,
                intensity: Some(Intensity::Low),
                caps: None, scroll: None, justification: None,
                mouse: None, attn: None, picture: None,
            }),
        ]);
        let text = "_Z+~CLRNAME  ~CLRDESC";
        let (model, _) = parse_model(text, &attrs);
        assert!(model.is_some());
        let m = model.unwrap();
        assert!(m.selection_field.is_some());
        assert_eq!(m.columns.len(), 2);
        assert_eq!(m.columns[0].variable, "CLRNAME");
    }
}
