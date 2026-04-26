# 19 — FIND

## Overview

FIND searches for text within the file. It is the most feature-rich command in the editor, supporting multiple string types, direction control, scope filtering, column ranges, label ranges, word matching modes, and integration with the exclude system.

## Syntax

```
FIND search-string [direction] [scope] [match-type] [col1 [col2]] [.lab1 [.lab2]] [:tag] [options]
```

**Min Abbreviation:** `F`

## Search String Types

SPF-Edit supports 8 string literal forms:

### Bare String

```
FIND hello
```

Unquoted word. Case behavior follows the profile default (case-insensitive by default).

### Quoted String — Q

```
FIND 'hello world'
FIND "hello world"
FIND `hello world`
```

Delimited by matching single quotes, double quotes, or backticks. Content is literal (spaces preserved).

### Delimited String

```
FIND /hello world/
FIND $hello$
FIND #error#
```

Any non-alphanumeric character can serve as a delimiter. The first character after FIND that isn't a space becomes the delimiter. The search string is the content between the two delimiters.

### Case-Sensitive — C'...'

```
FIND C'Hello'
```

Forces exact case matching regardless of profile CAPS setting.

### Case-Insensitive (Text) — T'...'

```
FIND T'hello'
```

Forces case-insensitive matching. When used in CHANGE as a replacement, preserves the case pattern of the found text.

### Hex — X'...'

```
FIND X'48656C6C6F'
```

Hex digit pairs representing byte values. Must be even-length, valid hex characters only. Useful for searching for non-printable characters.

### Picture — P'...'

```
FIND P'=#####'
```

Pattern matching using character class codes:

| Char | Matches |
|------|---------|
| `=` | Any character |
| `@` | Any alphabetic |
| `#` | Any numeric digit |
| `$` | Any special (non-alpha, non-numeric) |
| `.` | Any non-blank |
| `-` | Any non-numeric |
| `<` | Any lowercase letter |
| `>` | Any uppercase letter |
| `~` | Any non-alphabetic |
| `\x` | Literal character *x* (escape) |
| `[` | Left margin — match must start at left bound |
| `]` | Right margin — match must end at right bound |

### Regular Expression — R'...'

```
FIND R'error\s+\d+'
```

PCRE-compatible regular expression. `^` sets left-margin flag. `$` sets right-margin flag.

### Delimited Pair — D'...'

```
FIND D'<div\|</div>'
```

Matches text bracketed by a prefix and suffix separated by `\|`. Supports `\\` and `\|` escapes.

### Previous String — *

```
FIND *
```

Recalls the previous search string and all its type flags. Error if no previous string exists.

## Direction Keywords

| Keyword | Behavior |
|---------|----------|
| `NEXT` (default) | Search forward from cursor position |
| `PREV` | Search backward from cursor position |
| `FIRST` | Search from top of file, forward |
| `LAST` | Search from bottom of file, backward |
| `ALL` | Find all occurrences; report count |

## Scope Keywords

| Keyword | Behavior |
|---------|----------|
| `X` | Only search in excluded (hidden) lines |
| `NX` | Only search in non-excluded (visible) lines |
| `U` | Only search in lines marked with User flag |
| `NU` | Only search in non-User lines |

## Match Type Keywords

| Keyword | Behavior |
|---------|----------|
| `CHARS` (default) | Match substring anywhere within text |
| `WORD` | Match whole word only (bounded by delimiters on both sides) |
| `PREFIX` | Match at the start of a word |
| `SUFFIX` | Match at the end of a word |

Word boundaries are defined by the WORD delimiter set (default: spaces, punctuation).

## Column Range

```
FIND 'text' 10 20
```

- **Single column:** `FIND 'x' 10` — match must occur at exactly column 10.
- **Column range:** `FIND 'x' 10 20` — match must start at or after column 10 and end at or before column 20.
- Without column operands, BOUNDS settings are inherited automatically.

## Label Range

```
FIND 'text' .lab1 .lab2
```

Restricts the search to lines between the two labels (inclusive). Labels can be user-defined (`.mylab`) or system labels (`.ZFIRST`, `.ZLAST`).

## Tag Range (future)

```
FIND 'text' :tagname
```

Restricts the search to lines with the specified tag.

## LEFT / RIGHT Modifiers (future)

| Keyword | Behavior |
|---------|----------|
| `LEFT` | Find the leftmost occurrence on the line (re-searches forward from column 1) |
| `RIGHT` | Find the rightmost occurrence on the line (re-searches backward from end) |

Cannot be used with RegEx.

## CQT Filtering (future)

