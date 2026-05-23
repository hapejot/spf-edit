# Chapter 2. The ISPF user interface

Source file: f54ug00_v3r1.md
Start page: 43
Page span: 43-60

## Page 43

Chapter 2. The ISPF user interface
ISPF provides an action bar-driven interface that exploits many of the usability features of Common
User Access (CUA) interfaces. For more information about CUA, see Object-Oriented Interface Design: IBM
Common User Access Guidelines.
These action bars give you another way to move around in ISPF, as well as the ability to nest commands.
Command nesting allows you to suspend an activity while you perform a new one rather than having to
end a function to perform another function.
This chapter primarily explains the action bar-driven interface. If you use a non-programmable terminal
to access ISPF and you do not want to use the command nesting function, you can make selections by
typing in a selection number and pressing Enter.
Some terms you should know
These terms are used in this document:
action bar
The area at the top of an ISPF panel that contains choices that give you access to actions available on
that panel. When you select an action bar choice, ISPF displays a pull-down menu.
command procedure
A CLIST or REXX exec
data set
A sequential or partitioned data set
ellipsis
Three dots that follow a pull-down choice. When you select a choice that contains an ellipsis, ISPF
displays a pop-up window.
function key
In previous releases of ISPF, a programmed function (PF) key. This is a change in terminology only.
library
A partitioned data set
menu
A selection panel
mnemonics
Action bar choices can be defined with a underscored letter in the action bar choice text. You
can access the action bar choice with the ACTIONS command and parameter x, where x is the
underscored letter in the action bar choice text.
modal pop-up window
A type of window that requires you to interact with the panel in the pop-up before continuing. This
includes canceling the window or supplying information requested.
modeless pop-up window
A type of window that allows you to interact with the dialog that produced the pop-up before
interacting with the pop-up itself.
point-and-shoot text
Text on a screen that is cursor-sensitive. See “Point-and-Shoot text fields” on page 23 for more
information.
pop-up window
A bordered temporary window that displays over another panel.
pull-down menu
A list of numbered choices extending from the selection you made on the action bar. The action bar
selection is highlighted; for example, Utilities in Figure 6 on page 21 appears highlighted on your
© Copyright IBM Corp. 1980, 2024 15

## Page 44

screen. You can select an action either by typing in its number and pressing Enter or by selecting the
action with your cursor. ISPF displays the requested panel. If your choice contains an ellipsis (…),
ISPF displays a pop-up window. When you exit this panel or pop-up, ISPF closes the pull-down and
returns you to the panel from which you made the initial action bar selection.
select
In conjunction with point-and-shoot text fields and action bar choices, this means moving the cursor
to a field and simulating Enter.
terminal
Any of the supported display devices
Understanding ISPF panels
A panel is a predefined display image that you see on a display screen. ISPF formats all panels to fit on a
24-line by 80-character screen. On a 3278 Model 3 or 4, data that you can scroll occupies the full length
of the screen (32 or 43 lines). On a 3278 Model 5, ISPF normally displays information in default mode;
that is, 24 lines by 80 characters, with the same size characters as other models. "Browse" and "Edit"
data that is wider than 80 characters is displayed with the smaller native mode characters, that is, up to
132 per line. You can use the Settings option (0) to override the automatic switching of modes.
Panel format
Figure 4 on page 16 shows how ISPF formats the first three and last few lines of each display:
 Action Bar
 ───────────────────────────────────────────────────────────────────────────────
 Panel ID                       Title                           Short Message
 ⋮
 Long Message
 Option  ===>                                                  Scroll ===>
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
Figure 4. Panel Format
Note: The "Panel display CUA mode" field on the ISPF Settings panel determines where the Command
or Option line and long messages are displayed. The default setting selects "Panel display CUA mode",
which causes the Command or Option line to be displayed on the bottom of the panel. The default setting
also selects "Long message in pop-up", which causes long messages to be displayed in a pop-up window
directly above the Command or Option line. To display the command or option line and long messages
at the top of the panel, select option 0, deselect the "Panel display CUA mode" field, and deselect
the "Command line at bottom" field. See z/OS ISPF Dialog Developer's Guide and Reference for more
information about the "Panel display CUA mode" and "Command line placement" fields.
The fields on Figure 4 on page 16 function as follows:
Action Bar
The action bar provides access to pull-down menus that give you a new and faster way to move
around in the product as well as access to command stacking. See Figure 6 on page 21 for more
information about using the action bar.
Panel ID
This area can be overlaid by the system commands SYSNAME, USERID, SCRNAME, or PANELID. The
data displayed can be up to 17 characters wide. If none of these commands is in effect, data from the
panel title line is displayed.
Note: For more information about the system commands PANELID, SCRNAME, SYSNAME, USERID,
and the order of priority that determines what is displayed in this 17-character area (if you specify
more than one of these commands), see “ISPF system commands” on page 34.
Title
Identifies the function being carried out and, where appropriate, the library or data set information.
Understanding ISPF Panels
16  z/OS: z/OS ISPF User's Guide Vol I

