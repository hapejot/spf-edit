# Chapter 10. Edit primary commands

Source file: f54em00_v3r1.md
Start page: 223
Page span: 223-326

## Page 223

Chapter 10. Edit primary commands
Primary commands affect the entire data set being edited, whereas line commands usually affect only a
single line or block of lines. To enter a primary command, either:
• Type the command on the command line and press Enter, or
• Press the function key to which the command is assigned
Most primary commands can be abbreviated. In fact, many can be typed as a single letter, such as L
for LOCATE or F for FIND. In this topic, the syntax diagram for each command shows the allowable
abbreviations, if any. For a complete list of command abbreviations, see Appendix A, “ Abbreviations for
Commands and Other Values,” on page 427.
Each command description consists of:
Syntax
A syntax diagram for coding the command, including a description of any required or optional
operands.
Description
A summary of the function and operation of the command. This definition also refers to other
commands that can be used with this command.
Example
Sample usage of the command.
Edit primary command summary
This table summarizes the edit primary commands. See the complete description of the commands on the
referenced page.
Table 18. Summary of the primary commands
Command Description
“AUTOLIST—Create a Source Listing Automatically”
on page 194
Controls the automatic printing of data to the ISPF
list data set.
“AUTONUM—Number Lines Automatically” on page
195
Controls the automatic renumbering of data when
it is saved.
“AUTOSAVE—Save Data Automatically” on page
197
If the data is changed, automatically saves it when
you issue an END command.
“BOUNDS—Control the Edit Boundaries” on page
199
Sets the left and right boundaries.
“BROWSE—Browse from within an Edit Session” on
page 200
Browses a data set or member without leaving your
current edit session.
“BUILTIN—Process a Built-In Command” on page
201
Processes a built-in command even if a macro with
the same name has been defined.
“CANCEL—Cancel Edit Changes” on page 201 Ends the edit session without saving any of the
changes.
“CAPS—Control Automatic Character Conversion”
on page 202
Sets caps mode.
“CHANGE—Change a Data String” on page 203 Changes a data string into another string.
Edit Primary Command Summary
© Copyright IBM Corp. 1984, 2024 191

## Page 224

Table 18. Summary of the primary commands (continued)
Command Description
“COMPARE—Edit Compare” on page 207 Compares library member or data set with the data
being edited.
“COPY—Copy Data” on page 210 Copies a library member or data set into the data
being edited.
“CREATE—Create Data” on page 215 Writes the data you are editing into a library
member or data set only if it does not already exist.
“CUT—Cut and Save Lines” on page 218 Saves lines to a clipboard for later retrieval by
PASTE command.
“DEFINE—Define a Name” on page 220 • Assigns an alias to a macro or built-in command.
• Disables the use of a macro or built-in command.
• Identifies a macro that replaces a built-in
command of the same name.
• Identifies programs that are edit macros.
“DELETE—Delete Lines” on page 222 Deletes lines from the data you are editing.
“EDIT—Edit from within an Edit Session” on page
224
Edits a data set or member without leaving your
current edit session (recursive edit).
“EDITSET—Display the Editor Settings Dialog” on
page 227
Causes the Edit Settings panel to be displayed.
“END—End the Edit Session” on page 229 Ends the current edit session.
“EXCLUDE—Exclude Lines from the Display” on
page 230
Excludes lines from the panel.
“FIND—Find a Data String” on page 232 Finds a data string.
“FLIP—Reverse Exclude Status of Lines” on page
234
Reverses the exclude status of a specified range of
lines in a file or all the lines in the file.
“HEX—Display Hexadecimal Characters” on page
236
Specifies whether the hexadecimal form of the
data should be displayed.
“HIDE—Hide Excluded Lines Message” on page
239
Removes the "n Line(s) not Displayed" messages
from the display where lines have been excluded
by the EXCLUDE command.
“HILITE—Enhanced Edit Coloring” on page 240 Highlights in user-specified colors many language-
specific constructs, program logic features, the
phrase containing the cursor, and any strings that
match the previous FIND operation or those that
would be found by an RFIND or RCHANGE request.
Can also be used to set default colors for the data
area in non-program files and for any characters
typed since the previous Enter or function key
entry.
“IMACRO—Specify an Initial Macro” on page 244 Saves the name of an initial macro in the edit
profile.
“LEVEL—Specify the Modification Level Number”
on page 245
Sets the modification level number to be kept as
part of the PDF library statistics.
Edit Primary Command Summary
192  z/OS: z/OS ISPF Edit and Edit Macros

## Page 225

Table 18. Summary of the primary commands (continued)
Command Description
“LOCATE—Locate a Line” on page 247 Locates a line.
“MODEL—Copy a Model into the Current Data Set”
on page 249
Copies a model into the data you are editing or
defines the current model class.
“MOVE—Move Data” on page 252 Moves a library member or data set into the data
you are editing.
“NONUMBER—Turn Off Number Mode” on page
256
Turns off number mode.
“NOTES—Display Model Notes” on page 256 Specifies whether the MODEL command is to
display notes.
“NULLS—Control Null Spaces” on page 257 Controls null spaces.
“NUMBER—Generate Sequence Numbers” on page
258
Generates sequence numbers.
“PACK—Compress Data” on page 260 Specifies whether data is to be stored normally or
compressed.
“PASTE—Move or Copy Lines from Clipboard” on
page 261
Moves or copies lines from a clipboard into an edit
session.
“PRESERVE—Enable Saving of Trailing Blanks” on
page 262
Specifies whether trailing blanks should be saved
when data is stored.
“PROFILE—Control and Display Your Profile” on
page 262
Controls and displays your profile.
“RCHANGE—Repeat a Change” on page 265 Repeats the most recently processed CHANGE
command.
“RECOVERY—Control Edit Recovery” on page 266 Controls edit recovery.
“RENUM—Renumber Data Set Lines” on page 267 Renumbers data set lines.
“REPLACE—Replace Data” on page 269 Writes the data you are editing into a library
member even if it already exists.
“RESET—Reset the Data Display” on page 273 Resets the data display.
“RFIND—Repeat Find” on page 275 Locates the data string defined by the most
recently processed SEEK, FIND, or CHANGE
command, or excludes a line that contains the data
string from the previous EXCLUDE command.
“RMACRO—Specify a Recovery Macro” on page
276
Saves the name of a recovery macro in the edit
profile.
“SAVE—Save the Current Data” on page 276 Saves the current data without ending the edit
session.
“SETUNDO—Set the UNDO Mode” on page 278 Sets the UNDO mode.
“SORT—Sort Data” on page 280 Puts data in a specified order.
“STATS—Generate Library Statistics” on page 283 Specifies whether PDF library statistics are to be
created when this member is saved.
“SUBMIT—Submit Data for Batch Processing” on
page 283
Submits the data you are editing for batch
processing.
Edit Primary Command Summary
Chapter 10. Edit primary commands  193

## Page 226

Table 18. Summary of the primary commands (continued)
Command Description
“TABS—Define Tabs” on page 285 Defines tab positions for software, hardware, and
logical tabs.
“UNDO—Reverse Last Edit Interaction” on page
286
Removes the data modifications of a previous
interaction.
“UNNUMBER—Remove Sequence Numbers” on
page 289
Removes sequence numbers.
“VERSION—Control the Version Number” on page
290
Sets the version number to be kept as part of the
PDF library statistics.
“VIEW—View from within an Edit Session” on page
292
View a data set or member without leaving your
current edit session.
AUTOLIST—Create a Source Listing Automatically
The AUTOLIST primary command sets autolist mode, which controls the automatic printing of data to the
ISPF list data set.
Syntax
AUTOLIST
ON
OFF
ON
Generates a source listing in the ISPF list data set for eventual printing when you end an edit session
in which you changed and saved data.
OFF
No source listing is generated.
Description
Autolist mode is saved in the edit profile. To check the current setting of autolist mode:
1. On the command line, type:
PROFILE 3
2. Press Enter. The third line of the edit profile shows the autolist mode setting.
To turn on autolist mode:
1. On the command line, type:
AUTOLIST ON
2. Press Enter.
To turn off autolist mode:
1. On the command line, type:
AUTOLIST OFF
2. Press Enter.
AUTOLIST
194  z/OS: z/OS ISPF Edit and Edit Macros

## Page 227

Examples
This example shows how to use the AUTOLIST command to save a copy of a source code listing in the
ISPF list data set and to print the list data set.
1. As you edit a data set, you decide to store a listing of the source code in the ISPF list data set so that
you can print it later. Enter the PROFILE 3 command to display the first 3 lines of the edit profile. This
shows you whether autolist mode is on or off.
PROFILE 3
2. You can see from the edit profile that autolist mode is off:
 =PROF> ....PLI (VARIABLE - 72)....RECOVERY ON....NUMBER OFF....................
 =PROF> ....CAPS OFF....HEX OFF....NULLS OFF....TABS OFF........................
 =PROF> ....AUTOSAVE ON....AUTONUM OFF....AUTOLIST OFF....STATS ON..............
3. Enter the AUTOLIST ON command to turn on autolist mode:
AUTOLIST ON
The edit profile changes accordingly:
 =PROF> ....PLI (VARIABLE - 72)....RECOVERY ON....NUMBER OFF....................
 =PROF> ....CAPS OFF....HEX OFF....NULLS OFF....TABS OFF........................
 =PROF> ....AUTOSAVE ON....AUTONUM OFF....AUTOLIST ON....STATS ON...............
4. After editing the data set, save your changes by entering the END command. The changes are saved
because, as you can see in the preceding partial edit profile, autosave mode is on.
END
ISPF creates a list data set with the contents of the data set member that you were editing. The name
of the list data set is:
prefix.user-id.SPFn.LIST
Note: See z/OS ISPF User's Guide Vol I for information about list data sets.
5. Before leaving ISPF, use the jump function to go to option 0.2 and check the log/list defaults:
=0.2
The Log and List Defaults panel shows the current default settings for the handling of log and list data
sets.
6. Because you want to print the list data set, make sure that the PD option is entered in the Process
Option field under the List Data Set Default Options heading:
Process option   ===> PD
Note: Also, make sure that the appropriate JCL information is entered at the bottom of the Log and List
Defaults panel so that the print job is submitted.
7. You can now end the session, knowing that the list data set will be printed:
=X
8. When the session ends, TSO displays a message that says the print job has been submitted.
AUTONUM—Number Lines Automatically
The AUTONUM primary command sets autonum mode, which controls the automatic renumbering of data
when it is saved.
AUTONUM
Chapter 10. Edit primary commands  195

## Page 228

