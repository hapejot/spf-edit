# Appendix E. System variables

Source file: f54dg00_v3r1.md
Start page: 387
Page span: 387-400

## Page 387

Appendix E. System variables
The system variables are described with type and pool information in the following tables. The variables
are also discussed with the ISPF service to which they apply.
Commonly used system variables that a dialog can access are listed below. They are grouped by topic.
The first column gives the name of the variable. The second column indicates in which pool the variable
resides. The following abbreviations are used:
func
Function pool
shr
Shared pool
prof
Profile pool
any
Any pool.
The third column indicates the variable's type. The following abbreviations are used:
in
Input variable, set by a dialog to provide information to ISPF
out
Output variable, set by ISPF to provide information to dialogs
non
Non-modifiable output variable
i/o
Both an input and an output variable.
The fourth column gives the length of the variable.
The fifth column gives a brief description of the variable.
Numeric system variables set by ISPF are right-justified and padded with zeros on the left, if necessary.
If a program function uses the VCOPY service to access the variable, the value will be in character string
format rather than in fixed binary format.
Configuration utility
Table 36. System variables: Config ur ation  utility
Name Pool Type Len Description
ZCFGCMPD shr non 10 Current Configuration module compilation date. ZCFGCMPD contains
the national language delimiter and contains the date in the format
YYYY/MM/DD. For countries that use a delimiter other than a slash (/),
that delimiter replaces the slash in the date representation.
ZCFGCMPT shr non   5 Current Configuration module compilation time. ZCFGCMPT contains the
national language delimiter and contains the time in the format HH:MM.
For countries that use a delimiter other than a colon (:), that delimiter
replaces the colon in the time representation.
Note: This field will be blank for a configuration module compiled with a
previous version of ISPF.
System variables
© Copyright IBM Corp. 1980, 2025 359

## Page 388

Table 36. System variables: Config ur ation  utility (continued)
Name Pool Type Len Description
ZCFGKSRC shr non 54 Keyword source data set and member for the current configuration
module.
Note: This field will be blank for a configuration module compiled with a
previous version of ISPF.
ZCFGLVL shr non   8 Current Configuration module level.
ZCFGMOD shr non   8 Current Configuration module name.
Time and date
Table 37. System variables: Time and date
Name Pool Type Len Description
ZDATE shr non   8 Current date. The format of ZDATE depends on the current national
language (see ZDATEF and ZDATEFD).
ZDATEF shr non   8 Current national language date format using the characters DD for day,
MM for month, and YY for year. ZDATEF contains the national language
delimiter. For example, DD/MM/YY, YY/MM/DD, MM.DD.YY. For countries
that use a delimiter other than a slash (/), that delimiter replaces the
slash in the date representation.
ZDATEFD shr non   8 The date format as described under ZDATEF but with the national
language convention instead of DD, MM, and YY.
ZDATESTD shr non   8 Current date with a 4-digit year (YYYY/MM/DD). The format of ZDATESTD
depends on the current national language (see ZDATEF and ZDATEFD).
ZDAYOFWK shr non   8 The name of the day of the week.
ZDAY shr non   2 Day of month (2 characters)
ZJDATE shr non   6 Day-of-year date (format yy.ddd)
ZJ4DATE shr non   8 Day-of-year date (format yyyy.ddd)
ZMONTH shr non   2 Month of year (2 characters)
ZSTDYEAR shr non   4 All 4 digits of the current year (4 characters).
ZTIME shr non   5 Time of day (format hh:mm)
ZTIMEL shr non   Time of day (format hh:mm:ss:TQ —where T is tenths of a second, and Q
is hundredths)
ZYEAR shr non   2 Year (2 characters)
The current date is displayed in the appropriate format for the session language, where DD=DAY,
MM=MONTH, and YY=YEAR. For countries that use a delimiter other than a slash (/), that delimiter
replaces the slash in the date representation.
General
JES2 4.3 or later
System variables
360  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 389

