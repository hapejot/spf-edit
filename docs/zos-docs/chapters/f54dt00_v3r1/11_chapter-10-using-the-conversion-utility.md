# Chapter 10. Using the conversion utility

Source file: f54dt00_v3r1.md
Start page: 183
Page span: 183-202

## Page 183

Chapter 10. Using the conversion utility
The ISPF conversion utility is a tool that converts Dialog Tag Language (DTL) source files into ISPF panel
language source format or executable preprocessed ISPF format. There are two methods of invoking
the conversion utility: using the ISPF-supplied invocation panels, or using the conversion utility syntax.
In either case, the conversion utility must be run under ISPF control. In this chapter we explain both
methods of calling the conversion utility.
Using the ISPF-supplied invocation panels
Type this command on the command line to invoke the conversion utility and display the ISPF invocation
panel:
ISPDTLC
Invocation panel
This panel appears:
Figure 81. Conversion utility invocation panel (ISPCP01)
There are many options on this panel, so you need to scroll forward several times to view them all.
You must specify:
• At least one file containing DTL source
• The panel output file
• The message output file
Using the ISPF-supplied invocation panels
© Copyright IBM Corp. 1989, 2024 151

## Page 184

The language selection defaults to the current ISPF session language. The current selected language is
displayed as an information field on the panel.
Select the national language you want by using the Language action bar pull-down to enter a number
corresponding to the supported ISPF language. The language is used to provide formatting rules for tag
text. See “Text formatting” on page 13 for more information.
Panel input fields
Additional information about the panel input fields follows:
Member Name
If the member name is left blank or entered as a member pattern, a member list is displayed. You can
select one or more members to be converted from the member list.
DTL Source data set - n
You can specify up to three additional DTL source libraries on the invocation panel. See “Additional
DTL source files” on page 155 for more information.
Panel data set
If no panel output is required, you can specify NULLFILE or DUMMY in place of the panel output file
name.
Message data set
if no message output is required, you can specify NULLFILE or DUMMY in place of the message output
file name.
Log data set
The log file name is optional. If it is not specified and the messages are to be written to disk, log
output is written to the ISPF log file. If the log file is a PDS, a member name must be provided. You
may specify an asterisk to tell the conversion utility to use the input GML source file member name
as the output log file member name. However, if the input GML member is in the special DTLLST file
list format (discussed in “Conversion utility general information” on page 162) then a separate log file
member is created for each source member converted.
List data set
The list file name is optional. If it is not specified, list output is written to the ISPF list file. If the list
file is a PDS, a member name must be provided. You may specify an asterisk to tell the conversion
utility to use the input DTL source file member name as the output list file member name. However,
if the input GML member is in the special DTLLST file list format (discussed in “Conversion utility
general information” on page 162) then a separate list file member is created for each source member
converted.
SCRIPT data set
The SCRIPT file name is optional. If a SCRIPT output file is requested, it must be a PDS file. Member
names for the SCRIPT file are the same as the panel file.
Tables data set
The Tables file name is optional. If a tables file name is provided, it must be an 80-byte fixed-length
PDS file. When a tables file is provided, keylist and command table output is placed in this file.
Keylist Application ID
The optional Keylist Application ID is used when the APPLID attribute is omitted on the HELP, PANEL,
KEYL, or CMDTBL tags. This is the equivalent of the ID provided by the KEYAPPL option described in
“Conversion utility syntax” on page 156.
Conversion status message interval
When the conversion utility is running in interactive mode and the "Place ISPDTLC Messages in log
file" option is selected and the "List Source Convert Messages" option is deselected, a status message
containing the name of the current DTL source file member being converted is displayed in the long
message area. This message provides a conversion status when you are converting multiple members
using the DTLLST format member option. See “Converting multiple panels” on page 165 for more
information about the DTLLST syntax. The default message interval value is 1 which displays the
message for each member processed. This value can be set to 0 to suppress the message or to a
value that refreshes the message after a specified number of members have been converted.
Using the ISPF-supplied invocation panels
152  z/OS: z/OS ISPF DTL Guide

## Page 185

DISPLAY(W) option check interval
When the conversion utility is running in test mode and either the DISPLAY or DISPLAYW option is
selected, the converted panel is displayed for visual verification. A panel is displayed periodically
after the converted panel has been displayed to enable the user to control the DISPLAY or DISPLAYW
function.
The "DISPLAY(W) option check interval" option on the invocation panel controls the frequency of
the DISPLAY or DISPLAYW control function panel appearance. The default value is 1, so that the
control function panel is displayed after each converted panel display. The control panel enables you
to continue using the same display interval, cancel the DISPLAY or DISPLAYW option, or change the
control panel display interval.
All files specified must be preallocated.
When the log or list file is a PDS file and the member name is not an asterisk, all of the conversion results
are placed in the specified member. If the file name or member name is changed, the pending log or list
information is written to the previously specified member and a new log or list is generated beginning with
the next conversion. When the conversion utility ends, pending log and list files are written.
The log and list files can be either fixed length or variable length, with or without printer control. When the
file is allocated with print control specified, the conversion utility output begins in column 2; column 1 is
blank. When print control is not specified, the conversion utility output begins in column 1.
Panel options
The conversion utility options are displayed either as a multi-choice selection list by scrolling the
invocation panel, or in a series of multi-choice selection list panels with related options, through the
Options pull-down on the action bar.
You select the options by entering a "/" in front of the option description. If you want to deselect an
option, you must leave the selection choice field blank. These options are initially set to the default values
described in “Conversion utility syntax” on page 156. This table shows how the options and their valid
values are equivalent to conversion utility syntax. Note that b represents a blank.
Table 1. The equivalence of an option and valid value to conversion utility syntax
Options Valid values
Replace Panel/Message/SCRIPT/Keylist/
Command Members
/ is equivalent to REPLACE, the default.
b is equivalent to NOREPLACE.
Preprocess Panel Output / is equivalent to PREP, the default.
b is equivalent to NOPREP.
Place ISPDTLC Messages in log file b is equivalent to SCREEN, the default.
/ is equivalent to DISK.
Suppress Messages (ISPF extensions) b is equivalent to NOMSGSUPP, the default.
/ is equivalent to MSGSUPP.
Suppress Messages (CUA exceptions) b is equivalent to NOCUASUPP, the default.
/ is equivalent to CUASUPP.
Use CUA Panel Attributes / is equivalent to CUAATTR, the default.
b is equivalent to NOCUAATTR.
Using the ISPF-supplied invocation panels
Chapter 10. Using the conversion utility  153

