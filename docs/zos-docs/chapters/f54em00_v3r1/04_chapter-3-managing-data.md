# Chapter 3. Managing data

Source file: f54em00_v3r1.md
Start page: 73
Page span: 73-100

## Page 73

Chapter 3. Managing data
This topic gets you started using some of the basic line and primary commands to manipulate data.
The basic functions of the ISPF editor are similar to those of a word processor. You can create, copy,
move, search, and replace data, as well as perform several other word processing functions by using the
line and primary commands described in this chapter .
Creating and replacing data
Use the CREATE command to add a new member to a partitioned data set, create a new sequential data
set, or create a new z/OS UNIX file. Use the REPLACE command to rewrite a member, sequential data
set, or z/OS UNIX file. The process of creating and replacing data is very similar. However, remember that
when you replace data, the original data is deleted and replaced with the new data.
There are two ways you can use CREATE or REPLACE:
1. You can type either CREATE or REPLACE on the command line, followed by the name of a member,
the name of a data set and member, the name of a sequential data set, or the name of a z/OS UNIX
file to be created or replaced. You can add line labels that show the lines to be copied. If you omit the
labels, you can use the C (copy) or M (move) line commands to specify which lines are to be copied or
moved. Then press Enter. See “CREATE—Create Data” on page 215 and “REPLACE—Replace Data” on
page 269 for the complete syntax of the commands.
2. If you omit the member name, data set name and member, sequential data set name, or z/OS UNIX
file name, and just type CREATE or REPLACE and specify the lines to be used to create or replace the
member, the editor displays a panel requesting the name of the member or data set you want created
or replaced.
If you try to create or replace data that has inconsistent attributes (for example, replacing a sequential
data set with a member of a partitioned data set), the editor displays a warning and gives you an
opportunity to cancel the command:
                           EDIT - Confirm Replace
 Data set attributes are inconsistent. Truncation may result in
 the right-most portions of some records if replace is performed.
  "Target" data set attributes:
      Data set name. : USERID.PRIVATE.STUFF
      Record format. : VARIABLE
      Record length. : 133
  "Current" data set attributes:
      Data set name. : USERID.PRIVATE.EXEC(PGM1)
      Record format. : VARIABLE
      Record length. : 251
 Press ENTER key to allow replace with truncation.
 Enter END command to cancel replace.
Copying and moving data
While you are editing, you can copy or move another data set, member, or z/OS UNIX file into the current
data by using the COPY or MOVE primary commands. The process of moving and copying data is very
similar. However, remember that when you move data, the original information no longer exists in the
member, data set, or file that it is being moved from.
When moving or copying large data sets, you can reduce the processing time significantly by specifying
NUMBER OFF before the operation and NUMBER ON afterwards.
Creating and replacing data
© Copyright IBM Corp. 1984, 2024 41

## Page 74

This topic explains how to use the COPY and MOVE primary commands. See “C—Copy Lines” on page 152
and “M—Move Lines” on page 166 for information about the line commands.
The two ways to perform a move or copy operation are:
• You can type either COPY or MOVE, followed by name and either AFTER label or BEFORE label, where
name is the name of the member, data set, or z/OS file to be copied or moved and label is a label that is
defined in the line command field. The label can be defined by PDF, such as .ZFIRST for the first line of
data, or it can be one that you have defined. If you omit the label, you can use the A (after) or B (before)
line command to specify where the information is to go. When you press Enter, the member is copied
or moved. See “COPY—Copy Data” on page 210 and “MOVE—Move Data” on page 252 for the complete
syntax of the commands.
• If you omit the member name, data set name, or z/OS file, and just type the command and the
destination of the operation (using either the AFTER label or BEFORE label operand or the A or B line
command), the editor displays a panel on which you can specify the name of the member, data set, or
z/OS UNIX file to be copied or moved. The only difference between the Edit Move and Edit Copy panels
is that with Copy, you can specify the number of lines you want copied.
Note: When using the ASCII (or UTF-8) edit facility with a z/OS UNIX file and the COPY or MOVE
command is issued specifying another z/OS UNIX file as the source, ISPF checks if the CCSID of the
source file is set to 819 (1208 for UTF-8). If so, ISPF assumes it contains ASCII (or UTF-8) data. This
means when ISPF reads the source file, the data is split into records by using the ASCII (and UTF-8)
linefeed character (X'0A') and the ASCII (and UTF-8) carriage return character (X'0D') as the record
delimiter. For information on the ASCII edit facility, see “Working with ASCII data” on page 51. For
information on the UTF-8 edit facility, see “Working with UTF-8 data” on page 52.
Shifting data
When you edit data, the editor automatically shifts characters on a line to the left or right to accommodate
insertions or deletions. This shifting can be either implicit or explicit . Implicit shifts occur when the
CHANGE command string2 length is different from the string1 length. Explicit shifts occur when you use
these commands:
• Line commands
(
Column Shift Left
)
Column Shift Right
<
Data Shift Left
>
Data Shift Right
• Macro commands
Shift (
Column Shift Left
Shift )
Column Shift Right
Shift <
Data Shift Left
Shift >
Data Shift Right
See the descriptions of these commands for the syntax and examples of usage.
Shifting data
42  z/OS: z/OS ISPF Edit and Edit Macros

## Page 75

Two columns is the default for shift operations. When shifting a block of lines more or less than the
default, enter the amount on the first or last line of the block. If you enter it in both places, the line shifts
only if:
• Both amounts are the same, or
• The amounts differ, but one is the default (2). Here, the lines shift according to the non-default amount.
If the shift amounts are different and neither amount is the default, an error message appears and the
shift is not performed.
Shifting occurs within column boundaries. The default boundaries are typically the first and last columns
in which you can type source code for the particular programming language. See “Edit boundaries” on
page 23 for a discussion of default boundaries and the procedures for changing them.
Column shift
The simplest kind of shift is a column shift. Column shifting moves all characters within the bounds
without altering their relative spacing. Characters shifted past the bounds are deleted. That is, blanks
are inserted at the bound from which the characters are being shifted, and the characters are deleted at
the opposite bound. So, this shift is called a destructive shift because information shifts within column
boundaries without regard to its contents, and can result in the loss of data with no error being noted.
If the UNDO mode was on before you entered the shift command, you can recover by using the UNDO
command. Otherwise, you can use CANCEL.
Column shifting in lines that contain DBCS strings
These rules apply:
• If half of a DBCS character is in the shift, it is excluded from the operation; the shift count is changed
automatically.
• If a column shift causes a DBCS string and an EBCDIC string to be connected, a shift-out or shift-in
character, as appropriate, is inserted between the strings. The shift count is changed automatically.
• If left, right, or both boundaries are set, a DBCS character can cross the boundary. The DBCS character
that crosses the boundary is excluded from the operation, and the shift count is changed automatically.
• If a request to shift an odd number of columns causes an odd-length DBCS string, the requested shift
number is discarded. The shift is processed up to the next field boundary within the boundary, if any. If
no field boundary is found, the line number is replaced with this intensified warning message: ==ERR>.
Also, the short message for an incomplete data shifting error is displayed.
If you are using the column shifting or data shifting line command while editing a formatted data set, note
these points:
• The current boundaries are automatically changed during command processing, and are reset to the
original values after processing is complete. Changes are as follows:
– If the left boundary falls on the second byte of a DBCS character in a DBCS field, the boundary is
shifted to the left by 1 byte.
– If the right boundary does not fall on the same field as the left boundary, it is set to point to the last
byte of the field that contains the left boundary. If it falls on the same DBCS field as the left boundary,
and it also falls on the first byte of a DBCS character, the right boundary is shifted to the right by 1
byte.
• If you use the data shift or column shift line command to shift a DBCS field and you specify an
odd-length shift amount, the shift amount is decreased by one to preserve DBCS data integrity.
• If a shift cannot be completed, it is partially done and the line number is replaced by this intensified
warning message: ==ERR>. Remove the message by issuing the RESET primary command, or type over
the message or data on that line.
Shifting data
Chapter 3. Managing data  43

## Page 76

