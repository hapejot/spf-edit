# 28 — SPLIT, JOIN, APPEND, PREPEND, and COMPRESS (Primary Commands)

## Overview

These primary commands operate on line boundaries and line content at the command line level (as opposed to the line commands TS/TB/TJ/TG/TF described in [17-line-cmd-join-split.md](17-line-cmd-join-split.md)).

## SPLIT

**Syntax:** `SPLIT [col] [.lab1 [.lab2]]`
**Min Abbreviation:** `SPLIT`

### Behavior

Splits lines at a specified column position across a range.

- `SPLIT 40` — splits every line at column 40. Text before column 40 stays; text at and after column 40 goes to a new line.
- `SPLIT 40 .start .end` — splits only lines between labels.
- Without a column: splits at the cursor column position on the current line only.

### Use Cases

- Converting a wide file to a narrower format.
- Breaking long lines for display or format compliance.

## JOIN

**Syntax:** `JOIN [.lab1 [.lab2]] [glue-string]`
**Min Abbreviation:** `JOIN`

### Behavior

Joins consecutive line pairs:
- Without labels: joins the current line with the next line.
- With label range: joins all consecutive pairs in the range.
- Glue string separates the joined parts (default: single space).

Note: The JOIN primary command is simpler than the J/JJ line commands. For full control, use the line commands.

## APPEND

**Syntax:** `APPEND text [.lab1 [.lab2]] [col]`
**Min Abbreviation:** `APP`

### Behavior

Appends the specified text to the end of lines:
- `APPEND //` — adds `//` to the end of every line.
- `APPEND // .start .end` — adds `//` to lines in the label range.
- `APPEND // 60` — inserts `//` at column 60 (padding with spaces if needed).

### Options

| Option | Behavior |
|--------|----------|
| `TRIM` | Trim trailing spaces before appending |
| `NOTRIM` | Append directly after existing content (including trailing spaces) |

## PREPEND

**Syntax:** `PREPEND text [.lab1 [.lab2]]`
**Min Abbreviation:** `PREP`

### Behavior

Inserts the specified text at the beginning of lines:
- `PREPEND //` — adds `//` to the start of every line.
- `PREPEND // .start .end` — adds `//` to lines in label range.
- `PREPEND "  "` — indents all lines by 2 spaces.

## COMPRESS

**Syntax:** `COMPRESS [.lab1 [.lab2]]`
**Min Abbreviation:** `COMP`

### Behavior

Removes trailing spaces from all lines:
- `COMPRESS` — removes trailing spaces from entire file.
- `COMPRESS .start .end` — within label range only.
- Additionally compresses multiple consecutive blank lines into single blank lines (future option).

### Variants (future)

| Variant | Behavior |
|---------|----------|
| `COMPRESS TRAILING` | Remove trailing spaces only (default) |
| `COMPRESS LEADING` | Remove leading spaces (left-trim) |
| `COMPRESS BLANK` | Collapse consecutive blank lines to one |
| `COMPRESS ALL` | Trailing + Leading + Blank |

## Interactions

- **BOUNDS (future):** SPLIT honors BOUNDS — splits occur within the bounded region. APPEND/PREPEND operate within bounds.
- **Undo (future):** Each command pushes a single undo frame.
- **Browse mode:** All commands blocked.
- **Excluded lines (future):** Commands operate only on visible lines (NX scope default). Use `X` scope to include excluded lines.

## Error Conditions

| Condition | Message |
|-----------|---------|
| SPLIT with column > LRECL | `"Split column exceeds record length"` |
| APPEND would exceed LRECL | Line truncated or error depending on TRUNC setting |
| Command in browse mode | `"Command not valid in browse mode"` |

## Examples

1. `SPLIT 72` — split all lines at column 72.
2. `APPEND ;` — add semicolons to all line ends.
3. `PREPEND "# "` — prefix all lines with `# ` (Markdown heading).
4. `COMPRESS` — remove trailing whitespace from entire file.
5. `APPEND // 60 .start .end` — add `//` at column 60 for lines in range.

## Status

| Aspect | State |
|--------|-------|
| SPLIT primary command | **Not started** |
| JOIN primary command | **Not started** |
| APPEND | **Not started** |
| PREPEND | **Not started** |
| COMPRESS | **Not started** |
| Label range support | **Not started** |
| BOUNDS interaction | **Not started** |