## Page 186

Table 1. The equivalence of an option and valid value to conversion utility syntax (continued)
Options Valid values
Generate Statistics on Panel/Message/Script
Members / is equivalent to STATS, the default.
b is equivalent to NOSTATS.
Generate List file b is equivalent to NOLISTING, the default.
/ is equivalent to LISTING.
Generate List file with substitution b is equivalent to NOFORMAT, the default.
/ is equivalent to FORMAT.
Generate SCRIPT file b is equivalent to NOSCRIPT, the default.
/ is equivalent to SCRIPT.
Replace Log File Members / is equivalent to LOGREPL, the default.
b is equivalent to NOLOGREPL.
Replace List File Members / is equivalent to LISTREPL, the default.
b is equivalent to NOLISTREPL.
List Source Convert Msgs b is equivalent to NOLSTVIEW, the default.
/ is equivalent to LSTVIEW.
Use Expanded Message Format b is equivalent to NOMSGEXPAND, the
 default.
/ is equivalent to MSGEXPAND.
Allow DBCS b is equivalent to NODBCS, the default.
/ is equivalent to DBCS.
Specify KANA / is equivalent to KANA.
Specify NOKANA / is equivalent to NOKANA.
Create panels with Action bars / is equivalent to ACTBAR, the default.
b is equivalent to NOACTBAR.
Create panels with JSON API client display
controls / is equivalent to GUI, the default.
b is equivalent to NOGUI.
Add ISPDTLC version/timestamp to panel / is equivalent to VERSION, the default.
b is equivalent to NOVERSION.
Combine scrollable areas into panel body b is equivalent to NOMERGESAREA, the
 default.
/ is equivalent to MERGESAREA.
Display converted panels (*) b is equivalent to NODISPLAY, the default.
/ is equivalent to DISPLAY.
Using the ISPF-supplied invocation panels
154  z/OS: z/OS ISPF DTL Guide

## Page 187

Table 1. The equivalence of an option and valid value to conversion utility syntax (continued)
Options Valid values
Display converted panels in a window (*) b is equivalent to NODISPLAYW, the default.
/ is equivalent to DISPLAYW.
Bypass data set name validation (after first
cycle). b is equivalent to DSNCHK, the default.
/ is equivalent to NODSNCHK.
Enable graphic character display / is equivalent to GRAPHIC, the default.
b is equivalent to NOGRAPHIC.
Use full names in place of Z variables b is equivalent to ZVARS, the default.
/ is equivalent to NOZVARS.
Align DBCS prompt text with entry field b is equivalent to NODBALIGN, the default.
/ is equivalent to DBALIGN.
Preserve leading blanks when space is not
specified b is equivalent to NOPLEB, the default.
/ is equivalent to PLEB.
Process multiple line comment blocks b is equivalent to NOMCOMMENT, the
 default.
/ is equivalent to MCOMMENT.
Display additional DTL source data set list b —second input panel is not displayed.
/ —second input panel is displayed.
(*): If you specify DISPLAY or DISPLAYW, ISPDTLC must be run in test mode (Option 7) to force display
processing to use the current generated panel. An error message is issued if ISPDTLC is not being run in
test mode and either option is specified.
All of the entries from the panel (or panels) are saved in the user's profile.
Additional DTL source files
A second input panel is displayed for entry of up to twelve additional DTL source file data set names when
you perform one or more of these actions:
• You place the cursor on the point-and-shoot panel phrase "DTL input files 5-16" and press Enter.
• You select the "Display additional DTL source data set list" option from either the scrollable area of the
main panel or the Miscellaneous section of the Options action bar pull-down.
• You enter XDTL on the command line.
• You select Option 7 from the Commands action bar pull-down
Using the ISPF-supplied invocation panels
Chapter 10. Using the conversion utility  155

## Page 188

