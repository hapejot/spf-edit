# Chapter 1. ISPF general information

Source file: f54rs00_v3r1.md
Start page: 27
Page span: 27-54

## Page 27

Chapter 1. ISPF general information
© Copyright IBM Corp. 1989, 2024 1

## Page 28

Invoking an ISPF application—the ISPSTART command
ISPSTART
PANEL( panel_name)
OPT(ZSTART)
OPT( panopt
BASIC
cmd_stack_var_name
) ADDPOP
CMD( commandparm1parm2)
LANG( APL
CREX
)
PGM( program_name)
PARM( parameters)
ZSTART
option
BASIC
cmd_stack_var_name
CODEPAGE( codepage) CHARSET( character_set)
NEWAPPL
( application_id)
SHRPROF
EXCLPROF
SCRNAME( screen_name) TEST
TESTX
TRACE
TRACEX
NOLOGO
LOGO( logo_panel_name)
BATSCRW( screen_width) BATSCRD( screen_depth)
BDISPMAX( max_number_of_displays)
BREDIMAX( max_number_of_redisplays) BDBCS DANISH
ENGLISH
GERMAN
JAPANESE
PORTUGUE
SPANISH
KOREAN
FRENCH
ITALIAN
CHINESET
CHINESES
SGERMAN
UPPERENG
NESTMACS
ISPF system information
2  z/OS: z/OS ISPF Reference Summary

## Page 29

Files used by ISPF
Note: Files used by a given invocation of ISPF must be allocated before ISPF is invoked.
DDNAME(lib-type) Description
ISPFILE File tailoring output
ISPILIB Image library
ISPMLIB Message
ISPPLIB Panel
ISPPROF User profile
ISPSLIB Skeleton
ISPTABLE Table output
ISPTLIB Table input
SYSPROC REXX/CLIST library
SYSEXEC REXX library
Note:
The image library with the associated ddname ISPILIB is no longer used by ISPF.
ISPF system commands
ACTIONS
BACKWARD
BOTTOM
CANCEL
CMDE
COLOR
CRETRIEV
CUAATTR
CURSOR
DOWN
DDLIST
DSLIST
list_name
DSname_ level
DTEST parameter_number
END
Chapter 1. ISPF general information  3

## Page 30

ENVIRON
ENBLDUMP ON
OFF
TERMTRAC ON
ERROR
DUMP
OFF
TERMSTAT
QUERY
EPDF datasetname
BROWSE VIEW MACRO macroname
PROFILE profilename PANEL panelname
FORMAT formatname RECOVER MIXED YES
NO
EXHELP
EXIT
FKA
ON
SHORT
OFF
FORWARD
HELP
4  z/OS: z/OS ISPF Reference Summary

## Page 31

ISPDPTRC
END VIEW QUIET
DSP
DISPLAY
( NONE
IN
OUT
BOTH
) LIST
PNL
PANEL
( *
panel_name
panel_mask
)
READ( NONE
SUMMARY
DETAIL
)
SCR
SCREEN
( 0
*
screen_id
)
SECT
SECTION
( *
ALL
NONE
INIT REINIT PROC
NOINIT NOREINIT NOPROC
)
SVC
SERVICE
( NONE
DETAIL
)
ISPDTLC
Chapter 1. ISPF general information  5

## Page 32

ISPFTTRC
END VIEW QUIET LIST
READ( NONE
SUMMARY
DETAIL
)
REC
RECORDS
( *
ALL
NONE
SRC
SOURCE
DATA CNTL
NOSRC
NOSOURCE
NODATA NOCNTL
)
SCR
SCREEN
( 0
*
screen_id
) SVC
SERVICE
( NONE
DETAIL
)
SKL
SKEL
SKELETON
( *
skel_name
skel_mask
)
TBV
TBVARS
( NONE
DETAIL
)
6  z/OS: z/OS ISPF Reference Summary

## Page 33

ISPFVAR
LMSG ( ON
OFF
)
JUMP ( ON
OFF
)
ABTAB ( ON
OFF
)
PSTAB ( ON
OFF
)
SESM ( ON
OFF
)
EDPRT ( ON
OFF
)
EURO ( ON
OFF
)
SPLTLINE ( ON
OFF
)
SCRML ( ON
OFF
)
ISPFWORK
ISPLIBD
libtype
ISPPREP
ISPVCALL
ISRRLIST
ISRROUTE
KEYLIST
PRIVATE
SHARED
ON
OFF
KEYS
KEYSHELP
LEFT
LIST
PRINT
DELETE
KEEP
Chapter 1. ISPF general information  7

