# Chapter 2. Controlling the edit environment

Source file: f54em00_v3r1.md
Start page: 49
Page span: 49-72

## Page 49

Chapter 2. Controlling the edit environment
These topics describe the editing environment and how you can customize that environment to best suit
your needs:
• “What is an edit profile?” on page 17
• “Edit modes” on page 19
• “Flagged lines” on page 22
• “Edit boundaries” on page 23
• “Initial macros” on page 24
• “Application-wide macros” on page 25
• “Statistics for PDS members” on page 25
• “Version and modification level numbers” on page 26
• “Sequence numbers” on page 26
• “Enhanced and language-sensitive edit coloring” on page 28
• “Edit recovery” on page 38
ISPF defaults control much of the editing environment. However, you can use line and primary commands
to change number and statistical fields on a data display panel and to determine how the data appears.
What is an edit profile?
An edit profile controls your edit session through modes and temporary lines. These modes and lines
convert data to uppercase (caps mode), automatically renumber lines of data (autonum mode), or specify
the left and right boundaries used by other commands (=BNDS> line).
The library type (the last of the data set name qualifiers), record format (fixed or variable), or the record
length can implicitly specify an edit profile. You can choose an edit profile in three ways:
• Issue the PROFILE command with a profile name as parameter
• Fill in the Profile field on the Edit Entry panel
• Supply a PROFILE keyword and name when calling the EDIT service, such as:
ISPEXEC EDIT PROFILE(name) ...
Using edit profile types
Different kinds of data can have different edit profiles. For example, you could set up one edit profile for
COBOL programs, another edit profile for memos, and a third edit profile for test data. Your installation
determines how many different edit profiles are available to you. Typically, 25 edit profiles are available.
If you attempt to create more edit profiles than defined by your installation, the least-used edit profile is
deleted first. Locked edit profiles are not deleted unless all your edit profiles are locked. In that case, the
least-used locked edit profile is deleted first. Again, if you continue to add edit profiles, all of the unlocked
edit profiles are deleted before locked edit profiles.
You can control the use of profiles from the Edit Entry panel. If you leave the Profile Name field blank,
the profile name defaults to the data set type, which is the last qualifier in the data set name. If you type
a profile name, it overrides the data set type qualifier. In either case, if a profile of that name currently
exists, it is used. If it does not exist, a new profile is defined. The initial contents of the new profile include
the default mode settings, all-blank mask and tabs, and default bounds. To eliminate the profile lines from
your panel, use the RESET command.
Using edit profile types
© Copyright IBM Corp. 1984, 2024 17

## Page 50

When editing a z/OS UNIX file, if the file name has a suffix then the first 8 characters of the suffix are used
to identify the edit profile (any lowercase characters in the suffix are converted to uppercase). If the file
name does not have a suffix the profile name defaults to HFSPROF.
Displaying or defining an edit profile
You can display none, all, or part of an edit profile by entering the PROFILE command using this syntax:
PROFILE
name
5
number
where name is the name of the edit profile that you want to display and number is a number from 0 to 9.
Figure 5. Edit P r o file  display (ISREDDE2)
Note: See “Primary Edit panel action bar choices” on page 8 for information on the action bar choices on
this panel.
The first five lines of the edit profile (Figure 5 on page 18) are the current mode settings. The remaining
lines are the current contents of the =TABS>, =MASK>, and =BNDS> lines, with the =COLS> positioning
line. When no operands are entered, the first five lines, which contain the =PROF> flags, are always
displayed. However, the =MASK> and =TABS> lines do not appear if they contain all blanks. If the =MASK>
and =TABS> lines do contain data, they are displayed, followed by the =COLS> line.
The =BNDS> line does not appear if it contains the default boundary positions. It does appear when
the bounds are set to something other than the default, and no 'number' parameter is entered into the
PROFILE command.
Note: If enhanced edit coloring is not enabled for the edit session, the profile line displaying HILITE
status is not shown. If highlighting is available, and if you explicitly set the language, then the language
appears in RED on color terminals.
If you include the name of an existing profile, the editor immediately switches to the specified profile and
displays it.
If you include a new profile name, the editor defines a profile using the current modes, options and
temporary lines.
Displaying or defining an edit profile
18  z/OS: z/OS ISPF Edit and Edit Macros

## Page 51

The number operand controls the number of lines shown in the profile display. If you type the number 0,
the profile is not displayed. If you type a number from 1 through 8, that number of lines of the profile is
displayed. If you type the number 9, the complete profile is displayed, even if the =MASK> and =TABS>
lines are blank and the =BNDS> line contains the defaults. Because masks are ignored when using a
format name, the "=MASK>" line is not displayed by the profile command in formatted edit sessions.
Modifying an edit profile
You modify an edit profile by entering commands to set various modes, options, and temporary lines.
Whenever you change an edit profile value, ISPF saves the value (unless the edit profile is locked). The
next time you edit data using the edit profile, the data is retrieved and the environment is set up again.
This is easier than it sounds. First, there are defaults for all the modes, and, in most cases, you do
not need to change them. Second, if you decide that you want to change a mode, you just enter the
appropriate command. The edit profile is automatically changed and saved. See “Edit modes” on page
19 for more information about edit modes.
Locking an edit profile
Once you have an edit profile exactly the way you want it, you can lock it. To do this, type PROFILE LOCK
and press Enter. The edit profile is saved with all the current modes, options, and temporary lines, and it
is marked so that the saved copy of the edit profile is not changed. Usually, each time you begin an editing
session the edit profile you start with is exactly the way you locked it. The exceptions are caps, number,
stats, and pack, which are made to match the data and are noted with messages. You can change a mode
during an editing session, but if the edit profile is locked, the change affects only the current session; it
does not affect any later sessions.
If you have locked your current edit profile, you cannot change the initial macro name with IMACRO. For
information on IMACRO, see “IMACRO—Specify an Initial Macro” on page 244. For information on the
LOCK operand, see “PROFILE—Control and Display Your Profile” on page 262.
Edit modes
The edit modes control how your edit session operates. To set these modes, use the associated primary
commands. For example, if you are editing a COBOL program that is in uppercase and you want all your
input to be converted to uppercase, set caps mode on by entering CAPS ON.
The list shown here summarizes the primary commands you use to display and change your edit profile.
See Chapter 10, “Edit primary commands,” on page 191 for a complete description and for the operands
you can type with the commands.
PROFILE
Displays the current setting of each mode in this list and controls whether changes to these settings
are saved.
AUTOLIST
Controls whether a copy of the saved data is automatically stored in the ISPF list data set.
AUTONUM
Controls whether lines of data are automatically renumbered when the data is saved.
AUTOSAVE
Controls whether data is saved when you enter END.
CAPS
Controls whether alphabetic characters are stored in uppercase when the data is saved.
HEX
Controls whether data is displayed in hexadecimal format.
HILITE
Controls the use of enhanced edit color.
IMACRO
Names an edit macro used at the start of the edit session.
Modifying an edit profile
Chapter 2. Controlling the edit environment  19

