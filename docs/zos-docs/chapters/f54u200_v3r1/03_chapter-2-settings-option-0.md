# Chapter 2. Settings (option 0)

Source file: f54u200_v3r1.md
Start page: 65
Page span: 65-102

## Page 65

Chapter 2. Settings (option 0)
The Settings option allows you to display and change a variety of ISPF parameters at any time during the
ISPF session. Changes remain in effect until you change the parameter again, and ISPF saves them from
session to session. This topic explains how to use the fields on the ISPF Settings panel and the action bar
choices.
If you select option 0 on the ISPF Primary Option Menu, this panel is displayed.
Figure 23. ISPF Settings panel (ISPISMMN)
This facility can also be started from any command line with the SETTINGS command, or from the
Settings choice on the Menu pull-down on any action bar where it is available. Typically, the Settings
facility should be included as an option on an application's primary option menu or as a choice on a
pull-down on an application's primary option menu.
Some of the things you can specify are:
• Terminal characteristics
• Default options for processing the ISPF list and log data sets
• Function key assignments
• Placement of command lines
• List data set characteristics
• GDDM graphic print parameters
• Keylist modifications
• Dialog Test option
Settings (option 0)
© Copyright IBM Corp. 1980, 2024 27

## Page 66

• Default colors
• Values of CUA panel elements
• Point-and-shoot color and highlight changes
• ENVIRON command options.
ISPF Settings panel fields
Figure 23 on page 27 shows the initial default ISPF Settings. The display or the field-level help indicates
the allowable alternatives for these defaults. The values shown in Figure 23 on page 27 are for an ISPF
session in the English language. The corresponding panel displayed for a non-English session is similar,
but the accepted terminal types can be different.
Select options
Each option is described in this topic. Use a slash to select an option. Blank out the slash to deselect the
option.
Command line at bottom
Specifies that the command line is to appear at the bottom of each logical screen. If you have
specified that the panel should be displayed in CUA mode, the command line placement defaults to
the bottom.
Note:
1. The default is to have the command line placement at the bottom. However, if your current
application profile table specifies ASIS, the default does not override it.
2. If you deselect this field, the command line appears as specified in the panel definition
statements. Unless indicated in the panel definition, it appears at the top of the panel.
When you select the Command line at bottom option, these changes take place:
• The command line moves to the last line of the logical screen or the line above the function keys
depending on the CUA mode setting (see Table 1 on page 29).
• Each line that follows the command line shifts up one line.
• The long message overlays the line above the new command line location.
If the command line for a table display panel has been moved to the bottom and if no alternate
placement has been specified for the long message line, the line directly above the repositioned
command line is reserved (left blank) for the display of long messages. Otherwise, if you enter
erroneous data on that line, a long message could overlay that data.
• In general, the display location of the function key definitions depends on several variables.
– If the panel display mode CUA option is not selected, and the KEYLIST option is set to OFF, the
function key definitions display on the lines immediately above the long message line.
– If the panel display mode CUA option is on, and the KEYLIST option is set to OFF, the function key
definitions display below the long message line.
– If the KEYLIST option is set to ON, and the panel definition does not contain a )PANEL statement,
the positioning of the function keys depends on the CUA mode setting.
– If the KEYLIST option is set to ON, and the panel definition contains a )PANEL statement, the
positioning of the function keys is below the long message line.
If the Panel display CUA mode option is not selected, an exception to this situation occurs when
an alternate placement for the long message line has been specified using the LMSG keyword
on the )BODY header statement. Under these circumstances, the function key definitions display
immediately above the command line.
If a conflict occurs between the placement of the function key definition lines and those that are
to display long messages, short messages, and commands, the function keys will not overlay the
Settings (option 0)
28  z/OS: z/OS ISPF User's Guide Vol II

## Page 67

command line, the line containing the long or short message field, or any line above one of these
fields. Because of this condition, function key definition lines cannot appear at all on some screens.
When using the GDDM interface to display panels, the position of a graphics field does not change if
the command line moves to the bottom of the screen.
In split-screen mode, if the top screen specifies the Command line at bottom option, the command
line is moved to the line directly above the split line, and the long message line overlays the line above
the command line. Because the placement setting is stored in the application profile pool, the setting
for each logical screen is the same unless a user is running different applications in each screen.
Panel display CUA mode
Specifies that panels be displayed in CUA mode. This selection affects how the long message line,
command line, and function keys are displayed, as described in Table 1 on page 29.
The table summarizes how the command, long message, and function key area appear on the panel
depending on whether you select the Panel display in CUA mode option. Note that the table uses the
system default to position the long message field. An alternate long message field is not defined using
the LMSG keyword on the )BODY header statement.
Table 1. CUA mode effect on panel display
Panel Characteristic CUA mode selected CUA mode not selected
Command line at bottom Long message line
Command line
Fn=name
PF n=name
Long message line
Command line
Command line at bottom
deselected Title/short message
Command line
Long message line
⋮
Fn=name
Title/short message
Command line
Long message line
⋮
PF n=name
Table 2 on page 29 summarizes the effect of CUA mode on the top-row-displayed indicator.
Table 2. CUA mode effect on top-row-displayed indicator
CUA Mode Rows Top-Row-Displayed Message Message ID
YES ALL Row x to z of y ISPZZ102
YES SCAN Row x From y ISPZZ103
NO ALL Row x of y ISPZZ100
NO SCAN Row x of y ISPZZ100
Long message in pop-up
Specifies that long messages will be displayed in a pop-up window, regardless of the .WINDOW
setting in the message source.
Tab to action bar choices
Specifies that you want to use the Tab key to move the cursor among the action bar choices.
Tab to point-and-shoot fields
Specifies that you want to use the Tab key to move the cursor through the point-and-shoot fields on a
panel.
Restore TEST/TRACE options
When you select Dialog Test facility (option 7), certain TEST and TRACE options are established that
can be different than those specified during ISPF start up. If you select Restore TEST/TRACE options,
Settings (option 0)
Chapter 2. Settings (option 0)  29

## Page 68

the TEST or TRACE values are restored to the ISPF call values when you exit dialog test. If you
deselect the field, the TEST or TRACE values are not restored when you exit dialog test.
For more information about dialog test, see Chapter 9, “Dialog test (option 7),” on page 355.
Session Manager mode
Enter a slash to indicate that the Session Manager should handle any line mode output from the
processing program.
Jump from leader dots
Enter a slash to enable the ISPF jump function from field prompts that have leader dots ( . . or ... ).
Field prompts that have the ==> will always have the jump function enabled.
If the application developer defines the NOJUMP(ON) attribute keyword on a specific input field, this
disables the "Jump from leader dots" and takes precedence over the selected Settings "Jump from
leader dots" or the configuration table setting of "YES" for "Jump from leader dots".
Edit PRINTDS Command
Enter a slash to intercept the local print request to allow you to modify the statement before the
PRINTDS command begins. For more information on editing the PRINTDS command, see the Libraries
and Data Sets topic in z/OS ISPF User's Guide Vol I.
Always show split line
Specifies that the split line in split screen mode, as seen on a 3270 display, should always be shown.
The default for this option is that the option is selected. By deselecting this option, the split line does
not display when the screen is split at the top or the bottom of the screen.
Enable EURO Sign
Enter a slash to enable the EURO sign (currency symbol). Your terminal or emulator must support the
EURO sign for this option to work.
Scroll member list
Enter a slash to specify that ISPF should scroll to the first member selected in the member list after
processing. If the Option field is deselected, automatic member list scrolling is disabled and the
cursor is placed in front of the last member selected.
Allow empty member list
Specifies whether an empty member list will be displayed for a PDS that contains no members.
Allow empty member list (nomatch)
If the 'Allow empty member list' option is set, this field specifies whether an empty list that results
from a nonmatching pattern will be displayed.
Empty member list for edit only
Specifies whether empty member list options apply to non-edit functions such as View and Browse.
Terminal characteristics
The Terminal Characteristics portion of the ISPF Settings panel allows you to specify values for the screen
format and terminal type. Each of these characteristics is described here.
Screen format
Specification of screen format applies only to 327x and 3290 terminals (or a terminal emulator set
to a mode that emulates a 327x or 3290 terminal). ISPF ignores screen format for other types of
terminal.
Data
Format is based on data width.
Std
Format is always the primary screen size.
Max
Format is always the alternate screen size.
Part
Format uses hardware partitions (3290 only)
Settings (option 0)
30  z/OS: z/OS ISPF User's Guide Vol II

## Page 69

