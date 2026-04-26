# 20 — CHANGE

## Overview

CHANGE finds text and replaces it. It inherits all search capabilities from FIND (string types, directions, scopes, column ranges, etc.) and adds replacement-specific options like change style, case preservation, and truncation handling.

## Syntax

```
CHANGE search-string replace-string [direction] [scope] [match-type] [col1 [col2]] [.lab1 [.lab2]] [:tag] [CS|DS] [TRUNC] [options]
```

**Min Abbreviation:** `C`

## Search String

The first string uses the same forms as FIND (see [19-find.md](19-find.md)):
- Bare word, quoted, delimited, C'...', T'...', X'...', P'...', R'...', D'...'
- `*` recalls the previous search string

## Replace String

The second string supports these forms:

### Standard Replacement

```
CHANGE /old/new/
CHANGE 'old' 'new'
```

The found text is replaced with the literal replacement string.

### Case-Sensitive — C'...'

```
CHANGE /old/ C'New'
```

Replacement is inserted with exact case as typed.

### Case-Preserving — T'...'

```
CHANGE /old/ T'new'
```

The replacement text adopts the case pattern of the found text:
- Character by character: if the found character is uppercase, the replacement character is uppercased.
- For replacement characters beyond the found string's length, the case of the last alphabetic character in the found string is used.

### Hex — X'...'

```
CHANGE /old/ X'4E6577'
```

Replace with hex-specified bytes.

### Picture — P'...'

```
CHANGE P'@@@-####' P'=>>-####'
```

Picture replacement with formatting characters:
- `=` copy character from found string
- `<` lowercase the character
- `>` uppercase the character
- `~` copy without case change

### Format — F'...'

```
CHANGE /old/ F'...'
```

Format string — only valid for the replacement (second) literal. Contains formatting characters for specialized transformations.

### Null Replacement

```
CHANGE /old/ //
```

Replace with nothing (delete the found text).

### Previous String — *

```
CHANGE * 'new'
CHANGE 'old' *
```

`*` in either position recalls the previous search or replacement string.

## Change Style — CS vs DS

| Keyword | Behavior |
|---------|----------|
| `CS` (Column Shift) | Replacement may change line length — text after the replacement shifts left or right to accommodate the new length |
| `DS` (Data Shift) | Replacement overlays in place without changing line length — if replacement is shorter, trailing space fills; if longer, it may overwrite adjacent text |

Default comes from the profile setting (`CHANGEMODE`). `CS` is the standard for most editing. `DS` is useful for columnar data where column alignment must be preserved.

## TRUNC Keyword

When a replacement would extend the line beyond the maximum record length (LRECL for fixed-format files):
- Without TRUNC: the change is rejected for that line.
- With TRUNC: the change proceeds and the line is truncated at LRECL.

## Direction, Scope, Match Type, Column Range, Label Range

All options from FIND apply identically to the search portion of CHANGE. See [19-find.md](19-find.md).

## CHANGE ALL

```
CHANGE /old/new/ ALL
```

Replaces all occurrences in the file (or within the specified scope). Reports:
```
"CHARS 'old' triggered change processing 5 times in 3 lines"
```

### Loop Detection

If MINLEN > 0 and the search string is all blanks and the replacement is null, the change could loop infinitely. This is detected and blocked with an error.

## RCHANGE

**Syntax:** `RCHANGE`
**Min Abbreviation:** `RC`
**Key Binding:** F6 (future)

Repeats the last CHANGE operation.

### Special Behavior

If the last command was FIND (not CHANGE), and FIND was successful, RCHANGE performs the change at the already-found match location — effectively "accept this match and change it." This enables a FIND/RCHANGE workflow:

1. `FIND /old/` → finds first occurrence, highlights it.
2. User reviews the match.
3. If acceptable: `RCHANGE` (or F6) → changes it and finds the next.
4. If not: `RFIND` (or F5) → skips to next occurrence.

## EQChange Tracking

Each line modified by CHANGE gets the `%EQChange` flag set. This enables:
- `LOCATE CHANGE` — jump to the next changed line.
- Visual marking of changed lines (future).

## Interactions

- **Exclude system (future):** CHANGE with NX scope only changes visible (non-excluded) lines. CHANGE with X scope only changes excluded lines.
- **BOUNDS (future):** Column boundaries restrict where changes can occur.
- **UNDO (future):** Each CHANGE ALL pushes one undo frame covering all affected lines.
- **Browse mode:** CHANGE is blocked in browse mode.
- **FIND:** CHANGE uses the same search engine. After CHANGE, RFIND continues with the same search string.

## Error Conditions

| Condition | Message |
|-----------|---------|
| No search string | `"No search string entered"` |
| No replacement string | `"No replacement string entered"` |
| `*` with no prior string | `"No previous Find/Change string available"` |
| RCHANGE with no prior params | `"No prior Search/Change parameters"` |
| Loop detection trigger | `"MINLEN > 0 and CHANGE parameters may trigger a loop"` |
| CHANGE in browse mode | `"Command not valid in browse mode"` |
| All FIND errors also apply | See [19-find.md](19-find.md) |

## Examples

1. `CHANGE /error/warning/` — changes next occurrence of "error" to "warning".
2. `C /foo/bar/ ALL` — changes all "foo" to "bar".
3. `CHANGE C'Error' T'warning'` — case-sensitive search "Error", case-preserving replace → "Warning".
4. `C /old/new/ 10 40` — change only within columns 10-40.
5. `C /  / / ALL` — change double spaces to single spaces (all occurrences).
6. `C /text// ALL` — delete all occurrences of "text".
7. `RCHANGE` after `FIND /pattern/` → changes the found match and finds next.

## Status

| Aspect | State |
|--------|-------|
| CHANGE command | **Not started** |
| All search options from FIND | **Not started** (depends on FIND enhancements) |
| CS/DS change style | **Not started** |
| TRUNC | **Not started** |
| CHANGE ALL | **Not started** |
| RCHANGE | **Not started** |
| T'...' case preservation | **Not started** |
| P'...' picture replacement | **Not started** |
| F'...' format replacement | **Not started** |
| EQChange tracking | **Not started** |
| Loop detection | **Not started** |
