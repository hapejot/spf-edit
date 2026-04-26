# 13 — Line Commands: Replicate

## Overview

The Replicate command duplicates one or more lines in place, inserting copies immediately after the original. It is a standalone immediate command — no destination marker is needed.

## Syntax

| Form | Effect |
|------|--------|
| `R` | Duplicate the current line once (1 copy inserted after it) |
| `R`*n* | Duplicate the current line *n* times |
| `RR` ... `RR` | Duplicate the entire block once |
| `RR`*n* ... `RR` | Duplicate the entire block *n* times |

## Behavior

### Single Replicate (R)

1. The line where `R` is typed is duplicated.
2. The copy is inserted immediately after the original.
3. `R3` creates 3 copies (so 4 total lines: original + 3 copies).

### Block Replicate (RR)

1. Type `RR` (optionally with a repeat count) in the prefix of the first line.
2. Type `RR` in the prefix of the last line.
3. On Enter, the entire block is duplicated as a unit.
4. `RR3` on the first line means the block is duplicated 3 times (original block + 3 copies of the block, inserted immediately after the original).

### Execution Order

Replicates are processed **bottom-to-top** to preserve line indices. If multiple R commands are entered on different lines, the lowest one executes first.

### Deep Copy

When replicating special lines (future: TABS, WORD, MASK, BOUNDS, MARK), the copy is a deep copy — the special line's metadata is fully duplicated, not shared.

## Post-Processing Exclude — +/- (future)

| Suffix | Effect |
|--------|--------|
| `+` | After replication, exclude the original lines |
| `-` | After replication, exclude the newly created copies |

## Interactions

- **Undo (future):** Replicate operations push an undo frame.
- **Browse mode:** R/RR commands are blocked.
- **RESET CMD:** Clears pending RR markers without executing.
- **Renumbering:** After replication, all lines are renumbered.

## Error Conditions

| Condition | Message |
|-----------|---------|
| RR unpaired | Remains as pending command (yellow) |
| R on sentinel line | Silently ignored |

## Examples

1. `R` on line 000300 → line 000300 is duplicated; buffer now has the original at 000300 and a copy at 000301 (after renumber).
2. `R5` on line 000300 → 5 copies of line 000300 inserted after it.
3. `RR` on line 000100, `RR` on line 000300 → lines 100-300 duplicated as a block after line 300.
4. `RR3` on line 000100, `RR` on line 000300 → lines 100-300 duplicated 3 times.

## Status

| Aspect | State |
|--------|-------|
| R / Rn (single replicate) | **Implemented** |
| RR / RRn block replicate | **Implemented** |
| Bottom-to-top execution | **Implemented** |
| Pending RR display | **Implemented** |
| Deep copy of special lines | **Not started** |
| +/- exclude suffix | **Not started** |
