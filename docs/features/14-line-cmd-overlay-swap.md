# 14 — Line Commands: Overlay and Swap

## Overview

Overlay and Swap are destination commands used with Copy (C/CC) or Move (M/MM). Instead of inserting source lines at a new position, they merge source content into existing destination lines by overlaying characters or by swapping two ranges.

## OVERLAY — O / OO

### Syntax (Destination Commands)

| Form | Effect |
|------|--------|
| `O` | Overlay the source onto this single destination line |
| `O`*n* | Overlay onto *n* destination lines |
| `OO` ... `OO` | Overlay onto the marked destination block |

### Behavior

Overlay merges source text into destination text using **blank-fill** logic:

- For each character position, if the **source** character is non-blank, it replaces the destination character.
- If the source character is blank (space), the destination character is preserved.
- This allows "stamping" a pattern onto existing lines without erasing existing content.

If the source has fewer lines than the destination, the source cycles (wraps around). If the source has more lines, extra source lines are ignored.

### Example

```
Source:    "  XX  YY  "
Dest:      "AABBCCDDEE"
Result:    "AAXXCCYYEE"
```

Blanks in the source leave the destination intact. Non-blanks replace.

## OVERLAY REPLACE — OR / ORR

### Syntax (Destination Commands)

| Form | Effect |
|------|--------|
| `OR` | Overlay-replace onto a single destination line |
| `OR`*n* | Overlay-replace onto *n* lines |
| `ORR` ... `ORR` | Overlay-replace onto the marked block |

### Behavior

Unlike standard overlay, **all** source characters (including blanks) unconditionally replace the destination characters at the corresponding positions. This is a positional overwrite.

### Example

```
Source:    "  XX  YY  "
Dest:      "AABBCCDDEE"
Result:    "  XX  YY  "
```

Every character from the source replaces the destination, including spaces.

## SWAP — W / WW

### Syntax (Destination Commands, used with M/MM only)

| Form | Effect |
|------|--------|
| `W` | Swap this single line with the move source |
| `W`*n* | Swap *n* lines with the move source |
| `WW` ... `WW` | Swap the marked block with the move source block |

### Behavior

Swap exchanges two line ranges:

1. Source block (marked with M/MM) and destination block (marked with W/WW) trade positions.
2. The content that was at the source location moves to the destination location, and vice versa.
3. Both blocks must have the same number of lines, or the shorter block is padded conceptually.

### Restrictions

- Swap is only valid with Move (M/MM), not Copy (C/CC).
- Repeat values are not allowed with Swap: `"MM / WW cannot use repeat value"`.

### Implementation

Internally, Swap is typically implemented as:
1. Copy source block to a temp area.
2. Move destination block to the source location.
3. Move temp area to the destination location.

## Keep Suffix — K (future)

All overlay and swap destinations support the `K` suffix to keep them pending:

| Form | Effect |
|------|--------|
| `OK` | Overlay, keep pending |
| `ORK` | Overlay-replace, keep pending |

## Move with Overlay

When the source command is `M`/`MM` (move) and the destination is `O`/`OO`:

- The source lines are overlaid onto the destination.
- The source lines are only deleted if **all** source data was successfully "swallowed" by the overlay.
- If some source content extends beyond the destination, the source is not deleted, and an error is reported: `"MOVE data not deleted — all data not overlaid"`.

## Interactions

- **BOUNDS (future):** Overlay respects BOUNDS — only characters within the left/right bounds are overlaid.
- **Undo (future):** Overlay/Swap operations push an undo frame.
- **Browse mode:** All overlay and swap commands are blocked.

## Error Conditions

| Condition | Message |
|-----------|---------|
| O/OO without source (C/CC or M/MM) | `"No source for overlay destination"` |
| W/WW with Copy (not Move) | `"Swap requires Move source, not Copy"` |
| MM / WW with repeat value | `"MM / WW cannot use repeat value"` |
| Move overlay incomplete | `"MOVE data not deleted — all data not overlaid"` |

## Examples

1. `C` on line 100 + `O` on line 200 → non-blank characters from line 100 fill blanks in line 200.
2. `CC` on 100, `CC` on 105, `OO` on 200, `OO` on 205 → block overlay, source cycles if needed.
3. `M` on line 100 + `OR` on line 200 → line 100 content unconditionally replaces line 200 content; line 100 deleted.
4. `MM` on 100, `MM` on 110, `WW` on 200, `WW` on 210 → the two blocks swap positions.

## Status

| Aspect | State |
|--------|-------|
| O / OO (overlay) | **Not started** |
| OR / ORR (overlay replace) | **Not started** |
| W / WW (swap) | **Not started** |
| K suffix on overlay/swap | **Not started** |
| Move-with-overlay logic | **Not started** |
