# Chapter 3. Using commands, function keys, and cursor selection

Source file: f54ug00_v3r1.md
Start page: 61
Page span: 61-102

## Page 61

Chapter 3. Using commands, function keys, and
cursor selection
This topic explains how to use the ISPF system commands, the function keys and their default
assignments, and the cursor-select facilities.
You can use commands to request processing functions. These are the levels of commands:
System commands
Provided by the DM component and always available to a user, unless explicitly overridden by an
application, a user, or a site.
User or Site commands
Defined by the site administrator (in the ISPF Configuration table) and available to a user, in addition
to the system commands.
Application commands
Available to a user throughout the processing of an application.
Function commands
Meaningful only while using a particular function within an application.
System, user, site, and application commands are defined by using command tables. The DM component
processes these commands. System, user, site, and application command processing is generally
transparent to the dialog functions. For example, HELP is a system command.
Function commands include all commands that are processed by a dialog function. For example, the
NUMBER command within the ISPF Editor (option 2) is a function command.
You can enter a command by:
• Typing the information on the command line, or in the command field, and pressing the Enter key. This
includes the command field in View, Browse, Edit, and Table Displays, as well as the command field on a
panel.
• Pressing the function key set to the desired command.
• Selecting an Attention field by using the cursor-select key. The cursor-select key is a hardware feature
on 3179, 3179G, 3180, 3278, 3279, and 3290 terminals.
ISPF intercepts all commands, regardless of whether the command is typed in the command field or
entered with a function key or cursor-select key. ISPF performs the command if it matches an entry in the
application, user, or system command table. Otherwise, it is assumed to be a function command and is
passed to the dialog function.
Case sensitivity of a primary command is dependent on the attributes of the panel field from which the
command is issued. When the command field specifies CAPS(ON), any characters in the command field
are translated to upper case.
You can pass commands to the operating system by entering the appropriate ISPF-provided command
(TSO) followed by the actual TSO command. For example:
    ===> TSO LISTC LEVEL(Z77PHJ)
You can stack commands to be run by entering a special delimiter between the commands. For example,
entering:
    ===> UPDATE BLDG DEPT NAME; MENU ABC
© Copyright IBM Corp. 1980, 2024 33

## Page 62

causes the UPDATE command to run first. When it completes, the MENU command starts. The default
delimiter is a semicolon (;), which you can change with the ISPF SETTINGS option (see the Settings
(Option 0) topic of the z/OS ISPF User's Guide Vol II.
Commands cannot be stacked following the:
• HELP command. HELP processing deletes any remaining commands in the stack.
• RETRIEVE command.
ISPF system commands
This section describes the ISPF system commands in alphabetical order.
ACTIONS
ACTIONS
Moves the cursor between the action bar and the panel body.
AUTOTYPE
AUTOTYPE
Allows you to type a partial data set or member name, then press a function key and have ISPF complete
the name.
Note: AUTOTYPE is not a true ISPF system command because it is not built into the base code of ISPF
and it works only on panels that are written to understand it.
BACKWARD
BACKWARD
Alias for the UP command. Scrolls toward the top of the data.
BOTTOM
BOTTOM
Alias for the DOWN MAX command. Scrolls to the bottom of the data.
CANCEL
CANCEL
If CANCEL is requested from an action bar pull-down, the pull-down is removed and the cursor is
positioned on the first action bar choice.
If CANCEL is requested from a panel displayed using the DISPLAY, TBDISPL, or SELECT service calls, the
DM component places the command in ZCMD and sets a return code of 0 from the display screen.
If CANCEL is requested from a panel displayed using the DISPLAY or TBDISPL service calls and the panel
was defined with a PANEL tag (DTL) or a )PANEL statement, the DM component returns the command in
ZVERB and sets a return code of 8 from the display screen.
ISPF System Commands
34  z/OS: z/OS ISPF User's Guide Vol I

## Page 63

CMDE
CMDE
If CMDE is entered on any command line, a pop-up panel (ISPCMDE) with a 234-character command
input field is displayed.
You can enter up to 234 characters using the entry field provided. ISPF allows TSO commands, CLISTS,
and REXX execs and parameters to be entered in the input field. This panel is processed much like the
PDF Option 6 panel. Data passed to this panel is translated to uppercase characters. Data passed from
this panel remains as it appears on the panel.
If input has been entered on the panel from which CMDE was called, it is saved and displayed when the
pop-up panel ISPCMDE is displayed.
COLOR
COLOR
Changes the default colors on seven-color display devices.
CRETRIEV
CRETRIEV
The actions of the CRETRIEV (conditional retrieve) command are based on the position of the cursor when
you enter the command:
• If the cursor is within the primary input field when you enter the CRETRIEV command, the command
does the same processing as the RETRIEVE command; the DM component places the previous
command entered, if any, in the command input field.
• If the cursor is not within the primary input field, the CRETRIEV command does the same processing as
a CURSOR command; the DM component places the cursor at the beginning of the first input field on the
panel, which is usually the option or command field.
CUAATTR
CUAATTR
Changes the default values of panel colors, intensities, and highlights for CUA panel element attributes.
CURSOR
CURSOR
Moves the cursor to the first input field on the panel being displayed, generally the option or command
field, or moves the cursor to the alternate command field if one has been designated on the )BODY
statement. If invoked a second time on a panel with scrollable data, this command causes the cursor to
be moved to the second input field. Scrollable data panels include a View, Browse, Edit, or table display
panel or a panel with a scrollable dynamic area.
DOWN
DOWN
Scrolls toward the bottom of the data.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  35

## Page 64

DSLIST
DSLIST
list name
DSname level
Enables you to build a data set list from any command line. You can specify either a personal data set list
name or a data set list name level on the command. If you do not put quotation marks around the dsname
level, the TSO prefix is used as the first qualifier in the dsname level.
By issuing the command with no parameters, you cause a list of available personal data set lists to be
displayed.
The DSLIST command, which invokes ISRDSLST, accepts system symbols in the parameter. For example:
DSLIST 'SYS2.**.&SYSPLEX'
DTEST
DTEST parameter number
Enables you to start, or change the conditions of, a dialog test. Specifying a parameter number is
required, and different conditions of dialog test result. For example, if you enter DTEST 8 while running an
application under Dialog Test, the 7.8 Breakpoints panel is displayed. After setting the breakpoints, you
return to your application with the new breakpoints activated. The panels that you can call up with DTEST
are:
1
Invoke Functions panel
2
Invoke Display Panel panel
3
Invoke Variables panel
4
Invoke Tables panel
5
Display Browse log panel
6
Invoke Dialog Services panel
7
Invoke Traces selection panel
8
Invoke Breakpoint panel
END
END
Stops the current operation and returns to the previous menu. If the ISPF Primary Option Menu is
displayed, this command ends ISPF. See “Log and list data set processing at the end of a session” on
page 110 for a description of the processing that occurs when the END command is entered from the ISPF
Primary Option Menu.
When entered on a selection panel displayed by the SELECT service, the END command causes a
redisplay of the next higher menu in the hierarchy. When entered on a panel displayed by the tutorial
ISPF System Commands
36  z/OS: z/OS ISPF User's Guide Vol I

## Page 65

program, it stops the tutorial and causes a redisplay of the menu from which the tutorial was started or
the panel from which HELP was requested.
When the END command is entered on a panel displayed by a dialog function through the DISPLAY or
TBDISPL service, the dialog function must take whatever action is appropriate to terminate and return
control. Entry of the END command is signaled by a return code of 8 from the DISPLAY or TBDISPL
service.
ENVIRON
ENVIRON
ENBLDUMP ON
OFF
TERMTRAC ON
ERROR
DUMP
OFF
TERMSTAT
QUERY
Allows you to reduce service time by gathering data that can be helpful in diagnosing problems. Functions
provided include:
• Enabling Abend dumps when ISPF is not in TEST mode
• Tracing and dumping ISPF terminal input and output data and errors
• Collecting terminal characteristic information.
EPDF
EPDF datasetname
Browse View Macro macroname
Profile profilename Panel panelname Recover
Format formatname Mixed YES
NO
Enables you to edit, browse, or view a data set from a command line.
Browse
Invoke Browse instead of edit.
View
Use View mode (End/Save/Cancel disabled).
Macro macroname
Invoke the editor or view using the initial macro specified by macroname. Not valid with Browse.
Profile profilename
Invoke the editor or view using the edit profile specified by pr o filename . Not valid with Browse.
Panel panelname
Use alternate panel name specified by panelname.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  37

## Page 66

Recover
Perform edit recovery if a recovery is pending. If edit recovery is used, the file name and macro
specified on the command are ignored. If no edit recovery session is pending, the file is edited as
usual. Not valid with Browse.
Format formatname
Invoke the editor using the format table specified by formatname.
Mixed YES|NO
Use mixed option for 5550 terminals.
The EPDF command is a REXX exec. Consequentially it substitutes dialog variables specified with the
parameter. For example, this command edits the current ISPF Temporary Control Data Set:
EPDF '&ZTEMPF'
The EPDF command also processes system symbols within the data set name before passing to the Edit,
View, or Browse service. A return code 4 from VSYM indicates that one or more system symbols were
not recognised and these remain unchanged within the data set name. The ISPF service routine can then
resolve them as dialog variables.
Here are some examples:
This command entered in the command line of an ISPF panel edits the data set defined by the current
dialog variable, ZTEMPF:
EPDF '&ZTEMPF'
This command entered in the command line of an ISPF panel edits the data set SYS2.CLIST.SYSPLEX1
when executed on a system that is a member of a sysplex named SYSPLEX1:
EPDF 'SYS2.CLIST.&SYSPLEX'
EXHELP
EXHELP
Provides general information about the contents of a panel.
EXIT
EXIT
Requests that the current function be ended. When entered on a panel displayed by the tutorial program,
EXIT stops the tutorial and causes a redisplay of the menu from which the tutorial was started or the
panel from which HELP was requested.
• If EXIT is requested from a panel displayed using the DISPLAY, TBDISPL, or SELECT service calls, the
DM component returns the command in ZCMD and sets a return code of 0 from the display screen.
• If EXIT is requested from a panel displayed using the DISPLAY or TBDISPL service calls and the panel
was defined using a PANEL tag (DTL) or a )PANEL statement, the DM component returns the command
in ZVERB and sets a return code of 8 from the display screen.
EXPAND
EXPAND
ZEXPAND
ISPF System Commands
38  z/OS: z/OS ISPF User's Guide Vol I

## Page 67

Displays a variable in a dynamic area in a pop-up expand window. This only applies if the cursor is within
a scrollable field. If the scrollable field is input, you will be able to update the variable in the expand
window.
FKA
FKA
ON
SHORT
OFF
Toggles through the different forms of the function key area. The first time you enter the FKA command
(without parameters), the long form of the function key area is displayed. The long form includes the keys
that have a format specified as either long or short in the keylist. If you enter the command again, the
short form is displayed. The short form displays only those keys that have the short format specified in
the keylist. If you enter the command once again, the keys are removed from the display. Therefore, if you
continue to enter the command, the different choices are toggled:
• Long form (default)
• Short form
• No display.
The form that you select affects all panels displayed in the session. The DM component updates the
system variable ZFKA to represent the current state of the function key area form and saves the value in
the system profile.
FKA ON displays the long form of the function key area.
FKA SHORT displays the short form of the function key area.
FKA OFF specifies that the function key area will not be displayed.
FORWARD
FORWARD
Alias for the DOWN command. Scrolls toward the bottom of the data.
HELP
HELP
Displays additional information about an error message, or provides tutorial/help information for panels,
fields on panels, commands, and options.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  39