## Page 34

LOG
PRINT
DELETE
KEEP
MSGID
ON
OFF
NOP
NRETRIEV
PANELID
ON
OFF
PFSHOW
ON
OFF
TAILOR
PRINT
PRINTG
PRINT-HI
PRINTL
PRINTLHI
PSCOLOR
RCHANGE
REFACTD
nnnnnnnn xx
REFACTL
nnnnnnnn xx
REFADDD
nnnnnnnn xx
REFADDL
nnnnnnnn xx
REFLISTD
xx
REFLISTL
xx
REFOPEND
REFOPENL
8  z/OS: z/OS ISPF Reference Summary

## Page 35

RESIZE
RETF
RETP
RETRIEVE
RETURN
RFIND
RIGHT
SAREA
SCRNAME
screenname
PERM
ON
OFF
SETTINGS
SHRPROF
SPLIT
NEW
SPLITV
START
SWAP
LIST
PREV
NEXT
screenname
n
SYSNAME
ON
OFF
TOP
TSO
TSOCMD
TUTOR
panelid
UP
USERID
ON
OFF
Chapter 1. ISPF general information  9

## Page 36

WINDOW
ZKEYS
Command table actions
ALIAS
When followed by the name of another command and optional parameters, allows specification of
command aliases.
NOP
Causes the command to be functionless. System displays an "inactive command" message in this
case.
PASSTHRU
Causes the command to be passed to the dialog, as though it had not been found in the table.
SELECT
When followed by selection keywords, causes the selected dialog command, program or selection
panel to be given control immediately.
SETVERB
Causes the command to be passed to the dialog with the command verb stored separately from the
parameters.
Blank (no action)
Causes the table entry to be ignored, and scanning to continue (to search for additional entries having
the same verb).
Variable name
Begins with an ampersand. Its content may be one of the listed actions. Allows dynamic specification
of a command action.
Dialog test commands
Primary commands
You can enter these commands on the Command line while using Dialog Test (option 7).
CANCEL
END
Syntax
LOCATE
LOC
L
string
QUAL
RESUME
RES
Line commands
These line commands have special meaning during testing operations:
D
n
10  z/OS: z/OS ISPF Reference Summary

## Page 37

Delete one or n lines starting with this line.
I
n
Insert one or n lines directly after this line, with underscores and quotes in the appropriate fields.
R
n
Repeat this line once or n times.
PDF Browse primary commands
You can enter these commands on the command line while using the Browse function.
BROWSE
 BRO member
GEN generation
COLUMNS
COLS
COL
ON
OFF
DISPLAY
LINE start_line
end_line
COLS start_col
end_col
CCSIDccsid_number
ASCII
USASCII
EBCDIC
UCS2
UTF8
UTF16
UTF32
OR:
DISPLAY
DISPL
DISP
DIS
char
NOCC
CC
NORDW
RDW
EDIT
member
GEN generation
PDF Component General Information
Chapter 1. ISPF general information  11

## Page 38

FIND
F
string
UTF8
ASCII
USASCII
NEXT
ALL
FIRST
LAST
PREV
CHARS
PREFIX
SUFFIX
WORD
col-1
col-2
HEX
ON
OFF
VERT
DATA
LOCATE
LOC
L
line-number
label
RESET
SUBMIT
SUBSYS ( subsystem )
VIEW
member
GEN generation
You can use this format to enter label definitions on the command line:
.ccccc
Defines a label (PDF component internal symbol), which is equated to the top line on the screen. Can
be used with LOCATE to scroll directly to that line.
PDF member list commands
Primary commands
You can enter these commands on the command line on member list displays.
CONFIRM
FILTER
field operator value
FIND
F
string field NEXT
ALL
FIRST
LAST
PREV
PREFIX
SUFFIX
WORD
PDF member list commands
12  z/OS: z/OS ISPF Reference Summary

## Page 39

