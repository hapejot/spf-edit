# Chapter 5. Using personal data set lists and library lists

Source file: f54ug00_v3r1.md
Start page: 143
Page span: 143-166

## Page 143

Chapter 5. Using personal data set lists and library
lists
Personal lists are named lists of data sets, z/OS UNIX files and ISPF libraries that you can use to speed
up access to frequently used data sets. You can use personal lists to fill in panel fields quickly and to
create data set lists that are built from more than one level name. Personal data set lists contain data
set names, volumes, and z/OS UNIX files. Personal library lists contain lists of ISPF library names and
concatenations.
Reference lists are active lists of data sets, z/OS UNIX files, and libraries that you have referenced in your
ISPF session. ISPF adds a data set name to the data set reference list when you enter a data set name in
the Other Partitioned or Sequential Data Set Name field. ISPF also adds a z/OS UNIX file pathname to the
data set reference list when you enter a pathname in the Other Partitioned or Sequential Data Set, or z/OS
UNIX File Name field. A library is added to the library reference list when you enter a library in the ISPF
Library field. Only data sets and libraries that are successfully allocated by ISPF's ALLOCATE routine are
added to the reference lists.
Note: Reference lists can be manipulated just like any other personal list, but ISPF might dynamically
change reference lists when new data sets or libraries are referenced by ISPF.
You can have a personal data set list with the same name as a personal library list. ISPF reserves the
name REFLIST as the name of the reference lists, so there is a personal data set list called REFLIST, and a
personal library list called REFLIST.
Current lists are the most recently opened or the last list to which something was saved from within the
Personal List panels. One named data set list and one named library list are the current lists at any time.
The current list is used for the NRETRIEV key and in the RefList pull-downs. The current list names are
shown in the RefList pull-down choices, and in the lists of personal lists.
Personal lists
ISPF provides four types of personal lists:
Personal data set list
Lists of up to 30 data set names and z/OS UNIX files. For data sets, each name can include a member
name or a volume name, or both. z/OS UNIX file path names can be for regular files, directories, or
symbolic links to directories or regular files. Personal data set lists can also contain data set name
levels. See “Personal data set lists” on page 116.
Reference data set list
A special kind of personal data set list in which ISPF saves the names of the most recently used data
sets, data set name levels, and z/OS UNIX files and directories. This list is always named REFLIST. See
Reference data set lists.
Personal library list
Lists of up to 8 ISPF library names or ISPF library concatenations. ISPF library names contain three
qualifiers called project, group, and type. Personal library list entries can optionally contain a member
name. See “Personal library lists” on page 118.
Reference library list
A special kind of personal library list in which ISPF saves the names of the most recently used ISPF
library qualifiers (project, group 1, group 2, group 3, group 3, type, and member). This list is always
named REFLIST. See Reference library lists.
You can access personal lists from the RefList action bar choice on most panels that input library or data
set names.
© Copyright IBM Corp. 1980, 2024 115

## Page 144

Figure 29. RefList Pull-Down Menu
Note: The RefList pull-down is unavailable from Utilities options 8, 9, 11, 13, and 15; it does not offer
library list choices from Utilities options 4 and 6.
You can also access personal lists from the Workplace.
Personal data set lists
You can build lists of personal data sets that contain up to 30 data set names and z/OS UNIX file path
names. You can have as many lists as you like as long as each has a unique name. Personal data set lists
are a good way to group (by project, for example) those data sets and z/OS UNIX file path names that you
use frequently. You can use personal data set lists to avoid typing in data set names and z/OS UNIX file
path names and to create customized data set lists like those using ISPF Option 3.4.
For example, you might have a personal data set list that has all the data sets you need to build Dialog Tag
Language panels.
116  z/OS: z/OS ISPF User's Guide Vol I

## Page 145

How to create a personal data set list
There are several ways to create personal data set lists:
• Type data set names into an empty list.
1. Select the New choice from the File pull-down on the Personal Data Set List panel action bar or use
the NEW primary command to display an empty temporary list.
2. Type in the data set names.
3. Save the list.
• Use the reference data set list as a starting point.
1. Display the reference data set list by selecting the Reference Data Set List choice from the RefList
pull-down on the View Entry, Edit Entry, or Utilities panel action bar.
2. If you want to save this entire list as a personal data set list, select the Save as... choice from the File
pull-down on the Reference Data Set List panel action bar.
3. If you want to save some subset of the reference data set list, see the next item for more
information.
• Use an existing list as a base, edit it, and save it.
1. Display an existing list.
2. Modify this list by typing over data set names or adding new ones. Do not select any data sets.
3. Select the Save choice from the File pull-down on the Personal Data Set List panel action bar.
Note: Closing the list display by pressing Exit or End will Save the list. It is possible to have a single
list open on multiple screens. Therefore, it is recommended that a list only be open on one screen if
modifications are to be made. This will prevent the loss of updates when an unchanged list display is
closed after a modified one.
• Use an existing list as a base and save it under another name.
1. Display an existing list.
2. Modify this list by typing over data set names or adding new ones. Do not select a choice.
3. Select the Save as... choice from the File pull-down on the Personal Data Set List panel action bar
and assign a unique name to this list.
4. ISPF displays the new list.
• Issue the REFADDD command and specify a new list name. Issuing REFADDD NEWLIST from the
command line creates a new personal data set list called NEWLIST. The list will contain the last
referenced data set name.
How to retrieve a data set from a personal data set list
You have three choices for retrieving a data set name from a personal list.
• Use the NRETRIEV function key. Assign a function key to the value "NRETRIEV". On panels where
NRETRIEV is available (such as edit, view, and some of the utilities), pressing the NRETRIEV key fills in
the data set name field based on where the cursor is when the key is pressed. See “Command interface
to the personal list function” on page 134 for more information.
• Use the RefList pull-down on the View Entry, Edit Entry, or Utilities panel action bar. From either the
current personal list (option 1), or one of the personal data sets lists in the list of personal data set
lists (option 3), you can point to a data set name and press Enter to retrieve or use the name that you
selected. See “Personal list modes” on page 131 for more information.
• Use the REFACTD described in “Command interface to the personal list function” on page 134.
Reference data set list - REFLIST (Last 30 referenced data sets)
The Reference Data Set List is a special personal data set list that contains a list of up to 30 data set
names (and the volumes on which they are located), data set level names, and z/OS UNIX file path names
Chapter 5. Using personal data set lists and library lists  117

