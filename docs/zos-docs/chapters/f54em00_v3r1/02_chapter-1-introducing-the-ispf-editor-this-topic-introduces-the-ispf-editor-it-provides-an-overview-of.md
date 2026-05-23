# Chapter 1. Introducing the ISPF editor This topic introduces the ISPF editor. It provides an overview of:

Source file: f54em00_v3r1.md
Start page: 35
Page span: 35-48

## Page 35

Chapter 1. Introducing the ISPF editor
This topic introduces the ISPF editor. It provides an overview of:
• The ISPF editor functions
• A typical edit session
• Edit line commands and primary commands
• Edit macros
What is ISPF?
The Interactive System Productivity Facility (ISPF) is a dialog manager that provides tools to improve
program, dialog, and development productivity and control.
The PDF component of ISPF is an integrated work environment used to develop programs, dialogs,
and documents. PDF provides an MVS™-compatible hierarchical library and many productivity-improving
functions. Some examples of these functions are:
• ISPF dialog test tools
• Full-screen editor, with a dialog interface called edit macros
• Multiple update access to data sets
• Online tutorials
• Data set management
• Customized library controls
This document describes the ISPF editor and its dialog interface. A dialog is a program running under
ISPF. The interface allows a dialog to access the usual ISPF dialog functions and the ISPF editor
functions.
What the ISPF editor does
You can use the ISPF editor to create, display, and change data stored in ISPF libraries or other
partitioned or sequential data sets with these characteristics:
• Record Format (RECFM):
– Fixed or variable (non-spanned)
– Blocked or unblocked
– With or without printer control characters
• Logical Record Length (LRECL):
– From 1 to 32760, inclusive, for fixed-length records
– From 5 to 32756, inclusive, for variable-length records.
Note: For variable-length records, the amount of editable data in each record is 4 bytes less than the
logical record length.
Generally, the editor truncates variable-length lines by removing blanks at the end of each line during a
save. If a variable-length line is completely blank and has no line number, a blank is added so that the line
length is not zero.
However, with the PRESERVE function, you can save the trailing blanks of variable-length files. The
"Preserve VB record length" field on the Edit Entry panel and the PRESERVE edit and macro commands
enable you to save or truncate the blanks as you prefer.
What is ISPF?
© Copyright IBM Corp. 1984, 2024 3

## Page 36

Double-byte character set support
The ISPF editor supports DBCS alphabets in two ways:
• Formatted data where DBCS characters are in the column positions specified in the format definition
created with the Format Utility (option 3.11)
• Mixed characters delimited with the special shift-out and shift-in characters.
If you are using mixed mode and the record length of a data set is greater than 72 bytes, there is
a possibility that a DBCS character might encroach on the display boundary. Here, PDF attempts to
display the other characters by replacing an unpaired DBCS character byte with an SO or SI character.
If there is a possibility that the replaced SO or SI character was erased, the line number of the line is
highlighted. If you change the position of the SO and SI characters on the panel, or if you delete the SO
and SI characters entirely, the DBCS character on the boundary is removed to keep the rest of the data
intact.
How to use the ISPF editor
This topic provides an overview of an edit session and covers:
• Beginning an Edit Session
• Using the ISPF editor Basic Functions
• Ending an Edit Session
Beginning an edit session
To begin using the ISPF editor, select option 2 on the ISPF Primary Option Menu. ISPF then displays the
Edit Entry panel (Figure 2 on page 4).
Edit Entry panel (ISREDM01)
Figure 2. Edit Entry panel (ISREDM01)
How to use the ISPF editor
4  z/OS: z/OS ISPF Edit and Edit Macros

## Page 37