## Page 52

NOTES
Controls whether tutorial notes are included in an Edit model.
NULLS
Controls whether blank spaces at the end of a line are written to the panel as blanks or nulls. The
difference is that nulls allow you to insert data; blanks do not.
NUMBER
Controls the generation of sequence numbers in a data set.
PACK
Controls whether ISPF packs (compresses) the data when it is saved.
RECOVERY
Controls the recovery of an edit session following a system failure.
SETUNDO
Controls the method of saving changes for the UNDO command.
STATS
Controls whether statistics for a data set are generated.
TABS
Controls tab settings for aligning data.
Edit profile modes
The data you edit controls four special edit profile modes. These modes are set when data is first edited
or new data is copied in.
Caps mode
The editor sets caps mode on if it detects that a member to be edited contains no lowercase
characters and sets caps mode off if the member does contain lowercase characters.
Number mode
The editor sets number mode on and changes number options if it detects that the data contains valid
sequence numbers. It sets number mode off if the data does not contain valid sequence numbers.
Pack mode
The editor sets pack mode on if the data being edited was previously saved in packed format and sets
pack mode off if the data was not previously saved in packed format.
Stats mode
The editor sets STATS mode on if the member being edited currently has ISPF statistics and sets
STATS mode off if the member did not previously have ISPF statistics.
The ISPF editor changes the special data modes even if the original edit profile of the member edit profile
is locked. However, for locked profiles, it does not save the changes to the profile.
For your convenience, the editor changes the special data modes automatically to correspond to the data.
This allows you to use the default edit profile with a single data set, even though some members may
contain programs (CAPS ON) while other members contain text (CAPS OFF). Some of the members may
have statistics to be maintained, while other members are stored without statistics. Some members may
be in packed data format, while others are in standard data format. Finally, some members may have
sequence numbers while others do not.
When the editor changes your edit profile to correspond to the data, special message lines appear. If you
want to override the change, enter the appropriate command. For example, if the editor changes caps
mode from on to off because it finds lowercase characters in the data, type CAPS ON and press Enter to
reset it.
If you have special requirements, you might not want the editor to change the special modes. You may
want to have caps mode on, even if the data contains lowercase data, or you may want to generate
statistics on output, regardless of whether the member originally had statistics. If so, you can write an
initial macro to specify how the editor is to run these special modes. You would then use IMACRO to
associate the initial macro with the edit profile. See “Initial macros” on page 24 for more information on
initial macros.
Edit modes
20  z/OS: z/OS ISPF Edit and Edit Macros

## Page 53

Edit mode defaults
ISPF saves several different edit modes in an edit profile. The user can specify the desired edit profile on
the Edit Entry Panel. If the Profile field is left blank, the data set type is used as the profile name.
To preinitialize a set of edit profiles for first-time users, do these steps:
1. Start ISPF.
2. Select the Edit option.
3. Set the edit profile with the defaults you chose.
For example, to set "COBOL FIXED 80" in your profile, edit a member of a partitioned data set that has
a RECFM of F or FB, a LRECL of 80, and a type qualifier of COBOL (or enter COBOL as the profile name
on the Edit Entry Panel).
ISPF provides two methods for setting defaults for new edit profiles. You can set up a profile called
ZDEFAULT in the ISPTLIB concatenation, or you can modify the edit profile defaults in the ISPF
configuration table. The ISPF configuration table method is recommended because it is easier to maintain
than the ZDEFAULT method. The ZDEFAULT method can still be used by individual users.
Site-wide Edit Profile Initialization
When no ZDEFAULT profile exists in the ISPTLIB concatenation and the user has no edit profile member in
the ISPPROF concatenation, new edit profiles are created based on the settings in the ISPF configuration
table.
Attention: Be very careful if you override the IMACRO setting. When a setting is forced the editor
WILL CHANGE the users' profiles. For this reason it is usually better to use the site-wide initial
macro than to force the initial macro in each user's profile.
Using the configuration table, you can change any of the defaults for new edit profiles and you can
override (force) settings for PACK, RECOVERY, SETUNDO, STATS, and IMACRO in existing profiles.
It is helpful to understand when the ZDEFAULT profile is used and where it exists in a user's
concatenations. The ZEDFAULT profile exists as a row of the edit profile table named xxxEDIT, where
xxx is the application profile.
If ZDEFAULT exists in the edit profile table in the ISPTLIB concatenation, and the user has NO edit
profile table in the ISPPROF allocation, the ZDEFAULT profile is copied from ISPTLIB into the user's
edit profile when the user's edit profile is created. Therefore, many of your existing users might already
have a ZDEFAULT profile in their edit profile. Individual users can delete their ZDEFAULT profiles using
the PROFILE RESET command from within an edit session. Doing so allows them to use the site-wide
configuration for new profiles. You can also use a site-wide edit initial macro to issue a PROFILE RESET for
all users. ISPF does not ship any edit profiles.
Note: If you use the force settings such as PACK OFF, edit macro commands that attempt to change
forced settings will not receive a failing return code, but the settings will not change.
Creating a ZDEFAULT edit profile
Set up a special edit profile named ZDEFAULT (enter ZDEFAULT as the profile name on the Edit Entry
Panel). The ZDEFAULT profile is the one used for the initial settings whenever a new edit profile is
generated, regardless of the RECFM and LRECL values. For example, if you do not have an ASM profile
and you edit an ASM data set, an ASM profile is generated using ZDEFAULT for the initial settings.
If no ZDEFAULT profile exists, one is automatically generated with settings obtained from the ISPF
Configuration Table. This list shows an example:
Modes set on:
CAPS STATS NULLS NUMBER AUTOSAVE NOTE
Modes set off:
RECOVERY HEX TABS AUTONUM AUTOLIST PACK
Edit modes
Chapter 2. Controlling the edit environment  21