Syntax
AUTONUM
ON
OFF
ON
Turns on automatic renumbering. When number mode is also on, the data is automatically
renumbered when it is saved.
OFF
Turns off automatic renumbering. Data is not renumbered.
Description
When number mode is on (see (xref refid="number"), the first line of a data set or member is normally line
number 000100, the second number is 000200, and so forth. However, as lines are inserted and deleted,
the increment between line numbers can change.
For example, you might think that when a line is inserted between 000100 and 000200, line 000200
would be given the number 000300 and the new line would become 000200. Instead, the existing lines
retain their numbers and the new line is given line number 000110.
Therefore, if the original line number increments are important to you, the AUTONUM command
renumbers your lines automatically so that the original increments are maintained.
Autonum mode is saved in the edit profile. To check the current settings of number mode and autonum
mode:
1. On the command line, type:
PROFILE 3
2. Press Enter. The first line of the edit profile shows the number mode setting and the third line shows
the autonum mode setting.
To turn on autonum mode:
1. On the command line, type:
AUTONUM ON
2. Press Enter.
To turn off autonum mode:
1. On the command line, type:
AUTONUM OFF
2. Press Enter.
Examples
This example shows a practical application of AUTONUM command usage. You have been editing a data
set with number mode on.
Note: If you are editing a data set or member with number mode off and then decide to turn number
mode on, make sure that columns 1 through 6 of your data set are blank. Otherwise, the sequence
numbers created by the NUMBER command can overlay any of your data in columns 1 through 6. Use
either the COLUMN SHIFT or DATA SHIFT line command to indent the data.
You now want to end the edit session. However, since you had to insert and delete many lines, your line
numbering is no longer uniform. Therefore, you decide to use autonum mode so that the next time you
edit this data set the line numbers will be correct.
AUTONUM
196  z/OS: z/OS ISPF Edit and Edit Macros

## Page 229

1. First, check the edit profile to see whether autonum mode is already on by entering the PROFILE 3
command to display the first 3 lines of the edit profile.
PROFILE 3
2. You can see from the edit profile that autonum mode is off:
 =PROF> ....PLI (VARIABLE - 72)....RECOVERY ON....NUMBER OFF....................
 =PROF> ....CAPS OFF....HEX OFF....NULLS OFF....TABS OFF........................
 =PROF> ....AUTOSAVE ON....AUTONUM OFF....AUTOLIST OFF....STATS ON..............
3. Enter the AUTONUM ON command to turn on autonum mode:
AUTONUM ON
The edit profile changes accordingly:
 =PROF> ....PLI (VARIABLE - 72)....RECOVERY ON....NUMBER OFF....................
 =PROF> ....CAPS OFF....HEX OFF....NULLS OFF....TABS OFF........................
 =PROF> ....AUTOSAVE ON....AUTONUM ON....AUTOLIST ON....STATS ON................
4. After editing the data set, save your changes by entering the END command. The changes will be saved
because, as you can see in the preceding partial edit profile, autosave mode is on.
END
ISPF saves the data set that you were editing, along with any changes. The next time you edit the data
set, the line numbers will have the proper increments.
AUTOSAVE—Save Data Automatically
The AUTOSAVE primary command sets autosave mode, which controls whether changed data is saved
when you enter END.
Syntax
AUTOSAVE
ON
PROMPT
OFF
PROMPT
NOPROMPT
ON
Turns autosave mode on. When you enter END, any changed data is saved.
OFF PROMPT
Turns autosave mode off with the PROMPT operand. You are notified that changes have been made
and that either the SAVE command (followed by END) or CANCEL must be used. When you use
AUTOSAVE PROMPT by itself, it implies the OFF command.
OFF NOPROMPT
Turns autosave mode off with the NOPROMPT operand. You are not notified and the data is not saved
when you issue an END command. END becomes an equivalent to CANCEL. Use the NOPROMPT
operand with caution.
AUTOSAVE
Chapter 10. Edit primary commands  197

## Page 230

Description
Data is considered changed if you have operated on it in any way that could cause a change. Shifting a
blank line or changing a word to the same word does not actually alter the data, but the editor considers
this data changed. When you enter SAVE, the editor resets the change status.
Autosave mode, along with the PROMPT operand, is saved in the edit profile. To check the current setting
of autosave mode:
1. On the command line, type:
PROFILE 3
2. Press Enter. The third line of the edit profile shows the autosave mode setting.
To turn on autosave mode:
1. On the command line, type:
AUTOSAVE
Note: This is the equivalent of entering AUTOSAVE ON.
2. Press Enter. The next time you enter END, any changes that you made to the data set or member that
you were editing are saved.
To turn off autosave mode:
1. On the command line, type:
AUTOSAVE OFF
Note: This is the equivalent of entering AUTOSAVE OFF PROMPT.
2. Press Enter. The next time you enter END when a data set or member has been changed, the editor
prompts you to specify whether you want changes to the data set or member saved (SAVE) or not
saved (CANCEL). However, if no changes have been made to the data set or member, the edit session
ends without a prompt.
To turn off autosave mode and specify that you do not want to be prompted when data has changed:
1. On the command line, type:
AUTOSAVE OFF NOPROMPT
2. Press Enter. The next time you enter END when a data set or member has been changed, the edit
session ends without saving your changes, just as if you had entered CANCEL. You are not prompted to
save the changes.
For more information on saving data, see the CANCEL and END primary commands, and the
DATA_CHANGED, CANCEL, and END macro commands.
Examples
This example shows a practical application of AUTOSAVE usage.
1. You have been editing a data set member and now want to end the edit session. Enter END:
END
2. The member that you were editing remains with this message in the upper-right corner:
DATA CHANGED-SAVE/CANCEL
This message implies that autosave mode in the edit profile is set to AUTOSAVE OFF PROMPT. You are
prompted to enter either SAVE to save your changes, or CANCEL to end the edit session without saving
your changes.
AUTOSAVE
198  z/OS: z/OS ISPF Edit and Edit Macros

## Page 231

You also have the option to change autosave mode in the edit profile to AUTOSAVE ON. By doing so,
the next time you enter END, your changes will be saved and the edit session will end.
3. You decide to turn on autosave mode:
AUTOSAVE ON
4. Then you enter END again to save your changes and end the edit session.
END
BOUNDS—Control the Edit Boundaries
The BOUNDS primary command sets the left and right boundaries and saves them in the edit profile.
Syntax
BOUNDS
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
*
The current value of the boundary.
To reset the boundaries to the default columns:
1. On the command line, type:
BOUNDS
2. Press Enter. The boundaries are reset to the default columns.
See “Edit boundaries” on page 23 for more information, including tables that show commands affected by
bounds settings and default bounds settings for various types of data sets.
The column numbers are always data column numbers (see “Referring to column positions” on page
106). Thus, for a variable format data set with number mode on, data column 1 is column 9 in the record.
You cannot specify the same column for both boundaries.
Description
The BOUNDS primary command provides an alternative to setting the boundaries with the BOUNDS line
command or macro command; the effect on the member or data set is the same. However, if you use
both the BOUNDS primary command and the BOUNDS line command in the same interaction, the line
command overrides the primary command.
Examples
To set the left boundary to 1 and the right boundary to 72, type:
BOUNDS 1 72
BOUNDS
Chapter 10. Edit primary commands  199

## Page 232

To set the left boundary to 10 and leave the right as is, type:
BOUNDS 10 *
BROWSE—Browse from within an Edit Session
The BROWSE primary command allows you to browse a sequential data set, partitioned data set member,
or z/OS UNIX file during your current edit session.
Syntax
BROWSE
member
GEN generation
member
A member of the ISPF library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
generation
The generation of the member to be browsed. You may enter an absolute (positive) generation
number or a relative (negative) generation number. This parameter is valid only when the member is in
a PDSE version 2 data set that is configured for member generations.
Description
To browse a data set, member, or z/OS UNIX file during your current edit session:
1. On the command line, type:
BROWSE
or
BROWSE member
or
BROWSE member GEN generation
Here, member represents the name of a member of the partitioned data set you are editing, and
generation represents a generation of the member. The member and generation operands are
optional.
2. Press Enter.
If you specify a member name, the current library concatenation sequence finds the member. The
member displays for browsing. If you specify a generation number, the specified generation of the
member displays for browsing.
If you do not specify a member name, the Browse Command Entry panel, which is similar to the
regular Browse Entry panel, appears. You can enter the name of any sequential data set, partitioned
data set, or z/OS UNIX file to which you have access. When you press Enter, the data set, member, or
z/OS UNIX file displays for browsing.
The editor suspends your initial edit session until the browse session is complete.
3. To exit from the browse session, enter the END command. The current session resumes.
BROWSE
200  z/OS: z/OS ISPF Edit and Edit Macros

## Page 233

Examples
To browse member YYY of the current library concatenation:
1. On the command line, type:
BROWSE YYY
2. Press Enter.
BUILTIN—Process a Built-In Command
You can use the BUILTIN primary command with edit macros and the DEFINE command to process a
built-in edit primary command, even if a macro has been defined with the same name.
Syntax
BUILTIN cmdname
cmdname
The built-in command to be processed.
Description
To process a built-in primary command instead of a command with the same name that has been defined
as an alias:
1. On the command line, type:
BUILTIN cmdname
where cmdname is the name of a primary command.
2. Press Enter. The edit primary command is processed.
Examples
This example shows a practical application of BUILTIN command usage.
1. You have a macro named MACEND that you have created. You want to run your MACEND macro
instead of ISPF's built-in END command. Enter this command:
DEFINE END ALIAS MACEND
Note: If the END command is issued in your MACEND macro without being preceded by the BUILTIN
macro command, the MACEND macro would be run again, resulting in a loop.
2. To run your MACEND macro, enter:
END
3. To end the edit session without redefining END, use BUILTIN, as follows:
BUILTIN END
This command issues ISPF's built-in END command instead of your MACEND macro.
CANCEL—Cancel Edit Changes
BUILTIN
Chapter 10. Edit primary commands  201

## Page 234

The CANCEL primary command ends your edit session without saving any of the changes you have made.
Syntax
CANCEL
CAN
Description
CANCEL is especially useful if you have changed the wrong data, or if the changes themselves are
incorrect. To cancel changes to a data set:
1. On the command line, type:
CANCEL
2. Press Enter. The edit session ends without saving your changes.
Note: If you issue SAVE and later issue CANCEL, the changes you made before issuing SAVE are not
canceled.
See the DATA_CHANGED, AUTOSAVE, and END commands for more information about saving data.
CANCEL does not cause automatic recording in the ISPF list data set, regardless of the setting of the
autolist mode.
Examples
After editing the data, you decide that you want the data set the way it was before editing. Enter this
command:
CANCEL
The edit session ends with the data set in its original state.
CAPS—Control Automatic Character Conversion
The CAPS primary command sets the caps mode, which controls whether alphabetic data that you type at
the terminal is automatically converted to uppercase during the edit session.
Syntax
CAPS
ON
OFF
ON
Turns caps mode on.
OFF
Turns caps mode off.
Description
The editor sets the caps mode according to the data in the file retrieved for editing. If caps mode has
been on and the data contains lowercase letters, the mode switches and the editor displays a message
CAPS
202  z/OS: z/OS ISPF Edit and Edit Macros

## Page 235

indicating the change. Likewise, if caps mode is off and the editor contains all uppercase letters, the mode
switches and the editor displays a message.
Caps mode is saved in the edit profile. To override the automatic setting of caps mode, you can include the
CAPS command in an initial macro.
Caps mode is usually on during program development work. When caps mode is on, any alphabetic data
that you type, plus any other alphabetic data that already exists on that line, is converted to uppercase
when you press Enter or a function key.
To set caps mode on:
1. On the command line, type:
CAPS
2. Press Enter. Caps mode is set to on in the edit profile.
Caps mode is usually off when you edit text documentation. When caps mode is set to off, any alphabetic
data that you type remains just as you typed it. If you typed it in uppercase, it stays in uppercase; if you
typed it in lowercase, it stays in lowercase. Alphabetic data already typed on a line is not affected. To set
caps mode off:
1. On the command line, type:
CAPS OFF
2. Press Enter. Caps mode is set to off in the edit profile.
The CAPS command does not apply to DBCS fields in formatted data or to DBCS fields in mixed fields. If
you specify CAPS, the DBCS fields remain unchanged.
See the LC (lowercase) and UC (uppercase) line commands and the CAPS macro command for more
information about changing case.
Examples
This example shows a practical application of CAPS command usage.
1. You are editing a data set that contains all uppercase letters, with caps mode off. The data you are
typing contains both uppercase and lowercase letters, but you want all of the letters to be uppercase.
On the command line, type:
CAPS
2. Press Enter.
3. Move the cursor back to the line on which you were typing.
4. Finish typing the line or type over one or more of the existing letters.
5. Press Enter. All of the letters on the line are converted to uppercase.
CHANGE—Change a Data String
The CHANGE primary command changes one string into another.
CHANGE
Chapter 10. Edit primary commands  203

## Page 236

Syntax
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
string1
The search string you want to change. See “Finding, seeking, changing, and excluding data” on page
44.
string2
The string you want to replace string1. See “Finding, seeking, changing, and excluding data” on page
44.
labela, labelb
Labels identifying the start and end of the group of lines the CHANGE command is to search.
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
WORD
Locates string1 when it is delimited on both sides by blanks or other non-alphanumeric characters.
X
Scans only lines that are excluded from the display.
NX
Scans only lines that are not excluded from the display.
CHANGE
204  z/OS: z/OS ISPF Edit and Edit Macros

## Page 237

start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
The first column to be included in the range of columns to be searched.
right_col
The last column to be included in the range of columns to be searched.
Note:
1. For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
2. The CHANGE command allows you to control the starting point and the direction of the search by
positioning the cursor and using either the NEXT or PREV operand. For more information, see “Starting
point and direction of the search” on page 53.
Description
You can use the CHANGE command with the FIND and EXCLUDE commands to find a search string,
change it, and then exclude the line that contains the string from the panel.
To change the next occurrence of "ME" to "YOU" without specifying any other qualifications:
1. On the command line, type:
CHANGE ME YOU
2. Press Enter. This command changes only the next occurrence of the letters "ME" to "YOU". Since no
other qualifications were specified, the letters "ME" can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• In an excluded line or a non-excluded line
• Anywhere within the current boundaries
To change the next occurrence of "ME" to "YOU", but only if the letters are uppercase:
1. On the command line, type:
CHANGE C'ME' YOU
2. Press Enter. This type of change is called a character string change (note the C that precedes the
search string) because it changes the next occurrence of the letters ME to YOU only if the letters
are found in uppercase. However, since no other qualifications were specified, the change occurs no
matter where the letters are found, as outlined in the preceding list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Examples
The example shown changes the first plus ("+") in the data set to a minus ("-"). However, the plus must
occur on or between lines labeled .E and .S and it must be the first character of a word:
CHANGE '+' '-' .E .S FIRST PREFIX
CHANGE
Chapter 10. Edit primary commands  205

## Page 238

The example shown changes the last plus in the data set to a minus. However, the plus must occur on
or between lines labeled .E and .S; it must be the last character of a word; and it must be found on an
excluded line:
CHANGE '+' '-' .E .S LAST SUFFIX X
The example shown changes the plus that immediately precedes the cursor position to a minus. However,
the cursor must not be positioned ahead of the lines labeled .E and .S. Also, the plus must occur on or
between the labeled lines; it must be a standalone character (not part of any other word); it must be on a
non-excluded line; and it must exist within columns 1 and 5:
CHANGE '+' '-' .E .S PREV WORD NX 1 5
COLS—Display Fixed Columns Line
The COLS primary command displays a non-scrolling columns indicator line at the top of the data area.
Syntax
COLS
ON
OFF
ON
Display columns line.
OFF
Remove columns line from the display.
Description
The COLS command displays a columns indicator line at the top of the data area in Edit and View mode.
This works in the same manner as the columns line under Browse.
The columns line differs from that displayed by the COLS line command in that the line command field is
protected. This means that it cannot be copied, moved, or deleted by overtyping with line commands. The
line does not scroll with the data, and therefore the number of data lines displayed is reduced by one.
Entering COLS with no parameter toggles the display to the opposite. For example, if the columns line is
currently displayed, entering COLS removes it.
Examples
To display the columns indicator line, enter this command:
COLS ON
Figure 109 on page 207 shows an example of an edit screen displaying the columns indicator line.
COLS
206  z/OS: z/OS ISPF Edit and Edit Macros

## Page 239

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       LEEBURR.TEST($$ZZZZ) - 01.10                    Columns 00001 00072
 =COLS> ----+----1----+----2----+----3----+----4----+----5----+----6----+----7--
 ****** ***************************** Top of Data ******************************
 000001 //LEEBURRC JOB CLASS=A,MSGCLASS=X
 000002 //STEPPLX  EXEC PGM=AKEEPLX,REGION=2048K,PARM='SOURCE(SEG)'
 000003 //SYSPRINT DD  SYSOUT=A
 000004 //SYSUT1   DD  UNIT=SYSDA,SPACE=(TRK,(30,10))
 000005 //SYSUT2   DD  UNIT=SYSDA,DSN=&&ASM,DISP=(NEW,PASS),
 000006 //             SPACE=(TRK,(30,10))
 000007 //SYSUT3   DD  UNIT=SYSDA,SPACE=(TRK,(30,10))
 000008 //SYSUT4   DD  UNIT=SYSDA,SPACE=(TRK,(30,10))
 000009 //SYSLIB   DD  DISP=SHR,DSN=PDFTOS2C.LEEBURR.SOURCE
 000010 //         DD  DISP=SHR,DSN=PDFTOS2C.APARTEST.SOURCE
 ⋮
 Command ===> ________________________________________________ Scroll ===> CSR 
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 109. Member with COLS indicator line
COMPARE—Edit Compare
The COMPARE command compares the file you are editing with an external sequential data set, member
of a partitioned data set, or z/OS UNIX file. Lines that exist only in the file being edited are marked, and
lines that exist only in the file being compared are inserted as information lines in the file being edited.
The command operates as a primary command or an edit macro command.
If you compare the file you are editing with a member of a PDSE version 2 data set that is configured for
member generations, the current generation of the member is used for the comparison.
You can use the Delete and Make Data line commands to merge changes between files that are being
compared.
The COMPARE function supports all line lengths, but some SuperC options are ignored for line lengths
greater than 256 characters long.
When you are editing a cataloged data set, explicit data set names refer to cataloged data sets. However,
if you are editing an uncataloged data set and specify only a member name, COMPARE searches for the
member in the current uncataloged data set. For example, if you are editing an uncataloged data set
called "userid.TEMP", then the command
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
COMPARE
Chapter 10. Edit primary commands  207

## Page 240

Syntax
COMPARE
dsname
VOL( volser)
NEXT
SESSION
 * 
 / 
EXCLUDE SAVE
SYSIN
( supercdsname )
(/)
no operand
The "Edit Compare Settings and/or Command Parameters" panel is displayed.
This panel enables you to customize the comparison by selecting the relevant SuperC options to
use. The comparison is always a LINE compare with the options UPDLDEL, NOLISTL, LINECMP, and
CKPACKL specified.
You can also specify Compare Command Parameters. The Name field is used to specify the dsname,
NEXT, or * (session) parameters. The Volume field is used to enter the volume serial for an
uncataloged data set. The Exclude field is used to specify the EXCLUDE parameter. The SYSIN field is
used to specify the SYSIN parameter. The Save field is used to specify the SAVE parameter. The Set
SYSIN data set field is used to display a panel where the SYSIN data set name can be specified. See
below for a description of these parameters.
The SEQ, NOSEQ, or COBOL keywords are automatically specified depending on the NUMBER state
in the edit profile. Mixed data can be enabled, and is always assumed to be specified when you are
in an edit session with MIXED specified in the profile. Each field in the Edit Compare Settings and/or
Command Parameters panel has field level help.
Note: When don't process (DP) options are used, the resulting display shows DP lines in the current
file as unlabeled and does not show DP lines from the comparison file. This can be misleading.
Because comparisons which ignore parts of the file might show data in one file and not in the
other, use caution when using DP options. When you use options that ignore programming language
comments, the don't process reformatted lines option is recommended.
dsname
The name of a member, data set, or z/OS UNIX file to which the current file is compared. This variable
can be specified as a fully qualified data set name (in quotation marks), a partially qualified data set
name, a member name, or path name. (Also, see “Specifying z/OS UNIX pathnames with edit primary
and macro commands” on page 15.)
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
COMPARE
208  z/OS: z/OS ISPF Edit and Edit Macros

## Page 241

member. For example, if the current member is found in the third level of the concatenation, and a
like-named member exists at the fourth level, then the third and fourth level members are compared.
After data is saved in the lowest level, compares are done from that level upward. If you specify
dsname, the NEXT keyword cannot be used.
SESSION
Specifies that you want to compare the changes you have made during the edit session with the copy
of the data saved on disk. Use COMPARE SESSION (or COMPARE *) to see the changes you have made
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
Parameters panel. Valid numbers are 0 through 12, inclusive.
You can also use the COMPARE EXCLUDE command at any time to exclude all lines in a file except
lines with line labels and information lines, and the lines above and below those lines. When you
specify EXCLUDE without a data set name or NEXT, no comparison is done. Instead the labels and
information lines that already exist in the file are used to exclude functions.
/
Can be used when you need to enter a long path name for the z/OS UNIX file to be compared against.
This causes the display of a popup window containing a scrollable field for the input of a path name.
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
SYSIN
Specifies not to free the ddname SYSIN before calling SuperC to compare files. This enables you to
pass SuperC Process Statements to alter the comparison. No validation is done on the type of SYSIN
allocation or the contents of the data set.
supercdsname
The name of a data set containing SuperC process statements.
COMPARE
Chapter 10. Edit primary commands  209

## Page 242

/
Displays the Edit Compare SYSIN specification panel where you can specify the name of a data set
containing SuperC Process statements that are used for the compare. The SYSIN data set is freed
at the end of the compare.
Examples
To display the Edit Compare Settings and/or Command Parameters panel:
1. On the command line, type:
COMPARE
2. Press Enter.
Figure 110. Edit Compare Settings and/or Command Parameters panel
To compare the data to a member in the current data set or concatenation:
1. On the command line, type:
COMPARE (member
2. Press Enter.
COPY—Copy Data
The COPY primary command copies a sequential data set, a member of a partitioned data set, or z/OS
UNIX file into the data being edited.
COPY
210  z/OS: z/OS ISPF Edit and Edit Macros

## Page 243

If no options are specified with the COPY command, the Edit/View Copy panel is displayed.
Syntax
COPY
copy_options
copy_options
member
( member)
dsname
dsname( member)
pathname
AFTER
BEFORE
label 1 start_line end_line
ASCII
EBCDIC
UTF8
Notes:
1 If you don't specify the position using a label, you must specify the position by using an A or B line
command.
member
A member of the ISPF library or partitioned data set that you are editing. If a name of eight or fewer
characters is specified and it could be a member name or a data set name, COPY searches for a
member name first. If no member is found, then the name is used as a data set name.
dsname
A partially qualified or fully qualified data set name. If the data set is partitioned you can include a
member name in parentheses or select a member from a member list.
pathname
The path name for a z/OS UNIX regular file or directory. If a directory is specified, a directory
selection list is displayed, allowing you to select the file to be copied. (Also, see “Specifying z/OS
UNIX pathnames with edit primary and macro commands” on page 15.)
AFTER
The data is copied after the line with the specified label.
BEFORE
The data is copied before the line with the specified label.
label
Label identifying the line where the data is to be copied. It can be either a label that you define or one
of the editor-defined labels, such as .ZF or .ZL.
start_line
The number of the first line of the member, data set, or z/OS UNIX file to be included in the range of
lines to be copied. Must be greater than or equal to 1, and less than or equal to the number of lines in
the member, data set, or z/OS UNIX file. To specify standard, ISPF, or COBOL line numbers, omit the
member name, data set name, or z/OS UNIX file name to use the Extended Edit Copy panel.
end_line
The number of the last line to be included in the range of lines to be copied. Must be greater than or
equal to start_line and less than or equal to the number of lines in the member, data set, or z/OS UNIX
file.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being copied in from the external file is converted from the
COPY
Chapter 10. Edit primary commands  211

## Page 244

character set designated by the keyword to the character set specified for the file being edited or to
the terminal character set.
The label can be either a label that you define or one of the PDF editor-defined labels, such as .ZF
and .ZL.
If you have not defined a label and the ISPF editor-defined labels are not appropriate for your purpose,
use the A (after) or B (before) line command to specify where the data is to be copied.
If the data set or member that you are editing is empty, you do not need to specify a destination for the
data being copied.
Note: If the member name or data set name is less than 8 characters and the data set you are editing
is partitioned, a like-named member is copied. If a like-named member does not exist, the name is
considered to be a partially qualified data set name.
Description
COPY adds a copy of data that already exists to the data set, member, or z/OS UNIX file that you are
editing. Use MOVE if you want to move data from one data set, member, or z/OS UNIX file to another,
rather than just copy it.
To copy data into an empty data set, member, or z/OS UNIX file:
1. On the command line, type:
COPY member
or:
COPY dsname
or:
COPY pathname
The member, data set name, or path name operand is optional. If you do not specify the name of a
member, data set, or z/OS UNIX file to be copied, the Edit Copy panel appears. Enter the name of the
data set, member, or z/OS UNIX file on this panel.
You can specify the numbers of the first and last lines to be copied, along with the kind of line numbers
(standard, ISPFSTD, COBOL, or relative) on the Edit Copy panel. This allows you to copy only part of
the data set or member.
Note: When you select ISPFSTD line numbers and the STATS mode is ON, the editor uses the first 6
digits and ignores the 2-digit modification number. When the STATS mode is OFF, the editor uses all 8
digits.
2. Press Enter. The data is copied.
To copy data into a data set, member, or z/OS UNIX file that is not empty:
1. On the command line, type:
COPY member AFTER | BEFORE label  start_line end_line
or:
COPY dsname AFTER | BEFORE label  start_line end_line
or:
COPY pathname AFTER | BEFORE label  start_line end_line
COPY
212  z/OS: z/OS ISPF Edit and Edit Macros

## Page 245

The member, dsname, or pathname operand is optional. You should omit the member name only if you
do not know the member name, or if you are going to copy a sequential data set, z/OS UNIX file, or a
member of a different partitioned data set.
The AFTER label and BEFORE label operands are also optional. However, if the data set, member, or
z/OS UNIX file that is to receive the copied data is not empty, you must specify a destination for the
copied data. Therefore, if you do not want to use a label, you can substitute either the A (after) or B
(before) line command as the destination of the copied data. However, a number indicating that the
A or B command should be repeated cannot follow the line command. See the descriptions of these
commands for information about them.
If the data set, member, or z/OS UNIX file is not empty and you do not specify a destination, a "MOVE/
COPY Pending" message appears in the upper-right corner of the panel and the data is not copied.
When you type a destination and press Enter, the data is copied.
2. Press Enter. If you entered the name of a member, data set, or z/OS UNIX file, the member, data
set, or z/OS UNIX file is copied. Otherwise, the edit copy panel appears. If a range of line numbers is
specified, only those lines are copied. See the previous example for more information.
See “Copying and moving data” on page 41 if you need more information.
Examples
These steps show how you can copy data when you omit the member name and the ISPF editor panels
appear:
1. Type COPY on the command line and specify the destination of the operation. The panel in Figure 111
on page 213 shows you that the data is to be copied after line 000700, as specified by the A (after) line
command. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       P020136.PRIVATE.PLS(INTO) - 01.00               Columns 00001 00072
 Command ===> copy                                             Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000100
 000200 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 000300
 000400 THIS IS THE MEMBER INTO WHICH THE LINES ARE TO BE COPIED.
 000500
 000600      +---------------------+
 a 0700      |                     |
 000800      |                     |
 000900      |                     |
 001000      |                     |
 001100      +---------------------+
 001200
 001300 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 001400
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 111. Member before data is copied
2. When you press Enter, the Edit Copy panel appears. Specify the data you want copied.
The example in Figure 112 on page 214 copies the data set member named COPYFROM. Since you are
using the Edit/View - Copy panel, you can also specify the first and last lines you want copied.
COPY
Chapter 10. Edit primary commands  213

## Page 246

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Edit/View - Copy
 Command ===> _________________________________________________________________
                                                                    More:   -
    Project . . . PROJ1   
    Group . . . . USERID   . . . ________ . . . ________ . . . ________
    Type  . . . . CLIST   
    Member  . . .                (Blank or pattern for member selection list)
 From Other Partitioned or Sequential Data Set, or z/OS UNIX file:
    Data Set Name  . . _________________________________________________________
    Volume Serial  . . ______    (If not cataloged)
 Data Set Password  . .          (If password protected)
 Line Numbers (Blank for entire member or seq. data set)
    First line  . . . . ________
    Last line . . . . . ________
    Number type . . . . ________ (Standard, ISPFstd, COBOL, or Relative)
 Data Conversion option                                                        
     1. EBCDIC                                                                  
    2. ASCII                                                                   
    3. UTF-8                                                                   
    
    Press Enter key to copy, enter End command to cancel copy.
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
  F10=Actions   F12=Cancel
Figure 112. Edit/View - Copy panel (ISRECPY1)
3. Figure 113 on page 214 shows the contents of the COPYFROM member, which is copied into the
original data set. 
EDIT       P020136.PRIVATE.PLS(COPYFROM) - 01.00           Columns 00001 00072
****** ***************************** Top of Data ******************************
000100 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
000200 These are the lines that are to be copied.
000300  These are the lines that are to be copied.
000400 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
****** **************************** Bottom of Data ****************************
⋮
Figure 113. Contents of member to be copied
4. When you press Enter, the editor copies the data and displays a short message in the upper right side
of the panel. Figure 114 on page 214 shows the result of the copy operation. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       P020136.PRIVATE.PLS(INTO) - 01.00            Member COPYFROM copied
 Command ===>                                                  Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000100
 000200 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 000300
 000400 THIS IS THE MEMBER INTO WHICH THE LINES ARE TO BE COPIED.
 000500
 000600      +---------------------+
 000700      |                     |
 000710 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 000720 These are the lines that are to be copied.
 000730  These are the lines that are to be copied.
 000740 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 000800      |                     |
 000900      |                     |
 001000      |                     |
 001100      +---------------------+
 001200
 001300 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 114. Member after data has been copied
COPY
214  z/OS: z/OS ISPF Edit and Edit Macros

## Page 247

CREATE—Create Data
The CREATE primary command creates a member of a partitioned data set, a sequential data set, or z/OS
UNIX file from the data you are editing.
If no options are specified with the CREATE command, the Edit/View - Create panel is displayed.
Note: If you are editing a z/OS UNIX file and you issue the CREATE command to create a new z/OS UNIX
file, the file permissions for the new file are set to the same values as the file permissions of the file
you are currently editing. If you are editing a sequential data set or member and you issue the CREATE
command to create a new z/OS UNIX file, the file permissions are set to 700 (rwx------).
Syntax
CREATE
CRE create_options
create_options
member
( member)
dsname( member)
dsname
pathname
labela labelb 1 ASCII
EBCDIC
UTF8
Notes:
1 If you don't specify the group of lines using labels, you must specify the group by using C or M line
commands.
member
The name of the new member added to the partitioned data set currently being edited. If you are
using a concatenated sequence of libraries, the member is always written to the first library in the
sequence.
labela, labelb
Labels identifying the start and end of the group of lines which are added to the new member.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
dsname(member)
The name of a different partitioned data set and new member name to be added to the partitioned
data set. The data set name can be fully qualified or partially qualified.
dsname
The name of a different sequential data set to be added. The data set name can be fully qualified or
partially qualified.
pathname
The path name for a z/OS UNIX regular file to be created. (Also, see “Specifying z/OS UNIX pathnames
with edit primary and macro commands” on page 15.)
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being saved in the external file is converted to the character
set designated by the keyword.
Description
CREATE
Chapter 10. Edit primary commands  215

## Page 248

Note: CREATE adds a new member to a partitioned data set only if a member of the same name does not
already exist. Use REPLACE if the member already exists.
To create a member of a partitioned data set, a sequential data set, or a z/OS UNIX file:
1. On the command line, type one of these commands:
CREATE member labela labelb
CREATE (member) labela labelb
CREATE dsname(member) labela labelb
CREATE dsname labela labelb
CREATE pathname labela labelb
The member operand is optional unless you specify a data set name. It represents the name of the
member you want to create.
The labela and labelb operands specify the first and last lines in a group of lines used to create the new
member, sequential data set, or z/OS UNIX file.
If you omit the labela and labelb operands, you must specify the lines by using either the C (copy) or
M (move) line command. See the descriptions of these commands if you need more information about
them.
If you omit the labela and labelb operands and do not enter one of the preceding line commands, a
"CREATE Pending" message is displayed in the upper-right corner of the panel.
2. Press Enter. If you did not specify the name of the member, the name of another partitioned data set
along with the member name, or the name of a z/OS UNIX file to be created, the Edit Create panel
appears (see Figure 116 on page 217). Enter the member name on this panel and press Enter again. If
you used either a pair of labels or a C line command, the data is copied from the member that you are
editing into the member that you are creating. If you used the M line command, however, the data is
removed from the member that you are editing and placed in the member that you are creating.
If the data set specified does not exist, ISPF prompts you to see if the data set should be created. You
can create the data set using the characteristics of the cataloged source data set as a model, or specify
the characteristics for the new data set. You can suppress this function through the ISPF configuration
table, causing any CREATE request for a nonexistent data set to fail.
See “Creating and replacing data” on page 41 if you need more information about the CREATE command.
Examples
These steps show how you can create a new member when you omit the member name:
1. Type CREATE on the command line and specify which lines you want to copy or move into the new data
set or member. The example in Figure 115 on page 217 uses the MM (block move) line command to
move a block of lines from the data. 
CREATE
216  z/OS: z/OS ISPF Edit and Edit Macros

## Page 249

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(FROMDATA) - 01.00                  Columns 00001 00072
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 This line will be left in this member
 000200 This line will be left in this member
 000300 +----------------+
 000400 | This is the    |
 000500 | material to    |
 000600 | be created in  |
 000700 | another member |
 000800 +----------------+
 000900 This line will be left in this member
 001000 This line will be left in this member
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 115. Member before new member is created
2. When you press Enter, the Edit/View Create panel (Figure 116 on page 217) appears. Type the name
of a new member and press Enter. If you type the name of a member that already exists, an error
message appears and the CREATE fails. The name of the member created for this example is TODATA. 
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Edit/View - Create
 Command ===> _________________________________________________________________
                                                                    More:     +
 "Current" Data Set: PDFTDEV.USERSID.MSGGEN(FLMU00)
 To ISPF Library:
    Project . . . PROJ1   
    Group . . . . USERID  
    Type  . . . . CLIST   
    Member  . . .         
 To Other Sequential Data Set, Partitioned Data Set Member, or z/OS UNIX file:
    Name . . . . . TEST(TODATA)                                             +
    Volume Serial            (If not cataloged)
 Data Set Password  . .           (If password protected)
 Enter "/" to select option                      Data Conversion option
    Specify pack option for "CREATE" Data Set       1. EBCDIC                   
                                                    2. ASCII                    
                                                    3. UTF-8
 Press ENTER key to create. Enter END command to cancel create.
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
  F10=Actions   F12=Cancel
Figure 116. Edit/View Create panel (ISRECRA1)
3. Figure 117 on page 218 shows the lines remaining in the original member after the specified lines
were moved to the new member. 
CREATE
Chapter 10. Edit primary commands  217

## Page 250

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(FROMDATA) - 01.01                Member TODATA created
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 This line will be left in this member
 000200 This line will be left in this member
 000900 This line will be left in this member
 001000 This line will be left in this member
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 117. Member after new member has been created
4. Figure 118 on page 218 shows the contents of the new member. The data is renumbered only if both
number mode and autonum mode are on. A source listing of the data is also recorded in the ISPF
list data set for eventual printing if autolist mode is on. In this example, the lines have retained their
original line numbers. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(TODATA) - 01.00                    Columns 00001 00072
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000300 +----------------+
 000400 | This is the    |
 000500 | material to    |
 000600 | be created in  |
 000700 | another member |
 000800 +----------------+
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 118. New member created
CUT—Cut and Save Lines
The CUT primary command saves lines to one of 11 named clipboards for later retrieval by the PASTE
command. The lines can be appended to lines already saved by a previous CUT command or can replace
existing lines in a clipboard.
CUT
218  z/OS: z/OS ISPF Edit and Edit Macros

## Page 251

Syntax
CUT cut_options
DISPLAY
cut_options
.ZFIRST .ZLAST
labela labelb 1
linenum1 linenum2
DEFAULT
clipboard_name X
NX
APPEND
REPLACE
ASCII
EBCDIC
UTF8
Notes:
1 You can also specify the group of lines using C or M line commands.
labela, labelb
Labels identifying the start and end of the group of lines the CUT command is to copy or move to the
clipboard.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
linenum1, linenum2
Relative line numbers identifying the start and end of the group of lines the CUT command is to copy
or move to the clipboard.
clipboard_name
The name of the clipboard to use. If you omit this parameter, the ISPF default clipboard (named
DEFAULT) is used. You can define up to ten additional clipboards. The size of the clipboards and
number of clipboards might be limited by installation defaults.
X
Cut only lines that are excluded from the display.
NX
Cut only lines that are not excluded from the display.
REPLACE
Replace existing data in the clipboard.
You can select REPLACE as the default by entering the EDITSET command on the editor command
line. The default action depends on the setting specified in the panel displayed by the EDITSET. You
should always specify REPLACE (or APPEND) in a macro because the user can change the default
behavior.
APPEND
Add the data to the clipboard. You can select APPEND as the default by entering the EDITSET
command on the editor command line. The default action depends on the setting specified in the
panel displayed by the EDITSET. You should always specify APPEND (or REPLACE) in a macro because
the user can change the default behavior.
DISPLAY
Show a list of existing clipboards. From this list you can browse, edit, clear, or rename the clipboards.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being placed in the clipboard is converted to the character
set designated by the keyword and tagged as being in the designated character set.
CUT
Chapter 10. Edit primary commands  219

## Page 252

Description
CUT saves copies of lines from an edit session to a clipboard for later retrieval by the PASTE command.
The lines are moved or copied from the session to the named clipboard. Lines are specified by either the
C (Copy) or M (Move) line commands, CC or MM block line commands, or label names. If the C or CC line
commands or labels are used to identify the lines, the lines are copied to the clipboard. If the M or MM line
commands are used to identify the lines, the lines are copied to the clipboard and deleted from the edit
session (in effect, moving them).
All lines in the edit session are copied to the clipboard if you do not specify the lines using a label range on
the CUT command, or through the C or M commands.
If you specify a clipboard name, lines are copied to that clipboard. If the specified clipboard does not
yet exist, it is created. ISPF provides a default clipboard named DEFAULT. You can use up to 10 other
clipboards that you define. The defined clipboards exist as long as you are logged on to TSO and are
deleted when you log off.
To browse, edit, clear, or rename any of the clipboards, use the DISPLAY keyword of the CUT command:
CUT DISPLAY
Examples
This command saves to the default clipboard all the lines in the current file from the current cursor
position to the last line. These lines are appended to any lines that are already in the clipboard:
CUT .ZCSR .ZLAST APPEND
To save all the lines in the current file to a clipboard named USERC1, replacing any lines already in the
clipboard:
CUT .ZFIRST .ZLAST USERC1 REPLACE
This example assumes that you have APPEND set as the default behavior in the EDITSET command panel.
Because all lines are copied by default, in this case you could omit the labels .ZFIRST and .ZLAST.
DEFINE—Define a Name
The DEFINE primary command is used to:
• Identify a macro that replaces a built-in command of the same name
• Identify programs that are edit macros
• Assign an alias to a macro or built-in command
• Make a macro or built-in command inoperable
• Reset an inoperable macro or built-in command
• Disable a macro or built-in command
DEFINE is often used with the BUILTIN command.
DEFINE
220  z/OS: z/OS ISPF Edit and Edit Macros

## Page 253

Syntax
DEFINE
DEF
name MACRO
CMD
PGM
ALIAS name_2
NOP
RESET
DISABLED
name
The name for the command.
MACRO CMD
Identifies the name you are defining as a command language (CLIST or REXX exec) macro, which is
called in the same way as using the SELECT service CMD keyword with a percent symbol (%) preceding
the command. That means that you can specify only CLISTs or REXX EXECs.
MACRO PGM
Identifies the name that you are defining as a program (load module) macro.
ALIAS name_2
Identifies the name you are defining as an alias of another name, with the same characteristics. If
name_2 is already an alias, the editor replaces it with the command for which it is an alias. Therefore,
it is not possible to have an alias of an alias.
NOP
Makes the name that you are defining and all of its aliases inoperable until you reset them with RESET.
Therefore, when the name or an alias of the name is called, nothing is processed. NOP is similar to
DISABLED, except that disabled names cannot be reset by the RESET operand.
RESET
Resets the most recent definition of the name that you are defining to the status in effect before that
definition. For example, RESET makes inoperable names operable again.
DISABLED
Disables the name you are defining and all of its aliases until you completely exit the editor and
return to the ISPF Primary Option Menu. Therefore, when the name or an alias of the name is entered,
nothing is processed. A disabled command or macro cannot be restored by the RESET operand. To
disable RESET, use delimiters around 'RESET' to distinguish it from the keyword.
Description
The effects of a DEFINE command remain until you either issue DEFINE RESET or exit from the editor.
You enter the editor when you select option 2, and you do not exit the editor until you return to the ISPF
Primary Option Menu. Therefore, if you edit several members of a partitioned data set, one DEFINE at the
beginning affects them all.
To temporarily override the DEFINE command, use the BUILTIN command.
Stacking DEFINE commands
Except for the DISABLED operand, the DEFINE operations are stacked. The RESET operand unstacks
them. For example:
DEFINE A alias FIND
DEFINE A alias COPY
DEFINE A alias SAVE
stacks three definitions of A. Only the last one is effective. Here, A would be defined as SAVE.
This operation:
DEFINE
Chapter 10. Edit primary commands  221

## Page 254

DEFINE A RESET
removes one command from the stack, making the previous command effective. In the preceding
example, A would now be defined as COPY.
Examples
To define the name IJKDOIT as a CLIST or REXX macro, enter:
DEFINE IJKDOIT MACRO
To define the name SETITUP as a program macro, enter:
DEFINE SETITUP MACRO PGM
To define the name DOIT as an alias of the macro IJKDOIT, enter:
DEFINE DOIT ALIAS IJKDOIT
To define the name SAVE to have no effect, enter:
DEFINE SAVE NOP
To reset the definition of the name SAVE, enter:
DEFINE SAVE RESET
To define the name FINDIT as disabled, enter:
DEFINE FINDIT DISABLED
DELETE—Delete Lines
The DELETE primary command deletes lines from the data you are editing.
Note: As a precaution against error, there is no DELETE ALL command. To delete all lines, see
“Description” on page 223.
Syntax
DELETE
DEL
ALL
labela labelb
X
NX
ALL labela labelb
ALL
Specifies that all selected lines are deleted. The DELETE command, unlike FIND, CHANGE, and
EXCLUDE, does not accept NEXT, FIRST, PREV, or LAST. ALL is required to emphasize that NEXT is not
the default.
X
Restricts the lines deleted to those that are excluded.
NX
Restricts the lines deleted to those that are not excluded.
labela, labelb
Labels identifying the start and end of the group of lines which are deleted, including the lines with the
labels. To delete one line, enter the same label twice.
DELETE
222  z/OS: z/OS ISPF Edit and Edit Macros

## Page 255

For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
Description
To delete all lines, do one of these:
• To delete all lines by using the editor-defined labels:
DELETE ALL .ZFIRST .ZLAST
• To delete all lines by first resetting any excluded lines to make them not excluded, and then deleting all
lines that are not excluded:
RESET; DELETE ALL NX
Examples
In the examples that follow, .labela and .labelb represent the two labels that show the range of lines to be
deleted.
• To delete all excluded lines:
DELETE ALL X
• To delete all not excluded lines:
DELETE ALL NX
• To delete all excluded lines within a range:
DELETE .labela .labelb X
• To delete all not excluded lines within a range:
DELETE .labela .labelb NX
• To delete all lines within a range:
DELETE .labela .labelb
You can more easily determine which lines to delete in a large data set by excluding lines that meet some
criterion, or by leaving all lines that meet the criterion non-excluded. Then, with DELETE you can delete
many lines. For example, to delete all blank lines in a data set, type these commands on the command
line and press Enter after each one:
1. First, reset all excluded lines:
RESET X
2. Then, exclude lines containing characters that are not blanks:
EXCLUDE ALL P'¬'
3. Finally, delete the non-excluded lines, which contain only blanks:
DEL ALL NX
Another way to do the same thing is this:
1. First, exclude all lines:
EXCLUDE ALL
2. Then, find all lines containing a character that is not a blank:
DELETE
Chapter 10. Edit primary commands  223

## Page 256

FIND ALL P'¬'
3. Finally, delete the remaining excluded lines, which contain only blanks:
DEL ALL X
EDIT—Edit from within an Edit Session
The EDIT primary command allows you to edit another sequential data set, partitioned data set member,
or z/OS UNIX file during your current edit session.
Syntax
EDIT
member
GEN generation
member
A member of the ISPF library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
generation
The generation of the member to be edited. You may enter an absolute (positive) generation number
or a relative (negative) generation number. This parameter is valid only when the member is in a PDSE
version 2 data set that is configured for member generations.
Description
Editing one data set or member while you are already editing another is called recursive editing. To edit
another data set, member, or z/OS UNIX file during your current edit session:
1. On the command line, type:
EDIT
or
EDIT member
or
EDIT member GEN generation
Here, member represents the name of a member of the partitioned data set you are editing and
generation represents a generation of the member. The member and generation operands are optional.
2. Press Enter.
If you specify a member name, the current library concatenation sequence finds the member. The
member is displayed for editing. If you specify a generation number, the specified generation of the
member displays for editing.
If you do not specify a member name, the Edit Command Entry panel, which is identical to the regular
Edit Entry panel, appears. You can enter the name of any sequential, partitioned data set, or z/OS
UNIX file to which you have access. When you press Enter, the data set, member, or z/OS UNIX file is
displayed for editing.
EDIT
224  z/OS: z/OS ISPF Edit and Edit Macros

## Page 257

The editor suspends your initial edit session until the second-level edit session is complete. Editing
sessions can be nested until you run out of storage.
3. To exit from a nested edit session, enter an END or CANCEL command. The current edit session
resumes.
Examples
These steps show the use of the EDIT primary command:
1. Assume that you are editing a member named @INDEX and you need to edit a member in another data
set. So, you enter the EDIT command on the command line, omitting the member operand, as shown in
Figure 119 on page 225. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       ISP.SISPSAMP(@INDEX) - 01.00                    Columns 00001 00072
 Command ===> edit                                             Scroll ===> PAGE
 000302            that file.
 000303 ISRONLY  - Sample Edit Macro
 000304 **********************************************************************
 000305 * PDF Sample Programs for Creating Translate Tables
 000306 **********************************************************************
 000307 ISRAPLTT - A sample assembler module for creating your own set of
 000308            translate tables.  It contains the values for a 3278/3279
 000309            APL English terminal.
 000310 ISROWNTT - A sample assembler module for creating your own set of
 000311            translate tables.  It contains the values for a 3278/3279
 000312            English terminal.
 000313 **********************************************************************
 000314 * PDF Samples for Programming Languages
 000315 **********************************************************************
 000316 ISRASM   - Sample assembler program
 000317 ISRCOBOL - Sample cobol program
 000318 ISRFORT  - Fortran test program
 000319 ISRPLI   - Sample PL/I program
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 119. EDIT primary command example
2. When you press Enter, the Edit Command Entry panel (Figure 120 on page 226) appears. On this
panel, you enter the name of the partitioned data set and member that you want to edit: 
EDIT
Chapter 10. Edit primary commands  225

## Page 258

Figure 120. Edit Command Entry panel (ISREDM03)
3. When you press Enter again, the member is displayed for editing, as shown in Figure 121 on page 226: 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       ISP.SISPSAMP(ISRBOX) - 01.00                    Columns 00001 00072
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 ==MSG> -CAUTION- Profile is set to STATS ON. Statistics did not exist for
 ==MSG>           this member, but will be generated if data is saved.
 000001 /*********************************************************************/
 000002 /*                                                                   */
 000003 /* 5694-A01 (C) COPYRIGHT IBM CORP 1995, 2004                        */
 000004 /*                                                                   */
 000005 /* ISRBOX - Draw a box with its upper left corner at the             */
 000006 /*          cursor position                                          */
 000007 /*                                                                   */
 000008 /*********************************************************************/
 000009 ISREDIT MACRO
 000010 ISREDIT (ROW,COL) = CURSOR             /* Get cursor position     */
 000011
 000012 ISPEXEC CONTROL ERRORS RETURN          /* No macro error panel    */
 000013                                        /* Draw box over existing  */
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 121. Nested member editing example
EDIT
226  z/OS: z/OS ISPF Edit and Edit Macros

## Page 259

EDITSET—Display the Editor Settings Dialog
The EDITSET and EDSET primary commands cause the Editor Settings dialog to begin, enabling you to
modify Editor settings.
Syntax
EDITSET
EDSET
Description
The EDITSET primary command enables you to modify the Editor settings.
The Edit and View Settings panel
Entering the EDITSET or EDSET primary commands, or choosing the Edit_Settings action bar item causes
the panel shown in Figure 122 on page 227 to display:
Figure 122. Edit and View Settings panel (ISREDSET)
The fields on the panel are as follows:
User session initial macro
You can specify a macro to be run before you begin editing your sequential data set or any member of
a partitioned data set. This initial macro allows you to set up a particular editing environment for the
Edit session you are beginning. This initial macro runs in addition to any IMACRO value in your profile.
EDITSET
Chapter 10. Edit primary commands  227

## Page 260

Maximum initial storage allowed for Edit and View
The maximum amount of storage that edit and view use when initially loading the data into the edit
or view session. This number is in kilobytes and is rounded to the nearest 128 KB value. If you set a
limit on the initial amount of storage allowed, and a session requires more than that amount, the data
is shown in BROWSE mode instead of edit or view.
A value of zero indicates that the edit session should not impose any limits on initial storage used. If
this value is zero and there is not enough storage to load the data, a program error can result.
Target line for Find/Change/Exclude string
This indicates the line of the edit data display to which the target line of a FIND, CHANGE, or EXCLUDE
command should be positioned. The value can be from 1 to 99, the default is 2. If the value specified
is greater than the last line of the display, the target line is positioned to the last line of the display.
Always position Find/Change/Exclude string to target line
This determines whether the editor always positions the target line of a FIND, CHANGE, or EXCLUDE
command to the target line specified in the "Target line for found/changed/excluded string" field, or
only position the string if it is not currently on the display. The default is to only position the line if it is
not on the current display.
Remove action bars in ISPF edit and view panels
If this field is selected, the action bars in the edit or view panels are not shown. This field affects only
those panels that are included in ISPF, and has no effect on customized edit panels or edit panels
provided with products other than ISPF.
Force ISRE776 if RCHANGE passed arguments
If this field is selected then EDIT will ensure that when RCHANGE is issued from a PF key, it does not
try to process input from the command line. In this case RCHANGE will treat anything that you type
on the command line as an invalid parameter and will return an error message ISRE776. For more
information, see “Edit commands and PF key processing” on page 13.
CUT default
Append
If data exists on the clipboard, append the new data being cut to the end of the existing data.
Replace
If data exists on the clipboard, replace it with the new data being cut.
PASTE default
Delete
Remove the data from the clipboard after it has been pasted.
Keep
Do not remove the data from the clipboard after it has been pasted. This allows for data to be
pasted multiple times.
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
EDITSET
228  z/OS: z/OS ISPF Edit and Edit Macros

## Page 261

other changes have been made during the edit session up to that point, the confirmation panel is not
displayed.
Apply Setting Immediately
Controls whether a change in the setting applies to the current edit session (immediately) or on
the next edit session.
Preserve VB record length
You can select this option to cause the editor to store the original length of each record in variable-
length data sets and when a record is saved, the original record length is used as the minimum length
for the record.
Apply Setting Immediately
Controls whether a change in the setting applies to the current edit session (immediately) or on
the next edit session.
Examples
These steps show the use of the EDITSET primary command:
1. Assume that you are editing a member named PGM8 and you want to change the setting for
Confirming a Cancel, Move, or Replace action. So, you enter the EDITSET command on the command
line as shown in Figure 123 on page 229. 
Figure 123. EDITSET primary command example
2. When you press Enter, the Edit and View Settings panel (Figure 122 on page 227) appears.
3. If necessary, scroll down to display the Confirm Cancel/Move/Replace field. Enter or remove the slash
mark in the Confirm Cancel/Move/Replace field to make the setting as you want it to be.
END—End the Edit Session
The END primary command ends the editing of the current sequential data set or partitioned data set
member.
END
Chapter 10. Edit primary commands  229

## Page 262

Syntax
END
Description
To end an edit session by using END, either:
• Enter END on the command line, or
• Press a function key to which END is assigned. The default setting is F3
If no aliases have been defined for END, the editor's response to END depends on:
• Whether changes were made to the data during your current edit session
• If changes were made, whether SAVE was entered after the last change
• The setting of number mode, autonum mode, stats mode, autolist mode, and autosave mode in the edit
profile
• Whether you were editing a member that was an alias of another member
For additional explanation, see “Ending an edit session” on page 11.
Examples
To end the current edit session:
1. On the command line, type:
END
2. Press Enter.
EXCLUDE—Exclude Lines from the Display
The EXCLUDE primary command hides lines that contain a search string from view and replaces them with
a dashed line. To see the lines again, you enter either the FLIP, RESET or RESET EXCLUDED command.
Syntax
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
string
The search string you want to exclude. See “Finding, seeking, changing, and excluding data” on page
44.
labela, labelb
Labels identifying the start and end of the group of lines which the EXCLUDE command is to search.
EXCLUDE
230  z/OS: z/OS ISPF Edit and Edit Macros

## Page 263

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
String is delimited on both sides by blanks or other non-alphanumeric characters.
start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
Number of the first column the EXCLUDE command is to search.
right_col
Number of the last column the EXCLUDE command is to search.
Note:
1. For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
2. The EXCLUDE command allows you to control the starting point and the direction of the search by
positioning the cursor and using either the NEXT or PREV operand. For more information, see “Starting
point and direction of the search” on page 53.
Description
You can use the EXCLUDE command with the FIND and CHANGE commands to find a search string,
change it, and exclude the line that contains the string from the panel.
To exclude the next non-excluded line that contains the letters ELSE without specifying any other
qualifications:
1. On the command line, type:
EXCLUDE ELSE
2. Press Enter. Since no other qualifications were specified, the letters ELSE can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• Anywhere within the current boundaries
EXCLUDE
Chapter 10. Edit primary commands  231

## Page 264

To exclude the next line that contains the letters ELSE, but only if the letters are uppercase:
1. On the command line, type:
EXCLUDE C'ELSE'
2. Press Enter. This type of exclusion is called a character string exclusion (note the C that precedes the
search string) because it excludes the next line that contains the letters ELSE only if the letters are
found in uppercase. However, since no other qualifications were specified, the exclusion occurs no
matter where the letters are found on a non-excluded line, as outlined in the previous list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Examples
The example shown here excludes the first non-excluded line in the data set that contains the letters
ELSE. However, the letters must occur on or between lines labeled .E and .S and they must be the first
four letters of a word:
EXCLUDE ELSE .E .S FIRST PREFIX
The example shown here excludes the last non-excluded line in the data set that contains the letters
ELSE. However, the letters must occur on or between lines labeled .E and .S and they must be the last
four letters of a word.
EXCLUDE ELSE .E .S LAST SUFFIX
The example shown here excludes the first non-excluded line that immediately precedes the cursor
position and that contains the letters ELSE. However, the cursor must not be positioned ahead of the
lines labeled .E and .S. Also, the letters must occur on or between lines labeled .E and .S; they must be
standalone characters (not part of any other word); and they must exist within columns 1 and 5:
EXCLUDE ELSE .E .S PREV WORD 1 5
FIND—Find a Data String
The FIND primary command locates one or more occurrences of a search string.
Syntax
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
string
The search string you want to find. See “Finding, seeking, changing, and excluding data” on page 44.
FIND
232  z/OS: z/OS ISPF Edit and Edit Macros

## Page 265

labela, labelb
Labels identifying the start and end of the group of lines which FIND is to search.
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
String is delimited on both sides by blanks or other non-alphanumeric characters.
X
Scans only lines that are excluded from the display.
NX
Scans only lines that are not excluded from the display.
start_col
The first column to be included in the range of columns to be searched. When you specify only one
column, the editor finds the string only if the string starts in the specified column.
left_col
Number of the first column the FIND command is to search.
right_col
Number of the last column the FIND command is to search.
Note:
1. For more information about restricting the search to only a portion of each line, see “Limiting the
search to specified columns” on page 54.
2. The FIND command allows you to control the starting point and the direction of the search by
positioning the cursor and using either the NEXT or PREV operand. For more information, see “Starting
point and direction of the search” on page 53.
Description
You can use the FIND command with the EXCLUDE and CHANGE commands to find a search string,
change it, and exclude the line that contains the string from the panel.
To find the next occurrence of the letters ELSE without specifying any other qualifications:
1. On the command line, type:
FIND ELSE
FIND
Chapter 10. Edit primary commands  233

## Page 266

2. Press Enter. Since no other qualifications were specified, the letters ELSE can be:
• Uppercase or a mixture of uppercase and lowercase
• At the beginning of a word (prefix), the end of a word (suffix), or the entire word (word)
• In either an excluded or a non-excluded line
• Anywhere within the current boundaries
To find the next occurrence of the letters ELSE, but only if the letters are uppercase:
1. On the command line, type:
FIND C'ELSE'
2. Press Enter. This type of search is called a character string search (note the C that precedes the search
string) because it finds the next occurrence of the letters ELSE only if the letters are in uppercase.
However, since no other qualifications were specified, the letters can be found anywhere in the data
set or member, as outlined in the preceding list.
For more information, including other types of search strings, see “Finding, seeking, changing, and
excluding data” on page 44.
Examples
The example shown here finds the first occurrence in the data set of the letters ELSE. However, the letters
must occur on or between lines labeled .E and .S and they must be the first four letters of a word:
FIND ELSE .E .S FIRST PREFIX
The example shown here finds the last occurrence in the data set of the letters ELSE. However, the letters
must occur on or between lines labeled .E and .S; they must be the last four letters of a word; and they
must be found in an excluded line.
FIND ELSE .E .S LAST SUFFIX X
The example shown here finds the first occurrence of the letters ELSE that immediately precedes the
cursor position. However, the cursor must not be positioned ahead of the lines labeled .E and .S. The
letters must occur on or between lines labeled .E and .S; they must be standalone characters (not part of
any other word); they must be found in a non-excluded line; and they must exist within columns 1 and 5:
FIND ELSE .E .S PREV WORD NX 1 5
FLIP—Reverse Exclude Status of Lines
The FLIP primary command reverses the exclude status of a specified group of lines or of all the lines in a
file, including data, information, message, and note lines.
Syntax
FLIP
.ZFIRST .ZLAST
labela
labelb
labela, labelb
Labels identifying the start and end of the group of lines for which FLIP is to reverse the exclude
status. If labelb is not supplied, then the single line identified by labela is flipped.
FLIP
234  z/OS: z/OS ISPF Edit and Edit Macros

## Page 267

For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
Description
The FLIP primary command reverses the exclude status of a range of lines you specify with labels. It
can also reverse the exclude status of all the lines in a file. FLIP excludes all lines that are currently
visible, and makes all excluded lines visible. For example, if you have used the 'X ALL;FIND ALL xyz'
command to find lines containing a string (xyz), you can use FLIP to see the lines which do not contain
the string.
The range is optional. If no range is specified, the exclude status is reversed for all of the lines in the file.
To reverse the exclude status of all the lines in a file:
1. Enter this command on the command line:
FLIP
2. Press Enter.
All the excluded lines in the file are displayed, and all the previously displayed lines are excluded.
To reverse the exclude status of a range of lines:
1. Enter this command on the command line:
FLIP .A .B
Actual values are substituted for .a and .b and can be defined by an edit macro or by the user.
2. Press Enter.
All the lines with the specified range that were previously excluded are displayed, and all the lines
within the specified range that were displayed are excluded.
Examples
In the example shown in Figure 124 on page 235, the edit session contains 10 lines:
Figure 124. Example of data set
FLIP
Chapter 10. Edit primary commands  235

## Page 268

After excluding lines 4 through 7, the data set looks like Figure 125 on page 236:
Figure 125. Example of data set with excluded lines
After executing FLIP, all previously excluded lines are shown. All previously visible lines are excluded, as
shown in Figure 126 on page 236.
Figure 126. Example of data set using FLIP on excluded lines
HEX—Display Hexadecimal Characters
HEX
236  z/OS: z/OS ISPF Edit and Edit Macros

## Page 269

The HEX primary command sets hexadecimal mode, which determines whether data is displayed in
hexadecimal format.
Syntax
HEX ON
VERT
DATA
VERT
DATA
OFF
ON VERT
Displays the hexadecimal representation of the data vertically (two rows per byte) under each
character.
ON DATA
Displays the hexadecimal representation of the data as a string of hexadecimal characters (two per
byte) under the characters.
OFF
Does not display hexadecimal representation of the data.
Note: The command, HEX OFF, cancels the effect of any previous HX or HXX commands.
Description
The HEX command determines whether the editor displays hexadecimal representation in a vertical or
data string format. See Figure 128 on page 238 and Figure 129 on page 239 for examples of these two
formats.
When the editor is operating in hexadecimal mode, three lines are displayed for each source line. The
first line shows the data in standard character form, while the next two lines show the same data
in hexadecimal representation. This applies to every line except profile lines (=PROF>), excluded line
messages (- - - ), message lines (==MSG>), and informational lines (======).
Besides normal editing on the first of the three lines, you can change any characters by typing over the
hexadecimal representations.
You can also use the FIND, CHANGE, and EXCLUDE commands to find, change, or exclude invalid
characters or any specific hexadecimal character, regardless of the setting of hexadecimal mode. See the
discussion of picture strings and hexadecimal strings under “Finding, seeking, changing, and excluding
data” on page 44.
Examples
Suppose you are editing the data set member shown in Figure 127 on page 238:
HEX
Chapter 10. Edit primary commands  237

## Page 270

Figure 127. Member with hexadecimal mode off
Pressing Enter causes the hexadecimal value for each character on the panel, including blanks, to be
displayed in vertical format, as shown in Figure 128 on page 238.
Figure 128. Hexadecimal display, vertical representation
You can enter the HEX DATA command to change the display to data format, as shown in Figure 129 on
page 239.
HEX
238  z/OS: z/OS ISPF Edit and Edit Macros

## Page 271

Figure 129. Hexadecimal display, data representation
HIDE—Hide Excluded Lines Message
The HIDE command removes the "n Line(s) not Displayed" messages from the display where lines have
been excluded by the EXCLUDE command.
The HIDE function has dependencies on the value of the ISPF variable ZHIDEX, panel attributes,
and extended highlighting support of the terminal. These dependencies are described in the section
"Providing customized Browse and Edit panels" in the ISPF Planning and Customisation Guide.
Syntax
HIDE EXCLUDE
EXCLUDED
EXC
EX
X
X
Removes each "n Line(s) not Displayed" message from the display and underscores the line number
field of the preceding line.
Description
The HIDE command removes the "n Line(s) not Displayed" messages from the display where lines
have been excluded by the EXCLUDE command. Instead the line number field of the preceding line is
underscored (where the terminal supports the underscore attribute) to indicate that part of the data is not
being displayed.
The RESET HIDE command redisplays the excluded lines messages.
HIDE
Chapter 10. Edit primary commands  239

## Page 272

Examples
In Figure 130 on page 240, the edit session shows that three lines are excluded after line 000020 and
one line is excluded after line 000060:
 EDIT       SBURNF.PRIVATE.DATA(HIDEXMP) - 01.01            Columns 00001 00072
 ****** ***************************** Top of Data ******************************
 000010  example text line number 00010
 000020  example text line number 00020
 - - -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  3 Line(s) not Displayed
 000060  example text line number 00060
 - - -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  1 Line(s) not Displayed
 000080  example text line number 00080
 000090  example text line number 00090
 000100  example text line number 00100
 ****** **************************** Bottom of Data ****************************
 Command ===> hide x                                           Scroll ===> CSR 
Figure 130. Before the HIDE primary command
Figure 131 on page 240 shows the edit session after the HIDE X command is entered. Note that the line
number fields for lines 000020 and 000060 are underscored.
 EDIT       SBURNF.PRIVATE.DATA(HIDEXMP) - 01.01            Columns 00001 00072
 ****** ***************************** Top of Data ******************************
 000010  example text line number 00010
 000020  example text line number 00020
 000060  example text line number 00060
 000080  example text line number 00080
 000090  example text line number 00090
 000100  example text line number 00100
 ****** **************************** Bottom of Data ****************************
 Command ===>                                                  Scroll ===> CSR 
Figure 131. After the HIDE primary command
HILITE—Enhanced Edit Coloring
HILITE is used to control the use of color in the editor by changing the settings for the enhanced color and
language-sensitive editing features.
Note: Language-sensitive and enhanced coloring of the edit session is only available when enabled by the
installer or the person who maintains the ISPF product. For information on enabling the enhanced color
function, see z/OS ISPF Planning and Customizing.
HILITE with no operands presents a dialog (see “The HILITE dialog” on page 33) that allows you to
change coloring options, and to see which keywords are supported for each language.
Language and logic hiliting is not supported for ASCII or UTF-8 editing sessions and the HILITE command
is not available during these edit sessions.
HILITE
240  z/OS: z/OS ISPF Edit and Edit Macros

## Page 273

The following z Hilite variables represent the values of the active edit profile and are available for
reference in the shared memory pool. These are ZHIAUTO, ZHILANG, ZHICOLOR, ZHIPAREN, ZHIFIND,
and ZHICURSR. See Dialog variables in z/OS ISPF Reference Summary for more information.
Syntax
HILITE
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
Sets coloring OFF, with the exception of cursor, find, and parenthesis highlighting.
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
Chapter 10. Edit primary commands  241

## Page 274

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
Allows ISPF to determine the language. See “Automatic language selection” on page 29 for more
information.
DEFAULT
Highlights the data in a single color.
OTHER
Highlight the data as a pseudo-PL/I language. Limited CLIST support is also provided by OTHER.
ASM
Highlights the data as Assembler.
BOOK
Highlights the data as BookMaster.
C
Highlights the data as C.
COBOL
Highlights the data as COBOL
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
242  z/OS: z/OS ISPF Edit and Edit Macros

## Page 275

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
Toggles parenthesis matching. When parenthesis matching is active, only comments are specially
colored. All other code appears in the default color. Note that extra parenthesis highlighting is always
active when highlighting is active.
FIND
The HILITE FIND command toggles the highlighting color of any string that would be found by an
RFIND. The user can select the highlight color. The default is reverse video white.
Only non-picture strings are supported, and the only additional qualifiers recognized are hex strings
(X'…'), character strings (C'…'), text strings (T'…'), WORD, PREFIX and SUFFIX, and boundaries
specified in the FIND command. Hex strings may be highlighted, but non-displayable characters are
not highlighted. Labels are ignored when FIND strings are highlighted.
Because FIND highlighting is not quite as robust as the FIND command itself, the editor may highlight
more occurrences of the FIND string than FIND would actually locate. The FIND operand toggles
the display of search strings. If HILITE FIND is issued when FIND highlighting is in effect, FIND
highlighting is disabled. Similarly, if FIND highlighting is disabled, the HILITE FIND command enables
it.
Note: RESET has been enhanced, through the addition of a FIND operand, to temporarily disable the
highlighting of FIND strings until the next FIND, RFIND, CHANGE, or RCHANGE command is issued.
RESET with the FIND operand (or no operands at all), temporarily disables the highlighting of FIND
strings.
CURSOR
The CURSOR operand toggles the highlighting of the phrase that contains the cursor in a user
selectable color. The default is white.
Cursor highlighting in Edit is performed in a manner similar to the way it is done in Browse. The entire
phrase from the previous blank to the next blank is highlighted. The CURSOR operand toggles cursor
highlighting. If HILITE CURSOR is issued when CURSOR highlighting is in effect, CURSOR highlighting
is disabled. Similarly, if CURSOR highlighting is disabled, the HILITE CURSOR command enables it.
SEARCH
HILITE SEARCH finds the first unmatched END, ELSE, }, or ) above the last displayed line on the
screen. If a mismatched item is found, the file is scrolled so that the mismatch is at the top of the
screen. The search for mismatches only occurs for lines above the last displayed line, so you may
need to scroll to the bottom of the file before issuing the HI SEARCH command.
Search is not available when the DEFAULT language operand is used. Search for language keywords is
only supported for languages which supported by the logic option.
HILITE
Chapter 10. Edit primary commands  243