## Page 68

ISPDPTRC
ISPDPTRC
END VIEW QUIET
DISPLAY( None
In
Out
Both
) PANEL( *
panel_name
panel_mask
)
READ( None
Summary
Detail
) SCREEN( 0
*
screen_id
)
SECTION( *
All
None
Init
NOInit
Reinit
NOReinit
Proc
NOProc
)
SERVICE( None
Detail
)
Invokes the panel trace utility, which allows you to trace both the execution of panel service calls
(DISPLAY, TBDISPL, and TBQUERY) and the processing that occurs within the Dialog Manager panel
code. For more information, refer to the topic about diagnostic tools and information in z/OS ISPF Dialog
Developer's Guide and Reference.
ISPDTLC
ISPDTLC
Invokes the ISPF DTL Conversion Utility. See the z/OS ISPF Dialog Tag Language Guide and Reference for
additional parameters and calling options.
ISPF System Commands
40  z/OS: z/OS ISPF User's Guide Vol I

## Page 69

ISPFTTRC
ISPFTTRC
END VIEW QUIET
READ( None
Summary
Detail
)
RECORDS( *
All
None
Src
Source
NOSrc
NOSource
Data
NOData
Cntl
NOCntl
)
SCREEN( 0
*
screen_id
) SERVICE( None
Detail
)
SKELETON( *
skel_name
skel_mask
) TBVARS( None
Detail
)
Invokes the file tailoring trace utility, which allows you to trace both the execution of file tailoring service
calls (FTOPEN, FTINCL, FTCLOSE, and FTERASE) and the processing that occurs within the file tailoring
code and processing of each statement. For more information, refer to the topic about diagnostic tools
and information in z/OS ISPF Dialog Developer's Guide and Reference.
ISPFVAR
ISPFVAR
ABTAB( ON
OFF
)
EDPRT( ON
OFF
)
EURO( ON
OFF
)
JUMP( ON
OFF
)
LMSG( ON
OFF
)
PSTAB( ON
OFF
)
SCRML( ON
OFF
)
SESM( ON
OFF
)
SPLTLINE( ON
OFF
)
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  41

## Page 70

Sets these ISPF settings:
ABTAB
Tab to action bar choices
EDPRT
Edit the PRINTDS command
EURO
Enable the EURO currency symbol
JUMP
Jump from leader dots
LMSG
Display long message in pop-up
PSTAB
Tab to point-and-shoot fields
SCRML
Scroll member list
SESM
Select Session Manager mode
SPLTLINE
Always show split line
ISPFWORK
ISPFWORK
Starts the ISPF Workplace.
For more information, see ISPF object/action workplace in z/OS ISPF User's Guide Vol II.
ISPLIBD
ISPLIBD
libtype
Invokes the LIBDEF Display Utility. The optional parameter, libtype, identifies a specific LIBDEF library
definition to be displayed. All LIBDEF definitions for the current logical screen are displayed if the
parameter is omitted, if the parameter is longer than eight characters, or if the parameter specifies
ISPPROF as the library name. See z/OS ISPF Services Guide for more information about LIBDEF and the
ISPLIBD command.
ISPPREP
ISPPREP
Allows you to create preprocessed panels, those for which ISPF has partially processed the panel
definition before it is stored in the panel data set, either interactively or in batch mode.
ISRRLIST
ISRRLIST
The action bar interface into referral lists. This command takes a required parameter as input. Valid values
are PL1, PL2, LL1, and LL2.
ISPF System Commands
42  z/OS: z/OS ISPF User's Guide Vol I

## Page 71

PL1
Current® Data Set List
PL2
List of Personal Data Set Lists
LL1
Current Library List
LL2
List of Personal Library Lists
ISRROUTE
ISRROUTE
The action bar interface into the ISPF command stacking routing. ISRROUTE also provides an interface to
the SELECT service from the ACTION/RUN statement within a pull-down choice. The parameters are the
same as the ISPEXEC interface to the SELECT service.
This command is generally used for internal purposes.
KEYLIST
KEYLIST
PRIVATE
SHARED
OFF
ON
The parameters on this command determine where, or if, ISPF looks for keylists. The default setting for
KEYLIST is equivalent to issuing the KEYLIST PRIVATE command, which means that the program looks in
the user's profile table for the keylist specified on a panel before looking in the xxxxKEYS table allocated
in ISPTLIB.
The KEYLIST SHARED command means that ISPF looks only in the xxxxKEYS table allocated in ISPTLIB
for the keylist.
Using either the PRIVATE or SHARED parameter performs an implicit KEYLIST ON command. Both of the
parameters are local to each application, so setting PRIVATE for application X does not affect application
Y, which might be using SHARED.
By specifying KEYLIST OFF, you cause ISPF to ignore the keylist on all logical screens and use the ZPF
variables for controlling function keys. This is in effect only for the application for which you enter the
command.
The KEYLIST ON command causes ISPF to recognize keylists again, with the parameter (SHARED or
PRIVATE) that was in effect immediately before the KEYLIST OFF command. KEYLIST ON and OFF are
equivalent to the Enable and Disable keylist choices on the Function keys pull-down. Keylist Settings are
discussed in the Settings (Option 0) topic of the z/OS ISPF User's Guide Vol II. SHARED and PRIVATE also
appear on the Function keys pull-down in "Keylist Settings".
The KEYLIST command with no parameters causes the Keylist utility to start.
KEYS
KEYS
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  43

## Page 72

Displays the PF Key Definitions and Labels panel, which allows you to change the ZPF variable settings
(ZPFVARs), as in previous versions of ISPF. However, if the KEYS command is issued from a panel with an
active keylist, the associated Keylist Utility panel Change pop-up window is displayed.
Note: If the KEYLIST SHARED command has been issued, or the SYSTEM parameter has been specified
on the KEYLIST keyword on the )PANEL statement, this action causes only a BROWSE of the keylist. See
the z/OS ISPF Dialog Developer's Guide and Reference for more information about the SYSTEM parameter
in the )PANEL statement.
KEYSHELP
KEYSHELP
If KEYSHELP is defined, KEYSHELP provides you with a brief description of each key defined for a panel.
LEFT
LEFT
Scrolls left. If your cursor is in a scrollable field, this scrolls towards the beginning of the field.
LIST
LIST
PRINT
DELETE
KEEP
Allows you to process the list data set without exiting ISPF. See “Processing the log and list data sets” on
page 108 for a description of using the LIST command.
LOG
LOG
PRINT
DELETE
KEEP
Allows you to process the log data set without exiting ISPF. See “Processing the log and list data sets” on
page 108 for a description of using the LOG command.
MSGID
MSGID ON
OFF
With no parameters, displays a message indicating the message ID of the last message displayed. With
a parameter of ON or OFF, indicates whether a message number is to be added to the beginning of
interactive long message text. During entry to ISPF, the mode is initialized to OFF, and the message ID is
not displayed as part of the long message text on interactive displays. If the addition of the message ID
would cause long message text to be truncated, the message is displayed in a pop-up window.
Messages that have the message number included in the long message text will continue to display the
message number, even when MSGID OFF is in effect. Also, the message number will appear twice when
MSGID ON is in effect.
ISPF System Commands
44  z/OS: z/OS ISPF User's Guide Vol I

## Page 73

The MSGID ON/OFF command affects only the current logical screen, so when you are running in split
screen, one screen can have MSGID ON and the other MSGID OFF. The MSGID command will return only
the MSGID of a message for its own logical screen.
An option on the Log Data Set Defaults and List Data Set Defaults panels, which are choices on the
Log/List pull-down on the ISPF Settings panel,
    Log Message ID . . . _  (/ = Yes)
allows you to select whether the message ID is written to the log data set as part of the long message
text. The initial default is deselected. Note that not all lines in the log data set originate from a message
member. Therefore, not every line in the log data set will have a message number associated with it.
Note: This facility does not affect long message text returned by the GETMSG service, messages
displayed in the Error Box, or messages displayed by TRACEX.
NOP
NOP
The classic no operation command.
NRETRIEV
NRETRIEV
Data set and library name retrieved. See “Name retrieval with the NRETRIEV command” on page 132 for
more information.
PANELID
PANELID
ON
OFF
Indicates whether the panel identifier (ID) is to be displayed. If you enter PANELID without any
parameters, the command toggles the display of the panel ID immediately below the action bar. If an
action bar is not present, the ID is displayed in line 1 on the panel.
During initial entry to ISPF, the PANELID is set to OFF. The ID is displayed only if the panel contains a
protected-field attribute byte in row 1 column 1 (relative to the action bar) and is padded with one blank.
The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at the
start of the Title line. If more than one of these commands are specified, ISPF displays as many as will fit,
in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough room,
USERID is displayed, then PANELID, then SCRNAME.
PFSHOW
PFSHOW
ON
OFF
TAILOR
Toggles through the different forms of the function key area. The first time you enter the PFSHOW
command (without parameters), the long form of the function key area is displayed. If you enter the
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  45

## Page 74

command again, the short form is displayed. If you enter the command once again, the keys are removed
from the display. Therefore, if you continue to enter the command, the different choices are toggled:
• Long form (default)
• Short form
• No display.
The form that you select affects all panels displayed in the session. The DM component updates the
system variable ZPFSHOW to represent the current state of the function key area form and saves the
value in the system profile.
PFSHOW ON displays the long form of the function key area.
PFSHOW OFF specifies that the function key area will not be displayed.
PFSHOW TAILOR displays a panel that lets you specify the set of function keys (primary, alternate, or all)
for which definitions are to be displayed and the number of keys per line to display in each function key
definition line.
PRINT
PRINT
Records a snapshot of the physical screen image in the list data set for subsequent printing.
For the PRINT, PRINT-HI, PRINTL, and PRINTLHI commands, a screen image can exceed 121 characters.
When it does, the line must be split when the output is being directed to a printer other than a 3800. The
line length is obtained from a user-modifiable specification on the ISPF Settings panel for the list data set.
The default length for printing is 121 characters.
Using PRINT commands with DBCS
The PRINT commands are affected in the DBCS environment as follows:
• DBCS character printing
Because shift-out and shift-in characters do not occupy positions on a printer, ISPF inserts a blank
character before each shift-out and after each shift-in.
• Fields affected by the OUTLINE keyword
Field-outlining information is embedded in the record as a set-attribute (SA) order. Each SA order
occupies three bytes. One SA is required to start field-outlining, one to end field-outlining, and one to
change field-outlining. Therefore, each affected field normally takes six additional bytes.
Thus, the record-length of print command output is larger than the screen width. The LIST file should be
large enough to contain the expanded records. If not, the output might not print correctly.
PRINTG
PRINTG
Allows you to send the information on the current logical screen to a Graphical Data Display Manager
(GDDM) graphics printer.
Note:
1. In split-screen mode, ISPF adds the split line to the top logical screen. If you issue the PRINTG
command from the top screen, the split line is printed along with the logical screen.
2. Also, in split-screen mode, PRINTG prints all data in the visible portion of the logical screen, but only
the graphics area data in the nonvisible portion of the logical screen.
ISPF System Commands
46  z/OS: z/OS ISPF User's Guide Vol I

## Page 75