Edit entry panel action bar
The Edit Entry panel action bar choices function as follows:
Menu
For information on the Menu pull-down, see the topic about action bars in z/OS ISPF User's Guide Vol
I.
Reflist
The Reflist pull-down offers these choices:
1. Reference Data Set List
Displays the Reference Data Set List panel, which displays a list of up to 30 data set names you
have referenced in PDF panels.
2. Reference Library List
Displays the Reference Library List panel.
3. Personal Data Set List
Displays the Personal Data Set List panel, of which you can have any number, as long as each has
a unique name.
4. Personal Data Set List Open
Displays the Open dialog for all Personal Data Sets.
5. Personal Library List
Displays the Personal Library List panel, which maintains up to 8 lists, each with a unique name. If
more than one list exists, the most recently used list displays.
6. Personal Library List Open
Displays the Open dialog for all Personal Library Lists.
Refmode
Refmode sets reference lists to either retrieve or execute mode. The Refmode pull-down offers these
choices:
1. List Execute
Sets reference lists, personal data set list and personal library lists into an execute mode. When
you select an entry from the list, the information is placed into the ISPF Library or the “Other”
Data Set Name field and an Enter key is simulated. (If this setting is current, the choice is
unavailable.)
2. List Retrieve
Sets reference lists, personal data set list and personal library lists into a retrieve mode. When you
select an entry from the list, the information is placed into the ISPF Library or the “Other” Data Set
Name field, but the Enter key is not simulated. (If this setting is current, the choice is unavailable.)
Utilities
For information on the Utilities pull-down, see the topic about action bars in z/OS ISPF User's Guide
Vol I.
Help
The Help pull-down provides general information about the Edit environment as well as information
about the main options and edit commands.
Edit entry panel fields
You can specify a concatenated sequence of up to four ISPF libraries, but the libraries must have been
previously allocated to ISPF with the Data Set utility (3.2).
The fields on this panel are:
Project
The common identifier for all ISPF libraries belonging to the same programming project.
How to use the ISPF editor
Chapter 1. Introducing the ISPF editor  5

## Page 38

Group
The identifier for the particular set of ISPF libraries; that is, the level of the libraries within the library
hierarchy.
You can specify a concatenated sequence of up to four existing ISPF libraries.
The editor searches the ISPF libraries in the designated order to find the member and copies it into
working storage. If the editor does not find the member in the library, it creates a new member with
the specified name.
When you save the edited member, the editor places or replaces it in the first ISPF library in the
concatenation sequence, regardless of which library it was copied from.
Type
The identifier for the type of information in the ISPF library.
Member
The name of an ISPF library or other partitioned data set member. Leaving this field blank or entering
a pattern causes PDF to display a member list. See z/OS ISPF User's Guide Vol I for information about
entering patterns.
Data Set Name
Any fully qualified data set name, such as USERID.SYS1.MACLIB, VSAM data set name, or z/OS UNIX
file path name. If you include your TSO user prefix (defaults to user ID), you must enclose the data set
name in apostrophes. However, if you omit the TSO user prefix and apostrophes, your TSO user prefix
is automatically added to the beginning of the data set name.
If you specify a VSAM data set, ISPF checks the configuration table to see if VSAM support is enabled.
If it is, the specified tool is invoked. If VSAM is not supported by the configuration settings, an error
message is displayed.
Volume Serial
A real DASD volume or a virtual volume residing on an IBM 3850 Mass Storage System. To access
3850 virtual volumes, you must also have MOUNT authority, which is acquired through the TSO
ACCOUNT command.
Initial Macro
You can specify a macro to be processed before you begin editing your sequential data set or
any member of a partitioned data set. This initial macro allows you to set up a particular editing
environment for the Edit session you are beginning. This initial macro overrides any IMACRO value in
your profile.
If you leave the Initial Macro field blank and your edit profile includes an initial macro specification,
the initial macro from your edit profile is processed.
If you want to suppress an initial macro in your edit profile, type NONE in the Initial Macro field. See
“Initial macros” on page 24 and “IMACRO—Specify an Initial Macro” on page 244 for more details.
Profile Name
The name of an edit profile, which you can use to override the default edit profile. See the description
in “What is an edit profile?” on page 17.
Format Name
The name of a format definition or blank if no format is to be used.
Data Set Password
The password for OS password-protected data sets. This is not your RACF® password.
Record Length
Applicable when editing a z/OS UNIX file. ISPF normally treats z/OS UNIX files as having variable
length records. This field allows you to specify a record length which is used by the editor to load the
records from the file into the edit session as fixed-length records. When the file is saved, it is saved
with fixed-length records. The Record Length field allows you to convert a variable-length file to fixed
length. The value specified in this field must be able to accommodate the largest record in the file. If
the editor finds a record that is larger than the length specified, an error message is displayed and the
edit session does not proceed.
How to use the ISPF editor
6  z/OS: z/OS ISPF Edit and Edit Macros

