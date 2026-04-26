# 32 — Syntax Highlighting (Colorize)

## Overview

SPF-Edit uses a declarative syntax highlighting system based on SPFLite's `.auto` colorize file format. Each language profile references a `.auto` file that defines keywords, comment styles, string delimiters, and color assignments. The system maps source code tokens to 10 color slots.

## Colorize File Format (.auto)

### File Structure

Each `.auto` file is a plain-text declarative file with sections:

```
[Options]
CaseSensitive=Yes
...

[Comments]
Line=//
BlockStart=/*
BlockEnd=*/
...

[Strings]
...

[Color1]
keyword1
keyword2
...

[Color2]
...
```

### Sections

#### [Options]

| Key | Values | Description |
|-----|--------|-------------|
| `CaseSensitive` | Yes/No | Whether keyword matching is case-sensitive |
| `FirstColumnOnly` | Yes/No | Whether keywords must start at column 1 |

#### [Comments]

Defines comment delimiters:

| Key | Description |
|-----|-------------|
| `Line` | Single-line comment start (e.g., `//`, `#`, `--`) |
| `Line2` | Alternate single-line comment (e.g., `;`) |
| `BlockStart` | Block comment open (e.g., `/*`) |
| `BlockEnd` | Block comment close (e.g., `*/`) |
| `NestedBlock` | Yes/No — whether block comments can nest |
| `FirstColumnLine` | Comment that must start at column 1 (e.g., `*` in COBOL) |

Multiple line-comment and block-comment styles can be defined.

#### [Strings]

Defines string literal delimiters:

| Key | Description |
|-----|-------------|
| `Single` | Single-quote string (`'...'`) |
| `Double` | Double-quote string (`"..."`) |
| `Backtick` | Backtick string (`` `...` ``) |
| `EscapeChar` | Escape character within strings (e.g., `\`) |
| `DoubleUp` | Yes/No — whether doubling the delimiter escapes it (`''`) |

#### [Color1] through [Color10]

Each section lists keywords that should be displayed in that color slot. One keyword per line.

```
[Color1]
if
else
while
for
return
```

### Available .auto Files

SPFLite ships with 27+ language colorize files:

| File | Language |
|------|----------|
| `370asm.auto` | IBM 370 Assembler |
| `C#.AUTO` | C# |
| `cobol.auto` | COBOL |
| `delphi.auto` | Delphi/Pascal |
| `DOSbat.AUTO` | DOS Batch |
| `fortran.auto` | Fortran |
| `html.auto` | HTML |
| `java.auto` | Java |
| `JCL.auto` | IBM JCL |
| `macro.auto` | Macro |
| `pbcc.auto` | PowerBasic Console |
| `pbdos.auto` | PowerBasic DOS |
| `pcasm.auto` | PC Assembler |
| `php.auto` | PHP |
| `pl1.auto` | PL/I |
| `powerbasic.auto` | PowerBasic |
| `Python.auto` | Python |
| `rexx.auto` | REXX |
| `sql.auto` | SQL |
| `vb.auto` | Visual Basic |
| `vbnet.auto` | VB.NET |
| `MODEL.AUTO` | Template for creating new .auto files |

### Adding New Languages

1. Copy `MODEL.AUTO` to `NewLanguage.auto`.
2. Define comment styles in `[Comments]`.
3. Define string styles in `[Strings]`.
4. Fill `[Color1]` through `[Color10]` with keywords.
5. Associate the `.auto` file with a profile (see [35-profiles-config.md](35-profiles-config.md)).

## Color Slots

The 10 color slots map to semantic categories. The specific colors are configurable per profile:

| Slot | Typical Usage | Default Color |
|------|---------------|---------------|
| Color 1 | Language keywords (if, else, while) | Bright Cyan |
| Color 2 | Type keywords (int, string, bool) | Bright Green |
| Color 3 | Built-in functions | Yellow |
| Color 4 | Operators / special | Bright Magenta |
| Color 5 | Preprocessor directives | Bright Red |
| Color 6 | User-defined category 1 | Bright White |
| Color 7 | User-defined category 2 | Bright Blue |
| Color 8 | User-defined category 3 | Cyan |
| Color 9 | User-defined category 4 | Green |
| Color 10 | User-defined category 5 | Magenta |

Additionally, fixed semantic colors:

| Element | Color |
|---------|-------|
| Comments | Green (configurable) |
| Strings | Yellow (configurable) |
| Normal text | White/Light Gray |
| Numbers | Cyan (optional) |

## Tokenization Process

1. **Input:** A line of text and the current colorize state (in case of multi-line comments/strings).
2. **Comment check:** If inside a block comment, scan for the block-end delimiter.
3. **String check:** If inside a multi-line string, scan for the closing delimiter.
4. **Line comment check:** If the line starts with a line-comment delimiter, the entire line is a comment.
5. **Token scan:** Left to right:
   - Match block-comment start → switch to comment mode.
   - Match string delimiter → switch to string mode.
   - Match a keyword from any `[ColorN]` section → apply that color slot.
   - Anything else → normal text color.
6. **Output:** An array of `(column_range, color)` tuples for the line.

### Keyword Matching

- Keywords are matched as whole words (bounded by non-alphanumeric characters).
- If `CaseSensitive=No`, matching is case-insensitive.
- If `FirstColumnOnly=Yes`, keywords must start at column 1.
- Longest match wins (e.g., `foreach` matches before `for`).

### Multi-Line State

Block comments and multi-line strings require tracking state across lines. Each line stores its "entry colorize state" so that partial re-colorization is possible.

## CQT Integration

The tokenizer produces metadata about which regions are Comments (C), Quoted strings (Q), and normal Text (T). This metadata is used by:
- FIND/CHANGE with CQT filtering (see [19-find.md](19-find.md)).
- Line-specific operations that need to distinguish code from comments/strings.

## Recolorize Triggers

Syntax highlighting is recalculated when:
- The file is first loaded.
- A line is edited.
- Lines are inserted, deleted, moved, or copied.
- Case commands change text.
- The profile's colorize file is changed.

For efficiency, only the affected lines and subsequent lines (until the colorize state stabilizes) are re-tokenized.

## Interactions

- **Profiles (future):** Each profile specifies which `.auto` file to use.
- **HILITE command (future):** `HILITE ON|OFF` toggles syntax highlighting.
- **Hex mode (future):** Syntax highlighting is disabled in hex mode.
- **Performance:** Large files (10K+ lines) may need incremental or lazy colorization.

## Error Conditions

| Condition | Message |
|-----------|---------|
| .auto file not found | `"Colorize file not found: name.auto"` |
| .auto parse error | `"Error in colorize file at line N: description"` |
| Unknown section in .auto | Silently ignored |

## Examples

1. Load a `.py` file → Python profile applies → `Python.auto` colorize file loaded → keywords like `def`, `class`, `import` highlighted.
2. Edit `.auto` file to add new keyword → save → highlighting updates.
3. `HILITE OFF` → syntax highlighting disabled.

## Status

| Aspect | State |
|--------|-------|
| .auto file parser | **Not started** |
| Tokenizer engine | **Not started** |
| Color slot mapping | **Not started** |
| Comment detection | **Not started** |
| String detection | **Not started** |
| Keyword matching | **Not started** |
| Multi-line state tracking | **Not started** |
| CQT metadata | **Not started** |
| Incremental re-colorization | **Not started** |
| HILITE ON/OFF command | **Not started** |
| Hardcoded color scheme | **Implemented** (fixed ISPF-style colors) |
