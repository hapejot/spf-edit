# Appendix F. Accessibility

Source file: f54ug00_v3r1.md
Start page: 207
Page span: 207-226

## Page 207

Appendix F. Accessibility
Accessible publications for this product are offered through IBM Documentation for z/OS (www.ibm.com/
docs/en/zos).
If you experience difficulty with the accessibility of any z/OS documentation see How to Send Feedback to
IBM to leave documentation feedback.
© Copyright IBM Corp. 1980, 2024 179

## Page 208

180  z/OS: z/OS ISPF User's Guide Vol I

## Page 209

Notices
This information was developed for products and services that are offered in the USA or elsewhere.
IBM may not offer the products, services, or features discussed in this document in other countries.
Consult your local IBM representative for information on the products and services currently available in
your area. Any reference to an IBM product, program, or service is not intended to state or imply that
only that IBM product, program, or service may be used. Any functionally equivalent product, program, or
service that does not infringe any IBM intellectual property right may be used instead. However, it is the
user's responsibility to evaluate and verify the operation of any non-IBM product, program, or service.
IBM may have patents or pending patent applications covering subject matter described in this
document. The furnishing of this document does not grant you any license to these patents. You can
send license inquiries, in writing, to:
IBM Director of Licensing
IBM Corporation
North Castle Drive, MD-NC119
Armonk, NY 10504-1785
United States of America
For license inquiries regarding double-byte character set (DBCS) information, contact the IBM Intellectual
Property Department in your country or send inquiries, in writing, to:
Intellectual Property Licensing
Legal and Intellectual Property Law
IBM Japan Ltd.
19-21, Nihonbashi-Hakozakicho, Chuo-ku
Tokyo 103-8510, Japan 
The following paragraph does not apply to the United Kingdom or any other country where such
provisions are inconsistent with local law: INTERNATIONAL BUSINESS MACHINES CORPORATION
PROVIDES THIS PUBLICATION "AS IS" WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESS OR
IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF NON-INFRINGEMENT,
MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Some states do not allow disclaimer of
express or implied warranties in certain transactions, therefore, this statement may not apply to you.
This information could include technical inaccuracies or typographical errors. Changes are periodically
made to the information herein; these changes will be incorporated in new editions of the publication.
IBM may make improvements and/or changes in the product(s) and/or the program(s) described in this
publication at any time without notice.
This information could include missing, incorrect, or broken hyperlinks. Hyperlinks are maintained in
only the HTML plug-in output for IBM Documentation. Use of hyperlinks in other output formats of this
information is at your own risk.
Any references in this information to non-IBM websites are provided for convenience only and do not in
any manner serve as an endorsement of those websites. The materials at those websites are not part of
the materials for this IBM product and use of those websites is at your own risk.
IBM may use or distribute any of the information you supply in any way it believes appropriate without
incurring any obligation to you.
Licensees of this program who wish to have information about it for the purpose of enabling: (i) the
exchange of information between independently created programs and other programs (including this
one) and (ii) the mutual use of the information which has been exchanged, should contact:
IBM Corporation
Site Counsel
2455 South Road
© Copyright IBM Corp. 1980, 2024 181

## Page 210

Poughkeepsie, NY 12601-5400
USA
Such information may be available, subject to appropriate terms and conditions, including in some cases,
payment of a fee.
The licensed program described in this document and all licensed material available for it are provided by
IBM under terms of the IBM Customer Agreement, IBM International Program License Agreement or any
equivalent agreement between us.
Any performance data contained herein was determined in a controlled environment. Therefore, the
results obtained in other operating environments may vary significantly. Some measurements may have
been made on development-level systems and there is no guarantee that these measurements will be
the same on generally available systems. Furthermore, some measurements may have been estimated
through extrapolation. Actual results may vary. Users of this document should verify the applicable data
for their specific environment.
Information concerning non-IBM products was obtained from the suppliers of those products, their
published announcements or other publicly available sources. IBM has not tested those products and
cannot confirm the accuracy of performance, compatibility or any other claims related to non-IBM
products. Questions on the capabilities of non-IBM products should be addressed to the suppliers of
those products.
All statements regarding IBM's future direction or intent are subject to change or withdrawal without
notice, and represent goals and objectives only.
This information contains examples of data and reports used in daily business operations. To illustrate
them as completely as possible, the examples include the names of individuals, companies, brands, and
products. All of these names are fictitious and any similarity to the names and addresses used by an
actual business enterprise is entirely coincidental.
COPYRIGHT LICENSE:
This information contains sample application programs in source language, which illustrate programming
techniques on various operating platforms. You may copy, modify, and distribute these sample programs
in any form without payment to IBM, for the purposes of developing, using, marketing or distributing
application programs conforming to the application programming interface for the operating platform
for which the sample programs are written. These examples have not been thoroughly tested under
all conditions. IBM, therefore, cannot guarantee or imply reliability, serviceability, or function of these
programs. The sample programs are provided "AS IS", without warranty of any kind. IBM shall not be
liable for any damages arising out of your use of the sample programs.
Terms and conditions for product documentation
Permissions for the use of these publications are granted subject to the following terms and conditions.
Applicability
These terms and conditions are in addition to any terms of use for the IBM website.
Personal use
You may reproduce these publications for your personal, noncommercial use provided that all proprietary
notices are preserved. You may not distribute, display or make derivative work of these publications, or
any portion thereof, without the express consent of IBM.
Commercial use
You may reproduce, distribute and display these publications solely within your enterprise provided
that all proprietary notices are preserved. You may not make derivative works of these publications, or
182  z/OS: z/OS ISPF User's Guide Vol I