Figure 82. Panel ISPCP04
The entries for DTL source data sets 2-16 can be reset from the first invocation panel by placing the
cursor on the point-and-shoot field "Click here to reset DTL input files 2-16" and pressing Enter. The panel
redisplays with all entries except "DTL Source data set-1" reset to blanks.
Similarly, DTL source data sets 5-16 can be reset from the additional source files panel by placing the
cursor on the point-and-shoot field "Click here to reset DTL input files 5-16" and pressing Enter. The
invocation panel redisplays with all entries on the additional source files panel reset to blanks.
Converting multiple DTL source files
When ISPF finishes processing the DTL source you specify, the conversion utility displays the invocation
panel again to convert another DTL source file. This cycle continues until you exit or cancel the conversion
utility.
Calling help
You can get help for any field on the conversion utility invocation panel by moving the cursor to the field
and pressing the F1 key.
Using CUA panel attributes
CUA defines the default colors and emphasis techniques for individual panel elements. The conversion
utility generates panel element attributes that ISPF defines. The NOCUAATTR option can be used to
create panels with attributes that are not based on CUA defined elements.
See the z/OS ISPF Dialog Developer's Guide and Reference for more information about panel attributes.
Conversion utility syntax
This topic provides an alternative way to invoke the conversion utility. This alternative way provides
compatibility with previous ISPDTLC releases, and allows you to issue multiple calls from a user-specified
EXEC file. To read the conversion utility syntax, see “How to read the syntax diagrams” on page xxi for
more information.
Conversion utility syntax
156  z/OS: z/OS ISPF DTL Guide

## Page 189

You can view the allowable syntax and a description of the options by entering this command on the ISPF
command line:
   ISPDTLC ?
This command causes the general help panel to be displayed. The first line of information contains the
ISPDTLC version, APAR, and PTF numbers.
This diagram shows the conversion utility syntax:
ISPDTLC source-filespec(
REPLACE
NOREPLACE
SCREEN
DISK
NODBCS
DBCS
NOKANA
KANA
KEYLAPPL=xxxx
NOPANEL
PANEL
NOMSGSUPP
MSGSUPP
NOCUASUPP
CUASUPP
PREP
NOPREP
CUAATTR
NOCUAATTR
NOLSTVIEW
LSTVIEW
STATS
NOSTATS
NOSCRIPT
SCRIPT
NOLISTING
LISTING
NOFORMAT
FORMAT
NOMSGEXPAND
MSGEXPAND
LOGREPL
NOLOGREPL
LISTREPL
NOLISTREPL
ACTBAR
NOACTBAR
GUI
NOGUI
VERSION
NOVERSION
NOMERGESAREA
MERGESAREA
NODISPLAY
DISPLAY
NODISPLAYW
DISPLAYW
DSNCHK
NODSNCHK
GRAPHIC
NOGRAPHIC
ZVARS
NOZVARS
NODBALIGN
DBALIGN
NOPLEB
PLEB
NOMCOMMENT
MCOMMENT
NOV3PADC
V3PADC
NOGENACC
GENACC
NOZISPFRC
ZISPFRC
PROFILE=data-set-name
PROFDDN=ddname|* MAXFILES=
25
nnn
national-language
Conversion utility syntax
Chapter 10. Using the conversion utility  157

## Page 190

As shown in this diagram, when you specify options, a left parenthesis "(" is required before the first
option. If you specify mutually exclusive options such as SCREEN and DISK, the conversion utility issues
an error message and stops processing.
The syntax description follows:
source-filespec
Specify the sour c e -filespec  as a member of a partitioned data set (PDS) that contains the DTL source
to be converted to ISPF dialog elements. The first-level qualifier is the "user ID" and the second-level
qualifier is "GML" for the input data set name unless the PROFILE option is specified to override the
default.
Note: The conversion utility output is stored as commands, keylists, messages and panels. A single
source file might result in any or all of these objects. The source file might contain multiple command
tables, keylists, message members or panels. The names for the output objects are provided by the
CMDTBL, KEYL, MSGMBR, PANEL, and HELP tags. See the descriptions of these tags for additional
information.
REPLACE | NOREPLACE
Indicates whether members generated by the conversion utility replace existing members of the same
name. If you specify NOREPLACE, the conversion utility issues a warning message for each existing
member with the same name, but does not overwrite the existing member. If you specify REPLACE,
the conversion utility overwrites any existing member with the same name. REPLACE and NOREPLACE
affect keylists, commands, messages, panels, and SCRIPT files.
SCREEN | DISK
Indicates where to send information, warning, and error messages that occur while running the
conversion utility. If you specify SCREEN (the default), conversion messages are sent to the display
screen. If you specify DISK, conversion messages are sent to the designated log file.
Note: If your messages are not being written to the ISPF log, the specified conversion utility log file
must be preallocated. If your messages are being written to the ISPF log, the ISPF Settings option
must specify that an ISPF log is to be created.
Running the conversion utility with the DISK option causes additional messages to be appended to
the existing sequential ISPDTLC log file or the ISPF log. When using the conversion utility log file, a
separator record indicating the date and time of the execution is written to the log file before any
messages.
Messages are written to the screen automatically when:
• The conversion utility detects errors during initialization.
• System I/O errors occur.
DBCS | NODBCS
Indicates whether DBCS validation is performed on tag text following the tag suffix ">". Errors found
during DBCS validation cause the conversion utility to issue error or warning messages. DBCS shift-out
and shift-in characters are considered part of the text, thereby contributing to the length of the text.
Attention: DBCS strings cannot span records. That is, DBCS shift-out and shift-in characters
(shift-in characters end the DBCS string) must be on the same record. The conversion utility
ends with a severe error for incorrectly formed DBCS strings. If DBCS is specified and no
language is specified, the default language is Japanese.
KANA | NOKANA
Indicates whether the KANA keyword is added to the )BODY statement on panels and the message ID
line of messages. There is no default. If KANA is specified and no language is specified, the default
language is Japanese. See the z/OS ISPF Dialog Developer's Guide and Reference for more information.
Conversion utility syntax
158  z/OS: z/OS ISPF DTL Guide

