# Chapter 4. Edit macro commands

Source file: f54rs00_v3r1.md
Start page: 163
Page span: 163-206

## Page 163

Chapter 4. Edit macro commands
This chapter contains the syntax and return codes for the ISPF Edit macros. For a complete description of
the Edit macros see z/OS ISPF Edit and Edit Macros.
AUTOLIST - set or query Autolist mode
Macro command syntax
ISREDIT AUTOLIST
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = AUTOLIST
ISREDIT AUTOLIST =
ON
OFF
Return codes
 0
Normal completion.
20
Severe error.
AUTONUM—set or query Autonum mode
Macro command syntax
ISREDIT AUTONUM
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = AUTONUM
ISREDIT AUTONUM =
ON
OFF
Return codes
 0
Normal completion.
20
Severe error.
AUTOLIST edit macro
© Copyright IBM Corp. 1989, 2024 137

## Page 164

AUTOSAVE—set or query Autosave mode
Macro command syntax
ISREDIT AUTOSAVE
ON
PROMPT
OFF
PROMPT
NOPROMPT
Assignment statement syntax
ISREDIT ( var1, var2)  = AUTOSAVE
ISREDIT AUTOSAVE  = 
ON
PROMPT
OFF
PROMPT
NOPROMPT
Return codes
 0
Normal completion.
 4
OFF NOPROMPT specified.
20
Severe error.
BLKSIZE—query the block size
Assignment statement syntax
ISREDIT ( varname) = BLKSIZE
Return codes
 0
Normal completion.
Note: For a z/OS UNIX file, the BLKSIZE assignment statement returns a value of 0.
12
Syntax error.
20
Severe error.
AUTOSAVE edit macro
138  z/OS: z/OS ISPF Reference Summary

## Page 165

BOUNDS—set or query the edit boundaries
Macro command syntax
ISREDIT BOUNDS
BOUND
BNDS
BND
BOU
left-col right-col
Assignment statement syntax
ISREDIT ( var1, var2) = BOUNDS
ISREDIT BOUNDS =
left-col right-col
Return codes
 0
Normal completion.
 4
Right boundary greater than default; default right boundary used.
12
Invalid boundaries specified.
20
Severe error.
BROWSE—browse from within an edit session
Macro command syntax
ISREDIT BROWSE member
Return codes
0
Normal completion
12
Your error (invalid member name, recovery pending)
20
Severe error.
BUILTIN—process a built-in command
Macro command syntax
ISREDIT BUILTIN cmdname
BOUNDS edit macro
Chapter 4. Edit macro commands  139

## Page 166

Return codes
 n
Return code from the built-in command.
20
Severe error.
CANCEL—cancel edit changes
Macro command syntax
ISREDIT CANCEL
Return codes
 0
Normal completion.
20
Severe error.
CAPS—set or query Caps mode
Macro command syntax
ISREDIT CAPS
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = CAPS
ISREDIT CAPS =
ON
OFF
Return codes
 0
Normal completion.
20
Severe error.
CANCEL edit macro
140  z/OS: z/OS ISPF Reference Summary

## Page 167

CHANGE—change a search string
Macro command syntax
ISREDIT CHANGE string1 string2
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
Return codes
 0
Normal completion.
 4
String not found.
 8
Change error. String-2 is longer than string-1 and substitution was not performed on at least one
change.
12
Inconsistent parameters. The string to be found will not fit between the specified columns.
20
Severe error.
CHANGE_COUNTS—query change counts
Assignment statement syntax
ISREDIT ( var1, var2) = CHANGE_COUNTS
Return codes
 0
Normal completion.
20
Severe error.
CHANGE edit macro
Chapter 4. Edit macro commands  141

## Page 168

COMPARE—compare data set
Macro command syntax
ISREDIT COMPARE dsname
NEXT
SESSION
 * 
