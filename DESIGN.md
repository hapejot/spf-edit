# SPF-Edit Design Document

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────┐
│                    main.rs                      │
│  CLI args → Terminal setup → Event loop → Exit  │
└──────────────────────┬──────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────┐
│                  editor.rs                      │
│  Editor struct: coordinates all components      │
│  Owns: FileBuffer, Screen, InputHandler         │
│  Orchestrates: input → command → buffer → render│
└───┬──────────┬──────────┬──────────┬────────────┘
    │          │          │          │
    ▼          ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌────────┐ ┌──────────┐
│buffer  │ │screen  │ │command │ │input     │
│  .rs   │ │  .rs   │ │  .rs   │ │  .rs     │
│        │ │        │ │line_cmd│ │          │
│FileB.  │ │Screen  │ │  .rs   │ │InputH.   │
│LineS.  │ │render  │ │prefix  │ │KeyMap    │
│        │ │layout  │ │  .rs   │ │          │
└───┬────┘ └────────┘ └────────┘ └──────────┘
    │
    ▼
┌────────────────┐
│  file_io.rs    │
│  read / write  │
│  text + fixed  │
└────────────────┘
```

### Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `main.rs` | CLI argument parsing, terminal raw mode setup/teardown, panic hook, top-level event loop |
| `editor.rs` | `Editor` struct that owns all state. Receives input events, dispatches to command/line_cmd processors, triggers buffer mutations, requests screen redraws |
| `buffer.rs` | `FileBuffer` struct holding file metadata + `LineStore`. High-level operations: insert, delete, copy, move, renumber. Does NOT know about rendering |
| `line.rs` | `Line` struct and `LineType`/`LineFlags` enums. Pure data, no behavior beyond accessors |
| `line_store.rs` | `LineStore` trait + `VecLineStore` implementation. Abstraction layer over line storage for future swappability |
| `screen.rs` | `Screen` struct computing layout from terminal dimensions. Rendering methods translate buffer state → terminal output via crossterm |
| `prefix.rs` | Prefix area rendering (line number formatting) and raw prefix text → `ParsedLineCmd` parsing |
| `command.rs` | Primary command parsing (string → `PrimaryCommand` enum) and execution |
| `line_cmd.rs` | Line command validation (pairing, conflicts) and execution (calling buffer operations) |
| `input.rs` | `InputHandler` translating crossterm `Event` → `EditorAction` enum. Key mapping. Mode tracking |
| `file_io.rs` | `read_file()` and `write_file()` functions. Text + fixed-record-length format support. Tab expansion |
| `types.rs` | Shared enums, constants, type aliases used across modules |

---

## 2. Core Data Model

### 2.1 Line
Each line in the file is represented as:
- `line_type`: Data, TopOfData, BottomOfData, ColsRuler, Message
- `data`: UTF-8 string content
- `original_number`: line number at load time
- `current_number`: display line number (renumbered)
- `flags`: bitflags for modified, pending_cmd, excluded, inserted, cmd_error
- `prefix_cmd`: raw text typed in prefix area for pending commands

### 2.2 LineStore Trait
Abstraction over line storage with methods: `len()`, `get()`, `get_mut()`, `insert()`, `remove()`, `iter()`, `iter_range()`, `drain()`, `splice()`. MVP uses `VecLineStore` backed by `Vec<Line>`. Future implementations can swap in lazy-loading or rope-based storage.

### 2.3 FileBuffer
Owns LineStore plus file metadata: path, modified flag, record format (Variable/Fixed), line ending (Lf/CrLf/Cr), caps mode, number mode, nulls mode, column bounds, and label map.

---

## 3. Screen Layout

```
Row 0:  Title Line     "EDIT  filename.txt               Columns 00001 00080"
Row 1:  Command Line   "Command ===> ___________         Scroll ===> PAGE"
Row 2+: Data Lines     "000100 #include <stdio.h>"

Prefix width: 6 chars
Separator: 1 space
Data width: terminal_width - 7
Data rows: terminal_height - 2
```

### Color Scheme (Hardcoded ISPF-like)

| Element | Foreground | Background |
|---------|-----------|------------|
| Title line | White | Blue |
| Command prompt | Green | Black |
| Command input | White | Black |
| Prefix (line numbers) | Cyan | Black |
| Prefix (pending cmd) | Yellow | Black |
| Prefix (error) | Red | Black |
| Data area | White | Black |
| Sentinel lines | Blue | Black |
| Column ruler | Blue | Black |
| Found text | Black | Yellow |
| Error messages | Red | Black |

---

## 4. Input Processing

### Field Focus
Cursor can be in: CommandLine, ScrollField, PrefixArea{row}, DataArea{row}

### Processing Order on Enter
1. Collect data changes (overtyped lines → update Line.data)
2. Collect prefix commands (overtyped prefixes → parse as line commands)
3. Validate line commands (pairing, conflicts)
4. Execute line commands (deletes → moves → copies → inserts)
5. Parse primary command from command line
6. Execute primary command
7. Renumber if needed
8. Full screen redraw

---

## 5. Command Parsing

### Primary Commands (with minimum abbreviations)
SAVE(SA), END, CANCEL(CAN), FIND(F), RFIND(RF), TOP(T), BOTTOM(BOT), UP, DOWN(DO), LEFT(LE), RIGHT(RI), LOCATE(L), RESET(RES), COLS, NUMBER(NUM), NULLS(NUL), CAPS

### Line Commands
Single: I(n), D(n), R(n), C, M, A, B
Block: CC, MM, DD, RR(n)
Label: .xxxx

---

## 6. File I/O

### Reading
- Text: detect line endings, expand tabs to spaces (8-col stops), create sentinels
- Fixed: read N-byte chunks, decode UTF-8 lossy

### Writing
- Text: preserve line endings, optionally strip trailing blanks, atomic write
- Fixed: pad/truncate to record length, no line endings, atomic write

---

## 7. Extensibility

- **LineStore trait**: swap storage backend (lazy-loading, rope, remote)
- **Key mapping**: currently hardcoded, designed for future config file
- **Color theme**: constants in types.rs, designed for future override
