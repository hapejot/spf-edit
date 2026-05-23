# Chapter 1. Overview of ISPF

Source file: f54ug00_v3r1.md
Start page: 29
Page span: 29-42

## Page 29

Chapter 1. Overview of ISPF
ISPF is a multifaceted development tool set for the z/OS operating system. Since 1975, MVS
programmers have used ISPF for host-based application development productivity. ISPF forms the basis
of many TSO applications and provides extensive programmer-oriented facilities as well.
ISPF components
ISPF helps programmers develop interactive applications called dialogs. Dialogs are interactive because
ISPF uses them to communicate with terminal users through a series of panels while the users do
application development tasks.
ISPF panels:
• Provide access to ISPF functions through menus
• Request information from users through data entry panels
• Provide information from users through scrollable data displays
These are the main components of ISPF:
• Dialog Manager (DM): The Dialog Manager provides services to dialogs and end users. These include
display, variable services, input and output, user and application profiles, table management, system
interface services, dialog testing and debugging aids, and other services.
• Program Development Facility (PDF): The Program Development Facility provides services to assist the
dialog or application developer. These include the edit and browse functions, data set and catalog
utilities, TSO command interfaces, and data set search and compare functions.
• Software Configuration and Library Manager (SCLM): The SCLM facility provides library management
capabilities such as versioning, auditing, and promotion. It also provides configuration management
capabilities to track how all of the pieces of an application fit together, including source code, objects,
load modules, test cases, documentation, and other items. The Build function tracks and invokes the
necessary compilers, assemblers and linkage editors.
On May 15, 2018, IBM issued a statement of direction that the Software Configuration and Library
Management (SCLM) component of ISPF is functionally stabilized. While it will continue to be maintained
and supported, it won't be enhanced with new features in the future.
ISPF functions
ISPF can be used in these ways:
• Data processing administrators and system programmers can use ISPF to:
– Monitor and control program libraries
– Communicate with MVS through TSO commands, CLISTs, or REXX EXECs.
• Programmers can use ISPF to develop a batch, interactive, or any other type of program and its
documentation.
• Terminal users can call dialogs that use Dialog Manager (DM) component and Program Development
Facility (PDF) component dialog services to do the work of the application.
The View, Browse, and Edit functions, a wide range of utilities, foreground and batch compilers, program
library control, and other facilities are available to help you develop ISPF dialogs.
ISPF components
© Copyright IBM Corp. 1980, 2024 1

## Page 30

View, Browse, Edit, edit macros, and models
The View, Browse, and Edit functions allow you to look at the contents of a dialog. While editing a dialog,
you can change it by adding or deleting lines, typing over the existing source code, or copying lines from
another dialog to the one being edited.
To enhance the existing Edit function, you can write edit macros. Edit macros allow you to combine
several often-used functions so that you do not have to call each function separately. You can write
initial edit macros that are automatically run when the Edit option is selected. Other uses for edit macros
include:
• Overriding Edit commands
• Calling DM and PDF component dialog services
• Accessing cursor position and data location.
Also, ISPF provides online models that you can insert into the dialog. A model is an example of a
service call, panel format, table format, or message that contains the proper syntax and all the available
parameters for the programming language being used. Since these models are online, they can be called
directly into the member being edited.
See z/OS ISPF Edit and Edit Macros for more information.
Dialog services
The PDF component provides View, Browse, Edit, and library access services that can be combined in a
dialog with any of the ISPF services. The library access services carry out functions involving members
of a programming library. These functions include adding, finding, and deleting members, and displaying
member lists.
The PDF component includes a separate edit model of each service call for each programming language
ISPF supports: CLIST, COBOL, EXEC, FORTRAN, PL/I, Pascal, C, and REXX. See z/OS ISPF Services Guide
for complete information about the PDF component dialog services.
Note: For information about library access services that apply to the Software Configuration and Library
Manager (SCLM), refer to the z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference.
Utilities
ISPF provides a wide range of utilities. Utilities enable you to:
• Display and print library and data set member lists
• Reset statistics for ISPF library members
• Define commands to be used with specific dialogs
• Compare data sets and search for strings of data
• Move, copy, and print library and data set members.
Dialog Test, Foreground, and Batch
ISPF provides special facilities for testing dialogs. The Dialog Test option allows testing of individual
dialog elements and complete dialogs.
After a program has been developed, you can either assemble it or compile and link it using either the
Foreground or Batch option. The Foreground option allows you to watch the program being compiled. The
Batch option frees the terminal, which helps when you compile a long program. If errors occur, you can
use the debugging capabilities of the Dialog Test facility to correct them.
TSO commands, CLISTs, and REXX EXECs
While using ISPF, you can call TSO commands, CLISTs, or REXX EXECs by:
• Prefixing a command with TSO and entering it from any command line
ISPF functions
2  z/OS: z/OS ISPF User's Guide Vol I