If you use the other print commands (PRINT, PRINT-HI, PRINTL, and PRINTLHI) to print screen images
containing a graphics area, the part of the screen containing the graphics area prints as blanks.
If you issue the PRINTG command as a COMMAND option on a DISPLAY service request, only data already
defined to GDDM at the time the service request is issued will be printed. Any GDDM fields defined by the
dialog (using GDDM commands) before issuing the DISPLAY service request will be printed.
Before issuing the PRINTG command from a command line you must first have initialized the GDDM
graphic interface using the GRINIT service.
PRINTG does not provide return codes to a dialog; however, it does display completion or error messages.
For information about how to specify parameters related to using the PRINTG command, see the
information about Print Graphics Parms in the Settings (Option 0) topic of the z/OS ISPF User's Guide
Vol II.
PRINT-HI
PRINT-HI
Same as PRINT, except that high-intensity characters on the screen are printed with overstrikes to
simulate the dual-intensity display.
See “Using PRINT commands with DBCS” on page 46 for more information.
PRINTL
PRINTL
Causes a snapshot of the logical screen image to be recorded in the ISPF list file for subsequent printing.
In split-screen mode, the PRINTL command prints what would be seen if split-screen were not in effect.
See “Using PRINT commands with DBCS” on page 46 for more information.
PRINTLHI
PRINTLHI
Same as PRINTL, except that high-intensity characters on the logical screen are printed with overstrikes
to simulate the dual-intensity display.
See “Using PRINT commands with DBCS” on page 46 for more information.
PSCOLOR
PSCOLOR
Globally alters the color, intensity, and highlighting of point-and-shoot fields through a pop-up dialog.
Valid choices include:
  Color           Intensity        Highlight
  RED               HIGH            NONE
  PINK              LOW             BLINK
  GREEN                             REVERSE
  YELLOW                            USCORE
  BLUE
  TURQ (Turquoise)
  WHITE
To restore the ISPF default values, delete any new values you have entered (leaving the entry fields blank)
and press Enter, or select the Defaults field.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  47

## Page 76

RCHANGE
RCHANGE
Repeats the action of the previous CHANGE command (change one character string to another) (Edit and
View only).
REFACTD
REFACTD nnnnnnnn xx
Calls the personal data set list named nnnnnnnn and retrieves the data set in position xx. See “Command
interface to the personal list function” on page 134 for additional information.
REFACTL
REFACTL nnnnnnnn xx
Calls the personal library list named nnnnnnnn and retrieves the data set in position xx. See “Command
interface to the personal list function” on page 134 for additional information.
REFADDD
REFADDD nnnnnnnn
Updates the personal data set list named nnnnnnnn with the most recently referenced data set. See
“Command interface to the personal list function” on page 134 for additional information.
REFADDL
REFADDL nnnnnnnn
Updates the personal library list named nnnnnnnn with the most recently referenced library. See
“Command interface to the personal list function” on page 134 for additional information.
REFLISTD
REFLISTD xx
Calls the reference data set list dialog and retrieves the data set in position xx. See “Command interface
to the personal list function” on page 134 for additional information.
REFLISTL
REFLISTL xx
Calls the reference library list dialog and retrieves the library in position xx. See “Command interface to
the personal list function” on page 134 for additional information.
REFOPEND
REFOPEND
Calls the personal data set open dialog. See “Command interface to the personal list function” on page
134 for additional information.
ISPF System Commands
48  z/OS: z/OS ISPF User's Guide Vol I

## Page 77

REFOPENL
REFOPENL
Calls the personal library list open dialog. See “Command interface to the personal list function” on page
134 for additional information.
RESIZE
RESIZE
Increases the size of a pop-up window to fill the entire 3270 physical display area. The initial RESIZE
command increases the pop-up window to its maximum size, and the following RESIZE reduces the
window to its original size.
RETF
RETF
Retrieves commands from the command stack moving in the direction from the oldest command in
the command stack toward the most recent commands in the command stack. Forward retrieve (RETF)
retrieves the oldest command on the command stack, if RETF is entered immediately after a command is
executed, before performing a RETRIEVE. See “RETF command” on page 57 for more information.
RETP
RETP
Causes a pop-up panel to be displayed with a list of the last 25 commands in the retrieve stack. Retrieve
pop-up (RETP) enables you to select by number the command to be retrieved. The command selected
is retrieved to the command line, as it is when using other retrieve commands. You will not be able
to change the commands in the retrieve pop-up until the command is selected and retrieved to the
command line.
The RETP pop-up panel has an OPTIONS action bar choice that allows you to set the minimum number of
characters required to save a command in the retrieve stack and to choose whether to position the cursor
at the beginning or end of the retrieved command when the command is retrieved to the command line.
RETP displays the pop-up panel if the retrieve stack is empty, which allows the user to change the retrieve
options. See “RETP command” on page 57 for more information.
RETRIEVE
RETRIEVE
Repeatedly entering RETRIEVE causes the commands most recently entered from the primary input field,
usually the ZCMD field, to be displayed on the command line. The commands are displayed one at a time,
in the reverse sequence to which they were entered (last-in, first-out). This allows you to easily recall a
command for resubmission from the command line. You can edit the command before entering it if you
wish. See “RETRIEVE command” on page 55 for more information.
RETURN
RETURN
Causes an immediate return to a primary option menu or to the display from which you entered a nested
dialog. The RETURN command simulates repeated END commands, up to some appropriate stopping
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  49

## Page 78

point, without displaying intervening panels. See “Using the RETURN command” on page 57 for more
information.
RFIND
RFIND
Repeats the action of the previous FIND command (find one or more occurrences of a specified character
string) or the FIND part of the most recent CHANGE command (Browse, Edit, and View only).
RIGHT
RIGHT
Scrolls right. If your cursor is in a scrollable field, this scrolls towards the end of the field.
SAREA
SAREA
Displays the Status Area pop-up window.
SCRNAME
SCRNAME screen name
PERM
ON
OFF
Causes the logical screen in which the command is entered to be given the screen name specified. The
name can be any set of 2 to 8 characters that conform to member naming rules, except NEXT, PREV, LIST,
ON, and OFF.
PERM is an optional parameter to indicate that ISPF does not allow the SCRNAME parameter on a SELECT
statement, or the setting of the modifiable system variable ZSCRNAME, to override the value being
assigned by this SCRNAME command. The PERM setting lasts for the duration of the logical screen. After
you end the logical screen, the setting is no longer active.
SCRNAME ON causes the name that you specify for the screen to be displayed in the panelid area of the
screen. SCRNAME OFF removes the screen name from the visible display.
The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at the
start of the Title line. If more than one of these commands are specified, ISPF displays as many as will fit,
in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough room,
USERID is displayed, then PANELID, then SCRNAME.
SETTINGS
SETTINGS
Displays the ISPF Settings panel.
SPLIT
SPLIT
NEW
ISPF System Commands
50  z/OS: z/OS ISPF User's Guide Vol I

## Page 79

