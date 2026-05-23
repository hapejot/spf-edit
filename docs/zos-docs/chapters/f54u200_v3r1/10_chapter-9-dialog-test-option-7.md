# Chapter 9. Dialog test (option 7)

Source file: f54u200_v3r1.md
Start page: 393
Page span: 393-436

## Page 393

Chapter 9. Dialog test (option 7)
This topic describes Dialog Test, option 7 on the ISPF Primary Option Menu.
Dialog Test (option 7) provides you with facilities for testing both complete ISPF applications and ISPF
dialog parts, including functions, panels, variables, messages, tables, and skeletons. The Dialog Test
option allows you to:
• Call selection panels, command procedures, and programs
• Display panels
• Add new variables and change variable values
• Display a table's structure and status
• Display, add, modify, and delete table rows
• Browse the ISPF log
• Process dialog services
• Add, modify, and delete function and variable trace definitions
• Add, modify, and delete breakpoint definitions.
You can use TSO TEST to complement this option if you want to examine and manipulate non-ISPF
storage areas.
You usually test a dialog in one of two ways:
• Test individual dialog parts, including panels, skeletons, and messages, without calling a function or a
selection panel. Eventually, you end your test session by entering the END command on the Dialog Test
Primary Option Panel.
• Test dialog functions, including programs, commands, and selection panels, using the Functions option
(7.1). You can define traces and breakpoints before calling the function.
Any requested traces for variable usage and dialog service calls are written to the ISPF log. You can
browse the log using the Log option (7.5).
If you define a breakpoint and the function gets to it, dialog processing is suspended, and Dialog Test
displays the Breakpoint Primary Option Panel (Figure 238 on page 396). At this point, you can access and
manipulate dialog parts, such as variables, tables, and so forth. Then, if you select the Go option from the
Breakpoint Primary Option Panel, the dialog resumes processing.
When the processing is complete, you are returned to the Functions option (7.1). If you select the Cancel
option from the Breakpoint Primary Option Panel, the dialog is canceled and the first primary option panel
that you were shown during your terminal session is displayed again. For example, if the first screen
displayed when you began your session was a master application panel that is different from the ISPF
Primary Option Menu, that master application panel is displayed again.
The dialog test environment
The Dialog Test Primary Option Panel, shown in Figure 216 on page 356, follows the conventions for a
primary option panel. If you use the RETURN command from one of the selected Dialog Test options, the
Dialog Test Primary Option Panel is displayed again. If you use the END command from this panel, you
return to the ISPF Primary Option Menu.
When you enter Dialog Test from the ISPF Primary Option Menu, you enter a new user application with
an ID of ISR. When you enter Dialog Test from the ISPF primary option panel, you enter a new user
application with an ID of ISP. All options listed on the Dialog Test Primary Option Panel operate in this
context. If you call a new function using the Functions options (7.1), a SELECT service call is performed,
and the rules for the SELECT service are followed.
© Copyright IBM Corp. 1980, 2024 355

## Page 394

Menu  Utilities  Compilers  Options  Status  Help
┌───────────────────────────── Dialog Test ──────────────────────────────┐ ────
│   Menu  Utilities  View  Help                                          │
│ ─────────────────────────────────────────────────────────────────────  │
│                         Primary Option Panel                           │ RN
│                                                                        │
│ 1 Functions       Invoke dialog functions/selection panel              │
│ 2 Panels          Display panels                                       │
│ 3 Variables       Display/set variable information                     │ SH
│ 4 Tables          Display/modify table information                     │
│ 5 Log             Browse ISPF log                                      │
│ 6 Dialog Services Invoke dialog services                               │ RN
│ 7 Traces          Specify trace definitions                            │
│ 8 Breakpoints     Specify breakpoint definitions                       │ A
│ T Tutorial        Display information about Dialog Test                │ 5.5
│ X Exit            Terminate dialog testing                             │
│ Option ===>                                                            │
│  F1=Help       F2=Split      F3=Exit       F4=Expand     F5=Rfind      │
│  F6=Resize     F7=Backward   F8=Forward    F9=Swap      F10=Actions    │
⋘────────────────────────────────────────────────────────────────────────┘
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 216. Dialog test primary option panel (ISPYXD1)
Note: You can set the application ID under which you enter the Dialog Test function using the Dialog Test
appl ID choice from the Options pull-down on the ISPF Primary Option Menu or using the Dialog Test appl
ID choice from the Test pull-down on an Edit panel.
After you begin an application under Dialog Test, you can enter the DTEST command with one of its
parameters as a quicker way to start a dialog test function. For example, if you enter DTEST 8 on
the command line, the Option 7.8 Breakpoints panel is displayed. The other parameters of the DTEST
command also match the dialog test function they perform:
1
display the Invoke Dialog Functions/Selection panel
2
display the Display Panel panel
3
display the Variables panel
4
display the Tables panel
5
browse the log
6
display the Invoke Dialog Service panel
7
display the Traces panel
8
display the Breakpoints panel
You must use a parameter with the DTEST command, otherwise an error message appears. After you
complete the entries on whichever dialog test panel you invoke, leaving the panel returns you to the
application you were running with the new entries in place.
Dialog Test is itself a dialog and, therefore, uses dialog variables. Because it is important to allow your
dialog to operate without interference, as though in a production environment, Dialog Test accesses and
updates variables independently of your dialog variables.
356  z/OS: z/OS ISPF User's Guide Vol II

## Page 395

All breakpoints and traces that you set in Dialog Test exist only while you remain within the Dialog Test
option.
You should always allocate the ISPF log when using Dialog Test. Do not suppress its generation by typing
0 in the "Primary pages" field that appears on the "Log Data set defaults" and "List Data set defaults"
choices from the Log/List pull-down on the ISPF Settings panel. Dialog Test writes trace data to the log
when you request it. Also, if Dialog Test finds an unexpected condition, it writes problem data to the log.
When you enter Dialog Test, you are given these ISPF facilities:
• All functions you normally get by specifying the TEST parameter on the ISPSTART command
• Logging of all severe errors, both from user dialogs and Dialog Test. This is normally done when you
specify TRACE or TRACEX on the ISPSTART command.
• Suspension of the logging of all ISPEXEC dialog service requests. Such logging normally occurs when
you specify TRACE or TRACEX on the ISPSTART command. You should use the Traces option (7.7).
These facilities become active for all logical screens when you are using split-screen mode. At the
completion of the last dialog test session (dialog test is no longer active in any logical screen), these
options will be restored to the original values established during dialog manager start-up. Optionally, by
making the appropriate selection on the ISPF Settings panel, the facilities established by dialog test will
remain in effect after the last dialog test session terminates.
Dialog test primary option panel action bar
The Dialog Test Primary Option Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
View
Allows you to select whether the Variables panel (option 7.3) and the Invoke Dialog Services panel
(option 7.6) are displayed in a pop-up window. By displaying the panel in a pop-up window, you can
see the panel underneath and move the window. By displaying the panel as a full-screen display, you
can see more data. The setting you choose will remain until you change it.
Help
The Help pull-down provides general information about Dialog Test topics as well as information
about each available choice on the Dialog Test Primary Option panel.
Using variables
When you select the Dialog Test option from the ISPF Primary Option Menu, you are given a new function
pool, a new shared pool, and the profile pool for the application ID under which you entered Dialog Test.
When you select Dialog Test from the ISPF Primary Option Menu, you are given a new function pool, a new
shared pool, and the ISPPROF profile pool. These pools are used if you set a variable, display a panel,
call an ISPF service, and so forth. When you call a new dialog, Dialog Test uses the SELECT service, and
follows the rules for the creation of new variable pools. For example, if you call a new dialog using the
NEWPOOL option, Dialog Test creates new shared and function variable pools for you. The profile variable
pool, ISRPROF or ISPPROF, remains as it was.
If you set a dialog variable in the shared pool from a dialog running under Dialog Test and then call the
dialog again from the Command line, you cannot retrieve the value of that variable.
Dialog variables should be initialized and set in the context of the dialog's processing. A dialog's function
variable pools are created when it is called; that is, when the SELECT is done. Therefore, to set function
variables in newly created pools, you must define a breakpoint early in your dialog's processing, at a point
before the function is called.
Chapter 9. Dialog test (option 7)  357

## Page 396

For example, if you call a dialog with the NEWPOOL parameter, you must define a breakpoint in the dialog
before the first function is called to access that dialog's function and shared variable pools. You can
change the dialog's profile variable pool before calling the dialog, since a new profile variable pool is not
created.
When your dialog ends, all variable pools that were created when the dialog was called are deleted.
Note: ISPF does not support TSO global variables. You can find a severe dialog test error when testing a
dialog that refers to a global variable.
Severe error handling
If your dialog finds a severe error when it calls a dialog service, the error is handled as requested by the
dialog. The current CONTROL service ERRORS setting, CANCEL or RETURN, determines what is done. If
CANCEL is in effect, you can choose whether to continue dialog testing when the Error Message panel is
displayed.
Note: If you choose not to continue dialog testing, you return to the ISPF Primary Option Menu. The
TEST and TRACE options set by dialog test are restored to the values originally established during
dialog manager start-up. Optionally, by making the appropriate selection on the ISPF Settings panel, the
facilities established by dialog test will remain in effect after the last dialog test session terminates.
If you find a severe error when manipulating your dialog at a breakpoint, Dialog Test assumes that the
CONTROL service ERRORS setting is CANCEL. For example, if you display a panel at a breakpoint and that
panel is not found, the Error Message panel is displayed. This occurs even if your current dialog has an
ERRORS setting of RETURN.
Regardless of the ERRORS setting, all your severe errors are logged.
If Dialog Test finds a severe error during its processing, the details are logged and this message is shown
to you on an error message display:
Test severe error
Details precede this message in the ISPF log
Dialog Test errors can occur because:
• Proper ISPF libraries are not being used.
• A programming problem has been found.
• You have attempted to process Dialog Test.
• You have called a Dialog Test option without being in test mode or without calling Dialog Test first.
Browse the ISPF log to find the problem; see “Log (option 7.5)” on page 383 for more information.
Commands
You can enter ISPF primary commands on Dialog Test panels. Seven commands have special meaning
during Dialog Test operations. You enter them in the Command line of the applicable Dialog Test option
panel. These commands, and the Dialog Test options with which they function, are: 
Table 25. Primary commands
Primary Command Valid Options
CANCEL • Variables (option 7.3)
• Tables (option 7.4)1
• Traces (option 7.7)2
• Breakpoints (option 7.8)
1 Valid only with Tables options 3 and 4.
358  z/OS: z/OS ISPF User's Guide Vol II

## Page 397

Table 25. Primary commands (continued)
Primary Command Valid Options
END • Variables (option 7.3)
• Tables (option 7.4)3
• Traces (option 7.7) 2
• Breakpoints (option 7.8)
LOCATE • Variables (option 7.3)
• Tables (option 7.4)4
• Traces (option 7.7) 2
• Breakpoints (option 7.8)
NEXT/PREV • Tables (option 7.4)4
QUAL • Breakpoints (option 7.8)
RESUME • Breakpoints (option 7.8)
SORT • Variable (option 7.3)
Dialog Test has three line commands that have special meaning during testing operations. These
commands, and the options with which they function, are: 
Table 26. Line commands
Line Command Valid Options
D (delete) • Variables (option 7.3)
• Tables (option 7.4) 1
• Traces (option 7.7) 2
• Breakpoints (option 7.8)
I (insert) • Variables (option 7.3)
• Tables (option 7.4) 1
• Traces (option 7.7) 2
• Breakpoints (option 7.8)
R (repeat) • Tables (option 7.4) 1
• Traces (option 7.7) 2
• Breakpoints (option 7.8)
When using the Dialog Test primary and line commands, you should be aware that:
• You can specify both a primary command and line commands before you press the Enter key.
• You can enter multiple line commands on the display.
• You cannot carry out a deletion if one of the included lines contains another line command.
2 Valid only with Traces options 1 and 2.
3 Valid only with Tables options 1, 3, and 4.
4 Valid only with Tables options 1 and 3.
Chapter 9. Dialog test (option 7)  359