## Page 31

• Selecting the Command option and entering the TSO command, CLIST, or REXX exec on the panel
• Entering a TSO command, CLIST, or REXX exec in the Line Command field on a data set list display or a
member list display.
Software Configuration and Library Manager (SCLM)
To help you maintain different levels or versions of a library member, ISPF includes the Software
Configuration and Library Manager (SCLM). SCLM is a tool that helps you develop complex software
applications. Throughout the development cycle, SCLM automatically controls, maintains, and tracks all of
the software components of the application. And you can lock the version being edited in a private library
and then promote it. See z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference for
more information.
Other IBM program development products
You might want to call another IBM program development product while using ISPF. The IBM Products
option allows you to call the Tivoli® Information Management, COBOL Structuring Facility, and Screen
Definition Facility licensed programs without leaving ISPF.
Online tutorial
Learning to use ISPF is made easier by the online help and tutorial facilities, which are available while
using ISPF.
For example, if you need help filling in the data requested by an ISPF utility, you can use the tutorial to
help you understand the data entry requirements for that utility.
Supported data types
ISPF supports partitioned (PDS), sequential (SEQ), and partitioned extended (PDSE) data sets. These data
sets can be used in any of the ISPF options, such as Edit and Browse. ISPF does not support:
• Data sets with a record format of variable block spanned (VBS)
• Direct access data sets
• Multivolume data sets for the ISRLEMX program, SCLM, and File Tailoring
• Generation data group (GDG) base data sets
• Deletion of data sets allocated with an esoteric device type
• Member aliases, except under the ISPF Move/Copy utility (option 3.3)
• Partitioned data sets as File Tailoring control files (ISPCTLn)
• Unmovable data sets
– In the ISPF Move/Copy utility (option 3.3) or using the LMMOVE or LMCOPY service
– For allocation in the ISPF Data Set utility (option 3.2 option A) or the ISPF Data Set List utility (option
3.4 line command AL)
• Data sets allocated with the BUFNO parameter (ISPF handles its own buffering)
• Browse for packed multivolume data sets.
ISPF provides partial support for VSAM data sets and tape data sets.
• You can create and delete VSAM data sets and obtain VSAM data set information.
• VSAM data sets are supported for Edit, Browse, and View if the ISPF Configuration table has been
customized to enable such support.
• You can define an interface to an external utility such as DFSMSrmm that the Data Set List utility
(Option 3.4) can use to process data sets stored on tape or some other removable media. The interface
is configured in the ISPF configuration table. It enables the Data Set List utility to call the tape or
removable media interface for these line commands:
Supported Data Types
Chapter 1. Overview of ISPF  3

## Page 32

