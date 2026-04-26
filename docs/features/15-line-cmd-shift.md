# 15 — Line Commands: Shift

## Overview

Shift line commands move text left or right within lines. SPF-Edit provides three pairs of shift commands with different behaviors: column shift (honors BOUNDS), indent shift (tab-multiple), and data shift (safe, checks for data loss).

## Column Shift — ( and )

### Syntax

| Form | Effect |
|------|--------|
| `(` | Shift text left by 1 column (or default shift amount) |
| `(`*n* | Shift text left by *n* columns |
| `((` ... `((` | Block shift left |
| `((`*n* ... `((` | Block shift left by *n* columns |
| `)` | Shift text right by 1 column |
| `)`*n* | Shift text right by *n* columns |
| `))` ... `))` | Block shift right |
| `))`*n* ... `))` | Block shift right by *n* columns |

### Behavior

- **Shift Left `(`:** Removes characters from the left side of the text within BOUNDS. Characters shifted off the left edge are lost.
- **Shift Right `)`:** Inserts spaces at the left side of the text within BOUNDS. Characters may be truncated at the right bound.

### BOUNDS Interaction

When BOUNDS are set (left bound L, right bound R):

- Only text between columns L and R is affected.
- Text before column L and after column R is untouched.
- Three cases:
  1. Line shorter than left bound → no-op.
  2. Line within bounds → shift from left bound, pad/truncate at right bound.
  3. Line extends past right bound → shift the middle portion, maintaining text outside bounds.

Without BOUNDS, shift operates on the entire line.

### Post-Processing Exclude — +/- (future)

| Suffix | Effect |
|--------|--------|
| `+` | After shift, exclude the shifted lines |
| `-` | After shift, exclude other lines (inverse) |

## Indent Shift — [ and ]

### Syntax

| Form | Effect |
|------|--------|
| `[` | Shift left by 1 × tab width |
| `[`*n* | Shift left by *n* × tab width |
| `[[` ... `[[` | Block indent left |
| `[`*n* ... `[[` | Block indent left by *n* × tab width |
| `]` | Shift right by 1 × tab width |
| `]`*n* | Shift right by *n* × tab width |
| `]]` ... `]]` | Block indent right |
| `]]`*n* ... `]]` | Block indent right by *n* × tab width |

### Behavior

Indent shift works like column shift but the shift amount is multiplied by the tab/indent width:

- `[` shifts left by `tab_width` columns (e.g., 4 columns if tab is 4).
- `[2` shifts left by `2 × tab_width` columns.
- `]` shifts right by `tab_width` columns.

This is the standard way to change indentation level of code blocks.

## Data Shift — < and >

### Syntax

| Form | Effect |
|------|--------|
| `<` | Safe shift left by 1 column |
| `<`*n* | Safe shift left by *n* columns |
| `<<` ... `<<` | Block safe shift left |
| `<<`*n* ... `<<` | Block safe shift left by *n* columns |
| `>` | Safe shift right by 1 column |
| `>`*n* | Safe shift right by *n* columns |
| `>>` ... `>>` | Block safe shift right |
| `>>`*n* ... `>>` | Block safe shift right by *n* columns |

### Behavior

Data shift is a **safe** operation that checks for data loss:

- **Shift Left `<`:** Before shifting, checks if non-blank characters would be shifted off the left edge. If so, the line is flagged with a shift error and the shift is **not performed** on that line.
- **Shift Right `>`:** Before shifting, checks if non-blank characters would be shifted off the right edge (for fixed-record-length files). If so, flagged as error.
- After processing all lines, a summary error is reported: `"Data shifting incomplete on N line(s)"`.

Data shift does **not** honor BOUNDS — it operates on the entire line.

### Difference Summary

| Command | BOUNDS | Data Loss Check | Amount |
|---------|--------|-----------------|--------|
| `(` / `)` | Yes | No (data may be lost silently) | Raw columns |
| `[` / `]` | Yes | No | Tab-width multiples |
| `<` / `>` | No | Yes (reports errors) | Raw columns |

## Interactions

- **Undo (future):** Shift operations push an undo frame.
- **Browse mode:** All shift commands are blocked.
- **BOUNDS:** `(` and `[` honor BOUNDS. `<` and `>` do not.
- **Fixed record length:** Right shifts may truncate at LRECL in fixed-format files.
- **Recolorize (future):** After shifting, syntax highlighting is recalculated for affected lines.

## Error Conditions

| Condition | Message |
|-----------|---------|
| `<<`/`>>` unpaired | Remains pending (yellow) |
| Data loss on safe shift | `"Data shifting incomplete on N line(s)"` |
| Shift on sentinel line | Silently ignored |

## Examples

1. `)2` on a line with `"  Hello"` → becomes `"    Hello"` (shifted right 2).
2. `(3` on a line with `"   Hello"` → becomes `"Hello"` (shifted left 3).
3. `]` with tab width 4 on `"    code()"` → becomes `"        code()"` (indented one level).
4. `<5` on a line `"Hi there"` → error: data would be lost (non-blank in first 5 cols).
5. `<<` on line 100, `<<` on line 200, shift amount 4 → all lines 100-200 safe-shifted left 4.

## Status

| Aspect | State |
|--------|-------|
| ( / ) column shift | **Not started** |
| (( / )) block column shift | **Not started** |
| [ / ] indent shift | **Not started** |
| [[ / ]] block indent shift | **Not started** |
| < / > data shift (safe) | **Not started** |
| << / >> block data shift | **Not started** |
| BOUNDS interaction | **Not started** |
| Data loss error reporting | **Not started** |
