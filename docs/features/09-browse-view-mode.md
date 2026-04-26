# 09 — Browse and View Mode

## Overview

Browse mode provides read-only file viewing. The user can navigate, search, and scroll but cannot modify file content. View mode (future) is similar but allows some editing operations. These modes protect against accidental modification of reference files.

## Entering Browse Mode

### CLI Flag

```
spf-edit --browse filename
```

### BROWSE Command (future)

```
BROWSE [filename] [.profile]
```

Min abbreviation: `B`. Opens a file in browse mode in a new tab.

### VIEW Command (future)

```
VIEW [filename] [.profile]
```

Min abbreviation: `V`. Opens a file in view mode.

### MODE Command (future)

```
MODE BROWSE|EDIT|VIEW
```

Switches the current session between modes. Switching from BROWSE to EDIT enables editing. Switching from EDIT to BROWSE disables editing (changes are preserved but no new changes allowed).

## Behavior Differences

### Browse Mode Restrictions

| Action | Allowed? |
|--------|----------|
| Scrolling (UP/DOWN/LEFT/RIGHT/TOP/BOTTOM) | Yes |
| FIND / RFIND | Yes |
| LOCATE | Yes |
| COLS | Yes |
| NUMBER | Yes |
| RESET | Yes |
| Typing in command line | Yes |
| Typing in data area | **No** — keystrokes ignored |
| Typing in prefix area | **Should be blocked** |
| Line commands (I, D, C, M, R, etc.) | **No** |
| SAVE | **No** — error: "Cannot save in browse mode" |
| CHANGE | **No** |
| END | Yes — exits without saving |
| CANCEL | Yes — exits without saving |

### Title Line Indicator

The title line shows `BROWSE` instead of `EDIT`:

```
BROWSE  filename.ext                                  Columns 00001 00080
```

### View Mode (future)

View mode is identical to browse mode but allows:
- EXCLUDE/SHOW operations (toggling visibility without modifying data)
- Labels and tags
- Clipboard CUT (copying content without modifying the file)

## Interactions

- **FIND:** Works in all modes. Search results are highlighted.
- **EXCLUDE/SHOW (future):** Allowed in browse and view modes — these change display state, not file content.
- **Line commands:** All line commands that modify data (I, D, C, M, R, etc.) are blocked. Read-only line commands (X for exclude, S for show, F for first, L for last) should be allowed in browse/view mode.
- **Profiles (future):** A profile can specify the default open mode for certain file types.
- **CANCEL DELETE:** Not allowed in browse mode — the file is read-only by intent.

## Error Conditions

| Condition | Message |
|-----------|---------|
| SAVE in browse mode | `"Cannot save in browse mode"` |
| CHANGE in browse mode | `"Command not valid in browse mode"` |
| Data editing attempt | Silently ignored (no error message) |
| Line command in browse mode | `"Line commands not valid in browse mode"` |

## Examples

1. `spf-edit --browse /etc/config` → opens read-only, title shows BROWSE.
2. User tries to type in data area → nothing happens.
3. User types `FIND /error/` → search works normally.
4. User presses F3 → exits without save prompt.

## Status

| Aspect | State |
|--------|-------|
| --browse CLI flag | **Implemented** |
| Title shows BROWSE | **Implemented** |
| Data area editing blocked | **Implemented** |
| SAVE blocked in browse | **Implemented** |
| END exits without save in browse | **Implemented** |
| Prefix area blocked in browse | **Not started** — prefix typing still allowed |
| Line commands blocked in browse | **Not started** |
| BROWSE command (open from editor) | **Not started** |
| VIEW mode | **Not started** |
| MODE switching command | **Not started** |