## Page 398

• You can delete lines that contain an input error.
• The line commands are processed in row order when you press the Enter key. Any fields changed in the
row are handled before a line command is processed.
• A primary command is handled after processing for all line commands is complete.
• As in the ISPF editor, you can specify a number with each line command to denote repetitive operation,
unless you are using the Variables option (7.3). To avoid conflict with the I (insert) line command, the
Variables option does not allow you to type a number along with the D command to delete more than
one line simultaneously. Therefore, enter a single D line command on each line you want to delete.
Unlike the ISPF editor, the Variables option does not support block deletes; however, you can enter this
command on more than one line before pressing the Enter key.
Ending the current option without saving changes
The CANCEL command ends the current option. Any changes made to the data are ignored.
Saving changes
The END command ends the current option. Any changes made to the data now take effect.
Displaying long variable values
The EXPAND command can be entered on the Variables panel to display the first 2048 characters of
a variable value in a pop-up window. This command can be useful when the length of a variable value
exceeds the 57 characters that are displayed on the Variables panel. Enter the EXPAND command with
the cursor placed on the value to be expanded.
Scrolling variable values
The LEFT and RIGHT commands can be entered on the Variables panel to scroll the displayed variable
values. This command can be useful when the length of a variable value exceeds the 57 characters that
are displayed on the Variables panel. Enter the LEFT/RIGHT command with the cursor placed in the Value
column. All of the displayed variable values are scrolled.
Finding a character string
The LOCATE command searches for a character string and positions a scrollable display to the next row
that contains the string. The scan starts at the end of the first row currently being displayed. A message is
displayed indicating the result of the scan.
LOCATE string
where:
string
The character string you are trying to find. If the string ends in an asterisk (*), a scan for the characters
preceding the asterisk is done. The asterisk is optional in the Variables option (option 7.3).
Displaying breakpoint qualification data
The QUAL command can only be entered from the Breakpoints pop-up window. It displays the breakpoint
qualification data.
The same breakpoint qualification data can be obtained using the Qualifications choice on the Qualify
pull-down.
360  z/OS: z/OS ISPF User's Guide Vol II

## Page 399

Restoring the format of the Breakpoints panel
The RESUME command is entered on the Breakpoints panel when qualification parameter values are
shown. It restores the format of the Breakpoints panel. Each breakpoint that has qualification is flagged
by the characters *QUAL* in columns 75 to 80 on that line of the Breakpoints panel.
Dialog test line commands
These line commands have special meaning during testing operations:
D – deleting lines
The D command deletes one line or n lines starting with this line. The syntax is:
D
1
n
If you are using the Variables option (7.3), the n operand does not apply. To avoid conflict with the
I (insert) line command, the Variables option does not allow you to type a number along with the D
command to delete more than one line simultaneously. Therefore, enter a single D line command on each
line you want to delete. You can enter this command on more than one line before pressing the Enter key.
I – inserting lines
The I command inserts one line or n lines directly after this line, with underscores and quotation marks in
the appropriate fields. The syntax is:
I
1
n
R – repeating lines
The R command repeats this line once or n times. The syntax is:
R
1
n
Setting keylists for dialog test
Depending on your needs and preferences, you may wish to set dialog test-specific keylists to enhance
productivity. To modify the default function key settings, go to a Dialog Test panel, type KEYS on the
Command line, and press Enter to display the pop-up window shown in Figure 217 on page 362. Or, you
may perform these steps to display the pop-up:
1. Select Option 0 from the ISPF Primary Option Menu.
2. Select Keylist settings from the Function keys pull-down on the ISPF Settings panel action bar.
3. Locate the keylist that you wish to modify and enter the E (Edit) line command.
Chapter 9. Dialog test (option 7)  361

## Page 400

┌────────────────────────────────── Keylist Utility ──────────────────────────────────┐
 │   File                                                                      │
 │ ─────────────────────────────────────────────────────────────────────────────────── │
 │                         ISR Keylist ISRTEST Change         Row 1 to 9 of 24 │
 │                                                                             │
 │ Make changes and then select File action bar.                               │
 │                                                                             │
 │ Keylist Help Panel Name . . . ISRTESTH                                      │
 │                                                                             │
 │ Key       Definition                                 Format  Label          │
 │ F1 . . .  HELP                                       SHORT   Help           │
 │ F2 . . .  SPLIT                                      LONG    Split          │
 │ F3 . . .  EXIT                                       SHORT   Exit           │
 │ F4 . . .  EXPAND                                     SHORT   Expand         │
 │ F5 . . .  RFIND                                      SHORT   Rfind          │
 │ F6 . . .  RESIZE                                     SHORT   Resize         │
 │ F7 . . .  UP                                         LONG    Backward       │
 │ F8 . . .  DOWN                                       LONG    Forward        │
 │ F9 . . .  SWAP                                       LONG    Swap           │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 └────────────────────────────────────────────────────────────────────────────────────┘
Figure 217. Keylist change panel (ISPKLUCH)
Keylist ISRTEST is used for all of the Dialog Test panels with the exception of the Variables panel. The
Variables panel uses keylist ISRVTST.
On the Keylist Change panel, you can reassign existing function keys by typing over the information in the
data fields, or create a new function key assignment to suit your needs. For example, you could assign the
GO command to F4 if you typically issue GO many times during a dialog test.
If you press F3 or select Save and Exit from the File pull-down, the values that you have assigned will be
valid when you return to Dialog Test. If you select Cancel from the File pull-down, the fields return to their
original values.
Functions (option 7.1)
The Functions option (7.1) allows you to test a dialog function without having to build supporting code
or panels. Dialog functions include panels, command procedures, and programs. The name of the dialog
function and the parameters you can pass are the same as those that you can specify from a dialog
function when you call the SELECT service. When you press the Enter key, a SELECT service is called.
If you call a new function or selection panel at a breakpoint, the previous function or selection panel
is suspended and the new one is processed. When the new activity finishes, the Invoke Dialog Function/
Selection Panel is displayed. The old activity resumes when you enter the END command. When the
function that was called originally finishes processing, the Invoke Dialog Function/Selection Panel is
displayed again.
When you select the Functions option, the scrollable panel shown in Figure 218 on page 363 is displayed
to allow you to specify the dialog function that you want to test. Press F8=Forward to display the rest of
the panel.
Functions (option 7.1)
362  z/OS: z/OS ISPF User's Guide Vol II

## Page 401

Menu  Utilities  Compilers  Options  Status  Help
  ┌─────────────────────────────── Dialog Test ────────────────────────────────┐
  │   Menu  Save  Utilities  Help                                              │
  │ ─────────────────────────────────────────────────────────────────────────  │
  │                  Invoke Dialog Function/Selection Panel                    │
  │                                                               More:     +  │
  │ Invoke selection panel:                                                    │
  │    PANEL  . .                                                              │
  │    OPT  . . .                                                              │
  │                                                                            │
  │                                                                            │
  │ Invoke command:                                                            │
  │    CMD  . . .                                                              │
  │                                                                            │
  │    LANG . . .                       (APL, CREX, or blank)                  │
  │    MODE . . .                       (LINE, FSCR, or blank)                 │
  │                                                                            │
  │ Invoke program:                                                            │
  │    PGM  . . .                                                              │
  │    PARM . . .                                                              │
  │ Command ===>                                                               │
  │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind      │
  │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions    │
  ⋘────────────────────────────────────────────────────────────────────────────┘
Figure 218. Invoke Dialog Function/Selection panel (ISPYFP)
There are two alternate Invoke Dialog Function/Selection panels, ISPYFPA and ISPYFPB. ISPYFPA is
formatted with the most often used fields at the top of the scrollable area. ISPYFPB is similar, but it has
a selection field for panel, command, program, or request selection. Unlike panel ISPYFP and ISPYFPA,
when you use ISPYFPB the panel, command, program, and request selection fields can all contain values.
You can specify one of the alternate panels using the ISPF Configuration utility. See z/OS ISPF Planning
and Customizing for more information.
One of the advantages of placing dialog panels in pop-up windows is that you can move the pop-up within
the 3270 physical display area to reveal portions of the underlying panel.
See z/OS ISPF Dialog Developer's Guide and Reference for instructions on how to move a pop-up window.
Invoke dialog function/selection panel action bar
The Invoke Dialog Function/Selection Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Save
Allows you to specify that you want to save or clear input field information when you exit this panel.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
1
General
2
Invoke Function Panel
3
Usage Notes
4
General Dialog Test
Functions (option 7.1)
Chapter 9. Dialog test (option 7)  363

## Page 402

Invoke dialog panel fields
To call a function, you must specify a value for either the PANEL, CMD, or PGM field. You cannot specify
more than one of these fields.
The fields on the Invoke Dialog Function/Selection Panel function as follows:
Invoke selection panel:
Use these fields to call a selection panel:
PANEL
The name of the selection panel to be displayed.
OPT
An optional parameter indicating the first selection option that must be valid from the specified
selection panel. This input field continues onto the next line on the panel.
Invoke command:
Use these fields to call a command:
CMD
The name of a command procedure written in CLIST or REXX, or any TSO command, to be called
as a dialog function. You can include command parameters.
Use the percent sign (%) as a prefix symbol to tell ISPF to remove the Invoke Dialog Function/
Selection Panel and use the full screen to display the results of a CLIST or REXX exec call. A
complete CLIST or REXX exec call is indicated by three asterisks. Press the Enter key to return to
the Invoke Dialog Function/Selection Panel.
If you omit the % prefix, ISPF interprets the command as a TSO command, using line mode to
display the command results at the bottom of the Invoke Dialog Function/Selection Panel.
LANG
An optional parameter used to specify APL or CREX.
Type APL in this field to specify the use of the APL language. If this is your first APL request during
the session, the command specified in the CMD keyword is called and an APL2® environment is
established. If this is not your first APL request during this session, the string specified after the
CMD keyword is passed to the APL2 workspace and processed.
Type CREX in this field to specify that the command specified in the CMD keyword is a REXX exec
that has been compiled and link-edited into a load module, and that a CLIST/REXX function pool
is to be used rather than an ISPF module function pool. LANG(CREX) is optional if the compiled
REXX has been link-edited to include any of the stubs EAGSTCE, EAGSTCPP, or EAGSTMP.
See z/OS ISPF Dialog Developer's Guide and Reference for more information about Compiled REXX
processing.
To specify any language other than APL or Compiled REXX, leave this field blank.
MODE
An optional parameter that overrides:
• Automatic line mode entry, caused when a TSO command is entered.
• Automatic full-screen display caused by the % CLIST or REXX exec prefix. However, it does not
prevent ISPF from calling the command as a CLIST or REXX exec.
If you leave this field blank, the % prefix has its normal effect. The valid values for this field are:
LINE
Used to enter line mode when calling a CLIST or REXX exec.
FSCR
Used to enter full-screen mode when calling a TSO command.
Invoke program:
Use these fields to call a program:
Functions (option 7.1)
364  z/OS: z/OS ISPF User's Guide Vol II

