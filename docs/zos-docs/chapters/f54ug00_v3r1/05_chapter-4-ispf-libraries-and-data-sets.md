# Chapter 4. ISPF libraries and data sets

Source file: f54ug00_v3r1.md
Start page: 103
Page span: 103-142

## Page 103

Chapter 4. ISPF libraries and data sets
ISPF enables you to work with ISPF libraries, other partitioned data sets, and sequential data sets. This
topic describes how to allocate, create, and use libraries and data sets.
ISPF also provides some facilities for working with z/OS UNIX files.
• For more information about working with z/OS UNIX files, see "z/OS UNIX Directory List Utility (Option
3.17)" in the z/OS ISPF User's Guide Vol II.
An ISPF library is a cataloged partitioned data set (PDS) or a partitioned data set extended (PDSE). For
more information about PDSE, see “Partitioned Data Set Extended (PDSE)” on page 106.
The ISPF library has a three-level name consisting of a project, group, and type. An optional library
member name can also be included. A member consists of programming code, data, or text.
ISPF displays library names on line 1 of a data display, such as the member list shown in Figure 16 on
page 84. Each library generally contains members with the same type of information.
Before you can create and use a new ISPF library or data set, you must allocate it using option A of the
Data Set utility. See the topic about "Data Set Utility (Option 3.2)" in the z/OS ISPF User's Guide Vol II for
instructions on allocating data sets.
Naming ISPF libraries and data sets
On data entry panels that require a library or other data set name, such as the Data Set Utility (option 3.2)
and Edit Entry (option 2) panels, two groups of fields are provided: one for entering an ISPF library name
and one for entering another partitioned or sequential data set name.
ISPF library names
To name an ISPF library, you must specify at least a project, group, and type. For example:
Project  . . ISPFPROJ
Group  . . . TEST
Type . . . . PLI
Project
The common identifier for all ISPF libraries belonging to the same programming project. This name
must be your user ID unless you are using a specific project name that has been predefined in the
MVS master catalog.
Group
The identifier for a particular set of ISPF libraries, that is, the level of the libraries within the library
hierarchy. For example, the group name of your private library could be PRIVATE or perhaps your first
name, such as Joe in the example in Figure 14 on page 80.
Type
The identifier for the type of information in the ISPF library, such as PL/I, SCRIPT, or PANELS.
Standard ISPF naming conventions
Each component of the library name can be up to 8 alphanumeric or national characters; the first one
must be alphabetic. This conforms to standard TSO data set naming conventions. For convenience, any
cataloged data set (sequential or partitioned) with a three-level name can be entered in the Project,
Group, and Type fields, with one level of the name in each field. If a cataloged data set with four or more
levels is to be entered, multiple levels of the name may be entered in each field, with each level being
separated by a period.
If both a library and a data set name are specified on the same panel, the data set name takes priority.
Therefore, to specify a library, leave the Data Set Name field blank.
Naming ISPF libraries and data sets
© Copyright IBM Corp. 1980, 2024 75

## Page 104

When the library identification appears in a title line or message, the project name, group name, and type
name are separated with periods. A member name, if applicable, is enclosed in parentheses. For example:
PROJECT.GROUP.TYPE(MEMBER)
On most data entry panels that allow a library name to be specified, a Member field is available:
   Member  . . . PROJ1
Member
The name of an ISPF library or other partitioned data set member. Leaving this field blank or entering
a pattern causes ISPF to display a member list. See “Displaying member lists” on page 82 for more
information.
A pattern is a partial member name that uses either an * (asterisk) or % (percent sign) as place
holders:
• A * symbol represents a string of characters
• A % symbol represents only 1 character
ISPF matches the pattern to any like member names in the specified data set.
The ISPF library's project, group, and type must always accompany the member name, if entered. If you
try to edit a member that does not exist, ISPF provides an Edit display screen with a blank data area.
Member names entered in the Member field or those enclosed in parentheses and entered in the Data Set
Name field must follow standard ISPF naming conventions.
If you have a partitioned data set with members whose names do not follow ISPF naming conventions,
ISPF allows limited processing, as follows:
• View (option 1) allows any character string as a member name in either the Member or Data Set Name
field and attempts to View or Browse the specified member.
• Edit (option 2) allows an existing member with a nonstandard member name to be edited. You cannot
create a member with a nonstandard member name.
ISPF cannot process member names that begin with a blank or have embedded blanks which can cause
unpredictable results. Also, ISPF cannot process member names that include special characters, such as
an ampersand (&). CLIST processing in both Foreground (option 4) and Batch (option 5) can result in a
runtime error.
Other partitioned, sequential or VSAM data set, or z/OS UNIX file names
You can use this field to specify any partitioned or sequential data set, or z/OS UNIX file path name:
Other Partitioned, Sequential or VSAM Data Set, or z/OS UNIX file:
   Name . . . . .                                                            +
Note: The + at the end of the field indicates a scrollable field. The + indicates that the field may contain
more data than is able to be displayed on the current screen. It is commonly used to enter long UNIX file
path names but may appear on panels other than those associated with data set names.
See the descriptions of the ZEXPAND (“EXPAND” on page 38) and ZCLRSFLD (“ZCLRSFLD” on page 55)
commands which operate on scrollable fields.
Specifying a data set name
Type any fully-qualified partitioned, sequential, or VSAM data set name, such as:
   Name . . . . . 'USERID.SYS1.ASM'                                          +
You can include either a TSO user prefix or user ID as the first-level qualifier of the data set name. If you
omit the single quotes and if you have created a TSO user prefix, that prefix is automatically added to the
Naming ISPF libraries and data sets
76  z/OS: z/OS ISPF User's Guide Vol I

## Page 105

beginning of the data set name. If you omit the single quotes and if you do not have a TSO user prefix, no
prefix is added, and the name is used exactly as it appears.
If you include your user prefix or user ID, enclose the data set name with apostrophes. If you include the
apostrophe at the beginning of the data set name but omit the one at the end, ISPF inserts it for you.
Note: ISPF does not support multivolume data sets or partitioned data sets with record format FBS or
VBS.
For partitioned data sets, a member name enclosed in parentheses can follow the data set name. For
example:
   Name . . . . . 'SYS1.PROCLIB(ASMHC)'                                      +
If you include the parenthesis at the beginning of the member name but omit the one at the end, ISPF
inserts it for you.
When you omit the member name and parentheses or use a pattern ISPF displays a member list. See
“Displaying member lists” on page 82 for more information.
You can refer to generation data sets by using a signed or unsigned number in place of a member name in
the Data Set Name field only. For example:
   Name . . . . . 'gds.test(0)'                                             +
This example refers to the most recently allocated data set in the generation data group. Minus numbers
refer to previously allocated data sets; positive refer to unallocated.
Note: For Edit, Browse, and View, a VSAM data set can be specified if the ISPF Configuration Table
enables VSAM processing.
Specifying a z/OS UNIX file path name
Type a z/OS UNIX file path name, such as:
   Name . . . . . /u/jsmith/test/tst1.sh                                     +
The Name field is a scrollable field allowing you to enter an absolute pathname up to 1023 characters in
length.
Note: If you often enter long pathnames (greater than 56 characters), consider using the KEYLIST utility
to update the keylist for the panel and assign the ZEXPAND command to a function key. The ZEXPAND
command displays the scrollable input field in a scrollable dynamic area in a pop-up window, making the
task of entering a long pathname easier.
When you enter a z/OS UNIX file path name, a z/OS UNIX directory selection list is displayed.
When you enter a z/OS UNIX file path name containing glob characters and the entered value does not
match a z/OS UNIX directory or file, ISPF uses the C/C++ glob function to search the UNIX file system for
files and directories that match the mask. Unicode Conversion services are used to internally convert the
path name from the terminal codepage to codepage 1047 for use by the search function.
ISPF assumes a z/OS UNIX path name when the first character entered in the Name field is one of these
characters:
/
(Forward slash) Identifies an absolute path name.
~
(Tilde) The path name for your home directory.
.
(Period) The path name for your current working directory.
..
(Double period) The path name of the parent directory of your current working directory.
Naming ISPF libraries and data sets
Chapter 4. ISPF libraries and data sets  77

## Page 106

Glob characters and their meaning are:
?
Match any single character.
*
Match multiple characters.
[
Open a set of single characters.
]
Close the set of single characters. Each character in the set can match a single character at the
position specified.
Examples:
~/test/tst1.sh
Equivalent to specifying the absolute pathname
/u/jsmith/test/tst1.sh
when your home directory is defined as /u/jsmith.
./pgma.c
Equivalent to specifying the absolute pathname
/u/proj1/dev/pgma.c
when your current working directory is set to /u/proj1/dev.
../test/pgma.c
Equivalent to specifying the absolute pathname
/u/proj1/test/pgma.c
when your current working directory is set to /u/proj1/dev.
u/h*/t?st[123]*
Can match /u/harry/test1do and /u/henry/tost2nok.
Volume serials
Along with a data set name, you can optionally specify a volume serial. If you do, the system catalog is not
used. For example:
Volume Serial . . . ______    (If not cataloged)
Volume Serial
A real DASD volume or a virtual volume residing on an IBM 3850 Mass Storage System. To access
3850 virtual volumes, you need MOUNT authority, which is acquired through the TSO ACCOUNT
command or the RACF® TSO AUTH CLASS command.
Library concatenation
Whenever the first Group field is accompanied by three additional fields horizontally across the screen,
you can enter a library concatenation sequence, which is a series of group names chained together. ISPF
searches these groups in the sequence that you enter them.
You can concatenate libraries of the same type, but only libraries that belong to the same project. You
will usually concatenate the lowest-level library ahead of the next higher-level library, and so on, in
bottom-to-top order. Therefore, concatenation is usually most effective if this search sequence is the
same as the library hierarchy.
For example, new library members or members undergoing changes generally reside in libraries used
by program developers. A test library may contain members that have been unit tested and are ready
Library Concatenation
78  z/OS: z/OS ISPF User's Guide Vol I

## Page 107