EXCLUDE SAVE
SYSIN
Return codes
0
Normal completion
8
Member or data set not found, or an error opening the member or data set occurred.
12
No parameters specified, or another parameter error such as not valid NEXT or member specification.
20
Severe error. SuperC, allocation, or delta file error occurred.
COPY—copy data
ISREDIT COPY member
( member)
dsname
dsname( member)
pathname
AFTER
BEFORE
label
start_line
end_line
ASCII
EBCDIC
UTF8
Return codes
0
Normal completion.
8
End of data reached before last record read.
12
Invalid line pointer (lptr); member not found or BLDL error.
16
End of data reached before first record of specified range was reached.
20
Syntax error (invalid name, incomplete range,), or I/0 error.
COMPARE edit macro
142  z/OS: z/OS ISPF Reference Summary

## Page 169

CREATE—create a data set or data set member
ISREDIT CREATE member
( member)
dsname( member)
dsname
pathname
labela labelb
linenum1 linenum2
ASCII
EBCDIC
UTF8
Return codes
 0
Normal completion.
 8
Member already exists, member not created.
12
Invalid line pointer (lptr). The referenced line does not exist in the file.
20
Syntax error (invalid name or incomplete lptr range), or I/O error.
CURSOR—set or query the cursor position
Assignment statement syntax
ISREDIT ( var1, var2) = CURSOR
ISREDIT CURSOR = lptr
col
Return codes
 0
Normal completion.
 4
Column number beyond data, line number incremented.
12
Invalid line number.
20
Severe error.
Note: To set the cursor to the command line, exit your macro with a return code of 1.
CREATE edit macro
Chapter 4. Edit macro commands  143

## Page 170

CUT—cut and save lines
ISREDIT CUT
.ZFIRST .ZLAST
labela labelb
linenum1 linenum2
DEFAULT
clipboard_name X
NX
APPEND
REPLACE
ASCII
EBCDIC
UTF8
Return codes
 0
Normal completion.
12
Parameter error. Insufficient storage, or no more clipboards available.
20
Severe error.
DATA_CHANGED—query the data changed status
Assignment statement syntax
ISREDIT ( varname) = DATA_CHANGED
Return codes
 0
Normal completion.
20
Severe error.
DATA_WIDTH—query data width
Assignment statement syntax
ISREDIT ( varname) = DATA_WIDTH
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
CUT edit macro
144  z/OS: z/OS ISPF Reference Summary

## Page 171

DATAID—query data ID
Assignment statement syntax
ISREDIT ( varname) = DATAID
Return codes
 0
The data ID returned was passed to the editor.
 4
Data ID was generated by and will be freed by the editor.
 8
A previously generated data ID was returned.
20
Severe error.
DATASET—query the current data set name
Assignment statement syntax
ISREDIT ( var1, var2, var3) = DATASET
Return codes
 0
Normal completion.
20
Severe error.
DEFINE—define a name
Macro command syntax
ISREDIT DEFINE name
MACRO CMD
MACRO PGM
ALIAS name-2
NOP
RESET
DISABLED
Return codes
 0
Normal completion.
 8
RESET was attempted for a name not currently defined, or DEFINE name ALIAS name-2 requested
and name-2 is a NOP.
12
DEFINE was attempted for a name not currently defined.
DATAID edit macro
Chapter 4. Edit macro commands  145

## Page 172

20
Severe error (unknown command).
DELETE—delete lines
Macro command syntax
ISREDIT DELETE ALL X
NX linenum1
linenum2
labela
labelb
ALL
X
NX
linenum1
linenum2
labela
labelb
linenum1
linenum2
labela
labelb
Return codes
 0
Normal (lines deleted successfully).
 4
No lines deleted.
 8
No standard records exist.
12
Invalid line number.
20
Severe error.
DISPLAY_COLS—query display columns
Assignment statement syntax
ISREDIT ( var1, var2) = DISPLAY_COLS
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
DELETE edit macro
146  z/OS: z/OS ISPF Reference Summary

## Page 173

DISPLAY_LINES—query display lines
Assignment statement syntax
ISREDIT ( var1, var2) = DISPLAY_LINES
Return codes
 0
Normal completion.
 4
No visible data lines.
 8