## Page 403

PGM
The name of a program to be called as a dialog function.
PARM
Optional parameters to be passed to the program. This input field continues onto the next line on
the panel.
MODE
An optional parameter used to tell ISPF whether to display the program results in line mode or
full-screen mode. If you leave this field blank, ISPF uses line mode as the default. The valid values
for this field are:
LINE
Used to enter line mode when calling a program. Results of the program are displayed at the
bottom of the Invoke Dialog Function/Selection Panel.
FSCR
Used to enter full-screen mode when calling a program. ISPF removes the Invoke Dialog
Function/Selection Panel and uses the full screen to display the program results. Three
asterisks show program completion. Press the Enter key to return to the Invoke Dialog
Function/Selection Panel.
Options:
Use a slash to select these options:
NEWAPPL
Indication of whether a new application is being called. Select this option if the function is a new
application.
ID
A 1- to 4-character ID for a new application. If you call a new application and leave the ID field
blank, the default ID of ISP is used. Note that the ID determines the names of the profile and the
command table to be used for the application.
NEWPOOL
Indication of whether a new shared variable pool is to be created. Select this option if you want to
create a new shared variable pool; however, the selection is ignored if NEWAPPL is selected.
PASSLIB
Shows that the current set of application-level ISPF libraries, if any sets exist, is to be used by the
application being selected. You can select PASSLIB only if you also select NEWAPPL.
Note: For more information about the PASSLIB field, see the description of the SELECT service in
z/OS ISPF Services Guide.
Panels (option 7.2)
When you are developing panels, you can use the Panels option (7.2) to test newly created or changed
panels and messages without having to build supporting code to display them. Any variables referred to
and set during panel processing are written to the current function variable pool.
When you select the Panels option (7.2), the panel in Figure 219 on page 366 is displayed.
Panels (option 7.2)
Chapter 9. Dialog test (option 7)  365

## Page 404

Menu  Utilities  Compilers  Options  Status  Help
  ┌───────────────────── Dialog Test ─────────────────────┐ ───────────────────
  │   Menu  Save  Utilities  Help                         │
  │ ────────────────────────────────────────────────────  │
  │                    Display Panel                      │ er ID . : USERID
  │                                                       │ me. . . : 17:23
  │ Panel name  . . . . . . .                             │ rminal. : 3278
  │ Message id  . . . . . . .              (Optional)     │ reen. . : 1
  │ Cursor field  . . . . . .              (Optional)     │ nguage. : ENGLISH
  │ Cursor position . . . . .              (Optional)     │ pl ID . : ISR
  │ Message pop-up field  . .              (Optional)     │ O logon : ISPF
  │                                                       │ O prefix: USERID
  │ Enter "/" to select option                            │ stem ID : MVS8
  │    Display in window                                  │ S acct. : IBMGSA
  │                                                       │ lease . : ISPF 5.5
  │                                                       │
  │ Command ===>                                          │
  │  F1=Help      F2=Split     F3=Exit      F4=Expand     │
  │  F5=Rfind     F6=Resize    F7=Backward  F8=Forward    │
  ⋘───────────────────────────────────────────────────────┘
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 219. Display panel (ISPYP1)
Display panel action bar
The Display Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Save
Allows you to specify that you want to save or clear input field information when you exit this panel.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
1
General
2
Display Panel
3
Usage Notes
4
General Dialog Test
Display panel fields
The fields on the Display Panel function as follows:
Panel name
The name of the panel to be displayed.
Message id
The identifier of a message to be displayed on the panel.
Cursor field
The name of the field on the panel where the cursor is to be positioned.
Cursor position
An integer specifying the position in the field where the cursor is to be placed.
Panels (option 7.2)
366  z/OS: z/OS ISPF User's Guide Vol II

## Page 405

Message pop-up field
The name of a panel field the pop-up message window should be placed adjacent to. Note that the
message definition determines if the message will appear in a pop-up window.
Display in window
A slash ( /) specifies that the panel is to be displayed in a pop-up window.
If you specify a panel name, entries in "Message id", "Cursor field", "Cursor Position", "Message pop-up
field", and the Display in window option are optional.
With the exception of the Display in window option, these are the same parameters that a dialog function
can specify when calling the DISPLAY service. Selecting the Display in window field is the functional
equivalent to the dialog issuing an ADDPOP service before the DISPLAY service.
When the panel is displayed, the )INIT and )PROC sections of the panel are processed in the same way
the DISPLAY service would process them.
If you want to set variables before you display the panel, you can use the Variables option (7.3) to do
so. When you display the panel, you can type in new data or type over existing data, and then verify the
variables by using the Variables option (7.3) again. Data that you type on the panel is retained until you
change it, leave Dialog Test, or reset the function pool.
Figure 220 on page 367 shows the panel that is displayed if you specify message ID ISPD241 and,
optionally, a cursor position without identifying a panel name. The long message portion of the identified
message is displayed when you enter the HELP command on that panel.
                                                                 Invalid option
            This panel is used to display your specified message.
                ┌────────────────────────────────────────────┐
                │ The option that was entered was not valid. │
                ⋘────────────────────────────────────────────┘
 Command ===>                                                              
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 220. Message display panel (ISPYP2)
When you enter the END command from the panel being tested, the Display Panel reappears on the
screen.
Variables (Option 7.3)
The Variables option (7.3) allows you to:
• Display all the ISPF variables defined in the dialog application you are testing.
• Change the value of a variable by typing over it, unless the variable has an N (non-modifiable) attribute
or an H (Non-modifiable variable containing hexadecimal data) attribute.
• Define new variables by inserting lines or by changing the name or pool of a listed variable.
• Delete variable names and blank lines.
Variables (Option 7.3)
Chapter 9. Dialog test (option 7)  367

## Page 406

When you select this option, you can scroll a display showing all the current variables for the dialog being
tested, as shown in Figure 221 on page 368.
If the Variables panel is displayed in a pop-up window, you can increase the size of the Variables pop-up
window to fill the entire 3270 physical display area using the RESIZE command. The initial RESIZE
command increases the pop-up window to its maximum size, and the subsequent RESIZE reduces the
window to its original size.
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────
                     Variables - Application: ISR             Row 1 to 9 of 707
 Add, delete, and change variables. Underscores need not be blanked.
 Enter END command to finalize changes, CANCEL command to end without
 changes.
 Current scrollable width of variables is: 57
      Variable P A Value
                   ----+----1----+----2----+----3----+----4----+----5----+--
      Z        S N                                                          
      ZACCTNUM S N IBMGSA                                                   
      ZAPLCNT  S N 0000                                                     
      ZAPPLID  S N ISR                                                      
      ZBDMXCNT S N 000000000                                                
      ZCFGCMPD S N 2001/11/22                                               
      ZCFGCMPT S N                                                          
      ZCFGKSRC S N                                                          
      ZCFGLVL  S N 480R8001                                                 
 Command ===>                                              Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F4=Expand    F5=Rfind     F6=Resize
  F7=Backward  F8=Forward   F9=Swap     F10=Actions  F12=Cancel
Figure 221. Variables panel (ISPYVPN)
The Variables display is controlled by the selection you make in the View pull-down on the Dialog Test
Primary Option Panel action bar:
1
Display Variables in window. Variables are displayed in a pop-up window.
2
Display Variables full-screen. Variables are displayed full-screen.
Note: The current setting is shown as an unavailable choice; that is, it is displayed in blue (the default)
with an asterisk as the first digit of the selection number.
Variables panel action bar
The Variables Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
1
General Variables
Variables (Option 7.3)
368  z/OS: z/OS ISPF User's Guide Vol II

## Page 407

2
Definitions
3
Variables Panel
4
Manipulating Variables
5
Primary Commands
6
Line Commands
7
Usage Notes
8
General Dialog Test
Variables panel fields
Each line of the display represents a variable and contains a line command area. The fields on the
Variables panel function as follows:
Variable
Name of the variable. The variable name is alphanumeric, with the first character being either
alphabetic or one of the special characters $, #, or @. The variable name cannot be longer than
eight characters. All alphabetic characters are converted to uppercase when you press the Enter key.
The Variable field is required.
P
Pool that contains the variable; a required 1-character field, where:
V
Function pool; the variable was defined with the VDEFINE service.
I
Function pool; a variable that was created by a CLIST or REXX exec, or by using the Variables
option. This is an implicit variable, which was not explicitly defined by using the VDEFINE service.
S
Shared variable pool.
P
Profile variable pool.
A
Attributes of the variable, a non-modifiable 1-character field, where:
N
Non-modifiable variable. Some system-reserved variables are not modifiable.
H
Non-modifiable variable containing hexadecimal data. Some system-reserved variables are not
modifiable.
X
Variable containing hexadecimal data.
L
Long variable value. The variable value length exceeds the 57 characters displayed on the
Variables panel, but it does not exceed the 2048 characters displayed by the EXPAND command.
Use the LEFT and RIGHT commands to scroll the value or the EXPAND command to display the
complete value in a pop-up window.
Variables (Option 7.3)
Chapter 9. Dialog test (option 7)  369

## Page 408

T
Truncated variable value. The variable value length exceeds the 2048 characters displayed by
the EXPAND command. Use the LEFT and RIGHT commands to scroll the value or the EXPAND
command to display the first 2048 characters in a pop-up window. If the user changes the value,
the new value is limited to 2048 characters.
If the H, N, or X attribute is applicable to a variable and the L or T attribute is also applicable to the
variable, the H, N, or X attribute is displayed instead of the L or T attribute.
Value
Value of the variable. The value can be up to 2048 characters in length. Each value is scrollable, by
placing the cursor over the value and using the LEFT and RIGHT functions keys. The EXPAND function
key can be used to display a full screen of data and display hexadecimal values.
Variables commands
The Variables option (7.3) uses the CANCEL, END, EXPAND, LEFT, LOCATE, RIGHT, and SORT commands,
and the I (insert) and D (delete) Dialog Test line commands described in “Commands” on page 358. You
can change the displayed sort order using the SORT command. SORT with no operands sorts the list by
variable pools then by variable name. SORT NAME sorts the list by variable name and then by variable
pool. SORT VALUE sorts the list by the displayed Value field. The LOCATE command can be used to search
for a specific variable. LOCATE accepts as an operand the name, or first letters of the name of a variable.
If the name is not found, the list is positioned near the closest match. You can use the RFIND key to
continue searching other variable pools.
Normally, the variable pools are updated with the data from the display when you use the END command
to leave the option.
Manipulating variables
The rows of the display are sorted in this order:
1. By pool (function, then shared, then profile)
2. By function pool type (V, then I)
3. Alphabetically by variable name within each pool.
Insertions are left where they are typed on the display. Changes to the display are processed when you
press the Enter key. Updating of the variable pools occurs when you enter the END command.
Creating new variables
You can create new dialog variables, but you cannot create two variables with the same name in the same
variable pool.
To create a new variable, you can do one of two things:
• Use the I line command to insert a new row, and then type the variable name, pool, and value on
the new line. For each field, move the cursor to the start of the field and type new information. The
underscores are pad characters; you do not need to blank them out.
• Type over the name of an existing variable, its pool indicator, or both. This creates a new variable and
resets the old variable's value to nulls.
If you change a truncated value, the portion that cannot be displayed by the EXPAND command (beyond
character 2048) is lost. The new variable value is the value that can be displayed by the EXPAND
command.
New function pool variables are given an I (implicit) pool value and a CHAR format. If you type F in the P
(pool) field, ISPF changes it to I.
By using the second method, you can interchange the values of two or more variables by simply changing
their names. For example, you can interchange the values for variables A and B by changing the variable
name A to B and name B to A, and then pressing the Enter key.
Variables (Option 7.3)
370  z/OS: z/OS ISPF User's Guide Vol II