Causes the screen to be divided into two logical screens separated by a horizontal line or changes the
location of the horizontal line. If you have de-selected the Always show split line option in Settings, there
is no split line. See “Splitting the screen horizontally or vertically” on page 29 for more information.
SPLITV
SPLITV
On 3290 terminals, causes the screen to be separated into two vertical logical screens.
The SPLITV function is not active if the actual screen data display is more than 80 characters wide.
START
START
Starts a dialog in a new logical screen. If a logical screen does not exist, it will be created.
You can use the START command to:
• Issue a command from the ISPF command table; for example, START KEYLIST
• Issue a command with parameters (in single quotes; for example, START 'ISRROUTE BRI'
• Start a dialog; for example, START PANEL(ISRUTIL)
Note:
1. If you invoke START from a pull-down choice, the screen will be split where your cursor is located
within the pull-down.
2. This function does not change the limitation number of logical screens. If ISPF already has the
maximum number of screens when the START command is issued, the screen is re-split; that is, the
split line might move.
SWAP
SWAP
LIST
PREV
NEXT
screen name
n
When no parameters are given, moves the cursor to where it was previously positioned on the other
logical screen of a split-screen pair.
When operating in split-screen mode, pressing the SWAP key (F9) causes ISPF to ignore any entry on the
command line.
Entering SWAP LIST displays the ISPF task list. The task list displays this information about all of the
active logical screens:
• Screen ID (ZSCREEN)
• Screen name
• Panel ID
• Application ID
• Session type
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  51

## Page 80

You can select from this list the screen you want to display or start a new screen or application. The
screen you select replaces the screen from which you issued the command.
Using a listed parameter changes the display to the PREVious, NEXT, or specified logical screen.
PREV changes the focus or display to the next lower screen number until reaching 1, then wraps back to
32 or the last number used.
NEXT displays the next highest screen number until the last number used is reached (ISPF maximum 32,
your installation might vary), then will wrap back to number 1.
SWAP screen name changes the display or focus to the screen called screen name, if it is active. See the
SCRNAME command for more information about screen names.
SWAP n, where n is a number, changes the display or focus to the specified screen number, if it is active.
SWAPBAR
SWAPBAR
The list of logical screens can be activated by entering the SWAPBAR or SWAPBAR ON command on the
command line. The list is displayed on the last line of the physical screen.
The entry for each logical screen is the screen name if assigned or, if a screen name is not available, it
is the panel name of the current panel displayed for the logical screen. The entry for the active logical
screen has an asterisk (*) in the first character position and, if the name is 8 bytes long, the last character
is not displayed. Also, the alternate logical screen has a "-" in the first position and the 8th character is not
displayed.
The list remains active until you enter the SWAPBAR or SWAPBAR OFF command. The setting for the
SWAPBAR is maintained in the system profile member and applies across logons.
If the list is longer than the width of the screen, a ">" appears at the right of the list to indicate there are
more entries and you can scroll right by either positioning the cursor on the ">" and pressing Enter, or by
positioning the cursor on an entry and pressing PF11 which scrolls to the entry indicated. When the start
of the list is not displayed, a "<" is displayed at the left of the visible part of the list to indicate that you can
scroll left on the list by positioning the cursor on the "<" and Pressing Enter or by positioning the cursor on
an entry and pressing PF10.
The active logical screen can be changed by positioning the cursor on an entry and pressing Enter. This
logical screen then becomes the active logical screen. The SWAPBAR entries use the same physical
attribute as the action bar choices and, if in the options settings Tab to action bar choices is selected,
then tabbing to swapbar entries also occurs.
If the cursor is positioned on the swapbar entry for the currently active session and Enter is pressed, this
is treated the same as Enter being pressed within the active logical screen panel. When the SWAPBAR is
activated, the Always show split line option is deactivated and you are not able to reactivate it until the
SWAPBAR is deactivated.
If SWAPBAR is activated on a screen which is split and the lower panel of the split screen does not
contain enough rows to allow the SWAPBAR to be displayed, the necessary rows to allow the SWAPBAR
to be displayed will be removed from the upper panel display and added to the lower panel display.
You are able to customize the SWAPBAR settings. You can:
• Choose to have a separator line between the logical screen and the SWAPBAR.
• Set the colour of the SWAPBAR.
• Set the highlighting of all the fields within the SWAPBAR (for example, reverse video, underscore or
blinking attribute).
• Set the colour and highlighting attributes of an individual entry (a logical screen) with the SWAPBAR.
• Save the settings that apply to the SWAPBAR as a whole in your system profile, so that these settings
apply to future ISPF sessions (until you modify them).
ISPF System Commands
52  z/OS: z/OS ISPF User's Guide Vol I

## Page 81

To set the SWAPBAR customization settings, entering the command SWAPBAR / on the command line.
The panel ISPTLCPN is displayed. You can now set the customization attributes. The changes take effect
once you exit the panel. The setting for a separator line and the colour and highlighting settings for the
SWAPBAR as a whole are saved in your system profile and apply to future ISPF sessions. Any changes to
individual entries in the SWAPBAR only apply to the current ISPF session (that is, the session where the
SWAPBAR / command was entered), and are also lost if the logical screen should terminate and recover.
To clear the settings for a logical screen swap to that screen, enter the SWAPBAR / command and enter D
in the option field to clear the current session.
SYSNAME
SYSNAME ( ON
OFF
)
SYSNAME ON causes the name that you specify for the screen to be displayed in the panelid area of the
screen. SYSNAME OFF removes the system name from the visible display.
The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at the
start of the Title line. If more than one of these commands are specified, ISPF displays as many as will fit,
in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough room,
USERID is displayed, then PANELID, then SCRNAME.
TOP
TOP
Alias for the UP MAX command. Scrolls to the top of the data.
TSO
TSO
Allows the user to enter a TSO command, CLIST, or REXX command procedure.
Do not enter these commands after the TSO command:
• LOGON, LOGOFF
• ISPF, PDF, ISPSTART, and SPF
• TEST
• Commands that are restricted by TSO
You can enter a CLIST or REXX name after the TSO command, but these restrictions apply:
• The CLIST or REXX command procedure cannot invoke the restricted commands shown in the
preceding list.
• Restrictions that apply to CLIST attention exits are described in z/OS ISPF Dialog Developer's Guide and
Reference.
• TERMIN command procedure statements cause unpredictable results.
TSOCMD
TSOCMD
Displays the ISPF Command Shell panel.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  53

## Page 82

TUTOR
TUTOR
panelid
Calls the ISPTUTOR program to display specified tutorial panels.
To display a particular tutorial panel, enter the TUTOR command along with the panel identifier of the
desired tutorial panel as a parameter.
If you issue the TUTOR command without a parameter, the general tutorial help panel (ISP00000) is
displayed.
UDLIST
UDLIST
Enables you to build a z/OS UNIX directory list from any command line. You can specify either a personal
data set list name or a pathname for a z/OS UNIX directory. When a personal data set list name is
specified, the pathname entries in the list are used to build the displayed directory list.
By issuing the command with no parameters, you cause a list of available personal data set lists to be
displayed. You can then select the personal data set list to be used to build the displayed directory list.
This panel also provides a field to enter the pathname of the directory you want to list.
Note: z/OS UNIX pathnames are case sensitive. In general, ISPF command line fields are defined with
the CAPS(ON) attribute which causes data entered in the command line to be converted to uppercase.
Consequently, pathnames specified as a parameter on the command line are, in general, converted to
uppercase before being passed to the UDLIST command. This can result in the requested directory not
being found. If this occurs ISPF converts the specified pathname to lower case and re-issues the find for
the directory. If the requested directory is still not found, enter the UDLIST command without a parameter
and specify the case-sensitive directory pathname in the field on the personal lists selection panel.
UP
UP
Scrolls toward the top of the data.
USERID
USERID ( ON
OFF
)
USERID ON displays your user ID in the panelid area of the screen. USERID OFF removes your user ID
from the visible display.
The commands SYSNAME, USERID, PANELID, and SCRNAME share the same 17-character area at the
start of the Title line. If more than one of these commands are specified, ISPF displays as many as will fit,
in this order of priority: SYSNAME, if specified, is always displayed. Then, as long as there is enough room,
USERID is displayed, then PANELID, then SCRNAME.
WINDOW
WINDOW
Moves a pop-up that is currently displayed.
ISPF System Commands
54  z/OS: z/OS ISPF User's Guide Vol I

## Page 83

Type WINDOW at the command line. Then move the cursor to the position on your screen where you want
the pop-up to appear. Press Enter.
If WINDOW is assigned to a function key, move the cursor to the position on your screen where you want
the pop-up to appear, and press the function key.
You can move dialog pop-ups, help pop-ups, and message pop-ups.
If more than one pop-up is displayed on your logical screen, only the active (or most recent) pop-up will
move.
A pop-up can only be moved within the logical screen from which it originated.
The position of the cursor specifies the new location for the upper left corner of the pop-up. If the pop-up
will not fit on the terminal screen at its specified new location, ISPF positions the pop-up to fit on the
screen. The cursor will then appear in the same relative position it was in before the pop-up was moved.
ZCLRSFLD
ZCLRSFLD
If the cursor is on a scrollable input field, that field is cleared to blanks. If the field is part of a TABLE
DISPLAY operation a row select will occur when ENTER is next pressed. If the field is not scrollable the
command is passed to the application.
ZKEYS
ZKEYS
Displays a panel that lets you view and change the current function key variables. This command is
equivalent to selecting the Global PF Key settings choice from the Function keys pull-down on the ISPF
Settings panel.
Using the RETRIEVE, RETF, and RETP commands
This topic describes how to use the RETRIEVE, RETF, and RETP commands.
RETRIEVE command
The RETRIEVE command causes the most recently entered command to be displayed on the command
line. If the command recalled by RETRIEVE is longer than the current primary input field, ISPF truncates
the command to the size of the primary input field for display purposes. Only the data displayed in the
primary input field is processed and stored in the command retrieval stack when you press Enter or a
function key. However, the original command retains its full length in the retrieval stack.
If the current panel has no input fields, then the size of the primary input field is zero and the
retrieved command is not displayed. Normal stack processing occurs, however, and the internal pointer is
incremented to the next saved command. This can result in an unexpected command being recalled when
RETRIEVE is issued on a subsequent panel that has input fields.
If you issue the RETRIEVE command when the stack is empty, ISPF presents you with a blank command
line with the cursor in the first position. If the stack is not empty, ISPF places the cursor immediately
following the retrieved command.
If you are in the process of recalling a string of commands by issuing successive RETRIEVE commands,
you can cause ISPF to recycle to the top of the command retrieval stack by pressing Enter when the
primary input field (normally the command line) is blank.
When you are operating in split-screen mode, one stack retains commands for all logical screens.
There are five cases for which ISPF does not retain an entered command for retrieval:
• Commands entered using attention fields, such as cursor-select fields.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  55

## Page 84

• Commands entered through the use of function keys. This includes any portion of a compound
command that results from pressing a function key. For example, if you key PAGE into the primary
input field and then press the function key set to the DOWN command, only the PAGE portion of the
DOWN PAGE command is retained as a single element in the retrieval stack. The entire character string
entered from the primary input field in conjunction with a function key is always retained, whereas any
portion of the command resulting from the function key value is not retained.
• The RETRIEVE command, if entered as a single command. If RETRIEVE is one of the commands of a
chain being processed by ISPF, the entire chain is placed on the retrieval stack. However, processing
of the command chain ends when ISPF interprets the RETRIEVE command and displays the next
command in the stack. Any commands following RETRIEVE in the chain are not processed.
RETRIEVE can be part of a stack element as a parameter of another command. For example, you might
enter FIND RETRIEVE as a command.
• Commands entered on the COMMAND option of the DISPLAY service.
• Jump function (extended return) commands entered from a nondisplay field.
You can issue any retrieved command, as is, while it is being displayed, or you can edit the command line
and then issue the modified version.
Command retrieval works on a last-in, first-out basis. For example, assume that the last three commands
you have issued are PRINT, DOWN, and RIGHT, in that order. Now suppose that you want to again issue
the PRINT command. Assuming that F12 is set to RETRIEVE, the sequence of operations is:
1. Press F12. RIGHT displays on the command line.
2. Press F12 again. DOWN displays on the command line.
3. Press F12 a third time. PRINT displays on the command line.
4. Press Enter.
You can also use the RETRIEVE command to check and correct errors made in keying commands. For
example, suppose that you mistakenly enter PFSHOW TAYLOR. When ISPF advises you that TAYLOR is not
a valid parameter, you would:
1. Press F12. PFSHOW TAYLOR displays on the command line.
2. Type over the Y with an I.
3. Press Enter.
Each ISPF session supports only one command retrieval stack, to be shared by all logical screens. The
number of commands that ISPF saves for retrieval depends on:
• The size of the stack area allocated for this purpose by the installation. See z/OS ISPF Planning and
Customizing for information on changing the size of the stack area allocated for RETRIEVE command
processing.
• The lengths of the individual command lines that are saved.
As a command is entered, it goes to the top of the stack, pushing all other commands down. If there is not
enough room at the bottom of the stack to hold the entire bottom command, it is dropped from the stack.
Duplicate commands are allowed in the stack, except when the command being entered is a duplicate of
the command at the top of the stack. All command lines (except the RETRIEVE command) are placed in
the stack as entered, regardless of validity. Actually, these commands can be any character string, up to
255 bytes each, entered from the screen's primary input field (not necessarily the ZCMD field).
Jump function commands are stored in the stack unless they are entered from a nondisplay field,
regardless of whether the field is the primary input field or not.
If the RETRIEVE command is repeatedly entered until the bottom command in the stack displays, issuing
the RETRIEVE command once more causes the command at the top of the stack to be displayed again. To
force a return to the top of the stack, clear the command field and press Enter. Then, the next RETRIEVE
command causes the command line to be set to the command at the top of the stack.
ISPF System Commands
56  z/OS: z/OS ISPF User's Guide Vol I

## Page 85

RETF command
The forward retrieve (RETF) command recalls commands from the command retrieval stack from the
oldest command in the stack towards the most recent commands in the stack. This is useful when you
RETRIEVE too many times in an attempt to retrieve a specific command. RETF enables you to return to
the desired command without having to cycle through the entire retrieval stack.
RETP command
The retrieve pop-up (RETP) command causes a pop-up panel to be displayed with the last 25 commands
in the command retrieval stack listed. You can select the command you want to retrieve by number.
The selected command is retrieved to the command line. When using the RETP command, these
considerations apply:
1. If a command in the command retrieval stack is too long to fit in the retrieve pop-up, the last visible
character of the command is changed to a > to show that some characters are not displayed. However,
the entire command is retrieved to the command line when it is selected to be retrieved.
2. The default for the minimum number of characters is one, so any command entered is saved on the
retrieval stack. The user has the option of setting the value from 1-99 for the minimum number of
characters to save. Therefore, if you select three characters for the minimum number to be saved in
the retrieval stack and a one- or two-character command is entered, it is not added to the retrieval
stack. This prevents short commands that can be easily retyped from taking up space in the retrieval
stack. Changing the minimum number of characters to save in the retrieval stack does not affect
commands already in the retrieval stack. This setting is saved in the variable ZRETMINL, which is
saved in the user's ISPF system profile table ISPSPROF and across ISPF invocations.
3. The default for the cursor position when a command is retrieved is at the end of the command. The
cursor position setting is saved in the variable ZRETPOSC, which is saved in the user's ISPF system
profile table ISPSPROF and across ISPF invocations.
Using the RETURN command
The RETURN command causes the immediate return to a primary option menu or to the display from
which you entered a nested dialog. When a RETURN command is entered, the DM component takes this
action:
1. It simulates the END command on the panel that is currently displayed; that is, the DISPLAY or
TBDISPL service returns a code of 8.
2. For subsequent requests, made through the DISPLAY or TBDISPL service, for display of a different
panel, the panel is not displayed, and a return code of 8 is issued by the service.
3. However, when two consecutive display requests name the same panel, normal operation of the
DISPLAY and TBDISPL services is restored and processing proceeds as though RETURN had not been
entered. The DM component decides whether to proceed. Generally, because RETURN signals the
application user's desire to end the current processing, a developer can limit processing after the
RETURN is received to clean up and do final processing before returning control to the dialog element
from which the function was started.
4. If two consecutive requests do not specify the same panel, processing continues in the mode
described in item “2” on page 57 until control is returned to a primary option menu or a nested
dialog completes. Then, normal operation of the DISPLAY and TBDISPL services is restored.
It might be necessary to suspend processing of a panel temporarily so that other panels can be displayed.
Issue a CONTROL DISPLAY SAVE request to save the contents and control information of the panel whose
processing is to be suspended. Before resuming the processing of this panel, issue CONTROL DISPLAY
RESTORE to reinstate the contents and control information for the panel. If non-ISPF screens have been
displayed, issue CONTROL DISPLAY REFRESH to clear the screen.
This mode of operation continues until either a primary option menu is encountered or a nested dialog
completes. If a primary option menu is encountered, it is displayed. If a nested dialog completes, the
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  57

## Page 86

panel from which it was invoked is redisplayed. This panel is exactly as you last saw it, except that the
command field is blank. In either case, this completes the action of the RETURN command.
Note: A nested dialog is one invoked from any panel by a SELECT action command. The HELP and KEYS
commands invoke nested dialogs. In addition, the TSO system commands invoke nested dialogs when
they are used to execute a CLIST procedure that displays panels through ISPF services.
If a dialog function needs to distinguish between END and RETURN, it can do so in one of these ways:
• If the panel was defined using the panel definition statements, upon return from the DISPLAY or
TBDISPL service, with a return code of 8, the function can examine variable ZVERB in the shared pool. It
contains either END or RETURN.
• If the panel was defined using the DTL, upon return from the DISPLAY or TBDISPL service, with a
return code of 8, the function can examine variable ZVERB in the shared pool. It contains either EXIT or
CANCEL.
• Upon return from the SELECT service when the PANEL keyword was specified, the dialog function can
examine the return code from SELECT. Return code 0 indicates that the END command was entered
on the selected menu panel. Return code 4 indicates that the RETURN command was entered on the
selected menu panel or on some lower-level menu.
Using the jump function
The jump function allows you to go directly to any valid option from the primary option menu currently in
effect. See z/OS ISPF Dialog Developer's Guide and Reference for information about coding primary option
menus. To use the jump function, enter the option on the command line or in the command field of any
panel, preceded by an equal sign and followed by a blank. For example:
Command ===> =3.1
takes you directly to the first suboption of option 3 on the primary option menu in effect.
The action is as follows:
• If not entered on a primary option menu, the jump function causes repeated END commands to be
simulated until a primary option menu is encountered. What follows the equal sign is then used on
the primary option menu, and pressing of the Enter key is simulated. The primary option menu is not
displayed.
• If entered on a primary option menu, the jump function equal sign is ignored and the specified option is
selected.
Unlike the RETURN command, the jump function is not affected by nested dialogs. For example, from the
ISPF Edit option, you enter a HELP command to enter the tutorial. Then from the tutorial, you enter =1.
This causes the tutorial to end, Edit to end, and primary option 1 to be started.
For convenience, you can enter a jump function in two other places:
• Any field that is preceded by an arrow. The arrow must consist of at least two equal signs followed by a
greater-than sign (==>). Also, the arrow must immediately precede the input attribute byte.
• Any field preceded by leader dots (that is, ... or . .). ISPF looks at the three characters preceding the
field; they must be either three consecutive dots or two dots separated by a blank.
The command field is the only field that can be initialized to =n by the dialog and have the jump function
recognize it. Modifying the ZCMD field in the )PROC or )INIT section can affect jump function operation.
If ISPF encounters an error during jump function processing, the processing stops with the jump function
in error displayed on the command line, unless that function was entered from a nondisplay field.
Because a jump request generally signals a user's desire to end the current processing, the dialog
developer must limit processing to cleaning up and completing processing before returning control to the
selection in the jump request. Otherwise, the dialog developer can cancel the jump request/return mode
by providing two consecutive displays with the same panel name.
ISPF System Commands
58  z/OS: z/OS ISPF User's Guide Vol I

