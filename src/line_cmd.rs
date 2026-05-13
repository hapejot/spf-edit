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

/// Validate and execute all pending line commands.
/// Returns an error message if validation fails, None on success.
pub fn execute_line_commands(buffer: &mut FileBuffer) -> LineCmdResult {
    debug!(
        "execute_line_commands: scanning {} lines for prefix cmds",
        buffer.lines.len()
    );

    // 1. Parse pending prefix commands.
    let (cmds, parse_errors) = parse_pending_commands(buffer);
    flag_parse_errors(buffer, &parse_errors);
    if let Some((_, msg)) = parse_errors.first() {
        return LineCmdResult {
            error: Some(format!("Invalid line command: {msg}")),
        };
    }

    if cmds.is_empty() {
        debug!("  no line commands to execute");
        return LineCmdResult { error: None };
    }
    debug!("  found {} line commands", cmds.len());

    // 2. Categorise commands by kind, pairing block start/end markers.
    let (categorised, processed_lines) = match categorise_commands(cmds) {
        Ok(v) => v,
        Err(msg) => return LineCmdResult { error: Some(msg) },
    };

    // 3. Mark unpaired block starts as pending (correct SPF behaviour).
    flag_pending(buffer, &categorised.block_starts);

    // 4. Copy/move with no destination → leave sources pending and stop.
    if categorised.destination.is_none()
        && (!categorised.copy_sources.is_empty() || !categorised.move_sources.is_empty())
    {
        for src in categorised
            .copy_sources
            .iter()
            .chain(categorised.move_sources.iter())
        {
            if let Some(line) = buffer.lines.get_mut(src.line_index) {
                line.flags.set(LineFlags::PENDING_CMD);
            }
        }
        return LineCmdResult { error: None };
    }

    debug!(
        "  executing: {} labels, {} deletes, {} moves, {} copies, {} repeats, {} inserts",
        categorised.labels.len(),
        categorised.deletes.len(),
        categorised.move_sources.len(),
        categorised.copy_sources.len(),
        categorised.repeats.len(),
        categorised.inserts.len(),
    );

    // 5. Execute commands in their canonical order.
    apply_labels(buffer, &categorised.labels);
    apply_deletes(buffer, categorised.deletes);
    apply_moves(buffer, &categorised.move_sources, categorised.destination.as_ref());
    apply_copies(buffer, &categorised.copy_sources, categorised.destination.as_ref());
    apply_repeats(buffer, &categorised.repeats);
    apply_inserts(buffer, categorised.inserts);

    // 6. Clear processed prefix commands & renumber.
    info!("Clearing prefix commands from all lines");
    for i in processed_lines {
        if let Some(line) = buffer.lines.get_mut(i) {
            info!("Clearing prefix cmd from line {i}");
            line.clear_prefix_cmd();
        }
    }
    buffer.renumber();

    LineCmdResult { error: None }
}

// ---------------------------------------------------------------------------
// Phase 1 — parsing
// ---------------------------------------------------------------------------

/// Parse every prefix command currently set on the buffer. Returns the
/// successfully-parsed commands and any (index, error message) parse failures.
fn parse_pending_commands(buffer: &FileBuffer) -> (Vec<PendingLineCmd>, Vec<(usize, String)>) {
    let mut cmds = Vec::new();
    let mut errors = Vec::new();
    for i in 0..buffer.lines.len() {
        let Some(line) = buffer.lines.get(i) else {
            continue;
        };
        let Some(prefix_text) = line.prefix_cmd.as_ref() else {
            continue;
        };
        match parse_prefix_command(prefix_text) {
            PrefixParseResult::Command(cmd) => cmds.push(PendingLineCmd { cmd, line_index: i }),
            PrefixParseResult::Error(msg) => errors.push((i, msg)),
            PrefixParseResult::None => {}
        }
    }
    (cmds, errors)
}

fn flag_parse_errors(buffer: &mut FileBuffer, errors: &[(usize, String)]) {
    for (idx, msg) in errors {
        warn!("parse error at line {idx}: {msg}");
        if let Some(line) = buffer.lines.get_mut(*idx) {
            line.flags.set(LineFlags::CMD_ERROR);
        }
    }
}

fn flag_pending(buffer: &mut FileBuffer, pending: &[PendingLineCmd]) {
    for start in pending {
        if let Some(line) = buffer.lines.get_mut(start.line_index) {
            line.flags.set(LineFlags::PENDING_CMD);
        }
    }
}

