# Appendix E. ISRDDN diagnostic utility

Source file: f54ug00_v3r1.md
Start page: 187
Page span: 187-206

## Page 187

Appendix E. ISRDDN diagnostic utility
ISRDDN is a utility that assists IBM support in evaluating and solving problems. It provides a list of
allocated ddnames, a list of system ENQs, a list of data sets causing system ENQ contention, and a means
of viewing storage within a TSO user's address space. ISRDDN also provides some facilities for gathering
information about your system environment.
You can start ISRDDN by issuing the commands TSO ISRDDN or DDLIST from any ISPF command line.
The allocated ddname list shows you all of the ddnames allocated to your TSO session. From the list
you can perform functions such as Edit or Compress against individual data sets, ddnames, or sets of
ddnames. You can also perform actions against the entire list of displayed ddnames.
The ENQ list, available by typing ENQ on the allocation list command line, shows you ENQs on your
system. You can limit the size of the list by specifying the QNAME, RNAME, job, user or address space
name, and system name.
The ENQ contention list, available by typing CON on the allocation list command line, shows you ENQ
contentions on your system for data sets (QNAME SYSDSN).
You can Browse storage using the BROWSE primary command from the allocation list. You can only
browse storage which an unauthorized program can see (private and common).
ISRDDN can be used to manipulate the data sets that are allocated, but it also provides the ability to
answer questions like:
• Where did a module the user has loaded come from?
• What data sets contain a specific member?
• Are the I/O errors and ABENDs the user is getting due to mixed record format allocations?
• Who is currently allocated to "SYS1.BRODCAST"?
• What member names or LPA load modules are duplicated in the user's current allocations?
• How many members are in the allocated libraries and which libraries are empty?
• Is the user running ISPF out of LPA or from STEPLIB?
To invoke the ISRDDN program, type TSO ISRDDN on any ISPF command line.
The Current Data Set Allocations list
When you start ISRDDN, the Current Data Set Allocations list displays, as shown in Figure 42 on page 160.
© Copyright IBM Corp. 1980, 2024 159

## Page 188

Figure 42. Current Data Set Allocations List panel
On the right side of the display is a list of ddnames and their associated data sets. The list of data sets can
also contain indicators of DUMMY allocations, subsystem files, or allocations to the terminal. The ddname
is shown in white, unless the first data set in the concatenation is scrolled off the top of the screen. If the
first data set in a concatenation is not on the screen, the ddname is shown in yellow.
In the center of the display is a column of 1-character input fields, preceded by greater-than signs (>).
These input fields are used for line commands such as E for Edit and I for Information. For data set with
an XTIOT, this input field may be unavailable if XTIOT support is not enabled in the ISPF configuration
table. Data sets with an XTIOT have the data set name displayed in yellow. A column with a heading of X is
displayed next to the ENQW column. A value of Y is displayed in this column if the data set has an XTIOT.
This column is shown in Figure 44 on page 162.
The left side of the display contains columns of information about individual data sets. When you scroll
right or left, the left side of the screen changes. Initially, the left side of the screen contains the volume
name and disposition. If the disposition is red, there are other jobs waiting to use this data set as shown.
You can use the Q line command to see what jobs are waiting. You can view the VTOC information for a
volume by placing the cursor on the volume name and pressing the Enter key.
ISRDDN automatically checks for mixed concatenations when it is started. If you have concatenations of
mixed data set types or formats, you are shown a message to that effect when you press the Enter key or
scroll the first time. ISRDDN also checks for mixed concatenations when you use the RESET command.
If you scroll right once, you see the attributes of each data set, as shown in Figure 43 on page 161.
160  z/OS: z/OS ISPF User's Guide Vol I

## Page 189

Figure 43. Data Set Attributes in ISRDDN
For some types of allocations, such as subsystem allocations, you might see different information. If you
have mixed concatenations, a message with this information appears when you press the Enter key or
scroll the first time. You can suppress this message for future innovations of ISRDDN by using the CHECK
OFF command.
If you scroll right a second time, you see information that includes whether the ddname is open and if so,
by how many active DCBs, as shown in Figure 44 on page 162.
Appendix E. ISRDDN diagnostic utility  161

## Page 190

Figure 44. Additional DD Information
You also see the indicator *SMS* if the data set is SMS-managed, and information about jobs waiting on
the resource. For JES files you might see additional information such as the class and the writer name.
Some primary commands, MEMBER and COUNT, for example, put messages in a fourth status screen
(Figure 45 on page 163).
162  z/OS: z/OS ISPF User's Guide Vol I

## Page 191

Figure 45. Additional DD Information
If messages exist and you scroll right again, you see the messages. The message screen is only shown if
messages exist. If messages do not exist, a third scroll to the right returns you to the initial screen.
Using commands on the displayed list
The Current Data Set Allocations list supports both primary commands and line commands. The displayed
list is the list of ddnames that you can see by scrolling up and down. You can use primary commands
to limit what is displayed in the list. Many of the primary commands work only on the contents of the
displayed list.
ISRDDN can also create pseudo-ddnames that show useful data set names. For example, the LPA
command adds two pseudo-ddnames, LINKLIST and LPALIB, which contain lists of the current link list
and LPA libraries.
Allocation list primary commands
Primary commands are used to limit the contents of the displayed list, to add pseudo-ddnames, to
operate on all the contents of the displayed list and to invoke other ISRDDN options.
All primary commands can be invoked with their minimum unique names. For example, MEMBER can be
abbreviated as M, while CLIST can be abbreviated as CL. The allocation list primary commands follow.
You can specify an initial primary command when you start ISRDDN. For example, if you enter DDLISTB
10.??? on an ISPF command line, you will immediately browse the storage containing the TCB control
block. When you exit the Browse screen, you are not returned to the DD allocation list. This feature is
useful for calling ISRDDN from within a program when, for example, you want to limit the list to specific
dd names, view ENQs, save the current allocations, or browse storage.
Appendix E. ISRDDN diagnostic utility  163

