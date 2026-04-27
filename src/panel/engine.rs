//! Panel engine: modal event loop for panel interaction.
//!
//! Handles field-by-field input, cursor movement between fields,
//! VER validation on Enter, TRANS navigation dispatch, and F3/UP return.

use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    terminal,
};

use panel_model::*;
use super::renderer::{FieldInfo, PanelRenderer};
use super::vars::VarPool;

/// Result of running a panel's engine loop.
#[derive(Debug, Clone)]
pub enum PanelResult {
    /// User pressed F3 or command resolved to UP — return to previous panel.
    Up,
    /// Navigation to another panel (from TRANS).
    Navigate(String),
    /// Navigation to a list of panels (display in sequence).
    NavigateList(Vec<String>),
    /// A CTC (command-to-caller) was triggered.
    Ctc(String),
    /// Force quit (Ctrl+Q).
    Quit,
}

/// Runs the modal event loop for a single panel display.
pub struct PanelEngine;

impl PanelEngine {
    /// Display a panel and run its input loop until the user presses Enter or F3.
    ///
    /// Returns a `PanelResult` indicating what happened.
    pub fn run<W: Write>(
        stdout: &mut W,
        panel: &Panel,
        vars: &mut VarPool,
    ) -> io::Result<PanelResult> {
        // Apply )INIT section
        Self::apply_init(panel, vars);

        let mut command_text = String::new();
        let mut scroll_text = String::from("PAGE");
        let mut error_msg: Option<String> = None;
        let mut current_field_idx: usize = 0;

        loop {
            // Get terminal size
            let (width, height) = terminal::size()?;

            // Render the panel
            let fields = PanelRenderer::draw(
                stdout,
                panel,
                vars,
                &command_text,
                &scroll_text,
                error_msg.as_deref(),
                width,
                height,
            )?;

            if fields.is_empty() {
                // No input fields — just wait for Enter/F3
                queue!(stdout, MoveTo(0, height.saturating_sub(1)))?;
                stdout.flush()?;
            } else {
                // Position cursor on current field
                if current_field_idx >= fields.len() {
                    current_field_idx = 0;
                }
                let field = &fields[current_field_idx];
                let cursor_col = field.col + field.value.len().min(field.width) as u16;
                queue!(stdout, MoveTo(cursor_col.min(field.col + field.width as u16 - 1), field.row))?;
                stdout.flush()?;
            }

            // Read event
            let evt = event::read()?;
            error_msg = None; // Clear error on any key

            match evt {
                Event::Key(key_event) => {
                    match Self::handle_key(
                        key_event,
                        &mut command_text,
                        &mut scroll_text,
                        &fields,
                        &mut current_field_idx,
                        vars,
                    ) {
                        KeyAction::Continue => {}
                        KeyAction::Enter => {
                            // Collect all field values into vars
                            for f in &fields {
                                if !f.is_command {
                                    vars.set(&f.variable, &f.value);
                                }
                            }

                            // Check for UP command
                            let cmd = command_text.trim().to_uppercase();
                            if cmd == "UP" || cmd == "RETURN" || cmd == "END" {
                                return Ok(PanelResult::Up);
                            }

                            // Run )PROC validation
                            if let Some(ref proc_section) = panel.proc_section {
                                // Check validations
                                if let Some(err) = Self::validate(proc_section, vars) {
                                    error_msg = Some(err);
                                    // Apply )REINIT
                                    Self::apply_reinit(panel, vars);
                                    command_text.clear();
                                    continue;
                                }

                                // Check navigation (TRANS)
                                if let Some(ref nav) = proc_section.navigation {
                                    let val = vars
                                        .get(&nav.source_variable)
                                        .unwrap_or("")
                                        .to_string();
                                    let trimmed = val.trim().to_uppercase();

                                    for route in &nav.routes {
                                        if route.value.to_uppercase() == trimmed {
                                            match &route.action {
                                                NavAction::Panel { target } => {
                                                    command_text.clear();
                                                    return Ok(PanelResult::Navigate(
                                                        target.clone(),
                                                    ));
                                                }
                                                NavAction::List { targets } => {
                                                    command_text.clear();
                                                    return Ok(PanelResult::NavigateList(
                                                        targets.clone(),
                                                    ));
                                                }
                                                NavAction::Up => {
                                                    return Ok(PanelResult::Up);
                                                }
                                                NavAction::Blank => {
                                                    // Do nothing, re-display
                                                    command_text.clear();
                                                    continue;
                                                }
                                                NavAction::Ctc { command } => {
                                                    return Ok(PanelResult::Ctc(
                                                        command.clone(),
                                                    ));
                                                }
                                            }
                                        }
                                    }

                                    // No route matched — show error if configured
                                    if !trimmed.is_empty() {
                                        if let Some(ref err_panel) = nav.default_error {
                                            error_msg =
                                                Some(format!("Invalid selection: {err_panel}"));
                                        } else {
                                            error_msg =
                                                Some("Invalid selection".to_string());
                                        }
                                        command_text.clear();
                                        continue;
                                    }
                                }

                                // Apply assignments
                                for (var, val) in &proc_section.assignments {
                                    let resolved = vars.resolve(val);
                                    vars.set(var, &resolved);
                                }

                                // Check ZSEL = UP assignment
                                if let Some(zsel) = vars.get("ZSEL") {
                                    if zsel.to_uppercase() == "UP" {
                                        return Ok(PanelResult::Up);
                                    }
                                }
                            }

                            // No navigation — just re-display
                            command_text.clear();
                        }
                        KeyAction::F3 => {
                            return Ok(PanelResult::Up);
                        }
                        KeyAction::Quit => {
                            return Ok(PanelResult::Quit);
                        }
                    }
                }
                Event::Resize(_, _) => {
                    // Will re-draw on next iteration
                }
                _ => {}
            }
        }
    }

