# Chapter 12. ISPF object/action workplace (option 11)

Source file: f54u200_v3r1.md
Start page: 441
Page span: 441-468

## Page 441

Chapter 12. ISPF object/action workplace (option 11)
Option 11 gives you access to the ISPF Object/Action Workplace. The Workplace combines many of the
ISPF functions onto one object-action interface. The idea of object-action is to specify an object (such as
an ISPF library or data set name) and then specify an action to perform upon it. You can specify any of
these objects:
• An ISPF Library—a cataloged partitioned data set (PDS) with a three-level data set name in the
project.group.type format.
• A partitioned or sequential data set.
• A VSAM data set for use with the data set actions allocate, delete, or information.
• A DSLIST level for data set list actions, for example, 'YOURID.*' for all data sets beginning with YOURID.
• A volume serial number for uncataloged data sets to use with actions to retrieve volume information,
print volume information, build a DSLIST based on a volume serial number, or as a filter for a DSLIST
level.
• Personal data set lists or reference lists for use with action DL (DSLIST) only.
Additionally, Workplace provides ISPF Referral list fields to enable object selection through retrieval
from personal lists (pre-packaged lists of data sets which you create) or reference lists (lists of recently
referenced data sets which ISPF creates).
You can select an action by making a choice on an action bar or by using a command. Eighty-five ISPF
functions are available as workplace actions.
There is a fast path system command for starting the Workplace. Type ISPFWORK on any ISPF command
line and you are taken to the Workplace entry panel.
Selecting objects
The first step in using the Workplace to perform ISPF functions is to specify the particular object that
you want to perform an action on, for example, a sequential data set 'YOURID.SOURCE.DATA'. Object
specification takes place on the Workplace entry panel.
Workplace entry panel
When you first enter the Workplace, the entry panel that appears is called ISPF Workplace. It is possible
to display this panel in two distinct modes, called views: the Library View or the Data Set View.
The Library View panel has the words "Library View" as a heading just above the referral lists in the lower
portion of the screen. This view enables you to work with ISPF library concatenations and library lists.
The Data Set View panel has the words "Data Set View" as a heading just above the referral lists in
the lower portion of the screen. This view enables you to work with data set lists, sequential files, or
single partitioned data sets. You can choose to work with either entry panel view by using the command
LISTVIEW, or the function key ChgView (F11) to toggle between the two panels.
Library view
You use the Library View to work with a ISPF library concatenations. The panel that appears in Figure 242
on page 404 is the Library View entry panel for the Workplace.
Workplace (Option 11)
© Copyright IBM Corp. 1980, 2024 403

## Page 442

File  View  Options  Space  SuperC  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
           Smart Action         ISPF Workplace
 ISPF Library
    Project  . . .         
    Group  . . . .          . . .          . . .          . . .         
    Type . . . . . SOURCE  
    Member . . . .             (Blank or pattern for member action list)
 ┌───────────────── ISPF Referral lists for object selection ─────────────────┐
 │  Library View    Action      #1-8=Retrieve Entry  DL=DSLIST  /=Open List   │
 │  MYLIST   . . .              MANUAL DUMMY DATA              02/07/08 11:36 │
 │  REFLIST  . . .              Last 8 referenced libraries    -------- ----- │
 ************************* End of ISPF Referral lists *************************
 Action ===>                                                   Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F4=Settings  F5=PrvObj    F6=PrvAct
  F7=Backward  F8=Forward   F9=Swap     F10=Actions  F11=ChgView  F12=Cancel
Figure 242. Workplace entry panel - library view (ISRWORK1)
When using this Workplace entry panel, you can select a data set or a group of data sets to work with in
one of these ways:
• Fill in the ISPF Library fields. These fields are used the same way as they are on other ISPF panels. You
can use the traditional method of selecting a data set by entering its Project, Group, Type, and Member
names in the ISPF Library fields. Omitting the Member name gives you a list of members to choose
from.
• Select an object or list of objects using "ISPF Referral lists for object retrieval". See “ISPF referral lists
for object retrieval” on page 405 for more information. You can access personal and reference lists, and
then select libraries from these lists.
Data set view
You use the Data Set View to work with a single data set, a list of data sets, or any action that requires a
volume serial.
Note: Catalog, DSLIST, Volume information, and Print volume actions, are only available from the Data Set
View.
The panel that appears in Figure 243 on page 405 is the Data Set View entry panel for the Workplace.
Workplace (Option 11)
404  z/OS: z/OS ISPF User's Guide Vol II

## Page 443

Figure 243. Workplace entry panel - data set view (ISRWORK)
When using this Workplace entry panel, you can select a data set or a group of data sets to work with in
one of these ways:
• You can use the traditional method of selecting a data set by entering its name in the Object Name field.
For example, enter 'YOURID.SOURCE.DATA' to act upon a data set.
Note: The Object Name field supports the inclusion of system symbols.
• Select an object or list of objects using "ISPF Referral lists for object retrieval". You can access personal
and reference lists, and select data sets, libraries, VSAM files, and data set levels from each list.
ISPF referral lists for object retrieval
Both views of the ISPF Workplace entry panel enable you to use referral lists. The bottom of the ISPF
Workplace panel contains a reference list entry field (REFLIST), followed by a list of personal lists. You can
display either referral library lists or referral data set lists, depending on the view you choose.
All data sets and libraries referenced during an ISPF session are appended to the reference lists. You can
use the input fields next to the referral lists to access a referral data set in one of these ways:
• Entering a slash (/) in this field causes the personal data set list or library list (depending on the selected
view) to be displayed.
• Type DL in the input field and press Enter. This builds a DSLIST based on entries in the personal data set
list, personal library lists, or Reflists.
• Enter a library entry number (from 1 to 8). If you know the list numbers of your libraries, for example,
your panels library is number 1, you can type the number in this field and press Enter. ISPF retrieves the
respective library entry from the library reference list.
• Enter a data set entry number (from 1 to 30). If you know the order of your data sets, you can type
the number in this field and press Enter. ISPF retrieves the respective data set entry from the data set
reference list.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  405