## Page 45

Short Message
Shows:
• Current line for Browse, and column positions for View, Browse, and Edit.
• Current row position in a table display if the short message area is not overlaid by a function-
requested message.
• Successful completion of a processing function.
• Error conditions (with an audible alarm, if one is installed). See z/OS ISPF Dialog Developer's Guide
and Reference for information about coding an alarm.
Command/Option
Allows you to enter a command or, on a menu, to enter either a command or an option.
Scroll
Indicates the scroll amount (if scrolling applies). You can type over it to change it. The valid scroll
amounts are:
nnnn
A number of lines or columns (between 0 and 9999).
CSR
Move the line or column that contains the cursor to the edge of the scrollable area. For example,
if you scroll right, the cursor will be positioned at the left side of the scrollable area. If you scroll
down, the cursor will be positioned at the top of the scrollable area.
DATA
Scroll by one line or column less than a full page. For example, if you scroll up, the line that was
displayed at the top of the page becomes the bottom line in the new page of data.
HALF
Scroll by half the number of lines or columns in the scrollable area.
MAX
Scroll to the limit of the data. For example, if you enter MAX and scroll down, the last page of data
is displayed. If you enter MAX and scroll up, the first page of data is displayed. Note that MAX
only applies to the next scroll command. When the MAX scroll command has been processed, the
scroll amount reverts to the previous setting.
PAGE
Scroll by the full height (if scrolling up down) or width (if scrolling left or right).
Long Message
Displays an explanation of error conditions in a pop-up window when you enter the HELP command
(see “Getting help” on page 9). On some displays, data may be overlaid temporarily by a long
message.
Function Keys
Displays settings for the function keys. These settings are controlled through the Function keys
pull-down on the action bar on the ISPF Settings panel.
Panel types
When using ISPF, you see three basic types of panels:
• Menus (selection panels)
• Data-entry panels
• Scrollable data displays.
Menus
A menu, or selection panel, allows you to type a number or letter in the Option field and press Enter
to select one of the listed items. The number or letter can be typed in either uppercase or lowercase.
Understanding ISPF Panels
Chapter 2. The ISPF user interface  17

## Page 46

Allowable numbers and letters are shown in high intensity. You can also enter ISPF commands. See Figure
5 on page 18 for an example of a menu.
Note: If the word BLANK or blank is listed, leave the Option field blank and press Enter to select that
option. Do not type the word blank.
 1   Menu  Utilities  Compilers  Options  Status  Help
 ──────────────────────────────────────────────────────────────────────────────
                            ISPF Primary Option Menu
  2 
 0  Settings      Terminal and user parameters            User ID . : USERID
 1  View          Display source data or listings         Time. . . : 13:13
 2  Edit          Create or change source data            Terminal. : 3278
 3  Utilities     Perform utility functions               Screen. . : 1
 4  Foreground    Interactive language processing         Language. : ENGLISH
 5  Batch         Submit job for language processing      Appl ID . : ISR
 6  Command       Enter TSO commands                      TSO logon : ISPF
 7  Dialog Test   Perform dialog testing                  TSO prefix: USERID
 9  IBM Products  IBM program development products        System ID : MVS8
 10 SCLM          SW Configuration Library Manager        MVS acct. : IBMGSA
 11 Workplace     ISPF Object/Action Workplace            Release . : ISPF 7.5
 12 z/OS System   z/OS system programmer applications
 13 z/OS User     z/OS user applications
      Enter X to Terminate using Log/List defaults
 Option ===>                                                                   
 3   F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
 1 
