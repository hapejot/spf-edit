# 01 — Screen Layout

## Overview

SPF-Edit uses a fixed terminal screen layout modeled after IBM ISPF. The screen is divided into distinct regions, each with a specific purpose. The layout is consistent across all editing sessions and provides the user with persistent visual context: file identity, command input, line numbers, and data content are always visible in predictable positions.

## Screen Regions

### Row 0 — Title Line

The topmost row displays session metadata.

```
EDIT  filename.ext                                    Columns 00001 00080
```

| Zone | Position | Content |
|------|----------|---------|
| Mode indicator | Left | `EDIT`, `BROWSE`, or `VIEW` |
| Filename | After mode | Full or relative path of the open file |
| Modified flag | After filename | `Modified` if buffer has unsaved changes; blank otherwise |
| Message area | Center | Short feedback messages (e.g., search results, error text) |
| Column range | Right | `Columns XXXXX XXXXX` showing the visible horizontal column range |

- The title line uses a distinctive background color (blue) to visually separate it from data.
- When a message is displayed, it overwrites the center area and is cleared on the next keystroke or Enter.

### Row 1 — Command Line

The command input row provides two fields:

```
Command ===> user_input_here                         Scroll ===> PAGE
```

| Zone | Position | Content |
|------|----------|---------|
| Command prompt | Left | Fixed text `Command ===> ` |
| Command input | After prompt | User-typed primary command text |
| Scroll prompt | Right | Fixed text `Scroll ===> ` |
| Scroll amount | After scroll prompt | Current default scroll amount (PAGE, HALF, CSR, DATA, MAX, or a number) |

- The command input field accepts any primary command.
- The scroll amount field controls the default behavior of F7/F8 (UP/DOWN) when no explicit amount is typed.
- Both fields are editable when the cursor is positioned in them.

### Row 2+ — Data Area

The remaining rows display file content. Each data row has three zones:

```
000100 | This is the line content extending to the right...
```

| Zone | Width | Content |
|------|-------|---------|
| Prefix area | 6 characters | Line number, line command input, or special indicator |
| Separator | 1 character | Visual divider (space or pipe) |
| Data area | Remaining width | File content, horizontally scrollable |

#### Prefix Area Display

The prefix area shows context-dependent content:

| Line Type | Prefix Display | Description |
|-----------|----------------|-------------|
| Normal data | `000100` | 6-digit line number (when NUMBER ON) |
| Normal data | `======` | Equal signs (when NUMBER OFF) |
| Top sentinel | `******` | Top-of-Data marker |
| Bottom sentinel | `******` | Bottom-of-Data marker |
| Column ruler | `=COLS>` | Column ruler indicator |
| Message | `==MSG>` | Editor message line |
| Insert-pending | `''''''` | Insert-mode blank line |
| Pending command | The command | e.g., `CC    ` shown in highlight color |
| Error command | The command | Command text shown in error color |
| Excluded range | `- - -` | Summary of hidden lines (e.g., `- - - 5 line(s) excluded`) |

#### Sentinel Lines

Two non-editable sentinel lines bracket the file data:

```
****** ************************ Top of Data ****************************
000100   First line of file
000200   Second line of file
****** ********************** Bottom of Data ***************************
```

- Sentinels are always present and cannot be deleted.
- Cursor cannot edit sentinel line content.
- Line commands typed on sentinel lines are rejected (except where SPF allows, e.g., `I` on Top-of-Data inserts at line 1).

### Optional Regions

#### Column Ruler

When `COLS ON` is active, a ruler line is inserted after the Top-of-Data sentinel:

```
=COLS> ----+----1----+----2----+----3----+----4----+----5----+----6----+----7
```

- Tick marks at every column, `+` at every 5th, digits at every 10th.
- Ruler scrolls horizontally with the data area.
- Toggled by the `COLS` primary command.

#### Status Bar (future)

A bottom-row status bar showing: Insert/Overtype indicator, line count, column position, CAPS state, BOUNDS, encoding. Not currently rendered.

## Color Scheme

SPF-Edit uses a hardcoded ISPF-inspired color scheme:

| Element | Foreground | Background | Style |
|---------|-----------|------------|-------|
| Title bar | White | Blue | — |
| Command prompt | Green | Black (default) | — |
| Command input | White | Black | — |
| Scroll prompt | Green | Black | — |
| Prefix area (numbers) | Cyan | Black | — |
| Separator | DarkGrey | Black | — |
| Data area (normal) | White | Black | — |
| Sentinel text | Blue | Black | — |
| Column ruler | Blue | Black | — |
| Found text highlight | Black | Yellow | — |
| Pending command | Yellow | Black | — |
| Command error | Red | Black | — |
| Message text | White | Black | — |
| Error message | Red | Black | — |

## Field Focus Model

The screen has three focusable field types. The cursor can be in exactly one at a time:

| Field | Typing Behavior |
|-------|-----------------|
| CommandLine | Characters entered into command input buffer |
| ScrollField | Characters entered into scroll amount field |
| PrefixArea | Characters entered into the 6-char prefix of the current data line |
| DataArea | Characters entered into the data content of the current data line |

Focus cycling is handled by Tab (forward) and Shift+Tab (backward). See [02-cursor-navigation.md](02-cursor-navigation.md) for details.

## Screen Redraw

- **Full redraw:** Clears the entire terminal and redraws all rows. Triggered by most actions.
- **Partial redraw:** Only redraws the current line. Used when the `needs_full_redraw` flag is false (typing in the current line without structural changes).
- **Resize handling:** Terminal resize events trigger a full redraw with recalculated dimensions.

## Interactions

- **HEX mode** (future): Each data line expands to 4 rows (text + 3 hex rows), affecting visible line count.
- **Split screen** (future): Data area splits into two panels, each with its own scroll position.
- **Exclude system** (future): Excluded lines collapse into summary markers, changing visible line count.
- **COLS/TABS/BOUNDS/MASK** lines: Special lines inserted into the data area with distinctive prefix indicators.

## Status

| Aspect         | State                                                                                  |
| -------------- | -------------------------------------------------------------------------------------- |
| Title line     | **Implemented** — mode, filename, modified, message, column range                      |
| Command line   | **Implemented** — command input + scroll field                                         |
| Prefix area    | **Implemented** — line numbers, sentinels, COLS ruler, messages, pending/error display |
| Data area      | **Implemented** — horizontal scrolling, Unicode width support                          |
| Color scheme   | **Implemented** — hardcoded ISPF colors                                                |
| Field focus    | **Implemented** — 4 field types with Tab cycling                                       |
| Status bar     | **Not started**                                                                        |
| Partial redraw | **Partial** — flag exists but most actions trigger full redraw                         |