No existing data lines.
12
Invalid command format.
20
Severe error.
DOWN—scroll down
Macro command syntax
ISREDIT DOWN amt
Return codes
 0
Normal completion.
 2
No more data DOWN.
 4
No visible lines.
 8
No data to display.
12
Amount not specified.
20
Severe error.
EDIT—edit from within an edit session
Macro command syntax
ISREDIT EDIT member
Return codes
 0
Normal completion. Data was saved.
DISPLAY_LINES edit macro
Chapter 4. Edit macro commands  147

## Page 174

4
Normal completion. Data was not saved.
12
Your error (invalid member name, recovery pending).
14
Member in use.
20
Severe error.
28
No ISREDIT MACRO statement preceded this call, or BROWSE was substituted because of the size of
the member being edited.
END—end the edit session
Macro command syntax
ISREDIT END
Return codes
 0
Normal completion.
 4
New member saved.
12
END not done, AUTOSAVE OFF PROMPT set, or Data not saved (insufficient space).
20
Severe error.
EXCLUDE—exclude lines from the panel
Macro command syntax
ISREDIT EXCLUDE string
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
Return codes
 0
Normal completion.
 4
String not found.
END edit macro
148  z/OS: z/OS ISPF Reference Summary

## Page 175

8
Line(s) not excluded.
12
Inconsistent parameters
20
Severe error.
EXCLUDE_COUNTS—query exclude counts
Assignment statement syntax
ISREDIT ( var1, var2) = EXCLUDE_COUNTS
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
FIND—find a search string
Macro command syntax
ISREDIT FIND
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
Return codes
 0
Normal completion.
 4
String not found.
12
Syntax error.
20
Severe error.
EXCLUDE_COUNTS edit macro
Chapter 4. Edit macro commands  149

## Page 176

FIND_COUNTS—query find counts
Assignment statement syntax
ISREDIT ( var1, var2) = FIND_COUNTS
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
FLIP—reverse excluded status of lines
Macro command syntax
ISREDIT FLIP
label-range
Return codes
 0
Successful completion. The excluded status of the requested lines was reversed.
20
Severe error.
FLOW_COUNTS—query flow counts
Assignment statement syntax
ISREDIT ( var1, var2) = FLOW_COUNTS
Return codes
 0
Normal completion.
20
Severe error.
FIND_COUNTS edit macro
150  z/OS: z/OS ISPF Reference Summary

## Page 177

HEX—set or query Hexadecimal mode
Macro command syntax
ISREDIT HEX ON
VERT
DATA
VERT
DATA
OFF
Assignment statement syntax
ISREDIT ( var1, var2)  = HEX
ISREDIT HEX  = ON
VERT
DATA
VERT
DATA
OFF
Return codes
 0
Normal completion.
20
Severe error.
HIDE—hide excluded lines message
Macro command syntax
ISREDIT HIDE X
Return codes
 0
Normal completion.
20
Severe error.
HEX edit macro
Chapter 4. Edit macro commands  151

## Page 178

HILITE—enhanced edit coloring
Macro command syntax
ISREDIT HILITE
ON
OFF
LOGIC
IFLOGIC
DOLOGIC
NOLOGIC
AUTO
DEFAULT
OTHER
ASM
BOOK
C
COBOL
DTL
HTML
JCL
PANEL
PASCAL
PLI
REXX
SKEL
IDL
SUPERC
XML
MARGINS(  left, right) RESET PAREN FIND
CURSOR SEARCH DISABLED
Return codes
 0
Normal completion.
 8
One of the following conditions:
• LOGIC or SEARCH not supported in the current environment
• Invalid language
• HILITE unavailable.
12
One of the following conditions:
• HILITE dialog is invalid from an edit macro
• HILITE not available because of the installation defaults
• HILITE not available because the edit panel in use is not enabled for enhanced color
• Other error encountered.
HILITE edit macro
152  z/OS: z/OS ISPF Reference Summary

## Page 179