## Page 192

Only (O) and Exclude (EX, X)
ONLY and EXCLUDE are used to limit the ddnames in the displayed list. They take one operand: a
whole or partial ddname. For example, the command O PLI causes the list to contain only ddnames
that contain the string "PLI", such as STEPLIB and ISPPLIB.
The ONLY and EXCLUDE commands are useful when you want to limit the ddnames or pseudo-
ddnames that are operated on by commands like MEMBER and DUPLICATES. They are also helpful in
reducing the size of the displayed list for easier viewing.
Find (F) and Locate (L))
FIND and LOCATE search the list for a string. LOCATE looks only at ddnames and always locates the
first matching ddname. FIND looks at everything currently in the displayed list and finds the next
occurrence of the string following the current cursor position. You can repeat a FIND operation by
pressing the RFIND key.
When a string is found by FIND, the string is highlighted and the cursor is placed on the string. When a
string is found by LOCATE, the string is highlighted and the cursor is placed in the line command area
next to the located ddname.
Reset (R))
The RESET command rebuilds the list. In most screen formats the list is automatically rebuilt when
you press Enter. However, if you have used the COUNT command or the MEMBER command and have
messages showing in the list, you might need to use the RESET command to refresh the list.
Short (S) and Long (LON)
The SHORT and LONG commands alter the format of the list. The SHORT command places the
ddname of a concatenation next to the first data set (as shown in Figure 42 on page 160). The LONG
command formats the list with ddnames of concatenations placed on a separate line before the data
set names (as shown in Figure 46 on page 164). 
Figure 46. Current Data Set Allocations List in LONG Format
The SHORT format shows more information on one screen. Use the LONG format when you want to
use line commands that operate on whole concatenations, such as E and V, on only the first data set
in a concatenation.
164  z/OS: z/OS ISPF User's Guide Vol I

## Page 193

Member (M)
The MEMBER command is a very useful command in ISRDDN. MEMBER searches the displayed list (or
just ddnames containing a given string) for a member whose name matches a pattern. For example,
the command M ISRSUBS searches the data sets in the displayed list, the job pack area, and the link
pack directory for members named ISRSUBS. Data sets that contain the member are flagged with a
message on the left side of the list, as shown in Figure 47 on page 165. 
Figure 47. Results of the MEMBER Command
If the name is the name of a loaded module in the job pack area or LPA, you also see a panel similar to
the one in Figure 49 on page 168.
When a member name is used on the MEMBER command (such as, M ISRSUBS) and an E, V, or B
line command is used next to a data set in which that member is found, ONLY that member is Edited,
Viewed, or Browsed. When the M line command is used, the member list is shown with the selected
member at the top of the list.
When a member name pattern is used on the member command (such as, M ISR*), the E, V, B, and M
line commands display member lists with members that match the given pattern.
Use the MEMBER command in situations when you do not know from where a member is coming or
when you suspect that you might be accessing the wrong copy of a member. For example, if you are
developing ISPF panels and you do not see your version of the panel being displayed, you can issue
the MEMBER command to search for other copies of the panel.
Usually the MEMBER command operates on the entire displayed list. You can add a second operand
that is a partial ddname. For example, the command M ISRSUBS PL searches only ddnames
containing the string PL, such as ISPLLIB and STEPLIB. This avoids having to use the ONLY command
to limit the search.
Clist (CL) or Save (SA)
The CLIST command creates a CLIST that contains TSO ALLOCATE statements to reproduce
the allocations in the displayed list. The CLIST is saved in a sequential data set named
'userid.ISRDDN.CLIST' or 'pr efix .userid.ISRDDN.CLIST'. If the ISPF configuration table field
USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set to YES, an additional qualifier defined with
Appendix E. ISRDDN diagnostic utility  165

## Page 194

the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is included before the ISRDDN qualifier. You can
use the command name SAVE instead of CLIST.
Use this command when you want to change allocations for testing purposes. For example, to add a
panel library to your ISPPLIB concatenation:
• Enter ISRDDN
• Type O ISPPLIB to limit the displayed list to ddname ISPPLIB
• Type CLIST to create and edit the ISRDDN.CLIST data set
• Change the ALLOCATE statement to add your data set
• Exit ISPF
• Execute the CLIST (that is, EX ISRDDN)
Like the MEMBER command, you can add a whole or partial ddname to limit the number of ddnames
that are included in the generated CLIST. For example, to create a CLIST that only contains allocation
statements for ddnames containing the string ISP, type CLIST ISP or SAVE ISP.
Check (CH)
The CHECK command turns on or off automatic checking for mixed concatenations. CHECK or CHECK
ON enables automatic checking, and CHECK OFF disables it. When checking for mixed concatenations
is enabled, ISRDDN checks for concatenations with mixed record formats, mixed fixed record lengths,
and mixed data set organizations. Because there are times when these concatenations are intended,
you might want to turn off the warning generated by ISRDDN.
Count (C)
The COUNT command displays the number of members in a partitioned data set. The number of
members is shown in the message area on the left side of the list.
COUNT can be used to find out if you have empty data sets in your concatenations. For example, if you
want to find out if all members of an SCLM-controlled library system were successfully promoted, you
can edit the hierarchy, invoke ISRDDN, and use the COUNT command to verify that all of the expected
libraries in the concatenation are empty.
Like the MEMBER command, you can add a whole or partial ddname to limit the number of ddnames
that are searched.
Duplicates (DUP)
The DUPLICATES command searches all of the partitioned data sets in the displayed list and the LPA
and displays a list of duplicate names. From the duplicates list, you can use the E (edit), B (browse),
and V (view) line commands to view the PDS member or LPA storage. Use the DUPLICATES command
to see where you might have potential conflicts with old or modified versions of load modules, REXX
or CLIST programs, ISPF panels, or other PDS members.
For module names found in the Link Pack directory, the address of the module and its size are shown
on the left side of the screen. If the name is an alias of a different module, the real name (major name)
is shown instead of the size.
The duplicates list is shown in Figure 48 on page 167. Like the MEMBER command, you can add a
whole or partial ddname to limit the number of ddnames that are searched. For example, to search
only ddnames that contain the string LLIB, enter DUP LLIB.
166  z/OS: z/OS ISPF User's Guide Vol I

