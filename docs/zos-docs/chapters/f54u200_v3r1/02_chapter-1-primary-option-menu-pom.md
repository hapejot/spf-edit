# Chapter 1. Primary Option Menu (POM)

Source file: f54u200_v3r1.md
Start page: 39
Page span: 39-64

## Page 39

Chapter 1. Primary Option Menu (POM)
See:
• “The Primary Option Menu panel” on page 1
• “Status area on the Primary Option Menu” on page 5
The Primary Option Menu panel
The Primary Option Menu panel, shown in Figure 2 on page 1, is the first panel that displays when you
start ISPF.
 1 
Primary Options.
 2 
Action bar choices.
 3 
Dynamic status area.
Figure 2. ISPF Primary Option Menu (ISR@PRIM)
ISPF primary options
When you select one of these options, ISPF displays the selected panel. These options are described in
detail in other chapters within this book. Brief descriptions follow:
Option
Description
The Primary Option Menu panel
© Copyright IBM Corp. 1980, 2024 1

## Page 40

0
Settings displays and changes selected ISPF parameters, such as terminal characteristics and
function keys. See Chapter 2, “Settings (option 0),” on page 27 for more information.
 1
View displays data (you cannot change it) using the View or Browse function. Use View or Browse to
look at large data sets, such as compiler listings. You can scroll the data up, down, left, or right. If you
are using Browse, a FIND command, entered on the command line, allows you to search the data and
find a character string. If you are using View, you can use all the commands and macros available to
you in the Edit function. See Chapter 3, “View (option 1),” on page 65 for more information.
 2
You can use Edit to create or change source data, such as program code and documentation, using
the ISPF full-screen editor. You can scroll the data up, down, left, or right. You can change the data by
using Edit line commands, which are entered directly on a line number, and primary commands, which
are entered on the command line. See Chapter 3, “View (option 1),” on page 65 and refer to z/OS
ISPF Edit and Edit Macros for more information.
 3
Utilities perform library and data set maintenance tasks, such as moving or copying library or data
set members, displaying or printing data set names and volume table of contents (VTOC) information,
comparing data sets, and searching for strings of data. See Chapter 5, “Utilities (option 3),” on page
89 for more information.
 4
Foreground calls IBM language processing programs in the foreground. See Chapter 6, “Foreground
(option 4),” on page 309 for more information.
 5
Batch calls IBM language processing programs as batch jobs. ISPF generates Job Control Language
(JCL) based on information you enter and submits the job for processing. See Chapter 7, “Batch
(option 5),” on page 339 for more information.
 6
Command calls TSO commands, CLISTs, or REXX EXECs under ISPF. See Chapter 8, “Command
(option 6),” on page 351 for more information.
 7
Dialog Test tests individual ISPF dialog components, such as panels, messages, and dialog functions
(programs, commands, menus). See Chapter 9, “Dialog test (option 7),” on page 355 for more
information.
 9
You can use the IBM Products option to select other installed IBM program development products on
your system. Products supported are:
• Tivoli® Information Management (INFOMAN)
• COBOL Structuring Facility (COBOL/SF)
• Screen Definition Facility II (SDF II and SDF II-P)
See Chapter 10, “IBM products (option 9),” on page 399 for more information.
10
SCLM controls, maintains, and tracks all of the software components of an application. See Chapter
11, “SCLM (option 10),” on page 401 and refer to z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference for more information.
11
Workplace gives you access to the ISPF Workplace, which combines many of the ISPF functions onto
one object-action panel. See Chapter 12, “ISPF object/action workplace (option 11),” on page 403 for
more information.
The Primary Option Menu panel
2  z/OS: z/OS ISPF User's Guide Vol II

## Page 41

