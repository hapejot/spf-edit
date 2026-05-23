# Chapter 9. Edit line commands

Source file: f54em00_v3r1.md
Start page: 167
Page span: 167-222

## Page 167

Chapter 9. Edit line commands
Edit line commands affect only a single line or block of lines. You enter line commands by typing over
the 6-digit number in the line command field on one or more lines and pressing Enter. Most command
definitions in this documentation consist of:
Syntax
A syntax diagram is how you type the command. It includes a description of any required or optional
operands.
Description
A description explains the function and operation of the command. This description may also refer to
other commands that can be used with this command.
Example
An example gives a sample usage of the line command.
Rules for entering line commands
Enter a line command by performing one of these actions:
• Typing the command in the line command field and pressing Enter.
• Placing the cursor in the data or line command field and pressing a function key to which the command
is assigned.
These rules apply to all line commands:
• You can type several line commands and make multiple data changes before you press Enter. The
editor displays an error message if the line command is ambiguous. Because the line commands are
processed from top to bottom, it is possible to have one error message appear that masks a later error
condition. Only the first error condition found is displayed. After you have corrected that error condition,
processing can continue and the next error condition, if any, is displayed. If you type a line command
incorrectly, you can replace it before you press Enter by retyping it, blanking it out, or entering RESET.
• Generally, you need to type over only the first 1 or 2 characters of the line number to enter a line
command. Sometimes, however, typing a single character can be ambiguous. In this example, it is
unclear whether the intended line command is R to repeat line 31700, or R3 to repeat the line three
times:
031600
R31700
031800
In such cases, the ISPF editor assumes that you have not typed a number following the line command.
If you want to repeat the line three times, you can use any of these procedures:
– Leave the cursor on the character that immediately follows the R3:
R31700
– Type one or more blanks following the R3:
R3 700
– Type one or more blanks following the R but before the number, leaving the cursor on the character
that immediately follows the 3:
R 3700
– Type R3 and press the Erase EOF key to clear the rest of the line command field, or press the Erase
EOF key and then type R3
Rules for entering line commands
© Copyright IBM Corp. 1984, 2024 135

## Page 168

• You can type these line commands on the TOP OF DATA line by typing over the asterisks that appear in
its line command field:
I, In
Insert one or n lines ahead of the data.
A, An
Move or copy a line or lines one or n times ahead of the data.
TE, TEn
Type one or n text lines ahead of the data.
• You can type this line command on the BOTTOM OF DATA line by typing over the asterisks:
B, Bn
Move or copy a line or lines one or n times following the data.
Line command summary
Table 15 on page 136 summarizes line commands.
Table 15. Summary of the line commands
Command Description
“(—Column Shift Left” on page 138 Shifts columns left two positions or the specified
number of positions.
“)—Column Shift Right” on page 140 Shifts columns right two positions or the specified
number of positions.
“<—Data Shift Left” on page 141 Shifts data left two positions or the specified
number of positions.
“>—Data Shift Right” on page 143 Shifts data right two positions or the specified
number of positions.
“A, AK—Specify an After destination” on page 145 Identifies the line after which copied, moved, or
model lines are to be inserted.
“A, AK—Specify an After destination” on page 145 Identifies the line after which copied, moved, or
model lines are to be inserted, but indicates that
another destination of the form A, B, or O is
still required proceeding forward through the file
before the data is moved or copied to the multiple
destinations specified.
“B, BK—Specify a Before destination” on page 148 Identifies the line before which copied, moved, or
model lines are to be inserted.
“B, BK—Specify a Before destination” on page 148 Identifies the line before which copied, moved,
or model lines are to be inserted, but indicates
that another destination of the form A, B, or O is
still required proceeding forward through the file
before the data is moved or copied to the multiple
destinations specified.
“BOUNDS—Define Boundary Columns” on page 151 Displays the column boundary definition line.
“C—Copy Lines” on page 152 Copies a line from one location to another.
“C—Copy Lines” on page 152 Copies a block of lines from one location to
another.
“COLS—Identify Columns” on page 155 Displays a position identification line.
Line command summary
136  z/OS: z/OS ISPF Edit and Edit Macros

## Page 169

Table 15. Summary of the line commands (continued)
Command Description
“D—Delete Lines” on page 156 Deletes a line.
“D—Delete Lines” on page 156 Deletes a block of lines.
“F—Show the First Line” on page 158 Redisplays one or more lines at the beginning of a
block of excluded lines.
“I—Insert Lines” on page 161 Inserts one or more blank data entry lines.
“L—Show the Last Line(s)” on page 163 Redisplays one or more lines at the end of a block
of excluded lines.
“LC—Convert Characters to Lowercase” on page 164 Converts all uppercase alphabetic characters in a
line to lowercase.
“LC—Convert Characters to Lowercase” on page 164 Converts all uppercase alphabetic characters in a
block of lines to lowercase.
“M—Move Lines” on page 166 Moves a line from one location to another.
“M—Move Lines” on page 166 Moves a block of lines from one location to
another.
“MASK—Define Masks” on page 169 Displays the contents of the mask when used with
the I (insert), TE (text entry), and TS (text split)
line commands.
“MD—Make Dataline” on page 170 Converts a ==MSG&gt;, =NOTE=, =COLS&gt;, or
====== (information) line to data so that it can be
saved as part of your data set.
“MD—Make Dataline” on page 170 Converts a block of ==MSG&gt;, =NOTE=,
=COLS>, and ====== (information) lines to data
so that they can be saved as part of your data set.
“O, OK—Overlay Lines” on page 172 Identifies a line over which data is to be moved or
copied.
“O, OK—Overlay Lines” on page 172 Identifies the line over which data is to be moved
or copied, but indicates that another destination
of the form A, B, or O is still required proceeding
forward through the file before the data is moved
or copied to the multiple destinations specified.
“O, OK—Overlay Lines” on page 172 Identifies a block of lines over which data is to be
moved or copied.
“O, OK—Overlay Lines” on page 172 Identifies a block of lines over which data
is to be moved or copied, but indicates that
another destination of the form A, B, or O is
still required proceeding forward through the file
before the data is moved or copied to the multiple
destinations specified.
“R—Repeat Lines” on page 175 Repeats a line.
“R—Repeat Lines” on page 175 Repeats a block of lines.
“S—Show Lines” on page 177 Redisplays one or more lines with the leftmost
indentation in a block of excluded lines.
“TABS—Control Tabs” on page 179 Displays the tab definition line.
Line command summary
Chapter 9. Edit line commands  137

## Page 170

