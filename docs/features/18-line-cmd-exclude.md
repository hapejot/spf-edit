# 18 — Line Commands: Exclude (X, S, F, L)

## Overview

Exclude line commands hide or reveal lines from the display. Hidden lines still exist in the buffer but are not shown — they collapse into summary markers. This is fundamental to the ISPF workflow: exclude non-relevant lines to focus on what matters, then show them again when done.

See also [21-exclude-show-system.md](21-exclude-show-system.md) for the primary command interface to the exclude system.

## EXCLUDE — X / XX

### Syntax

| Form | Effect |
|------|--------|
| `X` | Exclude (hide) the current line |
| `X`*n* | Exclude *n* lines starting from the current line |
| `XX` ... `XX` | Exclude the marked block |

### Behavior

1. The specified lines are marked with the `%Invisible` flag.
2. A summary marker replaces the hidden lines on screen:
   ```
   - - -  5 line(s) excluded
   ```
3. Adjacent excluded ranges are merged into a single summary marker.
4. The cursor moves to the next visible line after the excluded range.
5. Lines that are already excluded are skipped (no double-exclude).
6. Sentinel lines (Top/Bottom of Data) are never excluded.
7. =FILE> lines (future, MEdit) are never excluded.

### Display of Excluded Ranges

Excluded lines are replaced by a single shadow line showing the count:

```
000100   visible line
- - -  3 line(s) excluded
000400   next visible line
```

The shadow line has a distinctive prefix (`- - - `) and cannot be edited.

## SHOW — S

### Syntax

| Form | Effect |
|------|--------|
| `S` | Show (un-exclude) 1 excluded line |
| `S`*n* | Show the first *n* lines of the excluded range |

### Behavior

1. If placed on a non-excluded line immediately before an excluded range, the command applies to that excluded range.
2. If placed on an excluded-range summary marker, it applies to that range.
3. Removes the `%Invisible` flag from the specified number of lines.
4. If *n* equals or exceeds the total excluded count, the entire range is revealed.
5. If placed on a =FILE> line (future), shows all lines in that file section.

### Placement

The `S` command is typed in the prefix area of the excluded-range summary line (`- - - N line(s) excluded`), or on the visible data line immediately above the excluded range.

## FIRST — F

### Syntax

| Form | Effect |
|------|--------|
| `F` | Show the first 1 line of the excluded range |
| `F`*n* | Show the first *n* lines of the excluded range |

### Behavior

1. Must be placed on an excluded-range summary marker.
2. "Pops" *n* lines from the **front** (top) of the excluded range, making them visible.
3. If *n* equals or exceeds the range, the entire range is revealed and the summary marker is removed.
4. The remaining range count is reduced by *n*.

### Example

```
Before:                          After F2:
- - -  5 line(s) excluded       000200   second line
                                 000300   third line
                                 - - -  3 line(s) excluded
```

## LAST — L

### Syntax

| Form | Effect |
|------|--------|
| `L` | Show the last 1 line of the excluded range |
| `L`*n* | Show the last *n* lines of the excluded range |

### Behavior

Mirror of F — "pops" *n* lines from the **bottom** (end) of the excluded range.

## Valid in Browse Mode

X, S, F, and L are all valid in browse mode because they change the display state, not the file content.

## Line Flags

The exclude system uses these flags on the `Line` struct:

| Flag | Meaning |
|------|---------|
| `%Invisible` | The line is hidden (not rendered) |
| `%Xclude` | This is a shadow marker line (`- - - N line(s) excluded`) |
| `%XPtr` | Exclude pointer (internal bookkeeping for range tracking) |
| `%Popped` | Line was revealed from an excluded range (e.g., by FIND hitting it) |

## Exclude Rebuild

After any structural edit (insert, delete, move, copy) or exclude/show operation, the exclude markers must be rebuilt:

1. Scan all lines sequentially.
2. For each run of consecutive `%Invisible` lines, create or update a `%Xclude` shadow marker with the count.
3. Remove stale shadow markers.

## Interactions

- **FIND (future):** FIND can search within excluded lines using the `X` scope modifier. If a match is found in an excluded line, that line is "popped" out of the excluded range and made visible.
- **CHANGE (future):** CHANGE respects the `NX` (non-excluded) scope by default — only changes visible lines.
- **DELETE (future):** DELETE can operate on excluded lines with the `X` scope.
- **DD block delete:** If a DD range spans excluded lines, those excluded lines are also deleted.
- **RESET EXCLUDED:** The `RESET` command with the `EXCLUDED` option reveals all excluded lines.
- **HIDE X ON/OFF (future):** `HIDE X ON` hides even the summary markers — excluded ranges are completely invisible.

## Error Conditions

| Condition | Message |
|-----------|---------|
| F/L on non-excluded line | `"FIRST/LAST allowed on EXCLUDED lines only"` |
| XX unpaired | Remains pending (yellow) |
| X on sentinel | Silently ignored |

## Examples

1. `X` on line 000300 → line 000300 hidden, replaced by `- - - 1 line(s) excluded`.
2. `X5` on line 000300 → lines 300-304 hidden.
3. `XX` on line 100, `XX` on line 500 → all lines 100-500 excluded.
4. `S3` on an excluded range of 10 → first 3 lines shown, 7 remain excluded.
5. `F1` on `- - - 10 line(s) excluded` → first line shown, 9 remain.
6. `L2` on `- - - 10 line(s) excluded` → last 2 lines shown, 8 remain.

## Status

| Aspect | State |
|--------|-------|
| `%EXCLUDED` flag on Line struct | **Exists** (defined but unused) |
| X / Xn / XX (exclude line cmd) | **Not started** |
| S / Sn (show line cmd) | **Not started** |
| F / Fn (first line cmd) | **Not started** |
| L / Ln (last line cmd) | **Not started** |
| Excluded-range shadow markers | **Not started** |
| Exclude rebuild after edits | **Not started** |
| HIDE X ON/OFF | **Not started** |