Table 38. General variables
Name Pool Type Len Description
Z shr non  0 Null Variable
ZACCTNUM shr non 40 The MVS account number specified at logon time.
ZAPLCNT shr non  4 Number of times APL invoked for a logical screen
ZAPPLID shr non  8 Application identifier
ZAPPTTL any in N/A Variable ZAPPTTL is no longer used by ISPF. If a dialog sets this variable,
the value has no effect on ISPF processing.
ZBDMAX shr i/o  9 Maximum number of displays that can occur within a batch mode
session. This value is obtained from the BDISPMAX keyword on the
ISPSTART command. See “Avoiding panel loop conditions in the batch
environment” on page 35.
ZBDMXCNT shr non  9 Count of current number of displays in a batch mode session
ZCLIENT shr non  4 If ISPF is communicating with a client using the JSON API, ZCLIENT will
be set to a value of JSON.
ZCS shr non  5 Multicultural support currency symbol
ZCSDLL shr non  8 Variable ZCSDLL is no longer used and is set to blanks.
ZDECS shr non  1 Multicultural support decimal separator character
ZDEL prof non  1 The delimiter is used to separate stacked commands. The default
delimiter is a semicolon (;).
ZEDLMSG shr in 79 Available for an edit macro to set the long message for the next display.
ZEDSMSG shr in 24 Available for an edit macro to set the short message for the next display.
ZENTKTXT any in 12 Variable ZENTKTXT is no longer used by ISPF. If a dialog sets this
variable, the value has no effect on ISPF processing.
ZENVIR shr non 32 Environment description:
• Characters 1 to 8 contain the product name and sequence number, in
the form ISPF x.y. The sequence number x.y has this meaning:
7.5 means ISPF for z/OS Version 2 Release 5.0
7.4 means ISPF for z/OS Version 2 Release 4.0
7.3 means ISPF for z/OS Version 2 Release 3.0
7.2 means ISPF for z/OS Version 2 Release 2.0
7.1 means ISPF for z/OS Version 2 Release 1.0
Note: See also the system variables ZISPFOS and ZOS390RL.
• Characters 9 to 16 contain the generic operating system name (MVS).
• Characters 17 to 24 contain the operating system environment (TSO or
BATCH).
• Characters 25 to 32 contain blanks and are reserved.
ZEURO shr non  1 The EURO currency symbol.
ZGUI shr non 68 On a client that is using the JSON API, ZGUI is set to the value CLIENT.
Otherwise, ZGUI is set to blanks.
System variables
Appendix E. System variables  361

## Page 390

Table 38. General variables (continued)
Name Pool Type Len Description
ZINICMD shr in 1 Set the value of ZINICMD to Y in the PROC section of the primary menu to
indicate that, in the initial invocation of the menu, a command has been
put in the value of the ZSEL variable.
ZISPFOS shr non 30 The level of ISPF code that is running as part of z/OS on your system. This
level might or might not match the z/OS level found in ZOS390RL.
ZISPFRC shr in  8 Return code from ISPSTART-selected dialog to invoking application.
ZKEYHELP any in  8 Keys help panel identifier. If a keys help panel is not specified on
the referenced keylist, the application can provide the keys help panel
name in this variable. If the help panel name is present as part of the
referenced keylist definition, it takes precedence over the ZKEYHELP
value. This system variable must be redefined each time the keys help
panel is to change.
ZLANG prof non  8 Session language
ZLOGO shr non  3 Indicates whether the user has requested bypass of LOGO panel. NO
indicates that the user has specified the NOLOGO keyword at the time
ISPF was called, thus, requesting that the LOGO panel be bypassed.
Otherwise, the value of the variable will be YES.
ZLOGON shr non  8 Stepname of TSO logon procedure
ZNESTMAC any in  2 When set to a value of NO, REXX and CLIST edit macros are not invoked
as nested commands, even when the NESTMACS parameter is specified
on the ISPSTART command.
ZMLPS shr non  3 Indicates whether the ISPF Profile Sharing feature is active. ZMLPS has a
value of either YES or NO.
ZOS390RL shr non 16 Indicates the z/OS release running on your system.
ZPANELID shr non  8 The name of the currently displayed panel.
ZPFKEY shr non  4 The name of the PF key (PFxx) in effect when the user exits the panel. If
ZPFKEY = PF00 then no PF key is in effect.
ZPLACE prof i/o  7 Command line placement (ASIS or BOTTOM)
ZPREFIX shr non  8 TSO user prefix
ZPROFAPP prof in  8 Name of application profile pool extension table
ZSCLMLVL shr non 60 Environment description:
• Characters 1 to 9 contain "SCLM FOR ".
• Characters 10 to 39 contain the value from ZISPFOS.
• Characters 40 to 42 contain the value x.y from ZENVIR.
• Characters 43 to 44 contain the SCLM function level.
• Characters 45 to 51 contain the ISPF FMID.
• Characters 52 to 60 contain blanks and are reserved.
ZSCRCUR shr non  4 Displays the number of logical screens currently in use.
ZSCREENC shr non  5 Cursor position within the logical screen data.
ZSCREENI shr non  ? Logical screen data. Size depends upon your screen size.
System variables
362  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 391