• If a request to shift an odd number of bytes causes an odd-length DBCS string, the shift volume is
decreased by one and the operation is performed. The line number is replaced with this intensified
warning message: ==ERR>.
Data shift
Data shifting attempts to shift the body of a program statement without shifting the label or comments,
and prevents loss of data. This shift is non-destructive because it stops before it shifts a nonblank
character past the bound. This shift is explicitly done with the < and > line commands, and the SHIFT <
and SHIFT > macro commands. The CHANGE command can cause an implicit shift of the same nature.
For data shift left attempts that exceed the current BOUNDS setting, text stops at the left bound and PDF
marks the shifted lines with ==ERR> flags. If an error occurs in an excluded line, you can find the error
with LOCATE, and remove the error flag by using RESET.
Data shifts are designed to work with typical program sources. In doing so, it makes certain general
assumptions about the format of the source code. For instance, the editor assumes:
• Anything beginning at the left bound is a label and should not be shifted.
• If there are two or more consecutive blanks, one can be added or deleted.
• Blanks within quotes (' or ") are to be treated as nonblanks.
• Source statements appear on the left followed by comments on the right.
• Single blanks are used between source code and comment words. Therefore, the only strings of
multiple blanks appear between the source code and the comment, and between the comment and
its ending delimiter (if there is one). In this example, LABEL and */ are at the left and right bounds,
respectively:
LABEL: DO I=1 TO 5;          /* The comment...   */
     A=A+B(I);               /* The comment...   */
     END;
Keeping the previous assumptions in mind, the editor attempts to move only the source code statement
when shifting data. The label and comments are left unchanged. However, if necessary, it shifts the
comment also.
Although the editor always uses these assumptions, data shifting is not language-sensitive. It only makes
generalities about syntax and individual code entry style.
Finding, seeking, changing, and excluding data
FIND, SEEK, CHANGE, and EXCLUDE allow you to find a specified search string, change one search string
to another, or exclude a line containing a specified search string. These commands provide powerful
editing functions because they operate on a complete data set rather than on a single line.
The characteristics of each command follow:
FIND
Causes all lines that it finds to be displayed, and moves the cursor (scrolling if necessary) to the first
occurrence of the search string.
SEEK
A special form of FIND that can only be used in an edit macro. It is different from FIND in that it does
not change the exclude status of the lines found.
CHANGE
Causes the same effect as FIND, but it also has a second string operand (string2). During a search,
whenever string1 is found, the editor replaces that string with string2. Data to the right is shifted, if
necessary.
EXCLUDE
Causes lines that match the search not to be displayed. These lines remain in the data, however.
Unlike FIND and CHANGE, it does not require a search string if you use the ALL operand. EXCLUDE
Finding, seeking, changing, and excluding data
44  z/OS: z/OS ISPF Edit and Edit Macros

## Page 77

ALL is often used with FIND and CHANGE because they cause excluded lines to be redisplayed. Use
RESET to cause all lines to be redisplayed.
The scrolling and positioning of the string can be controlled using the Edit_Settings action bar choice or
the EDITSET primary command when editing the data. See “EDITSET—Display the Editor Settings Dialog”
on page 227 for more information.
The syntax of each command is a variation of that listed here. See the command descriptions in Chapter
10, “Edit primary commands,” on page 191 and Chapter 11, “Edit macro commands and assignment
statements,” on page 295 for the exact syntax.
string
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
Specifying the search string
The primary control for any search is the search string because it represents the value for which you are
looking. Two operands, string1 and string2, are required for the CHANGE command to specify the new
value of the string once it is found. The rules for specifying string1 and string2 are the same, except that if
you type a single asterisk for string2, the previous value is used again.
You can define string, string1, and string2 to be EBCDIC, DBCS, and mixed strings in any combination. If
you delimit a DBCS search string with SO and SI characters, the SO and SI characters are not used as part
of the string. If you specify a mixed string that contains no EBCDIC characters, the string is treated as a
DBCS string; that is, the SO and SI characters are not used as part of the string.
The editor allows you to specify these kinds of strings:
Simple string
Any series of characters not starting or ending with a quote (' or ") and not containing any embedded
blanks, commas, or asterisks.
Delimited string
Any string enclosed (delimited) by either single quotes ( ' ) or double quotes ( " ). The beginning
and ending delimiters must be the same character. The string can contain the delimiter character.
However, if a delimiter character in the string is followed by a blank ( ) or a comma ( , ), that delimiter
character is processed as the ending delimiter. A delimiter character in the string is also processed
as the ending delimiter if it is followed by the letter c, p, r, t, or x. In these cases, the letter is
processed as an indication that the preceding string is a character, picture, regular expression, text, or
hexadecimal string.
Hexadecimal string
Any delimited string of valid hexadecimal characters, preceded or followed by the character X, such as
X'C27B'.
Character string
Any delimited string of characters, preceded or followed by the character C, such as C'conditions
for'. See “Character strings” on page 46 for more information.
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  45

## Page 78

Picture string
Any delimited string of picture characters, preceded or followed by the character P, such as P'.'. See
“Picture strings (string, string1)” on page 46 and “Picture strings (string2)” on page 48 for more
information.
Basic or extended regular expression
Any delimited string of characters, preceded or followed by the character R, such as R'h[aeiou]d',
or the characters RC, such as RC'M[ai]ster'. Use RC to request a case sensitive search be
performed. See “Regular expressions (string, string1)” on page 48 for more information.
Note: The Edit FIND, CHANGE, and EXCLUDE commands do not work with a search argument that
contains the command delimiter, even if string delimiters are used. You can specify a hexadecimal search
string or use the SETTINGS command to change the command delimiter to a different character.
Simple and delimited strings
If the string is a simple or delimited string, the characters are treated as being both upper and lowercase
even if caps mode is off. For example, this command:
FIND ALL 'CONDITION NO. 1'
successfully finds:
CONDITION NO. 1
Condition No. 1
condition no. 1
coNDitION nO. 1
Also, all of these commands have the same effect:
FIND 'Edit Commands'
FIND 'EDIT COMMANDS'
FIND 'edit commands'
You must use delimiters if a string contains embedded blanks or commas, or if a string is the same as
a command or keyword. You delimit strings with quotes, either ' or ". For example, to change the next
occurrence of EVERY ONE to ALL, type:
CHANGE 'EVERY ONE' 'ALL'
Note: When using a DBCS terminal, if you specify a text string that contains any SO and SI characters, the
string is considered a character string.
Character strings
Use a character string in a FIND, CHANGE, or EXCLUDE command if you want the search to be satisfied
by an exact character-by-character match. Lowercase alphabetic characters match only with lowercase
alphabetic characters, and uppercase alphabetic characters match only with uppercase.
For example, FIND C'XYZ' finds the characters XYZ only, not xyz.
Picture strings (string, string1)
A picture string in a FIND, CHANGE, or EXCLUDE command allows you to search for a particular kind of
character without regard for the specific character involved. You can use special characters within the
picture string to represent the kind of character to be found, as follows:
• String
Meaning
P'='
Any character
P'¬'
Any character that is not a blank
Finding, seeking, changing, and excluding data
46  z/OS: z/OS ISPF Edit and Edit Macros

## Page 79

P'.'
Any character that cannot be displayed
P'#'
Any numeric character, 0-9
P'-'
Any nonnumeric character
P'@'
Any alphabetic character, uppercase or lowercase
P'<'
Any lowercase alphabetic character
P'>'
Any uppercase alphabetic character
P'$'
Any special character, neither alphabetic nor numeric
If you are using an APL or TEXT keyboard, you can use these additional characters in a picture string:
• P'
 '
Any APL-specific or TEXT-specific character
P'_'
Any underscored nonblank character
A picture string can include alphanumeric characters, which represent themselves, mixed with other
characters. If the character does not have a special meaning (such as @ standing for any alphabetic), the
character is treated as itself.
When using a DBCS terminal, you cannot specify a DBCS field as the subject of a picture string for the
FIND operation.
When processing a picture string the interpretation of characters is based on the PDF terminal translation
tables that are loaded. For example, characters that cannot be displayed are determined using the
translation tables for generic string characters and generic string special characters.
Picture string examples
• To find a string of 3 numeric characters:
– FIND P'###'
• To find any 2 characters that are not blanks but are separated by a blank:
FIND P'¬ ¬'
• To find any character that cannot be displayed:
FIND P'.'
• To find a blank followed by a numeric character:
FIND P' #'
• To find a numeric character followed by AB:
FIND P'#AB'
• To find the next character in column 72 that is not a blank:
FIND P'¬' 72
• To change any characters in columns 73 through 80 to blanks:
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  47

## Page 80

CHANGE ALL P'=' ' ' 73 80
• To find the next line with a blank in column 1 and a character in column 2 that is not a blank:
FIND P' ¬'  1
When you use the special characters = or . and a character that cannot be displayed is found, that
character's hexadecimal representation is used in the confirmation message that appears in the upper-
right corner of the panel. For example, the command FIND P'..' could result in the message CHARS
X'0275' FOUND.
Picture strings (string2)
In a CHANGE command, string2 can be a picture string with these rules and restrictions:
• The length of string2 must be the same as the length of string1.
• The only valid special characters are =, >, and <.
String
Meaning
P'='
Equal to the corresponding character in string1
P'>'
Converts the corresponding character in string1 to uppercase
P'<'
Converts the corresponding character in string1 to lowercase
Picture string examples
• To change an alphabetic, alphabetic, numeric, numeric string so that the alphabetic characters become
uppercase characters and the numeric characters are unchanged:
CHG P'@@##' P'>>=='
• To change all characters to uppercase:
CHG ALL P'<' P'>'
Regular expressions (string, string1)
A regular expression in a FIND, CHANGE, or EXCLUDE command allows you to search for a string matching
a basic or extended regular expression.
ISPF uses the IBM C regcomp() — Compile regular expression (www.ibm.com/
docs/en/zos/2.5.0?topic=functions-regcomp-compile-regular-expression) and regexec() — Execute
compiled regular expression (www.ibm.com/docs/en/zos/2.5.0?topic=functions-regexec-execute-
compiled-regular-expression) functions to compile and execute a regular expression specified with a
FIND, CHANGE, or EXCLUDE command. These are supported by the C runtime library and the C runtime
library must be available.
ISPF queries the host code page defined for your TN3270 session. If the code page is one of the
following:
 00037   00871   01123   01156
 00273   00875   01140   01157
 00277   00924   01141   01158
 00278   00930   01142   01160
 00280   00933   01143   01165
 00284   00935   01144   01364
 00285   00937   01145   01371
 00290   00939   01146   01388
 00297   01025   01147   01390
Finding, seeking, changing, and excluding data
48  z/OS: z/OS ISPF Edit and Edit Macros

## Page 81

00424   01026   01148   01399
 00425   01027   01149   04971
 00500   01047   01153   05123
 00838   01112   01154   08482
 00870   01122   01155   12712
ISPF uses the IBM C setlocale function with LC_ALL to set the corresponding C locale. This is done so
that the special symbols (such as square brackets) within the regular expression are correctly interpreted
when the regcomp function is used to compile the regular expression.
If the TN3270 code page is not one of the listed code pages then the default C locale is used when
compiling the regular expression. If a regular expression is encountered on a FIND, CHANGE, or EXCLUDE
command that is specified in an Edit macro that is called from a batch Edit session (where no terminal is
attached), code page 1047 is used.
The simplest form of regular expression is a string of characters with no special meaning.
The following characters do have a special meaning; they are used to form extended regular expressions:
Symbol
Description
. (period)
The period symbol matches any one character except the terminal newline character.
For example, the regular expression d.g matches "dig", "dug", and "dog", but not "dg", though it
matches "dgg".
* (asterisk)
The asterisk symbol matches zero or more instances of the previous character.
For example, the regular expression he*ath matches "hath" and "heath" and (if it exists) "heeath".
? (question mark)
The question mark symbol matches zero or one instance of the previous character.
For example, the regular expression behaviou?r matches "behaviour" and "behavior".
+ (plus)
The plus symbol matches one or more instances of the previous character.
For example, the regular expression south+ern matches "southern" and "southhern", but not
"soutern". (If you also wanted a match for "soutern", use south*ern as the regular expression.)
| (vertical bar)
The vertical bar symbol acts as an OR operator and matches the values to the left and right of the
vertical bar.
For example, the regular expression Jack|Jill matches "Jack" and "Jill".
\ (backslash)
The backslash symbol acts as an escape sequence. Use it when you want search for a regular
expression symbol. The backslash character immediately precedes the symbol in the expression.
For example, the regular expression a.\+.b matches the string "a + b".
[string]
A string within square brackets matches any one of the characters in string.
For example, the regular expression d[iu]g matches "dig" and "dug", but not "dog".
[character-character]
The hyphen symbol, within square brackets, means through. It fills in the intervening characters
according to the current collating sequence. For example, [a-z] can be equivalent to [abc...xyz] or, with
a different collating sequence, it can be equivalent to [aAbBcC...xXyYzZ].
For example, the regular expression m[a-z]p matches "map" and "mop", but not "m9p", since 9 is
not in the range a to z.
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  49

## Page 82

[^string]
The caret symbol, when the first character inside square brackets, negates the following characters
within the square brackets.
For example, the regular expression d[^iu]g matches "dog", but not "dig" or "dug".
{m} {m,u} {m,}
Integer values enclosed in {} indicate the number of times to apply the preceding regular expression.
m is the minimum number, and u is the maximum number. {m} indicates an exact number of times
to apply the regular expression. {m,u} indicates a range of instances. {m,} indicates that there is a
minimum, but no maximum.
For example:
• m[eaiy]{2}n matches "main", "mien" and "mean", but it does not match "man", because there is
only one instance of the letters in the square brackets. Nor does it match "mayan", because this has
three instances of the letters in the square brackets.
• [0-9][a-z]{2,3}[0-9] matches "7ab5" and "4abc3", but not "7b5", nor "4abcd3".
• [0-9][a-z]{2,}[0-9] matches "4ab3", "4abc3", "4abcd3", and so on, but not "4a3".
(expression)
Used to group parts of the expression into sub-expressions. This can be used to limit an operator to a
sub-expression.
For example, the regular expression z/OS.((1\.1[0-3])|(2\.[1-2])) matches "z/OS 1.13" and
"z/OS 2.1".
Note: You can use the ] (right square bracket) alone within a pair of square brackets, but only if it
immediately follows either the opening left square bracket or if it immediately follows [^. For example: []-]
matches the ] and - characters.
Regular expressions cannot be used for string2 for a CHANGE command.
Effect of CHANGE command on column-dependent data
Column-dependent data is groups of nonblank source data separated by two or more blanks, such as a
table. When you use CHANGE to change column-dependent data, ISPF attempts to maintain positional
relationships. For instance, if you change a long word to a short word, the editor pads the short word with
blanks. This padding maintains the column position of any data to the right of the change by preventing it
from shifting left.
When only one blank separates words, as in most text data, padding does not occur. Changing a long word
to a short word causes data to the right of the change to shift left.
Using the CHANGE command with EBCDIC and DBCS data
If you are editing a data set that contains both EBCDIC and DBCS data, note these rules about CHANGE
strings:
• The SO and SI characters that delimit the CHANGE string are used as part of the string only if necessary.
If you specify replacement of an EBCDIC string with a DBCS string, they are used. If you specify
replacement of a DBCS string with another DBCS string, they are not used.
• If you specify in a CHANGE string that an SO or SI character be changed to another character, the result
is unpredictable.
• If you specify a CHANGE string that causes a field length of zero and the boundary falls between the
SO and SI characters, the SO/SI or SI/SO character strings that are next to each other are replaced
with a DBCS blank. If the boundary does not fall between the SO and SI characters, the SO/SI or SI/SO
characters that are next to each other are removed.
• If the lengths of the two strings specified in CHANGE are different, these actions occur:
Finding, seeking, changing, and excluding data
50  z/OS: z/OS ISPF Edit and Edit Macros

## Page 83

– If string1 is shorter than string2, the data to the right of string1 is shifted to the left up to some
breakpoint. Breakpoints include the border between an EBCDIC field and a DBCS field, a double or
single blank, or the right boundary set by a BOUNDS command.
– If string1 is longer than string2, blanks in the record to the right of string1 are used to make room.
When blanks in a DBCS field are used, they are used in units of 2 bytes.
• If a DBCS field crosses the right boundary, CHANGE can cause an odd-length DBCS field. If this
happens, the right boundary is ignored and the operation takes place.
Working with ASCII data
When you are working with an ASCII file, you can use the ASCII editing facility to translate data from and
to ASCII when displaying and receiving input from the terminal.
The ASCII editing facility converts the ASCII data to the corresponding EBCDIC representation prior to
displaying at the terminal. Also, when you enter data from the terminal, the data is converted from the
CCSID of the terminal to ASCII before being stored in the file you are editing.
To activate the ASCII editing facility for a PDS member or data set select 1 (ASCII) for the Data Encoding
option on the edit entry panel. Otherwise start editing the member or data set and then issue the
command SOURCE ASCII.
The ASCII editing facility is automatically invoked for a z/OS UNIX file tagged with a CCSID of 819. The
following can be used to activate the ASCII editing facility for a z/OS UNIX file that is not tagged with a
CCSID of 819:
• Select 1 (ASCII) for the Data Encoding option on the Edit/View entry panel.
• Specify the ASCII parameter when calling the EDIT or VIEW services.
• Specify the EA (Edit - ASCII) or VA (View - ASCII) line commands on the z/OS UNIX directory list display.
When using the ASCII editing facility with a z/OS UNIX file, ISPF ensures the file's CCSID is set to 819
when the file is saved.
Note: If you try to use the ASCII editing facility with a file enabled for z/OS UNIX automatic codeset
conversion, ISPF does not invoke the ASCII editing facility and codeset conversion is used to convert the
data to EBCDIC.
The ISPF editor then treats the source data as though it is ASCII data and converts it from ASCII to the
CCSID of the terminal for display purposes, although the data remains unchanged within the file. When
you input or modify data at the terminal, the ISPF editor translates the data entered from the CCSID of the
terminal to ASCII before storing the data in the file.
While editing a PDS member or data set, you can revert back to a normal mode, where the data is
not translated from and to ASCII when displaying and receiving input from the terminal, by issuing the
command:
RESET SOURCE
Restructuring data based on the linefeed character
ASCII data can contain linefeed characters (X'0A'). If the data has been uploaded from another computing
platform, the data may not be correctly structured based on the linefeed characters.
To restructure the data based on the linefeed character, issue the command LF.
Note:
1. There is no reverse process for restructuring the data based on the linefeed character. Consequently,
once you have saved the data after an LF command the change is permanent.
2. Do not enter the LF command more than once against the same file as blanks following linefeed
characters are interpreted as the leading data of the next record.
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  51

## Page 84

3. The ASCII editing facility uses MVS Conversion Services to translate the data between ASCII (CCSID
819) and the CCSID supported by the terminal. It is a requirement that MVS Conversion Services
be installed and the required translations specified to it, in order for the ASCII editing facility to be
operable.
When using the ASCII edit facility for a z/OS UNIX file, the LF primary command is not available as the
editor automatically restructures the data based on the linefeed character.
Working with UTF-8 data
When you are working with an UTF-8 file, you can use the UTF-8 editing facility to translate data from and
to UTF-8 when displaying and receiving input from the terminal.
The UTF-8 editing facility converts the UTF-8 data to the corresponding EBCDIC representation prior to
displaying at the terminal. Also, when you enter data from the terminal, the data is converted from the
CCSID of the terminal to UTF-8 before being stored in the file you are editing.
To activate the UTF-8 editing facility for a PDS member or data set select 2 (UTF8) for the Data Encoding
option on the edit entry panel.
The UTF-8 editing facility is automatically invoked for a z/OS UNIX file tagged with a CCSID of 1208. The
following can be used to activate the UTF-8 editing facility for a z/OS UNIX file that is not tagged with a
CCSID of 1208:
• Select 2 (UTF8) for the Data Encoding option on the Edit/View entry panel.
• Specify the UTF8 parameter when calling the EDIT or VIEW services.
• Specify the EU (Edit - UTF-8) or VU (View - UTF-8) line commands on the z/OS UNIX directory list
display.
When using the UTF-8 editing facility with a z/OS UNIX file, ISPF ensures the file's CCSID is set to 1208
when the file is saved.
The ISPF editor then treats the source data as though it is UTF-8 data and converts it from UTF-8 to the
CCSID of the terminal for display purposes, although the data remains unchanged within the file. When
you input or modify data at the terminal, the ISPF editor translates the data entered from the CCSID of the
terminal to UTF-8 before storing the data in the file.
Restructuring data based on the linefeed character
UTF-8 data can contain linefeed characters (X'0A'). If the data has been uploaded from another
computing platform, the data may not be correctly structured based on the linefeed characters.
To restructure the data based on the linefeed character, issue the command LF.
Note:
1. There is no reverse process for restructuring the data based on the linefeed character. Consequently,
once you have saved the data after an LF command the change is permanent.
2. Do not enter the LF command more than once against the same file as blanks following linefeed
characters are interpreted as the leading data of the next record.
3. The UTF-8 editing facility uses MVS Conversion Services to translate the data between UTF-8 (CCSID
1208) and the CCSID supported by the terminal. It is a requirement that MVS Conversion Services
be installed and the required translations specified to it, in order for the UTF-8 editing facility to be
operable.
When using the UTF-8 edit facility for a z/OS UNIX file, the LF primary command is not available as the
editor automatically restructures the data based on the linefeed character.
Controlling the search
After you specify the search string, you can then specify how much of the data you want to search, as well
as the starting point and direction of the operation.
Finding, seeking, changing, and excluding data
52  z/OS: z/OS ISPF Edit and Edit Macros

## Page 85

Extent of the search
You can limit the lines to be searched by first assigning a label to the first and last lines to be searched,
and then specifying the labels on the command (range operand).
If you want to limit the search to a single line, assign a label to it, and then specify the label twice to show
the first and last line of the range. For more information about labels, see “Labels and line ranges” on
page 59.
Starting point and direction of the search
To control the starting point and direction of the search, use one of these operands:
NEXT
Starts at the first position after the current cursor location and searches ahead to find the next
occurrence of string1. NEXT is the default.
ALL
Starts at the top of the data and searches ahead to find all occurrences of string1. The long verification
message, which PDF displays when you enter the HELP command in response to the short verification
message, shows the number of occurrences found. If you use this operand with CHANGE, the lines
changed are marked with ==CHG> flags, and lines that cannot be changed are marked with ==ERR>
flags. The status of these lines can be used by LOCATE and changed by RESET.
FIRST
Starts at the top of the data and searches ahead to find the first occurrence of string1.
LAST
Starts at the bottom of the data and searches backward to find the last occurrence of string1.
PREV
Starts at the current cursor location and searches backward to find the previous occurrence of string1.
If you specify NEXT, ALL, or FIRST, the direction of the search is forward. When you press the assigned
function keys, the RFIND or RCHANGE commands find or change the next occurrence of the designated
string. If you specify LAST or PREV, the direction of the search is backward. When you specify those
operands, the editor finds or changes the previous occurrence of the string.
The search proceeds until the editor finds one or all occurrences of string1, or the end of data.
If you omit the ALL operand on the CHANGE command, the editor searches only for the first occurrence of
string1 after the current cursor location. If the cursor is not in the data area of the panel, the search starts
at the beginning of the first line currently displayed. Scrolling is performed, if necessary, to bring the string
into view.
After you make the change, the cursor is positioned at the end of the changed string; a verification
message is displayed in the upper right corner of the panel.
Depending on the direction of the search, if the string is not found between the current cursor location
and the end or beginning of data, a message is displayed and an audible alarm, if installed, is sounded.
If string1 is not found, one of these actions takes place:
• A NO string1 FOUND message is displayed in the upper right-hand corner of the panel.
• If CHANGE or EXCLUDE was repeated using RFIND or RCHANGE, either BOTTOM OF DATA REACHED or
TOP OF DATA REACHED is displayed, depending on the direction of the search. When either of these
messages is displayed, you can enter RFIND or RCHANGE again to continue the search by wrapping to
the top or bottom of the data. If string1 is still not found, a NO string1 FOUND message is displayed.
When you type a primary command, the cursor is, of course, positioned on the command line. In the case
of the CHANGE, EXCLUDE, and FIND primary commands, if you specify NEXT or PREV, the search starts at
the current cursor location in a forward or backward direction respectively:
• If you specify NEXT and then press Enter without repositioning the cursor, the current cursor position is
taken to be at the top of the data. The search starts in a forward direction from that point.
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  53

## Page 86

• If you specify PREV and then press Enter without repositioning the cursor, the current cursor position is
taken to be at the bottom of the data. The search starts in a backward direction from that point.
To obtain the result you want, you may need to reposition the cursor after you have typed the primary
command, but before you press Enter.
Qualifying the search string
You can specify additional characteristics of string1 by using the operands PREFIX, SUFFIX, CHARS, and
WORD. You can abbreviate PREFIX, SUFFIX, and CHARS to PRE, SUF, and CHAR, respectively.
CHARS
Locates string1 anywhere the characters match. This is the default.
PREFIX
Locates string1 at the beginning of a word.
SUFFIX
Locates string1 at the end of a word.
WORD
string1 is delimited on both sides by blanks or other non-alphanumeric characters.
In this example, the editor would find the highlighted strings only:
CHARS 'DO'  - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
PREFIX 'DO' - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
SUFFIX 'DO' - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
WORD 'DO'   - DO DONE ADO ADOPT 'DO' +ADO (DONE) ADO-
If you do not specify an operand, the default is CHARS.
Limiting the search to specified columns
The left_col and right_col operands allow you to search only a portion of each line, rather than the entire
line. These operands, which are numbers separated by a comma or by at least one blank, show the
starting and ending columns for the search. These rules apply:
• If you specify neither left_col nor right_col, the search continues across all columns within the current
boundary columns.
• If you specify only left_col, the editor finds the string only if the string starts in the specified column.
• If you specify both left_col and right_col, the editor finds the string only if it is entirely within the
specified columns.
Split screen limitations
When string1 is not found within the data that is displayed on the screen, the search operation scrolls
the data so that string1 appears on the second displayed line of the data area. If only one line of data
is showing in split screen mode, the data on the second line (thus, string1) cannot be seen, and so the
cursor is placed on the command line.
Limiting the search to excluded or non-excluded lines
You can limit the lines to be searched by first using the X or NX operands:
X
Scan only lines that are excluded from the display.
NX
Scan only lines that are not excluded from the display.
Finding, seeking, changing, and excluding data
54  z/OS: z/OS ISPF Edit and Edit Macros

## Page 87

If you omit these operands, both excluded and non-excluded lines are searched. When you issue a FIND
or CHANGE command that includes searching excluded lines, all lines found are displayed. EXCLUDE can
also find labels assigned to excluded lines.
Using the X (Exclude) line command with FIND and CHANGE
You can use the X (exclude) line command with FIND and CHANGE to display only those lines containing
the search string or those lines that have been changed. For example, if your data set contains 99␠999
lines or less, type X99999 in the line command field of the first line to exclude all of the lines from the
display. Then enter a CHANGE command, such as:
CHANGE ALL XYZ ABC
All lines containing the search string XYZ are redisplayed with XYZ changed to ABC and with the cursor at
the end of the first string changed.
Similarly, you can enter a FIND command:
FIND ALL XYZ
Here, all lines containing the search string XYZ are redisplayed with the cursor at the beginning of the first
string found.
Repeating the FIND, CHANGE, and EXCLUDE commands
The easiest way to repeat FIND, CHANGE, and EXCLUDE without retyping them is to assign those
commands to function keys. The defaults are:
F5/17
RFIND
F6/18
RCHANGE
The search begins at the cursor. If the cursor has not moved since the last FIND, CHANGE, or EXCLUDE
command, the search continues from the string that was just found. Instead of retyping string1, you can
type an asterisk to specify that you want to use the last search string. If you decide to type RCHANGE or
RFIND on the command line instead of using a function key, position the cursor at the desired starting
location before pressing Enter.
All three commands share the same string1. Therefore:
FIND ABC
followed by:
CHANGE * XYZ
first shows you where ABC is, and then replaces it with XYZ. However, you can do this more easily by
typing:
CHANGE ABC XYZ
Then press F5/17 to repeat FIND. The editor finds the next occurrence of ABC. You can either press F5/17
to find the next ABC, or F6/18 to change it. Continue to press F5/17 to find remaining occurrences of the
string.
The previous value of a search string, specified by an asterisk or by use of RFIND or RCHANGE, is retained
until you end your editing session.
Examples
See:
Finding, seeking, changing, and excluding data
Chapter 3. Managing data  55

## Page 88

• “FIND command example” on page 56
• “CHANGE command example” on page 56
• “EXCLUDE command example” on page 57
FIND command example
To find all occurrences of "MIMIC" in a member such as the one shown in Figure 11 on page 56, type
FIND ALL MIMIC on the command line.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00             Columns 00001 00072
 Command ===> find all mimic                                   Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000001 /* REXX */
 000002 /* REXX */
 000003 ADDRESS TSO
 000004 /*                                                                   */
 000005 /* RECREATE THE OLD BACKUP DATA SETS                                 */
 000006 /*                                                                   */
 000007 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.ARCHDEF')"
 000008 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.CLIST')"
 000009 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.CPP')"
 000010 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.EXEC')"
 000011 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.GIF')"
 000012 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.GMLINC')"
 000013 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.HPP')"
 000014 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.HSAS65')"
 000015 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.LEL')"
 000016 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2LMAP')"
 000017 CALL MIMIC "ALLOC DA('PDFTDEV.SVT2.LOAD')"
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 11. Before FIND command (ISREDDE2)
After you press Enter, the editor searches for the string starting at the top of the data, places the cursor at
the beginning of the first occurrence ( 1 ), and displays the number of occurrences ( 2 ) as shown in Figure
12 on page 56.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00               2  21 CHARS 'MIMIC'
 Command ===>                                                  Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000001 /* REXX */
 000002 /* REXX */
 000003 ADDRESS TSO
 000004 /*                                                                   */
 000005 /* RECREATE THE OLD BACKUP DATA SETS                                 */
 000006 /*                                                                   */
 000007 CALL 1 MIMIC "ALLOC DA('PDFTDEV.SVT2.ARCHDEF')"
 ⋮
Figure 12. After FIND command
CHANGE command example
To change "MIMIC" to "WILLY" enter C ALL MIMIC WILLY as shown in Figure 13 on page 56.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00                21 CHARS 'MIMIC'
 Command ===> c all mimic willy                                Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000001 /* REXX */
 ⋮
Figure 13. Before CHANGE command
Finding, seeking, changing, and excluding data
56  z/OS: z/OS ISPF Edit and Edit Macros

## Page 89

The editor changes all occurrences of the string starting at the top of the data and inserts a ==CHG> flag
next to each changed line, as shown in Figure 14 on page 57.
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00           CHARS 'MIMIC' changed
 Command ===>                                                  Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000001 /* REXX */
 000002 /* REXX */
 000003 ADDRESS TSO
 000004 /*                                                                   */
 000005 /* RECREATE THE OLD BACKUP DATA SETS                                 */
 000006 /*                                                                   */
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.ARCHDEF')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.CLIST')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.CPP')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.EXEC')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.GIF')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.GMLINC')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.HPP')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.HSAS65')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.LEL')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2LMAP')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.LOAD')"
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 14. After CHANGE command
EXCLUDE command example
When you enter an EXCLUDE command like ex /* all on the command line (Figure Figure 15 on page
57), the editor excludes all lines with that string starting at the top of the data (Figure Figure 16 on page
57).
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00           CHARS 'MIMIC' changed
 Command ===> ex /* all                                        Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000001 /* REXX */
 000002 /* REXX */
 000003 ADDRESS TSO
 000004 /*                                                                   */
 000005 /* RECREATE THE OLD BACKUP DATA SETS                                 */
 000006 /*                                                                   */
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.ARCHDEF')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.CLIST')"
 ⋮
