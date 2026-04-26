# 02 — Cursor Navigation

## Overview

Cursor navigation controls how the user moves the editing cursor within and between screen fields. SPF-Edit follows ISPF conventions: the cursor lives in one of four field types (command line, scroll field, prefix area, data area), and movement keys behave differently depending on the active field.

## Arrow Keys

### Up / Down

- **In DataArea or PrefixArea:** Moves the cursor to the same column on the adjacent line. If the cursor is at the top or bottom visible line, scrolls the view by one line.
- **In CommandLine or ScrollField:** Up moves to the last visible data line. Down moves to the first visible data line.
- Cursor column is preserved across vertical movement.

### Left / Right

- **In DataArea:** Moves one column left/right within the data content. At left edge (column 0), wraps to the end of the previous line. At right edge, wraps to column 0 of the next line.
- **In PrefixArea:** Moves within the 6-character prefix. At left edge, stops (no wrapping). At right edge, stops.
- **In CommandLine:** Moves within command text. At left edge, stops. At right edge, stops.
- **In ScrollField:** Moves within the 4-character scroll amount. Wraps at edges within the field.

## Home / End

### Home

- **In DataArea:** Cursor moves to column 0 (first character) of the current data line.
- **In PrefixArea:** Cursor moves to the first column of the prefix.
- **In CommandLine:** Cursor moves to position 0 of the command input.

### End

- **In DataArea:** Cursor moves to the last non-space character of the line data (EndOfText behavior). If the line is blank, moves to column 0.
- **In CommandLine:** Cursor moves to the position after the last character of the command text.

## Word Movement (future)

### Ctrl+Left (WordLeft)

Move cursor to the beginning of the previous word. A word boundary is defined by transitions between word characters (alphanumeric + underscore) and non-word characters (spaces, punctuation).

### Ctrl+Right (WordRight)

Move cursor to the beginning of the next word.

### Behavior Details

- Skips consecutive whitespace or punctuation.
- At beginning/end of line, wraps to the previous/next line.
- Works in DataArea and CommandLine fields.

## Field Focus Cycling

### Tab (Forward Cycle)

Pressing Tab advances the cursor to the next field in this order:

```
CommandLine → DataArea (first visible line) → PrefixArea (next line) → DataArea (next line) → ... → CommandLine
```

Within the data area, Tab alternates between the data content and the prefix area of successive lines, then wraps back to the command line after the last visible row.

If TABS special line is active (future), Tab jumps to the next defined tab stop within the data area instead of cycling fields.

### Shift+Tab (Backward Cycle)

Reverses the Tab cycle direction.

### NewLine (future)

The NewLine key (distinct from Enter) moves the cursor to column 1 of the next line, scrolling if at the bottom. After the last visible data line, wraps to the command line.

## Special Navigation

### Column Jump (future)

The `(Column/N)` primitive positions the cursor at column N in the current data line. Used via key binding.

### LineNo Jump (future)

The `(LineNo)` primitive moves the cursor into the prefix/line-number area of the nearest data line.

### FirstLineCmd (future)

Jumps the cursor to the prefix area of the first visible data line.

### EndOfLine vs EndOfText

- **EndOfLine** — cursor moves to the rightmost column of the logical line (including trailing spaces, up to the record length or last character position).
- **EndOfText** — cursor moves to the column after the last non-space character (trimmed end).

Currently, End uses EndOfText behavior.

## Edge Scrolling

When the cursor moves past the visible boundary:

- **Vertical:** Moving up from the top visible line scrolls the view up by one line. Moving down from the bottom visible line scrolls down by one.
- **Horizontal:** If cursor column is beyond the visible data area, the horizontal offset adjusts to bring the cursor into view. The auto-scroll amount matches the data area width.

## Interactions

- **Insert/Overtype mode:** Does not affect navigation, only character entry behavior.
- **Browse mode:** Navigation works identically; only data modification is blocked.
- **HEX mode (future):** Vertical navigation steps by HexMode factor (4 rows per logical line).
- **BOUNDS (future):** Navigation is not constrained by BOUNDS; BOUNDS only affect editing operations.
- **Excluded lines (future):** Cursor skips over excluded line ranges during vertical movement.

## Error Conditions

None. Navigation never produces error messages — cursor silently stops at boundaries.

## Examples

1. User is on line 000500 in data area, presses Up → cursor moves to line 000400, same column.
2. User is on last visible line, presses Down → view scrolls down one line, cursor stays on screen.
3. User presses Tab from command line → cursor moves to the data area of the first visible data line.
4. User presses Home in data area → cursor moves to column 0.

## Status

| Aspect                          | State           |
| ------------------------------- | --------------- |
| Arrow keys (all fields)         | **Implemented** |
| Home/End                        | **Implemented** |
| Tab/Shift+Tab field cycling     | **Implemented** |
| Word movement (Ctrl+Left/Right) | **Not started** |
| NewLine key                     | **Not started** |
| Column/LineNo jump primitives   | **Not started** |
| Edge scrolling (vertical)       | **Implemented** |
| Edge scrolling (horizontal)     | **Implemented** |
| Excluded line skip              | **Not started** |