I
Information
S
Short Information
D
Delete
R
Rename
C
Catalog
M
Member List
P
Print
X
Print Index
CO
Copy
MO
Move
ISPF provides support for z/OS UNIX directories and files in the ISPF Edit and Browse options as well as
in the ISPF services BROWSE, EDIT and VIEW. The z/OS UNIX Directory List Utility (option 3.17) supports
processing of directories and files in a z/OS UNIX directory structure.
ISPF requires exclusive enqueues on data sets for many of its functions. If a data set is allocated as
SHAREd to a user and then is operated on by one of these functions, the allocation will be converted to
OLD by MVS dynamic allocation. This allocation of OLD may remain after ISPF frees its enqueue. This is a
restriction of the MVS operating system.
Member name conventions
Members created through ISPF must follow this naming convention:
• The first character must be alphabetic or special (@ # $)
• Characters 2-8 must be alphabetic, numeric, or special (@ # $)
Special characters are as defined in the U.S. English code page (037):
@ (X'7C'), # (X'7B'), $ (X'5B')
All member names created within ISPF are converted to uppercase. If you create members outside of
ISPF that do not meet these conventions, they are displayed in ISPF member lists and can be selected
from those lists. These member names can also be specified for the Browse service with the exception
of member names containing lowercase alphabetics. (ISPF converts the member name to uppercase
before searching for the member and therefore cannot process a lowercase member.) Member names not
meeting the ISPF naming convention are not supported for the other ISPF services.
Note: ISPF does not support using option M (member list) from a data set list and then selecting E to edit
a member whose name contains lowercase letters. ISPF uses the Edit service in this case, and its services
do not support lowercase member names.
Running ISPF
As an interactive dialog, ISPF communicates with you through panels and messages. Ordinarily, the first
panel you see when you enter the ISPF command is the ISPF Primary Option Menu, shown in Figure 2 on
Running ISPF
4  z/OS: z/OS ISPF User's Guide Vol I

## Page 33

page 6. Panels display data, selection lists, and data-entry fields, such as a data set name or an ISPF
command.
ISPF responds interactively to the information or command you enter by displaying another panel,
displaying a message, or carrying out a command. For more information about how panels and messages
are displayed, see “Understanding ISPF panels” on page 16.
One helpful aspect of your interaction with ISPF is the online tutorial. If you need information about using
the online tutorial, see the topic about ISPF Tutorial Panels in z/OS ISPF Dialog Developer's Guide and
Reference.
Starting ISPF
To start ISPF:
1. Log on to TSO.
2. When the READY prompt appears, type ISPF or PDF and press Enter.
If your installation has established an alias for ISPF, such as SPF, you can enter that instead.
The ISPF and PDF commands are aliases of ISPF module ISRPCP. When you run ISRPCP or one of its
aliases with no parameters, ISPF is started through this command:
ISPSTART PANEL(default_primary_panel) NEWAPPL(ISR)
The default primary panel is usually ISR@PRIM, the ISPF Primary Option Menu (see Figure 2 on page
6).
You can specify any of the ISPSTART parameters when invoking ISPF, PDF, or ISRPCP. However, if you do
this you must ensure that you specify all the parameters that ISPSTART needs to run your application in
the correct environment. This is because only those parameters you specify are passed to ISPSTART. For
example, if you specify:
ISPF NEWAPPL(ABC)
ISPF is invoked with this command:
ISPSTART NEWAPPL(ABC)
Note that in this case ISPF does not pass PANEL(ISR@PRIM), part of its normal default string, to
ISPSTART. Because the ISPSTART command generated does not contain a PANEL(…), PGM(…) or CMD(…)
keyword, there is no primary panel to display.
For information about the syntax and options for ISPSTART, including the rules for specifying or overriding
the default primary panel, refer to the z/OS ISPF Dialog Developer's Guide and Reference.
The Primary Option Menu panel
Figure 2 on page 6 shows the first panel, the ISPF Primary Option Menu, with the license information.
Running ISPF
Chapter 1. Overview of ISPF  5

## Page 34

Menu  Utilities  Compilers  Options  Status  Help
 ──────────────────────────────────────────────────────────────────────────────
                            ISPF Primary Option Menu
 0  Settings      Terminal and user parameters            User ID . : USERID
 1  View          Display source data or listings         Time. . . : 11:38
 2  Edit          Create or change source data            Terminal. : 3278
 3  Utilities     Perform utility functions               Screen. . : 1
 4  Foreground    Interactive language processing         Language. : ENGLISH
 5  Batch         Submit job for language processing      Appl ID . : ISR
 6  Command       Enter TSO commands                      TSO logon : ISPF
 7  Dialog Test   Perform dialog testing                  TSO prefix: USERID
 9  IBM Products  IBM program development products        System ID : MVS8
┌──────────────────────────────────────────────────────┐  r        MVS acct. : IBMGSA
│ Licensed Materials - Property of IBM         │          Release . : ISPF 7.5
│ 5650-ZOS     Copyright IBM Corp. 1980, 2021. │
│                                              │ s
│ US Government Users Restricted Rights -      │
│ Use, duplication or disclosure restricted    │
│ by GSA ADP Schedule Contract with IBM Corp.  │
⋘────────────────────────────────────────────────────┘
 Option ===>
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 2. ISPF Primary Option Menu (ISR@PRIM) with license information
Press the Enter key to dismiss the license information. The full Primary Option Menu is displayed.
   Menu  Utilities  Compilers  Options  Status  Help
 ──────────────────────────────────────────────────────────────────────────────
                            ISPF Primary Option Menu
 0  Settings      Terminal and user parameters            User ID . : USERID
 1  View          Display source data or listings         Time. . . : 11:49
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
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 3. ISPF Primary Option Menu (ISR@PRIM)
ISPF Primary Options
When you select one of these options, ISPF displays the selected panel. These options are described in
detail in the z/OS ISPF User's Guide Vol II. Brief descriptions follow:
Option
Description
Running ISPF
6  z/OS: z/OS ISPF User's Guide Vol I

## Page 35

0
Settings displays and changes selected ISPF parameters, such as terminal characteristics and
function keys.
For more information, see the Settings (Option 0) topic in the z/OS ISPF User's Guide Vol II.
 1
View displays data using the View or Browse function. You can use View or Browse to look at (but not
change) large data sets such as compiler listings. You can scroll the data up, down, left, or right. If you
are using Browse, a FIND command, entered on the command line, allows you to search the data for
a character string. If you are using View, you can use all the commands and macros available to you in
the Edit function.
For more information, see the View (Option 1) topic in the z/OS ISPF User's Guide Vol II.
 2
Edit allows you to create or change source data such as program code and documentation using the
ISPF full-screen editor. You can scroll the data up, down, left, or right. You can change the data by
using Edit line commands, which are entered directly on a line number, and primary commands, which
are entered on the command line.
For more information, see the Edit (Option 2) topic in the z/OS ISPF User's Guide Vol II, and refer to
z/OS ISPF Edit and Edit Macros.
 3
Utilities perform library and data set maintenance tasks, such as moving or copying library or data
set members, displaying or printing data set names and volume table of contents (VTOC) information,
comparing data sets, and searching for strings of data.
For more information, see the Utilities (Option 3) topic in the z/OS ISPF User's Guide Vol II.
 4
Foreground calls IBM language processing programs in the foreground.
For more information, see the Foreground (Option 4) topic in the z/OS ISPF User's Guide Vol II.
 5
Batch calls IBM language processing programs as batch jobs. ISPF generates Job Control Language
(JCL) based on information you enter and submits the job for processing.
For more information, see the Batch (Option 5) topic in the z/OS ISPF User's Guide Vol II.
 6
Command calls TSO commands, CLISTs, or REXX EXECs under ISPF.
For more information, see the Command (Option 6) topic in the z/OS ISPF User's Guide Vol II.
 7
Dialog Test tests individual ISPF dialog components, such as panels, messages, and dialog functions
(programs, commands, menus).
For more information, see the Dialog Test (Option 7) topic in the z/OS ISPF User's Guide Vol II.
 9
IBM Products allows you to select other installed IBM program development products on your
system. Products supported are:
• Tivoli Information Management
• COBOL Structuring Facility foreground dialog (COBOL/SF)
• Screen Definition Facility II (SDF II) licensed program
• Screen Definition Facility II-P (SDF II-P) licensed program.
For more information, see the IBM Products (Option 9) topic in the z/OS ISPF User's Guide Vol II.
10
SCLM controls, maintains, and tracks all of the software components of an application.
Running ISPF
Chapter 1. Overview of ISPF  7

## Page 36

For more information, see the z/OS ISPF Software Config ur ation  and Library Manager Guide and
Reference.
11
Workplace gives you access to the ISPF Workplace, which combines many of the ISPF functions onto
one object-action panel.
For more information, see the ISPF Object/Action Workplace (Option 11) topic in the z/OS ISPF User's
Guide Vol II.
X
EXIT leaves ISPF using the log and list defaults. You can change these defaults from the Log/List
pull-down on the ISPF Settings panel action bar.
Primary Option Menu action bar choices
The Primary Option Menu action bar choices function as follows:
Menu
This choice is available from most panels within ISPF. It displays many of the options listed on the
Primary Option Menu panel. See the topic about the Primary Option Menu in the z/OS ISPF User's
Guide Vol II for details on each choice.
Utilities
This choice is available from most panels within ISPF. It displays the options listed on the Utility
Selection panel (Option 3). See the topic about the Primary Option Menu in the z/OS ISPF User's Guide
Vol II for details on each choice.
Compilers
The Compilers pull-down provides access to the foreground and background compilers listed under
options 4 and 5, as well as the ISPPREP (Preprocessed Panel) utility and the ISPDTLC (Dialog Tag
Language Conversion) utility.
Options
The Options pull-down provides access to many ISPF settings, including CUA attributes and colors,
keylists, and point-and-shoot fields, and the Dialog Test Application ID pop-up. See the topic about
Settings (Option 0) in the z/OS ISPF User's Guide Vol II for details on each choice.
Status
The Status pull-down enables you to control what is displayed in the status area of the Primary Option
Menu.
Note: The current setting is shown as an unavailable choice. That is, it is colored blue (the default)
with an asterisk as the first digit of the selection number.
See the topic about the status area on the Primary Option Menu in the z/OS ISPF User's Guide Vol II for
more information about using these choices to tailor the status area.
Help
The Help pull-down provides access to the online help and tutorial topics for the main ISPF options.
User profiles
ISPF stores information in your user profile. This allows ISPF to insert a value in panel input fields by
using the values you last entered on either the same panel or a similar type of panel. Sometimes default
values are provided if you have not specified otherwise. Information maintained in your user profile
includes:
• Project name, group names, and type
• Job statement information 1
• SYSOUT class for printed output 1
• Defaults for list and log allocation and processing
1 This information is maintained separately for SCLM.
Running ISPF
8  z/OS: z/OS ISPF User's Guide Vol I

## Page 37

• Terminal characteristics and function key definitions
• Edit profiles, including mask, tabs, and bounds
• Current scroll amount for Browse, Edit, Data Set List, and member lists 1
• Processing options for each of the language processors
• Data set allocation/information parameters.
ISPF maintains this information automatically from one session to another. If you are a new user, you
have to enter certain information the first time. But then, you simply review the information and make
whatever changes you want before proceeding.
ISPF maintains sets of job statements for:
• JCL for printing the Log and List data sets
• The Hardcopy utility (3.6)
• The Outlist utility (3.8)
• The SuperC utility (3.12)
• The SuperCE utility (3.13)
• The Foreground Print Options panel (option 4, excluding SCRIPT/VS)
• The Batch option (5)
• The SCLM option (10)
• SCRIPT/VS draft output (4.9)
• SCRIPT/VS final output (4.9)
• Ending ISPF
Thus, you can provide different job statement parameters for each of these functions. For more
information, see “Job statement information” on page 113.
Getting help
The HELP command (F1/13) shows you general information about an ISPF system command, ISPF
option, or panel, or offers more information about a message that has been displayed in the upper-right
corner of the screen.
For short messages, HELP displays a one-line explanation. To get further information, enter the HELP
command a second time for the appropriate section of the tutorial. Long messages display (by default) in
a pop-up window. Enter END (F3/15) or RETURN (F4/16) to return to the screen that you were viewing
when you entered the HELP command.
Ending an ISPF function or ISPF
To end an ISPF function without ending ISPF, enter END or RETURN.
Note:
1. If you are using the Hardcopy utility (option 3.6) or the Batch option (option 5), entering END or
RETURN submits your job for processing. Type CANCEL (or press F12) to leave the Hardcopy utility
without submitting a job.
2. If a SuperC or Search-For member list is displayed, END processes any members you have selected.
Enter CANCEL or RETURN to leave one of these member lists without processing your selections.
To end ISPF from the ISPF Primary Option Menu, you can use the:
• EXIT command (F3)
• END command
• RETURN command
• Exit option (X).
Running ISPF
Chapter 1. Overview of ISPF  9

## Page 38

If the display screen is split, taking one of the actions listed ends ISPF on the active logical screen only.
See z/OS ISPF Dialog Developer's Guide and Reference for more information.
Exit option (X)
The Exit option ends ISPF using any defaults for processing log and list data sets that you have specified
using the Log/List pull-down on the ISPF Settings panel action bar. If you have not specified any defaults
and if a log or list data set has been generated, the Exit option displays the Specify Disposition of Log and
List Data Sets panel.
Using the Exit option (X) with the jump function
If the current primary options panel has been coded to select the EXIT command for the X selection,
you can use the jump function. Enter =X from any panel to immediately leave the current primary options
panel. If there is only one ISPF logical screen active and that logical screen has only one primary option
panel in effect, =X exits ISPF entirely.
ISPF recognizes =XALL as an extended version of =X. When you enter =XALL, ISPF propagates an =X to all
active logical screens to request the termination of all logical screens and the exit of ISPF entirely.
Note: In the situation where a logical screen is running an application that does not support the =X
command, ISPF suspends the =XALL termination processing at that logical screen. If you perform
additional processing in that logical screen before you terminate it, the =XALL termination processing
remains suspended. When that application is terminated, =XALL processing continues for any remaining
logical screens.
Running a sample ISPF session
This topic provides an example of an ISPF session. For new users, it is a quick introduction to ISPF. For
users with previous ISPF experience, it is a quick review. It can also be used to demonstrate that ISPF has
been properly installed and is operational.
The scenario requires the installation of a data set named ISP.SISPSAMP. This data set is included on the
ISPF basic distribution tape and should contain these four members:
ISRASM
Sample assembler source
ISRCOBOL
Sample COBOL source
ISRFORT
Sample FORTRAN source
ISRPLI
Sample PL/I source.
During this scenario, member ISRASM is copied from ISP.SISPSAMP to a user data set. The other three
members are not used.
Table 1. A sample ISPF session
Action Result
1. Log on to TSO. READY is displayed on your screen.
2. Type ISPF (or the appropriate alias) and press Enter. The ISPF Primary Option Menu is displayed.
3. On the Option line, type 3 to select the Utilities option.
Then press Enter.
The Utility Selection Panel is displayed.
4. On the Option line, type 2 to select the Data Set utility.
Then press Enter.
The Data Set Utility panel is displayed.
Running a sample ISPF session
10  z/OS: z/OS ISPF User's Guide Vol I

## Page 39

Table 1. A sample ISPF session (continued)
Action Result
5. On the Option line, type A to allocate a new data set.
Specify an ISPF library by typing this information, but
substitute your first name in the Group field:
Project  . . your_user_id
Group  . . . name
Type . . . . ASM
Now press Enter.
The Allocate New Data Set panel is displayed. Note:
The Project name must be defined as a valid high-
level identifier on your installation. Generally, user-
ids are defined as such. If this is not true for
your installation, ask your system programmer what
project names are valid for your system.
6. Type the information shown.
Note: Leave ALL fields blank except:
Space units . . . . . . TRKS
Primary quantity. . . . 2
Secondary quantity. . . 1
Directory blocks. . . . 1
Record format . . . . . FB
Record length . . . . . 80
Block size  . . . . . . 3120
Now press Enter.
Data set your_user_id.name.ASM is allocated on
scratch volume. The Data Set Utility panel is
displayed.
7. Select option A again and specify this ISPF library,
again substituting your first name in the Group field:
Project  . . your_user_id
Group  . . . name
Type . . . . OBJ
Press Enter.
The Allocate New Data Set panel is displayed.
8. Leave everything the same. Just press Enter. Data set your_user_id.name.OBJ is allocated. The
Data Set Utility panel is displayed.
9. Press F3. The Utility Selection Panel is displayed.
10. Press F3 again. The ISPF Primary Option Menu is displayed.
11. Type 3.3 to select the Move/Copy utility, bypassing
the Utility Selection Menu. Press Enter.
The Move/Copy Utility panel is displayed.
12. Now you will copy a data set. On the Option line, type
C to select Copy data set or member(s). Then,
under From Other Partitioned or Sequential Data Set:,
enter this data set name:
Data Set Name . . . 'ISP.SISPSAMP(ISRASM)'
Press Enter.
A panel titled COPY FROM ISP.SISPSAMP(ISRASM)
is displayed.
13. Under To ISPF Library:, type these values:
Project  . . your_user_id
Group  . . . name
Type . . . . ASM
Press Enter.
Member ISRASM is copied from data
set ISP.SISPSAMP to ISPF library
your_user_id.name.ASM. Then, the Move/Copy
Utility panel is displayed.
14. Press F3. The ISPF Primary Option Menu is displayed.
Running a sample ISPF session
Chapter 1. Overview of ISPF  11

## Page 40

Table 1. A sample ISPF session (continued)
Action Result
15. Now you will browse member ISRASM. On the Option
line, type 1 to select View and press Enter.
The View Entry Panel is displayed. Select Browse
Mode.
16. Type these details under ISPF LIBRARY:
Project  . . your_user_id
Group  . . . name
Type . . . . ASM
Note: Leave the Member field blank.
Press Enter.
A member list is displayed, showing ISRASM as the
only member in the your_user_id.name.ASM library.
17. Move the cursor to the left of ISRASM. Then type S to
select ISRASM and press Enter.
A panel titled BROWSE your_user_id.name.ASM
(ISRASM) is displayed. This is the first page of
member ISRASM.
18. Press F8 to scroll ahead one page. The second page of ISRASM is displayed.
19. Press F7 to scroll backward one page. The first page of ISRASM is displayed.
20. Type FIND COMMENT on the Command line and
press Enter.
The cursor moves to the first occurrence of
the character string COMMENT and the string is
intensified. Also, the message CHARS 'COMMENT'
FOUND is displayed in the upper-right corner of the
screen.
21. To find the next occurrence of COMMENT, press F5,
the RFIND command.
The cursor moves to the second occurrence of
COMMENT and once again the string is intensified.
22. Press F3. The member list is displayed.
23. Press F3 again. The View Entry Panel is displayed.
24. Press F3 one more time. The ISPF Primary Option Menu is displayed.
25. Now you will edit member ISRASM. On the Option
line, type 2 to select Edit and press Enter.
The Edit Entry Panel is displayed.
26. Type ISRASM in the Member field and press Enter. A panel titled EDIT
your_user_id.name.ASM(ISRASM) is displayed. This
is the first page of member ISRASM.
27. On the Command line, type FIND COMMENT and
press Enter to find the line containing the character string
COMMENT.
The cursor moves to the first occurrence of the
character string and the line number is intensified.
28. Delete COMMENT by pressing the End key. COMMENT is erased. Any characters to the right of
COMMENT are erased, also.
29. Press F5 to find the next occurrence of COMMENT. The cursor moves to the second occurrence of
COMMENT and the line number is intensified.
30. Move the cursor to the sequence number of the
line below COMMENT, then move the cursor up one line.
Repeat the COMMENT line by typing R over the first digit of
the line number and pressing Enter.
The line is repeated.
31. Try out more Edit commands if you like, but
remember: this program will be assembled later.
HAVE FUN!
Running a sample ISPF session
12  z/OS: z/OS ISPF User's Guide Vol I

## Page 41

Table 1. A sample ISPF session (continued)
Action Result
32. Press F3. Member ISRASM is saved in data set
your_user_id.name.ASM. The Edit Entry Panel is
displayed.
33. Press F3 again. The ISPF Primary Option Menu is displayed.
34. Move the cursor to Help on the action bar. Press
Enter. On the resulting pop-up window, select 18.
The beginning of the ISPF tutorial is displayed.
Follow the directions to learn more about ISPF.
When you have finished, press F3 to return to the
ISPF Primary Option Menu.
Table 2. Finishing the sample session
Action Result
35. With the ISPF Primary Option Menu on the screen,
press F3.
The ISPF Specify Disposition of Log and List Data
Sets panel is displayed.
36. Select the process option to print and delete both
the log and list data sets, and fill in the job statement
information as required by your installation. Press Enter.
Data sets will be printed, then deleted through
batch jobs submitted by ISPF. The job name is
displayed at the bottom of the screen.
37. You are now out of ISPF. To leave TSO, type LOGOFF
and press Enter.
This is the end of this usage scenario.
Running a sample ISPF session
Chapter 1. Overview of ISPF  13

## Page 42

Running a sample ISPF session
14  z/OS: z/OS ISPF User's Guide Vol I
