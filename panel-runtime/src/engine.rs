//! Panel engine: modal event loop for panel interaction.
//!
//! Handles field-by-field input, cursor movement between fields,
//! VER validation on Enter, TRANS navigation dispatch, and F3/UP return.

use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    queue, terminal,
};

use crate::renderer::{FieldInfo, PanelRenderer};
use crate::vars::VarPool;
use panel_model::*;
use tracing::{debug, info, warn};

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
        info!(panel_id = %panel.id, "running panel engine");

        // Load saved profile variables first so )INIT defaults
        // only apply to variables that have no saved value.
        vars.load_profile(&panel.id);

        // Apply )INIT section (skips vars already set by profile)
        Self::apply_init(panel, vars);

        vars.dump();
        let mut command_text = String::new();
        let mut scroll_text = String::from("PAGE");
        let mut error_msg: Option<String> = None;
        let mut current_field_idx: usize = 0;
        // Last command submitted via Enter — used to implement RETRIEVE.
        let mut last_submitted = String::new();

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
                queue!(
                    stdout,
                    MoveTo(
                        cursor_col.min(field.col + field.width as u16 - 1),
                        field.row
                    )
                )?;
                stdout.flush()?;
            }

            // Read event
            let evt = event::read()?;
            error_msg = None; // Clear error on any key

            match evt {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    let mut action = Self::handle_key(
                        key_event,
                        &mut command_text,
                        &mut scroll_text,
                        &fields,
                        &mut current_field_idx,
                        vars,
                    );

                    // Remap PF keys to standard actions, applying per-panel
                    // overrides on top of the runtime defaults.
                    if let KeyAction::PfKey(n) = action {
                        let cmd = Self::resolve_pf_key(panel, vars, n);
                        debug!(pf = n, command = %cmd, "PF key pressed");
                        if cmd.is_empty() {
                            action = KeyAction::Continue;
                        } else if cmd.eq_ignore_ascii_case("RETRIEVE") {
                            command_text = last_submitted.clone();
                            action = KeyAction::Continue;
                        } else if cmd.eq_ignore_ascii_case("HELP") {
                            if let Some(target) = panel
                                .init
                                .as_ref()
                                .and_then(|i| i.help_panel.clone())
                            {
                                Self::save_field_profile(vars, &panel.id, &fields);
                                return Ok(PanelResult::Navigate(target));
                            }
                            action = KeyAction::Continue;
                        } else if cmd.eq_ignore_ascii_case("END")
                            || cmd.eq_ignore_ascii_case("RETURN")
                            || cmd.eq_ignore_ascii_case("UP")
                        {
                            action = KeyAction::F3;
                        } else {
                            command_text = cmd;
                            action = KeyAction::Enter;
                        }
                    }

                    match action {
                        KeyAction::Continue => {}
                        KeyAction::PfKey(_) => {}
                        KeyAction::Enter => {
                            // Remember the submitted command for RETRIEVE.
                            let trimmed = command_text.trim().to_string();
                            if !trimmed.is_empty() {
                                last_submitted = trimmed;
                            }
                            // Store command text in var pool for TRANS navigation.
                            // Non-command field values are already up-to-date in
                            // vars (typed in real-time via handle_key), so we only
                            // need to sync the command field.
                            for f in &fields {
                                if f.is_command {
                                    vars.set(&f.variable, command_text.trim());
                                }
                            }

                            // Check for UP command
                            let cmd = command_text.trim().to_uppercase();
                            debug!(command = %cmd, panel_id = %panel.id, "user pressed Enter");
                            if cmd == "UP" || cmd == "RETURN" || cmd == "END" {
                                Self::save_field_profile(vars, &panel.id, &fields);
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
                                    let val =
                                        vars.get(&nav.source_variable).unwrap_or("").to_string();
                                    let trimmed = val.trim().to_uppercase();
                                    debug!(source_var = %nav.source_variable, value = %trimmed, "evaluating TRANS routes");

                                    for route in &nav.routes {
                                        if route.value.to_uppercase() == trimmed {
                                            match &route.action {
                                                NavAction::Panel { target } => {
                                                    info!(target = %target, "TRANS -> panel");
                                                    Self::save_field_profile(
                                                        vars, &panel.id, &fields,
                                                    );
                                                    command_text.clear();
                                                    return Ok(PanelResult::Navigate(
                                                        target.clone(),
                                                    ));
                                                }
                                                NavAction::List { targets } => {
                                                    Self::save_field_profile(
                                                        vars, &panel.id, &fields,
                                                    );
                                                    command_text.clear();
                                                    return Ok(PanelResult::NavigateList(
                                                        targets.clone(),
                                                    ));
                                                }
                                                NavAction::Up => {
                                                    Self::save_field_profile(
                                                        vars, &panel.id, &fields,
                                                    );
                                                    return Ok(PanelResult::Up);
                                                }
                                                NavAction::Blank => {
                                                    // Do nothing, re-display
                                                    command_text.clear();
                                                    continue;
                                                }
                                                NavAction::Ctc { command } => {
                                                    info!(command = %command, "TRANS -> CTC");
                                                    Self::save_field_profile(
                                                        vars, &panel.id, &fields,
                                                    );
                                                    return Ok(PanelResult::Ctc(command.clone()));
                                                }
                                            }
                                        }
                                    }

                                    // No route matched — show error if configured
                                    if !trimmed.is_empty() {
                                        warn!(value = %trimmed, "no TRANS route matched");
                                        if let Some(ref err_panel) = nav.default_error {
                                            error_msg =
                                                Some(format!("Invalid selection: {err_panel}"));
                                        } else {
                                            error_msg = Some("Invalid selection".to_string());
                                        }
                                        command_text.clear();
                                        continue;
                                    }
                                }

                                // Apply assignments
                                for (var, val) in &proc_section.assignments {
                                    debug!("applying assignment: {} = {}", var, val);
                                    let resolved = vars.resolve(val);
                                    vars.set(var, &resolved);
                                }

                                // Check ZSEL = UP assignment
                                if let Some(zsel) = vars.get("ZSEL") {
                                    if zsel.to_uppercase() == "UP" {
                                        Self::save_field_profile(vars, &panel.id, &fields);
                                        return Ok(PanelResult::Up);
                                    }
                                }
                            }

                            // No navigation — just re-display
                            command_text.clear();
                        }
                        KeyAction::F3 => {
                            debug!(panel_id = %panel.id, "F3/ESC pressed — returning UP");
                            Self::save_field_profile(vars, &panel.id, &fields);
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

    /// Save non-command field values into the per-panel profile.
    fn save_field_profile(vars: &mut VarPool, panel_id: &str, fields: &[FieldInfo]) {
        let field_vars: Vec<(String, String)> = fields
            .iter()
            .filter(|f| !f.is_command)
            .map(|f| {
                (
                    f.variable.clone(),
                    vars.get(&f.variable).unwrap_or("").to_string(),
                )
            })
            .collect();
        if !field_vars.is_empty() {
            vars.save_profile(panel_id, &field_vars);
        }
    }

    /// Resolve a PF key (1..24) to its command string.
    /// Per-panel `pfkeys` overrides take precedence over the runtime
    /// defaults stored in the variable pool.
    fn resolve_pf_key(panel: &Panel, vars: &VarPool, n: u8) -> String {
        let key = format!("F{n}");
        if let Some((_, def)) = panel
            .pfkeys
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(&key))
        {
            return def.command.clone();
        }
        vars.pf_key(n).map(|d| d.command.clone()).unwrap_or_default()
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
            // Numpad Enter has the ENTER (submit) semantic; the regular
            // Enter key is a no-op here so it can be used as a literal
            // newline in editor contexts. Disambiguation requires
            // `KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`
            // (kitty protocol).
            //
            // Windows note: crossterm 0.28 on Windows reads console
            // events directly through the WinAPI and does NOT surface
            // the ENHANCED_KEY bit, so both Enter keys arrive as
            // `KeyCode::Enter` with no `KEYPAD` state. To keep panels
            // usable there, we treat plain Enter as submit on Windows.
            KeyCode::Enter => {
                if key.state.contains(KeyEventState::KEYPAD) || cfg!(windows) {
                    KeyAction::Enter
                } else {
                    KeyAction::Continue
                }
            }
            KeyCode::Esc | KeyCode::F(3) => KeyAction::F3,
            KeyCode::F(n) if (1..=24).contains(&n) => KeyAction::PfKey(n),

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
                        let mut val = vars.get(&field.variable).unwrap_or("").to_string();
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
                        let mut val = vars.get(&field.variable).unwrap_or("").to_string();
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
                        let mut val = vars.get(&field.variable).unwrap_or("").to_string();
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
                if vars.get(var).is_none() {
                    vars.set(var, val);
                }
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
                            return Some(format!("Field {} must not be blank", validation.field));
                        }
                    }
                    ValidationRule::Boolean => {
                        let upper = val.to_uppercase();
                        if !matches!(upper.as_str(), "Y" | "N" | "YES" | "NO" | "") {
                            return Some(format!("Field {} must be Y or N", validation.field));
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
                            return Some(format!("Field {} must be alphabetic", validation.field));
                        }
                    }
                    ValidationRule::Hex => {
                        if !val.chars().all(|c| c.is_ascii_hexdigit()) && !val.is_empty() {
                            return Some(format!("Field {} must be hexadecimal", validation.field));
                        }
                    }
                    ValidationRule::List { values } => {
                        let upper = val.to_uppercase();
                        if !val.is_empty() && !values.iter().any(|v| v.to_uppercase() == upper) {
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
    /// PF key 1..24 was pressed.
    PfKey(u8),
}
