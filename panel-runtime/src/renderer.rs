//! Panel renderer: draws a panel to the terminal using crossterm.
//!
//! The renderer takes a loaded Panel and a VarPool, computes layout,
//! and draws each body row to the terminal. It returns information about
//! the input fields so the engine can manage cursor placement and input.

use std::io::{self, Write};

use chrono::Local;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use panel_model::*;
use crate::vars::VarPool;

// ─── Colors for panel display ───────────────────────────────────────────────

pub struct PanelColors;

impl PanelColors {
    pub const TITLE_FG: Color = Color::White;
    pub const TITLE_BG: Color = Color::Blue;

    pub const CMD_PROMPT_FG: Color = Color::Green;
    pub const CMD_PROMPT_BG: Color = Color::Black;

    pub const CMD_INPUT_FG: Color = Color::White;
    pub const CMD_INPUT_BG: Color = Color::Black;

    pub const TEXT_FG: Color = Color::Green;
    pub const TEXT_BG: Color = Color::Black;

    pub const TEXT_HIGH_FG: Color = Color::White;
    pub const TEXT_HIGH_BG: Color = Color::Black;

    pub const INPUT_FG: Color = Color::Cyan;
    pub const INPUT_BG: Color = Color::Black;

    pub const OUTPUT_FG: Color = Color::White;
    pub const OUTPUT_BG: Color = Color::Black;

    pub const OUTPUT_LOW_FG: Color = Color::Blue;

    pub const BOX_FG: Color = Color::Yellow;
    pub const BOX_BG: Color = Color::Black;

    pub const DIVIDER_FG: Color = Color::Blue;
    pub const DIVIDER_BG: Color = Color::Black;

    pub const ERROR_FG: Color = Color::Red;
    pub const ERROR_BG: Color = Color::Black;

    pub const STATUS_FG: Color = Color::Black;
    pub const STATUS_BG: Color = Color::Green;

    pub const SCROLL_FG: Color = Color::Green;
}

// ─── Field info returned to the engine ──────────────────────────────────────

/// Describes an input field's position on screen for cursor management.
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub variable: String,
    pub row: u16,
    pub col: u16,
    pub width: usize,
    pub value: String,
    pub is_command: bool,
}

// ─── Panel renderer ─────────────────────────────────────────────────────────

pub struct PanelRenderer;

impl PanelRenderer {
    /// Draw a panel to the terminal. Returns the list of input field positions.
    pub fn draw<W: Write>(
        stdout: &mut W,
        panel: &Panel,
        vars: &VarPool,
        command_text: &str,
        scroll_text: &str,
        error_msg: Option<&str>,
        width: u16,
        height: u16,
    ) -> io::Result<Vec<FieldInfo>> {
        let mut fields = Vec::new();
        let w = width as usize;

        // Clear screen
        queue!(stdout, Clear(ClearType::All))?;

        // ── Row 0: Title line ──────────────────────────────────────────
        // Layout:  product (left)   main (centered)   version (right)
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(PanelColors::TITLE_FG),
            SetBackgroundColor(PanelColors::TITLE_BG),
            Print(format!("{:w$}", "", w = w)),
        )?;

        let (product, main, version) = if let Some(ref title) = panel.title {
            let product = title
                .product_var
                .as_ref()
                .and_then(|v| vars.get(v))
                .map(|s| s.to_string())
                .unwrap_or_default();
            let main = vars.resolve(&title.text);
            let version = title
                .version_var
                .as_ref()
                .and_then(|v| vars.get(v))
                .map(|s| s.to_string())
                .unwrap_or_default();
            (product, main, version)
        } else {
            (String::new(), panel.id.clone(), String::new())
        };

        // Truncate to safe widths
        let product_t: String = product.chars().take(w).collect();
        let version_t: String = version.chars().take(w).collect();
        let main_t: String = main
            .chars()
            .take(w.saturating_sub(product_t.len() + version_t.len() + 2))
            .collect();

        // Print product on the left
        if !product_t.is_empty() {
            queue!(stdout, MoveTo(0, 0), Print(&product_t))?;
        }

        // Print main, centered
        if !main_t.is_empty() {
            let center_col = (w.saturating_sub(main_t.len())) / 2;
            queue!(stdout, MoveTo(center_col as u16, 0), Print(&main_t))?;
        }

        // Print version on the right (will be overwritten by error msg if present)
        if !version_t.is_empty() {
            let right_col = w.saturating_sub(version_t.len());
            queue!(stdout, MoveTo(right_col as u16, 0), Print(&version_t))?;
        }