Figure 15. Before EXCLUDE command
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.EXEC(FCEXMP) - 01.00                     5 chars '/*/
 Command ===>                                                  Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 - - -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  2 Line(s) not Displayed
 000003 ADDRESS TSO
 - - -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  3 Line(s) not Displayed
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.ARCHDEF')"
 ==CHG> CALL WILLY "ALLOC DA('PDFTDEV.SVT2.CLIST')"
 ⋮
Figure 16. After EXCLUDE command
Excluding lines
You can exclude lines from a data set using the X (exclude) line command as well as the EXCLUDE primary
command.
Excluding lines
Chapter 3. Managing data  57

## Page 90

When you are editing a program that exceeds the screen size, it can be difficult to determine whether the
control structure and indentation levels are correct. Excluding lines allows you to remove one line or a
block of lines from the display so that you can see the general control structure. Each block of excluded
lines is replaced by a single line containing a message in the form "n Line(s) not Displayed". Excluded lines
are treated as valid data lines. They are excluded from the display, but are not deleted from the data.
The X line command can be entered in these ways:
X
Xn
XX
The first two forms allow you to exclude one line or a specified number of lines.
The third form allows you to exclude a block by typing XX on the first and last lines of the block of lines
that you want to exclude. The first and last lines do not need to be on the same page; after typing the first
XX you can scroll to the second XX.
You can enter any line command that usually operates on a single line in the line command field of the
excluded lines message. For example, if you enter the D (delete) line command, the complete block of
excluded lines is deleted.
Hiding excluded lines
You can also suppress the lines containing the "n Line(s) not Displayed" message by entering HIDE as
a primary command or in an edit macro. HIDE removes the excluded lines messages from the display
and indicates the location of each block of excluded lines by underscoring the line number field of the
previous line.
The RESET HIDE primary command and edit macro command restores the lines containing the "n Line(s)
not Displayed" message to the display.
Redisplaying excluded lines
To display all excluded lines, enter the RESET EXCLUDED primary command. Alternatively, you can display
one or more excluded lines again by entering the S (show), F (first), or L (last) line commands, typing over
the dashes in the line command field. If these commands are typed outside the dashes of the command
line area, no action is taken.
You can add a number following any of these line commands to cause more than one line to appear again:
Sn
Fn
Ln
FIND and CHANGE also cause any excluded lines that meet the search criteria to appear again.
The S line command causes the editor to scan a block of excluded lines, and one or more lines is selected
to be appear again. The selected lines are those with the leftmost indentation levels; that is, the lines
that contain the fewest leading blanks. If you type S3, for example, the three lines with the leftmost
indentation level are displayed again. If more than three lines exist at this indentation level, only the first
three are displayed.
Note: If you enter an S line command to display all but one line of an excluded block, then that line is also
displayed. This could result in more lines being displayed than the number you requested. For example, if
five lines are excluded in a block, an S4 command causes all five lines to be displayed.
Excluding lines
58  z/OS: z/OS ISPF Edit and Edit Macros