## Page 276

DISABLED
Turns off all HILITE features and removes all action bars. This benefits performance at the expense
of function. Since DISABLED status is not stored in the edit profile, you need to reenter this operand
each time you enter the editor. When DISABLED is in effect, keylists are unavailable for that edit
session.
Description
The HILITE primary command can be used to highlight, in user-specified colors, many language-specific
constructs, program logic features, the phrase containing the cursor, and any strings that match the
previous FIND operation or those that would be found by an RFIND or RCHANGE request. In addition,
when HILITE is entered with no operands, a dialog appears that allows you to set default colors for the
data area in non-program files, for any characters typed since the previous Enter or PF key entry, and for
strings located by FIND.
Both HI and HILIGHT are valid synonyms for HILITE.
When the code page being used is not the English codepage, the HILITE primary command does
not detect key sequences if the control character within the key sequence has a different binary
representation in the code page being used from the binary representation in the codepage used for
English. For example, in the C language, a '\' is used as an escape sequence character to influence the
interpretation of the next character; however, the '\' has a different binary representation in different
codepages.
Note:
1. Highlighting is not available for edit sessions that involve:
• Data sets with record lengths greater than 255
• Mixed mode edit sessions (normally used when editing DBCS data)
• Formatted data
2. Five character labels starting with the letter "o", in the form ".Oxxxx", are used by the COMPARE
command. Any labels of this form can cause unpredictable highlighting results.
IMACRO—Specify an Initial Macro
The IMACRO primary command saves the name of an initial macro in the current edit profile.
See “Initial macros” on page 24 for more information on creating and using initial macros.
Syntax
IMACRO name
NONE
name
The name of the initial macro to be run when you are editing the data set type that matches the
current edit profile. This macro is run before any data appears.
For more information about displaying and defining a profile, see “Displaying or defining an edit
profile” on page 18.
NONE
Indicates that no macro is to be run at the beginning of each edit session. The edit profile shows a
value of NONE when no initial macro has been specified.
IMACRO
244  z/OS: z/OS ISPF Edit and Edit Macros