## Page 39

Line Command Table
Use this field to define a set of user line commands that you can use during the edit session. The table
you specify can be generated using the ISPF table editor and contains the line commands that you
wish to have available and associates each line command with an edit macro that will be run if the line
command is entered during the edit session.
PDSE Generation
This field gives you the opportunity to specify a generation number. You can use this field only when
you specify a PDS member in the ISPF Library or Other Data Set field.
Enter an absolute (positive) generation number or a relative (negative) generation number in this field
to edit a non-current generation of the member. This is valid only when the member is in a PDSE
Version 2 data set that is configured for member generations.
Confirm Cancel/Move/Replace
When you select this field with a "/", a confirmation panel displays when you request one of these
actions, and the execution of that action would result in data changes being lost or existing data being
overwritten.
• For MOVE, the confirm panel is displayed if the data to be moved exists. Otherwise, an error
message is displayed.
• For REPLACE, the confirm panel is displayed if the data to be replaced exists. Otherwise, the
REPLACE command functions like the edit CREATE command, and no confirmation panel is
displayed.
• For CANCEL, the confirmation panel is displayed if any data changes have been made, whether
through primary commands, line commands, or typing.
Note: Any commands or data changes pending at the time the CANCEL command is issued are
ignored. Data changes are "pending" if changes have been made to the displayed edit data, but
no interaction with the host (ENTER, PF key, or command other than CANCEL) has occurred. If no
other changes have been made during the edit session up to that point, the confirmation panel is not
displayed.
Mixed Mode
When you select this field with a "/", it specifies that the editor look for shift-out and shift-in delimiters
surrounding DBCS data. If you do not select it, the editor does not look for mixed data.
Preserve VB record length
You can select this option to cause the editor to store the original length of each record in variable-
length data sets and when a record is saved, the original record length is used as the minimum length
for the record.
Data Encoding
You can use this option to select whether to edit data as ASCII (CCSID 819) or UTF-8 (CCSID 1208).
When you select a value for this option, the editor uses the selected CCSID in converting the data to
the CCSID for the terminal.
You can also specify this option when creating a new file to contain ASCII or UTF-8 data.
For z/OS UNIX files, the editor breaks up data into records using the ASCII (and UTF-8) linefeed
character (X'0A') and the ASCII (and UTF-8) carriage return character (X'0D') as the record delimiter.
The linefeed and carriage return characters are removed from the data loaded into the editor, but
written back to the file when the data is saved. When the file is saved, ISPF ensures the file is tagged
with a CCSID of 819 (or 1208).
Creating a new data set
Before you can edit a new sequential data set, you must allocate space for it. When you specify an empty
sequential data set or nonexistent member of a partitioned data set, the first edit display contains several
empty lines between the Top of Data and Bottom of Data message lines (Figure 3 on page 8).
How to use the ISPF editor
Chapter 1. Introducing the ISPF editor  7

## Page 40

The editor replaces the quote marks on the left of the panel with sequence numbers when you type
information on the lines.
See “Creating and replacing data” on page 41 and “Word processing” on page 61 for more information
on using the editor to create data.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.DATA(EDITNEW) - 01.00            Columns 00001 00072
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 Command ===> ________________________________________________ Scroll ===> CSR
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 3. Creating a new data set (ISREDDE2)
Editing an existing data set
When you edit an existing data set, ISPF displays the Primary Edit Panel as shown in Figure 4 on page
8.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.DATA(EDITOLD) - 01.00            Columns 00001 00072
 ****** ***************************** Top of Data ******************************
 000100 PROC 0
 000200 EX 'PDFTOOL.COMMON.EXEC.(ALLOCPDF)' 'REL(DEV) FVT NOTOOLS'
 000300 PDF
 ****** **************************** Bottom of Data ****************************
 Command ===> ________________________________________________ Scroll ===> CSR
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 4. Example Primary Edit panel (ISREDDE2)
Primary Edit panel action bar choices
The Primary Edit panel action bar choices function as follows:
File.
The File pull-down offers you these choices:
How to use the ISPF editor
8  z/OS: z/OS ISPF Edit and Edit Macros