## Page 87

The jump function can be entered with the RETURN command or RETURN function key. For example, you
type =2 and then press the RETURN function key rather than pressing Enter. The result is the same as if
you had typed =2 and pressed Enter.
See “Using the Exit option (X) with the jump function” on page 10 for more information.
Using the scrolling commands
You can use the scrolling commands if the dialog function invokes the DISPLAY service for panels with
scrollable areas or scrollable dynamic areas, the table display service (TBDISPL), or the interfaces to the
PDF component VIEW, BROWSE, and EDIT services. During processing of the tutorial, ISPF interprets
these commands as follows:
UP (F7/19)
Same as the UP command
DOWN (F8/20)
Same as the SKIP command
LEFT (F10/22)
Same as the BACK command
RIGHT (F11/23)
Same as the Enter key (display the next page).
When scrollable data is displayed, scrolling enables you to move the screen window up, down, left, or
right across the information. When the cursor is within a scrollable field, scrolling enables you to move left
or right within the variable data. Only up and down scrolling is allowed for table displays and scrollable
areas.
When scrolling is allowed, a scroll amount is commonly displayed at the top of the screen (line 2). This
amount determines the number of lines, or columns, scrolled with each use of a scroll command. To
change the scroll amount, move the cursor to the scroll field and type over the displayed amount. Valid
scroll amounts are:
ZXSMIN-ZXSMAX
A value between ZXSMIN and ZXSMAX where ZXSMIN and ZXSMAX are system profile variables
containing the minimum and maximum scroll values as defined in the configuration table. Can be in
the range of 0 to 9999999. When the value is entered in the scroll field the user is limited to entering a
4-digit value but when the value is entered in the command field it can be any value between ZXSMIN
and ZXSMAX (inclusive).
Note: If you specify a scroll amount of 0, no scrolling occurs.
PAGE
Specifies scrolling by one page.
For scrolling purposes, a page is defined as the amount of information currently visible on the logical
screen. Function key definition lines are not a part of the page. In split-screen mode, for example, a
Browse display might have 12 lines by 80 columns of scrollable data. In this case, a scroll amount of
PAGE moves the text up or down by 12 lines, or right or left by 80 columns. If the cursor is within a
scrollable field, PAGE will move the text right or left the equivalent of the display field length.
DATA
For up and down scrolling, specifies scrolling by one line less than a page. For left and right scrolling,
it is one column less than a page. Within a scrollable field, it is one column less than the display field
length.
HALF
Specifies scrolling by half a page. Within a scrollable field, it is half the display field length.
MAX
Specifies scrolling to the top, bottom, left margin, right margin, beginning of field or end of field,
depending upon which scrolling command is used and the current cursor position. For scrollable
fields, the maximum right position is the field length minus the display length and the maximum left
position is 1.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  59

## Page 88

CSR
Specifies scrolling based on the current position of the cursor. The line or column indicated by the
cursor is moved to the top, bottom, left margin, or right margin of the screen, depending upon which
scrolling command is used. If the cursor is not in the body of the data or if it is already positioned at
the top, bottom, left margin, or right margin, a full-page scroll occurs.
Note: Scroll amount is not used for scrollable areas.
The current scroll amount is saved in the application profile. There are three scroll amount values: one
for Browse (ZSCBR), one for Edit and View (ZSCED), and one for member lists (ZSCML). When you type
over the scroll amount, the new value remains in effect until you change it again. The value MAX is an
exception. Following a MAX scroll, the scroll amount reverts to its previous value.
The scroll amount field is optional. If the input field following the command field in the panel body is
exactly four characters long, it is assumed to be the scroll amount field. If there Otherwise, the system
variable ZSCROLLD, which can be set by the dialog, is used to determine the default scroll amount. is no
scroll amount field and ZSCROLLD has not been set, the default is PAGE.
When you enter a scroll request, variables ZSCROLLA, ZSCROLLN, and ZSCOLNL are set. ZSCROLLA
contains the value of the scroll amount field (MAX or CSR, for example). ZSCROLLN and ZSCROLNL
contain the number of lines or columns to scroll, computed from the value in the scroll amount field or
entered as a scroll number. For example, if a dialog is in split-screen mode and if 12 lines are currently
visible and a user requests DOWN HALF, ZSCROLLN and ZSCROLNL each contain a value of '6'. ZSCROLLN
can support values up to '9999'. If a scroll number greater than '9999' is specified, ZSCROLLN is set to
a value of '9999'. ZSCROLNL can support values up to '9999999'. The system variable ZVERB contains
the scroll direction, DOWN in this case. If ZSCROLLA has a value of MAX, the values of ZSCROLLN and
ZSCROLNL are not meaningful.
You can also use any valid scroll amount as part of the scroll command. For example, type:
Command ===> UP 3
and press Enter, or type:
Command ===> 3
and press the UP function key. Either form results in a temporary, one-time override of the scroll amount.
If ISPF does not recognize the value specified on the command line as a valid scroll amount, such as
PAGE, DATA, HALF, MAX, CSR, or a positive integer, the value is interpreted as a command and passed to
the function in control.
Using the EXPAND command
The expand panel displays the variable in a scrollable dynamic area. Standard up and down scrolling is
supported. You can display the variable in character and hexadecimal using the primary command shown.
The setting will be remembered for subsequent expand processing.
HEX ON/OFF
Turn hexadecimal display on and off.
Using command tables to define commands
ISPF implements system, user, site and application commands through the use of command tables.
A system command table (ISPCMDS) is distributed with ISPF in the table input library. An application can
provide an application command table by including a table named xxxxCMDS in its table input library,
where xxxx is a 1- to 4-character application ID. You can also add up to 3 user command tables and
up to 3 site command tables to the ISPF Configuration table. This is a permanent place for your set of
user-defined commands. When IBM updates the ISPF command table, you do not need to re-add your
commands. By setting the Before or After option, you can search the site command tables either before
ISPF System Commands
60  z/OS: z/OS ISPF User's Guide Vol I

## Page 89

or after the ISP command table. The default option is Before. If the application's table input library is
defined with the LIBDEF service, the LIBDEF must be active when the SELECT service call that invokes the
application is issued, and the PASSLIB parameter must be specified.
You can define an application command table using either:
• The command table utility described in the Command Table Utility (Option 3.9) section of the z/OS ISPF
User's Guide Vol II.
• The Dialog Tag Language (DTL) and ISPF conversion utility. See the z/OS ISPF Dialog Tag Language
Guide and Reference for the tags you must use.
When a user enters a command, the DM component searches the application command table (if any),
then the user command tables (if any), then the site command tables (if any), and finally the system
command table, ISPCMDS. This is the default search order, which assumes the option Before for the site
command tables. If you choose the option After for these tables, they are searched after ISPCMDS. If it
finds the command, action is taken immediately. If it does not find the command in the application or
system tables, the command is passed to the dialog, unaltered, in the command field. The dialog must
then take appropriate action.
Command table format
A command table is an ISPF table in which each row contains the specification for one command. Each
column contains a variable for the command. The variables are:
ZCTVERB
Specifies the name of the command. A command name must be from 2-8 characters long and must
begin with an alphabetic character. Note that the terms command name and command verb are
synonymous and are used interchangeably.
ZCTTRUNC
Specifies the minimum number of characters that you must enter to find a match with the command
name. If this number is zero or equal to the length of the name, you must enter the entire name. This
number must not be one, or be greater than the length of the name.
ZCTACT
Specifies the action to be performed when the command specified in ZCTVERB is entered. Can be up
to 240 characters.
ZCTDESC
Contains a brief description of the purpose of the command. This variable is optional. It is not used by
the DM component in processing the command, but it is displayed by the command table utility. The
description is limited to 80 characters.
The dialog manager treats ZCTVERB, ZCTTRUNC, ZCTACT, and ZCTDESC as defined function variables.
They are not accessible to dialogs.
The valid actions that can be performed (ZCTACT) are:
SELECT
Followed by selection keywords causes the selected dialog (command, program, or menu) to be given
control immediately.
ALIAS
Followed by another command and any parameters allows specification of command aliases.
PASSTHRU
Causes the command to be passed to the dialog instead of continuing to search the system table.
SETVERB
Causes the command to be passed to the dialog with the command verb stored in ZVERB separately
from the parameters. The ISPF system commands distributed with the product that have SETVERB as
an action are not always passed through to the dialog. See “Passing commands to a dialog function”
on page 65 for further discussion.
NOP
Causes the command to be inactive. ISPF displays an inactive command message in this case.
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  61