## Page 277

Examples
To save STARTUP as the initial macro, type:
IMACRO STARTUP
To reset the profile with no initial macro, type:
IMACRO NONE
LEVEL—Specify the Modification Level Number
The LEVEL primary command allows you to control the modification level that is assigned to a member of
an ISPF library.
See “Version and modification level numbers” on page 26 for more information about level numbers.
Syntax
LEVEL num
num
The modification level. It can be any number from 0 to 99.
Description
To specify the modification level number:
1. On the command line, type:
LEVEL num
where num is the new level number.
2. Press Enter.
Examples
In Figure 132 on page 246, the version and modification level numbers on line 1 show that this is Version
1, Modification 3 (01.03). Type LEVEL 0 on the command line to reset the modification level number to
00.
LEVEL
Chapter 10. Edit primary commands  245

## Page 278

Figure 132. Member with modific ation  level of 03
After you press Enter, the editor resets the modification level, as shown in Figure 133 on page 246.
Figure 133. Member with modific ation  level reset to 00
LF—realign data on the ASCII linefeed character
The LF primary command allows you to realign the data being edited by interpreting the ASCII linefeed
character X'0A'. The LF primary command is not available when editing a z/OS UNIX file. Instead, use the
ASCII edit facility to automatically realign the data in a z/OS UNIX file based on the ASCII linefeed and
carriage return characters. See “Working with ASCII data” on page 51.
LF
246  z/OS: z/OS ISPF Edit and Edit Macros