## Page 444

Specifying actions
After you select the object you want to work with, choose the action to perform on it. You can select an
action by making a choice on an action bar or by using a command.
Choices on the Workplace action bar
The Workplace action bar makes available these choices.
File
The File action bar choice enables you to manipulate files. The pull-down choices for File are:
Choice
Description
List
Displays a pop-up menu that enables you to choose either a member list, a data set list list, a list of
personal data set lists or a list of personal library lists. You can perform any of the File actions except
DSLIST against the resulting member list.
Member list
Displays a list of members for a partitioned data set. To display a member list:
1. Type the library or data set information in the appropriate fields of the Workplace entry panel
(library view).
2. Specify blank or a pattern for the member name to display a member list.
3. Select the List action from the File action bar choice.
4. Select "Member list" from the List Action prompt panel.
Note: All member lists displayed by the ISPF Workplace are enhanced member lists, all supported
member list actions and commands are available on any member list display.
Data Set list
Displays a list of data sets based on a DSLIST level and, optionally, a volume serial number. The
data set list initial view can be set from the Workplace Settings panel. To create a data set list:
1. Type the data set level in the Object name field on the Workplace entry panel (data set view). If
you do not full qualify the data set level (by enclosing it in single quotes), your TSO prefix is set
as the first level. Optionally, you can enter a volume to view just the data sets that match the
DSLIST level on the volume entered. You can also optionally enter just a volume name to list all
data sets on the volume entered.
2. Select the List action from the File action bar choice.
3. Select Data Set List from the List Action prompt panel.
Personal Data Set lists
Displays a list of your personal data set lists. All valid personal list actions can be performed
against any selected personal list. The personal data set list you used most recently is the current
active list. The currently active list cannot be directly deleted from the list dialog, however all
other list actions are valid. To list your personal data set lists:
1. Select the List action from the File action bar choice.
2. Select Personal Data Set List from the List Action prompt panel. You can perform this action
from either view of the Workplace entry panel.
Personal Library lists
Displays a list of your personal library lists. All valid personal list actions can be performed against
any selected personal list. The personal library list you used most recently is the current active
list. The currently active list cannot be directly deleted from the list dialog, however all other list
actions are valid. To list your personal library lists:
Workplace (Option 11)
406  z/OS: z/OS ISPF User's Guide Vol II

## Page 445

1. Select the List action from the File action bar choice.
2. Select Personal Library List from the List Action prompt panel. You can perform this action from
either view of the Workplace entry panel.
Edit
Starts Edit action for a member or a sequential file.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select a member to Edit by typing s next to the member name.
To edit a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Edit from the File action bar choice.
View
Starts View action for a member or a sequential file.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select a member to View by typing s next to the member name.
To view a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select View from the File action bar choice.
Browse
Starts Browse action for a member or a sequential file.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select a member to Browse by typing s next to the member name.
To browse a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Browse from the File action bar choice.
Delete
Displays a pop-up prompt window with member or data set as the choices.
If you specify an asterisk (*) as the member name, all members of the PDS are deleted without a
member list being displayed.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select members to delete by typing s next to the member name.
Note: You can change how member name patterns are handled in your Workplace Settings. See Show
status for M,C,D,G actions for more information.
To delete a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Delete under the File action bar choice for member delete.
To delete a PDS or a sequential data set:
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  407

## Page 446

1. Enter the data set name in the Object name field, or enter a library in the ISPF Library field on the
Workplace panel.
2. Select Delete under the File action bar choice for data set delete.
Rename
Displays a pop-up prompt window with member or data set as the choices.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select a member to Rename by typing s next to the member name.
To rename a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Rename under the File action bar choice for member rename.
To rename a PDS or a sequential data set:
1. Type the data set name in the Object name field, or type a library in the ISPF Library field on the
Workplace panel.
2. Select Rename under the File action bar choice for data set rename.
For more information, see “Rename” on page 419.
Move
Starts the move action for a member or a sequential file. A Move entry panel is presented.
If you specify an asterisk (*) as the member name, all members of the PDS are moved without a
member list being displayed.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select members to move by typing s next to the member name.
Note: You can change how member name patterns are handled in your Workplace Settings. See Show
status for M,C,D,G actions for more information.
To move a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Move from the File action bar choice.
For more information, see “Move or copy” on page 418.
Copy
Starts the copy action for a member or a sequential file. A Copy entry panel is presented.
If you specify an asterisk (*) as the member name, all members of the PDS are copied without a
member list being displayed.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select members to copy by typing s next to the member name.
Note: You can change how member name patterns are handled in your Workplace Settings. See Show
status for M,C,D,G actions for more information.
To copy a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Copy from the File action bar choice.
Workplace (Option 11)
408  z/OS: z/OS ISPF User's Guide Vol II

## Page 447

For more information, see “Move or copy” on page 418.
Reset
Starts reset action for a member. A Reset prompt panel is presented for the member.
If you specify an asterisk (*) as the member name, all members of the PDS are reset without a
member list being displayed.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select members to reset by typing s next to the member name.
Note: You can change how member name patterns are handled in your Workplace Settings. See Show
status for M,C,D,G actions for more information.
To reset a single member:
1. Type the library or data set information in the appropriate fields of the ISPF Workplace panel.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
3. Select Reset from the File action bar choice.
For more information, see “Resetting member statistics” on page 415.
Open
Is defined on the Workplace Settings panel, making it a user customizable action. After you set this
action, it is performed automatically each time you open a member. The Open action can be set to
these actions:
User
Any TSO command, REXX exec, or CLIST set by the Open Command field on the Workplace
Settings panel.
E
Edit
V
View
B
Browse
D
Delete member
R
Rename member
M
Move
C
Copy
G
Reset
P
Print member
J
Submit
T
TSO command action
To open a single member:
1. Type the member name in the member field (for ISPF library view) or in parentheses after the data
set name (for data set view).
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  409