## Page 54

Profile set to:
UNLOCK
IMACRO set to:
NONE
SETUNDO set to:
STG
HILITE set to:
DEFAULT
The number of profiles you can establish also is described in the configuration table. See “Displaying or
defining an edit profile” on page 18 for more details. When you finish, exit ISPF. Your entire set of edit
profiles is saved in your profile library (referenced by ddname ISPPROF) as the ISREDIT member.
Flagged lines
Flagged lines are lines that contain highlighted flags in the line command field. These lines can be divided
into these categories:
• Changed lines
• Error lines
• Special lines
The flags in the line command field are not saved when you end an edit session.
Changed lines
==CHG>
Shows lines that were changed by a CHANGE or RCHANGE command.
Error lines
==ERR>
Shows lines in which ISPF finds an error when you enter a line command, primary command, or macro
command. For example, when you enter a CHANGE command, and there is not enough room on the
line to make the change.
Special lines
Special lines can be divided into two categories:
• Edit profile lines. The values associated with these lines are stored in your edit profile.
=PROF>
Contains the settings of the individual edit modes. This line is not saved as part of your data set or
member. See “Edit modes” on page 19 for more information.
=TABS>
Defines tab positions. This line is not saved as part of your data set or member.
=MASK>
Can contain data to be inserted into your data set or member when you use the I (insert) line
command. This line is not saved as part of your data set or member.
=BNDS>
Specifies left and right boundaries that are used by other commands. This line is not saved as part of
your data set or member.
=COLS>
Identifies the columns in a line.
The column identification line can be saved as part of the data set or member if you use the MD
(make dataline) line command to convert it to a data line.
Flagged lines
22  z/OS: z/OS ISPF Edit and Edit Macros

## Page 55

• Message lines, note lines, and information lines. These lines are not saved as part of the data set or
member unless you use the MD (make dataline) line command to convert them to data lines.
==MSG>
Message lines inform you of changes to the edit profile. These changes are caused by
inconsistencies between the data to be edited and the edit profile settings. Message lines also
warn you that the UNDO command is not available when edit recovery is off.
You can insert message lines manually by using an edit macro that contains the LINE_AFTER and
LINE_BEFORE assignment statements.
=NOTE=
Note lines display information when you insert edit models. However, these lines do not appear if
the edit profile is set to NOTE OFF.
You can insert note lines manually by using an edit macro that contains the LINE_AFTER and
LINE_BEFORE assignment statements.
======
Temporary information lines are lines you can add to provide temporary information that is not
saved with the data. They can be inserted into an edit session by using an edit macro containing the
LINE_AFTER and LINE_BEFORE assignment statements.
Edit boundaries
Boundary settings control which data in a member or data set is affected by other line, primary, and
macro commands. You can change the boundary settings by using the BOUNDS line command, primary
command, or macro command. Here are the commands that work within the column range specified by
the current boundary setting:
Line commands
<  >  (  )  O  TE  TF  TS
Primary commands
CHANGE  EXCLUDE  FIND  LEFT  RCHANGE  RFIND  RIGHT  SORT
Macro commands
CHANGE   EXCLUDE  FIND     LEFT  RCHANGE  RFIND  RIGHT   SEEK  SHIFT <
SHIFT >  SHIFT (  SHIFT )  SORT  TENTER   TFLOW  TSPLIT  USER_STATE
This column range is in effect unless you specify overriding boundaries when entering a command. See
the individual command descriptions for the effect the current bounds settings have.
If you do not explicitly set bounds, the editor uses the default bounds. These bounds change as the
number mode changes. If you have changed the bounds settings for a data set and would like to revert to
the default settings, you can use any BOUNDS command to do so. Table 3 on page 24 shows the default
bounds settings for various types of data sets:
Edit boundaries
Chapter 2. Controlling the edit environment  23

## Page 56

Table 3. Default bounds settings for data sets
RECFM Data Set Type Number Mode BNDS When
LRECL=80
BNDS Using Other
LRECL
FIXED ASM ON STD 1, 71 1, LRECL-8
OFF 1, 71 1, LRECL
COBOL OFF 1, 80 1, LRECL
ON STD 1, 72 1, LRECL-8
ON COBOL STD 7, 72 7, LRECL-8
ON COBOL 7, 80 7, LRECL
OTHER ON STD 1, 72 1, LRECL-8
OFF 1, 80 1, LRECL
VARIABLE ALL ON STD 9, record length N/A
OFF 1, record length N/A
If the default boundaries are in effect, they are automatically adjusted whenever number mode is turned
on or off. If you have changed the bounds from the default settings, they are not affected by the setting of
number mode.
If a left or right scroll request would cause the display to be scrolled 'past' a left or right bound, the
scrolling stops at the bound. A subsequent request then causes scrolling beyond the bound.
This scrolling feature is especially useful when you are working with data that has sequence numbers
in the columns to the left. It allows left and right scrolling up to (but not past) the bounds so that the
sequence numbers are normally excluded from the display.
If you specify an invalid value for either the left or right boundary when changing the current boundary
settings, the editor resets the value for that boundary to the default. These constitute invalid boundary
values:
• A right boundary value that is greater than the logical record length of a fixed-block file if the file is
unnumbered.
• A right boundary value that is greater than the logical record length-8 of a fixed-block file if the file with
standard numbers.
• A right boundary value that is greater than the logical record length-4 of a variable-block file.
• A left boundary value that is less than or equal to 8 for a variable-block file with standard numbers
• A left boundary value that is less than or equal to 6 for a file that is numbered with COBOL numbers
Initial macros
The editor runs an initial macro after the data is first read but before the data is displayed. An initial macro
can be used to do tasks such as initializing empty data sets, defining program macros, and initializing
function keys.
For example, if you want caps mode on even if the data contains lowercase data, create an initial macro
with a CAPS ON command. The editor first reads the edit profile and the data, then it sets caps mode to
correspond to the data. Next, it runs your initial macro, which overrides the edit profile setting of caps
mode.
To store an initial macro name in the edit profile, use the IMACRO command:
IMACRO initmac
See “IMACRO—Specify an Initial Macro” on page 244 for more information on the IMACRO command.
Initial macros
24  z/OS: z/OS ISPF Edit and Edit Macros