## Page 279

Note: If the data is saved, it is saved in the realigned state. There is no command to reverse the
alignment. The command should not be executed twice against the data, as the blanks following the
linefeed character will be interpreted as part of the data for the next line.
Syntax
LF
See “Restructuring data based on the linefeed character” on page 51 for more information.
Examples
To realign the data being edited by interpreting the ASCII linefeed character X'0A':
LF
LOCATE—Locate a Line
The LOCATE primary command allows you to scroll up or down to a specified line. The line then appears
as the first line on the panel. There are two forms of LOCATE: specific and generic.
Syntax
Specific LOCATE syntax
LOCATE label
linenum
The specific form of the LOCATE command positions a particular line at the top of the panel. You must
specify either a line number or a label.
label
A previously assigned label.
linenum
An edit line number. If that line number exists, it appears at the top. If the line number does not exist,
the line with the next lower number appears at the top of the data area.
The linenum operand is a numeric value of up to 8 digits. You do not need to type leading zeros. If
the operand contains 6 or fewer digits, it refers to the number in the line command field to the left of
each line. If linenum contains 7 or 8 digits, it refers to the sequence numbers in the data records. For
NUMBER ON STD, the editor refers to the modific ation  flag. For NUMBER OFF, it refers to the ordinal
line number (first=1, fifth=5, and so on). For NUMBER ON COBOL, it refers to the number in the line
command field, which is the data sequence number. See “Sequence number format and modification
level” on page 27 for more information.
LOCATE
Chapter 10. Edit primary commands  247

## Page 280

Generic LOCATE syntax
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
The generic LOCATE command positions the panel to the first, last, next, or previous occurrence of a
particular kind of line.
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
Searches for a special non-data (temporary) line:
• Bounds line flagged as =BNDS>
• Column identification lines flagged as =COLS>
• Information lines flagged as ======
• Mask lines flagged as =MASK>
• Message lines flagged as ==MSG>
• Note lines flagged as =NOTE=
• Profile lines flagged as =PROF>
• Tabs line flagged as =TABS>
INFOLINE
Searches for information lines flagged with ======
MSGLINE
Searches for message lines flagged with ==MSG>
LOCATE
248  z/OS: z/OS ISPF Edit and Edit Macros

## Page 281

NOTELINE
Searches for note lines flagged with =NOTE=
labela, labelb
Labels identifying the start and end of the group of lines to be searched.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
Examples
To find the next special line, type:
LOCATE SPE
To find the first error line (==ERR>), type:
LOCATE ERR FIRST
To find the next line with a label, type:
LOC NEXT LABEL
To find the next excluded line between .START and .END, type:
LOC X .START .END
To find the first excluded line between .E and .S, type:
L FIRST .E .S X
To find the first message line, type:
LOCATE FIRST MSGLINE
MODEL—Copy a Model into the Current Data Set
The model name form of the MODEL primary command copies a specified dialog development model
before or after a specified line.
The class name form of the MODEL primary command changes the model class that the editor uses
to determine which model you want. For more information on edit models, see Chapter 4, “Using edit
models,” on page 69.
Syntax
Model name syntax
MODEL
model_name
qualifier
AFTER
BEFORE
label
NOTES
NONOTES
MODEL
Chapter 10. Edit primary commands  249

## Page 282

If you omit the model name or a required qualifier, or if there is a validation error, the editor displays a
series of selection panels from which you can select the desired information.
model_name
The name of the model to be copied, such as VGET for the VGET service model. This operand can also
be one of the options listed on a model selection panel, such as V1 for the VGET service model. See
z/OS ISPF Planning and Customizing for a list of models and model names.
qualifier 
The name of a model on a secondary model selection panel, such as TBCREATE for the TBCREATE
service model. This operand can also be one of the options listed on a model selection panel, such as
G1 for the TBCREATE service model.
For example, a model selection panel allows you to enter T1 to choose table models. Another model
selection panel then appears for choosing table models, such as G1 for the TBCREATE service model.
Therefore, your MODEL primary command could use either TABLES or T1 as the model-name operand
and either TBCREATE or G1 at the qualifier operand. The simplest way would be to use TBCREATE
or G1 as the model-name operand and omit the qualifier operand. See z/OS ISPF Planning and
Customizing for a list of models and model names.
AFTER label
Identifies the line after which the model is to be copied. If you have not defined a label, use the A or
B line command to specify the destination. The only time this operand or the BEFORE label operand is
not required is when the data set or member is empty.
BEFORE label
Identifies the line before which the model is to be copied. If you have not defined a label, use the A or
B line command to specify the destination. The only time this operand or the AFTER label operand is
not required is when the data set or member is empty.
NOTES
Overrides the current edit profile setting for note mode, to include any notes that are part of the
model.
NONOTES
Overrides the current edit profile setting for note mode, to exclude any notes that are part of the
model.
Class name syntax
MODEL
CLASS
class_name
If you omit class_name, or if there is a validation error, the editor displays a series of selection panels
from which you can select the desired information.
CLASS
When entered without the optional class_name operand, the editor displays the Model Classes panel,
from which you can select a model class. When entered with the class_name operand, the macro
specifies that the current model class is to be replaced by class_name. In both cases, the new class
name is used for all models from that point on, until you change the model class again or end the edit
session.
class_name
Specifies a new class for the current edit session. It must be a name on the Model Classes panel or an
allowable abbreviation. The model class coincides with the type of model, such as REXX, COBOL, or
FORTRAN.
Examples
You are editing a new member named NEWMEM and have not decided which service to use first. Figure
134 on page 251 shows the display screen for NEWMEM. Type MODEL on the command line without any
MODEL
250  z/OS: z/OS ISPF Edit and Edit Macros

## Page 283

operands. Here, the model name form of the MODEL command is used and the A (after) line command is
used instead of the AFTER operand.
Figure 134. Before Model command
The data set type is EXEC, so the editor displays the REXX Models panel (Figure 135 on page 251) when
you press Enter. To begin with the VGET service, you type V1 on the Option line and press Enter.
Figure 135. REXX Models panel (ISREMRXC)
MODEL
Chapter 10. Edit primary commands  251

## Page 284

The editor inserts the VGET service model into the NEWMEM member, as shown in Figure 136 on page
252. Because the edit profile is set to NOTE ON, the model's notes are also included.
Figure 136. REXX model of VGET service
MOVE—Move Data
The MOVE primary command moves a sequential data set, member of a partitioned data set, or z/OS
UNIX file into the data being edited.
If no options are specified with the MOVE command, the Edit/View Move panel is displayed.
Syntax
MOVE
move_options
move_options
member
( member)
dsname
pathame
AFTER
BEFORE
label 1 ASCII
EBCDIC
UTF8
Notes:
1 If you don't specify the position using a label, you must specify the position by using an A or B line
command.
member
A member of the ISPF library or partitioned data set you are editing.
dsname
A partially qualified or fully qualified data set name. If the data set is partitioned you can include a
member name in parentheses or select a member from a member list.
MOVE
252  z/OS: z/OS ISPF Edit and Edit Macros

## Page 285