## Page 448

2. Select Open from the File action bar choice. When you press the ENTER key, the action for open
runs against the member, or a member list is displayed. See “Changing workplace settings” on
page 423 for more information.
Submit
Submits the member or sequential file to TSO for job execution.
If you do not specify a member name or if you specify a pattern and the specified data set is a PDS, a
member list is displayed. Select a member to submit by typing s next to the member name.
To submit a single member, fill in these fields of the ISPF Workplace panel:
1. Type the library or data set information in the appropriate fields.
2. Type the member name in the member field (for ISPF library view) or in parentheses after data set
name (for data set view).
3. Select Submit from the File action bar choice.
Print
The Print selection enables you to print information. The pull-down choices on the Print action prompt
panel are:
Data Set
Prints the entire data set. To print a data set:
1. Type the data set name in the Object name field (for the data set view) or enter an ISPF library
name in the ISPF Library fields (for the library view).
2. Select the Print action from the File action bar choice.
3. Select Data Set from the Print Action prompt panel.
Data Set index
Prints the data set index for the selected data set. To print a data set index:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF
Library fields.
2. Select the Print action from the File action bar choice.
3. Select Data Set Index from the Print Action prompt panel.
Data Set List
Prints the list of data sets for the selected data set name level. To print a data set list:
1. Type a data set level, or optionally a volume serial, in the appropriate fields on the ISPF
Workplace panel.
2. Select Print from the File action bar choice.
3. Select Data set List from the Print Action prompt prompt panel.
VTOC
Prints the VTOC information for the selected volume. To print a VTOC summary:
1. Type a volume serial in the proper field on the ISPF Workplace panel.
2. Select Print from the File action bar choice.
3. Select VTOC from the Print Action prompt panel.
Member
Prints the selected member. To print a member:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF
Library fields.
2. Select the Print action from the File action bar choice.
3. Select Member from the Print Action prompt panel.
Workplace (Option 11)
410  z/OS: z/OS ISPF User's Guide Vol II

## Page 449

Command
Enables you to enter TSO or ISPF commands. You are prompted to choose between types of
commands. The pull-down choices on the Command prompt are:
TSO Cmd
TSO or ISPF commands, passing the data set and member name and any additional parameters to
the TSO command entered. To run a TSO command against a single member, fill in these fields of
the ISPF Workplace panel:
1. Type the library or data set information in the appropriate fields.
2. Type the member name in the member field (for ISPF library view) or in parentheses after the
data set name (for data set view).
3. Select Command from the File action bar choice.
4. Select TSO from the Command Action prompt panel.
ISPF Command Shell
The ISPF command shell option enables TSO commands, CLISTs, and REXX execs to be run under
ISPF. You can enter the TSO commands, CLISTs, and REXX execs in the command input field of
any panel.
You can enter a long command that wraps to the next line if you want to. For more information
about the ISPF Command Shell, see “ISPF command shell” on page 417.
ISPF Command Table
The command table utility allows you to create or change application command tables.
A command table contains the specification of general commands that can be entered from any
panel during the execution of an application. Command tables are identified by application id, and
are maintained in the ISPF table input library.
Exit
Ends the Workplace, returning to the primary option panel.
View
The View action bar choice displays the object views that are available to you. The currently selected view
is unavailable.
The pull-down choices for View are:
Choice
Description
Data Set View
Changes the current view to reference data set list, personal data set lists, and Object name view.
To change to the data set view:
1. Select the View action bar choice.
2. Select Data Set View from the pull-down menu.
Library View
Changes the current view to reference library list, personal library lists, and ISPF Library view.
To change to the ISPF Library view:
1. Select the View action bar choice.
2. Select Library View from the pull-down menu.
By name
Changes the current view of the personal list by sorting on the name field.
By description
Changes the current view of the personal list by sorting on the description field.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  411

## Page 450

By created
Changes the current view of the personal list by sorting on the created field.
By referenced
Changes the current view of the personal list by sorting on the referenced field.
Options
The Options action bar choice displays the settings available. The pull-down choices for Options are:
Choice
Description
Workplace Settings
Displays the Workplace Settings panel. See “Changing workplace settings” on page 423 for more
information.
ISPF Settings
Displays the ISPF Settings panel. See Chapter 2, “Settings (option 0),” on page 27 for more
information.
CUA Attributes
Starts the ISPF CUA Attribute Change Utility dialog. See “CUA cttributes” on page 53 for more
information.
Keylists
Starts the ISPF Keylist Utility dialog. See “Working with function keys and keylists (the Function Keys
action bar choice)” on page 41 for more information.
Point-and-Shoot
Starts the ISPF CUA Attribute Change Utility dialog indexed to the point-and-shoot entry. See “CUA
cttributes” on page 53 for more information.
Colors
Starts the ISPF Global Color Change Utility dialog. See “Changing default colors (the Colors action bar
choice)” on page 52 for more information.
Space
The Space action bar choice enables you to create and maintain data sets. The pull-down choices
available for Space are:
Choice
Description
Allocate
Displays a pop-up menu for the allocate action. The choices on the prompt are:
Data Set
The allocate action is used to allocate a partitioned or sequential data set. To allocate a data set:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF
Library fields.
2. Select the Allocate action from the Space action bar choice.
3. Select Data Set from the Allocate Action prompt panel.
Enhanced Data Set
The enhanced allocate action is used to allocate an SMS-managed partitioned or sequential data
set. To allocate an SMS data set:
1. Type the data set name in the Object name field or enter an ISPF library name in the ISPF
Library fields.
2. Select the Allocate action from the Space action bar choice.
3. Select Enhanced Data Set from the Allocate Action prompt panel.
Workplace (Option 11)
412  z/OS: z/OS ISPF User's Guide Vol II

