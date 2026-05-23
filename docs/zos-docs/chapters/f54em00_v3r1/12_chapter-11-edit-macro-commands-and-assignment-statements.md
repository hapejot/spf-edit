# Chapter 11. Edit macro commands and assignment statements

Source file: f54em00_v3r1.md
Start page: 327
Page span: 327-458

## Page 327

Chapter 11. Edit macro commands and assignment
statements
This chapter documents intended Programming Interfaces that allow the customer to write programs
to obtain the services of ISPF. It also describes the edit macro commands and assignment statements
available for the PDF component. Edit macro commands and assignment statements must be included in
edit macros that you create.
Macro commands and assignment statements cannot be entered individually from the edit command line.
However, once you have created an edit macro, you can use the macro just like any other Edit primary
command. You can run an edit macro by:
• Typing the macro name on the command line and pressing Enter
• Pressing a function key to which the macro has been assigned, if any
Note: Edit macro commands should not be confused with TSO commands. Although both are programs,
edit macros must not be prefixed with the word 'TSO' when they are invoked.
All edit macros must have an ISREDIT MACRO statement as the first edit command. For more information
see “Syntax” on page 364.
Each command description in this documentation consists of:
Syntax
A syntax diagram for coding the macro command, including a description of any required or optional
operands.
Description
An explanation of the function and operation of the command. This description also refers to other
commands that can be used with this command.
Return Codes
A description of codes returned by the macro command. For all commands, a return code of 20 or
higher implies a severe error. See “Return codes from user-written edit macros” on page 109 and
“Return codes from PDF edit macro commands” on page 110 for more information.
Examples
Sample usage of the macro command.
Edit macro command summary
This table summarizes the edit macro commands. See the complete description of the commands on the
referenced page.
Table 19. Summary of the macro commands
Topic Description
“AUTOLIST—Set or Query Autolist
Mode” on page 300
Sets the current autolist mode or retrieves the value and
places it in a variable.
“AUTONUM—Set or Query Autonum
Mode” on page 301
Sets the current autonum mode or retrieves the value and
places it in a variable.
“AUTOSAVE—Set or Query Autosave
Mode” on page 303
Sets the current autosave mode or retrieves the value and
places it in a variable.
“BLKSIZE—Query the Block Size” on
page 304
Returns the block size of the data set being edited in a
specified variable.
Edit macro command summary
© Copyright IBM Corp. 1984, 2024 295

## Page 328

Table 19. Summary of the macro commands (continued)
Topic Description
“BOUNDS—Set or Query the Edit
Boundaries” on page 305
Sets the left and right boundaries or retrieves the values and
places them in variables.
“BROWSE—Browse from within an Edit
Session” on page 307
Browses another member in the data set.
“BUILTIN—Process a Built-In
Command” on page 308
Processes a built-in command even if a macro or macro
statement with the same name has been defined.
“CANCEL—Cancel Edit Changes” on
page 308
Ends the edit session without saving any changes.
“CAPS—Set or Query Caps Mode” on
page 309
Sets caps mode.
“CHANGE—Change a Search String” on
page 310
Changes a data string to another string.
“CHANGE_COUNTS—Query Change
Counts” on page 313
Retrieves the values set by the most recently processed
CHANGE command and places these values in variables.
“COMPARE—Edit Compare” on page 314 Compares a library member or data set with the data being
edited.
“COPY—Copy Data” on page 317 Copies a member of the library into the member being edited.
“CREATE—Create a Data Set or a Data
Set Member” on page 318
Creates a new member from the data that is being edited.
“CURSOR—Set or Query the Cursor
Position” on page 319
Sets the relative line and column number of the cursor or
retrieves the values and places them in variables.
“CUT—Cut and Save Lines” on page 322 Cut and save lines.
“DATA_CHANGED—Query the Data
Changed Status” on page 323
Retrieves the data changed status and places it in a variable.
“DATA_WIDTH—Query Data Width” on
page 324
Retrieves the logical data width and places it in a variable.
“DATAID—Query Data ID” on page 325 Retrieves the data ID for the data set being edited and places
it in a variable.
“DATASET—Query the Current and
Original Data Set Names” on page 326
Retrieves the name of a data set and places it in a variable.
“DEFINE—Define a Name” on page 326 • Assigns an alias to a macro or built-in command.
• Disables the use of a macro or built-in command.
• Identifies a macro that replaces a built-in command of the
same name.
• Identifies programs that are edit macros.
“DELETE—Delete Lines” on page 328 Deletes lines from the data.
“DISPLAY_COLS—Query Display
Columns” on page 330
Retrieves the column numbers for the first and last data
columns on the panel and places them in variables.
“DISPLAY_LINES—Query Display Lines”
on page 331
Retrieves the relative line numbers of the first and last data
lines that would appear if the macro ended and places them
in variables.
Edit macro command summary
296  z/OS: z/OS ISPF Edit and Edit Macros

## Page 329

Table 19. Summary of the macro commands (continued)
Topic Description
“DOWN—Scroll Down” on page 331 Scrolls data down from the current panel position.
“EDIT—Edit from within an Edit Session”
on page 333
Edits another member in the data set (recursive editing).
“END—End the Edit Session” on page
334
Ends the edit session.
“EXCLUDE—Exclude Lines from the
Display” on page 335
Marks lines in the data that should not appear.
“EXCLUDE_COUNTS—Query Exclude
Counts” on page 337
Retrieves the values set by the most recently processed
EXCLUDE command and places them in variables.
“FIND—Find a Search String” on page
338
Locates a search string. It is recommended that you do not
use FIND in a macro because any excluded data string found
is shown on the panel. Use SEEK to perform the identical
function without changing the line's exclude status.
“FIND_COUNTS—Query Find Counts” on
page 340
Retrieves values set by the most recently processed FIND or
RFIND command and places them in variables.
“FLIP—Reverse Exclude Status of Lines”
on page 341
Reverses the exclude status of a specified group of lines in a
file or of all the lines in a file.
“FLOW_COUNTS—Query Flow Counts”
on page 341
Retrieves values set by the most recently processed TFLOW
command and places them in variables.
“HEX—Set or Query Hexadecimal Mode”
on page 342
Sets the hexadecimal mode or retrieves the value and places
it in a variable.
“HIDE—Hide Excluded Lines Message”
on page 344
Removes the "n Line(s) not Displayed" messages from the
display where lines have been excluded by the EXCLUDE
command.
“HILITE—Enhanced Edit Coloring” on
page 344
Highlights in user-specified colors many language-specific
constructs, program logic features, the phrase containing
the cursor, and any strings that match the previous FIND
operation or those that would be found by an RFIND or
RCHANGE request. Can also be used to set default colors
for the data area in non-program files and for any characters
typed since the previous Enter or function key entry.
“IMACRO—Set or Query an Initial
Macro” on page 348
Sets or retrieves the value for the initial macro in the profile
and places it in a variable.
“INSERT—Prepare Display for Data
Insertion” on page 349
Displays one or more lines for data entry.
“LABEL—Set or Query a Line Label” on
page 351
Sets or retrieves the values for the label on the specified line
and places them in variables.
“LEFT—Scroll Left” on page 352 Scrolls data left from the current panel position.
“LEVEL—Set or Query the Modification
Level Number” on page 353
Sets the modification level number or retrieves the value and
places it in a variable.
“LINE—Set or Query a Line from the
Data Set” on page 354
Sets or retrieves the data from the data line and places it in a
variable.
Edit macro command summary
Chapter 11. Edit macro commands and assignment statements  297

## Page 330

Table 19. Summary of the macro commands (continued)
Topic Description
“LINE_AFTER—Add a Line to the Current
Data Set” on page 355
Adds a line after the specified line.
“LINE_BEFORE—Add a Line to the
Current Data Set” on page 357
Adds a line before the specified line.
“LINE_STATUS—Query Source and
Change Information for a Line in a Data
Set” on page 359
Retrieves source and change information for a specified data
line.
“LINENUM—Query the Line Number of a
Labeled Line” on page 360
Retrieves the relative line number of a specified label and
places it in a variable.
“LOCATE—Locate a Line” on page 361 Locates a line.
“LRECL—Query the Logical Record
Length” on page 364
Returns the logical record length of the data being edited in a
variable.
“MACRO—Identify an Edit Macro” on
page 364
Identifies a command as a macro. MACRO is required for all
macros and must be the first command in a CLIST or REXX
exec macro that is not a CLIST or REXX exec statement or the
first edit command in a program macro.
“MACRO_LEVEL—Query the Macro
Nesting Level” on page 366
Retrieves the nesting level of the macro being run and places
it in a variable.
“MACRO_MSG—Set or Query the Macro
Message switch” on page 366
Sets or retrieves the value of the macro_msg switch, which
controls whether macro processing delivers ISPF messages
to the macro.
“MASKLINE—Set or Query the Mask
Line” on page 367
Sets or retrieves the value of the mask line, which controls
the display formatting of input.
“MEMBER—Query the Current Member
Name” on page 368
Retrieves the name of the ISPF library member currently
being edited and places it in a variable.
“MEND—End a Macro in the Batch
Environment” on page 369
Ends a macro that is running in the batch environment. MEND
is obsolete.
“MODEL—Copy a Model into the Current
Data Set” on page 369
Copies a specified dialog development model before or after a
specified line.
“MOVE— Move a Data Set or a Data Set
Member” on page 371
Moves a member of a data set and places it after or before the
line specified.
“NONUMBER—Turn Off Number Mode”
on page 372
Turns off number mode.
“NOTES—Set or Query Note Mode” on
page 373
Sets the current note mode or retrieves the value and places
it in a variable.
“NULLS—Set or Query Nulls Mode” on
page 374
Sets the current nulls mode or retrieves the value and places
it in a variable.
“NUMBER—Set or Query Number Mode”
on page 375
Sets the current number mode or retrieves the value and
places it in a variable.
“PACK—Set or Query Pack Mode” on
page 378
Sets the current pack mode or retrieves the value and places
it in a variable.
“PASTE—Move or Copy Lines from
Clipboard” on page 379
Move or copy lines from a clipboard.
Edit macro command summary
298  z/OS: z/OS ISPF Edit and Edit Macros

## Page 331

Table 19. Summary of the macro commands (continued)
Topic Description
“PRESERVE—Enable Saving of Trailing
Blanks” on page 380
Sets the current pack mode or retrieves the value and places
it in a variable.
“PROCESS—Process Line Commands”
on page 381
Controls when the line commands or data changes typed at
the keyboard are to be processed.
“PROFILE—Set or Query the Current
Profile” on page 383
Allows you to view or change the default modes for your edit
session.
“RANGE_CMD—Query a Command That
You Entered” on page 384
Identifies the name of a line command typed at the keyboard
and processed by a macro.
“RCHANGE—Repeat a Change” on page
385
Repeats the most recently processed CHANGE command.
“RECFM—Query the Record Format” on
page 386
Retrieves the record format of the data set being edited and
places the value in variables.
“RECOVERY—Set or Query Recovery
Mode” on page 387
Sets the recovery mode or retrieves the value and places it in
a variable.
“RENUM—Renumber Data Set Lines” on
page 388
Sets number mode on and renumbers all data lines.
“REPLACE—Replace a Data Set or Data
Set Member” on page 390
Replaces the specified member in the library with the data
specified in the member being edited.
“RESET—Reset the Data Display” on
page 391
Restores the status of lines or deletes special temporary
lines.
“RFIND—Repeat Find” on page 393 Locates the data string defined by the most recently
processed SEEK, FIND, or CHANGE command, or excludes a
line that contains the data string from the previous EXCLUDE
command.
“RIGHT—Scroll Right” on page 394 Scrolls data to the right of the current panel position.
“RMACRO—Set or Query the Recovery
Macro” on page 395
Sets or retrieves the name of the macro set in this edit
session.
“SAVE—Save the Current Data” on page
396
Saves the data.
“SAVE_LENGTH—Set or Query Length
for Variable-Length Data” on page 397
Sets or queries the length to be used to save each record in a
variable-length file.
“SCAN—Set Command Scan Mode” on
page 398
Sets the current value of scan mode (for variable substitution)
or retrieves the value and places it in a variable.
“SEEK—Seek a Data String, Positioning
the Cursor” on page 399
Finds one or more occurrences of a data string. SEEK is
similar to FIND; however, when a string is found, the exclude
status of the line is not affected.
“SEEK_COUNTS—Query Seek Counts”
on page 402
Retrieves the values set by the most recently processed SEEK
command and places them in variables.
“SESSION—Query Session Type” on
page 402
Identifies the type of session in which the macro is running
“SHIFT (—Shift Columns Left” on page
405
Moves columns of data to the left.
Edit macro command summary
Chapter 11. Edit macro commands and assignment statements  299

## Page 332

Table 19. Summary of the macro commands (continued)
Topic Description
“SHIFT )—Shift Columns Right” on page
405
Moves columns of data to the right.
“SHIFT <—Shift Data Left” on page 406 Moves data to the left.
“SHIFT >—Shift Data Right” on page 407 Moves data to the right.
“SORT—Sort Data” on page 408 Puts data in a specified order.
“STATS—Set or Query Stats Mode” on
page 411
Sets the current stats mode or retrieves the value and places
it in a variable.
“SUBMIT—Submit Data for Batch
Processing” on page 412
Submits data that is to be processed as a batch job.
“TABS—Set or Query Tabs Mode” on
page 413
Sets the tabs mode or retrieves the mode and places it in a
variable.
“TABSLINE—Set or Query Tabs Line” on
page 415
Sets the tabs line or retrieves the tabs line and places it in a
variable.
“TENTER—Set Up Panel for Text Entry”
on page 416
Prepares the panel for power typing.
“TFLOW—Text Flow a Paragraph” on
page 418
Restructures paragraphs.
“TSPLIT—Text Split a Line” on page 418 Divides a line so data can be added.
“UNNUMBER—Remove Sequence
Numbers” on page 419
Removes the numbers from the data set and turns number
mode off.
“UP—Scroll Up” on page 420 Scrolls data up from the current panel position.
“USER_STATE—Save or Restore User
State” on page 421
Saves or restores the state of the edit profile values, FIND and
CHANGE values, and panel and cursor values.
“VERSION—Set or Query Version
Number” on page 422
Sets the version number or retrieves the value and places it in
a variable.
“VIEW—View from within an Edit
Session” on page 423
Views another member in the data set.
“VOLUME—Query Volume Information”
on page 424
Retrieves the volume serial number (or serial numbers) and
the number of volumes on which the data set resides.
“XSTATUS—Set or Query Exclude Status
of a Line” on page 425
Sets the exclude status of the specified data line or retrieves
the value and places it in a variable.
AUTOLIST—Set or Query Autolist Mode
The AUTOLIST macro command sets autolist mode, which controls the automatic printing of data to the
ISPF list data set.
The AUTOLIST assignment statement either sets autolist mode or retrieves the current setting of autolist
mode and places it in a variable.
Autolist mode is saved in the edit profile.
AUTOLIST
300  z/OS: z/OS ISPF Edit and Edit Macros

## Page 333

Syntax
ISREDIT AUTOLIST
ON
OFF
ON
Specifies that when you end an edit session and save changed data, the editor generates a source
listing in the ISPF list data set for eventual printing.
OFF
Does not generate a source listing.
ISREDIT ( varname)  = AUTOLIST
ISREDIT AUTOLIST  = 
ON
OFF
varname
The name of a variable that contains the setting of autolist mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Return codes
0
Normal completion
20
Severe error
Examples
To turn autolist mode on:
ISREDIT AUTOLIST ON
or
ISREDIT AUTOLIST = ON
To turn autolist mode off:
ISREDIT AUTOLIST OFF
or
ISREDIT AUTOLIST = OFF
AUTONUM—Set or Query Autonum Mode
The AUTONUM macro command sets autonum mode, which controls the automatic renumbering of data
when it is saved.
AUTONUM
Chapter 11. Edit macro commands and assignment statements  301

## Page 334

The AUTONUM assignment statement either sets autonum mode or retrieves the current setting of
autonum mode and places it in a variable.
Syntax
ISREDIT AUTONUM
ON
OFF
ON
Turns on automatic renumbering. When number mode is also on, the data is automatically
renumbered when it is saved.
OFF
Turns off automatic renumbering. Data is not renumbered.
ISREDIT ( varname)  = AUTONUM
ISREDIT AUTONUM  = 
ON
OFF
varname
The name of a variable containing the setting of autonum mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Description
When number mode is on, the first line of a data set or member is normally line number 000100,
the second number is 000200, and so on. However, as lines are inserted and deleted, the increments
between line numbers can change.
For example, you might think that when a line is inserted between 000100 and 000200, line 000200
would be given the number 000300 and the new line would become 000200. Instead, the existing lines
retain their numbers and the new line is given line number 000110.
Therefore, if the original line number increments are important to you, AUTONUM renumbers your lines
automatically so that the original increments are maintained.
Autonum mode is saved in the edit profile.
Return codes
0
Normal completion
20
Severe error
Examples
To turn autonum mode on:
ISREDIT AUTONUM ON
or
AUTONUM
302  z/OS: z/OS ISPF Edit and Edit Macros

## Page 335

ISREDIT AUTONUM = ON
To turn autonum mode off:
ISREDIT AUTONUM OFF
or
ISREDIT AUTONUM = OFF
AUTOSAVE—Set or Query Autosave Mode
The AUTOSAVE macro command sets autosave mode, which controls whether changed data is saved
when you issue the END command.
The AUTOSAVE assignment statement either sets autosave mode, or retrieves the current setting of
autosave mode and places it in variables.
Syntax
ISREDIT AUTOSAVE
ON
PROMPT
OFF
PROMPT
NOPROMPT
ON
Turns autosave mode on. When you enter END, any changed data is saved.
OFF PROMPT
Turns autosave mode off with the PROMPT operand. You are notified that changes have been made
and to use either SAVE (followed by END) or CANCEL. If you specify only the PROMPT keyword, OFF is
implied.
OFF NOPROMPT
Turns autosave mode off with the NOPROMPT operand. You are not notified and the data is not saved
when you issue an END command. END becomes an equivalent to CANCEL. Use the NOPROMPT
operand with caution.
ISREDIT ( var1, var2)  = AUTOSAVE
ISREDIT AUTOSAVE  = 
ON
PROMPT
OFF
PROMPT
NOPROMPT
var1
The name of a variable to contain the setting of autosave mode, either ON or OFF.
var2
The name of a variable to contain the prompt value, PROMPT or NOPROMPT.
ON
Same as macro command syntax.
AUTOSAVE
Chapter 11. Edit macro commands and assignment statements  303

## Page 336

OFF PROMPT
Same as macro command syntax.
OFF NOPROMPT
Same as macro command syntax.
Description
Data is considered changed if you have operated on it in any way that could cause a change. Shifting a
blank line or changing a name to the same name does not actually alter the data, but the editor considers
this data changed. When you enter SAVE, the editor resets the change status.
Autosave mode, along with the PROMPT operand, is saved in the edit profile.
See the DATA_CHANGED, CANCEL, and END macro commands, and the CANCEL and END primary
commands for more information on saving data.
Return codes
0
Normal completion
4
OFF NOPROMPT specified
20
Severe error
Examples
To turn autosave mode on:
ISREDIT AUTOSAVE ON
or
ISREDIT AUTOSAVE = ON
To turn autosave mode off and have the editor prompt you to use the SAVE or CANCEL command:
ISREDIT AUTOSAVE OFF
or
ISREDIT AUTOSAVE = OFF
To turn autosave mode off and not have the editor prompt you to use SAVE or CANCEL:
ISREDIT AUTOSAVE OFF NOPROMPT
or
ISREDIT AUTOSAVE = OFF NOPROMPT
BLKSIZE—Query the Block Size
The BLKSIZE assignment statement returns the block size of the data being edited in a specified variable.
BLKSIZE
304  z/OS: z/OS ISPF Edit and Edit Macros

## Page 337

Syntax
ISREDIT ( varname)  = BLKSIZE
varname
The name of a variable to contain the block size of the data being edited. The block size is a 6-digit
value that is left-padded with zeros.
Return codes
0
Normal completion
12
Syntax Error
20
Severe error
Note: For a z/OS UNIX file, the BLKSIZE assignment statement returns a value of 0.
Examples
To find the block size and continue processing if the block size is greater than 800:
ISREDIT (BSIZE) = BLKSIZE
IF &BSIZE > 000800 THEN -
   …
BOUNDS—Set or Query the Edit Boundaries
The BOUNDS macro command sets the left and right boundaries and saves them in the edit profile.
The BOUNDS assignment statement sets or retrieves the left and right boundaries and places the values
in variables.
Syntax
ISREDIT BOUNDS
BOUND
BNDS
BND
BOU
left_col
*
right_col
*
left_col
The left boundary column to be set.
right_col
The right boundary column to be set.
ISREDIT ( var1, var2)  = BOUNDS
ISREDIT BOUNDS  = 
left_col
*
right_col
*
BOUNDS
Chapter 11. Edit macro commands and assignment statements  305

## Page 338

var1
A variable containing the left boundary. If the variable is VDEFINEd in character format, it should be
defined with a length of 5. The returned value is left padded with zeros. For compatibility with earlier
releases of ISPF, a length of 3 or 4 is allowed if no data loss will result.
var2
A variable containing the right boundary. If the variable is VDEFINEd in character format, it should be
defined with a length of 5. The returned value is left padded with zeros. For compatibility with earlier
releases of ISPF, a length of 3 or 4 is allowed if no data loss will result.
left_col
Same as macro command syntax.
right_col
Same as macro command syntax.
Description
The BOUNDS macro command provides an alternative to setting the boundaries with the BOUNDS line
command or primary command; the effect on the member or data set is the same.
The column numbers are always data column numbers (see “Referring to column positions” on page
106). Thus, for a variable format data set with number mode on, data column 1 is column 9 in the record.
See “Edit boundaries” on page 23 for more information, including tables that show commands affected by
bounds settings and default bounds settings for various types of data sets.
Return codes
0
Normal completion
4
Right boundary greater than default, default right boundary used
12
Invalid boundaries specified
20
Severe error
Examples
To set the boundaries to their default values, type:
ISREDIT BOUNDS
To set one boundary while leaving the other value unchanged, type an asterisk (*) for the boundary to be
unchanged. For example, to set the left boundary from the variable &LEFT, and leave the right boundary
unchanged, type:
ISREDIT BOUNDS &LEFT *
To set the left boundary to 1, leaving the right boundary unchanged:
ISREDIT BOUNDS = 1 *
To save the value of the left boundary in the variable &LEFT:
ISREDIT (LEFT) = BOUNDS
To save the value of the right boundary in the variable &RIGHT:
BOUNDS
306  z/OS: z/OS ISPF Edit and Edit Macros

## Page 339

ISREDIT (,RIGHT) = BOUNDS
To evaluate numbers for bounds when NUMBER COBOL is on, or NUMBER is on for a variable blocked data
set:
/* REXX - Set physical bounds in a macro.  Input is 2 column       */
/*        numbers and result is bounds set on that physical column */
/*        regardless of number setting.  Bounds will not be set    */
/*        within line number areas.  This sample has minimal       */
/*        error checking.                                          */
Address isredit
'MACRO (LEFT,RIGHT)'                  /* Take left and right bounds*/
'(NUMBER,COBOL) = NUMBER'             /* Get number status         */
Parse Var cobol . cobol .             /* Get just left status      */
'(RECFM) = RECFM'                     /* Get record format         */
'(DW) = DATA_WIDTH'                   /* Get data width            */
If left='' Then left = 1              /* Assume col 1 for left     */
If right='' Then right = dw           /* Assume datawidth for right*/
shift = 0                             /* Assume no left seq numbers*/
If cobol='COBOL' Then                 /* If numbered as cobol      */
  shift = 6                           /*   Account for sequence num*/
Else If number='ON' & recfm='V' Then  /* If numbered variable block*/
  shift = 8                           /*   Account for sequence num*/
right = max(1,right - shift)          /* Adjust right column       */
right = min(right,dw)                 /* Adjust right column       */
left  = max(1,left  - shift)          /* Adjust left column        */
left  = min(left ,dw)                 /* Adjust left column        */
'BOUNDS 'min(left,right) max(left,right) /* Issue bounds command   */
'PROFILE'
BROWSE—Browse from within an Edit Session
The BROWSE macro command allows you to browse a member of the same partitioned data set during
your current edit session.
Syntax
ISREDIT BROWSE member
member
A member of the library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
Description
Your initial edit session is suspended until the browse session is complete.
To exit from the browse session, END or CANCEL must be processed by a macro or entered by you. The
current edit session resumes.
For more information on using the BROWSE service, refer to the z/OS ISPF Services Guide.
Return codes
0
Normal completion
12
Your error (invalid member name, recovery pending)
20
Severe error
BROWSE
Chapter 11. Edit macro commands and assignment statements  307

## Page 340

Examples
To browse the member OLDMEM in your current ISPF library:
ISREDIT BROWSE OLDMEM
BUILTIN—Process a Built-In Command
The BUILTIN macro command is used within an edit macro to process a built-in edit command, even if a
macro or macro statement with the same name has been defined.
Syntax
ISREDIT BUILTIN cmdname
cmdname
The built-in command to be processed.
Description
If you create a macro named MACEND and enter a DEFINE END ALIAS MACEND command, your MACEND
macro runs when you enter END. Within the MACEND macro you can perform logic and use a built-in END
command to actually end the edit session.
Note that if END is issued in your MACEND macro without being preceded by BUILTIN, the MACEND
macro would run again, resulting in an infinite loop.
Return codes
n
Return code from the built-in command
20
Severe error
Examples
To process the built-in END command:
ISREDIT BUILTIN END
To process the built-in CHANGE command:
ISREDIT BUILTIN CHANGE ALL " " "-"
CANCEL—Cancel Edit Changes
The CANCEL macro command ends your edit session without saving any of the changes you have made.
Syntax
ISREDIT CANCEL
BUILTIN
308  z/OS: z/OS ISPF Edit and Edit Macros