## Page 211

reproduce, distribute or display these publications or any portion thereof outside your enterprise, without
the express consent of IBM.
Rights
Except as expressly granted in this permission, no other permissions, licenses or rights are granted, either
express or implied, to the publications or any information, data, software or other intellectual property
contained therein.
IBM reserves the right to withdraw the permissions granted herein whenever, in its discretion, the use
of the publications is detrimental to its interest or, as determined by IBM, the above instructions are not
being properly followed.
You may not download, export or re-export this information except in full compliance with all applicable
laws and regulations, including all United States export laws and regulations.
IBM MAKES NO GUARANTEE ABOUT THE CONTENT OF THESE PUBLICATIONS. THE PUBLICATIONS
ARE PROVIDED "AS-IS" AND WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED,
INCLUDING BUT NOT LIMITED TO IMPLIED WARRANTIES OF MERCHANTABILITY, NON-INFRINGEMENT,
AND FITNESS FOR A PARTICULAR PURPOSE.
IBM Online Privacy Statement
IBM Software products, including software as a service solutions, ("Software Offerings") may use cookies
or other technologies to collect product usage information, to help improve the end user experience,
to tailor interactions with the end user, or for other purposes. In many cases no personally identifiable
information is collected by the Software Offerings. Some of our Software Offerings can help enable you
to collect personally identifiable information. If this Software Offering uses cookies to collect personally
identifiable information, specific information about this offering’s use of cookies is set forth below.
Depending upon the configurations deployed, this Software Offering may use session cookies that collect
each user’s name, email address, phone number, or other personally identifiable information for purposes
of enhanced user usability and single sign-on configuration. These cookies can be disabled, but disabling
them will also eliminate the functionality they enable.
If the configurations deployed for this Software Offering provide you as customer the ability to collect
personally identifiable information from end users via cookies and other technologies, you should seek
your own legal advice about any laws applicable to such data collection, including any requirements for
notice and consent.
For more information about the use of various technologies, including cookies, for these purposes, see
IBM’s Privacy Policy at ibm.com®/privacy and IBM’s Online Privacy Statement at ibm.com/privacy/details
in the section entitled “Cookies, Web Beacons and Other Technologies,” and the “IBM Software Products
and Software-as-a-Service Privacy Statement” at ibm.com/software/info/product-privacy.
Policy for unsupported hardware
Various z/OS elements, such as DFSMSdfp, JES2, and MVS, contain code that supports specific hardware
servers or devices. In some cases, this device-related element support remains in the product even after
the hardware devices pass their announced End of Service date. z/OS may continue to service element
code; however, it will not provide service related to unsupported hardware devices. Software problems
related to these devices will not be accepted for service, and current service activity will cease if a
problem is determined to be associated with out-of-support devices. In such cases, fixes will not be
issued.
Minimum supported hardware
The minimum supported hardware for z/OS releases identified in z/OS announcements can subsequently
change when service for particular servers or devices is withdrawn. Likewise, the levels of other software
products supported on a particular release of z/OS are subject to the service support lifecycle of those
Notices  183

## Page 212

products. Therefore, z/OS and its product publications (for example, panels, samples, messages, and
product documentation) can include references to hardware and software that is no longer supported.
• For information about software support lifecycle, see: IBM Lifecycle Support for z/OS (www.ibm.com/
software/support/systemsz/lifecycle)
• For information about currently-supported IBM hardware, contact your IBM representative.
Programming Interface Information
This publication primarily documents information that is NOT intended to be used as Programming
Interfaces of ISPF.
This publication also documents intended Programming Interfaces that allow the customer to write
programs to obtain the services of ISPF. This information is identified where it occurs, either by an
introductory statement to a chapter or section or by the following marking:
+---------------------Programming Interface information----------------------+
+------------------End of Programming Interface information------------------+
Trademarks
IBM, the IBM logo, and ibm.com are trademarks or registered trademarks of International Business
Machines Corp., registered in many jurisdictions worldwide. Other product and service names might be
trademarks of IBM or other companies. A current list of IBM trademarks is available on the Web at
Copyright and Trademark information (www.ibm.com/legal/copytrade.shtml).
Trademarks
184  z/OS: z/OS ISPF User's Guide Vol I