Table 15. Summary of the line commands (continued)
Command Description
“TE—Text Entry” on page 180 Inserts blank lines to allow power typing for text
entry.
“TF—Text Flow” on page 183 Restructures paragraphs following deletions,
insertions, splitting, and so forth.
“TS—Text Split” on page 185 Divides one or more lines so that data can be
added.
“UC—Convert Characters to Uppercase” on page
187
Converts all lowercase alphabetic characters in a
line to uppercase.
“UC—Convert Characters to Uppercase” on page
187
Converts all lowercase alphabetic characters in a
block of lines to uppercase.
“X—Exclude Lines” on page 188 Excludes a line from a panel.
“X—Exclude Lines” on page 188 Excludes a block of lines from a panel.
(—Column Shift Left
The ( (column shift left) line command moves characters on a line to the left without altering their relative
spacing. Characters shifted past the current BOUNDS setting are deleted. See “Shifting data” on page 42
for more information.
Syntax
(
2
n
((
2
n
n
A number that tells the ISPF editor how many positions to shift. If you omit this operand, the default is
2.
Description
To column shift one line toward the left side of your display:
1. Type ( in the line command field of the line to be shifted. Beside the command, type a number other
than 2 if you want to shift the line other than 2 columns.
2. Press Enter.
To column shift a block of lines toward the left side of your display:
1. Type (( in the line command field of the first line to be shifted. Beside the command, type a number
other than 2 if you want to shift the block of lines other than 2 columns.
2. Type (( in the line command field of the last line to be shifted. You can scroll (or use FIND or LOCATE)
between typing the first (( and the second ((, if necessary.
(—Column Shift Left
138  z/OS: z/OS ISPF Edit and Edit Macros

## Page 171

3. Press Enter. The lines that contain the two (( commands and all of the lines between them are column
shifted to the left.
The BOUNDS setting limits column shifting. If you shift columns beyond the current BOUNDS setting, the
editor deletes the text beyond the BOUNDS without displaying a warning message.
Examples
To shift a group of lines to the left three column positions, specify the number of columns and the range in
the line command field, as shown in Figure 54 on page 139. 
Figure 54. Before the ( (Column Shift Left) line command
Press Enter and the editor shifts the specified lines three columns to the right. See Figure 55 on page 139. 
Figure 55. After the ( (Column Shift Left) line command
(—Column Shift Left
Chapter 9. Edit line commands  139

## Page 172

)—Column Shift Right
The ) (column shift right) line command moves characters on a line to the right without altering their
relative spacing. Characters shifted past the current BOUNDS setting are deleted. See “Shifting data” on
page 42 for more information.
Syntax
)
2
n
))
2
n
n
A number that tells the ISPF editor how many positions to shift. If you omit this operand, the default is
2.
Description
To column shift one line toward the right side of your display:
1. Type ) in the line command field of the line to be shifted. Beside the command, type a number other
than 2 if you want to shift the data other than 2 columns.
2. Press Enter.
To column shift a block of lines toward the right side of your display:
1. Type )) in the line command field of the first line to be shifted. Beside the command, type a number
other than 2 if you want to shift the block of lines other than 2 columns.
2. Type )) in the line command field of the last line to be shifted. You can scroll (or use FIND or LOCATE)
between typing the first )) and the second )), if necessary.
3. Press Enter. The lines that contain the two )) commands and all of the lines between them are column
shifted to the right.
The BOUNDS setting limits column shifting. If you shift columns beyond the current BOUNDS setting, the
editor deletes the text beyond the BOUNDS without displaying a warning message.
Examples
To shift a group of lines to the right 3 column positions, specify the number of columns and the range in
the line command field, as shown in Figure 56 on page 141. 
)—Column Shift Right
140  z/OS: z/OS ISPF Edit and Edit Macros

## Page 173

Figure 56. Before the ) (Column Shift Right) line command
Figure 57 on page 141 shows that when you press Enter, the editor shifts the specified lines to the right 3
columns. 
Figure 57. After the ) (Column Shift Right) line command
<—Data Shift Left
The < (data shift left) line command moves the body of a program statement to the left without shifting
the label or comments. This command attempts to prevent loss of data. See “Shifting data” on page 42 for
more information.
<—Data Shift Left
Chapter 9. Edit line commands  141

## Page 174

Syntax
<
2
n
<<
2
n
n
A number that tells the ISPF editor how many positions to shift. If you omit this operand, the default is
2.
Description
To data shift one line toward the left side of your display:
1. Type < in the line command field of the line to be shifted. Beside the command, type a number other
than 2 if you want to shift the data other than 2 columns.
2. Press Enter.
To data shift a block of lines toward the left side of your display:
1. Type << in the line command field of the first line to be shifted. Beside the command, type a number
other than 2 if you want to shift the block of lines other than 2 columns.
2. Type << in the line command field of the last line to be shifted. You can scroll (or use FIND or LOCATE)
between typing the first << and the second <<, if necessary.
3. Press Enter. The lines that contain the two << commands and all of the lines between them are data
shifted to the left.
The BOUNDS setting limits data shifting. If you shift data beyond the current BOUNDS setting, the text
stops at the left bound and the shifted lines are marked with ==ERR> flags. If an error occurs in an
excluded line, you can find the error with LOCATE, and remove the error flag by using RESET.
Examples
To use a data shift to shift the body of the program statements (on lines 7 through 10) 7 spaces to the left,
specify the shift and the range in the line command field, as shown in Figure 58 on page 143.
<—Data Shift Left
142  z/OS: z/OS ISPF Edit and Edit Macros

## Page 175

Figure 58. Before the < (Data Shift Left) line command
When you press Enter, the editor deletes 7 blanks on the specified lines, as shown in Figure 59 on page
143. Notice that the editor does not shift the label on line 7 or the comments on lines 8 and 9.
Figure 59. After the < (Data Shift Left) line command
>—Data Shift Right
>—Data Shift Right
Chapter 9. Edit line commands  143

## Page 176

The > (data shift right) line command moves the body of a program statement to the right without shifting
the label or comments. This command attempts to prevent loss of data. See “Shifting data” on page 42 for
more information.
Syntax
>
2
n
>>
2
n
n
A number that tells the ISPF editor how many positions to shift. If you omit this operand, the default is
2.
Description
To data shift one line toward the right side of your display:
1. Type > in the line command field of the line to be shifted. Beside the command, type a number other
than 2 if you want to shift the line other than 2 columns.
2. Press Enter.
To data shift a block of lines toward the right side of your display:
1. Type >> in the line command field of the first line to be shifted. Beside the command, type a number
other than 2 if you want to shift the block of lines other than 2 columns.
2. Type >> in the line command field of the last line to be shifted. You can scroll (or use FIND or LOCATE)
between typing the first >> and the second >>, if necessary.
3. Press Enter. The lines that contain the two >> commands and all of the lines between them are data
shifted to the right.
The BOUNDS setting limits data shifting. If you shift data beyond the current BOUNDS setting, the text
stops at the right bound and the shifted lines are marked with ==ERR> flags. If an error occurs in an
excluded line, you can find the error with the LOCATE command, and remove the error flag by using
RESET.
Examples
To use a data shift to shift the body of the program statements (on lines 7 through 10) 7 spaces to the
right, specify the shift and the range in the line command field, as shown in Figure 60 on page 145. 
>—Data Shift Right
144  z/OS: z/OS ISPF Edit and Edit Macros

## Page 177

Figure 60. Before the > (Data Shift Right) line command
When you press Enter, the editor inserts 7 blanks on the specified lines. See Figure 61 on page 145.
Notice that the editor does not shift the label on line 7 or the comments on lines 8 and 9.
Figure 61. After the > (Data Shift Right) line command
A, AK—Specify an After destination
A, AK—Specify an After destination
Chapter 9. Edit line commands  145

## Page 178

When data is to be moved or copied, the A (after) line command specifies the line after which the data
is to be placed. When data is to be moved or copied to multiple destinations, the A (after) line command
specifies the final destination line after which the data is to be placed.
When data is to be moved or copied to multiple destinations, the AK (after, multiple targets) line
command specifies each multiple destination line (apart from the final destination line) after which the
data is to be placed.
Syntax
A
AK n
n
A number that tells the ISPF editor to repeat the associated line command a specified number of
times. If you do not type a number, or if the number you type is 1, the editor performs the command
only once. The number does not affect associated primary commands.
Description
To specify that data is to be moved or copied after a specific line:
1. Type one of the commands that are listed in this table. Line commands are typed in the line command
field. Primary commands are typed on the command line. 
Table 16. Line and primary commans for A and AK
Line commands Primary commands
“C—Copy Lines” on page 152 “COPY—Copy Data” on page 210
“M—Move Lines” on page 166 “MODEL—Copy a Model into the Current Data Set”
on page 249
  “MOVE—Move Data” on page 252
2. To specify a single destination for the data that is to be moved or copied, type A in the line command
field of the line that the moved or copied data is to follow. If you are specifying the destination
for a line command, a number after the A line command specifies the number of times the other
line command is performed. However, a number after the A command has no affect on a primary
command.
To specify multiple destinations for the data that is to be moved or copied:
a. Type AK in the line command field of each line (apart from the final destination) that the moved or
copied data is to follow.
b. Type A in the line command field of the final line that the moved or copied data is to follow.
3. Press Enter.
4. Some of the commands in the preceding table can cause another panel to be displayed if more
information is needed. If so, fill in the required information and press Enter to move, copy, or insert the
data. See the information about the specified command if you need help.
If no panel is displayed, the data is moved, copied, or inserted when you press Enter in step “3” on
page 146.
You must always specify a destination except when you are using a primary command to move, copy, or
insert data into a member or data set that is empty.
Two other line commands that are used to specify a destination are the B (before) command and the O
(overlay) command. See “B, BK—Specify a Before destination” on page 148 and “O, OK—Overlay Lines” on
page 172 for more information.
A, AK—Specify an After destination
146  z/OS: z/OS ISPF Edit and Edit Macros

## Page 179

Examples
Figure 62 on page 147 shows how you can move data with the M and A line commands. Type M in the line
command field of the line you want to move. Type A in the line command field of the line that you want the
moved line to follow.
Figure 62. Before the A (After) line command
When you press Enter, the line where you typed the M command is moved after the line where you typed
the A command. See Figure 63 on page 148.
Note:
1. If you press Enter before specifying where you want the data to go, the editor displays a MOVE/COPY
pending message at the top of the panel. The line does not move until you specify a destination.
2. The AK line command indicates that another destination of the form of A, B, or O is still required
proceeding forward through the file before the data is moved or copied to the multiple destinations
specified.
A, AK—Specify an After destination
Chapter 9. Edit line commands  147

## Page 180

Figure 63. After the A (After) line command
B, BK—Specify a Before destination
When data is to be moved or copied, the B (before) line command specifies the line before which the data
is to be placed. When data is to be moved or copied to multiple destinations, the B (before) line command
specifies the final destination line before which the data is to be placed.
When data is to be moved or copied to multiple destinations, the BK (before, multiple targets) line
command specifies each multiple destination line (apart from the final destination line) before which the
data is to be placed.
Syntax
B
BK n
n
A number that tells the ISPF editor to repeat the associated line command a specified number of
times. If you do not type a number, or if the number you type is 1, the command is not repeated. For
associated primary commands, this number has no effect.
Description
To specify that data is to be moved, copied, or inserted before a specific line:
1. Type one of the commands that are listed in this table. Line commands are typed in the line command
field. Primary commands are typed on the command line. 
B, BK—Specify a Before destination
148  z/OS: z/OS ISPF Edit and Edit Macros

## Page 181

Table 17. Line and primary commands for B
Line commands Primary commands
“C—Copy Lines” on
page 152
“COPY—Copy Data” on
page 210
“M—Move Lines” on
page 166
“MODEL—Copy a Model
into the Current Data
Set” on page 249
  “MOVE—Move Data” on
page 252
2. To specify a single destination for the data that is to be moved or copied, type B in the line command
field of the line that the moved or copied data is to precede. If you are specifying the destination
for a line command, a number after the B line command specifies the number of times the other
line command is performed. However, a number after the B command has no affect on a primary
command.
To specify multiple destinations for the data that is to be moved or copied:
a. Type BK in the line command field of each line (apart from the final destination) that the moved or
copied data is to precede.
b. Type B in the line command field of the final line that the moved or copied data is to precede.
3. Press Enter.
4. Some of the commands in the preceding table can cause another panel to be displayed if more
information is needed. If so, fill in the required information and press Enter to move, copy, or insert the
data. See the information about the specified command if you need help.
If no panel is displayed, the data is moved, copied, or inserted when you press Enter in step “3” on
page 149.
You must always specify a destination except when you are using a primary command to move, copy, or
insert data into a member or data set that is empty.
Two other line commands that are used to specify a destination are the A (after) command and the O
(overlay) command. See “A, AK—Specify an After destination” on page 145 and “O, OK—Overlay Lines” on
page 172 for more information.
Examples
Figure 64 on page 150 shows how you can copy data with the C and B line commands. Type C in the line
command field of the line you want to copy. Type B in the line command field of the line that the copied
line precedes.
B, BK—Specify a Before destination
Chapter 9. Edit line commands  149

## Page 182

Figure 64. Before the B (Before) line command
When you press Enter, the line where you typed the C command is moved before the line where you typed
the B command, as shown in Figure 65 on page 150.
Note:
1. If you press Enter before specifying where you want the data to go, the editor displays a MOVE/COPY
pending message at the top of the panel. The line does not move until you specify a destination.
2. The BK line command indicates that another destination of the form A, B, or O is still required
proceeding forward through the file before the data is moved or copied to the multiple destinations
specified.
Figure 65. After the B (Before) line command
B, BK—Specify a Before destination
150  z/OS: z/OS ISPF Edit and Edit Macros

## Page 183

BOUNDS—Define Boundary Columns
The BOUNDS line command displays the boundary definition line.
Syntax
BOUNDS
BOUND
BNDS
BND
BOU
Description
The BOUNDS line command provides an alternative to setting the boundaries with the BOUNDS primary
command or macro command; the effect on the member or data set is the same. However, if you use
both the BOUNDS primary command and the BOUNDS line command in the same interaction, the line
command overrides the primary command.
To display the boundary definition (=BNDS>) line:
1. Type BOUNDS in the line command field of any line that is not flagged.
2. Press Enter. The boundary definition line is inserted in the data set or member.
To change the BOUNDS settings:
1. Delete a < or > character. The < character shows the left BOUNDS setting and the > character shows
the right BOUNDS setting.
2. Move the cursor to a different location on the =BNDS> line.
Note: You can use the COLS line command with the BOUNDS line command to help check and
reposition the BOUNDS settings. The COLS line command displays the column identification line.
3. Retype the deleted character or characters.
Note: The < character must be typed to the left of the > character.
4. Press Enter. The new BOUNDS settings are now in effect.
To revert to the default settings:
1. Display the boundary definition line.
2. Blank out its contents with the Erase EOF key or the Del (delete) key.
3. Press Enter.
To remove the boundary definition line from the panel, you can either type D in the line command field
that contains the =BNDS> flag or type one of these commands on the command line:
• RESET (to reset all flagged lines), or
• RESET SPECIAL (to reset only the special lines)
The column numbers are always data column numbers (see “Referring to column positions” on page
106). Thus, for a variable format data set with number mode on, data column 1 is column 9 in the record.
See “Edit boundaries” on page 23 for more information, including tables that show commands affected by
BOUNDS settings and default bounds settings for various types of data sets.
BOUNDS—Define Boundary Columns
Chapter 9. Edit line commands  151

## Page 184

Examples
Figure 66 on page 152 shows the boundary definition line displayed with the column identification line.
Type BOUNDS in the line command field.
Figure 66. Before the BOUNDS line command
Figure 67 on page 152 shows that when you press Enter, the editor inserts the BOUNDS line and sets the
left bound at column 43 and the right bound at column 69.
Figure 67. After the BOUNDS line command
C—Copy Lines
C—Copy Lines
152  z/OS: z/OS ISPF Edit and Edit Macros

## Page 185

The C (copy) line command copies lines from one location to another.
Syntax
C
n
CC
n
The number of lines to be copied. If you do not type a number, or if the number you type is 1, only the
line on which you type C is copied.
Description
To copy one or more lines within the same data set or member:
1. Type C in the line command field of the line to be copied. If you also want to copy one or more lines
that immediately follow this line, type a number greater than 1 after the C command.
2. Next, specify the destination of the line to be copied by using either the A (after), B (before), or O
(overlay) line command.
3. Press Enter. The line or lines are copied to the new location.
To copy a block of lines within the same data set or member:
1. Type CC in the line command field of both the first and last lines to be copied. You can scroll (or use
FIND or LOCATE) between typing the first CC and the second CC, if necessary.
2. Use the A (after), B (before), or OO (overlay) command to show where the copied lines are to be
placed. Notice that when you use the block form of the C command (CC) to copy and overlay lines, you
should also use the block form of the O command (OO).
3. Press Enter. The lines that contain the two CC commands and all of the lines between them are copied
to the new location.
Note: Only blank characters in the lines specified with O or OO are overlaid with characters in the
corresponding columns from the source lines. Characters that are not blank are not overlaid. The overlap
affects only those characters within the current column boundaries.
To copy lines to another data set or member:
1. Type either CREATE or REPLACE on the command line.
2. Use one of the forms of the C command described previously.
3. Press Enter.
4. On the next panel that PDF displays, type the name of the data set or member that you want to create
or replace.
5. Press Enter. The lines are copied to the data set or member that you specified.
Note: To copy lines into an existing data set or member without replacing that data set or member, edit
the existing data set or member and use the COPY primary or macro command.
Examples
The example in Figure 68 on page 154 shows how to copy data by using the C and B line commands. Type
C in the line command field of the line you want to copy. Type B in the line command field of the line that
you want the copied line to precede.
C—Copy Lines
Chapter 9. Edit line commands  153

## Page 186

Figure 68. Before the C (Copy) line command
When you press Enter, the line where you typed the C command is copied preceding the line where you
typed the B command, as shown in Figure 69 on page 154.
Note: If you press Enter before specifying where you want the data to go, the editor displays a MOVE/
COPY pending message at the top of the panel. The line is not copied until you specify a destination.
While in MOVE/COPY pending mode, you can issue FIND and LOCATE primary commands, but you cannot
use a CHANGE command to change data until after the copy completes.
Figure 69. After the C (Copy) line command
C—Copy Lines
154  z/OS: z/OS ISPF Edit and Edit Macros

## Page 187

COLS—Identify Columns
The COLS line command displays a column identification line.
Syntax
COLS
COL
Description
To display the column identification (=COLS>) line:
1. Type COLS in the line command field of any line.
2. Press Enter.
The column identification line is inserted in the data set or member after the line in which you entered
COLS. The column identification line moves with the rest of the data when you scroll through the data
set or member. To display a non-scrolling, non-editable column indicator line, use the COLS primary
command. See “COLS—Display Fixed Columns Line” on page 206.
Note: You can use the COLS line command with the BOUNDS line command to help check and reposition
the bounds settings.
To remove the column identification line from the panel, you can either type D in the line command field
that contains the =COLS> flag, or type one of these commands on the command line:
• RESET (to reset all flagged lines), or
• RESET SPECIAL (to reset only the special lines)
Examples
The example in Figure 70 on page 156 shows the column identification line displayed with the boundary
definition line. The COLS command is typed in the line command field.
COLS—Identify Columns
Chapter 9. Edit line commands  155

## Page 188

Figure 70. Before the COLS line command
When you press Enter, the editor inserts the COLS line, as shown in Figure 71 on page 156.
Figure 71. After the COLS line command
D—Delete Lines
The D (delete) line command deletes lines from your display.
D—Delete Lines
156  z/OS: z/OS ISPF Edit and Edit Macros

## Page 189

Syntax
D
n
DD
n
The number of lines to be deleted. If you do not type a number, or if the number you type is 1, only the
line on which you type D is deleted.
Description
To delete one or more lines:
1. Type D in the line command field of the line to be deleted. If you also want to delete one or more lines
that immediately follow this line, type a number greater than 1 after the D command.
2. Press Enter.
The line or lines are deleted.
To delete a block of lines:
1. Type DD in the line command field of both the first and last lines to be deleted. You can scroll (or use
FIND or LOCATE) between typing the first DD and the second DD, if necessary.
2. Press Enter.
The lines that contain the two DD commands and all of the lines between them are deleted.
Examples
To delete two lines, type D2 in the line command field of the first line you want to delete. See Figure 72 on
page 157.
Figure 72. Before the D (Delete) line command
When you press Enter, the editor deletes the two lines specified. See Figure 73 on page 158.
D—Delete Lines
Chapter 9. Edit line commands  157

## Page 190

Figure 73. After the D (Delete) line command
F—Show the First Line
The F (show first line) line command redisplays one or more lines at the beginning of a block of excluded
lines. See “Redisplaying excluded lines” on page 58 for more information about excluding lines.
Syntax
F
n
n
The number of lines to be redisplayed. If you do not type a number, or if the number you type is 1, only
one line is redisplayed.
Description
To redisplay the first line or lines of a block of excluded lines:
1. Type F in the line command field next to the dashed line that shows where lines have been excluded.
The message in the dashed line tells you how many lines are excluded. If you want to redisplay more
than one line, type a number greater than 1 after the F command.
2. Press Enter.
The first line or lines are redisplayed.
Examples
The example in Figure 74 on page 159 shows how to redisplay the excluded lines of a member. To
redisplay the first three lines, type F3 in the line command field.
F—Show the First Line
158  z/OS: z/OS ISPF Edit and Edit Macros

## Page 191

Figure 74. Before the F (Show First Line) line command
When you press Enter, the editor displays the first three lines, as shown in Figure 75 on page 159.
Excluded lines do not need to be displayed again before saving the data. The excluded lines message line
is never saved.
Figure 75. After the F (Show First Line) line command
HX—Show data in hexadecimal format
The HX (hexadecimal) line command displays characters in a data set or member in hexadecimal format.
HX—Show data in hexadecimal format
Chapter 9. Edit line commands  159

## Page 192

Syntax
HX
n
HXX
n
The number of lines to be displayed in hexadecimal format. If you do not type a number, or if the
number you type is 1, only the line on which you type HX is displayed in hexadecimal format.
Description
To display characters on one or more lines in hexadecimal format:
1. Type HX in the line command field of the source code line that contains the characters you want to
display in hexadecimal format. If you also want to display characters in hexadecimal format on one or
more lines that immediately follow this line, type a number greater than 1 after the HX command.
2. Press Enter. The characters on the source code lines are displayed in hexadecimal format.
To display characters in a block of lines in hexadecimal format:
1. Type HXX in the line command field of both the first and last source code lines that you want to display
in hexadecimal format. You can scroll (or use FIND or LOCATE) between typing the first HXX and the
second HXX, if necessary.
2. Press Enter. The characters in the source code lines that contain the two HXX commands and in all of
the source code lines between them are displayed in hexadecimal format.
When the file is not being displayed totally in hexadecimal format (that is, the HEX ON primary command
is not in effect), records that have been marked by a HX or a HXX edit line command are displayed as a set
of four lines, similar to the way records are displayed when the HEX ON primary command is used. The HX
and HXX edit line commands act in a toggle manner to change the display of records. That is, if the record
is already displayed in hexadecimal format due to an HX or HXX command, then issuing another HX or
HXX command turns off the hexadecimal display for the record.
Note: The effect of any previous HX or HXX commands are canceled by the command, HEX OFF. HX is not
available with FASTPATH panels such as ISREDDE.
Examples
Figure 76 on page 161 shows how to use the HX command without any operands. To display a line in
hexadecimal format, type HX in the line command field of the line you want to display.
HX—Show data in hexadecimal format
160  z/OS: z/OS ISPF Edit and Edit Macros

## Page 193

Figure 76. Before the HX (display in hexadecimal format) line command
When you press Enter, the editor converts the characters in the line to hexadecimal format. See Figure 77
on page 161.
Figure 77. After the HX (display in hexadecimal format) line command
I—Insert Lines
I—Insert Lines
Chapter 9. Edit line commands  161

## Page 194

The I (insert) line command inserts one or more lines in your data set or member. The inserted lines are
blank unless you have defined a mask. See “MASK—Define Masks” on page 169 for more information
about defining a mask.
Syntax
I
n
n
The number of blank lines to insert. If you do not type a number, or if the number you type is 1, only
one line is inserted.
Description
To insert one or more lines in a data set or member:
1. Type I in the line command field of the line that the inserted line is to follow. If you want to insert more
than one line, type a number greater than 1 after the I command.
2. Press Enter. The line or lines are inserted.
If you type any information, even a blank character in the inserted line, the line becomes part of the
source data and is assigned a line number the next time you press Enter. However, if you do not type any
information, the space for the new line is automatically deleted the next time you press Enter.
If you type information on the last, or only, inserted line and the cursor is still in the data portion of that
line, the editor automatically inserts another line when you press Enter or a scroll function key, but only
if the new inserted line remains on the panel. If the new line is at the bottom of the panel, the editor
automatically scrolls down so that the new line is displayed at the bottom of the screen.
Examples
Figure 78 on page 162 shows how to insert lines in a member. To insert three lines, type I3 in the line
command field.
Figure 78. Before the I (Insert) line command
When you press Enter, the editor inserts three lines. See Figure 79 on page 163.
I—Insert Lines
162  z/OS: z/OS ISPF Edit and Edit Macros

## Page 195

Figure 79. After the I (Insert) line command
L—Show the Last Line(s)
The L (show last line) line command redisplays one or more lines at the end of a block of excluded lines.
See “Redisplaying excluded lines” on page 58 for more information about excluding lines.
Syntax
L
n
n
The number of lines to be redisplayed. If you do not type a number, or if the number you type is 1, only
one line is redisplayed.
Description
To redisplay the last line or lines of a block of excluded lines:
1. Type L in the line command field next to the dashed line that shows where lines have been excluded.
The message in the dashed line tells you how many lines are excluded. If you want to redisplay more
than one line, type a number greater than 1 after the L command.
2. Press Enter. The last line or lines are redisplayed.
Examples
Figure 80 on page 164 shows how to redisplay the last three excluded lines. To redisplay the last three
lines, type L3 in the line command field of the excluded lines.
L—Show the Last Line(s)
Chapter 9. Edit line commands  163

## Page 196

Figure 80. Before the L (Show Last Line) line command
When you press Enter, the editor redisplays the last three lines. See Figure 81 on page 164.
Note: Excluded lines do not need to be displayed again before saving the data. The excluded lines
message line is never saved.
Figure 81. After the L (Show Last Line) line command
LC—Convert Characters to Lowercase
LC—Convert Characters to Lowercase
164  z/OS: z/OS ISPF Edit and Edit Macros

## Page 197

The LC (lowercase) line command converts characters in a data set or member from uppercase to
lowercase. However, it does not affect the caps mode of the data that you are editing.
Syntax
LC
n
LCC
LCLC
n
The number of lines to be converted to lowercase. If you do not type a number, or if the number you
type is 1, only the line on which you type LC is converted to lowercase.
Description
To convert characters on one or more lines to lowercase:
1. Type LC in the line command field of the source code line that contains the characters you want to
convert. If you also want to convert characters on one or more lines that immediately follow this line,
type a number greater than 1 after the LC command.
2. Press Enter. The characters on the source code lines are converted to lowercase.
To convert characters in a block of lines to lowercase:
1. Type LCC in the line command field of both the first and last source code lines that contain characters
that are to be converted. You can scroll (or use FIND or LOCATE) between typing the first LCC and the
second LCC, if necessary.
2. Press Enter. The characters in the source code lines that contain the two LCC commands and in all of
the source code lines between them are converted to lowercase.
See the UC (uppercase) line command and the CAPS primary and macro commands, which are related, for
information about converting characters from uppercase to lowercase and vice versa.
Examples
Figure 82 on page 166 shows how to use the LC command without any operands. To convert a line, type
LC in the line command field of the line you want to convert.
LC—Convert Characters to Lowercase
Chapter 9. Edit line commands  165

## Page 198

Figure 82. Before the LC (Lowercase) line command
When you press Enter, the editor converts the characters in the line to lowercase. See Figure 83 on page
166.
Figure 83. After the LC (Lowercase) line command
M—Move Lines
The M (move) line command moves lines from one location to another.
M—Move Lines
166  z/OS: z/OS ISPF Edit and Edit Macros

## Page 199

Syntax
M
n
MM
n
The number of lines to be moved. If you do not type a number, or if the number you type is 1, only the
line on which you type M is moved.
Description
To move one or more lines within the same data set or member:
1. Type M in the line command field of the line to be moved. If you want to move one or more lines that
immediately follow this line, type a number greater than 1 after the M command.
2. Next, specify the destination of the line to be moved by using either the A (after), B (before), or O
(overlay) line command. See the descriptions of those commands if you need more information about
them.
3. Press Enter. The line or lines are moved to the new location.
To move a block of lines within the same data set or member:
1. Type MM in the line command field of both the first and last lines to be moved. You can scroll (or use
FIND or LOCATE) between typing the first MM and the second MM, if necessary.
2. Use the A (after), B (before), or OO (overlay) command to show where the moved lines are to be
placed. Notice that when you use the block form of the M command (MM) to move and overlay lines,
you should also use the block form of the O command (OO).
3. Press Enter. The lines that contain the two MM commands and all of the lines between them are moved
to the new location.
Note: Only blank characters in the lines specified with O or OO are overlaid with characters in the
corresponding columns from the source lines. Characters that are not blank are not overlaid. The overlap
affects only those characters within the current column boundaries.
To move lines to another data set or member:
1. Type either CREATE or REPLACE on the command line.
2. Use one of the forms of the M command described previously.
3. Press Enter.
4. On the next panel, type the name of the data set or member that you want to create or replace.
5. Press Enter. The lines are moved to the data set or member that you specified.
Note: To move lines into an existing data set or member without replacing that data set or member, use
the MOVE primary or macro command.
Examples
Figure 84 on page 168 shows how you can move data by using the M with the A (After) line command. To
move a line, type M in the line command field of the line you want to move. Type a A in the line command
field of the line you want the moved line to follow.
M—Move Lines
Chapter 9. Edit line commands  167

## Page 200

Figure 84. Before the M (Move) line command
When you press Enter, the editor moves the line where you typed the M command to a position
immediately after the line where you typed the A command, as shown in Figure 85 on page 168. If
you press Enter before specifying a destination, the editor displays a MOVE/COPY pending message at
the top of the panel. The line is not moved until you specify a destination.
Note: While in MOVE/COPY pending mode, you can issue FIND and LOCATE primary commands, but you
cannot use a CHANGE command to change data until after the copy completes.
Figure 85. After the M (MOVE) line command
M—Move Lines
168  z/OS: z/OS ISPF Edit and Edit Macros

## Page 201

MASK—Define Masks
The MASK line command displays the =MASK> line. On this line, you can type characters that you want to
insert into an unformatted data set or member. These characters, which are called the mask, are inserted
whenever you use the I (insert), TE (text entry), or TS (text split) line commands, or when you edit an
empty data set.
Syntax
MASK
Description
To display the =MASK> line:
1. Type MASK in the line command field of any line.
2. Press Enter. The =MASK> line is displayed.
Initially, the mask contains all blanks. To define a mask:
1. Add characters to or delete characters from the =MASK> line while it is displayed.
2. Press Enter. The mask is now defined.
Once a mask is defined, the contents of the =MASK> line are displayed whenever a new line is inserted.
This occurs when you use the I (insert), TE (text entry), and TS (text split) line commands, and when
you edit an empty data set. You can change the mask definition whenever you need to by repeating the
preceding steps.
To remove the =MASK> line from the panel, perform one of these actions:
• Type D in the line command field that contains the =MASK> flag and press Enter.
• Type RESET on the command line and press Enter.
• End the edit session by:
– Pressing F3 (if it is defined as the END command), or
– Typing END on the command line and pressing Enter
The mask line is never saved as part of the data. However, the mask remains in effect, even if it is not
displayed, until you change it. The contents of the mask are retained in the current edit profile, and are
automatically used the next time you edit the same kind of data.
The MASK command is ignored in formatted edit mode. You enter formatted edit mode when you type the
name of a previously defined format in the Format Name field on the Edit Entry panel when beginning an
edit session. If you have defined a mask before entering formatted edit mode, the mask is not retained in
the current edit profile.
Examples
In Figure 86 on page 170, the mask is displayed and the characters /* and */ are typed on the mask line.
MASK—Define Masks
Chapter 9. Edit line commands  169

## Page 202

Figure 86. Before the MASK line command
When you insert five lines, the new lines contain the contents of the mask. See Figure 87 on page 170.
Figure 87. After the MASK line command
MD—Make Dataline
The MD (make dataline) line command converts one or more ==MSG>, =NOTE=, =COLS>, or ======
(information) lines to data so they can be saved as part of your data set.
MD—Make Dataline
170  z/OS: z/OS ISPF Edit and Edit Macros

## Page 203

Syntax
MD
n
MDD
MDMD
n
The number of lines to be converted to data. If you do not type a number, or if the number you type is
1, only the line on which you type MD is converted.
Description
If you enter the MD line command on:
• Any line except a ==MSG>, =NOTE=, =COLS>, or ====== line, it is ignored.
• The TOP OF DATA and BOTTOM OF DATA lines, it is not allowed.
• An excluded line, any converted lines remain excluded and are converted.
• A line that contains a label, the label remains after the line is converted.
Note: The MD line command only works on the editable =COLS> lines produced by the COLS line
command. It does not work with the non-editable =COLS> indicator line produced by the COLS primary
command.
For best results, you should set your edit profile to NUMBER OFF and make sure that the record length of
your data set or member is at least 80 before entering the MD line command. Otherwise, data on the right
may be truncated.
To convert one or more lines to data:
1. Type MD in the line command field next to the line that is to be converted. If you also want to
convert one or more lines that immediately follow this line, type a number greater than 1 after the MD
command.
2. Press Enter. The lines are converted to data.
To convert a block of lines to data:
1. Type MDD in the line command field of both the first and last lines to be converted. You can scroll (or
use the FIND or LOCATE command) between typing the first MDD and the second MDD, if necessary.
2. Press Enter. The lines that contain the two MDD commands and all eligible lines between them are
converted to data.
Examples
Figure 88 on page 172 shows how you can convert a block of temporary lines to data by using the block
form of the MD line command. Type MDD over the =NOTE= line flags in the line command field of the first
and last lines of the block of lines that you want to convert to data.
MD—Make Dataline
Chapter 9. Edit line commands  171

## Page 204

Figure 88. Before the MD (Make Dataline) line command
When you press Enter, the lines on which the MDD commands are typed and all of the lines between them
are converted to data. See Figure 89 on page 172.
Figure 89. After the MD (Make Dataline) line command
O, OK—Overlay Lines
When data is to be copied or moved by the C (copy) or M (move) line commands and overlaid on one or
more existing lines of data, the O (overlay) line command specifies the destination for the data.
O, OK—Overlay Lines
172  z/OS: z/OS ISPF Edit and Edit Macros

## Page 205

If there are multiple destinations for the data, the OK (overlay, intermediate target) line command
specifies each intermediate destination for the data. You specify the final destination for the data with
either the O (overlay), A (after), or B (before) line commands. The final destination must be after the
intermediate destinations in the file. For more information about the A and B line commands, see:
“A, AK—Specify an After destination” on page 145
“B, BK—Specify a Before destination” on page 148
The data that is copied or moved overlays blanks in the destination lines of data. This allows you to
rearrange a single-column list of items into multiple column, or tabular, format.
When data is to be moved or copied and then overlaid on a single destination:
• Where the destination is a single line:
– The O (overlay) line command specifies the destination for the data.
You can type a number after the O line command to specify the number of times that the M or C line
command is to be performed. For example, typing the command O3 against a line causes the data to
be moved or copied and then overlaid on that line and also the next two lines.
• Where the destination is a block of lines:
– The OO (overlay, multiple-line target) line command specifies the first and last line of the destination
for the data.
When data is to be moved or copied and then overlaid on multiple destinations:
• Where each destination is a single line:
– The OK (overlay, intermediate target) line command specifies each intermediate destination (but not
the final destination) for the data.
You can type a number after the OK line command to specify the number of times that the M or C line
command is to be performed. For example, typing the command OK3 against a line causes the data to
be moved or copied and then overlaid on that line and also the next two lines.
– The O (overlay) line command specifies the final destination for the data.
You can type a number after the O line command as previously described.
• Where each destination is a block of lines:
– The OOK (overlay, intermediate multiple-line target) line command specifies the first and last line of
each intermediate destination (but not the final destination) for the data.
– The OO (overlay, multiple-line target) line command specifies the first and last line of the final
destination for the data.
Note: The OK and OOK line commands indicate that another destination of the form A, B, or O is
still required proceeding forward through the file before the data is moved or copied to the multiple
destinations specified.
Syntax
O
OK n
OO
OOK
n
The number of lines to be overlaid. If you do not type a number, or if the number you type is 1, only
one line is overlaid.
O, OK—Overlay Lines
Chapter 9. Edit line commands  173

## Page 206

Description
To overlay one or more single lines:
1. Type either M or C in the line command field of the line that is to be moved or copied.
2. To specify a single destination for the data that is to be moved or copied, type O in the line command
field of the line that the moved or copied line is to overlay.
To specify multiple destinations for the data that is to be moved or copied:
a. Type OK in the line command field of each intermediate destination line (but not the final
destination line) that the moved or copied data is to overlay.
To overlay data in the lines following an intermediate destination line, type a number after the OK
line command to specify the number of times that the M or C line command is to be performed.
b. Type O in the line command field of the final destination line that the moved or copied data is to
overlay. The final destination line must come after all the intermediate destination lines.
To overlay data in the lines following the final destination line, type a number after the O line
command to specify the number of times that the M or C line command is to be performed.
3. Press Enter. The data being moved or copied overlays the specified line or lines.
To overlay one or more blocks of lines:
1. Type either MM or CC in the line command field of the first and last lines of a block of lines that is to be
moved or copied. You can scroll (or use FIND or LOCATE) between typing the first command and the
second command, if necessary.
2. To specify a single destination for the data that is to be moved or copied, type OO in the line command
field of the first and last lines that the block of lines being moved or copied is to overlay. Again, you can
scroll (or use FIND or LOCATE) between typing the first OO and the second OO, if necessary.
To specify multiple destinations for the data that is to be moved or copied:
a. Type OOK in the line command field of the first and last lines of each intermediate destination (but
not the final destination) that the block of lines being moved or copied is to overlay.
b. Type OO in the line command field of the first and last lines of the final destination that the moved
or copied data is to overlay. The lines of the final destination must come after all those of the
intermediate lines.
3. Press Enter. The lines that contain the two CC or MM commands and all of the lines between them
overlay the lines that contain:
• Each pair of OOK commands and all of the lines between them.
• The two OO commands and all of the lines between them.
Only blank characters in the lines specified with O or OO (or OK or OOK) are overlaid with characters in the
corresponding columns from the source lines. Characters that are not blank are not overlaid. The overlap
affects only those characters within the current column boundaries.
The number of source and receiving lines need not be the same. If there are more receiving lines, the
source lines are repeated until the receiving lines are gone. If there are more source lines than receiving
lines, the extra source lines are ignored. The overlay operation involves only data lines. Special lines such
as MASK, TABS, BNDS, and COLS are ignored as either source or receiving lines.
Note: There is no special support for DBCS data handling. You are responsible for DBCS data integrity
when overlaying lines.
Examples
Figure 90 on page 175 illustrates the O (overlay) line command. Suppose you were editing a list in a single
left-adjusted column and wanted to place portions of the list side-by-side. First, using the ) (column shift
right) command, shift a portion of the list the appropriate amount to the right to overlay in a multiple
column format. Next, type MM in the line command field to mark the beginning and end of the block of
O, OK—Overlay Lines
174  z/OS: z/OS ISPF Edit and Edit Macros