for integration test. A master library might contain fully tested members that correspond to a previously
released version of the program.
Concatenated libraries must have consistent record formats and logical record lengths. You can use
concatenation with these ISPF functions:
• Viewing
• Browsing
• Editing
• Selecting Library Utility (option 3.1) functions:
– Print index or complete data set
– Browse, delete, edit, print, rename, or view members
– Compress data set.
• Copying data sets or members
• Compiling
• Assembling
• link-editing
• SCRIPT/VS processing.
Note: You can also use additional input libraries for compilations and assemblies.
Figure 14 on page 80 shows a sample three-level hierarchy consisting of a set of master libraries, a set
of test libraries, and three sets of private development libraries identified by user ID. Using this hierarchy,
a typical concatenation sequence for a project of ISPFPROJ, a type of DATA, and a member PGM1 is:
ISPF Library:
   Project . . . ISPFPROJ
   Group . . . . JOE      . . . TEST____ . . . MASTER__
   Type  . . . . DATA
   Member  . . . PGM1____
Library Concatenation
Chapter 4. ISPF libraries and data sets  79

## Page 108

Figure 14. Hierarchy of ISPF Libraries
In this example, the search for member PGM1 goes through libraries:
ISPFPROJ.JOE.DATA
ISPFPROJ.TEST.DATA
ISPFPROJ.MASTER.DATA
Concatenation during editing
Using concatenation during editing provides a way to copy members to your development library. Use the
concatenation sequence to search the libraries for the member to edit. The edited member is saved in
your development library, the first library in the concatenation sequence, while the unchanged version
remains in the test or master library. When the new version is fully tested, you can use the Move/Copy
utility (option 3.3) to move the new version to a higher-level library.
Library Concatenation
80  z/OS: z/OS ISPF User's Guide Vol I

## Page 109

Concatenation during language processing
The purpose of concatenation during language processing is to:
• Help you include source segments in their proper order when using INCLUDE or COPY statements or
when using SCRIPT imbed controls
• Allow debugging of new or changed programs without altering the contents of the test or master
libraries.
The output from a compilation or assembly (an object module) or from a link-edit (a load module) is
stored in the lowest-level OBJ or LOAD library, the first library in the concatenation sequence.
Concatenation of PDSE v2 member generation data sets
When specifying a PDSE generation value in a concatenated scenario during View, Browse, or Edit, it is
important to understand that the generation value does not affect the search method. The generation will
only be applied to the library in which the member is found. For example, lets assume library 1 and library
2 both contain member ABC. Library 2 contains generation 5, but library 1 does not. The user specifies
ABC along with generation 5. This will result in a failure since the first library(1) found containing member
ABC does not have generation 5 allocated.
Using member selection lists
A member selection list, also called a member list, is initially an alphabetic list of the members of an
ISPF library or TSO partitioned data set. Table 15 on page 81 provides a quick reference to the primary
options that display member lists and their differences. In the Type of Selection column, "Single" means
that ISPF processes only the line command that is the closest to the top of the list, ignoring all others.
"Multiple" means that you can enter more than one line command simultaneously. The numbers in
parentheses refer to notes following the table. See “Member selection list commands” on page 89 for
more information about the line commands shown in the table.
Table 15. Member Selection List Differences
Primary Options
Type of
Selection Valid Line Commands
Prompt Field
Available
View (1) Single S,V (4) No
Browse (1) Single S,B (4) No
Edit (2) Single S,E (4) No
Library (3.1) Multiple B,C,D,E,G,I,J,M,N,P,R,T,V Yes
Move/Copy (3.3) Multiple B,S (1) Yes
Data Set List (3.4) Multiple B,C,D,E,G,I,J,M,N,P,R,T,V (2) Yes
Reset (3.5) Multiple S No
SuperC (3.12) Multiple S No (3)
SuperCE (3.13) Multiple S No (3)
Search-For (3.14) Multiple S No
Foreground (4) Single S No
Batch (5) Single S No
Workplace (11) Multiple B,C,D,E,G,I,P,M,R,S,T,V No
Note:
Using member selection lists
Chapter 4. ISPF libraries and data sets  81

## Page 110

1. For the Move/Copy utility, B (browse member) enables you to browse members of an ISPF library or
another partitioned data set before moving or copying them without having to use browse on another
panel. Then, use S (select) to select the member or members to move or copy. See “Line commands
for the move/copy utility” on page 98 for more information.
2. When you select M (display member list) line command on a data set list, you can use B (browse
member), C (copy member), D (delete member), E (edit member), G (reset member statistics), I
(display member information), J (submit member), M (move member), N (display generation list), P
(print member), R (rename member), T (invoke TSO command for member), and V (view member). You
can also enter TSO commands, CLISTs, and REXX EXECs.
S (select) is valid also, but only when the B, CO, E, MO, RS, or V line commands are used on a data set
list.
3. Instead of a Prompt field, this member list has an OLDMEM field, which you can use to enter the name
of a member in the old data set. For more information about this field, see the topic on SuperC Member
Lists in the SuperC Utility (Option 3.12) section of the z/OS ISPF User's Guide Vol II.
4. For your convenience ISPF supports E as a select character from Edit member lists in addition to S and
point-and-shoot selection. Similarly, V is supported from View member lists, and B is supported from
Browse member lists.
5. When multiple members are selected from a member list that supports multiple selection and all
members have been processed, the member list is scrolled such that the last member processed is
positioned to the top of the member list display.
Displaying member lists
For each of the primary options listed in the preceding table, except Data Set List (option 3.4), you can
display a member list by:
• Leaving the Member field blank for an ISPF library
• Omitting the member name from the name of another partitioned data set
• Entering a pattern as the member name.
You can use a combination of asterisks and percent signs in the same pattern. However, the pattern,
including the asterisks and percent signs, can contain no more than 8 characters. For example, entering
this pattern in the Member field:
Member  . . . *prof___
could display this member list:
ISFPROF
ISPPROF
ISPSPROF
ISRPROF
LOCPROF
SUPCPROF
When using the Data Set List utility (option 3.4), you can display a member list by:
• Entering the M (display member list) line command
• Entering the V (view), B (browse), or E (edit) line command and then using one of the methods
described in the preceding list. This applies only if you are editing or browsing members of a partitioned
data set.
• Entering the CO (copy) line command
• Entering the MO (move) line command
• Entering the RS (reset) line command.
On any member list, PF10 and PF11 toggle between two different views of the member list data.
Note:
Using member selection lists
82  z/OS: z/OS ISPF User's Guide Vol I

## Page 111

1. The column headers on a member list display (with the exception of Prompt) are point-and-shoot sort
fields.
2. If you enter a slash in the line command field, the Member List Commands pop-up window shown in
the next figure is displayed so that you can select the command you want to use.
3. The line command field is a point-and-shoot field. If you select the line command field beside a
member name, the Member List Commands pop-up window shown in the next figure is displayed so
that you can select the command you want to use. 
Figure 15. Member List Commands Pop-Up Window (ISRCMLEP)
4. Member list count fields show an accurate count when the number of members in a PDS or PDSE is
less than 10 000 000. The row value will be truncated after member 9 999 999 and the total value will
be truncated on display of the list if more than 9 999 999 members exist.
5. The Info command displays the same information as the member list. When the Extended PDS
statistics function has been enabled, the extended line counts fields contain data.
Ending member lists
With two exceptions, you can end a member list by entering END (F15) or using = (the jump function) to
go to another option. For the two exceptions, SuperC and Search-For member lists, enter RETURN (F16),
CANCEL, or =. On these member lists, the END command processes your selections.
ISPF member statistics
On member lists, column headings appear in the national language. The information shown under the
column headings contains the ISPF statistics generated for each member. You can print these statistics
using option X (print index listing) of the Library utility (option 3.1) or option P (print data set list) of the
Data Set List utility (option 3.4). You can also use the SAVE command to write a member list or data set
list to the ISPF list data set or to a sequential data set. The statistics are displayed next to each member
name.
Figure 16 on page 84 shows an example of a member list with statistics and the 1-character line
command field to the left of the member names. If you want to see all of the statistics, you can scroll the
Using member selection lists
Chapter 4. ISPF libraries and data sets  83

## Page 112

