# 33 — Hex Mode

## Overview

Hex mode displays file content in a hexadecimal + character dump format, similar to a hex editor. It allows viewing and editing raw byte values, which is essential for working with binary data, non-printable characters, and fixed-record-length files.

## Syntax

```
HEX [ON|OFF|DATA|VERT]
```

**Min Abbreviation:** `HEX`

## Display Modes

### HEX ON (Horizontal / Data)

Each data line is displayed as three rows:

```
000100  Hello World
        48656C6C 6F20576F 726C6400 00000000    (hex top)
        00000000 00000000 00000000 00000000    (hex bottom)
```

Or in the classic ISPF 3-line format:

```
000100  H.e.l.l.o. .W.o.r.l.d...............  (character)
        4865 6C6C 6F20 576F 726C 6400 0000 0000  (hex high nybble)
        0000 0000 0000 0000 0000 0000 0000 0000  (hex low nybble)
```

### HEX VERT (Vertical)

Classic ISPF vertical hex: each character is shown with its hex value split vertically (high nybble above, low nybble below):

```
000100  H e l l o   W o r l d
        4 6 6 6 6 2 5 6 7 6 6
        8 5 C C F 0 7 F 2 C 4
```

Line 1: character display.
Line 2: high nybble of each byte.
Line 3: low nybble of each byte.

### HEX DATA

Shows hex and character side-by-side (traditional hex dump):

```
000100  48 65 6C 6C 6F 20 57 6F  72 6C 64 00 00 00 00 00  |Hello World.....|
```

### HEX OFF

Returns to normal text display mode.

## Editing in Hex Mode

### Character Row

Editing the character row works like normal overtype mode — typing replaces the character at the cursor position. The hex rows update automatically.

### Hex Rows

Editing hex digits directly modifies the byte value:
- Type valid hex digits (0-9, A-F) on the hex rows.
- Each hex digit updates the corresponding nybble.
- The character row updates automatically to reflect the new byte.
- Invalid characters (non-hex) are rejected.

### Cursor Navigation

In hex mode, the cursor moves through the hex display:
- Left/Right moves by one nybble (or character, depending on which row the cursor is on).
- Up/Down moves between the character row and the hex rows of the same line, or to the next/previous line.
- Tab cycles between character and hex rows.

## Fixed-Record Files

Hex mode is particularly useful for fixed-record-length files:
- The full record length is displayed, including trailing spaces/nulls.
- Padding bytes are visible.
- Binary data is viewable.

## Hex Display of Non-Printable Characters

| Byte Range | Display |
|------------|---------|
| 0x00-0x1F | `.` (dot) or control character name |
| 0x20-0x7E | Normal ASCII character |
| 0x7F | `.` |
| 0x80-0xFF | `.` or extended character |

## Interactions

- **Syntax highlighting:** Disabled in hex mode.
- **FIND:** FIND X'...' (hex search) works in both normal and hex mode. In hex mode, found bytes are highlighted.
- **CHANGE:** CHANGE with hex strings works in hex mode.
- **COLS:** The COLS ruler in hex mode shows byte positions rather than character positions.
- **Browse mode:** Hex mode works in browse mode (display-only).
- **Line commands:** Most line commands work normally in hex mode, but the display is different.
- **Scrolling:** Horizontal scrolling works but moves by byte position.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Invalid hex digit in edit | `"Invalid hex character"` |
| HEX with unknown operand | `"Unknown HEX option"` |

## Examples

1. `HEX ON` — enable hex display.
2. `HEX VERT` — enable vertical hex display.
3. `HEX DATA` — enable hex dump display.
4. `HEX OFF` — return to normal text display.
5. In hex mode, type `41` on the hex row to change a byte to `A` (0x41).

## Status

| Aspect | State |
|--------|-------|
| HEX ON (horizontal) | **Not started** |
| HEX VERT (vertical) | **Not started** |
| HEX DATA (dump) | **Not started** |
| HEX OFF | **Not started** |
| Hex editing (nybble edit) | **Not started** |
| Cursor navigation in hex | **Not started** |
| Non-printable display | **Not started** |