        // If there's an error message, show it right-aligned on title line
        if let Some(msg) = error_msg {
            let msg_start = w.saturating_sub(msg.len() + 1);
            queue!(
                stdout,
                MoveTo(msg_start as u16, 0),
                SetForegroundColor(PanelColors::ERROR_FG),
                SetBackgroundColor(PanelColors::TITLE_BG),
                Print(msg),
            )?;
        }

        queue!(stdout, ResetColor)?;

        // ── Body rows ──────────────────────────────────────────────────
        // Reserve the last 3 rows: 2 PF-key rows + 1 status bar.
        let mut row: u16 = 1;
        let max_row = height.saturating_sub(4);

        for body_row in &panel.body.rows {
            if row > max_row {
                break;
            }
            let used = Self::draw_body_row(
                stdout, body_row, vars, &mut fields,
                command_text, scroll_text,
                row, w, &panel.attributes,
            )?;
            row += used;
        }

        // Fill remaining rows with empty space
        while row <= max_row {
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(PanelColors::TEXT_FG),
                SetBackgroundColor(PanelColors::TEXT_BG),
                Print(format!("{:w$}", "", w = w)),
            )?;
            row += 1;
        }

        // ── PF key bar (rows H-3 and H-2) ─────────────────────────────
        Self::draw_pf_key_bar(stdout, panel, vars, w, height)?;

        // ── Status bar (last row) ─────────────────────────────────────
        Self::draw_status_bar(stdout, &panel.id, w, height)?;

        queue!(stdout, ResetColor)?;
        stdout.flush()?;

        Ok(fields)
    }

    /// Draw the two-row PF key bar (F1-F6 on the upper row, F7-F12 on
    /// the lower row). Per-panel `pfkeys` overrides take precedence over
    /// the runtime defaults stored in the variable pool.
    fn draw_pf_key_bar<W: Write>(
        stdout: &mut W,
        panel: &Panel,
        vars: &VarPool,
        width: usize,
        height: u16,
    ) -> io::Result<()> {
        if height < 4 {
            return Ok(());
        }
        let top_row = height.saturating_sub(3);
        let bot_row = height.saturating_sub(2);

        // Resolve the label for PF key n: per-panel override → default.
        let resolve = |n: u8| -> Option<String> {
            let key = format!("F{n}");
            if let Some(def) = panel
                .pfkeys
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(&key))
                .map(|(_, v)| v)
            {
                let label = def
                    .label
                    .clone()
                    .unwrap_or_else(|| def.command.to_uppercase());
                Some(label)
            } else {
                vars.pf_key(n).map(|d| d.label.clone())
            }
        };

        // Build six-cell rows
        let cell_w = width / 6;
        for (row_y, range) in [(top_row, 1u8..=6), (bot_row, 7u8..=12)] {
            // Clear row background
            queue!(
                stdout,
                MoveTo(0, row_y),
                SetForegroundColor(PanelColors::TEXT_FG),
                SetBackgroundColor(PanelColors::TEXT_BG),
                Print(format!("{:w$}", "", w = width)),
            )?;
            for (i, n) in range.enumerate() {
                let label = resolve(n).unwrap_or_default();
                if label.is_empty() {
                    continue;
                }
                let text = format!("F{n}={label}");
                let col = (i * cell_w) as u16;
                let avail = cell_w.min(width.saturating_sub(col as usize));
                let truncated: String = text.chars().take(avail).collect();
                queue!(
                    stdout,
                    MoveTo(col, row_y),
                    SetForegroundColor(PanelColors::BOX_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(&truncated),
                )?;
            }
        }
        queue!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw the status bar on the last terminal row.
    fn draw_status_bar<W: Write>(
        stdout: &mut W,
        panel_id: &str,
        width: usize,
        height: u16,
    ) -> io::Result<()> {
        let status_row = height.saturating_sub(1);

        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();

        let left = format!(" {} | Panel: {}", time_str, panel_id);
        let right = format!("SPF-Edit ");
        let padding = width.saturating_sub(left.len() + right.len());
        let bar = format!("{}{}{}", left, " ".repeat(padding), right);

        let display: String = bar.chars().take(width).collect();

        queue!(
            stdout,
            MoveTo(0, status_row),
            SetForegroundColor(PanelColors::STATUS_FG),
            SetBackgroundColor(PanelColors::STATUS_BG),
            Print(&display),
        )?;

        let remaining = width.saturating_sub(display.len());
        if remaining > 0 {
            queue!(stdout, Print(" ".repeat(remaining)))?;
        }

        queue!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw a single body row. Returns the number of terminal rows consumed (usually 1).
    fn draw_body_row<W: Write>(
        stdout: &mut W,
        body_row: &BodyRow,
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        command_text: &str,
        scroll_text: &str,
        row: u16,
        width: usize,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> io::Result<u16> {
        match body_row {
            BodyRow::Command { variable, scroll } => {
                Self::draw_command_row(
                    stdout, variable, scroll.as_ref(),
                    fields, command_text, scroll_text,
                    row, width,
                )?;
                Ok(1)
            }

            BodyRow::Blank => {
                Self::fill_row(stdout, row, width, PanelColors::TEXT_FG, PanelColors::TEXT_BG)?;
                Ok(1)
            }

            BodyRow::Text { content, style } => {
                let (fg, bg) = match style.as_deref() {
                    Some("high") => (PanelColors::TEXT_HIGH_FG, PanelColors::TEXT_HIGH_BG),
                    _ => (PanelColors::TEXT_FG, PanelColors::TEXT_BG),
                };
                Self::draw_text_line(stdout, &vars.resolve(content), row, width, fg, bg)?;
                Ok(1)
            }

            BodyRow::FieldRow { fields: row_fields } => {
                Self::fill_row(stdout, row, width, PanelColors::TEXT_FG, PanelColors::TEXT_BG)?;
                Self::draw_field_sequence(
                    stdout, row_fields, vars, fields, row, width, attrs,
                )?;
                Ok(1)
            }

            BodyRow::Input {
                variable,
                attribute: _,
                width: field_width,
                field_connector: _,
            } => {
                Self::draw_input_row(stdout, variable, *field_width, vars, fields, row, width)?;
                Ok(1)
            }

            BodyRow::Output { variable, .. } => {
                let val = vars.get(variable).unwrap_or("").to_string();
                Self::draw_text_line(
                    stdout, &val, row, width,
                    PanelColors::OUTPUT_FG, PanelColors::TEXT_BG,
                )?;
                Ok(1)
            }

            BodyRow::Divider { style } => {
                let ch = match style {
                    DividerStyle::Single => '─',
                    DividerStyle::Double => '═',
                };
                let line: String = std::iter::repeat(ch).take(width).collect();
                queue!(
                    stdout,
                    MoveTo(0, row),
                    SetForegroundColor(PanelColors::DIVIDER_FG),
                    SetBackgroundColor(PanelColors::DIVIDER_BG),
                    Print(&line),
                )?;
                Ok(1)
            }

            BodyRow::Box { style, rows: box_rows } => {
                Self::draw_box(stdout, *style, box_rows, vars, fields, row, width, attrs)
            }

            BodyRow::InlineGroup { fields: group_fields } => {
                Self::fill_row(stdout, row, width, PanelColors::TEXT_FG, PanelColors::TEXT_BG)?;
                Self::draw_field_sequence(
                    stdout, group_fields, vars, fields, row, width, attrs,
                )?;
                Ok(1)
            }

            BodyRow::ColumnHeader { columns } => {
                Self::fill_row(
                    stdout, row, width,
                    PanelColors::TEXT_HIGH_FG, PanelColors::TEXT_BG,
                )?;
                let header = columns.join("  ");
                queue!(
                    stdout,
                    MoveTo(1, row),
                    Print(&header[..header.len().min(width - 1)]),
                )?;
                Ok(1)
            }

            BodyRow::ColumnRuler => {
                let ruler = Self::build_column_ruler(width);
                queue!(
                    stdout,
                    MoveTo(0, row),
                    SetForegroundColor(PanelColors::DIVIDER_FG),
                    SetBackgroundColor(PanelColors::DIVIDER_BG),
                    Print(&ruler),
                )?;
                Ok(1)
            }

            BodyRow::Raw { content } => {
                Self::draw_text_line(
                    stdout, &vars.resolve(content), row, width,
                    PanelColors::TEXT_FG, PanelColors::TEXT_BG,
                )?;
                Ok(1)
            }
        }
    }

    /// Clear `row` with the given bg colour, full width.
    fn fill_row<W: Write>(
        stdout: &mut W,
        row: u16,
        width: usize,
        fg: crossterm::style::Color,
        bg: crossterm::style::Color,
    ) -> io::Result<()> {
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(format!("{:w$}", "", w = width)),
        )?;
        Ok(())
    }

    /// Fill a row with `bg` and print `text` starting at column 1, truncated to fit.
    fn draw_text_line<W: Write>(
        stdout: &mut W,
        text: &str,
        row: u16,
        width: usize,
        fg: crossterm::style::Color,
        bg: crossterm::style::Color,
    ) -> io::Result<()> {
        Self::fill_row(stdout, row, width, fg, bg)?;
        if width >= 2 {
            queue!(
                stdout,
                MoveTo(1, row),
                Print(&text[..text.len().min(width - 1)]),
            )?;
        }
        Ok(())
    }

    /// Draw a sequence of fields side-by-side, starting at column 1, with one
    /// column of spacing between them.
    fn draw_field_sequence<W: Write>(
        stdout: &mut W,
        row_fields: &[Field],
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        row: u16,
        width: usize,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> io::Result<()> {
        let mut col: u16 = 1;
        for field in row_fields {
            col = Self::draw_field(stdout, field, vars, fields, row, col, width, attrs)?;
            col += 1; // spacing between fields
        }
        Ok(())
    }

    /// Draw a top-level Input row (`%Z`-style line with a single input variable).
    fn draw_input_row<W: Write>(
        stdout: &mut W,
        variable: &str,
        field_width: Option<usize>,
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        row: u16,
        width: usize,
    ) -> io::Result<()> {
        let fw = field_width.unwrap_or(width.saturating_sub(2));
        let val = vars.get(variable).unwrap_or("").to_string();
        let display = format!("{:<fw$}", val, fw = fw);
        let start_col: u16 = 1;

        Self::fill_row(stdout, row, width, PanelColors::TEXT_FG, PanelColors::TEXT_BG)?;
        queue!(
            stdout,
            MoveTo(start_col, row),
            SetForegroundColor(PanelColors::INPUT_FG),
            SetBackgroundColor(PanelColors::INPUT_BG),
            Print(&display[..display.len().min(width - start_col as usize)]),
        )?;

        fields.push(FieldInfo {
            variable: variable.to_string(),
            row,
            col: start_col,
            width: fw,
            value: val,
            is_command: false,
        });
        Ok(())
    }

    /// Generate a column ruler like `----+----1----+----2...` of `width` chars.
    fn build_column_ruler(width: usize) -> String {
        let mut ruler = String::with_capacity(width);
        for i in 1..=width {
            if i % 10 == 0 {
                ruler.push(char::from_digit((i / 10) as u32 % 10, 10).unwrap_or('-'));
            } else if i % 5 == 0 {
                ruler.push('+');
            } else {
                ruler.push('-');
            }
        }
        ruler
    }

    /// Draw a Box body row with its top border, inner rows, and bottom border.
    /// Returns the number of terminal rows consumed.
    #[allow(clippy::too_many_arguments)]
    fn draw_box<W: Write>(
        stdout: &mut W,
        style: BoxStyle,
        box_rows: &[BodyRow],
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        row: u16,
        width: usize,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> io::Result<u16> {
        let (top, bottom, left, right) = match style {
            BoxStyle::Asterisk | BoxStyle::Announcement => ('*', '*', '*', '*'),
            BoxStyle::Single => ('─', '─', '│', '│'),
            BoxStyle::Double => ('═', '═', '║', '║'),
        };
        let (corner_tl, corner_tr, corner_bl, corner_br) = match style {
            BoxStyle::Asterisk | BoxStyle::Announcement => ('*', '*', '*', '*'),
            BoxStyle::Single => ('┌', '┐', '└', '┘'),
            BoxStyle::Double => ('╔', '╗', '╚', '╝'),
        };

        // Top border
        let top_line = Self::box_border_line(corner_tl, top, corner_tr, width);
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(PanelColors::BOX_FG),
            SetBackgroundColor(PanelColors::BOX_BG),
            Print(&top_line),
        )?;

        let mut current_row = row + 1;
        let max_inner_row = row + box_rows.len() as u16;
        let inner_width = width.saturating_sub(2);

        // Box content rows
        for inner_row in box_rows {
            if current_row > max_inner_row {
                break;
            }
            // Left border
            queue!(
                stdout,
                MoveTo(0, current_row),
                SetForegroundColor(PanelColors::BOX_FG),
                SetBackgroundColor(PanelColors::BOX_BG),
                Print(left),
            )?;

            Self::draw_box_inner(
                stdout, inner_row, vars, fields,
                current_row, inner_width, attrs,
            )?;

            // Right border
            queue!(
                stdout,
                SetForegroundColor(PanelColors::BOX_FG),
                SetBackgroundColor(PanelColors::BOX_BG),
                Print(right),
            )?;
            current_row += 1;
        }

        // Bottom border
        let bottom_line = Self::box_border_line(corner_bl, bottom, corner_br, width);
        queue!(
            stdout,
            MoveTo(0, current_row),
            SetForegroundColor(PanelColors::BOX_FG),
            SetBackgroundColor(PanelColors::BOX_BG),
            Print(&bottom_line),
        )?;

        Ok(current_row - row + 1)
    }

    /// Build a `corner + edge*(n-2) + corner` string of total `width` chars.
    fn box_border_line(left: char, mid: char, right: char, width: usize) -> String {
        format!(
            "{}{}{}",
            left,
            std::iter::repeat(mid).take(width.saturating_sub(2)).collect::<String>(),
            right,
        )
        .chars()
        .take(width)
        .collect()
    }

    /// Render a single row that lives inside a Box (already positioned at
    /// column 1, with `inner_width` characters of horizontal space).
    fn draw_box_inner<W: Write>(
        stdout: &mut W,
        inner_row: &BodyRow,
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        current_row: u16,
        inner_width: usize,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> io::Result<()> {
        match inner_row {
            BodyRow::Blank => {
                queue!(
                    stdout,
                    SetForegroundColor(PanelColors::TEXT_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(format!("{:w$}", "", w = inner_width)),
                )?;
            }
            BodyRow::Text { content, .. } => {
                let resolved = vars.resolve(content);
                queue!(
                    stdout,
                    SetForegroundColor(PanelColors::TEXT_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(format!("{:<w$}", resolved, w = inner_width)),
                )?;
            }
            BodyRow::FieldRow { fields: row_fields } => {
                queue!(
                    stdout,
                    SetForegroundColor(PanelColors::TEXT_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(format!("{:w$}", "", w = inner_width)),
                )?;
                let mut col: u16 = 1;
                for field in row_fields {
                    col = Self::draw_field(
                        stdout, field, vars, fields,
                        current_row, col, inner_width, attrs,
                    )?;
                    col += 1;
                }
            }
            BodyRow::Output { variable, .. } => {
                let val = vars.get(variable).unwrap_or("");
                queue!(
                    stdout,
                    SetForegroundColor(PanelColors::OUTPUT_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(format!("{:<w$}", val, w = inner_width)),
                )?;
            }
            _ => {
                queue!(
                    stdout,
                    SetForegroundColor(PanelColors::TEXT_FG),
                    SetBackgroundColor(PanelColors::TEXT_BG),
                    Print(format!("{:w$}", "", w = inner_width)),
                )?;
            }
        }
        Ok(())
    }

    /// Draw the command row: "Command ===> ___   Scroll ===> PAGE"
    fn draw_command_row<W: Write>(
        stdout: &mut W,
        variable: &str,
        scroll: Option<&ScrollField>,
        fields: &mut Vec<FieldInfo>,
        command_text: &str,
        scroll_text: &str,
        row: u16,
        width: usize,
    ) -> io::Result<()> {
        let prompt = "Command ===> ";
        let prompt_len = prompt.len();

        // Clear row
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(PanelColors::CMD_PROMPT_FG),
            SetBackgroundColor(PanelColors::CMD_PROMPT_BG),
            Print(format!("{:w$}", "", w = width)),
        )?;

        // Draw prompt
        queue!(
            stdout,
            MoveTo(0, row),
            Print(prompt),
        )?;

        // Draw scroll field on the right if present
        let cmd_field_width = if scroll.is_some() {
            let scroll_label = "Scroll ===> ";
            let scroll_width = 4; // e.g. "PAGE"
            let scroll_start = width.saturating_sub(scroll_label.len() + scroll_width);

            queue!(
                stdout,
                MoveTo(scroll_start as u16, row),
                SetForegroundColor(PanelColors::SCROLL_FG),
                Print(scroll_label),
                SetForegroundColor(PanelColors::CMD_INPUT_FG),
                Print(format!("{:<sw$}", scroll_text, sw = scroll_width)),
            )?;

            scroll_start.saturating_sub(prompt_len + 2)
        } else {
            width.saturating_sub(prompt_len)
        };

        // Draw command input area
        let display = format!("{:<w$}", command_text, w = cmd_field_width);
        queue!(
            stdout,
            MoveTo(prompt_len as u16, row),
            SetForegroundColor(PanelColors::CMD_INPUT_FG),
            Print(&display[..display.len().min(cmd_field_width)]),
        )?;

        fields.push(FieldInfo {
            variable: variable.to_string(),
            row,
            col: prompt_len as u16,
            width: cmd_field_width,
            value: command_text.to_string(),
            is_command: true,
        });

        Ok(())
    }

    /// Draw a single field (Text, Input, or Output) within a row.
    /// Returns the column position after the field.
    fn draw_field<W: Write>(
        stdout: &mut W,
        field: &Field,
        vars: &VarPool,
        fields: &mut Vec<FieldInfo>,
        row: u16,
        col: u16,
        max_width: usize,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> io::Result<u16> {
        match field {
            Field::Text { content, style } => {
                let resolved = vars.resolve(content);
                let (fg, _bg) = match style.as_deref() {
                    Some("high") => (PanelColors::TEXT_HIGH_FG, PanelColors::TEXT_HIGH_BG),
                    _ => (PanelColors::TEXT_FG, PanelColors::TEXT_BG),
                };
                let display_len = resolved.len().min(max_width.saturating_sub(col as usize));
                queue!(
                    stdout,
                    MoveTo(col, row),
                    SetForegroundColor(fg),
                    Print(&resolved[..display_len]),
                )?;
                Ok(col + display_len as u16)
            }

            Field::Input {
                variable,
                attribute,
                width: field_width,
                ..
            } => {
                let fw = field_width
                    .unwrap_or_else(|| Self::infer_input_width(attribute, attrs, max_width, col));
                let val = vars.get(variable).unwrap_or("").to_string();
                let display = format!("{:<fw$}", val, fw = fw);
                let display_len = display.len().min(max_width.saturating_sub(col as usize));

                // Determine color from attribute
                let fg = Self::input_color(attribute, attrs);

                queue!(
                    stdout,
                    MoveTo(col, row),
                    SetForegroundColor(fg),
                    Print(&display[..display_len]),
                )?;

                fields.push(FieldInfo {
                    variable: variable.clone(),
                    row,
                    col,
                    width: fw,
                    value: val,
                    is_command: false,
                });

                Ok(col + fw as u16)
            }

            Field::Output { variable, attribute } => {
                let val = vars.get(variable).unwrap_or("");
                let fg = if let Some(ch) = attribute {
                    if let Some(attr) = attrs.get(ch) {
                        match attr.intensity {
                            Some(Intensity::Low) => PanelColors::OUTPUT_LOW_FG,
                            _ => PanelColors::OUTPUT_FG,
                        }
                    } else {
                        PanelColors::OUTPUT_FG
                    }
                } else {
                    PanelColors::OUTPUT_FG
                };
                let display_len = val.len().min(max_width.saturating_sub(col as usize));
                queue!(
                    stdout,
                    MoveTo(col, row),
                    SetForegroundColor(fg),
                    Print(&val[..display_len]),
                )?;
                Ok(col + val.len() as u16)
            }
        }
    }

    /// Infer input field width from attribute or remaining space.
    fn infer_input_width(
        attribute: &Option<char>,
        attrs: &std::collections::HashMap<char, AttributeDef>,
        max_width: usize,
        col: u16,
    ) -> usize {
        // If the attribute has a scroll property, use remaining width
        if let Some(ch) = attribute {
            if let Some(attr) = attrs.get(ch) {
                if attr.scroll == Some(true) {
                    return max_width.saturating_sub(col as usize);
                }
            }
        }
        // Default: use 20 or remaining space
        20.min(max_width.saturating_sub(col as usize))
    }

    /// Determine input field color from attribute definition.
    fn input_color(
        attribute: &Option<char>,
        attrs: &std::collections::HashMap<char, AttributeDef>,
    ) -> Color {
        if let Some(ch) = attribute {
            if let Some(attr) = attrs.get(ch) {
                return match attr.intensity {
                    Some(Intensity::High) => PanelColors::TEXT_HIGH_FG,
                    Some(Intensity::Low) => PanelColors::OUTPUT_LOW_FG,
                    _ => PanelColors::INPUT_FG,
                };
            }
        }
        PanelColors::INPUT_FG
    }
}