screen either right or left by using PF keys 10 and 11. Figure 17 on page 84 shows an example of the
screen when you scroll right. Pressing either key repeatedly results in recycling of the screens.
   Menu  Functions  Confirm  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
 LIBRARY          USERID.EXEC                            Row 0000001 of 0000146
    Name     Prompt          Size    Created           Changed            ID
 _ ALLOCEXT                     5   2002/07/25   2002/07/25 16:28:48    USERID
 _ AMBLIST                      7   2001/01/04   2001/09/11 12:02:41    USERID
 _ AOPST                       10   2002/05/27   2002/05/27 10:38:15    USERID
 _ APCTOOLS                    20   2002/11/29   2003/01/21 09:59:50    USERID
 _ APCTOOLX                   193   2002/11/29   2003/01/21 10:16:42    USERID
 _ APCT2AZ                   2610   2002/11/29   2003/01/21 10:23:24    USERID
 _ APPLT1                       4   2002/05/22   2002/05/22 09:32:20    USERID
 _ APPLT2                       8   2002/05/22   2002/05/22 09:32:49    USERID
 _ APPLT3                       4   2002/05/22   2002/05/22 09:28:59    USERID
 _ ASMPROG1                     3   2002/01/23   2002/01/23 12:41:02    USERID
 _ BATCHCMP                     4   2002/02/21   2002/02/21 13:04:48    USERID
 _ CLEDIT                       3   2002/06/11   2002/06/11 09:31:02    USERID
 _ CMSED                       39   2002/12/06   2002/12/06 14:10:55    USERID
 _ COMP                        10   2002/10/09   2002/10/09 15:14:44    USERID
 _ CRZDIFF                     58   2002/11/18   2002/11/18 13:39:48    USERID
 _ CRZDISP                     14   2002/11/14   2002/11/14 15:02:03    USERID
 _ CRZLOAD                    105   2002/11/13   2002/11/18 14:30:48    USERID
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 16. Member List Display (ISRUDMM)
Members that have extended statistics appear highlighted in the member list when highlighting is
available. The line count values displayed on the member list have a maximum value of 65535. If
extended statistics are generated for a member and an extended line count value exceeds 65535,
>65535 is displayed on the member list. Use the Info command from the enhanced member list to
display the extended line count values. The maximum value of extended line counts is 2147483647.
   Menu  Functions  Confirm  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
 LIBRARY           USERID.EXEC                            Row 0000001 of 0000146
    Name     Prompt          Size     Init       Mod       VV MM          ID
 _ ALLOCEXT                     5        1         0       01.04        USERID
 _ AMBLIST                      7        7         0       01.08        USERID
 _ AOPST                       10       10         0       01.01        USERID
 _ APCTOOLS                    20       19         0       01.03        USERID
 _ APCTOOLX                   193      212         0       01.04        USERID
 _ APCT2AZ                   2610     2647         0       01.16        USERID
 _ APPLT1                       4        3         0       01.02        USERID
 _ APPLT2                       8        5         0       01.03        USERID
 _ APPLT3                       4        4         0       01.00        USERID
 _ ASMPROG1                     3        1         0       01.17        USERID
 _ BATCHCMP                     4        3         0       01.03        USERID
 _ CLEDIT                       3        2         0       01.01        USERID
 _ CMSED                       39       37         0       01.03        USERID
 _ COMP                        10       10         0       01.00        USERID
 _ CRZDIFF                     58       12         0       01.02        USERID
 _ CRZDISP                     14       14         0       01.01        USERID
 _ CRZLOAD                    105       57         0       01.15        USERID
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 17. Member List Display cont. (ISRUDMM)
Member list display panel action bar
The Member List Display panel action bar choices function as follows:
Menu
See the topic about the Primary Option Menu in the z/OS ISPF User's Guide Vol II for information about
the Menu pull-down.
Functions
The Functions pull-down offers these choices:
1
Save List  Saves the member list into the list data set.
Using member selection lists
84  z/OS: z/OS ISPF User's Guide Vol I

## Page 113

2
Change Colors  Displays the Member List Color Change Utility panel (Figure 18 on page 85) to
allow you to change one or more of the Member List Field Attributes and press Enter to see the
effect immediately. Clearing a field or selecting the Defaults field restores defaults.
Note: You can also display this panel by entering MLC on the Command line.
Figure 18. Member List Color Change Utility Panel (ISRMLCP)
3
Initial Sort View  Displays the Enhanced Member List Initial Sort panel (Figure 19 on page 86).
This panel enables you to select the field to be sorted on, before the display of an enhanced
member list. Member Name is the default View. Some other sort views that you can choose are:
• RECFM=BLK sets the initial view for blocked data set formats such as FB and VB.
• RECFM=U sets the initial view for unformatted data set formats such as Load.
• Any of these conditions result in a default sort view on member name:
– Library field selected as initial sort view and member list is based on a single data set.
– Created or Alias selected as initial sort view and extended command member list.
– A member name is given as input to member list action.
Using member selection lists
Chapter 4. ISPF libraries and data sets  85

## Page 114

Figure 19. Enhanced Member List Initial Sort Panel (ISRMLIS)
Note:
1. You can also display this panel by entering MLS on the Command line.
2. When using LLA in FREEZE mode, member lists might appear to be out of SORT order because
ISPF uses direct reads of the data set directory for initial member list build and SORT, and uses
BLDL for the displayed statistics.
Confirm
Select 1 to set delete confirmation ON. Select 2 to set delete confirmation OFF.
Utilities
See the topic about the Primary Option Menu in the z/OS ISPF User's Guide Vol II for details on the
Utilities pull-down.
Help
The Help pull-down provides general information about member list topics such as scrolling, pattern
matching, and member list statistics, as well as information about supported primary commands and
the S line command.
Member list display panel fields
ISPF generates statistics each time you edit a member, unless your edit profile is set to STATS OFF. The
fields shown identify the statistics in a member list:
Note: The column headers on a member list display (with the exception of Prompt) are point-and-shoot
sort fields.
Name
Name of the member.
Using member selection lists
86  z/OS: z/OS ISPF User's Guide Vol I

## Page 115

Prompt
The Prompt field serves a variety of purposes. You can rename a member by typing the new name to
the right of the member name. You can type a slash character (/) in the first position of the Prompt
field so you can define additional behaviors for a given action.
Also, the Prompt field acts as a status field, showing information about the last action taken for a
member. If you run an edit macro or TSO command against a member, the 7-character informational
status that is returned in the dialog variable ZPROMPT at the completion of the service is shown in this
field.
Lib
Library number. The Lib field appears only if you specify a concatenated sequence of libraries. It
shows the library that contains the member. In this example, if the member resides in the second
library in the sequence, a 2 appears in the Lib field.
VV.MM
Version number and modification level. The version number is set to 1 and the modification level is set
to 0 when the member is created. The modification level is the number of times this version has been
modified. For example, 02.15 means version 2, modification 15.
If a member name is just an alternate name for another member, ALIAS appears in this field.
Created
Date this version was created. The format used depends on your national format. For example,
90/06/27 means June 27, 1990 to some, but so does 06/27/90 and 27/06/90 mean it for others.
Changed
Date and time this version was last modified; date is shown in the national format. Time is shown
using a 24-hour format. For example, 17:20 means 5:20 p.m..
Size
Current number of lines. The largest number this field can display is 65 535. If extended statistics
are generated for a member and the current number of lines value in the extended statistics exceeds
65535, >65535 is displayed on the member list. Use the Info command from the enhanced member
list to display the extended line count values. The maximum value of extended line counts is
2147483647.
Init
Initial number of lines. The largest number this field can display is 65 535. If extended statistics
are generated for a member and the initial number of lines value in the extended statistics exceeds
65535, >65535 is displayed on the member list. Use the Info command from the enhanced member
list to display the extended line count values. The maximum value of extended line counts is
2147483647.
Mod
Number of lines in the current member that have been added or changed. If the data is unnumbered,
this number is zero. The largest number this field can display is 65 535. If extended statistics are
generated for a member and the number of lines added or changed value in the extended statistics
exceeds 65535, >65535 is displayed on the member list. Use the Info command from the enhanced
member list to display the extended line count values. The maximum value of extended line counts is
2147483647.
ID
The user ID of the person who created or last updated this version. If the user ID is 8 characters
and the member list panel layout does not allow for 8-character values, the first 6 characters are
displayed followed by >. To display the 8-character value on these panels, 8-character user ID layouts
must be enabled in the site configuration.
When you use View, Browse, and Edit, the current version and modification level are displayed in the title
area, line 1, following the library and member name. You can change the version number, the user ID,
or both, with the Reset ISPF Statistics utility (option 3.5) or with the LEVEL and VERSION Edit primary
commands. Changing the version number updates most of the other statistics.
Using member selection lists
Chapter 4. ISPF libraries and data sets  87

## Page 116

If you use the ISPF editor to delete all lines in a member of an ISPF library and then save the member,
the statistics show that the member still exists but has a length of zero. To delete a member, including its
statistics, use the Library utility (3.1).
Load module library member statistics
Figure 20 on page 88 shows that the ISPF library statistics displayed in a member list have a different
format for load module libraries. See “Member list display panel action bar” on page 84 for a description
of the action bar choices on this panel.
   Menu  Functions  Confirm  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
 LIBRARY           PDFTDEV.SVT.LOAD                      Row 0000001 of 0000480
    Name     Prompt         Alias-of      Size       TTR      AC    AM     RM
 _ FLM$CP                   FLMIO24     0000A938    01E70E    00     24     24
 _ FLM$CPI                              000000E8    01820C    00     31    ANY
 _ FLM$DE                   FLMIO24     0000A938    01E70E    00     24     24
 _ FLM$DT                   FLMIO24     0000A938    01E70E    00     24     24
 _ FLM$99                   FLMIO24     0000A938    01E70E    00     24     24
 _ FLMB                                 000A9970    01EF16    00     31    ANY
 _ FLMBCMD                  FLMDDL      00122360    029008    00     31    ANY
 _ FLMBD$                   FLMDDL      00122360    029008    00     31    ANY
 _ FLMCMD                   FLMS7C      000E62B8    02AE13    00     31    ANY
 _ FLMCNTGN                             0001E838    028C1D    00     31    ANY
 _ FLMCPCS                              00000150    01822A    00     31    ANY
 _ FLMCSLNK                 FLMIO24     0000A938    01E70E    00     24     24
 _ FLMCSPDB                             00001940    01E80A    00     31     24
 _ FLMCXCMD                 FLMIO24     0000A938    01E70E    00     24     24
 _ FLMCXCPD                 FLMIO24     0000A938    01E70E    00     24     24
 _ FLMCXCPM                 FLMIO24     0000A938    01E70E    00     24     24
 _ FLMCXCTN                 FLMIO24     0000A938    01E70E    00     24     24
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 20. Load Module Library Display (ISRUDMM)
If you want to see all of the statistics, you can scroll the screen either right or left by using PF keys 10 and
11. Pressing either key repeatedly results in recycling of the screens.
The fields on a member list display for a load module library are:
Note: The column headers on a member list display (with the exception of Prompt) are point-and-shoot
sort fields.
Name
Name of the member.
Prompt
The Prompt field serves a variety of purposes. You can rename a member by typing the new name to
the right of the member name. You can type a slash character (/) in the first position of the Prompt
field so you can define additional behaviors for a given action. Also, the Prompt field acts as a status
field, showing information about the last action taken for a member.
Lib
Library number. The Lib field appears only if you specify a concatenated sequence of libraries. It
shows the library that contains the member. In this example, the member resides in the second
library in the sequence.
Size
Size of the member in hexadecimal. The largest number this field can display is 'FFFFFFFF'X.
Using member selection lists
88  z/OS: z/OS ISPF User's Guide Vol I

## Page 117