Note:
1. Primary and alternate screen dimensions are determined by the VTAM® logmode and the
capabilities of the terminal or terminal emulator. These values can be displayed by the ISPF
ENVIRON settings panel and issuing the QUERY request.
2. ISPF supports a minimum screen size of 24 rows and 80 columns. The maximum screen width is
160 columns.
3. If you are in an Edit session or you are using the Edit service, ISPF does not allow you to change
the screen format.
Terminal type
Specify a valid terminal type. If you are using a terminal emulator, select the type of terminal that is
being emulated (usually a 3278 or 3278x).
You can select one of the standard terminal types from the list on the ISPF Settings panel (see
Figure 23 on page 27). If the selected terminal type seems to be incompatible with the current ISPF
language setting, a 'Terminal Type Warning' Message will be displayed, but the terminal type will be
accepted nevertheless.
If you want to use a custom terminal translation table that has been created for your site, select
OTHER to specify the name of the translation table. If the load of the new translation tables fails, ISPF
reverts to the previous terminal type setting.
You can also select a terminal type by using the ISPTTDEF program, as described in z/OS ISPF Dialog
Developer's Guide and Reference.
Specification of a terminal type allows ISPF to recognize valid (displayable) characters. Keep in
mind that the terminal type value that you specify to ISPF might not be the actual terminal type.
For example, if your terminal is a 3279, you specify 3278 because a 3279 terminal has the same
character set as a 3278. The keyboard character sets for the specified terminal and the actual
terminal are always compatible.
The terminal type designations in the text of this document are often the value to be specified to ISPF
rather than the actual terminal type.
This panel can also include one or more installation-dependent options for terminal type, for example:
3277KN
3277 Katakana terminals
3278CF
3278 Canadian French terminals
3278KN
3278 Katakana terminals
A 5550-3270 Kanji emulation Version 3 terminal has the same character set as a 3278 Katakana
terminal, so you should specify 3278KN as the terminal type. Also, because the 5550 running with the
Japanese 3270PC/G Version 3 or 3270 PC Version 5 has the same character set as a 3278 Katakana
terminal, in either case you should specify 3278KN as the terminal type.
The 5550 is run with the Japanese 3270PC V5 or 3270PC/G V3 emulation program. The terminal
type, set by the ISPF Settings panel, is set to 3278KN.
Print graphics parms
The Print Graphics Parms portion of the ISPF Settings panel allows you to specify to GDDM the family
printer type, device name, and aspect ratio. These parameters are described here:
Family printer type
This parameter has a default value of 2, which cannot be changed.
Device name
VTAM node name of the physical printer to which graphic display output is to be routed. This name is
supplied by your system programmer.
Settings (option 0)
Chapter 2. Settings (option 0)  31

## Page 70

Aspect ratio
How the graphics aspect ratio (relationship to displayed screen image) is to appear on the printed
output. Aspect ratio can be either of these:
0
Preserves the aspect ratio of the graphic area as displayed (Figure 24 on page 32). In other
words, the ratio of the graphic area width to its height is the same on the printed document (Figure
25 on page 33) as in the displayed view. Zero is the default value. Figure 25 on page 33 shows
how the graphic in Figure 24 on page 32 would appear if the PRINTG command were issued with
an aspect ratio of 0.
1
Preserves the positional relationship between the graphic and the alphanumeric characters
outside the graphics area. In other words, the printed graphic (Figure 26 on page 33) aligns
horizontally with characters outside the graphics area the same as it (the printed graphic) aligns in
the displayed image.
Figure 24. Screen containing graphics to be printed using PRINTG
Settings (option 0)
32  z/OS: z/OS ISPF User's Guide Vol II

## Page 71

Figure 25. Example of using aspect ratio parameter 0
 
Figure 26 on page 33 shows how the graphic in Figure 24 on page 32 would appear if the PRINTG
command were issued with an aspect ratio of 1.
Figure 26. Example of using aspect ratio parameter 1
General
The General portion of the ISPF Settings panel allows you to specify values for the input field pad and
command delimiter.
Settings (option 0)
Chapter 2. Settings (option 0)  33

## Page 72

Input field pad
Specifies a pad character that controls the initial padding of blank (unfilled) panel input fields,
including the selection panels, but not the data portion, of an Edit display. Within Edit, you control
null or blank padding with Edit commands.
The pad character specified can be a B (blank), N (nulls), or any special (non-alphanumeric) character.
Command delimiter
You can stack commands on the command line by separating them with a delimiter. The default
delimiter, the semicolon, can be changed using this option. Alphanumeric characters, the period (.),
and the equal sign (=) are not valid command delimiters. Stacking allows you to enter, for example:
===> FIND DEPT;HEX ON
which finds the characters DEPT and then displays the file at that point in hexadecimal mode.
The system variable for the delimiter is ZDEL. For more information about ZDEL, refer to the z/OS ISPF
Dialog Developer's Guide and Reference.
ISPF Settings panel action bar
The ISPF Settings panel action bar choices function as follows:
Log/List
The Log/List pull-down offers these choices:
1
Log Data set defaults. See “Log data set defaults” on page 35.
2
List Data set defaults. See “List data set defaults” on page 37.
3
List Data set characteristics. See “List data set characteristics” on page 38.
4
JCL. See “JCL” on page 40.
Function keys
The Function keys pull-down offers you these choices (see “Working with function keys and keylists
(the Function Keys action bar choice)” on page 41 for more information):
1
Non-Keylist PF Key settings. Displays the PF Key Definitions and Labels panel.
2
Keylist settings Displays the Keylist Utility for ISP pop-up.
3
Tailor function key display. Displays the Tailor Function Key Definition Display panel.
4
Show all function keys. Changes the function key display. This will be an unavailable choice if you
are currently showing all function keys.
5
Show partial function keys. Changes the function key display. This will be an unavailable choice if
you are currently showing a partial list of function keys.
6
Remove function key display. Removes function keys from your screen. This will be an
unavailable choice if you are currently not showing function keys.
7
Use private and shared. Equivalent to using the KEYLIST PRIVATE command.
8
Use only shared. Equivalent to using the KEYLIST SHARED command.
Settings (option 0)
34  z/OS: z/OS ISPF User's Guide Vol II

## Page 73

9
Disable keylists. Disables keylists. This choice is not available if you are currently running with
keylists disabled.
10
Enable keylists. Enables keylists. This choice is not available if you are currently running with
keylists enabled.
Colors
The Colors pull-down offers you these choices (see “Changing default colors (the Colors action bar
choice)” on page 52 for more information):
1
Global colors Displays the Global Color Change Utility panel.
2
CUA Attributes Displays the CUA Attribute Change Utility panel.
3
Point-and-Shoot Displays the CUA Attribute Change Utility panel, positioned on the Point-and-
Shoot panel element.
Environ
The Environ pull-down offers you these choices (see “Specifying ISPF ENVIRON settings (the Environ
action bar choice)” on page 55 for more information):
1
Environ settings Displays the ISPF ENVIRON Command Settings panel.
Identifier
The Identifier pull-down offers you these choices (see “Displaying message, system, user, panel, and
screen IDs” on page 57 for more information):
1
Message identifier Displays the Message Identifier pop-up.
2
Panel identifier Displays the Panel Identifier pop-up.
3
Screen Name Displays the Screen Name pop-up.
Help
The Help pull-down provides general information about the options available in the Settings panel and
action bar.
Specifying log and list defaults and characteristics (the Log/List action bar
choice)
The Log/List pull-down on the ISPF Settings panel action bar allows you to specify the log and list data set
defaults that are used when you terminate ISPF by issuing the RETURN or END command or by entering
an X on the ISPF Primary Option Menu.
The defaults can also be used when you issue the LOG or LIST command. You may specify the
characteristics of the records to be contained in the list data set when it is defined.
Log data set defaults
When you select "Log Data set defaults" from the Log/List pull-down on the ISPF Settings panel action
bar, the panel shown in Figure 27 on page 36 is displayed.
Settings (option 0)
Chapter 2. Settings (option 0)  35

## Page 74

Figure 27. Log Data Set Defaults panel (ISPISML1)
Local printer ID or writer name
Enter the name that your installation has assigned to an IBM 328x type of printer or the name of the
external writer program. The default is blank. If you enter a name in this field, be sure to leave the
Batch SYSOUT class field empty.
Lines per page
ISPF uses this value to determine when to cause a page eject if the eject control is not provided by the
dialog; for example, when the dialog issues a LIST service request without the CC keyword specified.
Lines per page can range from 1 to 999. The initial default is 60. Normal values for lines per page are:
60
When printing 6 lines per inch
80
When printing 8 lines per inch.
Primary/Secondary pages
Primary/secondary allocation parameters are specified in terms of the anticipated number of pages
of printout. These values are automatically converted by ISPF to the appropriate number of blocks
before allocating space for the log data set. The initial default setting is 100 for both Primary pages
and Secondary pages.
Specify a primary allocation of zero to prevent allocation and generation of the log.
If you modify the primary/secondary allocation parameters after the data set has been allocated, the
new values take effect the next time you start ISPF. The log data set is allocated the first time you
perform some action that results in a log message, such as saving edited data or submitting a batch
job.
Log Message ID
If you select the Log Message ID option, the message ID is automatically added to the long message
text written in the LOG data set.
Settings (option 0)
36  z/OS: z/OS ISPF User's Guide Vol II