## Page 91

Redisplaying a range of lines
The FLIP command lets you reverse the exclude status of a specified group of lines in a file or of all the
lines in the file. This is useful when you have used the 'X ALL;FIND ALL xyz' command to find lines
containing a string (xyz) and want to see the lines which do not contain the string. You can also use FLIP
to show excluded note, message, and information lines.
You can enter one or two labels to specify the range of lines whose include status you want to reverse. If
no labels are specified, the exclude status of all of the lines is reversed.
To reverse the exclude status of all the lines in a file, use this syntax:
FLIP
To reverse the exclude status of specified lines, use this syntax:
FLIP .a .b
The lines between labels .a and .b are redisplayed.
Labels and line ranges
A label is an alphabetic character string used to name lines or strings of data for easy reference. Because
labels remain with the lines to which they are assigned, they are especially useful in keeping track of lines
whose numbers might change. Most labels are assigned in macros, but certain labels are automatically
assigned by the ISPF editor.
You can assign a label to a line by typing the label over the line number on the left side of the panel. The
label is displayed in place of the number whenever the line is being displayed. If you then move the line,
the label moves with it. You cannot type a label on a non-data line or on the line that is displayed to show
one or more lines is excluded.
A label must begin with a period, and be followed by no more than 5 alphabetic characters (8 for edit
macros), the first of which cannot be a Z. Labels beginning with Z are reserved for use by the editor. No
special or numeric characters are allowed.
To eliminate a single label, blank it out. To eliminate all labels, use the RESET LABEL command.
An edit macro can assign labels to lines that the macro references frequently. See “Labels in edit macros”
on page 103 for details.
Editor-assigned labels
The editor automatically assigns special labels that begin with the letter Z. Only the editor can assign a
special label.
These built-in labels are:
.ZCSR
The data line on which the cursor is currently positioned.
.ZFIRST
The first data line (same as relative line number 1). Can be abbreviated .ZF.
.ZLAST
The last data line. Can be abbreviated .ZL.
Unlike other labels, .ZCSR, .ZFIRST, and .ZLAST do not stay with the same line. Label .ZCSR stays with the
cursor, and labels .ZFIRST and .ZLAST remain with the current first and last lines.
Note: Labels that are five characters long and begin with the letter 'O' have special meaning to the
HILITE feature of the ISPF editor. When a 5-character label starting with O, such as .OAAAA, is shown
on the screen, the language highlighting features are disabled and the lines with these special labels are
displayed in blue. This feature is used by the COMPARE command.
Labels and line ranges
Chapter 3. Managing data  59

