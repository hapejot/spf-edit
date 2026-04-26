# 34 — Split Screen

## Overview

Split screen divides the terminal into two independent editing views. Each view can show a different file or different regions of the same file. This is a fundamental ISPF feature enabling comparison, reference, and concurrent editing.

## Syntax

```
SPLIT [HORIZONTAL|VERTICAL] [position]
```

**Min Abbreviation:** `SPLIT` (as a screen command, distinct from the text SPLIT command)

Note: SPF-Edit may use a different command name to avoid ambiguity with the text SPLIT command. Candidates: `SCREEN SPLIT`, `VIEW SPLIT`, or a function key.

## Screen Layout

### Horizontal Split (Default)

The terminal is divided into top and bottom halves:

```
┌─────────────────────────────────┐
│ File: top.txt                    │  ← View 1 title bar
│ Command ===>                     │  ← View 1 command line
│ 000100  top file line 1          │
│ 000200  top file line 2          │
│─────────────────────────────────│  ← Split bar (double line)
│ File: bottom.txt                 │  ← View 2 title bar
│ Command ===>                     │  ← View 2 command line
│ 000100  bottom file line 1       │
│ 000200  bottom file line 2       │
└─────────────────────────────────┘
```

### Vertical Split (future)

The terminal is divided into left and right halves:

```
┌────────────────┬────────────────┐
│ File: left.txt │ File: right.txt│
│ Cmd ===>       │ Cmd ===>       │
│ 000100  left 1 │ 000100  right 1│
│ 000200  left 2 │ 000200  right 2│
└────────────────┴────────────────┘
```

### Split Position

```
SPLIT HORIZONTAL 50%
SPLIT HORIZONTAL 20
```

- Percentage: `50%` splits in the middle.
- Row number: `20` puts the split bar at row 20.
- Default: 50% of the terminal height.

## Active View

Only one view is active at a time:
- The active view receives keyboard input.
- The active view's command line is where commands are typed.
- Switch between views with a function key (typically F9 or Ctrl+Tab).

### Visual Indicator

The active view's title bar is highlighted (bright). The inactive view's title bar is dimmed.

## View Independence

Each view maintains its own:
- File buffer (or shared if viewing the same file)
- Cursor position
- Scroll position
- Command line history
- FIND/CHANGE parameters
- Exclude state

## Same File in Both Views

When both views show the same file:
- Edits in one view are immediately visible in the other.
- The buffer is shared — there is only one copy of the data.
- Each view has independent scroll position and cursor.
- Exclude state is shared (excluding in one view hides in both).

## SWAP

**Syntax:** `SWAP`

Exchanges the positions of the two views (top becomes bottom, bottom becomes top).

## UNSPLIT

**Syntax:** `UNSPLIT`

Closes the split, maximizing the active view to full screen. The other view is preserved in the background.

## SCREEN Command (future)

**Syntax:** `SCREEN [1|2|SWAP|SPLIT|UNSPLIT]`

| Form | Effect |
|------|--------|
| `SCREEN 1` | Switch to view 1 |
| `SCREEN 2` | Switch to view 2 |
| `SCREEN SWAP` | Swap view positions |
| `SCREEN SPLIT` | Create split |
| `SCREEN UNSPLIT` | Remove split |

## Function Keys

| Key | Action |
|-----|--------|
| F2 (future) | SPLIT (create split at cursor row) |
| F9 (future) | SWAP (switch active view) |

## Resize Handling

When the terminal is resized:
- The split ratio is maintained.
- Both views are re-rendered with new dimensions.
- If the terminal becomes too small (< 10 rows), the split is temporarily collapsed.

## Interactions

- **EDIT/VIEW commands (future):** `EDIT file.txt` in a split view opens the file in that view.
- **COMPARE (future):** Split screen is the foundation for a visual diff/compare mode.
- **Scrolling:** Each view scrolls independently. F7/F8 affect the active view only.
- **Line commands:** Line commands execute in the active view's file.
- **SAVE/END:** Affects only the active view's file. END in a split view closes that view's file and unsplits.

## Error Conditions

| Condition | Message |
|-----------|---------|
| SPLIT when already split | `"Screen is already split"` |
| UNSPLIT when not split | `"Screen is not split"` |
| Terminal too small to split | `"Terminal too small for split screen"` |
| SWAP when not split | `"Screen is not split"` |

## Examples

1. `SPLIT` — horizontal split at 50%.
2. Type `EDIT other.txt` in the new view → opens another file.
3. F9 → switches between views.
4. `SWAP` → exchanges top and bottom.
5. `UNSPLIT` → closes the split.

## Status

| Aspect | State |
|--------|-------|
| Horizontal split | **Not started** |
| Vertical split | **Not started** |
| Active view switching | **Not started** |
| Same-file dual view | **Not started** |
| SWAP | **Not started** |
| UNSPLIT | **Not started** |
| SCREEN command | **Not started** |
| Resize handling | **Not started** |