12
z/OS System gives you access to the z/OS System Programmer Primary Option Menu. It contains
options for z/OS elements that are used by system programmers and administrators. It includes
options for:
• GDDM Print Queue Manager
• HCD I/O configuration
• APPC Administration
• WLM Work Load Manager
• FFST dump formatting
• Infoprint Server
• RMF
• SMP/E
• TCP/IP NPF
13
z/OS User gives you access to the z/OS Applications panel. It contains options for z/OS elements that
are used by most ISPF users. It includes options for:
• DFSMSrmm/ISMF
• DFSMSdfp/ISMF
• IPCS
• z/OS UNIX Browse
• z/OS UNIX Edit
• z/OS UNIX Shell
• Security Server
• TSO/E Information Center Facility
• SDSF
X
EXIT leaves ISPF using the log and list defaults. You can change these defaults from the Log/List
pull-down on the ISPF Settings panel action bar.
Primary Option Menu action bar choices
The Primary Option Menu action bar offers a quick way of accessing many of the panels within ISPF.
Menu
This choice is available from most panels within ISPF and displays many of the options listed on the
Primary Option Menu panel. These choices are available from the Menu pull-down:
Settings
Displays the ISPF Settings panel.
View
Displays the View Entry panel.
Edit
Displays the Edit Entry panel.
ISPF Command Shell
Displays the ISPF Command Shell panel.
Dialog Test
Displays the Dialog Test Primary Option panel.
Other IBM Products
Displays the Additional IBM Program Development Products panel.
The Primary Option Menu panel
Chapter 1. Primary Option Menu (POM)  3

## Page 42

SCLM
Displays the SCLM Main Menu.
ISPF Workplace
Displays the Workplace entry panel.
Status Area
Displays the ISPF Status panel.
Exit
Exits ISPF.
Utilities
This choice is available from many panels within ISPF and displays the options listed on the Utility
Selection panel. These choices are available from the Utilities pull-down:
Library
Displays the Library Utility panel.
Data Set
Displays the Data Set Utility panel.
Move/Copy
Displays the Move/Copy Utility panel.
Data Set List
Displays the Data Set List Options panel.
Reset Statistics
Displays the Reset ISPF Statistics panel.
Hardcopy
Displays the Hardcopy Utility panel.
Outlist
Displays the Outlist Utility panel.
Commands
Displays the Command Table Utility panel.
Reserved
Reserved for future use by ISPF; an unavailable choice.
Format
Displays the Format Specification panel.
SuperC
Displays the SuperC Utility panel.
SuperCE
Displays the SuperCE Utility panel.
Search-for
Displays the Search-For Utility panel.
Search-forE
Displays the Search-ForE Utility panel.
Table Utility
Displays the ISPF Table Utility panel.
Directory List
Displays the z/OS UNIX Directory List Utility panel.
Compilers
The Compilers pull-down offers these choices:
Foreground Compilers
Displays the Foreground Selection Panel.
Background Compilers
Displays the Batch Selection Panel.
The Primary Option Menu panel
4  z/OS: z/OS ISPF User's Guide Vol II

## Page 43

ISPPREP Panel Utility
Displays the Preprocessed Panel Utility panel.
DTL Compiler
Displays the ISPF Dialog Tag Language Conversion Utility panel.
Options
The Options pull-down offers these choices:
General Settings
Displays the ISPF Settings panel.
CUA Attributes
Displays the CUA Attribute Change Utility panel.
Keylists
Displays the Keylist Utility panel.
Point-and-Shoot
Displays the CUA Attribute Change Utility panel, positioned on the Point-and-Shoot panel
element.
Colors
Displays the Global Color Change Utility panel.
Dialog Test appl ID
Displays the Dialog Test Application ID pop-up to allow you to change the application ID for Dialog
test so that you can look at variables in the application profile for an application that runs under a
different application ID than the one under which ISPF was started (by default, ISR).
Status
The Status pull-down offers these choices:
• Session
• Function keys
• Calendar
• User status
• User point and shoot
• None
See “Status area on the Primary Option Menu” on page 5 for more information about using these
choices to tailor the status area.
Help
The Help pull-down provides general information about ISPF topics and the changes in the current
release, as well as information about each of the options and areas on the Primary Option Menu.
Status area on the Primary Option Menu
The status area on the ISPF Primary Option Menu is a 21-column dynamic area that is composed of
a 12-character description field, one attribute byte, and an 8-character field to display the value of
the selected variable. The status area is limited to eleven description fields and their values. It can be
manipulated from two places:
• The Status choice on the ISPF Primary Option Menu action bar. Use this pull-down to specify what you
want to display in the status area. See “Status pull-down” on page 6 for additional information and
examples.
• The ISPF Status panel, which displays when you select Status Area from the Menu pull-down available
on most action bars throughout ISPF. Use this facility to define the contents of the status area. See
“Defining the status area” on page 15 for additional information and examples.
Note: The ISPF Status panel also contains an action bar choice called "Status". This does not affect
which Status option displays on the Primary Option Menu panel. It determines which Status option
displays within the ISPF Status panel.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  5

