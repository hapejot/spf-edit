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

/// Low-level key actions emitted by `handle_key`.
enum KeyAction {
    Continue,
    Enter,
    F3,
    Quit,
    /// PF key 1..24 was pressed.
    PfKey(u8),
}

/// Result of resolving a PF key into a follow-up action.
enum PfOutcome {
    /// Continue the loop with this remapped key action.
    Action(KeyAction),
    /// Exit the panel loop with this result (e.g. HELP navigation).
    Result(PanelResult),
}

/// Result of processing an Enter submission against a panel's )PROC.
enum EnterOutcome {
    /// Exit the panel loop with this result.
    Result(PanelResult),
    /// Show this error and re-display the panel.
    Error(String),
    /// Re-display the panel (clear command text).
    Done,
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
            let (width, height) = terminal::size()?;
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

            if !fields.is_empty() && current_field_idx >= fields.len() {
                current_field_idx = 0;
            }
            Self::position_cursor(stdout, &fields, current_field_idx, height)?;

            let evt = event::read()?;
            error_msg = None; // Clear error on any key

            let key_event = match evt {
                Event::Key(k) if k.kind == KeyEventKind::Press => k,
                _ => continue, // Resize / unhandled events: just re-draw.
            };

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
                match Self::process_pf_key(
                    panel,
                    vars,
                    n,
                    &mut command_text,
                    &last_submitted,
                    &fields,
                ) {
                    PfOutcome::Action(a) => action = a,
                    PfOutcome::Result(r) => return Ok(r),
                }
            }

            match action {
                KeyAction::Continue | KeyAction::PfKey(_) => {}
                KeyAction::Enter => {
                    Self::sync_command_field(&command_text, &fields, vars);

                    // Remember the submitted command for RETRIEVE.
                    let trimmed = command_text.trim().to_string();
                    if !trimmed.is_empty() {
                        last_submitted = trimmed;
                    }

                    debug!(command = %command_text.trim(), panel_id = %panel.id, "user pressed Enter");

                    match Self::process_enter(panel, vars, &command_text, &fields) {
                        EnterOutcome::Result(r) => return Ok(r),
                        EnterOutcome::Error(msg) => {
                            error_msg = Some(msg);
                            command_text.clear();
                        }
                        EnterOutcome::Done => {
                            command_text.clear();
                        }
                    }
                }
                KeyAction::F3 => {
                    debug!(panel_id = %panel.id, "F3/ESC pressed — returning UP");
                    Self::save_field_profile(vars, &panel.id, &fields);
                    return Ok(PanelResult::Up);
                }
                KeyAction::Quit => return Ok(PanelResult::Quit),
            }
        }
    }

    // ─── Render helpers ─────────────────────────────────────────────────

    /// Position the terminal cursor on the active field (or bottom row when none).
    fn position_cursor<W: Write>(
        stdout: &mut W,
        fields: &[FieldInfo],
        current: usize,
        height: u16,
    ) -> io::Result<()> {
        if fields.is_empty() {
            queue!(stdout, MoveTo(0, height.saturating_sub(1)))?;
        } else {
            let idx = if current >= fields.len() { 0 } else { current };
            let field = &fields[idx];
            let cursor_col = field.col + field.value.len().min(field.width) as u16;
            queue!(
                stdout,
                MoveTo(
                    cursor_col.min(field.col + field.width as u16 - 1),
                    field.row,
                )
            )?;
        }
        stdout.flush()
    }

    /// Sync the command-line text into any field marked `is_command`.
    /// Non-command field values are already kept up-to-date via `handle_key`.
    fn sync_command_field(command_text: &str, fields: &[FieldInfo], vars: &mut VarPool) {
        for f in fields {
            if f.is_command {
                vars.set(&f.variable, command_text.trim());
            }
        }
    }

    // ─── PF-key resolution ──────────────────────────────────────────────

    /// Resolve a pressed PF key into a follow-up action (or terminating result).
    fn process_pf_key(
        panel: &Panel,
        vars: &mut VarPool,
        n: u8,
        command_text: &mut String,
        last_submitted: &str,
        fields: &[FieldInfo],
    ) -> PfOutcome {
        let cmd = Self::resolve_pf_key(panel, vars, n);
        debug!(pf = n, command = %cmd, "PF key pressed");

        if cmd.is_empty() {
            return PfOutcome::Action(KeyAction::Continue);
        }
        if cmd.eq_ignore_ascii_case("RETRIEVE") {
            *command_text = last_submitted.to_string();
            return PfOutcome::Action(KeyAction::Continue);
        }
        if cmd.eq_ignore_ascii_case("HELP") {
            if let Some(target) = panel.init.as_ref().and_then(|i| i.help_panel.clone()) {
                Self::save_field_profile(vars, &panel.id, fields);
                return PfOutcome::Result(PanelResult::Navigate(target));
            }
            return PfOutcome::Action(KeyAction::Continue);
        }
        if cmd.eq_ignore_ascii_case("END")
            || cmd.eq_ignore_ascii_case("RETURN")
            || cmd.eq_ignore_ascii_case("UP")
        {
            return PfOutcome::Action(KeyAction::F3);
        }
        *command_text = cmd;
        PfOutcome::Action(KeyAction::Enter)
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

    // ─── Enter / )PROC processing ───────────────────────────────────────

    /// Run a panel's )PROC pipeline against the current variables: special
    /// commands (UP/RETURN/END), VER validation, TRANS navigation, assignments,
    /// and the magic ZSEL=UP shortcut.
    fn process_enter(
        panel: &Panel,
        vars: &mut VarPool,
        command_text: &str,
        fields: &[FieldInfo],
    ) -> EnterOutcome {
        let cmd = command_text.trim().to_uppercase();
        if matches!(cmd.as_str(), "UP" | "RETURN" | "END") {
            Self::save_field_profile(vars, &panel.id, fields);
            return EnterOutcome::Result(PanelResult::Up);
        }

        let Some(proc_section) = panel.proc_section.as_ref() else {
            return EnterOutcome::Done;
        };

        // VER validation
        if let Some(err) = Self::validate(proc_section, vars) {
            Self::apply_reinit(panel, vars);
            return EnterOutcome::Error(err);
        }

        // TRANS navigation
        if let Some(nav) = proc_section.navigation.as_ref() {
            if let Some(outcome) = Self::dispatch_navigation(nav, vars, panel, fields) {
                return outcome;
            }
        }

        // Assignments
        for (var, val) in &proc_section.assignments {
            debug!("applying assignment: {} = {}", var, val);
            let resolved = vars.resolve(val);
            vars.set(var, &resolved);
        }

        // ZSEL = UP shortcut
        if let Some(zsel) = vars.get("ZSEL") {
            if zsel.eq_ignore_ascii_case("UP") {
                Self::save_field_profile(vars, &panel.id, fields);
                return EnterOutcome::Result(PanelResult::Up);
            }
        }

        EnterOutcome::Done
    }

    /// Evaluate a TRANS section against the current variables.
    ///
    /// Returns `Some(outcome)` when a route matched (or an unmatched non-empty
    /// value triggered the default error). Returns `None` when no route matched
    /// and the source value was empty — caller should fall through to assignments.
    fn dispatch_navigation(
        nav: &Navigation,
        vars: &mut VarPool,
        panel: &Panel,
        fields: &[FieldInfo],
    ) -> Option<EnterOutcome> {
        let val = vars.get(&nav.source_variable).unwrap_or("").to_string();
        let trimmed = val.trim().to_uppercase();
        debug!(source_var = %nav.source_variable, value = %trimmed, "evaluating TRANS routes");

        for route in &nav.routes {
            if route.value.to_uppercase() != trimmed {
                continue;
            }
            return Some(match &route.action {
                NavAction::Panel { target } => {
                    info!(target = %target, "TRANS -> panel");
                    Self::save_field_profile(vars, &panel.id, fields);
                    EnterOutcome::Result(PanelResult::Navigate(target.clone()))
                }
                NavAction::List { targets } => {
                    Self::save_field_profile(vars, &panel.id, fields);
                    EnterOutcome::Result(PanelResult::NavigateList(targets.clone()))
                }
                NavAction::Up => {
                    Self::save_field_profile(vars, &panel.id, fields);
                    EnterOutcome::Result(PanelResult::Up)
                }
                NavAction::Blank => EnterOutcome::Done,
                NavAction::Ctc { command } => {
                    info!(command = %command, "TRANS -> CTC");
                    Self::save_field_profile(vars, &panel.id, fields);
                    EnterOutcome::Result(PanelResult::Ctc(command.clone()))
                }
            });
        }

        if !trimmed.is_empty() {
            warn!(value = %trimmed, "no TRANS route matched");
            let msg = match nav.default_error.as_ref() {
                Some(err) => format!("Invalid selection: {err}"),
                None => "Invalid selection".to_string(),
            };
            return Some(EnterOutcome::Error(msg));
        }

        None
    }

    // ─── Profile / )INIT / )REINIT ──────────────────────────────────────

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

    /// Apply )INIT section: assignments (only for unset vars) and conditionals.
    fn apply_init(panel: &Panel, vars: &mut VarPool) {
        let Some(init) = panel.init.as_ref() else {
            return;
        };

        // ZVARS — map positional variables (simplified: just note them)
        // In full ISPF, .ZVARS maps unlabeled fields to variable names.
        // For now we store the zvars list but don't do positional mapping.

        for (var, val) in &init.assignments {
            if vars.get(var).is_none() {
                vars.set(var, val);
            }
        }

        for cond in &init.conditionals {
            if Self::evaluate_condition(&cond.condition, vars) {
                for (var, val) in &cond.then_assignments {
                    vars.set(var, val);
                }
            }
        }
    }

    /// Apply )REINIT section after a validation failure.
    fn apply_reinit(panel: &Panel, vars: &mut VarPool) {
        if let Some(reinit) = panel.reinit.as_ref() {
            for (var, val) in &reinit.assignments {
                vars.set(var, val);
            }
        }
    }

    // ─── Validation ─────────────────────────────────────────────────────

    /// Run VER validations from )PROC. Returns `Some(error_message)` on failure.
    fn validate(proc_section: &ProcSection, vars: &VarPool) -> Option<String> {
        for validation in &proc_section.validations {
            let val = vars.get(&validation.field).unwrap_or("").trim().to_string();
            for rule in &validation.rules {
                if let Some(err) = Self::check_rule(&validation.field, &val, rule, vars) {
                    return Some(err);
                }
            }
        }
        None
    }

    /// Check a single validation rule against `val`. Returns the error string
    /// when the rule is violated.
    fn check_rule(
        field_name: &str,
        val: &str,
        rule: &ValidationRule,
        vars: &VarPool,
    ) -> Option<String> {
        match rule {
            ValidationRule::NonBlank => {
                if val.is_empty() {
                    return Some(format!("Field {field_name} must not be blank"));
                }
            }
            ValidationRule::Boolean => {
                let upper = val.to_uppercase();
                if !matches!(upper.as_str(), "Y" | "N" | "YES" | "NO" | "") {
                    return Some(format!("Field {field_name} must be Y or N"));
                }
            }
            ValidationRule::Numeric { range } => {
                if val.is_empty() {
                    return None;
                }
                let n: i64 = match val.parse() {
                    Ok(n) => n,
                    Err(_) => return Some(format!("Field {field_name} must be numeric")),
                };
                if let Some(r) = range {
                    let lo = Self::resolve_range_value(&r.min, vars);
                    let hi = Self::resolve_range_value(&r.max, vars);
                    if let (Some(lo), Some(hi)) = (lo, hi) {
                        if n < lo || n > hi {
                            return Some(format!(
                                "Field {field_name} must be between {lo} and {hi}"
                            ));
                        }
                    }
                }
            }
            ValidationRule::Alpha => {
                if !val.is_empty() && !val.chars().all(|c| c.is_alphabetic() || c == ' ') {
                    return Some(format!("Field {field_name} must be alphabetic"));
                }
            }
            ValidationRule::Hex => {
                if !val.is_empty() && !val.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Some(format!("Field {field_name} must be hexadecimal"));
                }
            }
            ValidationRule::List { values } => {
                let upper = val.to_uppercase();
                if !val.is_empty() && !values.iter().any(|v| v.to_uppercase() == upper) {
                    return Some(format!(
                        "Field {field_name} must be one of: {}",
                        values.join(", ")
                    ));
                }
            }
            ValidationRule::Picture { .. } => {
                // Picture validation is complex — skip for now
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

    /// Evaluate a simple condition (e.g., `&VAR = value` or `&VAR NE value`).
    fn evaluate_condition(condition: &str, vars: &VarPool) -> bool {
        let parts: Vec<&str> = condition.splitn(3, ' ').collect();
        if parts.len() != 3 {
            return false;
        }
        let left = vars.resolve(parts[0]);
        let right = vars.resolve(parts[2]);
        match parts[1] {
            "=" => left.trim().eq_ignore_ascii_case(right.trim()),
            "NE" => !left.trim().eq_ignore_ascii_case(right.trim()),
            _ => false,
        }
    }

    // ─── Key handling ───────────────────────────────────────────────────

    /// Translate a key event into a `KeyAction`, mutating field/command state
    /// for character input and intra-panel cursor movement.
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
                Self::move_field_linear(fields, current_field, true);
                KeyAction::Continue
            }
            KeyCode::BackTab => {
                Self::move_field_linear(fields, current_field, false);
                KeyAction::Continue
            }

            KeyCode::Char(ch) => {
                Self::input_char(fields, *current_field, command_text, vars, ch);
                KeyAction::Continue
            }
            // Delete is currently treated the same as Backspace.
            KeyCode::Backspace | KeyCode::Delete => {
                Self::delete_char(fields, *current_field, command_text, vars);
                KeyAction::Continue
            }

            KeyCode::Up => {
                Self::move_field_vertical(fields, current_field, false);
                KeyAction::Continue
            }
            KeyCode::Down => {
                Self::move_field_vertical(fields, current_field, true);
                KeyAction::Continue
            }

            _ => KeyAction::Continue,
        }
    }

    /// Insert a character at the active field (or command line).
    fn input_char(
        fields: &[FieldInfo],
        current: usize,
        command_text: &mut String,
        vars: &mut VarPool,
        ch: char,
    ) {
        let Some(field) = fields.get(current) else {
            return;
        };
        if field.is_command {
            command_text.push(ch);
            return;
        }
        let mut val = vars.get(&field.variable).unwrap_or("").to_string();
        if val.len() < field.width {
            val.push(ch);
            vars.set(&field.variable, &val);
        }
    }

    /// Remove the trailing character from the active field (or command line).
    fn delete_char(
        fields: &[FieldInfo],
        current: usize,
        command_text: &mut String,
        vars: &mut VarPool,
    ) {
        let Some(field) = fields.get(current) else {
            return;
        };
        if field.is_command {
            command_text.pop();
            return;
        }
        let mut val = vars.get(&field.variable).unwrap_or("").to_string();
        val.pop();
        vars.set(&field.variable, &val);
    }

    /// Move to the next (`forward = true`) or previous field, wrapping around.
    fn move_field_linear(fields: &[FieldInfo], current: &mut usize, forward: bool) {
        if fields.is_empty() {
            return;
        }
        if forward {
            *current = (*current + 1) % fields.len();
        } else if *current == 0 {
            *current = fields.len() - 1;
        } else {
            *current -= 1;
        }
    }

    /// Move to the closest field on the previous (`down = false`) or next
    /// (`down = true`) row, by row-distance.
    fn move_field_vertical(fields: &[FieldInfo], current: &mut usize, down: bool) {
        let Some(cur) = fields.get(*current) else {
            return;
        };
        let cur_row = cur.row;
        let mut best: Option<usize> = None;
        let mut best_dist = u16::MAX;
        for (i, f) in fields.iter().enumerate() {
            let dist = match (down, f.row.cmp(&cur_row)) {
                (true, std::cmp::Ordering::Greater) => f.row - cur_row,
                (false, std::cmp::Ordering::Less) => cur_row - f.row,
                _ => continue,
            };
            if dist < best_dist {
                best_dist = dist;
                best = Some(i);
            }
        }
        if let Some(idx) = best {
            *current = idx;
        }
    }
}