## Page 191

KEYLAPPL=xxxx
The KEYLAPPL=xxxx option, where "xxxx" is equal to the 1–4 character application ID, must be
specified when the user includes a key list or lists in the DTL source and the APPLID attribute is
omitted on the KEYL tag. The application ID is used by the conversion utility to write to the correct key
list file.
Note: You cannot use "ISP" as an application ID, because the conversion utility is running as an ISP
application.
See z/OS ISPF Dialog Developer's Guide and Reference for restrictions on updating key lists.
PANEL | NOPANEL
The PANEL keyword forces the conversion utility to display the invocation panel even if a sour c e - 
filespec  has been entered. The PANEL keyword is disregarded when the conversion utility is running in
a batch job.
MSGSUPP | NOMSGSUPP
The MSGSUPP keyword causes the conversion utility to suppress warning messages concerning panel
formatting.
CUASUPP | NOCUASUPP
The CUASUPP keyword causes the conversion utility to suppress warning messages concerning CUA
Architecture non-compliance.
PREP | NOPREP
The NOPREP keyword causes the preprocessing of the output panel to be bypassed. Panel output is
stored in ISPF panel format.
CUAATTR | NOCUAATTR
The NOCUAATTR option can be used to create panels with attributes that are not based on CUA
defined elements. CUAATTR causes panels to be created using CUA attribute types as defined in the
z/OS ISPF Dialog Developer's Guide and Reference.
Note: If you specify NOCUAATTR, the conversion utility issues a message and changes the default
GRAPHIC option to NOGRAPHIC because GRAPHIC support is implemented only for CUA attributes.
LSTVIEW | NOLSTVIEW
The LSTVIEW keyword causes the conversion utility to display the "converting source file" message
in line mode when the user has routed the log file messages to DISK. NOLSTVIEW causes the
"converting source file" message to be displayed as a long message in full-screen mode. The
NOLSTVIEW keyword is disregarded when the conversion utility is running in a batch job; the
"converting source file" message is written to file SYSTSPRT.
STATS | NOSTATS
The NOSTATS keyword causes the conversion utility to bypass the creation of member statistics on
created panels and messages. STATS and NOSTATS affect messages, panels, and SCRIPT files.
SCRIPT | NOSCRIPT
The SCRIPT keyword causes the conversion utility to create a panel image template as a member of
a file allocated to DTLSCR. The panel image template has BookMaster® tags included so that it may
be incorporated into documentation files. Input and output fields in the panel image are shown as
underscores. Runtime substitution variables are shown as "&varname". Editing is required to supply
appropriate information for input and output fields and "&varname" values.
Note: The specified conversion utility SCRIPT output file must be preallocated.
LISTING | NOLISTING
The LISTING keyword causes the conversion utility to create a list file of the processed source GML
records. This file is allocated to DTLLIST or if no file name is provided to the conversion utility, the
list is added to the standard ISPF list data set. The file you provide can be in either sequential or
partitioned format.
Conversion utility syntax
Chapter 10. Using the conversion utility  159

## Page 192

Note: If your messages are not being written to the ISPF list file, the specified conversion utility list
file must be preallocated.
Indentation of nested tags (to a limit of 30 columns) is provided for readability. The listing is limited to
an 80-column format. Tag contents that would extend beyond the right column are flowed to multiple
lines.
The formatted listing is unchanged from the original DTL source file except for indentation processing.
FORMAT | NOFORMAT
The FORMAT keyword causes the conversion utility to create a list file of the source GML records after
entity substitution is performed. (The FORMAT keyword implies the LISTING keyword.) The number at
the left side of the list indicates the file nest level. If the LISTING keyword is specified in combination
with the NOFORMAT keyword, all substitution is bypassed and the listing can be used as a formatted
input GML file.
MSGEXPAND | NOMSGEXPAND
The MSGEXPAND keyword causes the conversion utility to expand the warning and error messages to
include an indicator of the major type of tag in process (PANEL, HELP, KEYL, MSGMBR, CMDTBL) along
with the object name.
LOGREPL | NOLOGREPL
Indicates whether members generated by the conversion utility replace existing log file PDS members
of the same name. If you specify NOLOGREPL, the conversion utility issues a warning message for
each existing member with the same name but does not overwrite the existing member.
LISTREPL | NOLISTREPL
Indicates whether members generated by the conversion utility replace existing list file PDS members
of the same name. If you specify NOLISTREPL, the conversion utility issues a warning message for
each existing member with the same name but does not overwrite the existing member.
ACTBAR | NOACTBAR
Indicates whether the ISPF panel statements for action bars are added to the generated panel. If you
specify NOACTBAR, the panel sections for )ABC, )ABCINIT, and )ABCPROC and the action bar lines
from the panel body are not added to the output panel. (The DTL source for action bar creation is
syntax-checked in all cases.)
If a PANEL tag includes the keyword ACTBAR, this option is ignored for that panel.
GUI | NOGUI
The NOGUI keyword causes the panel keywords for mnemonics and check boxes to be removed from
the generated panel.
If you specify MNEMGEN=YES on the AB tag or CHKBOX=YES on the SELFLD tag, this option is ignored
for the specified tag. This option can be overridden by specifying the TYPE attribute on the PANEL tag.
VERSION | NOVERSION
Indicates whether the ISPDTLC version number, maintenance level, and member creation date and
time are added as comments following the )END panel statement and the last message of a message
member. In addition, VERSION causes the conversion language (ENGLISH, GERMAN, JAPANESE, and
so on), Panel ID, and ISPF version to be added to the )ATTR panel statement line as a comment. If you
specify NOVERSION, the comments are not added to the generated panel or message.
Note: If the PREP conversion option has been specified, the comments are not part of the final panel
because they are not processed by the ISPPREP utility.
NOMERGESAREA | MERGESAREA
Indicates whether scrollable areas are merged into panel body sections. Merging occurs only when
the entire scrollable area can be contained within the panel body, allowing for the function key area.
This option can be overridden by specifying the MERGESAREA attribute on the HELP or PANEL tag.
NODISPLAY | DISPLAY
Conversion utility syntax
160  z/OS: z/OS ISPF DTL Guide