## Page 146

that you have referenced (that is, entered on panels or called with services) throughout ISPF. Data set
names are also added to the list when ISPF refers to them, such as during a MOVE/COPY operation or a
DELETE function. ISPF adds z/OS UNIX file path names to the Reference Data Set List whenever they are
referenced in Edit, Browse, or View, or on the z/OS UNIX Directory List Utility entry panel. Data Set Level
names are added when entered on the Data Set List utility (option 3.4), ISPF Workplace (option 11), or as
a parameter on the command DSLIST.
The Reference Data Set List is a personal data set list with the name REFLIST. The name REFLIST is
reserved by ISPF to refer to the reference list, but you can use the list just like any other list. If you save
a personal list under the name REFLIST, the reference list reflects the names you save into it, but it is still
updated when other data set are referenced by ISPF.
Personal library lists
You can build personal library lists of up to 8 ISPF libraries. You can have as many lists as you like as long
as each has a unique name. Personal library lists are a good way to group (by project, for example) those
libraries that you use frequently.
For example, if you are on a team that is developing COBOL programs, you can have a personal library list
to include the library hierarchy concatenation for each of the types you use frequently.
Note: Personal library lists are not available from RefList pull-downs for any options that do not support
library names. For example, the data set list utility and data set print utilities do not support personal
library lists.
How to create a personal library list
There are several ways to create personal library lists:
• Type library names into an empty list.
1. Select the New choice from the File pull-down on the Personal Library List panel action bar or use
the NEW primary command to display an empty temporary list.
2. Type in the library names.
3. Save the list.
• Use the reference library list as a starting point.
1. Display the reference library list by selecting the Reference Library List choice from the RefList
pull-down on the View Entry, Edit Entry, or Utilities panel action bar.
118  z/OS: z/OS ISPF User's Guide Vol I

## Page 147

2. If you want to save this entire list as a personal library list, select the Save as... choice from the File
pull-down on the Reference Library List panel action bar.
3. If you want to save some subset of the reference library list, see the next item for more information.
• Use an existing list as a base, edit it, and save it.
1. Display an existing list.
2. Modify this list by typing over library names or adding new ones. Do not select any libraries.
3. Select the Save choice from the File pull-down on the Personal Library List panel action bar.
Note: Closing the list display by pressing Exit or End will Save the list. It is possible to have a single
list open on multiple screens. Therefore, it is recommended that a list only be open on one screen if
modifications are to be made. This will prevent the loss of updates when an unchanged list display is
closed after a modified one.
• Use an existing list as a base and save it under another name.
1. Display an existing list.
2. Modify this list by typing over library names or adding new ones. Do not select a choice.
3. Select the Save as... choice from the File pull-down on the Personal Library List panel action bar and
assign a unique name to this list.
4. ISPF displays the new list.
• Issue the REFADDL command and specify a new list name. Issuing REFADDL NEWLIST from the
command line creates a new personal library list called NEWLIST. The list will contain the last
referenced library specification.
How to retrieve a library from a personal library list
You have three choices for retrieving a data set name from a personal list.
• Use the NRETRIEV function key. Assign a function key to the value "NRETRIEV". On panels where
NRETRIEV is available (such as edit, view, and some of the utilities), pressing the NRETRIEV key fills in
the library name fields based on where the cursor is when the key is pressed. See “Command interface
to the personal list function” on page 134 for more information.
• Use the RefList pull-down on the View Entry, Edit Entry, or Utilities panel action bar. From either the
current personal list (option 2), or one of the personal library lists in the list of personal library lists
(option 4), you can point to a library name and press Enter to retrieve the name you selected. See
“Personal list modes” on page 131 for more information.
• Use the REFACTL described in “Command interface to the personal list function” on page 134.
Reference library list - REFLIST (Last 8 referenced libraries)
The reference library list is a special personal library list named REFLIST that contains a list of up to
8 library names that you have referenced through panels or ISPF services. The reference library list is
updated by the system when ISPF uses ISPF libraries. In all other respects, it functions like a regular
personal library list.
Personal list settings
You can control the personal lists by using the Personal List Settings choice from the Options action bar
choice on any personal list.
Chapter 5. Using personal data set lists and library lists  119

## Page 148

From the Personal List Settings panel, you can select any of the general options:
• Automatically update reference list
• Update REFLIST with Dsname Level
• Use selection immediately in VIEW
• Use selection immediately in EDIT
• Use selection immediately in DSLIST
• NRETRIEV verifies data set exists
• Display catalog name in Total view
• Display Total Tracks
You can specify whether to use a Data Set List View of Volume, Space, Attrib, or Total, just as you can from
the Data Set List Utility.
In addition, you can also control whether the reference list is automatically updated from the Workplace
Settings pull-down choice from the Options action bar choice on the ISPF Workplace (option 11), or the
DSLIST Settings pull-down choice from the Options action bar choice of the Data Set List utility (option
3.4).
How to get a list of your personal lists
To see a list of your personal lists, perform one of following actions:
• Select the List of Personal Data Set Lists or the List of Personal Library Lists choice from the RefList
pull-down on the Edit Entry, View Entry, or Utilities panel action bar.
• Select the Open List of Lists choice from the File pull-down on the Personal Data Set List or Personal
Library List panel action bar.
• Type MORE on the command line of the Personal Data Set List panel or the Personal Library List panel.
• Enter the REFOPEND (for data set lists) or REFOPENL (for library lists) command on any ISPF command
line.
How to use personal lists to create customized DSLIST displays
You can use any personal data set list or personal library list to create a data set list (similar to ISPF
option 3.4) which contains multiple level names. The easiest way to use a Personal Data Set List to create
a DSLIST is to type DSLIST listname on an ISPF command line, where listname is the name of the
personal data set list. If you are already viewing a DSLIST, you can add names listed in a personal data set
list by typing APPEND listname on the command line.
You can also create a list with the L action while displaying a personal list or a list of personal lists.
For example, you can create a personal data set list called Command that contains the data sets you use
to hold REXX and CLIST.
120  z/OS: z/OS ISPF User's Guide Vol I