## Page 90

Blank (no action)
Causes the table entry to be ignored. Scanning continues, searching for additional entries with the
same name.
A variable name
The name begins with an ampersand and can be one of the actions described in this list. This allows
dynamic specification of command action.
Additional action keywords are used to indicate system commands for which special processing is
required. These are CURSOR, PRINT, PRINTG, PRINT-HI, PRINTL, PRINTLHI, SPLIT, SPLITV, SWAP, and
RETRIEVE. Although these are valid actions, they are intended for use only in the system command table
distributed with ISPF and are intended to be used only with the associated command verb. They are not
intended for use in application command tables.
Customizing the ISPCMDS command table
Use these steps to customize your ISPCMDS command table:
1. Copy the ISPCMDS into a data set concatenated before the '*.SISPTENU' data set in the ISPTLIB DD
statement.
• Name the new member using a unique prefix of up to 4 characters, for example: RSMCMDS or
MOD1CMDS.
• Allocate the copied-to data set to the ISPTABL DD card. You can use this CLIST to do a LIBDEF
against ISPTABL if you have a ISPTABL DD allocated:
PROC 0
ISPEXEC LIBDEF ISPTABL DATASET ID(the_dataset_name)
WRITE &LASTCC
END
If your logon procedure does not allocate an ISPTABL DD card:
PROC 0
ALLOC F(ISPTABL) DA(the_dataset_name)
END
2. Using option 3.9, customize this member with your new commands.
• Option 3.9 will search the ISPTLIB DD for this member.
• Option 3.9 will save (UPDATE) this customized member to the output data set pointed to by
ISPTABL.
3. After you have customized this member, you can use option 3.1 or 3.4 member list to rename this
member to ISPCMDS.
4. Exit ISPF.
• This will nullify the LIBDEF on ISPTABL.
5. Re-invoke ISPF.
• When ISPF searches the ISPTLIB DD concatenation, your customized ISPCMDS will be found first.
SELECT action commands temporarily invoke a new dialog
A SELECT action command can be specified in a command table. The action is coded exactly the same as
for the SELECT service. All SELECT keywords are valid, including NEWAPPL.
The selected dialog is started immediately when a SELECT action command is entered on the command
line of any panel. This temporarily suspends the current dialog. When the selected dialog completes, the
screen is refreshed and the suspended dialog resumes.
ISPF System Commands
62  z/OS: z/OS ISPF User's Guide Vol I

## Page 91

Table 5. Examples of SELECT action commands
ZCTVERB ZCTTRUNC ZCTACT
UPDATE 0 SELECT PGM(PQRUPDT) PARM(&ZPARM)
PREPARE 4 SELECT CMD(XPREP &ZPARM) NEWPOOL
MENU 4 SELECT PANEL(&ZPARM)
In the example, the ZCTTRUNC variable indicates that the UPDATE and MENU command names cannot be
truncated. PREPARE, however, can be truncated to PREPAR, PREPA, or PREP. The functions and keywords
in the ZCTACT field indicate the actions that the commands perform.
The ZPARM variable that appears in the SELECT keywords indicates that command parameters are to be
substituted at that point. For example, if these commands were entered:
   ===> UPDATE BLDG DEPT NAME
   ===> PREPA LOG LISTING
   ===> MENU PQRMENU1
these SELECT actions would result:
   SELECT  PGM(PQRUPDT) PARM(BLDG DEPT NAME)
   SELECT  CMD(XPREP LOG LISTING) NEWPOOL
   SELECT  PANEL(PQRMENU1)
ZPARM, a dummy variable, is used only to substitute user-entered parameters into SELECT action
commands. It is not stored in a variable pool and is not accessible to dialogs.
Note:
1. Take care with ACTIONs that use ZPARM, as the ISPF parser will add a matching parenthesis if one
appears to be missing. Consider an entry of "SELECT CMD(%CMD &ZPARM) NEWAPPL(ISR)". If "(XYZ"
is passed then the command will receive "(XYZ) NEWAPPL(ISR)" as a parameter.
2. Use of SELECT action commands can cause recursive entry into dialog functions, which the DM
component allows. The dialog developer should either design functions for recursive use or display a
message if a user attempts to reenter a nonrecursive function.
The ISPF DISPLAY and TBDISPL services can be used recursively. The current display environment is
automatically saved whenever a SELECT action command is entered and is restored upon completion of
the command.
Assigning command aliases
A command alias is an alternate way of expressing a command. For example, you might assign to the
command UP MAX an alias of TOP to make it easier to remember and to issue. In the case of a command
that includes lengthy parameters, using an alias can be a much more efficient way of entering the
command. Also, using aliases can be helpful for writing dialogs in languages for which single words can
meaningfully replace multiword command-parameter expressions. Normally, alias entries are used in an
application command table to refer to system commands, which might or might not include parameter
fields. Issuing the command or its alias causes the same result.
An alias must precede, in the command table, any reference to the command to which it refers. You can
establish an alias by setting values in two command table variables. Set:
• ZCTVERB to the value you wish to use as the alias for an existing command
• ZCTACT to the keyword ALIAS followed by the command, including any parameters, for which you are
establishing the alias. Thus, the value of the ZCTACT variable can be either a single-word command,
such as HELP, or it can be a multipart command, such as UP MAX.
You can set the value of ZCTTRUNC in the command table to the minimum number of characters of the
alias name that must be entered. For example, for the alias FORWARD, if you set ZCTTRUNC to a value
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  63

## Page 92

of 3, issuing the first three characters (or more) has the same effect as issuing FORWARD. If you assign a
value of 0 in the ZCTTRUNC field, the complete alias name must be issued.
The maximum length of the value you can specify in ZCTACT, including the keyword ALIAS, followed
by a blank, and the command verb plus any parameters, is 240 characters. This leaves a maximum of
234 characters for the command, at least one blank following the command, and any parameters. ISPF
interprets up to the first 8 characters in the command name. ISPF issues an error message for command
names that do not contain from 2-8 characters.
Any parameters included in the ZCTACT command table field take precedence over any parameters
included with that command's alias issued from a command line. Thus, if you issue a command alias that
includes parameters, ISPF:
• Recognizes the command alias verb
• Ignores the parameters you specified with the alias
• Substitutes the parameters included with the command verb in ZCTACT.
If the command verb in ZCTACT does not include parameters, ISPF accepts parameters specified with the
command's alias from a command line. This can be useful when a command's parameters do not fit into
the 240 character spaces available in ZCTACT.
You can create a chain of command-parameter aliases in a command table as long as the result is a valid
executable action. The last command verb and parameter values that ISPF encounters in the alias chain
within the command table are the ones that are executed. The command verb and the parameter values
do not necessarily come from the same table entry.
Table 6. Examples of a chain of command-parameter aliases
ZCTVERB ZCTTRUNC ZCTACT
EASYKEY 0 ALIAS CMD PARM1 PARM2
CMD 0 ALIAS CMD1 PARM3
CMD1 0 ALIAS CMD2
In this example, if you entered EASYKEY from a command line, the command that would ultimately be
executed would be CMD2 PARM3.
Table 7. Some more examples of defining  alias values
ZCTVERB ZCTTRUNC ZCTACT
QUIT 0 ALIAS END
FORWARD 3 ALIAS DOWN
*TOP 0 ALIAS BACKWARD MAX
*BACKWARD 0 ALIAS UP
*ENDFILE 4 ALIAS LOW
*LOW 0 ALIAS DOWN MAX
*These four entries represent two-level chaining.
This example defines QUIT as an alias of END, FORWARD as an alias of DOWN, and so on. For example, if
you enter QUIT, the system responds as though you had entered END.
Looking at the two-level chaining examples, if you enter TOP, ISPF responds as though you had entered
UP MAX. This is because, at the second level when BACKWARD is replaced with UP, there is no second-
level parameter to replace MAX. In the case of ENDFILE, ISPF responds as though you had entered DOWN
MAX. ISPF replaces the verb LOW with DOWN and the blank parameter value with MAX.
Note: Command aliases included with ISPF in table ISPCMDS include TOP (UP MAX), BOTTOM (DOWN
MAX), BACKWARD (UP), and FORWARD (DOWN).
ISPF System Commands
64  z/OS: z/OS ISPF User's Guide Vol I

## Page 93

Overriding system commands
An application can override any system command simply by including the same command name in the
application command table.
Table 8. Examples of overriding a system command
ZCTVERB ZCTTRUNC ZCTACT
HELP 0 PASSTHRU
TSO 0 NOP
In this example, the dialog has overridden both the HELP and TSO commands. During ISPF processing,
if you enter HELP, the command is passed to the dialog function in control, which determines the action
to be taken. The action specified for the TSO command is NOP, which disables the TSO command. ISPF
displays an inactive command message when a NOP action command has been processed.
Passing commands to a dialog function
Any command that is not found in the application or system command table is passed, unaltered in the
command field, to the dialog. This occurs regardless of whether the command was typed in the command
field or entered by use of a function key or the attention field.
You can force a command to be passed to the dialog, even if the command exists in the command table,
by typing a greater-than symbol (>) in front of the command.
Any command in the command table that has an action of PASSTHRU is processed as though the
command were not found in the table. It is passed in the command field to the dialog.
Commands can also be passed to the dialog using the SETVERB action. This action causes the dialog
manager to separate the name from the command parameters, if any. The command is stored in variable
ZVERB, which is in the shared pool. The left-justified command parameters are passed in the command
field to the dialog.
Table 9. Examples of passing commands to the dialog
ZCTVERB ZCTTRUNC ZCTACT
QUERY 0 SETVERB
The verb QUERY is stored in variable ZVERB and the character string, such as DEPT 877 in the examples
shown, is passed in the command field.
These actions produce the same results:
• Typing QUERY DEPT 877 in the command field and pressing Enter.
• Typing DEPT 877 in the command field and pressing a function key that has been equated to the
character string QUERY.
• Pressing a function key that has been equated to the character string QUERY DEPT 877.
• Using the cursor-select key to select an attention field that contains the character string QUERY DEPT
877.
These system commands, distributed with the DM component, are defined as SETVERB action
commands:
END     UP
RETURN  DOWN
RFIND   LEFT
RCHANGE RIGHT
ISPF System Commands
Chapter 3. Using commands, function keys, and cursor selection  65

## Page 94

