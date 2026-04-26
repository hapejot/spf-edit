# 35 — Profiles and Configuration

## Overview

SPF-Edit uses a SQLite database for all persistent configuration. This includes editing profiles (per-language/file-type settings), key bindings, editor state, and global settings. Profiles are matched to files using regex patterns on the file path.

## Configuration Database

### Storage

All configuration is stored in a single SQLite database file:
```
~/.config/spf-edit/config.db      (Linux/macOS)
%APPDATA%\spf-edit\config.db      (Windows)
```

Using SQLite provides:
- Atomic transactions (no corrupt config files).
- Schema migration for upgrades.
- Query capability for complex lookups.
- Single-file portability.

### Schema Overview

| Table | Purpose |
|-------|---------|
| `profiles` | Editing profiles (one row per profile) |
| `profile_patterns` | Regex patterns mapping file paths to profiles |
| `key_bindings` | Key combo → action mappings |
| `settings` | Global editor settings (key-value) |
| `state` | Persistent editor state (per-file) |
| `clipboards` | Named clipboard contents |
| `macros` | Stored macros (future) |

## Profiles

### What Is a Profile

A profile is a named collection of editing settings applied when a file is opened. Each profile configures: colorize file, tab settings, bounds, record format, case behavior, and more.

### Profile Matching

When a file is opened, its full path is matched against regex patterns in `profile_patterns`:

```sql
CREATE TABLE profile_patterns (
    id INTEGER PRIMARY KEY,
    profile_name TEXT NOT NULL,
    pattern TEXT NOT NULL,        -- regex pattern
    priority INTEGER DEFAULT 0,   -- higher = checked first
    FOREIGN KEY (profile_name) REFERENCES profiles(name)
);
```

Example patterns:

| Pattern | Profile |
|---------|---------|
| `\.rs$` | rust |
| `\.py$` | python |
| `\.c$\|\.h$` | c |
| `\.txt$` | text |
| `Makefile$` | makefile |
| `\.auto$` | spflite-auto |
| `.*` | default |

Patterns are evaluated in priority order (highest first). The first match wins.

### Profile Settings

```sql
CREATE TABLE profiles (
    name TEXT PRIMARY KEY,
    colorize_file TEXT,           -- path to .auto file (or NULL for no highlighting)
    record_format TEXT DEFAULT 'V',  -- V=variable, F=fixed
    lrecl INTEGER DEFAULT 0,      -- logical record length (0=unlimited for V)
    tab_width INTEGER DEFAULT 4,
    tabs_to_spaces INTEGER DEFAULT 1,  -- 1=expand tabs to spaces on load
    line_ending TEXT DEFAULT 'AUTO',   -- AUTO, LF, CRLF, CR
    caps_mode TEXT DEFAULT 'OFF',      -- OFF, ON
    nulls_mode TEXT DEFAULT 'OFF',     -- OFF, ON (trailing space handling)
    preserve_mode TEXT DEFAULT 'OFF',  -- OFF, ON (preserve VB trailing spaces)
    autonum_mode TEXT DEFAULT 'OFF',   -- OFF, ON, STD, COBOL
    change_mode TEXT DEFAULT 'CS',     -- CS=column shift, DS=data shift
    bounds_left INTEGER DEFAULT 1,
    bounds_right INTEGER DEFAULT 0,    -- 0=unlimited
    word_delimiters TEXT,              -- custom word boundary characters
    indent_style TEXT DEFAULT 'SPACES', -- SPACES, TABS
    trim_trailing INTEGER DEFAULT 0,   -- 1=trim on save
    backup_mode TEXT DEFAULT 'NONE',   -- NONE, BAK, TIMESTAMP
    backup_path TEXT                   -- custom backup directory
);
```

### Default Profile

The `default` profile is always present and cannot be deleted. It provides fallback settings for files that don't match any pattern.

### Profile Commands

**PROFILE**

```
PROFILE                     (show current profile settings)
PROFILE name                (switch to named profile)
PROFILE SAVE                (save current settings to profile)
PROFILE RESET               (reset to profile defaults)
PROFILE LIST                (list all profiles)
```

**Min Abbreviation:** `PROF`

### Edit File Type (EFT) Table (future)

An extended profile mechanism from SPFLite. EFT defines additional file-type-specific behaviors:
- Auto-indent rules.
- Comment toggle patterns.
- Build/run commands.
- Custom key bindings per file type.

## Key Bindings

### Storage

```sql
CREATE TABLE key_bindings (
    id INTEGER PRIMARY KEY,
    context TEXT DEFAULT 'global',   -- 'global', 'command', 'data', profile name
    key_combo TEXT NOT NULL,          -- e.g., 'Ctrl+S', 'F3', 'Ctrl+Shift+F'
    action TEXT NOT NULL,             -- e.g., 'SAVE', 'FIND', 'END'
    UNIQUE(context, key_combo)
);
```

### Key Combo Format

Key combos are stored as normalized strings:
- Modifiers: `Ctrl`, `Shift`, `Alt`
- Keys: `A`-`Z`, `0`-`9`, `F1`-`F24`, `Enter`, `Tab`, `Escape`, `Insert`, `Delete`, `Home`, `End`, `PageUp`, `PageDown`, `Up`, `Down`, `Left`, `Right`
- Separator: `+`
- Example: `Ctrl+Shift+F`, `Alt+1`, `F3`