## Page 92

Specifying a range
Labels allow you to specify a line or a range of lines on a primary command. You can specify two labels to
define a range of lines to be processed on these commands:
CHANGE   DELETE   EXCLUDE
FIND     LOCATE   REPLACE
RESET    SORT     SUBMIT
The range operand is always optional. If you do not specify a range, it defaults to .ZFIRST and .ZLAST. For
example, the command:
CHANGE ALL 'TEST' 'FINAL'
starts at the first line of the data being edited and scans all lines up to and including the last line, changing
all occurrences of TEST to FINAL.
However, the command:
CHANGE .ZCSR .ZLAST ALL 'TEST' 'FINAL'
specifies a range, and is thus interpreted differently. The command changes only the last part of the data.
When you use labels to specify a range, you must always use two labels to define the first and last lines,
inclusively. To process a single line, repeat the label:
CHANGE ALL " " "_" .A .A
The command in the previous example is interpreted as "Change all blanks to underscores on the .A line."
The order in which you specify the labels is not important. The editor assumes that the line closer to the
beginning of the data set is the first line of the range, and the line closer to the end of the data set is the
last.
A common error when using a range is to assume that the search begins at the first character of the line
with the first label. Remember, however, that the default is NEXT and that the search starts at the cursor
location. Lines outside the range are logically the same as the TOP OF DATA and BOTTOM OF DATA
lines. Use the FIRST, LAST, or PREV operands to ensure that the search begins within the range.
Using labels and line ranges
The examples shown here show the results of using labels to identify ranges of lines. They show that the
order of both labels and other operands is not important, and that you can type both labels and operands
in either uppercase or lowercase.
• This command locates the first line flagged: ==CHG> between the line labeled .start and the line with
the cursor on it:
locate first chg .start .zcsr
• This command changes the last occurrence of "PRE" to "POST" between the first line and the line
marked with the .HERE label:
CHANGE LAST PRE POST .HERE .ZFIRST
• This command changes all occurrences of "PRE" to "POST" from the .MYLAB line to the last line of the
data set:
CHANGE PRE POST ALL .MYLAB .ZL
• This command finds the word "HIGHER" between the .START line and the .END line:
FIND HIGHER WORD .START .END
Labels and line ranges
60  z/OS: z/OS ISPF Edit and Edit Macros