The ZVERB variable can be used to distinguish between END and RETURN. The effect of END and RETURN
on the DISPLAY service is the same because RETURN is used to simulate repeated END commands, until
a primary option menu is reached.
RFIND and RCHANGE are used only by ISPF View, Browse, and Edit. Thus, these commands are not
passed back to a user dialog in ZVERB.
The commands UP, DOWN, LEFT, and RIGHT are only active when a scrollable panel is displayed. Use of
these commands from a nonscrollable panel results in a command is not active message.
Specifying command actions dynamically
You can specify a command action dynamically (as part of function processing) by the use of a dialog
variable. A variable action can be used to share commands, such as UP, DOWN, LEFT, and RIGHT, with the
DM component. It can also be used to enable or disable commands during certain points in the dialog.
Suppose, for example, an application command table includes these entries:
Table 10. Examples of entries in an application command table
ZCTVERB ZCTTRUNC ZCTACT
UP 0 &SCRVERT
DOWN 0 &SCRVERT
You can use the variable SCRVERT to dynamically control the action of the UP and DOWN vertical scroll
commands as follows:
• If SCRVERT is set to NOP, the commands are not available.
• If SCRVERT is set to PASSTHRU, the commands are passed to the dialog.
• If SCRVERT is set to blank, command scanning continues. In this case, the system definitions for UP and
DOWN in the system command table take effect.
• If SCRVERT is set to an action that is not valid, the commands are not available, as in NOP.
For this particular example, setting SCRVERT to SETVERB would have the same effect as setting it to
blank, because UP and DOWN are defined in the system command table as SETVERB action commands.
If the dialog overrides or shares the use of the scroll commands, it becomes that dialog's responsibility to
ensure that the commands have been redefined with an action of blank, or with SETVERB. This must be
done before starting any ISPF function that requires View, Browse, Edit, and Table Display. The same rule
applies to the RFIND command used by Browse and Edit and the RCHANGE command used by Edit.
Using function keys
Under ISPF, function keys are not automatically assigned to special functions. You equate each function
key to a character string. When you press a function key, it simulates command entry. The processing is
the same as if you had typed the character string in the command field and pressed the Enter key.
Note: On a 3270 display, the horizontal divider line that separates the logical screens is not considered
part of either logical screen. If the cursor is placed on this horizontal divider line and a function key is
pressed, the result is the same as if the ENTER key was pressed and the cursor is positioned on the active
logical screen's command line.
A dialog function cannot distinguish the difference between a command entered by a function key and a
command entered by typing in the command field. If the character string with which the function key is
equated is longer than the screen's command field, the string is truncated without warning.
If you type information on the command line and then press a function key, the function key definition,
followed by a blank, is concatenated ahead of the contents of the command field. For example, suppose
F7 is equated to the character string UP. If you type 4 in the command field and then press F7, the results
are exactly the same as if you had typed UP 4 in the command field and pressed the Enter key.
Using Function Keys
66  z/OS: z/OS ISPF User's Guide Vol I

## Page 95

ISPF does not require function keys for its operation. Commands can be entered in the command field of
any display, including View, Browse, Edit, and Table Display. However, for ease of use, function keys are
strongly recommended.
The default function key assignments distributed with ISPF for the 3x4 key pad on the right side of the
keyboard are shown in the next table. These are function keys 1-12 on a 12-key terminal or keys 13-24 on
a 24-key terminal.
Table 11. Function key arrangement
Function key Function
F1 HELP
F2 SPLIT
F3 END
F4 RETURN
F5 RFIND
F6 RCHANGE
F7 UP
F8 DOWN
F9 SWAP
F10 LEFT
F11 RIGHT
F12 RETRIEVE
Function keys can be displayed at the bottom of a panel. Using the FKA or PFSHOW command, you can
display either the long or short form of the keys, or remove the keys from the panel. See “ISPF system
commands” on page 34 for a complete description of how to display or remove the function keys.
For panels defined without the )PANEL section, the long and short form of the function key area is the
same. If you use a )PANEL section, you can use the KEYLIST command or the "Keylist settings" choice
from the Function keys pull-down on the ISPF Settings panel to determine which keys appear in each
form. For more information on Keylist settings, see the Settings (Option 0) topic of the z/OS ISPF User's
Guide Vol II.
Long
Displays the keys that appear in the short form along with all other keys you indicated should appear
for the long form. The long form is the default. An example of the long form follows:
Option ===> 
                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Short
Displays the keys that appear in the short form. An example of the short form follows:
Option ===> 
                                                                  
  F1=Help      F3=Exit     F10=Actions  F12=Cancel
No
Removes the function key area, making the space available for the application. The keys are still active
but are not displayed.
Using Function Keys
Chapter 3. Using commands, function keys, and cursor selection  67

## Page 96

Defining function key values
You can define function key values three ways:
• Use the KEYS command to display the Keylist Utility panel or the PF Key Definitions and Labels panel,
then change the function keys for the panel you are on.
• Use the ZKEYS command or select the "Non-Keylist PF Key settings" choice from the Function keys
pull-down on the ISPF Settings panel. Use this method to define the function keys when the )PANEL
statement has been coded on the panel. All DTL-generated panels have a )PANEL statement. For more
information on working with Function Keys and Keylists, see the 'Settings (Option 0)' topic of the z/OS
ISPF User's Guide Vol II.
• Use the KEYLIST command or select the "Keylist settings" choice from the Function keys pull-down on
the ISPF Settings panel. Use this method to define the function keys when the application panels are
defined with the DTL. For more information on Keylist settings, see the 'Settings (Option 0)' topic of the
z/OS ISPF User's Guide Vol II.
Changing the format of the function key area
The FKA and PFSHOW commands let you change the visual display of the function keys on a panel.
You can display the keys in long form or short form, or remove them completely. You can also use
the PFSHOW command with the TAILOR parameter or the "Tailor function key display" choice from the
Function keys pull-down on the ISPF Settings panel to display the Tailor Function Key Definition Display
panel shown in Figure 12 on page 68.
   ┌───────────────────────────── ISPF Settings ─────────────────────────────┐
 s │                 Tailor Function Key Definition Display                  │
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
   │ Press ENTER key to save changes.  Enter END to save changes and exit.   │
 T │                                                                         │
   │                                                                         │
   │ Command ===>                                                            │
 C │  F1=Help     F3=Exit    F12=Cancel                                      │
   ⋘─────────────────────────────────────────────────────────────────────────┘
Figure 12. Tailor Function Key Definition  Display Panel (ISPOPFA)
This panel lets you select:
• The number of function keys available for display.
See the information about 'Tailor Function Key Definition Display' in the Settings (Option 0) topic of z/OS
ISPF User's Guide Vol II for a discussion of the rules governing the number of keys available for display.
• The number of keys per line to display in each function key definition line. System variable ZPFFMT
holds the value selected. 
Using Function Keys
68  z/OS: z/OS ISPF User's Guide Vol I

## Page 97

Table 12. ZPFFMT system variable on PFSHOW
Choice Description ZPFFMT value
Six Always displays six keys per line SIX
Maximum possible Displays as many keys as will fit on each
line.
MAX
Note: The Maximum possible option is forced when you select the Panel display CUA mode option on
the ISPF Settings panel.
• The set of function keys that are to be the primary and alternate keys. System variable ZPRIKEYS holds
the value selected. 
Table 13. ZPRIKEYS values
Choice Description ZPRIKEYS value
Lower - 1 to 12 Primary keys are 1-12 LOW
Upper - 13 to 24 Primary keys are 13-24. UPP
The default value is Lower - 1 to 12.
• The set of function keys on terminals with 24 function keys for which definitions are to be displayed.
System variable ZPFSET holds the value selected. 
Table 14. ZPFSET system variable, on PFSHOW
Choice Description ZPFSET value
Primary - display keys 1 to 12 Primary set (1-12) PRI
Alternate - display keys 13 to 24 Alternate set (13-24) ALT
All - display all keys All keys (1-24). ALL
ISPF ignores these values for terminals with only 12 function keys.
Variables ZPFFMT, ZPRIKEYS, and ZPFSET are stored in the application profile pool. Dialogs can set these
values directly by using the VPUT statement in a panel definition, or by using the VPUT service in a dialog
function.
Dialog developers can control how the PFSHOW command behaves by using the ZPFCTL system variable.
ZPFCTL is also stored in the application profile pool. Its possible values are:
USER
The user can control the display of function key definition lines by using the PFSHOW command. This
is the default value.
ON
ISPF unconditionally displays function key definitions on all panels. Issuing PFSHOW OFF, FKA OFF, or
toggling to the no display setting causes ISPF to issue an error message.
OFF
ISPF does not display function key definition lines. If PFSHOW ON, PFSHOW TAILOR, FKA ON, or
toggling to the long or short form setting of either command is issued, ISPF displays an error message.
Applications can set the ZPFCTL variable value to either USER, ON, or OFF by using the VPUT service or by
using a VPUT statement with the PROFILE keyword.
Note: The ZPFCTL variable is ignored if the PFSHOW/FKA command is invoked from a panel containing
a )PANEL statement or if the panel was created using DTL.
Similarly, keylists can be controlled to some degree by the application.
Using Function Keys
Chapter 3. Using commands, function keys, and cursor selection  69

## Page 98

The ZKLUSE can be set to Y or N. If KEYLIST is ON, the value in ZKLUSE in the application profile is Y. If
KEYLIST is OFF, it is N. If an application VPUTs the variable to the application profile, the keylist setting is
altered.
These variables can be used by an application to determine what keylist is being used, and where it
comes from.
• ZKLNAME - If KEYLIST is ON and it is a panel with the )PANEL statement, ZKLNAME contains the name
of the keylist currently being used. Otherwise it is blank.
• ZKLAPPL - If KEYLIST is ON and it is a panel with the )PANEL statement, ZKLAPPL contains the
application ID that the keylist currently being used came from. Otherwise it is blank.
• ZKLTYPE - If KEYLIST is ON and it is a panel with the )PANEL statement, ZKLTYPE contains either P
(private) or S (shared), depending on the keylist currently being used. Otherwise it is blank.
Function key definitions appear at the bottom of each logical screen. There can be more than one logical
screen, such as when you are using the split-screen function. If the application has not issued an ADDPOP
service call, ISPF displays no more than four function key definition lines on one panel. If the application
has issued an ADDPOP service call and the set of keys to be displayed is primary or alternate, ISPF
displays no more than two lines. If all of the keys are to be displayed, no more than four lines appear. If all
of the keys will not fit on the lines, ISPF wraps the keys and truncates the last keys.
Saving function key definitions
This topic applies only if you created your application panels using the ISPF panel definition statements
and used the ZKEYS command or selected the "Global PF Key settings" choice from the Function keys
pull-down on the ISPF Settings panel. It does not apply for keys defined with the KEYLIST command or
through the "Keylist settings" choice from the Function keys pull-down on the ISPF Settings panel.
Function key definitions are kept in a set of system variables named ZPF01, ZPF02, ... ZPF24. Labels are
kept in a set of system variables named ZPFL01, ZPFL02, ... ZPFL24.
When you set the "Primary range" field on the Tailor Function Key Definition Display panel to Upper
- 13 to 24, variables ZPF13-ZPF24 and ZPFL13-ZPFL24 contain the primary PF key definitions and
labels. For 24-key terminals, these definitions correspond to physical keys 13-24. For 12-key terminals,
these definitions correspond to physical keys 1-12. Variables ZPF01-ZPF12 contain the alternate key
definitions, and are meaningful only for terminals with 24 function keys.
When you set the "Primary range" field on the Tailor Function Key Definition Display panel to Lower -
1 to 12, variables ZPF01-ZPF12 and ZPFL01-ZPFL12 contain the primary PF key definitions and labels.
For 24-key terminals, these definitions correspond to physical keys 1-12. For 12-key terminals, these
definitions correspond to physical keys 1-12. Variables ZPF13-ZPF24 contain the alternate key definitions
and are meaningful only for terminals with 24 function keys.
Current values for all 24 keys (variables ZPF01-ZPF24 and ZPFL01-ZPFL24) are kept in the application
profile. Hence, unique function key definitions can be associated with different applications.
An application can provide default function key settings for a new user by providing a default profile. An
application can prevent the user from changing the default function key settings by overriding the ZKEYS
command. It does this by assigning the command to NOP in the application command table.
Using the cursor-select key
ISPF permits fields on a panel to be detected with a cursor-select key. The cursor-select key is a hardware
feature on 3179, 3179G, 3180, 3278, 3279, and 3290 terminals.
Panel fields that are detectable by cursor selection can simulate a command entry, or give you an
alternate means of selecting options from a menu. Each field must be defined as an attention field. Use an
attribute character that has been defined with the ATTN(ON) keyword. The panel designer must provide
the number of blank characters that are required by the terminal hardware before and after the attention
attribute character.
Using the cursor select key
70  z/OS: z/OS ISPF User's Guide Vol I