## Page 193

Indicates whether the converted panel is displayed by the conversion utility immediately after the
panel is created. The display is in full-screen format. The DISPLAY keyword is disregarded when the
conversion utility is running in a batch job.
Note: If you specify DISPLAY, ISPDTLC must be run in test mode (Option 7) to force display
processing to use the current generated panel. An error message is issued if ISPDTLC is not being
run in test mode and this option is specified.
DISPLAY causes each converted panel to be displayed until the user enters DISPLAY OFF on the
command line of a displayed panel or selects option 2 from the display control panel. The control
panel is displayed periodically, according to the interval specified in the "DISPLAY(W) option check
interval" field on the invocation panel, or from the Miscellaneous choice on the Options action bar.
NODISPLAYW | DISPLAYW
Indicates whether the converted panel is displayed by the conversion utility immediately after the
panel is created. The display is within a window. The DISPLAYW keyword is disregarded when the
conversion utility is running in a batch job.
Note: If you specify DISPLAYW, ISPDTLC must be run in test mode (Option 7) to force display
processing to use the current generated panel. An error message is issued if ISPDTLC is not being run
in test mode and this option is specified.
DISPLAYW causes each converted panel to be displayed until the user enters DISPLAY OFF on the
command line of a displayed panel or selects option 2 from the display control panel. The control
panel is displayed periodically, according to the interval specified in the "DISPLAY(W) option check
interval" field on the invocation panel, or from the Miscellaneous choice on the Options action bar.
DSNCHK | NODSNCHK
Indicates whether file validation is performed on the files specified on the interactive panel after
the first conversion cycle has been completed. If you specify NODSNCHK and any specified file is
unavailable, the conversion fails when the conversion utility attempts to use the file. The NODSNCHK
keyword is disregarded when the conversion utility is running in a batch job.
GRAPHIC | NOGRAPHIC
Indicates, for host display only, whether the action bar separator line and visible horizontal divider
lines display as dashed lines or as solid lines. The GRAPHIC option can be overridden by the tag
definition that generates the line. See tag attribute descriptions for
• “AB (Action Bar)” on page 179,
• “AREA (Area)” on page 189,
• “CHDIV (Choice Divider)” on page 207,
• “DA (Dynamic Area)” on page 250,
• “DIVIDER (Area Divider)” on page 258,
• “GA (Graphic Area)” on page 295,
• “GRPHDR (Group Header)” on page 300,
• “LSTFLD (List Field)” on page 341, and
• “LSTGRP (List Group)” on page 345
for information about the creation of action bar separator and various types of visible divider lines. If
you specify NOGRAPHIC, the action bar separator line and visible divider lines are created as dashed
lines.
Note: If you specify NOCUAATTR, the conversion utility issues a message and change the default
GRAPHIC option to NOGRAPHIC because GRAPHIC support is implemented only for CUA attributes.
ZVARS | NOZVARS
Indicates whether variable names are formatted as Z variables. If you specify NOZVARS, the variable
name is used in panel )BODY or )AREA formatting unless the variable name is longer than the defined
field width.
Conversion utility syntax
Chapter 10. Using the conversion utility  161

## Page 194