## Page 75

If you request default processing options for the log data set, these rules apply:
• If you specify Print data set and delete (1), you must also specify a Batch SYSOUT class and job
statement information. If you specify Print data set and delete for both log and list, you can specify
different Batch SYSOUT classes, but only one job is submitted for printing both data sets.
• If you specify routing to a local printer, you must specify a Local printer ID or writer name, and Batch
SYSOUT must be blank. You can also enter a Local SYSOUT class if one is defined.
If you do not follow these rules or do not specify default processing options, primary option X or the
RETURN command causes the final termination panel to be displayed.
List data set defaults
When you select "List Data set defaults" from the Log/List pull-down on the ISPF Settings panel action
bar, the pop-up shown in Figure 28 on page 37 is displayed.
Figure 28. List Data Set Defaults panel (ISPISML2)
Local printer ID
Enter the name that your installation has assigned to an IBM 328x type of printer or the name of the
external writer program. The default is blank. If you enter a name in this field, be sure to leave the
Batch SYSOUT class field empty.
Lines per page
ISPF uses this value to determine when to cause a page eject if the eject control is not provided by the
dialog; for example, when the dialog issues a LIST service request without the CC keyword specified.
Lines per page can range from 1 to 999. The initial default is 60. Normal values for lines per page are:
60
When printing 6 lines per inch
80
When printing 8 lines per inch.
Settings (option 0)
Chapter 2. Settings (option 0)  37

## Page 76

Primary/Secondary pages
Primary/secondary allocation parameters are specified in terms of the anticipated number of pages
of printout. These values are automatically converted by ISPF to the appropriate number of blocks
before allocating space for the list data set. The initial default settings are 100 for Primary pages and
200 for Secondary pages.
If you modify the primary/secondary allocation parameters after the data set has been allocated, the
new values take effect the next time you enter ISPF. The list data set is allocated the first time you
request a print function or a dialog issues a LIST service request.
If you request default processing options for the list data set, these rules apply:
• If you specify Print data set and delete (1), you must also specify a Batch SYSOUT class and job
statement information. If you specify Print data set and delete for both log and list, you can specify
different Batch SYSOUT classes, but only one job is submitted for printing both data sets.
• If you specify routing to a local printer, you must specify a Local printer ID or writer name, and Batch
SYSOUT must be blank.
If you do not follow these rules or do not specify default processing options, primary option X or the
RETURN command causes the final termination panel to be displayed.
After reviewing or changing the parameters on this panel, enter the END command to return to the
previous menu.
List data set characteristics
When you select "List Data set characteristics" from the Log/List pull-down on the ISPF Settings
panel action bar, the panel shown in Figure 29 on page 39 is displayed to allow you to specify the
characteristics of the records to be contained in the list data set when it is defined. You can specify the
record format, the logical record length, and the line length to be printed. When the characteristics are
reset, their new values take effect at once unless the list data set has already been allocated. In that case,
the new values are used for the next list data set allocation. These values are saved in your user profile,
which ISPF automatically builds and maintains across sessions.
Specifications for logical record length and line length values can affect truncation of lines written to the
list data set by a LIST service request. See the description of the LIST service in z/OS ISPF Services Guide
for more information.
Settings (option 0)
38  z/OS: z/OS ISPF User's Guide Vol II

## Page 77

Figure 29. List Data Set Characteristics panel (ISPISML3)
The fields on this panel are described here:
Record Format
The record format specifies the format and characteristics of the records in the list data set. The
allowable record formats are:
FBA
Fixed-length records that contain ANSI-defined printer control characters
VBA
Variable-length records that contain ANSI-defined printer control characters.
The default setting is FBA.
Logical Record Length
The logical record length specifies the length, in bytes, of fixed-length records or the maximum length
allowed for variable-length records. The default value is 121. This value represents one ANSI-defined
control character and 120 bytes of data to be printed.
Line Length
The line length specifies the length of the logical line to be printed. If the specified line length is
greater than the logical record length of the list data set, data is truncated. The range of allowable
lengths is from 80 bytes to 160 bytes. The default value is 120.
The information supplied by the parameters allows for the printing of panels whose line lengths would
not otherwise be supported by the available printing facilities.
For example:
• If a panel to be printed is 160 bytes wide but printing capabilities allow only 132 bytes, you should
specify:
RECFM
FBA or VBA
Settings (option 0)
Chapter 2. Settings (option 0)  39

## Page 78

Line Length
130
LRECL
132 (allows for two ANSI-defined control characters).
The first page of output would contain the first 130 bytes of the panel. The second page would
contain the last 30 bytes. This technique is referred to as the cut and paste method of printing.
• If a panel to be printed is 132 bytes wide and the printer supports this line length, you should
specify:
RECFM
FBA or VBA
Line Length
132
LRECL
133 (allows for one ANSI-defined control character).
The entire panel would be printed on one page of output.
• If a panel to be printed is 80 bytes wide, ISPF uses the default values for the LIST parameters. The
entire panel would be printed on one page of output.
JCL
When you select JCL from the Log/List pull-down on the ISPF Settings panel action bar, the pop-up shown
in Figure 30 on page 40 is displayed. You can specify up to four default job statements to be used when
printing a log or list data set.
Figure 30. Log and List JCL panel (ISPISMLJ)
Settings (option 0)
40  z/OS: z/OS ISPF User's Guide Vol II

## Page 79

Working with function keys and keylists (the Function Keys action bar
choice)
The Function keys pull-down on the ISPF Settings panel action bar (see Figure 31 on page 42) allows
you to define function keys and to create, edit, delete, and view keylists.
Nearly all panels in ISPF have associated keylists, although specific keylists typically serve numerous
panels. There are several keylists used in the ISPF product panels. These keylists all start with the
characters ISR. In addition, ISPF contains some keylists that start with the characters ISP. They are not
used in any ISPF product panels, but can be used by an application if needed. Keylists are used when an
application panel contains a )PANEL statement.
To accommodate both users who require CUA-compliant keylists and those who prefer to use the
traditional ISPF function key assignments, F1-F12 are assigned CUA-compliant values, and F13-F24 are
assigned traditional ISPF values. Therefore, the user who runs in default mode (ZPRIKEYS set to UPP;
also see “Tailor function key definition display” on page 50) can retain the traditional key settings.
Note: Function keys in Edit are documented in z/OS ISPF Edit and Edit Macros. They are not CUA-
compliant.
The KEYS and KEYLIST commands have been modified to benefit the user as well. When KEYS is issued
from a panel that is not using a keylist, the PF Key Definitions and Labels panel is displayed, which allows
you to change the ZPF variable settings, as in previous versions of ISPF. However, if the keys command is
issued from a panel with an active keylist, the associated Keylist Utility panel Change panel is displayed.
The user can also control the use of keylists associated with panels using the KEYLIST command.
Specifying KEYLIST OFF causes ISPF to ignore the keylist in all logical screens running under the
application ID from which the KEYLIST OFF command was issued, and to use the ZPF variables
for controlling function keys. The KEYLIST ON command (the default) causes ISPF to recognize the
preeminence of keylists again. KEYLIST ON and OFF are equivalent to the Enable and Disable keylist
choices on the Function keys pull-downs discussed in “Keylist settings” on page 45.
ISPF default keylist
ISPKYLST is the ISPF default keylist. If you do not specify a keylist to be associated with a panel using the
KEYLIST attribute on the PANEL tag (DTL) or using the )PANEL statement, ISPF uses the keys defined for
ISPKYLST to display in the function area of the panel when it is displayed. The key settings and forms for
ISPKYLST are as shown in Table 3 on page 41.
Table 3. ISPKYLST key settings
Key Command Form
F1 HELP Short
F2 SPLIT Long
F3 EXIT Short
F9 SWAP Long
F12 CANCEL Short
F13 HELP Short
F14 SPLIT Long
F15 EXIT Short
F21 SWAP Long
F24 CANCEL Short
Settings (option 0)
Chapter 2. Settings (option 0)  41

## Page 80