| Keyword | Behavior |
|---------|----------|
| `C` (COMMENT) | Only match within comment regions (requires active colorization) |
| `Q` (QUOTED) | Only match within quoted string regions |
| `T` (TEXT) | Only match in non-comment, non-quoted text |

Requires syntax highlighting to be active.

## FIND ALL Behavior

`FIND ALL` counts all occurrences and reports the count:
```
"CHARS 'hello' found 5 times in 3 lines"
```

Future: FIND ALL with the exclude system should also set the `.ZLOC` label range covering the found lines.

## Wrapping

When a search reaches the bottom (forward) or top (backward) without finding a match:
- Message: `"Bottom of data reached"` or `"Top of data reached"`.
- The next RFIND wraps around to the opposite end and continues searching.

## NFIND (future)

**Syntax:** `NFIND search-string [options]`
**Min Abbreviation:** `NF`

Negative FIND — finds lines that do **not** contain the search string.

## RFIND

**Syntax:** `RFIND`
**Min Abbreviation:** `RFIND`
**Key Binding:** F5

Repeats the last FIND command in the same direction.

### Behavior

1. Uses the saved search parameters from the last FIND.
2. Continues searching from the current cursor position.
3. If the last FIND hit the top/bottom boundary, RFIND wraps around.
4. `RFIND REVERSE` (future) reverses the search direction for one search.

## FFIND (future)

File Manager variant of FIND that searches filenames in the file list.

## Interactions

- **Exclude system (future):** FIND with X/NX scope filters by exclude status. FIND in an excluded line "pops" that line out of the excluded range.
- **CHANGE:** Uses the same search engine. RFIND after CHANGE searches for the same string.
- **LOCATE FIND:** Jumps to the last FIND position.
- **Highlighting:** Found text is highlighted in a distinctive color (black on yellow).
- **Browse mode:** FIND works normally — it's a read-only operation.

## Error Conditions

| Condition | Message |
|-----------|---------|
| No search string | `"No search string entered"` |
| Null search literal | `"Search literal may not be Null"` |
| Mismatched quotes | `"Mis-matched or missing quotes detected"` |
| Unknown type code | `"Unknown literal type code"` |
| Invalid hex length | `"Invalid Hex literal length"` |
| Invalid hex chars | `"Restricted character in Hex literal"` |
| RegEx compile error | `"Regex error: ..."` |
| P'...' `[`/`]` with column ops | `"LM/RM in Picture cannot be used with column operands"` |
| `*` with no prior string | `"No previous Find string available"` |
| Not found (forward) | `"Bottom of data reached"` |
| Not found (backward) | `"Top of data reached"` |
| LEFT/RIGHT with RegEx | `"LEFT/RIGHT cannot be used with RegEx literals"` |
| All 3 CQT specified | `"Using all qualifiers C Q and T is disallowed"` |

## Examples

1. `FIND /error/` — finds next occurrence of "error" (case-insensitive).
2. `F C'Error'` — finds "Error" with exact case.
3. `FIND 'TODO' ALL` — counts all occurrences of "TODO".
4. `F P'###-####'` — finds patterns like "123-4567".
5. `FIND R'\berror\b' FIRST` — regex word-boundary search from top.
6. `FIND /text/ 10 40 .start .end` — search columns 10-40 between labels.
7. `F 'bug' PREV` — search backward.

## Status

| Aspect | State |
|--------|-------|
| Basic text search | **Implemented** |
| Quoted strings ('...', "...") | **Implemented** |
| Delimited strings (/.../) | **Implemented** |
| Bare word search | **Implemented** |
| Direction (NEXT/PREV/FIRST/LAST) | **Implemented** |
| ALL (count occurrences) | **Implemented** |
| Case-insensitive default | **Implemented** |
| RFIND (F5) | **Implemented** |
| Found-text highlighting | **Implemented** |
| C'...' case-sensitive | **Not started** |
| T'...' case-insensitive | **Not started** |
| X'...' hex strings | **Not started** |
| P'...' picture strings | **Not started** |
| R'...' regex | **Not started** |
| D'...' delimited pair | **Not started** |
| WORD/PREFIX/SUFFIX modes | **Not started** |
| Column range | **Not started** |
| Label range | **Not started** |
| Tag range | **Not started** |
| X/NX/U/NU scope | **Not started** |
| LEFT/RIGHT modifiers | **Not started** |
| CQT filtering | **Not started** |
| NFIND | **Not started** |
| `*` recall | **Not started** |
| RFIND direction memory | **Partial** — always searches forward |
| Wrap-around on RFIND | **Not started** |