Action bar. You can select any of the action bar choices and display a pull-down.
 2 
Options Menu. The fields in this column are point-and-shoot text fields.
 3 
Function Key bar. Displays the Function Keys that are active on the current panel.
Figure 5. Primary Option Menu showing key features
Data entry panels
A data-entry panel is a panel on which you specify information, such as data set names, job statement
parameters, and language processing options. If you do not enter a required value or if you enter
inconsistent values, ISPF prompts you with a message.
Some data-entry fields retain their previous values. If so, the next time you use the panel, you do not
have to type them again. Just press Enter. If you do not want those values, type over them and then press
Enter.
The retained values come from your user profile, which ISPF automatically builds and maintains across
sessions. See “User profiles” on page 8 for more information about user profiles.
Edit modes and defaults are also maintained in the Edit portion of your user profile. See z/OS ISPF Edit
and Edit Macros for more information.
Option selection
You can select an ISPF option three ways:
• Select a choice from one of the pull-downs on the action bar. See Figure 6 on page 21 for more
information.
Understanding ISPF Panels
18  z/OS: z/OS ISPF User's Guide Vol I

## Page 47

• Select one of the point-and-shoot fields. See “Point-and-Shoot text fields” on page 23 for more
information.
• Type an option number on the Option line and press Enter.
Many options have a secondary list of options. To bypass the second menu, type two selections,
separating them with a period, on the ISPF Primary Option Menu. For example, entering 3.1 on the ISPF
Primary Option Menu is the same as entering 3 on the ISPF Primary Option Menu and 1 on the Utility
Selection Panel.
An even faster way to select an option is to bypass both the ISPF Primary Option Menu and the secondary
menus. To do this, include your options in the ISPF (or alias) command. For example:
ISPF 2
To go directly to the Edit option.
ISPF 3.1
To go directly to the Library utility (3.1).
Action bars
Action bars give you another way to move through ISPF. Most ISPF panels have action bars at the top;
the choices appear on the screen in white by default. Many panels also have point-and-shoot text fields,
which appear in turquoise by default. The panel shown in Figure 5 on page 18 has both.
If you use a non-programmable terminal to access ISPF and you do not want to take advantage of the
command nesting function, you can make selections by typing a selection number and pressing Enter.
If the cursor is located somewhere on the panel, there are several ways to move it to the action bar:
• Use the cursor movement keys to manually place the cursor on an action bar choice.
• Type ACTIONS on the command line and press Enter to move the cursor to the first action bar choice.
• Press F10 (Actions) or the Home key to move the cursor to the first action bar choice.
If mnemonics are defined for action bar choices, you can:
– On the command line, type ACTIONS and the mnemonic letter that corresponds to an underscored
letter in the action bar choice text. This results in the display of the pull-down menu for that action
bar choice.
– On the command line enter the mnemonic letter that corresponds to an underscored letter in the
action bar choice text, and press the function key assigned to the ACTIONS command. This results in
the display of the pull-down menu for that action bar choice.
Use the tab key to move the cursor among the action bar choices.
Note:
1. ISPF does not provide a mouse emulator program. This document uses select in conjunction with
point-and-shoot text fields and action bar choices to mean moving the cursor to a field and simulating
Enter.
Some users program their mouse emulators as follows:
• Mouse button 1: position the cursor to the pointer and simulate Enter
• Mouse button 2: simulate F12 (Cancel)
2. If you want the Home key to position the cursor at the first input field on an ISPF panel, type
SETTINGS on any command line and press Enter to display the ISPF Settings panel. Deselect the "Tab
to action bar choices" option.
When you select one of the choices on the action bar, ISPF displays a pull-down menu.
Understanding ISPF Panels
Chapter 2. The ISPF user interface  19

## Page 48