ISPF default keylist for help panels
You can specify a keylist to be associated with a help panel by using the keylist attribute on the HELP tag
(DTL) or by using a )PANEL statement in your panel definition. If you do not specify a keylist, ISPF uses the
keys defined for ISPHELP to display in the function area of the help panel when it is displayed. The key
settings and forms for ISPHELP are shown in Table 4 on page 42.
Table 4. ISPHELP key settings
Key Command Form
F1 HELP Short
F2 SPLIT Long
F3 EXIT Short
F5 EXHELP Short
F6 KEYSHELP Short
F7 UP Long
F8 DOWN Long
F9 SWAP Long
F10 LEFT Long
F11 RIGHT Long
F12 CANCEL Short
Figure 31 on page 42 shows the Function keys pull-down on the ISPF Settings panel action bar. Each
pull-down choice is described following the panel.
Figure 31. Function keys pull-down on the ISPF settings panel action bar (ISPISMMN)
Settings (option 0)
42  z/OS: z/OS ISPF User's Guide Vol II

## Page 81

Non-keylist PF key settings
When you select Non-Keylist PF key settings from the Function keys pull-down on the ISPF Settings panel
action bar, the PF Key Definitions and Labels panel shown in Figure 32 on page 43 is displayed. If you
enter the KEYS command on the command line of any panel, the system displays one of two panels:
• If you are not using keylists (that is, keylists are disabled) or if there is no keylist associated with the
panel from which you enter the KEYS command, the PF Key Definitions and Labels panel shown in
Figure 32 on page 43 is displayed.
• If you are using keylists (that is, keylists are enabled) and there is a keylist associated with the panel
from which you enter the KEYS command, the Keylist Change panel shown in Figure 37 on page 48 is
displayed.
If you define your application panels using panel definition statements, use the PF Key Definitions and
Labels - Primary Keys panel to assign function keys and associated labels to ISPF commands.
Note: See “Keylist settings” on page 45 to find out how to assign function keys that are associated with a
keylist.
                         PF Key Definitions and Labels
                                                                    More:     +
 Number of PF Keys . . . 12                         Terminal type  . : 3278
 PF1 . . . HELP                                                                
 PF2 . . . SPLIT                                                               
 PF3 . . . END                                                                 
 PF4 . . . RETURN                                                              
 PF5 . . . RFIND                                                               
 PF6 . . . RCHANGE                                                             
 PF7 . . . UP                                                                  
 PF8 . . . DOWN                                                                
 PF9 . . . SWAP                                                                
 PF10  . . LEFT                                                                
 PF11  . . RIGHT                                                               
 PF12  . . RETRIEVE                                                            
 PF1  label  . .            PF2  label  . .            PF3  label  . .         
 PF4  label  . .            PF5  label  . .            PF6  label  . .         
 PF7  label  . .            PF8  label  . .            PF9  label  . .         
 PF10 label  . .            PF11 label  . .            PF12 label  . .         
 Press ENTER key to display alternate keys. Enter END command to exit.
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 32. PF Key Definitions  and Labels panel (ISPOPT3D)
Note: The panel in Figure 32 on page 43 is displayed for terminals with 12 function keys. For terminals
with 24 function keys, the first panel displayed shows the primary keys (F1-F12). When you press the
Enter key, ISPF displays a panel showing the alternate keys (F13-F24). To alternate between the two
panels, press the Enter key.
You can assign function keys to system commands, such as HELP or END, to commands that are
meaningful within a particular function or environment, such as the Edit FIND and CHANGE commands,
and to line commands, such as the Edit or dialog test I or D commands.
Before changing function key assignments, verify the terminal type selected on the ISPF Settings panel
and the number of function keys (12 or 24). For a list of valid terminal types refer to Figure 23 on page 27.
You can define or change a function key function simply by equating the key to a command. For example:
 PF9 . . . CHANGE ALL ABC XYZ
 PF12  . . PRINT
 
In the example, F9 has been equated to an Edit command, and F12 has been equated to the system-
defined PRINT command.
Settings (option 0)
Chapter 2. Settings (option 0)  43

## Page 82

If you enter a blank for any function key definition, the key is restored to its ISPF default.
A function key definition beginning with a colon ( : ) is treated as a special case. The colon is stripped off,
and the command to which the key is equated is inserted in the first input field on the line at which the
cursor is currently positioned.
A function key definition beginning with a greater-than sign ( > ) is another special case. It causes the
command to be passed to the dialog, regardless of whether the command appears in the command
tables. When an ISPF function is executing, do not press the RESET key and then attempt to enter
information or use a function key, because the results are unpredictable.
The label fields shown in Figure 32 on page 43 allow you to specify user-defined labels for the displayed
representations of function key definitions. This provides for displaying meaningful words of eight
characters or fewer, rather than the first eight, possibly meaningless, characters of a lengthy function
key definition.
If a label is not assigned, the definitions displayed for that function key consist of the first eight characters
of the function key definition.
If the label value is BLANK, the function key number and the equal sign display, but the value portion of
that function key definition displays as actual blanks. This label might be used if, for example, a function
key is not defined or if it is meaningless to the user, but the dialog developer wants each function key
number to appear sequentially in the function key definition lines.
No function key information, not even the number, appears if the label value for that key is NOSHOW.
Figure 33 on page 44 shows how the function key panel can be used to assign definitions and labels. In
this example, F4 has been equated to a TSO data management command, while F12 has been equated to
a command that requests job submission. Labels for several function keys are defined as well.
                         PF Key Definitions and Labels
                                                                    More:     +
 Number of PF Keys . . . 24                         Terminal type  . : 3278
 PF1 . . . HELP                                                                
 PF2 . . . SPLIT                                                               
 PF3 . . . END                                                                 
 PF4 . . . TSO LISTALC ST                                                      
 PF5 . . . RFIND                                                               
 PF6 . . . RCHANGE                                                             
 PF7 . . . UP                                                                  
 PF8 . . . DOWN                                                                
 PF9 . . . SWAP                                                                
 PF10  . . LEFT                                                                
 PF11  . . RIGHT                                                               
 PF12  . . TSO SUBMIT NOTIFY                                                   
 PF1  label  . .            PF2  label  . . BLANK      PF3  label  . .         
 PF4  label  . . DATASETS   PF5  label  . . FIND       PF6  label  . . CHANGE  
 PF7  label  . . NOSHOW     PF8  label  . . NOSHOW     PF9  label  . .         
 PF10 label  . .            PF11 label  . .            PF12 label  . . SUBMIT  
 Press ENTER key to display alternate keys. Enter END command to exit.
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 33. Using the PF Key Definitions  and Labels panel (ISPOPT3E)
This figure shows the function key settings that are displayed on a panel when defined using the key
definitions and labels in Figure 33 on page 44.
Settings (option 0)
44  z/OS: z/OS ISPF User's Guide Vol II

## Page 83

------------------------ EMPLOYEE SERIAL -------------- DUPLICATE NUMBER
COMMAND ===>
ENTER EMPLOYEE SERIAL BELOW:
   EMPLOYEE SERIAL ==> 598304       (MUST BE 6 NUMERIC DIGITS)
PRESS ENTER TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
PRESS END KEY (PF3) TO END THIS SESSION.
 F1=HELP      F2=          F3=END       F4=DATASETS   F5=FIND   F6=CHANGE
 F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 34. Example screen with function key definition  lines
Keylist settings
To create or change a keylist associated with your panels, or to refer to or delete a keylist help panel from
your keylist, select the " Keylist settings" choice from the Function keys pull-down on the ISPF Settings
panel action bar, or enter the KEYLIST command. The first panel displayed is similar to Figure 35 on page
45. If you Enter the KEYS command from a panel that uses a keylist, the keylist change panel for the
keylist active on the original panel is displayed.
  ┌─────────────────────────── Keylist Utility ───────────────────────────┐
─ │   File  View                                                          │ ──
  │ ────────────────────────────────────────────────────────────────────  │
  │                     Keylist Utility for ISR         Row 1 to 11 of 16 │  +
O │                                                                       │
  │ Actions:   N=New  E=Edit  V=View  D=Delete  /=None                    │
  │                                                                       │
  │    Keylist   Type                                                     │
  │    ISRHELP   SHARED                                                   │
  │    ISRHLP2   SHARED                                                   │
  │    ISRNAB    SHARED                                                   │
  │    ISRNSAB   SHARED                                                   │
  │    ISRREFL   SHARED                                                   │
  │    ISRREFO   SHARED                                                   │
  │    ISRSAB    SHARED    *** Currently active keylist ***               │
  │    ISRSCRVT  SHARED                                                   │
  │    ISRSLAPP  SHARED                                                   │
  │    ISRSNAB   SHARED                                                   │
T │    ISRSPBC   SHARED                                                   │
  │                                                                       │
  │ Command ===>                                       Scroll ===> PAGE   │
C │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward   │
  │  F9=Swap      F10=Actions   F12=Cancel                                │