Table 38. General variables (continued)
Name Pool Type Len Description
ZSCRNAME shr in  8 Screen name set by dialog. The screen name is in effect only for the
select level in which it was defined. Option 7.3 can alter ZSCRNAME, but
this will have no impact.
See “ZSCRNAME examples” on page 365 for examples of its use.
ZSCRMAX shr non  4 Displays the number of logical screens allowed by the installation.
ZSCTPREF shr non  4 First site command table prefix
ZSCTPRE2 shr non  4 Second site command table prefix
ZSCTPRE3 shr non  4 Third site command table prefix
ZSCTSRCH shr non  1 Search order for site command tables relative to system command table.
Set to either B (Before ISP) or A (After ISP).
ZSEQ shr non  5 Unique number within the sysplex.
ZSM shr i/o  3 Indicates whether session manager panels will be used for ISPF options
4 and 6. This variable is initialized from the ISPF configuration table
keyword USE_SESSION_MANAGER at startup and stored in the shared
variable pool. Once initialized it can only be changed with Option 0 -
Settings or by use of the RESET_USE_SESSION_MANAGER configuration
option.
ZSTART prof in N/A Default command stack variable. See “Syntax for issuing the ISPSTART
command” on page 8 for format and use.
ZSTARTPR prof non 1 The value of ZSTARTPR can be checked in the panel processing sections
of the primary menu to determine if ISPF is processing an initial stack
provided in a variable specified on the ISPF command. The following
values are possible:
N
Processing of the commands in the initial command stack is
completed.
I
An initial command stack was provided in a variable specified on the
ISPF command. This is the initial invocation of the primary menu
and the value of ZCMD is either 'ZSTART DEFAULT' or the name of a
variable containing the initial command stack.
Y
The commands in the initial command stack are currently being
processed.
S
No initial command stack was provided in a variable specified on the
ISPF command.
ZSYSICON shr non  8 The 8-character variable that contains the command to be executed
when the system icon is double-clicked or close is selected.
ZSYSID shr non  8 The 8-character SYSNAME obtained from the SYS1.PARMLIB member
IEASYSxx which is read at IPL time. NONAME is the default value of
SYSNAME. The operator can change this value at IPL time. See the z/OS
MVS Initialization and Tuning Reference for more information.
System variables
Appendix E. System variables  363

## Page 392

Table 38. General variables (continued)
Name Pool Type Len Description
ZSYSNODE shr non 12 The network node name of your installation's JES. This name identifies
the local JES in a network of systems or system complexes being used
for network job entry (NJE) tasks. The node name returned in ZSYSNODE
derives from the NODE initialization statement of JES.
If the system finds that the subsystem is not active, the ZSYSNODE
variable contains the string --INACTIVE-- (note the string delimiters).
If the system finds that the subsystem is JES2 4.2 or earlier, or
JES3 5.1.0 or earlier, the ZSYSNODE variable contains the string --
DOWNLEVEL-- (note the string delimiters).
The value in ZSYSNODE remains the same throughout the ISPF session.
Note: If, for instance, the JES subsystem is taken down during an ISPF
session and the node name is changed, the value in ZSYSNODE will still
contain the value as determined at ISPF initialization.
ZSYSPLEX shr non  8 The MVS sysplex name as found in the COUPLExx or LOADxx member
of SYS1.PARMLIB. If no sysplex name is specified in SYS1.PARMLIB,
ZSYSPLEX contains blanks.
ZSYSPROC shr non  8 TSO Logon Procedure name. In foreground, will have the name of the
current logon procedure; in batch, will have the value 'INIT'; a Started
Task will have the Started Task procedure name.
ZTEMPF shr non 44 Name of temporary data set for file tailoring output
ZTEMPN shr non  8 DDNAME of temporary data set for file tailoring output
ZTERMCID shr non  5 CCSID coded character set identifier of the terminal. Set by ISPF based
on the code page and character set of the terminal. If the terminal code
page and character set cannot be queried or if they are not supported by
ISPF, this variable will be blank.
ZTERMCP shr non  4 CECP support 4-digit code page.
Note: ZTERMCS is defined as character length 4. It cannot handle
5-character character sets. For example, the character set 65535 is
displayed in ZTERMCS as "5535". This does not mean that ISPF has
defined character set 5535 (X'159F'). Two other Z variables, ZTERMCS5
and ZTERMCP5, for character set and code page respectively, were
created to handle 5-character character sets and code pages. For
example, the character set 65535 is displayed in ZTERMCP5 as 65535.
ZTERMCP5 shr non  5 CECP support 5-digit code page
ZTERMCS5 shr non  5 CECP support 5-character set
ZTERMCS shr non  4 CECP support 4-digit character set
ZTHS shr non  1 Multicultural support thousands separator character
ZTS shr non  1 Multicultural support time separator character
ZTSICMD shr non 3276
7
The entire initial invocation command string which invoked the ISPF
environment. If storage cannot be obtained at startup, only the first 50
characters will be saved. The maximum length is 32767.
System variables
364  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 393