## Page 149

Then, you can use the command DSLIST COMMAND to display a data set list with these three data sets:
How to use personal lists to create customized z/OS UNIX list displays
You can use any personal data set list to create complex lists of z/OS UNIX files and directories, similar to
those displayed using the z/OS UNIX Directory List Utility (option 3.17). The easiest way to use a personal
data set list to create a z/OS UNIX list is to type UDLIST listname on an ISPF command line, where
listname is the name of the personal data set list. ISPF uses each pathname entry in the personal list to
build the displayed list of files and directories.
For example, you can create a personal data set list called Gateway to hold commonly used files for the
ISPF gateway.
Then, you can use the UDLIST GATEWAY command to display a Unix Directory List with these files.
Note: A pathname containing glob characters is permitted in a personal list.
Personal Data Set List panel
Use the Personal Data Set List panel to save, delete, or edit a list. You can also create a data set list or a
UNIX directory list from the data sets or files on the panel.
Chapter 5. Using personal data set lists and library lists  121

## Page 150

You can work with your personal data set lists in three ways:
• Use the choices in the File, View, or Options pull-downs.
• Select one of the point-and-shoot options (for example, Save As).
• Type an action mnemonic in the Action field and press Enter. Actions are listed at the top of the panel.
There are two primary commands that you can use on this panel:
MORE
Displays the list of all your personal data set lists. This is the same action as selecting Open List of
Lists from the File pull-down.
NEW
Saves the current list and displays a new list with the data sets from the previous list. This is the same
action as selecting New List from the File pull-down.
Personal Data Set List panel action bar choices
The Personal Data Set List panel action bar choices function as follows:
File
The File pull-down offers you the following choices:
1 - New List
Displays a temporary personal data set list. After you save this list, it is permanent until you delete
it.
2 - Open List of Lists
Displays a list of your personal data set lists. You can open a list to change it and make it the
current active list.
3 - Save
Saves the current contents of a personal data set list.
4 - Save as...
Saves the current contents to a new personal data set list. You are prompted for a list name and
optional description.
5 - Delete
Deletes the current personal data set list. You are not asked to confirm the delete action. After the
current list is deleted, ISPF displays an empty personal data set list as if you had requested a New
action.
6 - Edit
Enters the personal list edit dialog.
7 - DSLIST
Builds a DSLIST based on list entries.
8 - UDLIST
Builds a z/OS UNIX directory list based on list entries.
9 - Cancel
Cancels the function.
10 - Exit
Returns you to the panel from which you accessed the personal list.
View
The View pull-down offers you the following choices:
1 – Standard view
The list contains the data set entries.
2 – Extended view
The list contains the data set entries with notes.
122  z/OS: z/OS ISPF User's Guide Vol I

## Page 151

3 - Sort by data set name
The data set list is sorted by the Data Set Name field. The sort routine deletes duplicate names,
sorts blank entries to the end of the list, saves the list, and redisplays the personal list.
Note: If a volume or entry description exists without a data set field entry, the volume and
description are deleted by the sort routine.
4 - Sort by data set volume
The data set list is sorted by data set volume field. The sort routine deletes duplicate names, sorts
blank entries to the end of the list, saves the list, and redisplays the personal list.
Note: If a volume or entry description exists without a data set field entry, the volume and
description are deleted by the sort routine.
5 - Sort by data set note
The data set list is sorted by data set note field. The sort routine deletes duplicate names, sorts
blank entries to the end of the list, saves the list, and redisplays the personal list.
Note: If a volume or entry description exists without a data set field entry, the volume and
description are deleted by the sort routine.
Options
The Options pull-down offers you the following choices:
1 - Personal List Settings
Displays the Personal List Settings panel. From this panel, you can alter all settings that affect
personal lists. See “Personal list settings” on page 119 for additional information.
2 - Browse shared lists
Displays shared personal data set lists. See “Shared personal lists” on page 131 for additional
information.
Help
The Help pull-down provides access to the online tutorial.
Personal Data Set List panel fields
The fields on the Personal Data Set List Panel function as follows:
Action
These choices are valid in the Action field:
Note: The dots in the Action field are point-and-shoot selectable. If you select a data set name, and
you started this dialog from a panel that supports the RefList pull-down, ISPF retrieves the selected
data set, terminates this panel, and places the name that you selected in the ISPF Other Data Set
Name field. If you have the RefMode set to "List Execute", ISPF also simulates pressing the Enter key
on the panel.
S=Save
Saves the current list. If the list is new, you are prompted to enter a name for the list.
A=Save as
Saves the current list with a different list name.
D=Delete this list
Deletes the personal data set list that you are working with. You are asked to confirm this delete
action. The currently active list cannot be deleted.
E=Extended edit
Starts the personal list editor dialog. The editor enables you to insert, repeat, and delete lines in
the list. You can also add or change the notes for the data sets.
L=DSLIST
Starts DSLIST based on list entries.
U=UDLIST
Displays a z/OS UNIX directory list based on the list entries.
Chapter 5. Using personal data set lists and library lists  123

## Page 152

Name
The name of the personal data set list.
Description
A brief description of the personal data set list. The Description field is an input field. You can change
the description for all personal lists except the reference list (REFLIST).
Created
The date the personal data set list was created.
Referenced
The last date/time the personal data set list was referenced.
Personal Library List panel
Use the Personal Library List panel to save, delete, or edit a list. You can also create a data set list from
the libraries on the panel.
You can work with your personal library lists in three ways:
• Use the choices in the File, View, or Options pull-downs.
• Select one of the point-and-shoot options (for example, Save As).
• Type an action mnemonic in the Action field and press Enter. Actions are listed at the top of the panel.
There are two primary commands that you can use on this panel:
MORE
Displays the list of all your personal library lists. This is the same action as selecting Open List of Lists
from the File pull-down.
NEW
Saves the current list and displays a new list with the libraries from the previous list. This is the same
action as selecting New List from the File pull-down.
Personal Library List panel action bar choices
The Personal Library List panel action bar choices function as follows:
File
The File pull-down offers you these choices:
1 - New List
Displays a temporary personal library list. After you save this list, it is permanent until you delete
it.
124  z/OS: z/OS ISPF User's Guide Vol I