F ⋘───────────────────────────────────────────────────────────────────────┘
Figure 35. Keylist Utility panel (ISPKLUP)
In Figure 35 on page 45, ISPHELP, ISPHLP2, ISPNAB, ISPSAB, ISPSNAB, ISPTEST, ISRHELP, ISRNAB,
and ISRNSAB have already been created for application ISR. ISPSAB, the currently active keylist, and
ISPSNAB are keylists for the keylist utility panels. ISPKYLST is the ISPF default keylist for any DTL
application panel or any panel defined with a )PANEL section that does not have a keylist defined.
ISPHELP is the ISPF keylist for help panels created using DTL or using a )PANEL section.
The application ID is shown on the title line of the panel (ISR in Figure 35 on page 45) and defaults to the
application ID of the keys table in which the keylist was found when the KEYLIST command was entered.
You can specify the keylist application ID on the )PANEL statement, or, if using DTL, it can be specified
when you call the ISPF conversion utility using the KEYLAPPL option on the ISPDTLC command. If the
panel does not specify an application ID, ISPF searches the currently executing application's keys table
for a keylist that has the same name as the name specified on the PANEL tag. If the keylist is not found,
and the current application ID is not ISP, ISPF searches the ISP application.
Settings (option 0)
Chapter 2. Settings (option 0)  45

## Page 84

The column marked Type indicates whether a keylist is shared or is a private copy. For information about
the KEYLIST SHARED and KEYLIST PRIVATE system commands, see the topic about Using Commands,
Function Keys, and Cursor Selection in the z/OS ISPF User's Guide Vol I. Shared keylists are created by the
ISPF DTL Conversion Utility. They cannot be deleted by the keylist utility. If a shared keylist is modified
by the keylist utility, it is saved as a private keylist copy in a table named xxxxPROF, where xxxx is the
application ID. The keylist utility now shows the keylist as a private copy. If you have issued the KEYLIST
SHARED command, you can still modify a keylist, but you cannot see the changes reflected in the function
keys until the KEYLIST PRIVATE command is issued.
Note: The keylist utility is meant for users to modify function keys for their own use. To define function
keys for all users of an application or for site-wide use, the definitions in the Dialog Tag Language should
be modified and a new xxxxKEYS table should be generated.
The Keylist Utility panel action bar choices function as follows:
File
Allows you to create a new keylist, to edit, view, or delete existing keylists, or to exit the keylist utility.
View
Enables you to display another set of keylists. These options are described in “View pull-down” on
page 50.
File pull-down
To create, edit, view, or delete a keylist, use either of these methods:
• Use a slash in the Select column to select a keylist from those displayed, then select the appropriate
choice from the File pull-down.
• Select a keylist from those displayed using one of these actions: N(New), E(Edit), V(View), or D(Delete).
If you use N(New), you are prompted for the name of the keylist you are about to create.
The choices on the File pull-down function as follows:
New
To create a keylist, enter the keylist name when prompted. You are prompted after selecting New
from the File pull-down, or after typing N next to any displayed keylist and pressing Enter. The screen
shown in Figure 36 on page 46 is displayed. 
 ┌────────────────────────────── Keylist Utility ──────────────────────────────┐
 │   File  Defaults                                                            │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                         ISR Keylist SAMPLE1 Create         Row 1 to 9 of 24 │
 │                                                                             │
 │ Make changes and then select File action bar choice.                        │
 │                                                                             │
 │ Keylist Help Panel Name . . .                                               │
 │                                                                             │
 │ Key       Definition                                 Format  Label          │
 │ F1 . . .                                                                    │
 │ F2 . . .                                                                    │
 │ F3 . . .                                                                    │
 │ F4 . . .                                                                    │
 │ F5 . . .                                                                    │
 │ F6 . . .                                                                    │
 │ F7 . . .                                                                    │
 │ F8 . . .                                                                    │
 │ F9 . . .                                                                    │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 36. Keylist Create panel (ISPKLUCR)
The Keylist Create panel action bar choices function as follows:
File
The File pull-down offers these choices:
1
Cancel. Cancels the creation of this keylist and returns to the Keylist Utility panel.
Settings (option 0)
46  z/OS: z/OS ISPF User's Guide Vol II

## Page 85

2
Save and Exit. Saves the keylist and returns to the Keylist Utility panel.
Defaults
The Defaults pull-down offers you the choice of the five default function key settings described in
Table 5 on page 47. 
Table 5. Default key settings
Condition Function key settings
No defaults No values are filled in.
Non-scrollable, no action bar  F1  HELP
 F2  SPLIT
 F3  EXIT
 F9  SWAP
 F12 CANCEL
Scrollable, no action bar  F1  HELP
 F2  SPLIT
 F3  EXIT
 F7  BACKWARD
 F8  FORWARD
 F9  SWAP
 F12 CANCEL
Non-scrollable, with action bar  F1  HELP
 F2  SPLIT
 F3  EXIT
 F9  SWAP
 F10 ACTIONS
 F12 CANCEL
Scrollable, with action bar  F1  HELP
 F2  SPLIT
 F3  EXIT
 F7  BACKWARD
 F8  FORWARD
 F9  SWAP
 F10 ACTIONS
 F12 CANCEL
If you are creating a keylist on a terminal defined to have 24 keys, the 13-24 keys are set the same
as the 1-12 keys. For example, F13 is automatically set the same as F1. HELP, EXIT, ACTIONS, and
CANCEL all have display format SHORT. SPLIT, UP, DOWN, and SWAP have display format LONG.
Edit
To edit the key definitions, display formats, and labels for a keylist, enter the keylist name when
prompted. Select a keylist with a slash and select Edit from the File pull-down, or type E next to a
keylist name and press Enter. The screen shown in Figure 37 on page 48 is displayed, showing the
existing values. 
Settings (option 0)
Chapter 2. Settings (option 0)  47

## Page 86

┌────────────────────────────── Keylist Utility ──────────────────────────────┐
 │   File                                                                      │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │ SHARED                  ISR Keylist SAMPLE1 Change         Row 1 to 9 of 24 │
 │                                                                             │
 │ Make changes and then select File action bar.                               │
 │                                                                             │
 │ Keylist Help Panel Name . . .                                               │
 │                                                                             │
 │ Key       Definition                                 Format  Label          │
 │ F1 . . .  HELP                                       SHORT   Help           │
 │ F2 . . .  SPLIT                                      LONG    Split          │
 │ F3 . . .  EXIT                                       SHORT   Exit           │
 │ F4 . . .                                                                    │
 │ F5 . . .                                                                    │
 │ F6 . . .                                                                    │
 │ F7 . . .  UP                                         LONG    Backward       │
 │ F8 . . .  DOWN                                       LONG    Forward        │
 │ F9 . . .  SWAP                                       LONG    Swap           │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 37. Keylist Change panel (ISPKLUCH)
The Keylist Change panel action bar choice functions as follows:
File
The File pull-down offers these choices:
1
Cancel. Cancels the changes to this keylist and returns to the Keylist Utility panel.
2
Save and Exit. Saves the changes to this keylist and returns to the Keylist Utility panel or the
panel from which you issued the KEYS command.
These fields appear on the Keylist Utility Change panel:
Row x to x of xx
Indicates that you must scroll the panel to access the remaining label definitions.
Keylist Help Panel Name
To refer to a help panel from this keylist, enter the help panel name in this field in this format:
• Must be 1-8 characters
• First, or only, character must be A-Z or a-z
• Remaining characters, if any, must be A-Z, a-z, 0-9
To remove a help panel name from a keylist, replace the help panel name with blanks.
Definition
If a display format or a label is specified, a definition must also be specified. Any definition is valid.
Format
The only valid display formats are:
LONG
The default. Indicates that the key label should be displayed in the function key area when the
FKA command is toggled to the first cycle after OFF.
SHORT
Indicates that the key label should be displayed in the function key area when the FKA
command is toggled to the first or second cycle after OFF. A key will display more often in the
function key area if it is given the SHORT display format.
NO
Indicates that the key label should never be displayed in the function key area, regardless of
the FKA command toggle cycle.
Settings (option 0)
48  z/OS: z/OS ISPF User's Guide Vol II

## Page 87