## Page 451

VSAM Data Set
The VSAM action is used to define, delete, or retrieve information for a VSAM data set. To define,
delete, or retrieve information for a VSAM data set:
1. Type the VSAM data set name in the Object name field or enter an ISPF library name in the
ISPF Library fields.
2. Select the Allocate action from the Space action bar choice.
3. Select VSAM Data Set from the Allocate Action prompt panel.
Compress
The Compress action is used to recover unused space in a partitioned or sequential data set. To
compress a data set:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF Library
fields.
2. Select the Compress action from the Space action bar choice.
Catalog
The Catalog action is used to catalog a partitioned or sequential data set on a direct access device. To
catalog a data set:
1. Type the data set name in the Object name field.
2. Type the volume name in the volume field.
3. Select the Catalog action from the Space action bar choice.
Note: You cannot catalog an SMS-managed data set.
Uncatalog
The Uncatalog action is used to uncatalog a partitioned or sequential data set from a direct access
device. To uncatalog a data set:
1. Type the data set name in the Object name field.
2. Select the Uncatalog action from the Space action bar choice.
Note: You cannot uncatalog an SMS-managed data set.
A confirmation dialog appears if specified in the Workplace Settings panel. See “Changing workplace
settings” on page 423 for more information.
Information
The Data Set Information action is used to retrieve information about a partitioned or sequential data
set. To retrieve data set information:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF Library
fields.
2. Select Information from the Space action bar choice.
3. Select one of these choices from the Information Action prompt panel:
Data Set Long
Displays information about the selected data set.
Data Set Short
Displays a subset of information about the selected data set.
VTOC summary
Displays VTOC information about the selected volume. You must type the volume serial of the
VTOC in the volume serial field of the Workplace entry panel.
SuperC
The SuperC action bar choice gives you access to SuperC compare and search dialogs for your data sets.
The data set you specify on the Workplace panel is automatically filled in for you in the SuperC dialog you
choose. For more information, see “SuperC utility (option 3.12)” on page 183.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  413

## Page 452

The SuperC pull-down choices are:
Choice
Description
SuperC
Compare two data sets. To SuperC compare two data sets:
1. Type the first data set name in the Object name field or type an ISPF library name in the ISPF
Library fields.
2. Select the SuperC action from the SuperC action bar choice. The SuperC Compare Utility— New
Data Set Specification panel appears with the data set information entered in it. Make sure the
panel is filled in the way you want it to be.
3. Press Enter to display the Old SuperC comparison panel, and fill in the panel.
4. Press Enter again to submit the comparison.
For more information, see “SuperC utility (option 3.12)” on page 183.
SuperCE
Compare two data sets using extended options. For more information, see “SuperCE utility (option
3.13)” on page 192.
Search-For
Search data sets for strings of data. To SuperC search for strings of data:
1. Type the data set name in the Object name field or type an ISPF library name in the ISPF Library
fields.
2. Select the Search-For action from the SuperC action bar choice. The Search-For Utility panel
appears with the data set information entered in it. Make sure the panel is filled in the way you
want it to be.
For more information, see “Search-For utility (option 3.14)” on page 203.
Search-ForE
Search a data set using extended options. For more information, see “Search-ForE utility (option
3.15)” on page 209.
Test
The Test action bar choice gives you access to the ISPF services that help you test dialogs, such as
Chapter 9, “Dialog test (option 7),” on page 355. For more information, refer to the z/OS ISPF Dialog
Developer's Guide and Reference, and the z/OS ISPF Edit and Edit Macros.
The Test pull-down choices are:
Choice
Description
Functions
Displays the Dialog Test Function/Selection panel. Select the Functions action from the Test action bar
choice. For more information, see “Functions (option 7.1)” on page 362.
Panels
Displays the Dialog Test Display panel. Select the Panels action from the Test action bar choice. For
more information, see “Panels (option 7.2)” on page 365.
Variables
Displays the Dialog Test Variables panel. Select the Variables action from the Test action bar choice.
For more information, see “Variables (Option 7.3)” on page 367.
Tables
Displays the Dialog Test Tables panel. Select the Tables action from the Test action bar choice. For
more information, see “Tables (option 7.4)” on page 372.
Workplace (Option 11)
414  z/OS: z/OS ISPF User's Guide Vol II

## Page 453

Log
Displays the ISPF Transaction Log panel. Select the Log action from the Test action bar choice. For
more information, see “Log (option 7.5)” on page 383.
Services
Displays the Invoke Dialog Service panel. Select the Services action from the Test action bar choice.
For more information, see “Dialog services (option 7.6)” on page 385.
Traces
Displays the Dialog Test Traces panel. Select the Traces action from the Test action bar choice. For
more information, see “Traces (option 7.7)” on page 388.
Break Points
Displays the Dialog Test Breakpoints panel. Select the Break Points action from the Test action bar
choice. For more information, see “Breakpoints (option 7.8)” on page 391.
Dialog Test
Displays the Dialog Test Primary Option panel. Select the Dialog Test action from the Test action bar
choice. For more information, see Chapter 9, “Dialog test (option 7),” on page 355.
Dialog Test appl ID
Displays the Dialog Test Application ID panel for changing the Dialog Test application ID. Select the
Dialog Test appl ID action from the Test action bar choice.
Help
The Help action bar choice provides access to the program tutorials.
Actions that require prompt windows for more information
Some actions that you call from the Workplace require additional information. You provide this
information through the use of pop-up prompt windows. Some common actions of this type are:
• Resetting member statistics
• Using TSO commands
• Using the ISPF command shell
• Moving or copying data
• Renaming data sets
Here are the actions and the pop-up windows that accompany each one.
Resetting member statistics
Figure 244 on page 416 shows the pop-up prompt window that appears when you choose Reset from the
File action bar, after you choose a member to work with.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  415