## Page 41

1. Save
Executes the SAVE command.
2. Cancel
Executes the CANCEL command (which ignores all changes made to the member) and redisplays
the Edit Entry panel.
3. Exit
Executes the END command (which saves the data set or member) and redisplays the Edit Entry
panel.
Edit
The Edit pull-down offers you these choices:
1. Reset
Performs the RESET command.
2. Undo
Performs the UNDO command.
3. Hilite
Displays the Edit Color Settings pop-up.
4. Cut
Cuts the selected data from the file, placing it on the clipboard.
5. Paste
Puts the selected data from the clipboard into the chosen area of the current file.
Edit_Settings
When selected, causes an additional panel to display to enable you to set the characteristics of your
edit sessions.
1. Edit settings
Causes the additional panel to display.
Menu
For information on the Menu pull-down, see the topic about action bars in z/OS ISPF User's Guide Vol
I.
Utilities
For information on the Utilities pull-down, see the topic about action bars in z/OS ISPF User's Guide
Vol I.
Compilers
The Compilers pull-down provides shortcuts to the compilers on the Foreground Selection Panel and
Batch Selection Panel, the ISPPREP panel preprocessing utility, and the DTL compiler.
Test
The Test pull-down offers you these choices:
1. Functions
Displays the Dialog Test Function/Selection panel.
2. Panels
Displays the Dialog Test Display panel.
3. Variables
Displays the Dialog Test Variables panel.
4. Tables
Displays Dialog Test Tables panel.
5. Log
Displays the ISPF Transaction Log panel.
6. Services
Displays the Invoke Dialog Service panel.
7. Traces
Displays the Dialog Test Traces panel.
How to use the ISPF editor
Chapter 1. Introducing the ISPF editor  9

## Page 42

8. Break Points
Displays the Dialog Test Breakpoints panel.
9. Dialog Test
Displays the Dialog Test Primary Option panel.
10. Dialog Test appl ID
Displays the Dialog Test Application ID panel.
Help
The Help pull-down provides general information about the main options available during an Edit
session as well as information about edit line commands and primary commands.
Editing the data set
When the editor displays existing data, each line consists of a 6-column line command field followed by
a 72-column data field. The line command fields contain the first 6 digits of the sequence numbers in the
data. If the data has no sequence numbers, the line command fields contain relative numbers that start at
1 and are incremented by 1.
Based on your action, the ISPF editor places the cursor in the most useful position. To help you find the
cursor, the editor intensifies the line command field that contains the cursor.
If the data contains characters that cannot be displayed, blanks replace those characters on the panel but
not in the data. You cannot type over the blanks. You can display and edit undisplayable characters by
entering hexadecimal mode or by using the FIND and CHANGE commands with hexadecimal strings. See
“HEX—Display Hexadecimal Characters” on page 236 for information on entering hexadecimal mode.
Printer control characters, if present, are displayed and are treated as part of the data. ASA control
characters are alphanumeric and you can edit them. Machine control characters, however, cannot be
displayed and are replaced on the panel with blanks.
When you are editing existing data, the selected member or sequential data set is read into virtual
storage, where it is updated during edit operations. Use of virtual storage for editing work space results in
high performance, but might require a large user region. If you use all available storage, an ABEND occurs,
and you lose the work space unless recovery mode is on.
Using the ISPF editor basic functions
The basic functions of the ISPF editor are simple and can be used immediately:
• To alter data, type over the existing material or use the Ins (Insert) and Del (Delete) keys to add or
remove characters.
• To view data that is not displayed, use the scroll commands. These are PDF default values:
Fn key
Action
F7/F19
Scrolls up
F10/F22
Scrolls left
F8/F20
Scrolls down
F11/F23
Scrolls right
• To insert a line between existing lines, type I over a number in the line command field and press Enter.
The line command field is the 6-column row displayed on the left side of the panel when you create or
edit a data set. The new line is inserted after the one on which you typed the I.
How to use the ISPF editor
10  z/OS: z/OS ISPF Edit and Edit Macros