## Page 409

Deleting variables
Any dialog variable in the shared and profile pools can be deleted, unless it has an N or H attribute.
Though you cannot delete a variable from the function pool, you can set its value to blanks.
To delete a variable, use the D line command. However, to avoid conflict with the I (insert) line command,
the Variables option does not allow you to type a number along with the D command to delete more than
one line simultaneously. Therefore, enter a single D line command on each line you want to delete. You
can type this command on more than one line before pressing the Enter key.
Variables usage notes
When using the Variables option (7.3), you should be aware of:
Input errors
Correct any errors before leaving a display. If you cannot correct the errors, use the CANCEL
command.
Length and format errors in variables defined with the VDEFINE service are detected when you enter
the END command. If ISPF finds such an error, it prompts you to fix the variable value.
Test mode
Variable manipulations carried out under Dialog Test at a breakpoint are considered an extension
of your dialog and, as such, are handled in user mode. Dialog variables, table data, and service
return codes that you introduce, delete, or change are treated as though your dialog had made those
changes.
Variable life
Profile variables that you create remain in your profile pool from one Dialog Test session to another.
Shared and function variables exist only for the duration of Dialog Test.
Split-screen mode
In split-screen mode, two logical screens can share a profile variable pool. Since the Variables
option (7.3) takes a snapshot of the variables, any change to a profile variable on one screen is not
immediately reflected on the other screen. To get the latest changes, select the Variables option (7.3)
again. Also, when one profile variable is changed on two logical screens using split-screen mode, the
changed profile variable on the screen where the last END command was entered takes precedence.
Variable value
Variables defined with the VDEFINE service as non-character are displayed in converted form. Any
changes made to the variable's value should conform to the defined format.
Do not change them using the hexadecimal representation. A format or length error causes a message
to be displayed when you use the END command. When a VDEFINE error occurs, a panel identifies
the data and its value and describes the error. You must then correct the error and press the Enter
key. If you create a new variable by changing the pool indicator of an existing variable defined as
non-character, the new variable has character (CHAR) format.
Hexadecimal data
Hexadecimal data that cannot be displayed is converted to displayable characters or typed using the
form:
X'nnnnnnnn'
where:
n
An integer 0 through 9 or an alphabetic character A through F. There must be an even number of
characters within the quotation marks.
Variables (Option 7.3)
Chapter 9. Dialog test (option 7)  371

## Page 410

DBCS data
A variable defined as DBCS by the VDEFINE service or displayed through the field with
FORMAT(DBCS) specified in the test environment is displayed using the form:
'¬[DBDBDB]'
where:
[ and ]
Represent the SO (shift-out) and SI (shift-in) characters, respectively.
If you type a DBCS value in this format on the Variables panel, only the DBCS characters are stored.
Tables (option 7.4)
The Tables option (7.4) allows you to examine and manipulate the rows of a table, and to display table
structure and status. When you select this option, the panel in Figure 222 on page 372 is displayed, on
which you show the table function you want and the parameters needed to identify the table.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  View  Help                                               │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                                   Tables                                    │
 │                                                                More:     +  │
 │ 1 Display row            3 Modify row             5 Display structure       │
 │ 2 Delete row             4 Add row                6 Display status          │
 │                                                                             │
 │ Table Name  . .           Open tables  . .          (NOWRITE or WRITE or    │
 │                                                      blank for no TBOPEN)   │
 │ Row identification:            Current row  . :                             │
 │ By row number  . . *           (* = current row)                            │
 │  Variable   Value              (Search for row if row number blank)         │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ DBCS column specification:                                                  │
 │ Option ===>                                                                 │
 │  F1=HELP        F2=            F3=END         F4=DATASETS    F5=FIND        │
 │  F6=CHANGE      F9=SWAP       F10=LEFT       F11=RIGHT      F12=SUBMIT      │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 222. Tables panel (ISPYTPI)
Tables panel action bar
The Tables panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
View
Allows you to select whether the Add row, Modify row, and Display row panels are displayed in pop-up
windows. By displaying the panel in a pop-up window, you can see the panel underneath and move
the window. By displaying the panel as a full-screen display, you can see more data. The setting you
choose will remain in effect until you change it.
Help
The Help pull-down offers you these choices:
 1
General Tables
Tables (option 7.4)
372  z/OS: z/OS ISPF User's Guide Vol II

## Page 411

2
Definitions
 3
Tables Panel
 4
Display Row
 5
Delete Row
 6
Modify Row
 7
Add Row
 8
Display Structure
 9
Display Status
10
Usage Notes
11
View Action Bar
12
General Dialog Test
Tables panel fields
The fields on the Tables panel function as follows:
Table Name
The name of the table in which you are interested. The table must be open for all, except the Display
Status option (6) of the Tables option (7.4). The table can be opened using the Dialog Services option
(7.6) or can be opened automatically by specifying the Open tables field WRITE/NOWRITE option.
Open tables
The table must be open for all but the Display Status option (6) of the Tables option (7.4). Specifying
WRITE or NOWRITE will cause Tables option to automatically open the table in the respective mode. A
blank field directs Tables to bypass the automatic open. If Tables automatically opens the table, it will
be closed automatically when the Tables option is terminated. This is done with a TBCLOSE service
which will cause any table changes to be preserved if the table were automatically opened with the
WRITE option.
This field is ignored if the table has been opened outside of the Tables (7.4) option.
Row identification
Identify a row, either directly by row number or indirectly by specifying table variable names and their
values as search operands.
Current row
The position number of the current row after you have identified a table. This field is not modifiable.
By row number
The position number of the table row that you want, or, if you are adding a row, you can use:
TOP
Makes the new row first in the table.
BOTTOM
Makes the new row last in the table.
Tables (option 7.4)
Chapter 9. Dialog test (option 7)  373

## Page 412

Variable
The names of variables whose values are to be used to search the table for a row with matching
contents. You insert them by typing over the underscores beneath this heading.
Value
The value to be used in the search, up to 54 characters. For an abbreviated search, type the beginning
characters followed by an asterisk.
You can specify a DBCS value in the form:
¬[DBDBDB]
where:
[ and ]
Represent the SO (shift-out) and SI (shift-in) characters, respectively. For an abbreviated search,
type a 2-byte asterisk (*) at the end of the DBCS value. For example:
¬[DBDB**]
where ** represents the 2-byte asterisk character.
DBCS column specification
The variable names of the values that are DBCS data. The value of the variable is displayed using the
form:
¬[DBDBDB]
If you type a DBCS value in this format on the Modify Row panel or the Add Row panel, only the DBCS
characters are stored, regardless of the DBCS column specification.
Option
The number of one of the functions displayed on the Tables panel.
Note: The option names (for example, Display row) are point-and-shoot fields; however, if an option is
already specified at the Option prompt, it takes precedence over your point-and-shoot selection.
Once you specify a table name, it is retained until you change it or until you leave Dialog Test.
For the Display row (1), Delete row (2), Modify row (3), and Add row (4) options of the Tables option (7.4),
you must identify the row you want to display, delete, modify, or add. To do this, you can specify a row
number in the "By row number" field, or you can use the Variable and Value fields to specify a list of
search operands. To show the current row, leave the asterisk in the "By row number" field. If you specify
both a row number and a search operand, the row number is used and the search operand is ignored.
The current row pointer in the table can be changed only at your request or by your dialog.
The list of search operands consists of variable names and values that allow you to specify the values that
specific variables have in a row. You can specify the complete value, abbreviate the value with an asterisk
to find a row containing a variable beginning with specified characters, or leave the row blank. The search
begins with the row following the current row. If a row matching the search operand is not found, the
current row pointer is set to the top. You can repeat the search, if necessary.
Tables panel options
Subsequent topics describe the options at the top of the Tables panel.
1—display row
You can use the Display row option to display the contents of an existing row in an open table. When you
select the Display row option, perform these tasks on the Tables panel:
• Specify the name of a table in the Table Name field. If the table is not open, specify NOWRITE or WRITE
in the "Open tables" field.
Tables (option 7.4)
374  z/OS: z/OS ISPF User's Guide Vol II

## Page 413

• Specify a row number or a search operand list to identify a row.
Note: Use the View action bar choice on the Tables panel to specify whether this display is to be in a
pop-up or full-screen.
When you press Enter, you are shown the table row data on a display that you can scroll (Figure 223
on page 375). The pop-up window can also be resized using the RESIZE command. In the figure, the
variables constitute one table row.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                 Display row   Table ISPKEYS   row 1       Row 1 to 13 of 74 │
 │                                                                             │
 │ Variable   T  A   Value                                                     │
 │ KEYLISTN   K      ISPTEST                                                   │
 │ KEY1DEF    N      HELP                                                      │
 │ KEY1LAB    N      Help                                                      │
 │ KEY1ATR    N      SHORT                                                     │
 │ KEY2DEF    N      SPLIT                                                     │
 │ KEY2LAB    N      Split                                                     │
 │ KEY2ATR    N      LONG                                                      │
 │ KEY3DEF    N      EXIT                                                      │
 │ KEY3LAB    N      Exit                                                      │
 │ KEY3ATR    N      SHORT                                                     │
 │ KEY4DEF    N                                                                │
 │ KEY4LAB    N                                                                │
 │ KEY4ATR    N                                                                │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 223. Display row panel (ISPYTPD)
Each line on the display shows:
Variable
Variable name
T
Type of variable:
K
Key variable.
N
Name variable; non-key.
S
Save (extension) variable.
A
Attribute of each variable:
T
Truncated to 58 characters for display.
Value
The first 58 characters of the variable value.
Display row commands
The Display Row option uses the END and LOCATE commands described in “Commands” on page 358.
2—delete row
You can use the Delete row option to remove an existing row from an open table. When you select the
Delete row option, perform these tasks on the Tables panel:
Tables (option 7.4)
Chapter 9. Dialog test (option 7)  375

## Page 414

• Specify the name of a table in the Table Name field. If the table is not open, specify NOWRITE or WRITE
in the "Open tables" field.
• Specify a row number or a search operand list to identify a row.
When you press Enter, a panel is displayed (Figure 224 on page 376) to allow you to confirm the delete
request.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  View  Help                                               │
 │ ─ ┌─────────────── Confirm Table Row Delete ────────────────┐ ────────────  │
 │ I │ ISPYTPCD                                                │               │
 │   │                                                         │  More:     +  │
 │ 1 │ Table name  : ISPKEYS                                   │ ructure       │
 │ 2 │                                                         │ atus          │
 │   │ Row number  : 1                                         │               │
 │ T │                                                         │ r WRITE or    │
 │   │                                                         │  no TBOPEN)   │
 │ R │          Press ENTER key to confirm delete.             │               │
 │ B │                                                         │               │
 │   │          Press END key to cancel delete.                │ lank)         │
 │   │ Command ===>                                            │               │
 │   │  F1=Help       F2=Split      F3=Exit       F7=Backward  │               │
 │   │  F8=Forward    F9=Swap      F12=Cancel                  │               │
 │   ⋘─────────────────────────────────────────────────────────┘               │
 │                                                                             │
 │                                                                             │
 │ DBCS column specification:                                                  │
 │ Option ===> 2                                                               │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 224. Confirm  table row delete panel (ISPYTPCD)
