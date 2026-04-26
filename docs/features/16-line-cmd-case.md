# 16 — Line Commands: Case Conversion

## Overview

Case conversion line commands change the letter case of text in one or more lines. Four case modes are supported: uppercase, lowercase, sentence case, and title case. All honor BOUNDS settings.

## UPPERCASE — UC

### Syntax

| Form | Effect |
|------|--------|
| `UC` | Uppercase the current line |
| `UC`*n* | Uppercase *n* lines starting from the current line |
| `UCC` ... `UCC` | Uppercase the marked block |
| `UCUC` ... `UCUC` | Alternate block form (same as UCC) |

### Behavior

Converts all alphabetic characters in the line(s) to uppercase. Non-alphabetic characters are unaffected.

## LOWERCASE — LC

### Syntax

| Form | Effect |
|------|--------|
| `LC` | Lowercase the current line |
| `LC`*n* | Lowercase *n* lines |
| `LCC` ... `LCC` | Lowercase the marked block |
| `LCLC` ... `LCLC` | Alternate block form |

### Behavior

Converts all alphabetic characters to lowercase.

## SENTENCE CASE — SC

### Syntax

| Form | Effect |
|------|--------|
| `SC` | Sentence-case the current line |
| `SC`*n* | Sentence-case *n* lines |
| `SCC` ... `SCC` | Sentence-case the marked block |
| `SCSC` ... `SCSC` | Alternate block form |

### Behavior

1. All text is lowercased first.
2. The first alphabetic character of the first line in the range is capitalized.
3. The first alphabetic character after each sentence terminator (`.`, `?`, `!`) is capitalized.
4. Sentence terminators must be followed by whitespace to trigger capitalization.

## TITLE CASE — TC

### Syntax

| Form | Effect |
|------|--------|
| `TC` | Title-case the current line |
| `TC`*n* | Title-case *n* lines |
| `TCC` ... `TCC` | Title-case the marked block |
| `TCTC` ... `TCTC` | Alternate block form |

### Behavior

Capitalizes the first letter of every word. A word boundary is a transition from a non-alphanumeric character (or start of line) to an alphanumeric character.

## BOUNDS Interaction

When BOUNDS are set (left bound L, right bound R):

- Only characters between columns L and R are affected by case conversion.
- Characters outside the bounds are untouched.

Without BOUNDS, the entire line content is converted.

## Post-Processing Exclude — +/- (future)

| Suffix | Effect |
|--------|--------|
| `+` | After conversion, exclude the converted lines |
| `-` | After conversion, exclude non-converted lines |

## Interactions

- **Undo (future):** Case conversions push an undo frame.
- **Browse mode:** All case commands are blocked.
- **CAPS ON:** CAPS ON affects typed input; case line commands affect existing data. They are independent.
- **Recolorize (future):** After case conversion, syntax highlighting is recalculated.
- **Only Data and Note lines:** Case commands operate only on normal data lines and note lines. Special lines (sentinels, COLS, TABS, etc.) are skipped.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Block form unpaired | Remains pending (yellow) |
| Command on sentinel line | Silently ignored |

## Examples

1. `UC` on `"hello world"` → `"HELLO WORLD"`
2. `LC` on `"HELLO WORLD"` → `"hello world"`
3. `SC` on `"hello world. goodbye world."` → `"Hello world. Goodbye world."`
4. `TC` on `"hello world"` → `"Hello World"`
5. `UCC` on line 100, `UCC` on line 200 → all lines 100-200 uppercased.

## Status

| Aspect | State |
|--------|-------|
| UC / UCn / UCC / UCUC | **Not started** |
| LC / LCn / LCC / LCLC | **Not started** |
| SC / SCn / SCC / SCSC | **Not started** |
| TC / TCn / TCC / TCTC | **Not started** |
| BOUNDS interaction | **Not started** |
| +/- exclude suffix | **Not started** |