## Page 195

Duplicate members list                  Row 1 of 562
 Address  Siz/Maj  DDname   Act Member   Data set name    Actions: B, E, V
                   ISPLLIB  > _ FLM$CP   PDFTDEV.SVT.LOAD
 00D8A5F8 FLMIO24  --LPA--- > _
                   ISPLLIB  > _ FLM$CPI  PDFTDEV.SVT.LOAD
 04668F20 000000E0 --LPA--- > _
                   ISPLLIB  > _ FLM$DE   PDFTDEV.SVT.LOAD
 00D8B218 FLMIO24  --LPA--- > _
                   ISPLLIB  > _ FLM$DT   PDFTDEV.SVT.LOAD
 00D8B9D8 FLMIO24  --LPA--- > _
                   ISPLLIB  > _ FLM$99   PDFTDEV.SVT.LOAD
 00D88DF8 FLMIO24  --LPA--- > _
                   ISPLLIB  > _ FLM@SCAN PDFTOOL.COMMON.LOAD
                   ISPLLIB  > _          PDFTOOL.FLM@SCAN.LOAD
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 48. The Duplicates List Display
The SAVE command can be entered from the duplicate list display to
have the duplicate member data written to a sequential data set named
'userid.ISRDDN.DUPLICAT' or 'pr efix .userid.ISRDDN.DUPLICAT'. If the ISPF configuration table field
USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set to YES, an additional qualifier defined with the
ISPF _TEMPORARY_DATA_SET_QUALIFIER field is included before the ISRDDN qualifier.
APF, Linklist (LI), Parmlib, and Lpa (LP)
The APF, LINKLIST, PARMLIB, and LPA commands add and remove pseudo-ddnames that show
the defined APF libraries, link list libraries, PARMLIB libraries, and LPA libraries respectively. These
pseudo-ddnames are shown as if they are allocated ddnames, but no actual allocation to the libraries
is made. You can use most of the primary and line commands with these names, just as you would
with real ddnames.
In the confirmation panel, you can type YES to process the libraries, or SKIP to process the libraries
and avoid the confirmation panel in the future. Dynamic LPA, Link lists, PARMLIB, and APF lists are all
supported.
The LINKLIST and LPA commands add both the LINKLIST and LPALIB pseudo-ddnames. To delete
any pseudo-ddname, enter the appropriate command a second time. For example, to add APF
libraries to the list, use the APF command. To remove the APF libraries from the list, enter the APF
command a second time.
Select (S) and Load (L)
The SELECT command searches the job pack area (JPA) and link pack area (LPA) to see if a module is
loaded. If the module is found, you see the CSVQUERY Results panel shown in Figure 49 on page 168. 
Appendix E. ISRDDN diagnostic utility  167

## Page 196

┌─────────────────────────────────────────────────────────────────────┐  197
   │                          CSVQUERY Results                           │
   │                                                        More:     +  │ Q
   │ Module ISRSUBS  was found to be already loaded. Note that           │
   │ invocations of this program name may pick up another copy from      │
   │ STEPLIB or a LIBDEF'ed data set or from a tasklib such as ISPLLIB.  │
   │ Tab to a box and press enter to view the module in storage.         │
   │    +-------------------------+         +-------------------------+  │
   │    | Job pack area resident  |         | PLPA resident           |  │
   │    | Resident above 16 Meg   |         | Resident above 16 Meg   |  │
   │    | Loaded by program fetch |         | Module address:05437000 |  │
   │    |  from ISPLLIB   (Lib 4) |         | Module size:   000D03C0 |  │
   │    | PDFTDEV.STG.LOAD        |         | Reentrant               |  │
   │    | Module address:15EC6000 |         | Serially reusable       |  │
   │    | Module size:   000D3000 |         | Not loadable only       |  │
   │    | Reentrant               |         | AMODE 31                |  │
   │    | Serially reusable       |         | Authorized library      |  │
   │    | Not loadable only       |         | Not Authorized program  |  │
   │    | AMODE 31                |         +-------------------------+  │
   │    | Not Authorized program  |                                      │
   │    +-------------------------+                                      │
 C │ Command ===>                                                        │ PAGE
   │  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel              │
 F ⋘─────────────────────────────────────────────────────────────────────┘