## Page 57

To execute an initial macro for the current session, use one of these methods:
• Type the macro name in the INITIAL MACRO field on the Edit Entry panel:
INITIAL MACRO ===> initmac
• Specify the initial macro name on the EDIT service call:
ISPEXEC EDIT DATASET(dsname) MACRO(initmac) ...
Once the initial macro is stored in a profile, it runs at the start of each edit session that uses the profile.
It can be overridden by an initial macro typed in the INITIAL MACRO field on the Edit Entry panel or
specified on the EDIT service call. You can type NONE in the INITIAL MACRO field to suppress the initial
macro defined in the profile.
Note:
1. If the current profile is locked, the IMACRO command cannot be run.
2. Remember that commands referencing display values (DISPLAY_COLS, DISPLAY_LINES, DOWN, LEFT,
RIGHT, UP, LOCATE) are invalid in an initial macro because no data has been displayed.
3. If the initial macro issues either an END or CANCEL command, the member is not displayed.
Application-wide macros
You can specify a macro to run at the beginning of your edit sessions by placing a variable called
ZUSERMAC in either the shared or profile pool. ZUSERMAC must contain the name of the macro and
cannot include any operands. ZUSERMAC must not be longer than 8 characters.
If ZUSERMAC exists in the profile or shared pool, the macro it specifies is run after the site-wide initial
macro, and before the initial macro specified on the edit panel, on EDIT service command, or in the edit
profile.
If you want to remove the user application-wide macro, you can issue the VERASE service to remove
ZUSERMAC from the shared or profile pool.
Statistics for PDS members
If STATS mode is on, ISPF creates and maintains statistics for partitioned data set members. The
following sections explain the effect STATS mode has on your statistics, first when you are beginning
an edit session and then when you are saving data.
• “Effect of Stats mode when beginning an edit session” on page 25
• “Effect of Stats mode when saving data” on page 26
Note: Stats mode is ignored for sequential data sets.
Included in the statistics are version and modification levels. These numbers can be useful in controlling
library members. See “Sequence number format and modification level” on page 27 for a discussion of
how the generation of statistics affects the format of sequence numbers.
Effect of Stats mode when beginning an edit session
Whenever a member is retrieved for editing, the ISPF editor checks the setting of STATS mode. ISPF does
not display any warning messages if the STATS mode and the member are consistent. For example:
• If the STATS mode is on and the member has statistics
• If the STATS mode is off and the member does not have statistics
If the STATS mode and the member are not consistent, however, ISPF displays a warning message. For
example:
Application-wide macros
Chapter 2. Controlling the edit environment  25

## Page 58

• If STATS mode is on and the member has no statistics, ISPF displays a warning message, but does not
change the STATS mode.
• If STATS mode is off and the member has statistics, ISPF automatically turns on STATS mode and
displays a message indicating the mode change.
Effect of Stats mode when saving data
If STATS mode is on when you save the member, ISPF updates the statistics, or creates statistics if the
member did not previously have them.
If STATS mode is off when you save the member, ISPF does not store any statistics; any previous statistics
are destroyed.
Stats mode is saved in the edit profile.
Version and modification level numbers
Two of the statistics that the editor creates and maintains for members of ISPF libraries and partitioned
data sets (when STATS mode is on) are the version and modification level numbers. These numbers are
displayed in the form VV.MM at the top of the edit panel following the data set name.
When the editor creates statistics for a new member, the default version and modification level numbers
are 01 and 00, respectively. Otherwise, the values are taken from the previous statistics stored with the
member.
You can change the version number with the VERSION command.
The modification level number appears in the last 2 digits of the line numbers for new or changed lines
to provide a record of activity. The number is automatically incremented by one when the first change is
made to the data. It can also be changed explicitly with the LEVEL command. The numbers for both can
range from 00 to 99, inclusive. After the modification level number reaches 99, it does not increment by
one to return to level 00.
The editor normally increments the modification level the first time that data is changed. This
incrementing is suppressed if:
• You have set the modification level with a LEVEL command before making the first change.
• Statistics did not previously exist, and the editor has set the modification level to 0 for a new member.
If both STATS mode and standard sequence number mode are on, the current modification level replaces
the last two positions of the sequence number for any lines that are changed. At the time the data is
saved, it is also stored for any lines that already are marked with a modification level higher than the
current modification level. If you type LEVEL 0, press Enter, and then save the data, all lines are reset to
level 0. See “LEVEL—Specify the Modification Level Number” on page 245 for more information.
Sequence numbers
Each line on the panel represents one data record. You can generate and control the numbering of lines in
your data with these commands:
AUTONUM
Automatically renumbers data whenever it is saved, preserving the modification level record.
NUMBER
Turns number mode on or off, and selects the format.
RENUM
Renumbers all lines, preserving the modification level number.
UNNUMBER
Turns off numbering and blanks the sequence number fields on all lines. This deletes all modification
level records.
Version and modification level numbers
26  z/OS: z/OS ISPF Edit and Edit Macros

## Page 59