20
Severe error. Possibly extra parameters.
IMACRO—set or query an initial macro
Macro command syntax
ISREDIT IMACRO name
NONE
Assignment statement syntax
ISREDIT ( varname) = IMACRO
ISREDIT IMACRO = name
Return codes
 0
Normal completion.
 4
IMACRO set not accepted; profile is locked.
12
Invalid name specified.
20
Severe error.
INSERT—prepare display for data insertion
Macro command syntax
ISREDIT INSERT lptr
numlines
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
LABEL—set or query a line label
Assignment statement syntax
ISREDIT ( var1, var2) = LABEL lptr
ISREDIT LABEL lptr = labelname
level
IMACRO edit macro
Chapter 4. Edit macro commands  153

## Page 180

Return codes
 0
Normal completion.
 4
Label name not returned, specified line has no label.
 8
Label set, but an existing label at the same level was deleted.
12
Line number specified is beyond the end of data.
20
Severe error.
LEFT—scroll left
Macro command syntax
ISREDIT LEFT amt
Return codes
 0
Normal completion.
 4
No visible lines.
 8
No data to display.
12
Amount not specified.
20
Severe error.
LEVEL—set or query the mod level number
Macro command syntax
ISREDIT LEVEL num
Assignment statement syntax
ISREDIT ( varname) = LEVEL
ISREDIT LEVEL = num
Return codes
 0
Normal completion.
 4
Statistics mode is off; the command is ignored.
12
Invalid value specified.
LEFT edit macro
154  z/OS: z/OS ISPF Reference Summary

## Page 181

20
Severe error.
LF—realign data on the ASCII linefeed character
Macro command syntax
ISREDIT LF
Return codes
 0
Normal completion.
LINE—set or query a line from the data set
Assignment statement syntax
ISREDIT ( varname) = LINE lptr
ISREDIT LINE lptr = data
Return codes
 0
Normal completion.
 4
Data truncated (line shorter than data supplied).
 8
Variable not found.
12
Invalid line number.
16
Variable data truncated.
20
Severe error.
LINE_AFTER—add a line to the current data set
Assignment statement syntax
ISREDIT LINE_AFTER linenum
label
 = 
DATALINE
INFOLINE
MSGLINE
NOTELINE
data
Return codes
 0
Normal completion.
LF edit macro
Chapter 4. Edit macro commands  155

## Page 182

4
Data truncated.
12
Invalid line number.
20
Severe error.
LINE_BEFORE—add a line to the current data set
Assignment statement syntax
ISREDIT LINE_BEFORE linenum
label
 = 
DATALINE
INFOLINE
MSGLINE
NOTELINE
data
Return codes
 0
Normal completion.
 4
Data truncated.
12
Invalid line number.
20
Severe error.
LINE_STATUS—query source and change information for a line in a
data set
Assignment statement syntax
ISREDIT ( varname) = LINE_STATUS lptr
Return codes
 0
Normal completion.
12
Line number not valid.
20
Severe error.
LINENUM—query the line number of a labeled line
Assignment statement syntax
ISREDIT ( varname) = LINENUM label
LINE_BEFORE edit macro
156  z/OS: z/OS ISPF Reference Summary

## Page 183

Return codes
 0
Normal completion.
 4
Line 0 specified.
 8
Label specified, but not found (variable set to 0).
12
Invalid line number.
20
Severe error.
LOCATE—locate a line
Specific locate syntax
ISREDIT LOCATE lptr
Generic locate syntax
ISREDIT LOCATE
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
Return codes
 0
Normal completion.
 4
Line not located.
 8
Empty member or data set.
20
Severe error.
LRECL—query the logical record length
Assignment statement syntax
ISREDIT ( varname) = LRECL
LOCATE edit macro
Chapter 4. Edit macro commands  157

## Page 184

Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
MACRO—identify an edit macro
Macro command syntax
ISREDIT MACRO
(
,
variable )
PROCESS
NOPROCESS
Return codes
 0
Normal completion.
 8
