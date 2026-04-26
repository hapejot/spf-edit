# 31 — External File Operations: COPY, CREATE, REPLACE, RENAME, CLONE, BACKUP, REVERT

## Overview

These primary commands interact with external files — importing content from other files, exporting content to new files, and managing the current file's identity and backups.

## COPY (from external file)

**Syntax:** `COPY filepath [AFTER|BEFORE] [.lab]`
**Min Abbreviation:** `COPY`

### Behavior

Reads the contents of an external file and inserts them into the current file.

| Form | Effect |
|------|--------|
| `COPY path/file.txt` | Insert file contents after the cursor line |
| `COPY path/file.txt AFTER .lab` | Insert after the labeled line |
| `COPY path/file.txt BEFORE .lab` | Insert before the labeled line |

### Options

| Option | Behavior |
|--------|----------|
| `AFTER` (default) | Insert after the target line |
| `BEFORE` | Insert before the target line |

### Notes

- The external file's line-ending style is auto-detected.
- Tab expansion is applied per the current profile settings.
- The imported lines are marked as inserted/modified.

## CREATE (write to new file)

**Syntax:** `CREATE filepath [.lab1 [.lab2]] [col1 [col2]]`
**Min Abbreviation:** `CREATE`

### Behavior

Writes lines from the current file to a new external file.

| Form | Effect |
|------|--------|
| `CREATE path/new.txt` | Write entire file to new.txt |
| `CREATE path/new.txt .start .end` | Write lines in label range |
| `CREATE path/new.txt 10 40` | Write columns 10-40 only |

### Safety

- If the target file already exists, an error is raised: `"File already exists. Use REPLACE to overwrite."`
- This prevents accidental overwrites.

## REPLACE (overwrite external file)

**Syntax:** `REPLACE filepath [.lab1 [.lab2]] [col1 [col2]]`
**Min Abbreviation:** `REP`

### Behavior

Like CREATE, but overwrites an existing file. If the file doesn't exist, it creates it.

## RENAME

**Syntax:** `RENAME new-filepath`
**Min Abbreviation:** `REN`

### Behavior

Changes the current file's name/path:

1. The file is saved to the new path.
2. The old file is **not** deleted (use the OS for that).
3. The editor's title bar and state are updated to reflect the new name.
4. Subsequent SAVE commands write to the new path.

### Restrictions

- The new path's directory must exist.
- The new path must not already be occupied by another file (unless the user confirms overwrite).

## CLONE

**Syntax:** `CLONE new-filepath`
**Min Abbreviation:** `CLONE`

### Behavior

Creates a copy of the current file at a new path without changing the current file's identity.

1. The file content is written to the new path.
2. The editor continues editing the original file.
3. The original file's modified state is unchanged.

Difference from SAVE-AS: CLONE doesn't change which file the editor is working on.

## BACKUP

**Syntax:** `BACKUP [filepath]`
**Min Abbreviation:** `BACKUP`

### Behavior

Creates a backup copy of the current file.

| Form | Effect |
|------|--------|
| `BACKUP` | Creates backup with default naming (`.bak` extension or timestamp) |
| `BACKUP path/backup.txt` | Creates backup at the specified path |

### Default Backup Naming

The default backup path is determined by profile settings:
- Same directory with `.bak` extension.
- Or a configured backup directory.
- Or timestamp-based naming: `file.txt.20240101_120000.bak`.

## REVERT

**Syntax:** `REVERT`
**Min Abbreviation:** `REVERT`

### Behavior

Discards all changes and reloads the file from disk.

1. Prompts for confirmation: `"Discard all changes and reload from disk? (Y/N)"`
2. If confirmed: reads the file fresh from disk, resetting all modifications.
3. The modified flag is cleared.
4. Undo history is cleared.
5. Labels and exclude state are reset.

### Difference from CANCEL

- `CANCEL` exits without saving. `REVERT` reloads and continues editing.

## Interactions

- **Undo (future):** COPY (import) pushes an undo frame. REVERT clears undo history.
- **Browse mode:** COPY, CREATE, REPLACE blocked. BACKUP and CLONE allowed (they don't modify the buffer). REVERT allowed.
- **MEdit (future):** COPY/CREATE/REPLACE operate on the current file section.
- **File locking (future):** RENAME may need to release and re-acquire file locks.
- **Modified flag:** COPY (import) sets modified flag. REVERT clears it. BACKUP/CLONE/CREATE/REPLACE don't change it.

## Error Conditions

| Condition | Message |
|-----------|---------|
| COPY: file not found | `"File not found: path"` |
| CREATE: file exists | `"File already exists. Use REPLACE to overwrite."` |
| RENAME: directory doesn't exist | `"Directory not found: path"` |
| RENAME: file exists at target | `"File already exists at target path"` |
| REVERT: no file on disk | `"No file on disk to revert to"` |
| REVERT: not confirmed | Operation cancelled |
| Permission denied | `"Permission denied: path"` |

## Examples

1. `COPY ../header.txt BEFORE .ZFIRST` — insert a header file at the top.
2. `CREATE output.txt .start .end` — write labeled range to a new file.
3. `REPLACE output.txt .start .end` — overwrite output.txt with the range.
4. `RENAME ../newname.txt` — rename the current file.
5. `CLONE snapshot.txt` — save a copy without changing current file identity.
6. `BACKUP` — create a backup with default naming.
7. `REVERT` — discard all changes and reload.

## Status

| Aspect | State |
|--------|-------|
| COPY (import from file) | **Not started** |
| CREATE (write to new file) | **Not started** |
| REPLACE (overwrite file) | **Not started** |
| RENAME | **Not started** |
| CLONE | **Not started** |
| BACKUP | **Not started** |
| REVERT | **Not started** |