The action bar choices available vary from panel to panel, as do the choices available from their pull-
downs. However, Menu and Utilities are basic action bar choices, and the choices on their pull-down
menus are always the same.
Menu action bar choice
The following choices are available from the Menu pull-down:
Settings
Displays the ISPF Settings panel
View
Displays the View Entry panel
Edit
Displays the Edit Entry panel
ISPF Command Shell
Displays the ISPF Command Shell panel
Dialog Test
Displays the Dialog Test Primary Option panel
Other IBM Products
Displays the Additional IBM Program Development Products panel
SCLM
Displays the SCLM Main Menu
ISPF Workplace
Displays the Workplace entry panel
Status Area
Displays the ISPF Status panel
Exit
Exits ISPF
Utilities action bar choice
The following choices are available from the Utilities pull-down:
Library
Displays the Library Utility panel
Data Set
Displays the Data Set Utility panel
Move/Copy
Displays the Move/Copy Utility panel
Data Set List
Displays the Data Set List Options panel
Reset Statistics
Displays the Reset ISPF Statistics panel
Hardcopy
Displays the Hardcopy Utility panel
Reserved
Reserved for future use by ISPF; an unavailable choice
Outlist
Displays the Outlist Utility panel
Commands
Displays the Command Table Utility panel
Reserved
Reserved for future use by ISPF; an unavailable choice
Understanding ISPF Panels
20  z/OS: z/OS ISPF User's Guide Vol I

## Page 49

Format
Displays the Format Specification panel
SuperC
Displays the SuperC Utility panel
SuperCE
Displays the SuperCE Utility panel
Search-for
Displays the Search-For Utility panel
Search-forE
Displays the Search-ForE Utility panel
Table Utility
Displays the ISPF Table Utility panel
Directory List
Displays the z/OS UNIX Directory List Utility panel
Figure 6 on page 21 shows the pull-down menu displayed when you select Utilities on the ISPF Primary
Option Menu action bar.
 1 
The selected action bar choice is highlighted.
Figure 6. Panel with an Action Bar Pull-Down Menu
To select a choice from the Utilities pull-down menu, type its number in the entry field (underlined) and
press Enter or select the choice. To cancel a pull-down menu without making a selection, press F12
(Cancel). For example, if you select choice 9, ISPF displays the Command Table Utility pop-up, as shown
in Figure 8 on page 23.
Note: If a choice displays in blue (the default) with an asterisk as the first digit of the selection number,
the choice is unavailable for one of these reasons:
• Recursive entry is not permitted here
Understanding ISPF Panels
Chapter 2. The ISPF user interface  21

## Page 50

• The choice is the current state; for example, the Status is currently set to Session in Figure 7 on page
22. 
Figure 7. An Unavailable Choice on a Pull-Down
Interaction of command and action bar choice
If you enter a command on the command line before selecting an action bar choice, the command is
processed and the pull-down menu is not displayed. The CANCEL, END, and RETURN commands are
exceptions. These three commands are not processed and the cursor is repositioned to the first input field
in the panel body. If there is no input field, the cursor is repositioned under the action bar area.
Understanding ISPF Panels
22  z/OS: z/OS ISPF User's Guide Vol I

## Page 51

Figure 8. Pop-Up Selected from an Action Bar Pull-Down
Point-and-Shoot text fields
Point-and-shoot text fields are cursor-sensitive; if you select a field, the action described in that field is
performed. For example, if you select Option 0, Settings, in Figure 9 on page 24, ISPF displays the ISPF
Settings panel.
Note:
1. If you have entered a command on the command line, it is processed before any point-and-shoot
command.
2. As the cursor-sensitive portion of a field often extends past the field name, you may want to make
this area visible. To display point-and-shoot fields in reverse video, use the PSCOLOR command to set
Highlight to REVERSE.
3. You can use the Tab key to position the cursor to point-and-shoot fields by selecting the "Tab to
point-and-shoot fields" option on the ISPF Settings panel (Option 0).
Understanding ISPF Panels
Chapter 2. The ISPF user interface  23

## Page 52

1 
Action bar. You can select any of the action bar choices and display a pull-down.
 2 
Options. The fields in this column are point-and-shoot text fields.
 3 