No parameters are permitted for this processing.
12
Syntax error.
20
Severe error.
MACRO_LEVEL—query the macro nesting level
Assignment statement syntax
ISREDIT ( varname) = MACRO_LEVEL
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
MACRO_MSG—set or query the macro message switch
Assignment statement syntax
ISREDIT ( varname)  = MACRO_MSG
MACRO edit macro
158  z/OS: z/OS ISPF Reference Summary

## Page 185

ISREDIT MACRO_MSG  = 
ON
OFF
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
MASKLINE—set or query the mask line
Assignment statement syntax
ISREDIT ( varname) = MASKLINE
ISREDIT MASKLINE = data
Return codes
 0
Normal completion.
 4
Data truncated.
16
Variable data truncated.
20
Severe error.
MEMBER—query the current member name
Assignment statement syntax
ISREDIT ( varname) = MEMBER
Return codes
 0
Normal completion.
12
Invalid command format.
20
Severe error.
MASKLINE edit macro
Chapter 4. Edit macro commands  159

## Page 186

MEND—end a macro in the batch environment
Macro command syntax
ISREDIT MEND
Return codes
 0
Normal completion.
20
Severe error.
Note: Only required in the MVS/370 environment.
MODEL—copy a model into the current data set
Macro command model name syntax
ISREDIT MODEL
model_name
qualifier
AFTER
BEFORE
linenum
label
NOTES
NONOTES
Macro command class name syntax
ISREDIT MODEL CLASS class-name
Return codes
 0
Normal completion.
 4
Data truncated (the model exceeded the right-hand margin of the data being edited).
12
Invalid line pointer.
20
Severe error.
MOVE—move a data set member
ISREDIT MOVE member
( member)
dsname
pathname
AFTER
BEFORE
linenum
label ASCII
EBCDIC
UTF8
MEND edit macro
160  z/OS: z/OS ISPF Reference Summary

## Page 187

Return codes
 0
Normal completion.
 8
End of data before last record read, or the specified data set is in use.
12
Invalid line pointer (lptr); member not found or BLDL error.
16
End of data before first record read.
20
Syntax error (invalid name, incomplete range), or I/O error.
NONUMBER—turn off Number mode
Syntax
ISREDIT NONUMBER
Return codes
 0
Normal completion.
20
Severe error.
NOTES—set or query Note mode
Macro command syntax
ISREDIT NOTES
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = NOTES
ISREDIT NOTES =
ON
OFF
Return codes
 0
Normal completion.
20
Severe error.
NONUMBER edit macro
Chapter 4. Edit macro commands  161

## Page 188

NULLS—set or query Nulls mode
Macro command syntax
ISREDIT NULLS
ON STD
ON
ALL
STD
ALL
OFF
Assignment statement syntax
ISREDIT ( var1, var2)  = NULLS
ISREDIT NULLS  = 
ON STD
ON
ALL
STD
ALL
OFF
Return codes
 0
Normal completion.
20
Severe error.
NUMBER—set or query Number mode
Macro command syntax
ISREDIT NUMBER
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
NULLS edit macro
162  z/OS: z/OS ISPF Reference Summary

## Page 189

Assignment statement syntax
ISREDIT ( var1, var2) = NUMBER
ISREDIT NUMBER =
ON STD DISPLAY OFF
COBOL STD COBOL NOSTD NOCOBOL
NOSTD NOCOBOL
Return codes
 0
Normal completion.
20
Severe error.
PACK—set or query Pack mode
Macro command syntax
ISREDIT PACK
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = PACK
ISREDIT PACK =
ON
OFF
Return codes
 0
Normal completion.
20
Severe error.
PASTE—move or copy lines from clipboard
Macro command syntax
ISREDIT PASTE
DEFAULT
clipboard_name
AFTER
BEFORE
linenum
label
DELETE
KEEP ASIS
PACK edit macro
Chapter 4. Edit macro commands  163

## Page 190

Return codes
 0
Normal completion.
12
Parameter error. Clipboard empty or does not exist.
20
Severe error.
PRESERVE—enable saving of trailing blanks
Macro command syntax
ISREDIT PRESERVE
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = PRESERVE
ISREDIT PRESERVE =
ON
OFF
Return codes
 0