pathname
The pathname for a z/OS UNIX regular file or directory. If a directory is specified, a directory selection
list is displayed, allowing you to select the file to be moved. (Also, see “Specifying z/OS UNIX
pathnames with edit primary and macro commands” on page 15.)
AFTER
The data is moved after the line with the specified label.
BEFORE
The data is moved before the line with the specified label.
label
Label identifying the line where the data is to be copied. It can be either a label that you define or one
of the editor-defined labels, such as .ZF or .ZL.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being moved in from the external file is converted from the
character set designated by the keyword to the character set specified for the file being edited or to
the terminal character set.
The label can be either a label you define or one of the editor-defined labels, such as .ZF and .ZL. If
you have not defined a label and the editor-defined labels are not appropriate for your purpose, use the A
(after) or B (before) line command to specify the data's destination.
If the data set, member, or z/OS UNIX file that you are editing is empty, you do not need to specify a
destination for the data being moved.
Note: If the member name or data set name is less than 8 characters and the data set you are editing
is partitioned, a like-named member is copied. If a like-named member does not exist, the name is
considered to be a partially qualified data set name.
Description
MOVE adds data that already exists to the data set, member, or z/OS UNIX file that you are editing. Use
MOVE if you want to move data rather than copy it from one data set, member, or z/OS UNIX file to
another.
The member, sequential data set, or z/OS UNIX file is deleted after the move. For a concatenated
sequence of ISPF libraries, the deletion occurs only if the member was in the first library.
To move data into an empty data set, member, or z/OS UNIX file:
1. On the command line, type:
MOVE member
or:
MOVE dsname
or:
MOVE pathname
The member, dsname, and path name operands are optional. If you do not specify the name of a
member, data set, or z/OS UNIX file to be moved, the Edit Move panel appears. Enter the data set,
member name, or z/OS UNIX file on this panel.
2. Press Enter. The data is moved.
To move data into a data set, member, or z/OS UNIX file that is not empty:
1. On the command line, type:
MOVE member AFTER | BEFORE label
MOVE
Chapter 10. Edit primary commands  253

## Page 286

or:
MOVE dsname AFTER | BEFORE label
or:
MOVE pathname AFTER | BEFORE label
The member, dsname, and path name operands are optional.
The AFTER label and BEFORE label operands are optional, also. However, if the data set, member,
or z/OS UNIX file that is to receive the moved data is not empty, you must specify a destination for
the moved data. Therefore, if you do not use a label, substitute either the A (after) or B (before)
line command as the destination of the moved data. However, a number indicating that the A or B
command should be repeated cannot follow the line command.
If the data set, member, or z/OS UNIX file is not empty and you do not specify a destination, a "MOVE/
COPY Pending" message is displayed in the upper-right corner of the panel and the data is not moved.
When you type a destination and press Enter, the data is moved.
2. Press Enter. If you entered the name of a member, data set , or z/OS UNIX file, the member, data set,
or z/OS UNIX file is moved. Otherwise, the Edit Move panel appears. See the previous example for
more information.
See “Copying and moving data” on page 41 if you need more information.
Examples
These steps show how you can move data when you omit the member name and the editor panels
appear:
1. Type MOVE on the command line and specify the destination of the operation. In Figure 137 on page
254, the data is to be moved after line 000400, as specified by the A (after) line command. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(DESTDATA) - 01.01                  Columns 00001 00072
 Command ===> move                                             Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 000200 This is the member into which the lines are to be moved.
 000300      +---------------------+
 a00400      |                     |
 000500      |                     |
 000600      |                     |
 000700      |                     |
 000800      +---------------------+
 000810
 000900 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 001000
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 137. Member before data is moved
2. When you press Enter, the Edit/View Move panel appears. Specify the data you want moved.
This example (Figure 138 on page 255) moves the data set member named TODATA.
MOVE
254  z/OS: z/OS ISPF Edit and Edit Macros

## Page 287

Menu  RefList  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
                                 Edit/View Move
 Command ===>
 "Current" Data Set: USERSID.TEST(DESTDATA)
 From ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . USERSID  . . .          . . .          . . .         
    Type  . . . . MSGGEN  
    Member  . . .            (Blank or pattern for member selection list)
 From Other Partitioned or Sequential Data Set, or z/OS UNIX file:
    Name . . . . . TEST(TODATA)                                               +
    Volume Serial            (If not cataloged)
 Data Set Password  . .           (If password protected)
 Data Conversion option                                                        
    1. EBCDIC                                                                  
    2. ASCII                                                                   
    3. UTF-8
 Press ENTER key to move. (Member or sequential data set may be deleted)
 Enter END command to cancel move.
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 138. Edit Move panel (ISREMOV1)
3. Figure 139 on page 255 shows the contents of the TODATA member which is moved into the original
data set. This panel is shown only for this example, so you can see the data that is being moved. It is
not displayed during a move sequence. 
 EDIT       USERSID.TEST(TODATA) - 01.00                    Columns 00001 00072
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 +----------------+
 000200 | This is the    |
 000300 | material to    |
 000400 | be created in  |
 000500 | another member |
 000600 +----------------+
 ****** **************************** Bottom of Data ****************************
⋮
Figure 139. Data set to be moved
4. When you press Enter, the editor moves the data and displays a short message in the upper right
corner of the panel. Figure 140 on page 256 shows the result of using MOVE. 
MOVE
Chapter 10. Edit primary commands  255

## Page 288

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(DESTDATA) - 01.01                  Member TODATA moved
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
 000200 This is the member into which the lines are to be moved.
 000300      +---------------------+
 000400      |                     |
 000410 +----------------+
 000420 | This is the    |
 000430 | material to    |
 000440 | be created in  |
 000450 | another member |
 000460 +----------------+
 000500      |                     |
 000600      |                     |
 000700      |                     |
 000800      +---------------------+
 000810
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 140. Member after data has been moved
NONUMBER—Turn Off Number Mode
The NONUMBER primary command turns off number mode, which controls the numbering of lines in the
current data.
Syntax
NONUMBER
NONUMBR
NONUMB
NONUM
Description
You can also use NUMBER OFF to turn off number mode.
When number mode is off, NONUMBER prevents any verification of valid line numbers, generation of
sequence numbers, and the renumbering of lines that normally occurs when autonum mode is on.
Examples
To turn number mode off by using NONUMBER, enter this command:
NONUMBER
NOTES—Display Model Notes
The NOTES primary command sets note mode, which controls whether notes are displayed when a dialog
development model is inserted into the data.
NONUMBER
256  z/OS: z/OS ISPF Edit and Edit Macros

## Page 289

Syntax
NOTES
NOTE
ON
OFF
ON
Displays explanatory notes when a model is copied into the data being edited or when notes are
added to the edit session by an edit macro.
OFF
Does not display explanatory notes.
Description
Note mode is saved in the edit profile. To check the setting of note mode:
1. On the command line, type:
PROFILE 4
2. Press Enter. The note mode setting appears as either NOTE ON or NOTE OFF on the fourth line of the
edit profile.
You can set the note mode with a primary command and then use the NOTES or NONOTES operand on the
MODEL command to override the default mode for a particular model.
See “MODEL—Copy a Model into the Current Data Set” on page 249 for information about copying dialog
development models.
Examples
To set note mode on:
1. On the command line, type:
NOTES ON
2. Press Enter. The next time you insert a model, the explanatory notes appear along with the model.
To set note mode off:
1. On the command line, type:
NOTES OFF
2. Press Enter. The next time you insert a model, the explanatory notes are not displayed along with the
model.
NULLS—Control Null Spaces
The NULLS primary command sets nulls mode, which determines whether trailing spaces in each data
field are written to the panel as blanks or nulls.
NULLS
Chapter 10. Edit primary commands  257

## Page 290

Syntax
NULLS
NULL
NUL
ON STD
ON
ALL
STD
ALL
OFF
ON STD
Specifies that in fields containing any blank trailing space, the space is written as one blank followed
by nulls. If the field is entirely empty, it is written as all blanks.
ON ALL
Specifies that all trailing blanks and all-blank fields are written as nulls.
OFF
Specifies that trailing blanks in each data field are written as blanks.
Description
Blank characters (X'40') and null characters (X'00') both appear as blanks. When you use the I (insert)
line command, the data entry area appears as blanks for NULLS ON STD and as nulls for NULLS ON ALL.
Trailing nulls simplify use of the Ins (insert) key on the IBM 3270 keyboard. You can use this key to insert
characters on a line if the line contains trailing nulls.
Besides using the NULLS command, you can create nulls at the end of a line by using the Erase EOF or Del
(delete) key. Null characters are never stored in the data; they are always converted to blanks.
Note: When you swap screens in split screen mode, the nulls are replaced by spaces until you press an
interrupt key, such as Enter, or a function key.
Examples
To set nulls mode on with all trailing blanks and all-blank fields written as nulls, enter this command:
NULLS ON ALL
To set nulls mode on with blank trailing space written as one blank followed by nulls and empty fields
written as all blanks, enter this command:
NULLS ON STD
To set nulls mode off and thus have trailing blanks in each data field, enter this command:
NULLS OFF
NUMBER—Generate Sequence Numbers
The NUMBER primary command sets number mode, which controls the numbering of lines in the current
data.
NUMBER
258  z/OS: z/OS ISPF Edit and Edit Macros

## Page 291

Syntax
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
ON
Automatically verifies that all lines have valid numbers in ascending sequence and renumbers any
lines that are either unnumbered or out of sequence. You can also use RENUM to turn number mode
on and renumber lines.
The editor interprets the STD, COBOL, and DISPLAY operands only when number mode is turned on.
OFF
Turns number mode off. You can also use NONUMBER to turn number mode off. If you alter or delete
sequence numbers and enter NONUMBER on the Command line at the same time, the editor issues
the message Some input data ignored and discards the data typed over the sequence numbers.
The editor converts the original sequence numbers to data.
STD
Numbers the data in the standard sequence field.
COBOL
Numbers the data in the COBOL field.
STD COBOL
Numbers the data in both fields.
If both STD and COBOL numbers are generated, the STD number is determined and then used as the
COBOL number. This can result in COBOL numbers that are out of sequence if the COBOL and STD
fields were not synchronized. Use RENUM to force synchronization.
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
if required, so that the left most column of the data window is the first column displayed.
Description
Attention: If number mode is off, make sure the first 6 columns of your data set are blank before
turning COBOL number mode on. Otherwise, the data in these columns is replaced by sequence
numbers. If that happens and if edit recovery or SETUNDO is on, you can use the UNDO command
NUMBER
Chapter 10. Edit primary commands  259

## Page 292

to recover the data. You can also use CANCEL at any time to end the edit session without saving
the data.
When number mode is on, NUMBER verifies that all lines have valid numbers in ascending sequence. It
renumbers any lines that are either unnumbered or out of sequence, but it does not otherwise change
existing numbers.
In number mode, the editor automatically generates sequence numbers in the data for new lines created
when data is copied or inserted. The editor also automatically renumbers the data when it is saved if
autonum mode is in effect.
If the number overlays the shift-in (SI) or shift-out (SO) characters, the double-byte characters appear
incorrectly and results are unpredictable.
Examples
To number data in the standard sequence field, enter this command:
NUMBER ON STD
To number data in both the standard and COBOL fields and include sequence numbers in the display,
enter this command:
NUMBER ON STD COBOL DISPLAY
PACK—Compress Data
The PACK primary command sets pack mode, which controls whether the data is to be stored in packed
format.
The PACK command saves the pack mode setting in the edit profile. See “Packing data” on page 15 for
more information about packing data.
Syntax
PACK
ON
OFF
ON
Saves data in packed format.
Note: You cannot specify PACK ON when editing a z/OS UNIX file.
OFF
Saves data in unpacked (standard) format.
Examples
To set pack mode on, enter this command:
PACK ON
To set pack mode off, enter this command:
PACK OFF
PACK
260  z/OS: z/OS ISPF Edit and Edit Macros

## Page 293

PASTE—Move or Copy Lines from Clipboard
The PASTE primary command moves or copies lines from a clipboard into an edit session.
Syntax
PASTE
DEFAULT
clipboard_name
AFTER
BEFORE
label
DELETE
KEEP ASIS
clipboard_name
The name of the clipboard to use. If you omit this parameter, the ISPF default clipboard (named
DEFAULT) is used. You can define up to ten additional clipboards. The size of the clipboards and
number of clipboards might be limited by installation defaults.
AFTER label
The data is copied after the line with the specified label.
BEFORE label
The data is copied before the line with the specified label.
KEEP
The copied lines are not removed from the clipboard.
DELETE
The copied lines are removed from the clipboard.
ASIS
The PASTE command determines the character set of the data in the clipboard. If this is different
to the character set being used for the file being edited an automatic conversion occurs for the data
being pasted into the file.
If ASIS is specified, then the automatic conversion does not take place.
Note:
1. You should always specify KEEP or DELETE in an edit macro because the default behavior may have
been changed by the user.
2. You can specify the default behavior (KEEP or DELETE) using the EDITSET primary command.
Description
PASTE copies or moves lines from a specified clipboard to the current edit session. If lines in the
clipboard are longer than the lines in the edit session, they are truncated.
Only the data portion of the line is saved in the clipboard. Line numbers are not saved. If the data was
CUT from a data set that had sequence numbers and is PASTEd into an edit session without sequence
numbers, or if it was CUT from a data set without sequence numbers and PASTEd into a session with
sequence numbers, some shifting of data is likely to occur.
Examples
To paste data from the default clipboard to the line after the last line in the edit session:
PASTE AFTER .ZLAST
To paste data from the default clipboard to the line after the first line in the edit session, without clearing
the contents of the clipboard:
PASTE AFTER .ZFIRST KEEP
PASTE
Chapter 10. Edit primary commands  261

## Page 294

PRESERVE—Enable Saving of Trailing Blanks
The PRESERVE primary command enables or disables the saving of trailing blanks in the editor. This gives
you the ability to override the setting for the "Preserve VB record length" field on the edit entry panel.
Syntax
PRESERVE
ON
OFF
ON
The editor preserves the record length of the record when the data is saved.
OFF
Turns truncation on. ISPF removes trailing blanks when saving variable-length files.
Regardless of the PRESERVE setting, if a line has a length of zero, ISPF saves 1 blank.
Description
PRESERVE ON causes the editor to save trailing blanks for variable length files. The number of blanks
saved for a particular record is determined by one of these:
• The original length of the record when it was read in to the editor
• The number of blanks required to pad the record length specified by the SAVE_LENGTH edit macro
command
• The length of the record that was saved on disk during a previous SAVE request in the same edit session
PRESERVE OFF causes the editor to truncate trailing blanks. If a line is empty ISPF saves 1 blank.
Use of the PRESERVE command does not prevent the editor from working on data past the specified
record length. The length set and returned by the PRESERVE command is only used when the data is
written and does not affect the operation of other edit functions.
Examples
To enable the editor to remove trailing blanks when data is saved, enter this command:
PRESERVE OFF
To save the trailing blanks, enter this command:
PRESERVE ON
PROFILE—Control and Display Your Profile
There are three forms of the PROFILE primary command:
• The control form displays your current edit profile, defines a new edit profile, or switches to a different
edit profile.
• The lock form locks or unlocks the current edit profile.
• The reset form specifies that the site-wide configuration for new edit profiles is to be used.
PRESERVE
262  z/OS: z/OS ISPF Edit and Edit Macros

## Page 295

Syntax
Profile control
PROFILE
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
The number of lines, from 0 through 9, of profile data to be displayed. When you type 0 as the number,
no profile data is displayed. When no operands are entered, the first five lines, which contain the
=PROF> flags, are always displayed. However, the =MASK> and =TABS> lines are not displayed if they
contain all blanks; if the =MASK> or =TABS> lines do contain data they are displayed, followed by the
=COLS> line.
For more information about displaying and defining a profile, see “Displaying or defining an edit profile”
on page 18.
Profile LOCK syntax
PROFILE LOCK
UNLOCK
LOCK
Specifies that the current values in the profile are saved in the edit profile table and are not modified
until the profile is unlocked. The current copy of the profile can be changed, either because of
commands you enter that modify profile values (BOUNDS and NUMBER, for example) or because of
differences in the data from the current profile settings. However, unless you unlock the edit profile,
the saved values replace the changes when you end the edit session.
CAPS, NUMBER, STATS, and PACK mode are automatically changed to fit the data. These changes
occur when the data is first read or when data is copied into the data set. Message lines (==MSG>) are
inserted in the data set to show you which changes occurred.
Note: To force CAPS, NUMBER, STATS, or PACK mode to a particular setting, use an initial macro. Be
aware, however, that if you set number mode on, data may be overlaid.
UNLOCK
Specifies that the editor saves changes to profile values.
See “Locking an edit profile” on page 19 for more information about locking and unlocking the profile.
Profile RESET syntax
PROFILE RESET
RESET
Specifies that the ZDEFAULT profile is to be removed and the site-wide configuration for new edit
profiles is to be used.
Description
PROFILE
Chapter 10. Edit primary commands  263

## Page 296

To display the current edit profile:
1. On the command line, type:
PROFILE number
2. Press Enter. The current edit profile appears.
To switch edit profiles or define a new edit profile without displaying the new profile:
1. On the command line, type:
PROFILE name 0
where name is the name of the edit profile to which you want to switch. This also specifies that no lines
are to be displayed. If you want to display the new profile, you can omit the number or enter a number
from 1 to 9.
2. Press Enter. The profile specified by the name operand becomes the active edit profile, but is not
displayed if you entered 0. If the profile does not exist, an entry is created for it in the edit profile table,
using the values of the current edit profile.
To lock the current edit profile:
1. On the command line, type:
PROFILE LOCK
2. Press Enter. The values in the current edit profile are saved in the edit profile table. From this point on,
any changes you make to the current edit profile affect only the current edit session. Values that were
saved when the current profile was locked are used the next time you begin an edit session with this
profile.
To unlock an edit profile:
1. On the command line, type:
PROFILE UNLOCK
2. Press Enter. From this point on, any changes that you make to the current edit profile replace any
values that may have been saved for this profile in the edit profile table. Also, these changes are saved
when you end the current edit session.
Examples
Figure 141 on page 265 shows a typical edit profile for a REXX data set. The display results from entering
PROFILE with no operands. The =TABS> and =MASK> lines appear because they contained data. If they
had been empty, they would not have appeared.
PROFILE
264  z/OS: z/OS ISPF Edit and Edit Macros

## Page 297

