# Appendix B. Edit-related sample macros

Source file: f54em00_v3r1.md
Start page: 463
Page span: 463-464

## Page 463

Appendix B. Edit-related sample macros
The edit macros listed here are included in the ISPF samples library.
These sample macros are explained in Part 2, “Edit macros,” on page 77. They demonstrate various
techniques you can use when writing, running, and testing macros.
ISRBLOCK
Source code for the Block Letter Model selection panel.
ISRBOX
Edit macro that draws a box with its upper left corner at the cursor position.
ISRCHGS
Sample edit macro that shows the lines most recently changed and excludes all other lines.
ISRCOUNT
Edit macro that finds occurrences of a string and returns a count of the number found. Demonstrates
passing parameters, and retrieving and returning information.
ISRDASH
Edit macro that deletes all lines that begin with a dash except the first one.
ISRFLAG
ISPF/PDF edit macro to add change flags to a new file based on the differences between the new file
and an ancestor of that file.
ISRIMBED
Sample edit macro that builds a list of imbed (.im) statements found in the member that is entered as
an operand.
ISRMASK
Sample edit macro that overlays lines with data from a mask line, for example to place a comment
area over existing lines.
ISRMBRS
Processes all members of partitioned data set, running a second, user-specified, ISPF edit macro
against each member.
ISRONLY
An ISPF Edit macro written in REXX that combines the ISPF Edit commands EXCLUDE and FIND such
that only the lines containing the search string are displayed.
ISRSEPC
Version of the macro ISRSLREX written in COBOL. Demonstrates calling edit functions from a COBOL
program.
ISRSEPP
Version of the macro ISRSLREX written in PL/I. Demonstrates calling edit functions from a PL/I
program.
ISRSLREX
REXX version of an edit macro that separates each line of data with a line of dashes.
ISRTDATA
Edit macro that demonstrates using a loop structure and conditional logic to generate test data.
ISRTDWRI
A version of the sample edit macro ISRTDATA that demonstrates using CLIST WRITE statements as a
debugging aid.
ISRTRYIT
Processes another edit macro command and displays the return code. Useful for experimenting with
command or assignment statements without actually writing a complete macro.
Edit-related sample macros
© Copyright IBM Corp. 1984, 2024 431

## Page 464

Edit-related sample macros
432  z/OS: z/OS ISPF Edit and Edit Macros