The fields on the panel are:
Table name
Name of an open table
Row number
Number of the row to be deleted.
Press Enter to delete the row, or enter the END or CANCEL command to cancel the deletion.
3—modify row
You can use the Modify row option to change the contents of an existing row of an open table. When you
select the Modify row option, perform these tasks on the Tables panel:
• Specify the name of a table in the Table Name field. If the table is not open, specify NOWRITE or WRITE
in the "Open tables" field.
• Specify a row number or a search operand list to identify a row.
Note: Use the View action bar choice on the Tables panel to specify whether this display is to be in a
pop-up or full-screen.
When you press Enter, a display that you can scroll (Figure 225 on page 377) is shown. The pop-up
window can also be resized using the RESIZE command.
Tables (option 7.4)
376  z/OS: z/OS ISPF User's Guide Vol II

## Page 415

┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                 Modify row   Table ISPKEYS   row 1        Row 1 to 10 of 74 │
 │                                                                             │
 │ Modify variable values and savenames. Underscores need not be blanked.      │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │       Variable  T A Value                                                   │
 │       KEYLISTN  K   ISPTEST                                                 │
 │       KEY1DEF   N   HELP                                                    │
 │       KEY1LAB   N   Help                                                    │
 │       KEY1ATR   N   SHORT                                                   │
 │       KEY2DEF   N   SPLIT                                                   │
 │       KEY2LAB   N   Split                                                   │
 │       KEY2ATR   N   LONG                                                    │
 │       KEY3DEF   N   EXIT                                                    │
 │       KEY3LAB   N   Exit                                                    │
 │       KEY3ATR   N   SHORT                                                   │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 225. Modify row panel (ISPYTPM)
Each line on the panel represents a variable in row 6 of the table and contains a line command field and
these fields:
Variable
Variable name, modifiable only for save variables.
T
Type of variable, non-modifiable:
K
Key variable.
N
Name variable; non-key.
S
Save (extension) variable.
A
Attribute of each variable, non-modifiable:
T
Truncated to 2048 characters for display.
Value
Value of the variable, up to 2048 characters.
Type in new values or change the current values for the key, name, and save variables in the Value
column. Enter new save variables by typing over the underscores in the Name column with the variable
names and specifying the desired values. The underscores are pad characters; you do not need to blank
them out.
When using the Modify row option, be aware that:
• If the table has keys, the values for the keys in the added row must be different from those in the
existing rows when you leave the Modify row option. Otherwise, a message is displayed and the row is
displayed again so you can change the keys.
• If the table was sorted using the TBSORT dialog service and a sort field is modified, the row's position in
the table can change to preserve the search order.
• You cannot change the variable name for a key variable or name variable; if you do, an error message is
displayed and the original name is restored.
• You cannot delete a key or name variable and its value from the display or table row.
Tables (option 7.4)
Chapter 9. Dialog test (option 7)  377

## Page 416

• If you delete a save variable, assume that the variable no longer exists in this row.
• If more than one variable entry has the same name, all instances of that variable are assigned the value
of the last occurrence of the variable; that is, the occurrence closest to the bottom of a display that you
can scroll.
• Blank save names are ignored and do not need to be deleted, even if data is left in the value.
• Hexadecimal data that usually cannot be displayed is converted to characters that can be displayed or
is typed by using the form:
X'nnnnnnnn'
where:
n
An integer 0-9 or an alphabetic character A-F. There must be an even number of characters within
the quotation marks.
• Variables defined with the VDEFINE service as non-character are shown in converted form; do not
change them by using the hexadecimal representation. A format or length error causes a message to be
displayed when you use the END command.
• When you leave the Modify row option by using the END command, the row is replaced, and the
message Row modified is issued.
Modify row commands
The Modify row option uses the CANCEL, END, and LOCATE commands, and the D (delete), I (insert), and
R (repeat) Dialog Test line commands described in “Commands” on page 358. Inserted and repeated
lines always have a type of S, because only save variables can be added to (or deleted from) a row of an
existing table.
4—add row
You can use the Add row option to add a new row after a selected row of an opened table. When you
select the Add row option, perform these tasks on the Tables panel:
• Specify the name of a table in the Table Name field. If the table is not open, specify NOWRITE or WRITE
in the "Open tables" field.
• Specify a row number or a search operand list to identify a row.
Note: Use the View action bar choice on the Tables panel to specify whether this display is to be in a
pop-up or full-screen.
Tables (option 7.4)
378  z/OS: z/OS ISPF User's Guide Vol II

## Page 417

┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                Add row   Table ISPKEYS   after row 1      Row 1 to 10 of 73 │
 │                                                                             │
 │ Add variable values and savenames. Underscores need not be blanked.         │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │       Variable  T A Value                                                   │
 │       KEYLISTN  K                                                           │
 │       KEY1DEF   N                                                           │
 │       KEY1LAB   N                                                           │
 │       KEY1ATR   N                                                           │
 │       KEY2DEF   N                                                           │
 │       KEY2LAB   N                                                           │
 │       KEY2ATR   N                                                           │
 │       KEY3DEF   N                                                           │
 │       KEY3LAB   N                                                           │
 │       KEY3ATR   N                                                           │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 226. Add row panel (ISPYTPA)
When you press Enter, a scrollable display is shown (Figure 226 on page 379), containing all the key and
name variables in the table. The pop-up window can also be resized using the RESIZE command.
Each row of the display contains a line command field and these fields:
Variable
Variable name.
T
Type of variable, non-modifiable:
K
Key variable.
N
Name variable; non-key.
S
Save (extension) variable.
A
Attribute of each variable. This attribute is non-modifiable and is not used for this option.
Value
Space for the variable value to be added, up to 2048 characters.
Type the values for the key and name variables in the Value column, which is originally initialized to all
nulls. You cannot change the names of the key and name variables because they were established when
the table was created.
You can enter save variables, identified as TYPE S, by typing over the underscores with the save variable
names and specifying the desired values. The underscores are pad characters; you do not need to blank
them out.
You can add a row with no values to the table, but you are asked to confirm such an action to guard
against inadvertent use of the END command.
When using the Add row option, be aware that:
• The position of the new row in the table depends on whether the table was previously sorted using the
TBSORT dialog service. If the table was sorted, the new row is placed in sort order; if it has not been
sorted, the new row is placed after the row you specified.
• You cannot delete a key or name variable and its value from the display or table row.
Tables (option 7.4)
Chapter 9. Dialog test (option 7)  379

## Page 418

• You cannot change the variable name for a key or name variable; if you do, an error message is
displayed and the original name is restored.
• If more than one variable entry has the same name, all instances of that variable are assigned the value
of the last occurrence of the variable; that is, the occurrence closest to the bottom of the display that
you can scroll.
• If the table has keys, the values for the keys in the added row must be different from those in all the
existing rows when you leave the Add Row option. Otherwise, a message is displayed and the row is
displayed again so you can change the keys.
• Blank save names are ignored and do not need to be deleted, even if data is left in the value.
• Hexadecimal data that usually cannot be displayed is converted to characters that can be displayed or
is typed by using the form:
X'nnnnnnnn'
where:
n
An integer 0-9 or an alphabetic character A-F. There must be an even number of characters within
the quotation marks.
• Variables defined with the VDEFINE service as non-character are shown in converted form; do not
change them by using the hexadecimal representation. A format or length error causes an error
message to be displayed when you use the END command.
Add row commands
The Add row option uses the CANCEL, END, and LOCATE commands, and the D (delete), I (insert), and
R (repeat) Dialog Test line commands described in “Commands” on page 358. Inserted and repeated
lines always have a type of S, because only save variables can be added to (or deleted from) a row of an
existing table.
5—display structure
When you select the Display structure option on the Tables panel, you are shown a display of the table
structure for the table specified in the Table Name field. You can scroll this display (Figure 227 on page
380) using the scroll commands. The table name appears in the panel header.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │ ISPYTPSR                Structure of Table ISPKEYS         Row 1 to 8 of 17 │
 │                                                                             │
 │ Number of keys . . : 1            Number of rows  . . . : 7                 │
 │ Number of names  . : 72           Current row pointer . : 1                 │
 │ KEYS:      KEYLISTN                                                         │
 │                                                                             │
 │ NAMES:     KEY1DEF      KEY1LAB      KEY1ATR      KEY2DEF      KEY2LAB      │
 │            KEY2ATR      KEY3DEF      KEY3LAB      KEY3ATR      KEY4DEF      │
 │            KEY4LAB      KEY4ATR      KEY5DEF      KEY5LAB      KEY5ATR      │
 │            KEY6DEF      KEY6LAB      KEY6ATR      KEY7DEF      KEY7LAB      │
 │            KEY7ATR      KEY8DEF      KEY8LAB      KEY8ATR      KEY9DEF      │
 │            KEY9LAB      KEY9ATR      KEY10DEF     KEY10LAB     KEY10ATR     │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=HELP        F2=            F3=END         F4=DATASETS    F5=FIND        │
 │  F6=CHANGE      F9=SWAP       F10=LEFT       F11=RIGHT      F12=SUBMIT      │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel                                                        
Figure 227. Structure of table panel (ISPYTPSR)
The display contains these fields:
Tables (option 7.4)
380  z/OS: z/OS ISPF User's Guide Vol II

## Page 419

Number of keys
Number of key variables in a row.
Number of names
Number of name variables in a row.
Number of rows
Number of rows currently in the table.
Current row pointer
Current row pointer value.
KEYS
A list of the names of all the key variables.
NAMES
A list of the names of all the name variables.
Display structure command
The KEYS and NAMES lists can be scrolled, and you can use the LOCATE command to find a specific
variable name. See “Finding a character string” on page 360 for information about its use.
6—display status
If you select the Display status option from the Tables panel, one of two data information panels is
displayed for the table specified in the Table Name field. The information reflects all operations using the
specified table, including those done at your request by the Tables options under Dialog Test.
Table not open
If the table is not open for your user ID, you are shown a Status of Table panel (Figure 228 on page 381)
with the value NOT OPEN in the "Status for this screen" field.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                          Status of Table ISPKEYS                            │
 │                                                                More:     +  │
 │ Status for this screen  : NOT OPEN      Date created        : 1995/01/17    │
 │ Table available         : YES           Time created        : 13:47:06      │
 │                                         Last date modified  : 2002/06/03    │
 │                                         Last time modified  : 22:10:26      │
 │                                         Last modified by    : JPHILP        │
 │                                         Original row count  : 1             │
 │                                         Current row count   : 7             │
 │                                         Modified row count  : 7             │
 │                                         Update count        : 83            │
 │                                                                             │
 │                                       Virtual storage                       │
 │ Command ===>                                                                │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel                                                        
Figure 228. Status of table panel with table not open (ISPYTPS1)
The panel contains these fields:
Status for this screen
Shows that the table is NOT OPEN for this logical screen.
Table available
YES or NO; whether you can open the table.
Date created
Date the table was created; shown in national format.
Tables (option 7.4)
Chapter 9. Dialog test (option 7)  381

## Page 420

