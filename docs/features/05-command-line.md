# 05 — Command Line

## Overview

The command line is the primary interface for issuing editor commands. It occupies Row 1 of the screen and consists of two fields: the command input area and the scroll amount field. All primary commands (FIND, CHANGE, SAVE, etc.) are entered here and executed when the user presses Enter.

## Layout

```
Command ===> user_typed_command_here                 Scroll ===> PAGE
```

- `Command ===> ` is a fixed prompt (not editable).
- The command input area extends from after the prompt to the scroll field.
- `Scroll ===> ` is a fixed prompt for the scroll amount.
- The scroll amount is a 4-character field showing the current default (PAGE, HALF, CSR, DATA, MAX, or a number).

## Command Entry

### Typing

When the cursor is in the command input field, typed characters accumulate in the command buffer. Standard editing applies:

- Backspace deletes the character before the cursor.
- Delete removes the character at the cursor.
- Home moves cursor to position 0.
- End moves cursor past the last character.
- Left/Right move within the command text.
- Insert toggles insert/overtype mode for the field.

### Execution (Enter / Attention)

Pressing Enter triggers the **Attention processing sequence**:

1. The command text is extracted from the command input buffer.
2. Line commands in prefix areas are collected and validated.
3. Line commands are executed in order (labels → deletes → moves → copies → repeats → inserts).
4. The primary command is parsed and executed.
5. The command line is cleared on success. On error, the command text is preserved and an error message is displayed.
6. Line numbers are renumbered.
7. The screen is redrawn.

### Command Clearing

- On successful execution: the command line is cleared.
- On error: the command text is preserved so the user can correct it.
- The `&` prefix (future): keeps the command in the command line after execution for repeated use.

## Command History

A history of previously executed commands is maintained (capacity: 50 entries). Commands are pushed onto the history after successful execution.

### Retrieval

| Key | Action |
|-----|--------|
| F12 | Retrieve the previous command from history (cycles backward) |
| CRETRIEV (future) | Retrieve the next command from history (cycles forward) |
| RETF (future) | Same as CRETRIEV |

Each press of F12 replaces the command line content with the next older history entry. The history pointer wraps around.

## Command Parsing

### Minimum Abbreviation

Commands are matched using a minimum-abbreviation scheme. Each command has a minimum number of characters required:

| Command | Min Chars | Examples that match |
|---------|-----------|---------------------|
| SAVE | 2 | `SA`, `SAV`, `SAVE` |
| FIND | 1 | `F`, `FI`, `FIN`, `FIND` |
| CHANGE | 1 | `C`, `CH`, `CHA`, `CHANGE` |
| DOWN | 2 | `DO`, `DOW`, `DOWN` |
| BOTTOM | 3 | `BOT`, `BOTT`, `BOTTOM` |
| CANCEL | 3 | `CAN`, `CANC`, `CANCEL` |
| LOCATE | 1 | `L`, `LO`, `LOC`, `LOCATE` |
| RESET | 3 | `RES`, `RESE`, `RESET` |
| NUMBER | 3 | `NUM`, `NUMB`, `NUMBER` |
| NULLS | 3 | `NUL`, `NULL`, `NULLS` |
| CAPS | 4 | `CAPS` |
| COLS | 3 | `COL`, `COLS` |

Matching is case-insensitive. The command verb is separated from operands by whitespace.

### Command Stacking (future)

Multiple commands can be entered on a single command line separated by a delimiter character (typically `;`):

```
Command ===> FIND /error/ ALL; EXCLUDE /error/ ALL
```

Each command is executed in order. If any command fails, subsequent commands are skipped.

## Scroll Field

The scroll amount field is a separate 4-character input area. Editing it directly changes the default scroll amount for F7/F8.

Valid values: `PAGE`, `HALF`, `CSR`, `DATA`, `MAX`, or a number (1-4 digits).

The field is reached by Tab cycling or by clicking (future mouse support).

## Interactions

- **Browse mode:** Command line works identically; browse-incompatible commands return an error.
- **Macro execution (future):** Macros can inject commands into the command line via `SPF_Cmd`.
- **Key bindings (future):** PF keys can have commands assigned that are executed as if typed on the command line.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Unknown command verb | `"Unknown command: XXXX"` |
| Command verb matches nothing | `"Unknown command: XXXX"` |
| Error in command operands | Command-specific error message |

## Examples

1. User types `FIND /hello/` and presses Enter → FIND is executed, command line cleared.
2. User types `XYZZY` and presses Enter → `"Unknown command: XYZZY"` displayed, command text preserved.
3. User presses F12 → last successful command appears in command line.
4. User presses F12 again → previous command appears.

## Status

| Aspect | State |
|--------|-------|
| Command input editing | **Implemented** |
| Enter triggers Attention sequence | **Implemented** |
| Minimum abbreviation matching | **Implemented** |
| Command history (F12) | **Implemented** |
| Scroll field editing | **Implemented** |
| Command clearing on success/error | **Implemented** |
| Command stacking (;) | **Not started** |
| & prefix (keep command) | **Not started** |
| CRETRIEV/RETF (forward history) | **Not started** |
| SCROLL command | **Not started** |