## Page 207

lines you want to move, then type OO in the line command field to mark the destination of the lines you
want to overlay.
Figure 90. Before the O (Overlay) line command
When you press Enter, the editor overlays the lines you marked to move on the destination block. See
Figure 91 on page 175.
Figure 91. After the O (Overlay) line command
R—Repeat Lines
R—Repeat Lines
Chapter 9. Edit line commands  175

## Page 208

The R (repeat) line command repeats one or more lines in your data set or member immediately after the
line on which the R command is entered.
Syntax
R
n
RR
n
n
The number of lines to be repeated. If you do not type a number, or the number you type is 1, only the
line on which you type R is repeated.
Description
To repeat one or more lines:
1. Type R in the line command field of the line that is to be repeated. If you want to repeat the line more
than once, type a number that is greater than 1 immediately after the R command.
2. Press Enter. The editor inserts a duplicate copy or copies of the line immediately after the line that
contains the R command.
To repeat a block of lines:
1. Type RR in the line command field of both the first and last lines to be repeated. You can scroll (or use
FIND or LOCATE) between typing the first RR and the second RR, if necessary.
2. Press Enter. The lines that contain the two RR commands and all of the lines between them are
repeated immediately after the line that contains the second RR command.
Examples
Figure 92. Before the R (repeat) line command
When you press Enter, the editor repeats line 000400 five times. See Figure 93 on page 177.
R—Repeat Lines
176  z/OS: z/OS ISPF Edit and Edit Macros