## Page 93

Word processing
This topic is a general overview of three line commands for word or text processing: TF (text flow), TS (text
split), and TE (text entry). The editor also provides three corresponding edit macro commands: TFLOW,
TSPLIT, and TENTER. For the sake of simplicity, only the line commands are referred to. However, the
descriptions apply to the macro commands, as well.
TF, TS, and TE assume that the data is grouped in paragraphs. A paragraph is a group of lines that begin
in the same column. The first line of a paragraph is excluded from the grouping. The editor interprets
any indentation or blank line as representing a new paragraph. It also recognizes word processor control
words that are used by the Document Composition Facility as the beginning of a paragraph. These control
words begin with a period, a colon, or an ampersand.
If you use text line commands frequently, you can assign both the TS and TF commands to function keys.
Use KEYS to reassign the keys. For example:
F10 ===> :TS
F11 ===> :TF
Now you can split text by moving the cursor to the desired split point within a line and pressing F10.
Having typed the new material, press F11 to restructure the text from the line containing the cursor to the
end of the paragraph.
Formatting paragraphs
The TF (text flow) line command formats paragraphs. It assumes that the sentences are roughly in
paragraph form with a ragged right margin when it attempts to recognize groupings. TF can be followed by
a number (TF72 for example) that specifies the desired right side column for the paragraph. If you do not
specify a number, the right side of the panel is used unless you have set bounds different from the default.
In that case, the right boundary is used. The editor assumes that because the first line of a paragraph may
be at a different indentation level than the remainder of the paragraph, the starting column of the second
line is the left side of the paragraph.
When formatting paragraphs, the editor:
• Moves text so that each line contains the maximum number of words. TF limits its activity to within the
bounds. Thus, it can be used to flow text within a border.
• Keeps any blanks between words.
• Assumes one blank between the word at the end of a line and the word on the next line except when the
line ends with a period. In that case, the editor inserts two blanks.
The end of the paragraph is denoted by a blank line, a change in indentation, or the special characters
period (.), colon (:), ampersand (&), or left angle bracket (<) in the left boundary column. These special
characters are used as Document Composition Facility (SCRIPT/VS) control word delimiters.
The restructure operation removes trailing blanks on a line by using words from the following line. It does
not, however, remove embedded blanks within a line. Accordingly, if one or more words in a line are to be
removed, delete the words rather than type over them.
The text to be restructured is taken from within the currently defined column boundaries. Any text outside
the bounds is not included in the restructuring. The restructured text is also positioned within the current
boundaries. If the original text was indented from the left boundary, that indentation is preserved.
Using text flow on a DBCS terminal
You can restructure paragraphs containing lines that include DBCS strings based on these rules:
• If a character in a DBCS string encroaches on the rightmost column position for the restructured text,
the string is divided before that character. An SI character is added at the end of the line, and an SO
character is added at the beginning of the new line.
Word processing
Chapter 3. Managing data  61

