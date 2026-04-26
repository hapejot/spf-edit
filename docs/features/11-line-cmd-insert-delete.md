# 11 — Line Commands: Insert and Delete

## Overview

Insert and Delete are the most fundamental line commands. They add or remove lines from the file. Line commands are typed into the 6-character prefix area and executed on Enter.

## INSERT — I

### Syntax

| Form | Effect |
|------|--------|
| `I` | Insert 1 blank line after the current line |
| `I`*n* | Insert *n* blank lines after the current line (e.g., `I5` inserts 5 lines) |

### Behavior

1. New blank lines are inserted immediately after the line where `I` was typed.
2. Inserted lines are marked as insert-mode lines and display `''''''` in the prefix area until the next Enter.
3. If the MASK special line is active (future), inserted lines are pre-filled with the MASK template instead of being blank.
4. The cursor moves to the first inserted line's data area, indented to match the line above (future: auto-indent).
5. On the next Enter, if an inserted line was left completely blank, it is automatically removed (insert-line cleanup).

### Special Cases

- `I` on the Top-of-Data sentinel: inserts at the very beginning of the file (before line 1).
- `I` on the Bottom-of-Data sentinel: not valid — use `I` on the last data line instead.
- Numeric suffix: `I0` is a no-op (insert 0 lines).

## NEWLINE INSERT — N (future)

### Syntax

| Form | Effect |
|------|--------|
| `N` | Insert 1 normal data line after the current line |
| `N`*n* | Insert *n* normal data lines |

### Behavior

Unlike `I`, lines inserted with `N`:
- Are immediately treated as normal data lines (no `''''''` display).
- Are not auto-cleaned if left blank on the next Enter.
- Do not use the MASK template.

`N` is useful when you want persistent blank lines (e.g., adding empty lines for formatting).

## DELETE — D / DD

### Syntax

| Form | Effect |
|------|--------|
| `D` | Delete the current line |
| `D`*n* | Delete *n* lines starting from the current line (e.g., `D5` deletes 5 lines) |
| `DD` ... `DD` | Delete all lines in the marked block (paired) |

### Behavior

1. Lines are removed from the buffer immediately.
2. Sentinel lines (Top/Bottom of Data) cannot be deleted — the command is ignored.
3. After deletion, all remaining lines are renumbered.
4. The cursor moves to the line that was below the deleted range.

### Block Delete (DD)

1. Type `DD` in the prefix of the first line to delete.
2. Type `DD` in the prefix of the last line to delete.
3. Press Enter — all lines from the first `DD` to the second `DD` (inclusive) are deleted.
4. If only one `DD` is entered (unpaired), it becomes a **pending command** shown in yellow. It will pair with the next `DD` entered on a subsequent Enter.

### Execution Order

Deletes are processed **bottom-to-top** to preserve line indices during deletion. This means if both `D3` on line 10 and `D2` on line 20 are entered, line 20's deletion executes first.

### Interaction with Excluded Lines

When `DD` range spans excluded lines (future), the excluded lines within the range are also deleted. The exclude markers are rebuilt after deletion.

## Interactions

- **Undo (future):** Delete operations push an undo frame. `UNDO` restores deleted lines.
- **Browse mode:** I and D commands are blocked.
- **RESET CMD:** Clears pending DD markers without executing.
- **MEdit (future):** Deleting a `=FILE>` separator line deletes the entire file section.

## Error Conditions

| Condition | Message |
|-----------|---------|
| D on sentinel line | Silently ignored |
| DD unpaired | Remains as pending command (yellow) |
| DD range overlaps other line commands | `"Line command at line X overlaps subsequent line commands"` |

## Examples

1. Type `I` on line 000300 → blank line inserted after 000300, numbered 000301 (or renumbered).
2. Type `I3` on line 000300 → 3 blank lines inserted after 000300.
3. Type `D` on line 000300 → line 000300 removed.
4. Type `D5` on line 000300 → lines 000300-000304 removed.
5. Type `DD` on line 000100 and `DD` on line 000500 → all lines 000100-000500 deleted.
6. Type `DD` on line 000100 only → prefix shows `DD` in yellow (pending). Next time user types `DD` on another line and presses Enter, the block deletes.

## Status

| Aspect | State |
|--------|-------|
| I / In (insert) | **Implemented** |
| D / Dn (single delete) | **Implemented** |
| DD block delete | **Implemented** |
| Insert-line cleanup on Enter | **Implemented** |
| N / Nn (newline insert) | **Not started** |
| MASK template on insert | **Not started** |
| Auto-indent on insert | **Not started** |
| Pending DD display (yellow) | **Implemented** |