Figure 49. The CSVQUERY Results Panel
The information shown in the CSVQUERY Results panel is mostly derived by issuing a CSVQUERY
macro. The data set name from which the module was loaded is shown if it can be determined.
However, because of the way this information is gathered, the data set name can be incorrect if the
original ddname from which the data set was loaded has been reallocated since the module was
loaded.
On the CSVQUERY Results panel, you can use the TAB key to place the cursor inside the boxes
describing the load module. If you then press Enter, you can browse the load module in storage.
The SELECT command is useful in situations where you need to know where a loaded program came
from, for example, when you think you might be running mixed levels of ISPF or of an application
under ISPF.
If a module is not loaded but you want to see its attributes, you can use the LOAD command instead
of the SELECT command. LOAD uses the current tasklib such as ISPLLIB, but you should verify that
the loaded module came from the source you were expecting it to come from. LOAD automatically
browses the load module storage.
Custom (CU)
The CUSTOM command shows several settings about your ISPF installation. It shows the values that
used to be set in the ISPDFLTS CSECT but are now in the ISPF Configuration table, and it shows the
values configured in module ISPTCM. This command is helpful when you are having trouble with the
way certain programs are invoked. For more information about ISPTCM, refer to z/OS ISPF Planning
and Customizing.
MList (ML)
The MLIST command displays the eyecatchers for some of the ISPF CSECTs contained in modules
ISPSUBS and ISRSUBS. This command can sometimes be used to verify that you are running with
a particular level of maintenance because the eyecatchers in most ISPF modules contain a release
number or a PTF level.
Browse (B)
ISRDDN provides a method of browsing storage using ISPF BROWSE. The storage can be browsed as
unformatted data, as minimally formatted data, or as a side-by-side hexadecimal and EBCDIC dump
format. ISRDDN also enables you to automatically chain lists, view arrays, and view the data pointed
to by control blocks that are mainly lists of pointers (such as CVT).
The BROWSE primary command accepts a storage address, module name, or TSO TEST address
locator string. 
168  z/OS: z/OS ISPF User's Guide Vol I

## Page 197

Table 26. Some examples invoking BROWSE
Command Explanation
B ISRSUBS Browse the already loaded module named ISRSUBS.
B 10. Browse storage at hexadecimal location 10. To distinguish hexadecimal
addresses from module names, absolute addresses must end with a
period.
B 0.+21c?+b4?+108?+8 Browse storage based on a TSO TEST style string. In this case, the
control block called the Protected Step Control Block or PSCB is
shown.
B ISRSUBS+60? Browse the address pointed to by the 4 bytes at offset hexadecimal 60
into module ISRSUBS.
B ? or B +0? When executed from within the storage browser, this command uses
the address 0 bytes from the beginning of the displayed storage as a
pointer and starts a new browse session to show that storage.
Enq (E)
You can view ENQs on the system using the ENQ command. A display similar to the one shown in
Figure 50 on page 169 appears. You can reduce the size of the list by specifying a QNAME, RNAME,
address space name, and a system name. All entries are treated as prefixes, so you might not need to
specify complete names. 
                           System ENQ Status                       Row 1 of 183
        Scroll LEFT or RIGHT to see type or system name.
 Major name prefix . . . SYSDSN    (SYSDSN, SPFEDIT, etc)
 Minor name prefix . . .                                              (dsn etc)
 Address id prefix . . . USERID    (Job name, User id, etc)
 System prefix . . . . .           (System name)
   Major      Minor                                                  Job Name
 ┌──────────┬──────────────────────────────────────────────────────┬──────────┐
 │ SYSDSN   │ AOP.SAOPEXEC                                         │ USERID   │
 │ SYSDSN   │ AOP.SAOPMENU                                         │ USERID   │
 │ SYSDSN   │ AOP.SAOPPENU                                         │ USERID   │
 │ SYSDSN   │ AZZ.V1R1.SAZZCLIB                                    │ USERID   │
 │ SYSDSN   │ AZZ.V1R1.SAZZMENU                                    │ USERID   │
 │ SYSDSN   │ AZZ.V1R1.SAZZPENU                                    │ USERID   │
 │ SYSDSN   │ AZZ.V1R1.SAZZSENU                                    │ USERID   │
 │ SYSDSN   │ BZZ.SBZZCLIB                                         │ USERID   │
 │ SYSDSN   │ BZZ.SBZZMENU                                         │ USERID   │
 │ SYSDSN   │ BZZ.SBZZPENU                                         │ USERID   │
 │ SYSDSN   │ BZZ.SBZZSENU                                         │ USERID   │
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 50. The System ENQ Status List Panel
The Major column shows the QNAME. The Minor field shows the RNAME and if the RNAME is 8 bytes
or less, it shows the hexadecimal representation of the RNAME next to the EBCDIC representation.
In the System ENQ Status list, the Job Name field is color-coded to indicate the type of ENQ that the
address space holds or is waiting for. Green indicates a shared ENQ. Red indicates an exclusive ENQ.
If an address space does not hold the ENQ but is waiting for it, the job name is shown highlighted in
reverse video.
On narrow screens, you can scroll right or left for more information. By scrolling left and right you see
the system name and ENQ options (SYS (system), SYSS (systems), STEP, G (global), and R (reserve)).
On wide screens you see all of the information on one screen without scrolling left or right.
On the System ENQ Status display, press END to return to the Current Data Set Allocations list, or
enter CON to view the System ENQ Contention display. You can also use the ALL command to view all
ENQs or use the RESET command to see only the data set ENQs (QNAME SYSDSN) for your TSO user
id.
Appendix E. ISRDDN diagnostic utility  169