Dynamic status area. You can specify what you want to be displayed in this area.
Figure 9. Panel with an Action Bar and Point-and-Shoot Fields
Function keys
ISPF uses CUA-compliant definitions for function keys F1-F12 (except inside the Edit function). F13-F24
are the same as in ISPF Version 3. By default you see the CUA definitions because your "Primary range"
field is set to 1 (Lower - 1 to 12).
To use non-CUA-compliant keys, select the "Tailor function key display" choice from the Function keys
pull-down on the ISPF Settings (option 0) panel action bar. On the Tailor Function Key Definition Display
panel, specify 2 (Upper - 13 to 24) in the "Primary range" field.
These function keys help you navigate in ISPF:
F1
Help. Displays Help information. If you press F1 (and it is set to Help) after ISPF displays a short
message, a long message displays in a pop-up window.
F2
Split. Divides the screen into two logical screens separated by a horizontal line or changes the
location of the horizontal line.
F3
Exit (from a pull-down). Exits the panel underneath a pull-down.
F3
End. Ends the current function.
Understanding ISPF Panels
24  z/OS: z/OS ISPF User's Guide Vol I

## Page 53

F7
Backward. Moves the screen up the scroll amount.
F8
Forward. Moves the screen down the scroll amount.
F9
Swap. Moves the cursor to where it was previously positioned on the other logical screen of a
split-screen pair.
F10
Actions. Moves the cursor to the action bar. If you press F10 a second time, the cursor moves to the
command line.
F12
Cancel. Issues the Cancel command. Use this command to remove a pull-down menu if you do
not want to make a selection. F12 also moves the cursor from the action bar to the Option ==>
field on the ISPF Primary Option Menu. See z/OS ISPF Dialog Developer's Guide and Reference for
cursor-positioning rules.
F16
Return. Returns you to the ISPF Primary Option Menu or to the display from which you entered a
nested dialog. RETURN is an ISPF system command.
Selection fields
ISPF uses these CUA-compliant conventions for selection fields:
A single period (.)
Member lists that use a single period in the selection field recognize only a single selection. For
example, within the Edit function you see this on your screen:
│EDIT     USER1.UTIL.CNTL                                 Row 0000001 of 0000023  │
│ Command ===>                                                  Scroll ===> CSR   │
│ Name         Prompt          Size    Created           Changed            ID    │
│ . ADDUSER                     42   1996/12/02   2014/05/13 21:47:40    IBMUSER  │
│ . ADDUSERS                    21   1996/03/11   2014/02/06 07:05:30    USER3    │
You can select only one member to edit.
A single underscore (_)
Selection fields marked by a single underscore prompt you to use a slash (/) to select the choice.
You may use any nonblank character. For example, the "Panel display CUA mode" field on the ISPF
Settings panel has a single underscore for the selection field:
Options
  Enter "/" to select option
  _  Command line at bottom
  _  Panel display CUA mode
  _  Long message in pop-up
An underscored field (____)
Member lists or text fields that use underscores in the selection field recognize multiple selections.
For example, from the Display Data Set List Option panel, you may select multiple members for print,
rename, delete, edit, browse, or view processing.
Entering commands in ISPF
ISPF provides flexibility by accepting various types of commands and having many methods for entering
them. Table 3 on page 26 provides an overview of the entry methods and commands available.
Understanding ISPF Panels
Chapter 2. The ISPF user interface  25

## Page 54