## Page 43

Note: The editor does not distinguish between input mode and edit mode. Use the I or TE line
commands to insert new lines, either between existing lines or at the end of the data.
• To delete a line, type D over the number to the left and press Enter.
• To save your work and leave the editor, type END on the command line and press Enter.
Ending an edit session
Usually, you complete your editing session with the END command and, based on the values in your edit
profile, PDF performs these tasks:
• If autosave mode is on and you have made changes to the data:
– If both number mode and autonum mode are on, the data is renumbered. If not, the numbers remain
unchanged.
– The data is automatically saved. Special temporary lines, such as =PROF>, =MASK>, ==ERR>,
==CHG>, =BNDS>, =TABS>, ==MSG>, =NOTE=, =COLS>, and ====== are not part of the data and
are not saved. However, you can convert =COLS>, ==MSG>, =NOTE=, and ====== lines to data lines
and save them as part of the data set by using the MD (make dataline) line command before entering
END.
– If STATS mode is on and the data is a member of an ISPF library or other partitioned data set,
the statistics are either generated or updated, depending on whether statistics were previously
maintained for the member. If the member is an alias, the alias indicator is turned off.
– If autolist mode is on, a source listing of the data is recorded in the ISPF list data set for eventual
printing.
• If autosave mode is off with the PROMPT operand, a prompting message is displayed. You can issue
SAVE to save the data or CANCEL to end the edit session without saving the data.
• If autosave mode is off with the NOPROMPT operand, the data is not saved. The result is the same as
that which occurs if you enter a CANCEL command. (You can opt to confirm cancellations by selecting
that option from the Primary Edit panel action bar Confirm choice.)
• PDF returns to the previous panel, which is either a member list or the Edit Entry panel.
• When END is issued from a macro, the edit session does not complete until the macro terminates
all processing. For example, when a Rexx macro executes the EXIT statement or a COBOL language
program executes STOP RUN.
You can end editing without saving by using CANCEL.
By default, the editor truncates variable-length lines by removing blanks at the end of each line during a
save. If a variable-length line is completely blank and has no line number, a blank is added so that the line
length is not zero.
If you select "Preserve VB record length" on the edit entry panel, or specify PRESERVE on the edit service,
the editor stores the original length of each record in variable-length data sets and when a record is
saved, the original record length is used as the minimum length for the record. The minimum line length
can be changed by using the SAVE_LENGTH edit macro command. The editor always includes a blank at
the end of a line if the length of the record is zero.
Because VIEW is a special type of edit session, it is important to note that the use of the REPLACE or
CREATE commands from within VIEW always honors the setting of the "Preserve VB record length" option
on the edit entry panel. This setting can be overridden by using the PRESERVE primary command.
Attention: CANCEL cancels all changes made since the beginning of the edit session or the last
SAVE command, whichever is most recent.
The RETURN command is logically equivalent to the repeated use of the END command. PDF performs the
same actions at the end of the edit session.
When a space ABEND such as D37 occurs, ISPF deallocates the data set so that you can swap to another
screen or user ID and reallocate the data set. This does not occur for data sets that were edited using the
DDNAME parameter of the EDIT service.
How to use the ISPF editor
Chapter 1. Introducing the ISPF editor  11

## Page 44

