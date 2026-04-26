# 30 — Clipboard: CUT, PASTE, and CLIP

## Overview

SPF-Edit provides clipboard operations for transferring text within the editor and to/from the system clipboard. Named clipboards allow multiple independent cut/paste buffers.

## CUT

**Syntax:** `CUT [name] [REPLACE|APPEND] [.lab1 [.lab2]] [col1 [col2]]`
**Min Abbreviation:** `CUT`

### Behavior

Removes lines from the file and places them in a named clipboard buffer.

| Form | Effect |
|------|--------|
| `CUT` | Cut current line to the default clipboard |
| `CUT .start .end` | Cut lines in label range to default clipboard |
| `CUT myname` | Cut to named clipboard "myname" |
| `CUT myname APPEND` | Append to existing clipboard content |
| `CUT myname REPLACE` | Replace clipboard content (default) |

### Column Cut (future)

```
CUT myname 10 40 .start .end
```

Cuts only columns 10-40 from the specified lines. The cut content is stored as a rectangular (columnar) block.

### Line Command Integration

The CUT command can also be driven by line commands:

| Line Command | Effect |
|--------------|--------|
| `X` (with CUT pending) | Mark lines for cut |

Or more commonly, CUT operates on the label range or the entire file.

## PASTE

**Syntax:** `PASTE [name] [AFTER|BEFORE] [.lab] [col]`
**Min Abbreviation:** `PASTE`

### Behavior

Inserts clipboard content into the file.

| Form | Effect |
|------|--------|
| `PASTE` | Paste default clipboard after cursor line |
| `PASTE myname` | Paste named clipboard |
| `PASTE BEFORE .lab` | Paste before the labeled line |
| `PASTE AFTER .lab` | Paste after the labeled line |
| `PASTE myname 20` | Columnar paste at column 20 |

### Columnar Paste

If the clipboard contains a rectangular block (from a column CUT), PASTE inserts it at the specified column. Existing text is shifted right to accommodate.

## CLIP

**Syntax:** `CLIP [IN|OUT|CLEAR] [name]`
**Min Abbreviation:** `CLIP`

### Behavior

Interacts with the **system clipboard** (OS-level copy/paste).

| Form | Effect |
|------|--------|
| `CLIP OUT` | Copy the named clipboard (or default) to the system clipboard |
| `CLIP IN` | Import the system clipboard contents into a named clipboard buffer |
| `CLIP CLEAR` | Clear the named clipboard buffer |

### System Clipboard Integration

- `CLIP OUT` converts the internal clipboard to plain text and places it on the OS clipboard.
- `CLIP IN` reads the OS clipboard as plain text and stores it in the internal clipboard buffer, split into lines.

## Named Clipboards

| Feature | Description |
|---------|-------------|
| Default clipboard | Used when no name is specified. Unnamed. |
| Named clipboards | Any alphanumeric name (e.g., `CUT myblock`). Persist for the session. |
| Persistence (future) | Named clipboards can be saved in SQLite for cross-session access. |

## Line Command CUT/PASTE (future)

| Line Command | Effect |
|--------------|--------|
| `CT` | Cut this line to clipboard |
| `CT`*n* | Cut *n* lines |
| `CTT` ... `CTT` | Cut block |
| `PT` | Paste after this line |
| `PTB` | Paste before this line |

These provide a line-command alternative to the primary CUT/PASTE commands.

## Interactions

- **Undo (future):** CUT and PASTE push undo frames.
- **Browse mode:** CUT is blocked. PASTE is blocked. CLIP OUT works (read-only export).
- **BOUNDS (future):** Column CUT/PASTE respects BOUNDS.
- **Excluded lines:** CUT on a range that includes excluded lines also cuts the excluded lines.
- **COPY/CREATE:** For file-level operations (copy to another file), see [31-external-copy-create.md](31-external-copy-create.md).

## Error Conditions

| Condition | Message |
|-----------|---------|
| PASTE with empty clipboard | `"Clipboard is empty"` |
| CUT in browse mode | `"Command not valid in browse mode"` |
| CLIP IN with empty system clipboard | `"System clipboard is empty"` |
| Clipboard name too long | `"Clipboard name too long (max 16 chars)"` |

## Examples

1. `CUT .start .end` — cut lines between labels to default clipboard.
2. `PASTE` — paste default clipboard after cursor line.
3. `CUT errors REPLACE` — cut to "errors" clipboard, replacing previous content.
4. `PASTE errors AFTER .marker` — paste "errors" clipboard after `.marker`.
5. `CLIP OUT` — export default clipboard to OS clipboard.
6. `CLIP IN` — import OS clipboard into default clipboard buffer.

## Status

| Aspect | State |
|--------|-------|
| CUT primary command | **Not started** |
| PASTE primary command | **Not started** |
| Named clipboards | **Not started** |
| Default clipboard | **Not started** |
| CLIP IN/OUT (system clipboard) | **Not started** |
| Columnar CUT/PASTE | **Not started** |
| CT/PT line commands | **Not started** |
| Clipboard persistence (SQLite) | **Not started** |