Time created
Time the table was created.
Last date modified
Date the table was last modified; shown in national format.
Last time modified
Time the table was last modified.
Last modified by
User ID of the user who last changed the table.
Original row count
The number of rows that were added to a newly created table before closing the table for the first
time.
Current row count
The number of rows currently in the table.
Modified row count
The number of rows in the table that have been changed at least once. A row that has been added to
an existing table is also considered a changed row.
Update count
Number of times the table has been modified. One or more updates during any table open or close
sequence increments this counter by one.
Virtual storage
The number of bytes of virtual storage required by the table when it is open.
The Modify row option on the Tables panel allows you to change a key of a keyed table by adding the new
row and deleting the old row. The row counts thus reflect this processing when changing a key value.
Table open
If the table is open for your user ID, you are shown a Status of Table panel (Figure 229 on page 382) with
the value OPEN in the Status for this screen field.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                          Status of Table ISPKEYS                            │
 │                                                                More:     +  │
 │ Status for this screen  : OPEN            Date created  . . . : 1995/01/17  │
 │ Open option . . . . . . : NOWRITE         Time created  . . . : 13:47:06    │
 │ Table on disk . . . . . : YES             Last date modified. : 2002/06/03  │
 │ Last table service  . . : TBQUERY         Last time modified. : 22:10:26    │
 │ Last service return code: 00              Last modified by  . : JPHILP      │
 │ Current row pointer . . : TOP             Original row count. : 1           │
 │                                           Current row count . : 7           │
 │                                           Modified row count. : 7           │
 │                                           Update count  . . . : 83          │
 │                                                                             │
 │                                       Virtual storage                       │
 │ Command ===>                                                                │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel                                                        
Figure 229. Status of table panel with table open (ISPYTPS1)
This panel contains these fields:
Status for this screen
Shows that the table is OPEN for this logical screen.
Open option
Option used to open the table; this value can be WRITE, NOWRITE, SHR WRITE, or SHR NOWRITE.
Tables (option 7.4)
382  z/OS: z/OS ISPF User's Guide Vol II

## Page 421

Table on disk
Whether the table has been saved on disk; this value can be YES or NO.
Last table service
Name of the last table service called.
Last service return code
Last table services return code.
Current row pointer
Current position in the table.
Date created
Date the table was created; shown in national format.
Time created
Time the table was created.
Last date modified
Date the table was last modified; shown in national format.
Last time modified
Time the table was last modified.
Last modified by
User ID of the user who last changed the table.
Original row count
The number of rows that were added to a newly created table before closing the table for the first
time.
Current row count
The number of rows currently in the table.
Modified row count
The number of rows in the table that have been changed at least once. A row that has been added to
an existing table is also considered a changed row.
Update count
Number of times the table has been modified. One or more updates during any table open or close
sequence increments this counter by one.
Virtual storage
The number of bytes of virtual storage required by the table.
The Modify row option on the Tables panel allows you to change a key of a keyed table by adding the new
row and deleting the old row. The row counts thus reflect this processing when changing a key value.
Log (option 7.5)
The Log option (7.5) allows you to display and browse data recorded in the ISPF transaction log, as shown
in Figure 230 on page 384.
Log (option 7.5)
Chapter 9. Dialog test (option 7)  383

## Page 422