## Page 341

Description
CANCEL is especially useful if you have changed the wrong data, or if the changes themselves are
incorrect. See the DATA_CHANGED, AUTOSAVE, and END commands for more information about saving
data.
Note:
1. If you issue SAVE and later issue CANCEL, the changes you made before issuing SAVE are not
canceled.
2. When CANCEL is entered in the macro field in the edit prompt panel (ISRUEDIT), the macro name
is not saved in the profile for use in future sessions. This is to avoid having the editor appear to do
nothing when it is invoked from the data set list.
CANCEL does not cause automatic recording in the ISPF list data set, regardless of the setting of the
autolist mode.
Return codes
0
Normal completion
20
Severe error
Examples
To cancel the current edit session:
ISREDIT CANCEL
CAPS—Set or Query Caps Mode
The CAPS macro command sets caps mode, which controls whether alphabetic data that you type at the
terminal is automatically converted to uppercase during edit operations.
The CAPS assignment statement either sets caps mode or retrieves the setting of caps mode and places it
in a variable.
Syntax
ISREDIT CAPS
ON
OFF
ON
Turns caps mode on.
OFF
Turns caps mode off.
Assignment statement syntax
ISREDIT ( varname)  = CAPS
ISREDIT CAPS  = 
ON
OFF
CAPS
Chapter 11. Edit macro commands and assignment statements  309

## Page 342

varname
The name of a variable containing the setting of caps mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Description
When the editor retrieves data, it sets the caps mode on if the data contains all uppercase letters, or off if
the data contains lowercase letters. The editor displays a message when the caps mode changes.
Caps mode is saved in the edit profile. To override the automatic setting of caps mode, you can include the
CAPS command in an initial macro.
Caps mode is normally on for program development work. When caps mode is set to on, any alphabetic
data that you type, plus any other alphabetic data that already exists on that line, is converted to
uppercase when you press Enter or a function key.
Caps mode is normally off when you edit text documentation. When caps mode is set to off, any
alphabetic data that you type remains just as you typed it. If you typed it in uppercase, it stays in
uppercase; if you typed it in lowercase, it stays in lowercase. Also, alphabetic data that is already typed on
that line is not affected.
CAPS does not apply to DBCS fields in formatted data or to DBCS fields in mixed fields. If you specify
CAPS, the DBCS fields remain unchanged. See the LC (lowercase) and UC (uppercase) line commands and
the CAPS primary command for more information about changing cases.
Return codes
0
Normal completion
20
Severe error
Examples
To save the value of caps mode in variable &CAPMODE:
ISREDIT (CAPMODE) = CAPS
To turn caps mode OFF:
ISREDIT CAPS = OFF
To set the value of caps mode from variable &CAPMODE:
ISREDIT CAPS &CAPMODE
CHANGE—Change a Search String
The CHANGE macro command changes one search string into another.
CHANGE
310  z/OS: z/OS ISPF Edit and Edit Macros

## Page 343