## Page 94

• If the boundaries are defined and a DBCS character is on the boundary, the DBCS character is in the
text flow operation. An SO or SI character is added to both lines to ensure that DBCS character strings
remain enclosed with SO and SI characters.
• If the mask contains DBCS fields and some of the DBCS fields cross the left, right, or both boundaries,
the result may be unpredictable.
• If a DBCS string crosses the left, right, or both boundaries, the result may be unpredictable.
• When a text flow operation causes a field length of zero, the SO/SI or SI/SO character strings that are
next to each other are removed.
If you use the TF line command while editing a formatted data set, note these points:
• The current boundaries are automatically changed during command processing, and are reset to the
original values after processing is complete. Changes are as follows:
– If the left boundary falls on the second byte of a DBCS character in a DBCS field, the boundary is
shifted to the left by 1 byte.
– If the right boundary does not fall on the same field as the left boundary, it is shifted to the last byte
of the field that contains the left boundary. If it falls on the same DBCS field as the left boundary, and
it also falls on the first byte of a DBCS character, the right boundary is shifted to the right by 1 byte.
• If you specify the column number with the TF command, and if the column falls on the first byte of a
DBCS character in a DBCS field, the column number increases by one.
Splitting lines
The TS (text split) line command splits a line into two lines. The cursor shows where the line is to be split.
The editor moves the characters to the right of the cursor or to a new line following the original line and
aligns the new line with the left side of the paragraph. As mentioned earlier, the left side of a paragraph is
determined by looking for a pattern in the lines preceding or succeeding a paragraph.
If the line being split is the first line in a paragraph, the new line is aligned with the rest of the lines in the
paragraph. If there are no other lines in the paragraph, the portion of the line to the right of the cursor
aligns itself with the first portion of the line.
One or more blank lines are inserted after the line being split, depending on what you specify when you
enter the TS command. Note that the TSPLIT macro command inserts only one blank line.
To rejoin lines, use the TF (text flow) line command. See “Formatting paragraphs” on page 61 for more
information.
Splitting lines within a DBCS string
You can split a line within a DBCS string based on these rules:
• When splitting at a DBCS character, an SI character is added to the end of the line and an SO character
is added at the beginning of the new line.
• If the cursor is placed at the SO character, the SO character becomes the first character to be moved.
• If the cursor is placed at the SI character, the character following the SI character becomes the first
character to be moved.
• If the mask contains DBCS fields and some of the DBCS fields cross the left, right, or both column
boundaries, the result is unpredictable.
If you use the TS line command while editing a formatted data set, you make special considerations for
the current boundaries. These boundaries are automatically changed during command processing, and
are reset to the original values after processing is complete. Changes are as follows:
• If the left boundary falls on the second byte of a DBCS character in a DBCS field, the boundary is shifted
to the left by 1 byte.
Word processing
62  z/OS: z/OS ISPF Edit and Edit Macros

## Page 95

• If the right boundary does not fall on the same field as the left boundary, it is shifted to the last byte of
the field that contains the left boundary. If it falls on the same DBCS field as the left boundary, and it
also falls on the first byte of a DBCS character, the right boundary is shifted to the right by 1 byte.
Entering text (power typing)
The TE (text entry) line command allows you to powertype. When using this command, the display is filled
with blank lines. The line number field normally on the left of the display disappears, so that you can
type all of your data as if it were one continuous line. Because the editor is doing the formatting, you can
continue typing and ignore the wrap around on the display. Any explicit cursor movement is interpreted as
your personal formatting and results in embedded blanks.
The editor assumes that you are typing text as paragraphs. If you explicitly move the cursor down and
leave a blank line, the editor assumes that the blank line should be there. The text that follows the blank
line is consequently a new paragraph. Similarly, if you leave a specified number of blanks between words,
the editor leaves them there. Also, if you tab to the beginning of the next line before completing the
current line, the editor does not flow these sentences together. Remember that skipping a line specifies
the start of a new paragraph.
Note: You cannot use logical or hardware tabs during text entry.
When you press Enter, the text is flowed in the same manner as the TF (text flow) line command, except
that it uses the bounds as the right and left sides of the paragraphs.
Entering text on a DBCS terminal
If you are using the TE line command in a formatted data set, note these points:
• The current boundaries are automatically changed during command processing, and are reset to the
original values after processing is complete. Changes are as follows:
– If the left boundary falls on the second byte of a DBCS character in a DBCS field, the boundary is
shifted to the left by 1 byte.
– If the right boundary does not fall on the same field as the left boundary, it is shifted to the last byte
of the field that contains the left boundary. If it falls on the same DBCS field as the left boundary, and
it also falls on the first byte of a DBCS character, the right boundary is shifted to the right by 1 byte.
• The attribute of the field where the left boundary falls is used for the text input area attribute. The new
input data is reformatted to fit within the current boundaries.
Using tabs
This section discusses hardware, software, and logical tabs, defining and controlling tabs, defining tab
positions, and using attribute bytes.
Types of tabs
Software and hardware tabs
The editor uses software and hardware tabs to reposition the cursor within the current display window.
You can define tabs with the TABS line command. Use underscores (_) or hyphens (-) to define software
tabs and asterisks (*) to define hardware tabs.
Logical tabs
The editor uses logical tabs to reposition strings of data. You can use TABS primary and macro
commands, and the TABS assignment statement to define a special character. The tab character locates
the beginning of each string. Edit repositions the strings one space to the right of hardware tab positions.
Note:
Using tabs
Chapter 3. Managing data  63

## Page 96

1. You cannot use the command delimiter that you defined on the Terminal Characteristics panel (option
0.1) as a special tab character.
2. Tabs are not functional when you are using the TE (text entry) line command.
Effect of TABS commands on tab types
If you are using hardware or logical tabs, the TABS line command must be used with one of the other
TABS commands or the TABS assignment statement. For example, hardware tab positions defined by the
TABS line command do not take effect until tabs mode is turned on, which the line command cannot
do. Conversely, a logical tab character defined with the TABS primary or macro command, or the TABS
assignment statement, cannot be used to position data strings horizontally unless hardware tab positions
are defined with the TABS line command. However, if you are using software tabs, you do not need to turn
tabs mode on. The TABS primary and macros commands, and the TABS assignment statement, have no
effect on software tabs.
Defining and controlling tabs
Three TABS commands help you to position the cursor where you want to start typing. These commands
are the TABS line command, primary command, and macro command. There is also a TABS assignment
statement.
You type the TABS line command in the line command field over the line numbers. This command:
• Displays the =TABS> (tab-definition) line
• Defines tab positions for software, hardware, and logical tabs
You type the TABS primary command on the command line. The TABS macro command is processed from
within an edit macro. The TABS primary and macro commands can:
• Turn tabs mode on and off
• Define the logical tab character
• Control the insertion of attribute bytes at hardware tab positions that have been defined with the TABS
line command
The TABS assignment statement is processed from within an edit macro. It can do everything that the
TABS macro command can do. In addition, the TABS assignment statement can retrieve the setting of
tabs mode and place it in a variable.
You can use PROFILE to check the setting of tabs mode and the logical tab character.
Defining software tab positions
If you display the =TABS> line and type software tab definitions, they take effect immediately. Each line
contains a software tab or a tab field at the designated column positions. The TABS primary command has
no effect on software tab definitions.
To define software tab positions:
1. Type TABS in the line command field and press Enter.
2. Type an underscore (_) or a hyphen (-) at each desired column position on the =TABS> line.
3. Press Enter again to start the tabs.
You can move the cursor from one column position to the next by continuing to press Enter. See “Using
software and hardware tabs” on page 179 for an example of using software tabs.
Defining hardware tab positions
Hardware tab definitions do not take effect until you turn on tabs mode by using the TABS primary
command. The asterisks define the column positions, but the insertion of attribute bytes (hardware tabs)
or the repositioning of data strings (logical tabs) does not occur unless tabs mode is on.
Using tabs
64  z/OS: z/OS ISPF Edit and Edit Macros

## Page 97

