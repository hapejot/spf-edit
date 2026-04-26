# 10 — NULLS, CAPS, and PRESERVE

## Overview

These three settings control text entry and storage behavior. NULLS affects trailing spaces on write, CAPS affects character case during input, and PRESERVE controls trailing blank retention. Each can be toggled via primary commands and stored in the file profile.

## NULLS

**Syntax:** `NULLS ON|OFF`
**Min Abbreviation:** `NUL`

Controls whether trailing spaces are stripped from lines when the file is written.

| Setting | Behavior |
|---------|----------|
| `ON` (default) | Trailing spaces are stripped from each line before writing |
| `OFF` | Lines are written exactly as they are in the buffer, including trailing spaces |

### Use Cases

- **NULLS ON:** Standard for most text files. Keeps files clean.
- **NULLS OFF:** Needed for fixed-format files where trailing spaces are significant (columnar data, mainframe formats).

### Interaction with Data Entry

In ISPF, NULLS ON also affects the data area behavior: fields are "null-terminated" rather than space-padded, which allows the cursor to stop at the end of data rather than continuing through trailing spaces. This behavior is a future enhancement for SPF-Edit.

## CAPS

**Syntax:** `CAPS ON|OFF`
**Min Abbreviation:** `CAPS`

Controls whether typed characters are automatically uppercased.

| Setting | Behavior |
|---------|----------|
| `ON` | All alphabetic characters typed in any field are converted to uppercase |
| `OFF` (default) | Characters are entered as typed |

### Scope

When CAPS is ON, uppercasing applies to:
- Characters typed in the data area
- Characters typed in the prefix area
- Characters typed in the command line
- Characters typed in the scroll field

### AUTOCAPS (future)

**Syntax:** `AUTOCAPS ON|OFF`

A more sophisticated case mode. When AUTOCAPS is ON:
- If the text being edited is mostly uppercase, typed characters are uppercased.
- If mixed case, characters are entered as typed.
- Useful for mainframe source code (traditionally uppercase) while allowing modern mixed-case editing.

### CAPS AUTO (future)

`CAPS AUTO` toggles between active/inactive auto mode, where the editor decides per-line whether to uppercase based on existing line content.

## PRESERVE

**Syntax:** `PRESERVE ON|OFF`
**Min Abbreviation:** (future)

Controls trailing blank preservation during editing.

| Setting | Behavior |
|---------|----------|
| `ON` | Trailing blanks in lines are preserved as the user edits. Lines maintain their original trailing whitespace. |
| `OFF` (default) | Trailing blanks are stripped from lines after editing operations (on Enter/Attention). |

### PRESERVE C (future)

A special mode where trailing blanks are preserved up to the last backslash (`\`) character on the line. Used for C/C++ line continuations.

### Difference from NULLS

- **NULLS** affects file write behavior (whether trailing spaces are stripped when saving).
- **PRESERVE** affects in-memory editing behavior (whether trailing spaces are stripped after each Attention/Enter cycle).

They are independent settings:
- PRESERVE OFF + NULLS ON: Spaces trimmed during editing and on save.
- PRESERVE ON + NULLS OFF: Spaces kept during editing and on save.
- PRESERVE ON + NULLS ON: Spaces kept during editing but trimmed on save.

## MINLEN (future)

**Syntax:** `MINLEN n`

Sets the minimum line length. Lines shorter than MINLEN are padded with spaces to reach the minimum. Increasing MINLEN triggers a pad operation across all lines.

Use case: columnar editing where all lines must be at least N characters wide for overlay and column operations.

## Interactions

- **Profile (future):** NULLS, CAPS, and PRESERVE defaults are stored per-profile.
- **Record format:** NULLS OFF is typically used with RECFM F (fixed-length records).
- **FIND/CHANGE:** Search operations work on the in-memory line content. NULLS only affects write, not search.
- **Browse mode:** CAPS has no effect (no editing). NULLS/PRESERVE settings can be viewed but have no effect.

## Error Conditions

| Condition | Message |
|-----------|---------|
| NULLS with no operand | `"NULLS requires ON or OFF"` |
| NULLS with invalid operand | `"NULLS requires ON or OFF"` |
| CAPS with no operand | `"CAPS requires ON or OFF"` |
| CAPS with invalid operand | `"CAPS requires ON or OFF"` |

## Examples

1. `NULLS OFF` — file writes will preserve trailing spaces.
2. `CAPS ON` — user types "hello" → "HELLO" entered.
3. `NULLS ON` + line contains `"Hello   "` → written as `"Hello"`.

## Status

| Aspect | State |
|--------|-------|
| NULLS ON/OFF | **Implemented** |
| CAPS ON/OFF | **Implemented** |
| PRESERVE | **Not started** |
| AUTOCAPS | **Not started** |
| CAPS AUTO | **Not started** |
| MINLEN | **Not started** |