## Page 454

Reset Member Statistics                          
                                                                       
  Data Set Name:                                                       
  'JOHNLEV.TEST.DATA(EMP)'                                             
                                                                       
  Options                                                              
     1. Reset ISPF statistics                                          
     2. Delete ISPF statistics                                         
                                                                          
  New Userid  . . .          (If userid is to be changed)              
  New Version . . .          (If version number is to be changed)      
  New Mod . . . . .          (If mod number is to be changed)               
                                                                         
  Press ENTER to process action. Press CANCEL to cancel reset.          
                                                                          
                                                                          
  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward   
  F9=Swap      F12=Cancel                                              
Figure 244. Reset statistics panel (ISRURSET)
For more information about how the Reset statistics option works, see “Reset ISPF statistics utility
(option 3.5)” on page 163. You can set these items from this window:
Options
Select 1 to Reset ISPF statistics, or 2 to Delete ISPF statistics.
New Userid
Sets the ID field in the statistics. If you want to change the ID the statistics are kept under, enter the
new ID here. If you do not specify a new version number, this field is required to be filled in.
New Version
Enter a number here is you want to change the version number. This field is required if you do not
enter a new userid. It is ignored if you have chosen the delete action.
New Mod
Enter a number here to change the version number.
TSO command
Figure 245 on page 416 shows the pop-up prompt window that appears when you choose Command,
from the File action bar choice, then select TSO Command from the Command Action prompt panel.
   Menu  Functions  Confirm  Utilities  Help
┌───────────────────────────────────────────────────────────────────────────────┐
│                             TSO Command Action                               │
│                                                                              │
│ The "/" character can be used within the command string to represent the     │
│ following fully qualified and quoted data set name:                          │
│ 'MYPROJ.DEV.SOURCE(TEST)'                                                    │
│                                                                              │
│ Enter TSO Command and any additional parameters as needed:                   │
│                                                                              │
│                                                                              │
│                                                                              │
│ Press ENTER to execute command, press CANCEL to cancel action.               │
│  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward      │
│  F9=Swap       F12=Cancel                                                    │
⋘───────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 245. TSO command panel (ISRUTCES)
You can set these items from this window:
Workplace (Option 11)
416  z/OS: z/OS ISPF User's Guide Vol II

## Page 455

TSO Command
The name of the TSO command you want to use. The command name can be followed by command
parameters with the / character appearing anywhere within the parameter string.
ISPF command shell
Figure 246 on page 417 shows the pop-up prompt window that appears when you enter the ISPF
command shell. To get to this window, choose Command from the File action bar choice, then choose
ISPF Command Shell on the Command Action prompt panel.
Figure 246. ISPF Command Shell panel (ISRTSO)
The ISPF Command Shell option enables you to run TSO commands, CLISTs, and REXX execs under ISPF.
This panel has one input field. Type the command and its parameters into the input field, leaving at least
one space between the command name and the first parameter. The input field continues for two full
lines below the start of the input field. The maximum number of characters that you can enter is 234. For
example:
Enter TSO commands below
===> SEND 'THIS MESSAGE DEMONSTRATES THAT A TSO COMMAND ENTERED UNDER
ISPF CAN EXCEED ONE LINE ON THE 3270' USER(ALICE)
You can also enter ISPF commands, such as END or RETURN, in this field.
Note: If you enter HELP or CANCEL, it is interpreted as the ISPF Help or Cancel command. To issue TSO
Help, enter:
===> TSO HELP xxxxx
To issue TSO Cancel, enter:
===> TSO CANCEL xxxxx
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  417

## Page 456

The ISPF command shell option enables you to enter most TSO commands under ISPF. Here is a list of
commands that are not supported:
• LOGON
• LOGOFF
• ISPSTART, PDF, and ISPF
• TEST
• Commands that you are restricted from using by TSO
• Commands requiring large parameter lists (234 characters is the maximum allowed, including
command name)
You can run command procedures under ISPF, subject to these restrictions:
• CLISTs and execs must not invoke restricted commands listed previously.
• TERMIN command procedure statements are not supported.
These restrictions also apply to commands entered from other panels.
After you type a command in the input field, press ENTER to start the command. If you are not operating
in Session Manager mode, the cursor is positioned below the command input field. Line-at-a-time I/O
from the command, if any, starts at the cursor position. When the command finishes, three asterisks (***)
may appear on the screen. To return to ISPF full-screen mode, press ENTER.
The ISPF command shell panel is then redisplayed with the command you entered displayed in the
command list (unless you entered the TSO prefix, or List mode is set to update off).
Move or copy
Figure 247 on page 418 shows the pop-up prompt window that appears when you choose Move from the
File action bar, after you choose a member to work with. The panel that appears when you choose Copy is
similar to this one.
   RefList  Help
 ───────────────────────────────────────────────────────────────────────────────
                                MOVE Entry Panel
                                                                    More:     +
 CURRENT from data set: 'MYPROJ.DEV.SOURCE(TEST)'
 To Library                       Options:
    Project . . . MYPROJ             Enter "/" to select option
    Group . . . . DEV                _  Replace like-named members
    Type  . . . . SOURCE             /  Process member aliases
 To Other Data Set Name
    Data Set Name . . . _____________________________________________
    Volume Serial . . . ______    (If not cataloged)
 NEW member name  . . . ________  (Blank unless member to be renamed)
 Options
    Sequential Disposition        Pack Option         SCLM Setting
    2  1. Mod                     1  1. Default       3  1. SCLM
       2. Old                        2. Pack             2. Non-SCLM
 Command ===> ________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 247. Move panel (ISRUMVC)
For more information about the Move/Copy utility, see “Move/Copy utility (option 3.3)” on page 119. You
can set these items from this window:
To Library
The library to which you want to move or copy the selected data.
To Other Data Set Name
The data set to which you want to move or copy the selected data.
Workplace (Option 11)
418  z/OS: z/OS ISPF User's Guide Vol II