Table 3. Entry Methods and Command Types
Entry Methods
TSO Commands,
CLISTs, and REXX
EXECs
ISPF Primary
Commands
ISPF Line
Commands
ISPF Command Shell (option 6) X X  
Command or Option field (1) X (2) X  
Line Command fields (1 - 6 characters) X
Line Command fields (9 characters) X    X
Note:
1. Case sensitivity of a primary command is dependent on the attributes of the panel field from
which the command is issued. When the command field specifies CAPS(ON), any characters in the
command field are translated to upper case.
2. Available when prefixed by TSO.
For example, to use the TSO ALLOCATE command, you could enter:
Command ===> TSO ALLOCATE
Also, CLIST names and REXX exec names can be preceded by a percent (%) symbol, as in:
Option ===> TSO %CLIST
This symbol informs TSO that the command is a CLIST or REXX exec, not a TSO command.
You can also use two command entry methods not shown in the table: the PA keys and the function keys.
PA1 (ATTENTION) and PA2 (RESHOW) are hardware keys that you cannot redefine. You can use function
keys to enter all commands.
Types of commands
There are two types of commands that you can enter in the ISPF Command or Option field:
• TSO commands, CLISTs, and REXX EXECs
You invoke commands through the MVS/TSO operating system. These include TSO commands, CLISTs,
REXX EXECs, and the commands assigned to the PA1 and PA2 keys.
• ISPF Primary commands
You call ISPF functions, such as ISPLIBD to display active LIBDEFs, or enter commands to navigate
through a dialog, such as CANCEL to cancel execution or DOWN to scroll down.
TSO commands, CLISTs, and REXX EXECs
ISPF gives you access to the MVS/TSO operating system by letting you enter TSO commands, CLISTs,
and REXX EXECs from within ISPF. z/OS TSO/E Command Reference contains descriptions of all TSO
commands.
You can enter most TSO commands, CLISTs, and REXX EXECs from all three panel types: data entry;
menu; and scrollable data display. However, some TSO commands, such as LOGON and LOGOFF, can
cause unwanted results when you enter them from ISPF. This is also true of CLISTs and REXX EXECs
that contain these TSO commands. The rules for entering TSO Commands, CLISTs and REXX EXECS are
described in the Command (Option 6) topic in the z/OS ISPF User's Guide Vol II.
Understanding ISPF Panels
26  z/OS: z/OS ISPF User's Guide Vol I

## Page 55

ISPF primary commands
ISPF primary commands are valid from all three types of panels. However, the validity of some commands
depends on the type of panel displayed or the type of terminal you use. For example:
• The scroll commands (UP, DOWN, LEFT, and RIGHT) are valid only on scrollable data displays.
• The SPLITV command is valid only on a 3290 display terminal.
• Some commands are only valid for specific functions. For example:
– The CAPS command is valid only when using the Edit or View function.
– The DISPLAY command is valid only when using the Browse function.
– The SPROF command is valid only when using the SCLM Edit function.
For a description of the ISPF commands, default function key settings, and the PA1 and PA2 keys, see
“ISPF system commands” on page 34. For information about which commands are valid for a particular
function, see the relevant topic in z/OS ISPF Edit and Edit Macros for edit commands, z/OS ISPF Software
Config ur ation  and Library Manager Guide and Reference for SCLM commands, or z/OS ISPF User's Guide
Vol II for commands for the various ISPF options.
ISPF line commands
Line commands affect one or more specified lines that you select. For example:
• The C or CC Edit line command can copy lines or blocks of lines within a data set.
• The D line command on the ISPF Dialog Variables panel deletes one variable from the profile pool.
• The R line command on the DSLIST panel of the Data Set Utility renames an entire data set.
• The R line command on a member list panel renames a single member in a partitioned data set.
Multicultural support
Note: The term "multicultural support" has replaced the previous term "National Language Support" (or
"NLS").
Multicultural support gives countries the option of translating commands and keywords so that users can
enter them in the country's national language. In addition, panels, messages, and literal modules are
provided with the product for the Japanese language.
When multicultural support is enabled, users can be required to enter certain keywords in the national
language and not in English.
When the Danish, French, Korean, Traditional Chinese, Simplified Chinese, Spanish, Brazilian-Portuguese,
Italian, German, or Swiss German session language is specified, its respective literal module is used.
However, the ISPF product panels and messages are displayed in English.
ISPF command syntax notation
The notation conventions for ISPF command syntax follow.
• Uppercase commands and their uppercase parameters show required entry.
• Lowercase parameters show variables (substitute your values for them).
Stacking commands
To enter more than one command, you can stack them by typing a special delimiter between them. The
default delimiter is a semicolon. Use the Settings option (0) to change the delimiter. For example, to stack
two Edit CHANGE commands, use:
Command ===> CHANGE ALL ABC XYZ;CHANGE ALL PQR GHIJK
multicultural support
Chapter 2. The ISPF user interface  27

## Page 56