Table 38. General variables (continued)
Name Pool Type Len Description
ZTSSCMD shr non 3276
7
SELECT portion of the initial invocation command. The maximum length
is 32767.
ZUCTPREF shr non  4 First user command table name
ZUCTPRE2 shr non  4 Second user command table name
ZUCTPRE3 shr non  4 Third user command table name
ZUSER shr non  8 User ID
ZVERB shr out  8 Command verb after a SETVERB command table action
ZWINTTL any in N/A Title to be displayed in pop-up window frame
ZWSCDPG shr non  4 Variable ZWSCDPG is no longer used and is set to blanks.
ZWSCON shr non 68 Variable ZWSCON is no longer used and is set to blanks.
ZWSOPSYS shr non 16 Variable ZWSOPSYS is no longer used and is set to blanks.
ZSCRNAME examples
These three examples show you what can happen when you use ZSCRNAME.
Example 1
On the ISPF primary option panel the user issues the command SCRNAME POP. The primary option
panel's screen name is now POP. The user then invokes CLIST1.
CLIST1
   PROC 0
   ISPEXEC DISPLAY PANEL(PANELA)
   SET &ZSCRNAME = EDIT1
   ISPEXEC VPUT (ZSCRNAME) SHARED
   ISPEXEC EDIT DATASET ('PROJECT.GROUP.TYPE(BBBBBB)')
   SET &ZSCRNAME = EDIT2
   ISPEXEC VPUT (ZSCRNAME) SHARED
   ISPEXEC EDIT DATASET ('PROJECT.GROUP.TYPE(CCCCCC)')
   SET &ZSCRNAME = BROWSE1
   ISPEXEC VPUT (ZSCRNAME) SHARED
   ISPEXEC BROWSE DATASET ('PROJECT.GROUP.TYPE(DDDDDD)')
   SET &ZSCRNAME = LASTPAN
   ISPEXEC VPUT (ZSCRNAME) SHARED
   ISPEXEC DISPLAY PANEL(PANELA)
After the CLIST processes, the following results occur:
1. PANELA displays with screen name POP.
2. The EDIT session displays with the screen name EDIT1.
3. The next EDIT session displays with the screen name EDIT2.
4. The BROWSE session displays with the screen name BROWSE1.
5. PANELA displays with the screen name LASTPAN.
6. End from PANELA and the primary option panel displays with screen name POP.
Example 2
On the ISPF primary option panel the user issues the command SCRNAME POP. The primary option
panel's screen name is now POP. The user then invokes CLIST1 with the following results:
1. PANELA displays with screen name POP.
2. The EDIT session displays with the screen name EDIT1.
System variables
Appendix E. System variables  365

## Page 394