## Page 198

Con (C)
You can view ENQ contention on the system by using the CON command. When ENQ contention
exists, you see a screen similar to the one in Figure 50 on page 169, but without the input fields. When
no contention exists, a message displays instead of the list.
Allocation list line commands
Allocation list line commands are entered next to a ddname or data set. By default the allocation list is
in short format. This means that for concatenations, the ddname is next to the first data set name in the
concatenation.
When a line command is entered next to a ddname, the command is intended to work on the DD
allocation rather than the data set name on that line. For example, an E command next to the ddname
that refers to a concatenation edits the whole concatenation. If you want to edit just the first data set in a
concatenation, use the LONG command to place the list in long format. In long format, the ddname for a
concatenation is on a separate line so that you can place line commands next to the first data set name in
the concatenation.
The Edit, Browse, View, and Member list commands are sensitive to the results of the MEMBER primary
command. When the MEMBER primary command searches the displayed list for a member or members
matching a name pattern, the member or pattern is shown in the list. Placing an E, B, V, or M next to
a name in which the member or pattern was found displays either a member list with member names
matching the pattern or the specific found member.
E - Edit
The E line command edits a data set or concatenation. It can be used on any data set or any ddname
allocated to a data set (real or VIO). You might want to use the E line command for editing temporary
files such as JCL that was created by file tailoring and written to the ISPCTLn ddname.
B - Browse
The B line command browses a data set or concatenation. It can be used on any data set or any
ddname allocated to a data set (real or VIO). You can use the B line command for browsing allocated
files. For example, the compress option in the PDF utilities, option 3.1, creates a listing data set that
is sometimes allocated to the ISPCTL1. When you press the HELP key after compressing a data set in
option 3.1, you might see that the listing was saved in a temporary data set. The B line command in
ISRDDN is an easy way to browse that data set.
V - View
Use the V line command to view a data set or concatenation. This is similar to E (Edit) but there is no
SAVE command. Use this when you want to view a data set and modify it for easier viewing without
risking changes to the data set.
M - Member list
The M command displays an enhanced member list for a data set or concatenation. This gives you
greater flexibility in working with allocated data sets. You might use this command when you have
several different operations to perform on members.
F - Free
Use the FREE command to free an allocation. The Free command must be specified next to a ddname.
F commands next to data sets in a concatenation with an F next to the ddname are ignored because
those data sets are removed from the list before the F commands are processed.
The F command uses SVC 99 (dynamic allocation) to free the ddname. However, if SVC 99 cannot
free the data set, ISRDDN invokes the TSO FREE command. The TSO FREE command might write a
message to the screen with information on why the free failed. This command is useful when you
need to free allocations such as those left by prematurely terminated or poorly behaved programs.
C or Z - Compress
Use the COMPRESS command to compress partitioned data sets. The COMPRESS command can be
used with data sets that are allocated as shared and can be used next to data set names or ddnames.
170  z/OS: z/OS ISPF User's Guide Vol I

## Page 199

I - Information
The I command attempts to invoke the PDF data set information utility to display information about
a data set. It can be used next to any real data set name. VIO data sets are not supported. This
command can provide information such as the number of allocated directory blocks or a data set's
SMS management class, or other information that is not shown by scrolling the Current Data Set
Allocations list left or right.
Q - Query ENQs
The Q command shows all SYSDSN and SPFEDIT ENQs that exist for a data set. This command is
useful when you want to see what other users or jobs are using a data set you have allocated. Using
the Q command provides the same information as using the ENQ primary command and selecting an
RNAME of the data set name.
T - Test Directory
The T line command reads the directory of a PDS directly, and performs a BLDL command on each
member to see if the BLDL service returns accurate information for the directory. The results are
displayed in a separate Browse session. The T command can be used to debug problems such as I/O
errors or the need to refresh LLA or other directory caching systems.
K- VTOC Information
The K line command displays VTOC information for the first volume on which the data set resides. The
information returned is the same as in the Data Set List utility (option 3.4, command V). You can also
view VTOC information by placing the cursor on the volume name and pressing Enter. If the data set is
not on a physical volume, the K command does not provide any information.
Browsing storage and loaded modules
You can use the BROWSE command within ISRDDN to view the contents of storage within your address
space. When you are browsing storage, you can use any of the standard ISPF Browse primary commands.
In addition, there are several commands you can use to format and move around in the storage list.
If you are not using one of the special display formats (CHAIN, ARRAY, or ARRAYP), you can scroll UP
even when the "Top of Data" line is displayed so that you can see what data exists before your requested
storage location. After you scroll up once, you can scroll up or down to the limits of the contiguous
addressable storage.
ARRAY command
ARRAY
dimension
4
length
where:
dimension
The number of array elements in decimal.
length
The length of each element in hexadecimal.
When you are viewing an array, you can show the array elements as separate blocks of storage.
For example, the static link list table is an array. Assuming that each element is 45 bytes (hexadecimal
2D) and that you want the first 30 entries, enter ISRDDN and type:
B 10.?+4DC?+8
ARRAY 30 2D
You see a screen similar to Figure 51 on page 172.
Appendix E. ISRDDN diagnostic utility  171

## Page 200