// ---------------------------------------------------------------------------
// Phase 2 — categorisation
// ---------------------------------------------------------------------------

/// All pending commands grouped by execution category.
#[derive(Default)]
struct CategorisedCmds {
    inserts: Vec<PendingLineCmd>,
    deletes: Vec<PendingLineCmd>,
    repeats: Vec<PendingLineCmd>,
    copy_sources: Vec<PendingLineCmd>,
    move_sources: Vec<PendingLineCmd>,
    destination: Option<PendingLineCmd>,
    labels: Vec<PendingLineCmd>,
    /// Block-start commands awaiting their matching end marker.
    block_starts: Vec<PendingLineCmd>,
}

/// Distribute parsed commands into category buckets, pairing block
/// start/end markers as we go. Returns the categorised buckets and the list
/// of all line indices we touched (for later prefix-clear).
fn categorise_commands(
    cmds: Vec<PendingLineCmd>,
) -> Result<(CategorisedCmds, Vec<usize>), String> {
    let mut cat = CategorisedCmds::default();
    let mut processed = Vec::with_capacity(cmds.len());

    for cmd in cmds {
        processed.push(cmd.line_index);
        match &cmd.cmd {
            ParsedLineCmd::Insert(_) => cat.inserts.push(cmd),
            ParsedLineCmd::Delete(_) => cat.deletes.push(cmd),
            ParsedLineCmd::Repeat(_) => cat.repeats.push(cmd),
            ParsedLineCmd::CopySingle => cat.copy_sources.push(cmd),
            ParsedLineCmd::MoveSingle => cat.move_sources.push(cmd),
            ParsedLineCmd::After | ParsedLineCmd::Before => {
                if cat.destination.is_some() {
                    return Err("Multiple destinations (A/B) specified".to_string());
                }
                cat.destination = Some(cmd);
            }
            ParsedLineCmd::Label(_) => cat.labels.push(cmd),

            // Block markers: pair with a previous start, or queue as new start.
            ParsedLineCmd::CopyBlockStart
            | ParsedLineCmd::MoveBlockStart
            | ParsedLineCmd::DeleteBlockStart
            | ParsedLineCmd::RepeatBlockStart(_) => cat.block_starts.push(cmd),

            ParsedLineCmd::CopyBlockEnd => {
                if let Some(start) = pair_block(&mut cat.block_starts, &cmd, |s| {
                    matches!(s.cmd, ParsedLineCmd::CopyBlockStart)
                }) {
                    cat.copy_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopySingle,
                        line_index: start.line_index,
                    });
                    cat.copy_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopySingle,
                        line_index: cmd.line_index,
                    });
                } else {
                    cat.block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::CopyBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::MoveBlockEnd => {
                if let Some(start) = pair_block(&mut cat.block_starts, &cmd, |s| {
                    matches!(s.cmd, ParsedLineCmd::MoveBlockStart)
                }) {
                    cat.move_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveSingle,
                        line_index: start.line_index,
                    });
                    cat.move_sources.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveSingle,
                        line_index: cmd.line_index,
                    });
                } else {
                    cat.block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::MoveBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::DeleteBlockEnd => {
                if let Some(start) = pair_block(&mut cat.block_starts, &cmd, |s| {
                    matches!(s.cmd, ParsedLineCmd::DeleteBlockStart)
                }) {
                    let count = cmd.line_index - start.line_index + 1;
                    cat.deletes.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Delete(count),
                        line_index: start.line_index,
                    });
                } else {
                    cat.block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::DeleteBlockStart,
                        line_index: cmd.line_index,
                    });
                }
            }
            ParsedLineCmd::RepeatBlockEnd => {
                if let Some(start) = pair_block(&mut cat.block_starts, &cmd, |s| {
                    matches!(s.cmd, ParsedLineCmd::RepeatBlockStart(_))
                }) {
                    let count = match start.cmd {
                        ParsedLineCmd::RepeatBlockStart(n) => n,
                        _ => 1,
                    };
                    cat.repeats.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Repeat(count),
                        line_index: start.line_index,
                    });
                    // 0 = end marker (consumed in apply_repeats)
                    cat.repeats.push(PendingLineCmd {
                        cmd: ParsedLineCmd::Repeat(0),
                        line_index: cmd.line_index,
                    });
                } else {
                    cat.block_starts.push(PendingLineCmd {
                        cmd: ParsedLineCmd::RepeatBlockStart(1),
                        line_index: cmd.line_index,
                    });
                }
            }
        }
    }

    Ok((cat, processed))
}