DBALIGN | NODBALIGN
For DBCS language conversions only. Indicates whether fields with PMTLOC=ABOVE are aligned so
that the first position of the prompt text is formatted above the first position of the field.
PLEB | NOPLEB
Indicates whether leading blanks in ENTITY text strings are processed. This option is effective only for
ENTITY definitions that do not specify the "space" keyword.
MCOMMENT | NOMCOMMENT
Indicates whether multiple line comment blocks, starting with <! -- or <: -- and ending with the first
--> found are valid. Comment blocks can include DTL tags.
NOV3PADC | V3PADC
Indicates whether ISPDTLC Version 3 padding is added to global definitions for input fields in
the )ATTR panel section. When ISPDTLC is invoked with the V3PADC option, the ISPF keyword
PADC('_') is added to input attribute definitions if there is no PAD or PADC attribute specified on
the PANEL tag.
GENACC | NOGENACC
Indicates whether DTL formatting of leader dots will be contiguous dots (.....), and whether default
formatting of multiple column, single choice selection lists will be in left-to-right, top-to-bottom order.
NOZISPFRC | ZISPFRC
Indicates whether the compiler places the final return code into the ISPF shared pool variable
ZISPFRC. This option is not supported by the panel interface.
PROFILE=data-set-name | PROFDDN=ddname | PROFDDN=*
The PROFILE or PROFDDN option provides access to the data set name that contains the conversion
utility defined ddnames and associated PDS or sequential file names to be used by the conversion
utility during I/O. A sample profile member ISPDTLP is included in the ISPSLIB skeleton library.
The data-set-name value must be a fully qualified data set name that specifies either a sequential or a
partitioned data set. If the profile entry is part of a partitioned data set, then the member name must
be included in the data-set-name specification.
The ddname value specifies a ddname allocated to a profile data set.
The "*" value specifies that the ddnames used in the conversion are found as preallocated files. See
“Default data set names” on page 167 for the ddnames used by ISPDTLC.
The profile data set and all data sets defined within the profile must be preallocated.
MAXFILES=25|nnn
The maximum number of nested embed files that the compiler can process. If the message ISPC810E
is issued this number can be increased to as high as 999. Doing so does not guarantee a successful
compilation and the DTL may still have to be restructured.
national-language
Specifies the language rules to be used for formatting the tag text. Supported language keywords are:
CHINESES     ENGLISH      ITALIAN      PORTUGUE     UPPERENG
CHINESET     FRENCH       JAPANESE     SGERMAN
DANISH       GERMAN       KOREAN       SPANISH
Note: ISPF must have been installed to support the language requested by the conversion utility.
Conversion utility general information
The output panel library can be defined as either fixed length or variable length. A fixed-length record
library must have a record length of 80, 132, or 160 bytes. Record lengths for variable-length record
libraries must be increased by 4. Variable-length libraries defined with a record length other than 84, 136,
or 164 are treated as the next smaller standard size. Thus, a variable-length file of 255 bytes is treated as
164, and a variable-length file of 100 bytes is treated as 84.
Conversion utility syntax
162  z/OS: z/OS ISPF DTL Guide

## Page 195

The NOPREP option directs the conversion utility to write the panels being processed directly to the
specified panel output file in the ISPF source format. The overall width for the created panels is limited by
the record length of the designated file. Thus, if you have specified a panel library with a fixed length of 80
bytes (or a variable length of 84 bytes), the maximum panel width allowed on the PANEL tag is 80.
The PREP (default) option causes the creation of a temporary panel library to receive the ISPF source
panel format file. The temporary library is created with a record length of 160 bytes. Multiple panels
created in PREP mode are stored in the temporary library and converted through one call to ISPPREP.
When all of the panels are converted, the temporary library is deleted. ISPPREP is called by the
conversion utility when you do this:
• Change the name of the output panel library on the ISPDTLC invocation panel and then convert another
panel.
• Deselect the Preprocess Panel Output option on the ISPDTLC invocation panel and then convert another
panel.
• Change the Generate Statistics on Panel/Message/Script Members option on the ISPDTLC invocation
panel and then convert another panel.
• Enter "PREP" on the command line of the ISPDTLC invocation panel or select "PREP" from the
Commands action bar pull-down.
• Exit from the conversion utility.
ISPPREP is also called when:
• The number of extents of the temporary library exceeds 5.
• The number of members written to the temporary library exceeds 50.
ISPPREP output for panels longer than 80 bytes can be stored in a panel library with a fixed record length
of 80 (or a variable record length of 84). Thus, you can create larger than standard panels in PREP mode
while directing the final panel output to a library defined with a standard length. It is the developer's
responsibility to ensure that the WIDTH specified on the PANEL tag is appropriate for the device intended
to display the panel.
When the log or list files are specified as members of a partitioned data set, and the log or list file member
name is specified as an asterisk (*) the member is written before the invocation panel is redisplayed.
Otherwise, the log or list file is stored in memory (and added to for additional DTL source conversions)
until one of these occurances:
• The output log or list data set name is changed and another conversion is performed.
• The member name of the log or list file is changed on the invocation panel and another conversion is
performed.
• The input DTL source member name is changed when the log or list member name is specified as an
asterisk.
• You enter on the command line or select from the Commands action bar pull-down:
SAVELOG
to save the log file
SAVELIST
to save the list file
SAVEALL
to save both log and list files.
• You exit the conversion utility.
When the log file is specified as a partitioned data set, messages issued when the conversion utility ends
are directed to the screen.
When the CANCEL command is entered, ISPDTLC displays a cancellation confirmation panel. This panel
provides options for disposition of pending log and list file members and for any panels to be processed
by ISPPREP. An option is also provided to ignore the CANCEL command and resume ISPDTLC processing.
Conversion utility syntax
Chapter 10. Using the conversion utility  163

## Page 196

ISPF Dialog Tag Language Conversion Utility
 You have entered the CANCEL command.
 Specify termination processing choice.
 _  1. Do not save log or list files. Do not
       preprocess pending panels.
    2. Save log and list files only.
    3. Preprocess pending panels only.
    4. Save pending log and list files.
       Preprocess pending panels.
    5. Ignore CANCEL command and resume
       processing.
  F1=Help      F3=End       F4=Return    F5=Rfind
  F6=Rchange  F10=Left     F11=Right    F12=Cretriev