## Page 153

2 - Open List of Lists…
Displays a list of your personal library lists. You can open a list to change it and make it the current
active list.
3 - Save
Saves the current contents of a personal library list.
4 - Save as...
Saves the current contents to a new personal library list. You are prompted for a list name and
optional description.
5 - Delete
Deletes the current personal library list. You are not asked to confirm the delete action.
6 - Edit
Enters the personal list edit dialog.
7 - DSLIST
Builds a DSLIST based on list entries.
8 - Cancel
Cancels the function.
9 - Exit
Returns you to the panel from which you accessed the personal list.
View
The View pull-down offers you these choices:
1 - By libraries
The list contains the library entries.
2 - By libraries and notes
The list contains the library entries with notes.
Options
The Options pull-down offers you these choices:
1 - Personal List Settings
Displays the Personal List Settings panel. From this panel, you can alter all settings that affect
personal lists. See “Personal list settings” on page 119 for additional information.
2 - Browse shared lists
Displays shared personal data set lists. See Shared personal lists for additional information.
Help
The Help pull-down provides access to the online tutorial.
Personal Library List panel fields
The fields on the Personal Library List panel function as follows:
Action
These choices are valid in the Action field:
Note: The dots in the Action field are point-and-shoot selectable. If you select a library name, and
you invoked this dialog from a panel that supports the RefList pull-down, ISPF retrieves the selected
library name, terminates this panel, and places the name that you selected in the ISPF library field. If
you have the RefMode set to "List Execute", ISPF also simulates pressing the Enter key on the panel.
S=Save
Saves the current list. If the list is new, you are prompted to enter a name for the list.
A=Save as
Saves the current list with a different list name.
D=Delete this list
Deletes the personal library list that you are working with. You are asked to confirm this delete
action. The currently active list cannot be deleted.
Chapter 5. Using personal data set lists and library lists  125

## Page 154

E=Extended edit
Invokes the personal list editor dialog. This enables you to insert, repeat, and delete lines in the
list. You can also add or change the notes for the libraries.
L=DSLIST
Invokes DSLIST based on list entries.
Name
The name of the personal data set list.
Description
A brief description of the personal library list. The Description field is an input field. You can change
the description for all personal lists except the reference list (REFLIST).
Created
The date the personal library list was created.
Referenced
The last date/time the personal library list was referenced.
Personal Data Set Lists panel
The Personal Data Set Lists panel shows a list of your personal data set lists. You can show the Personal
Data Set Lists panel by selecting it from the RefList pull-down or by using the REFOPEND command.
Personal Data Set Lists panel action bar choices
The Personal Data Set Lists Panel action bar choices function as follows:
Note: The current setting is shown as an unavailable choice; that is, it displays in blue (the default) with
an asterisk as the first digit of the selection number.
File
The File pull-down offers you these choices:
1 - New List
Displays a new personal list.
2 - Open
Displays the entries for the selected list.
3 - Save as...
Saves the selected list to a new list.
126  z/OS: z/OS ISPF User's Guide Vol I

## Page 155

4 - Delete...
Deletes the selected list. You will be asked to confirm the delete action.
5 - Edit
Invokes the personal list edit dialog for the selected list.
6 - DSLIST
Invokes DSLIST based on the entries in the selected list.
7 - UDLIST
Builds a z/OS UNIX directory list based on entries in the selected list.
8 - Exit
Returns you to the panel from which you accessed the Open dialog.
View
The View pull-down offers you these choices:
1 - Standard View
Displays a list of personal lists with list name, list description, and list statistics.
2 - Extended View
Displays a list of personal lists with list name, list description, list statistics, and a partial view of
list entries.
3 - Sort by name
Sorts the displayed list alphabetically by the Name field.
4 - Sort by description
Sorts the displayed list alphabetically by the Description field.
5 - Sort by created
Sorts the displayed list in descending order by the Created field.
6 - Sort by referenced
Sorts the displayed list in descending order by the Referenced field.
Options
The Options pull-down offers you these choices:
1 - Personal List Settings...
Displays the Personal List Settings panel, from which you can alter all settings that affect personal
lists. See “Personal list settings” on page 119 for additional information.
2 - Browse shared lists...
Displays shared personal data set lists. See Shared personal lists for additional information.
Help
The Help pull-down provides access to the online tutorial.
Personal Data Set Lists panel fields
The fields on the Personal Data Set Lists Panel function as follows:
The current (Active) list is indicated to the left of the panel title.
Action
These choices are valid in the Action field:
Note: The dots in the Action field are point-and-shoot selectable. Selecting a list opens the list. This
means that you can open a list by moving the cursor to the action field and pressing Enter.
N=New
Displays an empty (temporary) personal data set list. Once you save this list, it is permanent until
you delete it.
O=Open
Opens the selected list to modify it, perform actions, or selections of data sets.
Chapter 5. Using personal data set lists and library lists  127

## Page 156