Browse log - USERID.SPFLOG2.LIST                     Line 00000000 Col 007 086
********************************* Top of Data **********************************
Time                *** ISPF transaction log ***                        Userid:
09:03   Start of ISPF Log - - -  - Session # 16 --------------------------------
10:15      TSO     - Command  -  - SUBMIT NOTIFY
10:37   ***** Dialog Error ***** - Application(ISR); Function Module (ISR@USER);
10:37      Line from panel:      - )BODY  EXPAND(//) WIDTH(&ZWIDTH)  CMD(ZCMD)
10:37      Panel 'ISPYLP1' error - Invalid WIDTH value, (must be numeric chars,
******************************** Bottom of Data ********************************
 Command ===>                                                  Scroll ===> PAGE
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 230. ISPF transaction log (ISPYLP1)
You can use all the Browse commands, except BROWSE, while looking at the ISPF log.
ISPF transaction log not available
Sometimes the log is not available for browsing. This can occur when:
• The log data set is empty.
• The log data set was not created for this session because 0 was entered in the "Primary pages" field on
the Log Set Defaults and List Set Defaults pop-ups, which can be reached by selecting Option 0 from
the ISPF Primary Option Menu and then selecting the Log Data set defaults and List Data set defaults
choices from the Log/List pull-down.
• No data has been written to the log during this session, and although the log data set exists and is
not empty, you did not end the last ISPF session normally; for example, an abend can have ended the
session. You can browse the log if you take an action that causes a log entry to be written.
• The log data set was previously allocated with a disposition of OLD. It must be allocated with a
disposition of MOD.
• The log data set has been previously allocated to SYSOUT.
Trace output in ispf log
This trace output is written to the ISPF log:
• Trace header entries
• Function trace entries
• Variable trace entries.
Each type of entry follows the format of other log entries: a short summary on the left, and a detailed
entry on the right.
Trace header entries
The first line of trace data is a trace header that identifies the trace and shows the current application ID,
the current function, and the current screen. For split-screen mode, the original screen is 1 and the screen
generated by the SPLIT command is 2. The summary section of the header entry identifies the entry as
a dialog trace. The trace header entry is written during the test session whenever a function or variable
trace entry is to be written for an application, function, or screen that differs from the last.
Log (option 7.5)
384  z/OS: z/OS ISPF User's Guide Vol II

## Page 423

For example, a trace of logical screen 1 of function TESTF1 in application ISR would place this line in the
ISPF log:
DIALOG TRACE ---- - APPLICATION(ISR)  FUNCTION(TESTF1)  SCREEN(1)
Function trace entries
A pair of function trace entries, a BEGIN entry and an END entry, is generated during a function trace
for each traced dialog service that is called. A service can be called from a user dialog that is currently
processing, or from a Dialog Test action for the user. The summary portion of each of these entries shows
the name of the dialog service, whether it is the beginning or the end of its processing, and whether it was
called indirectly from a Dialog Test panel. If the word TEST does not appear, the user dialog called the
service directly. For END entries, the service return code is shown on a second line.
The detailed section of the log entries contains an image of the service call and the parameters used to
call that service, using two lines if necessary. For example:
DISPLAY .. BEGIN ... TEST - DISPLAY PANEL(XYZ)
⋮
DISPLAY .. END.. ... TEST - DISPLAY PANEL(XYZ)
  ..RETURN CODE (0)
There can be many log entries between the begin and end entries. For example, any active variable traces
can cause log entries during a SELECT trace.
Note these aspects about the service call image:
• The image is truncated after the second line.
• ISPEXEC calls are shown as typed in the dialog.
• ISPLINK and ISPLNK calls (except for the ISREDIT service) are displayed with their parameter values
separated by commas. Name-lists are shown as typed in the dialog, in either string or structure
format. Structure format includes the count, element length, and list of names. For a variable services
parameter whose context is defined by the name-list parameter on the service call, the first four bytes
of the parameter value are displayed in hexadecimal format (X'nnnnnnnn').
• Dialog Test calls are shown using the command call format without the ISPEXEC prefix.
Variable trace entries
Two variable trace entry lines are generated for each variable trace log entry. The variable can be referred
to or set by a user dialog directly or indirectly by a dialog service, or explicitly set by a Dialog Test option
for a user. The summary parts of these entry lines identify the trace. Line one shows the name of the
variable, the pool that contains it (F for function, S for shared, P for profile), and an indicator (TEST) if a
Dialog Test option set the value. Line two shows the operation done for the variable (GET, PUT, or CHG)
and the name of the dialog service that did the operation for non-TEST entries.
The current value of the variable is printed in the detail section of the log entry and can span two lines. For
example:
LIB1.... POOL(P) .... - VALUE(FLAG)
 ...GET by EDIT       -
The value is truncated after the second line.
If the variable value contains characters that cannot be displayed, the value is displayed in hexadecimal
format (X'nnnnnnnn').
Dialog services (option 7.6)
The Dialog Services option (7.6) allows you to call a dialog service by entering the service call with or
without the ISPEXEC characters.
Figure 231 on page 386 shows the Invoke Dialog Service panel.
Dialog services (option 7.6)
Chapter 9. Dialog test (option 7)  385

## Page 424

Menu  List  Mode  Functions  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Invoke Dialog Service
 Enter dialog service and its parameters:
 ===>                                                                           
                                                        
 Place cursor on choice and press enter to RETRIEVE command.
 => rempop
 => addpop
 => ispexec display
 => ispexec display panel(sample)
 =>
 =>
 =>
 =>
 =>
 =>
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F4=Expand    F5=Rfind     F6=Resize
  F7=Backward  F8=Forward   F9=Swap     F10=Actions  F12=Cancel
Figure 231. Invoke dialog service panel (ISPYS1)
On this panel, if you want to display panel XYZ, enter:
===> DISPLAY PANEL(XYZ)
or
===> ISPEXEC DISPLAY PANEL(XYZ)
The service is called when you press the Enter key. You are informed of the service's completion and
return code.
You can call any dialog service that is valid in the command environment except CONTROL at a breakpoint
or before calling a function.
The Invoke Dialog Service panel has a saved command area (the bottom portion of the screen) that
contains a list of up to 10 commands that you have saved. These commands are point-and-shoot fields.
The mode you specify from the Mode pull-down menu on the action bar determines what happens when
you select a command.
Invoke dialog service panel action bar
The Invoke Dialog Service panel action bar choices function as follows:
Note: The Invoke Dialog Service panel action bar contains three pull-down choices that let you control the
saved command area.
• List
• Mode
• Functions.
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
List
The List pull-down offers you these choices:
Note: The current setting is shown as an unavailable choice; that is, it is displayed in blue (the default)
with an asterisk as the first digit of the selection number.
Dialog services (option 7.6)
386  z/OS: z/OS ISPF User's Guide Vol II

## Page 425

Update On
Makes the list of commands in the saved command area live; that is, new commands are
appended to the list automatically.
Update Off
Makes the list of commands in the saved command area static; that is, new commands are not
appended to the list automatically.
Mode
The Mode pull-down offers you these choices:
Note: The current setting is shown as an unavailable choice; that is, it is displayed in blue (the default)
with an asterisk as the first digit of the selection number.
Retrieve
Allows commands to be retrieved from the saved command area and placed on the TSO Command
Entry field (==>) so that you can edit them before they are executed. This mode is the default.
Execute
Allows commands to be retrieved from the saved command area and executed in one step.
Delete
Allows you to delete commands from the saved command area without executing the commands.
Place the cursor on the command to be deleted and press Enter. The command will be blanked
out.
Functions
The Functions pull-down offers you this choice:
Compress List
Compresses the saved command area by removing deleted entries.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
1
General
2
General Dialog Test
Special display panel
If you issue the DISPLAY service call with only a message parameter, or with no parameter at all, the
Special Display Panel is shown (Figure 232 on page 388).
Dialog services (option 7.6)
Chapter 9. Dialog test (option 7)  387

## Page 426

┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │                           Special Display Panel                             │
 │                                                                             │
 │ This panel is used for two special DISPLAY conditions:                      │
 │                                                                             │
 │ 1.  When DISPLAY is invoked without a panel name.                           │
 │                                                                             │
 │ 2.  When TBDISPL is invoked without a panel name. All of the other          │
 │     parameters are ignored.                                                 │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ Command ===>                                                                │
 │  F1=Help     F2=Split    F3=Exit     F9=Swap    F12=Cancel                  │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 232. Special display panel (ISPYS2)
Traces (option 7.7)
The Traces option (7.7) allows you to define, change, and delete trace specifications. You can trace
processed dialog services, except for the VPUT or VGET service issued from a panel, and dialog variables
to which are referred during dialog processing. Trace data is placed in the transaction log, where you can
browse it by using the Log option (7.5) or print it when you leave ISPF. You can also print the log data set
during an ISPF session by using the ISPF LOG command.
Since tracing can degrade dialog performance and create large amounts of output, you should be careful
in setting the scope of trace definitions.
When you select this option, a selection panel is displayed (Figure 233 on page 388) on which you can
show the type of trace you want to define.
   Menu  Utilities  Compilers  Options  Status  Help
 ─ ┌──────────────────────── Dialog Test ─────────────────────────┐ ───────────
   │   Menu  Utilities  Help                                      │
   │ ───────────────────────────────────────────────────────────  │
 0 │                           Traces                             │ : SUEBURN
 1 │                                                              │ : 11:00
 2 │ 1  Function Traces   Monitor dialog service calls            │ : 3278
 3 │ 2  Variable Traces   Monitor dialog variable usage           │ : 1
 4 │                                                              │ : ENGLISH
 5 │ Option ===>                                                  │ : ISR
 6 │  F1=Help        F2=Split       F3=Exit        F4=Expand      │ : ISPF
 7 │  F5=Rfind       F6=Resize      F7=Backward    F8=Forward     │ : SUEBURN
 9 ⋘──────────────────────────────────────────────────────────────┘ : MVS8
 10 SCLM          SW Configuration Library Manager        MVS acct. : IBMGSA
 11 Workplace     ISPF Object/Action Workplace            Release . : ISPF 5.9
 12 z/OS System   z/OS system programmer applications
 13 z/OS User     z/OS user applications
      Enter X to Terminate using Log/List defaults
 Option ===> 7
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 233. Traces panel (ISPYRI1)
Subsequent topics describe the options shown at the top of the Traces panel.
Traces (option 7.7)
388  z/OS: z/OS ISPF User's Guide Vol II

## Page 427

1—function traces
The Function Traces option on the Traces panel is used to establish criteria for recording the names
of dialog service calls, the service parameters, and return code in the ISPF log. If either a dialog or
Dialog Test processing causes a service call, that call is recorded in the trace. An example of Dialog Test
processing that causes a service call is the use of the Panels option (7.2) to display a panel. Whenever a
new application or function causes data to be recorded, a header is placed in the trace.
When you select the Function Traces option, you are shown a panel that you can scroll (Figure 234
on page 389). The pop-up window can also be resized using the RESIZE command. The panel lists all
currently defined function traces.
You can add, delete, and change function trace definitions by using this panel, either before calling a
function or at a breakpoint.
 ┌────────────────────────────── Function Traces ──────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                                                            Row 1 to 7 of 13 │
 │                                                                             │
 │ Add, delete, and change traces.  Underscores need not be blanked.           │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │         Function    Active       Dialog services to be traced               │
 │        (Required)  (YES,NO)     (No entry=all)                              │
 │                 (No entry=YES)  ("OR" is assumed between names)             │
 │                                                                             │
 │         ALL           NO                                                    │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 234. Function traces panel (ISPYRFP)
Each line defines a function trace, showing a line command area and these fields:
Function
The name of the user function that should contain the trace, or ALL to trace every dialog function.
Initially, ALL is presented on the display but is not started. Change the NO to a YES in the Active
column to start such a trace. If you want to trace a function whose name is ALL, enclose the name in
single quotes to distinguish it; that is, type 'ALL', not ALL.
Active
Whether the trace is to be active now:
YES
The trace is currently active.
NO
The trace is currently not active.
Blank
The trace is currently active.
Dialog services to be traced
Names of dialog services to be traced. No entry in this field shows all calls to dialog services for the
function are to be traced.
All function traces exist until you leave Dialog Test, or until you delete them from this panel. Enter new
information by typing over the existing data. The underscores are pad characters to show the starting and
ending positions for each field; you do not need to blank them out. You can create several function traces
before you press the Enter key.
Traces (option 7.7)
Chapter 9. Dialog test (option 7)  389

## Page 428

During dialog processing, to determine whether the criteria for a function trace have been met, Dialog Test
processes a logical AND of the Function, Active, and Dialog services fields specified for that function trace.
Dialog Test also processes a logical OR within the Dialog services field to determine whether a particular
dialog service has been matched. Therefore, if you want more than one trace for a function, you should
create multiple rows.
Function traces commands
The Function Traces option uses the CANCEL, END, and LOCATE commands, and the D (delete), I (insert),
and R (repeat) Dialog Test line commands described in “Commands” on page 358.
2—variable traces
The Variable Traces option on the Traces panel is used to establish criteria for recording variable usage. A
variable's usage is recorded if an ISPF service is directly asked to operate on the variable (such as VGET,
VPUT, and VCOPY), or if an ISPF service is indirectly asked to operate on the variable (such as DISPLAY).
Variables changed under the Variables option (7.3) are also recorded if the trace specifications are met.
When you select the Variable Traces option, you are shown a display that you can scroll (Figure 235
on page 390). The pop-up window can also be resized using the RESIZE command. The display lists
all currently defined variable traces. You can add, delete, and change variable trace definitions at a
breakpoint, or by using this panel before calling a function.
 ┌────────────────────────────── Variable Traces ──────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │                                                            Row 1 to 8 of 13 │
 │                                                                             │
 │ Add, delete, and change traces.  Underscores need not be blanked.           │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │          Variable       Pool        Operation      Function     Active      │
 │         (Required) (No entry=all) (GET,PUT,CHG) (No entry=all) (YES,NO)     │
 │                                 (No entry=all)              (No entry=YES)  │
 │         ALL                                                      NO         │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ Command ===>                                             Scroll ===> PAGE   │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 235. Variable traces panel (ISPYRVP)
Each line defines a variable trace, showing a line command area and these fields:
Variable
Name of the variable to be traced, or ALL to show tracing of all variables. Initially, ALL is presented on
the display but is not started. Change the NO in the Active column to YES to start such a trace. If you
want to trace a variable whose name is ALL, enclose that name in single quotes to distinguish it; that
is, type 'ALL', not ALL.
Pool
Pool of interest for variable tracing:
F
Function variable pool.
S
Shared variable pool.
Traces (option 7.7)
390  z/OS: z/OS ISPF User's Guide Vol II

## Page 429

P
Profile variable pool.
Blank
All pools.
Operation
Type of variable reference to trace:
GET
Accesses to the variable's value.
PUT
Stores to the variable's value.
CHG
Changes to the variable's value.
Blank
All references to variable are traced.
Function
If there is no entry, this variable is traced for all functions.
Active
Indication of whether the trace is to be active:
YES
The trace is currently active.
NO
The trace is currently not active.
Blank
The trace is currently active.
All variable trace definitions exist until you leave Dialog Test, or until you delete them from this panel.
Enter new information by typing over the existing data. The underscores are pad characters to show the
start and end of each field; you do not need to blank them out. You can create several variable traces
before you press the Enter key.
During dialog processing, to determine whether the criteria for a variable trace have been met, Dialog
Test processes a logical AND of the Variable, Pool, Operation, Function, and Active fields specified for that
variable trace. Therefore, if you want more than one trace for a variable, you should create multiple rows.
Variable traces commands
The Variable Traces option uses the CANCEL, END, and LOCATE commands, and the D (delete), I (insert),
and R (repeat) Dialog Test line commands described in “Commands” on page 358.
Breakpoints (option 7.8)
A breakpoint is a location at which the processing of your dialog is suspended so that you can use Dialog
Test facilities. The Breakpoints option (7.8) allows you to show where such temporary suspensions should
occur. At a breakpoint, you can examine and manipulate dialog data such as tables and variables. You can
also specify new test conditions, such as traces and other breakpoints.
Breakpoints are located immediately before a dialog service receives control or after it relinquishes
control. Breakpoint definitions cause special handling within the ISPLINK, ISPLNK, or ISPEXEC interfaces
to dialog services; no user dialog code is modified. When the criteria for a breakpoint are satisfied, your
dialog is suspended. You can then do any of the functions shown on the Breakpoint Primary Option Panel.
You cannot use as a breakpoint the VPUT or VGET service issued from a panel, nor will breakpoints occur
for selections from a menu (selection) panel. Breakpoints occur only for dialog service calls that use the
ISPLINK, ISPLNK, or ISPEXEC interfaces.
Breakpoints (option 7.8)
Chapter 9. Dialog test (option 7)  391

## Page 430

Along with several menu bar items common across ISPF Version 4.1, the Breakpoints panel has added
the Qualify pull-down. You can now display the qualification parameter values from the Breakpoints panel
in two ways:
• Enter the QUAL primary command
• Select the Qualifications choice from the Qualify pull-down.
The Function and Active columns are overlaid with a column of data titled Qualification Parameter Values;
this column was logically off the screen to the right of the first Breakpoints panel. To resume the format of
the Breakpoints panel, you can either:
• Enter the RESUME primary command, or
• Select the Breakpoints choice from the Qualify pull-down
Specifying breakpoints
When you select the Breakpoints option, you are shown a display that you can scroll (Figure 236 on
page 392). The pop-up window can also be resized using the RESIZE command. The display lists all
currently defined breakpoints for this session. You can use this panel to add, delete, or change breakpoint
definitions, either before calling a function or at a breakpoint.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Qualify  Utilities  Help                                            │
 │ ─────────────────────────────────────────────────────────────────────────── │
 │                                 Breakpoints                Row 1 to 5 of 13 │
 │                                                                             │
 │ Add, delete, and change breakpoints. Underscores need not be blanked.       │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │         Service            When            Function        Active           │
 │        (Required)   (BEFORE,AFTER,Rnn)  (No entry=all)    (YES,NO)          │
 │                       (No entry=all)                   (No entry=YES)       │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ Command ===>                                              Scroll ===> PAGE  │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 236. Breakpoints panel (ISPYBP1)
Breakpoints panel action bar
The Breakpoints panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Qualify
Displays the Qualification parameter values field on the Breakpoints panel so that you can further
constrain the conditions under which a breakpoint is to occur.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
Breakpoints (option 7.8)
392  z/OS: z/OS ISPF User's Guide Vol II

## Page 431

1
General
2
Definitions
3
Breakpoints Panel
4
Qualification Panel
5
Line Commands
6
Primary Commands
7
Usage Notes
8
General Dialog Test
Breakpoints panel fields
Each line defines a breakpoint and includes a line command area and these fields:
Service
Name of the dialog service at which to interrupt dialog processing. This field is required.
When
Indication of when the breakpoint should occur:
BEFORE
Before the service receives control.
AFTER
After the service finishes processing.
Rnn
After the service finishes processing, but only if the return code is the integer nn.
Blank
Before and after service processing.
Function
The program function or command function that must be processing for the breakpoint to be taken.
No entry in this field shows that the breakpoint can occur for all functions.
Active
Indication of whether the breakpoint is to be active now:
YES
It is currently active.
NO
It is currently not active.
Blank
It is currently active.
*QUAL*
If present at the end of a row, shows that qualification data exists for the breakpoint. This field is
non-modifiable. See “Qualification parameter values” on page 394 for additional information.
All input fields contain underscores. Empty lines are added to the first display to fill up the screen. If
you delete all the lines used for defining breakpoints, the display is automatically refreshed with enough
empty lines to fill the screen again.
Breakpoints (option 7.8)
Chapter 9. Dialog test (option 7)  393

## Page 432

All breakpoints exist until you end or cancel your Dialog Test session, or until you delete them from this
panel. Enter new information by typing over the existing data. The underscores are pad characters to
show the starting and ending positions for each field; you do not need to blank them out. You can create
several breakpoints before you press the Enter key.
Breakpoints commands
From the Breakpoints panel, you can use the CANCEL, END, LOCATE, QUAL, and RESUME commands, and
the D (delete), I (insert), and R (repeat) Dialog Test line commands described in “Commands” on page
358.
Qualification parameter values
A different part of the Breakpoints panel allows you to further constrain the conditions under which a
breakpoint is to occur by entering qualification parameter values. On this part of the panel, you can list
parameter data with which the named service must have been called.
The Breakpoints panel with the Qualification parameter values field is displayed (Figure 237 on page 394)
if you enter the QUAL primary command on the first part of the Breakpoints panel or if you select the
Qualifications choice from the Qualify pull-down. The Function and Active columns are overlaid with a
column of data titled Qualification parameter values; this column was logically off the screen to the right
of the first Breakpoints panel. To resume the format of the Breakpoints panel, use the RESUME primary
command or select the Breakpoints choice from the Qualify pull-down.
 ┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Qualify  Utilities  Help                                            │
 │ ─────────────────────────────────────────────────────────────────────────── │
 │                                 Breakpoints                Row 1 to 5 of 13 │
 │                                                                             │
 │ Add, delete, and change breakpoints. Underscores need not be blanked.       │
 │ Enter END command to finalize changes.                                      │
 │                                                                             │
 │       Service          When                 Qualification parameter values  │
 │      (Required)  (BEFORE,AFTER,Rnn)                  (No entry=none)        │
 │                    (No entry=all)        ("AND" is assumed between values)  │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │ Command ===>                                              Scroll ===> PAGE  │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F5=Rfind       │
 │  F6=Resize      F7=Backward    F8=Forward     F9=Swap       F10=Actions     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 237. Breakpoints panel with qualific ation  parameter values (ISPYBP2)
The lines on the Breakpoints panel with qualification parameter values correspond to the lines on the first
Breakpoints panel; “Specifying breakpoints” on page 392 describes the Service and When fields. In the
Qualification parameter values field, for all services except SELECT, you can enter any combination of:
• One or more parameter values, separated by blanks, that the dialog passes to the service. No order is
implied by the specification of the parameter values.
For example, if you want a breakpoint to occur when message ABC0001 is included on a DISPLAY
service request, specify ABC0001. If the breakpoint should occur only when message ABC0001 and
panel XYZ are both included, specify ABC0001 XYZ.
• One or more command call keywords, separated by blanks, that have values that are not blank when
a dialog calls the service. For ISPLINK or ISPLNK calls, the keywords matching the calling sequence
parameter positions are used.
Breakpoints (option 7.8)
394  z/OS: z/OS ISPF User's Guide Vol II

## Page 433

For example, if you want a breakpoint to occur whenever the DISPLAY service is called with a message,
then specify MSG.
For ISPF's SELECT and ISREDIT services, you can enter one or more parameter strings that would be
entered on these two service calls. A parameter string is a series of characters delimited by a blank, a
comma, a single quotation mark, or a left or right parenthesis.
For example, if a SELECT call is:
SELECT PGM(ABC) PARM(1 2 3 5 '6'),
then all or any of these strings can be used: SELECT, PGM, ABC, 1, 2, 3, 5, 6.
For a breakpoint to be taken, all qualification data listed must be matched.
All line commands and change capabilities are still available on the Breakpoints panel with qualification
parameter values.
During dialog processing, to determine whether the criteria for a breakpoint have been met, Dialog Test
processes a logical AND of the Service, When, Function, Active, and Qualification fields specified for
that breakpoint. Therefore, if you want more than one breakpoint for an ISPF service, you should create
multiple rows.
When you use the Breakpoints option (7.8), be aware of these items:
Qualification
If you plan to qualify several breakpoints, it can be more efficient to specify all breakpoint data on the
Breakpoints panel with qualification parameter values.
END command
You can use the END primary command from either the first Breakpoints panel or the Breakpoints
panel with qualification parameter values.
Input errors
You must correct input errors before leaving any display using the END, QUAL, or RESUME command.
You can use the CANCEL command to end the Breakpoints option, even if input errors remain on the
display.
Syntax checking
A dialog service call must pass a basic syntax check before a breakpoint is honored.
Control display
If any CONTROL service settings for DISPLAY LINE or DISPLAY SM (Session Manager) were in effect
before the breakpoint, such settings are lost.
Finding a breakpoint
If you call a dialog function or selection panel and find a breakpoint, the Breakpoint Primary Option
Panel is displayed. Figure 238 on page 396 shows this selection panel at a breakpoint just after the ISPF
DISPLAY service was called while processing the TEST function in application PAY.
Breakpoints (option 7.8)
Chapter 9. Dialog test (option 7)  395

## Page 434

┌──────────────────────────────── Dialog Test ────────────────────────────────┐
 │   Menu  Utilities  Help                                                     │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │             Breakpoint Primary Option Panel - BEFORE VDEFINE   End of field │
 │                                                                             │
 │ 1 Functions       Invoke dialog functions/selection panel                   │
 │ 2 Panels          Display panels                                            │
 │ 3 Variables       Display/set variable information                          │
 │ 4 Tables          Display/modify table information                          │
 │ 5 Log             Browse ISPF log                                           │
 │ 6 Dialog Services Invoke dialog services                                    │
 │ 7 Traces          Specify trace definitions                                 │
 │ 8 Breakpoints     Specify breakpoint definitions                            │
 │ T Tutorial        Display information about Dialog Test                     │
 │ G Go              Continue execution from breakpoint                        │
 │ C Cancel          Cancel dialog testing                                     │
 │                                                                             │
 │ Current status:                                                             │
 │ Application . : PAY      Function . : TEST      Return Code . . 8           │
 │ Breakpoint:                                                                 │
 │ FVR96 ISPFVR97 ISPFVR98 ISPFVR99 ISPFVR00,X'C1C2C3C4',X'C3C8C1D9',4,LIST )  │
 │ <                                                                           │
 │ Option ===>                                              Scroll ===> PAGE   │
 │  F1=HELP        F2=            F3=END         F4=DATASETS    F5=FIND        │
 │  F6=CHANGE      F9=SWAP       F10=LEFT       F11=RIGHT      F12=SUBMIT      │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 238. Breakpoint Primary Option panel (ISPYXM1)
