# 36 - ISPF Command Reference Map

## Overview

This document maps SPF-Edit command behavior to canonical IBM z/OS ISPF source material from the extracted manuals under docs/zos-docs.

Use this map when implementing or validating command semantics so each behavior can be traced to:

- SPF-Edit feature specs in docs/features
- IBM User-facing guidance (User's Guides)
- IBM command-precise reference (Edit and Edit Macros, Reference Summary, Messages and Codes)

## Canonical IBM Sources

| Manual | Scope | Local chapter index |
|---|---|---|
| User's Guide Volume I (f54ug00) | Core ISPF interaction model, command usage, abbreviations | [docs/zos-docs/chapters/f54ug00_v3r1/CHAPTER_INDEX.md](../zos-docs/chapters/f54ug00_v3r1/CHAPTER_INDEX.md) |
| User's Guide Volume II (f54u200) | Option-level workflows, utilities, Edit entry points | [docs/zos-docs/chapters/f54u200_v3r1/CHAPTER_INDEX.md](../zos-docs/chapters/f54u200_v3r1/CHAPTER_INDEX.md) |
| Edit and Edit Macros (f54em00) | Edit line commands, Edit primary commands, macros | [docs/zos-docs/chapters/f54em00_v3r1/CHAPTER_INDEX.md](../zos-docs/chapters/f54em00_v3r1/CHAPTER_INDEX.md) |
| Reference Summary (f54rs00) | Compact syntax and return-code oriented quick reference | [docs/zos-docs/chapters/f54rs00_v3r1/CHAPTER_INDEX.md](../zos-docs/chapters/f54rs00_v3r1/CHAPTER_INDEX.md) |
| Messages and Codes (f54mc00) | Message IDs and diagnostics for command errors | [docs/zos-docs/chapters/f54mc00_v3r1/CHAPTER_INDEX.md](../zos-docs/chapters/f54mc00_v3r1/CHAPTER_INDEX.md) |

## Primary Command Map

| Command family | SPF-Edit spec | IBM user guidance | IBM detailed reference |
|---|---|---|---|
| Command entry and abbreviations | [docs/features/05-command-line.md](05-command-line.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md), [docs/zos-docs/chapters/f54ug00_v3r1/10_appendix-c-abbreviations-for-commands-and-other.md](../zos-docs/chapters/f54ug00_v3r1/10_appendix-c-abbreviations-for-commands-and-other.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md), [docs/zos-docs/chapters/f54em00_v3r1/13_appendix-a-abbreviations-for-commands-and-other-values.md](../zos-docs/chapters/f54em00_v3r1/13_appendix-a-abbreviations-for-commands-and-other-values.md) |
| FIND and RFIND | [docs/features/19-find.md](19-find.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md), [docs/zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md](../zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md) |
| LOCATE and navigation targets | [docs/features/23-locate.md](23-locate.md), [docs/features/24-labels-tags-handles.md](24-labels-tags-handles.md), [docs/features/03-scrolling.md](03-scrolling.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md) |
| RESET scopes | [docs/features/21-exclude-show-system.md](21-exclude-show-system.md), [docs/features/23-locate.md](23-locate.md), [docs/features/24-labels-tags-handles.md](24-labels-tags-handles.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md), [docs/zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md](../zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md) |
| TOP/BOTTOM/UP/DOWN/LEFT/RIGHT | [docs/features/03-scrolling.md](03-scrolling.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md), [docs/zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md](../zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md) |
| SAVE/END/CANCEL | [docs/features/08-save-end-cancel.md](08-save-end-cancel.md) | [docs/zos-docs/chapters/f54ug00_v3r1/02_chapter-1-overview-of-ispf.md](../zos-docs/chapters/f54ug00_v3r1/02_chapter-1-overview-of-ispf.md), [docs/zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md](../zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md) | [docs/zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md](../zos-docs/chapters/f54em00_v3r1/11_chapter-10-edit-primary-commands.md) |
| NUMBER, COLS, NULLS, CAPS | [docs/features/06-line-numbers.md](06-line-numbers.md), [docs/features/01-screen-layout.md](01-screen-layout.md), [docs/features/10-nulls-caps-preserve.md](10-nulls-caps-preserve.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md), [docs/zos-docs/chapters/f54u200_v3r1/03_chapter-2-settings-option-0.md](../zos-docs/chapters/f54u200_v3r1/03_chapter-2-settings-option-0.md) | [docs/zos-docs/chapters/f54em00_v3r1/03_chapter-2-controlling-the-edit-environment.md](../zos-docs/chapters/f54em00_v3r1/03_chapter-2-controlling-the-edit-environment.md), [docs/zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md](../zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md) |

## Line Command Map

| Line command family | SPF-Edit spec | IBM user guidance | IBM detailed reference |
|---|---|---|---|
| I and D (single and counted) | [docs/features/11-line-cmd-insert-delete.md](11-line-cmd-insert-delete.md) | [docs/zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md](../zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md) | [docs/zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md](../zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md) |
| C, M, A, B, CC, MM | [docs/features/12-line-cmd-copy-move.md](12-line-cmd-copy-move.md) | [docs/zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md](../zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md) | [docs/zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md](../zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md), [docs/zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md](../zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md) |
| R and RR | [docs/features/13-line-cmd-replicate.md](13-line-cmd-replicate.md) | [docs/zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md](../zos-docs/chapters/f54u200_v3r1/05_chapter-4-edit-option-2.md) | [docs/zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md](../zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md) |
| Labels (.name) and label-driven navigation | [docs/features/24-labels-tags-handles.md](24-labels-tags-handles.md), [docs/features/23-locate.md](23-locate.md) | [docs/zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md](../zos-docs/chapters/f54ug00_v3r1/04_chapter-3-using-commands-function-keys-and-cursor-selection.md) | [docs/zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md](../zos-docs/chapters/f54em00_v3r1/10_chapter-9-edit-line-commands.md), [docs/zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md](../zos-docs/chapters/f54rs00_v3r1/02_chapter-1-ispf-general-information.md) |

## Error and Diagnostic Cross-References

Use this set when implementation behavior differs from expectations and a message-level decision is needed.

| Need | Primary source |
|---|---|
| ISPF command error IDs (ISP/ISR), wording, operator response | [docs/zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md](../zos-docs/chapters/f54mc00_v3r1/02_chapter-1-ispf-messages-starting-with-isp.md), [docs/zos-docs/chapters/f54mc00_v3r1/03_chapter-2-ispf-messages-starting-with-isr.md](../zos-docs/chapters/f54mc00_v3r1/03_chapter-2-ispf-messages-starting-with-isr.md) |
| Dialog return codes and diagnostics | [docs/zos-docs/chapters/f54mc00_v3r1/08_chapter-6-return-codes-from-terminating-dialogs.md](../zos-docs/chapters/f54mc00_v3r1/08_chapter-6-return-codes-from-terminating-dialogs.md), [docs/zos-docs/chapters/f54mc00_v3r1/09_chapter-7-diagnostic-tools-and-information.md](../zos-docs/chapters/f54mc00_v3r1/09_chapter-7-diagnostic-tools-and-information.md) |

## Implementation Workflow

1. Start from the SPF-Edit feature file for intended behavior.
2. Validate user-facing semantics in f54ug00 or f54u200.
3. Resolve exact command syntax and edge rules in f54em00 and f54rs00.
4. Normalize error text and recovery behavior against f54mc00.

## Scope Note

This map is intentionally command-centric. It does not replace deep dialog-service coverage from f54dg00 or DTL coverage from f54dt00, which are better used when implementing panel/dialog infrastructure rather than editor command semantics.
