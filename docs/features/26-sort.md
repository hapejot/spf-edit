# 26 — SORT

## Overview

SORT reorders lines in the file (or a range) by the content of specified columns. It supports ascending/descending order, multiple sort keys, numeric vs. character sorting, and optional duplicate removal.

## Syntax

```
SORT [col1 [col2]] [A|D] [N] [I] [DUP] [.lab1 [.lab2]] [X|NX]
```

**Min Abbreviation:** `SORT`

## Basic Usage

```
SORT                    (sort entire file, ascending, by full line)
SORT A                  (ascending — same as default)
SORT D                  (descending)
SORT 10 20              (sort by columns 10-20)
SORT 10 20 D            (sort by columns 10-20, descending)
```

## Sort Keys

### Single Key

```
SORT col1 col2 [A|D]
```

Sorts by the text in columns `col1` through `col2`.

### Multiple Keys

```
SORT col1a col2a [A|D] col1b col2b [A|D] ...
```

Multiple column ranges can be specified. Earlier keys have higher priority (primary sort key first). Each key can have its own direction.

Example:
```
SORT 1 10 A 20 30 D
```
Sort by columns 1-10 ascending; within ties, sort by columns 20-30 descending.

## Options

### Direction

| Option | Behavior |
|--------|----------|
| `A` (Ascending) | Default. a < b < c, 0 < 1 < 2 |
| `D` (Descending) | Reverse order |

### Numeric Sort — N

```
SORT 10 20 N
```

Treats the sort key as a numeric value rather than a character string:
- Leading spaces and zeros are ignored.
- Negative numbers sort correctly.
- Non-numeric values sort before or after numeric values.

Without `N`, sorting is lexicographic (character-by-character).

### Case-Insensitive — I

```
SORT I
```

Ignores case when comparing. `"Apple"` and `"apple"` are treated as equal.

### Remove Duplicates — DUP

```
SORT DUP
```

After sorting, removes consecutive duplicate lines (comparing the full line or the sort key columns). Equivalent to SORT followed by DELETE DUP ALL.

## Range Restriction

### Label Range

```
SORT .start .end
```

Sorts only lines between `.start` and `.end` (inclusive). Lines outside the range are untouched.

### Scope

| Option | Behavior |
|--------|----------|
| `X` | Sort only excluded lines |
| `NX` | Sort only non-excluded (visible) lines |

## Stable Sort

The sort algorithm is stable — lines with equal sort keys retain their original relative order.

## Interactions

- **BOUNDS (future):** If BOUNDS are set and no column range is specified, SORT uses the BOUNDS columns as the sort key range.
- **Undo (future):** SORT pushes a single undo frame covering all reordered lines.
- **Browse mode:** SORT is blocked.
- **Exclude system:** With `X`/`NX` scope, only the matching lines participate in the sort. The other lines stay in place.
- **Renumbering:** After sort, lines are renumbered.
- **Labels:** Labels move with their lines during sort.

## Error Conditions

| Condition | Message |
|-----------|---------|
| col1 > col2 | `"Left column must be less than right column"` |
| Column out of range | Silently adjusts to line length |
| SORT in browse mode | `"Command not valid in browse mode"` |

## Examples

1. `SORT` — sort entire file ascending by full line content.
2. `SORT D` — sort entire file descending.
3. `SORT 1 10` — sort by first 10 columns.
4. `SORT 1 10 A 20 30 D` — primary key cols 1-10 asc, secondary key cols 20-30 desc.
5. `SORT 5 15 N` — numeric sort on columns 5-15.
6. `SORT DUP` — sort and remove duplicates.
7. `SORT .top .bottom` — sort only the labeled range.

## Status

| Aspect | State |
|--------|-------|
| Basic SORT (ascending, full line) | **Not started** |
| Column range sort keys | **Not started** |
| Multiple sort keys | **Not started** |
| Ascending / Descending | **Not started** |
| Numeric sort (N) | **Not started** |
| Case-insensitive (I) | **Not started** |
| DUP removal | **Not started** |
| Label range | **Not started** |
| X/NX scope | **Not started** |
| Stable sort | **Not started** |