## Page 209

Figure 93. After the R (Repeat) line command
S—Show Lines
The S (show line) line command causes one or more lines in a block of excluded lines to be redisplayed.
The redisplayed lines have the leftmost indentation levels; they contain the fewest leading blanks. See
“Redisplaying excluded lines” on page 58 for more information about redisplaying excluding lines.
Syntax
S
n
n
The number of lines to be redisplayed. If there are more than 2 excluded lines, and you do not type a
number or if the number you type is 1, only one line is redisplayed.
Note: If you enter an S line command to display all but one line of an excluded block, then that line is also
displayed. This could result in more lines being displayed than the number you requested. For example, if
five lines are excluded in a block, an S4 command causes all five lines to be displayed.
Description
To redisplay a line or lines of a block of excluded lines:
1. Type S in the line command field next to the dashed line that shows where a line or lines has been
excluded. The message in the dashed line tells you how many lines are excluded.
If you want to redisplay more than one line, type a number greater than 1 after the S command. If you
type S3, for example, the three lines with the leftmost indentation level are displayed again. If more
than three lines exist at this indentation level, only the first three are displayed.
2. Press Enter. The line or lines with the fewest leading blanks are redisplayed.
S—Show Lines
Chapter 9. Edit line commands  177

## Page 210

Examples
Figure 94 on page 178 shows how to redisplay a member's excluded lines. To redisplay four lines, type S4
in the line command field.
Figure 94. Before the S (Show) line command
When you press Enter, the four lines are redisplayed. See Figure 95 on page 178.
Note: Excluded lines do not need to be displayed again before saving the data. The excluded lines
message line is never saved.
Figure 95. After the S (Show) line command
S—Show Lines
178  z/OS: z/OS ISPF Edit and Edit Macros

