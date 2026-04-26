# 27 — ALIGN

## Overview

ALIGN reformats text by aligning content around a delimiter character or at specific column positions. It is used to align assignments, table columns, comments, and other structured text.

## Syntax

```
ALIGN [delimiter] [col1 [col2]] [.lab1 [.lab2]] [LEFT|RIGHT|CENTER] [PAD char]
```

**Min Abbreviation:** `ALIGN`

## Basic Usage

### Align on Delimiter

```
ALIGN =
```

Aligns text around the `=` character. Each line is adjusted so the first `=` sign appears at the same column (the maximum column among all affected lines, or a specified target column).

Before:
```
x = 1
longVariable = 2
y = 3
```

After `ALIGN =`:
```
x            = 1
longVariable = 2
y            = 3
```

### Align on Column

```
ALIGN 20
```

Right-pads or shifts text so a specific feature aligns at column 20.

## Delimiter Characters

Any single character can serve as a delimiter:
- `=` — assignment alignment
- `:` — JSON/YAML key-value alignment
- `//` — comment alignment (aligns `//` to the same column)
- `,` — CSV column alignment

## Alignment Mode

| Mode | Behavior |
|------|----------|
| `LEFT` | Align text to the left of the delimiter (default) |
| `RIGHT` | Align text to the right of the delimiter |
| `CENTER` | Center text around the delimiter |

## Range

### Label Range

```
ALIGN = .start .end
```

Only aligns lines between the two labels.

### Column Range

```
ALIGN = 10 40
```

Only considers delimiter occurrences within columns 10-40.

### Line Command Range

The ALIGN command can also be applied using line commands:

| Form | Effect |
|------|--------|
| `AL` | Align this line (with primary command parameters) |
| `AL`*n* | Align *n* lines |
| `ALL` ... `ALL` | Align the block |

(Note: `ALL` as a line command is distinct from `ALL` as a keyword.)

## PAD Character

```
ALIGN = PAD .
```

Uses `.` as the padding character instead of spaces:
```
x............ = 1
longVariable. = 2
y............ = 3
```

## Multiple Passes (future)

```
ALIGN = = :
```

Multiple delimiter characters can be specified. Alignment is performed left-to-right: first all `=` signs are aligned, then all `:` characters.

## Interactions

- **BOUNDS (future):** If BOUNDS are set, alignment operates only within the bounded columns.
- **Undo (future):** ALIGN pushes a single undo frame.
- **Browse mode:** ALIGN is blocked.
- **SORT:** Often used after SORT to re-align columns.

## Error Conditions

| Condition | Message |
|-----------|---------|
| No delimiter specified | Default behavior (align on first non-space column) |
| ALIGN in browse mode | `"Command not valid in browse mode"` |

## Examples

1. `ALIGN =` — aligns on `=` in the entire file.
2. `ALIGN = .start .end` — aligns on `=` within label range.
3. `ALIGN : LEFT` — left-aligns on `:`.
4. `ALIGN // 60` — aligns `//` comments to column 60.

## Status

| Aspect | State |
|--------|-------|
| ALIGN primary command | **Not started** |
| Delimiter alignment | **Not started** |
| LEFT/RIGHT/CENTER modes | **Not started** |
| Column/label range | **Not started** |
| PAD character | **Not started** |
| AL line command | **Not started** |
| Multiple passes | **Not started** |