Edit commands
You can use two kinds of commands to control editing operations: line commands and primary
commands.
Line commands
Line commands affect only a single line or block of lines. You enter line commands by typing them
in the line command field on one or more lines and pressing Enter. The line command field is usually
represented by a column of 6-digit numbers on the far left side of your display. When you are editing an
empty data set or member, however, the line command field contains quotes. This field can also be used
to define labels and to display flags that indicate special lines, such as the =NOTE= flag, which indicates a
note line.
You can use line commands to:
• Insert or delete lines
• Repeat lines
• Rearrange lines or overlay portions of lines
• Simplify text entry and formatting
• Define an input mask
• Shift data
• Include or exclude lines from the display
• Control tabs and boundaries for editing
• Convert some types of special temporary lines to data lines
You can enter edit line commands as primary commands on the command line by prefixing them with
a colon (:) and placing the cursor on the target line. For example, if you enter :D3 on the command line
and move your cursor to line 12 of the file, the three lines 12, 13, and 14 are deleted from the file. This
technique is normally used for PF key assignments.
See Chapter 3, “Managing data,” on page 41 for ways you can use line commands to manipulate data
and Chapter 9, “Edit line commands,” on page 135 for the line command syntax.
Primary commands
Primary commands affect the entire data set being edited. You enter primary commands by typing them
on the command line (Command ===>), usually located on line 2, and pressing Enter. Any command
entered on the edit command line is first intercepted by ISPF. If the command entered is an Edit Primary
Command or an Edit Macro, PDF processes the command.
You can use primary commands to:
• Control your editing environment
• Find a specific line
• Find and change a character string
• Combine several members into one
• Split a member into two or more members
• Submit data to the job stream
• Save the edited data or cancel without saving
• Sort data
• Delete lines
• Access dialog element models
• Run an edit macro
Edit commands
12  z/OS: z/OS ISPF Edit and Edit Macros

## Page 45

If you have a primary command that is too long for the input field in the command line the ISPF command
ZEXPAND can be used to display a popup window with the input field expanded to a length of 255
characters. The long primary command can then be entered in this expanded input field. After you exit the
popup window and return to the data display press Enter to have the editor process the command. This
popup window is only for the input of edit primary commands. To input other commands (for example TSO
commands) that are too long for the command field, use the CMDE command.
Note:
• A long editor command entered in the popup window is truncated at the length of the edit panel
command field when saved in the command retrieve stack.
• The support for an expandable command field is enabled for the IBM-supplied edit panels ISREDDE2,
ISREDDE3, ISREDDE4, ISREDDE5, and FLMEDDE. The LEFT and RIGHT commands cannot be used to
scroll data in the command field.
You can prefix any primary command with an ampersand to keep the command displayed on the
command line after the command has processed. This technique allows you to repeat similar commands
without retyping the command. For example, if you type:
&CHANGE ALL ABCD 1234
the command is displayed after the change has been made, which allows you then to change the
operands and issue another CHANGE command. You can recall previous commands with the ISPF
RETRIEVE command.
See Chapter 3, “Managing data,” on page 41 for some of the ways you can use primary commands
to manipulate data and Chapter 10, “Edit primary commands,” on page 191 for the primary command
syntax.
Edit commands and PF key processing
In the Edit function there are some differences between the way ISPF processes commands when they
are entered from the command line as compared to when they are entered by a combination of the
command line and a function (PF) key. In most applications, when you press a PF key, ISPF concatenates
the contents of the command line to the definition of the function key. The result is handled as a single
command by ISPF or by the application.
When you use a PF key defined as a scroll command (UP, DOWN, LEFT, or RIGHT) the system processes
the command as follows:
• If the concatenation of the scroll command PF key definition and the contents of the command line
does not create a valid scroll command:
– If the word after the scroll command PF key definition begins with a numeric character (0-9), you get
a message telling you the scroll amount was not valid.
– Otherwise, edit processes the contents of the command line as an edit command, then processes the
scroll command using the default scroll amount.
In this case, the processing of the command line contents as an edit command bypasses the
command table, because the command table is used to resolve the scroll key.
• If the concatenation of the scroll command PF key definition and the contents of the command line
does create a valid scroll command edit scrolls the screen the specified amount.
If you manually type a scroll command on the command line (you do not use any PF keys) and it has an
operand, the operand is checked for validity. However, in the case of a scroll operand that is not valid, the
operand is not processed as a separate edit command as it is when used with a PF key.
When you use a PF key defined as RFIND or RCHANGE, first the command line is processed and then the
PF key is actioned. For example, if you type a Find command then press PF5, the new find string is passed
to RFIND:
Edit commands
Chapter 1. Introducing the ISPF editor  13

## Page 46