Sequence number format and modification level
Sequence numbers can be generated in the standard sequence field, the COBOL sequence field, or both:
• The standard sequence field  is the last 8 characters for fixed-length records, or the first 8 characters
for variable-length records, regardless of the programming language. Use NUMBER ON STD to generate
sequence numbers in the standard sequence field.
For members of partitioned data sets, the format of standard sequence numbers depends on whether
statistics are being generated. If statistics are being generated, standard sequence numbers are 6 digits
followed by a 2-digit modification level number. The level number flag reflects the modification level
of the member when the line was created or last changed. If, for example, a sequence number field
contains 00040002, the line was added or last changed at modification level 02. The sequence number
is 000400.
If STATS mode is off, or if you are editing a sequential data set, standard sequence numbers are 8 digits,
right-justified within the field.
• The COBOL sequence field  is always the first 6 characters of the data and is valid only for fixed-length
records. Use the NUMBER ON COBOL or NUMBER ON STD COBOL to generate COBOL sequence
numbers.
Attention:
If number mode is off, make sure the first 6 columns of your data set are blank before using
either the NUMBER ON COBOL or NUMBER ON STD COBOL command. Otherwise, the data in
these columns is replaced by the COBOL sequence numbers. If that happens and if edit recovery
or SETUNDO is on, you can use the UNDO command to recover the data. Or, you can use CANCEL
at any time to end the edit session without saving the data. COBOL sequence numbers are
always 6 digits and are unaffected by the setting of STATS mode.
Sequence numbers usually start at 100 and are incremented by 100. When lines are inserted, the tens
or units positions are used. If necessary, one or more succeeding lines are automatically renumbered to
keep the sequence numbers in order.
Sequence number display
For numbered data, the line command field displayed to the left of each line duplicates the sequence
number in the data. Normally, the editor automatically scrolls left or right to avoid showing the data
columns that contain the sequence numbers. However, you can explicitly scroll left or right to display the
sequence numbers. The DISPLAY operand of the NUMBER and RENUMBER commands also causes the
editor to display the sequence numbers.
For example, assume that the data has COBOL numbers in columns 1 through 6 and the number mode
is NUMBER ON COBOL. When the data is displayed, column 7 is the first column displayed. If you change
number mode to NUMBER OFF, the data is scrolled so that column 1 is the first column displayed. If
you then change number mode to NUMBER ON, the data is scrolled back to column 7. But if you change
number mode to NUMBER ON DISPLAY, the sequence numbers in columns 1 through 6 remain displayed.
The sequence numbers in columns 1 through 6 become part of the data window, but cannot be modified.
Initialization of number mode
When you retrieve data for editing, the editor determines whether it contains sequence numbers. The
editor always examines the standard sequence field. It examines the COBOL sequence field if the data set
type (the lowest level qualifier in the data set name) is COBOL.
If all lines contain numeric characters in either the standard or COBOL sequence field positions, or both,
and if the numbers are in ascending order, the editor assumes the data is numbered and turns on number
mode. Otherwise, the editor turns off number mode.
If the first setting of the number mode differs from the setting in the edit profile, a message indicating
that the editor has changed the mode is displayed. For new members or empty sequential data sets, the
Sequence numbers
Chapter 2. Controlling the edit environment  27

## Page 60

first setting of number mode is determined by the current edit profile. For a new edit profile, the default is
NUMBER ON for standard sequence fields, and NUMBER ON COBOL if the data set type is COBOL.
Enhanced and language-sensitive edit coloring
The editor provides language-sensitive coloring as a productivity aid for users who are editing program
source. It is used in a variety of programming languages. Some coloring enhancements are also useful for
editing data other than program source.
Note: Language-sensitive and enhanced coloring of the edit session is only available when enabled by the
installer or the person who maintains the ISPF product. For information on enabling the enhanced color
functions, see z/OS ISPF Planning and Customizing.
These enhancements allow programmers to immediately see simple programming errors, such as
mismatched quotes or parentheses, unclosed comments, and mismatched logical constructs. The
language-sensitive component allows you to take advantage of the editor's coloring capabilities for a
number of programming languages simultaneously. Enhanced coloring is also a general productivity aid,
because it improves your ability to locate text quickly.
The editor provides enhanced highlighting in these areas:
1. Programming language constructs, including:
• Keywords for each individual language
• Comments
• Quoted strings (using both single and double quotes)
• Compiler directives (C, COBOL, PL/I, and PASCAL only)
• Special characters that the user chooses
2. Language-sensitive program logic features, such as logical blocks and IF/ELSE logic.
3. Any strings that match the previous FIND operation or that would be found by an RFIND or RCHANGE
request.
4. Default color for the data area in non-program files.
5. The phrase containing the cursor in the data area.
6. Characters that have been input since the previous Enter or function key entry was pressed.
Note: Highlighting is not available for edit sessions that involve:
• Only CURSOR and FIND highlighting is valid for data sets with record lengths greater than 255
• Mixed mode edit sessions (normally used when editing DBCS data)
• Formatted data
Language support
These languages are supported for language-sensitive coloring:
• Assembler
• BookMaster®
• C
• COBOL
• HTML
• ISPF Dialog Tag Language (DTL)
• ISPF Panels (non-DTL)
• ISPF Skeletons
• JCL (Job Control Language)
• Pascal
Enhanced edit coloring
28  z/OS: z/OS ISPF Edit and Edit Macros

## Page 61