Label
Any label is valid. If the Label field is left blank, it will default to the definition. This will happen
only if the field is blank. If the Label field is not blank and the definition is changed, the Label field
will not change automatically.
View
To view a keylist, but not modify it, enter the keylist name when prompted. Select the keylist with a
slash, then select View from the File pull-down, or type V next to the keylist name displayed and press
Enter. The screen shown in Figure 38 on page 49 is displayed. 
 ┌────────────────────────────── Keylist Utility ──────────────────────────────┐
 │ SHARED                  ISR Keylist SAMPLE1 View          Row 1 to 11 of 24 │
 │                                                                             │
 │ The definition of the SAMPLE1 keylist is below.                             │
 │                                                                             │
 │ Keylist Help Panel Name . . : ISPSAB                                        │
 │                                                                             │
 │ Key       Definition                                Format  Label           │
 │ F1 . . .  HELP                                      SHORT   Help            │
 │ F2 . . .  SPLIT                                     LONG    Split           │
 │ F3 . . .  EXIT                                      SHORT   Exit            │
 │ F4 . . .                                                                    │
 │ F5 . . .                                                                    │
 │ F6 . . .                                                                    │
 │ F7 . . .  UP                                        LONG    Backward        │
 │ F8 . . .  DOWN                                      LONG    Forward         │
 │ F9 . . .  SWAP                                      LONG    Swap            │
 │ F10  . .                                                                    │
 │ F11  . .                                                                    │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F12=Cancel                                                   │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 38. Keylist utility view panel (ISPKLUB)
If you select View, the help panel name, key definitions, display formats, and labels are displayed but
cannot be changed.
Delete
To delete a private copy of a keylist, enter the keylist name when prompted. Select the keylist with
a slash, then select Delete from the File pull-down, or type D next to the keylist name displayed and
press Enter. The Delete Keylist Confirmation pop-up shown in Figure 39 on page 49 is displayed.
Note: Shared keylists can only be deleted using the Dialog Tag Language.
   ┌─────────────────────────── Keylist Utility ───────────────────────────┐
 ─ │   File  View                                                          │ ──
   │ ─ ┌───────── Keylist Utility ──────────┐ ───────────────────────────  │
   │   │    Delete Keylist Confirmation     │ SR         Row 2 to 12 of 17 │  +
 O │   │                                    │                              │
   │ A │ Enter "/" to select option         │ e  /=View                    │
   │   │    Confirm Delete of SAMPLE1       │                              │
   │   │                                    │                              │
   │   │                                    │                              │
   │   │  F1=Help    F2=Split   F3=Exit     │                              │
   │   │  F9=Swap   F12=Cancel              │                              │
   │   ⋘────────────────────────────────────┘                              │
   │   ┌────────────────┐                                                  │
   │   │ Delete Warning │                                                  │
   │   ⋘────────────────┘   *** Currently active keylist ***               │
   │    ISRSCRVT  SHARED                                                   │
   │    ISRSLAPP  SHARED                                                   │
   │    ISRSNAB   SHARED                                                   │
 T │    ISRSPBC   SHARED                                                   │
   │                                                                       │
   │ Command ===>                                       Scroll ===> PAGE   │
 C │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward   │
   │  F9=Swap      F10=Actions   F12=Cancel                                │
 F ⋘───────────────────────────────────────────────────────────────────────┘
Figure 39. Keylist utility with delete keylist c onfirmation  pop-up (ISPKLUP)
Use caution when deleting a keylist from an application that is currently running. If you delete a
keylist that is required by a panel in the application, an error message appears and the panel does not
display.
Settings (option 0)
Chapter 2. Settings (option 0)  49

## Page 88

Exit
Select Exit from the File pull-down to exit the keylist utility.
View pull-down
To display another set of keys on the Keylist Utility panel, select View on the action bar.
The View pull-down choices function as follows:
By current panel keylist
Displays the list of keys related to the current panel.
By current dialog keylist
Displays the list of keys related to the dialog that is currently running.
Specify a keylist application ID
Displays the list of keys for another application.
Tailor function key definition display
The Tailor Function Key Definition Display panel (shown in Figure 40 on page 50) allows you to change
the format of the function key definition lines that are displayed at the bottom of the screen. To display
this panel, perform one of these actions:
• Select the "Tailor function key display" choice from the Function keys pull-down.
• Issue the PFSHOW TAILOR command from any command line.
   ┌───────────────────────────── ISPF Settings ─────────────────────────────┐
 ─ │                 Tailor Function Key Definition Display                  │
   │                                                                         │
   │ For all terminals:                                                      │
 O │   Number of keys  . . 2  1. 12                                          │
   │                          2. 24                                          │
   │                                                                         │
   │   Keys per line . . . 1  1. Six                                         │
   │                          2. Maximum possible                            │
   │                                                                         │
   │   Primary range . . . 1  1. Lower - 1 to 12                             │
   │                          2. Upper - 13 to 24                            │
   │                                                                         │
   │ For terminals with 24 PF keys:                                          │
   │   Display set . . . . 1  1. Primary - display keys 1 to 12              │
   │                          2. Alternate - display keys 13 to 24           │
   │                          3. All - display all keys                      │
   │                                                                         │
 T │ Press ENTER key to save changes.  Enter END to save changes and exit.   │
   │                                                                         │
   │ Command ===>                                                            │
 C │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward     │
   │  F9=Swap      F12=Cancel                                                │
 F ⋘─────────────────────────────────────────────────────────────────────────┘
Figure 40. Tailor Function Key Definition  Display panel (ISPOPFA)
From the Tailor Function Key Definition Display panel you can set these function key parameters:
Number of keys
The number of function keys you specify controls the particular set of function key definitions
currently in use.
ISPF automatically determines the terminal type, screen size, and number of function keys:
• If the screen size is greater than 24 lines, but the terminal type specified implies a maximum of 24
screen lines, ISPF sets the terminal type to 3278.
• If you press a function key higher than 12, but the value specified for the number of function keys on
your terminal is 12, ISPF sets the terminal type to 3278 and the number of function keys to 24.
ISPF cannot determine the terminal type or number of function keys in these cases:
• If you switched between a 3277 and 3278 Model 2, both of which are 24-line terminals
Settings (option 0)
50  z/OS: z/OS ISPF User's Guide Vol II

## Page 89

• If you switched from a terminal with 24 function keys to a terminal with 12 function keys.
In these cases, you must inform ISPF of the terminal type and number of function keys you are using.
Otherwise, ISPF uses an incorrect character set and invalid function key definitions.
ISPF automatically sets, or changes, the number of function keys in these cases:
• If you specify 3277, ISPF initializes the number of keys to 12.
• If you specify 3278, ISPF initializes the number of keys to whatever was stored from the user's last
ISPF session. If no number is stored from a prior session, the number of keys is initialized to 12.
• If you press a function key higher than 12, ISPF sets the number of keys to 24. ISPF cannot set the
number of keys to 24 for the 3278T terminal.
Keys per line
You can specify the number of keys per line to be displayed on the function key definition lines. Six
or Maximum possible can be specified, indicating either six keys or the maximum possible keys. Six
ensures consistency across all panels. Maximum possible can save space on crowded panels. The
Maximum possible option is forced when you select the Panel display CUA mode option on the ISPF
Settings panel.
Primary range
You can specify that the primary key range be:
Lower
Primary keys are F1-F12.
Upper
Primary Keys are F13-F24.
The default value is lower.
Display set
For terminals with 24 function keys, you can choose to display only the primary set of function keys
(F1-F12, the default range), the alternate set of function keys (F13-24), or all 24 keys. Your display
choices depend on which range you specify for the Primary range option. For terminals with 12
function keys, this setting is ignored.
The Function keys pull-down
The Function keys pull-down provides choices that enable you to display the function keys in various
forms.
Choices for changing PF key definitions
You can change the PF Keys that you have defined by using one of the first three choices on the Function
Keys pull-down. Choosing "Non-keylist PF Key settings" calls the PF Key Definitions and Labels panel,
where you can assign PF keys to ISPF commands, and label them. This choice is like using the KEYS
command.
Choosing "Keylist settings" from the pull-down is like using the KEYLIST command, and the "Tailor
Function Key Display" choice calls up the Tailor Function Key Definition Display panel.
Choices for showing PF keys on the display screen
By selecting "Show all function keys", "Show partial function keys", or "Remove function key display", you
can specify that ISPF use the long form of function key display, the short form of function key display, or
no function keys, respectively.
Each of these pull-down choices has an equivalent PFSHOW and FKA command associated with it. The
commands operate as toggles; the pull-down choices become unavailable if they are the current setting.
This table explains the relationship between the pull-down choices and their related command
combinations.
Settings (option 0)
Chapter 2. Settings (option 0)  51

## Page 90