## Page 213

Index
Special Characters
? (invalid load module directory fields) 89
{ } (one operand required) 27
* (in source listing) 146
/ /* lines, job statement information 113
= (equal sign), used to initiate jump function 58
= repeat command 98
> (greater than)
using to pass a command to a dialog function 65
| (OR symbol) 27
Numerics
1 to 9999 parameter, Scroll field 17
3850 virtual volumes
accessing 78
A
abbreviations for commands and other values
commands 155
field values 155
keywords/operands 155
programming languages 156
scroll amounts 156
AC field, load module library display 89
accessibility
contact IBM 179
ACCOUNT command, TSO 78
action bar 24
ACTIONS system command 34
activities
nesting 29
ALIAS command action, specified by ZCTACT 61, 63
Alias-of field, load module library display 89
alias, as substitute for ISPF 19
aliases for commands 63
aliases for scrolling commands 64
allocating ISPF libraries 141
allocating libraries, defined 75
Allocation List Line Commands
Browse 170
Compress 170
Edit 170
Free 170
Information 171
Member list 170
Query ENQs 171
Test Directory 171
View 170
VTOC Information 171
Allocation List Primary Commands
APF 167
Browse 168
Check 166
Clist 165
Allocation List Primary Commands (continued)
Con 170
Count 166
Custom 168
Duplicates 166
Enq 169
Exclude 164
Find 164
Linklist 167
Load 167
Locate 164
Long 164
LPA 167
Member 165
Mlist 168
Only 164
Parmlib 167
Reset 164
Save 165
Select 167
Short 164
Allocations List, Current Data Set 159
alternate DBCS libraries 143
application commands description 33
ARRAY, browsing storage command 171
ARRAYP, browsing storage command 172
Assembler H (option 4.1) allocation data sets
SYSIN 157
SYSLIB 157
SYSLIN 158
SYSPRINT 157
SYSPUNCH 158
SYSTERM 158
SYSUT1 158
assistive technologies 179
asterisk (*) parameter, SELECT command 94
attention field
command entry 65, 70
selection 70
attention key (PA1) 71
ATTN keyword, defining attention fields 70
Attributes field, load module library display 89
automatic completion
data set and member names 72
AUTOTYPE
automatic completion
data set name 72
member name 72
cursor position sensitivity 73
enabling applications to use 73
function key for 72
restrictions 73
B
B (browse) line command
Data Set List utility (option 3.4) 98
Index  185

## Page 214

B (browse) line command (continued)
Library utility (option 3.1) 98
Move/Copy utility (option 3.3) 98
BACKUP temporary data sets 113
BACKWARD
scrolling command 34, 64
system command 34
Batch (option 5)
output listings 112
overview 7
blank options 18
block line command, defined 29
BOTTOM scrolling command 64
BOTTOM system command 34
Browse Mode
browsing DBCS data as EBCDIC data 106
mixed mode 106
overview 7
BROWSE, browsing storage command 173
BUFNO parameter 3
bypassing display using jump function 58
bypassing menus 19
C
CANCEL, browsing storage command 173
CHAIN, browsing storage command 173
Changed field, member list display 87
Changing Member List Field Attributes 97
Changing the Default Sort Order for Member Lists 97
character conversion for APL and TEXT keyboards 153
CLIST
allocating library 142
restrictions 71
CMDE system command 35
CNTL temporary data sets 113
COLOR
system command 35
Command (option 6)
overview 7
command abbreviations 155
command alias 63
command error processing 28
command field
entering commands 33
use with jump function 58
Command field, ISPF panels 17
command line 28
command notation 27
command retrieving 49
command stacking
restriction with HELP command 34
restriction with RETRIEVE command 34
command tables
action commands 61
dynamically specified command actions 66
format of 60
ISPCMDS system command table 60
used for assigning command aliases 63
used for overriding system commands 65
used for passing commands to a dialog function 33, 65
commands
aliases 63
application 33
commands (continued)
entering 17
error processing 28
function 33
how to enter 33
interception by ISPF 33
ISPF notation 27
levels 33
line 7
nesting 29
passing to a dialog function 65
PDF command
using an alias 19
using option parameter 19
primary 7
processing, by ISPF 33
reading syntax diagrams xvi
specifying action dynamically 66
stacking for execution 33
stacking with delimiter 27
system 34
commands, entering
command error processing 28
ISPF command notation 27
commands, member selection list 89
Common User Access (CUA) guidelines 15
concatenation
during editing 80
during language processing 81
of function key-entered value with command entry,
restrict 66
PDSE v2 member generation data sets 81
conditional retrieve 35
CONFIRM command
Data Set List utility (option 3.4) 91
parameters
OFF 91
ON 91
contact
z/OS 179
Created field
member list display 87
CRETRIEV system command 35
CSR (cursor) scrolling amount 60
CUA (Common User Access) guidelines 15
CUAATTR system command 35
Current Data Set Allocations List
line commands 170
primary commands 163
using commands 163
cursor control of scrolling 60
CURSOR parameter, Scroll field 17
Cursor position sensitivity, AUTOTYPE 73
CURSOR system command 35
cursor-select key
processing of selected fields 70
selection of attention field 70
D
D (delete lines)
Data Set List utility (option 3.4) 98
Library utility (option 3.1) 98
data entry panels 18
186  z/OS: z/OS ISPF User's Guide Vol I

## Page 215

DATA parameter
Scroll field 17
DATA scrolling amount 59
Data Set Allocations List, Current 159
Data Set List utility (option 3.4)
data set list listings 150
member list line commands 98
primary commands
CONFIRM 91
source and index listings 145
data set lists
personal 115–117
reference 115
Data Set Password field, library/data set entry panels 105
DATA, browsing storage command 174
DBCS
allocating alternate libraries 143
session messages in English 143
default function key assignments 66
default function key settings
ending ISPF or an ISPF function 9
getting help 9
default mode, defined 16
default operands 27
Default Sort Order for Member Lists, Changing 97
DELETE command, TSO 91
delimiters, used to stack commands 27
Diagnostic Utility, ISRDDN 159
dialog
development of 139
nested 57
recursive entry into 63
Dialog Test (option 7)
overview 7
dialog, defined 1
direct access data sets 3
direct access storage device, using with packed data 106
DISASM, browsing storage command 175
display format 16
double-byte character set (DBCS)
Browse or Edit DBCS data as EBCDIC data 106
Edit and Browse 106
format definition 105
Format Specification utility (option 3.11) 105
formatted data, View, Edit, and Browse 105
hexadecimal format 106
invalid mixed data 106
Mixed Mode field 106
DOWN system command 35, 59
DSLIST system command 36
DTEST system command 36
dual command processing 28
DUMP, browsing storage command 175
dynamic status area 24
E
Edit (option 2)
editing DBCS data as EBCDIC 106
mixed mode 106
overview 7
EDIT command 98
END
system command 36
END command
ISPF 9
ending
a function or a dialog
END system command 36
RETURN system command 49
display, END system command 36
ending ISPF or an ISPF function, Exit option (X) 10
ending member lists 83
entering a command 33
entering a command, defined 25
ENVIRON system command 37
EPDF system command 37
equal sign used to initiate jump function 58
EXHELP system command 38
Exit (option X)
ending ISPF from the ISPF Primary Option Menu 9
overview 8
use of log/list defaults 10
EXIT system command 38
EXPAND system command 39, 60
extended binary coded decimal interchange code (EBCDIC)
Browse or Edit DBCS data as EBCDIC 106
format definition 105
hexadecimal format 106
invalid mixed data 106
Mixed Mode field 106
F
field selection
using cursor position 70
field value abbreviations 155
field1 parameter, SORT command 95
field2 parameter, SORT command 95
file tailoring
allocating 142
output libraries 142
FKA system command 39
Foreground (option 4)
output listings 112
overview 7
format definition 105
Format Name field
Edit Entry Panel 105
effect on Mixed Mode field 105
View Entry Panel 105
Format Selection List panel 105
Format Specification utility (option 3.11)
IBM 5550 terminal support 105
FORMAT, browsing storage command 175
format, panel 16
FORWARD
scrolling command 64
system command 39
FORWARD system command 39
fragments, syntax diagrams xvi
function (F) keys, See function keys
function commands 33
function key
AUTOTYPE 72
function keys
changing content and format (TAILOR) 45, 68
command entry 66
Index  187

## Page 216

function keys (continued)
defining functions 68
PFSHOW system command 45
providing default settings 70
saving definitions 70
used for command entry 33, 66
G
generation data group (GDG) data sets 3
generation data sets 77
Group field, data entry panels 75
H
HALF parameter, Scroll field 17
HALF scrolling amount 59
Hardcopy utility (option 3.6)
source and index listings 145
HELP command
ISPF 9
HELP system command 34, 39
horizontal split-screen mode 29
I
IBM Products (option 9)
overview 7
ID field, member list display 87
index listings
for load libraries 147
for source libraries 146
Init field, member list display 87
interactive application, defined 1
internal character representations
for APL keyboards 153
ISPCMDS system command table 60, 64
ISPDPTRC system command 40
ISPDTLC system command 40
ISPF
command notation 27
command types 26
ending 9
entering commands 25
functions 1
getting help 9
interacting with 4
member name conventions 4
option selection 18
overview 1
primary options, option selection 18
starting 5
supported data types 3
understanding panels 16
user profiles 8
ISPF commands
END 9
HELP 9
taking a screen snapshot 107
ISPF library names 75
ISPF library setup 139
ISPF library, defined 75
ISPF log listings 148
ISPF member statistics 83
ISPF Settings (option 0 ), overview 7
ISPFILE file tailoring output library 142
ISPFTTRC system command 41
ISPFVAR system command 42
ISPFWORK system command 42
ISPLIBD system command 42
ISPLLIB link library 142
ISPMALT alternate message library 143
ISPMLIB message library 139
ISPPALT alternate panel library 143
ISPPLIB panel library 139
ISPPREP system command 42
ISPPROF default application profile pool 139
ISPSALT alternate skeleton library 143
ISPSLIB skeleton library 139
ISPSTART command, session language override 143
ISPTABL table output library 141
ISPTLIB table input library 139, 141
ISRDDN
allocation list primary commands 163
ISRDDN Diagnostic Utility
Browsing storage commands
ARRAY 171
ARRAYP 172
BROWSE 173
CANCEL 173
CHAIN 173
DATA 174
DISASM 175
DUMP 175
FORMAT 175
LIMIT 176
LOAD 176
NARROW 176
RAW 176
REFRESH 177
SETDATA 177
WIDE 177
Browsing storage, loaded modules 171
ISRRLIST system command 42
ISRROUTE system command 43
J
JCL for allocating libraries for MVS 141
job statement information
parameters
/ /* lines 113
jump function 29, 58
K
keyboard
navigation 179
PF keys 179
shortcut keys 179
KEYLIST system command 43
KEYS system command 44
KEYS system command, defining function key functions 68
KEYSHELP system command 44
keyword/operand abbreviations 155
keywords, syntax diagrams xvi
188  z/OS: z/OS ISPF User's Guide Vol I

## Page 217

L
language for ISPF session, specifying 143
lcmd parameter, SELECT command 94
LEFT system command 44, 59
levels of ISPF commands 33
LIB field
load module library display 88
member list display 87
libraries and data sets
data set passwords 105
format definitions 105
list and log data sets 107
member selection lists 81
mixed mode 106
naming ISPF libraries and data sets 75
packed data sets 106
partitioned data set extended (PDSE) 106
sample ISPF session 10
library concatenation
during editing 80
during language processing 81
functions that use concatenation 79
Group field 78
overview 78
PDSE v2 member generation data sets 81
library lists
personal 115
library member
definition of 75
Library utility (option 3.1)
member list line commands 98
Library utility (option 3.1), source and index listings 145
LIMIT, browsing storage command 176
line command fields 28
line commands
defined 7
definition of 90
member selection list 90
Line Commands, Allocation List 170
linking requirement for split-screen mode 142
list and log data sets
foreground and batch output listings
list-id parameter 112
prefix parameter 112
userid parameter 112
job statement information 113
list data set 107
log data set 107
other temporary data sets 112
prefix parameter 107
printing list/log data sets 111
taking a screen snapshot 107
temporary names 107
userid parameter 107
list data set
list-id 93
prefix 93
processing 108
LIST system command 44
LIST temporary data sets 113
list-id parameter
Foreground and Batch output listings 112
SAVE command 93
List, Current Data Set Allocations 159
LIST, system command
conditions for using 109
description 108
using 108
listing formats
data set list listings 151
ISPF log listings 148
member list listings 150
source and index listings 145
load module library
invalid directory fields (?) 89
member statistics 88
LOAD, browsing storage command 176
LOCATE command
member selection list 93
parameters
string 93
log data set
processing 108
LOG system command
conditions for using 109
description 44, 108
Log/List (option 0)
changing defaults 8
effect on Exit (X) option 10
logical screens (split-screen mode) 29
Long Message field, ISPF panels 17
M
managed data set
defined 106
MAX parameter, Scroll field 17
MAX scrolling amount 59
Member field, data entry panels 76
Member List Field Attributes, Changing 97
member list listings
formats 150
load libraries 149
source libraries 148
member list lists 148
Member List Positioning 101
member list primary commands
SRCHFOR 96
Member Lists, Refreshing 97
member name conventions 4
member selection list
defined 81
displaying member lists 82
ending member lists 83
fields
load module library 88
source library 86
ISPF member statistics 83
line commands
B (browse) 98
C (copy) 98
D (delete) 98
E (edit) 98
G (reset statistics) 98
I (display data set information) 98
J (submit) 98
M (move) 98
Index  189

## Page 218

member selection list (continued)
line commands (continued)
P (print) 98
R (rename) 98
S (select) 94, 98
T (invoke TSO command) 98
V (view) 98
list data set 93
load module library member statistics 88
primary commands
LOCATE 93
RESET 93
SAVE 93
SELECT 94
SORT 95
table of differences 81
updating a member list 100
Member selection list commands 89
Member Selection List Primary Commands
MLC 97
MLS 97
Refresh 97
menu 17
Menu pull-down 20
messages
as means of communication 4
conditions for display 28
displayed by HELP command 9
English for DBCS session 143
failure to enter required value 18
inconsistent values 18
Long Message field 17
Short Message field 17
mixed data
assumed 106
format definition 105
invalid, examples of 106
Mixed Mode field 106
non-mixed mode 106
unformatted 106
Mixed Mode field
Edit Entry Panel 106
View Entry Panel 106
MLC Command 97
MLS Command 97
Mod field, member list display 87
MOUNT authority 78
move members 80
Move/Copy utility (option 3.3)
member list line commands 98
Move/Copy utility (option 3.3), source and index listings 145
MSGID system command 44
multicultural support 27, 95
multiple, defined for line commands 81
multivolume data sets 3
N
Name field
load module library display 88
member list display 86
naming ISPF libraries and data sets
ISPF library names 75
Other Partitioned or Sequential Data Set Names 76
naming ISPF libraries and data sets (continued)
volume serials 78
NARROW, browsing storage command 176
national language for ISPF session 143
National Language Support, See multicultural support
native mode, defined 16
navigation
keyboard 179
nested commands 29
nested dialogs 58
NLS, See multicultural support
non-ISPF displays, using REFRESH 57
NOP action, specified using ZCTACT 61
NOP system command 45
notation conventions 27
NRETRIEV command 132, 133
NRETRIEV system command 45
NX parameter, Attributes field 89
O
OFF parameter
CONFIRM command 91
OL parameter, Attributes field 89
ON parameter
CONFIRM command 91
operand notation
optional([ ]) 27
lowercase 27
one required({ }) 27
OR symbol (|) 27
stacked 27
underscored defaults 27
uppercase 27
operating system, passing commands 33
Option field, ISPF panels 17
option number 24
option selection
0 - ISPF Settings 7
1 - View 7
10 - SCLM 7
2 - Edit 7
3 - Utilities 7
4 - Foreground 7
5 - Batch 7
6 - Command 7
7 - Dialog Test 7
9 - IBM Products 7
X - Exit 8
optional ISPF libraries 141
Other Partitioned or Sequential Data Set Names 76
other temporary data sets 112
OUTLINE keyword
fields affected by 46
on PRINT system command 46
OUTLIST temporary data sets 113
Outlist utility (option 3.8), source and index listings 145
OV parameter, Attributes field 89
overriding, system commands 65
overview of ISPF
commands, entering 25
interacting with ISPF 4
ISPF primary options 6
starting ISPF 5
190  z/OS: z/OS ISPF User's Guide Vol I

## Page 219

overview of ISPF (continued)
understanding ISPF panels 16
user profiles 8
P
P (print) line command
Data Set List utility (option 3.4) 98
Library utility (option 3.1) 98
PA keys
definition 71
PA1 71
PA2 71
packed data
format defined 106
requirements for using 106
PAGE parameter, Scroll field 17
PAGE scrolling amount 59
panel format 16
panel ID field, ISPF panels 16
panel types
data entry panels 18
menus 17
overview 17
PANELID system command 45
panels
as means of communication 4
basic types 17
default mode 16
defined 16
menus 17
native mode 16
panel format 16
retaining previous values 18
what they display 5
parameter
parenthesis added to 63
parenthesis, added to command parameter 63
partitioned data set extended (PDSE) 106
partitioning the display, screen-split screen mode 29
passing commands
to a dialog function 65
to the operating system 33
PASSTHRU action
specified using ZCTACT 61, 65
pattern parameter, SELECT command 94
pattern, defined 76
PDSE 106
personal data set list 116, 117
Personal Data Set List panel 121–123
personal data set lists 115
Personal Data Set Lists panel 126, 127
personal library list 118, 119
Personal Library List panel
action bar 124
fields 125
personal library lists 115
Personal Library Lists panel 128–130
Personal list modes 131
personal lists
library 118, 119
personal data set list 116, 117
reference data set list 117
reference library list 119
PFSHOW system command
changing content and format (TAILOR) 45, 68
ZPFCTL system variable 69
ZPFFMT system variable 68
ZPFSET system variable 69
ZPRIKEYS system variable 69
positioning, member list 101
prefix parameter
Foreground and Batch output listings 112
list and log data sets 107
primary commands
defined 7, 89
member selection list 89
Primary Commands, Allocation List 163
PRINT system command 46
PRINT-HI system command 47
PRINTDS
editing the command 111
printing the Log and List data sets 111
PRINTG system command 46
PRINTL system command 47
PRINTLHI system command 47
profiles, user 8
program access (PA) keys 71
program library, allocating 142
program linking requirement for split-screen mode 142
programming language abbreviations 156
Project field, data entry panels 75
PSCOLOR system command 47
Q
Query ENQs 171
R
R (rename) line command
Data Set List utility (option 3.4) 98
Library utility (option 3.1) 98
RACF (Resource Access Control Facility) 105
RAW, browsing storage command 176
RCHANGE system command 48
recursive entry into dialog functions 63
redisplaying contents of a screen using PA key 71
REFACTD system command 48
REFACTL system command 48
REFADDD system command 48
REFADDL system command 48
reference data set list 117
reference data set lists 115
reference library list 119
reference library lists 115
referral lists 115
REFLISTD system command 48
REFLISTL system command 48
REFOPEND system command 48
REFOPENL system command 49
Refresh Command 97
REFRESH, browsing storage command 177
Refreshing Member Lists 97
Rename field on member list display 87, 88
repeatable items, syntax diagrams xvi
repeating, commands (RETRIEVE) 49
Index  191

## Page 220

required ISPF libraries 139
RESET command
member selection list 93
RESET key
description 71
reshow key (PA2) 71
RESIZE system command 49
Resource Access Control Facility (RACF) 105
RETF system command 49, 57
RETP system command 49, 57
RETRIEVE system command 49, 55
RETURN system command 49, 57
REXX - allocating library 142
RF parameter, Attributes field 89
RFIND system command 50
RIGHT system command 44, 50, 59
RN parameter, Attributes field 89
RU parameter, Attributes field 89
rules for AUTOTYPE 73
S
S (select) line command
member selection list 94
Move/Copy utility (option 3.3)
98
sample ISPF session 10
SAREA system command 50
SAVE command
Data Set List utility (option 3.4) 148
member selection list 93
parameters
asterisk (*) 94
lcmd 94
list-id 93
pattern 94
saving function key definitions 70
SCLM (option 10)
overview 7
SCRNAME system command 50
scroll amount abbreviations 156
Scroll field, ISPF panels 17
scrolling
command aliases 64
commands to control 59
scroll amount 59
tutorial panels 59
SELECT action
command 62
specified by ZCTACT 61
SELECT command
member selection list 94
SETDATA, browsing storage command 177
Settings (option 0)
changing default delimiter 27
overriding mode switching 16
SETTINGS system command 50
SETVERB action
specified by ZCTACT 61
Shared personal lists 131
shift-in characters
DBCS character strings 106
formatted data, Edit, View and Browse 105
invalid mixed data, examples of 106
shift-in characters (continued)
treatment in non-mixed mode 106
shift-out characters
DBCS character strings 106
formatted data Edit and Browse 105
invalid mixed data, examples of 106
treatment in non-mixed mode 106
Short Message field, ISPF panels 17
shortcut keys 179
single selection, defined for line commands 81
Size field
load module library display 88
member list display 87
skipping panel display using jump function 58
snapshot of a screen 107
SORT command
member selection list 95
parameters
field1 95
field2 95
Sort Order for Member Lists, Changing the Default 97
source listings 145
source segments 81
specifying action dynamically 66
SPFTEMP temporary data sets 113
SPLIT command 29
SPLIT system command 51
split-screen mode
entering 29
logical screens 29
maximum number 29
partitioning display screen 29
program linking requirement 142
RETRIEVE function 55
terminating 29
VSAM restrictions 32
split-screen mode, maximum number 29
SPLITV system command 31, 51
SRCHFOR command, Member List utility 96
stack - (RETRIEVE command) 55
stacked operands 27
stacking commands
for execution 33
restrictions 34
start column, defined 146
START system command 51
statistics
ISPF members 83
load module libraries 88
storage, browsing commands
ARRAY 171
ARRAYP 172
BROWSE 173
CANCEL 173
CHAIN 173
DATA 174
DISASM 175
DUMP 175
FORMAT 175
LIMIT 176
LOAD 176
NARROW 176
RAW 176
REFRESH 177
192  z/OS: z/OS ISPF User's Guide Vol I

## Page 221

storage, browsing commands (continued)
SETDATA 177
WIDE 177
string parameter
LOCATE command 93
summary of changes xxv
SuperC listing title lines
index listings 146
source listings 145
suspending an activity 29
SWAP system command 51
SWAPBAR options
customizing 52
SWAPBAR system command 52
syntax diagrams, how to read xvi
SYSIN data set
Assembler H 157
VS COBOL II 157
SYSLIB data set
Assembler H 157
VS COBOL II 157
SYSLIN data set
Assembler H 158
VS COBOL II 158
SYSNAME
system command 53
SYSNAME system command 53
SYSPRINT data set
Assembler H 157
VS COBOL II 157
SYSPUNCH data set
Assembler H 158
VS COBOL II 158
system commands
description 33
overriding by use of command tables 65
System commands
function key defaults 34
list of 34
System Commands
ACTIONS 34
BACKWARD 34
BOTTOM 34
CMDE 35
COLOR 35
CRETRIEV 35
CUAATTR 35
CURSOR 35
DOWN 35
DSLIST 36
DTEST 36
ENVIRON 37
EPDF 37
EXHELP 38
EXIT 38
EXPAND 39, 60
FKA 39
FORWARD 39
HELP 39
ISPDPTRC 40
ISPDTLC 40
ISPFTTRC 41
ISPFVAR 42
ISPFWORK 42
System Commands (continued)
ISPLIBD 42
ISPPREP 42
ISRRLIST 42
ISRROUTE 43
KEYLIST 43
KEYS 44
KEYSHELP 44
LEFT 44
LIST 44
LOG 44
MSGID 44
NOP 45
NRETRIEV 45
PANELID 45
PFSHOW 45
PRINT 46
PRINT-HI 47
PRINTG 46
PRINTL 47
PRINTLHI 47
PSCOLOR 47
RCHANGE 48
REFACTD 48
REFACTL 48
REFADDD 48
REFADDL 48
REFLISTD 48
REFLISTL 48
REFOPEND 48
REFOPENL 49
RESIZE 49
RETF 49
RETP 49
RETRIEVE 49
RETURN 49
RFIND 50
RIGHT 44, 50
SAREA 50
SCRNAME 50
SETTINGS 50
SPLIT 51
SPLITV 51
START 51
SWAP 51
SWAPBAR 52
SYSNAME 53
TOP 53
TSO 53
TSOCMD 53
TUTOR 54
UDLIST 54
UP 54
USERID 54
WINDOW 54
ZCLRSFLD 55
ZEXPAND 39
ZKEYS 55
system variables, saving function key definitions 70
SYSTERM data set
Assembler H 158
VS COBOL II 158
SYSUT1 data set
Assembler H 158
Index  193

## Page 222

SYSUT1 data set (continued)
VS COBOL II 158
SYSUT2 to SYSUT7 data sets, VS COBOL II 158
T
table libraries, allocating 141
TAILOR operand on PFSHOW command 45, 68
tape data sets, ISPF support for 3
temporary data sets 112
terminating
a function or dialog
END system command 36
RETURN system command 49
display by using END system command 36
TSO commands of CLIST by using PA Key 71
Title field, ISPF panels 16
TOP
scrolling command 64
system command 53
TOP system command 53
trademarks 184
transmission codes 153
TS parameter, Attributes field, load module library 89
TSO
commands
ACCOUNT 78
DELETE 91
TSO system command 53
TSOCMD system command 53
TTR field, load module library display 89
TUTOR system command 54
Tutorial (Option T), finding ISPF information 5
Type field, data entry panels 75
U
UDLIST
INIX directory list command 54
system command 54
underscored operands 27
understanding ISPF panels
overview 16
panel format 16
panel types 17
UP
scroll command 54, 59
system command 54, 59
updating a member list 100
user interface
ISPF 179
TSO/E 179
user profiles 8
USERID
system command 54
userid parameter
Foreground and Batch output listings 112
list and log data sets 107
USERID system command 54
using AUTOTYPE 72
Utilities (option 3)
overview 7
Utilities pull-down menu 20
V
V (view) line command
Data Set List utility (option 3.4) 98
Library utility (option 3.1) 98
variable block spanned (VBS) data sets 3
variables, syntax diagrams xvi
vertical split-screen mode 31
VIO, allocating temporary data sets 112
Volume Serial field
data entry panels 78
VS COBOL II compiler (option 4.2)
allocation data sets
SYSIN 157
SYSLIB 157
SYSLIN 158
SYSPRINT 157
SYSPUNCH 158
SYSTERM 158
SYSUT1 158
SYSUT2 to SYSUT7 158
VSAM data sets, ISPF support for 3
VSAM restrictions, split-screen mode 32
VTOC Information 171
VV.MM field, member list display 87
W
WIDE, browsing storage command 177
WINDOW system command 54
WORK temporary data sets 113
Workplace option description 8
writing a list to a list data set
member list 93
writing dialogs 139
Z
ZCLRSFLD system command 55
ZCTACT
command table field 63
system variable 60
ZCTDESC system variable 60
ZCTTRUNC system variable 60
ZCTVERB
command table field (alias value) 63
system variable 60
ZEXPAND system command 39
ZKEYS system command 55
ZLANG system variable 143
ZPARM system variable
parenthesis added to 63
ZPF01, ZPF02, ... ZPF24 system variables 70
ZPFCTL system variable, on PFSHOW 69
ZPFFMT system variable on PFSHOW 68
ZPFSET system variable, on PFSHOW 69
ZPRIKEYS system variable on PFSHOW system command 69
ZSCBR system variable 60
ZSCED system variable 60
ZSCML system variable 60
ZSCROLLA system variable 60
ZSCROLLD system variable 60
ZSCROLLN system variable 60
194  z/OS: z/OS ISPF User's Guide Vol I

## Page 223

ZSCROLNL system variable 60
ZTEMPF system variable 142
ZTEMPN system variable 142
ZVERB system variable 66
Index  195

## Page 224

196  z/OS: z/OS ISPF User's Guide Vol I

## Page 225



## Page 226

IBM®
Product Number: 5655-ZOS
SC19-3627-60
