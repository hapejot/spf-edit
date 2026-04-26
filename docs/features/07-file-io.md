# 07 — File I/O

## Overview

File I/O handles reading files from disk into the editor buffer and writing them back. SPF-Edit supports multiple record formats, line endings, character encodings, and tab handling. Files are written atomically to prevent corruption.

## Opening a File

### CLI Invocation

```
spf-edit [options] <filename>
```

| Option | Effect |
|--------|--------|
| `<filename>` | Path to the file to open |
| `--lrecl N` | Open as fixed-record format with record length N |
| `--browse` | Open in read-only browse mode |
| `--trace` | Enable trace logging |

If the file does not exist, an empty buffer is created with the given filename (new file mode).

### Read Process

1. Read the entire file into memory as bytes.
2. Detect line ending format (see below).
3. Split into lines based on detected line ending.
4. Expand tabs in each line (see below).
5. Wrap each line in a `Line` struct with type `Data` and appropriate flags.
6. Prepend a `TopOfData` sentinel line.
7. Append a `BottomOfData` sentinel line.
8. Number all lines sequentially.

### EDIT Command (future)

**Syntax:** `EDIT [filename] [.profile] [FORCE]`
**Min Abbreviation:** `E`

Opens a file for editing in a new tab. If the file is already open, switches to that tab. `FORCE` opens a new session even if already open. `.profile` overrides the auto-detected profile.

### BROWSE Command (future)

**Syntax:** `BROWSE [filename] [.profile]`
**Min Abbreviation:** `B`

Opens a file in read-only browse mode.

### VIEW Command (future)

**Syntax:** `VIEW [filename] [.profile]`
**Min Abbreviation:** `V`

Opens a file in view mode (read-only with some editing capabilities).

## Record Formats

### Variable-Length (RECFM V) — Default

Lines have variable length. Each line is stored as-is. This is the standard text file format.

### Fixed-Length (RECFM F)

All lines are exactly `LRECL` characters wide. On read, lines shorter than LRECL are padded with spaces. On write, lines are truncated or padded to exactly LRECL bytes.

Set via `--lrecl N` CLI flag or `LRECL` command (future).

### Other Formats (future)

| Format | Description |
|--------|-------------|
| RECFM U | Undefined — binary/unstructured |
| RECFM VA | Variable with ASA carriage control |
| RECFM VB | Variable blocked |

## Line Endings

### Auto-Detection

On read, the line ending format is auto-detected by scanning the first occurrence:

| Format | Sequence | Detection |
|--------|----------|-----------|
| CRLF | `\r\n` | Windows default |
| LF | `\n` | Unix/Linux/macOS |
| CR | `\r` | Classic Mac |

If no line ending is found (single-line file), defaults to the platform native (CRLF on Windows, LF on Unix).

### Preservation

The detected line ending is stored and used when writing the file back. The user can override it with the `EOL` command (future).

### EOL Command (future)

**Syntax:** `EOL CRLF|LF|CR|NONE|AUTO`

Changes the line ending format for the current file. `NONE` writes no line endings (binary mode). `AUTO` re-detects.

## Character Encoding

### Current: UTF-8 Only

Files are read as UTF-8. Invalid UTF-8 bytes are replaced with the Unicode replacement character (U+FFFD) via `from_utf8_lossy`.

### Future Encodings

| Encoding | Description |
|----------|-------------|
| ANSI | Windows-1252 / system locale |
| UTF-8 | Current default |
| UTF-16 LE/BE | Windows Unicode |
| EBCDIC | Mainframe character set |

The `SOURCE` command (future) will set the encoding: `SOURCE UTF8|ANSI|UTF16|UTF16BE|EBCDIC`.

### BOM Handling (future)

UTF-8 BOM (0xEF 0xBB 0xBF) and UTF-16 BOM are detected on read and optionally preserved on write. Controlled by `BOM ON|OFF` command.

## Tab Handling

### Tab Expansion on Read

Tab characters (`\t`) are expanded to spaces on read. The tab stop interval is 8 columns (configurable via `XTABS` command in future).

Algorithm: for each tab character, insert spaces to the next multiple of the tab stop width.

### Tab Insertion on Write (future)

Optional: convert runs of spaces back to tabs on write if the profile specifies tab mode.

### XTABS Command (future)

**Syntax:** `XTABS [n]`

Sets the tab expansion width. `XTABS 4` treats tabs as 4-column stops.

## Writing a File

### Atomic Write

1. Write to a temporary file (`.spf-edit.tmp` in the same directory).
2. On success, rename the temporary file to the target filename.
3. This ensures that a crash during write does not corrupt the original file.

### Variable-Length Write

- Lines are written with the detected (or configured) line ending.
- If NULLS is ON, trailing spaces are stripped before writing.
- If NULLS is OFF, trailing spaces are preserved.
- Sentinel lines (TopOfData, BottomOfData) are not written.
- Special lines (COLS ruler, messages) are not written.

### Fixed-Length Write

- Each line is padded or truncated to exactly LRECL characters.
- Lines are written without line endings (pure fixed-record).
- **Known bug:** Byte-level truncation can split multi-byte UTF-8 characters.

## DCB Command (future)

**Syntax:** `DCB RECFM value LRECL n EOL value`

Combined command to set record format, logical record length, and line ending in a single operation. Validates that the combination is logical (e.g., RECFM F requires LRECL > 0).

## Interactions

- **Profiles (future):** The file profile determines default record format, encoding, line ending, and tab handling.
- **Browse mode:** File is read but cannot be written (SAVE blocked).
- **PRESERVE (future):** Controls trailing blank behavior on write (see [10-nulls-caps-preserve.md](10-nulls-caps-preserve.md)).
- **BACKUP (future):** Before overwriting, an automatic backup may be created.

## Error Conditions

| Condition | Message |
|-----------|---------|
| File not found (read) | Creates empty buffer (new file) |
| Permission denied (write) | `"Failed to save: permission denied"` |
| Disk full (write) | `"Failed to save: ..."` (OS error) |
| Atomic rename failure | Falls back to direct write |

## Examples

1. `spf-edit myfile.txt` — opens UTF-8 text file with auto-detected line endings.
2. `spf-edit --lrecl 80 mainframe.dat` — opens as fixed 80-byte records.
3. `spf-edit --browse readme.md` — opens read-only.

## Status

| Aspect | State |
|--------|-------|
| Variable-length text read/write | **Implemented** |
| Fixed-length read/write | **Implemented** (UTF-8 truncation bug) |
| Line ending auto-detection | **Implemented** (LF, CRLF, CR) |
| Line ending preservation | **Implemented** |
| Tab expansion on read | **Implemented** (TAB_STOP=8) |
| Atomic write | **Implemented** |
| UTF-8 encoding | **Implemented** |
| NULLS-based trailing space stripping | **Implemented** |
| EDIT/BROWSE/VIEW commands | **Not started** |
| EOL command | **Not started** |
| SOURCE/encoding command | **Not started** |
| EBCDIC support | **Not started** |
| BOM handling | **Not started** |
| XTABS command | **Not started** |
| DCB command | **Not started** |
| RECFM U/VA/VB | **Not started** |