Figure 141. Edit P r o file  display
The sample profile contains:
• The first profile line (=PROF>) shows the profile name (EXEC), the data set record format and
length (FIXED - 80), and the settings for edit recovery mode (RECOVERY ON) and number mode
(NUMBER ON STD).
• The second profile line shows the settings for caps mode (CAPS ON), hexadecimal mode (HEX OFF),
nulls mode (NULLS OFF), tabs mode (TABS OFF), and UNDO mode (SETUNDO STG).
• The third profile line shows the settings for the auto modes: autosave (AUTOSAVE ON), autonum
(AUTONUM OFF), and autolist (AUTOLIST OFF). It also shows the setting for stats mode (STATS ON).
• The fourth profile line shows the lock status of the EXEC profile (PROFILE UNLOCK), the name, if any,
of the initial macro called at the beginning of the edit session (IMACRO NONE), and the settings for pack
mode (PACK OFF) and note mode (NOTE ON).
• The fifth profile line shows the current hilite status (HILITE OFF).
• The last four lines of the edit profile show the tabs settings (=TABS>), edit mask (=MASK>), bounds
settings (=BNDS>), and the column position line (=COLS>).
RCHANGE—Repeat a Change
RCHANGE repeats the change requested by the most recent CHANGE command.
Syntax
RCHANGE
Description
You can use this command to repeatedly change other occurrences of the search string. After a string
NOT FOUND message appears, the next RCHANGE issued starts at the first line of the current range for a
RCHANGE
Chapter 10. Edit primary commands  265

## Page 298

forward search (FIRST or NEXT specified) or the last line of the current range for a backward search (LAST
or PREV specified).
Note: RCHANGE is normally assigned to a program function key, although you can issue it directly from
the command line.
RECOVERY—Control Edit Recovery
RECOVERY sets edit recovery mode, which allows you to recover data after a system failure or power
outage.
Syntax
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
ON
The system creates and updates a recovery data set for each change.
OFF
The system does not create and update a recovery data set.
WARN
This operand no longer has a practical function due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
NOWARN
This operand no longer has a practical function due to a software change. However, the primary
command continues to accept the operand for compatibility reasons.
SUSP
This operand functions the same as the ON operand.
Note: When SETUNDO is enabled during installation, both the RECOVERY primary command and edit
macro command continue to accept the NOWARN and WARN keywords for compatibility reasons, but the
value is ignored. NOWARN will always be in effect.
Description
You cannot edit data recursively while you are in recovery.
Attention:
If the data set to be recovered was edited by another user before edit recovery, the changes made
by the other user will be lost if you save the recovered data.
See “Undoing edit interactions” on page 66 for more information.
To turn on edit recovery mode:
1. On the command line, type:
RECOVERY ON
RECOVERY
266  z/OS: z/OS ISPF Edit and Edit Macros

## Page 299

RECOVERY can be abbreviated REC. This command can also ensure that your edit session is not lost
due to a system failure.
2. Press Enter. The editor begins recording an audit trail of your interactions. After a system failure, the
editor uses that record to reestablish the edit session at the time of failure.
Note: For edit recovery to work properly, the data set to be recovered, the edit recovery data set, and
the edit recovery table all must exist, be cataloged, and be intact. For example, with RECOVERY on,
uncataloging a data set and then trying to recover it fails.
To turn off edit recovery mode:
1. On the command line, type:
RECOVERY OFF
2. Press Enter. The editor stops recording your interactions. Edit recovery is not available following a
system failure. When an edit session is recovered, the data is scrolled all the way to the left when the
recovery edit session begins.
See “Edit recovery” on page 38 for more information about edit recovery.
RENUM—Renumber Data Set Lines
RENUM immediately turns on number mode and renumbers all lines, starting with number 100 and
incrementing by 100. For members exceeding 10 000, the increment is less than 100.
Syntax
RENUM
REN
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
Numbers the data in the standard sequence field. This is the default for all non-COBOL data set types.
COBOL
Numbers the data in the COBOL field. This is the default for all COBOL data set types.
Attention:
If number mode is off, make sure the first 6 columns of your data set are blank before using
either the NUMBER ON COBOL or NUMBER ON STD COBOL command. Otherwise, the data
in these columns is replaced by the COBOL sequence numbers. If that happens and if edit
recovery or SETUNDO is on, you can use the UNDO command to recover the data. Or, you can
use CANCEL at any time to end the edit session without saving the data.
STD COBOL
Numbers the data in both fields.
RENUM
Chapter 10. Edit primary commands  267

## Page 300

If both STD and COBOL numbers are generated, the STD number is determined and then used as the
COBOL number. This can result in COBOL numbers that are out of sequence if the COBOL and STD
fields are not synchronized. Use RENUM to synchronize them.
DISPLAY
Causes the width of the data window to include the sequence number fields. Otherwise the width of
the window does not include the sequence number fields. When you display a data set with a logical
record length of 80 and STD numbering, the sequence numbers are not shown unless you are using a
3278 Model 5 terminal, which displays 132 characters. The editor automatically scrolls left or right, if
required, so that the left most column of the data window is the first column to appear.
Description
To renumber all lines using the standard sequence fields only:
RENUM STD
To renumber all lines using both the standard and COBOL sequence fields:
RENUM STD COBOL
To renumber all lines using the COBOL sequence fields only:
RENUM COBOL
To renumber all lines using both the standard and COBOL sequence fields and specifying that the data
window is to include the sequence number fields:
RENUM STD COBOL DISPLAY
To renumber all lines by using the standard sequence fields only and specifying that the data window is to
include the sequence number fields:
RENUM DISPLAY
Here, the DISPLAY operand is the only operand needed because STD is the default.
Examples
In Figure 142 on page 269, the line numbers are not incremented uniformly. Type RENUM on the
command line. Figure 143 on page 269 shows how the lines are renumbered after you press Enter.
RENUM
268  z/OS: z/OS ISPF Edit and Edit Macros

## Page 301

Figure 142. Member before lines are renumbered
Figure 143. Member after lines are renumbered
REPLACE—Replace Data
The REPLACE primary command replaces a sequential data set, member of a partitioned data set, or z/OS
UNIX file with data you are editing. If a member or z/OS UNIX file you want to replace does not exist, the
editor creates it. If a member you want to replace exists and the member is in a PDSE version 2 data set
that is configured for member generations, the editor creates a new generation of the member. This new
REPLACE
Chapter 10. Edit primary commands  269

## Page 302

generation becomes the current generation (also known as generation zero). The editor cannot create a
new sequential data set.
If no options are specified with the REPLACE command, the Edit/View Replace panel is displayed.
Syntax
REPLACE
REPL
REP
replace_options
replace_options
member
( member)
dsname( member)
dsname
pathname
labela labelb 1 ASCII
EBCDIC
UTF8
Notes:
1 If you don't specify the group of lines using labels, you must specify the group by using C or M line
commands.
member
The name of the member to be replaced in the partitioned data set currently being edited. If a name
of eight characters or fewer is specified and it could be a member name or a data set name, REPLACE
searches for a member name first. If no member is found, then the name is used as a data set name.
If the member does not exist, the editor creates it. If you are using a concatenated sequence of
libraries, the editor writes the member to the first library in the sequence. This operand is optional.
To replace a sequential data set or a member of a different partitioned data set, enter REPLACE
without a member operand. The editor displays the Edit Replace panel, from which you can enter the
data set name.
dsname
A partially qualified or fully qualified sequential data set you want to replace.
pathname
The pathname for a z/OS UNIX regular file to be replaced. If the file does not exist, it is created. (Also,
see “Specifying z/OS UNIX pathnames with edit primary and macro commands” on page 15.)
dsname(member)
A partially qualified or fully qualified partitioned data set and member you want to replace.
labela, labelb
Labels identifying the start and end of the group of lines to replace the member or data set.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
ASCII, EBCDIC, UTF8
When one of these keywords is supplied, if the data is using a different character set to that
designated by the keyword, the data being replaced in the external file is converted to the character
set designated by the keyword.
Description
To replace a member of a partitioned data set, a sequential data set, or a z/OS UNIX file:
1. On the command line, type one of these commands:
REPLACE
270  z/OS: z/OS ISPF Edit and Edit Macros

## Page 303

REPLACE member labela labelb
REPLACE (member) labela labelb
REPLACE dsname labela labelb
REPLACE dsname(member) labela labelb
REPLACE pathname labela labelb
The member operand is optional unless you specify the name of a partitioned data set. It represents
the name of the member that you want to replace. If you specify a data set name only, it must be a
sequential data set.
The labela and labelb operands are optional, also. They represent a pair of labels that show the first
and last lines in a group of lines used to replace the member.
If you omit the labela and labelb operands, you must specify the lines by using either the C (copy) or
M (move) line command. See the descriptions of these commands if you need more information about
them.
If you omit the labela and labelb operands, and do not enter one of the preceding line commands, a
"REPLACE Pending" message is displayed in the upper-right corner of the panel.
2. Press Enter. If you did not specify the name of a member, data set, or z/OS UNIX file, the Edit/View
Replace panel is displayed. Enter the name of the member, data set, or z/OS UNIX file to be replaced
on this panel and press Enter again. If you used either a pair of labels or a C line command, the data is
copied from the member, data set, or z/OS UNIX file that you are editing into the member, data set, or
z/OS UNIX file that you are replacing. If you used the M line command, however, the data is removed
from the member, data set, or z/OS UNIX file that you are editing and placed in the member, data set,
or z/OS UNIX file that you are replacing.
If the data set specified does not exist, ISPF prompts you to see if the data set should be created. You
can create the data set using the characteristics of the cataloged source data set as a model, or specify
the characteristics for the new data set. You can suppress this function through the ISPF configuration
table, causing any CREATE request for a nonexistent data set to fail.
See “Creating and replacing data” on page 41 for more information about the REPLACE command.
Examples
These steps show how you can replace a member when you omit the member name. These same steps
apply when you create data.
1. Type REPLACE and specify which lines you want to copy or move into the data set or member. The
example in Figure 144 on page 272 uses the MM (block move) line command to move a block of lines
from the data. 
REPLACE
Chapter 10. Edit primary commands  271

## Page 304

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(FROMDATA) - 01.02                  Columns 00001 00072
 Command ===> replace                                          Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 This line will be left in this member
 000200 This line will be left in this member
 MM0300 +----------------+
 000400 | This is the    |
 000500 | material to    |
 000600 | be created in  |
 000700 | another member |
 MM0800 +----------------+
 000900 This line will be left in this member
 001000 This line will be left in this member
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 144. Member before other member is replaced
2. When you press Enter, the Edit/View Replace panel (Figure 145 on page 272) appears. Type the name
of the member to be replaced and press Enter. A member is created when you type the name of a
member that does not already exist. The name of the member replaced in this example is DELDATA. 
   Menu  RefList  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
                               Edit/View Replace
 Command ===>                                                                  
 "Current" Data Set: USERSID.TEST(FROMDATA)
 To ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . USERSID  . . .          . . .          . . .         
    Type  . . . . MSGGEN  
    Member  . . .         
 To Other Sequential Data Set, Partitioned Data Set Member, or z/OS UNIX file:
    Name . . . . . TEST(DELDATA)                                              +
    Volume Serial . .           (If not cataloged)
 Data Set Password  . .           (If password protected)
 Enter "/" to select option              Data Conversion option                 
   Pack "Replace" Data Set                 1. EBCDIC                           
                                           2. ASCII                            
                                           3. UTF-8
 Press ENTER key to replace. Enter END command to cancel replace.
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 145. Edit/View Replace panel (ISRERPL1)
3. Figure 146 on page 273 shows the lines remaining in the data being edited after the specified lines
were moved. 
REPLACE
272  z/OS: z/OS ISPF Edit and Edit Macros

## Page 305

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(FROMDATA) - 01.03               Member DELDATA created
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000100 This line will be left in this member
 000200 This line will be left in this member
 000900 This line will be left in this member
 001000 This line will be left in this member
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 146. Member after the other member has been replaced
4. Figure 147 on page 273 shows the contents of the replaced member. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       USERSID.TEST(DELDATA) - 01.02                   Columns 00001 00072
 Command ===>                                                  Scroll ===> PAGE
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000300 +----------------+
 000400 | This is the    |
 000500 | material to    |
 000600 | be created in  |
 000700 | another member |
 000800 +----------------+
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 147. Other member replaced
RESET—Reset the Data Display
The RESET primary command can restore line numbers in the line command field when those line
numbers have been replaced by labels, pending line commands, error flags, and change flags. RESET
can also delete special lines from the display, redisplay excluded lines and excluded lines messages, and
temporarily disable the highlighting of FIND strings.
RESET
Chapter 10. Edit primary commands  273

## Page 306

Syntax
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
ALL
.ZFIRST .ZLAST
labela labelb
You can type the operands in any order. If you do not specify any operands, RESET processes all operands
except LABEL.
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
SEEK and EXCLUDE do not return the highlighting of FIND strings in this manner.
The resetting of FIND highlighting does not honor the range specified on the RESET command.
HIDE
Redisplays all "n Line(s) not Displayed" messages for excluded lines that were hidden through the
HIDE command.
LABEL
Removes labels from the line command field.
SOURCE
Revert back from ASCII editing mode to EBCDIC editing mode such that the data is not translated
from or to ASCII when displaying and receiving input from the terminal.
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
RESET
274  z/OS: z/OS ISPF Edit and Edit Macros

## Page 307

ALL
Removes all changes to the line number field.
labela, labelb
Labels identifying the start and end of the group of lines to be reset.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
Description
RESET scans every line of data. If you want to delete a small number of special lines, you can get faster
response time if you use the D (delete) line command.
Examples
To reset all lines except those that contain labels:
RESET
To reset only the lines that contain labels:
RESET LABEL
To reset only the lines that contain pending line commands:
RESET COMMAND
To reset only the lines that contain ==ERR> flags:
RESET ERROR
To reset only the lines that contain ==CHG> flags:
RESET CHANGE
To reset only the special (temporary) lines:
RESET SPECIAL
To reset only the excluded lines:
RESET EXCLUDED
To reset only the excluded lines messages:
RESET HIDE
To reset all lines between and including the .START and .STOP labels, except those that contain labels:
RESET .START .STOP
RFIND—Repeat Find
RFIND locates the search string defined by the most recent SEEK, FIND, or CHANGE command, or
excludes a line containing the search string defined by the previous EXCLUDE command.
RFIND can be used repeatedly to find other occurrences of the search string. After a "string NOT FOUND"
message is displayed, the next RFIND issued starts at the first line of the current range for a forward
RFIND
Chapter 10. Edit primary commands  275

## Page 308

search (FIRST or NEXT specified), or the last line of the current range for a backward search (LAST or
PREV specified).
Syntax
RFIND
Note: RFIND is normally assigned to a program function key, although you can issue it directly from the
command line.
RMACRO—Specify a Recovery Macro
RMACRO saves the name of a recovery macro in the edit profile.
Syntax
RMACRO name
! name
NONE
name
The name of the recovery macro to be run. The name can be preceded by an exclamation point (!) to
show that it is a program macro.
NONE
The name to prevent a recovery macro from being run.
Description
To specify the name of a recovery macro:
1. On the command line, type:
RMACRO name
where name is the name of the recovery macro that you want to run.
2. Press Enter.
See “Recovery macros” on page 109 for more information.
Examples
To define RESTART as the recovery macro, type:
RMACRO RESTART
To reset the profile with no recovery macro, type:
RMACRO NONE
SAVE—Save the Current Data
SAVE saves edited data without ending your edit session. Generally, you do not need to use SAVE if
recovery mode is on. See AUTOSAVE, CANCEL, and END for more information about saving data.
RMACRO
276  z/OS: z/OS ISPF Edit and Edit Macros

## Page 309

Syntax
SAVE
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
SAVE writes the data to the same data set from which it was retrieved unless you invoke Edit with a
concatenated sequence of partitioned data sets. In that case, the data is saved in the first library in the
concatenation sequence, regardless of from which library it came. For a sequential data set, the complete
data set is rewritten. For a partitioned data set, the member is rewritten with the same member name.
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
If both number mode and autonum mode are on, the data is automatically renumbered before it is saved.
If SAVE cannot successfully rewrite the data because of I/O errors or insufficient space, the system
displays a message in the upper-right corner of the panel, accompanied by an audible alarm, if installed.
You can then try to save the data in another data set by taking these steps:
1. Enter CREATE or REPLACE with no operand on the command line. Use CREATE only if the destination is
a member of a partitioned data set, such as an ISPF library member.
2. Type CC on the first and last data lines to specify that all lines are to be copied. Then press Enter.
3. Fill in the data set and member name of the alternate library on the Edit Create or Edit Replace panel,
and press Enter.
SAVE
Chapter 10. Edit primary commands  277

## Page 310

When a space ABEND such as D37 occurs, ISPF deallocates the data set so that you can swap to another
screen or user ID and reallocate the data set. This does not occur for data sets that were edited using the
DDNAME parameter of the EDIT service.
See “Creating and replacing data” on page 41 for more information.
Examples
To save the data in the data set or member that you are editing:
1. On the command line, type:
SAVE
2. Press Enter.
When you are editing generation zero of a member in a PDSE version 2 data set and you want the data to
be saved to the same generation (rather than create a new generation):
1. On the command line, type:
SAVE NOGEN
2. Press Enter.
SETUNDO—Set the UNDO Mode
The SETUNDO primary command determines whether the UNDO command is available and how the
history of changes should be managed.
Note: The SETUNDO command is ignored if UNDO from storage is not enabled by the installer or person
who maintains the ISPF product. For information on enabling UNDO from storage, see z/OS ISPF Planning
and Customizing.
Syntax
SETUNDO
SETU
STORAGE
KEEP
RECOVER
ON
OFF
STORAGE
Enables the saving of edit changes in storage. If the setting is changed, and the profile lines are
displayed, the profile lines show the value (SETUNDO STG) after the change. Valid abbreviations for
STORAGE are STO, STG, STOR and STORE.
KEEP
Has the same effect as STORAGE except the UNDO buffers are not cleared when a SAVE is issued.
Note: The effect of KEEP (UNDO buffers not cleared when a SAVE is issued) ceases if SETUNDO is
subsequently issued without the KEEP keyword.
RECOVER
Enables the saving of edit changes through the recovery file only. If recovery is off, it is turned on by
this command. If the setting is changed and the profile lines are displayed, the profile lines show the
value (SETUNDO REC) after the change. A valid abbreviation for RECOVER is REC.
SETUNDO
278  z/OS: z/OS ISPF Edit and Edit Macros

## Page 311