## Page 457

NEW member name
If the "To" and "From" data sets are the same, you can rename the member here.
Replace like-named members
Select this option to allow replacement of a member in the "To" data set with a like-named member in
the "From" data set.
Process member aliases
Select this option to allow the primary member and all alias members to be moved together.
Sequential disposition
Select 1 if Mod, 2 if Old
1
Mod adds new data at the end of data currently contained in the data set.
2
Old begins placing new data at the beginning of the data set, writing over existing data.
Pack option
Indicates how you want the data to be stored in the "To" data set.
1
Data set is packed according to your default settings.
2
Data set is packed.
SCLM setting
Indicates how you want the data to be stored in the "To" data set.
1
SCLM
2
Non-SCLM
3
As is
Rename
Figure 248 on page 419 shows the pop-up prompt window that appears when you choose Rename from
the File action bar, after you choose a member to work with.
   File  View  Options  Space  SuperC  Test  Help
 ─ ┌───────────────────────────────────────────────┐ ──────────────────────────
   │        Workplace Rename Action Prompt         │
   │                                               │
 D │ Rename  . .    1. Data Set                    │
   │                2. Member                      │
   │                                               │ re a volume serial)
   │ Select a choice and press ENTER to continue   │
 ┌ │                                               │ lection ─────────────────┐
 │ │  F1=Help        F2=Split       F3=Exit        │  DL=DSLIST  /=Open List  │
 │ │  F7=Backward    F8=Forward     F9=Swap        │ ta sets   -------- ----- │
 │ ⋘───────────────────────────────────────────────┘           02/10/01 12:06 │
 │  TEST2    . . .              Second Test List               02/10/01 12:05 │
 ************************* End of ISPF Referral lists *************************
 Action ===> RP                                                Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F4=Settings  F5=PrvObj    F6=PrvAct
  F7=Backward  F8=Forward   F9=Swap     F10=Actions  F11=ChgView  F12=Cancel
Figure 248. Rename prompt panel (ISRURNAM)
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  419

## Page 458

You choose to rename either a data set or a member from this panel. If you choose data set, the panel in
Figure 249 on page 420 appears.
   ┌─────────────────────────────────────────────────────────────────────────┐
 ─ │                            Rename Data Set                              │
 I │                                                                         │
   │ Data Set Name . . : MYPROJ.DEV.SOURCE                                   │
 D │ Volume Serial . . : MVS8WF                                              │
   │                                                                         │
   │ Enter new name below:  (The data set will be recataloged.)              │
   │                                                                         │
 ┌ │ ISPF Library:                                                           │
 │ │    Project  . .                                                         │
 │ │    Group  . . .                                                         │
 │ │    Type . . . . SOURCE                                                  │
 │ │                                                                         │
 * │ Other Partitioned or Sequential Data Set:                               │
   │    Data Set Name . . . 'MYPROJ.DEV.SOURCE'                              │
   │                                                                         │
   │                                                                         │
   │                                                                         │
   │                                                                         │
   │                                                                         │
   │ Command ===>                                                            │
 A │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward     │
   │  F9=Swap      F10=Actions   F12=Cancel                                  │
   ⋘─────────────────────────────────────────────────────────────────────────┘
Figure 249. Rename data set panel (ISRUARP1)
You can set these items from this window:
New name
The name that you want to use for the renamed data set.
If you choose member, the panel in Figure 250 on page 420 appears.
   Menu  Functions  Confirm  Utilities  Help
 ─ ┌─────────────────────────────────────┐ ────────────────────────────────────
 I │           Member Rename             │                   Row 00001 of 00001
   │                                     │ d           Changed            ID
 S │ Enter a new member name:            │ /08   2002/07/08 13:32:15    GRAHAMP
   │                                     │
   │ Old Name  . . : TEST                │
   │                                     │
   │ New Name  . . .                     │
   │                                     │
   │                                     │
   │ Press ENTER to rename member.       │
   │ Press CANCEL to cancel rename.      │
   │                                     │
   │  F1=Help           F2=Split         │
   │  F3=Exit           F7=Backward      │
   ⋘─────────────────────────────────────┘
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 250. Rename member panel (ISRUREN)
You can set these items from this window:
New name
The name that you want to use for the renamed member.
Commands
You can use primary commands in the command area (Action line) of the Workplace entry panels.
Workplace (Option 11)
420  z/OS: z/OS ISPF User's Guide Vol II

## Page 459

Table 27. Workplace commands
Command Description Valid For:
A Allocate Data sets
ACTBAR or NOACTBAR Display or do not display action bar on panel Action prompt
AP Allocate Action prompt
B Browse Members and non-PDS data sets
C Copy Members and non-PDS data sets
COLOR Global color change Action prompt
CP Command Action prompt
CUAATTR CUA attributes Action prompt
D Delete Members and non-PDS data sets
DF Delete Data sets
DL DSLIST Data set name level
DP Delete Action prompt
DVT VTOC summary Data summary
E Edit Members and non-PDS data sets
EP Edit Action prompt
G Reset member statistics Members
I Full information Data sets
ICS ISPF command shell Action prompt
ICT ISPF command table Action prompt
IP Information Action prompt
J Submit Members and non-PDS data sets
K Catalog Data sets
KEYLIST Keylist utility Action prompt
L Print data set Data sets
LOCATE, LOC, or L Find a specified referral list in the scrollable
display of referral lists
Referral lists
LP List Action prompt
LV or LISTVIEW List view Action prompt
M Move Members and non-PDS data sets
ML Member list Partitioned data sets
N Rename Data sets
O Open Members and non-PDS data sets
OPD Personal data set lists Referral lists
OPL Personal library lists Referral lists
P Print Members and non-PDS data sets
PDL Print data set list Data sets
PP Print Action prompt
PSCOLOR Point and shoot Action prompt
PVT Print VTOC information Data sets
Q VSAM Data sets
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  421