A=Save as
Saves the current contents of the selected list to a personal data set list. You will be prompted for
a list name and optional description.
D=Delete
Deletes the selected personal data set list. You will be asked to confirm the delete action. The
currently active list cannot be deleted.
E=Edit
Invokes the personal list editor dialog for the selected personal data set list.
L=DSLIST
Invokes DSLIST based on the entries in the selected personal data set list.
U=UDLIST
Displays a z/OS UNIX directory list based on the entries in the personal data set list.
Name
The name of the personal data set list. The Name field is a point-and-shoot sort field.
Description
A brief description of the personal data set list. The Description field is a point-and-shoot sort field.
Created
The date the personal data set list was created. The Created field is a point-and-shoot sort field.
Referenced
The last date/time the personal data set list was referenced. The Referenced field is a point-and-
shoot sort field.
Note: A personal list is updated whenever a save action is performed against it.
The LOCATE command is supported as follows:
• L xxxxxxxx
• LOC xxxxxxxx
• LOCATE xxxxxxxx
Where: xxxxxxxx is the name of the list that you want to locate.
An asterisk is supported in the last position of the list name. For example, enter LOCATE PRIV* to locate
the list named PRIVATE.
The SELECT command is supported as supported as follows:
• S nnnnnnnn A
• SEL nnnnnnnn A
• SELECT nnnnnnnn A
Where: nnnnnnnn is the name of the list that you want to select, and A is the action to perform. If you do
not enter an action, the list is opened.
An asterisk is supported in the last position of the list name. For example, enter SELECT PRIV* L to
select the list named PRIVATE, with a DSLIST action of "L".
Personal Library Lists panel
The Personal Library Lists panel shows a list of your personal library lists. You can show the Personal
Library Lists panel by selecting it from the RefList pull-down or by using the REFOPENL command.
128  z/OS: z/OS ISPF User's Guide Vol I

## Page 157

Personal Library Lists panel action bar choices
The Personal Library Lists Panel action bar choices function as follows:
Note: The current setting is shown as an unavailable choice; that is, it displays in blue (the default) with
an asterisk as the first digit of the selection number.
File
The File pull-down offers you these choices:
1 - New List
Displays a new personal list.
2 - Open
Displays the entries for the selected list.
3 - Save as...
Saves the selected list to a new list.
4 - Delete...
Deletes the selected list. You will be asked to confirm the delete action.
5 - Edit
Invokes the personal list edit dialog for the selected list.
6 - DSLIST
Invokes DSLIST based on the entries in the selected list.
7 - Exit
Returns you to the panel from which you accessed the Open dialog.
View
The View pull-down offers you these choices:
1 - Standard View
Displays a list of personal lists with list name, list description, and list statistics.
2 - Extended View
Displays a list of personal lists with list name, list description, list statistics, and the first library in
the list.
3 - Sort by name
Sorts the displayed list alphabetically by the Name field.
4 - Sort by description
Sorts the displayed list alphabetically by the Description field.
5 - Sort by created
Sorts the displayed list in descending order by the Created field.
6 - Sort by referenced
Sorts the displayed list in descending order by the Referenced field.
Options
The Options pull-down offers you these choices:
Chapter 5. Using personal data set lists and library lists  129

## Page 158

1 - Personal List Settings...
Displays the Personal List Settings panel, from which you can alter all settings that affect personal
lists. See “Personal list settings” on page 119 for additional information.
2 - Browse shared lists...
Displays shared personal data set lists. See Shared personal lists for additional information.
Help
The Help pull-down provides access to the online tutorial.
Personal Library Lists panel fields
The fields on the Personal Library Lists panel function as follows:
The current (Active) list is indicated to the left of the panel title.
Action
These choices are valid in the Action field:
Note: The dots in the Action field are point-and-shoot selectable. Selecting a list opens the list. This
means that you can open a list by moving the cursor to the action field and pressing Enter.
N=New
Displays an empty (temporary) personal data set list. Once you save this list, it is permanent until
you delete it.
O=Open
Opens the selected list to modify it, perform actions, or selections of data sets.
A=Save as
Saves the current contents of the selected list to a personal data set list. You will be prompted for
a list name and optional description.
D=Delete
Deletes the selected personal data set list. You will be asked to confirm the delete action. The
currently active list cannot be deleted.
E=Edit
Invokes the personal list editor dialog.
L=DSLIST
Invokes DSLIST based on list entries.
Name
The name of the personal library list. The Name field is a point-and-shoot sort field.
Description
A brief description of the personal data set list. The Description field is a point-and-shoot sort field.
Created
The date the personal data set list was created. The Created field is a point-and-shoot sort field.
Referenced
The last date/time the personal data set list was referenced. The Referenced field is a point-and-
shoot sort field.
Note: A personal list is updated whenever a save action is performed against it.
The LOCATE command is supported as follows:
• L xxxxxxxx
• LOC xxxxxxxx
• LOCATE xxxxxxxx
Where: xxxxxxxx is the name of the list that you want to locate.
An asterisk is supported in the last position of the list name. For example, enter LOCATE PRIV* to locate
the list named PRIVATE.
130  z/OS: z/OS ISPF User's Guide Vol I

## Page 159

The SELECT command is supported as supported as follows:
• S nnnnnnnn A
• SEL nnnnnnnn A
• SELECT nnnnnnnn A
Where: nnnnnnnn is the name of the list that you want to select and A is the action to perform. If you do
not enter an action, the list is opened.
An asterisk is supported in the last position of the list name. For example, enter SELECT PRIV* L to
select the list named PRIVATE, with a DSLIST action of "L".
Personal list modes
The action taken when you select a data set or a library from a list depends on how you have the Mode
set. All personal lists can be set to either Retrieve or Execute mode from the RefMode pull-down on the
action bar of the View Entry, Edit Entry, and most Utilities panels, as shown in Figure 30 on page 131.
List Retrieve displays in blue (the default) with an asterisk as the first digit of the selection number, which
indicates that RefMode is currently set to Retrieve.
Figure 30. RefMode Pull-Down Menu
The RefMode pull-down offers you these choices:
List Execute
Sets personal data set lists and personal library lists to Execute mode; that is, when you select an
entry from the list, the information is placed into the ISPF Library or Other Data Set Name field, and
ISPF proceeds as if you also pressed the Enter key.
List Retrieve
Sets personal data set lists and personal library lists to Retrieve mode; that is, when you select an
entry from the list, the information is placed into the ISPF Library or Other Data Set Name field, but
the simulated pressing of the Enter key is not performed. This allows you to set other options before
you press Enter.
Shared personal lists
Personal lists (library and data set) can be shared with other users on the system. Tables ISRPLSHR for
data sets and ISRLLSHR for libraries are shared lists. They are kept in an ISPTLIB concatenated data set.
Chapter 5. Using personal data set lists and library lists  131

