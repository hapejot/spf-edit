# Chapter 3. View (option 1)

Source file: f54u200_v3r1.md
Start page: 103
Page span: 103-120

## Page 103

Chapter 3. View (option 1)
The View option (1) displays the View Entry Panel shown in Figure 52 on page 65.
Figure 52. View Entry panel (ISRBRO01)
This option enables you to view or browse source data and listings stored in ISPF libraries, other
partitioned or single-volume or multivolume sequential data sets, or z/OS UNIX files.
View
Allows you to use all Edit line commands, primary commands, and macros to manipulate the data.
View functions exactly like Edit, with the exception of these primary commands:
SAVE
When you enter the SAVE command, ISPF issues a message that you must use the CREATE
command to save any data you have changed.
END
When you enter the END command, ISPF terminates the View function; no data changes are
saved.
Browse
Allows you to use the Browse primary commands described in “Browse primary commands” on page
69 to manipulate data.
View is enabled by default. You can disable View, thus allowing only Browse, by modifying the ISPF
Configuration Table. You must set the keyword IS_VIEW_SUPPORTED to NO. For more information, see
the topic about the ISPF Configuration Table in z/OS ISPF Planning and Customizing.
You can view or browse data that has these characteristics:
• Record Format (RECFM):
© Copyright IBM Corp. 1980, 2024 65

## Page 104

– Fixed, variable (non-spanned), or undefined
Note: If you try to view a data set with RECFM=U, the Browse function is substituted.
– Blocked or unblocked
– With or without printer control characters
• Logical Record Length (LRECL):
– For fixed-length records, up to 32 760 characters. For view only, the minimum record length is 1
character.
– For variable-length records, up to 32 756 characters. For view only, the minimum record length is 5
characters.
• VSAM data
– VSAM data can be browsed if the ISPF Configuration table has been customized to enable VSAM
support (that is, VSAM_BROWSE_ENABLED or VSAM_VIEW_ENABLED is set to "YES").
Note: When VSAM support is enabled, the default value for VSAM_BROWSE_COMMAND is "FMNINV
DSB /" and for VSAM_VIEW_COMMAND is "FMNINV DSV /". If the command is not available,
IKJ56500I COMMAND FMNINV NOT FOUND, is issued as a TSO message.
• z/OS UNIX files.
View Entry Panel action bar
The View Entry Panel action bar choices function as follows:
Menu
See the details about the Action Bar Choice in the ISPF User Interface topic of the z/OS ISPF User's
Guide Vol I for information about the Menu pull-down.
RefList
See the Using Personal Data Set Lists and Library Lists topic in the z/OS ISPF User's Guide Vol I for
information about referral lists.
RefMode
See the information about Personal List Modes in the Using Personal Data Set Lists and Library Lists
topic in the z/OS ISPF User's Guide Vol I for information about referral list modes.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about the options and commands available in View,
as well as information about each available choice on the View Entry Panel.
View Entry Panel fields
The "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I contains information about
all the fields on the View Entry Panel except these:
Initial Macro
You can specify an Edit macro to be processed before you begin viewing your sequential data set
or any member of a partitioned data set. This initial macro enables you to set up a particular
environment for the View session you are beginning.
If you leave the Initial Macro field blank and your Edit profile includes an initial macro specification,
the initial macro from your Edit profile is processed.
If you want to suppress the processing of an initial macro in your Edit profile, enter NONE in the Initial
Macro field.
66  z/OS: z/OS ISPF User's Guide Vol II

## Page 105