Table 1. Examples of passing a string to RFIND
Command Action Result
F STR1 press Enter Edit finds the next occurrence of STR1
F STR2 press PF5 RFIND finds the next occurrence of STR2
If you type C STR1 STR2 and press Enter to change STR1 to STR2, then on the command line type F
STR3 and press the RCHANGE key, this results in the command C STR3 STR2 being run:
Table 2. An example of passing string values to RCHANGE
Command Action Result
C STR1 STR2 press Enter Edit changes the next occurrence of STR1 to STR2
F STR3 press PF6 RCHANGE changes the next occurrence of STR3 to STR2
You can change this behavior of RCHANGE by using the EDITSET command to set an option, Force
ISRE776 if RCHANGE passed arguments. If this option is set, RCHANGE will treat anything that you type
on the command line as an invalid parameter and will return an error message ISRE776.
Edit macros
Edit macros are primary commands or line commands that you write. You can save time and keystrokes
by using macros to perform often-repeated tasks.
Primary command macros
To run a primary command macro, type its name and any operands on the command line, and press Enter.
Your installation may have written and documented common macros for your use. Of course, you can also
write your own edit macros.
The rules for running a specific macro, and the expected results, depend on the particular macro.
Your installation is responsible for documenting these rules and results. If you want to write your own
macros, read Part 2, “Edit macros,” on page 77 and Chapter 11, “Edit macro commands and assignment
statements,” on page 295.
ISPF enables the installer of the program to specify an edit macro that runs for all users. If a macro name
is specified in the ISPF configuration table, then that macro runs before any macros specified in the users'
profiles, in programs that invoke edit, or on the edit entry panels.
The site-wide macro can be used to alter existing profiles, enforce site-wide standards, track edit
usage, deny edit and view of a data set member, or for any other purposes for which edit macros are
designed. Site-wide macros normally end with a return code of 1 (one) in order to place the cursor on the
command line. Site-wide macros must be available to each user in the appropriate data set concatenation
(SYSPROC, STEPLIB, and so forth) or in Linklist or LPA (program macros only).
Users can also set an application-wide macro if they choose. See “Application-wide macros” on page 25
for more information.
The effect of running a macro depends on the implementation of the macro. Results such as cursor
positioning, output messages, and so on, may or may not conform to the results that you expect from
built-in edit commands.
Line command macros
You can define a table of user line commands and associated user macros using the ISPF table editor.
To run a user line command, type its name over the 6-digit number in the line command field and press
Enter (in the same way as for any other line command). ISPF then invokes the associated user macro.
Edit macros
14  z/OS: z/OS ISPF Edit and Edit Macros

## Page 47

If you want to write your own line command to invoke a specific macro, see “Working with an edit line
command table” on page 84.
Editing data in SCLM-controlled libraries
For information about editing libraries that are controlled under SCLM, refer to z/OS ISPF Software
Config ur ation  and Library Manager Guide and Reference.
Packing data
Data can be saved in either packed or standard format. You can control the format by using the PACK
primary command to change the edit profile. The editor reads the data in and you can edit it the way you
normally would. When you end the editing session, the data is packed and stored. See “PACK—Compress
Data” on page 260 and “PACK—Set or Query Pack Mode” on page 378 for more information.
The packed data format has the advantage of saving space. It allows for a more efficient use of DASD by
replacing repeating characters with a sequence that shows the repetition.
There are two disadvantages:
• The space saving is at the expense of additional processing when the data is read or written.
• The data cannot be directly accessed by programs. You must access the data through PDF dialogs and
library access services. You would not, for example, pack an executable such as a CLIST or REXX exec. A
packed CLIST or REXX exec would not run, because pack mode analysis is not done before the member
is passed to the system for execution.
Specifying z/OS UNIX pathnames with edit primary and macro
commands
These edit primary and macro commands support the specification of a z/OS UNIX pathname as an
operand:
• COMPARE
• COPY
• CREATE
• MOVE
• REPLACE
You can specify a pathname in the format accepted as input in the "Other Partitioned, Sequential or
VSAM Data Set, or z/OS UNIX file" data set name field. If you are editing a z/OS UNIX file when these
commands are used, you can specify a + (plus) as the first character of the pathname to represent the
pathname of the directory containing the file being edited. For example, if you are currently editing the
file /u/usr1/prog1, the command copy +/src1 copies in the data in file /u/usr1/src1.
Edit macros
Chapter 1. Introducing the ISPF editor  15

## Page 48

Edit macros
16  z/OS: z/OS ISPF Edit and Edit Macros