### Default Key Bindings

| Key | Action | Context |
|-----|--------|---------|
| F1 | HELP | global |
| F2 | SPLIT | global |
| F3 | END | global |
| F5 | RFIND | global |
| F6 | RCHANGE | global |
| F7 | UP | global |
| F8 | DOWN | global |
| F9 | SWAP | global |
| F10 | LEFT | global |
| F11 | RIGHT | global |
| F12 | RETRIEVE | global |
| Ctrl+S | SAVE | global |
| Ctrl+Q | FORCE_QUIT | global |
| Insert | TOGGLE_MODE | global |
| Escape | RESET | global |

### KEYS Command

```
KEYS                        (show current key bindings)
KEYS SET key_combo action   (set a binding)
KEYS CLEAR key_combo        (remove a binding)
KEYS RESET                  (reset to defaults)
KEYS LIST                   (list all bindings)
```

**Min Abbreviation:** `KEYS`

### Context Resolution

When a key is pressed, bindings are checked in order:
1. Current profile context.
2. Current field context (`command` or `data`).
3. `global` context.

First match wins.

## Global Settings

### Storage

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### Settings

| Key | Default | Description |
|-----|---------|-------------|
| `confirm_cancel` | `true` | Prompt before discarding changes |
| `confirm_delete` | `false` | Prompt before delete operations |
| `scroll_amount` | `PAGE` | Default scroll amount (PAGE, HALF, CSR, DATA, N) |
| `command_retrieve_depth` | `25` | Number of commands to remember |
| `undo_depth` | `100` | Maximum undo frames |
| `colorize_dir` | `~/.config/spf-edit/colorize/` | Path to .auto files |
| `backup_dir` | `~/.config/spf-edit/backups/` | Default backup directory |
| `font_size` | (terminal default) | Future: for GUI mode |
| `theme` | `ispf` | Color theme name |

### SET Command

```
SET key value               (set a global setting)
SET key                     (show current value)
SET                         (show all settings)
```

**Min Abbreviation:** `SET`

## State Persistence

### Per-File State

When a file is closed, its editing state is saved:

```sql
CREATE TABLE state (
    file_path TEXT PRIMARY KEY,
    cursor_line INTEGER,
    cursor_col INTEGER,
    scroll_top INTEGER,
    scroll_left INTEGER,
    profile_name TEXT,
    labels TEXT,              -- JSON: {".lab1": line_id, ...}
    exclude_state TEXT,       -- JSON: serialized exclude ranges
    last_find TEXT,           -- last FIND parameters
    last_change TEXT,         -- last CHANGE parameters
    modified_time TEXT,       -- last save timestamp
    access_time TEXT          -- last open timestamp
);
```

When the same file is opened again, state is restored:
- Cursor returns to its last position.
- Scroll position is restored.
- Labels are restored (if lines haven't changed).
- Exclude state is restored.
- Last FIND/CHANGE parameters are available.

### State Expiry

State entries older than a configurable age (default: 90 days) are automatically pruned on startup.

## Configuration Commands Summary

| Command | Purpose |
|---------|---------|
| `PROFILE` | Manage editing profiles |
| `KEYS` | Manage key bindings |
| `SET` | Manage global settings |
| `SETTINGS` | Open settings dialog (future, alias for SET) |

## First-Run Setup

On first run:
1. The config database is created with default schema.
2. The `default` profile is inserted.
3. Default key bindings are inserted.
4. Default global settings are inserted.
5. Colorize files are copied to the colorize directory.

## Interactions

- **File opening:** Profile matched → settings applied → colorize loaded → state restored.
- **File saving:** State saved to database.
- **SAVE/END:** State updated on exit.
- **Schema migration:** On upgrade, the database schema is migrated with `ALTER TABLE` or `CREATE TABLE IF NOT EXISTS`.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Database not writable | `"Cannot write config database: path"` |
| Invalid regex in pattern | `"Invalid profile pattern: error"` |
| Profile not found | `"Profile 'name' not found"` |
| Key combo parse error | `"Invalid key combination: combo"` |
| Unknown setting | `"Unknown setting: key"` |

## Examples

1. `PROFILE` — show current profile settings.
2. `PROFILE rust` — switch to the "rust" profile.
3. `KEYS SET Ctrl+D DELETE` — bind Ctrl+D to DELETE command.
4. `SET scroll_amount HALF` — default scroll to half-page.
5. `SET confirm_cancel true` — enable cancel confirmation.

## Status

| Aspect | State |
|--------|-------|
| SQLite config database | **Not started** |
| Profiles table | **Not started** |
| Profile pattern matching (regex) | **Not started** |
| Default profile | **Not started** |
| Key bindings table | **Not started** |
| Default key bindings | **Partial** (hardcoded in input.rs) |
| Global settings table | **Not started** |
| State persistence | **Not started** |
| PROFILE command | **Not started** |
| KEYS command | **Not started** |
| SET command | **Not started** |
| First-run setup | **Not started** |
| Schema migration | **Not started** |
| Colorize directory management | **Not started** |