Table 6. Displaying forms of the function keys
Pull-down Choice Command Equivalent Result
Show all function keys PFSHOW
PFSHOW ON
FKA
FKA ON
Long setting; all available function
keys displayed. This is the default
setting.
Show partial function
keys PFSHOW (second time issued)
FKA (second time issued)
FKA SHORT*
Short setting; a partial listing of the
function keys displayed.
Remove function key
display PFSHOW (third time issued)
PFSHOW OFF
FKA (third time issued)
FKA OFF
No function keys displayed. If
PFSHOW or FKA is issued for a
fourth time, the display returns to
the long, or ON, setting.
Note: * The FKA SHORT command can be issued at any time to invoke the short setting.
Choices for determining who can use your PF keylist
The Function keys pull-down has two choices that are equivalent to the KEYLIST PRIVATE and KEYLIST
SHARED commands:
Use private and shared
Causes ISPF to use the keylist defined as private (equivalent to the KEYLIST PRIVATE command).
Private is the default. It is unavailable if it is the current setting.
Use only shared
Causes ISPF to use the keylist defined as shared (equivalent to KEYLIST SHARED). It is unavailable if
it is the current setting.
Choices for enabling keylists
The Function keys pull-down has two choices that are equivalent to the KEYLIST ON and KEYLIST OFF
commands:
Enable keylists
Causes ISPF to use the keylist defined with the panel (equivalent to the KEYLIST ON command).
Enable keylists is the default. It is unavailable if it is the current setting.
Disable keylists
Causes ISPF to ignore the keylist defined with the panel (equivalent to KEYLIST OFF). It is unavailable
if it is the current setting.
Changing default colors (the Colors action bar choice)
The Colors pull-down on the ISPF Settings action bar provides access to the Global Color Change Utility,
the ISPF CUA Attribute Change Utility, and the Point-and-shoot Color Change panel.
Global colors
For ISPF-supported seven-color terminals, ISPF provides the Global Color Change Utility to allow you to
globally change the current colors ISPF uses for display.
To invoke the utility appropriate for your environment, perform one of these actions:
• Select the Global colors... choice from the Colors pull-down.
• Issue the ISPF system command COLOR from any ISPF command line.
Settings (option 0)
52  z/OS: z/OS ISPF User's Guide Vol II

## Page 91

ISPF displays the Global Color Change Utility panel shown in Figure 41 on page 53.
Global color change utility
From the panel shown in Figure 41 on page 53, you can change the ISPF-defined default colors.
   ┌─────────────────────── ISPF Settings ────────────────────────┐   Help
 ─ │                 Global Color Change Utility                  │ ───────────
   │                                                              │
   │ Globally change one or more of the ISPF default colors and   │ More:     +
 O │ press ENTER to immediately see the effect. Clearing a color  │
   │ field and pressing ENTER restores the default color or       │
   │ selecting the Defaults point-and-shoot field restores all    │
   │ default colors.                                              │
   │                                                              │
   │ Enter the EXIT command to save changes or enter the CANCEL   │
   │ command to exit without saving.                              │
   │                                                              │
   │ ISPF Default Color                                           │
   │   Blue  . . . . _______                                      │
   │   Red . . . . . _______                                      │
   │   Pink  . . . . _______                                      │
   │   Green . . . . _______                                      │
   │   Turquoise . . _______                                      │
 T │   Yellow  . . . _______                                      │
   │   White . . . . _______                                      │
   │ Command ===> ____________________________________ Defaults   │
 C │  F1=Help        F2=Split       F3=Exit        F7=Backward    │
   │  F8=Forward     F9=Swap       F12=Cancel                     │ 9=Swap
 F ⋘──────────────────────────────────────────────────────────────┘
Figure 41. Global Color Change Utility panel (ISPOPT10)
Enter a new value in the color field beside the ISPF-defined default color to be changed. The valid color
choices are RED, PINK, GREEN, YELLOW, BLUE, TURQ, and WHITE.
Color changes are reflected on the panel display immediately after you press Enter. For example, if you
type BLUE in the field next to RED and press Enter, any panel element attributes defined as red change to
blue.
You can restore an ISPF-defined color to its default value by setting its field to blank and pressing Enter.
To restore all the ISPF-defined colors to their default values, select the Defaults point-and-shoot field at
the end of the command line.
The EXIT command ends the Global Color Change Utility function and saves global color changes in the
ISPSPROF system profile table. The CANCEL command ends the Global Color Change Utility function and
restores the global color definitions as they were before the utility was invoked.
Changes to the globally defined colors affect all logical screens whether they are displayed directly by
ISPF or whether ISPF has requested that GDDM perform the display. Line mode output, fields, and
graphics that the dialog has placed on the screen using direct calls to GDDM are not affected by global
color changes.
CUA cttributes
ISPF provides the CUA Attribute Change Utility to allow you to change the default values of panel colors,
intensities, and highlights for panel element attributes. See the z/OS ISPF Dialog Developer's Guide and
Reference for a description of TYPE values for CUA panel element attributes.
To invoke the ISPF CUA Attribute Change Utility, perform one of these actions:
• Select the CUA attributes... choice from the Colors pull-down.
• Issue the ISPF system command CUAATTR from any ISPF command line.
The CUA Attribute Change Utility panel shown in Figure 42 on page 54 is displayed. This is a scrollable
panel that contains the current values for CUA panel element attribute colors, intensities, and highlights.
Settings (option 0)
Chapter 2. Settings (option 0)  53

## Page 92

┌─────────────────────────────── ISPF Settings ───────────────────────────────┐
 │                        CUA Attribute Change Utility                         │
 │                                                                             │
 │ Change colors, intensities, or highlights for panel attribute elements.     │
 │ Enter the EXIT command to save changes or enter the CANCEL command to exit  │
 │ without saving. To restore the defaults for a type, clear the field and     │
 │ press Enter or select the Defaults point-and-shoot field to restore all     │
 │ default settings for all types.                                             │
 │                                                                             │
 │ Panel Element                  Color         Intensity  Highlight           │
 │                                                                More:     +  │
 │ AB Selected Choice . . . . . . YELLOW        LOW        NONE                │
 │ AB Separator Line  . . . . . . BLUE          LOW        NONE                │
 │ AB Unselected Choice . . . . . WHITE         HIGH       NONE                │
 │ Action Message Text  . . . . . RED           HIGH       NONE                │
 │ Active Window Frame  . . . . . BLUE          HIGH                           │
 │ Caution Text . . . . . . . . . YELLOW        HIGH       NONE                │
 │ Choice Entry Field . . . . . . TURQ          LOW        USCORE              │
 │ Column Heading . . . . . . . . BLUE          HIGH       NONE                │
 │ Descriptive Text . . . . . . . GREEN         LOW        NONE                │
 │ Command ===>                                                     Defaults   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F12=Cancel                                                   │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 42. CUA Attribute Change Utility panel (ISPOPT11)
You can change the default values by typing over the existing values in the table with new values. Table 7
on page 54 shows valid change values:
You can restore an attribute to its default value by setting its field to blank and pressing Enter. To restore
all the attributes to their default values, select the Defaults point-and-shoot field at the end of the
command line.
Table 7. Valid CUA attribute change values
Color Choices Intensity Choices Highlight Choices
RED HIGH NONE
PINK LOW BLINK
GREEN REVERSE
YELLOW USCORE
BLUE
TURQ
WHITE
In the CUA Attribute Change Utility table, if a field is left blank and Enter is pressed, the field defaults
to the ISPF provided CUA-defined default value. Changes made to AB Selected Choice, AB Unselected
Choice, Action Message Text, Function Keys, Informational Message Text, and Warning Message Text take
effect immediately. Changes to other panel elements are reflected in the next panel display. The values of
the panel colors, intensities, and highlights are saved across ISPF invocations in your ISPF system profile
table, ISPSPROF. The changes to the panel element values affect all logical screens.
The CUA Attribute Change Utility affects only ISPF's CUA-defined attribute keywords. For example, the
CUA Attribute Change Utility does not override this panel element attribute:
    % TYPE(OUTPUT) COLOR(RED)
Color changes made using the ISPF Global Color Change Utility override changes made using the ISPF
CUA Attribute Change Utility. For example, you can use the Global Color Change Utility and change red to
blue. You might then use the CUA Attribute Change Utility and change normal text to red. Normal text will
display as blue.
The ISPF-supported CUA-defined default values for the panel element attributes are listed in z/OS ISPF
Dialog Developer's Guide and Reference.
Settings (option 0)
54  z/OS: z/OS ISPF User's Guide Vol II

## Page 93

