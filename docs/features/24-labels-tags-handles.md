# 24 — Labels, Tags, and Handles

## Overview

Labels, Tags, and Handles are named references that can be attached to lines. They enable repeatable navigation, range specification for commands, and persistent bookmarks.

## LABELS

### What They Are

A label is a named bookmark attached to a specific line. Labels are prefixed with `.` and used in commands like LOCATE, FIND, CHANGE, EXCLUDE, etc. to specify positions or ranges.

### Setting Labels

Labels are set by typing the label name in the prefix area:

```
.mylab  000100   This is the labeled line
```

- Labels start with `.` followed by 1-5 alphanumeric characters.
- Labels are case-insensitive (stored uppercase).
- Only one label per line. Setting a new label on a line replaces the old one.
- The label occupies the prefix area; the line number is not displayed while a label is present.

### Removing Labels

- Type spaces over the label in the prefix area.
- Or use: `RESET LABEL .mylab`
- Or: `RESET LABEL` (removes all user labels).

### Using Labels in Commands

Labels define positions and ranges:

```
FIND /text/ .start .end         (search between .start and .end)
CHANGE /old/new/ .top .bottom   (change within label range)
LOCATE .mylab                   (jump to labeled line)
DELETE /text/ .a .b ALL         (delete matching lines in range)
```

### Label Persistence (future)

Labels are saved with the file state in the SQLite database, so they persist across editing sessions.

### System Labels

Reserved labels set automatically by the editor:

| Label | Points to |
|-------|-----------|
| `.ZFIRST` | First data line (after Top-of-Data) |
| `.ZLAST` | Last data line (before Bottom-of-Data) |
| `.ZCSR` | Current cursor line |
| `.ZFIND` | Line of last FIND match |
| `.ZCHANGE` | Line of last CHANGE |
| `.ZLOC` | Line of last LOCATE target |
| `.ZTOP` | Top of visible screen |
| `.ZBOT` | Bottom of visible screen |

System labels cannot be manually set or removed.

## TAGS (future)

### What They Are

Tags are named markers that can be applied to multiple lines. Unlike labels (one per line, unique name), a tag name can be applied to many lines, creating a set.

### Setting Tags

```
Command ===> TAG :bugfix 100 200
```

Tags all lines from 100 to 200 with the tag `:bugfix`.

Or using the line command:

```
:bugfix  000100   This line is tagged
```

### Using Tags in Commands

Tags act as filter sets:

```
FIND /error/ :bugfix            (search only in tagged lines)
CHANGE /old/new/ :bugfix ALL    (change only in tagged lines)
DELETE :bugfix ALL               (delete all tagged lines)
```

### Removing Tags

```
RESET TAG :bugfix
```

Removes the tag from all lines.

### Difference from Labels

| Aspect | Labels | Tags |
|--------|--------|------|
| Per-line | One label per line | Multiple tags per line |
| Uniqueness | Unique name → one line | Name → many lines |
| Command use | Position / range endpoints | Filter set |
| Prefix | `.` (dot) | `:` (colon) |

## HANDLES (future)

### What They Are

Handles are internal line identifiers that survive structural edits. When you insert, delete, move, or copy lines, handles track which physical line a reference originally pointed to.

### Purpose

In macro programming, you might save a reference to "line 100." If lines are inserted above, that line becomes 105. A handle would still point to the correct line.

### Implementation

Each line has a unique monotonically increasing ID assigned at creation. Handles map to these IDs. When lines are renumbered, the handle-to-ID mapping is unchanged.

### API (future macro system)

```
handle = LINE_HANDLE(100)      -- get handle for line 100
linenum = HANDLE_LINE(handle)  -- resolve handle to current line number
```

## TRACK POINTS (future)

Track points are lightweight handles maintained by the editor for specific purposes:

| Track Point | Purpose |
|-------------|---------|
| Cursor | Tracks cursor position across structural edits |
| Find | Tracks last FIND position |
| Change | Tracks last CHANGE position |
| Scroll | Tracks scroll anchor line |

When lines are inserted or deleted above the tracked position, the track point is adjusted automatically.

## Interactions

- **FIND/CHANGE/DELETE/EXCLUDE:** All accept label ranges (`.lab1 .lab2`) and tag filters (`:tag`).
- **LOCATE:** Can locate by label name or by type (LOCATE LABEL).
- **RESET:** Clears labels (RESET LABEL) and tags (RESET TAG).
- **State persistence (future):** Labels and tags saved in SQLite state database.
- **Renumbering:** Labels and tags are attached to line identity, not line numbers. They survive renumbering.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Label name too long | `"Label name too long (max 5 chars)"` |
| Label already exists on another line | `"Label .xxx already exists at line NNN"` |
| System label modification attempt | `"Cannot modify system label"` |
| Label not found | `"Label .xxx not found"` |
| Tag not found | `"Tag :xxx not found"` |

## Examples

1. Type `.top` in the prefix of line 000100 → line 100 is labeled `.TOP`.
2. `LOCATE .top` → scrolls to line 100.
3. `FIND /error/ .top .bottom` → searches between `.top` and `.bottom`.
4. `TAG :review 200 300` → tags lines 200-300 with `:review`.
5. `FIND /bug/ :review ALL` → counts bugs in reviewed lines.
6. `RESET LABEL` → removes all user labels.

## Status

| Aspect | State |
|--------|-------|
| User labels (set via prefix) | **Implemented** |
| Label navigation (LOCATE .lab) | **Implemented** |
| Label ranges in FIND | **Not started** |
| System labels (.ZFIRST etc.) | **Not started** |
| Label persistence (SQLite) | **Not started** |
| RESET LABEL | **Not started** |
| Tags | **Not started** |
| Handles | **Not started** |
| Track points | **Not started** |