## Page 44

The first five logical screens, created by a SPLIT or related command, each have their own status view. For
each screen after that, the view defaults to the setting of the first screen.
Status pull-down
When you select one of the choices in the Status pull-down on the ISPF Primary Option Menu action bar
(shown in Figure 3 on page 6), you specify what you want to display in the status area on the Primary
Option Menu panel.
Figure 3. Status pull-down on the ISPF Primary Option Menu (ISR@PRIM)
Note: The current setting is shown as an unavailable choice; that is, displays in blue (the default) with an
asterisk as the first digit of the selection number.
Session
The Session view, shown in Figure 4 on page 7, displays this information in the status area:
• User ID
• Time
• Terminal
• Screen
• Language
• Application ID
• TSO logon
• TSO prefix
• System ID
• MVS account
• Release.
Status area on the Primary Option Menu
6  z/OS: z/OS ISPF User's Guide Vol II

## Page 45

Figure 4. ISPF Primary Option Menu status area – session view
System ID is a point-and-shoot field. MVS Acct and Release are point-and-shoot fields if over 8
characters long. Select these fields to display pop-up windows that contain additional information about
the MVS account number and the ISPF environment.
MVS Acct
The account number identifying this MVS user.
System ID
Shows the SYSPLEX and SYSNODE.
SYSPLEX
The MVS sysplex name as found in the COUPLExx or LOADxx member of SYS1.PARMLIB.
SYSNODE
The network node name of your installation's JES.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  7

## Page 46

Figure 5. System information pop-up
Release
Displays these variables:
• ZOS390RL— The z/OS Release running on your system.
• ZISPFOS— The level of ISPF code that is running as part of z/OS on your system. This might or
might not match ZOS390RL.
• ZENVIR— The ISPF Environment description. See the table of system variables in the z/OS ISPF
Dialog Developer's Guide and Reference for a complete explanation.
Status area on the Primary Option Menu
8  z/OS: z/OS ISPF User's Guide Vol II

## Page 47

Figure 6. Environment pop-up (release information)
Function keys
The Function Keys view, shown in Figure 7 on page 10, displays this information in the status area:
• Number of keys
• Keys displayed per line
• Primary range (lower or upper)
• Display set (primary or alternate)
• List name (name of the currently active keylist)
• List applid (application ID for the currently active keylist)
• List type (private or shared)
• Keylists (on or off).
Note: See “Working with function keys and keylists (the Function Keys action bar choice)” on page 41 for
information about changing these settings.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  9

## Page 48

Figure 7. ISPF Primary Option Menu status area – function keys view
Calendar
The Calendar view, shown in Figure 8 on page 11, displays the calendar for the current month in the
status area.
Status area on the Primary Option Menu
10  z/OS: z/OS ISPF User's Guide Vol II

## Page 49

Figure 8. ISPF Primary Option Menu status area – calendar view
All of the fields on the calendar are point-and-shoot fields that function as follows:
If you select
ISPF displays
<
the previous month.
calendar
the current month.
>
the next month.
Month, e.g. July
the Calendar Month pop-up. Allows you to specify the month. See “Customizing the calendar” on page
18 for details.
Year, e.g. 2003
the Calendar Year pop-up. Allows you to specify the year. See “Customizing the calendar” on page
18 for details.
Day
the Calendar Start Day pop-up. Allows you to specify Saturday, Sunday, or Monday as the start day for
the calendar. See “Customizing the calendar” on page 18 for details.
Date
the Julian Date pop-up. Provides the Julian date for the date selected. 
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  11