The system variable for the delimiter is ZDEL. See the topic about system variables in the z/OS ISPF Dialog
Developer's Guide and Reference for more information about ZDEL.
Dual command processing
You can enter information on a command or option line in combination with pressing a related function
key. The command is called first. For example, typing 4 on the command line and pressing F7 (UP
command) is the same as typing UP 4 on the command line and pressing Enter.
If the command you type is unrelated to the command assigned to a function key you press, ISPF passes
the entry to the function in control, which either processes or ignores the entry. For example, if the
Edit function is in control, ISPF may display an error message. However, if the Tutorial is in control, the
command is ignored. ISPF processes any stacked valid commands.
Line command fields
Line command fields can take many forms. Some have headings, some do not. Most are blank, but some
contain single quotation marks or sequence numbers. For some, you type one character; for others, you
type up to nine characters (even typing over data set names). Table 4 on page 28 shows the functions
that provide the line command fields.
Table 4. Characteristics of the Line Command Field
Functions Providing Line Command Fields
Heading
Displayed
Characters
Allowed
Initial
Contents
Member Selection List None 1-9 Blank
Edit (option 2) None 6 Quotes or
numbers
Data Set List utility (option 3.4) Command 9 Blank
Command Table utility (option 3.9) None 4 Quotes
Format Specification utility (option 3.11) None 1 Blank
SCRIPT/VS (option 4.9) Line Cmd 1 Quotes
Dialog Test (option 7):
Variables (option 7.3)
Tables (option 7.4)
Traces (option 7.7)
Breakpoints (option 7.8)
None 4 Quotes
The line command field for member selection lists is blank and has no heading. This includes typical
member lists, which have a 1-character line command field, and the member list displayed when you use
option M of the Data Set List utility, which has a 9-character line command field.
Quotes appear when you create a new data set or member, or when you insert one or more lines.
Sequence numbers appear if you have NUMBER ON in your Edit profile.
Data set lists with 9-character line command fields allow you to type over data set names, thus extending
the length of the fields to allow you to type long TSO commands, CLIST names, and REXX exec names.
For information about entering TSO commands, CLISTs, and REXX EXECs in a line command field, see the
Data Set List Utility (Option 3.4) topic in the z/OS ISPF User's Guide Vol II. Also, the rules for entering TSO
Commands, CLISTs and REXX EXECS from within ISPF are described in the Command (Option 6) topic in
the z/OS ISPF User's Guide Vol II.
multicultural support
28  z/OS: z/OS ISPF User's Guide Vol I

## Page 57

PDF component line commands
Most PDF component line commands use only one letter, such as S, for selecting a member from a
member list. Others, such as many of the Edit line commands, use more than one letter and sometimes
allow you to add a number so the command affects more than one line. For example, the UC line
command plus the number 3, as in UC3, converts three lines to uppercase.
Another type of line command is the block line command, which affects the block of lines between
and including the lines on which the commands are entered. For example, the UCC line command,
when entered beside two different lines, converts all lines between and including the two commands to
uppercase.
Command nesting
You can use the action bars to suspend an activity while you perform a new one.
For example, if you are editing a data set and want to allocate another data set, select the Data set choice
from the Utilities pull-down on the Edit panel action bar. ISPF suspends your edit session and displays the
Data Set Utility panel. When you have allocated the new data set and ended the function, ISPF returns
you directly to your edit session.
By contrast, if you used the jump function (=3.2), ISPF would end your edit session before displaying the
Data Set Utility.
Splitting the screen horizontally or vertically
While using a dialog, you can use the SPLIT command to partition the display into two or more logical
screens. The logical screens are treated as though they are independent ISPF sessions.
The maximum number of screens available to you depends on the value of the
MAXIMUM_NUMBER_OF _SPLIT_SCREENS keyword in the ISPF Configuration table. ISPF ships with a
default figure of 8. Support for up to 32 split screens is available for all terminal types except the 3290.
Note: Although a 3270 screen can only display two screens at one time, there can be other screens (up to
32) that are not visible. You can select which logical screen to display by using the SWAP LIST command
to display a list of logical screens.
The SPLIT command
You enter split-screen mode by using the SPLIT command. You also use this command to reposition the
horizontal line that separates the two logical screens on a 3270 display. On a 3270 display the location of
the cursor identifies the active logical screen. On a 3270 display, the horizontal divider line that separates
the logical screens is not considered part of either logical screen. If the cursor is placed on this horizontal
divider line and a function key is pressed, the result is the same as if the ENTER key was pressed and the
cursor is positioned on the active logical screen's command line.
SPLIT command without parameters
If only one screen is currently being used, the physical display is divided into two logical screens with
a divider at the cursor. If two or more screens exist, the divider line is moved, but no new screen is
started.
SPLIT NEW command
A new logical screen is added each time the command is given, until the maximum number is reached.
After the limit is reached, a message appears when the command is issued again. Each new logical
screen is added below the cursor, where the split line appears. If two or more screens already exist,
the new one replaces the screen in which the SPLIT command was not entered.
End split-screen mode by ending the application on all but one logical screen. The remaining logical
screen is then expanded to the full size of the display screen.
Splitting the Screen
Chapter 2. The ISPF user interface  29

