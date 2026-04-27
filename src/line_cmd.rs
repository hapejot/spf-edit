//! Line command collection, validation, and execution.
//!
//! When the user presses Enter, `editor.rs` calls `execute_line_commands`
//! which:
//!   1. Re-parses every prefix_cmd in the buffer.
//!   2. Groups commands by type (inserts, deletes, copies, moves, repeats,
//!      labels, destinations).
//!   3. Pairs block commands (CC↔CC, DD↔DD, etc.).
//!   4. Validates (e.g. copy/move needs A or B destination).
//!   5. Executes in order: labels → deletes → moves → copies → repeats → inserts.
//!      Deletes/inserts are processed bottom-to-top to preserve indices.
//!
//! ## Known Issues
//!
//! - `collect_line_commands` is defined but never called; `execute_line_commands`
//!   does its own inline collection.  `collect_line_commands` could be removed
//!   or `execute_line_commands` could delegate to it.
//! - Block delete pairing has a subtle bug: it pushes a single D(1) then
//!   immediately replaces it with D(range_count), but if there are also single
//!   Dn commands in the same Enter, the index manipulation may interleave.
//!   TODO: test DD block delete together with individual D commands.
//! - Unpaired block starts are left pending (PENDING_CMD flag), which is
//!   correct SPF behavior, but the editor doesn't visually distinguish
//!   "pending from last Enter" vs "just typed" yet.

use tracing::{debug, info, warn};

use crate::buffer::FileBuffer;
use crate::line::LineFlags;
use crate::line_store::LineStore;
use crate::prefix::{ParsedLineCmd, PrefixParseResult, parse_prefix_command};

/// A pending line command with its buffer line index.
#[derive(Debug, Clone)]
pub struct PendingLineCmd {
    pub cmd: ParsedLineCmd,
    pub line_index: usize,
}

/// Collect and validate result.
pub struct LineCmdResult {
    pub error: Option<String>,
}

/// Collect all pending line commands from modified prefix areas.
pub fn collect_line_commands(buffer: &FileBuffer) -> Vec<PendingLineCmd> {
    let mut commands = Vec::new();

    for i in 0..buffer.lines.len() {
        if let Some(line) = buffer.lines.get(i) {
            if let Some(ref prefix_text) = line.prefix_cmd {
                match parse_prefix_command(prefix_text) {
                    PrefixParseResult::Command(cmd) => {
                        commands.push(PendingLineCmd { cmd, line_index: i });
                    }
                    PrefixParseResult::Error(msg) => {
                        commands.push(PendingLineCmd {
                            cmd: ParsedLineCmd::Label(String::new()), // placeholder
                            line_index: i,
                        });
                        // We'll handle the error in validation
                    }
                    PrefixParseResult::None => {}
                }
            }
        }
    }

    commands
}

