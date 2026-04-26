# 03 — Scrolling

## Overview

Scrolling controls which portion of the file is visible on screen. SPF-Edit provides both vertical and horizontal scrolling via primary commands, function keys, and the scroll amount field. The scroll model follows ISPF conventions: the scroll amount is configurable per-session and can be overridden per-command.

## Primary Commands

### UP

**Syntax:** `UP [amount]`
**Min Abbreviation:** `UP`

Scrolls the view upward (toward the beginning of the file).

### DOWN

**Syntax:** `DOWN [amount]`
**Min Abbreviation:** `DO`

Scrolls the view downward (toward the end of the file).

### LEFT

**Syntax:** `LEFT [amount]`
**Min Abbreviation:** `LE`

Scrolls the horizontal view leftward (decreasing column offset).

### RIGHT

**Syntax:** `RIGHT [amount]`
**Min Abbreviation:** `RI`

Scrolls the horizontal view rightward (increasing column offset).

### TOP

**Syntax:** `TOP`
**Min Abbreviation:** `TOP`

Scrolls to show the first line of the file (equivalent to `UP MAX`).

### BOTTOM

**Syntax:** `BOTTOM`
**Min Abbreviation:** `BOT`

Scrolls to show the last line of the file (equivalent to `DOWN MAX`). The view positions so that the last line is visible on screen with preceding lines filling the remaining rows.

## Scroll Amounts

When a scroll command is issued without an explicit amount, the **default scroll amount** from the scroll field on the command line is used. The amount can be:

| Amount       | Meaning                                                                        |
| ------------ | ------------------------------------------------------------------------------ |
| `PAGE`       | Scroll by one full screen of data lines (visible rows minus header)            |
| `HALF`       | Scroll by half a screen                                                        |
| `CSR`        | Scroll to place the cursor line at the top (UP) or bottom (DOWN) of the screen |
| `DATA`       | Scroll by (screen height - 1) lines — one line of overlap                      |
| `MAX`        | Scroll to the absolute top or bottom of the file                               |
| *n* (number) | Scroll by exactly *n* lines or columns                                         |

### Resolution

The `ScrollAmount::resolve()` method converts the symbolic amount to a concrete line/column count based on the current terminal dimensions:

- `PAGE` → `visible_data_rows` (terminal height - HEADER_ROWS)
- `HALF` → `visible_data_rows / 2`
- `DATA` → `visible_data_rows - 1`
- `CSR` → distance from cursor to top/bottom edge
- `MAX` → total line count (effectively jumps to start/end)
- `n` → literal value *n*

### Default Scroll Amount

The scroll field on the command line (right side of Row 1) stores the default amount. It can be changed by:

1. Typing directly into the scroll field when the cursor is positioned there.
2. Using the `SCROLL` command (future): `SCROLL PAGE` / `SCROLL HALF` / etc.

Default is `PAGE` at session start.

## Function Key Bindings

| Key | Command | Default Amount |
|-----|---------|---------------|
| F7 / PageUp | Scroll up | Uses scroll field default |
| F8 / PageDown | Scroll down | Uses scroll field default |
| F10 | Scroll left | Full data area width |
| F11 | Scroll right | Full data area width |

## Scroll Behavior Details

### Vertical Scrolling

- The view is defined by `top_line` — the index of the first buffer line displayed on the first data row.
- `scroll_up(n)` subtracts from `top_line`, clamped to 0.
- `scroll_down(n)` adds to `top_line`, clamped so at least one data line remains visible.
- After scrolling, the cursor row may need adjustment if it falls outside the visible range.

### Horizontal Scrolling

- The view is defined by `horizontal_offset` — the character column of the leftmost visible character in the data area.
- `scroll_left(n)` subtracts from `horizontal_offset`, clamped to 0.
- `scroll_right(n)` adds to `horizontal_offset`. No upper clamp (user can scroll past all data).
- The column range in the title line updates to reflect the visible range.

### Interaction with Cursor

- After vertical scroll, if the cursor was on a data line that is no longer visible, the cursor repositions to the nearest visible data line.
- After horizontal scroll, the cursor column is preserved; if the cursor column is now off-screen, the display shows it at the edge.

### ensure_visible

When an operation (e.g., FIND, LOCATE) targets a specific line, `ensure_visible(target_line)` adjusts `top_line` so the target is within the visible data rows.

## Interactions

- **Excluded lines (future):** Scroll amounts count visible (non-excluded) lines only. Scrolling past an excluded range skips over the hidden lines.
- **HEX mode (future):** Each logical line occupies 4 display rows. Scroll amounts are divided by the HEX factor to maintain line-based scrolling.
- **Split screen (future):** Each panel has its own `top_line` and `horizontal_offset`. Scrolling affects only the active panel.
- **Browse mode:** Scrolling works identically to edit mode.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Invalid scroll amount text | `"Invalid scroll amount"` |
| More than one operand to UP/DOWN/LEFT/RIGHT | `"Invalid scroll amount"` |

## Examples

1. `DOWN 10` — scrolls forward 10 lines.
2. `UP HALF` — scrolls backward by half a screen.
3. `TOP` — jumps to the first line.
4. `RIGHT 40` — shifts the horizontal view 40 columns to the right.
5. User sets scroll field to `CSR`, then presses F8 → current line scrolls to the top of the screen.

## Status

| Aspect | State |
|--------|-------|
| UP/DOWN/LEFT/RIGHT commands | **Implemented** |
| TOP/BOTTOM | **Implemented** |
| All scroll amounts (PAGE/HALF/CSR/DATA/MAX/n) | **Implemented** |
| F7/F8 bindings | **Implemented** |
| F10/F11 bindings | **Implemented** |
| Scroll field editing | **Implemented** |
| ensure_visible | **Implemented** |
| SCROLL command (set default) | **Not started** |
| Excluded-line-aware scrolling | **Not started** |
| HEX mode scroll factor | **Not started** |