TTR
Relative block address.
Alias-of
Name of the member for which this member is an alias. See following note.
AC
Authorization code.
AM
Addressing mode.
RM
Residency mode.
Attributes
The member's attributes:
NX
Not executable
DATA
Can be loaded only
OVLY
In overlay structure
RF
Refreshable
RN
Can be reentered
RU
Reusable
TEST
Module to be tested.
SSI
System Status Index
Note: Question marks (?) are placed in the member list of a load module library for members that have
load module directory fields that are not valid. For example, module 14, shown in Figure 20 on page 88,
contains alias and authorization code information that is not valid.
Load module library lists displayed using the Data Set List utility (option 3.4) contain an extended line
command field and do not display the created date.
Member selection list commands
If the member list is too large for the screen, you can see other parts of the list by using the UP and DOWN
scroll commands. These commands are valid for all member list displays. However, because a member
list display can be no wider than 80 characters, you cannot use the LEFT and RIGHT scroll commands.
See the z/OS ISPF Dialog Developer's Guide and Reference for more information about scroll commands.
These primary commands can be entered on the Command line in all member list displays:
CONFIRM
FILTER
field operator value
FIND
RFIND
Using member selection lists
Chapter 4. ISPF libraries and data sets  89

## Page 118

LOCATE string
RESET
SAVE
list-id
SELECT pattern
lcmd
SORT
field1
A
D
field2
A
D
SRCHFOR string
MLC
MLS
REFRESH
These line commands can be used with member lists. These are 1-character commands that are entered
to the left of the member name. The option you are using determines:
• Whether you can enter more than one line command simultaneously
• Which line commands are valid
• Whether a Prompt or OLDMEM field is available.
Table 15 on page 81 provides a quick reference to the differences between member lists and the line
commands available on each one. The line commands are:
• B (browse member)
• C (copy member)
• D (delete member)
• E (edit member)
• G (reset member statistics)
• I (display member information)
• J (submit member)
• M (move member)
• N (display generation list)
• P (print member)
• R (rename member)
• S (select member)
• T (invoke TSO command for member)
• V (view member)
• = (repeat last command).
The S line command is available for all member list displays except the Library and Data Set List utilities.
See “S Line Command” on page 94 for more information.
Using member selection lists
90  z/OS: z/OS ISPF User's Guide Vol I

## Page 119

The B line command is available only for the Library, Move/Copy, and Data Set List utilities. The D, E, P, R,
and V line commands are available only for the Library and Data Set List utilities.
Note: For the Data Set List utility, these line commands are valid only after you enter the M (display
member list) line command. See “Library and data set list utility line commands” on page 98 for
information.
ISPF ignores any unprocessed member list commands when you leave a member list.
Primary commands
See:
• “Require delete commands to be confirmed (CONFIRM)” on page 91
• “Display a subset of members (FILTER)” on page 91
• “Find a character string (FIND and RFIND)” on page 92
• “Locate a data string (LOCATE)” on page 93
• “Remove unwanted line commands and messages (RESET)” on page 93
• “Write a member list to a sequential data set (SAVE)” on page 93
• “Select a member (SELECT)” on page 94
• “Sort a member list (SORT)” on page 95
• “Search for members (SRCHFOR)” on page 96
• “Change member list field attributes (MLC)” on page 97
• “Change the default sort order for member lists (MLS)” on page 97
• “Refresh the member lists (REFRESH)” on page 97
Require delete commands to be c on firmed  (CONFIRM)
The CONFIRM primary command controls display of the Confirm Delete panel. Use the format:
CONFIRM
ON
OFF
You can use these operands with the CONFIRM command:
ON
Tells ISPF to display the Confirm Delete panel when you enter the D (delete data set) line command or
TSO DELETE command. This is the default setting.
OFF
Tells ISPF not to display the Confirm Delete panel.
For example, this command would tell ISPF not to display the Confirm Delete panel:
CONFIRM OFF
Note: Confirm is forced ON from the workplace member list with a default action of "D".
Display a subset of members (FILTER)
Use the FILTER command to display only the subset of members whose attributes match the supplied
comparison argument. You specify the comparison argument in this format:
Using member selection lists
Chapter 4. ISPF libraries and data sets  91

## Page 120

FILTER field EQ
NE
LE
LT
GE
GT
value
Note:
1. It is possible to filter the member list using any of the fields on the member list panel except for
Prompt.
2. If a member has no value for the specified field, the member will be considered a match for the LT and
NE operators.
3. If no members match the filter criteria, the member list remains unchanged.
4. Entering the FILTER command with no parameters displays a panel in which you can select the field
and operator and enter the value.
5. The FILTER command can be applied repeatedly to drill down to the subset of members that match a
particular combination of attributes.
For example, to display all the members whose modification level is greater than 02, enter: FILTER
MM GT 02
To further subset this list to display only the members in which more than 200 lines have been
modified, enter: FILTER MOD GT 200
6. Enter the REFRESH command to restore the full member list.
7. EQ and NE are the only operators that can be used to filter the fields AM, RM, and ATTR.
8. The value must be specified in the appropriate format for the field type. For example, version number
must be specified as a 1-digit or 2-digit number, creation date must be specified in date format, and
load module size must be specified as a hex string:
FILTER VV EQ 02
FILTER CRE LT 2000/01/01
FILTER SIZE GT FFFFFF
Find a character string (FIND and RFIND)
To find a character string within the specified field use the FIND or the RFIND command. Only one of the
fields on the member list can be specified at a time for the search. Use this format:
FIND
F 
string
Name
field
NEXT
ALL
FIRST
LAST
PREV
PREFIX
SUFFIX
WORD
RFIND
NAME is the default field. NEXT is the default operand. For example, this command tells ISPF to find the
last occurrence of the character string XLC in the NAME field:
FIND XLC NAME LAST
Using member selection lists
92  z/OS: z/OS ISPF User's Guide Vol I

## Page 121

ISPF automatically scrolls to bring the line containing the character string to the top of the list.
Use RFIND to repeat the search without reentering the character string.
Locate a data string (LOCATE)
To find a data string, you can enter a LOCATE command in the Command field on any member list display.
The format of the command is:
LOCATE string
where:
string
A data string that is used to find an entry based on how the member list is sorted.
ISPF searches the field by which the member list was sorted for an entry equal to string. Either the entry,
if found, or else the entry that immediately precedes the entry that you are searching for is scrolled to the
top of the list.
For example, if the member list shown in Figure 20 on page 88 is sorted by name, this command causes
member MODULE12 to scroll to the top of the list:
LOCATE MODULE12
Remove unwanted line commands and messages (RESET)
The RESET command removes unprocessed line commands and messages that show the result of line
command processing.
RESET
Write a member list to a sequential data set (SAVE)
The SAVE primary command writes a member selection list to a sequential data set. The format of the
SAVE command is:
SAVE
list-id LONG
where:
list-id
Optional. A user-specified qualifier of the sequential data set to which the member list is written.
LONG
Optional. Results in additional information in the saved member list:
• All dates for the member are in yyyy/mm/dd format.
• For PDS data sets not containing load libraries, the untranslated member name is written after the
member name.
• For members with extended statistics, an additional line is written that contains the extended
statistics line count values.
ISPF names the data set:
prefix.userid.list-id.MEMBERS
pr efix 
Your data set prefix, as specified in your TSO user profile. If you have no prefix set, or if your prefix is
the same as your user ID, the prefix is omitted and the data set name is userid.list-id.MEMBERS.
Using member selection lists
Chapter 4. ISPF libraries and data sets  93

## Page 122

userid
Your TSO user ID.
The data set is created if it does not exist, or written over if it exists and has compatible attributes.
ISPF writes the member list in the current sort order and as it appears on the display, except for the
column headings, line command fields, and anything you have typed on the display.
If you omit the list ID, ISPF writes the member selection list in the current sort order, including column
headings, to the ISPF list data set. Processing is the same as using option X of the Library utility (primary
option 3.1), except that data set information is not printed.
Select a member (SELECT)
You can use the SELECT, or S, command as either a primary command or a line command.
SELECT primary command
The SELECT primary command allows you to select one or more members in a member list, whether they
are displayed or not. When you enter it on a member list displayed using the Edit option, this command
even creates a member if you specify the complete member name of a member that does not exist.
The SELECT command optionally provides a quick method of calling the same line command for one or
more members. The format of the SELECT command is:
SELECT pattern
*
lcmd
where:
pattern
Either a complete member name or a partial member name that contains one or more asterisks (*),
percent signs (%), or both as place holders. See “Displaying member lists” on page 82 for more
information about using patterns.
*
An asterisk, which means you want to select all members in a member list.
lcmd
One of these optional line commands: S (select), B (browse), V (view), D (delete), E (edit), or P (print).
On a member list that has an expanded line command field, such as one generated by the M (member
list) line command in the Data Set List utility (option 3.4), you can also enter a TSO command, CLIST,
or REXX exec. If you do not enter a line command, S is the default.
The member list shown in Figure 16 on page 84 contains members INT and INTTOOL. This command
selects these members for printing:
SELECT INT* P
S Line Command
You can enter the S line command at the beginning of a line, ahead of one or more member names. For
example, in Figure 20 on page 88, you could select member MODULE9 by moving the cursor to the left of
the member name, typing S, and pressing Enter.
Note: On member lists displayed with the View, Browse, Edit, Foreground, and Batch options, ISPF
processes only the first S entered, ignoring all others.
With the Move/Copy utility and the Convert utility, you can rename members by entering new member
names in the Prompt field to the right of the member name.
Using member selection lists
94  z/OS: z/OS ISPF User's Guide Vol I

## Page 123