ON
The same as STORAGE.
OFF
Disables the saving of edit changes in storage. If SETUNDO OFF is specified and recovery is on, then
a state of SETUNDO RECOVER is set and UNDO is available from the recovery file. All transactions on
the storage UNDO chain are removed, and no changes before SETUNDO OFF can be undone (unless
RECOVERY ON is specified). If the setting is changed and the profile lines are displayed, the profile
lines show the value (SETUNDO OFF or SETUNDO REC) after the change.
Description
SETUNDO allows you to specify how changes you make during your edit session are to be recorded and
used by the UNDO command. UNDO can be run when either SETUNDO or RECOVERY is on. Changes can
be recorded in storage, in the recovery file, or in both places. Saving the changes in storage only is the
fastest method.
To enable recording in storage:
1. On the command line, type one of these commands:
• SETUNDO STORAGE
• SETUNDO KEEP
• SETUNDO
2. Press Enter.
The value of ON is accepted to complement the OFF state.
To use the recovery file:
1. On the command line, type:
SETUNDO RECOVER
2. Press Enter.
If RECOVERY is off, it is turned on by this command.
To turn off recording and disable the UNDO command, enter:
SETUNDO OFF
Note: If recovery is on, setting SETUNDO OFF is the same as specifying SETUNDO REC, and the recovery
file is used for UNDO.
Examples
The edit profile shown in Figure 148 on page 280 shows SETUNDO set to STORAGE and RECOVERY OFF.
SETUNDO
Chapter 10. Edit primary commands  279

## Page 312

Figure 148. SETUNDO STORAGE and RECOVERY OFF
SORT—Sort Data
The SORT primary command puts data in a specified order.
Syntax
SORT
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
Labels identifying the start and end of the group of lines to be sorted.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
X
Sorts only lines that are excluded.
NX
Sorts only lines that are not excluded.
sor t _ field 
Specifies the field to be used in sorting data. You can specify up to five sort fields using these
operands:
SORT
280  z/OS: z/OS ISPF Edit and Edit Macros

## Page 313

A
Specifies ascending order. It can either precede or follow the column specification.
D
Specifies descending order. It can either precede or follow the column specification.
start_col
Defines the starting column of the field that is to be compared. It must be within the current
boundaries.
end_col
Defines the ending column of the field that is to be compared. It must be within the current
boundaries. If it is not supplied, then the ending column is the current right boundary. For more
information on boundaries, see “Edit boundaries” on page 23.
If you specify several fields, you must specify both the starting and ending columns of each field. The
fields cannot overlap. If you supply the sort order for one field, you must supply it for all fields.
Description
SORT operates in two different modes, based on the hexadecimal mode status. If hexadecimal mode is
on, the data is ordered according to its hexadecimal representation. If hexadecimal mode is off, data is
sorted in the collating sequence defined for the national language being used.
Sorting data without operands
For SORT with no operands, the editor compares the data within the current boundaries character by
character, and then orders it line by line in the proper collating sequence. It ignores data outside the
current boundaries during both operations. Therefore only the data inside the current boundaries is
changed. Labels, excluded lines, line numbers, and change, error, and special line flags are considered
associated with the data, and therefore point to the same data fields after the sort as they did before the
sort.
For example, if you issue a CHANGE ALL that changes the first, third, and sixth lines in a data set, these
lines are flagged with the change flag, ==CHG>. If you then issue a SORT command that results in the
former lines 1, 3, and 6 becoming the first, second and third lines of the sorted file, the changed line flags
would now exist on the first, second and third lines of the sorted data set.
It is important to properly set the boundaries before issuing SORT. SORT is a powerful tool for editing data
that may be formatted in multiple columns. You can set the boundaries, for example, to the first half of a
record and sort one column of data. Then you can set the boundaries to the last half of the record and sort
a second column of data.
Limiting the SORT command
Sorting is limited to data within the current boundaries. You can specify up to five sort fields by labeling
starting and ending columns. You can also identify each field as having data sorted in either ascending or
descending order.
Optionally, you can limit sorting to a range of lines by specifying the labels of the first and last lines of the
range. You can also limit sorting to either excluded or non-excluded lines.
If you have labels or line ranges that are between the labels or line ranges specified with SORT, you can
keep SORT from rearranging them by:
• Excluding them before you enter SORT
• Using the NX operand to sort only lines that are not excluded
For more information, see the definition of the NX operand and “EXCLUDE—Exclude Lines from the
Display” on page 230.
SORT
Chapter 10. Edit primary commands  281

## Page 314

Sorting DBCS data
When sorting data that contains DBCS character strings, you must ensure that no DBCS string crosses the
boundaries. Also, all records must have the same format at the boundaries, although the format of the left
and right boundaries can differ.
If a boundary divides a DBCS character, or if all records do not have the same format at the boundaries,
the result is unpredictable.
Examples
This form of the SORT command sorts in ascending order. The start-column is the left boundary and the
end-column is the right boundary:
SORT
This form of the SORT command sorts in descending order. The start-column is the left boundary and the
end-column is the right boundary:
SORT D
This form of the SORT command sorts in ascending order. The start-column is column 5 and the end-
column is the right boundary:
SORT 5
This form of the SORT command sorts in descending order. The start-column is column 5 and the
end-column is the right boundary:
SORT 5 D
SOURCE—describe format of data
The SOURCE primary command instructs the editor to treat the source data as though it is in the specified
format and converts it from that format to the CCSID of the terminal for display purposes, although
the data remains unchanged within the file. When you input or modify data at the terminal, the editor
translates the data entered from the CCSID of the terminal to the specified format prior to storing the data
in the file.
Syntax
SOURCE character_encoding
The SOURCE ASCII primary command is not available when editing a z/OS UNIX file. Instead, use the
ASCII edit facility to have the data automatically translated from ASCII to the CCSID of the terminal.
character_encoding
The type of character encoding to be used for translating data when displaying or receiving input from
the terminal.
Valid values are:
• ASCII
See “Working with ASCII data” on page 51 for more information.
SOURCE
282  z/OS: z/OS ISPF Edit and Edit Macros

## Page 315

Examples
To set source mode to ASCII:
SOURCE ASCII
To revert back to normal mode, use the RESET command:
RESET SOURCE
STATS—Generate Library Statistics
The STATS primary command sets stats mode, which creates and maintains statistics for a member of a
partitioned data set.
Syntax
STATS
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
See “Statistics for PDS members” on page 25 for more information.
Examples
To set stats mode on:
STATS ON
To set stats mode off:
STATS OFF
SUBMIT—Submit Data for Batch Processing
STATS
Chapter 10. Edit primary commands  283

## Page 316

The SUBMIT primary command submits the member or data set you are editing (or the part of the
member or data set defined by the range of line pointers or the X or NX parameters) to be processed as a
batch job.
Syntax
SUBMIT
SUB
.ZFIRST .ZLAST
labela labelb X
NX
SUBSYS ( subsystem )
labela, labelb
Labels identifying the start and end of the group of lines to be submitted.
For more information about using labels to identify a group of lines, see “Labels and line ranges” on
page 59.
X
Submits only lines that are excluded from the display.
NX
Submits only lines that are not excluded from the display.
SUBSYS(subsystem)
Identifies the name of the emergency subsystem as supported by the TSO submit command. The
name is limited to 4 characters.
Description
The editor does not supply a job statement when you enter the SUBMIT command. You can supply job
statements as part of the data being submitted. When you supply a job statement, only the job name is
logged to the ISPF log data set to ensure the protection of sensitive data.
If the file being edited is described as ASCII or UTF-8 then the data submitted to the internal reader is
converted to EBCDIC.
ISPF uses the TSO SUBMIT command to submit the job.
Examples
To submit lines between labels .START and .END as a batch job:
SUBMIT .START .END
To submit all of the data as a batch job:
SUBMIT
To submit only non-excluded lines as a batch job:
SUBMIT NX
To submit with an emergency subsystem:
"SUBMIT SUBSYS(HASP)"
SUBMIT
284  z/OS: z/OS ISPF Edit and Edit Macros

## Page 317

TABS—Define Tabs
The TABS primary command:
• Turns tabs mode on and off
• Defines the logical tab character
• Controls the insertion of attribute bytes at hardware tab positions defined with TABS
Use PROFILE to check the setting of tabs mode and the logical tab character. See “Using tabs” on page 63
if you need more information about using tabs.
Syntax
TABS
TAB
ON STD
ALL
tab_character
OFF
ON
Turns tabs mode on, which means that logical tabs can be used to break up strings of data. This is the
default operand. If no other operands are included, all hardware tab positions (asterisks) that contain
a blank or null character are activated because STD is also a default operand. The "TABS ON STD"
message is displayed in the profile.
OFF
Turns tabs mode off, which means that logical tabs cannot be used. Attribute bytes are deleted from
all hardware tab positions, causing the Tab Forward and Tab Backward keys to ignore hardware tabs
defined on the =TABS> line. Blanked-out characters that occupy these positions reappear. The "TABS
OFF" message is displayed in the profile.
STD
Activates all hardware tab positions (asterisks) that contain a blank or null character. The editor
inserts attribute bytes, which cannot be typed over, at these positions. STD is the default operand.
You can use the Tab Forward and Tab Backward keys to move the cursor one space to the right of the
attribute bytes. The "TABS ON STD" message is displayed in the profile.
ALL
Causes an attribute byte to be inserted at all hardware tab positions. Characters occupying these
positions are blanked out and the attribute bytes cannot be typed over. The Tab Forward and Tab
Backward keys can be used to move the cursor one space to the right of these attribute bytes. The
"TABS ON ALL" message is displayed in the profile.
tab_character
Defines a single character that is not a number, letter, or command delimiter as the logical tab
character. This character is used with hardware tab definitions. The "TABS ON tab_character"
message is displayed in the profile.
You can enclose the character in quotes (' or "), although this is not necessary unless a quote or a
comma (,) is used as the tab character.
The tab_character operand causes the data string that follows the logical tab character to align itself
one space to the right of the first available hardware tab position when you press Enter. No attribute
bytes are inserted.
If no hardware tabs are defined, the editor aligns the data vertically. If software tabs are defined,
the first data string is aligned under the first software tab position and the remaining data strings are
aligned at the left boundary. If neither software nor hardware tabs are defined, the editor aligns all the
data strings at the left boundary.
TABS
Chapter 10. Edit primary commands  285

## Page 318

With the tab_character operand, the Tab Forward and Tab Backward keys ignore hardware tab
positions because no attribute bytes are inserted.
You can type the operands in any order, but keep these rules in mind:
• The tab_character and ALL operands cannot be used together, because the tab_character operand does
not allow ISPF to insert attribute bytes at tab positions, while the ALL operand does.
• The TABS primary command has no effect on software tabs. Whenever software tabs are defined, you
can always press Enter to move the cursor to a software tab position in the data, even if tabs mode is
off. Attribute bytes are not inserted at software tab positions.
Examples
Define the number sign (#) as a logical tab character by typing this command and pressing Enter:
TAB #
Now, enter the COLS line command by typing COLS in the line command field and pressing Enter. A partial
=COLS> line with positions 9 through 45 is shown in the example.
To use the logical tab character you have defined (#), you also need at least one hardware tab. For this
example, we will assume that three hardware tabs have already been defined in columns 20, 30, and 40:
=COLS> -1----+----2----+----3----+----4----+
=TABS>            *         *         *
If you then type this information on a line:
#$4237#$ 596#$  81
the data $4237 is repositioned after the first tab column, defined by an * in the =TABS line, when you
press Enter. The $ 596 is repositioned after the next tab column and so forth, as follows:
=COLS> -1----+----2----+----3----+----4----+
=TABS>            *         *         *
                   $4237     $ 596     $  81
UNDO—Reverse Last Edit Interaction
The UNDO primary command allows you to remove the data modifications of a previous interaction.
Note: The SETUNDO command is ignored if UNDO from storage is not enabled by the installer or person
who maintains the ISPF product. For information on enabling UNDO from storage, see z/OS ISPF Planning
and Customizing.
Syntax
UNDO
Description
Each time you enter UNDO, it reverses edit interactions, one at a time, in the order in which they have
been entered. To use UNDO, you must have either RECOVERY on or SETUNDO on. You can undo only
those changes made after RECOVERY or SETUNDO was turned on. SETUNDO and RECOVERY can be
specified in your edit profile. You can also use the edit macro command ISREDIT SETUNDO to turn UNDO
processing on and off. See “SETUNDO—Set UNDO Mode” on page 403 for more information.
UNDO
286  z/OS: z/OS ISPF Edit and Edit Macros

## Page 319

RECOVERY is now optional and is not required to run UNDO. Performance improves if the editor is run
with SETUNDO STORAGE and RECOVERY OFF. In this mode, non-data changes, such as setting line labels,
adding note lines, and inserting blank lines, can be undone by UNDO even if no data changes have been
made. With RECOVERY ON, only changes made after (and including) the first change to edit data can be
undone.
Note: Changes made by initial edit macros cannot be undone.
See “Understanding differences in SETUNDO processing” on page 67 for more information on the
differences between SETUNDO RECOVER and SETUNDO STORAGE processing.
Each time you press Enter, an interaction occurs between you and ISPF. If you combine line and primary
commands in one entry, ISPF considers this one interaction. Therefore, UNDO would cause all of the
commands to be reversed. ISPF also considers running edit macros that contain a combination of macro
commands and assignment statements, while entering a combination of edit line and primary commands
at the same time, as one interaction.
Profile changes, such as HEX ON, LEVEL, and CAPS, cannot be undone separately. Profile changes are
associated with the data change that came before them, and can be undone only when preceded by a
data change. The data change and the profile change are undone at the same time. For example, if you
make a change to the data, change the version number, set caps off, turn hex on, and then enter UNDO,
the version number, caps setting, and hex mode all revert to the way they existed before the data change.
The data change is also undone.
Note: UNDO is not accepted if any line commands or data changes are also specified since it would be
unclear what is to be undone.
To undo the last changes:
1. Type on the command line:
UNDO
2. Press Enter.
Note: UNDO is reset by SAVE. Once you save your data for the current edit session, you can no longer
recover any interactions made before the data was saved.
Failures in recovery processing due to I/O errors no longer terminate the UNDO function if SETUNDO
STORAGE is active. When UNDO is processed, the editor scrolls the data all the way to the left.
See “Undoing edit interactions” on page 66 for more information.
Examples
You are editing the member shown in Figure 149 on page 288 and decide to delete all of the lines. You
have type the block form of the D (DELETE) command in the line command field.
UNDO
Chapter 10. Edit primary commands  287

## Page 320

Figure 149. Member before lines are deleted
Figure 150 on page 288 shows the member after the lines have been deleted. However, you have changed
your mind and want to put the lines back again. Therefore, type UNDO on the command line.
Figure 150. Member after lines are deleted
Figure 151 on page 289 shows the member after UNDO has been entered and the deleted lines have been
restored.
UNDO
288  z/OS: z/OS ISPF Edit and Edit Macros

## Page 321

Figure 151. Member after lines have been restored
UNNUMBER—Remove Sequence Numbers
The UNNUMBER primary command sets all sequence fields to blanks, turns off number mode, and
positions the data so that column 1 is the first column displayed.
Syntax
UNNUMBER
UNNUMB
UNNUM
UNN
Description
UNNUMBER is valid only when number mode is also on. The standard sequence field, the COBOL
sequence field, or both, are blanked out. If you alter or delete sequence numbers and enter UNNUMBER
on the command line at the same time, the editor issues the message Some input data ignored and
discards the data you typed over the sequence numbers.
To set all sequence fields to blanks, turn number mode off, and position the panel so that column 1 is the
first column to appear:
UNNUMBER
Examples
You are editing the member in Figure 152 on page 290 and you want to turn off the sequence numbers.
Enter UNNUMBER on the command line.
UNNUMBER
Chapter 10. Edit primary commands  289

## Page 322

Figure 152. Member before lines are unnumbered
Figure 153 on page 290 shows the member after the sequence numbers have been turned off.
Figure 153. Member after lines are unnumbered
VERSION—Control the Version Number
The VERSION primary command allows you to change the version number assigned to a member of an
ISPF library.
VERSION
290  z/OS: z/OS ISPF Edit and Edit Macros

## Page 323

Syntax
VERSION
VERS
VER
num
num
The version number. It can be any number from 1 to 99.
Description
To change the version number of the member that you are editing:
1. On the command line, type:
VERSION num
where num is the new version number.
2. Press Enter.
See “Version and modification level numbers” on page 26, for more information about version numbers.
Examples
Version and modification level numbers are shown on the first line of an edit data display in the format
VV.MM, where VV is the version number and MM is the modification level number.
You are editing the member shown in Figure 154 on page 291 and you want to change the version number
from 01 to 02. Enter VERSION on the command line.
Figure 154. Member before version number is changed
Figure 155 on page 292 shows the member with the changed version number.
VERSION
Chapter 10. Edit primary commands  291

## Page 324

Figure 155. Member after version number is changed
VIEW—View from within an Edit Session
The VIEW primary command allows you to view a sequential data set, partitioned data set member, or
z/OS UNIX file during your current edit session.
Syntax
VIEW
member
GEN generation
member
A member of the ISPF library or other partitioned data set you are currently editing. You may enter a
member pattern to generate a member list.
generation
The generation of the member to be viewed. You may enter an absolute (positive) generation number
or a relative (negative) generation number. This parameter is valid only when the member is in a PDSE
version 2 data set that is configured for member generations.
Description
To view a data set, member, or z/OS UNIX file during your current edit session:
1. On the command line, type:
VIEW
or
VIEW member
VIEW
292  z/OS: z/OS ISPF Edit and Edit Macros

## Page 325

or
VIEW member GEN generation
Here, member represents the name of the partitioned data set you are editing, and generation
represents a generation of the member. The member and generation operands are optional.
2. Press Enter.
If you specify a member name, the current library concatenation sequence finds the member. The
member is displayed for viewing. If you specify a generation number, the specified generation of the
member displays for viewing.
If you do not specify a member name, the View Command Entry panel, which is similar to the regular
View Entry panel, appears. You can enter the name of any sequential data set, partitioned data set, or
z/OS UNIX file to which you have access. When you press Enter, the data set, member, or z/OS UNIX
file is displayed for viewing.
The editor suspends your initial edit session until the view session is complete. Viewing sessions can
be nested until you run out of storage.
3. To exit from the view session, enter the END command. The current edit session resumes.
Examples
To view member YYY of the current library concatenation:
1. On the command line, type:
VIEW YYY
2. Press enter.
VIEW
Chapter 10. Edit primary commands  293

## Page 326

VIEW
294  z/OS: z/OS ISPF Edit and Edit Macros