Syntax
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
string1
The search string you want to change. See “Finding, seeking, changing, and excluding data” on page
44.
Note: For edit macros written in CLIST, strings that contain an open comment delimiter (/*) must be
placed within quotes within the &STR() such as &STR('/*XXX'). The maximum allowable length of the
string is 256 bytes. If you are specifying a hex string, the maximum is 128 hexadecimal characters.
string2
The string you want to replace string1. The maximum allowable length of the string is 256 bytes. If
you are specifying a hex string, the maximum is 128 hexadecimal characters. See “Finding, seeking,
changing, and excluding data” on page 44.
labela, labelb
Labels identifying the start and end of the group of lines CHANGE searches.
If the cursor is currently placed above the start label and the PREV occurrence of a string is requested,
or the cursor is currently placed below the end label and the NEXT occurrence of a string is requested,
the process returns a return code of 4 and the string is not found, even if it exists within the label
range.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
NEXT
Starts at the first position after the current cursor location and searches ahead to find the next
occurrence of string1.
ALL
Starts at the top of the data and searches ahead to find all occurrences of string1.
FIRST
Starts at the top of the data and searches ahead to find the first occurrence of string1.
LAST
Starts at the bottom of the data and searches backward to find the last occurrence of string1.
PREV
Starts at the current cursor location and searches backward to find the previous occurrence of string1.
CHARS
Locates string1 anywhere the characters match.
PREFIX
Locates string1 at the beginning of a word.
SUFFIX
Locates string1 at the end of a word.
CHANGE
Chapter 11. Edit macro commands and assignment statements  311

## Page 344

WORD
Locates string1 when it is delimited on both sides by blanks or other non-alphanumeric characters.
X
Scans only lines that are excluded from the display.
NX
Scans only lines that are not excluded from the display.
start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
The first column to be included in the range of columns CHANGE is to search.
right_col
The last column to be included in the range of columns CHANGE is to search.
Note: For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
Description
CHANGE is often used with FIND, EXCLUDE, and SEEK, and the CHANGE_COUNTS assignment statement.
To change the next occurrence of "ME" to "YOU" without specifying any other qualifications, include this
command in an edit macro:
ISREDIT CHANGE ME YOU
This command changes only the next occurrence of the letters "ME" to "YOU". Since no other
qualifications were specified, the letters "ME" can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• In an excluded line or a non-excluded line
• Anywhere within the current boundaries
To change the next occurrence of "ME" to "YOU", but only if the letters are uppercase, include this
command in an edit macro:
ISREDIT CHANGE C'ME' YOU
This type of change is called a character string change (note the C that precedes the search string)
because it changes the next occurrence of the letters "ME" to "YOU" only if the letters are found in
uppercase. However, since no other qualifications were specified, the change occurs no matter where the
letters are found, as outlined in the preceding list.
When you would like to issue CHANGE, but you are unsure of the exclude status of a line, you can use the
XSTATUS assignment statement with SEEK. First, find the particular line with SEEK. Then, determine the
exclude status with the XSTATUS assignment statement. Use CHANGE to change the string; and finally,
reset the exclude status with another XSTATUS assignment statement. For example:
ISREDIT SEEK ABC
DO WHILE &LASTCC=0
  ISREDIT (X) = XSTATUS .ZCSR
  ISREDIT CHANGE ABC DEF .ZCSR .ZCSR
  ISREDIT XSTATUS .ZCSR = &X
  ISREDIT SEEK ABC
END
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
CHANGE
312  z/OS: z/OS ISPF Edit and Edit Macros

## Page 345

Return codes
0
Normal completion
4
String not found
8
Change error. string2 is longer than string1 and substitution was not performed on at least one
change.
12
Inconsistent parameters. The string to be found does not fit between the specified columns.
20
Severe error
Examples
Before changing the current member name, put it into a variable name such as MEMNAME. To add an
identifier to that name, if it is in columns 1 to 10 and lies within the first line and the line labeled .XLAB:
ISREDIT (MEMNAME) = MEMBER
ISREDIT CHANGE WORD &MEMNAME "MEMBER:&MEMNAME" 1 10 .ZFIRST .XLAB
CHANGE_COUNTS—Query Change Counts
The CHANGE_COUNTS assignment statement retrieves values set by the most recently processed
CHANGE command and places these values in variables.
Syntax
ISREDIT ( var1, var2)  = CHANGE_COUNTS
var1
The name of a variable to contain the number of strings changed. It must be an 8-character value that
is left-padded with zeros.
var2
The name of a variable to contain the number of strings that could not be changed. It also must be an
8-character value that is left-padded with zeros.
Return codes
0
Normal completion
20
Severe error
Examples
To put the number of changes resulting from the most recent CHANGE command into the variable
&CHGED:
ISREDIT (CHGED) = CHANGE_COUNTS
To put the number of change errors into variable &ERRS:
ISREDIT (,ERRS) = CHANGE_COUNTS
CHANGE_COUNTS
Chapter 11. Edit macro commands and assignment statements  313

## Page 346

To put the number of changes and change errors into variables &CHG and &ERR:
ISREDIT (CHG,ERR) = CHANGE_COUNTS
COMPARE—Edit Compare
The COMPARE command compares the file you are editing with an external sequential data set, member
of a partitioned data set, or z/OS UNIX file. Lines that exist only in the file being edited are marked, and
lines that exist only in the file being compared are inserted as information lines in the file being edited.
The command operates as a primary command or an edit macro.
If you compare the file you are editing with a member of a PDSE version 2 data set that is configured for
member generations, the current generation of the member is used for the comparison.
You can use the Delete and Make Data line commands to merge changes between files that are being
compared.
The COMPARE function supports all line lengths, but some SuperC options are ignored for line lengths
greater than 256 characters long.
When you are editing a cataloged data set, explicit data set names refer to cataloged data sets. However,
if you are editing an uncataloged data set, explicit member names refer to cataloged data sets, but if you
specify only a member name, COMPARE searches for the member in the current uncataloged data set. For
example, if you are editing an uncataloged data set called "userid.TEMP", the command
COMPARE TEMP
first looks for member TEMP in the current, uncataloged data set, then looks for a cataloged data set
named TEMP (TSO prefix rules apply). If it finds data set TEMP, and the data set being edited is a PDS
member, then the same named member is searched for in data set TEMP.
Use of COMPARE when editing concatenations that contain uncataloged data sets is not supported and
can lead to unpredictable results.
If you have made changes to the data before issuing the COMPARE command, the COMPARE command
uses the current contents of the edit session during the comparison. Because COMPARE does not require
the data to be saved on disk, you can use the COMPARE command from EDIF, VIIF, or EDIREC sessions.
However, COMPARE NEXT and COMPARE SESSION are not supported in EDIF, VIIF, or EDIREC sessions.
Syntax
ISREDIT COMPARE dsname
VOL( volser)
NEXT
SESSION
 * 
EXCLUDE
SAVE SYSIN
( supercdsname )
(/)
dsname
The name of a member, data set, or z/OS UNIX file to which the current file is compared. This variable
can be specified as a fully qualified data set name (in quotation marks), a partially qualified data
set name, a member name, or a path name. (Also, see “Specifying z/OS UNIX pathnames with edit
primary and macro commands” on page 15.)
COMPARE
314  z/OS: z/OS ISPF Edit and Edit Macros

## Page 347

If you specify only a member name, it can be preceded by a left parenthesis symbol. The right
parenthesis is allowed but not required. The current edit session must be of a member of a partitioned
data set. The current edit concatenation is searched for the member to compare.
If you specify only a data set name and the current file is a member of a PDS, then the specified data
set is searched for a member of the same name as the member being edited.
VOL(volser)
Used when comparing against an uncataloged data set. Specifies the volser of the volume containing
the uncataloged data set.
NEXT
Specifies to do a comparison between the currently edited member and the next member of the same
name found at a higher level of the hierarchy (or next level of the edit concatenation) than the current
member. For example, if the current member is found in the third level of the concatenation, and a
like-named member exists at the fourth level, then the third and fourth level members are compared.
After data is saved in the lowest level, compares are done from that level upward.
SESSION
Specifies that you want to compare the changes you have made during the edit session with the copy
of the data saved on disk. Use COMPARE SESSION or COMPARE * to see the changes you have made
to the edit data since the beginning of the edit session or since the last SAVE command.
*
Same as SESSION.
EXCLUDE
Specifies that all matching lines in the compared data sets are excluded from the display except for
a specified number of lines above and below the differences. The differences themselves are also
shown in the display. The specified number of lines that are shown is set on the Edit Compare Settings
and/or Command Parameters panel. If you do not specify a new number for this edit session, then
whatever was the last number set is still valid. To change this number, issue the COMPARE command
with no operand and change the EXCLUDE field on the Edit Compare Settings and/or Command
Parameters panel. Valid numbers are 0 through 12, inclusive. You cannot display the Edit Compare
Settings and/or Command Parameters panel from a macro.
You can also use the COMPARE EXCLUDE command at any time to exclude all lines in a file except
lines with line labels and information lines, and the lines above and below those lines. When you
specify EXCLUDE without a data set name or NEXT, no comparison is done. Instead the labels and
information lines that already exist in the file are used to exclude functions. See “Examples” on page
316 for a macro that uses this technique.
SAVE
Specifies that SuperC (which performs the actual compare function) create a listing. The listing is
saved in a data set with one of these names:
• tsopref.EDIT.COMPARE.LIST (where tsopref is your TSO prefix).
• tsopref.userid.EDIT.COMPARE.LIST (where userid is your TSO user ID and it does not match your
TSO prefix).
• userid.EDIT.COMPARE.LIST (where no TSO prefix is defined in your TSO user profile).
Note: If the ISPF configuration table keyword USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER keyword is
included before the EDIT qualifier.
The save function is intended for debugging purposes, but it also provides a way to create a SuperC
listing. The listing produced is a Change listing (option CHNGL). No notification is given regarding
successful creation of the listing, and errors allocating the listing do not cause the comparison to end.
Note: Because of the way the SuperC comparison is done, the file currently being edited is shown in
the SuperC listing as the old file, and the file to which the current file is being compared is listed as the
new file. Therefore, insertions refer to lines that are not in the current file, and deletions refer to lines
that are only in the current file.
COMPARE
Chapter 11. Edit macro commands and assignment statements  315

## Page 348

SYSIN
Specifies not to free the ddname SYSIN before calling SuperC to compare files. This enables you to
pass SuperC Process Statements to alter the comparison. No validation is done on the type of SYSIN
allocation or the contents of the data set.
supercdsname
The name of a data set containing SuperC process statements.
/
Displays the Edit Compare SYSIN specification panel where you can specify the name of a data set
containing SuperC Process statements that are used for the compare. The SYSIN data set is freed
at the end of the compare.
Return codes
0
Normal completion
8
Member, data set, or z/OS UNIX file not found, or an error opening the member or data set occurred.
12
No parameters specified, or another parameter error such as not valid NEXT or member specification.
20
Severe error. SuperC, allocation, or delta file error occurred.
Examples
To compare the current file to another file called X.Y.Z and to save the SuperC output file in
EDIT.COMPARE.LIST:
   ISREDIT COMPARE X.Y.Z SAVE
To compare the current file to a member in the same partitioned data set, and exclude everything but the
context in which changes exist:
   ISREDIT COMPARE (memname) EXCLUDE
To find all of the occurrences of a string in a file and exclude lines to show the context in which the strings
were found, you can use this macro:
/* REXX - Edit macro to find a string, show only lines with the   */
/*        string and a few lines above and below found strings.   */
/*        This uses the COMPARE EXCLUDE command to perform the    */
/*        line exclude function.                                  */
/* -------------------------------------------------------------- */
Address isredit                  /*                               */
'MACRO (PARM)'                   /* Accept input string           */
If parm ^= '' Then               /* Do nothing if no parameters   */
  Do                             /*                               */
    'RESET LABEL'                /* Remove all existing labels    */
    'F FIRST 'parm               /* Find first string occurrence  */
    Do While(rc=0)               /*   For each occurrence         */
      'LABEL .ZCSR = 'label()' 0'/*      Assign a label to line   */
      'RFIND'                    /*      Find next occurrence     */
    End                          /*                               */
    'COMPARE X'                  /* Exclude everything except     */
                                 /*  Labels and above/below lines */
    'RESET LABEL'                /* Remove all labels             */
    '(XSTAT) = XSTATUS .ZFIRST'  /* Save exclude status of line 1 */
    'LOCATE .ZFIRST'             /* Move display to line 1        */
    'XSTATUS .ZFIRST = 'xstat    /* Restore line 1 exclude status */
  End                            /*                               */
Exit 0                           /* Always return a zero          */
/* -------------------------------------------------------------- */
label:Procedure Expose labelnum  /* Routine to generate a unique  */
If datatype(labelnum,'N')=0 Then /*   Edit line label             */
  labelnum=0                     /*                               */
COMPARE
316  z/OS: z/OS ISPF Edit and Edit Macros

## Page 349

Else                             /*                               */
  labelnum=labelnum+1            /*                               */
Return '.'translate(right(labelnum,4,'0'),'ABCDEFGHIJ','0123456789')
COPY—Copy Data
The COPY macro command copies a sequential data set, a member of a partitioned data set, or a z/OS
UNIX file into the data are editing.
Syntax
Macro command syntax
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
member
A member of the ISPF library or partitioned data set that you are editing.
dsname
A partially or fully qualified data set name. If the data set is partitioned, you must include a member
name in parentheses. If a name of eight or fewer characters is specified and it could be a member
name or a data set name, COPY searches for a member name first. If no member is found, the name is
used as a data set.
pathname
The path name for a z/OS UNIX regular file to be copied. (Also, see “Specifying z/OS UNIX pathnames
with edit primary and macro commands” on page 15.)
AFTER
The data is copied after the line with the specified label.
BEFORE
The data is copied before the line with the specified label.
label
Label identifying the line where the data is to be copied. It can be either a label that you define or one
of the editor-defined labels, such as .ZF or .ZL. 
start_line
The number of the first line of the member to be copied. Must be greater than or equal to 1, and less
than or equal to the number of lines in the member.
end_line
The number of the last line of the member to be copied. Must be greater than or equal to start_line
and less than or equal to the number of lines in the member. If not specified, the last line of the
member is used.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being copied in from the external file is converted from the
COPY
Chapter 11. Edit macro commands and assignment statements  317

## Page 350

character set designated by the keyword to the character set specified for the file being edited or to
the terminal character set.
Note: If the member name or data set name is less than 8 characters and the data set you are editing
is partitioned a like-named member is copied. If a like-named member does not exist the name is
considered to be a partially qualified data set name.
Return codes
0
Normal completion
8
End of data reached before last record read
12
Invalid label or linenum; member not found or BLDL error
16
End of data reached before first record of specified range was reached
20
Syntax error (invalid name, incomplete range), or I/O error.
Examples
To copy all of the member MEM1 at the end of the data:
ISREDIT COPY MEM1 AFTER .ZLAST
To copy all of data set MOVECOPY.DATA before the first line of data:
ISREDIT COPY MOVECOPY.DATA BEFORE .ZFIRST
To copy the first three lines of the member MEM1 before the first line of data:
ISREDIT COPY MEM1 BEFORE .ZF 1 3
CREATE—Create a Data Set or a Data Set Member
The CREATE macro command creates a member of a partitioned data set or a z/OS UNIX file from the
data you are editing. This command cannot be used to create a sequential data set. Use the Data Set
Utility (option 3.2) to allocate a sequential data set.
Syntax
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
CREATE
318  z/OS: z/OS ISPF Edit and Edit Macros

## Page 351

member
The name of the new member added to the partitioned data set currently being edited. If you are
using a concatenated sequence of libraries, the member is always written to the first library in the
sequence.
dsname(member)
The name of a different partitioned data set and new member to be added to the partitioned data set.
The data set name can be fully or partially qualified.
dsname
The name of a different sequential data set to be added. The data set name can be fully qualified or
partially qualified.
pathname
The path name for a z/OS UNIX regular file to be created. (Also, see “Specifying z/OS UNIX pathnames
with edit primary and macro commands” on page 15.)
labela, labelb
Labels identifying the start and end of the group of lines used to create the new member.
linenum1, linenum2
Relative line numbers identifying the start and end of a group of lines used to create the new member.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being saved in the external file is converted to the character
set designated by the keyword.
Description
CREATE adds a member to a partitioned data set only if a member with the same name does not already
exist. Use REPLACE if the member already exists.
Return codes
0
Normal completion
8
Member already exists, member not created
12
Invalid label or relative line number. The referenced line does not exist in the file.
Or
The specified data set does not exist.
20
Syntax error (invalid name or incomplete label or relative line number range), or I/O error.
Examples
To create a new 10-line member from the first 10 lines of the member being edited:
ISREDIT CREATE MEM1 1 10
CURSOR—Set or Query the Cursor Position
CURSOR
Chapter 11. Edit macro commands and assignment statements  319

## Page 352

The CURSOR assignment statement either sets the column number and relative line number of the cursor
location within the data, or retrieves the column number and relative line number of the cursor location
within the data and places them in variables.
Syntax
ISREDIT ( var1, var2)  = CURSOR
ISREDIT CURSOR  = linenum
label col
var1
The name of a variable containing the line number. The line number is a 6-digit value that is left-
padded with zeros. It is the ordinal number (not the sequence number) of the line. If the variable
is VDEFINEd in character format, it should be defined with a length of 8. The returned value is
left-padded with zeros. For compatibility with previous releases of ISPF, a length of 6 or 7 is allowed in
cases where no data loss will occur.
var2
The name of a variable containing the data column number. The data column number is a 3-digit
number that is left-padded with zeros. If the variable is VDEFINEd in character format, it should
be defined with a length of 5. The returned value is left padded with zeros. For compatibility with
previous releases of ISPF, a length of 3 or 4 is allowed in cases where no data loss will occur.
The columns are numbered starting with 1 at the first data column. If the cursor is in the command
line, the cursor value is column 0 of the first data line on the panel; the value is column 0 if the cursor
is in the line command field. When you retrieve the cursor position in an empty member, the line
number and column number are both set to 0.
linenum
The relative line number of the line on which the cursor is to be located. Make sure when you set the
cursor to a line number that the line number exists.
label
The label of the line on which the cursor is to be located.
Note: If you try to use a label that has not been assigned, you receive a return code of 20. To avoid
this, use the LINENUM assignment statement.
ISREDIT (X) = LINENUM .LABEL
When using the LINENUM statement, a return code of 8 is issued if the label does not exist.
col
The data column number where the cursor is to be located.
If the column number is beyond the end of the data area when setting the cursor, the cursor is
positioned to the next line, which is equivalent to the first position of the line command field.
Description
The position of the cursor shows the starting or ending location for the SEEK, FIND, CHANGE, and
EXCLUDE commands. It is also used as the text split point for TSPLIT. See “Referring to column positions”
on page 106 for more information on how the column number is determined.
When you run a macro, the cursor value is the cursor position on the panel at run time.
Note: To position the cursor on the command line, issue a return code of 1 from the macro. For example,
in CLIST code EXIT CODE(1) as the last statement in your EDIT MACRO to position the cursor on the
command line.
CURSOR
320  z/OS: z/OS ISPF Edit and Edit Macros

## Page 353

These statements can change the cursor position:
CHANGE     CURSOR   EXCLUDE
FIND       SEEK     TSPLIT
USER_STATE
Table 20 on page 321 shows the line and column numbers returned, depending on the location of the
cursor.
Table 20. Cursor position
If the CURSOR location is: The LINE number is: The COLUMN number is:
Command line First display line 0
Line number field Line by the cursor 0
Left sequence number (the
sequence number is on the left
of the data when number mode
is on)
Line by the cursor 0
Right sequence number Line by the cursor Column by the cursor
Left or right of the bounds Line by the cursor Column by the cursor
Data within the bounds Line by the cursor Column by the cursor
Insert blank space Line above the cursor. If the cursor
is at the top of the panel, then the
line number returned is the line below
the cursor and the column number is
column 0.
Column by the cursor
Non-data line and its line
command field (above the last
data line)
Line below the non-data line. 0
Non-data line (below the last
data line)
Line number of the last line of data Width of the last line of data
plus 1
Return codes
0
Normal completion
4
Column number beyond data, line number incremented
12
Invalid line number
20
Severe error
Examples
To put the line number of the current cursor position into variable &LINE:
ISREDIT (LINE) = CURSOR
To set the cursor position to data line 1, column 1:
ISREDIT CURSOR = 1 1
CURSOR
Chapter 11. Edit macro commands and assignment statements  321

## Page 354

To set the cursor position to column 1 of the last data line:
ISREDIT CURSOR = .ZLAST 1
To set the cursor position to the line with the label .LAB, without changing the column position:
ISREDIT CURSOR = .LAB
CUT—Cut and Save Lines
The CUT macro command saves lines to one of eleven named clipboards for later retrieval by the PASTE
command. The lines can be appended to lines already saved by a previous CUT command or the lines can
replace the existing contents of a clipboard.
Syntax
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
labela, labelb
Labels identifying the start and end of the group of lines in the current member that are to be added
to, or replace, data in the clipboard.
linenum1, linenum2
Relative line numbers identifying the start and end of a group of lines in the current member that are
to be added to, or replace, data in the clipboard.
clipboard_name
The name of the clipboard to use. If you omit this parameter, the ISPF default clipboard (named
DEFAULT) is used. You can define up to ten additional clipboards. The size of the clipboards and
number of clipboards might be limited by installation defaults.
X|NX
Specify X to cut only lines that are excluded from the display. Specify NX to cut only lines that are not
excluded from the display. The default is to cut all lines in the range (both excluded and nonexcluded
lines) to the clipboard.
REPLACE|APPEND
Specify REPLACE to replace existing data in the clipboard. If you do not specify REPLACE, the lines in
the current CUT are added to the end of the existing data within the clipboard.
If you specify APPEND, you add the data to the clipboard. This is the default.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being placed in the clipboard is converted to the character
set designated by the keyword and tagged as being in the designated character set.
CUT
322  z/OS: z/OS ISPF Edit and Edit Macros

## Page 355

Description
CUT saves copies of lines from an edit session to a clipboard for later retrieval by the PASTE command.
The lines are copied from the session to the named clipboard. Lines are specified by label names on the
CUT command. The edit macro CUT command always copies lines to the clipboard and does not delete
them from the edit session.
If you specify a clipboard name, lines are copied to that clipboard. If the specified clipboard does not
yet exist, it is created. ISPF provides a default clipboard named DEFAULT. You can use up to 10 other
clipboards that you define. The defined clipboards exist as long as you are logged on to TSO and are
deleted when you log off.
Return codes
0
Normal completion
12
Parameter error. Insufficient storage, or no more clipboards available.
20
Severe error
Examples
To save all the lines in the current file to the default clipboard, appending them to lines already in the
clipboard:
ISREDIT CUT .ZFIRST .ZLAST
To save all the lines in the current file to a clipboard named USERC1, replacing any lines already in the
clipboard:
ISREDIT CUT .ZFIRST .ZLAST USERC1 REPLACE
DATA_CHANGED—Query the Data Changed Status
The DATA_CHANGED assignment statement retrieves the current data-changed status and places it in a
variable.
Syntax
ISREDIT ( varname)  = DATA_CHANGED
varname
The name of a variable containing the data-changed status, either YES or NO. The data-changed
status is initially set to NO at the beginning of an edit session, and is reset to NO whenever a save is
done. If you change data on your screen, but issue the END command, the data-changed status is still
NO. When data is changed, or if a command is issued which might have changed the data, the changed
status is set to YES.
Description
This command returns information about whether the data might have changed. However, it does not
specify whether data is saved when the END command is issued. Data can be saved without being
changed if there is a change to the version, number, stats, or pack mode. When DATA_CHANGED returns a
value of NO, an 8 character variable called ZEDSAVE is set to indicate whether the data is saved. ZEDSAVE
DATA_CHANGED
Chapter 11. Edit macro commands and assignment statements  323

## Page 356

will contain either "SAVE " or "NOSAVE". See AUTOSAVE, CANCEL, SAVE and END for more information
about saving data.
Return codes
0
Normal completion
20
Severe error
Examples
To determine whether data has been changed and, if it has, to issue the built-in SAVE command:
ISREDIT (CHGST) = DATA_CHANGED
IF &CHGST = YES THEN ISREDIT BUILTIN SAVE
DATA_WIDTH—Query Data Width
The DATA_WIDTH assignment statement retrieves the current logical data width and places it in a
variable.
Syntax
ISREDIT ( varname)  = DATA_WIDTH
varname
The name of a variable to contain the logical data width. The logical data width is a 3-digit value that
is left-padded with zeros. If the variable is VDEFINEd in character format, it should be defined with a
length of 5. The returned value is left padded with zeros. For compatibility with previous releases of
ISPF, a length of 3 or 4 is allowed in cases where no data loss occurs.
Description
The logical data width is the maximum space, in bytes, that is available for data only. It does not include
any COBOL or sequence number fields or, for variable-length records, the 4-byte record descriptor word
(RDW).
The value returned by the DATA_WIDTH assignment statement depends on the record format (fixed or
variable) and the setting of number mode, as shown in Table 21 on page 324. See “NUMBER—Generate
Sequence Numbers” on page 258 if you need more information about number mode.
Table 21. Data width return value
Number mode setting Logical data width for fixed-
length records
Logical data width for variable-
length records
OFF LRECL LRECL - 4
ON STD LRECL - 8 LRECL - 12
ON COB LRECL - 6 N/A “1” on page 324
ON STD COB LRECL - 14 N/A “1” on page 324
Note:
1. COBOL numbering is invalid for variable-length records.
DATA_WIDTH
324  z/OS: z/OS ISPF Edit and Edit Macros

## Page 357

Use the LRECL assignment statement to get the maximum space, in bytes, that is available for data,
COBOL number fields, and sequence number fields.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
Examples
To put the data width in variable &MAXCOL and override the boundary setting for SEEK:
ISREDIT (MAXCOL) = DATA_WIDTH
ISREDIT SEEK 1 &MAXCOL &ARGSTR
DATAID—Query Data ID
The DATAID assignment statement retrieves the data ID for the data set currently being edited and places
it in a variable.
Syntax
ISREDIT ( varname)  = DATAID
varname
The name of a variable containing the data ID of the data set currently allocated for editing.
Description
The data ID is created by the LMINIT service to identify a data set.
If you begin an edit session with a data ID, the data ID is returned when you issue this command. If you
begin an edit session without a data ID, then an LMINIT service obtains a data ID and returns it. On return
from a top-level macro, the editor releases any data ID it has obtained.
For further information about the use of library access services, refer to the z/OS ISPF Services Guide.
Return codes
0
The data ID returned was passed to the editor
4
Data ID was generated by and is freed by the editor
8
A previously generated data ID was returned
20
Severe error
DATAID
Chapter 11. Edit macro commands and assignment statements  325

## Page 358

Examples
To store the data ID in variable &DID, and then find the member MEM1 of that data set by using the
LMMFIND library access service:
ISREDIT (DID) = DATAID
ISPEXEC LMMFIND DATAID(DID) MEMBER(MEM1)
IF &LASTCC = 0 THEN ...
DATASET—Query the Current and Original Data Set Names
The DATASET assignment statement retrieves these items and places them in selected variables:
• The name of the data set into which the data currently being edited will be stored
• The name of the data set from which the data currently being edited originated
• The library concatenation number of the originating data set
• The path name of the file (when editing a z/OS UNIX file)
Syntax
ISREDIT ( var1, var2, var3)  = DATASET
var1
The name of a variable to contain the name of the data set currently being edited. The data set name
is fully qualified without quotation marks (').
When editing a z/OS UNIX file, the path name of the file.
var2
The name of a variable to contain the name of the data set where the data currently being edited
originated from. The data set name is fully qualified without quotation marks ('). If the data currently
being edited is new, a blank is returned in this variable. If the original data is deleted, the name of the
data set where the data currently being edited originated from is still returned in this variable.
var3
The library concatenation number of the original data set. If the data currently being edited is new,
zeros are returned.
Return codes
0
Normal completion
20
Severe error
Examples
To place the name of the data set you are editing and the library concatenation number in the variables
&CURDSN and &LIBNUM:
ISREDIT (CURDSN, ,LIBNUM) = DATASET
DEFINE—Define a Name
The DEFINE macro command is used to:
DATASET
326  z/OS: z/OS ISPF Edit and Edit Macros

## Page 359

• Identify a macro that replaces a built-in command of the same name
• Identify programs that are edit macros
• Assign an alias to a macro or built-in command
• Make a macro or built-in command inoperable
• Reset an inoperable macro or built-in command
• Disable a macro or built-in command
DEFINE is often used with the BUILTIN command.
Syntax
ISREDIT DEFINE name MACRO
CMD
PGM
ALIAS name_2
NOP
RESET
DISABLED
name
The name with which you process the command.
MACRO CMD
Identifies the name that you are defining as a command language (CLIST or REXX exec) macro, which
is called in the same way as using the SELECT service CMD keyword with a percent symbol (%)
preceding the command. That means that you can specify only CLISTs or REXX EXECs.
MACRO PGM
Identifies the name that you are defining as a program (load module) macro, which is called by the
SELECT PGM service.
ALIAS name2
Identifies the name that you are defining as an alias of another name, with the same characteristics.
If name2 is already an alias, the editor replaces it with the command it names. Therefore, it is not
possible to have an alias of an alias.
NOP
Makes the name you are defining and all of its aliases inoperable until you reset them with the RESET
operand. Therefore, when the name or an alias of the name is called, nothing is processed. NOP is
similar to DISABLED, except that disabled names cannot be reset by the RESET operand.
RESET
Resets the most recent definition of the name that you are defining to the status in effect before that
definition. For example, RESET makes inoperable names operable again.
DISABLED
Makes the name that you are defining and all of its aliases disabled until you end the edit session.
Therefore, when the name or an alias of the name is called, nothing is processed. A disabled
command or macro cannot be restored by RESET.
Description
The effects of the DEFINE macro command apply only to the edit session of the member or sequential
data set being edited when the macro is run. This effect is different from the DEFINE primary command.
To temporarily override DEFINE, use BUILTIN.
Note: To define RESET as disabled, enclose it in quotes ('RESET'). If you do not use quotes, the editor
interprets RESET as a keyword.
DEFINE
Chapter 11. Edit macro commands and assignment statements  327

## Page 360

Return codes
0
Normal completion
8
RESET was attempted for a name not currently defined, or DEFINE name ALIAS name2 requested and
name2 is an NOP
12
DEFINE was attempted for a name not currently defined
20
Severe error (unknown command)
Examples
To define the name IJKDOIT as a CLIST or REXX macro:
ISREDIT DEFINE IJKDOIT MACRO
To define the name SETITUP as a program macro:
ISREDIT DEFINE SETITUP MACRO PGM
To define the name DOIT as an alias of the macro IJKDOIT:
ISREDIT DEFINE DOIT ALIAS IJKDOIT
To define the name SAVE to have no effect:
ISREDIT DEFINE SAVE NOP
To reset the definition of the name SAVE:
ISREDIT DEFINE SAVE RESET
To define the name FINDIT as disabled:
ISREDIT DEFINE FINDIT DISABLED
To create and update library statistics when data is saved, first set the stats mode on. Then make it
impossible to turn off by defining it as disabled. Note that none of the commands that are defined as
disabled can be called while you are editing a member.
ISREDIT MACRO
ISREDIT STATS ON
ISREDIT DEFINE STATS DISABLED
DELETE—Delete Lines
The DELETE macro command deletes lines from the data you are editing.
DELETE
328  z/OS: z/OS ISPF Edit and Edit Macros

## Page 361

Syntax
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
ALL
Specifies that all selected lines are deleted. The DELETE command, unlike FIND, CHANGE, and
EXCLUDE, does not use NEXT, FIRST, PREV, or LAST. ALL is required to emphasize that NEXT is not the
default.
X
Restricts the lines deleted to those that are excluded.
NX
Restricts the lines deleted to those that are not excluded.
labela, labelb
Labels identifying the start and end of the group of lines to be deleted. To delete one line, enter one
label.
linenum1
Relative line number identifying a line, or the start of a group of lines, to be deleted.
linenum2
Relative line number identifying the end of a group of lines to be deleted.
Description
DELETE can specify a single line or a range of lines. It can limit the lines to be deleted to all excluded or
non-excluded lines in the data, or to all excluded or non-excluded lines within a line pointer range.
Return codes
0
Normal (lines deleted successfully)
4
No lines deleted
8
No standard records exist
12
Invalid line number
20
Severe error
DELETE
Chapter 11. Edit macro commands and assignment statements  329

## Page 362

Examples
To delete all non-excluded lines:
ISREDIT DELETE ALL NX
To delete all lines between labels .A and .B with a blank in column 1:
ISREDIT RESET X .A .B
ISREDIT EXCLUDE ALL " " 1 .A .B
ISREDIT DELETE ALL X .A .B
To delete the last line of data in the current data set:
ISREDIT DELETE .ZLAST
To delete the first 10 lines of data in the current data set:
ISREDIT DELETE 1 10
DISPLAY_COLS—Query Display Columns
The DISPLAY_COLS assignment statement retrieves the column numbers of the first and last data
columns that you are seeing, and places them in variables.
Syntax
ISREDIT ( var1, var2)  = DISPLAY_COLS
var1
The name of a variable containing the column number of the first data column visible to you. The
column number is a 3-digit value that is left-padded with zeros. If the variable is VDEFINEd in
character format, it should be defined with a length of 5. The returned value is left padded with zeros.
For compatibility with previous releases of ISPF, a length of 3 or 4 is allowed in cases where no data
loss will occur.
var2
The name of a variable containing the column number of the last data column visible to you. The
column number is a 3-digit value that is left-padded with zeros. If the variable is VDEFINEd in
character format, it should be defined with a length of 5. The returned value is left padded with zeros.
For compatibility with previous releases of ISPF, a length of 3 or 4 is allowed in cases where no data
loss will occur.
Description
Columns that contain sequence numbers are not considered data columns. Do not use this assignment
statement in initial macros because the columns displayed are not known until the data first appears. See
“Referring to column positions” on page 106 for more information.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
DISPLAY_COLS
330  z/OS: z/OS ISPF Edit and Edit Macros

## Page 363

Examples
To put the leftmost and rightmost column values displayed to you in variables &LEFT and &RIGHT:
ISREDIT (LEFT,RIGHT) = DISPLAY_COLS
DISPLAY_LINES—Query Display Lines
The DISPLAY_LINES assignment statement retrieves the relative line numbers of the first and last data
lines that would appear at this point if the macro ended, and places them in variables. Other non-data
lines might be on the display. Do not use this assignment statement in an initial macro because the lines
displayed are not known until the data is first displayed.
Syntax
ISREDIT ( var1, var2)  = DISPLAY_LINES
var1
The name of a variable containing the relative line number of either the first visible data line or block
of excluded lines if the macro ended at this point. The relative line number is a 6-digit value that is
left-padded with zeros. If the variable is VDEFINEd in character format, it should be defined with a
length of 8. The returned value is left-padded with zeros. For compatibility with previous releases of
ISPF, a length of 6 or 7 is allowed in cases where no data loss will occur.
var2
The name of a variable containing the relative line number of either the last visible data line or block
of excluded lines. The relative line number is a 6-digit value that is left-padded with zeros. If the
variable is VDEFINEd in character format, it should be defined with a length of 8. The returned value is
left-padded with zeros. For compatibility with previous releases of ISPF, a length of 6 or 7 is allowed in
cases where no data loss will occur.
Return codes
0
Normal completion
4
No visible data lines
8
No existing data lines
12
Invalid command format
20
Severe error
Examples
To place the top and bottom line numbers in variables &TOP and &BOT:
ISREDIT (TOP,BOT) = DISPLAY_LINES
DOWN—Scroll Down
The DOWN macro command scrolls data down from the current panel position.
DISPLAY_LINES
Chapter 11. Edit macro commands and assignment statements  331

## Page 364

Syntax
ISREDIT DOWN amt
amt
The number of lines (0-9999) to scroll, or one of these operands:
MAX
Scrolls to the end of data in the specified direction.
HALF
Displays the next sequential half panel of data.
PAGE
Displays the next sequential full panel of data.
CURSOR
Scrolls until the line on which the cursor is located becomes the first data line on the panel.
DATA
Scrolls until the last data line on the current panel of data becomes the first data line on the next
panel of data.
Description
To scroll down using the panel position when the macro was first issued, use USER_STATE assignment
statements to save and then restore the panel position operands.
When you issue DOWN, the non-data lines on the panel affect the number of lines scrolled. However, if
you define a macro named DOWN, it only overrides the DOWN command when used from another macro.
DOWN does not change the cursor position and cannot be used in an initial macro.
The actual number of lines appearing on the panel is determined by:
• The number of lines excluded from the display
• The terminal display size and split-panel line
• The number of special temporary lines appearing, such as the ==ERR>, ==CHG>, =COLS>, ======,
=PROF>, ==MSG>, =NOTE=, =BNDS>, =TABS> or =MASK> lines
The first line appearing is determined in one of two ways: (1) a LOCATE command can set the line first on
the panel, and (2) the first line to appear depends on whether the cursor was set explicitly by a CURSOR
assignment statement or implicitly by a SEEK, FIND, CHANGE, or TSPLIT command. Since the cursor must
be on the panel, the line that is the first line on the panel may be different from the line that was first
when you called the macro.
Return codes
0
Normal completion
2
No more data DOWN
4
No visible lines
8
No data to display
12
Amount not specified
20
Severe error
DOWN
332  z/OS: z/OS ISPF Edit and Edit Macros

## Page 365

Examples
To scroll down to the end of the data set:
ISREDIT DOWN MAX
To display the next half panel of data:
ISREDIT DOWN HALF
To display the next full panel of data:
ISREDIT DOWN PAGE
To make the line where the cursor is placed the first one on the display:
ISREDIT DOWN CURSOR
To display the next page less one line:
ISREDIT DOWN DATA
EDIT—Edit from within an Edit Session
The EDIT macro command allows you to edit a member of the same partitioned data set during your
current edit session.
Syntax
ISREDIT EDIT member
member
A member of the library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
Description
Editing one data set or member while you are already editing another is called recursive editing. Your
initial edit session is suspended until the second-level edit session is complete. Editing sessions can be
nested until you run out of storage.
To exit from a nested edit session, END or CANCEL must be processed by a macro or entered by you. The
current edit session resumes.
The EDIT service call, ISPEXEC EDIT, is an alternate method of recursively starting the editor. It offers the
option of editing another data set and specifying an initial macro.
For more information on using the EDIT service for recursive editing, refer to the z/OS ISPF Services Guide.
Return codes
0
Normal completion, data was saved
4
Normal completion, data was not saved
12
Your error (invalid member name, recovery pending)
EDIT
Chapter 11. Edit macro commands and assignment statements  333

## Page 366

14
Member in use
20
Severe error
28
No ISREDIT MACRO statement preceded this call, or BROWSE was substituted because of the size of
the member being edited.
Examples
To recursively edit the member OLDMEM in your current ISPF library:
ISREDIT EDIT OLDMEM
END—End the Edit Session
The END macro command ends the editing of the current sequential data set or partitioned data set
member.
Syntax
ISREDIT END
Description
If an edit macro contains an ISREDIT END statement, there can be no other ISREDIT or ISPEXEC
statements following it. If one of these kinds of statements does follow an ISREDIT END, the edit
macro ends with an error when that statement occurs. However, any other CLIST, REXX exec, or program
statements can follow an ISREDIT END statement and process normally.
If no aliases have been defined for END, the response of the editor to the END command depends on:
• Whether changes were made to the data during your current edit session
• If changes were made, whether a SAVE command was entered after the last change
• The setting of number mode, autonum mode, stats mode, autolist mode, and autosave mode in the edit
profile
• Whether you were editing a member that was an alias of another member
Note: When END is entered in the macro field in the edit prompt panel (ISRUEDIT), the macro name is not
saved in the profile for use in future sessions. This is to avoid having the editor appear to do nothing when
it is invoked from the data set list.
See “Ending an edit session” on page 11 for more information.
Return codes
0
Normal completion
4
New member saved
12
END not done, AUTOSAVE OFF PROMPT set, or Data not saved (insufficient space)
20
Severe error
END
334  z/OS: z/OS ISPF Edit and Edit Macros

## Page 367

Examples
To end the current edit session:
ISREDIT END
EXCLUDE—Exclude Lines from the Display
The EXCLUDE macro command hides lines that contain a search string from view, and replaces them with
a dashed line. To see the lines again, you enter either the RESET or RESET EXCLUDED command.
Syntax
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
string
The search string you want to exclude. See “Finding, seeking, changing, and excluding data” on page
44.
Note: For edit macros written in CLIST, strings that contain an open comment delimiter (/*) must be
placed within quotes within the &STR() such as &STR('/*XXX'). The maximum allowable length of the
string is 256 bytes. If you are specifying a hex string, the maximum is 128 hexadecimal characters.
labela, labelb
Labels identifying the start and end of the group of lines within which the EXCLUDE command is to
search.
If the cursor is currently placed above the start label and the PREV occurrence of a string is requested,
or the cursor is currently placed below the end label and the NEXT occurrence of a string is requested,
the process returns a return code of 4 and the string is not found, even if it exists within the label
range.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
NEXT
Starts at the first position after the current cursor location and searches ahead to find the next
occurrence of string.
ALL
Starts at the top of the data and searches ahead to find all occurrences of string.
FIRST
Starts at the top of the data and searches ahead to find the first occurrence of string.
LAST
Starts at the bottom of the data and searches backward to find the last occurrence of string.
PREV
Starts at the current cursor location and searches backward to find the previous occurrence of string.
EXCLUDE
Chapter 11. Edit macro commands and assignment statements  335

## Page 368

CHARS
Locates string anywhere the characters match.
PREFIX
Locates string at the beginning of a word.
SUFFIX
Locates string at the end of a word.
WORD
Locates string when it is delimited on both sides by blanks or other non-alphanumeric characters.
start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
The first column to be included in the range of columns to be searched.
right_col
The last column to be included in the range of columns to be searched.
Note: For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
Description
You can use the EXCLUDE command with the FIND and CHANGE commands to find a search string,
change it, and then exclude the line that contains the string from the panel.
To exclude the next non-excluded line that contains the letters ELSE without specifying any other
qualifications, include this command in an edit macro:
ISREDIT EXCLUDE ELSE
Since no other qualifications were specified, the letters ELSE can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• Anywhere within the current boundaries
To exclude the next line that contains the letters ELSE, but only if the letters are uppercase, include this
command in an edit macro:
ISREDIT EXCLUDE C'ELSE'
This type of exclusion is called a character string exclusion (note the C that precedes the search string)
because it excludes the next line that contains the letters ELSE only if the letters are found in uppercase.
However, since no other qualifications were specified, the exclusion occurs no matter where the letters
are found on a non-excluded line, as outlined in the previous list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Return codes
0
Normal completion
4
String not found
8
Lines not excluded
12
Inconsistent parameters
EXCLUDE
336  z/OS: z/OS ISPF Edit and Edit Macros

## Page 369

20
Severe error
Examples
This example excludes the first non-excluded line in the data set that contains the letters ELSE. However,
the letters must occur on or between lines labeled .E and .S and they must be the first four letters of a
word:
ISREDIT EXCLUDE ELSE .E .S FIRST PREFIX
This example excludes the last non-excluded line in the data set that contains the letters ELSE. However,
the letters must occur on or between lines labeled .E and .S and they must be the last four letters of a
word.
ISREDIT EXCLUDE ELSE .E
.S LAST SUFFIX
This example excludes the first non-excluded line that immediately precedes the cursor position and
that contains the letters ELSE. However, the cursor must not be positioned ahead of the lines labeled .E
and .S. Also, the letters must occur on or between the labeled lines; they must be standalone characters
(not part of any other word); and they must exist within columns 1 and 5:
ISREDIT EXCLUDE ELSE .E .S PREV WORD 1 5
EXCLUDE_COUNTS—Query Exclude Counts
The EXCLUDE_COUNTS assignment statement retrieves values set by the most recently processed
EXCLUDE command and places them in variables.
Syntax
ISREDIT ( var1, var2)  = EXCLUDE_COUNTS
var1
The name of a variable to contain the number of strings found. The number of strings is an 8-digit
value that is left-padded with zeros.
var2
The name of a variable to contain the number of lines excluded. The number of lines excluded is an
8-digit value that is left-padded with zeros.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
Examples
To determine the number of lines that contain the word BOX:
ISREDIT EXCLUDE ALL BOX
ISREDIT (,BOXLINES) = EXCLUDE_COUNTS
EXCLUDE_COUNTS
Chapter 11. Edit macro commands and assignment statements  337

## Page 370

FIND—Find a Search String
The FIND macro command locates one or more occurrences of a search string.
Syntax
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
string
The search string you want to find. See “Finding, seeking, changing, and excluding data” on page 44.
Note: For edit macros written in CLIST, strings that contain an open comment delimiter (/*) must be
placed within quotes within the &STR() such as &STR('/*XXX'). The maximum allowable length of the
string is 256 bytes. If you are specifying a hex string, the maximum is 128 hexadecimal characters.
labela, labelb
Labels identifying the start and end of the group of lines within which the FIND command is to search.
If the cursor is currently placed above the start label and the PREV occurrence of a string is requested,
or the cursor is currently placed below the end label and the NEXT occurrence of a string is requested,
the process returns a return code of 4 and the string is not found, even if it exists within the label
range.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
NEXT
Starts at the first position after the current cursor location and searches ahead to find the next
occurrence of string.
ALL
Starts at the top of the data and searches ahead to find all occurrences of string.
FIRST
Starts at the top of the data and searches ahead to find the first occurrence of string.
LAST
Starts at the bottom of the data and searches backward to find the last occurrence of string.
PREV
Starts at the current cursor location and searches backward to find the previous occurrence of string.
CHARS
Locates string anywhere the characters match.
PREFIX
Locates string at the beginning of a word.
FIND
338  z/OS: z/OS ISPF Edit and Edit Macros

## Page 371

SUFFIX
Locates string at the end of a word.
WORD
Locates string when it is delimited on both sides by blanks or other non-alphanumeric characters.
X
Scans only lines that are excluded from the display.
NX
Scans only lines that are not excluded from the display.
start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
The first column to be included in the range of columns to be searched.
right_col
The last column to be included in the range of columns to be searched.
Note: For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
Description
Use the SEEK macro command instead of FIND if you want to locate a string without changing the exclude
status of the line that contains the string.
You can use FIND with the EXCLUDE and CHANGE commands to find a search string, change it, and then
exclude the line that contains the string from the panel.
To find the next occurrence of the letters ELSE without specifying any other qualifications, include this
line in an edit macro:
ISREDIT FIND ELSE
Since no other qualifications were specified, the letters ELSE can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• In either an excluded or a non-excluded line
• Anywhere within the current boundaries
To find the next occurrence of the letters ELSE, but only if the letters are uppercase:
ISREDIT FIND C'ELSE'
This type of search is called a character string search (note the C that precedes the search string) because
it finds the next occurrence of the letters ELSE only if the letters are in uppercase. However, since no
other qualifications were specified, the letters can be found anywhere in the data set or member, as
outlined in the preceding list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Return codes
0
Normal completion
4
String not found
FIND
Chapter 11. Edit macro commands and assignment statements  339

## Page 372

12
Syntax error
20
Severe error
Examples
The example shown here finds the first occurrence in the data set of the letters ELSE. However, the letters
must occur on or between lines labeled .E and .S and they must be the first four letters of a word:
ISREDIT FIND ELSE .E .S FIRST PREFIX
The example shown here finds the last occurrence in the data set of the letters ELSE. However, the letters
must occur on or between lines labeled .E and .S; they must be the last four letters of a word; and they
must be found in an excluded line.
ISREDIT FIND ELSE .E .S LAST SUFFIX X
The example shown here finds the first occurrence of the letters ELSE that immediately precedes the
cursor position. However, the cursor must not be positioned ahead of the lines labeled .E and .S. Also, the
letters must occur on or between lines labeled .E and .S; they must be standalone characters (not part of
any other word); they must be found in a non-excluded line; and they must exist within columns 1 and 5:
ISREDIT FIND ELSE .E .S PREV WORD NX 1 5
FIND_COUNTS—Query Find Counts
The FIND_COUNTS assignment statement retrieves values that were set by the most recently entered
FIND or RFIND command, and places these values in variables.
Syntax
ISREDIT ( var1, var2)  = FIND_COUNTS
var1
The name of a variable to contain the number of strings found. The number of strings is an 8-digit
value that is left-padded with zeros.
var2
The name of a variable to contain the number of lines on which strings were found. The number of
lines on which strings were found is an 8-digit value that is left-padded with zeros.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
Examples
To find all occurrences of && in the line labeled .A and loop through and process them:
ISREDIT FIND .A .A && ALL
ISREDIT (FINDS) = FIND_COUNTS
FIND_COUNTS
340  z/OS: z/OS ISPF Edit and Edit Macros

## Page 373

DO WHILE &FINDS > 0
  ...
END
FLIP—Reverse Exclude Status of Lines
The FLIP macro command lets you reverse the exclude status of a specified range of lines or of all the
lines in a file, including data, information, message, and note lines.
Syntax
ISREDIT FLIP
label-range
labela, labelb
Labels identifying the start and end of the group of lines within which the FLIP command is to reverse
the exclude status. If one label is specified, only that labeled line is reversed.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
Return codes
0
Successful completion. The excluded status of the requested lines was reversed.
20
Severe error
Examples
These are examples of statements using the FLIP commands from an Edit macro. The actual values for .a
and .b can be defined by edit macro or by the user:
ISREDIT FLIP          /* Flip all lines                              */
ISREDIT FLIP .ZL .ZF  /* Flip all lines                              */
ISREDIT FLIP .ZF      /* Flip first line in file                     */
ISREDIT FLIP .a .b    /* Flip lines between and including .a and .b  */
ISREDIT FLIP .a       /* Flip line labeled .a                        */
FLOW_COUNTS—Query Flow Counts
The FLOW_COUNTS assignment statement retrieves values that were set by the most recently entered
TFLOW command, and places these values in variables.
Syntax
ISREDIT ( var1, var2)  = FLOW_COUNTS
var1
The name of a variable to contain the number of original lines that participated in the text flow
operation. The number of original lines is an 8-digit value that is left-padded with zeros.
var2
The name of a variable to contain the number of lines that were generated by the text flow operation.
The number of lines is an 8-digit value that is left-padded with zeros.
FLIP
Chapter 11. Edit macro commands and assignment statements  341

## Page 374

If the value in var1 is larger than the value in var2, the difference is the number of lines that were
deleted from the current data because of the text flow operation. If the value in var1 is less than the
value in var2, the difference is the number of lines that were added to the current data because of the
text flow operation.
Return codes
0
Normal completion
20
Severe error
Examples
To retrieve the value of the rightmost column displayed, allow a margin of 8 for the text flow, and then
take action if lines were added because of the text flow operation:
ISREDIT (,MAXCOL) = DISPLAY_COLS
ISREDIT TFLOW .ZCSR &EVAL(MAXCOL - 8)
ISREDIT (INLINE,OUTLIN) = FLOW_COUNTS
IF &OUTLIN > &INLINE THEN DO
  ...
HEX—Set or Query Hexadecimal Mode
The HEX macro command sets hexadecimal mode, which determines whether data appears in
hexadecimal format.
The HEX assignment statement either sets hexadecimal mode or retrieves the current values of
hexadecimal mode, and places them in variables.
Syntax
ISREDIT HEX ON
VERT
DATA
VERT
DATA
OFF
ON DATA
Displays the hexadecimal representation of the data as a string of hexadecimal characters (two per
byte) under the characters.
ON VERT
Displays the hexadecimal representation of the data vertically (two rows per byte) under each
character.
OFF
Does not display hexadecimal representation of the data.
Note: The command, HEX OFF, cancels the effect of any previous HX or HXX commands.
ISREDIT ( var1, var2)  = HEX
HEX
342  z/OS: z/OS ISPF Edit and Edit Macros

## Page 375

ISREDIT HEX  = ON
VERT
DATA
VERT
DATA
OFF
var1
The name of a variable to contain ON or OFF.
var2
The name of a variable to contain DATA, VERT, or blanks.
ON DATA
Same as macro command syntax.
ON VERT
Same as macro command syntax.
OFF
Same as macro command syntax.
Description
The HEX macro command and assignment statement determines whether the editor displays
hexadecimal representation in a vertical or data string format.
When the editor is operating in hexadecimal mode, three lines are displayed for each source line. The
first line shows the data in standard character form, while the next two lines show the same data in
hexadecimal representation.
Besides normal editing on the first of the three lines, you can change any characters by typing over the
hexadecimal representations.
You can also use the FIND, CHANGE, and EXCLUDE commands to find, change, or exclude invalid
characters or any specific hexadecimal character, regardless of the setting of hexadecimal mode. See the
discussion of picture strings and hexadecimal strings under “Finding, seeking, changing, and excluding
data” on page 44.
Return codes
0
Normal completion
20
Severe error
Examples
To put the value of hexadecimal mode (on or off) in variable &HEXMODE and to process if hexadecimal
mode is on:
ISREDIT (HEXMODE) = HEX
IF &HEXMODE = ON THEN -
   ...
To turn hexadecimal mode off:
ISREDIT HEX OFF
HEX
Chapter 11. Edit macro commands and assignment statements  343

## Page 376

HIDE—Hide Excluded Lines Message
The HIDE command removes the "n Line(s) not Displayed" messages from the display where lines have
been excluded by the EXCLUDE command.
The HIDE function has dependencies on the value of the ISPF variable ZHIDEX, panel attributes,
and extended highlighting support of the terminal. These dependencies are described in the section
"Providing customized Browse and Edit panels" in the ISPF Planning and Customisation Guide.
Syntax
ISREDIT HIDE EXCLUDE
EXCLUDED
EXC
EX
X
X
Removes each "n Line(s) not Displayed" message from the display and underscores the line number
field of the preceding line.
Return codes
0
Successful completion. Any "n Line(s) not Displayed" messages were removed from the display.
4
HIDE X not supported
20
Severe error
Examples
These statements show how to use the HIDE command from an Edit macro to hide excluded lines, then
the RESET HIDE command to display the excluded lines again:
ISREDIT HIDE X        /* Hide excluded lines                         */
ISREDIT RESET HIDE    /* Redisplay excluded lines                    */
HILITE—Enhanced Edit Coloring
HILITE is used to control the use of color in the editor by changing the settings for the enhanced color and
language-sensitive editing features.
The HILITE dialog is not available in the Edit Macro environment.
Note: Language sensitive and enhanced coloring of the edit session is only available if it is enabled by
the installer or person who maintains the ISPF product. For information on enabling the enhanced color
functions, see z/OS ISPF Planning and Customizing.
Language and logic hiliting is not supported for ASCII or UTF-8 editing sessions and the HILITE command
is not available during these edit sessions.
HIDE
344  z/OS: z/OS ISPF Edit and Edit Macros

## Page 377

The following z Hilite variables represent the values of the active edit profile and are available for
reference in the shared memory pool. These are ZHIAUTO, ZHILANG, ZHICOLOR, ZHIPAREN, ZHIFIND,
and ZHICURSR. See Dialog variables in z/OS ISPF Reference Summary for more information.
Syntax
ISREDIT HILITE
OFF
ON
NOLOGIC
LOGIC
IFLOGIC
DOLOGIC
AUTO
DEFAULT
OTHER
ASM
BOOK
C
COBOL
DTL
HTML
IDL
JCL
PANEL
PASCAL
PLI
REXX
SKEL
SUPERC
XML
MARGINS left_col
*
right_col
*
RESET
PAREN FIND CURSOR SEARCH DISABLED
ON
Sets program coloring ON and turns LOGIC coloring off.
OFF
Sets coloring OFF, with the exception of cursor highlighting.
LOGIC
LOGIC highlighting matches logical language-specific keywords in the same color. If an unmatched
closing keyword is found, such as END for PL/I or :eul. for BookMaster, it is highlighted in reverse
video pink only if HILITE LOGIC is active. When logic is being highlighted, only comments are
highlighted along with it.
Logic highlighting is available only for PL/I, PL/X, REXX, OTHER, C, SKELS, Pascal, and BookMaster.
HILITE LOGIC turns on both IFLOGIC and DOLOGIC.
Note: LOGIC highlighting can be turned off by issuing HILITE ON, HILITE NOLOGIC, or HILITE RESET
commands. Changing the HILITE language does not change the LOGIC setting.
HILITE
Chapter 11. Edit macro commands and assignment statements  345

## Page 378

IFLOGIC
Turns on IF/ELSE logic matching. IFLOGIC matches IF and ELSE statements. When IFLOGIC is
enabled, unmatched ELSE keywords are highlighted in reverse video pink.
DOLOGIC
Turns on DO/END logic matching. DOLOGIC matches logical blocks such as DO/END in PL/I or :ol/:eol
in BookMaster. For the C language, DOLOGIC matches curly braces ({ and }). C trigraphs for curly
braces are not recognized and are not supported by DOLOGIC highlighting. When DOLOGIC is
enabled, unmatched logical block terminators (such as END keywords in PL/I, :e tags in BookMaster
or right braces ( } ) in C) are highlighted in reverse video pink.
NOLOGIC
Same as ON.
AUTO
Allows ISPF to determine the language.
DEFAULT
Highlights the data in a single color.
OTHER
Highlight the data as a pseudo-PL/I language.
ASM
Highlights the data as Assembler.
BOOK
Highlights the data as BookMaster.
C
Highlights the data as C.
COBOL
Highlights the data as COBOL.
DTL
Highlights the data as Dialog Tag Language.
HTML
Highlights the data as HTML.
IDL
Highlights the data as IDL.
JCL
Highlights the data as MVS Job Control Language.
PANEL
Highlights the data as ISPF Panel Language.
PASCAL
Highlights the data as Pascal.
PLI
Highlights the data as PL/I.
REXX
Highlights the data as REXX.
SKEL
Highlights the data as ISPF Skeleton Language.
SUPERC
Highlights the data as a SuperC Listing.
XML
Highlights the data as XML.
MARGINS [left-margin | * [right-margin | * ] ]
Specifies either or both of the left-margin or right-margin parameters for languages C, PL/I, and PL/X.
The MARGINS keyword can be included on the same command that includes one of these languages.
HILITE
346  z/OS: z/OS ISPF Edit and Edit Macros

## Page 379

It cannot be specified when the language AUTO is specified, even if the language would subsequently
be determined to be C, PL/I, or PL/X.
left-margin
The left hand margin for processing the language source. The value must be within the range
as defined by the language. The maximum value is 254 for C, 100 for PL/I, and 65 for PL/X. If
left-margin exceeds the last input column or if an asterisk (*) is specified, the default left margin
is obtained from the ISPF configuration table keyword for this language (HILITE_MARGIN_C,
HILITE_MARGIN_PLI, or HILITE_MARGIN_PLX).
right-margin
The right hand margin for processing the language source. The value must be within the range
as defined by the language. The maximum value is 255 for C, 200 for PL/I, and 80 for PL/X. If
right-margin exceeds the last input column or if an asterisk (*) is specified, the default right margin
is obtained from the ISPF configuration table keyword for this language (HILITE_MARGIN_C,
HILITE_MARGIN_PLI, or HILITE_MARGIN_PLX).
RESET
Resets defaults (LANG AUTO, COLOR ON, LOGIC OFF, FIND ON and CURSOR ON).
PAREN
Toggles parenthesis matching. When parenthesis matching is active, only comments and quoted
strings are specially colored. All other code appears in the default color. Note that extra parenthesis
highlighting is always active when highlighting is active.
Parentheses within quoted strings and comments are not checked or highlighted by the parenthesis
matching function.
FIND
The HILITE FIND command toggles the highlighting color of any string that would be found by an
RFIND. The user can select the highlight color. The default is reverse video white.
Only non-picture strings are supported, and the only additional qualifiers recognized are hex strings
(X'…'), character strings (C'…'), text strings (T'…'), WORD, PREFIX and SUFFIX, and boundaries
specified in the FIND command. Hex strings may be highlighted. but non-displayable characters are
not highlighted. Default bounds and labels are ignored when FIND strings are highlighted.
Because FIND highlighting is not quite as robust at the FIND command itself, the editor may highlight
more occurrences of the FIND string than FIND would actually locate.
The RESET edit command has been enhanced, through the addition of a FIND operand, to temporarily
disable the highlighting of FIND strings until the next FIND, RFIND, CHANGE, or RCHANGE command
is issued. RESET with the FIND operand (or no operands at all), temporarily disables the highlighting
of FIND strings.
CURSOR
The CURSOR operand toggles the highlighting of the phrase that contains the cursor in a user-
selectable color. The default is white.
Cursor highlighting in Edit is performed in a manner similar to the way it is done in Browse. The entire
phrase from the previous blank to the next blank is highlighted.
SEARCH
HILITE SEARCH finds the first unmatched END, ELSE, }, or ) above the last displayed line on the panel.
If a mismatched item is found, the file is scrolled so that the mismatch is at the top of the panel. The
search for mismatches only occurs for lines above the last displayed line, so you may need to scroll to
the bottom of the file before issuing the HI SEARCH command.
Search is not available for the when the DEFAULT language operand is used.
DISABLED
Turns off all HILITE features and removes all action bars. This benefits performance at the expense
of function. Since DISABLED status is not stored in the edit profile, you need to reenter this operand
each time you enter the editor. If ISREDIT HILITE DISABLED is issued by a macro, any attempts to
restore highlighting within the same macro invocation are ignored.
HILITE
Chapter 11. Edit macro commands and assignment statements  347

## Page 380

Description
The HILITE macro command can be used to highlight, in user-specified colors, many language-specific
constructs, program logic features, the phrase containing the cursor, and any strings that match the
previous FIND operation or those that would be found by an RFIND or RCHANGE request. In addition,
when HILITE is entered with no operands, a dialog appears that allows you to set default colors for the
data area in non-program files, for any characters typed since the previous Enter or function key entry,
and for strings located by the FIND command.
Both HI and HILIGHT are valid synonyms for HILITE.
When the code page being used is not the English codepage, the HILITE primary command does
not detect key sequences if the control character within the key sequence has a different binary
representation in the code page being used from the binary representation in the codepage used for
English. For example, in the C language, a '\' is used as an escape sequence character to influence the
interpretation of the next character; however, the '\' has a different binary representation in different
codepages.
Note: Highlighting is not available for edit sessions that involve:
• Data sets with record lengths greater than 255
• Mixed mode edit sessions (normally used when editing DBCS data)
• Formatted data
If a macro issues HILITE in any of these situations, a return code of 12 is set.
Return codes
0
Normal completion.
8
Logic or search not supported in the current environment. Invalid language.
12
Hilite dialog is invalid from an edit macro or Hilite not available because of the installation defaults or
because the edit panel in use is not enabled for enhanced color.
20
Severe error. Possibly extra parameters.
IMACRO—Set or Query an Initial Macro
The IMACRO macro command saves the name of an initial macro in the current edit profile.
The IMACRO assignment statement sets or retrieves the value for the initial macro in the current profile,
and places it in a variable.
See “Initial macros” on page 24 for more information on creating and using initial macros.
Syntax
Macro command syntax
ISREDIT IMACRO name
NONE
name
Identifies the initial macro to be run when editing the data set type that matches this profile. This
macro is run before any data is displayed.
IMACRO
348  z/OS: z/OS ISPF Edit and Edit Macros

## Page 381

NONE
Shows that no macro is to be run at the beginning of each edit session. The editor returns a value of
NONE when no initial macro has been specified.
Assignment statement syntax
ISREDIT ( varname)  = IMACRO
ISREDIT IMACRO  = name
varname
The name of a variable to contain the name of the initial macro.
name
Same as macro command syntax.
Return codes
0
Normal completion
4
IMACRO set not accepted; profile is locked
12
Invalid name specified
20
Severe error
Examples
To set the initial macro name to ISCRIPT:
ISREDIT IMACRO ISCRIPT
To set no initial macro:
ISREDIT IMACRO NONE
To store the name of the initial macro in the variable &IMACNAM:
ISREDIT (IMACNAM) = IMACRO
INSERT—Prepare Display for Data Insertion
The INSERT macro command inserts one or more blank lines, and allows you to fill them with data.
Syntax
ISREDIT INSERT label
linenum numlines
label
A label that shows which line you want the inserted line or lines to follow.
linenum
A relative line number that shows which line you want the inserted line or lines to follow.
INSERT
Chapter 11. Edit macro commands and assignment statements  349

## Page 382

numlines
The number of lines to appear for data input; these lines are not saved until they contain data. If you
do not type a number or if the number you type is 1, only one data input line appears.
Description
Use the INSERT macro command for data input. Inserted lines are initialized with data from the mask line.
However, they are not data lines and cannot be referred to by any macro. Inserted lines are deleted if they
do not contain data.
You must specify that the line referenced on INSERT should be displayed; otherwise, you will not see the
inserted line. Use LOCATE to position a line at the top of the display.
Do not use this command for adding lines with specific data; instead, use the LINE_BEFORE and
LINE_AFTER assignment statements.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To open a 5-line area for data input after the line with the label .POINT, locate .POINT to position it to the
top of the display. Then issue INSERT:
ISREDIT LOCATE .POINT
ISREDIT INSERT .POINT 5
LF—realign the data based on the ASCII linefeed character
The LF macro command allows you to realign the data being edited by interpreting the ASCII linefeed
character X'0A'.
The LF macro command is not available when editing a z/OS UNIX file. Instead, use the ASCII edit facility
to automatically realign the data in a z/OS UNIX file based on the ASCII linefeed and carriage return
characters. See “Working with ASCII data” on page 51.
Note: If the data is saved, it is saved in the realigned state. There is no command to reverse the
alignment. The command should not be executed twice against the data, as the blanks following the
linefeed character will be interpreted as part of the data for the next line.
Syntax
LF
See “Restructuring data based on the linefeed character” on page 51 for more information.
Examples
To realign the data being edited by interpreting the ASCII linefeed character X'0A':
LF
LF
350  z/OS: z/OS ISPF Edit and Edit Macros

## Page 383

LABEL—Set or Query a Line Label
The LABEL assignment statement sets or retrieves the values for the label on the specified line and places
the values in variables.
Syntax
ISREDIT ( var1, var2)  = LABEL label
linenum
ISREDIT LABEL labelname
linenum
 = label
level
var1
The name of a variable to contain the name of the label.
var2
The name of a variable to contain the nesting level of the label. It must be a 3-character value that is
left-padded with zeros.
label
A label identifying the line for which a label must be set or retrieved.
See the LOCATE and RESET command descriptions, which use labels to specify line ranges.
linenum
A relative line number identifying the line for which a label must be set or retrieved.
Use the LINENUM assignment statement to obtain the current relative line number of a line with a
label.
labelname
The name of the label.
For more information about using labels, see “Labels and line ranges” on page 59.
The LINENUM assignment statement can be used to determine whether a label exists. For more
information, see “LINENUM—Query the Line Number of a Labeled Line” on page 360.
level
The highest nesting level at which this label is visible to you or to a macro. Level 0 is the highest level.
Labels at this level are visible to you and to all levels of nested macros. Level 1 is not visible to you,
but it is visible to all macros, and so on. The level can never exceed the current nesting level. The
maximum nesting level is 255. The level number defaults to the current nesting level.
Description
A range of labels is particularly useful for commands that operate on a range of lines, such as those in this
list:
  CHANGE   EXCLUDE   LOCATE    SEEK
  CREATE   FIND      REPLACE   SORT
  DELETE   FLIP      RESET     SUBMIT
Return codes
0
Normal completion
LABEL
Chapter 11. Edit macro commands and assignment statements  351

## Page 384

4
Label name not returned, specified line has no label
8
Label set, but an existing label at the same level was deleted
12
Line number specified is beyond the end of data
20
Severe error
Examples
To get the line of data at the cursor, look for the next occurrence of the string in the variable &ARG, and
then label the line if it is found and currently unlabeled:
ISREDIT (NAME) = LINE .ZCSR
ISREDIT FIND &ARG
IF &LASTCC = 0 THEN -
  ISREDIT (LBL,NEST) = LABEL .ZCSR
IF &LBL=&STR() THEN -
  ISREDIT LABEL .ZCSR = .POINT 0
LEFT—Scroll Left
The LEFT macro command scrolls data to the left of the current panel position.
Syntax
ISREDIT LEFT amt
amt
The scroll amount, the number of columns (0-9999) to scroll, or one of these operands:
MAX
Displays the first page of data to the left.
HALF
Displays the next half-panel of data to the left.
PAGE
Displays the next full panel of data to the left.
CURSOR
Scrolls until the column on which the cursor is located becomes the first data column on the
panel.
DATA
Scrolls until the first column on the current panel of data becomes the last column on the next
panel.
Description
The editor stops scrolling when it reaches the current BOUNDS setting. For example, if the left bound
is position 9 and positions 21 to 92 are displayed, issuing ISREDIT LEFT 20 leaves positions 9 to 80
displayed, not 1 to 72.
To scroll to the left using the panel position when the macro was issued, use USER_STATE assignment
statements to save and then restore the panel position operands.
LEFT
352  z/OS: z/OS ISPF Edit and Edit Macros

## Page 385

If you define a macro named LEFT, it overrides the LEFT command when used from another macro. LEFT
does not change the cursor position and cannot be used in an initial macro. For further information, see
the BOUNDS and DISPLAY_COLUMNS descriptions.
Return codes
0
Normal completion
4
No visible lines
8
No data to display
12
Amount not specified
20
Severe error
Examples
To scroll the display to the left by the number of columns specified in variable &COL:
ISREDIT LEFT &COL
LEVEL—Set or Query the Modification Level Number
The LEVEL macro command allows you to control the modification level that is assigned to a member of
an ISPF library.
The LEVEL assignment statement either sets the modification level or retrieves the current modification
level and places it in a variable.
See “Version and modification level numbers” on page 26 for more information about level numbers.
Syntax
ISREDIT LEVEL num
num
The modification level. It can be any number from 0 to 99.
ISREDIT ( varname)  = LEVEL
ISREDIT LEVEL  = num
varname
The name of a variable to contain the modification level. The modification level is a 2-digit value that
is left-padded with zeros.
num
The modification level. A 2-digit value, left-padded with zeros.
Return codes
0
Normal completion
LEVEL
Chapter 11. Edit macro commands and assignment statements  353

## Page 386

4
Statistics mode is off; the command is ignored
12
Invalid value specified
20
Severe error
Examples
To reset the modification level to 1:
ISREDIT LEVEL = 1
To save the value of the modification level in variable &MODLVL:
ISREDIT (MODLVL) = LEVEL
LINE—Set or Query a Line from the Data Set
The LINE assignment statement either sets or retrieves the data from the data line specified by a relative
line number or label, and places it in a variable.
Syntax
ISREDIT ( varname)  = LINE linenum
label
ISREDIT LINE linenum
label
 = data
varname
Specifies the name of a variable to hold the contents of the specified data line.
linenum
A relative line number identifying the data line.
label
A label identifying the data line.
data
Specifies that these forms can be used:
• Simple string
• Delimited string
• Variable
• Template (< col,string >)
• Merge format (string1 + string2, operand + string2, string1 + operand)
• Operand:
LINE
Data from this line is used.
LINE linenum
Data from the line with the given relative line number.
LINE label
Data from the line with the given label.
LINE
354  z/OS: z/OS ISPF Edit and Edit Macros

## Page 387

MASKLINE
Data from the mask line.
TABSLINE
Data from the tabs line.
Description
The logical data width of the line determines how many characters are retrieved or set. See the
description of the DATA_WIDTH command for information on determining the current logical data width.
You must specify the line pointer to set or retrieve a line. To set data on a line, you can use a variety of
data formats: (variable), templates, or merging a line with other data. The data on the line is completely
overlaid with the data specified on this command.
Return codes
0
Normal completion
4
Data truncated (line shorter than data supplied)
8
Variable not found
12
Invalid line number
16
Variable data truncated
20
Severe error
Examples
To replace the data on line 7 with data from a variable named NEWDAT:
ISREDIT LINE 7 = (NEWDAT)
Note: This syntax is preferred to:
ISREDIT LINE 7 = &NEWDAT
because the variable is not rescanned by either the language processor or ISPF.
To set comment delimiters in columns 40 and 70, blanking the rest of the line:
ISREDIT LINE 1 = < 40 '&STR(/*)' 70 '&STR(*/)' >
To overlay the first 2 columns of line 2 with //:
ISREDIT LINE 2 = LINE + //
To merge mask line data with data from variable &VAR:
ISREDIT LINE 3 = MASKLINE + (VAR)
LINE_AFTER—Add a Line to the Current Data Set
The LINE_AFTER assignment statement adds a line after a specified line in the current data set.
LINE_AFTER
Chapter 11. Edit macro commands and assignment statements  355

## Page 388

Syntax
ISREDIT LINE_AFTER linenum
label
 = 
DATALINE
INFOLINE
MSGLINE
NOTELINE
data
linenum
A relative line number identifying the data line after which the new line is to be inserted. A line pointer
of 0 causes the new line to be inserted at the beginning of the current data set.
label
A label identifying the data line after which the new line is to be inserted.
DATALINE
The line inserted is a data line.
INFOLINE
The line inserted is a temporary, non-data line. The line command field shows ====== in high
intensity and the data on the line is in high intensity, also. The line can be scrolled left and right and
can be as long as the current record length. An information line is protected. Once it has been added
to the data, it cannot be referenced.
MSGLINE
The line inserted is a temporary, non-data line. The line command field contains ==MSG> in high
intensity and the data on the line is also in high intensity. A message line has a data length of 72
characters, regardless of the data width. Once it has been added to the data, it cannot be referenced.
NOTELINE
The line inserted is a temporary, non-data line. The line command field shows =NOTE= in high
intensity and the data on the line is in low intensity. A note line has a data length of 72 characters,
regardless of the data width. It cannot be referenced after it is added to the data.
data
Specifies that these data formats can be used:
• Simple string
• Delimited string
• Variable
• Template (< col,string >)
• Merge format (string1 + string2, operand + string2, string1 + operand)
• Operand:
LINE
Data from the line preceding this line.
LINE linenum
Data from the line with the given relative line number.
LINE label
Data from the line with the given label.
MASKLINE
Data from the mask line.
TABSLINE
Data from the tabs line.
Description
This statement is used for adding lines with specific data. Use the INSERT command for data input.
LINE_AFTER
356  z/OS: z/OS ISPF Edit and Edit Macros

## Page 389

Result: If the cursor is located within the data when the LINE_AFTER assignment statement is processed,
the cursor is repositioned, if necessary, so that it remains on the same data after the statement is
processed.
Return codes
0
Normal completion
4
Data truncated
12
Invalid line number
20
Severe error
Examples
To add data after line 4 with data from a variable named NEWDAT:
ISREDIT LINE_AFTER 4 = (NEWDAT)
Note: This syntax is preferred to ISREDIT LINE_AFTER 4 = &NEWDAT because the variable is not
rescanned by either the language processor or ISPF.
To put a new line that contains the string:
This is the new top line of the data
as the first line of the data set:
ISREDIT LINE_AFTER 0 = "This is the new top line of the data"
To put the contents of the line labeled .START on a new line following the line labeled .END:
ISREDIT LINE_AFTER .END = LINE .START
To put the contents of the mask line modified by the variable &DATA after the line whose number is in
variable &N:
ISREDIT LINE_AFTER &N = MASKLINE + &DATA
LINE_BEFORE—Add a Line to the Current Data Set
The LINE_BEFORE assignment statement adds a line before a specified line in the current data set.
Syntax
ISREDIT LINE_BEFORE linenum
label
 = 
DATALINE
INFOLINE
MSGLINE
NOTELINE
data
linenum
A relative line number identifying the data line before which the new line is to be inserted. A line
pointer of 0 is invalid.
LINE_BEFORE
Chapter 11. Edit macro commands and assignment statements  357

## Page 390

label
A label identifying the data line before which the new line is to be inserted.
DATALINE
The line inserted is a data line.
INFOLINE
The line inserted is a temporary, non-data line. The line command field shows ====== in high
intensity. The data on the line is shown in high intensity also. The line can be scrolled left and right and
can be as long as the current record length. An information line is protected. Once it has been added
to the data, it cannot be referenced.
MSGLINE
The line inserted is a temporary, non-data line. The line command field contains ==MSG> in high
intensity. The data on the line is shown in high intensity also. A message line has a data length of 72
characters, regardless of the data width. Once it has been added to the data, it cannot be referenced.
NOTELINE
The line inserted is a temporary, non-data line. The line command field shows =NOTE= in high
intensity. The data on the line is shown in low intensity. A note line has a data length of 72 characters,
regardless of the data width. It cannot be referenced once it has been added to the data.
data
Specifies that these data formats can be used:
• Simple string
• Delimited string
• Variable
• Template (< col,string >)
• Merge format (string1 + string2, operand + string2, string1 + operand)
• Operand (those allowed follow):
LINE
Data from the line following this line.
LINE linenum
Data from the line with the given relative line number.
LINE label
Data from the line with the given label.
MASKLINE
Data from the mask line.
TABSLINE
Data from the tabs line.
Description
The LINE_BEFORE statement is used for adding lines with specific data. Use INSERT for data input.
Result: If the cursor is located within the data when the LINE_BEFORE assignment statement is
processed, the cursor is repositioned, if necessary, so that it remains on the same data after the
statement is processed.
Return codes
0
Normal completion
4
Data truncated
12
Invalid line number
LINE_BEFORE
358  z/OS: z/OS ISPF Edit and Edit Macros

## Page 391

20
Severe error
Examples
To add data before line 4 with data from a variable named NEWDAT:
ISREDIT LINE_BEFORE 4 = (NEWDAT)
Note: This syntax is preferred to ISREDIT LINE_BEFORE 4 = &NEWDAT because the variable is not
rescanned by either the language processor or ISPF.
To put the contents of the line labeled .START on a new line preceding the line labeled .END:
ISREDIT LINE_BEFORE .END = LINE .START
To put the contents of the mask line modified by the variable &DATA before the line whose number is in
variable &N:
ISREDIT LINE_BEFORE &N = MASKLINE + &DATA
LINE_STATUS—Query Source and Change Information for a Line in
a Data Set
The LINE_STATUS assignment statement retrieves the source and change information for the data line
specified by a line pointer, and places it in a variable. This information indicates how the line was originally
added to the data, and how it has been changed during the edit session.
Syntax
ISREDIT ( varname)  = LINE_STATUS linenum
label
varname
The name of a variable to contain the status string for the specified line. This is a 32-character
variable containing character 1s and 0s:
Characters 1-7 are "source" information.
Character 1
Line is an original record (it existed when the edit session started)
Character 2
Line was created by the Move line command
Character 3
Line was created by the Copy or Repeat line command
Character 4
Line was created by the MOVE primary or macro command
Character 5
Line was created by the COPY primary or macro command
Character 6
Line was created by the TE line command
Character 7
Line was created by the Insert line command
Characters 8-14 are "change" information.
LINE_STATUS
Chapter 11. Edit macro commands and assignment statements  359

## Page 392

Character 8
Line was changed (one of these characters will also be set to show HOW the line was changed)
Character 9
Data on the line was typed over
Character 10
Data was changed by the CHANGE primary command or the Overlay line command
Character 11
Data was changed by the Column Shift line command [ used the (, ((, ), or )) command]
Character 12
Data was changed by the Data Shift line command [ used the <, <<, >, or >> command]
Character 13
Data was changed by the TE, TF, or TS line command
Character 14
The line was renumbered
Characters 15-32 are reserved for future use.
linenum
A relative line number identifying the data line.
label
A label identifying the data line.
Return codes
0
Normal completion
12
Line number not valid
20
Severe error
Examples
To determine if line number one of your data has changed and to display a message informing you of its
status:
ISREDIT (LINESTAT) = LINE_STATUS 1
If linestat(1) = '1' Then
   Say 'Line is an ORIGINAL record'
Else
   Say 'Line was created during this edit session'
If linestat(8) = '1' Then
   Say 'Line has been changed'
Else
   Say 'Line has not been changed'
 
LINENUM—Query the Line Number of a Labeled Line
The LINENUM assignment statement retrieves the current relative line number of a specified label, and
places it in a variable.
Syntax
ISREDIT ( varname)  = LINENUM label
LINENUM
360  z/OS: z/OS ISPF Edit and Edit Macros

## Page 393

varname
The name of a variable to contain the line number of the line with the specified label. The line number
is a 6-digit value that is left-padded with zeros. If the variable is VDEFINEd in character format, it
should be defined with a length of 8. The returned value is left-padded with zeros. For compatibility
with previous releases of ISPF, a length of 6 or 7 is allowed in cases where no data loss will occur.
label
The name of the label for the line whose line number is needed.
Return codes
0
Normal completion
4
Line 0 specified
8
Label specified, but not found (variable set to 0)
12
Invalid line number
20
Severe error
Description
Once the line number is retrieved and placed in a variable, it can be used in arithmetic operations. Note
that line numbers are relative to the position of the line: first=1, second=2, and so on. Therefore, the value
returned by the LINENUM assignment statement is not always be correct if lines are added or deleted
before the line number is obtained.
Examples
To determine the number of lines in the data set and set variable &VAR to the last line number:
ISREDIT (VAR) = LINENUM .ZLAST
That number is 0 if there are no lines.
To set variable &NUM to the line number containing the label .MYLAB:
ISREDIT (NUM) = LINENUM .MYLAB
LOCATE—Locate a Line
The LOCATE macro command scrolls up or down to a specified line. The line is then displayed as the first
line on the panel. There are two forms of LOCATE, specific and generic.
The specific form of LOCATE positions a particular line at the top of the panel. You must specify either a
line number or a label.
The generic LOCATE command positions the panel to the first, last, next, or previous occurrence of a
particular kind of line.
LOCATE
Chapter 11. Edit macro commands and assignment statements  361

## Page 394

Syntax
Specific LOCATE macro command syntax
ISREDIT LOCATE label
linenum
linenum
A relative line number identifying the data line.
label
A label identifying the data line. It must be a label that you have previously defined or an editor-
defined label, such as .ZFIRST or .ZLAST.
Generic LOCATE macro command syntax
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
linenum1 linenum2
FIRST
Searches from the first line, proceeding forward.
LAST
Searches from the last line, proceeding backward.
NEXT
Searches from the first line of the page displayed, proceeding forward.
PREV
Searches from the first line of the page displayed, proceeding backward.
CHANGE
Searches for a line with a change flag (==CHG>).
COMMAND
Searches for a line with a pending line command.
ERROR
Searches for a line with an error flag (==ERR>).
EXCLUDED
Searches for an excluded line.
LABEL
Searches for a line with a label.
SPECIAL
Searches for any special non-data (temporary) line:
• Bounds line flagged as =BNDS>
• Column identification lines flagged as =COLS>
• Information lines flagged as ======
• Mask lines flagged as =MASK>
• Message lines flagged as ==MSG>
LOCATE
362  z/OS: z/OS ISPF Edit and Edit Macros

## Page 395

• Note lines flagged as =NOTE=
• Profile lines flagged as =PROF>
• Tabs line flagged as =TABS>
INFOLINE
Searches for information lines flagged with ======
MSGLINE
Searches for message lines flagged with ==MSG>
NOTELINE
Searches for note lines flagged with =NOTE=
labela, labelb
Labels identifying the start and end of the group of lines in which to search.
Note: If you try to locate a line using a label that has not been assigned, you will receive a return code
of 20. To avoid this, use the LINENUM assignment statement. When using the LINENUM statement, a
return code of 8 is issued if the label does not exist.
ISREDIT (X) = LINENUM .LABEL
linenum1, linenum2
Relative line numbers identifying the start and end of a group of lines in which to search.
Return codes
0
Normal completion
4
Line not located
8
Empty member or data set
20
Severe error
Examples
To locate the next occurrence of a line with a label:
ISREDIT LOCATE NEXT LABEL
To locate the first occurrence of a special (non-data) line:
ISREDIT LOCATE FIRST SPECIAL
To locate the last excluded line:
ISREDIT LOCATE LAST X
To locate the previous line that contains an unprocessed line command:
ISREDIT LOCATE PREV CMD
To locate the first message line:
ISREDIT LOCATE FIRST MSGLINE
LOCATE
Chapter 11. Edit macro commands and assignment statements  363

## Page 396

LRECL—Query the Logical Record Length
The LRECL assignment statement returns the maximum space, in bytes, available for data, COBOL
number fields, and sequence number fields.
Syntax
ISREDIT ( varname)  = LRECL
varname
The name of a variable to contain the logical record length of the data being edited. The logical record
length is a 3-digit value that is left-padded with zeros. If the variable is VDEFINEd in character format,
it should be defined with a length of 5. The returned value is left padded with zeros. For compatibility
with previous releases of ISPF/PDF, a length of 3 or 4 is allowed in cases where no data loss occurs.
Description
The value returned by the LRECL assignment statement includes the sequence number field and, for
fixed-length records, the COBOL number field, if these number fields are used. For variable-length
records, the value returned by LRECL does not include the 4-byte record descriptor word (RDW).
Use the DATA_WIDTH assignment statement to get the maximum space, in bytes, available for data.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
Examples
To check the logical record length of the data and process the data if the logical record length (LRECL) is
80:
ISREDIT (RECLEN) = LRECL
IF &RECLEN = 80 THEN -
   ...
MACRO—Identify an Edit Macro
The MACRO macro command identifies a command as a macro.
Syntax
ISREDIT MACRO
(
,
variable )
PROCESS
NOPROCESS
LRECL
364  z/OS: z/OS ISPF Edit and Edit Macros

## Page 397

variable
The names of the variables that contain parameters, if a macro allows parameters to be specified.
Parameters are parsed and placed into the named variables in the order in which they are typed. The
last variable contains any remaining parameters. Variables that do not receive a parameter are set to
a null string. A parameter is a simple or quoted string, separated by blanks or commas. Quotes can be
single (') or double ("), but must be matched at the beginning and end of the string.
PROCESS
Immediately processes all changes and line commands typed at the keyboard.
For edit line macros, see note under NOPROCESS.
NOPROCESS
Processes changes and line commands typed at the keyboard when the macro completes processing
or a PROCESS statement is found. NOPROCESS must be used if the macro is to use line commands as
input to its processing.
See “PROCESS—Process Line Commands” on page 381 for more information.
Note: For edit line macros, the NOPROCESS keyword must be used. The PROCESS macro statement is
used within the macro to set the labels relating to the line command.
For more information, see “Working with an edit line command table” on page 84.
Description
The MACRO macro command is required in all macros. It must be the first command in a CLIST or REXX
macro that is not a CLIST or REXX statement. Similarly, it also must be the first edit command in a
program macro.
Return codes
0
Normal completion
8
No parameters are permitted for this processing
12
Syntax Error
20
Severe error
Examples
To begin a macro, first accepting a member name and optionally a line number range to be placed in the
variable &PARM:
ISREDIT MACRO (PARM)
ISREDIT COPY AFTER .ZCSR &PARM
To begin a macro, checking parameters before processing panel information, testing for missing input,
excess input, and nonnumeric input:
ISREDIT MACRO NOPROCESS (COL,X)
IF &STR(&COL) = &STR() THEN -
  ISREDIT (,COL) = DISPLAY_COLS
ELSE -
  IF &DATATYPE(&COL) = CHAR THEN -
    GOTO MSG
  IF &STR(&X) ¬= &STR() THEN -
    GOTO MSG
ISREDIT PROCESS
MACRO
Chapter 11. Edit macro commands and assignment statements  365

## Page 398

MACRO_LEVEL—Query the Macro Nesting Level
The MACRO_LEVEL assignment statement retrieves the current nesting level of the macro being run, and
places the nesting level in a variable.
Syntax
ISREDIT ( varname)  = MACRO_LEVEL
varname
The name of a variable to contain the macro nesting level. The nesting level is a 3-digit value that is
left-padded with zeros.
Description
The nesting level can be any number between 1 (a macro that you start) and 255. MACRO_LEVEL is used
to adjust processing based on whether the macro is started by you or called by another macro. It is
required if labels are to be set for the starter of this macro. See “LABEL—Set or Query a Line Label” on
page 351 for more information.
Return codes
0
Normal completion
12
Invalid command format
20
Severe error
Examples
To set the label for the caller of the macro at 1 less than the current level:
ISREDIT (NESTLEV) = MACRO_LEVEL
ISREDIT LABEL .ZCSR = .XSTR &EVAL(&NESTLEV -1)
MACRO_MSG—Set or Query the Macro Message switch
The MACRO_MSG assignment statement sets or retrieves the value of the macro_msg switch, which
controls whether macro processing delivers ISPF messages to the macro.
Syntax
ISREDIT ( varname)  = MACRO_MSG
ISREDIT MACRO_MSG  = 
ON
OFF
varname
The name of a variable containing the setting of MACRO_MSG.
MACRO_LEVEL
366  z/OS: z/OS ISPF Edit and Edit Macros

## Page 399

ON
ISPF messages generated by macro commands are formatted.
OFF
ISPF messages are not formatted.
Description
The MACRO_MSG assignment statement sets a switch for subsequent macro processing. When set ON,
any message that is generated by a macro command is formatted and made available in variables in
ZEDILMSG, ZEDISMSG, and ZEDMSGNO.
This is a diagnostic switch and should only be used to extract messages as required. Macros that perform
operations on many edit lines may experience a performance degradation if this switch is ON.
Return codes
0
Normal completion
20
Severe error
Examples
To set macro_MSG:
ISREDIT MACRO_MSG = ON
MASKLINE—Set or Query the Mask Line
The MASKLINE assignment statement sets or retrieves the value of the mask line, which controls the
display formatting of your input.
Syntax
ISREDIT ( varname)  = MASKLINE
ISREDIT MASKLINE  = data
varname
The name of a variable containing maskline contents.
data
Specifies that these forms can be used:
• Simple string
• Delimited string
• Variable
• Template (< col,string >)
• Merge format (string1 + string2, operand + string2, string1 + operand)
• Operand:
LINE linenum
Data from the line with the given relative line number.
LINE label
Data from the line with the given label.
MASKLINE
Chapter 11. Edit macro commands and assignment statements  367

## Page 400

MASKLINE
Data from the mask line.
TABSLINE
Data from the tabs line.
Description
The MASKLINE assignment statement places the mask line contents in a variable or sets the mask line
from a variable. The mask line can contain any characters and serves to initialize inserted lines to the
value of the mask line. See the description of templates in “Overlays and templates” on page 97 for more
information on the setting of a mask line.
Be careful not to destroy a DBCS string in the mask line. If shift-out (SO) or shift-in (SI) characters in a
mask line are overlaid through the MASKLINE statement, the result is unpredictable.
Return codes
0
Normal completion
4
Data truncated
16
Variable data truncated
20
Severe error
Examples
To set the mask line to place comment delimiters starting at lines 40 and 70:
ISREDIT MASKLINE = <40 '&STR(/*)' 70 '&STR(/*)'>
To set the mask line to blanks:
ISREDIT MASKLINE = " "
MEMBER—Query the Current Member Name
The MEMBER assignment statement retrieves the name of the library member currently being edited, and
places it in a variable. If a sequential data set is being edited, the variable is set to blanks.
Syntax
ISREDIT ( varname)  = MEMBER
varname
The name of a variable to contain the name of the library member currently being edited.
Return codes
0
Normal completion
12
Invalid command format
MEMBER
368  z/OS: z/OS ISPF Edit and Edit Macros

## Page 401

20
Severe error
Examples
To determine if you are editing a library member with a prefix of MIN:
ISREDIT (MEMNAME) = MEMBER
IF &SUBSTR(1:3,&MEMNAME  ) = MIN THEN -
   ...
MEND—End a Macro in the Batch Environment
Note: The MEND command is obsolete.
The MEND macro command ends a macro that is running in the batch environment. It was required for
CLISTs that ran in the batch environment using the MVS/370 operating system. It is not required for z/OS,
but can be used.
Syntax
ISREDIT MEND
Return codes
0
Normal completion
MODEL—Copy a Model into the Current Data Set
The model name form of the MODEL macro command copies a specified dialog development model
before or after a specified line.
The class name form of the MODEL macro command changes the model class that the editor uses to
determine the model you want. For more information on edit models, see Chapter 4, “Using edit models,”
on page 69.
Syntax
ISREDIT MODEL
model_name
qualifier
AFTER
BEFORE
linenum
label
NOTES
NONOTES
model_name
The name of the model to be copied, such as VGET for the VGET service model. This operand can
also be one of the options listed on a model selection panel, such as V1 for the VGET service model.
However, to use these options with the MODEL macro command, you must already know what they
MEND
Chapter 11. Edit macro commands and assignment statements  369

## Page 402

are or else display a model selection panel by using the MODEL primary command. The MODEL macro
command does not display model selection panels. See z/OS ISPF Planning and Customizing for a list
of models and model names.
qualifier 
The name of a model on a secondary model selection panel, such as TBCREATE for the TBCREATE
service model. This operand can also be one of the options listed on a model selection panel, such as
G1 for the TBCREATE service model.
For example, a model selection panel allows you to enter T1 to choose table models. It then displays
another model selection panel for choosing table models, such as G1 for the TBCREATE service
model. Therefore, your MODEL macro command could use either TABLES or T1 as the model-name
operand and either TBCREATE or G1 as the qualifier operand. The simplest way would be to use
TBCREATE or G1 as the model-name operand and omit the qualifier operand.
To use options with the MODEL macro command, you must already know what they are or else display
a model selection panel by using the MODEL primary command. The MODEL macro command does
not display model selection panels. See z/OS ISPF Planning and Customizing for a list of models and
model names.
AFTER
Specifies that the model is to be copied after the line specified by linenum or label.
BEFORE
Specifies that the model is to be copied before the line specified by linenum or label.
linenum
A relative line number identifying where the model should be copied.
label
A label identifying where the model should be copied.
NOTES
Explanatory notes appear when a model is copied.
NONOTES
No explanatory notes appear.
Macro command class name syntax
ISREDIT MODEL
CLASS
class_name
CLASS
Specifies that the current model class is to be replaced by class-name. The new class name is used
for all models from that point on, until you change the model class again or end the edit session.
class_name
Specifies the model class for the current edit session. It must be a name on the Model Classes
panel or an allowable abbreviation. The model class coincides with the type of model, such as REXX,
COBOL, or FORTRAN.
Return codes
0
Normal completion
4
Data truncated (the model exceeded the right-hand margin of the data being edited)
12
Invalid line number (linenum) or label (label)
20
Severe error
MODEL
370  z/OS: z/OS ISPF Edit and Edit Macros

## Page 403

Examples
To copy the VGET model at the end of the current data:
ISREDIT MODEL VGET AFTER .ZL
MOVE— Move a Data Set or a Data Set Member
The MOVE macro command moves a sequential data set, member of a partitioned data set, or z/OS UNIX
file into the data you are editing.
Syntax
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
member
A member of the ISPF library or partitioned data set you are editing.
dsname
A partially or fully qualified data set name. If the data set is partitioned you must include a member
name in parentheses.
pathname
The pathname for a z/OS UNIX regular file to be moved. (Also, see “Specifying z/OS UNIX pathnames
with edit primary and macro commands” on page 15.)
AFTER
Specifies that the member is to be moved after the target specified by linenum or label.
BEFORE
Specifies that the member is to be moved before the target specified by the label.
linenum
A relative line number identifying the target of the move.
label
A label identifying the target of the move. It can be either a label that you define, or one of the
editor-defined labels, such as .ZF and .ZL.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being moved in from the external file is converted from the
character set designated by the keyword to the character set specified for the file being edited or to
the terminal character set.
Note: If member or dsname is less than 8 characters and the data set you are editing is partitioned, a
like-named member is copied. If a like-named member does not exist, the name is considered to be a
partially qualified data set name.
Description
The member, data set, or z/OS UNIX file is deleted after the move. For a concatenated sequence of ISPF
libraries, the deletion occurs only if the member was in the first library of the concatenation sequence.
See Copying and moving data if you need more information.
MOVE
Chapter 11. Edit macro commands and assignment statements  371

## Page 404

Return codes
0
Normal completion
8
End of data before last record read or the specified data set is in use
12
Invalid line pointer (linenum or label); member not found or BLDL error
16
End of data before first record read
20
Syntax error (invalid name, incomplete range), or I/O error
Examples
To move the contents of member ABC after the first line in the current data:
ISREDIT MOVE ABC AFTER .ZF
To move all of data set MOVECOPY.DATA before the line where the cursor is currently positioned:
ISREDIT MOVE MOVECOPY.DATA BEFORE .ZCSR
NONUMBER—Turn Off Number Mode
The NONUMBER macro command turns off number mode, which controls the numbering of lines in the
current data.
Syntax
ISREDIT NONUMBER
Description
You can also use the NUMBER OFF macro command to turn off number mode.
When number mode is off, NONUMBER prevents any verification of valid line numbers, generation of
sequence numbers, and the renumbering of lines that normally occurs when autonum mode is on.
Return codes
0
Normal completion
20
Severe error
Examples
To turn number mode off by using the NONUMBER command:
ISREDIT NONUMBER
NONUMBER
372  z/OS: z/OS ISPF Edit and Edit Macros

## Page 405

NOTES—Set or Query Note Mode
The NOTES macro command sets note mode, which controls whether notes are to appear when a dialog
development model is inserted into the data.
The NOTES assignment statement either sets note mode, or retrieves the setting of note mode and places
it in a variable.
See “MODEL—Copy a Model into the Current Data Set” on page 249 for information about copying dialog
development models.
Syntax
ISREDIT NOTES
NOTE
ON
OFF
ON
Displays explanatory notes when a model is copied into the data being edited.
OFF
Does not display explanatory notes.
ISREDIT ( varname)  = NOTES
ISREDIT NOTES  = 
ON
OFF
varname
The name of a variable to contain the value of note mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Return codes
0
Normal completion
20
Severe error
Examples
To set note mode off:
ISREDIT NOTES = OFF
To store the value of note mode in variable &NOTEMODE:
ISREDIT (NOTEMODE) = NOTES
NOTES
Chapter 11. Edit macro commands and assignment statements  373

## Page 406

NULLS—Set or Query Nulls Mode
The NULLS macro command sets nulls mode, which determines whether trailing blanks in each data field
are written to the panel as blanks or nulls.
The NULLS assignment statement either sets nulls mode or retrieves the setting of nulls mode and places
it in a variable.
Syntax
ISREDIT NULLS
ON STD
ON
ALL
STD
ALL
OFF
ON STD
Specifies that in fields that contain any blank trailing space, the space is to be written as one blank
followed by nulls. If the field is entirely empty, it is written as all blanks.
ON ALL
Specifies that all trailing blanks and all-blank fields are written as nulls.
OFF
Specifies that trailing blanks in each data field are written as blanks.
ISREDIT ( var1, var2)  = NULLS
ISREDIT NULLS  = 
ON STD
ON
ALL
STD
ALL
OFF
var1
The name of a variable to contain either ON or OFF.
var2
The name of a variable to contain ALL, STD, or blanks.
ON STD
Same as macro command syntax.
ON ALL
Same as macro command syntax.
OFF
Same as macro command syntax.
NULLS
374  z/OS: z/OS ISPF Edit and Edit Macros

## Page 407

Description
The term data field  normally refers to the 72 characters of data on each line. Using hardware tabs,
however, you can split each line into multiple fields. See “TABS—Define Tabs” on page 285 for more
details.
Blank characters (X'40') and null characters (X'00') both appear as blanks. When you use the I (insert)
line command, the data entry area appears as blanks for NULLS ON STD and as nulls for NULLS ON ALL.
Trailing nulls simplify use of the Ins (insert) key on the IBM 3270 keyboard. You can use this key to insert
characters on a line if the line contains trailing nulls.
Besides using NULLS, you can create nulls at the end of a line by using the Erase EOF or Del (delete) key.
Null characters are never stored in the data; they are always converted to blanks.
Return codes
0
Normal completion
20
Severe error
Examples
To set nulls mode on with blank trailing space written as one blank followed by nulls and empty fields
written as all blanks:
ISREDIT NULLS = ON STD
To set nulls mode off and thus have trailing blanks in each data field:
ISREDIT NULLS = OFF
NUMBER—Set or Query Number Mode
The NUMBER macro command sets number mode, which controls the numbering of lines in the current
data.
The NUMBER assignment statement either sets number mode, or retrieves the setting of number mode
and places it in variables.
Syntax
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
NUMBER
Chapter 11. Edit macro commands and assignment statements  375

## Page 408

ON
Automatically verifies that all lines have valid numbers in ascending sequence and renumbers any
lines that are either unnumbered or out of sequence. You can also use the RENUM command to turn
number mode on and renumber lines.
The editor interprets the STD, COBOL, and DISPLAY operands only when number mode is turned on.
OFF
Turns number mode off. You can also use the NONUMBER command to turn number mode off.
STD
Numbers the data in the standard sequence field.
COBOL
Numbers the data in the COBOL field.
Note: The NUMBER ON COBOL mode is not supported for formatted data sets.
Attention: If number mode is off, make sure the first 6 columns of your data set are blank
before using either the NUMBER ON COBOL or NUMBER ON STD COBOL command. Otherwise,
the data in these columns is replaced by the COBOL sequence numbers. If that happens and if
edit recovery or SETUNDO is on, you can use the UNDO command to recover the data. You can
also use CANCEL at any time to end the edit session without saving the data.
STD COBOL
Numbers the data in both fields.
If both STD and COBOL numbers are generated, the STD number is determined and then used as the
COBOL number. The COBOL numbers can be out of sequence if the COBOL and STD fields were not
synchronized. Use RENUM to force synchronization.
NOSTD
Turns standard number mode off.
NOCOBOL
Turns COBOL number mode off.
NOSTD NOCOBOL
Turns both the standard number mode and COBOL number mode off.
DISPLAY
Causes the width of the data window to include the sequence number fields. Otherwise, the width of
the window does not include the sequence number fields. When you display a data set with a logical
record length of 80 and STD numbering, the sequence numbers are not shown unless you are using a
3278 Model 5 terminal, which displays 132 characters. Automatic left or right scrolling is performed,
if required, so that the leftmost column of the data window is the first column displayed.
Assignment statement syntax
ISREDIT ( var1, var2)  = NUMBER
ISREDIT NUMBER  = 
ON STD
COBOL
STD COBOL
NOSTD
NOCOBOL
NOSTD NOCOBOL
DISPLAY
OFF
var1
The name of a variable to contain either ON or OFF.
NUMBER
376  z/OS: z/OS ISPF Edit and Edit Macros

## Page 409

var2
The name of a variable to contain one of the eight combinations in this list:
  NOSTD   NOCOBOL   DISPLAY
  STD     NOCOBOL   DISPLAY
  NOSTD   COBOL     DISPLAY
  STD     COBOL     DISPLAY
  NOSTD   NOCOBOL   NODISPL
  STD     NOCOBOL   NODISPL
  NOSTD   COBOL     NODISPL
  STD     COBOL     NODISPL
The value STD, COBOL, or DISPLAY can be placed in var2, even when var1 is set to off. This allows
the macro to save and restore number mode. It also allows the macro to set number mode off, while
specifying defaults to be used when number mode is changed to on.
ON
Same as for macro command syntax.
OFF
Same as for macro command syntax.
STD
Same as for macro command syntax.
COBOL
Same as for macro command syntax.
NOSTD
Turns standard number mode off.
NOCOBOL
Turns COBOL number mode off.
NOSTD NOCOBOL
Turns both the standard number mode and COBOL number mode off.
STD COBOL
Same as for macro command syntax.
DISPLAY
Same as for macro command syntax.
Description
When number mode is on, NUMBER verifies that all lines have valid numbers in ascending sequence. It
renumbers any lines that are either unnumbered or out of sequence, but it does not otherwise change
existing numbers.
In number mode, the editor automatically generates sequence numbers in the data for new lines that
are created when data is copied or inserted. The editor also automatically renumbers the data when it is
saved if autonum mode is in effect.
If the number overlays the shift-in (SI) or shift-out (SO) characters, the double-byte characters are
displayed incorrectly and results are unpredictable.
Return codes
0
Normal completion
20
Severe error
NUMBER
Chapter 11. Edit macro commands and assignment statements  377

## Page 410

Examples
To save the current value of number mode, set number mode off for processing, and then restore the
value of number mode:
ISREDIT (STAT,VALUE) = NUMBER
ISREDIT NUMBER OFF
    ...
ISREDIT NUMBER = (STAT VALUE)
PACK—Set or Query Pack Mode
The PACK macro command sets pack mode, which controls whether the data is stored in packed format.
The PACK assignment statement either sets pack mode, or retrieves the setting of pack mode and places
it in a variable.
The PACK command saves the pack mode setting in the edit profile. See “Packing data” on page 15 for
more information about packing data.
Syntax
ISREDIT PACK
ON
OFF
ON
Saves data in packed format.
OFF
Saves data in unpacked (standard) format.
If you change pack mode, data is written when an END command is issued.
ISREDIT ( varname)  = PACK
ISREDIT PACK  = 
ON
OFF
varname
The name of a variable to contain the setting of pack mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Return codes
0
Normal completion
20
Severe error
Examples
To set pack mode off:
PACK
378  z/OS: z/OS ISPF Edit and Edit Macros

## Page 411

ISREDIT PACK OFF
PASTE—Move or Copy Lines from Clipboard
The PASTE macro command moves or copies lines from a clipboard into an edit session.
Syntax
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
clipboardname
The name of the clipboard to use. If you omit this parameter, the ISPF default clipboard (named
DEFAULT) is used. You can define up to ten additional clipboards. The size of the clipboards and
number of clipboards might be limited by installation defaults.
BEFORE
The destination of the data that is being transferred from the clipboard. BEFORE copies the data
before the specified label linenum or label.
AFTER
The destination of the data that is being transferred from the clipboard. AFTER copies the data after
the specified label linenum or label.
linenum
A relative line number identifying the line after, or before, which the lines from the clipboard are
copied or moved.
label
A label identifying the line after, or before, which the lines from the clipboard are copied or moved.
KEEP
Records are copied and not removed from the clipboard.
DELETE
Records are moved and deleted from the clipboard.
ASIS
The PASTE command determines the character set of the data in the clipboard. If this is different
to the character set being used for the file being edited an automatic conversion occurs for the data
being pasted into the file.
If ASIS is specified, then the automatic conversion does not take place.
Description
PASTE copies or moves lines from a specified clipboard to the current edit session. If lines in the
clipboard are longer than the lines in the edit session, they are truncated.
The portion of the line that is saved in the clipboard is only the data portion of the line. Line numbers
are not saved. If the data was CUT from a data set that had sequence numbers and is PASTEd into an
edit session without sequence numbers, or if it was CUT from a data set without sequence numbers and
PASTEd into a session with sequence numbers, some shifting of data is likely to occur.
PASTE
Chapter 11. Edit macro commands and assignment statements  379

## Page 412

Return codes
0
Normal completion
12
Parameter error. Clipboard is empty or does not exist.
20
Severe error
Examples
To paste data from the default clipboard to the line after the last line in the edit session:
ISREDIT PASTE AFTER .ZLAST DELETE
To paste data from the default clipboard to the line after the first line in the edit session, without clearing
the contents of the clipboard:
ISREDIT PASTE AFTER .ZFIRST KEEP
PRESERVE—Enable Saving of Trailing Blanks
The PRESERVE macro command enables or disables the saving of trailing blanks in the editor. This
enables you to override the setting for the field on the edit entry panel called "Preserve VB record length".
Syntax
ISREDIT PRESERVE
ON
OFF
ON
The editor saves all trailing blanks in the record.
OFF
Turns truncation on. ISPF removes trailing blanks when saving variable-length files. If a line is empty
ISPF saves 1 blank.
ISREDIT ( varname)  = PRESERVE
ISREDIT PRESERVE  = 
ON
OFF
varname
The name of a variable to contain the setting of PRESERVE mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Description
PRESERVE ON causes the editor to save trailing blanks for variable length files. The number of blanks
saved for a particular record is determined by one of these:
PRESERVE
380  z/OS: z/OS ISPF Edit and Edit Macros

## Page 413

• The original record length of the record when it was read in to the editor
• The number of blanks required to pad the record length specified by the SAVE_LENGTH edit macro
command
• The length of the record that was saved on disk during a previous SAVE request in the same edit session
PRESERVE OFF causes the editor to truncate trailing blanks. If a line is empty ISPF saves 1 blank.
Use of the PRESERVE command does not prevent the editor from working on data past the specified
record length. The length set and returned by the PRESERVE command is only used when the data is
written and does not affect the operation of other edit functions.
Return codes
0
Normal completion
6
Record format is not variable.
16
Error setting variable.
20
Severe error
Examples
To save the value of the PRESERVE mode in variable &TRMODE:
ISREDIT (TRMODE) = PRESERVE
To enable the editor to remove trailing blanks when the data is saved:
ISREDIT PRESERVE OFF
PROCESS—Process Line Commands
The PROCESS macro command allows the macro to control when line commands or data changes typed
at the keyboard are processed.
Syntax
ISREDIT PROCESS
DEST RANGE cmd1
cmd2
DEST
Specifies that the macro can capture an A (after) or a B (before) line command that you enter.
The .ZDEST label is set to the line preceding the insertion point. If A or B is not entered, .ZDEST points
to the last line in the data.
Note: If the PROCESS macro command is issued within a line macro, see separate note.
RANGE
Must be followed by the names of one or two line commands, either of which you can enter. Use the
RANGE_CMD assignment statement to return the value of the line command entered. This allows the
macro to define and then capture a line command that you enter. It can also modify its processing
based on which of the two commands was entered.
PROCESS
Chapter 11. Edit macro commands and assignment statements  381

## Page 414

Note: If the PROCESS macro command is issued within a line macro, see separate note.
cmd1 and cmd2
Specifies one or two line command names, which can be 1 to 6 characters; however, if the name is 6
characters long it cannot be used as a block format command (to specify multiple lines) by doubling
the last character. The name can contain any alphabetic or special character except blank, hyphen (-),
or apostrophe ('). It cannot contain any numeric characters.
The .ZFRANGE label is set to the first line identified by the line command that you have entered,
and .ZLRANGE is set to the last line. They can refer to the same line. If the expected RANGE line
command was not entered, .ZFRANGE points to the first line in the data and .ZLRANGE points to the
last line in the data.
Note:
Sequence of processing when PROCESS command issued within a line macro
If the PROCESS command is issued within a line macro, it sets the DEST and RANGE labels, but does
not influence the normal processing order of line commands. Line commands that appear before the
user line command will have already been executed, and line commands that occur after the user line
command are not executed until the user line command macro has completed.
For more information, see “Working with an edit line command table” on page 84.
Description
If a line is retrieved before the PROCESS macro command is called, changes made to this line will not be
seen. The DEST and RANGE operands allow the macro to identify the line commands that you can enter
as additional input to the macro.
This command cannot be specified without first coding the MACRO command with a NOPROCESS
operand.
For more information about using the PROCESS command, see “Using the PROCESS command and
operand” on page 107.
Return codes
0
Normal completion.
4
A RANGE was expected by the macro, but one was not specified; default values set.
8
A DEST (destination) was expected by the macro, but one was not specified; default values set.
12
Both a RANGE and a DEST (destination) were expected by the macro, but were not specified; default
values set.
16
You entered incomplete or conflicting line commands.
20
Severe error
Note: ISPF does not consider a return code of 12 from the PROCESS edit macro command an error and
does not terminate a macro that receives a return code of 12 from the PROCESS edit macro.
Examples
To set up the macro to process the line commands * and # (defined by the macro writer):
ISREDIT MACRO NOPROCESS
ISPEXEC CONTROL ERRORS RETURN
PROCESS
382  z/OS: z/OS ISPF Edit and Edit Macros

## Page 415

ISREDIT PROCESS RANGE * #
IF &LASTCC >= 16 THEN EXIT CODE(&LASTCC)
ISREDIT (CMD) = RANGE_CMD
ISREDIT (FIRST) = LINENUM .ZFRANGE
ISREDIT (LAST)  = LINENUM .ZLRANGE
IF &STR(&CMD) = &STR(*) THEN -
   ...
To place data depending on the location of the A (after) or B (before) line command:
ISREDIT MACRO NOPROCESS
ISREDIT PROCESS DEST
ISREDIT LINE_AFTER .ZDEST = "&DATA"
To allow processing of the A and B destination line commands and the specification of a range by using
the * line command (defined by the macro writer):
ISREDIT MACRO NOPROCESS
ISREDIT PROCESS DEST RANGE *
See “Using the PROCESS command and operand” on page 107.
PROFILE—Set or Query the Current Profile
The control form of the PROFILE macro command displays your current edit profile, defines a new edit
profile, or switches to a different edit profile.
The lock form of the PROFILE macro command locks or unlocks the current edit profile.
The PROFILE assignment statement retrieves the name and lock status of the current edit profile and
stores those values in variables.
Syntax
ISREDIT PROFILE
current_edit_profile
name
5
number
name
The profile name. It can consist of up to 8 alphanumeric characters, the first of which must be
alphabetic. The edit profile table is searched for an existing entry with the same name. That profile is
then read and used. If one is not found, a new entry is created in the profile table.
If you omit this operand, the current edit profile is used.
number
The number of lines, from 0 through 8, of profile data to be displayed. When you type 0 as the
number, no profile data is displayed. When you omit the number operand, the profile modes appear;
the =MASK> and =TABS> lines are displayed if they contain data, followed by the =COLS> line.
The =BNDS> line does not appear if it contains the default boundary positions. It does appear when
the bounds are set to something other than the default, and no 'number' parameter is entered into the
PROFILE command.
For more information about displaying and defining a profile, see “Displaying or defining an edit profile”
on page 18.
ISREDIT PROFILE LOCK
UNLOCK
PROFILE
Chapter 11. Edit macro commands and assignment statements  383

## Page 416

LOCK
Specifies that the current values in the profile are saved in the edit profile table and are not modified
until the profile is unlocked. The current copy of the profile can be changed, either because of
commands you enter that modify profile values (BOUNDS and NUMBER, for example) or because of
differences in the data from the current profile settings. However, unless you unlock the edit profile,
the saved values replace the changes when you end the edit session.
Caps, number, stats, and pack mode are automatically changed to fit the data. These changes occur
when the data is first read or when data is copied into the data set. Message lines (==MSG>) are
inserted in the data set to show you which changes occurred.
Note: To force caps, number, stats, or pack mode to a particular setting, use an initial macro. Be
aware, however, that if you set number mode on, data may be overlaid.
UNLOCK
Specifies that the editor saves changes to profile values.
See “Locking an edit profile” on page 19 for more information about locking and unlocking the profile.
ISREDIT PROFILE RESET
RESET
Specifies that the ZEDFAULT profile is to be removed and the site-wide configuration for new edit
profiles is to be used.
See “Locking an edit profile” on page 19 for more information about locking and unlocking the profile.
ISREDIT ( var1, var2)  = PROFILE
var1
The name of a variable to contain the name of the current edit profile.
var2
The name of a variable to contain the profile status, LOCK or UNLOCK.
Description
Profile names cannot be set by an assignment statement. Instead, use PROFILE to change a profile name,
thereby changing the current edit profile and the edit profile values.
Return codes
0
Normal completion
20
Severe error
Examples
To check the lock status of the profile and perform processing if the profile is locked:
ISREDIT (,STATUS) = PROFILE
IF &STATUS = LOCK THEN -
     ...
RANGE_CMD—Query a Command That You Entered
The RANGE_CMD assignment statement identifies the name of a line command entered from the
keyboard and processed by a macro.
RANGE_CMD
384  z/OS: z/OS ISPF Edit and Edit Macros

## Page 417

Syntax
ISREDIT ( varname)  = RANGE_CMD
varname
The name of a variable to contain the line command that you entered.
Description
The macro must first issue a PROCESS command to identify all line commands to be processed by this
macro. A particular line command within a range can be found by using the RANGE_CMD. For instance, if
this PROCESS command is issued by a macro:
PROCESS RANGE Q $
The RANGE_CMD statement returns either a Q or a $. If a range such as Q5 is entered, only Q is returned.
Return codes
0
Normal completion
4
Line command not set
8
Line command setting not acceptable
20
Severe error
Examples
To determine which line command (* or #) you entered and to process the line command (defined by the
macro writer):
ISREDIT MACRO NOPROCESS
ISREDIT PROCESS RANGE * #
ISREDIT (CMD) = RANGE_CMD
IF &STR(&CMD) = &STR(*) THEN -
        ...
ELSE IF &STR(&CMD) = &STR(#) THEN -
        ...
RCHANGE—Repeat a Change
The RCHANGE command repeats the change requested by the most recent CHANGE command.
Syntax
ISREDIT RCHANGE
Description
You can use this command to repeatedly change other occurrences of the search string. After a string
NOT FOUND message appears, the next RCHANGE issued starts at the first line of the current range for a
forward search (FIRST or NEXT specified) or the last line of the current range for a backward search (LAST
or PREV specified).
RCHANGE
Chapter 11. Edit macro commands and assignment statements  385

## Page 418

Return codes
0
Normal completion
4
String not found
8
Change error (string2 longer than string1 and substitution was not performed on at least one change)
12
Syntax error
20
Severe error
Examples
To perform a single-line change and then repeat the change from the top if the string was not found:
ISREDIT CHANGE C'.  the' C'.  The' 1 8
IF &LASTCC = 4 THEN—
  ISREDIT RCHANGE
RECFM—Query the Record Format
The RECFM assignment statement retrieves the record format of the data set being edited, and places the
value in a variable.
Syntax
ISREDIT ( var1, var2)  = RECFM
var1
The name of a variable to contain the type of record format of the data being edited, either F or V:
F
Fixed-length records.
V
Variable-length records.
var2
The name of a variable to contain the remaining record format information of the data being edited, in
the combination of M, A, S, BM, BA, BS, BSM, or BSA:
B
Blocked records.
S
Standard or spanned records.
M
Machine print control character records.
A
ASA print control character records.
When editing a z/OS UNIX file, var2 is set to blanks.
RECFM
386  z/OS: z/OS ISPF Edit and Edit Macros

## Page 419

Return codes
0
Normal completion
20
Severe error
Examples
To place the type of record format in variable RECFM1 and then use either the logical data width (for a
fixed data set) or the right display column (for a variable data set):
ISREDIT (RECFM1) = RECFM
IF &RECFM1 = F THEN -
  ISREDIT (WIDTH) = DATA_WIDTH
ELSE -
  ISREDIT (,WIDTH) = DISPLAY_COLS
To place the remaining record format information in variable RECFM2:
ISREDIT (,RECFM2) = RECFM
To place the type of record format information in variable RECFM1, and the remaining record format
information in variable RECFM2:
ISREDIT (RECFM1,RECFM2) = RECFM
RECOVERY—Set or Query Recovery Mode
The RECOVERY macro command sets edit recovery mode, which allows you to recover data after a system
failure or power outage.
The RECOVERY assignment statement either sets edit recovery mode, or retrieves the edit recovery mode
setting and places it in a variable.
Syntax
ISREDIT RECOVERY
ON
SUSP
OFF
WARN
NOWARN
ON
The system creates and updates a recovery data set for each change thereafter.
OFF
The system does not create and update a recovery set.
WARN
This operand no longer has a practical function, due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
NOWARN
This operand no longer has a practical function, due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
RECOVERY
Chapter 11. Edit macro commands and assignment statements  387

## Page 420

SUSP
This operand, when specified with the ON operand has no function. It allows existing macros which
save and restore the recovery state to continue working. When SUSP is specified by itself, it functions
like the ON operand.
See “Edit recovery” on page 38 for more information about edit recovery.
ISREDIT ( var1, var2)  = RECOVERY
ISREDIT RECOVERY  = 
ON
SUSP
OFF
WARN
NOWARN
var1
The name of a variable to contain the setting of recovery mode, either ON or OFF.
var2
The name of a variable that contains the warning setting, either WARN, NOWARN (when RECOVERY is
OFF), or blank or SUSP (when RECOVERY is ON).
ON
The system creates and updates a recovery data set for each change thereafter.
OFF
The system does not create and update a recovery set.
WARN
This operand no longer has a practical function, due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
NOWARN
This operand no longer has a practical function, due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
SUSP
This value indicates that recovery is ON, but that it is suspended due to a previous error.
Return codes
0
Normal completion
20
Severe error
Examples
To save the value of recovery mode in variable &RECOV:
ISREDIT (RECOV) = RECOVERY
To set recovery mode OFF:
ISREDIT RECOVERY = OFF
RENUM—Renumber Data Set Lines
RENUM
388  z/OS: z/OS ISPF Edit and Edit Macros

## Page 421

The RENUM macro command immediately turns on number mode and renumbers all lines, starting with
number 100 and incrementing by 100. For any members exceeding 10 000 lines, the increment would be
less than 100.
Syntax
Macro command syntax
ISREDIT RENUM
ON
STD
COBOL
1
STD COBOL DISPLAY
Notes:
1 STD is the default for non-COBOL data set types. COBOL is the default for COBOL data set types.
ON
Automatically verifies that all lines have valid numbers in ascending sequence and renumbers any
lines that are either unnumbered or out of sequence. It also turns number mode on and renumbers
lines.
The STD, COBOL, and DISPLAY operands are interpreted only when number mode is turned on.
STD
Numbers the data in the standard sequence field.
COBOL
Numbers the data in the COBOL field.
STD COBOL
Numbers the data in both fields.
If both STD and COBOL numbers are being generated, the STD number is determined and then used
as the COBOL number. This can result in COBOL numbers that are out of sequence if the COBOL and
STD fields were not synchronized. Use RENUM to force synchronization.
DISPLAY
Causes the width of the data window to include the sequence number fields. Otherwise, the width of
the window does not include the sequence number fields. When you display a data set with a logical
record length of 80 and STD numbering, the sequence numbers are not shown unless you are using a
3278 Model 5 terminal, which displays 132 characters. The editor automatically scrolls left or right, if
required, so that the leftmost column of the data window is the first column displayed.
Return codes
0
Normal completion
20
Severe error
Examples
To renumber all data lines with standard numbering:
ISREDIT RENUM
To renumber all data lines with standard and COBOL numbering:
ISREDIT RENUM STD COBOL
To renumber all data lines with COBOL numbering, bringing the sequence numbers within the data
window:
RENUM
Chapter 11. Edit macro commands and assignment statements  389

## Page 422

ISREDIT RENUM COBOL DISPLAY
To turn sequence numbers off:
ISREDIT RENUM OFF
REPLACE—Replace a Data Set or Data Set Member
The REPLACE macro command adds or replaces data in a member of the partitioned data set that you are
editing, in a member of another partitioned data set, in a sequential data set, or in a z/OS UNIX file. If a
member you want to replace exists and the member is in a PDSE version 2 data set that is configured for
member generations, the editor creates a new generation of the member. This new generation becomes
the current generation (also known as generation zero).
Syntax
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
member
The name of the member to be replaced in the partitioned data set currently being edited. If a name
of eight or fewer characters is specified and it could be a member name or a data set name, REPLACE
searches for a member name first. If no member name is found, then the name is used as a data
set. If the member does not exist, the editor creates it. If you are using a concatenated sequence of
libraries, the member is always written to the first library in the sequence.
dsname
The name of a sequential data set that is to be replaced. The data set name can be fully or partially
qualified.
dsname(member)
The name of a different partitioned data set and member name to be replaced in the partitioned data
set. The data set name can be fully or partially qualified.
pathname
The pathname for a z/OS UNIX regular file to be replaced. If the file does not exist, it is created. (Also,
see Specifying z/OS UNIX pathnames with edit primary and macro commands.)
labela, labelb
Labels identifying the start and end of the group of lines in the current member that replace data in
the other member.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
REPLACE
390  z/OS: z/OS ISPF Edit and Edit Macros

## Page 423

linenum1, linenum2
Relative line numbers identifying the start and end of a group of lines in the current member that
replace data in the other member.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being replaced in the external file is converted to the character
set designated by the keyword.
Return codes
0
Normal completion
8
Member in use
12
Invalid line pointer
20
Syntax error (invalid name, incomplete line pointer value), or I/O error
Examples
To replace member MEM1 with the first 10 lines of the current data:
ISREDIT REPLACE MEM1 1 10
RESET—Reset the Data Display
The RESET macro command can restore line numbers in the line command field when those line numbers
have been replaced by labels, pending line commands, error flags, and change flags. However, to reset
any pending line commands, you must have specified the NOPROCESS operand in the MACRO command.
RESET can also delete special lines from the display, redisplay excluded lines, and temporarily disable the
highlighting of FIND strings.
Syntax
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
You can type the operands in any order. If you do not specify any operands, RESET processes all operands
except LABEL.
RESET
Chapter 11. Edit macro commands and assignment statements  391

## Page 424

CHANGE
Removes ==CHG> flags from the line command field.
COMMAND
Removes any pending line commands from the line command field.
ERROR
Removes ==ERR> flags from the line command field.
EXCLUDED
Redisplays any excluded line.
FIND
Turns off highlighting of FIND strings until the next FIND, RFIND, CHANGE, or RCHANGE command.
However, SEEK and EXCLUDE do not return the highlighting of FIND strings in this manner.
RESET with no operands has the same effect on highlighted FIND strings as RESET FIND.
HIDE
Redisplays all "n Line(s) not Displayed" messages for excluded lines that were hidden through the
HIDE command.
LABEL
Removes labels from the line command field.
SPECIAL
Deletes any temporary line from the panel:
• Bounds line flagged as =BNDS>
• Column identification lines flagged with =COLS>
• Information lines flagged with ======
• Mask lines flagged as =MASK>
• Message lines flagged as ==MSG>
• Note lines flagged with =NOTE=
• Profile lines flagged as =PROF>
• Tabs line flagged as =TABS>
ALL
Removes all changes to the line number field.
labela, labelb
Labels identifying the start and end of the group of lines to be reset.
For more information about using labels to identify a group of lines, see Labels and line ranges.
linenum1, linenum2
Relative line numbers identifying the start and end of a group of lines to be reset.
Description
RESET scans every line of data for conditions to be reset. If you want to delete a small number of special
lines, you can get faster response time if you use the D (delete) line command.
Return codes
0
Normal completion
20
Severe error
RESET
392  z/OS: z/OS ISPF Edit and Edit Macros

## Page 425

Examples
To remove all change flags from the current data:
ISREDIT RESET CHANGE
To remove all error flags from the current data:
ISREDIT RESET ERROR
To redisplay all excluded lines messages that are hidden:
ISREDIT RESET HIDE
To redisplay all excluded lines between the .START and .STOP labels:
ISREDIT RESET EXCLUDED .START .STOP
To remove all labels from the current data between and including the .START and .STOP labels:
ISREDIT RESET LABEL .START .STOP
To remove all special lines from the current data between lines 100 and 200:
ISREDIT RESET SPECIAL 100 200
RFIND—Repeat Find
The RFIND macro command locates the search string defined by the most recent SEEK, FIND, or CHANGE
command, or excludes a line containing the search string defined by the previous EXCLUDE command.
The RFIND command can be used repeatedly to find other occurrences of the search string. After a string
NOT FOUND message appears, the next RFIND issued starts at the first line of the current range for a
forward search (FIRST or NEXT specified), or the last line of the current range for a backward search
(LAST or PREV specified).
Syntax
ISREDIT RFIND
Return codes
0
Normal completion
4
String not found
12
Syntax error
20
Severe error (string not defined)
Examples
To find a character string, process it, and then repeat the operation for the rest of the data:
ISREDIT FIND FIRST C'. the'
SET RETCODE = &LASTCC;
RFIND
Chapter 11. Edit macro commands and assignment statements  393

## Page 426

DO WHILE &RETCODE = 0
 
    ...
 
  ISREDIT RFIND
  SET RETCODE = &LASTCC;
END
RIGHT—Scroll Right
The RIGHT macro command scrolls data to the right of the current panel position.
Syntax
ISREDIT RIGHT amount
amount
The scroll amount. The number of columns (0-9999) to scroll,
MAX
Displays the last panel of data to the right.
HALF
Displays the next half-panel of data to the right.
PAGE
Displays the next full panel of data to the right.
CURSOR
Scrolls until the column on which the cursor is located becomes the first data column on the
panel.
DATA
Scrolls until the last column on the current panel of data becomes the first column on the next
panel of data.
Description
The editor stops scrolling when it reaches the current BOUNDS setting. For example, if the right bound is
position 100, and positions 9 to 80 are displayed, issuing ISREDIT RIGHT 100 leaves positions 29 to 100
being displayed, not positions 109 to 180.
To scroll to the right using the panel position when the macro was issued, use USER_STATE assignment
statements to save and then restore the panel position operands.
If you define a macro named RIGHT, it overrides RIGHT when used from another macro, but has no
effect for you. RIGHT does not change the cursor position and cannot be used in an initial macro. See
“BOUNDS—Set or Query the Edit Boundaries” on page 305 and “DISPLAY_COLS—Query Display Columns”
on page 330 for further information.
Return codes
0
Normal completion
4
No visible lines
8
No data to display
12
Amount not specified
RIGHT
394  z/OS: z/OS ISPF Edit and Edit Macros

## Page 427

20
Severe error
Examples
To scroll the display to the right by the number of columns specified in variable &RCOL:
ISREDIT RIGHT &RCOL
RMACRO—Set or Query the Recovery Macro
The RMACRO macro command sets the name of the recovery macro.
The RMACRO assignment statement sets or retrieves the name of the recovery macro set in this edit
session.
See “Recovery macros” on page 109 for more information.
Syntax
ISREDIT RMACRO name
! name
NONE
name
The name of the recovery macro to be run. The name can be preceded by an exclamation point (!) to
show that it is a program macro.
NONE
The name to prevent a recovery macro from being run; conversely, a value of NONE is returned when
no recovery macro has been specified.
ISREDIT ( varname)  = RMACRO
ISREDIT RMACRO name
NONE
varname
The name of a variable to contain the name of the recovery macro.
name
Same as macro command syntax.
NONE
Same as macro command syntax.
Return codes
0
Normal completion
12
Invalid name specified
20
Severe error
RMACRO
Chapter 11. Edit macro commands and assignment statements  395

## Page 428

Examples
To set the RMACRO name from the variable &RMAC:
ISREDIT RMACRO = &RMAC
SAVE—Save the Current Data
The SAVE macro command stores the current data on disk. Generally, you do not need to use SAVE
if recovery mode is on. See the DATA_CHANGED, AUTOSAVE, CANCEL, and END commands for more
information about saving data.
Syntax
ISREDIT SAVE
NEWGEN
NOGEN
NEWGEN
Applicable only when editing a member in a PDSE version 2 data set that is configured for member
generations. Saves the member in a new generation. This new generation becomes the current
generation, also known as generation zero. The generation being edited is left unchanged. This is the
default behavior when editing the current generation.
NOGEN
Applicable only when editing a member in a PDSE version 2 data set that is configured for member
generations. Saves the member to the same generation that is being edited. This is the default
behavior when editing a non-current generation.
Note: The default SAVE behavior when editing a non-current member generation can be changed in the
ISPF site configuration table.
Description
The SAVE command writes the data to the same data set from which it was retrieved unless you invoke
Edit with a concatenated sequence of partitioned data sets. In that case, the data is saved in the first
library in the concatenation sequence, regardless of which library it came from. For a sequential data set,
the complete data set is rewritten. For a partitioned data set, the member is rewritten with the same
member name.
For a member in a PDSE version 2 data set that is configured for member generations, the behavior
depends on the member generation being edited:
• When editing the current generation, also known as generation zero, the default behavior is to write the
member to a new generation. Refer to the last bullet in this list for an exception.
• When editing a non-current generation, the default behavior is to write the member to the same
generation that is being edited. Refer to the last bullet in this list for an exception. The default behavior
when editing a non-current generation can be changed in the ISPF site configuration table.
• These default behaviors for member generations can be overridden using the NEWGEN and NOGEN
keywords. Refer to the last bullet in this list for an exception.
• If you invoke Edit with a concatenated sequence of partitioned data sets and the data comes from a
data set that is not first in the concatenation sequence, the data is not written back to that data set.
The NEWGEN and NOGEN keywords have no effect in this scenario. The data is saved to a member with
the same member name in the first data set in the concatenation sequence. If the first data set in the
concatenation sequence is a PDSE version 2 data set that is configured for member generations, the
saved data becomes the current generation of the member.
If stats mode is on, the library statistics for the member are automatically updated.
SAVE
396  z/OS: z/OS ISPF Edit and Edit Macros

## Page 429

If both number mode and autonum mode are on, the data is automatically renumbered before it is saved.
Return codes
0
Normal completion
4
New member saved
8
Data is not saved; the SAVE command was entered while in View.
12
Data is not saved; possible reasons are:
• There is not enough PDS space or directory space available to satisfy the command.
• The NEWGEN or NOGEN parameter was specified while editing a file that is not a member in a PDSE
version 2 data set that is configured for member generations.
20
Severe error
Examples
To check autosave mode and, if it is set to OFF, ensure that changes are saved:
ISREDIT (VAR) = AUTOSAVE
IF &VAR = OFF THEN -
   ISREDIT SAVE
When you are editing generation zero of a member in a PDSE version 2 data set and you want the data to
be saved to the same generation (rather than create a new generation) use:
ISREDIT SAVE NOGEN
SAVE_LENGTH—Set or Query Length for Variable-Length Data
The SAVE_LENGTH macro command sets or queries the length to be used to save each record in a
variable-length file. It does not enable you to truncate the nonblank portion of a record, but it does enable
you to extend a record. When records are written to disk, they are padded on the end with blanks as
needed.
Syntax
ISREDIT ( varname)  = SAVE_LENGTH label
linenum
ISREDIT SAVE_LENGTH linenum
label
 = value
Description
You can use the SAVE_LENGTH macro command to set or query the minimum length that is used to store
an individual record in a variable-length data set.
When setting a length, the length is automatically adjusted to include the nonblank portion of the line.
SAVE_LENGTH
Chapter 11. Edit macro commands and assignment statements  397

## Page 430

When retrieving the length, the number returned reflects the line length that would be used if the line
were saved immediately. This is the greater of these two values:
• The length of the nonblank portion of the line and the length set by a previous SAVE_LENGTH request.
• The length of the nonblank portion of the line and the original line length.
You can use the SAVE_LENGTH command in edit macros to define line commands to prompt the user for
final record lengths or to check the record length. You might also use it to substitute a visible character for
trailing blanks to make editing easier.
Use of the SAVE_LENGTH command does not prevent the editor from working on data past the specified
record length. The length set and returned by the SAVE_LENGTH command is only used when the data is
written and does not affect the operation of any other edit functions.
Return codes
0
Normal completion
4
Value supplied on set call was out of range. If the supplied length was too great, it is adjusted to equal
the maximum record length. Otherwise, the length was adjusted to the length of the nonblank data
portion of the record.
6
Record format is not variable. Any value on an assignment request is ignored.
16
Error setting variable.
20
Severe error
Examples
To save the number of characters that are saved for the last line in the file when PRESERVE OFF is active:
ISREDIT (NCHARS) = SAVE_LENGTH .ZLAST
To set the minimum line length for the last line in the file and to set PRESERVE ON active:
ISREDIT SAVE_LENGTH .ZLAST = 74
Another edit macro sample using the SAVE_LENGTH command can be found in the ISRSETLN member of
the ISPF EXEC library.
SCAN—Set Command Scan Mode
The SCAN macro command sets scan mode, which controls the automatic replacement of variables in
command lines passed to the editor.
The SCAN assignment statement either sets the value of scan mode (for variable substitution), or
retrieves the value of scan mode and places it in a variable.
Syntax
ISREDIT SCAN
ON
OFF
SCAN
398  z/OS: z/OS ISPF Edit and Edit Macros

## Page 431

ON
Specifies that the editor automatically replaces variables in command lines.
OFF
Specifies that the editor does not automatically replace variables.
Scan mode is initialized to ON when a macro is started.
ISREDIT ( varname)  = SCAN
ISREDIT SCAN  = 
ON
OFF
varname
The name of a variable to contain the setting of scan mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
Return codes
0
Normal completion
20
Severe error
Examples
To set a line whose number is in variable &LNUM to:
&SYSDATE is a CLIST built-in function
set scan mode off and issue the LINE command with &&SYSDATE as the CLIST function name. The CLIST
processor strips off the first &, but, because scan mode is off, the editor does not remove the second &:;
ISREDIT SCAN OFF
ISREDIT LINE &LNUM = "&&SYSDATE is a CLIST built-in function"
ISREDIT SCAN ON
Because the ISPEXEC call interface for REXX EXECs allows you to specify parameters as symbolic
variables, a single scan always takes place before the syntax check of a statement. Therefore, the rule of
using two ampersands (&) before variable names to avoid substitution of variable names also applies to
REXX EXECs.
SEEK—Seek a Data String, Positioning the Cursor
The SEEK macro command finds one or more occurrences of a search string without changing the exclude
status of the line.
SEEK
Chapter 11. Edit macro commands and assignment statements  399

## Page 432

Syntax
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
string
The search string you want to find. The maximum allowable length of the string is 256 bytes. If
you are specifying a hex string, the maximum is 128 hexadecimal characters. See “Finding, seeking,
changing, and excluding data” on page 44.
labela, labelb
Labels identifying the start and end of the group of lines SEEK is to search.
If the cursor is currently placed above the start label and the PREV occurrence of a string is requested,
or the cursor is currently placed below the end label and the NEXT occurrence of a string is requested,
the process returns a return code of 4 and the string is not found, even if it exists within the label
range.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
NEXT
Starts at the first position after the current cursor location and searches ahead to find the next
occurrence of string.
ALL
Starts at the top of the data and searches ahead to find all occurrences of string.
FIRST
Starts at the top of the data and searches ahead to find the first occurrence of string.
LAST
Starts at the bottom of the data and searches backward to find the last occurrence of string.
PREV
Starts at the current cursor location and searches backward to find the previous occurrence of string.
CHARS
Locates string anywhere the characters match.
PREFIX
Locates string at the beginning of a word.
SUFFIX
Locates string at the end of a word.
WORD
Locates string when it is delimited on both sides by blanks or other non-alphanumeric characters.
X
Scans only lines that are excluded from the display.
NX
Scans only lines that are not excluded from the display.
left_col
The first column to be included in the range of columns SEEK is to search.
SEEK
400  z/OS: z/OS ISPF Edit and Edit Macros

## Page 433

right_col
The last column to be included in the range of columns SEEK is to search.
Description
Use the FIND macro command instead of SEEK if you want to locate a string and change the exclude
status of the line that contains that string at the same time.
You can use SEEK to find a search string, change it with CHANGE, and then exclude it from the display
with EXCLUDE.
To find the next occurrence of the letters ELSE without specifying any other qualifications, include this
line in an edit macro:
ISREDIT SEEK ELSE
Since no other qualifications were specified, the letters ELSE can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• In either an excluded or a non-excluded line
• Anywhere within the current boundaries
To find the next occurrence of the letters ELSE, but only if the letters are uppercase:
ISREDIT SEEK C'ELSE'
This type of search is called a character string search (note the C that precedes the search string) because
it finds the next occurrence of the letters ELSE only if the letters are in uppercase. However, since no
other qualifications were specified, the letters can be found anywhere in the data set or member, as
outlined in the preceding list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Return codes
0
Normal completion
4
String not found
12
Syntax error
20
Severe error
Examples
The example shown here finds the last occurrence in the data set of the letters ELSE. However, the letters
must occur on or between lines labeled .E and .S; they must be the last four letters of a word; and they
must be found in an excluded line.
ISREDIT SEEK ELSE .E .S LAST SUFFIX X
The example shown here finds the first occurrence of the letters ELSE that immediately precedes the
cursor position. However, the cursor must not be positioned ahead of the lines that are labeled .E and .S.
Also, the letters must occur on or between lines labeled .E and .S; they must be stand-alone characters
SEEK
Chapter 11. Edit macro commands and assignment statements  401

## Page 434

(not part of any other word); they must be found in a non-excluded line; and they must exist within
columns 1 and 5:
ISREDIT SEEK ELSE .E .S PREV WORD NX 1 5
SEEK_COUNTS—Query Seek Counts
The SEEK_COUNTS assignment statement retrieves the values set by the most recently entered SEEK
command and places them in variables.
Syntax
ISREDIT ( var1, var2)  = SEEK_COUNTS
var1
The name of a variable to contain the number of strings found. It must be an 8-character value that is
left-padded with zeros.
var2
The name of a variable to contain the number of lines on which strings were found. It must be an
8-character value that is left-padded with zeros.
Return codes
0
Normal completion
20
Severe error
Examples
To seek all lines with a blank in column 1 and store the number of such lines in variable &BLNKS:
ISREDIT SEEK ALL " " 1
ISREDIT (BLNKS) = SEEK_COUNTS
SESSION—Query Session Type
The SESSION assignment statement identifies the type of session in which the macro is running, Edit,
View, EDIF, or VIIF. It also identifies if SCLM is active or not.
Syntax
ISREDIT ( var1, var2)  = SESSION
var1
This variable contains either EDIF, EDIT, VIEW, or VIIF to identify the type of session.
var2
This variable contains SCLM if the SCLM edit environment is active, or four asterisks (****) if not. Until
SCLM edit is initialized and is active, edit commands such as SAVE will not update SCLM correctly.
Note: SCLM edit is not available during execution of the site-wide initial edit macro.
SEEK_COUNTS
402  z/OS: z/OS ISPF Edit and Edit Macros

## Page 435

Return codes
0
Normal completion
20
Severe error
SETUNDO—Set UNDO Mode
The SETUNDO macro command allows the UNDO function to be turned on or off and retrieves the current
UNDO status.
Syntax
ISREDIT SETUNDO
STORAGE
KEEP
RECOVER
ON
OFF
STORAGE
Enables edit changes to be saved in storage.
KEEP
Has the same effect as STORAGE except the UNDO buffers are not cleared when a SAVE is issued.
Note: The effect of KEEP (UNDO buffers not cleared when a SAVE is issued) ceases if SETUNDO is
subsequently issued without the KEEP keyword.
RECOVER
Enables edit changes to be saved through the recovery file only. If edit recovery is off, SETUNDO
RECOVER turns recovery on.
ON
The same as STORAGE.
OFF
Disables the saving of edit changes in storage. If edit recovery is available, the undo command uses
the edit recovery file.
Assignment statement syntax
ISREDIT ( varname)  = SETUNDO
ISREDIT SETUNDO  = 
STORAGE
KEEP
RECOVER
ON
OFF
varname
The name of a variable containing the setting of the UNDO mode, either OFF, RECOVER, STORAGE, or
KEEP.
SETUNDO
Chapter 11. Edit macro commands and assignment statements  403

## Page 436

STORAGE
Enables edit changes to be saved in storage.
KEEP
Has the same effect as STORAGE except the UNDO buffers are not cleared when a SAVE is issued.
Note: The effect of KEEP (UNDO buffers not cleared when a SAVE is issued) ceases if SETUNDO is
subsequently issued without the KEEP keyword.
RECOVER
Enables edit changes to be saved through the recovery file only. If edit recovery is off, SETUNDO
RECOVER turns recovery on.
ON
Enables edit changes to be saved in storage.
OFF
Disables the saving of edit changes in storage. If edit recovery is available, the undo command uses
the edit recovery file.
Description
The SETUNDO macro command enables undo processing. It does not perform the undo function itself.
Valid operands are STORAGE, KEEP, RECOVER, ON, or OFF.
If SETUNDO is set on by a macro and was not on already, the UNDO function is enabled for all interactions
started from the point SETUNDO was turned on.
Note:
1. Changes are saved on the undo chain after:
• SETUNDO STORAGE or SETUNDO KEEP is specified in a macro, and it was previously OFF or REC, or
• SETUNDO REC is specified in a macro, and it was previously OFF
It is possible to undo back to a particular point in a macro. This is helpful in debugging edit macros.
2. If SETUNDO is disabled through the configuration table, the SETUNDO macro command is accepted
and returns a zero return code. It does not turn recovery on.
3. The SETUNDO command is ignored if UNDO from storage is not enabled by the installer or person who
maintains the ISPF product. For information on enabling UNDO from storage, see z/OS ISPF Planning
and Customizing.
Return codes
0
Successful completion. SETUNDO was turned on or off, or status remains unchanged because UNDO
was already on or off.
20
Severe error. Probably a parameter error (something other than STG, KEEP, REC, or OFF was
specified).
Examples
To disable the saving of edit changes in storage:
ISREDIT SETUNDO OFF
To enable the saving of edit changes in storage:
ISREDIT SETUNDO = STORAGE
To store the value of SETUNDO in the variable &SET:
SETUNDO
404  z/OS: z/OS ISPF Edit and Edit Macros

## Page 437

ISREDIT (SET) = SETUNDO
SHIFT (—Shift Columns Left
The SHIFT ( macro command moves characters on a line to the left without altering their relative spacing.
Characters shifted past the current BOUNDS setting are deleted. See “Shifting data” on page 42 for more
information.
Syntax
ISREDIT SHIFT ( linenum
label
2
n
linenum
A relative line number identifying the line on which characters are to be moved to the left.
label
A label identifying the line on which characters are to be moved to the left.
n
Specifies the number of columns to shift.
Description
The SHIFT ( command is limited to shifting columns of data on a single line. If you want to shift columns
of data on several lines, each line of data columns must be moved individually.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To shift columns of data 10 columns to the left on the line that contains the cursor:
ISREDIT SHIFT ( .ZCSR 10
To shift columns of data 2 columns to the left on the line with the label .LAB:
ISREDIT SHIFT ( .LAB
SHIFT )—Shift Columns Right
The SHIFT ) macro command moves characters on a line to the right without altering their relative
spacing. Characters shifted past the current BOUNDS setting are deleted. See “Shifting data” on page 42
for more information.
SHIFT (
Chapter 11. Edit macro commands and assignment statements  405

## Page 438

Syntax
ISREDIT SHIFT ) linenum
label
2
n
linenum
A relative line number identifying the line on which characters are to be moved to the right.
label
A label identifying the line on which characters are to be moved to the right.
n
Specifies the number of columns to shift.
Description
The SHIFT ) command is limited to shifting columns of data on a single line. If you want to shift columns
of data on several lines, each line of data columns must be moved individually.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To shift columns of data 4 columns to the right on the line that contains the cursor:
ISREDIT SHIFT ) .ZCSR 4
To shift columns of data 2 columns to the right on the line with the label .LAB:
ISREDIT SHIFT ) .LAB
SHIFT <—Shift Data Left
The SHIFT < macro command moves the body of a program statement to the left without shifting the
label or comments. This command prevents loss of nonblank characters by stopping before shifting
nonblank characters past the bound. See “Shifting data” on page 42 for more information.
Syntax
ISREDIT SHIFT < linenum
label
2
n
linenum
A relative line number identifying the line on which the body of a program statement is to be moved to
the left.
label
A label identifying the line on which the body of a program statement is to be moved to the left.
SHIFT <
406  z/OS: z/OS ISPF Edit and Edit Macros

## Page 439

n
Specifies the number of columns to shift.
Description
The SHIFT < command is limited to shifting data on a single line. To shift data on several lines, you must
shift data on each line individually.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To shift data 4 columns to the left on the line that contains the cursor:
ISREDIT SHIFT < .ZCSR 4
To shift data 2 columns to the left on the line with the label .LAB:
ISREDIT SHIFT < .LAB
SHIFT >—Shift Data Right
The SHIFT > macro command moves the body of a program statement to the right without shifting
the label or comments. This command prevents loss of nonblank characters by stopping before shifting
nonblank characters past the bound. See “Shifting data” on page 42 for more information.
Syntax
ISREDIT SHIFT > linenum
label
2
n
linenum
A relative line number identifying the line on which the body of a program statement is to be moved to
the right.
label
A label identifying the line on which the body of a program statement is to be moved to the right.
n
Specifies the number of columns to shift.
Description
The SHIFT > command is limited to shifting data on a single line. To shift data on several lines, you must
shift data on each line individually.
SHIFT >
Chapter 11. Edit macro commands and assignment statements  407

## Page 440

Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To shift data 4 columns to the right on the line that contains the cursor:
ISREDIT SHIFT > .ZCSR 4
To shift data 2 columns to the right on the line with the label .LAB:
ISREDIT SHIFT > .LAB
SORT—Sort Data
The SORT macro command puts data in a specified order.
Syntax
ISREDIT SORT
.ZFIRST .ZLAST
labela labelb X
NX
sort_field
sort_field
A
D
start_col
end_col
labela, labelb
Labels identifying the start and end of the group of lines for the sort operation.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
X
Specifies that only excluded lines are to be sorted.
NX
Specifies that only non-excluded lines are to be sorted.
sor t _ field 
Specifies the field to be used in sorting data. You can specify up to five sort fields using these
operands:
A
Specifies ascending order. It can either precede or follow the column specification.
D
Specifies descending order. It can either precede or follow the column specification.
SORT
408  z/OS: z/OS ISPF Edit and Edit Macros

## Page 441

start_col
Defines the starting column of the field that is to be compared. It must be within the current
boundaries.
end_col
Defines the ending column of the field that is to be compared. It must be within the current
boundaries.
If you specify several fields, you must specify both the starting and ending columns of each field. The
fields cannot overlap. If you specify A or D for one field, you must specify it for all fields.
Description
The SORT command operates in two different modes, based on the hexadecimal mode status. If
hexadecimal mode is on, the data is ordered according to its hexadecimal representation. If hexadecimal
mode is off, data is sorted in the collating sequence defined for the national language being used.
Sorting data without operands
For a SORT command with no operands, the editor compares the data within the current boundaries
character by character, and then orders it line by line in the proper collating sequence. It ignores data
outside the current boundaries during both operations. This means that only the data inside the current
boundaries is changed. Labels, excluded lines, line numbers, and change, error, and special line flags are
considered associated with the data, and therefore points to the same data fields after the sort as they did
before the sort.
For example, if you issue a CHANGE ALL command that changes the first, third, and sixth lines in a data
set, these lines are flagged with the change flag, ==CHG>. If you then issue a SORT command that results
in the former lines 1, 3 and 6 becoming the first, second and third lines of the sorted file, the changed line
flags would now exist on the first, second and third lines of the sorted data set.
It is important to properly set the boundaries before issuing the SORT command. SORT is a powerful tool
for editing data that may be formatted in multiple columns. You can set the boundaries, for example, to
the first half of a record and sort one column of data. Then you can set the boundaries to the last half of
the record and sort a second column of data.
Limiting the SORT command
You can specify up to five sort fields by labeling starting and ending columns. You can identify each field
as having data sorted in ascending or descending order.
Optionally, you can limit sorting to a range of lines by specifying the labels of the first and last lines of the
range. You can also limit sorting to either excluded or non-excluded lines.
If you have labels or line ranges that are between the labels or line ranges specified with the SORT
command, you can keep SORT from rearranging them by:
• Excluding them before you enter the SORT command
• Using the NX operand to sort only lines that are not excluded
See the definition of the NX operand and “EXCLUDE—Exclude Lines from the Display” on page 230 for
more information.
Sorting DBCS data
When sorting data that contains DBCS character strings, you must ensure that no DBCS string crosses the
boundaries. Also, all records must have the same format at the boundaries, although the format of the left
and right boundaries can differ.
SORT
Chapter 11. Edit macro commands and assignment statements  409

## Page 442

If a boundary divides a DBCS character, or if all records do not have the same format at the boundaries,
the result is unpredictable.
Return codes
0
Normal completion
4
Lines were already in sort order
8
No records to sort
16
Not enough storage to perform sort
20
Severe error
Examples
To sort the data in descending order, using the sort key in columns 15 through 20:
ISREDIT SORT D 15 20
To sort all excluded lines in ascending order:
ISREDIT SORT X A
SOURCE—describe format of data
The SOURCE macro command instructs the editor to treat the source data as though it is in the specified
format and converts it from that format to the CCSID of the terminal for display purposes, although
the data remains unchanged within the file. When you input or modify data at the terminal, the editor
translates the data entered from the CCSID of the terminal to the specified format prior to storing the data
in the file.
Syntax
SOURCE character_encoding
The SOURCE ASCII macro command is not available when editing a z/OS UNIX file. Instead, use the ASCII
edit facility to have the data automatically translated from ASCII to the CCSID of the terminal.
character_encoding
The type of character encoding to be used for translating data when displaying or receiving input from
the terminal.
Valid values are:
• ASCII
See “Working with ASCII data” on page 51 for more information.
Examples
To set source mode to ASCII:
SOURCE ASCII
SOURCE
410  z/OS: z/OS ISPF Edit and Edit Macros

## Page 443

To revert back to normal mode, use the RESET command:
RESET SOURCE
STATS—Set or Query Stats Mode
The STATS macro command sets stats mode, which creates and maintains statistics for a member of a
partitioned data set.
The STATS assignment statement either sets stats mode, or retrieves the setting of stats mode and places
it in a variable.
Syntax
ISREDIT STATS
ON
OFF
EXT
ON
Creates or updates library statistics when the data is saved.
If extended statistics are enabled in the site configuration and any of the line number statistic values
exceed 65535, the statistics are automatically stored as extended statistics. Otherwise, the statistics
are automatically stored as non-extended statistics. Extended statistics contain extended line count
fields that can store values up to 2147483647; non-extended statistics do not contain the extended
line count fields and can only store line number values up to 65535. If extended statistics are not
enabled in the site configuration, 65535 is stored for line number statistic values that exceed 65535.
OFF
Does not create or update library statistics.
If STATS mode is off when you save a member, any previous statistics are lost.
EXT
Has the same function as ON.
ISREDIT ( varname)  = STATS
ISREDIT STATS  = 
ON
OFF
EXT
varname
The name of a variable to contain the setting of stats mode, either ON or OFF.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
EXT
Same as macro command syntax.
See “Statistics for PDS members” on page 25 for more information.
STATS
Chapter 11. Edit macro commands and assignment statements  411

## Page 444

Return codes
0
Normal completion
20
Severe error
Examples
To put the value of stats mode in variable &LIBSTAT:
ISREDIT (LIBSTAT) = STATS
To set stats mode on:
ISREDIT STATS = ON
To set stats mode off:
ISREDIT STATS OFF
To reset stats mode from the mode saved in variable &LIBSTAT:
ISREDIT STATS = &LIBSTAT
SUBMIT—Submit Data for Batch Processing
The SUBMIT macro command submits the member or data set you are editing (or the part of the member
or data set defined by the range of line pointers or the X or NX parameters) to be processed as a batch job.
Syntax
Macro command syntax
ISREDIT SUBMIT
.ZFIRST .ZLAST
labela labelb X
NX
SUBSYS ( subsystem )
labela, labelb
Labels identifying the start and end of the group of lines to be submitted.
For more information about using labels to identify a group of lines, see Labels and line ranges.
X
Submits only lines that are excluded from the display.
NX
Submits only lines that are not excluded from the display.
SUBSYS(subsystem)
Identifies the name of the emergency subsystem as supported by the TSO submit command. The
name is limited to 4 characters.
SUBMIT
412  z/OS: z/OS ISPF Edit and Edit Macros

## Page 445

Description
The editor does not supply a job statement when you enter the SUBMIT command. You can supply job
statements as part of the data being submitted. When you supply a job statement, only the job name is
logged to the ISPF log data set to ensure the protection of sensitive data.
If the file being edited is described as ASCII or UTF-8 then the data submitted to the internal reader is
converted to EBCDIC.
PDF uses TSO SUBMIT to submit the job.
Return codes
0
Normal completion
20
Severe error (submit failed)
Examples
To submit the first 20 lines of the data as a batch job:
ISREDIT SUBMIT 1 20
To submit all of the data as a batch job:
ISREDIT SUBMIT
To submit only the non-excluded lines as a batch job:
ISREDIT SUBMIT NX
To submit with an emergency subsystem:
"ISREDIT SUBMIT SUBSYS(HASP)"
TABS—Set or Query Tabs Mode
The TABS macro command:
• Turns tabs mode on and off
• Defines the logical tab character
• Controls the insertion of attribute bytes at hardware tab positions defined with the TABS line command
The TABS assignment statement does everything the macro command can do. It can also retrieve the
setting of tabs mode and place it in a variable.
Use PROFILE to check the setting of tabs mode and the logical tab character. See “Using tabs” on page 63
if you need more information about using tabs.
Syntax
ISREDIT TABS
ON STD
ALL
tab_character
OFF
TABS
Chapter 11. Edit macro commands and assignment statements  413

## Page 446

ON
Turns tabs mode on, which means that logical tabs can be used to break up strings of data.
OFF
Turns tabs mode off, which means that logical tabs cannot be used. Attribute bytes are deleted from
all hardware tab positions, causing the Tab Forward and Tab Backward keys to ignore hardware tabs
defined on the =TABS> line. Blanked-out characters occupying these positions reappear. The TABS
OFF message appears in the profile display.
STD
Activates all hardware tab positions (asterisks) that contain a blank or null character. The editor
inserts attribute bytes, which cannot be typed over, at these positions. You can use the Tab Forward
and Tab Backward keys to move the cursor one space to the right of the attribute bytes. The TABS ON
STD message appears in the profile display.
ALL
Causes an attribute byte to be inserted at all hardware tab positions. Characters occupying these
positions are blanked out and the attribute bytes cannot be typed over. The Tab Forward and Tab
Backward keys can be used to move the cursor one space to the right of these attribute bytes. The
TABS ON ALL message appears in the profile display.
tab_character
Defines a single character that is not a number, letter, or command delimiter as the logical tab
character. This character is used with hardware tab definitions. The TABS ON tab character
message appears in the profile display.
You can enclose the character in quotes (' or "), although this is not necessary unless you want to use
one of these characters as the tab character:
=  '  "  <  ,  (  +
The ampersand (&), left bracket ([), and right bracket (]) should not be used as tab characters at all.
The tab_character operand causes the data string that follows the logical tab character to align itself
one space to the right of the first available hardware tab position when you press Enter. No attribute
bytes are inserted.
If no hardware tabs are defined, the editor aligns the data vertically. If software tabs are defined,
the first data string is aligned under the first software tab position and the remaining data strings are
aligned at the left boundary. If neither software nor hardware tabs are defined, the editor aligns all the
data strings at the left boundary.
With the tab_character operand, the Tab Forward and Tab Backward keys ignore hardware tab
positions when the tab_character operand is used because no attribute bytes are inserted.
ISREDIT ( var1, var2)  = TABS
ISREDIT TABS  = 
ON STD
ALL
tab_character
OFF
var1
The name of a variable to contain the setting of tabs mode, either ON or OFF.
var2
The name of a variable to contain the tab character and either ALL or STD. This variable may be blank.
ON
Same as macro command syntax.
OFF
Same as macro command syntax.
TABS
414  z/OS: z/OS ISPF Edit and Edit Macros

## Page 447

STD
Same as macro command syntax.
ALL
Same as macro command syntax.
tab_character
Same as macro command syntax.
Return codes
0
Normal completion
20
Severe error
Examples
To set the tab character to \ and set the tabs mode ON:
ISREDIT TABS ON \
To set the value of tabs mode from variable &TABVAL:
ISREDIT TABS = (TABVAL)
TABSLINE—Set or Query Tabs Line
The TABSLINE assignment statement either sets the tabs line, or retrieves the tabs line and places it in a
variable.
Syntax
ISREDIT ( varname)  = TABSLINE
ISREDIT TABSLINE  = data
varname
Specifies the name of a variable to hold the contents of the current tabs line.
data
Specifies the data used to set the tabs line. The only valid tab characters for this data are blanks,
asterisks (*), hyphens (-), and underscores (_). These forms can be used:
• Simple string
• Delimited string
• Variable
• Template (< col,string >)
• Merge format (string1 + string2, operand + string2, string1 + operand)
• Operand:
LINE linenum
Data from the line with the given relative line number.
LINE label
Data from the line with the given label.
TABSLINE
Chapter 11. Edit macro commands and assignment statements  415

## Page 448

MASKLINE
Data from the mask line.
TABSLINE
Data from the tabs line.
Return codes
0
Normal completion
4
Data truncated
8
Invalid data detected and ignored
20
Severe error (invalid input)
Examples
To store the value of the tabs line in variable &OLDTABS:
ISREDIT (OLDTABS) = TABSLINE
To set the tabs line to "*___* *":
ISREDIT TABSLINE = "*___*   *"
To clear the tabs line:
ISREDIT TABSLINE = " "
To set tabs in columns 1 and 35:
ISREDIT TABSLINE = <1,*,35,*>
To add a tab in column 36:
ISREDIT TABSLINE = TABSLINE + <36,*>
TENTER—Set Up Panel for Text Entry
The TENTER macro command provides one very long line wrapped around onto many rows of the panel to
allow power typing for text entry. The editor does the formatting for you.
The TENTER command is different from the INSERT command in that the INSERT command inserts a
specified number of separate, blank lines and the mask, if any, just as you typed it. With the TENTER
command, however, mask line characters are applied only to the new lines created when the text is
flowed outside the boundaries. Any mask line characters within the bounds are ignored.
Syntax
ISREDIT TENTER linenum
label numlines
linenum
A relative line number identifying the line.
TENTER
416  z/OS: z/OS ISPF Edit and Edit Macros

## Page 449

label
A label identifying the line.
numlines
Specifies the number of lines displayed for text entry; these lines are not saved unless they contain
data. If you do not type a number, the remainder of the panel appears for text entry.
Description
It is important to make sure that the line referenced by the line pointer on TENTER appears; otherwise,
the text area will not be visible to you. Use LOCATE to find and display the line for you.
Before you enter text entry mode:
• If you are going to be typing text in paragraph form, such as for a memo or letter, make sure caps mode
is off. Otherwise, when you press Enter, your text will change to uppercase.
• You may want to turn off number mode to prevent sequence numbers from writing over any of your text.
• Make sure the bounds setting is where you want it so that the text flows correctly when you end text
entry mode.
• Once you enter text entry mode, no macros can be run.
To enter text entry mode:
1. Include this command in an edit macro:
ISREDIT TENTER linenum numlines
or
ISREDIT TENTER label numlines
If numlines is greater than the number of rows remaining on the panel, the vertical bar that indicates
where you will run out of room does not appear and the keyboard does not lock at the last character
position on the panel. When you run the edit macro (see step “2” on page 417), you can scroll down to
bring the additional blank text entry space into view.
2. Run the edit macro. The editor inserts a single continuous blank area for the specified number of rows
or to the bottom of the panel.
To begin a new paragraph:
1. Use the return (Enter), cursor movement, or Tab keys to advance the cursor enough spaces to leave
one blank row on the panel.
If there are insufficient blank spaces on the panel, the keyboard locks when you try to type beyond the
last character position. A vertical bar (|) appears above the cursor at the locked position.
To generate more blank spaces:
1. Press the Reset key to unlock the keyboard.
2. Press Enter.
To end text entry mode:
1. Press Enter. The data is flowed together into a paragraph and any embedded blanks are preserved. The
left and right sides of the paragraph are determined by the current bounds.
See “Word processing” on page 61 and “Entering text (power typing)” on page 63 for more information.
Return codes
0
Normal completion
TENTER
Chapter 11. Edit macro commands and assignment statements  417

## Page 450

12
Invalid line number
20
Severe error
Examples
To find the last line in the data and set up the display for text entry following the last line:
ISREDIT LOCATE .ZL
ISREDIT TENTER .ZL
TFLOW—Text Flow a Paragraph
The TFLOW macro command restructures paragraphs. This is sometimes necessary after deletions,
insertions, splitting, and so forth. See “Word processing” on page 61 and “Formatting paragraphs” on
page 61 for more information.
Syntax
ISREDIT TFLOW linenum
label col
linenum
A relative line number identifying the line.
label
A label identifying the line.
col
Specifies the column to which the text should be flowed. If the column number is omitted, it defaults
to the right boundary. This is different from the TF (text flow) line command, which defaults to the
panel width when default boundaries are in effect.
If a number greater than the right boundary is specified, the right boundary is used.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To limit the flow of text, starting at label .PP, to the displayed columns:
ISREDIT (,RCOL) = DISPLAY_COLS
ISREDIT TFLOW .PP &RCOL
TSPLIT—Text Split a Line
TFLOW
418  z/OS: z/OS ISPF Edit and Edit Macros

## Page 451

The TSPLIT macro command moves part or all of a line of text to the following line. This makes it easier for
you to add new material to existing text.
Syntax
ISREDIT TSPLIT
linenum
label
col
linenum
A relative line number identifying the line where the split is to occur.
label
A label identifying the line where the split is to occur.
col
Specifies the column at which the text is to be split.
If you omit both operands, the split point is assumed to be the current cursor position.
Description
The TSPLIT macro command is affected by the current setting of the boundaries. For instance, data
beyond the right boundary is not moved to the line added by TSPLIT. Data between the split column and
the right boundary is moved to a new line. The cursor position is set to the split point.
To rejoin lines, use the TFLOW macro command. See “TFLOW—Text Flow a Paragraph” on page 418 for
more information.
For more information about splitting lines and other word processing commands, see “Word processing”
on page 61 and “Splitting lines” on page 62.
Return codes
0
Normal completion
12
Invalid line number
20
Severe error
Examples
To split the line labeled .TOP at column 15:
ISREDIT (LINENBR) = LINENUM .TOP
ISREDIT TSPLIT &LINENBR 15
UNNUMBER—Remove Sequence Numbers
The UNNUMBER macro command sets all sequence fields to blanks, turns off number mode, and
positions the data so that column 1 is the first column displayed.
Syntax
ISREDIT UNNUMBER
UNNUMBER
Chapter 11. Edit macro commands and assignment statements  419

## Page 452

Description
The UNNUMBER command is valid only when number mode is also on. The standard sequence field, the
COBOL sequence field, or both, are blanked out.
Return codes
0
Normal completion
12
Number mode not on
20
Severe error
Examples
To set all sequence fields to blanks, turn number mode off, and position the panel so that column 1 is the
first column displayed:
ISREDIT UNNUMBER
UP—Scroll Up
The UP macro command scrolls data up from the current panel position.
Syntax
ISREDIT UP amt
amt
The scroll amount, the number of lines (0-9999) to scroll, or one of these operands:
MAX
Displays the first panel of data.
HALF
Displays the previous half-panel of data.
PAGE
Displays the previous full panel of data.
CURSOR
Scrolls until the line on which the cursor is located becomes the last data line on the panel.
DATA
Scrolls until the first data line on the current panel becomes the last data line on the next panel.
Description
To scroll up using the panel position when the macro was issued, use USER_STATE assignment
statements to save and then restore the panel position operands.
When you issue the UP command, the non-data lines on the panel affect the number of lines scrolled.
However, if you define a macro named UP, it only overrides UP when used from another macro. UP does
not change the cursor position and cannot be used in an initial macro.
The actual number of lines to appear on the panel is determined by:
• The number of lines excluded from the panel
• The terminal display size and split panel line
UP
420  z/OS: z/OS ISPF Edit and Edit Macros

## Page 453

• The number of special temporary lines displayed, such as the ==ERR>, ==CHG>, =PROF>, =MASK>,
=BNDS>, =TABS>, ==MSG>, =NOTE=, =COLS>, and ====== lines.
The first line displayed is determined in one of two ways: (1) a LOCATE command can actually set the line
to be first on the panel, or (2) the first line to be displayed depends on whether the cursor was explicitly
set by a CURSOR assignment statement or implicitly set by a SEEK, FIND, CHANGE, or TSPLIT command.
Since the cursor must be on the panel, the line that is first on the panel may be different from the line that
was first when you started the macro.
Return codes
0
Normal completion
2
No more data UP
4
No visible lines
8
No data to display
12
Amount not specified
20
Severe error
Examples
To scroll up to the top of the data set:
ISREDIT UP MAX
To display the previous half panel of data:
ISREDIT UP HALF
To display the previous full panel of data:
ISREDIT UP PAGE
To make the line where the cursor is placed the last one on the display:
ISREDIT UP CURSOR
To display the previous page less one line:
ISREDIT UP DATA
USER_STATE—Save or Restore User State
The USER_STATE assignment statement saves or restores the state of edit profile values, FIND, CHANGE,
SEEK, and EXCLUDE values, and panel and cursor values.
Syntax
ISREDIT ( varname)  = USER_STATE
USER_STATE
Chapter 11. Edit macro commands and assignment statements  421

## Page 454

ISREDIT USER_STATE  = ( varname)
varname
The name of a variable to contain your status information.
Note: The information in the variable is saved in an internal format that is subject to change.
Dependence on the format can lead to macro errors.
Description
USER_STATE can be used at the beginning of a macro to save conditions, and at the end of a macro
to restore the conditions that may have changed during processing. Many of the values saved by
USER_STATE can be saved and restored individually. The USER_STATE assignment statement is a simple
way of saving many values with a single statement.
These edit modes and values are saved and restored by USER_STATE:
  AUTOLIST  CURSOR        NOTES    RECOVERY
  AUTONUM   HEX           NULLS    STATS
  AUTOSAVE  IMACRO        NUMBER   TABS
  BOUNDS    MASKLINE      PACK     TABSLINE
  CAPS      MODEL CLASS   PROFILE
Return codes
0
Normal completion
20
Severe error
Examples
To save the user state in variable &STATUS:
ISREDIT (STATUS) = USER_STATE
To restore the user state from variable &STATUS:
ISREDIT USER_STATE = (STATUS)
VERSION—Set or Query Version Number
The VERSION macro command allows you to change the version number assigned to a member of an
ISPF library.
The VERSION assignment statement either sets the version number, or retrieves the version number and
places it in a variable.
For more information about version numbers, see “Version and modification level numbers” on page 26.
Syntax
ISREDIT VERSION num
num
The version number. It can be any number from 1 to 99.
VERSION
422  z/OS: z/OS ISPF Edit and Edit Macros

## Page 455

ISREDIT ( varname)  = VERSION
ISREDIT VERSION  = num
varname
The name of a variable to contain the version number. The version number is a 2-digit value that is
left-padded with zeros.
num
Same as macro command syntax.
Return codes
0
Normal completion
4
Stats mode is off, the command is ignored
12
Invalid value specified (the version must be 1 to 99)
20
Severe error
Examples
To save the version number in variable &VERS:
ISREDIT (VERS) = VERSION
To set the version number to 1:
ISREDIT VERSION 1
To set the version number from variable &VERS:
ISREDIT VERSION = &VERS
VIEW—View from within an Edit Session
The VIEW macro command allows you to view a member of the same partitioned data set during your
current edit session.
Syntax
ISREDIT VIEW member
member
A member of the library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
Description
Your initial edit session is suspended until the view session is complete. Editing sessions can be nested
until you run out of storage.
To exit from the view session, END or CANCEL must be processed by a macro or entered by you. The
current edit session resumes.
VIEW
Chapter 11. Edit macro commands and assignment statements  423

## Page 456

The VIEW service call, ISPEXEC VIEW, is an alternate method of starting view. It offers the option of
viewing another data set and specifying an initial macro.
For more information on using the VIEW service, refer to the z/OS ISPF Services Guide.
Return codes
0
Normal completion
12
Your error (invalid member name, recovery pending)
20
Severe error
Examples
To view the member OLDMEM in your current ISPF library:
ISREDIT VIEW OLDMEM
VOLUME—Query Volume Information
The VOLUME assignment statement retrieves the volume serial number (or serial numbers) and the
number of volumes on which the data set resides.
Syntax
ISREDIT ( var1, var2, var3)  = VOLUME
var1
The name of a variable to contain the serial number of the volume on which the data set resides. For a
multivolume data set, this will be the serial number of the first volume. The volume serial number is a
six character value.
var2
The name of a variable to contain the number of volumes the data set occupies. The number of
volumes is a two-character value.
var3
The name of a variable to contain the serial number of the volume of the original data set.
Return codes
0
Normal completion
4
The data set is a multivolume data set and the shared pool variable ZEDMVOL is set to contain all the
volume serial numbers of the data set. ZEDMVOL has the length of the number of volumes times six.
20
Severe error
Examples
To retrieve just the volume serial number of the data set:
ISREDIT (VOL) = VOLUME
VOLUME
424  z/OS: z/OS ISPF Edit and Edit Macros

## Page 457

To retrieve just the number of volumes the data set occupies:
ISREDIT (,NUMVOL) = VOLUME
To retrieve both the volume serial number and the number of volumes the data set occupies:
ISREDIT (VOL,NUMVOL) = VOLUME
XSTATUS—Set or Query Exclude Status of a Line
The XSTATUS assignment statement either sets the exclude status of the specified data line, or retrieves
the exclude status of the specified data line and places it in a variable.
Syntax
ISREDIT ( varname)  = XSTATUS linenum
label
ISREDIT XSTATUS linenum
label
 = X
NX
varname
The name of a variable to contain the exclude status, either X or NX.
linenum
A relative line number identifying the line.
label
A label identifying the line.
X
Specifies that the specified line is to be excluded.
NX
Specifies that the specified line is to be shown (non-excluded).
Description
Exclude status determines whether the line is excluded.
If you want to exclude several lines at one time, the EXCLUDE command should be used. Similarly, to
show several lines at one time, use the FIND command.
Return codes
0
Normal completion
8
An attempt to set a line status to NX could not be performed. The line has a pending line command
on it. For example, if an excluded line contains an M line command in the line command field, then the
MOVE/COPY IS PENDING message is displayed and the lines cannot be shown. The reset command
can be used to remove your line commands from the line command field.
12
Line number is not an existing line.
20
Severe error
XSTATUS
Chapter 11. Edit macro commands and assignment statements  425

## Page 458

Examples
Use XSTATUS together with SEEK and CHANGE to preserve the exclude status of a line. For example, to
store the exclude status of the line whose number is in variable &N in variable &LINEX:
ISREDIT (LINEX) = XSTATUS &N
To exclude line 1:
ISREDIT XSTATUS 1 = X
To locate a string and change it, saving and then restoring the exclude status:
ISREDIT SEEK &DATA
IF &LASTCC = 0 THEN -
  DO
    ISREDIT (XLINE) = XSTATUS .ZCSR
    ISREDIT CHANGE &DATA &NEWDATA .aZCSR .ZCSR
    ISREDIT XSTATUS .ZCSR = (XLINE)
  END
XSTATUS
426  z/OS: z/OS ISPF Edit and Edit Macros