3. The user enters SCRNAME MYEDIT, so the screen name becomes MYEDIT.
4. After the EDIT session ends, the CLIST sets ZSCRNAME to EDIT2.
5. The EDIT session displays with the screen name EDIT2.
6. After this EDIT session ends, the CLIST sets ZSCRNAME to BROWSE1.
7. The BROWSE session displays with the screen name BROWSE1.
8. The user enters SCRNAME MYBROWSE PERM, so the screen name becomes MYBROWSE.
9. After the BROWSE session ends, the CLIST sets ZSCRNAME to LASTPAN.
10. PANELA displays with the screen name MYBROWSE. The CLIST command ZSCRNAME=LASTPAN is
ignored because the user issued the SCRNAME MYBROWSE command with the PERM parameter.
11. The CLIST completes and the primary option panel displays with the screen name MYBROWSE (again
because the user issued the SCRNAME MYBROWSE command with the PERM parameter).
Example 3
On the ISPF primary option panel the user issues the command SCRNAME POP. The primary option
panel's screen name is now POP. The user then invokes CLIST2.
CLIST2
   PROC 0
   SET &ZSCRNAME = STATE
   ISPEXEC VPUT (ZSCRNAME) SHARED
   ISPEXEC SELECT PANEL(MENUA) SCRNAME(NATION)
   ISPEXEC DISPLAY PANEL(PANELA)
After the CLIST processes, the following results occur:
1. MENUA displays with screen name NATION.
2. PANELA displays with the screen name STATE.
3. End from PANELA and the primary option panel displays with screen name POP.
Terminal and function keys
Table 39. System variables: Terminal and function keys
Name Pool Type Len Description
ZCOLORS shr non  4 Number of colors supported by the terminal type (either 1 or 7)
ZDBCS shr non  3 DBCS terminal capability (YES or NO)
ZFKA prof non  8 Current state of the function key area form (LONG, SHORT, OFF (no
display))
ZGE shr non  3 Terminal support for graphic escape order:
YES
graphic escape is supported
NO
graphic escape is not supported
Note: On a client that is using the JSON API, ZGE will be set to NO.
ZHILITE shr non  3 Extended highlighting availability (YES or NO)
ZIPADDR shr non 15 Variable ZIPADDR is no longer used and is set to blanks.
ZIPADD6 shr non 39 Variable ZIPADD6 is no longer used and is set to blanks.
ZIPPORT shr non  4 Variable ZIPPORT is no longer used and is set to zeros.
System variables
366  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 395

Table 39. System variables: Terminal and function keys (continued)
Name Pool Type Len Description
ZLUNAME shr non  8 VTAM® LU name of the current TSO session. Entering a TERMSTAT QUERY
command will refresh the value.
ZKEYS prof out  4 Number of Function keys
ZKLAPPL shr non  4 If KEYLIST is ON and it is a panel with the )PANEL statement, this
contains the application id where the current keylist came from.
ZKLNAME shr non  8 If KEYLIST is ON and it is a panel with the )PANEL statement, this
contains the name of the current keylist.
ZKLTYPE shr non  1 If KEYLIST is ON and it is a panel with the )PANEL statement, this
contains either P (for Private) or S (for Shared) for the current keylist.
ZKLUSE prof i/o  1 If KEYLIST is ON this contains Y, if it is OFF, it contains an N.
ZPFCTL prof i/o  5 User authorization to use PFSHOW command
• USER—User controls function key display with PFSHOW command
• ON—Display function key definitions on all panels
• OFF—Do not display function key definitions
ZPFFMT prof i/o  4 Number of Function key definitions displayed per line
• SIX—Always display six keys per line
• MAX—Display as many keys as will fit on each line
ZPFSET prof i/o  4 Function key definition set displayed
• PRI—Primary set (1-12)
• ALT—Alternate set (13-24)
• ALL—All keys (1-24)
ZPFSHOW prof out  4 PFSHOW command status
ZPFxx prof i/o 255 Setting for Function keys:
ZPF13-ZPF24 contain settings for the primary keys (for 12-key terminals:
physical keys 1-12; for 24-key terminals: physical keys 13-24)
ZPF01-ZPF12 contain settings for the alternate keys (for 24-key
terminals only: physical keys 1-12)
The maximum length is 255.
ZPFLxx prof i/o  8 Setting for Function key labels:
ZPFL13-ZPFL24 contain labels for the primary keys
ZPFL01-ZPFL12 contain labels for the alternate keys
ZPRIKEYS prof i/o  4 Indicates the set of Function keys that will be the primary keys
• LOW—1 to 12 are primary keys
• UPP—13 to 24 are primary keys
System variables
Appendix E. System variables  367

## Page 396

