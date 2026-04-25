# SPF-Edit Feature List

## Overview
SPF-Edit is a terminal-based text editor implementing the SPF/ISPF editing paradigm in Rust using crossterm. It targets programmers familiar with mainframe SPF editors (SPFPC, ISPF, SPF/SE, xe) and provides the same line-oriented, command-driven editing model in a modern cross-platform terminal.

---

## MVP Features (v0.1)

### F-001: File Opening and Display
- Open a file specified on the command line
- Display file content with SPF-style screen layout:
  - Title line showing mode (EDIT), filename, and visible column range
  - Command line with `Command ===>` prompt and `Scroll ===>` indicator
  - Data lines with 6-character prefix area (line numbers) and data area
  - Top-of-Data and Bottom-of-Data sentinel lines
- Support terminal resize with dynamic layout recomputation

### F-002: Line Number Display
- Display line numbers in the prefix area (6 chars, zero-padded, e.g., `000100`)
- STD numbering mode: increment by 100
- Support NUMBER ON/OFF command to toggle display
- Unnumbered mode displays `======` in prefix area

### F-003: Cursor Navigation
- Arrow key movement within and between fields
- Three field types: command line, prefix area, data area
- Tab key cycles between fields
- Home/End within current field
- Cursor wrapping at line boundaries

### F-004: Overtype Editing
- Default overtype mode: typed characters replace character at cursor position
- Insert mode toggle via Insert key
- In insert mode: characters shift existing text right
- Respect CAPS mode (auto-uppercase when enabled)
- Respect column bounds for editing operations
- Track per-line modification state

### F-005: Command Line Input
- Text editing on the command line row
- Command history with retrieval (F12 or up-arrow)
- Command processed on Enter key press

### F-006: Line Commands — Basic
- **I / In**: Insert 1 or n blank lines after target line
- **D / Dn**: Delete 1 or n lines starting at target
- **R / Rn**: Repeat (duplicate) target line 1 or n times
- **C**: Mark single line as copy source (requires A or B destination)
- **M**: Mark single line as move source (requires A or B destination)
- **A**: Destination — place copied/moved lines after this line
- **B**: Destination — place copied/moved lines before this line

### F-007: Line Commands — Block Operations
- **CC/CC**: Mark block for copy (requires A or B destination)
- **MM/MM**: Mark block for move (requires A or B destination)
- **DD/DD**: Delete all lines in block (no destination needed)
- **RR/RRn**: Repeat entire block 1 or n times

### F-008: Line Commands — Pending State
- Incomplete commands (e.g., CC without second CC, or C without A/B) remain pending
- Pending commands highlighted in yellow in the prefix area
- Invalid/conflicting commands highlighted in red with error message
- RESET command clears all pending line commands

### F-009: Line Labels
- `.xxxx` in prefix area sets a named label on a line (e.g., `.A`, `.HERE`)
- Labels usable as targets in primary commands (LOCATE .label)
- System labels: `.ZFIRST`, `.ZLAST`, `.ZCSR`

### F-010: Primary Command — FIND
- `FIND 'string'` or `FIND string` (no spaces)
- Direction modifiers: NEXT (default), PREV, FIRST, LAST, ALL
- On match: position cursor on found text, display "CHARS 'string' FOUND"
- On no match: display "CHARS 'string' NOT FOUND"
- Store last search for RFIND (F5)
- ALL: highlight all occurrences, display count

### F-011: Primary Commands — Navigation
- **TOP**: Scroll to top of data
- **BOTTOM / BOT**: Scroll to bottom of data
- **UP n**: Scroll up by n lines or PAGE/HALF/CSR/DATA/MAX
- **DOWN n**: Scroll down by n lines or PAGE/HALF/CSR/DATA/MAX
- **LEFT n**: Scroll left by n columns
- **RIGHT n**: Scroll right by n columns
- **LOCATE n**: Scroll to line number n
- **LOCATE .label**: Scroll to labeled line

### F-012: Primary Commands — File Operations
- **SAVE**: Write buffer to file, clear modified flag
- **END**: Save if modified, then exit
- **CANCEL / CAN**: Exit without saving; prompt if data changed

### F-013: Primary Commands — Display Control
- **RESET**: Clear pending line commands, labels
- **COLS**: Toggle column ruler display
- **NUMBER ON/OFF**: Toggle line number display
- **NULLS ON/OFF**: Toggle trailing blank handling
- **CAPS ON/OFF**: Toggle auto-uppercase mode

### F-014: Scrolling
- F7 = scroll up, F8 = scroll down
- F10 = scroll left, F11 = scroll right
- Scroll amount controlled by `Scroll ===>` field: PAGE, HALF, CSR, DATA, MAX, or numeric
- Horizontal scrolling shifts data area view; prefix area stays fixed

### F-015: Function Key Mapping
- F3 = END (save and exit)
- F5 = RFIND (repeat last find)
- F7 = UP (scroll up)
- F8 = DOWN (scroll down)
- F10 = LEFT (scroll left)
- F11 = RIGHT (scroll right)
- F12 = RETRIEVE (command history)

### F-016: File Format Support — Text Files
- Read text files with LF, CRLF, or CR line endings
- Detect line ending style on read, preserve on write
- Expand tab characters to spaces on read (8-column tab stops)
- Write as spaces (no tab reinsertion)

### F-017: File Format Support — Fixed-Record-Length
- `--lrecl N` command-line argument to open as fixed-record file
- Read in N-byte chunks (no line ending characters expected)
- Display lines padded to N characters
- Write lines padded/truncated to exactly N bytes, no line endings

### F-018: Browse Mode
- `--browse` command-line argument for read-only viewing
- Disallow data editing, line commands that modify data, SAVE
- Allow navigation, FIND, COLS, scrolling

### F-019: Color Scheme
- Hardcoded ISPF-like colors:
  - Title line: white on blue
  - Command line: green on black
  - Prefix area (line numbers): turquoise/cyan on black
  - Data area: white on black
  - Sentinel lines (Top/Bottom of Data): blue on black
  - Pending line commands: yellow on black
  - Error indicators: red on black
  - Found text: reverse video

### F-020: Terminal Safety
- Enter raw mode and alternate screen on startup
- Restore terminal state on normal exit
- Panic hook to restore terminal state on crash
- Handle SIGWINCH / resize events

---

## Deferred Features (Post-MVP)

### D-001: Exclude/Show System
### D-002: CHANGE Command
### D-003: Undo/Redo
### D-004: Profiles
### D-005: Syntax Highlighting
### D-006: Split Screen
### D-007: Hex Edit Mode
### D-008: Column Shifting
### D-009: Text Flow and Split
### D-010: Case Conversion Line Commands
### D-011: Sort Command
### D-012: Overlay Operations
### D-013: COPY/MOVE from External File
### D-014: MASK and TABS Lines
### D-015: Configuration File
### D-016: Lazy Loading
