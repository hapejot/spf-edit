# 08 — Save, End, Cancel

## Overview

These commands control file persistence and session termination. They follow ISPF conventions: END saves and exits, CANCEL discards and exits, SAVE writes without exiting.

## SAVE

**Syntax:** `SAVE`
**Min Abbreviation:** `SA`

Writes the current buffer contents to disk.

### Behavior

1. If the file is in browse mode, returns error: `"Cannot save in browse mode"`.
2. Calls the file write routine (see [07-file-io.md](07-file-io.md)).
3. On success: clears the `modified` flag, displays `"File saved"` message.
4. On failure: displays error message with OS details.

### SAVEALL (future)

**Syntax:** `SAVEALL`

Saves all modified open tabs in a multi-tab session.

### SAVEAS (future)

**Syntax:** `SAVEAS [filename]`

Saves the current buffer under a new filename. If no filename is given, prompts for one. Validates that the target doesn't already exist (or asks for confirmation).

## END

**Syntax:** `END`
**Min Abbreviation:** `END`
**Key Bindings:** F3, Escape

### Behavior

1. If the buffer is not modified, or the session is browse mode: exit immediately.
2. If the buffer is modified: save the file first, then exit.
3. If the save fails: display error, do not exit.

### END KEEP (future)

`END KEEP` closes the tab without saving, used internally for crash-save recovery. Not for user use.

## CANCEL

**Syntax:** `CANCEL [DELETE]`
**Min Abbreviation:** `CAN`

### Behavior

1. If the buffer is not modified: exit immediately.
2. If the buffer is modified: prompt the user for confirmation (`"File modified. Cancel without saving? (Y/N)"`).
   - If user confirms (Y): discard changes and exit.
   - If user declines (N): return to editing, command line cleared.
3. `CANCEL DELETE` (future): permanently deletes the file from disk, then exits. Prompts for confirmation if configured.

### Restrictions

- `CANCEL DELETE` is not allowed in browse mode.
- `CANCEL DELETE` is not allowed for read-only files.

## EXIT (future)

**Syntax:** `EXIT`
**Alias:** `=X`

Same as CANCEL for the current tab, then closes additional tabs or exits the editor entirely.

## RELOAD (future)

**Syntax:** `RELOAD`

Discards all changes and reloads the file from disk. Equivalent to CANCEL + re-open.

## ForceQuit

**Key Binding:** Ctrl+Q

Immediately exits the editor without saving, without any confirmation prompt. This is an emergency exit for situations where the editor is unresponsive or the user wants to abandon all changes instantly.

## Interactions

- **Browse mode:** SAVE is blocked. END exits without saving. CANCEL exits without saving.
- **Modified detection:** The `modified` flag is set whenever a data line is changed, inserted, or deleted. Cleared on SAVE.
- **Multi-tab (future):** END closes the current tab. If it's the last tab, the editor exits. SAVEALL saves all tabs.
- **BACKUP (future):** SAVE may trigger automatic backup creation before overwriting.
- **AUTOSAVE (future):** Periodic automatic saves based on action count or timer.

## Error Conditions

| Condition | Message |
|-----------|---------|
| SAVE in browse mode | `"Cannot save in browse mode"` |
| Write failure | `"Failed to save: <OS error>"` |
| END with save failure | `"Failed to save: <OS error>"` (does not exit) |
| CANCEL DELETE in browse mode | `"DELETE option cannot be used in BROWSE mode"` |

## Examples

1. User modifies file, types `SAVE` → file written, "File saved" displayed, editing continues.
2. User types `END` (or F3) → file saved and editor exits.
3. User types `CAN` on modified file → "File modified. Cancel without saving? (Y/N)" prompt.
4. User presses Ctrl+Q → immediate exit, no save, no prompt.

## Status

| Aspect | State |
|--------|-------|
| SAVE | **Implemented** |
| END (save + exit) | **Implemented** |
| END in browse mode (just exit) | **Implemented** |
| CANCEL (unmodified) | **Implemented** |
| CANCEL (modified, Y/N prompt) | **Not started** — currently exits without prompt |
| ForceQuit (Ctrl+Q) | **Implemented** |
| F3 / Esc bindings | **Implemented** |
| SAVEALL | **Not started** |
| SAVEAS | **Not started** |
| CANCEL DELETE | **Not started** |
| EXIT / =X | **Not started** |
| RELOAD | **Not started** |