Profile Name
You can specify a profile name to override the default Edit profile.
Format Name
Contains the name of a format definition, which is used to view or browse a formatted data set.
Browse Mode
Specifies that you want to browse the data set using the Browse function. This function is useful for
large data sets and data sets that are formatted RECFM=U.
Confirm Cancel/Move/Replace
Specifies that you want ISPF to display a confirmation panel whenever you issue a Cancel, Move, or
Replace command.
Mixed Mode
Specifies that you want to view or browse unformatted data that contains both EBCDIC and DBCS
characters.
Warn on First Data Change
Specifies that you want ISPF to warn you that changes cannot be saved in View. The warning is
displayed when the first data change is made.
Record Length
Can be used when browsing a z/OS UNIX file. The numeric value entered in this field is used by
ISPF to display the data in the file as fixed-length records, rather than using the newline character to
delimit each record. This is useful for browsing files which would otherwise have very large records if
the newline character is used as the record delimiter.
Line Command Table
Use this field to define a set of user line commands that you can use during the view session. The
table you specify can be generated using the ISPF table editor and contains the line commands that
you wish to have available and associates each line command with an edit macro that will be run if the
line command is entered during the view session.
PDSE Generation
This field gives you the opportunity to specify a generation number. You can use this field only when
you specify a PDS member in the ISPF Library or Other Data Set field.
Enter an absolute (positive) generation number or a relative (negative) generation number in this field
to view or browse a non-current generation of the member. This is valid only when the member is in a
PDSE Version 2 data set that is configured for member generations.
Data Encoding
You can use this option to select whether to view data as ASCII (CCSID 819) or UTF-8 (CCSID 1208).
You can also specify this option when creating a new file, data set, or member containing ASCII or
UTF-8 data. When you select a value for this option, the editor uses the selected CCSID in converting
the data to the CCSID for the terminal.
For ASCII or UTF-8 z/OS UNIX files, the editor breaks up data into records using the ASCII linefeed
character (X'0A') and the ASCII carriage return character (X'0D') as the record delimiter. The linefeed
and carriage return characters are removed from the data loaded into the editor, but written back to
the file when the data is saved.
It is not necessary to use the Data Encoding option when the z/OS UNIX file is tagged with a CCSID
of 819 or 1208. If ISPF detects the file is tagged with CCSID 819 or 1208, it converts the data from
ASCII or UTF-8 to the CCSID of the terminal. When the file is saved, ISPF ensures the file is tagged
with a CCSID of 819 or 1208.
Browsing a data set
If you select Browse Mode on the View Entry Panel, ISPF displays either a member selection list or a
Browse data display. For information about displaying a member list, see the topic about Using Member
Selection Lists in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
Browsing a data set
Chapter 3. View (option 1)  67

## Page 106

Note: If you specify a volume serial on the View Entry Panel, you can browse a single volume of a
non-SMS multivolume data set.
A Browse data display is shown in Figure 53 on page 68.
Figure 53. Browse - data display (ISRBROBA)
Each character in the data that cannot be displayed is changed on the display to either a period or
a character that you have specified. Using the DISPLAY command, you can specify whether printer
carriage-control characters are to be treated as part of the data, and thus displayed.
You can browse data that is stored in a Unicode format. MVS Conversion Services must first be set up for
the appropriate conversions. See z/OS ISPF Planning and Customizing.
During Browse, four-way scrolling is available through the scroll commands. You can also use the FIND
and LOCATE commands to scroll to a particular character string, line number, or symbolic label.
Whenever you enter a command, such as FIND or one of the scroll commands, that puts the cursor under
a character string in the data set, ISPF highlights that character string. This highlighting occurs whether
you type the command on the command line and press Enter or press a function key that the command is
assigned to.
Ending browse
To end a Browse data display, use the END command. This returns you to the previous panel, which
is either a member list display or the View Entry panel. If a member list is displayed, the name of the
member you just browsed is at the top of the list. You can select another member from the list or enter
the END command again to return to the View Entry Panel.
When the View Entry Panel is displayed again, you can select another data set or member, or you can use
the END command to return to the ISPF Primary Option menu.
Browsing a data set
68  z/OS: z/OS ISPF User's Guide Vol II

## Page 107