## Page 211

TABS—Control Tabs
The TABS line command:
• Displays the =TABS> (tab-definition) line
• Defines tab positions for software, hardware, and logical tabs
Use PROFILE to check the setting of tabs mode and the logical tab character. See “Using tabs” on page 63
if you need more information about using tabs.
Syntax
TABS
TAB
Description
When you type TABS in the line command field, =TABS> is displayed along with any previously defined tab
positions. To remove the =TABS> line, use the D (delete) line command or the RESET primary command,
or end the edit session. The =TABS> line is never saved as part of the data.
The tab definitions remain in effect, even if they are not displayed, until you change them. Tab definitions
are retained in the current edit profile, and are automatically used the next time you edit the same kind of
data.
Using software and hardware tabs
Edit a data set, type TABS ALL on the command line, and press Enter:
Command ===> TABS ALL
Now, type COLS in the line command field and press Enter again. A partial =COLS> line with positions 9
through 45 is shown in this example:
=COLS> -1----+----2----+----3----+----4----+
Next use the TABS line command to define software and hardware tabs. Type TABS in the line command
field beneath the =COLS> line and press Enter.
When the =TABS> line appears, type hyphens in columns 15, 25, and 35, and asterisks in columns 20, 30,
and 40, using the =COLS> line to find these columns:
=COLS> -1----+----2----+----3----+----4----+
=TABS>       -    *    -    *    -    *
With the preceding =TABS> line, you can move the cursor to a software tab position (hyphen) by pressing
Enter, even if another character already occupies that position. To move the cursor to a hardware tab
position (one space to the right of an asterisk), press either the Tab Forward or Tab Backward key. See
Figure 96 on page 180.
TABS—Control Tabs
Chapter 9. Edit line commands  179