LOCATE
LOC
L
string
MLC
MLS
REFRESH
RESET
RFIND
SAVE
list-id
SELECT
SEL
S
pattern
* lcmd
SORT
field1
A
D
field2
A
D
SRCHFOR
string
Line commands
On all member list displays except those for option 3.1 and 3.4, you can enter this 1-character command
at the beginning of a line.
S
Selects the member.
On option 3.1 and 3.4 member list displays, you can enter these 1-character commands at the beginning
of a line.
B
Browses the member.
C
Copies the member.
D
Deletes the member.
E
Edits the member.
G
Resets the member.
J
Submits the member.
M
Moves the member.
PDF member list commands
Chapter 1. ISPF general information  13

## Page 40

N
Displays the generation list for the member (if PDSE generation type).
P
Prints the member.
R
Renames the member. When using this command, you must also enter the new name to the right of
the member name.
T
TSO command.
V
Views the member.
TSO commands, CLISTs, and REXX EXECs can be entered in member lists that have an expanded line
command field. These are member lists displayed by using option M of the Data Set List utility. Here, any
command other than B, D, E, P, R, or V is considered to be a TSO command, CLIST, or REXX exec.
PDF member generation list commands
The member generation list is accessible via the member list panel from options 3.1 and 3.4. You can
enter these 1-character line commands.
B
Browses the generation.
D
Deletes the generation.
E
Edits the generation.
P
Prints the generation.
V
Views the generation.
I
Displays information about a generation.
/
Displays the Action for Generation panel.
Multiple line commands cannot be entered at one time. TSO commands are also not supported on
member generation lists.
PDF data set list commands
Primary commands
You can enter these commands on the command line on option 3.4 data set list displays.
APPEND
CONFIRM
CON
C
ON
OFF
DSLIST
EXCLUDE
PDF member generation list commands
14  z/OS: z/OS ISPF Reference Summary

## Page 41

FIND
F
string
NEXT
ALL
FIRST
LAST
PREV
CHARS
PREFIX
SUFFIX
WORD
LC
LOCATE
LOC
L
lparm
REFRESH
RESET
RFIND
SAVE
list-id
SHOWCMD
SHOW ON
OFF
SORT
field1
field2
VA
VS
VT
VW
Line commands
On option 3.4 data set list displays, you can enter the following 1-character commands at the beginning of
a line. Any other command entered at the beginning of a line is considered to be a TSO command, CLIST,
or REXX exec.
B
For a library or partitioned data set, displays a member list. You can then use the S command to select
a member to browse. For a sequential data set, displays the data set in browse mode.
C
Catalogs the data set.
CO
Copies a data set.
D
Deletes an entire data set. Displays a Confirm Delete panel if you request confirmation.
PDF Component General Information
Chapter 1. ISPF general information  15

## Page 42

E
For a library or partitioned data set, displays a member list. You can then use the S command to select
a member to edit. For a sequential data set, displays the data set in edit mode.
F
Frees unused space in a data set.
I
Displays library or data set information.
M
For a library or partitioned data set, displays a member list.
MO
Moves a data set.
NX
Unexclude a line from display.
NXF
Unexclude the first of a set of excluded data sets.
NXL
Unexclude the last of a set of excluded data sets.
P
Prints the library or data set.
PX
Prints an index listing.
R
Displays a panel, on which you can rename the library or data set.
RA
Adds a data set to a reference list.
RS
Resets statistical data.
S
Displays library or data set information in short format.
U
Uncatalogs the data set.
V
For a library or partitioned data set, displays a member list. You can then use the S command to select
a member to view. For a sequential data set, displays the data set in view mode.
X
Excludes a data set from the list.
Z
Compresses a library or data set.
=
Repeats the last line command entered.
PDF Edit and View commands
Primary commands
While you are using the PDF editor to edit or view data, these commands can be entered on the command
line.
PDF Component General Information
16  z/OS: z/OS ISPF Reference Summary

## Page 43

AUTOLIST
ON
OFF
AUTONUM
ON
OFF
AUTOSAVE
ON
OFF
PROMPT
NOPROMPT
BOUNDS
BOUND
BNDS
BND
BOU
left_col
*
right_col
*
BROWSE
member
GEN generation
BUILTIN cmdname
CANCEL
CAN
CAPS
ON
OFF
CHANGE
CHA
CHG
C
string1 string2
.ZFIRST .ZLAST
labela labelb
NEXT
ALL
FIRST
LAST
PREV
CHARS
PREFIX
SUFFIX
WORD
X
NX
start_col
left_col right_col
COLS
ON
OFF
PDF Component General Information
Chapter 1. ISPF general information  17

## Page 44

