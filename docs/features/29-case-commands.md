# 29 — Case Commands (Primary Commands: UC, LC, SC, TC)

## Overview

Primary-level case commands convert the case of text across the file or a range. These complement the line-level case commands (see [16-line-cmd-case.md](16-line-cmd-case.md)) by operating from the command line with search and range options.

## Syntax

```
UC [search-string] [ALL] [col1 [col2]] [.lab1 [.lab2]] [X|NX]
LC [search-string] [ALL] [col1 [col2]] [.lab1 [.lab2]] [X|NX]
SC [search-string] [ALL] [col1 [col2]] [.lab1 [.lab2]] [X|NX]
TC [search-string] [ALL] [col1 [col2]] [.lab1 [.lab2]] [X|NX]
```

**Min Abbreviations:** `UC`, `LC`, `SC`, `TC`

## Commands

### UC — Uppercase

Converts all alphabetic characters to uppercase.

### LC — Lowercase

Converts all alphabetic characters to lowercase.

### SC — Sentence Case

Lowercases everything, then capitalizes:
- First character of the range.
- First alphabetic character after `.`, `?`, `!` followed by whitespace.

### TC — Title Case

Capitalizes the first letter of every word. A word starts after a non-alphanumeric character or at the beginning of the text.

## Without Search String

```
UC                      (uppercase entire file)
LC .start .end          (lowercase the label range)
TC 10 40                (title-case columns 10-40 only)
```

Without a search string, the command applies to all lines (or the specified range/columns).

## With Search String

```
UC /error/ ALL
```

Uppercases the matched text only — the rest of the line is untouched. This is a "selective case change":

1. FIND the search string.
2. Convert only the matched characters to the target case.
3. With `ALL`, repeat for all occurrences.

Reports: `"Case changed in N occurrences across M lines"`

All FIND search options apply (string types, directions, scopes, match modes).

## Column Range

```
UC 10 40
```

Only converts characters within columns 10-40. Characters outside this range are untouched.

## Label Range

```
LC .start .end
```

Only processes lines between the two labels.

## Scope

| Option | Behavior |
|--------|----------|
| `X` | Only process excluded lines |
| `NX` | Only process non-excluded lines (default) |

## BOUNDS Interaction (future)

If BOUNDS are set and no column range is specified, the case command respects BOUNDS.

## Interactions

- **Line commands (UC, LC, SC, TC):** The line commands and primary commands are complementary. Line commands are immediate on specific lines; primary commands support search-based selective conversion.
- **CAPS ON:** CAPS ON affects input; case commands affect existing data. Independent.
- **Undo (future):** Case commands push a single undo frame.
- **Browse mode:** Case commands are blocked.
- **Recolorize (future):** Syntax highlighting is recalculated after case changes.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Command in browse mode | `"Command not valid in browse mode"` |
| All FIND errors apply when search string is used | See [19-find.md](19-find.md) |

## Examples

1. `UC` — uppercase the entire file.
2. `LC .start .end` — lowercase lines between labels.
3. `UC /error/ ALL` — uppercase only the word "error" everywhere.
4. `TC 1 1 ALL` — capitalize the first character of every line (column 1).
5. `SC .start .end` — sentence-case a paragraph.

## Status

| Aspect | State |
|--------|-------|
| UC primary command (full file) | **Not started** |
| LC primary command | **Not started** |
| SC primary command | **Not started** |
| TC primary command | **Not started** |
| Search-based selective case | **Not started** |
| Column range | **Not started** |
| Label range | **Not started** |
| X/NX scope | **Not started** |