BROWSE    STORAGE  Start:00F3E6C0                    Line 00000001 Col 001 080
      +1 (00F3E6C0)   0CE2E8E2 F14BD3C9 D5D2D3C9 C2404040  * .SYS1.LINKLIB    *
         (00F3E6D0)   40404040 40404040 40404040 40404040  *                  *
         (00F3E6E0)   40404040 40404040 40404040 40        *                  *
      +2 (00F3E6ED)   0BE2E8E2 F14BD4C9 C7D3C9C2 40404040  * .SYS1.MIGLIB     *
         (00F3E6FD)   40404040 40404040 40404040 40404040  *                  *
         (00F3E70D)   40404040 40404040 40404040 40        *                  *
      +3 (00F3E71A)   0BE2E8E2 F14BC3E2 E2D3C9C2 40404040  * .SYS1.CSSLIB     *
         (00F3E72A)   40404040 40404040 40404040 40404040  *                  *
         (00F3E73A)   40404040 40404040 40404040 40        *                  *
      +4 (00F3E747)   11E2E8E2 E74BC9E2 C4F14BD3 C9D5D2D3  * .SYSX.ISD1.LINKL *
         (00F3E757)   C9C24040 40404040 40404040 40404040  * IB               *
         (00F3E767)   40404040 40404040 40404040 40        *                  *
      +5 (00F3E774)   15E2E8E2 E74BE2E8 E2D7D3C5 E7C44BD3  * .SYSX.SYSPLEXD.L *
         (00F3E784)   C9D5D2D3 C9C24040 40404040 40404040  * INKLIB           *
         (00F3E794)   40404040 40404040 40404040 40        *                  *
 Command ===>                                                  Scroll ===> PAGE
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 51. ARRAY Storage Format
In the ARRAY format display, the offsets on the left are the array element number followed by the address
of the displayed line.
ARRAYP command
ARRAYP
dim
length
where:
dim
The decimal number of pointers in the array.
length
The hexadecimal length of each element pointed to by the pointer. The default for length is whatever
fits on one line in the current display format.
Many control blocks are mainly list of pointers. For example, the Communications Vector Table (CVT) is
an MVS control block that points to many other control blocks. The ARRAYP command shows the data
pointed to by the pointers in a control block. Use the ARRAYP command when you are looking for the
offset of a pointer to a particular storage location.
For example, to see what is pointed to by the elements of CVT, enter ISRDDN and type
B 10.?
ARRAYP
You see a screen similar to Figure 52 on page 173.
In the ARRAYP format display, the offsets on the left are the offsets within the array of pointers followed
by the pointer itself. This is followed by the data to which the pointer refers.
172  z/OS: z/OS ISPF User's Guide Vol I

## Page 201

BROWSE    STORAGE  Start:00FC6CB8                    Line 00000001 Col 001 080 
      +0 (00000218)   00889E88 00889E88 00FC4D80 00F90100  * .hãh.hãh..(Ï.9.. * 
      +4 (00FDEFC4)   0DA01211 A7240008 10114111 00001111  * .Á..x.....á..... * 
      +8 (00FC6C34)   00000000 00000000 00000000 00000000  * ................ * 
      +C (00FC72A0)   C1E4E2C3 C2010000 00FCA440 40404040  * AUSCB.....u      * 
     +10 (00000000)   000A0000 000130E1 00000000 00000000  * .......¸........ * 
     +14 (00FEB70C)   00FEB63C 00FEB63C 00FEB63C 00FEB63C  * .┌Â..┌Â..┌Â..┌Â. * 
     +18 (00FE7096)   58F00224 58F0F06C 58F0F070 58F0F004  * ý0..ý00%ý00°ý00. * 
     +1C (00FDA0E8)   47F0F028 47F0F034 47F0F020 47F0F018  * Õ00.Õ00.Õ00.Õ00. * 
     +20 (00FD9F1C)   47F0F028 47F0F0E6 47F0F020 47F0F018  * Õ00.Õ00WÕ00.Õ00. * 
     +24 (0181B7F8)   D3D3C3C2 04820000 00000000 00F8BB00  * LLCB.b.......8]. * 
     +28 (0126F150)   47F0F01C 16C3E2E5 D3D3E3D9 D440F0F2  * Õ00..CSVLLTRM 02 * 
     +2C (00FD8C50)   05F047F0 F00600E6 05A04AF0 F00407FF  * .0Õ00..W.Áó00... * 
     +30 (00F29C70)   0088FF8E FA0E0000 00000000 00E4C3C2  * .h.▪│........UCB * 
     +34 (00FDC2F0)   0DF058F0 F2020BEF 00FDC34C 00FDC360  * .0ý02..ı.┘C<.┘C- * 
     +38 (0103034F)   0C4104A0 045000E1 08961060 7E584000  * .á.Á.&.¸.o.-=ý . * 
     +3C (00FC72C8)   15C7A300 D4E2C5D9 15CD6B40 000000FF  * .Gt.MSER.‗, .... * 
     +40 (00F16000)   02000000 52000000 00000000 1000263C  * ....Û........... * 
     +44 (00FEDE78)   47F0F008 41EE0002 1FCC43CE 00009110  * Õ00.áË...÷õ¾..j. * 
     +48 (00FDEFE8)   0DF04111 00000BE0 58F0F00E 0BEF0000  * .0á....\ý00..ı.. * 
     +4C (00000000)   000A0000 000130E1 00000000 00000000  * .......¸........ * 
 Command ===> ________________________________________________ Scroll ===> PAGE 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 52. ARRAYP Storage Format