Figure 83. ISPF Dialog Tag Language conversion utility - c onfirm  cancel
The panel appears with option 1 preselected. You may choose another option to save log and list files
only, preprocess pending panels only, save log and list files and preprocess pending panels, or resume
processing.
When you enter the SUBMIT command, ISPDTLC creates and submits a batch job, using the file names
and options specified on the interactive panel. After the job is submitted, the interactive panel is
redisplayed. The batch JCL file is built using the ISPF skeleton ISPDTLB.
You can also run ISPDTLC from ISPF options 4 and 5 and from the workplace member list.
Note: From the workplace member list, enter "T" (TSO) in front of the member name to be processed. On
the TSO pop-up panel enter
"ISPDTLC / (PANEL RETURN"
to run a foreground conversion or
"ISPDTLC / (PANEL SUBMIT"
to submit a batch job.
After you complete the required ISPDTLC invocation panel fields and press Enter, the conversion runs or
the job is submitted, and control is returned to the previous option.
Extremely large DTL input source files (source files that contain multiple panel, message, key list, and
application command table definitions) might cause memory capacity to be exceeded. Should this occur,
split the DTL input source file into multiple files with fewer panels, message members, key lists, or
command table definitions or reduce the record length of the input source file.
When ISPDTLC is invoked recursively, that is, more than 1 time from the same ISPF screen, this panel is
displayed.
   ISPF Dialog Tag Language Conversion Utility
 CAUTION:
 ISPDTLC has been invoked recursively.
 Results are not predictable.
 Enter processing option.
 _  1. CANCEL this invocation of ISPDTLC.
    2. Proceed with recursive execution.
  F1=Help     F3=End      F4=Return   F5=Rfind
  F6=Rchange F10=Left    F11=Right   F12=Cancel
Figure 84. ISPF Dialog Tag Language conversion utility - recursive invoke
Conversion utility syntax
164  z/OS: z/OS ISPF DTL Guide

## Page 197

The panel appears with option 1 preselected. If you select option 2, the new invocation is processed.
Because of possible region size limitations, results are not predictable.
The recursive invocation check is based on the setting of a profile variable that is unique for each active
screen. If the recursive check panel appears following an abend, the profile variable was not properly
reset when the abend occurred. In this case, select option 2 to allow ISPDTLC to continue.
If the conversion utility is called without a sour c e -filespec  or if the PANEL option has been specified, the
invocation panel is displayed. If other options have been specified, they are merged with the options from
the profile before the display. The PROFILE option is disregarded when the invocation panel is displayed.
The national-language selection UPPERENG causes the conversion utility to use the uppercase version
of the ENGLISH program literals. In addition, the tag text for all tags except <SOURCE> is translated to
uppercase during the conversion process.
The national-language selection SGERMAN causes the conversion utility to use a special German-to-
Swiss German conversion routine to create Swiss German panels from either German or Swiss German
DTL source files.
Converting multiple panels
The sour c e -filespec  can be a special file which is a list of other files to be converted. When you use this
option, you can convert multiple panels with a single call to the conversion utility. The format of the file
list is:
DTLLST source-filespec 1
DTLLST source-filespec 2
⋮
The format of sour c e -filespec  is the same as any other call to the conversion utility. Duplicate sour c e - 
filespec  names within DTLLST are ignored.
ISPF conversion utility messages
During processing, the conversion utility can issue information, warning, and error messages. For
unsupported DTL tags and attributes that generate warning messages, the conversion utility either
ignores the tag or attribute, or sets attribute values to the conversion utility defaults. If the conversion
causes error messages, the conversion utility does not generate the ISPF file (key list, panel, application
command table, or message member) that would have been created had the error not occurred.
In the message listing, the line numbers displayed in the messages might not always match the line
numbers of the source file that caused the message. This occurs because the conversion utility must
sometimes continue to read the source file until it encounters an end tag or a new tag before issuing a
message. You should be able to determine which source line created the message by examining the DTL
source file.
There are two options required to suppress all noncritical messages.
• The MSGSUPP option is used to suppress messages related to ISPDTLC formatting.
• The CUASUPP option is used to suppress messages related to CUA architecture deviations allowed by
ISPDTLC. Examples include nonstandard use of F1/F13, F3/F15, and F12/F24 keylist commands, and
the use of the SMSG attribute on the MSG tag to create a short message.
When each DTL source file conversion is completed, the conversion utility issues a message listing the
number of warning and error messages generated. If the MSGSUPP or CUASUPP option(s) have been
specified, an additional message is issued with the total number of messages suppressed.
When the conversion utility is finished, it issues a message listing the total number of warning and error
messages generated. If the MSGSUPP or CUASUPP option(s) have been specified, a message is issued
with the total number of messages suppressed. The end of job messages listing the total number of
messages are placed in the ISPF log file, if the log file is available; otherwise the overall totals are written
to the terminal.
Converting multiple panels
Chapter 10. Using the conversion utility  165

## Page 198

Return codes
Here is a list of return codes that explains the results of the conversion invocation.
 0
No warnings, errors, or severe errors
 1
All messages were suppressed.
 4
CANCEL command ended ISPDTLC
 8