## Page 50

Figure 9. Julian date pop-up
Time
the Calendar Time Format pop-up. Allows you to specify a 12-hour or 24-hour time format for the
calendar. See “Customizing the calendar” on page 18 for details.
Day of year
the Standard Date pop-up. Provides the standard date for the day specified in the popup (defaults to
the date selected in the calendar). 
Status area on the Primary Option Menu
12  z/OS: z/OS ISPF User's Guide Vol II

## Page 51

Figure 10. Standard date pop-up
User status
The User Status view, shown in Figure 11 on page 14, displays the status information that you have
defined in the Status Area panel.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  13

## Page 52

Figure 11. ISPF Primary Option Menu status area – user status view
User point and shoot
The User Point-and-Shoot view, shown in Figure 12 on page 15, displays the point-and-shoot function
you have defined in the Status Area panel.
Status area on the Primary Option Menu
14  z/OS: z/OS ISPF User's Guide Vol II

## Page 53

Figure 12. ISPF Primary Option Menu status area – user point-and-shoot view
None
If you select None from the Status pull-down, nothing will be displayed in the status area.
Defining the status area
When you select Status Area from the Menu pull-down, ISPF displays the ISPF Status pop-up window
(shown in Figure 13 on page 16), with an independent view of the status area. This panel is used to
define the contents of the status area choices. You can change the choice displayed in this window by
using the Status pull-down on the action bar.
Note: Changing the status area viewed in this panel will not affect the choice selected on the ISPF Primary
Option Menu panel.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  15

## Page 54

Figure 13. ISPF status pop-up (ISPSAMMN)
Status
The Status pull-down, shown in Figure 14 on page 17, offers these choices:
1
Session
2
Function keys
3
Calendar
4
User status
5
User point and shoot
6
None
Status area on the Primary Option Menu
16  z/OS: z/OS ISPF User's Guide Vol II

## Page 55

Figure 14. Status pull-down in ISPF status pop-up
Options
The Options pull-down, shown in Figure 15 on page 18, offers these choices:
1
Calendar start day Displays the Calendar Start Day pop-up, where you can specify Saturday,
Sunday, or Monday as the start day for the calendar.
2
Calendar colors Displays the Calendar Colors pop-up, where you can change the colors on the
calendar.
3
User status customization Displays the User View Customization pop-up, where you can define
what you want displayed in the status area.
4
User point and shoot customization Displays the User Point and Shoot Customization pop-up,
where you can define point-and-shoot fields to be displayed in the status area.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  17

## Page 56

Figure 15. Options pull-down in ISPF status pop-up
Customizing the calendar
You can customize the calendar to show a different month, year or starting day. You can set the time
display to a 12 or 24 hour clock. You can also customize the colors used to display the calendar.
Note:
1. Changes to the month or year display will last for the current session only. Changes to the starting day
and time format will be saved between sessions.
2. You can use the point-and-shoot fields on the calendar displayed on the Primary Option Menu panel or
in the Status Area pop-up. Changes made in either location will affect the display in both locations.
Changing the month display
You can change the month that displays in the calendar in a number of ways:
• Click the < or > symbols to display the previous or next month.
• Click the Calendar point-and-shoot field to display the current month.
• Set the month display to a particular month by selecting the month name point-and-shoot field and
entering a number in the Calendar Month pop-up window: 
Status area on the Primary Option Menu
18  z/OS: z/OS ISPF User's Guide Vol II

## Page 57

Figure 16. Calendar month pop-up window
Changing the year display
You can change the year that displays in the calendar by selecting the year point-and-shoot field and
entering the required year (between 1801 and 2099) in the Calendar Year pop-up window.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  19

## Page 58

Figure 17. Calendar year pop-up window
Changing the starting day
You can change the calendar display so that the weeks begin on a Saturday, Sunday, or Monday.
1. To display the Calendar Start Day pop-up window, use either of these methods:
• Select any day name point-and-shoot field (e.g. Mo or Tu).
• From the Menu action bar, select Status Area. Then, from the Options action bar, choose 1. Calendar
Start day.
2. Enter option 1. Sunday, 2. Monday or 3. Saturday. 
Status area on the Primary Option Menu
20  z/OS: z/OS ISPF User's Guide Vol II