Table 39. System variables: Terminal and function keys (continued)
Name Pool Type Len Description
ZSCREEN shr non  1 Logical screen number up to 32 screens (1-9, A-W)
ZSCREEND shr non  4 Screen depth available for dialog use. In batch mode, this variable is set
by the value specified for BATSCRD on the ISPSTART call.
ZSCREENW shr non  4 Screen width available for dialog use. In batch mode this variable is set
by the value specified for BATSCRW on the ISPSTART call.
ZSCREEND and ZSCREENW are generally the dimensions of the physical
display screen. There are two exceptions:
1. On a 3290, if a dialog is executing on a display with a width of 160
characters and the user does a vertical split, then ZSCREENW is 80.
2. On a 3278 model 5, if a user has specified SCREEN FORMAT IS STD,
then ZSCREENW is 80 and ZSCREEND is 24, rather than the maximum
physical size of 132 by 27.
ZSCRMAXD shr non  4 Maximum screen depth available for dialog use. In batch mode, this
variable is set by the value specified for BATSCRD on the ISPSTART call.
ZSCRMAXW shr non  4 Maximum screen width available for dialog use. In batch mode, this
variable is set by the value specified for BATSCRW on the ISPSTART call.
ZSCRMAXD and ZSCRMAXW are identical to ZSCREEND and ZSCREENW,
except for terminals on which an alternate size is available. In that case,
ZSCRMAXD and ZSCRMAXW contain the screen configuration size that
produces the largest screen.
For the 3290, these variables contain sizes of the hardware partition on
which ISPF is operating.
ZSPLIT shr non  3 Split-screen mode in effect (YES or NO)
ZSWPBR prof non  1 List of logical screens displayed at bottom of screen.
Has a value of Y if the SWAPBAR feature is turned on. If ZSWPBAR is
not present, or does not have a value of Y then when ISPF is entered,
SWAPBAR is not automatically started.
ZTERM prof out  8 Terminal type as defined by option 0
Scrolling
Table 40. Scrolling variables
Name Pool Type Len Description
ZAMT prof i/o  4 Scroll amount for functions such as Dialog Test, the Keylist Utility, the
Command Table Utility, and the LIBDEF Utility
System variables
368  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 397

Table 40. Scrolling variables (continued)
Name Pool Type Len Description
ZDYNSCR any in  4 If ISPF was invoked by a client and a panel with a dynamic area that
can be scrolled is to be displayed, the application can set the value of
ZDYNSCR to indicate whether the dynamic area can be scrolled up, down,
left, or right on the next display. The variable value must be 4 bytes: 
• Byte 1 set to Y when the area can be scrolled up.
• Byte 2 set to Y when the area can be scrolled down.
• Byte 3 set to Y when the area can be scrolled left.
• Byte 4 set to Y when the area can be scrolled right.
ZSCBR prof i/o  4 Scroll amount for the BROWSE service
ZSCED prof i/o  4 Scroll amount for the EDIT service
ZSCML prof i/o  4 Scroll amount for member lists
ZSCRML shr non  1 Specifies if ISPF should scroll to the first member selected in the
member list after processing or disable the member list from automatic
scrolling and instead place the cursor in front of the last member
selected.
ZSCROLLA shr out  4 Value from scroll amount field (PAGE, MAX, number)
ZSCROLLD any in  4 Value to be used as default scroll value for scrollable dynamic areas and
table display
ZSCROLLN shr out  4 Scroll number as computed from the value in the scroll amount field
or entered as a scroll value. The maximum scroll number supported for
ZSCROLLN is 9999. If a scroll value greater than 9999 is entered the
value for ZSCROLLN is set to 9999. 
ZSCROLNL shr out  8 Scroll number as computed from the value in the scroll amount field
or entered as a scroll value. ZSCROLNL supports scroll numbers up to
9999999. 
ZTBLSCR any in  4 If ISPF was invoked by a client and the application will issue a table
display and use a variable model line to dynamically build the display
area for the table rows, the application can set the value of ZTBLSCR to
indicate whether the table display can be scrolled up, down, left, or right
on the next display. The variable value must be 4 bytes: 
• Byte 1 set to Y when the table can be scrolled up.
• Byte 2 set to Y when the table can be scrolled down.
• Byte 3 set to Y when the table can be scrolled left.
• Byte 4 set to Y when the table can be scrolled right.
ZXSMAX shr non  4 Maximum scroll amount allowed to be used in any scroll operation.
ZXSMIN shr non  4 Minimum scroll amount allowed to be used in any scroll operation.
ZUSC prof i/o  4 Scroll amount for the Data Set List Utility
System variables
Appendix E. System variables  369