## Page 460

Table 27. Workplace commands (continued)
Command Description Valid For:
R Rename Members and non-PDS data sets
RP Rename Action prompt
S Short information Data sets
SC SuperC Data sets
SCE SuperC extended Data sets
SELECT, SEL, or S Select a specified referral list in the scrollable
display of referral lists
Referral lists
SETTINGS ISPF settings Action prompt
SF SearchFor Data sets
SFE SearchFor extended Data sets
T TSO command Members and non-PDS data sets
U Uncatalog Data sets
V View Members and non-PDS data sets
VP View Action prompt
WPSET Workplace settings Action prompt
X Print data set index Data sets
Y Allocate SMS (enhanced) Data sets
Z Compress Data sets
= (equal sign) Repeat last command. If no previous action,
view is the default.
Members and non-PDS data sets
Default CUA function key settings
Table 28 on page 422 shows how the function keys are defined for the main Workplace panel when the
mode is set to keylist ON and function keys are set to primary LOWER.
Table 28. Workplace function key settings
Key Action Description:
F1 Help Workplace help
F2 Split Split screen
F3 Exit Exit Workplace
F4 Settings ISPF Workplace settings
F5 PrvObj Recall last object
F6 PrvAct Repeat last action
F7 Backward Scroll up Reflist
F8 Forward Scroll down Reflist
F9 Swap Swap screen
F10 Actions Cursor to action bar
F11 ChgView Change Workplace view
F12 Cancel Exit Workplace
Workplace (Option 11)
422  z/OS: z/OS ISPF User's Guide Vol II

## Page 461

Table 28. Workplace function key settings (continued)
Key Action Description:
F13 Help Help
F14 Split Split
F15 End End
F16 Return Return
F17 Rfind Repeat find
F18 Rchange Repeat change
F19 Up Up (Scroll up)
F20 Down Down (Scroll down)
F21 Swap Swap
F22 Left Left (Scroll left)
F23 Right Right (Scroll right)
F24 Cretriev Cursor/retrieve
Changing workplace settings
Figure 251 on page 424 shows the pop-up prompt window that appears when you choose the Workplace
Settings pull-down from the Options choice on the Workplace action bar. You can also start this
function by entering WPSET on the command line. The workplace settings determine how your particular
workplace behaves for various functions.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  423

## Page 462

Figure 251. Workplace Settings panel (ISRUSETM)
You can set these items from this window:
Display Edit/View entry panel
When selected, causes the Edit/View prompt panel to appear before you can start an edit or view
action for a sequential data set. If you want to display a member list, the prompt panel is only
displayed if a slash (/) is entered in the Prompt field of the member list.
The default for this setting is selected.
Display Browse entry panel
When selected and you are using DBCS code page, causes the Browse prompt panel to appear before
you can start a browse action for sequential data set. If you are displaying a member list, the prompt
panel is only displayed if a slash is entered in the Prompt field of the member list.
The default for this setting is selected.
Automatically Update reference lists
When selected, specifies that any data set or library, or both, is added to the respective reference list.
The default for this setting is selected.
Update REFLIST with Dsname Level
When selected, specifies that the ISPF Reference List is updated with the Dsname pattern entered in
Object Name.
Keep member field value
When selected, specifies that the member name field for ISPF Library is not to be cleared upon return
from a library action.
Workplace (Option 11)
424  z/OS: z/OS ISPF User's Guide Vol II

## Page 463

The default for this setting is selected.
Member List for M,C,D,G actions
When selected, specifies that the actions Move, Copy, Delete, and Reset result in a member list. When
not selected, these actions act upon all members that match the pattern without displaying a member
list.
The default for this setting is selected.
Show status for M,C,D,G actions
When selected, displays a status panel for the actions Move, Copy, Delete, and Reset. When not
selected, no status panel is displayed.
The default for this setting is selected.
Confirm Member delete
When selected, specifies that the delete confirmation panel is displayed before a member is deleted.
The default for this setting is selected.
Confirm Data Set delete
When selected, specifies that the delete confirmation panel is displayed before a data set is deleted.
The default for this setting is selected.
Show Workplace Action bar
When selected, specifies that the action bar appears on the workplace panels.
The default for this setting is selected.
Frame ISPF Referral list area
When selected, specifies that the ISPF referral list area be framed, using the character specified in the
Reflist Frame Char field.
Smart Action Retrieve Entry
When selected, specifies that ISPF executes the smart action option against the retrieved data set.
Display Catalog Name
When selected, specifies that the Total view of a Data Set List displays the catalog name in which the
data set was located.
Display Total Tracks
When selected, specifies that a Total Tracks header line is displayed on the data set list above the
column headings for the Space and Total view.
View Options
Specifies how to display the data set list.
volume
Displays data set list with a volume view.
space
Displays data set list with a space view.
attrib
Displays data set list with an attribute view.
total
Displays data set list in total view.
Member List View
Specifies how to display the member list
standard
Displays a member list with a 1-character command entry field.
extended
Displays a member list with an 8-character command entry field.
Reflist Frame Char
The character used to frame the ISPF referral list area on your workplace panels.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  425

## Page 464

Workplace Settings panel action bar
These action bar choices appear on the Workplace Settings panel:
File
The file pull-downs give you the options to either cancel or exit the current file.
Defaults
You can choose the default enter or open actions from these pull- downs:
Default Enter action
You can select a default action to perform automatically whenever you do an Enter action. The
available actions are:
• Smart Action
The Smart Action enables ISPF to choose the action needed based on the characteristics of the
object you are using. ISPF chooses the appropriate action according to these rules:
Object type
Action selected by ISPF
ISPF Library
Member list
Partitioned Data Set
Member list
Pattern containing "*" or "%"
Data Set List
Volume (with no object name)
Data Set List
Member Object
User selectable *
Sequential Data Set
User selectable *
* Use the Smart Action action bar choice to select the action for member objects and sequential
data sets.
• Member List
• Data Set List
• Edit
• View
• Browse
• Rename member
• Move
• Copy
• Reset Stats
• Open
• Repeat action
Default Open action
You can select a default action to perform automatically whenever you do an Open action from the
Workplace or workplace member lists. The available actions are:
• User command (a user defined command)
• Edit
• View
• Browse
Workplace (Option 11)
426  z/OS: z/OS ISPF User's Guide Vol II