## Page 59

Figure 18. Calendar start pop-up window
Changing the time format
You can change the time format to a 12-hour or 24-hour clock. To do this, select the Time point-and-
shoot field and enter option 1 or 2 in the Calendar Time Format pop-up window.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  21

## Page 60

Figure 19. Calendar time format pop-up window
Changing the calendar colors
To change the colors on the calendar:
1. From the ISPF Status action bar, select Options and then 2. Calendar colors. The Calendar Colors
pop-up, Figure 20 on page 23, is displayed.
2. Type a valid color number in the field next to each calendar element to be changed and press Enter.
The color will change immediately in the Sample area.
To restore a default color, clear the element field and press Enter.
3. Press EXIT (F3) or END to exit and save the changes. Press CANCEL (F12) to exit without saving the
changes.
Status area on the Primary Option Menu
22  z/OS: z/OS ISPF User's Guide Vol II

## Page 61

Figure 20. Calendar colors panel (ISPCALGC)
Customizing the user status area
To define the contents of this area:
1. From the ISPF Status action bar, select Options and then 3. User status customization. The User View
Customization panel, Figure 21 on page 24, is displayed. 
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  23

## Page 62

Figure 21. User view customization panel (ISPSAMUS)
2. Enter a heading in the Description field and the name of a System variable or site-defined variable in
the Variable name field. See the z/OS ISPF Dialog Developer's Guide and Reference for a list of System
Variable names.
3. Press EXIT (F3) or END to exit and save the changes. Press CANCEL (F12) to exit without saving the
changes.
For example, entering:
Description:        Variable name:
User ID:            ZUSER
Date:               ZDATE
Time:               ZTIME
would result in the display shown in Figure 11 on page 14.
Customizing the user point-and-shoot status area
You can define up to nine point-and-shoot fields, which you can set to any SELECT service parameter. To
do this:
1. From the ISPF Status action bar, select Options and then 4. User point and shoot customization. The
User Point-and-Shoot panel, Figure 22 on page 25, is displayed. 
Status area on the Primary Option Menu
24  z/OS: z/OS ISPF User's Guide Vol II

## Page 63

Menu  Utilities  Compilers  Options  Status  Help
 ─ ┌ ┌───────────────────────── User Point-and-Shoot ──────────────────────────┐
   │ │                                                                         │
   │ │                                                                         │
 0 │ │ Press EXIT or END to exit and save the changes.                         │
 1 │ │ Press CANCEL to exit without saving the changes.                        │
 2 │ │                                                                         │
 3 │ │ Enter Point-and-shoot text and SELECT keywords.                         │
 4 │ │                                                                         │
 5 │ │ Point-and-shoot text:  SELECT service parameters:                       │
 6 │ │                                                            More:     +  │
 7 │ │ ___________________  ________________________________________________   │
 9 │ │                      ________________________________________________   │
 1 │ │                      ________________________________________________   │
 1 │ │                      ________________________________________________   │
   │ │                      ________________________________________________   │
   │ │ ___________________  ________________________________________________   │
   │ │                      ________________________________________________   │
   │ │                      ________________________________________________   │
   │ │                      ________________________________________________   │
   │ │                      ________________________________________________   │
 O │ │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward     │
   ⋘ e  F9=Swap      F12=Cancel                                                │
 F10 ⋘─────────────────────────────────────────────────────────────────────────┘
Figure 22. User point-and-shoot panel (ISPSAMUP)
2. Enter the text to appear in the status area, in the field on the left of the panel.
3. Enter the Service parameters to be invoked in the lines on the right of the panel. See z/OS ISPF
Services Guide for information about these parameters.
4. Press EXIT (F3) or END to exit and save the changes. Press CANCEL (F12) to exit without saving the
changes.
Status area on the Primary Option Menu
Chapter 1. Primary Option Menu (POM)  25

## Page 64

Status area on the Primary Option Menu
26  z/OS: z/OS ISPF User's Guide Vol II
