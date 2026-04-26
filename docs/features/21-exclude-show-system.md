# 21 — Exclude/Show System (Primary Commands)

## Overview

The Exclude/Show system allows hiding and revealing lines based on search criteria. This is distinct from the line commands (X/S/F/L in [18-line-cmd-exclude.md](18-line-cmd-exclude.md)) which work on specific lines. The primary commands apply search-based filtering across the entire file or a range.

This is one of the most powerful ISPF features — it enables the user to focus on relevant lines by hiding everything else.

## EXCLUDE

**Syntax:** `EXCLUDE search-string [ALL] [direction] [scope] [match-type] [col1 [col2]] [.lab1 [.lab2]]`
**Min Abbreviation:** `EX` (also `X` as primary command)

### Behavior

Finds lines matching the search string and marks them as excluded (hidden).

- `EXCLUDE /pattern/ ALL` — hides all matching lines.
- `EXCLUDE /pattern/` — hides the next matching line.
- All FIND search options apply (string types, directions, scopes, etc.).

### Display

Excluded lines are replaced by shadow markers:
```
000100   visible line A
- - -  3 line(s) excluded
000500   visible line B
```

## NEXCLUDE

**Syntax:** `NEXCLUDE search-string [ALL] [options]`
**Min Abbreviation:** `NX`

Negative exclude — hides lines that do **not** contain the search string.

- `NEXCLUDE /important/ ALL` — hides all lines NOT containing "important" (shows only lines with "important").

This is the standard ISPF workflow: `NEXCLUDE /pattern/ ALL` to focus on matching lines.

## SHOW

**Syntax:** `SHOW search-string [ALL] [options]`
**Min Abbreviation:** `SHOW`

Reveals (un-excludes) matching excluded lines.

- `SHOW /pattern/ ALL` — reveals all excluded lines containing "pattern".
- Only searches within excluded lines (X scope is implicit).

## NSHOW

**Syntax:** `NSHOW search-string [ALL] [options]`
**Min Abbreviation:** `NSHOW`

Negative show — reveals excluded lines that do **not** contain the string.

## FLIP

**Syntax:** `FLIP search-string [ALL] [options]`
**Min Abbreviation:** `FLIP`

Toggles the exclude status of matching lines:
- Excluded lines containing the pattern → shown.
- Visible lines containing the pattern → excluded.

## NFLIP

**Syntax:** `NFLIP search-string [ALL] [options]`
**Min Abbreviation:** `NFLIP`

Negative flip — toggles on non-matching lines.

## HIDE

**Syntax:** `HIDE X|FILE ON|OFF`
**Min Abbreviation:** `HIDE`

Controls visibility of exclude markers themselves:

| Operand         | Behavior                                                                                                              |
| --------------- | --------------------------------------------------------------------------------------------------------------------- |
| `HIDE X ON`     | Hide the shadow markers (`- - - N line(s) excluded`). Excluded ranges are completely invisible — no visual indicator. |
| `HIDE X OFF`    | Show the shadow markers (default).                                                                                    |
| `HIDE FILE ON`  | Hide the =FILE> separator lines (MEdit).                                                                              |
| `HIDE FILE OFF` | Show =FILE> lines (default).                                                                                          |

## RESET EXCLUDED

```
RESET EXCLUDED
```

Reveals all excluded lines. All `%Invisible` flags are cleared and shadow markers are removed.

```
RESET
```

Without operands, RESET clears both pending line commands AND reveals all excluded lines.

## Common Workflows

### Focus on matching lines

```
Command ===> EXCLUDE /.*/ ALL       (hide everything)
Command ===> SHOW /error/ ALL       (reveal only lines with "error")
```

Or more concisely:

```
Command ===> NEXCLUDE /error/ ALL   (hide everything NOT containing "error")
```

### Progressive refinement

```
Command ===> EXCLUDE /comment/ ALL   (hide comments)
Command ===> EXCLUDE /blank/ ALL     (hide blank lines too)
Command ===> SHOW /TODO/ ALL         (reveal TODOs even if in comments)
```

### Review and restore

```
Command ===> RESET EXCLUDED          (show everything again)
```

## Search Options

All EXCLUDE/SHOW/FLIP commands accept the same options as FIND:
- String types: bare, quoted, C'', T'', X'', P'', R''
- Match modes: CHARS, WORD, PREFIX, SUFFIX
- Column ranges: col1 col2
- Label ranges: .lab1 .lab2
- Direction: NEXT, PREV, FIRST, LAST, ALL

Most commonly used with `ALL` to process the entire file at once.

## Interactions

- **FIND:** `FIND /text/ X` searches only in excluded lines. `FIND /text/ NX` searches only in visible lines.
- **CHANGE:** `CHANGE /old/new/ NX ALL` changes only in visible lines.
- **DELETE:** `DELETE /text/ ALL` deletes matching lines. `DELETE /text/ X ALL` deletes matching excluded lines.
- **LOCATE EXCLUDED:** Jumps to the next excluded range.
- **Scrolling:** Scroll amounts count only visible lines. Excluded ranges are skipped.
- **Line commands:** DD range spanning excluded lines deletes them too.
- **Browse mode:** All exclude/show commands work in browse mode (display-only change).

## Error Conditions

| Condition             | Message                      |
| --------------------- | ---------------------------- |
| HIDE with no operand  | `"HIDE requires an operand"` |
| All FIND errors apply | See [19-find.md](19-find.md) |

## Examples

1. `EX /debug/ ALL` — hide all lines containing "debug".
2. `NX /error/ ALL` — hide everything except lines containing "error".
3. `SHOW /critical/ ALL` — reveal excluded lines containing "critical".
4. `FLIP /todo/ ALL` — toggle: hidden todos become visible, visible todos become hidden.
5. `HIDE X ON` — completely hide excluded ranges (no shadow markers).
6. `RESET` — show all lines, clear pending commands.

## Status

| Aspect                    | State           |
| ------------------------- | --------------- |
| EXCLUDE primary command   | **Not started** |
| NEXCLUDE                  | **Not started** |
| SHOW primary command      | **Not started** |
| NSHOW                     | **Not started** |
| FLIP / NFLIP              | **Not started** |
| HIDE X ON/OFF             | **Not started** |
| RESET EXCLUDED            | **Not started** |
| Shadow marker rendering   | **Not started** |
| Exclude-aware scrolling   | **Not started** |
| Exclude-aware FIND/CHANGE | **Not started** |