## Page 465

• Delete member
• Rename member
• Move
• Copy
• Reset member
• Print member
• TSO Cmd
Colors
You can choose the colors for the member list or the data set list from this action bar.
Help
Provides general workplace settings help, and default enter and open help.
Workplace example scenario
The scenario here illustrates some of the advantages provided by the ISPF Workplace function. To provide
you with a reference point of view, the scenario includes points on how you can accomplish the same task
using ISPF in the traditional way.
For this example, say that your task is to:
1. Copy a sequential data set into a member of a concatenated ISPF Library.
2. SuperC compare it to another member.
3. Rename the member.
4. Change the Version number in the ISPF statistics.
Subtask 1
Your first step is to copy a sequential data set into a member of a concatenated PDS.
Traditional ISPF
Use the 3.3 Move/Copy Utility.
Workplace
Use the Copy Action against the sequential data set object.
Choose the Workplace option (Option 11) on the main menu. Use the PF11 key to toggle to the data set
view. In the Workplace you have a choice of working from a data set list or issuing commands against a
single data set.
If you are list-oriented you can specify a wildcard character in the Object Name field (such as, 'USERID.*')
to generate a data set list containing the sequential data set.
If you prefer to specify the sequential data set directly you can type it into the Object Name field either
with or without single quotes (that is, SEQ.FILE or 'USERID.SEQ.FILE').
In either case these accelerated methods can be alternatives to remembering and typing the input:
• You might be able to retrieve a recently referenced data set name or pattern from the REFLIST in the
bottom half of the Workplace.
• You might be able to retrieve a recently referenced data set name or pattern using the recall key PF5.
• You might be able to retrieve a data set name or pattern from a personal list you previously created.
These also appear in the bottom half of the Workplace.
Now that the Object has been specified you must specify the Action. In this example, the action is COPY.
You can do this several ways, depending on your preferences.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  427

## Page 466

• If you are in a list, you can use the CO line command to copy the data set, or you can put a slash (/) in
the line command field to be prompted with a list of available commands to select.
• If you specified the "from" data set directly (not from a list) you can use the Copy option from the File
action bar choice, or you can type the C fast path command in the Action ==> field to copy the data set.
In either case, a pop-up panel prompts you for the target data set, member name, and other parameters.
Subtask 2
The second step is to SuperC compare one member of a concatenated PDS to another.
Traditional ISPF
Use the 3.13 SuperC compare utility.
Workplace
Use the Workplace-to-SuperC Interface.
Using PF11, toggle back to the Workplace ISPF Library View.
Specify the ISPF library concatenation and member name of the new member you just created by the
COPY action. These accelerated methods can be used as alternatives to remembering and typing the
input:
• You might be able to retrieve a recently referenced ISPF Library concatenation from the REFLIST in the
bottom half of the Workplace.
• You might be able to retrieve a recently referenced ISPF Library concatenation using the recall key PF5.
• You might be able to retrieve an ISPF Library concatenation from a personal list you previously created.
These also appear in the bottom half of the Workplace.
Now that the Object has been specified you must specify the Action. The action at this point is SuperCE.
Again, specifying this action can be done several ways depending on your preferences.
• You can use the SuperCE option from the SuperC action bar.
• You can type the "SCE" fast path command in the Action ==> field and press Enter.
In either case, Workplace enters the SuperCE dialog. Note that your ISPF Library concatenation is
transferred to the correct New DSN fields in the SuperCE concatenation panel, so you do not have to
type it yourself.
After running your compare, exit the SuperC Utility to return to Workplace.
Subtask 3
The next step is to rename a member of an ISPF Library.
Traditional ISPF
Use the 3.1 Library Utility.
Workplace
Use the Rename member Action.
The ISPF library concatenation and member name of the new member you just compared remains on the
Workplace panel. Now you must specify the Rename Action.
How do you prefer to do this?
• You can use the Rename option from the File action bar.
• You can type the "R" fast path command in the Action ==> field, then press Enter.
Workplace (Option 11)
428  z/OS: z/OS ISPF User's Guide Vol II

## Page 467

• You can work from a member list and issue the "R" line command to rename the member.
Member lists can be created a number of ways in Workplace:
• Just press Enter if your default enter action is Smart Action, a mode that analyzes the object and selects
an appropriate action. Select the Workplace Settings option from the Options action bar to view or
change your defaults.
• Enter the List option from the File action bar.
• Enter the "ML" fast path command.
In any case, Workplace displays a pop-up panel to prompt you for the new member name.
Subtask 4
The final step in this scenario is to change a member's Version number in the ISPF statistics.
Traditional ISPF
Use the 3.5 Reset Statistics Utility.
Workplace
Use the Reset Action.
The ISPF library concatenation and member name of the new member you just compared remains on the
Workplace panel. Now you must specify the Reset Action.
Again you have a choice about how to do this:
• You can use the Reset option from the File action bar.
• You can type the "G" fast path command in the Action ==> field and press Enter.
• You can work from a member list and issue the "G" line command to rename the member or specify the
"/" line command to be prompted with an action selection list.
In all cases, Workplace displays a pop-up panel to prompt you for Reset parameters.
Workplace (Option 11)
Chapter 12. ISPF object/action workplace (option 11)  429

## Page 468

Workplace (Option 11)
430  z/OS: z/OS ISPF User's Guide Vol II