Normal completion.
 6
Record format is not variable.
16
Error setting variable.
20
Severe error.
PROCESS—process the panel
Macro command syntax
ISREDIT PROCESS
DEST RANGE cmd1
cmd2
Return codes
 0
Normal completion.
 4
A RANGE was expected by the macro, but one was not specified; default values set.
 8
A DEST (destination) was expected by the macro, but one was not specified; default values set.
PRESERVE edit macro
164  z/OS: z/OS ISPF Reference Summary

## Page 191

12
Both a RANGE and a DEST (destination) were expected by the macro, but were not specified; default
values set.
16
You entered incomplete or conflicting line commands.
20
Severe error.
Note: ISPF does not consider a return code of 12 from the PROCESS edit macro command an error. A
macro that receives a return code of 12 from the PROCESS edit macro does not terminate.
PROFILE—set or query the current profile
Macro command profile control syntax
ISREDIT PROFILE
name number
Macro command profile lock syntax
ISREDIT PROFILE LOCK
UNLOCK
Macro command profile reset syntax
ISREDIT PROFILE RESET
Assignment statement syntax
ISREDIT ( var1, var2) = PROFILE
Return codes
 0
Normal completion.
20
Severe error.
RANGE_CMD—query a command that you entered
Assignment statement syntax
ISREDIT ( varname) = RANGE_CMD
Return codes
 0
Normal completion.
 4
Line command not set.
 8
Line command setting not acceptable.
PROFILE edit macro
Chapter 4. Edit macro commands  165

## Page 192

20
Severe error.
RCHANGE—repeat a change
Macro command syntax
ISREDIT RCHANGE
Return codes
 0
Normal completion.
 4
String not found.
 8
Change error (string-2 longer than string-1 and substitution was not performed on at least one
change).
12
Syntax error.
20
Severe error.
RECFM—query the record format
Assignment statement syntax
ISREDIT ( var1, var2) = RECFM
Return codes
 0
Normal completion.
20
Severe error.
RECOVERY—set or query Recovery mode
Macro command syntax
ISREDIT RECOVERY
ON
SUSP
OFF
WARN
NOWARN
Assignment statement syntax
ISREDIT ( var1, var2) = RECOVERY
RCHANGE edit macro
166  z/OS: z/OS ISPF Reference Summary

## Page 193

ISREDIT RECOVERY =
ON
SUSP
OFF
WARN OFF
NOWARN
Return codes
 0
Normal completion.
20
Severe error.
RENUM—renumber data set lines
Macro command syntax
ISREDIT RENUM
ON
STD
COBOL
1
STD COBOL DISPLAY
Notes:
1 STD is the default for non-COBOL data set types. COBOL is the default for COBOL data set types.
Return codes
 0
Normal completion.
20
Severe error.
REPLACE—replace a data set or data set member
Macro command syntax
ISREDIT REPLACE member
( member)
dsname( member)
dsname
pathname
labela labelb
linenum1 linenum2
ASCII
EBCDIC
UTF8
RENUM edit macro
Chapter 4. Edit macro commands  167

## Page 194

Return codes
 0
Normal completion.
 8
Member in use.
12
Invalid line pointer; member not found or BLDL error.
20
Syntax error (invalid name, incomplete line pointer value), or I/O error.
RESET—reset the data display
Macro command syntax
ISREDIT RESET
CHANGE
COMMAND
ERROR
EXCLUDED
FIND
HIDE
LABEL
SPECIAL
ALL
.ZFIRST .ZLAST
labela labelb
linenum1 linenum2
Return codes
 0
Normal completion.
20
Severe error.
RFIND—Repeat Find
Macro command syntax
ISREDIT RFIND
Return codes
 0
Normal completion.
 4
String not found.
12
Syntax error.
20
Severe error (string not defined).
RESET edit macro
168  z/OS: z/OS ISPF Reference Summary

## Page 195

RIGHT—scroll right
Macro command syntax
ISREDIT RIGHT amt
Return codes
 0