## Page 58

The SWAP command
Although you can alternately use any logical screen, only one of the logical screens is considered active at
a time. The location of the cursor identifies the active screen. You make a screen active by using the SWAP
command and its parameters to choose the desired screen.
The parameters on the SWAP command (LIST, PREV, NEXT, screen_name, and n) control which screens
you see displayed.
SWAP command without parameters
If only one screen exists, this command has no effect. If more than one screen exists, this command
moves the cursor between the two logical screens that are displayed.
SWAP PREV|NEXT|screen_name|n commands
Entering SWAP PREV changes the display to the next lower screen number from the one where the
command is entered. Repeatedly issuing the same command causes each lower-numbered screen to
display until screen number 1 is reached, then the counter wraps back to screen number 32 (or your
installation's maximum number).
Entering SWAP NEXT changes the display to the next higher screen number from the one where the
command is entered. Repeatedly issuing the same command causes each higher-numbered screen to
display until screen number 32 (or your maximum) is reached, then the counter wraps back to screen
number 1.
Entering SWAP screen_name changes the display to the screen named screen_name if it is active.
Entering SWAP n changes the display to the screen numbered n (ZSCREEN variable) if it is active.
SWAP LIST command
This command displays the ISPF Task List (Figure 10 on page 30), from which you can select which
screen to display. The screen you select replaces the screen on which you entered the command.
Figure 10. ISPF Task List
The result when choosing one of the fields on the ISPF Task List panel are as follows:
Splitting the Screen
30  z/OS: z/OS ISPF User's Guide Vol I

## Page 59

Start a new screen
Starts a new logical ISPF screen.
Start a new application
This field is used in conjunction with the Application Name field. If you choose "Start a new
application" you must enter an application name in the "Application Name" field.
Application Name
The name of an application you want to start by choosing the "Start new application" field on
the ISPF Task List panel. This application is started in a new logical screen. ISPF invokes the
application through the ISPF START command, so any application name and parameters that
are valid for the START command are valid in the Application Name input field. If you need
more space to enter the application name and parameters, press the Expand PF key to display
a pop-up window that contains a longer input field.
For example, if a user types "keylist" in the Application Name input field and presses Enter, the
ISPF KEYLIST application is invoked in a new logical screen.
Select a screen from the list of active sessions
Provides a list of active sessions for you to choose from.
3290 terminals
On 3290 terminals, in addition to splitting the screen horizontally, you can use the SPLITV command to
split the screen vertically, for a total of four logical screens. In the case of the 3290 terminal, four is the
maximum number of screens possible. (The SPLITV function is not active if the data being displayed on
a screen is more than 80 characters wide.) Figure 11 on page 31 shows the effect of SPLIT and SPLITV,
starting in single-screen mode.
Figure 11. Splitting the 3290 Screen
Note:
Splitting the Screen
Chapter 2. The ISPF user interface  31

## Page 60

1. ISPF logical screens are separate subtasks that do not share subpool 0 (attached with SZERO=NO
parameter.) Thus, VSAM data sets cannot be accessed from more than one logical screen.
2. If you are in a VSAM application, perform a split screen operation, then enter another VSAM
application in the second session, you must be sure to end the second session before you end the
first session, or an abend can occur.
3. On 3290 hardware, using the jump function to move from screen to screen might result in the loss of
data that has been typed but not processed. The use of the 3290 hardware jump is not recommended.
4. In split-screen mode, if you type a command on the command line and swap screens before pressing
Enter, the command is erased.
Splitting the Screen
32  z/OS: z/OS ISPF User's Guide Vol I