Sort a member list (SORT)
The SORT primary command arranges a member list according to the fields you specify. The sort
sequence, ascending or descending, is determined by the fields you choose and is maintained between
member list displays.
The format of this command is:
SORT field1
A
D
field2
A
D
where:
field1 
The primary field by which the member list is sorted.
field2 
The secondary field by which the member list is sorted.
A|D
The direction in which values are sorted for this field (A=ascending, D=descending).
Table 16 on page 95 and Table 17 on page 95 show:
• Valid values for field1  and field2 
• The default sort sequence used for each field
• A description of each field name.
Note: When multicultural support is enabled, the field names listed in Table 16 on page 95 and Table 17
on page 95 may be displayed in the national language. If they are then the SORT command will expect
field1  or field2  to be entered in the national language, or the standard abbreviation to be used.
Table 16. Sort Fields for Source Libraries
Field Sequence Description
Name Ascending Member name
Lib Ascending Library in concatenation sequence
VV Ascending ISPF version number
MM Ascending ISPF modification level
Created Descending Creation date
Changed Descending Date and time last changed
Size Descending Current number of records
Init Descending Initial number of records
Mod Descending Number of modified records
ID Ascending Last user
Prompt Descending Prompt field
Table 17. Sort Fields for Load Libraries
Field Sequence Description
Name Ascending Member name
Using member selection lists
Chapter 4. ISPF libraries and data sets  95

## Page 124

Table 17. Sort Fields for Load Libraries (continued)
Field Sequence Description
Lib Ascending Library in concatenation sequence
Size Descending Load module size
TTR Ascending TTRN of beginning of load module
Alias-Of Ascending Member this is an alias of
AC Ascending Authorization code
AM Descending 2 Addressing mode
RM Descending 2 Residency mode
Attributes Descending Load module attributes
SSI Ascending System Status Index
Prompt Descending Prompt field
For example, to sort a member list by size and then by track record, enter:
SORT SIZE TTR
To sort a member list by creation date in ascending order, enter:
SORT CREATED A
Search for members (SRCHFOR)
Use the SRCHFOR primary command to search the members in the member list for one or more strings of
data using the SuperC Utility (see Option 3.14). Use this format:
SRCHFOR string
The string parameter is optional but always converted to uppercase. If string is specified, the search is
performed using the current settings in the MEMBER LIST Srchfor Options panel. For example, if "Any
case" is not selected and "Filter list" is selected, the command SRCHFOR LBLBOX will list members that
contain the string "LBLBOX". A member that contained "lblbox" but not "LBLBOX" would not be listed.
If string is not specified, the MEMBER LIST Srchfor Options panel is displayed. You can use this panel to
specify multiple search strings, process options, and output options.
The operands WORD, SUFFIX, and PREFIX can be specified after each search string. Note that the search
strings are case sensitive and must match exactly as specified. If you want to disregard case, use the "Any
case" process option.
Select the "ASCII" process option to cause ISPF to process the data in the member as ASCII. The data
read from the members is converted from ASCII to EBCDIC. Any search string given in hexadecimal
notation is assumed to be in ASCII, matching the original input data. The ASCII code page is assumed to
be ISO 8859-1 (CCSID 819). The terminal code page is used as the EBCDIC code page. If the terminal
code page cannot be determined code page 1047 is used.
You can use the C (continuation) operand to specify that both the current and previous string must be
found on the same line to constitute a match. Otherwise, lines with either string are treated as matching.
2 For the AM and RM columns, the value ANY is considered to be the largest value and will therefore sort to
the top of the list.
Using member selection lists
96  z/OS: z/OS ISPF User's Guide Vol I

## Page 125

