# 04 — Insert / Overtype Mode

## Overview

SPF-Edit supports two text entry modes that control how typed characters interact with existing content:

- **Overtype mode** (default): Characters replace the character at the cursor position.
- **Insert mode**: Characters are inserted at the cursor position, pushing existing text to the right.

This applies to all editable fields: command line, scroll field, prefix area, and data area.

## Toggling

| Key | Action |
|-----|--------|
| Insert | Toggle between Insert and Overtype mode |

Future primitives:
- `(SetINS)` / `(SetInsert)` — force Insert mode
- `(SetOVR)` / `(ResetInsert)` — force Overtype mode
- `(ClearInsert)` — reset to profile default

## Behavior by Field

### Command Line

- **Overtype:** Character replaces the character at `command_cursor_pos`. If cursor is past the end of the command string, the character is appended.
- **Insert:** Character is inserted at `command_cursor_pos`, shifting subsequent characters right.

### Scroll Field

- **Both modes:** The scroll field is a 4-character field. In overtype, character replaces at position. In insert, character is inserted and the field is truncated to 4 characters.

### Prefix Area

- **Both modes:** The prefix is a 6-character field. Characters are placed at the cursor column within the prefix. Insert shifts existing prefix characters right; overtype replaces in place. Truncated to 6 characters.

### Data Area

- **Overtype:** Character replaces the character at the cursor's logical column (accounting for horizontal scroll offset). If the cursor is past the end of the line, the line is padded with spaces up to the cursor position, then the character is placed.
- **Insert:** Character is inserted at the cursor's logical column. All characters at and after that position shift right. Line length increases by one.

## Cursor Shape (future)

The cursor shape should indicate the current mode:

| Mode | Cursor Shape |
|------|-------------|
| Overtype | Full block or underline (larger cursor) |
| Insert | Thin vertical bar (smaller cursor) |

SPFLite supports configurable cursor height percentages (`CursNormal`, `CursInsert`) and optional vertical insert cursor.

## Visual Indicator (future)

The status bar should display the current mode:

| Mode | Status Display |
|------|---------------|
| Insert | `INS` |
| Overtype | `OVR` |

Currently, there is no on-screen indicator of the current mode.

## InsReset on Enter (future)

SPFLite optionally resets the insert mode to the profile default on every Enter (Attention). This is controlled by a per-profile `InsReset` setting. When enabled, the mode reverts after each command execution.

## Tab Bounds Mode (future)

When `TABBNDS ON` is set and Insert mode is active:

- Text between tab stops behaves as independent columns.
- Inserting within a tab zone only shifts text up to the next tab stop boundary.
- If the zone overflows (text would push past the tab stop), the editor beeps and blocks the insertion.
- `TabRelease` frees tab checking for the current line.

## Interactions

- **CAPS ON:** Typed characters are uppercased regardless of insert/overtype mode.
- **Browse mode:** Character entry is blocked in data area and prefix area; mode toggle still works but has no effect.
- **HEX mode (future):** In hex display rows, typed characters are hex nibbles that modify the underlying byte value.
- **NULLS mode:** Affects trailing space handling but not insert/overtype behavior itself.

## Error Conditions

None. Mode toggling never produces errors.

## Examples

1. Mode is Overtype. Cursor on column 5 of "Hello World". User types 'X' → line becomes "HelloXWorld" (replaced space at col 5... or rather "HelloXWorld" if char was 'W', result is "HelloXorld").
2. Mode is Insert. Same position. User types 'X' → line becomes "HelloX World" (X inserted, space+World shifted right).
3. User presses Insert key → mode toggles. No visible change until status bar is implemented.

## Status

| Aspect | State |
|--------|-------|
| Insert/Overtype toggle (Insert key) | **Implemented** |
| Overtype behavior (all fields) | **Implemented** |
| Insert behavior (all fields) | **Implemented** |
| Cursor shape change | **Not started** |
| On-screen mode indicator | **Not started** |
| SetINS/SetOVR/ClearInsert primitives | **Not started** |
| InsReset on Enter | **Not started** |
| Tab Bounds mode interaction | **Not started** |