• PL/I
• REXX
• SuperC Listing
• XML
• OTHER, which includes languages that use constructs similar to PL/I, such as DO, BEGIN, END, SELECT,
and so forth. Limited support for CLIST is provided with the OTHER language. OTHER does not support
any compiler directives.
Automatic language selection
If you choose not to set the language explicitly, the editor can automatically determine the language of
the file being edited. The language is determined by looking at the first nonblank string in the file. In cases
where ambiguity exists between languages, as in the case of C and JCL (where both may start with / /)
and in the case of PL/I and REXX (where both may start with a /* comment), the last qualifier of the data
set name may be used to determine the language. The rules for automatic language recognition are as
follows:
Assembler
Asterisk in column 1 or a recognized opcode of CSECT, DSECT, MACRO, TITLE, START or COPY.
Note: *PROCESS starting in column 1 is recognized as PL/I.
BookMaster
First character is . or : in column 1.
C
Any of these:
• First string is #
• First string is / / and last qualifier of the data set name is not .CNTL, .JCL, or .ISPCTLx
• First string is /* and last qualifier of the data set name is .C
COBOL
First nonblank is a * or / in column 7.
HTML
First nonblank character is <, and the first tag in the file that is not a comment is either a <!DOCTYPE
HTML> tag or a <?HTML> tag.
ISPF DTL
First nonblank character is <, and the file is not identified as an HTML or XML file.
ISPF Panel
First string is ) in column 1, followed by a panel section name, or the first string is % in column 1.
ISPF Skeleton
) in column 1 in a file that does not seem to be a panel.
JCL
Any of these:
• / /anything followed by the word COMMAND, DD, ELSE, ELSEIF, EXEC, IF, INCLUDE, JCLLIB, JOB,
OUTPUT, PROC, SET, XMIT, or any word beginning with the characters 'MSG'
• / /* in column 1
• / / in column 1, and the last qualifier of the data set name is .CNTL, .JCL, or ISPCTLx
• Any of these starting in column 1:
*$
/*JOBPARM
/*MESSAGE
/*NETACCT
/*NOTIFY
Enhanced edit coloring
Chapter 2. Controlling the edit environment  29

## Page 62

/*OUTPUT
/*PRIORITY
/*ROUTE
/*SETUP
/*SIGNOFF
/*SIGNON
/*XEQ
/*XMIT
Pascal
First string is (*, or the first string is /* and the last qualifier of the data set name is .PASCAL.
PL/I
First string is % or /* or the first string is *PROCESS starting in column 1. The use of carriage control
characters in column one may cause PL/I detection to fail. When the last qualifier of the data
set name starts with "PL", automatic language detection is retried ignoring column one if the first
nonblank characters occur in column one, and no language can be detected. See REXX, C, and Panel
for more information.
REXX
First string is a /* comment containing REXX, or the first string is a /* comment, and the last qualifier
of the data set name is .EXEC or .REXX.
SuperC
Either of these starting in column 3 or 4:
• ISRSUPC -
• ASMFSUPC -
XML
First nonblank character is <, and the first tag in the file that is not a comment is either a <!DOCTYPE
XML> tag or a <?XML> tag.
Other
First word is PROC, CONTROL, ISPEXEC, or ISREDIT.
HILITE AUTO selects a language based on the first nonblank line, and in some cases, the last qualifier of
the data set name.
ISPF only scans up to the first 72 bytes in each line to determine the language. If the data that would
identify the language is past the 72nd column, the language may be determined incorrectly.
Language processing limitations and idiosyncracies
Because ISPF does not provide true parsing, the built-in language scanner does not operate as a syntax
checker. Keywords or built-in function names that are used as variables, and therefore not used in a
language context, will be highlighted as keywords. For example, in context sensitive languages such as
PL/I, the word 'ELSE' may be used as a variable name. ISPF treats 'ELSE' as a keyword in all cases, both
for highlighting and logic determination.
In addition, the varying implementations and release schedules of the supported languages may result in
keyword highlighting that does not reflect the latest version of the language.
Note: Nested comments are only supported when the language is REXX. When sequence numbers are in
use, the editor only highlights the editable data. The sequence numbers are shown in the overtype color.
Also, because the language scanners of edit highlighting do not provide true parsing, when an unmatched
end tag is encountered and the LOGIC option is enabled, subsequent end tags might be highlighted as
unmatched, even if they appear to be properly matched.
Recognized special symbols
Special characters can be highlighted for each specific language. The characters are only highlighted if
they are not part of another class of constructs such as a comment, a string, or a compiler directive. The
default set of characters for each language follows:
Enhanced edit coloring
30  z/OS: z/OS ISPF Edit and Edit Macros

## Page 63

Assembler
-+*/=<>&¬|:,
BookMaster
&.,!?$
C
-+*/=<>&¬|:!;|%?#[] \
COBOL
.
DTL
<>()=
HTML
<>()=
Panel
&
Skel
&?!<|>
JCL
(),|<>¬&=
Pascal
-+*/=<>&¬|:[]
PL/I
-+*/=<>&¬|:
REXX
-+*/=<>&¬|:%\
SuperC
None
XML
<>()[]=
Other
-+*/=<>&¬|:
These character sets may be changed by each user using the HILITE dialog.
Assembler
Highlighting is performed only in columns 1 through 72.
Specific keywords are not highlighted. Any word where an opcode would be expected is highlighted as a
keyword.
BookMaster
Only BookMaster tags that begin with a colon (:) are highlighted. All tags should be terminated by a
period, because ISPF highlights up to the next period. Dot control words (.xx) are never highlighted.
The keyword list supplied by the ISPF comprises the tags used to do logic matching (:xxx/:exxx). Tags that
have an optional end tag must have a matching end tag in the edited data for logical highlighting to work.
The LOGIC option highlights unmatched end tags (:exxx tags which do not have a corresponding :xxx tag)
in reverse video pink.
BookMaster tags are not checked for validity. If you specify a colon (:) as a special character to highlight,
the editor does not recognize BookMaster tags.
C
C++ comments (/ /) are recognized.
Enhanced edit coloring
Chapter 2. Controlling the edit environment  31

## Page 64

Logical highlighting highlights curly braces ({ and }).
Keywords are case-sensitive in C. Only the lowercase versions of keywords are highlighted.
COBOL
Highlighting is performed only in columns 7 through 72.
Both single quotes (') and double quotes (") are treated as unique open and close quote characters,
although some COBOL languages only specifies double quotes as string delimiters. Compiler directives
(also called compiler-directing statements) are supported for IBM SAA AD/Cycle COBOL/370 Version 1.1.
DTL, HTML, and XML
Only items in tags are highlighted. Any less than sign (<) is assumed to start a tag. This may cause
highlighting errors if the '<' symbol appears outside of a tag.
Panels and skeletons
Quoted strings are terminated at the end of a line. For the most part, ISPF does not parse panels or
skeletons. Usually any data on a line that starts with a ')' in column 1 is highlighted as a keyword.
JCL
Because automatic language determination recognizes C++ comments (/ /), JCL is recognized only if any of
these conditions is met:
• The last qualifier of the data set name is JCL, CNTL, or PROCLIB or ISPCTLx (where x is any character)
• The second nonblank 'word' of the first nonblank line is DD, JOB, EXEC, or PROC
• The second nonblank 'word' of the first nonblank line starts with 'MSG'. This is for JCL with no JOB card,
but with MSGLEVEL or MSGCLASS.
• The first three characters in the first nonblank line are / /*.
Conditional JCL logic (IF/ELSE) is highlighted, but is not supported by the LOGIC option.
When the word DATA appears as the first word in a line or statement, HILITE assumes that this is a DD
DATA statement and colors subsequent lines as in-stream data. To avoid this, ensure that DATA is not the
first word on a line by placing other keywords before it. For example, instead of coding
//DCOBA2 PROC PROG=,
//   OPTCOB='DYN',
//   DATA='DATA(24)',
//   OUT='*',
//   USER='D0000',
move the operand starting with "DATA" to the same line as the previous operand:
//DCOBA2 PROC PROG=,
//   OPTCOB='DYN', DATA='DATA(24)',
//   OUT='*',
//   USER='D0000',
PL/I
For fixed-length record format data sets, column 1 is not scanned after the first nonblank line, except to
search for *PROCESS statements.
REXX
Logic highlighting does not support a terminating semicolon in the IF expression, or a semicolon before
the THEN or ELSE instructions.
In addition, IF statements which have the THEN keyword on the following line but do not have a
continuation character at the end of the IF expression will cause highlighting errors.
Enhanced edit coloring
32  z/OS: z/OS ISPF Edit and Edit Macros

## Page 65

For example, although these statements are valid in REXX, the ELSEs will be highlighted as a mismatched
ELSEs:
     IF a=b; THEN say 'ok'; ELSE; say 'Not OK';
     IF a=b
          THEN say 'ok';
          ELSE say 'Not OK';
SuperC
Supports both ISPF SuperC (ISRSUPC) and High Level Assembler Tooklit SuperC (ASMFSUPC). Page,
column, and section headings are used to determine the different sections within a SuperC listing.
Most forms of the SuperC listing are supported, including SuperC search-for and SuperC file, line, word,
and byte compares. Both Wide and Narrow listings, with or without the printer control column, are
supported.
SuperC SRCHFOR and SRCHFORC strings are highlighted (as FIND strings) within the source section of
the listing. Other SRCHFOR and SRCHFORC statements parameters are processed and the ANYC process
option is used for case insensitivity.
No specific action is taken with any other SuperC process option or process statement.
Other
When OTHER is in effect, ISPF tries to determine if the program is a CLIST by checking for a first word
of PROC, CONTROL, ISPEXEC or ISREDIT. If ISPF determines that the data being edited is a CLIST, then
CLIST comment closure and continuation rules apply.
The HILITE command and dialog
ISPF Edit supports enhanced and language-sensitive coloring through the HILITE command. The HILITE
edit primary command is described in “HILITE—Enhanced Edit Coloring” on page 240. The HILITE edit
macro command is described in “HILITE—Enhanced Edit Coloring” on page 344.
The HILITE dialog
The HILITE dialog is shown in Figure 6 on page 34. You can display this panel by entering the HILITE
command with no operands from an edit panel, or by selecting Hilite from the Edit pull-down.
Enhanced edit coloring
Chapter 2. Controlling the edit environment  33

## Page 66

Figure 6. HILITE Initial Screen (ISREP1)
This dialog enables you to:
• Specify a language to be used for coloring, or enable automatic language detection.
• Assign colors for different language elements on a language-by-language basis or for all languages at
once.
• Enable or disable logic or parenthesis matching.
• Turn FIND coloring on or off and assign the color for FIND highlighting.
• Turn cursor coloring on or off and assign the color for cursor phrase highlighting.
• Specify special symbols to be highlighted on a language-by-language basis.
• View keyword lists for each language.
Note: Keyword lists and default highlighted symbols for each language are supplied with ISPF. IBM does
not supply facilities for adding additional languages.
However, it is possible to add or remove keywords. This facility involves assembling and link-editing an
installation-modified keyword or symbol list. The keyword and symbol lists, and directions for changing
them, are in member ISRPXASM in the IBM-supplied ISPF sample library.
HILITE initial panel action bar
Some of the functions of the HILITE dialog are provided through the action bar. The action bar choices on
the HILITE Initial panel are:
File
Restart application
Resets all settings on all panels back to the point that HILITE was invoked.
Default All Settings
Resets all settings on this panel back to the point that HILITE was invoked.
Enhanced edit coloring
34  z/OS: z/OS ISPF Edit and Edit Macros

## Page 67

Save and Exit
Saves changes and exits application.
Cancel
Ends application and discards changes.
Languages
This pull-down menu allows you to change the way that specific supported languages are highlighted,
including the symbols that are highlighted and the colors that are used for the various language
elements.
• Select a language to change the highlighting options for that language.
• Select All to change the highlighting options for all supported languages.
• Select Other to change the highlighting options for languages similar to PL/I.
• Select Default to specify the language to be used when AUTO is specified, but the language cannot
be determined.
Colors
Overtype Color
Changes the color used for typed data.
Find String Color
Changes the color used to find strings.
Cursor Phrase Color
Changes the color of the phrase which contains the cursor.
Note: On a PC, the terminal emulator can affect the color. Some terminals do not support features
such as "blink"; if this is selected with a color, another color might display.
Help
Immediately enters help panels, which include these choices:
• Overview
• HILITE command
• Supported Languages
• Automatic Language Determination
• Additional Functions
• Supported Comment Types
• FIND and CURSOR highlighting
• Logic Highlighting
• Notes relating to specific languages
Set Overtype, Find String, Cursor Phrase Color action bars
These action bar choices function as follows:
File
The File pull-down offers these choices:
Reset
Resets the settings on this panel to the values they had when the panel first appeared.
Default
Sets the values to the IBM-supplied defaults.
Save and Exit
Exits this panel. Changes will be saved when the HILITE dialog completes, unless Cancel is
specified.
Cancel
Exits this panel and discards changes.
Enhanced edit coloring
Chapter 2. Controlling the edit environment  35

## Page 68

Help
Immediately enters help panels for the HILITE command and dialog.
After selecting a specific language from the Languages pull-down on the HILITE Initial panel (Figure 6 on
page 34), Figure 7 on page 36 is displayed:
Figure 7. HILITE Language Element Specific ation  Screen (ISREPC1)
Note:
1. If the selected language supports alternate margins (such as PL/I in Figure 7 on page 36), you can
enter left and right boundaries in the Margins input field.
2. If the JCL language is selected, the Compiler Directives field in the pop-up window is replaced by a
field named "DD * and Data Lines".
3. If a field is not applicable to a language, it is supplied with *n/a*.
4. When a new color is typed in, the input field is shown in that color when you press Enter.
Edit Color Settings action bar
The Edit Color Settings action bar choices function as follows:
File
The File pull-down offers these choices:
Restart 'language'
Resets colors and symbols to the settings they had upon entry to this panel.
Defaults
Resets colors and symbols to default values.
Save and Exit
Exits this panel. Changes will be saved when the HILITE dialog completes, unless Cancel is
specified.
Cancel
Exits this panel and discards changes.
Enhanced edit coloring
36  z/OS: z/OS ISPF Edit and Edit Macros

## Page 69

View
The View pull-down choice is:
View Keywords
Displays a list of keywords for a particular language. See Figure 8 on page 37 for an example of a
Language Keyword list.
Help
Immediately enters help panels.
If no keywords exist for a given language choice, a message is displayed instead of a Language
Keyword list.
Figure 8. HILITE Language Keyword List (ISREPK)
Language Keyword List action bar
The Language Keyword List action bar choices function as follows:
File
The File pull-down choice is:
Cancel
Exit this panel. (No changes are possible on this panel.)
Help
Immediately enters help panels.
Highlighting status and the edit profile
Colors are assigned to each character in the data area when the data appears. As you type in characters,
they appear in the 'overtype' color. When Enter or a function key is pressed, the file is scanned again and
the new characters are displayed in the appropriate colors for the type of data being edited. The actual
color definitions and symbol sets for each language affect the entire ISPF session. However, only the
language, coloring type (ON/OFF status), and logic type are saved in the edit profile.
The HILITE edit profile line shows the status of edit highlighting. Figure 9 on page 38 shows some
examples. If edit highlighting is not available, the profile line is not shown. If highlighting is available, and
you explicitly set the language, the language appears in red. If you have customized the left and right
Enhanced edit coloring
Chapter 2. Controlling the edit environment  37

## Page 70

margins, the values appear in red. If you have not customized the margins, the default values for the
language are displayed.
 ....HILITE PLI LOGIC CURSOR FIND MARGINS(5,70)..........................
 or
 ....HILITE PLI LOGIC PAREN CURSOR FIND MARGINS(2,80)....................
 or
 ....HILITE COBOL CURSOR FIND............................................
 or
 ....HILITE OFF..........................................................
Figure 9. Examples of edit pr o file  lines showing HILITE options
The information shown on the PROFILE command is saved in the edit profile.
Edit recovery
Edit recovery helps you to recover data that might otherwise be lost. For example, you would use edit
recovery to re-establish the edit session at the point of failure after a power outage or system failure.
Turning recovery mode on causes the data to be written to a temporary backup file. This is independent of
whether changes have been made to the data.
You can turn on edit recovery mode by performing either of these actions:
• Entering the RECOVERY primary command:
RECOVERY ON
• Running an edit macro that contains the RECOVERY macro command:
ISREDIT RECOVERY ON
If recovery mode is on when a system crash occurs, automatic recovery takes place the next time you
attempt to use edit. Recovery mode is remembered in your edit profile.
When you begin an edit session, if there is data to recover, the Edit Recovery panel appears, shown in
Figure 10 on page 38.
Figure 10. Edit Recovery panel (ISREDM02)
Edit recovery
38  z/OS: z/OS ISPF Edit and Edit Macros

## Page 71

Note: For information about the Data Set Password field, refer to the topic about Libraries and Data Sets
in z/OS ISPF User's Guide Vol I.
If you continue with, defer, or cancel recovery and you have other data to be recovered, the Edit Recovery
panel is displayed again for the next data set. You can control the number of data sets to be recovered
with the edit recovery table, a system data set that contains entries for each level of nested editing
sessions that can be recovered. For information on changing edit recovery operands, refer to z/OS ISPF
Planning and Customizing.
You may experience B37 (space) abends on the recovery data set if the guidelines in the z/OS ISPF
Planning and Customizing have not been followed.
Note:
• You cannot recursively edit data while you are in an edit session that is the result of an edit recovery.
• Edit recovery is not supported when editing a generation other than the current generation (also
known as generation zero) of a member of a PDSE version 2 data set that is configured for member
generations.
Attention: If the data set to be recovered was edited by another user before you continue with edit
recovery, the changes made by the other user are lost if you save the data.
If you press Enter to continue editing the data set, the editor runs a recovery macro if you had previously
specified one by using the RMACRO primary or macro command. See “Recovery macros” on page 109 and
the descriptions of the RMACRO primary and macro commands for more information.
Despite edit recovery's benefits in recovering data, there are times when you might not want to use it. You
might want to turn edit recovery off in these situations:
• Operating with recovery mode off eliminates the I/O operations that maintain the recovery data and can
therefore result in improved response time.
• Besides recording actual data changes, recovery mode records temporary changes, such as excluding
lines and defining labels. These temporary changes are recorded to allow UNDO to undo other edit
interactions besides those that change data. Therefore, when edit recovery is on, the recording of both
data and temporary changes affects the amount of DASD space that is used.
You can turn off edit recovery mode by performing either of these actions:
• Entering the RECOVERY primary command:
RECOVERY OFF
• Running an edit macro that contains the RECOVERY macro command:
ISREDIT RECOVERY OFF
See Chapter 10, “Edit primary commands,” on page 191 for details on using RECOVERY.
Edit recovery
Chapter 2. Controlling the edit environment  39

## Page 72

Edit recovery
40  z/OS: z/OS ISPF Edit and Edit Macros