/// Remove and return the most recent pending block start that satisfies
/// `matches_start`, or `None` when there is no matching start.
fn pair_block<P>(
    block_starts: &mut Vec<PendingLineCmd>,
    _end_cmd: &PendingLineCmd,
    matches_start: P,
) -> Option<PendingLineCmd>
where
    P: Fn(&PendingLineCmd) -> bool,
{
    let pos = block_starts.iter().rposition(matches_start)?;
    Some(block_starts.remove(pos))
}

// ---------------------------------------------------------------------------
// Phase 3 — execution helpers
// ---------------------------------------------------------------------------

fn apply_labels(buffer: &mut FileBuffer, labels: &[PendingLineCmd]) {
    for cmd in labels {
        if let ParsedLineCmd::Label(name) = &cmd.cmd {
            buffer.set_label(name.clone(), cmd.line_index);
            if let Some(line) = buffer.lines.get_mut(cmd.line_index) {
                line.clear_prefix_cmd();
            }
        }
    }
}

fn apply_deletes(buffer: &mut FileBuffer, mut deletes: Vec<PendingLineCmd>) {
    // Process bottom-to-top to preserve indices.
    deletes.sort_by(|a, b| b.line_index.cmp(&a.line_index));
    for cmd in &deletes {
        if let ParsedLineCmd::Delete(count) = cmd.cmd {
            buffer.delete_lines(cmd.line_index, count);
        }
    }
}

/// Resolve a (sources, destination) pair into `(src_start, src_end, dest_idx)`
/// after applying the A/B destination semantics. Returns None when there is
/// no destination (caller should not invoke the move/copy in that case).
fn resolve_block_target(
    sources: &[PendingLineCmd],
    destination: Option<&PendingLineCmd>,
) -> Option<(usize, usize, usize)> {
    let dest = destination?;
    if sources.is_empty() {
        return None;
    }
    let (src_start, src_end) = if sources.len() >= 2 {
        let s = sources[0].line_index;
        let e = sources[1].line_index;
        (s.min(e), s.max(e))
    } else {
        let s = sources[0].line_index;
        (s, s)
    };
    let actual_dest = if matches!(dest.cmd, ParsedLineCmd::Before) {
        dest.line_index.saturating_sub(1)
    } else {
        dest.line_index
    };
    Some((src_start, src_end, actual_dest))
}

fn apply_moves(
    buffer: &mut FileBuffer,
    sources: &[PendingLineCmd],
    destination: Option<&PendingLineCmd>,
) {
    if let Some((s, e, d)) = resolve_block_target(sources, destination) {
        buffer.move_lines(s, e, d);
    }
}

fn apply_copies(
    buffer: &mut FileBuffer,
    sources: &[PendingLineCmd],
    destination: Option<&PendingLineCmd>,
) {
    if let Some((s, e, d)) = resolve_block_target(sources, destination) {
        buffer.copy_lines(s, e, d);
    }
}

/// Walk the repeats list, build (start, end, count) triples (a single Rn
/// becomes (i, i, n); a paired RR becomes (start_idx, end_idx, n)), and apply
/// them bottom-to-top.
fn apply_repeats(buffer: &mut FileBuffer, repeats: &[PendingLineCmd]) {
    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    let mut i = 0;
    while i < repeats.len() {
        let ParsedLineCmd::Repeat(count) = repeats[i].cmd else {
            i += 1;
            continue;
        };
        if count == 0 {
            i += 1; // unpaired end marker; skip
            continue;
        }
        // A trailing 0-count entry is the end marker for a block repeat.
        let next_is_end = repeats
            .get(i + 1)
            .map(|r| matches!(r.cmd, ParsedLineCmd::Repeat(0)))
            .unwrap_or(false);
        if next_is_end {
            pairs.push((repeats[i].line_index, repeats[i + 1].line_index, count));
            i += 2;
        } else {
            pairs.push((repeats[i].line_index, repeats[i].line_index, count));
            i += 1;
        }
    }
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    for (start, end, count) in pairs {
        buffer.repeat_lines(start, end, count);
    }
}

fn apply_inserts(buffer: &mut FileBuffer, mut inserts: Vec<PendingLineCmd>) {
    // Bottom-to-top to preserve indices.
    inserts.sort_by(|a, b| b.line_index.cmp(&a.line_index));
    info!("Processing {} inserts", inserts.len());
    for cmd in &inserts {
        if let ParsedLineCmd::Insert(count) = cmd.cmd {
            buffer.insert_lines_after(cmd.line_index, count);
        }
    }
}