## Page 212

Figure 96. TAB line command example
Using software tab fields
You can define a software tab field  by typing underscores or hyphens in two or more consecutive columns.
This moves the cursor to the first nonblank character in the field. If the field contains all blanks, the cursor
moves to the beginning of the field.
Using the example in the preceding section, create a software tab field by typing hyphens in columns 10
through 14. Then type some data inside the field and at each of the other tab positions, but below the
=TABS> line:
=COLS> -1----+----2----+----3----+----4----+
=TABS>  ------    *    -    *    -    *
          123          456       789_
Notice in the preceding example that the cursor is positioned to the right of data string 789. With the
cursor in this position, press Enter. The cursor moves under the 1 in the 123 data string, not to column 10,
which is the beginning of the field.
TE—Text Entry
The TE (text entry) line command provides one very long line wrapped around many lines of the display to
allow power typing for text entry. The editor does the formatting for you.
The TE line command is different from the I (insert) line command. The I command inserts a specified
number of separate, blank lines as well as the mask, if there is one, as you typed it. With the TE command,
the input data is formatted, only mask line characters outside the current boundaries are added to the
formatted lines.
Syntax
TE
n
TE—Text Entry
180  z/OS: z/OS ISPF Edit and Edit Macros

## Page 213

n
The number of blank lines to be added. If you do not type a number, the display is filled with blanks
from the line following the TE to the bottom of the screen.
Description
Before you enter text entry mode:
• If you are going to be typing text in paragraph form, make sure caps mode is off. Otherwise, when you
press Enter, your text changes to uppercase.
• You may want to turn off number mode to prevent sequence numbers from writing over any of your text.
• Make sure the bounds setting is where you want it so that the text will flow correctly when you end text
entry mode.
To enter text entry mode:
1. Type TE in the line command field. If you want to specify several blank lines, type a number greater
than 1 immediately after the TE command. If the number that you type is greater than the number
of lines remaining on the display, the vertical bar that shows where you will run out of room is not
displayed and the keyboard does not lock at the last character position on the display. You can scroll
down to bring the additional blank text entry space into view.
2. Press Enter. The editor inserts a single continuous blank area for the specified number of lines or to the
bottom of the display.
To begin a new paragraph:
1. Use the return (Enter), cursor movement, or Tab keys to advance the cursor enough spaces to leave
one blank line on the display.
If there are insufficient blank spaces on the display, the keyboard locks when you try to type beyond the
last character position. A vertical bar (|) is displayed above the cursor at the locked position.
To generate more blank spaces:
1. Press the Reset key to unlock the keyboard.
2. Press Enter.
To end text entry mode:
1. Press Enter. The data is flowed together into a paragraph and any embedded blanks are preserved. The
left and right sides of the paragraph are determined by the current bounds.
See “Word processing” on page 61 and “Entering text (power typing)” on page 63 if you need more
information.
Examples
Figure 97 on page 182 shows how the TE (text entry) command allows you to use power typing and word
wrap to input text. The edit profile is set to NUMBER OFF and CAPS OFF. Also, the left bound is set to
1 and the right bound is set to 72. A new data set member called CHAP10 has been started and the TE
command is typed in the line command field.
TE—Text Entry
Chapter 9. Edit line commands  181

