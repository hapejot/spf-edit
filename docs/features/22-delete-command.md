# 22 — DELETE Primary Command

## Overview

The DELETE primary command removes lines from the file based on search criteria. Unlike the `D`/`DD` line commands which delete specific lines by position, the DELETE primary command uses pattern matching to find and delete lines — similar to EXCLUDE but permanently removing them instead of hiding them.

## Syntax

```
DELETE search-string [ALL] [direction] [scope] [match-type] [col1 [col2]] [.lab1 [.lab2]] [DUP] [NOTE|xNOTE]
```

**Min Abbreviation:** `DEL`

## Behavior

### Single Delete

```
DELETE /pattern/
```

Finds the next line matching the pattern and deletes it. Direction defaults to NEXT.

### Delete All

```
DELETE /pattern/ ALL
```

Deletes all lines matching the pattern. Reports the count of deleted lines.

**Safety:** To delete the entire file contents (all lines), the command must be fully spelled out:
```
DELETE ALL
```
The abbreviation `DEL ALL` is accepted, but if a search string is also provided, `ALL` is treated as a search option.

### Negative Delete — NDELETE

**Syntax:** `NDELETE search-string [ALL] [options]`
**Min Abbreviation:** `NDEL`

Deletes lines that do **not** contain the search string.

```
NDELETE /keep-this/ ALL
```

Deletes every line not containing "keep-this" — effectively keeping only matching lines.

## DUP Mode

```
DELETE DUP [ALL]
```

Removes consecutive duplicate lines. Only the first line of each group of identical consecutive lines is kept.

- `DELETE DUP` removes the next duplicate.
- `DELETE DUP ALL` removes all consecutive duplicates in the file.

## NOTE/xNOTE Mode (future)

```
DELETE NOTE ALL
DELETE ANOTE ALL
```

Removes NOTE-type special lines. `NOTE` removes all note types. `xNOTE` (where x is A-Z) removes notes of a specific variety.

## Search Options

All FIND search options apply:
- String types: bare, quoted, C'', T'', X'', P'', R''
- Match modes: CHARS, WORD, PREFIX, SUFFIX
- Column ranges, label ranges
- Scope: X, NX, U, NU
- Direction: NEXT, PREV, FIRST, LAST, ALL

## RSRCH / RCHNG

```
DELETE /pattern/ RSRCH
```

Reuses the prior search parameters (from a previous FIND or DELETE).

RFIND can repeat a DELETE search (finds the next match without deleting, allowing review before deletion).

## Interactions

- **Exclude system (future):** `DELETE /text/ X ALL` deletes matching excluded lines. `DELETE /text/ NX ALL` deletes only from visible lines.
- **Undo (future):** DELETE pushes an undo frame. Deleted lines can be restored with UNDO.
- **Browse mode:** DELETE is blocked.
- **Line commands:** D/DD are immediate line-level deletes. DELETE is search-based.
- **Renumbering:** After deletion, lines are renumbered.

## Error Conditions

| Condition | Message |
|-----------|---------|
| DELETE with no operand | `"No search string entered"` |
| DELETE ALL (entire file) not fully spelled | `"To delete entire file, fully spell out DELETE ALL"` |
| DELETE in browse mode | `"Command not valid in browse mode"` |
| All FIND errors also apply | See [19-find.md](19-find.md) |

## Examples

1. `DELETE /debug/ ALL` — removes all lines containing "debug".
2. `NDEL /important/ ALL` — removes all lines not containing "important".
3. `DEL DUP ALL` — removes consecutive duplicate lines.
4. `DELETE /TODO/ 10 40 ALL` — removes lines with "TODO" in columns 10-40.
5. `DELETE /temp/ .start .end ALL` — removes matching lines between labels.

## Status

| Aspect | State |
|--------|-------|
| DELETE primary command | **Not started** |
| NDELETE | **Not started** |
| DUP mode | **Not started** |
| NOTE/xNOTE mode | **Not started** |
| RSRCH/RCHNG | **Not started** |
| All FIND search options | **Not started** |