To define hardware tab positions:
1. Type TABS in the line command field and press Enter.
2. Type an asterisk (*) at each desired column position on the =TABS> line.
3. Press Enter again.
When tabs mode is turned on using either the ON or ALL operand, the Tab Forward and Tab Backward
keys can be used to move the cursor to the space following the next attribute byte.
Note: If the ALL operand is not used, attribute bytes are inserted only in spaces that contain a blank or
null character, causing the Tab Forward and Tab Backward keys to recognize only these tab definitions.
When tabs mode is turned on using the tab-character operand, the Tab Forward and Tab Backward keys
do not recognize hardware tab definitions because no attribute bytes are inserted.
Limiting the size of hardware tab columns
To limit the size of hardware tab columns, type consecutive asterisks between columns to define
hardware tab fields . The consecutive asterisks:
• Allow you to determine the length of the data string to be typed in a column
• Cause the cursor to automatically move to the next column when the current column is full
This procedure works only with asterisks (hardware tabs). When you type hyphens or underscores
(software tabs), PDF does not insert attribute bytes. Because attribute bytes cannot be typed over, they
limit the tab column size.
Insert the asterisks from the point where you want the column to end to the point where the next column
begins. For instance, suppose you want to limit each tab column to five spaces. You could do so by
following these steps:
1. Type COLS in the line command field and press Enter. A partial =COLS> line with positions 9 through
45 is shown in this example:
=COLS> -1----+----2----+----3----+----4----+
2. Type TABS ALL on the command line and press Enter again. This command causes PDF to insert an
attribute byte at each hardware tab position defined by an asterisk (*).
3. Using the TABS line command, change the =TABS> line as follows:
=COLS> -1----+----2----+----3----+----4----+
=TABS>            *     *****     *****
With the =TABS> line altered as shown, the cursor automatically skips to the next tab column when 5
characters, blank spaces, or a combination of both are typed in each column.
Using attribute bytes
Attribute bytes overlay characters only on the display; the attribute bytes are never recorded in the data.
If your data set contains DBCS fields, however, attribute bytes can invalidate them. If you start hardware
tabs and insert an attribute byte in the middle of a DBCS field, you invalidate the DBCS field, and it is
displayed as an EBCDIC field. When you turn tabs mode off, the attribute bytes are removed and the
overlaid character at each tab position is displayed again.
When you are in formatted data edit mode, TABS is ignored.
In tabs mode, you temporarily remove the attribute bytes from a single line. There are two ways to do this:
• Blank out the entire line command field using the Erase EOF key.
• Place the cursor directly under one of the attribute bytes and press Enter. When you press Enter again,
the attribute bytes are reinserted.
Using tabs
Chapter 3. Managing data  65

## Page 98

Undoing edit interactions
If you enter an edit primary, line, or macro command, or type over existing data by mistake, you can
restore your data with the UNDO primary command. UNDO has no operands.
Each time you enter UNDO it undoes one interaction. A single interaction might be a data change and
Enter key, a data change and function key, or the invocation of an edit macro. All changes caused by an
edit macro are considered to be one interaction. You can continue to undo interactions, one at a time,
until you have reversed all changes made back to the beginning of your edit session unless you have
done a save or undo recycled. If you have done a save or if undo recycled, you can only undo interactions
back to that point. At that point, if you enter UNDO again, a message informs you that there are no more
interactions to undo.
UNDO has certain limitations. Edit interactions that the command does not undo are:
• Changes that are made by an initial edit macro or recovery edit macro.
• Edit interactions before any data changes are made.
• Edit interactions in previous edit sessions.
• Reset of changed flags (==CHG>) by use of RESET or by typing over the command line area.
• Changes you make to other data sets or members by using the CREATE, REPLACE, or MOVE commands.
Because UNDO affects only the member or data set that you are editing, it removes lines from your
display if they were inserted there by MOVE. However, it does not put those lines back into the data set
or member from which they came.
See “UNDO—Reverse Last Edit Interaction” on page 286 for a discussion of UNDO limitations.
UNDO is reset by SAVE. This means that you can UNDO interactions for the current edit session until you
save your data. After the save, you can undo only interactions made following the time you saved your
data.
UNDO can be run from data kept in storage or from the recovery file (as in previous releases) depending
on what you specify in the Edit Profile for the data you are entering. The SETUNDO primary or macro
command is used to control the profile setting. To use UNDO, you must have either RECOVERY on or
SETUNDO on. You can undo only those changes made after RECOVERY or SETUNDO was turned on.
SETUNDO allows you to specify how changes you make during your edit session are to be recorded and
used by UNDO. You can specify SETUNDO STORAGE (or SETUNDO KEEP or SETUNDO ON) or SETUNDO
RECOVER. SETUNDO STORAGE (or SETUNDO KEEP or SETUNDO ON) specifies UNDO from storage.
SETUNDO RECOVERY specifies UNDO from recovery and turns recovery on if it is off. See “SETUNDO—Set
the UNDO Mode” on page 278 for more details. “Understanding differences in SETUNDO processing” on
page 67 explains how the SETUNDO operands differ.
If not enough storage is available to run UNDO from storage but RECOVERY is on, UNDO processing
continues to be available by using the recovery file. This makes UNDO available for very large files. It also
provides users of machines with less storage with the benefit of UNDO for their larger files.
Note: If you have specified RECOVERY OFF and your installation allows UNDO from storage, the message
that UNDO is unavailable does not display when you enter an edit session. If UNDOSIZE = 0, the message
appears as before.
The UNDOSIZE specifies the number of kilobytes allowed for saving edit transactions for UNDO and the
value is in the configuration table. For more details, refer to z/OS ISPF Planning and Customizing.
If UNDOSIZE is set to zero, all undo documented functions work as in ISPF/PDF Version 3.3 and previous
releases. This means that the Profile lines do not show the status of SETUNDO, and that warning
messages will be shown informing you that UNDO is unavailable until RECOVERY is turned on.
UNDO processing
When the storage allocated for changes is exhausted, UNDO recycles itself and puts up the message UNDO
RECYCLED. Recycling is the process of saving the current image of the file as a new base from which to
work. UNDO is then available after the next transaction. No transactions made before the recycling can be
Undoing edit interactions
66  z/OS: z/OS ISPF Edit and Edit Macros

## Page 99

undone. This is because UNDO saves an image of the original file and keeps an incremental list of changes
to that image.
If there is not enough storage to save the initial image, UNDO attempts to use the recovery file for undo
processing. If recovery is off or suspended, the message UNDO SUSPENDED is shown with an alarm, and
the profile status line is changed to SETUNDO SUSP. If recovery is available, the message UNDO FROM
RECOVERY is shown with an alarm, and the profile status line is changed to SETUNDO REC. This affects
the display but does not affect the edit profile values.
To resume SETUNDO STG, enter the SETUNDO primary command. If there is still not enough storage to
hold the original copy of the file, the recycling procedure is repeated.
Note: Edit recovery can no longer process edit recovery files created under previous releases of ISPF/PDF.
A panel is displayed, but no other action is taken if an old recovery file is used.
Understanding differences in SETUNDO processing
SETUNDO STORAGE (or SETUNDO KEEP or SETUNDO ON) and SETUNDO RECOVERY work essentially the
same way; however, there are some important differences. SETUNDO REC is available only after the edit
recovery file is initialized, that is, until the first data change is made. Because SETUNDO STG keeps its
record of changes in storage, it does not incur the same performance penalty as using the SETUNDO REC.
SETUNDO STG can start to save editing changes earlier than SETUNDO REC, because even non-data
changes, such as setting line labels, adding note lines, and inserting blank lines, cause SETUNDO STG to
initialize its record of changes. You can undo these changes using UNDO even if no data changes have
been made. When SETUNDO REC is in effect, only changes made after and including the first change to
edit data can be undone.
UNDO reverses changes made during a single edit transaction. It is important to note, however, that
changes to the profile, such as HEX ON, LEVEL, and CAPS, are not undone separately. A data change
followed by one or more profile changes is usually considered a single transaction. For example, if you
change the data and then the profile, and then enter UNDO, the data and profile return to their status
before the data change. Profile changes usually cannot be undone if they are not preceded by a data
change. SETUNDO STG and SETUNDO REC may work slightly differently in this regard. Since SETUNDO
STG keeps the record of changes in storage, it is not a substitute for recovery. To recover the edit session
after a system failure, you must have recovery on during the edit session. SETUNDO STG and RECOVERY
ON can be in effect simultaneously, however, after a system crash and a recovery, no transactions can be
undone using SETUNDO STG because the in-storage record will be empty.
If you are running both SETUNDO STG and RECOVERY ON, the UNDO command causes the last change to
be backed out using the in-storage record of edit changes, and the recovery data set to be reinitialized.
If you issue a SETUNDO REC command, after you use UNDO (from storage), there will be no more
transactions to UNDO since the recovery file has been reinitialized.
Undoing edit interactions
Chapter 3. Managing data  67

## Page 100

Undoing edit interactions
68  z/OS: z/OS ISPF Edit and Edit Macros