    /// Handle a key event within the panel engine.
    fn handle_key(
        key: KeyEvent,
        command_text: &mut String,
        _scroll_text: &mut String,
        fields: &[FieldInfo],
        current_field: &mut usize,
        vars: &mut VarPool,
    ) -> KeyAction {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
            return KeyAction::Quit;
        }

        match key.code {
            KeyCode::Enter => KeyAction::Enter,
            KeyCode::Esc | KeyCode::F(3) => KeyAction::F3,

            KeyCode::Tab => {
                // Move to next input field
                if !fields.is_empty() {
                    *current_field = (*current_field + 1) % fields.len();
                }
                KeyAction::Continue
            }
            KeyCode::BackTab => {
                // Move to previous input field
                if !fields.is_empty() {
                    if *current_field == 0 {
                        *current_field = fields.len() - 1;
                    } else {
                        *current_field -= 1;
                    }
                }
                KeyAction::Continue
            }

            KeyCode::Char(ch) => {
                if !fields.is_empty() && *current_field < fields.len() {
                    let field = &fields[*current_field];
                    if field.is_command {
                        command_text.push(ch);
                    } else {
                        // Update the field value in the var pool
                        let mut val = vars
                            .get(&field.variable)
                            .unwrap_or("")
                            .to_string();
                        if val.len() < field.width {
                            val.push(ch);
                            vars.set(&field.variable, &val);
                        }
                    }
                }
                KeyAction::Continue
            }

            KeyCode::Backspace => {
                if !fields.is_empty() && *current_field < fields.len() {
                    let field = &fields[*current_field];
                    if field.is_command {
                        command_text.pop();
                    } else {
                        let mut val = vars
                            .get(&field.variable)
                            .unwrap_or("")
                            .to_string();
                        val.pop();
                        vars.set(&field.variable, &val);
                    }
                }
                KeyAction::Continue
            }

            KeyCode::Delete => {
                // Delete character at cursor (simplified: same as backspace for now)
                if !fields.is_empty() && *current_field < fields.len() {
                    let field = &fields[*current_field];
                    if field.is_command {
                        command_text.pop();
                    } else {
                        let mut val = vars
                            .get(&field.variable)
                            .unwrap_or("")
                            .to_string();
                        val.pop();
                        vars.set(&field.variable, &val);
                    }
                }
                KeyAction::Continue
            }

            KeyCode::Up => {
                // Move to field above (find field with closest col on previous row)
                if !fields.is_empty() && *current_field < fields.len() {
                    let current = &fields[*current_field];
                    let mut best: Option<usize> = None;
                    let mut best_dist = u16::MAX;
                    for (i, f) in fields.iter().enumerate() {
                        if f.row < current.row {
                            let dist = current.row - f.row;
                            if dist < best_dist {
                                best_dist = dist;
                                best = Some(i);
                            }
                        }
                    }
                    if let Some(idx) = best {
                        *current_field = idx;
                    }
                }
                KeyAction::Continue
            }

            KeyCode::Down => {
                if !fields.is_empty() && *current_field < fields.len() {
                    let current = &fields[*current_field];
                    let mut best: Option<usize> = None;
                    let mut best_dist = u16::MAX;
                    for (i, f) in fields.iter().enumerate() {
                        if f.row > current.row {
                            let dist = f.row - current.row;
                            if dist < best_dist {
                                best_dist = dist;
                                best = Some(i);
                            }
                        }
                    }
                    if let Some(idx) = best {
                        *current_field = idx;
                    }
                }
                KeyAction::Continue
            }

            _ => KeyAction::Continue,
        }
    }

    /// Apply )INIT section: set cursor, help panel, ZVARS, assignments.
    fn apply_init(panel: &Panel, vars: &mut VarPool) {
        if let Some(ref init) = panel.init {
            // ZVARS — map positional variables (simplified: just note them)
            // In full ISPF, .ZVARS maps unlabeled fields to variable names.
            // For now we store the zvars list but don't do positional mapping.

            // Assignments
            for (var, val) in &init.assignments {
                vars.set(var, val);
            }

            // Conditionals
            for cond in &init.conditionals {
                if Self::evaluate_condition(&cond.condition, vars) {
                    for (var, val) in &cond.then_assignments {
                        vars.set(var, val);
                    }
                }
            }
        }
    }

    /// Apply )REINIT section after a validation failure.
    fn apply_reinit(panel: &Panel, vars: &mut VarPool) {
        if let Some(ref reinit) = panel.reinit {
            for (var, val) in &reinit.assignments {
                vars.set(var, val);
            }
        }
    }

    /// Run VER validations from )PROC. Returns Some(error_message) on failure.
    fn validate(proc_section: &ProcSection, vars: &VarPool) -> Option<String> {
        for validation in &proc_section.validations {
            let val = vars.get(&validation.field).unwrap_or("").trim().to_string();

            for rule in &validation.rules {
                match rule {
                    ValidationRule::NonBlank => {
                        if val.is_empty() {
                            return Some(format!(
                                "Field {} must not be blank",
                                validation.field
                            ));
                        }
                    }
                    ValidationRule::Boolean => {
                        let upper = val.to_uppercase();
                        if !matches!(upper.as_str(), "Y" | "N" | "YES" | "NO" | "") {
                            return Some(format!(
                                "Field {} must be Y or N",
                                validation.field
                            ));
                        }
                    }
                    ValidationRule::Numeric { range } => {
                        if !val.is_empty() {
                            match val.parse::<i64>() {
                                Ok(n) => {
                                    if let Some(r) = range {
                                        let min_val = Self::resolve_range_value(&r.min, vars);
                                        let max_val = Self::resolve_range_value(&r.max, vars);
                                        if let (Some(lo), Some(hi)) = (min_val, max_val) {
                                            if n < lo || n > hi {
                                                return Some(format!(
                                                    "Field {} must be between {} and {}",
                                                    validation.field, lo, hi
                                                ));
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Some(format!(
                                        "Field {} must be numeric",
                                        validation.field
                                    ));
                                }
                            }
                        }
                    }
                    ValidationRule::Alpha => {
                        if !val.chars().all(|c| c.is_alphabetic() || c == ' ') && !val.is_empty() {
                            return Some(format!(
                                "Field {} must be alphabetic",
                                validation.field
                            ));
                        }
                    }
                    ValidationRule::Hex => {
                        if !val.chars().all(|c| c.is_ascii_hexdigit()) && !val.is_empty() {
                            return Some(format!(
                                "Field {} must be hexadecimal",
                                validation.field
                            ));
                        }
                    }
                    ValidationRule::List { values } => {
                        let upper = val.to_uppercase();
                        if !val.is_empty()
                            && !values.iter().any(|v| v.to_uppercase() == upper)
                        {
                            return Some(format!(
                                "Field {} must be one of: {}",
                                validation.field,
                                values.join(", ")
                            ));
                        }
                    }
                    ValidationRule::Picture { .. } => {
                        // Picture validation is complex — skip for now
                    }
                }
            }
        }

        None
    }

    /// Resolve a range value (literal or variable reference).
    fn resolve_range_value(rv: &RangeValue, vars: &VarPool) -> Option<i64> {
        match rv {
            RangeValue::Literal(n) => Some(*n),
            RangeValue::Variable(name) => vars.get(name).and_then(|v| v.parse().ok()),
        }
    }

    /// Evaluate a simple condition (e.g., "&VAR = value").
    fn evaluate_condition(condition: &str, vars: &VarPool) -> bool {
        let parts: Vec<&str> = condition.splitn(3, ' ').collect();
        if parts.len() == 3 && parts[1] == "=" {
            let left = vars.resolve(parts[0]);
            let right = vars.resolve(parts[2]);
            left.trim().eq_ignore_ascii_case(right.trim())
        } else if parts.len() == 3 && parts[1] == "NE" {
            let left = vars.resolve(parts[0]);
            let right = vars.resolve(parts[2]);
            !left.trim().eq_ignore_ascii_case(right.trim())
        } else {
            false
        }
    }
}

enum KeyAction {
    Continue,
    Enter,
    F3,
    Quit,
}