Browse primary commands
You can prefix any BROWSE command with an ampersand to keep the command displayed on the
command line after the command has been processed. This technique allows you to repeat similar
commands without reentering the data. For example, if you enter:
Command ===> &FIND ABCD
the command is displayed after the string has been found, which allows you to then change the operand
and issue another FIND command.
Browse provides the functions described in these topics, each of which is controlled by a command that
you can type on the command line:
“BROWSE—browse recursively” on page 69
“COLUMNS—identify columns” on page 70
“DISPLAY—control the display” on page 70
“EDIT—edit a member” on page 72
“FIND—find character strings” on page 73
“HEX—display data in hexadecimal format” on page 77
“LOCATE—locate lines” on page 79
“RESET—remove the column-identification line” on page 80
“SUBMIT—submit a job stream for background execution” on page 80
“VIEW—view a member” on page 80
BROWSE—browse recursively
The BROWSE command allows you to browse another member of the same data set. It also allows you to
browse any other data set or a z/OS UNIX file without ending your current Browse session.
The BROWSE command has this syntax:
BROWSE
member
GEN generation
where:
member
An optional member of the ISPF library or other partitioned data set that you are currently browsing.
You may enter a member pattern to generate a member list.
generation
The generation of the member to be browsed. You might enter an absolute (positive) generation
number or a relative (negative) generation number. This parameter is valid only when the member is in
a PDSE version 2 data set that is configured for member generations.
For example, if you were browsing a member of library ISPFDEMO.XXX.COBOL, you could enter this
command to display the panel shown in Figure 53 on page 68:
BROWSE CBLMAIN
If library ISPFDEMO.XXX.COBOL is a PDSE version 2 data set that is configured for member generations,
you could enter this command to display the previous generation of the panel:
BROWSE CBLMAIN GEN -1
If you do not specify a member name, the Browse Command - Entry Panel is displayed.
You end a nested Browse session the same way you would a normal one. When you end the nested
Browse session, the current Browse session resumes.
Browsing a data set
Chapter 3. View (option 1)  69

## Page 108

COLUMNS—identify columns
You can use the COLUMNS command to provide a temporary indication of where columns occur on the
panel. This command displays a column-identification line on the first line of the data area. The command
has this syntax:
COLUMNS
ON
OFF
where:
ON
The default. Displays the column-identification line.
OFF
Removes the column-identification line from the display.
Note: You can also remove the column-identification line by entering the RESET command.
An example of the column-identification line is shown in Figure 54 on page 70. The digits on the
identification line show the tens positions: 1 shows column 10, 2 shows column 20, and so forth. The plus
signs (+) show the fives positions (columns 5, 15, 25, and so forth.)
   Menu  Utilities  Compilers  Help
 ───────────────────────────────────────────────────────────────────────────────
 BROWSE    SBURNF.CALL.TRACE                          Line 00000017 Col 001 080
 Command ===>                                                  Scroll ===> CSR
----+----1----+----2----+----3----+----4----+----5----+----6----+----7----+----8
     ISPVCALL Command: ISPVCALL
     ISPF Invocation   ISPF
==============================< Cached Panels >=================================
          ISR@PRIM
==========================< Active Command Tables >=============================
          USERCMDS  SITECMDS  ISPCMDS
======================< Entries in Open Command Tables >========================
   ⋮