Only warnings were found
16
At least one DTL conversion had at least one error
20
At least one DTL conversion ended with a severe error.
For multiple conversions, the highest return code is used.
Conversion results
The results of the conversion are placed in the shared pool.
• The variable ZDTLRC contains the return code.
• The variable ZDTLNWRN contains the number of warning messages.
• The variable ZDTLNERR contains the number of error messages.
• The variable ZDTLNSUP contains the number of suppressed messages.
Conversion utility file names
The conversion utility is provided as a REXX exec on the ISPF product tape.
The ISPDTLC exec can reside in a CLIST data set allocated to SYSPROC or in an EXEC data set allocated
to SYSEXEC. For more information about the use of REXX execs on MVS™, refer to the z/OS TSO/E REXX
User's Guide.
Additional Requirements:
• All data sets must be allocated before running the conversion utility. In addition, the conversion utility
uses ISPF services to produce command table and key list output, which means that a partitioned data
set must be allocated to ISPTABL. See the topic on allocating ISPF libraries in the z/OS ISPF User's
Guide Vol I for more information.
• To allow the user to specify the source and destination data sets when using the conversion utility
syntax, seven ddnames have been reserved in an allocation profile with associated data set names to be
provided by the user.
Note: ISPDTLC profiles from previous releases can be used without change. However, a warning
message is issued if the DTLMIN or DTLNLS ddname records are encountered.
• A sample profile member ISPDTLP is included in the ISPSLIB skeleton library. You can modify the
data set names for installation or user use. DTL format comments (<!--comment text--> or (<:--
comment text-->) can be used in the profile data set or member. Do not modify the DDNAMEs in this
table (column one). A sample user updated profile member follows:
DDNAME
Data Set
DTLGML
any.GML.input
ISPF conversion utility messages
166  z/OS: z/OS ISPF DTL Guide

## Page 199

DTLPAN
your.panel.output
DTLMSG
your.msg.output
DTLLOG (*)
your.log.output
DTLLIST (*)
your.list.output
DTLSCR
your.script.output
DTLTAB
your.table.output
(*) The sequential data set name associated with the DTLLOG and DTLLIST ddnames should have the
same characteristics and attributes as the LOG and LIST data sets for ISPF.
DTLGML is the input file to the conversion utility. The last 6 files are for output and are usually the user's
own data sets.
• For compatibility with previous ISPDTLC releases, the user can provide the allocation profile name on
invocation:
ISPDTLC source-filespec (disk PROFILE=User.profile
The data set name following the PROFILE keyword must be a fully qualified data set name. When
specifying the data set name, do not include quotes.
The profile data-set-name can specify either a sequential or a partitioned data set. If the profile entry
is part of a partitioned data set, then the member name must be included in the data-set-name
specification. The profile data set and all data sets defined within the profile must be preallocated.
The profile can contain multiple entries for each ddname. For output files, the first valid data set name
in the profile is used. For the input GML file, each data set is checked in the order they are found in
the profile for the member name specified. The first match by member name is used as the file to be
converted.
When the data set associated with either the DTLLOG or DTLLIST ddname in the profile is a PDS, the
member name may be a single asterisk. When the asterisk notation is present, the conversion utility
uses the same name for the log or list file as the source GML member name.
Default data set names
Here is a table that shows an example of the default data set names used for the conversion utility.
USERID is the user's TSO prefix.
Table 2. Default data set names used for conversion utility
DDNAME Data Set Type Description
DTLGML userid.GML PDS The DTL source PDS where GML members
reside.
DTLPAN userid.PANELS or NULLFILE or
DUMMY
PDS PDS for panel member output. May be
specified as NULLFILE or DUMMY for cases
where no panel output is required.
DTLMSG userid.MSGS or NULLFILE or
DUMMY
PDS PDS for message member output. May be
specified as NULLFILE or DUMMY for cases
where no message output is required.
Default data set names
Chapter 10. Using the conversion utility  167

## Page 200

Table 2. Default data set names used for conversion utility (continued)
DDNAME Data Set Type Description
DTLLOG userid.ISPDTLC.LOG
userid.LOGLIB(logmem)
SEQ or
PDS
Optional. User's log data set for conversion
utility messages. If not specified, log
messages are written to the standard ISPF
log data set. If file is a PDS, member name
must be included in the data set name
specification.
DTLLIST userid.ISPDTLC.LIST
userid.LISTLIB(listmem)
SEQ or
PDS
Optional. User's list data set for conversion
utility messages. If not specified, list
messages are written to the standard ISPF
list data set. If file is a PDS, member name
must be included in the data set name
specification.
DTLSCR userid.SCRIPT PDS Optional. PDS for panel member
documentation output. The DTLSCR data set
is required only if the SCRIPT option is
specified.
DTLTAB userid.TABLES PDS Optional. PDS for keylist and command table
output. If specified, a LIBDEF is performed
for ISPTLIB and ISPTABL and the keylist and
command table output is written to the data
set.
Default data set names
168  z/OS: z/OS ISPF DTL Guide

## Page 201

Part 2. Dialog Tag Language (DTL) reference
This part contains these chapters:
• Chapter 11, “Markup declarations and DTL macro reference,” on page 171
A reference listing for each DTL markup declaration.
• Chapter 12, “Tag reference,” on page 179
A reference listing for each DTL tag. Each reference listing contains a syntax diagram and attribute
definition list, a description, and examples of usage.
© Copyright IBM Corp. 1989, 2024 169

## Page 202

170  z/OS: z/OS ISPF DTL Guide