## Page 160

Private lists are tables ISRPLIST (for data set lists) and ISRLLIST (for library lists). They are kept in the
ISPPROF user profile data set.
Create a shared list by using the Move/Copy Utility (option 3.3) to copy an existing personal list table from
a user profile data set to a data set concatenated to ISPTLIB. You must rename the table to ISRPLSHR (for
data set lists) or ISRLLSHR (for library lists) during the copy operation.
For example, if you want to share a personal data set list called ISP from your profile data set,
'USER1.PROFILE', use the Move/Copy Utility to copy member ISRPLIST from your profile data set to a
data set in the ISPTLIB concatenation, 'TEAMPROJ.TABLES' setting the new member name to ISRPLSHR.
Then, anyone who also has 'TEAMPROJ.TABLES' in their ISPTLIB concatenation can see all the personal
data set lists you copied from 'USER1.PROFILE'.
To use a shared personal list, use the primary commands REFOPEND (for a data set list) or the REFOPENL
(for a library list). From the Options action bar choice, select 2, Browse shared lists.
You must save the shared list to a personal list before you can retrieve names from it.
These actions are available for shared personal lists:
• Open (to see the entries in the list)
• Save As (to save the contents of the selected list to a new personal list)
You cannot update or delete a shared list.
For example, to retrieve names from the shared list ISP, select the list with the A action. Enter a name and
optionally a description on the Personal Data Set List Save As panel. When you return to your Personal
Data Set Lists, the newly saved list appears. You can use the new list like any other personal list.
Name retrieval with the NRETRIEV command
The ISPF command table contains an entry named NRETRIEV. On enabled panels such as Edit, NRETRIEV
retrieves the library names from the current library referral list, or data set name or z/OS UNIX file name
from the current data set referral list. You are responsible for assigning the NRETRIEV command to a
function key.
When the cursor is not in the Other Data Set Name field or the Volume Serial field, and the NRETRIEV key
is pressed, the ISPF library fields are filled in from the current list. As long as the cursor is not placed in
these fields, subsequent presses of the NRETRIEV key will retrieve the next library concatenation from
the list.
When the cursor is in the Other Data Set Name field or the Volume Serial field, and the NRETRIEV key is
pressed, the data set name or z/OS UNIX file name is filled in from the current data set list. ISPF attempts
to determine if the name in the list is a z/OS UNIX file name or a data set name. As long as the cursor is
placed in these fields, subsequent presses of the NRETRIEV key will retrieve the next data set name or
z/OS UNIX file name from the list.
Use the personal list settings panel to force the NRETRIEV command to verify the existence of a data set
before retrieving it. If verification is active, then a check is made to see if a data set name exists before a
132  z/OS: z/OS ISPF User's Guide Vol I

## Page 161

retrieval attempt. If a volume name is not in the personal list entry, then the catalog is checked to see if
the data set name is cataloged. If a volume name exists, an OBTAIN macro is used to check the volume
for the data set. Verification does not check ISPF library names or z/OS UNIX file names, and does not
check for the existence of PDSE members. In the data set list Dsname Level field, verification is inactive.
NRETRIEV is enabled on the following options:
• View, including extended move, copy, create, and replace panels
• Edit, including extended move, copy, create, and replace panels
• Library Utility (Option 3.1)
• Data Set Utility (Option 3.2)
• Move/Copy Utility (Option 3.3)
• Data Set List (Option 3.4)
• Reset ISPF Statistics (Option 3.5)
• Hardcopy Utility (Option 3.6)
• SuperC (Options 3.12 and 3.14)
• ISPF Table Utility (Option 3.16)
• z/OS UNIX Directory List Utility (Option 3.17)
• SCLM Options:
– View (Option 1)
– Edit (Option 2)
– Member list (Option 3.1)
– Migration (Option 3.3)
– Unit of Work (Option 3.11)
– Build (Option 4)
– Promote (Option 5)
– Easy Cmds (Option 6A)
SCLM considerations for NRETRIEV
The NRETRIEV command is enabled to work in several of the SCLM options. There are certain restrictions
and considerations to keep in mind when you choose to use NRETRIEV in SCLM.
SCLM restrictions
SCLM has the following restrictions for the NRETRIEV command:
• The NRETRIEV key within SCLM does not use the standard reference list or personal lists. Instead,
it uses a stack that is stored internally. The stack is not editable. The stack is saved from session to
session as a single-line table called ISRSLIST.
Note: In the SCLM View option, the Other Data Set Name field does use the standard reference list
because the Other Data Set Name field has no particular meaning to SCLM.
• In SCLM, there is no validation of saved or retrieved names. That means that if you type in a library
name and press Enter, it is added to the list of saved names, even if SCLM does not process it. This
contrasts with the standard reference list processing, which does not add a data set or library name
until the data set or library is successfully allocated.
• On name retrieval (when the NRETRIEV key is pressed) there is no validation of the existence of data
sets or libraries.
• The regular NRETRIEV command is screen independent (it uses a separate list indicator for each screen
in split screen mode). There is only 1 position locator for SCLM lists. This means that split screens
Chapter 5. Using personal data set lists and library lists  133

## Page 162

with SCLM NRETRIEV will use the same pointer into the list. An NRETRIEV on screen 1 followed by an
NRETRIEV on screen 2 will get list entries 1 and 2 respectively.
Stack management for SCLM
A library name (or concatenation) is added to the list of saved library names by pressing Enter on a panel
that supports saving names. If the library or concatenation exists in the list already, it is moved to the
top of the list. Where the Project field or the first Group field is an output field (SCLM options 2, 3, 4, and
5), the output fields are not used in the comparison between what was typed on the panel and what is
already in the list. This enables you to work in different but similar projects.
In other words, on the edit screen that has both the Project and Group1 as output fields, the
concatenation:
SCLM Library:
   Project...: PDFTDEV
   Group ....: DGN      ....STG    ....INT    ....SVT
   Type .....: ARCHDEF
   Member ...:
would match:
SCLM Library:
   Project...: PDFTOS25
   Group ....: JSM      ....STG    ....INT    ....SVT
   Type .....: ARCHDEF
   Member ...:
Similarly, where groups 2, 3, and 4 are not used, those groups are not used when checking to see if the
name already exists.
If a match is found, the existing entry in the list is moved to the top of the list.
A library name (or concatenation) is added to the list of saved library names by pressing Enter on a panel
that supports saving names. If the library or concatenation exists in the list already, it is moved to the
top of the list. Where the Project field or the first Group field is an output field (SCLM options 2, 3, 4, and
5), the output fields are not used in the comparison between what was typed on the panel and what is
already in the list. This enables you to work in different but similar projects.
In other words, on the edit screen that has both the Project and Group1 as output fields, the
concatenation:
SCLM Library:
   Project...: PDFTDEV
   Group ....: DGN      ....STG    ....INT    ....SVT
   Type .....: ARCHDEF
   Member ...:
would match:
SCLM Library:
   Project...: PDFTOS25
   Group ....: JSM      ....STG    ....INT    ....SVT
   Type .....: ARCHDEF
   Member ...:
Similarly, where groups 2, 3, and 4 are not used, those groups are not used when checking to see if the
name already exists.
If a match is found, the existing entry in the list is moved to the top of the list.
Command interface to the personal list function
You can use these commands to access the referral list function rather than using the action bar pull-
down menus:
134  z/OS: z/OS ISPF User's Guide Vol I

## Page 163

REFLISTD xx
Start the personal data set list dialog with the reference data set list and retrieve the data set in
position xx. The xx parameter is optional. This sets the current data set list to the reference list
(REFLIST).
REFLISTL x
Start the personal library list dialog with the reference library list and retrieve the library in position x.
The x parameter is optional. This sets the current library list to the reference list (REFLIST).
REFACTD nnnnnnnn xx
Start the personal data set list named nnnnnnnn and retrieve the data set in position xx. For example,
enter
REFACTD MYLIST 2
to retrieve the second data set from the personal data set list named MYLIST and place it in the Data
Set Name field. The nnnnnnnn and xx parameters are optional.
REFACTL nnnnnnnn x
Start the personal library list named nnnnnnnn and retrieve the library in position x. For example,
enter
REFACTL MYLIB 1
to retrieve the first library from the personal library list named MYLIB and place it in the Library field.
The nnnnnnnn and x parameters are optional.
REFOPEND
Start the personal data set open dialog.
REFOPENL
Start the personal library open dialog.
REFADDD nnnnnnnn
Update the personal data set list named nnnnnnnn with the most recently referenced data set. For
example, enter
REFADDD NEWLIST
to add the most recently referenced data set to the personal data set list named NEWLIST.
REFADDL nnnnnnnn
Update the personal library list named nnnnnnnn with the most recently referenced library. For
example, enter
REFADDL NEWLIB
to add the most recently referenced library to the personal library list named NEWLIB.
NRETRIEV
Retrieve a name from the current library or data set list on panels which support NRETRIEV
commands. This command is normally assigned to a program function (PF) key. NRETRIEV uses the
position of the cursor to determine what type of personal list to use and what fields on the panel to
fill in. See “Name retrieval with the NRETRIEV command” on page 132 for more information about
NRETRIEV.
Using function keys with personal lists
You can set function keys to process any of the personal list commands, as shown in Figure 31 on page
136.
Chapter 5. Using personal data set lists and library lists  135

## Page 164

Figure 31. Defining  Function Keys to Issue Referral List Commands
If you used these function key settings, you could:
• Press F4 to place the second data set name on the personal data set list named MYLIST in the Data Set
Name field.
• Press F5 to place the first library name on the personal library list named MYLIB in the Library field.
• Press F6 to display the personal data set list named MYLIST so that you can select a data set to process.
Note: You could also type a number on the command line and press F6 to place the data set name in the
specified position on the personal data set list named MYLIST in the Data Set Name field; for example, if
you type 6 on the command line and press F6, the sixth data set on MYLIST would be placed in the Data
Set Name field.
Example of an ISPF panel that uses a referral list
Here is the panel definition for a panel that uses a referral list.
)PANEL KEYLIST(ISRSAB,ISR)
)ATTR DEFAULT(...) FORMAT(MIX)
 0B TYPE(AB)
 04 TYPE(ABSL)
 05 TYPE(PT)
 09 TYPE(FP)
 0A TYPE(NT)
 0C TYPE(NT) SKIP(ON)
 11 TYPE(SAC)
 12 TYPE(CEF) PADC(USER)
 13 TYPE(NEF) PADC(USER)
 19 TYPE(DT)
 22 TYPE(WASL) SKIP(ON)
 08 TYPE(CH)
 10 TYPE(ET)
)ABC DESC('RefList')
 PDC DESC('Reference Data Set List')
   ACTION RUN(ISRRLIST) PARM('RL1')
 PDC DESC('Reference Library List')
   ACTION RUN(ISRRLIST) PARM('LR1')
 PDC DESC('Personal Data Set List')
   ACTION RUN(ISRRLIST) PARM('PL1')
 PDC DESC('Personal Data Set List Open')
   ACTION RUN(ISRRLIST) PARM('PL2')
 PDC DESC('Personal Library List')
   ACTION RUN(ISRRLIST) PARM('LL1')
 PDC DESC('Personal Library List Open')
   ACTION RUN(ISRRLIST) PARM('LL2')
)ABCINIT
136  z/OS: z/OS ISPF User's Guide Vol I

## Page 165