Like the Dialog Test Primary Option Panel, the Breakpoint Primary Option Panel allows you to use the
RETURN command from any one of the selected test options to display the Breakpoint Primary Option
Panel again. At the Breakpoint Primary Option Panel, the END and RETURN commands have no effect. You
must use the Go option (G) to end processing at this breakpoint and continue processing the dialog being
tested, or the Cancel option (C) to cancel the Dialog Test option (7). This protects against inadvertent loss
of data.
The Breakpoint Primary Option Panel contains all the options of the Dialog Test Primary Option Panel
except Exit (7.x) and, as such, presents all but one of the Dialog Test functions to you.
This panel also contains two options not shown on the Dialog Test Primary Option Panel: Go (G) and
Cancel (C). When a breakpoint occurs, these options allow you to continue processing or stop processing,
respectively:
G
The Go option continues dialog processing from a breakpoint. The user dialog resumes processing
from the point at which it was suspended.
C
The Cancel option ends dialog testing and displays the first primary option panel you displayed at
the beginning of your ISPF session again. All trace and breakpoint definitions are lost when you leave
Dialog Test.
When a user dialog finds a breakpoint, the current dialog environment is saved. When you select the Go
option, the environment is restored, except that:
• If you change variable, table, and file tailoring data at a breakpoint, these actions are an extension of the
suspended dialog; it is as though the dialog had taken all the actions itself during processing.
• If you change the service return code on the Breakpoint Primary Option Panel, the new return code is
passed back to the dialog as though the service had set the new return code itself.
• If you process the PANELID command at the breakpoint, the last setting for displaying panel identifiers
is retained.
• If any CONTROL service settings for DISPLAY LINE or DISPLAY SM (Session Manager) were in effect
before the breakpoint, such settings are lost.
Note that the manipulation of one dialog part can cause a change to another dialog part. For example, if a
panel is displayed, variables can be set.
All trace and breakpoint definitions are lost if you select the Cancel option.
Breakpoints (option 7.8)
396  z/OS: z/OS ISPF User's Guide Vol II

## Page 435

The Breakpoint Primary Option Panel also displays this information:
AFTER or BEFORE
An indication of whether the dialog has been suspended after or before the service has processed.
Service Name
The name of the service at which the dialog has been suspended. In Figure 238 on page 396, the
service name is DISPLAY.
Current status:
The application's current status when the breakpoint occurred. These fields show this status:
Application
The application identifier of the suspended user dialog.
Function
The program or command name of the suspended user dialog.
Return code
The dialog service return code. This field is displayed only if the breakpoint occurs after the dialog
service has processed. The Return code field is modifiable; its value is passed back to the dialog
(as the service's) when you select the Go option. This helps test dialog error handling.
Breakpoint
One scrollable line showing an image of the dialog service call. Place the cursor over the image
and use LEFT, RIGHT, and EXPAND functions to scroll the area. < and > appear below the line to
indicate in which direction more data may be available. A maximum of 2048 characters may be
displayed.
ISPEXEC calls are shown as typed.
ISPLINK (ISPLNK) calls are displayed with their parameter values separated by commas. Name-
lists are shown as typed in the dialog, in string format or in structure format. Structure format
includes the count, element length, and list of names. For variable services parameters whose
context is defined by the name-list parameter on the service call (for example, the variable value
areas for a VDEFINE), the first four bytes of the parameter value are displayed in hexadecimal
format (X'nnnnnnnn').
ISPEXEC calls from a program are the same as ISPEXEC calls from a command except that
ISPEXEC is not displayed.
Tutorial (option 7.T)
The Tutorial option (7.T) allows you to display information about the Dialog Test facilities. Figure 239 on
page 398 shows the first panel displayed when you select the Tutorial option.
Tutorial (option 7.T)
Chapter 9. Dialog test (option 7)  397

## Page 436

Tutorial ------------------- Dialog Test Tutorial ------------------- Tutorial
                          ────────────────────────────
                          │     ISPF Dialog Test     │
                          ────────────────────────────
 This tutorial provides information about the features and operation of Dialog
 Test.
 The Dialog Test tutorial consists of two parts: one describes the Dialog Test
 option, as selected from the ISPF Primary Option Panel, and the other
 describes the Dialog Test facilities available when a user dialog encounters a
 "breakpoint" in its processing.
 Beginning users should review the Dialog Test Option topic first.
 The following topics are presented in sequence, or can be selected by number:
    1  - Dialog Test Option
    2  - At A User Dialog Breakpoint
 ------ Cur panel = ISP70000 Prev panel = ISPYXD1  Last msg = ISPYP014   ------
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F4=Resize    F5=Exhelp    F6=Keyshelp
  F7=PrvTopic  F8=NxtTopic  F9=Swap     F10=PrvPage  F11=NxtPage  F12=Cancel
Figure 239. Dialog Test Tutorial - firs t  panel (ISP70000)
The default function key command assignments for a terminal with 12 function keys are shown at the
bottom of the screen if you enter the PFSHOW command.
Exit (option 7.X)
The Exit option (7.X) ends your Dialog Test session. All trace and breakpoint definitions are lost.
Exit (option 7.X)
398  z/OS: z/OS ISPF User's Guide Vol II
