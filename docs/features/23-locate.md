# 23 — LOCATE

## Overview

LOCATE positions the display so that a specific line or type of line appears at the top of the screen. It is the primary navigation command for jumping to labels, line numbers, excluded ranges, special lines, and other structural elements.

## Syntax

```
LOCATE target [direction]
```

**Min Abbreviation:** `L` (also `LOC`)

## Target Types

### Line Number

```
LOCATE 500
```

Scrolls so line 500 is at the top of the data area. If line 500 doesn't exist (e.g., deleted), scrolls to the nearest line.

### Label

```
LOCATE .mylab
```

Scrolls to the line with the specified user label. Labels are alphanumeric names prefixed with `.` (see [24-labels-tags-handles.md](24-labels-tags-handles.md)).

### System Labels

| Label | Target |
|-------|--------|
| `.ZFIRST` | First data line |
| `.ZLAST` | Last data line |
| `.ZCSR` | Current cursor position |
| `.ZFIND` | Last FIND match location |
| `.ZCHANGE` | Last CHANGE location |
| `.ZLOC` | Last LOCATE target |

### Line Type

```
LOCATE type [direction]
```

| Type | Jumps to |
|------|----------|
| `EXCLUDED` | Next excluded-range shadow marker |
| `SPECIAL` | Next special line (COLS, TABS, MASK, BOUNDS, NOTE) |
| `COLS` | Next COLS ruler line |
| `TABS` | Next TABS line |
| `MASK` | Next MASK line |
| `BOUNDS` | Next BOUNDS line |
| `NOTE` | Next NOTE line |
| `CHANGE` | Next line changed by CHANGE command (has %EQChange flag) |
| `COMMAND` | Next pending line command |
| `ERROR` | Next line with a command error flag |
| `LABEL` | Next labeled line |

### First / Last

```
LOCATE FIRST
LOCATE LAST
```

- `LOCATE FIRST` is equivalent to `TOP` — scrolls to top of data.
- `LOCATE LAST` is equivalent to `BOTTOM` — scrolls to bottom.

## Direction

| Direction | Behavior |
|-----------|----------|
| `NEXT` (default) | Search forward from current position |
| `PREV` | Search backward |

## RLOC — Repeat Locate

**Syntax:** `RLOC`
**Min Abbreviation:** `RLOC`

Repeats the last LOCATE command in the same direction.

## RLOCFIND

**Syntax:** `RLOCFIND`
**Min Abbreviation:** `RLOC` (context-dependent)

Repeats the last LOCATE FIND — jumps to the next line matching the last FIND criteria.

## Interactions

- **FIND:** `LOCATE .ZFIND` jumps to the last FIND match.
- **Scrolling:** LOCATE repositions the viewport. The cursor moves to the located line.
- **Excluded lines:** `LOCATE EXCLUDED` finds the next shadow marker. Locating a line number within an excluded range "pops" it out.
- **TOP/BOTTOM:** `TOP` and `BOTTOM` are convenience aliases for `LOCATE FIRST` and `LOCATE LAST`.
- **Browse mode:** LOCATE works normally.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Label not found | `"Label .xxx not found"` |
| Line number out of range | Scrolls to nearest line (no error) |
| Type not found | `"No EXCLUDED/SPECIAL/etc. line found"` |
| No prior LOCATE for RLOC | `"No prior LOCATE to repeat"` |

## Examples

1. `L 100` — scroll to line 100.
2. `LOC .start` — scroll to the line labeled `.start`.
3. `LOCATE EXCLUDED` — jump to next excluded range.
4. `L CHANGE PREV` — jump to previous line changed by CHANGE.
5. `L .ZFIND` — jump to last FIND match.
6. `RLOC` — repeat last locate.

## Status

| Aspect | State |
|--------|-------|
| LOCATE by line number | **Implemented** |
| LOCATE by label | **Implemented** |
| TOP / BOTTOM | **Implemented** |
| LOCATE EXCLUDED | **Not started** |
| LOCATE SPECIAL/COLS/TABS etc. | **Not started** |
| LOCATE CHANGE/ERROR/COMMAND | **Not started** |
| System labels (.ZFIRST etc.) | **Not started** |
| RLOC | **Not started** |
| RLOCFIND | **Not started** |
| Direction (NEXT/PREV) | **Not started** |