COPY
member
( member)
dsname
dsname( member)
pathname
AFTER
BEFORE
label
start_line end_line
CREATE
CRE member
( member)
dsname( member)
dsname
pathname
labela labelb 1
Notes:
1 If you don't specify the group of lines using labels, you must specify the group by using C or M line
commands.
CUT
.ZFIRST .ZLAST
labela labelb 1
DEFAULT
clipboard_name X
NX
APPEND
REPLACE
DISPLAY
Notes:
1 You can also specify the group of lines using C or M line commands.
DEFINE
DEF
name MACRO
CMD
PGM
ALIAS name_2
NOP
RESET
DISABLED
DELETE
DEL
ALL
labela labelb
X
NX
ALL labela labelb
EDIT
member
GEN generation
EDITSET
EDSET
PDF Component General Information
18  z/OS: z/OS ISPF Reference Summary

## Page 45

END
EXCLUDE
EXCLUDED
EXC
EX
X
string
.ZFIRST .ZLAST
labela labelb
NEXT
ALL
FIRST
LAST
PREV
CHARS
PREFIX
SUFFIX
WORD
start_col
left_col right_col
FIND
F
string
.ZFIRST .ZLAST
labela labelb
NEXT
ALL
FIRST
LAST
PREV
CHARS
PREFIX
SUFFIX
WORD
X
NX
start_col
left_col right_col
FLIP
.ZFIRST .ZLAST
labela
labelb
HEX ON
VERT
DATA
VERT
DATA
OFF
HIDE EXCLUDE
EXCLUDED
EXC
EX
X
IMACRO name
NONE
LEVEL num
LF
PDF Component General Information
Chapter 1. ISPF general information  19

## Page 46

LOCATE label
linenum
LOCATE
NEXT
FIRST
LAST
PREV
CHANGE
COMMAND
ERROR
EXCLUDED
LABEL
SPECIAL
INFOLINE
MSGLINE
NOTELINE
.ZFIRST .ZLAST
labela labelb
MODEL
model_name
qualifier
AFTER
BEFORE
label
NOTES
NONOTES
MODEL
CLASS
class_name
MOVE
member
( member)
dsname
pathame
AFTER
BEFORE
label 1
Notes:
1 If you don't specify the position using a label, you must specify the position by using an A or B line
command.
NONUMBER
NONUMBR
NONUMB
NONUM
NOTES
NOTE
ON
OFF
PDF Component General Information
20  z/OS: z/OS ISPF Reference Summary

## Page 47

NULLS
NULL
NUL
ON STD
ON
ALL
STD
ALL
OFF
NUMBER
NUMB
NUM
ON
STD
COBOL
1
STD COBOL
NOSTD
NOCOBOL
NOSTD NOCOBOL
DISPLAY
OFF
Notes:
1 STD is the default for non-COBOL data set types. COBOL is the default for COBOL data set types.
PACK
ON
OFF
PASTE
DEFAULT
clipboard_name
AFTER
BEFORE
label
DELETE
KEEP
PRESERVE
ON
OFF
PROFILE
current_edit_profile
name
5
number
PROFILE LOCK
UNLOCK
PROFILE RESET
RCHANGE
PDF Component General Information
Chapter 1. ISPF general information  21

## Page 48

RECOVERY
RECOVER
RECOVRY
RECVRY
RECOV
RECVR
ON
SUSP
OFF
WARN
NOWARN
RENUM
REN
ON
STD
COBOL
1
STD COBOL DISPLAY
Notes:
1 STD is the default for non-COBOL data set types. COBOL is the default for COBOL data set types.
REPLACE
REPL
REP
member
( member)
dsname( member)
dsname
pathname
labela labelb 1
Notes:
1 If you don't specify the group of lines using labels, you must specify the group by using C or M line
commands.
RESET
RES CHANGE
COMMAND
ERROR
EXCLUDED
FIND
HIDE
LABEL
SOURCE
SPECIAL
.ZFIRST .ZLAST
labela labelb
RFIND
RMACRO name
! name
NONE
PDF Component General Information
22  z/OS: z/OS ISPF Reference Summary

## Page 49

SAVE
NEWGEN
NOGEN
SETUNDO
SETU
STORAGE
KEEP
RECOVER
ON
OFF
SORT
.ZFIRST .ZLAST
labela labelb X
NX
sort_field
sort_field:
A
D
start_col
end_col
SOURCE character_encoding
STATS
ON
OFF
EXT
SUBMIT
SUB
.ZFIRST .ZLAST
labela labelb X
NX
SUBSYS ( subsystem )
TABS
TAB
ON STD
ALL
tab_character
OFF
UNDO
UNNUMBER
UNNUMB
UNNUM
UNN
PDF Component General Information
Chapter 1. ISPF general information  23