## Page 99

Processing of cursor-selected fields is handled in much the same way as function key processing. The
entire contents of the selected field are treated as a command and processed as though they had been
typed into the command field. If the command is found in the tables, it is performed immediately. If the
command is not found in the tables, it is inserted into the command field, and the entire command field is
passed to the dialog. But unlike function keys, information in the command field is not concatenated with
the contents of the attention field. They should not be used on data entry panels, because any information
that is typed in an input field, including command fields, is lost when the attention occurs.
Attention fields can be used on a menu to simulate option selection. The panel designer must truncate
any unwanted characters resulting from an attention entry into the command field. Here is an example:
)ATTR
  $ TYPE(TEXT) ATTN(ON)
)BODY
%------------------------------- SOME MENU -------------------------------
%SELECT OPTION ===>_ZCMD                                                 + 
%
$   1 - BROWSE   +DISPLAY SOURCE DATA OR LISTINGS
$   2 - QUERY    +FIND OUT INFORMATION ABOUT SOMETHING
⋮
)PROC
 &ZCMD = TRUNC (&ZCMD, ' ')
 &ZSEL = TRANS (TRUNC (&ZCMD, '.')
                 1, 'PGM(ISPBRO)'
                 2, 'PANEL(XYZ)'
                 ⋮
Figure 13. Use of the attention-select Attribute
In the example, a cursor-selection of the first option would place the character string 1 - BROWSE in the
ZCMD field and simulate the Enter key. In the )PROC section, the contents of the ZCMD field are truncated
at the first blank before the ZSEL variable is set, based on a translation of the ZCMD field.
Panels that are included with the ISPF product do not contain the ATTN(ON) keyword in the attribute
section. If cursor selection is used, it is the user's responsibility to add the ATTN(ON) keyword to the
attribute section of the desired panel. See the z/OS ISPF Dialog Developer's Guide and Reference for
complete descriptions of the various panel sections.
How Program Access (PA) keys affect ISPF operation
The two Program Access (PA) keys are defined as follows. These definitions cannot be changed.
ATTENTION (PA1)
Normally, you should not use this key while you are in ISPF full-screen mode. The text following
discusses exceptions.
RESHOW (PA2)
Redisplays the contents of the screen. PA2 can be useful if you have pressed the ERASE INPUT or
CLEAR key accidentally or have typed unwanted information but not yet pressed the Enter key or a
function key.
Generally, PA1 is used to terminate TSO commands or CLISTs running under ISPF. However, some TSO
commands and CLISTs process PA1 in their own way.
Restrictions that apply to CLIST attention exits are described in the z/OS ISPF Dialog Developer's Guide
and Reference. Also, ISPF should not be started from a CLIST that contains an attention exit because
results are unpredictable.
If PA1 is pressed while ISPF is in full-screen mode after the keyboard has been unlocked, it is treated as a
RESHOW request. If PA1 is pressed again, the current function is terminated and either the primary option
menu or a top-level selection panel supplied by the dialog developer is displayed.
When an ISPF function is running, if the RESET key is pressed to unlock the keyboard and PA1 is pressed,
ISPF attempts to terminate the current function and redisplay the primary option menu. The attempt
Using the cursor select key
Chapter 3. Using commands, function keys, and cursor selection  71

## Page 100

might not always be successful; for example, if there is an error in MVS allocation, the attempt fails. A
failure might cause unpredictable results such as waits, loops, abends, or incorrect and unrelated error
messages.
In a 3270 SNA environment, the ATTN key is treated the same as the PA1 key. It is a program interrupt
and, like the PA1 key, causes the ISPF attention exit to get control.
The AUTOTYPE function, for automatic data set name and member
name completion
The AUTOTYPE function is not available on all ISPF panels. It works only on panels that are specifically
written to understand it.
If you assign the value of AUTOTYPE to a function key, you can type a partial name into a library, member,
or data set name field, then press the function key to have ISPF complete the name for you.
AUTOTYPE automatically searches the catalog or PDS directory to find names that match what you
entered. You can even type a pattern to limit the names that AUTOTYPE will return. AUTOTYPE works only
on panels that have been enabled to use the function. You can also enable your own applications to use
AUTOTYPE (see “Enabling applications to use AUTOTYPE” on page 73).
If you are using a terminal emulator, you can assign an easily reachable key to the function key that
invokes AUTOTYPE. For example, you can use a control key combination or any other key or combination
that is within easy reach. You can also use the function key directly.
Within ISPF, AUTOTYPE is enabled for these panels:
• Edit, Browse, and View (options 1 and 2, including recursive edit/browse/view, copy, replace and move
panels).
• Library Utility panels (option 3.1)
• Data Set Utility panels (option 3.2, including Rename)
• Move/Copy (option 3.3)
• Data Set List entry panel (option 3.4)
• Reset ISPF Statistics (option 3.5)
• Hardcopy Utility (option 3.6)
• SuperC Compare (options 3.12 and 3.13 in all fields)
• SuperC Search (options 3.14 and 3.15, all fields)
• SCLM View and Edit (options 10.1 and 10.2)
• SCLM Library Utility (option 10.3.1)
• Sublibrary Management (option 10.3.2)
• SCLM Migration Utility (option 10.3.3)
• SCLM Delete from Group Utility (option 10.3.9)
• SCLM Build and Promote (options 10.4 and 10.5)
• Preprocessed panel utility (ISPPREP)
• Dialog tag language compiler (ISPDTLC)
On panels that are not enabled for AUTOTYPE, pressing the AUTOTYPE key is the same as pressing
ENTER. The variable ZVERB is set to the value 'AUTOTYPE'.
How to use AUTOTYPE
1. Type a partial name (zero or more characters) into a Library field (project, group, type, or member) or
the Other Data Set Name field.
2. Press the function key that has been set to AUTOTYPE. ISPF sets the field to the correct value.
AUTOTYPE function, for Data Set Name Completion
72  z/OS: z/OS ISPF User's Guide Vol I

## Page 101

3. If you immediately press the function key again, ISPF retrieves the next data set or member name. Up
to 100 data set names and 700 member names can be retrieved.
Rules for specifying the 'Other Data Set Name' field
The prefix or pattern you specify is that which is to the left of the cursor. For example, if the field is
'CLIST(ABC)', with the cursor under the letter 'S', then the pattern used is 'CLI'. For more information, see
“Cursor position sensitivity” on page 73.
• If the content of the Data Set Name field does not begin with a quote, your TSO prefix is added. If the
field does begin with a quote, no prefix is added. Unquoted data set names are not processed if you do
not have a TSO prefix (except for on the Data Set List Utility panel).
• The pattern can be any pattern similar to what you use in your data set list (except that you don't have
to specify your TSO prefix. Trailing wildcards are automatically added. For example, in a Data Set Name
field, typing 'CHR' will result in a pattern of pr efix .CHR*.** when searching the catalog.
• If you include a left parenthesis, followed optionally by a member name or pattern, the data set
is assumed to be a PDS and the member name is returned. For example 'CLIST(XY' would search
pr efix .CLIST for members matching the pattern XY*. A trailing * is always added to the member name to
create a pattern.
Rules for Library fields - Project, Group, Type, Member
• If the cursor is in the Project field, the other fields are not used as part of the data set name search and
are erased.
• If the cursor is in a Group field, the project name and type name (if any) are added to create the search
pattern. Each group name is used only once, even if there are many types in that group. As each group
name is displayed, the first type name found for that project.group combination is also retrieved. The
member name, if any, is not used and is erased.
• If the cursor is in the Type field, the project and first group name are used to create the pattern. The
Type field is updated and the member name is erased.
• If the cursor is in the Member field, the project, first group, and type are used as the data set name. The
second, third, and fourth groups are not used.
Cursor position sensitivity
The pattern or prefix used to search for names is only that which is to the left of the cursor. In this way you
can refine your search simply by moving the cursor.
For example; suppose you have one hundred data sets called 'SYS1.A234.RGG.*' and you plan to use one
named 'SYS1.A234.RGG.DBD0223.L422.FEB0299.TERRA'. You could type 'SYS1.A234.RGG' and press
the AUTOTYPE key. That might return 'SYS1.A234.RGG.DBD0211.X331.AUG0599.FIRMA'. You can refine
the next value returned by typing a '2' over the first '1' in 'DB0211' and then pressing the AUTOTYPE key
again. This will use the new pattern and get you closer to the desired value.
Restrictions
AUTOTYPE only retrieves cataloged alias names. It will not retrieve generation data group or generation
data set names. AUTOTYPE does not use the Volume field on any panel. It sets the Volume field
associated with the current data set name field to blank.
AUTOTYPE does not use any ISPF name change exits or data set list retrieval exits.
AUTOTYPE retrieves a maximum or 100 data sets and 700 member names before cycling through the list
from the beginning.
Enabling applications to use AUTOTYPE
You can enable applications to use AUTOTYPE by making some minor panel modifications as follows:
AUTOTYPE function, for Data Set Name Completion
Chapter 3. Using commands, function keys, and cursor selection  73

## Page 102

1. At the beginning of the )REINIT section, add the lines shown. Make sure the subsequent line in
the )REINIT section starts in column 1 so that it does not become part of the IF clause you have
inserted.
IF (&ZNXTMSG='ISRT') .CSRPOS = &ZCSRP
                     .CURSOR = &ZCSRV
2. At the end of the )REINIT section add this line in column 1.
REFRESH (*)
3. At the beginning of the )PROC section add the lines shown. Make sure the subsequent line in
the )PROC section starts in column 1 so that it does not become part of the IF clause you have
inserted. If there is a line that says: .RET = OFF in the )PROC section, it should go before these lines.
&ZCSRV = .CURSOR
&ZCSRP = .CSRPOS
&ZNAMES='ZCSRV ZCSRP PRJ1 LIB1 LIB2 LIB3 LIB4 TYP1 MEM DSN ZCMD
PANEXIT ( (ZNAMES) , LOAD,ISRAUTOT)
IF (&ZNXTMSG='ISRT') EXIT
Modify the line that assigns the variable &ZNAMES. This assignment contains a list of variable names
on the panel. They must all be specified in order. Use an asterisk (*) for names that are not relevant for
your panel.
The values in the &ZNAMES variable are:
1. The variable containing the Cursor field name
2. The variable containing the cursor offset
3. The name of the Project variable on the panel
4. The name of the first Group variable.
5. The name of the second Group variable
6. The name of the third Group variable
7. The name of the fourth Group variable
8. The name of the Type variable
9. The name of the Member variable
10. The name of the Other Data Set Name variable
11. The name of the command line variable (clears the command line)
The cursor and name variables (described here as ZCSRV, ZCSRP, and ZNAMES) can have any names
you choose but they must match the names used in the )INIT section and the PANEXIT statement in
the )PROC section.
If a name references a read-only field, add a dash to the end of the name. If you need to limit the size
of the returned name, you can append the maximum length, after a period, to the name; for example,
ODSN.44. You can disable member searches for a data set name field by adding a percent sign to the end
of the field name; for example, ODSN.44% or ODSN%.
Panels defined in Dialog Tag Language (DTL) can be enabled for AUTOTYPE through DTL keywords. See
the z/OS ISPF Dialog Tag Language Guide and Reference for more information.
AUTOTYPE function, for Data Set Name Completion
74  z/OS: z/OS ISPF User's Guide Vol I