Point-and-shoot
The Point-and-Shoot panel element on the CUA Attribute Change Utility panel (shown in Figure 43 on
page 55) allows you to adjust the color, intensity and highlighting of point-and-shoot fields.
See the ISPF User Interface topic in the z/OS ISPF User's Guide Vol I for information on the point-and-
shoot feature.
To display this panel, positioned on the Point-and-Shoot panel element, perform one of these actions:
• Select the Point-and-Shoot... choice from the Colors pull-down.
• Issue the ISPF system command PSCOLOR from any ISPF command line.
 ┌─────────────────────────────── ISPF Settings ───────────────────────────────┐
 │                        CUA Attribute Change Utility                         │
 │                                                                             │
 │ Change colors, intensities, or highlights for panel attribute elements.     │
 │ Enter the EXIT command to save changes or enter the CANCEL command to exit  │
 │ without saving. To restore the defaults for a type, clear the field and     │
 │ press Enter or select the Defaults point-and-shoot field to restore all     │
 │ default settings for all types.                                             │
 │                                                                             │
 │ Panel Element                  Color         Intensity  Highlight           │
 │                                                                More:   -    │
 │ PD Unavailable Choices . . . . BLUE          LOW        NONE                │
 │ Reference Phrase . . . . . . . WHITE         HIGH       NONE                │
 │ Scroll Information . . . . . . WHITE         HIGH       NONE                │
 │ Sel. Available Choices . . . . WHITE         LOW        NONE                │
 │ Sel. Unavailable Choices . . . BLUE          LOW        NONE                │
 │ Variable Output Info.  . . . . TURQ          LOW        NONE                │
 │ Warning Message Text . . . . . YELLOW        HIGH       NONE                │
 │ Warning Text . . . . . . . . . RED           HIGH       NONE                │
 │ Work Area Separator Line . . . BLUE          LOW        NONE                │
 │ Command ===>                                                     Defaults   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F12=Cancel                                                   │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 43. CUA Attribute Change Utility panel positioned on the point-and-shoot panel element (ISPOPT1X)
To change any of the three attributes, type over the existing values. The changes are reflected on the next
panel displayed after you exit this panel. Table 8 on page 55 shows valid change values:
Table 8. Valid point-and-shoot change values values
Color Choices Intensity Choices Highlight Choices
RED HIGH NONE
PINK LOW BLINK
GREEN REVERSE
YELLOW USCORE
BLUE
TURQ
WHITE
Specifying ISPF ENVIRON settings (the Environ action bar choice)
Figure 44 on page 56 shows the ISPF ENVIRON Command Settings panel from which you can choose
parameter options for the ENVIRON command. To display this panel, perform one of these actions:
• Select the Environ choice on the ISPF Settings panel action bar, then select option 1, "Environ
settings…".
• Issue the ISPF system command ENVIRON from any ISPF command line.
Settings (option 0)
Chapter 2. Settings (option 0)  55

## Page 94

Figure 44. ISPF ENVIRON Settings panel (ISPENVA)
The panel text provides an overview of the command and its parameters. For a complete description of
the ENVIRON command and its parameters, see z/OS ISPF Dialog Developer's Guide and Reference.
Specifying shared profile settings (the Environ action bar choice)
Figure 45 on page 57 shows the Multi-Logon Profile Sharing Settings panel from which you can choose
parameter options for the SHRPROF command. To display this panel, perform one of these actions:
• Select the Environ choice on the ISPF Settings panel action bar, then select option 2, "Shared Profile
settings…".
• Issue the ISPF system command SHRPROF from any ISPF command line.
Settings (option 0)
56  z/OS: z/OS ISPF User's Guide Vol II

## Page 95

Figure 45. Multi-Logon P r o file  Sharing Settings (ISPISSA)
The panel text provides an overview of the command and its parameters. For a complete description of
the SHRPROF command and its parameters, see z/OS ISPF Dialog Developer's Guide and Reference.
Displaying message, system, user, panel, and screen IDs
The Identifier action bar choice allows you to display message IDs with the message text, and to display
system, user, panel, and screen identifiers at the start of the Title line.
Settings (option 0)
Chapter 2. Settings (option 0)  57

## Page 96

Figure 46. Identifier  pull-down on the ispf settings panel action bar (ISPISMMN)
Message identifier
You can specify that you want to display message identifiers in either of two ways:
• Select the "Message identifier" choice from the Identifier pull-down on the ISPF Settings panel action
bar, as shown in Figure 46 on page 58.
• Issue the ISPF system command MSGID ON.
When you select "Message identifier" from the Identifier pull-down, the Message Identifier panel is
displayed.
If you select the "Display message identifier" option, the message identifier is set to On. The identifier
will now display within the message text whenever a long message option is accessed (that is, when you
enter the HELP command). Deselect this choice (or issue the MSGID OFF command) to set the message
identifier to Off.
This setting only applies to the current ISPF session. To retain the setting across ISPF sessions, select
"Default setting for message identifier".
Figure 47 on page 59 shows an error message on the ISPF Settings panel displayed with the message
identifier set to on.
Settings (option 0)
58  z/OS: z/OS ISPF User's Guide Vol II

## Page 97

Figure 47. Panel displayed with the message identifier  set to on
System name
You can specify that you want to display the system name in either of two ways:
• Select the "System name" choice from the Identifier pull-down on the ISPF Settings panel action bar.
• Issue the ISPF system command SYSNAME ON.
When you select "System name" from the Identifier pull-down, the System Name Identifier panel is
displayed.
If you select the "Display system name identifier" option, the system name identifier is set to On. The
identifier will now display in the panelid area. Deselect this choice (or issue the SYSNAME OFF command)
to set the system name identifier to Off.
This setting only applies to the current ISPF session. To retain the setting across ISPF sessions, select
"Default setting for system name".
This figure shows the top portion of the ISPF Settings panel displayed with the screen identifier set to On.
Settings (option 0)
Chapter 2. Settings (option 0)  59

## Page 98

Figure 48. Panel displayed with the system name set to on
Note: The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at
the start of the Title line. If more than one of these commands are specified, ISPF displays as many as will
fit, in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough
room, USERID is displayed, then PANELID, then SCRNAME.
User ID
You can specify that you want to display the user ID in either of two ways:
• Select the "User ID" choice from the Identifier pull-down on the ISPF Settings panel action bar.
• Issue the ISPF system command USERID ON.
When you select "User ID" from the Identifier pull-down, the User Identifier panel is displayed.
If you select the "Display user identifier" option, the user identifier is set to On. The identifier will now
display in the panelid area. Deselect this choice (or issue the USERID OFF command) to set the user
identifier to Off.
This setting only applies to the current ISPF session. To retain the setting across ISPF sessions, select
"Default setting for user identifier".
This figure shows the top portion of the ISPF Settings panel displayed with the user identifier (and the
system name) set to On.
Settings (option 0)
60  z/OS: z/OS ISPF User's Guide Vol II

## Page 99

Figure 49. Panel displayed with the user ID set to on
Note: The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at
the start of the Title line. If more than one of these commands are specified, ISPF displays as many as will
fit, in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough
room, USERID is displayed, then PANELID, then SCRNAME.
Panel identifier
You can specify that you want to display panel identifiers in either of two ways:
• Select the "Panel identifier" choice from the Identifier pull-down on the ISPF Settings panel action bar,
as shown in Figure 46 on page 58.
• Issue the ISPF system command PANELID ON.
When you select "Panel identifier" from the Identifier pull-down, the Panel Identifier panel is displayed.
If you select the "Display panel identifier" option, the panel identifier is set to On. The identifier will now
display in the panelid area. Deselect this choice (or issue the PANELID OFF command) to set the panel
identifier to Off.
This setting only applies to the current ISPF session. To retain the setting across ISPF sessions, select
"Default setting for panel identifier".
This figure shows the top portion of the ISPF Settings panel displayed with the panel identifier set to On.
Settings (option 0)
Chapter 2. Settings (option 0)  61

## Page 100

Figure 50. Panel displayed with the panel identifier  set to on
Note: The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at
the start of the Title line. If more than one of these commands are specified, ISPF displays as many as will
fit, in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough
room, USERID is displayed, then PANELID, then SCRNAME.
Screen name
You can specify that you want to display the screen name in either of two ways:
• Select the "Screen name" choice from the Identifier pull-down on the ISPF Settings panel action bar.
• Issue the ISPF system command SCRNAME ON.
When you select "Screen name" from the Identifier pull-down, the Screen Name Identifier panel is
displayed.
If you select the "Display screen identifier" option, the screen identifier is set to On. The identifier will now
display in the panelid area. Deselect this choice (or issue the SCRNAME OFF command) to set the screen
identifier to Off.
This setting only applies to the current ISPF session. To retain the setting across ISPF sessions, select
"Default setting for screen identifier".
This figure shows the top portion of the ISPF Settings panel displayed with the screen identifier set to On.
Settings (option 0)
62  z/OS: z/OS ISPF User's Guide Vol II

## Page 101

Figure 51. Panel displayed with the screen identifier  set to on
Note: The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at
the start of the Title line. If more than one of these commands are specified, ISPF displays as many as will
fit, in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough
room, USERID is displayed, then PANELID, then SCRNAME.
Settings (option 0)
Chapter 2. Settings (option 0)  63

## Page 102

Settings (option 0)
64  z/OS: z/OS ISPF User's Guide Vol II