## Page 398

PRINTG command
Table 41. System variables: PRINTG command
Name Pool Type Len Description
ZASPECT func in  4 Aspect ratio of printed output from PRINTG
ZDEVNAM func in  8 Device name for PRINTG
ZFAMPRT func non  4 Family printer type for PRINTG
Table display service
Table 42. System variables: Table display service
Name Pool Type Len Description
ZTDADD func out  3 More rows needed to satisfy scroll request (YES|NO)
ZTDAMT func out  4 Number of rows that the dialog should add to satisfy scroll up to 9999.
Set to 9999 when number of rows is greater than 9999. 
ZTDAMTL func out  8 Number of rows that the dialog should add to satisfy scroll
ZTDLROWS func in  6 Number of rows in the logical table (dynamic table expansion)
ZTDLTOP func in  6 Maps current top row in physical table to its position in logical table.
ZTDMARK any in See
note
User-defined text for table display Bottom-of-Data marker
Note: Value can be any length that is not more than the screen width.
ZTDMSG any in  8 User-defined message ID for table display top-row-displayed indicator
ZTDRET func in  8 Defines whether dialog wants to use scroll return feature.
ZTDROWS func out  6 Number of table rows upon return from table display
ZTDSCRP func in/ou
t
 6 CRP of top row to be displayed after the scroll
ZTDSELS func out  4 Number of selected table rows upon return from each table display
ZTDSIZE func out  4 Size (number of model sets) of the table display scrollable section
ZTDSRID func out  6 Rowid of the row pointed to by ZTDSCRP
ZTDTOP func out  6 Row number (CRP) of top row displayed during most recent table display
ZTDVROWS func out  6 Number of visible table rows upon return from table display
LIST service
Table 43. System variables: LIST service
Name Pool Type Len Description
ZLSTLPP shr non  4 Number of lines per page in list data set
ZLSTNUML shr non  4 Number of lines written to current list data set page
ZLSTTRUN shr non  4 List data set record length truncation value
System variables
370  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 399

LOG and LIST data sets
Table 44. System variables: LOG and LIST data sets
Name Pool Type Len Description
ZLOGNAME shr non  44 Contains the fully qualified data set name of the log data set.
ZLSTNAME shr non  44 Contains the fully qualified data set name of the list data set.
Dialog error
Table 45. System variables: Dialog error
Name Pool Type Len Description
ZERRALRM func out   3 Message alarm indicator (YES or NO)
ZERRHM func out   8 Name of help panel associated with error message
ZERRLM func out 512 Long error message text
ZERRMSG func out   8 Error message-id
ZERRSM func out  24 Short error message text
ZERRTYPE func out   8 Error message type
ZERRWIND func out   6 Error message window type
Tutorial panels
Table 46. System variables: Tutorial panels
Name Description
ZCONT Name of next continuation panel
ZHINDEX Name of first index panel
ZHTOP Name of top panel
ZIND YES specifies an index page
ZUP Name of parent panel
Selection panels
Table 47. System variables: Selection panels
Name Description
ZCMD Command input field
ZPARENT Parent menu name (when in explicit chain mode)
ZPRIM YES specifies panel is a primary option menu
ZSEL Command input field truncated at first period
System variables
Appendix E. System variables  371

## Page 400

DTL panels or panels containing a )PANEL section
Table 48. System variables: DTL panels or panels containing a )PANEL section
Name Pool Type Len Description
ZCURFLD func out  8 Name of field (or list column) containing the cursor when the user exits
the panel.
ZCURINX func out  8 For table display panels, the current row number of the table row
containing the cursor. The value ZCURINX is in character format. If the
cursor is not within a table row, this value will be 0.
ZCURPOS func out  4 Position of the cursor within the field specified by ZCURFLD when the
user exits the panel. The value in ZCURPOS is in character format. If the
cursor is not within a field, ZCURPOS will contain a 1.
Note: These variables will contain the values that would result if they were set to .CURSOR, .CSRPOS,
and .CSRROW, as the first statements in the panel's )PROC section.
System variables
372  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