BROWSE command
BROWSE
modname
address.
where:
modname
The name of the module you want to browse.
address
The address of the module you want to browse. The address must be terminated with a period.
The BROWSE command lets you browse a module that is already loaded. If it is not loaded, you can use
the LOAD command to explicitly load and browse it.
You can also use the BROWSE command in "point and shoot" mode. Type BROWSE on the command line,
place the cursor over an address within the display, and press Enter. A new browse session is started
to view the storage pointed to by the cursor. If the cursor is not on a valid, accessible address, an error
message is displayed.
When the BROWSE command is invoked within an existing browse session, a new browse session is
started. The END command returns you to the previous browse session.
CANCEL command
CANCEL
The CANCEL command ends all browse sessions and returns to the Current Data Set Allocations list.
CHAIN command
CHAIN
0
offset length
where:
Appendix E. ISRDDN diagnostic utility  173

## Page 202

offset
A hexadecimal offset of the 4-byte pointer to the next link.
length
The length of each element in hexadecimal. The default for length is whatever fits on one line in the
current display format.
When you are viewing a linked list, you can use the CHAIN command to view more than one link at a time.
The chain is considered terminated when one of these is found:
• A pointer of zero.
• A pointer to the first node.
• A pointer to unavailable storage.
Entering the CHAIN command a second time turns the chain formatting off.
For example, to see the current ASCB chain, enter ISRDDN and type
B 10.??+C?
CHAIN 4 20
You see a screen similar to Figure 53 on page 174.
 BROWSE    STORAGE  Start:00F90100                    Line 00000001 Col 001 080
      +0 (00F90100)   C1E2C3C2 00F92B80 00F90280 00000000  * ASCB.9.Ï.9.Ï.... *
     +10 (00F90110)   008FD788 00029982 00000000 00000000  * ..Ph..rb........ *
     +30 (00F92B80)   C1E2C3C2 00F98500 00F90100 00000000  * ASCB.9e..9...... *
     +40 (00F92B90)   008FD880 000005B4 00000000 00000000  * ..QÏ...®........ *
     +60 (00F98500)   C1E2C3C2 00F90700 00F92B80 00000000  * ASCB.9...9.Ï.... *
     +70 (00F98510)   008FD598 0000E527 00000000 00000000  * ..Nq..V......... *
     +90 (00F90700)   C1E2C3C2 00F90580 00F98500 00000000  * ASCB.9.Ï.9e..... *
     +A0 (00F90710)   008FD690 00009130 00000000 00000000  * ..O...j......... *
     +C0 (00F90580)   C1E2C3C2 00000000 00F90700 00000000  * ASCB.....9...... *
     +D0 (00F90590)   008FD788 00007EC0 00000000 00000000  * ..Ph..={........ *
******************************** Bottom of Data 
********************************
 Command ===> ________________________________________________ Scroll ===> PAGE
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 53. CHAIN Storage Format
In the CHAIN format display, the offsets on the left are the offsets within a particular link followed by the
actual address of the start of the line.
DATA command
DATA
The DATA command displays the storage as an unformatted string of data with offsets shown on the left
side of the screen. Use this format to give you a better context of the storage you are browsing.
174  z/OS: z/OS ISPF User's Guide Vol I

## Page 203

DISASM command
DISASM
ON
OFF
where:
ON
Disassembles the load module being browsed.
OFF
Releases any resources acquired for the disassembly function and returns to the previous browse
mode.
While browsing a load module in storage, you can enter the DISASM command (with either no parameter
or the NO parameter) to disassemble the load module you are browsing being browsed and display the
resulting instructions.
When you have finished browsing the disassembly, you should issue the DISASM command again (this
time with the OFF parameter) to release the resources obtained to support the disassembly. If you
terminate the browse without issuing the DISASM OFF command, the resources are not freed until you
terminate the logical screen.
DUMP command
DUMP
The DUMP command lets you view storage in dump format.
Note: The DUMP command is analogous with the FORMAT command.
FORMAT command
FORMAT
The FORMAT command displays the storage in both hexadecimal and EBCDIC, the way you might see the
format in a data dump (see Figure 54 on page 176).
When data is displayed in wide format (see “WIDE command” on page 177), the format includes 16 bytes
per line (8 sets of 4 bytes). When data is displayed in narrow format (see “NARROW command” on page
176), the format contains 8 bytes per line (4 sets of 4 bytes).
FORMAT is the default format that appears whenever a storage browse session is started.
Appendix E. ISRDDN diagnostic utility  175

## Page 204

BROWSE    ISPSUBS JPA Start:15D8C890 Size:000E5770   Line 00000000 Col 001 080
********************************* Top of Data **********************************
      +0 (15D8C890)   C9E2D7E3 E2C3F040 000003BC 15E03450  * ISPTSC0 ...».\.& *
     +10 (15D8C8A0)   15DCE850 15DB0C48 15E1EDB0 15DE49A0  * .³Y&.¹.þ.¸Ê^.·±Á *
     +20 (15D8C8B0)   15DE7BF8 15E21780 15E00E48 15E71848  * .·#8.S.Ï.\.þ.X.þ *
     +30 (15D8C8C0)   15E1E1A0 15DCEA40 15E1F400 15E1FEE0  * .¸¸Á.³. .¸4..¸┌\ *
     +40 (15D8C8D0)   15E0D540 15E0E148 15E20038 15E6BE98  * .\N .\¸þ.S...W┤q *
     +50 (15D8C8E0)   15E106D0 15E202B8 15E0E950 15DDD380  * .¸.}.S.¢.\Z&.¨LÏ *
     +60 (15D8C8F0)   15D8DC30 15E0F7C0 15E10308 15E0F1A0  * .Q³..\7{.¸...\1Á *
     +70 (15D8C900)   15E1EB98 15E20700 15DBA490 15E0B518  * .¸Èq.S...¹u..\º. *
     +80 (15D8C910)   15E6D018 15DA8AC0 15DA5D58 15E20AE0  * .W}...½{..)ý.S.\ *
     +90 (15D8C920)   15E25D20 15DA9A30 15E6D3D0 15E6DB98  * .S)...¬..WL}.W¹q *
     +A0 (15D8C930)   15E212C0 15E21540 15DFFB08 15E6F128  * .S.{.S. .....W1. *
     +B0 (15D8C940)   15DAE7D8 15E083C8 15DB0428 15E0EFB0  * ..XQ.\cH.¹...\ı^ *
     +C0 (15D8C950)   15E0F628 15E0F290 15E0AB60 15E209D8  * .\6..\2..\┐-.S.Q *
     +D0 (15D8C960)   15E224F0 15DFDBE0 15E0D050 15DCD0B0  * .S.0..¹\.\}&.³}^ *
     +E0 (15D8C970)   15DCD3D0 15DCD190 15E23A80 15E6F480  * .³L}.³J..S.Ï.W4Ï *
     +F0 (15D8C980)   15E23B58 15E24BE8 15D8D630 15E24E40  * .S.ý.S.Y.QO..S+  *
    +100 (15D8C990)   15D8CC50 15E252A8 15E25468 15E25B20  * .Q÷&.SÛy.SÞÃ.S$. *
    +110 (15D8C9A0)   15E29E30 15E6FDA8 15E2B778 15E0D358  * .Sã..W┘y.S...\Lý *
    +120 (15D8C9B0)   15E70AC8 15DCEB10 15E2B998 15DFF458  * .X.H.³È..S¥q..4ý *
 Command ===> ________________________________________________ Scroll ===> PAGE
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 54. Storage Viewed in FORMAT Mode
LIMIT command
LIMIT
The LIMIT command shows the address limits and size of the contiguous storage area being browsed and
resets the currently browsed address to the lowest address in that storage.
LOAD command
LOAD modname
where:
modname
The name of the module you want to load and browse.
The LOAD command loads and browses a module.
NARROW command
NARROW
The NARROW command switches the display from wide format to narrow format.
To return to wide format, issue the WIDE command.
The wide or narrow format, set by the WIDE and NARROW commands respectively:
• Is maintained from one session to the next by means of a profile variable.
• Applies to data when it is displayed with a type setting of FORMAT, DATA, DUMP, or RAW. When data is
displayed with a type setting of DISASM, the WIDE and NARROW commands have no effect.
RAW command
RAW
The RAW command displays storage data as unformatted text. Storage is shown on the screen without
any formatting. The data on a line is the data that immediately follows the data on the previous line.
Because the FIND command is actually searching the screen image and not storage itself, it is best to
search storage while in the RAW display format. Note that even in unformatted displays, if your search
string would span lines, FIND does not locate the string. To avoid this, search for the string in RAW format,
176  z/OS: z/OS ISPF User's Guide Vol I