## Page 50

VERSION
VERS
VER
num
VIEW
member
GEN generation
Line commands
Under Edit or View, you can enter these line commands at the beginning of a line by typing over the line
number. If you do not enter a value of n, the default is 1 except for:
• The shift commands, which default to 2 column positions
• The TE command, which defaults to the number of lines remaining on the screen
• The TF command, which defaults to the current right boundary.
(
((
2
n
Shifts columns left the specified number of positions
)
))
2
n
Shifts columns right the specified number of positions
<
<<
2
n
Shifts data left the specified number of positions (default 2).
>
>>
2
n
Shifts data right the specified number of positions (default 2).
A
AK n
Identifies the line after which copied, moved, or model lines are to be inserted.
B
BK n
Identifies the line before which copied, moved, or model lines are to be inserted.
PDF Component General Information
24  z/OS: z/OS ISPF Reference Summary

## Page 51

BOUNDS
BOUND
BNDS
BND
BOU
Displays the column boundary definition line.
C
n
CC
Copies one or more lines from one location to another.
COLS
COL
Displays a position identification line.
D
n
DD
Deletes one or more lines.
F
n
Redisplays one or more lines at the beginning of a block of excluded lines.
HX
n
HXX
Displays characters in hexadecimal format.
I
n
Inserts a blank data entry line.
L
n
Redisplays one or more lines at the end of a block of excluded lines.
LC
n
LCC
LCLC
Converts all uppercase alphabetic characters in one or more lines to lowercase.
PDF Component General Information
Chapter 1. ISPF general information  25

## Page 52

M
n
MM
Moves one or more lines from one location to another.
MASK
Displays the contents of the mask when used with the I (insert), TE (text entry), and TS (text split) line
commands.
MD
n
MDD
MDMD
Makes NOTE, MSG, INFO, and COLS lines into data lines.
O
OK n
OO
OOK
Rearranges a single column list of items into multiple column, or tabular, format.
R
RR n
Repeats one or more lines.
S
n
Redisplays one or more lines with the leftmost indentation in a block of excluded lines.
TABS
TAB
Displays the tab definition line.
TE
n
Inserts blank lines to allow power typing for text entry.
TF
n
Restructures paragraphs following deletions, insertions, splitting, and so forth.
TS
n
Divides a line so that data can be added.
PDF Component General Information
26  z/OS: z/OS ISPF Reference Summary

## Page 53

UC
n
UCC
UCUC
Converts all lowercase alphabetic characters in one or more lines to uppercase.
X
n
XX
Excludes one or more lines from a panel.
Picture search strings – special characters for string-1:
Table 1. Picture search strings - special characters for string-1
String Character
P'=' Any character
P'¬' Any character that is not a blank
P'.' Any character that cannot be displayed
P'#' Any numeric character, 0-9
P'-' Any nonnumeric character
P'@' Any alphabetic character, uppercase or lowercase
P'<' Any lowercase alphabetic character
P'>' Any uppercase alphabetic character
P'$' Any special character, neither alphabetic nor
numeric.
If you are using an APL or TEXT keyboard, you can use the following additional characters in a picture
string:
Table 2. Picture search strings - additional characters
String Character
P' 
 ' Any APL-specific or TEXT-specific character
P'_' Any underscored nonblank character.
Picture search strings – special characters for string-2
Table 3. Picture search strings - special characters for string-2
String Character
P'=' Equal to the corresponding character in string-1
P'>' Converts the corresponding character in string-1 to
uppercase
P'<' Converts corresponding character in string-1 to
lowercase.
PDF Component General Information
Chapter 1. ISPF general information  27

## Page 54

Character search string format
Table 4. Character search string format
String Character
Simple string: cccccc (no embedded blanks or commas)
Delimited string: 'ccccc' or "ccccc"
Hex string: X'hhhh' or 'hhhh'X
Text string: T'cccc' or 'cccc'T
Picture string: P'ssss' or 'ssss'P
Character string: C'cccc' or 'cccc'C
Previous string: * (single asterisk)
PDF Component General Information
28  z/OS: z/OS ISPF Reference Summary