## Page 214

Figure 97. Before the TE (Text Entry) line command
When you press Enter, the editor begins text entry mode. The cursor shows where text input begins and
the vertical bar in the lower-right corner of the panel shows how much room you have to work with. See
Figure 98 on page 182.
Figure 98. After the TE (Text Entry) line command
When you enter text, some of the words are split between lines, with part of the word at the right end of a
line and the remainder of the word at the beginning of the next line. See Figure 99 on page 183.
TE—Text Entry
182  z/OS: z/OS ISPF Edit and Edit Macros

## Page 215

Figure 99. Sample text during text entry mode
When you press Enter, the editor exits text entry mode. As shown in Figure 100 on page 183, the text
flows between the bounds settings and the line numbers are displayed in the line command field.
Figure 100. Sample text after text entry mode
TF—Text Flow
The TF (text flow) line command restructures paragraphs. This is sometimes necessary after deletions,
insertions, or splitting.
TF—Text Flow
Chapter 9. Edit line commands  183

## Page 216

Syntax
TF
n
n
The column number to which the text should be flowed. The default is the panel width when default
boundaries are in effect. If you are using nondefault bounds, the right boundary is used. This is
different from the TFLOW macro command, which always defaults to the right boundary.
If a number greater than the right boundary is specified, the right boundary is used.
Description
To flow text:
1. Type TF in the line command field of the line at which you want the text to begin flowing. If you
want to specify the rightmost column position for the restructured text, type a number greater than 1
immediately after the TF command.
2. Press Enter. The text is flowed from the beginning of that line to the end of the paragraph.
See “Word processing” on page 61 and “Formatting paragraphs” on page 61 for more information.
Examples
Figure 101 on page 184 demonstrates text restructuring. The bounds are set at columns 1 and 72. A
TF50 command is typed on line 000041.
Figure 101. Before the TF (Text Flow) line command
When you press Enter, the editor takes all text in that paragraph between columns 1 and 72 and reformats
it between columns 1 and 50. See Figure 102 on page 185.
TF—Text Flow
184  z/OS: z/OS ISPF Edit and Edit Macros

