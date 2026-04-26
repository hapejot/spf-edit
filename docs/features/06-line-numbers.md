# 06 — Line Numbers

## Overview

Line numbers appear in the 6-character prefix area of each data line. They provide visual reference for line positions and are used by commands like LOCATE. The numbering system can be toggled on or off, and lines are automatically renumbered after structural edits.

## NUMBER Command

**Syntax:** `NUMBER ON|OFF`
**Min Abbreviation:** `NUM`

| Operand | Effect |
|---------|--------|
| `ON` | Display numeric line numbers in the prefix area |
| `OFF` | Display `======` in the prefix area instead of numbers |

When toggled to ON, all lines are renumbered starting from the configured increment.

## Display Format

### NUMBER ON

```
000100   First line of file
000200   Second line of file
000300   Third line of file
```

- 6-digit zero-padded format.
- Numbers are assigned sequentially: 1, 2, 3, ... (increment of 1).
- Maximum line number before overflow: 999999.

### NUMBER OFF

```
======   First line of file
======   Second line of file
======   Third line of file
```

- All data lines show `======` in the prefix.
- Line numbers are still tracked internally (used by LOCATE and other commands).

### Special Line Prefixes

Regardless of NUMBER mode, special lines have fixed prefixes:

| Line Type | Prefix |
|-----------|--------|
| Top of Data sentinel | `******` |
| Bottom of Data sentinel | `******` |
| Column ruler | `=COLS>` |
| Message line | `==MSG>` |
| Insert-pending line | `''''''` |
| Excluded range (future) | `- - - ` |
| TABS line (future) | `=TABS>` |
| BOUNDS line (future) | `=BNDS>` |
| MASK line (future) | `=MASK>` |
| MARK line (future) | `=MARK>` |
| NOTE line (future) | `=NOTE>` |
| Page break (future) | `=PAGE>` |

## Renumbering

Renumbering is triggered automatically after any structural edit:

- Insert (I line command)
- Delete (D/DD line command)
- Copy (C/CC + A/B)
- Move (M/MM + A/B)
- Repeat (R/RR)

The renumber operation:
1. Iterates all lines in the buffer.
2. Assigns sequential numbers (1, 2, 3, ...) to data lines only.
3. Sentinel and special lines retain their type-specific display.

### Number Increment (future)

SPFLite uses a configurable increment (e.g., 100, so lines are numbered 000100, 000200, 000300...). This allows inserting lines without immediate renumbering. Currently, SPF-Edit uses increment 1.

### STD Numbering (future)

ISPF supports `NUMBER STD` where lines are numbered with a standard increment (typically 100 or 1000), and the rightmost 8 columns of each line contain the sequence number. This is used for mainframe source files where line numbers are embedded in the data.

## Interactions

- **Line commands in prefix:** When a user types a line command (e.g., `I`, `DD`) in the prefix area, it replaces the line number display. After execution, the line number is restored.
- **Pending commands:** A pending line command (e.g., one half of a CC block) shows the command text in the prefix in a highlight color.
- **Labels:** When a label (`.name`) is set on a line, LOCATE can reference it by label instead of number.
- **FIND/LOCATE:** LOCATE can use a line number to jump to a specific line.

## Error Conditions

| Condition | Message |
|-----------|---------|
| NUMBER with no operand | `"NUMBER requires ON or OFF"` |
| NUMBER with invalid operand | `"NUMBER requires ON or OFF"` |

## Examples

1. `NUMBER ON` — prefixes change from `======` to `000100`, `000200`, etc.
2. `NUMBER OFF` — prefixes change from numbers to `======`.
3. User deletes line 000300 → remaining lines renumber: 000100, 000200, 000300 (was 000400).

## Status

| Aspect | State |
|--------|-------|
| NUMBER ON/OFF command | **Implemented** |
| 6-digit prefix display | **Implemented** |
| Auto-renumbering after edits | **Implemented** |
| Configurable increment | **Not started** (hardcoded to 1) |
| NUMBER STD mode | **Not started** |
