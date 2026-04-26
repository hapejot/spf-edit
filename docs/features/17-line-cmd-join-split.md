# 17 — Line Commands: Join, Split, and Text Flow

## Overview

These line commands manipulate line boundaries — joining multiple lines into one, splitting a line at the cursor, or reflowing paragraph text within margins. They are essential for text editing and formatting.

## JOIN — J / JJ

### Syntax

| Form | Effect |
|------|--------|
| `J` | Join this line with the next line |
| `J`*n* | Join this line with the next *n* lines |
| `JJ` ... `JJ` | Join all lines in the marked block into one |

### Behavior

1. The text of joined lines is concatenated, separated by a **single space**.
2. Trailing spaces on each line are trimmed before joining.
3. The resulting combined line replaces the first line; subsequent lines are deleted.
4. Only operates on normal, visible data lines.

### Example

```
Before:          After J:
000100 Hello     000100 Hello World
000200 World
```

## GLUE — G / GG

### Syntax

| Form | Effect |
|------|--------|
| `G` | Glue this line with the next line |
| `G`*n* | Glue this line with the next *n* lines |
| `GG` ... `GG` | Glue all lines in the block into one |

### Behavior

Like Join, but uses a **configurable glue string** instead of a single space. The default glue character is configured via environment settings.

After execution, reports: `"Glued with "X""` where X is the glue character.

## TEXT JOIN — TJ / TJJ

### Syntax

| Form | Effect |
|------|--------|
| `TJ` | Text join this line with the next line |
| `TJ`*n* | Text join with next *n* lines |
| `TJJ` ... `TJJ` | Text join the block |

### Behavior

Like Join, but with **trimming**:
- All lines except the first are left-trimmed (leading spaces removed).
- All lines except the last are right-trimmed (trailing spaces removed).
- Joined with a single space.

This produces cleaner results when joining indented code or wrapped text.

## TEXT GLUE — TG / TGG

### Syntax

| Form | Effect |
|------|--------|
| `TG` | Text glue with next line |
| `TG`*n* | Text glue with next *n* lines |
| `TGG` ... `TGG` | Text glue the block |

### Behavior

Like Text Join but uses the configurable glue string instead of a space. Same trimming logic.

## TEXT SPLIT — TS

### Syntax

| Form | Effect |
|------|--------|
| `TS` | Split the line at the cursor position |
| `TS`*n* | Split at cursor, inserting *n* blank insert-mode lines between the halves |

### Behavior

1. The cursor must be positioned on the line where `TS` is typed.
2. Text to the left of the cursor stays on the current line (right-trimmed).
3. Text at and to the right of the cursor moves to a new line (left-trimmed), indented to match the original line's indentation.
4. `TS0` splits with no blank lines between.
5. `TS3` inserts 3 blank insert-mode lines between the two halves.
6. Honors BOUNDS: split occurs within the bounded region only.

### Cursor Requirement

The cursor **must** be on the TS line. If the cursor is elsewhere, an error is reported.

## TEXT BREAK — TB / TBB

### Syntax

| Form | Effect |
|------|--------|
| `TB` | Break line at cursor position |
| `TB`*n* | Break at cursor, inserting *n* blank normal lines between halves |
| `TBB` ... `TBB` | Block break |

### Behavior

Same as TS but inserts **normal data lines** (not insert-mode lines) between the split parts. Normal lines are not auto-cleaned on the next Enter.

## TEXT FLOW — TF / TFF / TFTF

### Syntax

| Form | Effect |
|------|--------|
| `TF` | Reflow the paragraph starting at this line |
| `TF`*n* | Reflow with right margin at column *n* |
| `TFF` ... `TFF` | Reflow all paragraphs within the block |
| `TFTF` ... `TFTF` | Alternate block form |

### Behavior

Text Flow reflows paragraph text to fit within margins:

1. **Paragraph detection:** Starting from the TF line, scans downward until encountering:
   - A blank line
   - A change in indentation (different from the second line's indent)
   - A script delimiter at line start (`.`, `:`, `&`, `<`)
   - End of file
   - End of the block range (for TFF)

2. **Word extraction:** All words from the paragraph are extracted.

3. **Re-wrapping:** Words are placed on lines within the margin:
   - First line uses the original first-line indent.
   - Subsequent lines use the second-line indent (supporting hanging indent).
   - Lines are filled to the right margin without breaking words.

4. **Right margin:** `TFn` sets the right margin to column *n*. Without a number, uses the current screen width or BOUNDS right value.

### BOUNDS Interaction

- When BOUNDS are set, text flow operates only within the bounded columns.
- Text outside bounds is untouched.
- If `TFn` specifies a right margin AND global BOUNDS are set, it's an error: `"TF right bound specified, but global BNDS are set. TF abandoned."`

### Hanging Indent

If the second line of the paragraph has greater indentation than the first, the reflow preserves this hanging indent pattern:

```
Before:                    After TF:
This is the first line     This is the first line of a
  of a very long             very long paragraph that
  paragraph that wraps.      wraps nicely within the
                             margins.
```

## Interactions

- **Undo (future):** All join/split/flow operations push an undo frame.
- **Browse mode:** All commands blocked.
- **BOUNDS:** TS/TB honor BOUNDS for split position. TF uses BOUNDS for margins.
- **Excluded lines:** Join/split operate only on visible data lines.

## Error Conditions

| Condition | Message |
|-----------|---------|
| J/G/TJ/TG: range contains non-data lines | `"Join/Glue range must be only normal Data lines"` |
| J/G: no next line to join with | `"No next line to join with"` |
| TS/TB: cursor not on the command line | `"Cursor was NOT on the TS/TB/TBB lines"` |
| TS/TB: n > 100 | `"TS/TB [n] seems unreasonable"` |
| TF: right bound with global BNDS | `"TF right bound specified, but global BNDS are set. TF abandoned."` |
| TF: indents exceed right bound | `"Line indents exceed the right bound, Text Flow abandoned."` |
| Block forms unpaired | Remains pending (yellow) |

## Examples

1. `J` on line 100 → lines 100 and 101 joined with a space.
2. `JJ` on line 100, `JJ` on line 105 → all 6 lines merged into one.
3. `TS` on line 100 with cursor at column 20 → line splits at column 20.
4. `TF72` on a paragraph → paragraph reflowed to 72-column width.
5. `TFF` on line 100, `TFF` on line 200 → all paragraphs in the range reflowed.

## Status

| Aspect | State |
|--------|-------|
| J / Jn / JJ (join) | **Not started** |
| G / Gn / GG (glue) | **Not started** |
| TJ / TJn / TJJ (text join) | **Not started** |
| TG / TGn / TGG (text glue) | **Not started** |
| TS / TSn (text split) | **Not started** |
| TB / TBn / TBB (text break) | **Not started** |
| TF / TFn / TFF / TFTF (text flow) | **Not started** |