Figure 54. Browse - c olumn -identific ation  line (ISRBROBA)
DISPLAY—control the display
The DISPLAY command allows you to view data that would not normally be displayed, such as carriage
control characters and Unicode data. For other nondisplayable data you can specify a character to
represent each character that cannot be displayed.
The DISPLAY command has two formats. The first is used to display carriage-control characters and
other nondisplay characters. The second allows data stored using a Unicode CCSID (Coded Character Set
Identifier) to be displayed using the CCSID of the terminal.
Carriage-control characters and other nondisplay characters
DISPLAY
char
NOCC
CC
NORDW
RDW
You must enter at least one operand, but you can enter them in any order. If you enter only one operand,
the other operand retains its current value.
where:
char
The character you want to use to represent characters that cannot be displayed on the screen. It can
be a single character, or a single character enclosed in apostrophes (') or quotation marks ("). If you
specify a blank as the character, you must enclose it in apostrophes or quotation marks.
Browsing a data set
70  z/OS: z/OS ISPF User's Guide Vol II

## Page 109

CC
Shows that carriage control characters are to be displayed and are to be considered part of the data.
NOCC
Shows that carriage control characters are not to be displayed and are not to be considered part of the
data.
RDW
Indicates that the record descriptor word (RDW) is to be displayed and hex mode is to be turned on.
Only applicable when the data consists of variable length records. If hex mode is turned off using
the HEX command, display of the record descriptor word is also turned off. The RDW is a 4-byte
field describing the record. The first 2 bytes contain the length of the logical record. The length value
includes the length of the RDW as well as any carriage control characters, even when they are not
displayed.
NORDW
Indicates that the record descriptor word is not to be displayed. When display of the record descriptor
word is turned off, hex mode is also turned off. Only applicable when the data consists of variable
length records.
The char, CC, and NOCC operands are stored in your user profile and are in effect whenever you are using
Browse. You need to reenter the DISPLAY command only if you want to change one of these operands.
The RDW and NORDW operands are not stored in your user profile. The initial settings for display mode
are period (.), NOCC, and NORDW, but the carriage control character status has no effect if the data that
you are browsing has no carriage control characters.
Unicode data
DISPLAY
LINE
start_line end_line
COLS
start_col end_col
CCSID ccsid_number
ASCII
USASCII
EBCDIC
UCS2
UTF8
UTF16
UTF32
RESET
where:
LINE start_line end_line
Specifies the number of the first and last lines within which Unicode data is displayed. If the LINE
keyword is not specified, the DISPLAY command applies to the all lines in the data. If only one value is
specified, the DISPLAY command only applies to that line.
COLS start_col end_col
Specifies the number of the first and last column within which Unicode data is displayed. If the COLS
keyword is not specified, the DISPLAY command applies to the full range of columns in the data. If
only one value is specified, the DISPLAY command only applies to that column.
CCSID ccsid_number | ASCII | USASCII | EBCDIC | UCS2 | UTF8 | UTF16 | UTF32
Specifies the CCSID of the data. The CCSID represents a code page and character set combination.
RESET
This format of the command resets all definitions made with the DISPLAY command. All data is
displayed as though it had been stored using the terminal CCSID.
LINE and COLS are optional. LINE, COLS, and CCSID can be specified in any order.
You can issue multiple DISPLAY commands, in which case the specifications are merged. If a range of
data is specified more than once, later specifications take precedence over earlier specifications. For
example, if you specify one CCSID that applies to rows 3 to 10, then specify another CCSID that applies to
Browsing a data set
Chapter 3. View (option 1)  71

## Page 110

columns 30 to 60, the second CCSID takes effect in the area where the DISPLAY commands overlap—that
is, columns 30 to 60 in rows 3 to 10.
When you exit the current Browse session, all definitions are reset (as though you had entered DISPLAY
RESET).
Examples
• To use blanks to represent characters that cannot be displayed, enter:
DISPLAY " "
• To use a vertical bar (|) to represent characters that cannot be displayed, enter:
DISPLAY |
• To suppress the display of carriage control characters, enter:
DISPLAY NOCC
• To display columns 20 through 30 of lines 5 through 15 as though the data had been stored using a
CCSID of UTF16:
DISPLAY LINE 5 15 COLS 20 30 CCSID 1200
• To display lines 4 through 18 as though the data had been stored using a CCSID of UTF16, except for
column 72 in lines 12 through 18, which should be displayed as though the data had been stored using
a CCSID of ASCII:
DISPLAY LINE 4 18 UTF16
DISPLAY LINE 12 18 COLS 72 ASCII
• To revert to displaying the data as though it had been stored using the CCSID of the terminal:
DISPLAY RESET
EDIT—edit a member
The EDIT command allows you to edit another member of the same data set. It also allows you to edit any
other data set or z/OS UNIX file without ending your current Browse session.
The EDIT command has this syntax:
EDIT
member
GEN generation
where:
member
An optional member of the ISPF library or other partitioned data set that you are currently browsing.
You may enter a member pattern to generate a member list.
generation
The generation of the member to be edited. You might enter an absolute (positive) generation number
or a relative (negative) generation number. This parameter is valid only when the member is in a PDSE
version 2 data set that is configured for member generations.
For example, if you were browsing a member of library ISPFDEMO.XXX.COBOL, you could enter this
command to display the panel shown in Figure 53 on page 68:
EDIT CBLMAIN
Browsing a data set
72  z/OS: z/OS ISPF User's Guide Vol II

## Page 111

If library ISPFDEMO.XXX.COBOL is a PDSE version 2 data set that is configured for member generations,
you could enter this command to display the previous generation of the panel:
EDIT CBLMAIN GEN -1
If you do not specify a member name, the Edit Command - Entry Panel is displayed.
You end a nested Edit session the same way you would a normal one. When you end the nested Edit
session, the current Browse session resumes.
FIND—find character strings
The FIND command allows you to find a specified character string. The syntax of the FIND command is:
FIND string
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
left_col right_col
Note: FIND as a Browse command, shown here, has the same syntax as FIND as an Edit command,
except the optional X/NX/EX and line range operands are not included.
where:
string
Required operand. The character string you want to find.
NEXT | ALL | FIRST | LAST | PREV
Optional operands that define the starting point, direction, and extent of the search. NEXT is the
default.
CHARS | PREFIX | SUFFIX | WORD
Optional operand. Operands that set the conditions for a character string match. CHARS is the default.
UTF8 | ASCII | USASCII
Optional operand which specifies that the FIND string is either a UTF8, ASCII, or USASCII character
string. This form of the FIND command is only available for the character FIND command, and only for
UTF8, ASCII, or USASCII strings.
left_col and right_col
Optional operands. Numbers that identify the columns the FIND command is to search.
You can separate the operands with blanks or commas and you can type them in any order, but right_col,
if typed, must follow left_col.
Specifying find  strings
The string operand specifies the characters to be found. For examples of different string formats, refer to
the description of the FIND command in z/OS ISPF Edit and Edit Macros.
The default is not to differentiate between uppercase and lowercase characters when searching. Except
for the character (C) string, differences between uppercase and lowercase strings are ignored. For
example, this command:
FIND ALL 'CONDITION NO. 1'
successfully finds any of these:
Browsing a data set
Chapter 3. View (option 1)  73

## Page 112

CONDITION NO. 1
Condition No. 1
condition No. 1
condition no. 1
Omitting string delimiters
Generally, you enter the strings without delimiters. For example, to find all occurrences of ABC, enter:
FIND ALL ABC
Using string delimiters
You must use delimiters if a string contains embedded blanks or commas, or if a string is the same as a
command keyword. You delimit strings with either apostrophes (') or quotation marks ("). For example,
to find the next occurrence of every one, enter:
FIND 'every one'
The FIND command does not find the apostrophe or quotation mark string delimiters.
Note: The Browse FIND command does not work with a search argument that contains the command
delimiter, even if string delimiters are used. You can specify a hexadecimal search string or use ISPF
Option 0 to change the command delimiter to a different character.
Starting point, direction, and extent of search
You can control the starting point, direction, and extent of the search by using one of these operands:
NEXT
The scan starts at the first position after the current cursor location and searches ahead to find the
next occurrence of the string. NEXT is the default.
ALL
The scan starts at the top of the data and searches ahead to find all occurrences of the string.
A message in the upper-right corner of the screen shows the number of occurrences found. The
second-level message that is displayed when you enter the HELP command shows which columns
were searched.
FIRST
The scan starts at the top of the data and searches ahead to find the first occurrence of the string.
LAST
The scan starts at the bottom of the data and searches backward to find the last occurrence of the
string.
PREV
The scan starts at the first position before the current cursor location and searches backward to find
the previous occurrence of the string.
If you specify FIRST, ALL, or NEXT, the direction of the search is forward; pressing the RFIND function key
(F5/17) finds the next occurrence of the designated string. If you specify LAST or PREV, the direction of
the search is backward; pressing the RFIND function key finds the previous occurrence of the string. The
other optional operands remain in effect, as specified in the last FIND command. These operands include
CHARS, WORD, PREFIX, SUFFIX, and left_col, right_col.
The search proceeds until one or all occurrences of the string are found, or until the end of data is found.
If the string is not found, one of these actions takes place:
• If the FIND command was entered on the Command line, a NO string FOUND message is displayed in
the upper-right corner of the screen.
• If the FIND command was repeated using the RFIND command, either a BOTTOM OF DATA REACHED
message or a TOP OF DATA REACHED message is displayed, depending on the direction of the search.
When these messages appear, you can press the RFIND function key again to continue the search by
Browsing a data set
74  z/OS: z/OS ISPF User's Guide Vol II

## Page 113

wrapping to the top or bottom of the data. If the string is still not found anywhere in the data, a NO
string FOUND message is displayed.
Conditions for character string matches
The operands CHARS, PREFIX, SUFFIX, and WORD control the conditions for a successful match with the
string based on whether the data string begins and/or ends with a non-alphanumeric character; that is,
a special character or a blank. You can abbreviate PREFIX, SUFFIX, and CHARS to PRE, SUF, and CHAR,
respectively.
In this example, the highlighted strings would be found and the strings that are not highlighted would be
ignored:
CHARS 'DO'  - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
PREFIX 'DO' - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
SUFFIX 'DO' - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
WORD 'DO'   - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
If you do not specify an operand, the default is CHARS.
Using text strings
Text strings are processed exactly the same as delimited strings. They are provided for compatibility with
prior versions of the product.
Using character strings
A character string, which may be used as a string operand in a FIND command, requires that the search
be satisfied by an exact character-by-character match. Lowercase alphabetic characters match only with
lowercase alphabetic characters and uppercase alphabetic characters match only with uppercase.
If you specify a text string that contains any SO or SI characters, the string is considered a character
string.
Specifying the keyword UTF8, ASCII, or USASCII with this form of the FIND command will find
occurrences of 'string' within the data being browsed, where 'string' has been stored in the corresponding
CCSID format.
Here are some examples:
To find the next occurrence of the characters XYZ only if they are in uppercase:
FIND C'XYZ'
To find the next occurrence of the characters xyz only if they are in lowercase:
FIND C'xyz'
To find the next occurrence of the UTF-8 string 'Found' (but not 'FOUND', 'found', or 'FoUnD'):
FIND  C'Found' UTF8
Using picture strings
A picture string used as a string operand in a FIND command allows you to search for a particular type
of character, without regard for the specific character involved. You can use special characters within the
picture string to represent the type of character to be found, as follows:
String
Meaning
P'='
Any character
Browsing a data set
Chapter 3. View (option 1)  75

## Page 114

P'¬'
Any nonblank character
P'.'
Any nondisplayable (invalid) character
P'#'
Any numeric character (0-9)
P'-'
Any nonnumeric character
P'@'
Any alphabetic character (uppercase or lowercase).
String
Meaning
P'<'
Any lowercase alphabetic character
P'>'
Any uppercase alphabetic character
P'$'
Any special character (not alphabetic or numeric).
If an APL or TEXT keyboard is being used, this additional character can be used in a picture string:
P'
  '
Any APL-specific or TEXT-specific character
P'_'
Any underscored alphabetic APL character and delta.
Only the special characters listed are valid within a picture string, but the string can include alphabetic or
numeric characters that represent themselves.
A DBCS subfield cannot be specified as the subject of a picture string for the FIND command.
Examples of picture strings:
P'###'
A string of 3 numeric characters
P'¬ ¬'
Any 2 nonblank characters separated by a blank
P'.'
Any nondisplayable character
P' #'
A blank followed by a numeric character
P'#AB'
A numeric character followed by 'AB'.
Examples of FIND commands using picture strings:
FIND P'.'
Find next nondisplayable character
FIND P'¬' 72
Find next nonblank character in column 72
F P' ¬' 1
Find the next line with a blank in column 1 followed by a nonblank.
Browsing a data set
76  z/OS: z/OS ISPF User's Guide Vol II

## Page 115

When you use the special characters '=' or '.' and a nondisplayable character is found, a hexadecimal
representation is used in the confirmation message that appears in the upper-right corner of the screen.
For example:
FIND P'..'
could result in the message CHARS X'0205' FOUND.
Column limitations
The left_col and right_col operands allow you to search only a portion of each line, rather than the
complete line. These operands, which are integers separated by a comma or by at least one blank, show
the starting and ending columns for the search. These rules apply:
• If you specify neither left_col nor right_col, the search continues across all columns within the current
boundary columns.
• If you specify left_col without right_col, the string is found only if it starts in the specified column.
• If you specify both left_col and right_col, the complete string, not just part of it, must be within the
specified columns.
• If the DISPLAY RDW command is entered to display the record descriptor word, the left_col and
right_col operands should not include the 4-byte record descriptor word that appears at the start of
each displayed record.
Using RFIND
The RFIND command, which is usually assigned to the F5/17 key, allows you to repeat the previous FIND
command without retyping it. Therefore, you can use this command to find successive occurrences of the
string specified in the last FIND command. You can also use the RFIND command to return to the top of
your data and continue searching when the BOTTOM OF DATA REACHED message appears. If you enter
the RFIND command on the Command line instead of using a function key, you must position the cursor to
the desired starting location before pressing Enter.
If you specify a 1-byte hexadecimal string as the FIND string and the string is found at the second byte of
a double-byte character set (DBCS) character, hardware sets the cursor to the first byte of the character.
If you then request RFIND, the same data is found again. To find the next occurrence of the string, you
must move the cursor to the next character position before requesting RFIND again.
HEX—display data in hexadecimal format
The HEX command causes data to be displayed in hexadecimal format. The syntax of the command is:
HEX ON
VERT
DATA
VERT
DATA
OFF
You can specify the operands in any order:
where:
ON
Turns hexadecimal mode on. This is the default.
OFF
Turns hexadecimal mode off. If the DISPLAY RDW command was used to display the record descriptor
word, display of the record descriptor word is also turned off.
Browsing a data set
Chapter 3. View (option 1)  77

## Page 116

VERT
Valid only when hexadecimal mode is ON. This is the default. Figure 55 on page 78 shows how
VERT causes the hexadecimal representation to be displayed vertically, two rows per byte, under each
character.
DATA
Valid only when hexadecimal mode is ON. Figure 56 on page 79 shows how DATA causes the
hexadecimal representations to be displayed as a string of hexadecimal characters, two per byte.
Because the hexadecimal string is twice the length of the data string, it occupies two rows. If you omit
this operand, VERT is assumed.
When using browse and placing the cursor anywhere within the record, SCROLL UP positions the data
where the cursor is located as the last complete line record on the display. A complete line record
consists of the standard character form line, two hexadecimal character lines, and a separator line.
For example, this command would display the hexadecimal notation vertically:
HEX VERT
Three lines are displayed for each source line. The first line shows the data in standard character
form. Figure 55 on page 78 shows the next two lines with the same data in vertical hexadecimal
representation. A separator line is displayed between the two representations to make it easier for you to
read the data.
Figure 55. Browse hexadecimal display - vertical (ISRBROBA)
To display the hexadecimal notation horizontally, use this command:
HEX DATA
Figure 56 on page 79 shows the next two lines with the same data in DATA hexadecimal representation.
Browsing a data set
78  z/OS: z/OS ISPF User's Guide Vol II

## Page 117

Figure 56. Browse hexadecimal display - data (ISRBROBA)
You can use the FIND command to find invalid characters or any specific hexadecimal character
regardless of the setting of hexadecimal mode. See the syntax for picture strings and hexadecimal strings
under the description of the FIND command in z/OS ISPF Edit and Edit Macros.
LOCATE—locate lines
Use the LOCATE command to bring a particular line to the top of the display. You can identify the line by
either its relative line number or a previously defined label.
During Browse, the current position of the screen window is shown by the line/column numbers in the
upper-right corner of the screen. The line number refers to the first line of data following the two header
lines, and shows the relative position of that line in the data. The Top of Data message is treated as
relative line zero. You must enter either a line number or a label as an operand.
LOCATE line-number
label
where:
line-number
A numeric value less than 2147483648 that shows the position of the line from the beginning of the
data. The line number is displayed in the upper-right corner.
label
Defined by scrolling to the top of the screen the line with which you want to associate the label. You
then type the label on the command line in the form:
.ccccccc
Browsing a data set
Chapter 3. View (option 1)  79

## Page 118

For example, to find line 18463, you could enter this command:
LOCATE 18463
ISPF then moves line 18463 to the top of the screen. You can assign a label to it by entering:
.label
The label is a period followed by up to seven characters that can be displayed, except the comma and the
space. It is treated as an internal symbol and is equated to the top line on the screen. You are required to
specify the period when you define the label. The next time you want to find this line, you can enter:
LOCATE .label
The period is usually optional when you use it as an operand in a LOCATE command. However, if the first
character in the label is a number, you must specify the period to distinguish the label from a line number.
The latest assignment of a label overrides any previous assignments. You can assign several labels to the
same line. Labels are not retained when you leave the Browse option.
RESET—remove the column-identification line
The RESET command removes the column-identification line that you can display by using the COLUMNS
command (see “COLUMNS—identify columns” on page 70). This command has no operands.
SUBMIT—submit a job stream for background execution
The SUBMIT command is used to submit a job stream that is being browsed. If the data set being
browsed is modified and saved by another user or by the same user on another screen, the SUBMIT
command will submit the updated data set, not the copy being browsed. The TSO SUBMIT command is
invoked directly to submit the job stream, so the data set has to be fixed-record format with a record
length of 80.
Note: The Browse SUBMIT command is not supported if the underlying data is packed.
The syntax of the command is:
SUBMIT
SUBSYS ( subsystem )
where:
subsystem
An optional emergency subsystem name which identifies where the job will run. It is limited to 4
characters.
VIEW—view a member
The VIEW command allows you to view another member of the same data set. It also allows you to view
any other data set or z/OS UNIX file without ending your current Browse session.
The VIEW command has this syntax:
VIEW
member
GEN generation
where:
member
An optional member of the ISPF library or other partitioned data set that you are currently browsing.
You may enter a member pattern to generate a member list.
Browsing a data set
80  z/OS: z/OS ISPF User's Guide Vol II

## Page 119

generation
The generation of the member to be viewed. You might enter an absolute (positive) generation
number or a relative (negative) generation number. This parameter is valid only when the member is in
a PDSE version 2 data set that is configured for member generations.
For example, if you were browsing a member of library ISPFDEMO.XXX.COBOL, you could enter this
command to display the panel shown in Figure 53 on page 68:
VIEW CBLMAIN
If library ISPFDEMO.XXX.COBOL is a PDSE version 2 data set that is configured for member generations,
you could enter this command to display the previous generation of the panel:
VIEW CBLMAIN GEN -1
If you do not specify a member name, the View Command - Entry Panel is displayed.
You end a nested View session the same way you would a normal one. When you end the nested View
session, the current Browse session resumes.
Browsing a data set
Chapter 3. View (option 1)  81

## Page 120

Browsing a data set
82  z/OS: z/OS ISPF User's Guide Vol II