.ZVARS=REFLIST
)ABC DESC('RefMode')
 PDC DESC('List Execute') UNAVAIL(ZRME1)
   ACTION RUN(ISRRLIST) PARM('EEX')
 PDC DESC('List Retrieve') UNAVAIL(ZRME2)
   ACTION RUN(ISRRLIST) PARM('ERT')
)ABCINIT
   .ZVARS=LISTFILE
  VGET (ZELIST) PROFILE
  IF (&ZELIST = 'EXECUTE')
    &zrme1 = 1
    &zrme2 = 0
    &listfile = 2
  ELSE
    &zrme1 = 0
    &zrme2 = 1
    &listfile = 1
)BODY  CMD(ZCMD)
⋮
)INIT
⋮
)REINIT
   REFRESH (PRJ1,LIB1,LIB2,LIB3,LIB4,TYP1,MEM,DSN) /*refresh panel vars*/
)PROC
  /* the following is the logic for reference or personal data set list*/
 VGET (ZRDSN) SHARED            /* get data set reflist key var     */
 IF (&ZRDSN ^= ' ')             /* if reflist has set dsname var    */
   &DSN = &ZRDSN                /* set panel other dsname to zrdsn  */
   VGET (ZREFVOLM) PROFILE      /* get volume retrieve mode         */
   IF (&ZREFVOLM = 'ON')        /* if volume retrieve on            */
     &VOL = &ZRVOL              /* set panel volume to zrvol        */
   &ZRDSN = ' '                 /* blank zrdsn                      */
   &ZRVOL = ' '                 /* blank zrvol                      */
   VPUT (ZRDSN ZRVOL) SHARED    /* return blank reflist vars        */
   .CURSOR = DSN                /* set cursor to panel dsname field */
   VGET (ZELIST) PROFILE        /* get edit execute/retrieve mode   */
   IF (&ZELIST ^= 'EXECUTE')    /* determine if retrieve or execute */
     .MSG = ISRDS003            /* force redisplay if retrieve mode */
  /* End of logic for reference or personal data set list        */
  /* the following is the logic for reference or personal library list */
 VGET (DSALSEL) SHARED          /* get library reflist key var      */
 IF (&DSALSEL ^= ' ')           /* if reflist has set lib indicator */
   VGET (DSA1,DSA2,DSA3,DSA4,DSA5,DSA6,DSA7) SHARED /* get vars  */
   &PRJ1 = &DSA1                /* set panel project                */
   &LIB1 = &DSA2                /* set panel library 1              */
   &LIB2 = &DSA3                /* set panel library 2              */
   &LIB3 = &DSA4                /* set panel library 3              */
   &LIB4 = &DSA5                /* set panel library 4              */
   &TYP1 = &DSA6                /* set panel type                   */
   &MEM  = &DSA7                /* set panel member                 */
   &DSN = ' '                   /* blank panel other dsname         */
   &DSALSEL = ' '               /* blank reflist lib indicator      */
   VPUT (DSALSEL) SHARED        /* return to shared pool            */
   .CURSOR = MEM                /* set cursor to panel member field */
   VGET (ZELIST) PROFILE        /* get edit execute/retrieve ind    */
   IF (&ZELIST ^= 'EXECUTE')    /* determine if retrieve or execute */
     .MSG = ISRDS003            /* setmsg if retrieve mode          */
  /* End of logic for reference or personal library list         */
)END
The library name variables on the panel are set from the following variables in the shared pool:
• DSA1 - project name
• DSA1 - group 1 name
• DSA3 - group 2 name
• DSA4 - group 3 name
• DSA5 - group 4 name
• DSA6 - type name
• DSA7 - member name
Example of an ISPF panel that enables NRETRIEV
Figure 32 on page 138 shows the panel definition for a panel that uses a referral list.
Chapter 5. Using personal data set lists and library lists  137

## Page 166

)BODY
%---------------------------  NRETRIEV Test Panel  ---------------------------%
%COMMAND%===>_ZCMD
               +
+
+  Project ===>_PROJECT +
+  Group   ===>_GROUP1  +===>_GROUP2  +===>_GROUP3  +===>_GROUP4  +
+  Type    ===>_TYPE    +
+  Member  ===>_MEMBER  +
+  DS Name ===>_OTHERDSN                                     +
+  Volume  ===>_VOLUME+
)INIT
 .NRET = ON                      /* Make NRETRIEV key active  */
)REINIT
   REFRESH (*)
   .NRET = ON                    /* Make NRETRIEV key active  */
)PROC
.NRET = OFF          /*IMPORTANT - Make NRETRIEV key inactive */
VGET (ZVERB) SHARED
 IF (&ZVERB = NRETRIEV)          /* if NRETRIEV was entered   */
  IF (.CURSOR NE OTHERDSN, VOLUME)
    .NRET = LIB                  /* Reset data set counter    */
    IF (&ZNRLIB = YES)           /* If library retrieve was OK*/
      .CURSOR = MEMBER           /* set cursor to member field*/
      &PROJECT  = &ZNRPROJ       /* set library variables from*/
      &GROUP1   = &ZNRGRP1       /* the variables set by      */
      &GROUP2   = &ZNRGRP2       /*  NRETRIEV                 */
      &GROUP3   = &ZNRGRP3
      &GROUP4   = &ZNRGRP4
      &TYPE     = &ZNRTYPE
      &MEMBER   = &ZNRMEM
      &OTHERDSN = &Z             /* Blank out odsn field      */
      &VOLUME   = &Z             /* Blank out volume field    */
      .MSG = ISRDS013            /* Indicate good retrieval   */
    ELSE .MSG = ISRDS011         /* Else bad library list     */
   ELSE
    .NRET = DSN
    IF (&ZNRDS = YES)            /* If dsname retrieve was OK */
      .CURSOR = OTHERDSN         /* Move cursor to dsn name   */
      &OTHERDSN = &ZNRODSN       /* Set other dsn name        */
      &VOL      = &ZNRVOL        /* Set volume variable       */
      .MSG = ISRDS014            /* Indicate good retrieval   */
    ELSE .MSG = ISRDS012         /* Else bad ds referral list */
)END
Figure 32. Example Panel Definition  Enabling NRETRIEV
For more information about the .NRET control variable and the function pool variables to hold the data set
name values, see z/OS ISPF Dialog Developer's Guide and Reference.
138  z/OS: z/OS ISPF User's Guide Vol I