## Page 205

then enter the command B +20 to find the string again. This shifts the display by 32 bytes (hexadecimal
20) and the line breaks occur in different places.
REFRESH command
REFRESH
Use the REFRESH command to scroll the display back to the +0 offset. REFRESH is useful if you have
scrolled up past the initial "Top of Data" line and want to return to your original referenced storage
location.
REFRESH is not available in CHAIN, ARRAY, or ARRAYP formatted displays.
SETDATA command
SETDATA
0
offset
where:
offset
The offset at which ISRDDN is to treat the code as data rather than as an instruction.
The SETDATA command lets you specify an offset at which you want ISRDDN to treat the code as data
rather than as an instruction.
WIDE command
WIDE
The WIDE command switches the display from narrow format to wide format.
If the screen is too narrow to handle the wide format, you must scroll right and left to see all of the data.
To return to narrow format, issue the NARROW command.
The wide or narrow format, set by the WIDE and NARROW commands respectively:
• Is maintained from one session to the next by means of a profile variable.
• Applies to data when it is displayed with a type setting of FORMAT, DATA, DUMP, or RAW. When data is
displayed with a type setting of DISASM, the WIDE and NARROW commands have no effect.
Defining named storage locations
If you browse the same storage locations or control blocks frequently, you might want to set up a file that
names those storage locations so that you can use a name in the BROWSE command.
To enable the BROWSE command to use a named reference to storage, you must allocate a sequential file
to the ddname ISRDDN. Each line in that file is either a comment or a named storage location. Comments
start with a semi-colon (;).
Location definitions have a name as the first word, followed by a TSO TEST style locator string. Anything
after the locator string is ignored. The TSO TEST locator string can use another defined name as a starting
point. If the locator string cannot be resolved because of syntax or other errors, the line is ignored.
For example, if you allocate a sequential file like the one shown in Figure 55 on page 178 to ddname
ISRDDN, you could then browse your User Profile Table, which stores your TSO PROFILE settings, by
typing B UPT on the command line.
Appendix E. ISRDDN diagnostic utility  177

## Page 206

CVT        10.?          Communications Vector Table
PSCB       JSCB+108?     TSO Protected Step Control Block
JSCB       TCB+B4?       Job/Step Control Block
TCB        CVT??         Task Control Block
UPT        PSCB+34?      User Profile Table
Figure 55. Sample ISRDDN Named Storage File
178  z/OS: z/OS ISPF User's Guide Vol I