Normal completion.
 4
No visible lines.
 8
No data to display.
12
Amount not specified.
20
Severe error.
RMACRO—set or query the recovery macro
Macro command syntax
ISREDIT RMACRO name
NONE
Assignment statement syntax
ISREDIT ( varname) = RMACRO
ISREDIT RMACRO = name
NONE
Return codes
 0
Normal completion.
12
Invalid name specified.
20
Severe error.
SAVE—save the current data
Macro command syntax
ISREDIT SAVE
RIGHT edit macro
Chapter 4. Edit macro commands  169

## Page 196

Return codes
 0
Normal completion.
 4
New member saved.
12
Data not saved; not enough PDS space or directory space.
20
Severe error.
SAVE_LENGTH—set or query length for variable-length data
Macro command syntax
ISREDIT ( variable ) = SAVE_LENGTH . lptr
ISREDIT SAVE_LENGTH . lptr = value
Return codes
 0
Normal completion.
 4
Value supplied on set call was out of range. If the supplied length was too great, it is adjusted to
equal the maximum record length. Otherwise, the length is adjusted to the length of the nonblank
data portion of the record.
 6
Record format is not variable. Any value of an assigned request is ignored.
16
Error setting variable.
20
Severe error.
SCAN—set command scan mode
Macro command syntax
ISREDIT SCAN
ON
OFF
Assignment statement syntax
ISREDIT ( varname) = SCAN
ISREDIT SCAN =
ON
OFF
SAVE_LENGTH edit macro
170  z/OS: z/OS ISPF Reference Summary

## Page 197

Return codes
 0
Normal completion.
20
Severe error.
SEEK—seek a data string, positioning the cursor
Macro command syntax
ISREDIT SEEK string
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
Return codes
 0
Normal completion.
 4
String not found.
12
Syntax error.
20
Severe error.
SEEK_COUNTS—query seek counts
Assignment statement syntax
ISREDIT ( var1, var2) = SEEK_COUNTS
Return codes
 0
Normal completion.
20
Severe error.
SESSION—identify type of session
Assignment statement syntax
ISREDIT ( var1, var2) = SESSION
SEEK edit macro
Chapter 4. Edit macro commands  171

## Page 198

Return codes
 0
Normal completion.
20
Severe error.
SETUNDO—set UNDO mode
Macro command syntax
ISREDIT SETUNDO
STORAGE KEEP RECOVER ON
OFF
Assignment statement syntax
ISREDIT ( varname) = SETUNDO
ISREDIT SETUNDO =
STORAGE KEEP RECOVER
ON OFF
Return codes
 0
Successful completion. SETUNDO was turned on or off, or status remains unchanged because UNDO
was already on or off.
20
Severe error. Probably a parameter error (something other than STG, REC, or OFF was specified).
SHIFT ( —shift columns left
Macro command syntax
ISREDIT SHIFT ( lptr
2
n
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
SETUNDO edit macro
172  z/OS: z/OS ISPF Reference Summary

## Page 199

SHIFT ) —shift columns right
Macro command syntax
ISREDIT SHIFT ) lptr
2
n
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
SHIFT <—shift data left
Macro command syntax
ISREDIT SHIFT < lptr
2
n
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
SHIFT > —shift data right
Macro command syntax
ISREDIT SHIFT > lptr
2
n
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
SHIFT ) edit macro
Chapter 4. Edit macro commands  173

## Page 200

SORT—sort data
Macro command syntax
ISREDIT SORT
label-range X
NX
sort-field1
sort-field2
sort-field3
sort-field4
sort-field5
Return codes
 0
Normal completion.
 4
Lines were already in sort order.
 8
No records to sort.
16
Not enough storage to perform sort.
20
Severe error.
SOURCE—describe format of data
Macro command syntax
ISREDIT SOURCE character_encoding
Return codes
 0
Normal completion.
STATS—set or query Stats mode
Macro command syntax
ISREDIT STATS
ON
OFF
EXT
SORT edit macro
174  z/OS: z/OS ISPF Reference Summary