## Page 217

Figure 102. After the TF (Text Flow) line command
TS—Text Split
The TS (text split) line command moves part or all of a line of text to the following line. This makes it
easier for you to add new material to existing text.
Syntax
TS
n
n
The number of blank lines to be inserted between the split lines. If you do not type a number, or if the
number that you type is 1, the editor inserts only one blank line.
Description
To split a line:
1. Type TS in the line command field of the line you would like to split. If you want to insert more
than one blank line between the split lines, type a number greater than 1 immediately after the TS
command.
2. Move the cursor to the desired split point.
3. Press Enter.
To rejoin lines, use the TF (text flow) line command. See “TF—Text Flow” on page 183 for more
information.
For more information about splitting lines and other word processing commands, see “Word processing”
on page 61 and “Splitting lines” on page 62.
TS—Text Split
Chapter 9. Edit line commands  185

## Page 218

Examples
Figure 103 on page 186 shows how to split text and to insert blank lines. To split the text and insert three
lines, type TS3 in the line command field of the line you want to split and place the cursor where you want
the line split.
Figure 103. Before TS (Text Split) line command
When you press Enter, the line is split at the cursor position and the editor inserts the number of blank
lines specified, as shown in Figure 104 on page 186.
Figure 104. After TS (Text Split) line command
TS—Text Split
186  z/OS: z/OS ISPF Edit and Edit Macros

## Page 219

UC—Convert Characters to Uppercase
The UC (uppercase) line command converts characters in a data set or member from lowercase to
uppercase. However, it does not affect the caps mode of the data that you are editing.
Syntax
UC
n
UCC
UCUC
n
The number of lines to be converted to uppercase. If you do not type a number, or if the number you
type is 1, only the line on which you type UC is converted to uppercase.
Description
To convert characters on one or more lines to uppercase:
1. Type UC in the line command field of the source code line that contains the characters that you want
to convert. To convert characters on lines following this one, type a number greater than 1 after the UC
command.
2. Press Enter. The characters on the source code line or lines are converted to uppercase.
To convert characters in a block of lines to uppercase:
1. Type UCC in the line command field of both the first and last source code lines that contain characters
that are to be converted. You can scroll (or use FIND or LOCATE) between typing the first UCC and the
second UCC, if necessary.
2. Press Enter. The characters in the source code lines that contain the two UCC commands and in all of
the source code lines between them are converted to uppercase.
See the “LC—Convert Characters to Lowercase” on page 164 line command and the CAPS primary
command (“CAPS—Control Automatic Character Conversion” on page 202) and macro command (“CAPS—
Set or Query Caps Mode” on page 309) for information about converting characters from uppercase to
lowercase and vice versa.
Examples
Figure 105 on page 188 shows how to convert lines of text to uppercase. To convert lines of text to
uppercase, place the UC command and the number of lines you want to convert in the line command field
where you want the conversion to start.
UC—Convert Characters to Uppercase
Chapter 9. Edit line commands  187

## Page 220

Figure 105. Before the UC (Uppercase) line command
When you press Enter, the editor converts the lines specified to uppercase. See Figure 106 on page 188.
Figure 106. After the UC (Uppercase) line command
X—Exclude Lines
The X (exclude) line command replaces one or more lines on the panel with a dotted line. The dotted line
contains a message that specifies how many lines have been excluded.
X—Exclude Lines
188  z/OS: z/OS ISPF Edit and Edit Macros

## Page 221

The excluded lines are not erased. They are simply hidden from view and can still be affected by edit line,
primary, and macro commands.
Syntax
X
n
XX
n
The number of lines to be excluded. If you do not type a number, or if the number that you type is 1,
PDF excludes only the line on which you type the X command.
Description
To exclude one or more lines:
1. Type X in the line command field of the line that you want to exclude. If you want to exclude one or
more lines that immediately follow this line, type a number greater than 1 immediately after the X
command.
2. Press Enter. The lines are excluded from the panel.
To exclude a block of lines:
1. Type XX in the line command field of both the first and last lines that you want to exclude. You can
scroll (or use FIND or LOCATE) between typing the first XX and the second XX, if necessary.
2. Press Enter. The lines that contain the two XX commands and all of the lines between them are
excluded.
See “Excluding lines” on page 57 for more information on using this command.
Examples
Figure 107 on page 189 shows how lines are excluded from a member. To exclude six lines, type X6 in the
line command field.
Figure 107. Before the X (Exclude) line command
X—Exclude Lines
Chapter 9. Edit line commands  189

## Page 222

When you press Enter, the editor excludes the specified lines. See Figure 108 on page 190. 
Figure 108. After the X (Exclude) line command
To redisplay excluded lines, use the F (show first line), L (show last line), or S (show lines) line command.
X—Exclude Lines
190  z/OS: z/OS ISPF Edit and Edit Macros