/// Validate and execute all pending line commands.
/// Returns an error message if validation fails, None on success.
pub fn execute_line_commands(buffer: &mut FileBuffer) -> LineCmdResult {
    // Collect commands from prefix areas
    let mut cmds = Vec::new();
    let mut parse_errors = Vec::new();
    debug!(
        "execute_line_commands: scanning {} lines for prefix cmds",
        buffer.lines.len()
    );

    for i in 0..buffer.lines.len() {
        if let Some(line) = buffer.lines.get(i) {
            if let Some(ref prefix_text) = line.prefix_cmd {
                match parse_prefix_command(prefix_text) {
                    PrefixParseResult::Command(cmd) => {
                        cmds.push(PendingLineCmd { cmd, line_index: i });
                    }
                    PrefixParseResult::Error(msg) => {
                        parse_errors.push((i, msg));
                    }
                    PrefixParseResult::None => {}
                }
            }
        }
    }

    // Mark parse errors
    for (idx, _msg) in &parse_errors {
        warn!("parse error at line {idx}: {_msg}");
        if let Some(line) = buffer.lines.get_mut(*idx) {
            line.flags.set(LineFlags::CMD_ERROR);
        }
    }

    if !parse_errors.is_empty() {
        return LineCmdResult {
            error: Some(format!("Invalid line command: {}", parse_errors[0].1)),
        };
    }

    if cmds.is_empty() {
        debug!("  no line commands to execute");
        return LineCmdResult { error: None };
    }
    debug!("  found {} line commands", cmds.len());

    // Separate commands by type
    let mut inserts: Vec<PendingLineCmd> = Vec::new();
    let mut deletes: Vec<PendingLineCmd> = Vec::new();
    let mut repeats: Vec<PendingLineCmd> = Vec::new();
    let mut copy_sources: Vec<PendingLineCmd> = Vec::new();
    let mut move_sources: Vec<PendingLineCmd> = Vec::new();
    let mut destination: Option<PendingLineCmd> = None;
    let mut labels: Vec<PendingLineCmd> = Vec::new();

    // Block tracking
    let mut block_starts: Vec<PendingLineCmd> = Vec::new();

    let mut processed_lines = vec![];

    for cmd in cmds {
        processed_lines.push(cmd.line_index);
        match &cmd.cmd {
            ParsedLineCmd::Insert(_) => inserts.push(cmd),
            ParsedLineCmd::Delete(_) => deletes.push(cmd),
            ParsedLineCmd::Repeat(_) => repeats.push(cmd),
            ParsedLineCmd::CopySingle => copy_sources.push(cmd),
            ParsedLineCmd::MoveSingle => move_sources.push(cmd),
            ParsedLineCmd::After | ParsedLineCmd::Before => {
                if destination.is_some() {
                    return LineCmdResult {
                        error: Some("Multiple destinations (A/B) specified".to_string()),
                    };
                }
                destination = Some(cmd);
            }
            ParsedLineCmd::CopyBlockStart => block_starts.push(cmd),
            ParsedLineCmd::CopyBlockEnd => {
                // Find matching start
                if let Some(start_pos) = block_starts
                    .iter()
                    .rposition(|s| matches!(s.cmd, ParsedLineCmd::CopyBlockStart))
                {
                    let start = block_starts.remove(start_pos);
                    // Convert to copy source range
                    copy_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopySingle, // We'll use line_index range
                        line_index: start.line_index,
                    });
                    copy_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopySingle,
                        line_index: cmd.line_index,
                    });
                } else {
                    // This is a second CC, treat as block start for pending
                    block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopyBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::MoveBlockStart => block_starts.push(cmd),
            ParsedLineCmd::MoveBlockEnd => {
                if let Some(start_pos) = block_starts
                    .iter()
                    .rposition(|s| matches!(s.cmd, ParsedLineCmd::MoveBlockStart))
                {
                    let start = block_starts.remove(start_pos);
                    move_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveSingle,
                        line_index: start.line_index,
                    });
                    move_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveSingle,
                        line_index: cmd.line_index,
                    });
                } else {
                    block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::DeleteBlockStart => block_starts.push(cmd),
            ParsedLineCmd::DeleteBlockEnd => {
                if let Some(start_pos) = block_starts
                    .iter()
                    .rposition(|s| matches!(s.cmd, ParsedLineCmd::DeleteBlockStart))
                {
                    let start = block_starts.remove(start_pos);
                    deletes.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Delete(1),
                        line_index: start.line_index,
                    });
                    // Use special marker: delete range
                    deletes.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Delete(cmd.line_index - start.line_index + 1),
                        line_index: start.line_index,
                    });
                    // Remove the first single-delete we just pushed
                    deletes.remove(deletes.len() - 2);
                } else {
                    block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::DeleteBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::RepeatBlockStart(_n) => block_starts.push(cmd),
            ParsedLineCmd::RepeatBlockEnd => {
                if let Some(start_pos) = block_starts
                    .iter()
                    .rposition(|s| matches!(s.cmd, ParsedLineCmd::RepeatBlockStart(_)))
                {
                    let start = block_starts.remove(start_pos);
                    let count = match start.cmd {
                        ParsedLineCmd::RepeatBlockStart(n) => n,
                        _ => 1,
                    };
                    repeats.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Repeat(count),
                        line_index: start.line_index,
                    });
                    // Store end index by adding a second marker
                    repeats.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Repeat(0), // 0 = end marker
                        line_index: cmd.line_index,
                    });
                } else {
                    block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::RepeatBlockStart(1),
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::Label(_name) => labels.push(cmd),
        }
    }

    // If there are unpaired block starts, leave them pending
    if !block_starts.is_empty() {
        for start in &block_starts {
            if let Some(line) = buffer.lines.get_mut(start.line_index) {
                line.flags.set(LineFlags::PENDING_CMD);
                // Keep the prefix_cmd so it shows as pending
            }
        }
        // Don't return error — pending is normal
    }

    // Check if copy/move sources need a destination
    if (!copy_sources.is_empty() || !move_sources.is_empty()) && destination.is_none() {
        // Leave as pending
        for src in copy_sources.iter().chain(move_sources.iter()) {
            if let Some(line) = buffer.lines.get_mut(src.line_index) {
                line.flags.set(LineFlags::PENDING_CMD);
            }
        }
        return LineCmdResult { error: None };
    }

    // --- Execute commands ---
    // Order: labels first, then deletes (high to low), then moves, then copies, then repeats, then inserts
    debug!(
        "  executing: {} labels, {} deletes, {} moves, {} copies, {} repeats, {} inserts",
        labels.len(),
        deletes.len(),
        move_sources.len(),
        copy_sources.len(),
        repeats.len(),
        inserts.len()
    );

    // Labels
    for cmd in &labels {
        if let ParsedLineCmd::Label(name) = &cmd.cmd {
            buffer.set_label(name.clone(), cmd.line_index);
            if let Some(line) = buffer.lines.get_mut(cmd.line_index) {
                line.clear_prefix_cmd();
            }
        }
    }

    // Deletes — process from bottom to top to preserve indices
    deletes.sort_by(|a, b| b.line_index.cmp(&a.line_index));
    for cmd in &deletes {
        if let ParsedLineCmd::Delete(count) = cmd.cmd {
            buffer.delete_lines(cmd.line_index, count);
        }
    }

    // Moves
    if !move_sources.is_empty() {
        if let Some(dest) = &destination {
            let (src_start, src_end) = if move_sources.len() >= 2 {
                let s = move_sources[0].line_index;
                let e = move_sources[1].line_index;
                (s.min(e), s.max(e))
            } else {
                let s = move_sources[0].line_index;
                (s, s)
            };
            let dest_idx = dest.line_index;
            let before = matches!(dest.cmd, ParsedLineCmd::Before);
            let actual_dest = if before {
                dest_idx.saturating_sub(1)
            } else {
                dest_idx
            };
            buffer.move_lines(src_start, src_end, actual_dest);
        }
    }

    // Copies
    if !copy_sources.is_empty() {
        if let Some(dest) = &destination {
            let (src_start, src_end) = if copy_sources.len() >= 2 {
                let s = copy_sources[0].line_index;
                let e = copy_sources[1].line_index;
                (s.min(e), s.max(e))
            } else {
                let s = copy_sources[0].line_index;
                (s, s)
            };
            let dest_idx = dest.line_index;
            let before = matches!(dest.cmd, ParsedLineCmd::Before);
            let actual_dest = if before {
                dest_idx.saturating_sub(1)
            } else {
                dest_idx
            };
            buffer.copy_lines(src_start, src_end, actual_dest);
        }
    }

    // Repeats — process pairs (start_idx, end_idx) from bottom to top
    let mut repeat_pairs: Vec<(usize, usize, usize)> = Vec::new(); // (start, end, count)
    let mut i = 0;
    while i < repeats.len() {
        if let ParsedLineCmd::Repeat(count) = repeats[i].cmd {
            if count > 0 {
                // Check for end marker
                if i + 1 < repeats.len() {
                    if let ParsedLineCmd::Repeat(0) = repeats[i + 1].cmd {
                        // Block repeat
                        repeat_pairs.push((
                            repeats[i].line_index,
                            repeats[i + 1].line_index,
                            count,
                        ));
                        i += 2;
                        continue;
                    }
                }
                // Single line repeat
                repeat_pairs.push((repeats[i].line_index, repeats[i].line_index, count));
            }
        }
        i += 1;
    }

    repeat_pairs.sort_by(|a, b| b.0.cmp(&a.0));
    for (start, end, count) in repeat_pairs {
        buffer.repeat_lines(start, end, count);
    }

    // Inserts — process from bottom to top to preserve indices
    inserts.sort_by(|a, b| b.line_index.cmp(&a.line_index));
    info!("Processing {} inserts", inserts.len());
    for cmd in &inserts {
        if let ParsedLineCmd::Insert(count) = cmd.cmd {
            buffer.insert_lines_after(cmd.line_index, count);
        }
    }

    // Clear all processed prefix commands
    info!("Clearing prefix commands from all lines");
    for i in processed_lines {
        if let Some(line) = buffer.lines.get_mut(i) {
            info!("Clearing prefix cmd from line {i}");
            line.clear_prefix_cmd();
        }
    }

    // Clear destination
    // dest index may have shifted, but we clear all non-pending cmds above

    buffer.renumber();

    LineCmdResult { error: None }
}
