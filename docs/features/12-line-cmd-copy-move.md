# 12 — Line Commands: Copy and Move

## Overview

Copy and Move line commands transfer lines from a source location to a destination within the file. They require both a source marker and a destination marker to be entered before pressing Enter. This two-step interaction is a core ISPF paradigm.

## Source Commands

### COPY — C / CC

| Form | Effect |
|------|--------|
| `C` | Mark a single line as copy source |
| `C`*n* | Mark *n* consecutive lines as copy source |
| `CC` ... `CC` | Mark a block of lines as copy source (paired) |

### MOVE — M / MM

| Form | Effect |
|------|--------|
| `M` | Mark a single line as move source |
| `M`*n* | Mark *n* consecutive lines as move source |
| `MM` ... `MM` | Mark a block of lines as move source (paired) |

**Difference:** Copy duplicates lines to the destination. Move removes them from the source and places them at the destination.

## Destination Commands

### AFTER — A

| Form | Effect |
|------|--------|
| `A` | Insert copied/moved lines after this line |
| `A`*n* | Insert *n* copies of the source after this line (repeat factor) |

### BEFORE — B

| Form | Effect |
|------|--------|
| `B` | Insert copied/moved lines before this line |
| `B`*n* | Insert *n* copies of the source before this line |

### AFTER BLOCK — AA (future)

| Form | Effect |
|------|--------|
| `AA` ... `AA` | Interspersed after: insert source after every *n* lines within the AA range |

With a numeric suffix: `AA3` means insert source block after every 3 lines within the destination range.

### BEFORE BLOCK — BB (future)

| Form | Effect |
|------|--------|
| `BB` ... `BB` | Interspersed before: insert source before every *n* lines within the BB range |

### HERE — H / HH (future)

| Form | Effect |
|------|--------|
| `H` | Replace this line with the source (delete H line, insert source) |
| `H`*n* | Replace *n* lines with the source |
| `HH` ... `HH` | Replace the block with the source |

## Execution Process

1. On Enter, all prefix commands are collected.
2. Source commands (C/CC/M/MM) and destination commands (A/B) are identified.
3. Block commands are paired (first CC with second CC, first MM with second MM).
4. Validation: exactly one source and one destination must be present. Multiple sources or destinations is an error.
5. The source range is determined (single line, N lines, or block range).
6. Copy: lines are duplicated at the destination. Move: lines are removed from source and inserted at destination.
7. If the source has a repeat factor and the destination also has a repeat factor, it's an error ("Multiple repeat values entered").

## Keep Suffix — K (future)

Adding `K` to a destination command keeps it pending after execution:

| Form | Effect |
|------|--------|
| `AK` | After — but stays pending for the next copy/move |
| `BK` | Before — stays pending |

This allows repeated copy/move operations to the same destination without re-typing A or B each time.

## Post-Processing Exclude — +/- (future)

Copy and Move support post-processing exclude flags:

| Suffix | Effect |
|--------|--------|
| `+` | After execution, exclude the source lines |
| `-` | After execution, exclude the destination (newly placed) lines |

## Interactions

- **Pending state:** If a source is entered without a destination (or vice versa), the entered command becomes pending (displayed in yellow). It persists until a matching command is entered or RESET CMD is used.
- **Excluded lines (future):** If the destination is an excluded line, the source is inserted after the end of the excluded range.
- **=FILE> lines (future, MEdit):** Move of a =FILE> line can only use A/B destinations and has additional restrictions.
- **Undo (future):** Copy/Move operations push an undo frame.
- **Browse mode:** All copy/move commands are blocked.

## Error Conditions

| Condition | Message |
|-----------|---------|
| Source without destination | Remains pending (yellow) |
| Destination without source | `"No source for After/Before destination"` |
| Multiple sources | `"Multiple source commands entered"` |
| Multiple destinations | `"Multiple destination commands entered"` |
| Source and destination overlap | `"Overlapping line ranges entered"` |
| Both source and dest have repeat | `"Multiple Repeat values entered"` |
| CC unpaired | Remains pending |
| MM unpaired | Remains pending |
| Move source includes destination | Adjusted automatically (destination recalculated after source removal) |

## Examples

1. `C` on line 100 + `A` on line 500 → line 100 copied to after line 500.
2. `CC` on line 100 + `CC` on line 300 + `B` on line 50 → lines 100-300 copied before line 50.
3. `M` on line 200 + `A` on line 600 → line 200 moved to after line 600 (removed from position 200).
4. `MM` on line 100 + `MM` on line 200 + `A3` on line 500 → block 100-200 moved and inserted 3 times after line 500.
5. `CC` on line 100 only → shows `CC` in yellow. User enters `CC` on line 200 and `A` on line 400 on next Enter → block copy executes.

## Status

| Aspect | State |
|--------|-------|
| C / Cn (single copy) | **Implemented** |
| CC block copy | **Implemented** |
| M / Mn (single move) | **Implemented** |
| MM block move | **Implemented** |
| A / An (after) | **Implemented** |
| B / Bn (before) | **Implemented** |
| Pending state (yellow) | **Implemented** |
| Block pairing (CC-CC, MM-MM) | **Implemented** |
| AA / BB interspersed | **Not started** |
| H / HH (here/replace) | **Not started** |
| K suffix (keep) | **Not started** |
| +/- exclude suffix | **Not started** |
