# 25 — BOUNDS, COLS, TABS, and Column Controls

## Overview

These features control column-based behavior: defining visible column rulers, setting operational boundaries that restrict where commands operate, and configuring tab stops.

## COLS — Column Ruler

### Primary Command

**Syntax:** `COLS [ON|OFF]`
**Min Abbreviation:** `COLS`

Toggles display of a column ruler line at the top of the data area (below the command line).

### Line Command

**Syntax:** `COLS` typed in the prefix area

Inserts a COLS ruler line at that position in the display. The ruler shows column numbers:

```
----+----1----+----2----+----3----+----4----+----5----+----6----+----7----+----8
```

### Behavior

- COLS ruler lines are non-editable display-only lines.
- They have the LineType `ColsRuler`.
- Multiple COLS lines can exist simultaneously.
- The ruler respects horizontal scroll offset — if scrolled right by 20, the ruler starts at column 21.
- COLS lines are not saved with the file.

## BOUNDS — Column Boundaries

### Primary Command

**Syntax:** `BOUNDS [left [right]]`
**Min Abbreviation:** `BND` (also `BNDS`)

Sets the left and right column boundaries that restrict where certain commands operate.

| Form | Effect |
|------|--------|
| `BOUNDS` | Display current bounds; insert BOUNDS special line |
| `BOUNDS 10 72` | Set left bound to 10, right bound to 72 |
| `BOUNDS 1 80` | Reset to full-width bounds |

### Behavior

When bounds are set:
- **FIND/CHANGE:** Only search within the bounded columns.
- **Shift commands (`(`, `)`):** Only shift text within bounds.
- **Case commands (UC, LC, etc.):** Only convert within bounds.
- **Text Flow (TF):** Uses bounds as margins.
- **Overlay:** Only overlays within bounds.

### BOUNDS Special Line

The BOUNDS line command inserts a special line showing the current boundaries visually:

```
<                         >
```

Where `<` marks the left bound and `>` marks the right bound. The user can edit these markers to change bounds interactively.

### Default

By default, bounds are 1 to the maximum record length (effectively unbounded). For variable-length records, the right bound defaults to the screen width.

## TABS

### Primary Command

**Syntax:** `TABS [ON|OFF|STD|ALL|col1 col2 ...]`
**Min Abbreviation:** `TABS`

Controls tab stop positions and display.

| Form | Effect |
|------|--------|
| `TABS ON` | Display the TABS special line |
| `TABS OFF` | Hide the TABS line |
| `TABS STD` | Set standard tab stops (every 8 columns) |
| `TABS ALL` | Set tab stops at every column (i.e., single-column tabs) |
| `TABS 5 10 15 20` | Set custom tab stops at columns 5, 10, 15, 20 |

### TABS Special Line

Displays tab stops visually:
```
- - - -5- - - 10 - - -15 - - -20
```

Or with marker characters at each tab position. The user can edit the TABS line to add/remove tab stops interactively.

### Tab Key Behavior

When the user presses the Tab key:
- In insert mode: the cursor jumps to the next tab stop.
- In overtype mode: the cursor jumps to the next tab stop (no spaces inserted).
- With TABS STD: every 8th column.
- With custom tabs: the next defined tab position.

### Hardware Tabs vs Software Tabs

| Setting | Behavior |
|---------|----------|
| Software Tabs (default) | Tab key moves cursor; no tab characters inserted |
| Hardware Tabs (XTABS) | Tab key inserts actual `\t` character |

## XTABS — Extended Tabs

**Syntax:** `XTABS [ON|OFF]`
**Min Abbreviation:** `XTABS`

When ON, the Tab key inserts a literal tab character (`\t`) instead of moving the cursor to the next soft tab stop. This is "hardware tab" mode.

## TABBNDS — Tab Boundaries (future)

**Syntax:** `TABBNDS [left [right]]`
**Min Abbreviation:** `TABBNDS`

Defines left and right boundaries for tab expansion. Only tab characters within these columns are expanded to spaces.

## MASK (future)

### Primary Command

**Syntax:** `MASK [ON|OFF]`

The MASK line provides a template for newly inserted lines. When MASK is ON and the user inserts a line (with `I`), the new line is pre-filled with the MASK content instead of being blank.

### MASK Special Line

Insert with the `MASK` line command in the prefix area. The mask content can be edited directly on the MASK special line.

### Use Cases

- Pre-filling comment prefixes: `//` for C-style comments.
- Template lines for fixed-format data.
- Column markers for alignment.

## WORD (future)

### Primary Command

**Syntax:** `WORD [delimiters]`

Defines the character set used for word boundaries. Word boundaries affect:
- FIND with WORD/PREFIX/SUFFIX match types.
- Ctrl+Left/Right word-wise cursor movement.
- Double-click word selection (future).

### Default Delimiters

Space, tab, and common punctuation: `.,;:!?()[]{}'"<>/\|@#$%^&*-+=~`

### Custom Delimiters

```
WORD .,;:()
```

Sets only the specified characters as word delimiters. All other characters are treated as word characters.

## AUTONUM / NUMBER

### Interaction with Bounds

When BOUNDS are set, NUMBER/AUTONUM settings for display are unaffected — line numbers always appear in the prefix. But BOUNDS restrict where editing commands (shift, change, etc.) operate within the data area.

## Interactions

- **FIND/CHANGE:** Bounds limit search/replace column range. Column operands on FIND/CHANGE override BOUNDS.
- **Shift commands:** `(` and `)` honor BOUNDS. `<` and `>` do not.
- **Case commands:** Honor BOUNDS.
- **Text Flow (TF):** Uses BOUNDS for margins (unless TF specifies its own right margin).
- **RESET:** `RESET BOUNDS` clears bounds to default. `RESET TABS` clears custom tabs. `RESET MASK` removes mask line.
- **Profile (future):** Default BOUNDS, TABS, and MASK settings can be saved per profile.

## Error Conditions

| Condition | Message |
|-----------|---------|
| BOUNDS left > right | `"Left bound must be less than right bound"` |
| BOUNDS right > LRECL | `"Right bound exceeds record length"` |
| TABS column out of range | Silently ignored |

## Examples

1. `COLS` → toggles the column ruler.
2. `BOUNDS 5 72` → restricts operations to columns 5-72.
3. `TABS 4 8 12 16 20` → sets tab stops at specified columns.
4. `XTABS ON` → Tab key inserts literal tab characters.
5. `BOUNDS` → shows current bounds and inserts BOUNDS special line.
6. `RESET BOUNDS` → clears column boundaries.

## Status

| Aspect | State |
|--------|-------|
| COLS ruler display | **Implemented** |
| COLS line command | **Implemented** |
| COLS respects horizontal scroll | **Implemented** |
| BOUNDS primary command | **Not started** |
| BOUNDS special line | **Not started** |
| BOUNDS interaction with commands | **Not started** |
| TABS primary command | **Not started** |
| TABS special line | **Not started** |
| Tab key → soft tab stops | **Partial** (fixed 8-col tab stop) |
| XTABS (hardware tabs) | **Not started** |
| TABBNDS | **Not started** |
| MASK | **Not started** |
| WORD (delimiter config) | **Not started** |