## Page 201

Assignment statement syntax
ISREDIT ( varname) = STATS
ISREDIT STATS =
ON
OFF
EXT
Return codes
 0
Normal completion.
20
Severe error.
SUBMIT—submit data for batch processing
Macro command syntax
ISREDIT SUBMIT
.ZFIRST .ZLAST
labela labelb X
NX
SUBSYS ( subsystem )
Return codes
 0
Normal completion.
20
Severe error (submit failed).
TABS—set or query Tabs mode
Macro command syntax
ISREDIT TABS
ON STD
ALL
tab_character
OFF
Assignment statement syntax
ISREDIT ( var1, var2) = TABS
SUBMIT edit macro
Chapter 4. Edit macro commands  175

## Page 202

ISREDIT TABS  = 
ON STD
ALL
tab_character
OFF
Return codes
 0
Normal completion.
20
Severe error.
TABSLINE—set or query tabs line
Assignment statement syntax
ISREDIT ( varname) = TABSLINE
ISREDIT TABSLINE = data
Return codes
 0
Normal completion.
 4
Data truncated.
 8
Invalid data detected and ignored.
20
Severe error (invalid input).
TENTER—set up panel for text entry
Macro command syntax
ISREDIT TENTER lptr
numlines
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
TABSLINE edit macro
176  z/OS: z/OS ISPF Reference Summary

## Page 203

TFLOW—text flow a paragraph
Macro command syntax
ISREDIT TFLOW lptr
col
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
TSPLIT—text split a line
Macro command syntax
ISREDIT TSPLIT
lptr col
Return codes
 0
Normal completion.
12
Invalid line number.
20
Severe error.
UNNUMBER—remove sequence numbers
Macro command syntax
ISREDIT UNNUMBER
Return codes
 0
Normal completion.
12
Number mode not on.
20
Severe error.
TFLOW edit macro
Chapter 4. Edit macro commands  177

## Page 204

UP—scroll up
Macro command syntax
ISREDIT UP amt
Return codes
 0
Normal completion.
 2
No more data UP.
 4
No visible lines.
 8
No data to display.
12
Amount not specified.
20
Severe error.
USER_STATE—save or restore user state
Assignment statement syntax
ISREDIT ( varname) = USER_STATE
ISREDIT USER_STATE = ( varname)
Return codes
 0
Normal completion.
20
Severe error.
VERSION—set or query version number
Macro command syntax
ISREDIT VERSION num
Assignment statement syntax
ISREDIT ( varname) = VERSION
ISREDIT VERSION = num
Return codes
 0
Normal completion.
UP edit macro
178  z/OS: z/OS ISPF Reference Summary

## Page 205

4
Stats mode is off, the command is ignored.
12
Invalid value specified (the version must be 1 to 99).
20
Severe error.
VIEW—view from within an edit session
Macro command syntax
ISREDIT VIEW member
Return codes
0
Normal completion
12
Your error (invalid member name, recovery pending)
20
Severe error.
VOLUME—query volume information
Assignment statement syntax
ISREDIT ( var1, var2, var3) = VOLUME
Return codes
 0
Normal completion.
 4
The data set is a multivolume data set and the shared pool variable ZEDMVOL is set to contain all the
volume serial numbers of the data set. ZEDMVOL has the length of the number of volumes times six.
20
Severe error.
XSTATUS—set or query exclude status of a line
Assignment statement syntax
ISREDIT ( varname) = XSTATUS lptr
ISREDIT XSTATUS lptr =
X
NX
Return codes
 0
Normal completion.
VIEW edit macro
Chapter 4. Edit macro commands  179

## Page 206

8
An attempt to set a line status to NX could not be performed. The line has a pending line command
on it. For example, if an excluded line contains an M line command in the line command field, then the
MOVE/COPY IS PENDING message is displayed and the lines cannot be shown. The reset command
can be used to remove your line commands from the line command field.
12
Line number is not an existing line.
20
Severe error.
XSTATUS edit macro
180  z/OS: z/OS ISPF Reference Summary