You can use the process options "Set EDIT FIND string" and "Set BROWSE FIND string" to initialize the
FIND string in Edit and Browse from the first SRCHFOR string. Use the output option "Filter list" to list only
the subset of members that contain one of the search strings.
Table 18. MEMBER LIST Srchfor Options panel: search string examples
Search strings Explanation
===> ABC
===> EFG
Either string ABC or EFG may be found in the search members.
===> ABC WORD
===> EFG C
The two strings (ABC and EFG) must be found on the same line. ABC must be a
complete word, while EFG (a continuation definition) can be part of any word.
===> ABcD prefix The string (ABcD) is detected if the case of each letter matches and it is a prefix
of a word.
===> X'7b00' The hex string is specified as the search string. The listing must be browsed with
'HEX ON'.
===> 'AB C''D' The string (AB C'D) is specified.
To start the search from the MEMBER LIST Srchfor Options panel, press Enter. To cancel the request and
return to the Member List, enter END or CANCEL.
Output is in the listing DSN you specify and in the MESSAGE field in the DSLIST. Sort on this field to
consolidate results.
Change member list field  attributes (MLC)
The MLC command displays the Member List Color Change Utility. Use this panel to change one or more
of the member list field attributes and to see the change immediately. Clearing a field restores the
field's default setting. Use the Defaults point-and-shoot field to restore all field attributes to ISPF default
settings.
MLC
You can also change the member selection field to use the ISPF Settings input field padding character
instead of the member list field default padding character. The member list default padding character for
single command selection lists is a period (.), and for multiple command selection lists it is an underscore
(_).
Change the default sort order for member lists (MLS)
The MLS command displays the Enhanced Member List Initial Sort panel. Use this panel to change the
default sort order for all ISPF enhanced member lists. You can specify separate sort orders for Load and
non-Load data sets.
MLS
Refresh the member lists (REFRESH)
The REFRESH command refreshes the member list, adding new members, adding renamed members
under their new names, and deleting members that have been removed from the list. It also resets
the line command field and prompt field on the member list. Unprocessed line commands and input or
messages in the prompt fields are erased by the REFRESH command.
REFRESH
Using member selection lists
Chapter 4. ISPF libraries and data sets  97

## Page 126

Line commands
See:
• “Line commands for the move/copy utility” on page 98
• “Library and data set list utility line commands” on page 98
Line commands for the move/copy utility
On member list displays for the Move/Copy utility (option 3.3), you can enter these line commands at the
beginning of a line, ahead of one or more member names:
B
Browse the member
S
Select the member.
The B (browse) line command allows you to browse a member or members to determine whether you
really want to move or copy them. You can enter the B line command beside as many members as
you want to. The first member that has a B line command beside it is browsed when you press Enter.
When you finish browsing each member, the member list is redisplayed along with the unprocessed line
commands. Press Enter again to browse the next member.
Once you have decided which members to move or copy, use the S (select) line command to select those
members.
Library and data set list utility line commands
On member list displays for the Library utility (option 3.1) and the Data Set List utility (option 3.4), you can
enter these line commands at the beginning of a line, ahead of one or more member names:
B
Browse the member
C
Copy the member
D
Delete the member
E
Edit the member
G
Reset the member statistics
I
Display the member information
J
Submit the member
M
Move the member
P
Print the member
R
Rename the member
T
Invoke a TSO command for the member
V
View the member
=
Repeat last command
Using member selection lists
98  z/OS: z/OS ISPF User's Guide Vol I

## Page 127

Note:
1. Member lists displayed with the M line command have a 9-character line command field to
accommodate TSO commands, CLISTs, and REXX EXECs. For more information, see the topics "M-
Display Member List" and "TSO Commands, CLISTs, and REXX EXECs" in the Data Set List Utility
(Option 3.4) section of the z/OS ISPF User's Guide Vol II.
Any data in the prompt field is passed as an argument to any TSO command, CLIST or REXX EXEC.
When the '=' command is used the previous prompt data is also passed. Any prompt data that starts
with '*' is ignored.
2. Where the member to be deleted by the D line command is the name of a primary member, the
primary name and all associated alias names are deleted. Where the member is an alias member, only
the alias name and its directory entry are deleted.
3. When you use the R line command, enter the new member name in the Prompt field to the right of the
member name.
4. Where the data set refers to a partitioned data set load library (RECFM=U), and the member to be
renamed is the name of an primary member, the user data component of any associated alias names
will be updated to refer to the renamed primary name.
5. The Info command displays the same information as the member list. When extended line counts are
available, this panel can be used to display the values. Otherwise these panel fields are blank.
6. When you use the T line command, enter the name of the TSO command you want to execute in the
Prompt field to the right of the member name. The fully-qualified data set name, including the member
is passed as a parameter to the TSO command. If you want to execute a member that is a REXX exec
or CLIST, use the T line command on the line for that member, and enter EXEC in the Prompt field. If
you leave the Prompt field blank, the TSO Command Action panel allows you to enter the command
you want to execute.
Consider the following items when using line commands with members in a PDSE version 2 data set that
is configured for member generations:
• When you use the B, E, or V line command to browse, edit, or view a member, you can use the Prompt
field to access previous generations of the member. In addition to entering the line command, enter a
slash (/) in the Prompt field to display a panel on which you can enter the generation that you want to
access.
• When you use the D line command to delete a member, the current generation and all previous
generations of the member are deleted.
• When you use the C line command to copy a member, only the current generation of the member is
copied.
• When you use the M or R line command to move or rename a member, the current generation of the
member is moved or renamed and all previous generations of the member are deleted.
When you press Enter, each member preceded by a line command is processed unless:
• The V (view), B (browse), or E (edit) line command is followed by another line command. When you
return to the member list after viewing, browsing, or editing a member, you must press Enter again to
call any remaining line commands.
• You enter a line command for a member that was deleted. The names of deleted members are not
removed from the member list until it is updated. Remove the line command that precedes the deleted
member, and press Enter again. See “Updating a member list” on page 100 for more information.
• You enter an R (rename) line command, but do not put a new name in the Prompt field. Enter a new
member name, and press Enter again.
You can then perform one of these actions:
• Enter additional primary or line commands
• Scroll, if necessary, to bring additional members into view
• Enter the END command to return to the previous panel.
Using member selection lists
Chapter 4. ISPF libraries and data sets  99

## Page 128

The next two figures show before and after examples that print members TEST and TEST1, delete
member TEST8, and rename member TEST4 to OLDTEST.
   Menu  Functions  Confirm  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
 LIBRARY           USERID.TEST.SOURCE                    Row 0000001 of 0000009
    Name     Prompt          Size    Created           Changed            ID
 P TEST                         1   2003/02/03   2003/02/03 17:04:14    USERID
 P TEST1                        1   2003/02/03   2003/02/03 17:04:14    USERID
   TEST2                        1   2003/02/03   2003/02/03 17:04:23    USERID
   TEST3                        1   2003/02/03   2003/02/03 17:04:14    USERID
 R TEST4    OLDTEST             1   2003/02/03   2003/02/03 17:04:14    USERID
   TEST5                        1   2003/02/03   2003/02/03 17:04:14    USERID
   TEST6                        1   2003/02/03   2003/02/03 17:04:14    USERID
   TEST7                        1   2003/02/03   2003/02/03 17:04:14    USERID
 D TEST8                        1   2003/02/03   2003/02/03 17:04:14    USERID
   **End**
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 21. Library Utility before Print, Rename, and Delete (ISRUDMM)
   Menu  Functions  Confirm  Utilities  Help
 ──────────────────────────────────────────────────────────────────────────────
 LIBRARY           USERID.TEST.SOURCE                    Row 0000001 of 0000009
    Name     Prompt          Size    Created           Changed            ID
 _ TEST     *Printed            1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST1    *Printed            1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST2                        1   2003/02/03   2003/02/03 17:04:23    USERID
 _ TEST3                        1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST4    *Renamed
 _ TEST5                        1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST6                        1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST7                        1   2003/02/03   2003/02/03 17:04:14    USERID
 _ TEST8    *Deleted
   **End**
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 22. Library Utility after Print, Rename, and Delete (ISRUDMM)
Updating a member list
When a member list is redisplayed after the completion of a function or command, it does not include
these types of member:
Using member selection lists
100  z/OS: z/OS ISPF User's Guide Vol I

## Page 129

• For View, Browse, and Edit:
– New members created by recursive Edit calls.
– New members created using the CREATE command in EDIT. These do not display in an Enhanced
Member List display, but do display when using the traditional member list processing.
– Members created by another user.
– In split-screen mode, members created on another logical screen.
• For Library, Move/Copy, Data Set List, and Convert:
– New names of members that have been renamed.
– Fewer member names when members are deleted (Library and Data Set List) or moved (Move/Copy).
To display an up-to-date list, return to the previous panel, leave the member name blank or enter a
pattern. See “Displaying member lists” on page 82 for more information about displaying member lists.
To set your system to display a traditional member list when using the E, V, or B commands (Edit, View,
Browse) specifically, see the topic about DSLIST Settings in the z/OS ISPF User's Guide Vol II. This can be
found in the Utilities (Option 3) topic, in the information about Data Set List Utility Options.
Member list positioning
After selected members have been processed, the member list is redisplayed and positioned based on
the members selected and the setting of the Scroll Member List option.
When the Scroll Member List option is selected:
• If a single member is selected, the member list is redisplayed with the selected member scrolled to the
top of the display. However, if the CANCEL command is used to exit the selected member, the member
list is not scrolled.
• If multiple members are selected and some selections are contained in the last screen displayed before
the Enter key was pressed, the first selected member on that screen is scrolled to the top of the display.
• If multiple members are selected and none of the selections are contained in the last screen displayed
before the Enter key was pressed, the first selected member in the member list is scrolled to the top of
the display.
When the Scroll Member List option is not selected:
• If only single selections are allowed and the selection is contained in the last screen displayed before
the Enter key was pressed, the member list is not scrolled. The cursor is positioned in front of the
selected member.
• If only single selections are allowed and the selection is not contained in the last screen displayed
before the Enter key was pressed, the member list is redisplayed showing the last screen containing the
selected member. The cursor is positioned in front of the selected member.
• If multiple selections are allowed and the last selected member is contained in the last screen
displayed before the Enter key was pressed, the member list is not scrolled. The cursor is positioned in
front of the last selected member.
• If multiple selections are allowed and the last selection is not contained in the last screen displayed
before the Enter key was pressed, the member list is redisplayed with the last selected member
scrolled to the top. The cursor is positioned in front of the last selected member.
Using member generation selection lists
Use the N line command on a member list displayed in option 3.1 and 3.4 to display a member generation
list. A generation list only displays for members of PDSEs that support generations. The generation
list panel (ISRUGEN) is similar to the member list panel (ISRUDSM). However, there are some subtle
differences. Additionally, not all the functions available in member lists are supported in generation lists.
From a display perspective, there are two distinct differences between the two panels.
Using member generation selection lists
Chapter 4. ISPF libraries and data sets  101

## Page 130

1. On the title line, the member name precedes the data set name so that the member name is visible
even when a message displays.
2. On the heading line, the generation list contains "RGEN" rather than "Name".
RGEN stands for relative generation number. Relative numbers used for generations are negative
numbers. The negative sign is implied in this list. It is also worth noting that this list is limited to 8
characters so the lowest relative number that can be displayed is "-99999999". The actual number of
generations that can be allocated for a data set member is 2 trillion.
All other fields on this panel, including the statistics and prompt field are the same fields displayed for
member lists. Here is an example of Generation List panel, ISRUGEN:
Figure 23. Example panel of Generation List (ISRUGEN)
The line commands supported on a generation list are a subset of what is supported on a member list.
The supported line commands are:
• B (browse generation)
• D (delete generation)
• E (edit generation)
• I (display generation information)
• P (print generation)
• V (view generation)
• / (option list)
Using member generation selection lists
102  z/OS: z/OS ISPF User's Guide Vol I

## Page 131

Multiple line commands are not supported. Only the first line command entered on the panel is executed.
Subsequent line commands are ignored. TSO commands are not supported on this panel.
Primary command support is also limited from the generation list. The available commands are LOCATE,
CONFIRM, and REFRESH.
To obtain information about the member including the absolute generation number, issue the I line
command on the generation. The I command displays the Member Generation Information panel,
ISRUMGI, which contains the following information:
Figure 24. Example panel of Generation Information (ISRUMGI)
Generation numbers explained
Generation numbers are used to identify specific versions of a data set member. When you create a
member for the first time, the initial version is assigned a generation number of 0. Generation 0 is referred
to as the current generation. All subsequent generations are non-current generations. Generations have
two assigned numbers, an absolute value and a relative value.
Absolute numbers
Absolute numbers are positive numbers that are assigned when non-current generations are created and
they are not reused. When you create a generation, the current generation (0) is always updated to reflect
the newest version. Hence generation 0 is always the newest version. The current version that previously
existed is assigned an absolute number of the next available positive number in ascending order. When
you delete a non-current generation, absolute numbers are not reused. For example, assume you have
Generation numbers explained
Chapter 4. ISPF libraries and data sets  103

## Page 132

three generations numbered 0, 1, and 2. If you delete generation 1, you have a gap in your generations,
leaving you with 0 and 2. Creating a new non-current generation results in generations 0, 2, and 3.
Rules for absolute numbers and non-current generations:
• The first non-current generation is assigned a value of 1.
• If you create a second non-current generation, it is assigned a value of 2.
• The highest absolute number assigned to a member is always the most recent non-current generation.
• If you delete a non-current generation, the absolute number assigned to that generation is removed and
is not used again. (0 is the exception to this rule).
Relative numbers
Relative generations are negative numbers that represent the order in which the generations are created
and updated. The first non-current generation is assigned a relative number of -1. In fact, anytime
you create a new non-current generation it is assigned a value of -1. When you create a non-current
generation, all relative numbers of any prior non-current generations are decremented by 1, so -1
becomes -2, -2 becomes -3, and so on. Unlike absolute numbers, there is never a gap in relative numbers
as these numbers are not assigned to a particular generation version. As new generations are added or
deleted, relative numbers are adjusted to account for the order of most to least recent with -1 always
being the most recent. The new generation list panel, ISRUGEN, displays generations by using the relative
numbering scheme, which is logically simpler in nature.
Here is an example of how generation numbers would be assigned assuming a total of four generations
were created without deletion.
MEMBER (XYZ) Relative Generation Absolute Generation
Current/Newest 0 0
-1 3
-2 2
Oldest -3 1
Generation 0
It is important to understand that generation 0 is unique in that it is the primary version of the member. If
you access the member without specifying a generation number, the generation number defaults to 0.
ISPF does not allow generation 0 to be deleted. However, there are other products that can be used to
delete generation 0.
Note:
• If you delete generation 0, the remaining generations are orphaned.
• If generation 0 is deleted, the member is not displayed in the member list.
• If a member has orphaned generations, you can access non-current generations by using option 1,
View, or option 2, Edit, and supplying the member name on the panel with an existing non-current
generation.
• If you edit generation -1 of an orphaned member by using option 2, Edit, and then save the generation
(SAVE NEWGEN primary command), the editor creates generation 0. As a result, the generations are no
longer orphaned.
z/OS UNIX directory selection lists
A z/OS UNIX directory selection list is displayed when you specify the pathname for a directory:
• On the View and Edit entry panels (ISPF options 1 and 2).
z/OS UNIX directory selection lists
104  z/OS: z/OS ISPF User's Guide Vol I

## Page 133

• On a call to the BROWSE, EDIT, and VIEW services.
• With the edit and view MOVE and COPY primary commands.
• On the EDIT, VIEW, and BROWSE command entry panels displayed when using these primary
commands within the browse, view, or edit function.
The directory selection list is almost identical to the list displayed from the z/OS UNIX Directory List Utility
(PDF option 3.17). The selection list supports the S (select) line command, allowing you to select the file
to be processed with the function that invoked the list.
For the directory list displayed by the edit MOVE and COPY commands, the only valid line commands are S
(Select), B (Browse), and L (List subdirectory). For all other directory selection lists, all the line commands
supported by the z/OS UNIX Directory List Utility are also supported.
Data set passwords
A Data Set Password field is included on library and data set entry panels:
Data Set Password  . .           (If password protected)
The Data Set Password field contains the password for OS password-protected data sets. By assigning
more than one password to the same data set, you can give some users read-only access while giving
others read/write access.
Nondisplay input fields are used so that the passwords do not appear on the screen. When you specify a
concatenated sequence of libraries, the password applies to all data sets in the sequence.
If you replace a long password with a shorter password, blank out the remaining spaces of the Data Set
Password field.
You can use ISPF with the Resource Access Control Facility (RACF). RACF provides extensive facilities for
data set security. However, when using RACF, do not enter a password on the ISPF panels, because RACF
relies on your TSO user ID and logon password to identify you and check for proper authorization.
Format definitions
A Format Name field is included on the View Entry Panel and on the Edit Entry Panel:
Format Name  . . . ________
The Format Name field contains the name of a format definition, which is used to view, browse, or edit
a formatted data set. A formatted data set contains records that consist of subfields. The locations and
lengths of these subfields are fixed throughout the data set. The formatted data set support in View,
Browse, and Edit is particularly useful for data that contains double-byte character (DBCS) data but does
not contain shift-out (SO) and shift-in (SI) characters.
The format name can consist of up to eight alphanumeric characters; the first one must be alphabetic.
A format definition can include Extended Binary Coded Decimal Interchange Code (EBCDIC) fields, DBCS
fields, and mixed fields. If the specified format includes a mixed field definition, the Mixed Mode field
is ignored, even if you select it. See “Mixed mode” on page 106 for information. For information about
defining formats for formatted data sets, see the topic about the Format Specifications Utility (Option
3.11) in the z/OS ISPF User's Guide Vol II. The Format Specifications utility is provided to support the IBM
5550 terminal that uses DBCS.
When formatted data is displayed, an attribute character that does not reside in the data set and is not
stored in the data set precedes each field. Therefore, the column position on the display is different from
the column position in the data set.
The allowable maximum length is decreased two bytes per field definition from the standard View,
Browse, and Edit allowable maximum length.
Data set passwords
Chapter 4. ISPF libraries and data sets  105

## Page 134

Mixed mode
A Mixed Mode field is included on the View Entry Panel and the Edit Entry Panel:
_ Mixed Mode
The Mixed Mode field specifies whether you want to view, browse, or edit unformatted mixed data that
contains both EBCDIC (single-byte) and DBCS (double-byte) characters. Use a slash to select mixed
mode. If your terminal does not support DBCS, the value in this field is ignored.
DBCS strings are enclosed with SO (X'0E') and SI (X'0F') characters in unformatted mixed data. The SO
character precedes the DBCS character string and the SI character follows the string.
If the view, browse, or edit line contains mixed data that are not valid, ISPF assumes the line can contain
only EBCDIC characters. Examples of mixed data that are not valid include:
• Unpaired SO and SI characters
• Incorrect DBCS characters between SO and SI characters
• An odd number of bytes between SO and SI characters.
If you call View, Browse, or Edit from the Library utility (option 3.1) or the Data Set List utility (option 3.4),
ISPF assumes that you want to use mixed mode.
If you want to view, browse, or edit DBCS data as EBCDIC data, you must do so in non-mixed mode. You
can do this by operating from a terminal that does not support DBCS or by deselecting the Mixed Mode
field.
In non-mixed mode, SO and SI characters are not treated as special characters; instead, they are treated
as characters that cannot be displayed. Thus, you can view, browse, or edit the data in the conventional
way.
You can also view, browse, or edit DBCS data in hexadecimal format, just as you would EBCDIC data.
For information about specifying hexadecimal display, see the information about "HEX-Displaying Data in
Hexadecimal Format" in the View (Option 1) topic in the z/OS ISPF User's Guide Vol II.
Note: Do not edit a record in hexadecimal format when a DBCS string encroaches on the display
boundary.
DBCS data that is not valid is not supported. If DBCS fields or DBCS strings in a mixed field contain any
bytes with hexadecimal code ranging from X'00' to X'3F', you may get unwanted results.
Partitioned Data Set Extended (PDSE)
Partitioned Data Set Extended (PDSE) is a data set type that is managed by DFSMS. Externally, a PDSE is
very similar to a PDS. Internally, the PDSE has a different directory structure, member format, and record
format. A PDSE is indistinguishable from a PDS through most interfaces used to access a PDS directory or
member. All ISPF functions support the PDSE.
You can concatenate a PDSE library with a PDS library if they have consistent record formats and logical
record lengths. All functions in the Library Utility (option 3.1) support PDSEs with the exception of the
compress function.
Packed data sets
The packed data set format allows you to use direct access storage devices (DASD) more efficiently.
In this format, ISPF replaces any repeating characters with a sequence showing how many times the
character is repeated. Before you can properly use data stored in this format as input to processing
programs, such as compilers, you must first tell ISPF to unpack and expand the data.
The two requirements for using packed data sets are:
• To store data in packed format:
Mixed Mode
106  z/OS: z/OS ISPF User's Guide Vol I

## Page 135

– Enter the PACK ON Edit primary command while editing a data set or PDS member.
– Select the Pack Option field (under To Data Set Options:) when copying or moving members using the
Move/Copy utility (option 3.3).
• To unpack and expand packed data for processing, select the Source Data Packed field on the
Foreground Selection panel or the Batch Selection panel. You must select this field if any of the input
data, including that referred to in COPY or INCLUDE statements, is in packed format.
List and log data sets
ISPF helps you get hardcopy listings of source modules, and maintains a log of significant user activities.
These items are kept in data sets called the list data set and the log data set, respectively.
When needed, the two data sets are allocated automatically. They are temporary data sets named:
prefix.userid.SPFn.LIST
prefix.userid.SPFLOGn.LIST
Note: The data set name used can be modified under the operation of site-defined options. See the
section "Temporary data set names" in ISPF Planning and Customizing Guide.
prefix
The data set prefix in your TSO profile. Used only if it is different from your user ID.
userid
Your user ID.
n
A number from 0 to 9.
If you have specified in your TSO profile a data set prefix that differs from your user ID, the data set
names begin with your data set prefix, followed by your user ID. Once generated, these data sets remain
open throughout your ISPF session. However, even though they are open, you can still process them by
using the ISPF LIST and LOG commands.
List data set
The list data set is used for temporary storage for data to be printed at a later time. This data includes, for
example, data written as a result of:
• Using the LIST service
• Issuing the PRINT, PRINT-HI, PRINTL, or PRINTLHI commands (but not PRINTG)
• Using Option 3 utilities.
To avoid generating an ISPF list data set, do not request any print functions.
Log data set
The log data set is used to capture data that can be useful for such things as diagnosing problems. This
data includes, for example, data written as a result of:
• Using the LOG service
• Test and trace data such as:
– ISPF TRACE mode data
– Dialog Test option 7.7 dialog trace data.
Use the Log/List pull-down from the ISPF Settings panel action bar to prevent generating the ISPF log
data set. However, if you use the Dialog Test option (7), allow generating the log data set because Dialog
Test writes trace data to the log when you request it. Also, if Dialog Test finds an unexpected condition,
problem data and error messages are written to the log.
List and Log Data Sets
Chapter 4. ISPF libraries and data sets  107

## Page 136

Processing the log and list data sets
You can process the log and list data sets either:
• During an ISPF session, using the LOG and LIST commands
• At the end of a session.
ISPF processes (prints, keeps, deletes) only data sets that it has allocated. Any attempt to process a
log or list data set that has been preallocated by the user results in an appropriate ISPF message. Any
references to ISPF processing of log or list data sets refer to data sets that ISPF has allocated. Users can
supply routines to process preallocated data sets after ISPF has terminated.
How to specify log and list data set processing options
The log and list data set processing options can be specified through any of these:
• Use of the LOG and LIST commands during an ISPF session.
• Use of the Log/List pull-down on the ISPF Settings panel for setting default options.
• The ISPF termination panel, which can display when you exit from ISPF. See “Log and list data set
processing at the end of a session” on page 110 to find out under what conditions ISPF will display this
panel.
Processing the log and list data sets during an ISPF session
The LOG and LIST commands allow you to process the log and list data sets, respectively, at any time
during an ISPF session. The log and list data sets must have been allocated. You control the data set
processing by specifying on the LOG or LIST command one of the three keyword options: PRINT, DELETE,
or KEEP.
If you issue the LOG or LIST command with no parameter specified, ISPF displays a panel that allows you
to select the data set processing options. The panels for the LOG and LIST commands are shown in Figure
25 on page 108 and Figure 26 on page 109, respectively.
                      Specify Disposition of Log Data Set
   Log Data Set (USERID.SPFLOG2.LIST) Disposition:
     Process Option . . . .    1. Print data set and delete
                               2. Delete data set without printing
                               3. Keep existing data set and
                                  continue with new data set
     Batch SYSOUT class . .                
     Local printer ID or
     writer-name  . . . . .                  
     Local SYSOUT class . .                
   Press ENTER key to process the log data set.
   Enter END command to exit without processing the log data set.
 Job statement information:  (Required for system printer)
  ===>                                                                         
  ===>                                                                         
  ===>                                                                         
  ===>                                                                         
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 25. Log Data Set Defaults Panel (ISPLLP01)
List and Log Data Sets
108  z/OS: z/OS ISPF User's Guide Vol I

## Page 137

Specify Disposition of List Data Set
   List Data Set (USERID.SPF1.LIST) Disposition:
     Process Option . . . .    1. Print data set and delete
                               2. Delete data set without printing
                               3. Keep existing data set and
                                  continue with new data set
     Batch SYSOUT class . .                
     Local printer ID or
     writer-name  . . . . .                  
     Local SYSOUT class . .                
   Press ENTER key to process the list data set.
   Enter END command to exit without processing the list data set.
 Job statement information:  (Required for system printer)
  ===>                                                                         
  ===>                                                                         
  ===>                                                                         
  ===>                                                                         
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 26. List Data Set Defaults Panel (ISPLLP02)
With the appropriate panel displayed, type in the process option of your choice. If you specify Print data
set and delete, you must also specify a Batch SYSOUT class, or local printer ID or writer name. After you
have typed in all information that you wish to specify, press Enter to pass the input to ISPF. ISPF takes the
specified action for the data set and then returns you to the panel from which you issued the LOG or LIST
command. ISPF issues a message indicating whether the action requested was successful.
If you issue the END command from the Log or List Data Set Defaults panel, ISPF returns you to the panel
from which you issued the LOG or LIST command without processing the data set.
ISPF initializes the Log or List Data Set Defaults panel fields with the default values specified with the
Log/List pull-down on the ISPF Settings panel. If a default disposition of Keep data set has been specified,
ISPF translates the value to Keep data set and allocate new data set before displaying the panel. If you
modify the process option field, the new value is used to process the data set; however, it is not saved in
the system profile. All other fields modified on the panel are saved in the system profile and become the
default values the next time the data set is processed.
If you issue the LOG or LIST command with the PRINT, DELETE, or KEEP option, ISPF does not display
a panel. Specifying PRINT, DELETE, or KEEP on the command causes data set processing equivalent
to specifying Print data set and delete, Delete data set, and Keep data set and allocate new data set,
respectively, on the Log or List Data Set Defaults panel.
Two system variables, ZLOGNAME and ZLSTNAME, contain the fully qualified names of the log and list
data sets, respectively. If either data set is not allocated or has not been used in the session, the
corresponding system variable value is blank.
Note: The values of ZLOGNAME and ZLSTNAME are set to blank immediately after the log and list data
sets have been processed because the data sets are freed by the LOG/LIST command processing. A new
data set will not be allocated until it is written to. If you intend to use the log or list data set name for your
processing, be sure to retrieve it before issuing the LOG or LIST command.
The system variables are summarized in z/OS ISPF Dialog Developer's Guide and Reference.
Conditions for using the LOG and LIST commands
You can issue the LOG or LIST command from any command line except in these situations:
• The command panel for the related log or list data set is active in any logical screen.
List and Log Data Sets
Chapter 4. ISPF libraries and data sets  109

## Page 138

• The ISPF termination panel is active.
• The data set to be processed is not allocated or was preallocated.
• Dialog Test option 7.5 (Browse ISPF log) is active, and you are attempting to process the log data set.
Log and list data set processing at the end of a session
Figure 27 on page 110 shows the panel that ISPF displays at the end of a session if one of these is true:
• The initial dialog began with the display of a menu, and the dialog is ended with the END command
issued from that menu.
• The initial dialog began with the performance of a function, and the function ends with a return code of
0.
• The log and list data set processing defaults have not been specified, or the default values are not valid.
If the application ends with a nonzero return code, the termination panel is not displayed.
If the termination panel does not display for one of these reasons, the log and list data sets are processed
using the default options.
                 Specify Disposition of Log and List Data Sets
                                                                    More:     +
 Log Data Set (USERID.SPFLOG2.LIST) Disposition:
 Process Option . . . .    1. Print data set and delete
                           2. Delete data set without printing
                           3. Keep data set - Same
                              (allocate same data set in next session)
                           4. Keep data set - New
                              (allocate new data set in next session)
 Batch SYSOUT class . .                
 Local printer ID or
 writer-name  . . . . .                  
 Local SYSOUT class . .                
 List Data Set (USERID.SPF1.LIST) Disposition:
 Process Option . . . .    1. Print data set and delete
                           2. Delete data set without printing
                           3. Keep data set - Same
                              (allocate same data set in next session)
                           4. Keep data set - New
                              (allocate new data set in next session)
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 27. Specify Disposition of Log and List Data Sets Panel (ISPPFT03)
The valid process options shown in Figure 27 on page 110 are described in “Data set processing options”
on page 110.
Data set processing options
For each term defined here, the first value shown is the processing option that you can specify on the
Log/List pull-down from the ISPF Settings panel, on the Specify Disposition of Log and List Data Sets
panel, or on the Log or List Data Set Defaults panel. The value in parentheses is the corresponding LOG or
LIST command parameter.
1. Print data set and delete (PRINT)
Print the data set, then delete it. You must specify a Batch SYSOUT class or local printer ID or writer
name.
List and Log Data Sets
110  z/OS: z/OS ISPF User's Guide Vol I

## Page 139

• If the Batch SYSOUT class is specified, ISPF submits a background job to print and deletes the data
set or sets.
• If a local printer ID or writer name is specified, ISPF uses the TSO PRINTDS command to route the
data set to the specified printer or external writer program and then deletes the data set.
Note: If you have selected Edit PRINTDS Command on the ISPF Settings panel (option 0), ISPF
displays the Local Print Command Edit panel to allow you to intercept and edit the PRINTDS
command before it processes. See “Editing the PRINTDS command” on page 111 for additional
information.
ISPF uses file tailoring services to print data on a system printer. Therefore, if this option is specified
during an ISPF session, along with a Batch SYSOUT class, file tailoring must not be active on the
logical screen from which the LOG or LIST command is issued. If an FTOPEN or FTINCL has been
issued without a subsequent FTCLOSE, ISPF issues an appropriate message.
2. Delete data set without printing (DELETE)
Delete the data set.
3. Keep data set - Same
Not applicable to LOG or LIST command. Close and free the data set. For the LOG or LIST data set,
allocate the same data set at the beginning of the next session. If the data set does not exist, ISPF
creates one with the same name.
4. Keep data set - New (KEEP)
Close and free the data set. Allocate a different data set for the next time log or list information is
generated in this session or in the next session.
Editing the PRINTDS command
If you have selected Edit PRINTDS Command on the ISPF Settings panel (option 0) and you specify a local
printer ID or writer name on either the Log and List Data Set Termination Options panel or the Hardcopy
Utility panel, ISPF displays the Local Print Command Edit panel shown in Figure 28 on page 111 to allow
you to edit the PRINTDS command before it processes.
                            Local Print Command Edit
 Select function to perform and press Enter to exit and print.
 End or Cancel will exit without printing.
 Local Print Command Options:
    Function to perform  . . . 1  1. Exit and issue PRINTDS command
                                  2. Exit without printing
 PRINTDS Header:
  . . : PRINTDS DATASET('USERID.SPFLOG3.LIST') DEST(PRINTER1) CCHAR
 Configuration table PRINTDS operands:
  . . . NONUM                                                                  
                                                                               
 User PRINTDS operands:
  . . .                                                                        
                                                                               
                                                                               
                               
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 28. Local Print Command Edit Panel (ISPCHPLP)
The fields on this panel function as follows:
List and Log Data Sets
Chapter 4. ISPF libraries and data sets  111

## Page 140

Function to perform
Specify the print function you want ISPF to perform:
1
Exit ISPF and issue the PRINTDS command (as edited)
2
Exit ISPF without printing.
Note: If you arrive at this panel from ISPF termination processing, you will continue with
termination and exit the product after your print request is issued or canceled.
PRINTDS Header
This field cannot be edited. It contains the PRINTDS command, the data set name, the printer ID or
writer name, and the CCHAR operand, if appropriate.
Configuration table PRINTDS operands
These operands operate at a system level and can be altered only in the ISPF Configuration table.
User PRINTDS operands
Enter additional operands (for example, COPIES or FORMS). These operands can be edited and are
saved in the application command table.
If you enter CANCEL (or select Cancel), the PRINTDS command is not issued. If you enter END or RETURN
or use a jump function, the PRINTDS command is issued and you receive a completion message.
Foreground and batch output listings
These additional listing data sets are allocated as needed for foreground or batch processing:
prefix.userid.list-id.LIST
prefix.userid.list-id.LINKLIST
prefix.userid.list-id.TERM
prefix.userid.list-id.TESTLIST
prefix
The data set prefix in your TSO profile. Use it only if you have one and it is different from your user ID.
userid
Your user ID.
list-id
The name specified in the List ID field on the foreground or batch data entry panel. This name is
required for sequential data sets. However, for partitioned data sets, the member name becomes the
default list-id if the List ID field is blank.
The particular data set names you use depend on the foreground or batch processing option chosen.
For batch processing, the output can either be directed to a list data set or printed as part of the batch
job. When batch processing is finished, you can browse the list data set, and then use the Hardcopy utility
(option 3.6) to print it. Using this utility, show whether you want to keep the data set or delete it after
printing. ISPF does not delete these data sets when you end ISPF.
For the foreground option, the output listing is directed to a list data set and automatically displayed for
browsing. When you end the browse function, ISPF displays a selection panel that allows you to choose
whether to print, keep, or delete the list data set. Again, ISPF does not delete this data set when you end
ISPF.
Other temporary data sets
If you are using virtual I/O (VIO), you can allocate space for temporary data sets, and then VIO assigns
them system-generated names. Otherwise, ISPF allocates temporary control and listing data sets, as
Foreground and batch output listings
112  z/OS: z/OS ISPF User's Guide Vol I

## Page 141

needed, for its own internal use. You are usually not aware of their existence. They are assigned these
names:
prefix.userid.SPFTEMPn.CNTL
prefix.userid.SPFTEMPn.LIST
prefix.userid.SPFTEMPn.WORK
prefix.userid.appl-idzzzz.BACKUP
prefix.userid.appl-idzzzz.BACKUPI
prefix.userid.SPFnnn.OUTLIST
Note: The data set name used can be modified under the operation of site-defined options. See the
section "Temporary data set names" in ISPF Planning and Customizing Guide.
prefix
The data set prefix in your TSO profile. It is used only if you have one and it is different from your user
ID.
userid
Your user ID.
n
A number that corresponds to the logical screen that is active. n can be between 0-9 and A-W for
CNTL data sets and between 1-9 and A-W for LIST and WORK data sets, where 1 is the first logical
screen, 9 is the ninth logical screen, A is the tenth logical screen, and so on.
appl-id
The application ID.
zzzz
A number from 0001-0008, or higher if customized, controlled by the edit recovery table (appl-
id EDRT for the EDREC service and appl-id EIRT for the EDIREC service) and the number of concurrent
edit calls that are active.
nnn
A number generated by ISPF, which has a range of 100-999.
These data sets are deleted:
• By edit recovery when the data sets are no longer needed
• When you specifically request that they be deleted.
• By Move/Copy when no IEBCOPY errors are encountered.
Job statement information
ISPF allows you to submit Batch jobs for printing and language processing. However, before submitting a
Batch job, you must supply job statement information. For this purpose, four lines are provided on each
job submission panel.
You can use the lines that contain //*:
• As continuation lines by removing the asterisk (*)
• To enter other JCL statements, such as JOBLIB DD.
If you do not need these lines, you can blank them out. Blank lines are not submitted to the job stream.
Job statement information
Chapter 4. ISPF libraries and data sets  113

## Page 142

Job statement information
114  z/OS: z/OS ISPF User's Guide Vol I
