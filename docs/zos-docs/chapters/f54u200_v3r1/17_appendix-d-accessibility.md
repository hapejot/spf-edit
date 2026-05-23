# Appendix D. Accessibility

Source file: f54u200_v3r1.md
Start page: 545
Page span: 545-574

## Page 545

Appendix D. Accessibility
Accessible publications for this product are offered through IBM Documentation for z/OS (www.ibm.com/
docs/en/zos).
If you experience difficulty with the accessibility of any z/OS documentation see How to Send Feedback to
IBM to leave documentation feedback.
© Copyright IBM Corp. 1980, 2024 507

## Page 546

508  z/OS: z/OS ISPF User's Guide Vol II

## Page 547

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
© Copyright IBM Corp. 1980, 2024 509

## Page 548

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
510  z/OS: z/OS ISPF User's Guide Vol II

## Page 549

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
Notices  511

## Page 550

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
512  z/OS: z/OS ISPF User's Guide Vol II

## Page 551

Index
Special Characters
: (colon), using with function key definition 44
.* SuperC process statement 448
.IM control word 313
* SuperC process statement 448
/ (data set name character), Data Set List utility (option 3.4)
141
&xxxxx action, ACTION column 177
% field, Data Set List utility 140
%Used field, VTOC Summary Information panel 147
%USED keyword, SORT command 162
= (repeat last command) line command, Data Set List utility
(option 3.4) 155
= (repeat previous) line command
z/OS UNIX Directory List Utility (option 3.17)
277
= workplace command 422
> (greater than)
using with function key definition 44
Numerics
3850 virtual volumes
allocating a data set to 106
5550 terminal 178
A
A (ANSI) parameter, Printer Carriage Control field 173
A (ASA printer characters) record format 107
AA (auditor auditing) line command
z/OS UNIX Directory List Utility (option 3.17)
252
accessibility
contact IBM 507
ACCOUNT command, TSO 106
ACCOUNT, TSO 85
ACTBAR workplace command 421
Action column 176
Activate/Create Profile DS Name field, SuperCE Profile
Manager panel 201
AD/Cycle C/370 compiler
batch processing 349
batch processing (option 5.15) 349
foreground processing 337
foreground processing (option 4.15) 337
Additional IBM Program Development Products panel 399
Additional Input Libraries field, Foreground Print Options
panel 316
AFTER parameter
Breakpoint Primary Option Menu 397
When field 393
AIX (*AIX*) entry in Volume field 129
Alias action, Action column 176
alias entries, processing 167
Alias members, searching 472
Alias members, SuperC compare 472
ALL parameter
FIND command 74
Function field 389
Variable field 390
allocate action prompt workplace command 421
Allocate command 155
ALLOCATE command, TSO 104
allocate SMS workplace command 422
allocate workplace command 421
allow mixed-case in action field option 176
Always save table in originating data set 221, 231
American National Standard Institute 173
APF authorized status 266, 273
APPEND command
Data Set List utility (option 3.4) 156
parameters 156
Application ID field, Command Table Utility panel 175
ASCII 75
aspect ratio parameter for PRINTG 32
Assembler concatenation sequence 316
Assembler H (option 4.1) allocation data sets
overview 318
Assembler H (option 5.1) 343
Assembler Options field
Foreground Assembler H (option 4.1) 319
assistive technologies 507
Attrib display view, Initial View field 131
audit information, z/OS UNIX 266
B
B (batch) parameter, Mode field
Search-For Utility panel 205
SuperC Utility panel 185
B (blocked) record format, Record Format field 107
B (browse) line command
Data Set List utility (option 3.4) 150
z/OS UNIX Directory List Utility (option 3.17)
253
Batch (option 5)
AD/Cycle C/370 compiler (option 5.16) 349
Assembler H (option 5.1) 343
C/370 compiler (option 5.13) 349
JCL generation for Assembler and Linkage Editor 343
JCL generation for compilers 342
Linkage Editor (option 5.7) 347
Member Parts List (option 5.12) 348
overview 2
password protection, lack of 340
PL/I optimizing compiler (option 5.5) 346
processing sequence 339
VS COBOL II compiler (option 5.2) 344
VS COBOL II interactive debug (option 5.10) 348
VS FORTRAN compiler (option 5.3) 345
VS Pascal compiler (option 5.6) 346
Batch options
Index  513

## Page 552

Batch options (continued)
ISPDTLC 349
OS/390 C/C++
349
BATCH parameter, Print Mode field 168
BEFORE parameter
Breakpoint Primary Option Menu 397
When field 393
Binary copy option 254, 257, 258, 260
Blank action, Action column 177
Blank lines after headings (in export report) 233
blank parameter
Active field
breakpoints 393
function traces 389
variable traces 391
DSORG field 140
Operation field 391
Pool field 391
Printer Carriage Control field 173
Select Pack option for “To” data set field 122
Sequence Numbers field 188
Specify additional search strings field 204
When field 393
BLKS parameter, Space Units field 106
BLKSZ
field 141
keyword 162
SORT command 162
Block size field, Allocate New Data Set panel 108
breakpoints
defined 391
Breakpoints (option 7.8)
Cancel (option C) 396
CANCEL primary command 394
control display 395
D (delete) line command 394
END primary command 394, 395
finding a breakpoint 396
Go (option G) 396
I (insert) line command 394
input errors 395
LOCATE primary command 394
primary command 394
QUAL primary command 394
qualification 395
qualification parameter values 394
R (repeat) line command 394
specifying breakpoints 392
syntax checking 395
Browse
providing Unicode support 68
BROWSE command
BROWSE 69
COLUMNS 70
DISPLAY 70
displaying after processing 69
FIND 73
HEX 77
LOCATE 79
overview 69
parameter member 69
RESET 80
SUBMIT 69, 80
Browse Command - Entry Panel 69
Browse field, SCRIPT/VS Processor panel 325
Browse Mode
Browse data display 67
current position in data set 79
ending Browse 68
overview 2, 69
selecting Browse 67
valid logical record length 66
valid record format 65
BROWSE Output field, SuperCE Utility panel 196
browse output mode 185, 198, 205, 213
Browse table command 219
browse table panel 220
Browse workplace command 421
Byte parameter, Compare Type field 194
C
C (catalog data set) line command, Data Set List Utility
(option 3.4) 152
C (continuation) keyword, Search-For utility (option 3.14)
207
C or CO (copy out) line command
z/OS UNIX Directory List Utility (option 3.17)
253
C/370 compiler
batch processing (option 5.13) 349
foreground processing (option 4.13) 337
C/C++ for OS/390 (option 4.19) 338
CALL statement format 334
CANCEL command
Breakpoints (option 7.8) 394
Command Table utility (option 3.9) 177
Dialog Test (option 7) 360
table utility 223
Tables (option 7.4) 378, 380
Traces (option 7.7) 390
Variables (option 7.3) 370
CANCEL, Traces (option 7.7) 391
catalog workplace command 421
CATALOG, Sort command field 162
CC parameter, DISPLAY command 71
CCSID
Unicode support in Browse 68
CCSID parameter, DISPLAY command 71
char parameter, DISPLAY command 70
character string 75
character string, LOCATE 360
CHARS parameter, FIND command 73
CHG operation 391
CHGNV SuperC process statement 447
CHNG parameter, Listing Type field 187, 195
CI (copy in) line command
z/OS UNIX Directory List Utility (option 3.17)
257
Class field, Outlist Utility panel 173
CLIST
error exits 354
CLIST TERMIN command procedure statements 354
closing a data set 68
CMD field 364
CMPBOFS SuperC process statement 449
CMPCOLM SuperC process statement 450
514  z/OS: z/OS ISPF User's Guide Vol II

## Page 553

CMPCOLMN SuperC process statement 450
CMPCOLMO SuperC process statement 450
CMPLINE SuperC process statement 451
CMPSECT SuperC process statement 452
COBOL Interactive debug (option 4.10A)
overview 330
print output data sets 332
processing sequence 331
symbolic debug data sets 332
COBOL parameter, Sequence Numbers field 188
col-1 and col-2 parameters, FIND command 73
COLHEAD SuperC process statement 454
COLOR
change defaults 52
Color used to display table key values 220, 231
COLS parameter, DISPLAY command 71
column limitations 77
column-identification line
defined 70
removing with RESET command 80
COLUMNS command, Browse 70
Command (option 6)
interrupting a TSO command, CLIST, or REXX exec 354
overview 2
restrictions for entering a CLIST or REXX exec 354
Session Manager, using 354
terminal settings 354
TSO commands, CLISTs, and REXX EXECs, entering 353
command action prompt workplace command 421
Command field, display view panels 138
command line placement, specifying 28
command prefix (&), displaying commands after processing
69
command stacking
specifying delimiter 34
command table line commands
edit 178
view 178
Command Table utility (option 3.9)
line commands
D (delete) 177
I (insert) 178
R (repeat) 178
primary commands
CANCEL 177
END 177
command time limit
z/OS UNIX directory list
283
commands
Dialog Test 358
displaying after processing 69
entering long commands 351
ISPF command shell 417
line 2
primary 2
reading syntax diagrams xxvi
TSO command 416
Workplace 420
Compare Type field, SuperCE Utility panel 194
Compare types, SuperC
Byte 194
File 194
Line 194
Compare types, SuperC (continued)
Word 194
compress request exit 91, 153
compress workplace command 422
Cond parameter, Browse Output field 196
CONFIRM command
Data Set List utility (option 3.4) 156
parameters
OFF 156
ON 156
Confirm Delete field, Data Set List Utility panel 131
confirm member delete 131
contact
z/OS 507
conversion table
copying z/OS UNIX files 254, 257, 258,
260
COPY command 153
copy member 99
COPY statement 313
Copy workplace command 421
Created field
Data Set List utility 141
CREATED keyword, SORT command 162
CSECT compare, SuperC 473
CUA
defaults
function key settings 422
CUA attributes workplace command 421
CUA attributes… 53
CUA panel element default values 53
CUA panel element values (changing) 53
CUAATTR option, Change Utility 53
CYLS parameter, Space Units field 106
D
D (DBCS) parameter, Field Type field 180
D (delete lines)
Command Table utility (option 3.9) 177
Data Set List utility (option 3.4) 151
Dialog Test (option 7) 361
Format Specification utility (option 3.11) 182
SCRIPT/VS (option 4.9) 328
D (delete) line command
Breakpoints (option 7.8) 394
Tables (option 7.4) 378, 380
Variables (option 7.3) 370
z/OS UNIX Directory List Utility (option 3.17)
261
DA data set organization, DSORG field 140
DASD volume 85
Data class field, Allocate New Data Set panel 105
DATA parameter
HEX command 78
data set allocation exit 104
data set control blocks 147
data set list primary commands
CONFIRM 156
EXCLUDE 157
FIND and RFIND 157
LOCATE 158
MEMBER 158
REFRESH 159
Index  515

## Page 554

data set list primary commands (continued)
RESET 159
SAVE 159
SHOWCMD 160
SORT 161
SRCHFOR 162
VA,VS,VT,VV 163
Data Set List utility (option 3.4)
data set list exit 128
display views and panels
Attrib 131
Space 131
Total 131
Volume 130
line commands
= (repeat last command) 155
B (browse data set) 150
C (catalog data set) 152
D (delete data set) 151
E (edit data set) 149
F (free unused space) 153
I (data set information) 152
M (display member list) 150
P (print data set) 152
R (rename data set) 152
S (information (short)) 152
U (uncatalog data set) 152
V (view data set) 149
X (print index listing) 153
Z (compress data set) 152
line commands that do not support managed data sets
C (catalog data set) 152
U (uncatalog data set) 152
line commands that support managed data sets
I (data set information) 152
R (rename data set) 152
S (information (short)) 152
X (print index listing) 153
list data set 159
options
blank - display data set list 133
P - Print data set list 146
PV - Print VTOC information 147
V - Display VTOC information 146
primary commands
CONFIRM 156
FIND 157
LOCATE 158
RFIND 157
SAVE 159
SHOWCMD 160
SORT 161
data set list utility line commands
Allocate 155
copy 153
exclude data set 153
move 153
RefAdd 153
reset 153
Search-For 154
Search-ForE 154
SuperC 154
SuperCE 154
unexclude data set 153
data set list utility line commands (continued)
unexclude first data set 154
unexclude last data set 154
Data Set List Utility Primary Commands 155
Data Set Name field
Foreground Member Parts List panel 335
SuperC Submit Batch Jobs panel 190
Data set name type field, Allocate New Data Set panel 108
data set organization 140
Data Set Password 85
Data Set utility (option 3.2)
options
A - Allocate new data set 102
blank - data set information 114
C - Catalog data set 110
D - Delete entire data set 112
R - Rename entire data set 110
S - Data set information (short) 114
U - Uncatalog data set 112
options that do not support managed data sets
C - Catalog data set 110
U - Uncatalog data set 112
options that support managed data sets
blank - Data set information 114
R - Rename entire data set 111
S - Data set information (short) 114
processing with RACF
D - Delete 113
R - Rename 111
R - rename entire data set (GDG) 111
DAU data set organization, DSORG field 140
DBCS considerations, SuperC 184
DBCS data, copying 254, 257, 258, 260
DDDD format, Expiration Date field 109
Debug command data set field 348
Debug Options field, Foreground FORTRAN interactive debug
(option 4.11) 333
default keylist for Help Panels 42
delete action prompt workplace command 421
DELETE command, TSO 131, 156
Delete workplace command 421
DELETE, Traces (option 7.7) 390, 391
Delta parameter, Listing Type field 187, 195
Description column 177
Description field, SCRIPT/VS Formatting Style panel 328
Device Data information, z/OS UNIX 266
Device field, Data Set List utility 140
DEVICE keyword, SORT command 162
device name parameter for PRINTG 31
DFSMSdfp
striped data set type 94, 98
Dialog Services (option 7.6), calling a dialog service 385
dialog test
accessing and updating variables 356
available ISPF facilities 357
breakpoint cancel (option C) 355
environment 355
ISPF log generation 357
message displayed for severe errors 358
NEWPOOL option 357
severe error found at breakpoint 358
Dialog Test
DBCS value format 374
log entry for current value of variable 385
516  z/OS: z/OS ISPF User's Guide Vol II

## Page 555

Dialog Test (continued)
reasons errors occur 358
variable usage 357
Dialog Test (option 7)
Breakpoints (option 7.8) 391
commands 358
D (delete lines) 361
Dialog Services (option 7.6) 385
environment 396
exceptions to restoration when using Go (option G) 396
Exit (option 7.X) 398
Functions (option 7.1) 362
I (insert lines) 361
Log (option 7.5) 383
overview 2
Panels (option 7.2) 365
R (repeat lines) 361
severe error handling 358
Traces (option 7.7) 388
Tutorial (option 7.T) 397
variable usage 357
Variables (option 7.3) 367
Directory blocks field, Allocate New Data Set panel 107
Directory List Options panel, z/OS UNIX 282
DISALLOW_WILDCARDS_IN_HLQ option 128
Display Area field 230
DISPLAY command
Browse 70
parameters 70, 71
Display Directory List option, z/OS UNIX 242
display generations 100
display member information 100
Display mode for export data set 232
Display Style Options field, SCRIPT/VS Processor panel 325
Document Composition Facility (DCF) 324
double-byte character set (DBCS)
column specification in dialog test 374
defining data type 180
Dialog Test variables format 372
finding a 1-byte hexadecimal string 77
Format Specification utility (option 3.11) 178
ISPF editor 86
DPLINE SuperC process statement 455
DPLINEC SuperC process statement 455
DS1-DS4 fields
Extended Search-For - Concatenation Data Set Entry
panel 212
DSLIST exit 128
DSLIST workplace command 421
Dsname Level field, Data Set List Utility panel 128
Dsorg field, Data Set List utility 140
DSORG keyword, SORT command 162
DTEST system command 356
E
E (EBCDIC) parameter, Field Type field 180
E (edit data set) line command, Data Set List utility (option
3.4) 149
edit
double-byte character set support 86
Edit (option 2)
editing a data set 83
overview 2
Edit (option 2) (continued)
valid logical record length 83
valid record format 83
edit action prompt workplace command 421
EDIT command, Browse 72
EDIT primary command
z/OS UNIX directory list
278
Edit table command 218, 219
edit table panel 220
Edit workplace command 421
edit, command table line command 178
editing a member 99, 288, 289, 291–294, 297–299, 301,
303, 305, 306
END
Traces (option 7.7) 390, 391
END command
Breakpoints (option 7.8) 394, 395
Command Table utility (option 3.9) 177
Dialog Test (option 7) 360
Variables (option 7.3) 370
Enter Session Manager Mode field 354
Environ settings 55
ENVIRON system command 55
environment, Dialog Test 355
errors
message displayed during dialog test processing 358
reasons for occurring during Dialog Test 358
trigger statement 315
Eview output mode 185, 198
EXCLUDE command 157
Exclude Data Set command 153
Execution Parms field
COBOL interactive debug 331
Exit (option 7.X) ending Dialog Test 398
Exit (option X)
overview 3
EXPAND primary command 221
Expanding Packed Data 312
expansion trigger, defined 312
Expiration data field, Allocate New Data Set panel 109
Expires field, Data Set List utility 141
EXPIRES keyword, SORT command 162
explosion chain, defined 336
EXPORT command 224
export table to data set
data set attributes 233
export data set 225
Export Layout panel 224
export report options 232, 235
EXTENDED
data set name type 94, 98
Extended Attributes field, Allocate New Data Set panel 108
Extended Attributes information, z/OS UNIX 266
extended binary coded decimal interchange code (EBCDIC)
defining data type 180
using numbered data 181
Extended Search-For Compare utility
applications 433
options
B - submit batch search-for 212
Blank - search-for strings 212
E - edit Search-For statements data set 214
P - select Search-For process options 214
Index  517

## Page 556

Extended Search-For Compare utility (continued)
overview 209
process options 434
SuperC program description 432
utility differences 431
Extended Search-For listing
printing in batch mode 213
extended utilities 431
extension variables 221
F
F (foreground) parameter, Mode field 185
F (foreground) parameter, Mode field, Search-For Utility
panel 205
F (free unused space) line command, Data Set List utility
(option 3.4) 153
F (function) pool, Pool field 390
F record format, Record Format field 107
false match, correcting in SuperC 471
family printer type parameter for PRINTG 31
FEXPORT command 225
field
Active 389, 391, 393
AFTER 397
Application 397
BEFORE 397
Breakpoint 397
by row number 373
BY ROW NUMBER 373
current row 373
Current row count 382, 383
Current row pointer 381, 383
Current status 397
Cursor field 366
Cursor position 366
Date created 381, 383
DBCS column specification 374
Dialog services to be traced 389
Display in window 367
Function 389, 391, 393, 397
ID 365
KEYS 381
LANG 364
Last date modified 382, 383
Last modified by 382, 383
Last service return code 383
Last table service 383
Last time modified 382, 383
Message id 366
Message pop-up field 367
MODE 364, 365
Modified row count 382, 383
NAMES 381
NEWAPPL 365
NEWPOOL 365
Number of keys 381
Number of names 381
Number of rows 381
Open option 382
Open tables 373
Operation 391
Original row count 382, 383
P (pool) 369
field (continued)
PANEL 364
Panel name 366
PASSLIB 365
PGM 365
Pool 390
Qual 393
Qualification parameter values 394
Return code 397
row identification 373
Row number 376
Service 393
Service Name 397
specifying a DBCS value 374
Status for this screen 381, 382
Table available 381
table name 373
Table name 376
Table on disk 383
Time created 382, 383
Update count 382, 383
using the Value field 374
variable 374
Variable 369, 377
Virtual storage size 382, 383
When 393
Field Heading field, table utility 224
Field Length field, Format Definition panel 180
Field Number field, Format Definition panel 180
Field Type field, Format Definition panel 180
field1 parameter, SORT command 161
field2 parameter, SORT command 161
FILE command, table utility 225
File parameter, Compare Type field 194
FILTER primary command
z/OS UNIX directory list
278
FIND command
ASCII strings 75
Browse
1-byte hexadecimal strings 77
character strings, use of 75
column limitations 77
conditions for character string matches 75
displaying column numbers searched 74
omitting string delimiters 74
picture strings, use of 75
rules for using col-1 and col-2 77
specifying FIND strings 73
starting point, direction, and extent of search 74
string not found actions 74
text strings, use of 75
using RFIND 77
using string delimiters 74
search operands 73, 74
string matching operands 73
table utility 226
USASCII strings 75
UTF8 strings 75
FIND primary command
z/OS UNIX directory list
279
FIRST parameter, FIND command 74
Foreground (option 4)
518  z/OS: z/OS ISPF User's Guide Vol II

## Page 557

Foreground (option 4) (continued)
AD/Cycle C/370 compiler (option 4.15) 337
AD/Cycle C/370 compiler (option 4.16) 337
Assembler H (option 4.1) 318
C/370 compiler (option 4.13) 337
COBOL interactive debug (option 4.10A) 330
defined 309
expanding packed data 312
FORTRAN interactive debug (option 4.11) 333
input data sets 316
Linkage Editor (option 4.7) 322
list data sets 316
Members Parts List (option 4.12) 333
object data sets 317
overview 2
password protection 317
PL/I optimizing compiler (option 4.5) 321
processing sequence 309
REXX/370 compiler (option 4.14) 337
SCRIPT/VS (option 4.9) 324
using the TSO/E Information Center Facility 318
VS COBOL II compiler (option 4.2) 319
VS COBOL II interactive debug (option 4.10) 330
VS FORTRAN compiler (option 4.3) 320
VS Pascal compiler (option 4.6) 322
Foreground Environment Feature 324
Foreground Options
C/C++ for OS/390
338
ISPDTLC 337
Format Name field
Format Specification panel 179
Format Specification utility (option 3.11)
line commands
D (deleting a format) 182
R (renaming a format) 183
S (selecting a format) 183
U (updating a format) 183
options
A - add a new format 180
C - copy formats 181
D - delete a format 182
L or Blank - display format list 182
U - update a format 182
primary commands
LOCATE 182
SORT 183
FORTRAN interactive debug (option 4.11) 333
fragments, syntax diagrams xxvi
Free DSCBS field, VTOC Information Display 147
Free Extents field, VTOC Information Display 147
From Format field, Format Specification panel 179
From Table field, Format Specification panel 147, 179
FS (file system) line command
z/OS UNIX Directory List Utility (option 3.17)
262
FSCR parameter, MODE field 364, 365
full information workplace command 421
function key defaults, workplace 422, 423
function key settings, workplace 422
function keys
defining functions 43
specifying labels 43
specifying number 50
Functions (option 7.1), testing a dialog function 362
G
Generate headings (in export report) 233
generic unit address 106
GET operation 391
global color change utility 53
global color change workplace command 421
graphics interface mode, effect on Session Manager 310
graphics, settings parameters for PRINTG 31
Groups For Primary Members field
Foreground Member Parts List panel 335
Member Parts List Display 336
H
Hardcopy utility (option 3.6)
additional batch printing information 170
using the TSO/E Information Center Facility 170
HELP command
displaying column numbers searched 74
HEX command
Browse 77
operands
DATA 78
OFF 77
ON 77
VERT 78
hexadecimal data, Dialog Test variables format 371
HFS
data set name type 94, 98
high-level qualifier
wildcards in 128
I
I (data set information) line command, Data Set List utility
(option 3.4) 152
I (implicit) variable, P (pool) field 369
I (information) line command
z/OS UNIX Directory List Utility (option 3.17)
264
I (insert lines), Dialog Test (option 7) 361
I (insert) line command
Breakpoints (option 7.8) 394
Tables (option 7.4) 378, 380
Variables (option 7.3) 370
I (insert) line command, Command Table utility (option 3.9)
178
IBM Products (option 9)
overview 2
IEBCOPY utility 91, 153
IEBGENER utility 170
If Partitioned, Replace Like-Named Members field 122
If Sequential, “To” Data Set Disposition field 122
implicit variable 369
import table from data set 234
Include Additional Qualifiers (in data set list) 131
INCLUDE statement 313
inconsistent attributes, defined 110
information action prompt workplace command 421
Information Center Facility
Index  519

## Page 558

Information Center Facility (continued)
foreground processing sequence 311
Hardcopy utility 167
Outlist utility 173
Initial Macro field, Edit Entry panel 66
Initial View field
defined 130
views
Attrib 131
sequence of views 130
Space 131
Total 131
Volume 130
input data sets 316
INSERT command 226
INSERT, Traces (option 7.7) 390, 391
installation exits
compress request 91, 153
data set allocation 104
data set list (DSLIST) 128
print utility 167, 173, 311
Intensity used to display table key values 220, 232
ISPCMDS 174
ISPDTLC (option 4.18) 337
ISPDTLC compile, batch 349
ISPF
default keylist 41, 42
restrictions
multivolume data sets 105, 128
tape 105, 148
ISPF Command field 351
ISPF command shell 417
ISPF command shell workplace command 421
ISPF command table workplace command 421
ISPF Primary Option Menu - status area 5
ISPF referral list
Workplace (Option 11) 405
ISPF restrictions
multivolume data sets 105, 128
tape 105, 148
ISPF Settings (option 0 ), overview 2
ISPF Table Utility 215
ISPTLIB 174
ISRFORM table 181
ISRLEMX
description 313
member parts list 333
return codes 315
ISRSCAN
description 313
member parts list 342
return codes 315
J
JCL generation
Assembler and Linkage Editor 343
compilers 342
job control language, generating and submitting print jobs
169
job statement information
preventing JES line counting 327
job stream, submitting a 69, 80
JobID field, Outlist Utility panel 173
Jobname field, Outlist Utility panel 172
jump function, from leader dots 30
K
K (key) variable 375, 377, 379
KB parameter, Space Units field 106
keyboard
navigation 507
PF keys 507
shortcut keys 507
keylist
defaults for Help Panels 42
help panel name 48
modification choices 46
keylist utility 41, 42
keylist workplace command 421
keys
PA1 354
Reset 354
keywords, syntax diagrams xxvi
L
L (List) line command
z/OS UNIX Directory List Utility (option 3.17)
268
label fields
defining 44
using PFSHOW command 44
label parameter, LOCATE command 79
Language field, Foreground Member Parts List panel 334
LARGE
data set name type 94, 98
large files, partitioning and processing in SuperC 471
Largest field, VTOC Information Display 147
LAST parameter, FIND command 74
LC command 158
LC command, Data Set List utility (option 3.4) 158
leader dots, and jump function 30
LEFT primary command
z/OS UNIX directory list
279
Library utility (option 3.1)
options
B - browse member 99
Blank - display member list 91
C - compress data set 91
D - delete member 99
E - edit member 99, 288, 289, 291–294, 297–299,
301, 303, 305, 306
I - data set information 92
L - print entire data set 92
P - print member 100
R - rename member 100
S - data set information (short) 96
V - view member 101
X - print index listing 92
options that support managed data sets
I - data set information 95
S - data set information (short) 97
X - print index listing 92
LINE - Dialog Test (option 7) 361
520  z/OS: z/OS ISPF User's Guide Vol II

## Page 559

Line Cmd field, SCRIPT/VS Formatting Style panel 328
line command prefix characters
z/OS UNIX directory list 249
line commands
Command Table utility (option 3.9) 177
Data Set List utility (option 3.4) 141, 147
defined 2
Format Specification utility (option 3.11) 183
SCRIPT/VS (option 4.9) 328
Line parameter, Compare Type field 194
LINE parameter, DISPLAY command 71
LINE parameter, MODE field 364, 365
line-number operand, LOCATE command 79
Linkage Editor (option 4.7)
concatenation sequence 323
LEL control statements 323
overview 322
Linkage Editor (option 5.7) 347
Linkage Editor concatenation sequence 323
Linkage Editor language 323
Linkage Editor Options field, Foreground Linkage Editor
(option 4.7) 323
list action prompt workplace command 421
list data set
list-id 159
prefix 159
specifying characteristics 38
specifying defaults 35
List ID field
Batch Selection Panel 340
Foreground Print Options panel 316
Foreground Selection panel 311
list view workplace command 421
list-id parameter
SAVE command 159
Listing DS Name field
Extended Search-For Utility panel 211
SuperC Utility-Old Data Set Name panel 187
Listing Dsn field, SuperCE Utility panel 195
Listing DSNAME field, Search-For Utility panel 205
listing file examples
NARROW listing 485
Listing Type field
SuperC Utility-Old Data Set Name panel 187
SuperCE Utility panel 195
LNCT SuperC process statement 458
Local parameter, Print Mode field 168
Local Printer ID field, Hardcopy Utility panel 168
LOCATE
Dialog Test (option 7) 360
Traces (option 7.7) 390, 391
LOCATE character string 360
LOCATE command
Breakpoints (option 7.8) 394
Browse 79
Data Set List utility (option 3.4) 158
Format Specification utility (option 3.11) 182
parameters
label 79
line-number 79
lparm 158
name 182
Variables (option 7.3) 370
LOCATE primary command
LOCATE primary command (continued)
z/OS UNIX directory list
279
Locate table command 218, 220
locate workplace command 421
Log (option 7.5)
displaying the ISPF log 383
logged trace output 384
reasons log is not available 384
log data set
specifying defaults 35
Log/List (option 0)
changing defaults 3
long message
line placement 29
specifying 29
long message areas, displaying in a pop-up 29
LONG parameter, Listing Type field 187, 195
lowercase, defining parameters in 176
lparm parameter, LOCATE command 158
LPSFV SuperC process statement 459
Lrecl field, Data Set List utility 141
LRECL keyword, SORT command 162
LSTCOLM SuperC process statement 458
M
M (display member list) line command, Data Set List utility
(option 3.4) 150
M (machine) parameter, Printer Carriage Control field 173
M (mixed ) parameter, Field Type field 180
M record format, Record Format field 107
MA (modify ACL) line command
z/OS UNIX Directory List Utility (option 3.17)
268
managed data set
displaying information for 95
Management class field, Allocate New Data Set panel 104
matching input files, SuperC 470
Maximum rows searched to determine column width 221,
231
MB parameter, Space Units field 106
member
copy from z/OS UNIX file
253
copy to z/OS UNIX file 257
MEMBER command
description 158
member expansion return codes 315
Member list workplace command 421
member not found 336
member parameter
BROWSE command 69
SYSTERM DD statement 343
Member Parts List (option 4.12)
CALL statement format 334
member not found 336
options
1 - browse/print member parts list 335
2 - write member parts data set 336
overview 333
Procedure Division 334
record format 336
Member Parts List (option 5.12) 348
Index  521

## Page 560

membered PDS, defined 186
Message field, display view panels 138
MESSAGE keyword, SORT command 162
MF (modify format) line command
z/OS UNIX Directory List Utility (option 3.17)
270
MG (modify group) line command
z/OS UNIX Directory List Utility (option 3.17)
271
mixed case, defining parameters in 176
mixed data
defining data type 180
E (edit data set) line command 149
using numbered data 181
V (view data set) line command 149
Mixed Mode 86
MM (modify mode) line command
z/OS UNIX Directory List Utility (option 3.17)
271
MO (modify owner) line command
z/OS UNIX Directory List Utility (option 3.17)
272
Mod parameter, “To” Data Set Disposition field 122
Mode field
Search-For Utility panel 205
SuperC Utility panel 185
mode fields information, z/OS UNIX 265
Mode, Mixed 86
MOUNT authority 106
MOVE command 153
move member 100
Move workplace command 421
move/copy 418
Move/Copy utility (option 3.3)
alias entries 125
options
C and CP - copying data sets 121
M and MP - moving data sets 123
with load modules 124
Multiple Search Strings panel 206
multiple search strings, entering 206
multivol indicator 139
multivolume data sets 105, 128
MVOL, Sort command field 162
MX (modify extended attributes) line command
z/OS UNIX Directory List Utility (option 3.17)
273
N
N (new) line command
z/OS UNIX Directory List Utility (option 3.17)
273
Name field
display view panels 138
NAME keyword
LOCATE command 182
SORT command 162, 183
name, Writer 168
navigation
keyboard 507
NCHGT SuperC process statement 447
new data set, defined for SuperC
description 184
New DS Name field, SuperCE Utility panel 193
New Output Class field, Outlist Utility panel 173
New UserID field, Reset ISPF Statistics panel 165
New Version Number field, Reset ISPF Statistics panel 165
NEWAPPL field 365
NEWPOOL field 365
NEXCLUDE SuperC process statement 457
NEXT parameter, FIND command 74
NFOCUS SuperC process statement 457
NO parameter
Active field
breakpoints 393
function traces 389
variable traces 391
Browse Output field 196
Confirm Delete Request field, deselect 131
Replace like-named PDS members field 122
Select pack option For “To” data set field 122
Specify additional search strings field, deselect 204
Table available field 381
Table on disk field 383
NOACTBAR workplace command 421
NOCC parameter, DISPLAY command 71
Nolist parameter, Listing Type field 187, 195
NOP action, ACTION column 177
NORDW parameter, DISPLAY command 71
NOSEQ parameter, Sequence Numbers field 188
NOT OPEN parameter 381, 382
NOWRITE parameter 382
NTITLE SuperC process statement 462
Number of Copies field, Hardcopy Utility panel 171
NY2AGE SuperC process statement 463
NY2C SuperC process statement 464
NY2D SuperC process statement 464
NY2P SuperC process statement 464
NY2Z SuperC process statement 464
O
object data sets 317
Object field
Foreground VS FORTRAN compiler (option 4.3) 321
OBROWSE command
browsing z/OS UNIX regular file
253
OCHGT SuperC process statement 447
OEDIT command
editing ASCII file 261
editing UTF8 file 261
editing z/OS UNIX regular file 261,
278
OEXCLUDE SuperC process statement 457
OFF parameter
CONFIRM command 156
HEX command 77
SHOWCMD command 160
OFOCUS SuperC process statement 457
OGET command
copying z/OS UNIX file to a data set 254
copying z/OS UNIX file to PDS member
255
Old DS Name field, SuperCE Utility panel 193
Old parameter, “To” Data Set Disposition file 122
ON parameter
522  z/OS: z/OS ISPF User's Guide Vol II

## Page 561

ON parameter (continued)
CONFIRM command 156
HEX command 77
SHOWCMD command 160
Open option field
NOWRITE parameter 382
SHR NOWRITE parameter 382
SHR WRITE parameter 382
WRITE parameter 382
OPEN parameter 381, 382
Open table in SHARE mode 231
Open workplace command 421
OPT field 364
option
Cancel (option C) 396
Go (option G) 396
option selection
0 - ISPF Settings 2
1 - View 2
10 - SCLM 2
2 - Edit 2
3 - Utilities 2
4 - Foreground 2
5 - Batch 2
6 - Command 2
7 - Dialog Test 2
9 - IBM Products 2
X - Exit 3
Options pull-down menu, switching UIDs 307
options, table utility 231
OPUT command
copying member to z/OS UNIX file
259
Order field, table utility 224, 228
OS/390 C/C++ compile, batch 349
Other field
Batch Assembler H (option 5.1) 344
Batch Linkage Editor (option 5.7) 347
Batch VS COBOL II compiler (option 5.2) 345
Batch VS FORTRAN compiler (option 5.3) 345
Foreground PL/I optimizing compiler (option 4.5) 322
Foreground VS COBOL II compiler (option 4.2) 320
Foreground VS FORTRAN compiler (option 4.3) 321
Foreground VS Pascal compiler (option 4.6) 322
OTITLE SuperC process statement 462
OUTDD DD field, SuperC Submit Batch Jobs panel 191
Outlist utility (option 3.8)
options
Blank - display job output 174
D - delete job output from SYSOUT hold queue 173
L - list job names/IDs via the TSO STATUS command
173
P - print job output and delete from SYSOUT hold
queue 173
R - requeue job output to a new output class 174
overview of ISPF
ISPF primary options 1
OVSUM parameter, Listing Type field 187, 195
owner information, z/OS UNIX 265, 268
OY2AGE SuperC process statement 463
OY2C SuperC process statement 464
OY2D SuperC process statement 464
OY2P SuperC process statement 464
OY2Z SuperC process statement 464
P
P (print) line command
Data Set List utility (option 3.4) 152
P (profile) pool, Pool field 391
P (profile) variable, P (pool) field 369
PA1 key 354
packed data
defined 312
packing data, Foreground (option 4) 309
pad characters for panel input fields 34
panel
Breakpoint Primary Option 395
Panels (option 7.2), testing dialog panels 365
parameter
parenthesis added to 176
parameter string 395
parenthesis, added to command parameter 176
PARM field 365
partitioned data set
copy from files in z/OS UNIX directory
255
copy to files in z/OS UNIX directory 258
partitioning and processing in SuperC, large files 471
PASSLIB field 365
Passthru action
Action column 176
Password field
Foreground Data Entry panel 317
Foreground Selection panel 311
password protection 317
Password, Data Set 85
PATH (*PATH*) entry in Volume field 129
path name substitution character
z/OS UNIX directory list 283
patterns
Dsname Level field 128
PDS Member List field
defined 194
parameters 210
permissions, display in octal format 283
personal data set lists command 421
personal library lists command 421
Pgm Control status 266, 273
physical record length, defined 108
picture strings, use of 75
PL/I optimizing compiler
batch (option 5.5) 346
foreground (option 4.5) 321
PO data set organization, DSORG field 140
point and shoot workplace command 421
POU data set organization, Dsorg field 140
prefix parameter
data set list to ISPF list data set 159
FIND command 73
SYSTERM DD statement 343
PREFIX parameter, FIND command 73
PREV parameter, FIND command 74
primary commands
Browse 69
CANCEL 360
Command Table utility (option 3.9) 177
Data Set List utility (option 3.4) 155
defined 2
Index  523

## Page 562

primary commands (continued)
END 360
Format Specification utility (option 3.11) 182
LOCATE 360
QUAL 360
RESUME 361
SU 307
Primary Commands, Data Set List Utility 155
primary library, defined 336
primary member, defined 336
Primary quantity field, Allocate New Data Set panel 107
print action prompt workplace command 421
print data set command 421
print data set index workplace command 422
Print data set list workplace command 421
Print Directory List option, z/OS UNIX 251
Print ID field, COBOL Interactive Debug panel 331, 332
Print Mode field
parameters
BATCH parameter 168
LOCAL parameter 168
SCRIPT/VS Processor panel 327
print output data sets 332
print utility exit
foreground processing sequence 311
Hardcopy utility 167
Outlist utility 173
Print VTOC information workplace command 421
Print workplace command 421
Printer Carriage Control field, Outlist Utility panel 173
Printer Format field, Hardcopy Utility panel 171
Printer Location field, Hardcopy Utility panel 171
PRINTG
parameters 31
specifying printer output 31
printing
output 167
Procedure Division 334
Process Options field
Extended Search-For Utility panel 211
SuperCE Utility panel 195
Profile DSN field, SuperC Utility panel 185
Profile Name field, Edit Entry Panel 67
profile, defined for SuperCE 185
Prog ID field, COBOL Interactive Debug panel 331
PS data set organization, DSORG field 140
PS-E 140
PS-L 140
PSU data set organization, Dsorg field 140
Purge Data Set field, Confirm Purge panel 113
PUT operation 391
Q
QUAL
command - Breakpoints (option 7.8) 394
Dialog Test (option 7) 360
qualification parameter values
Qualify action bar pull-down
Breakpoints... choice 394
Qualifications... choice 394
R
R - Rename entire data set (Data set utility options) 110
R (rename) line command
Data Set List utility (option 3.4) 152
Format Specification utility (option 3.11) 183
z/OS UNIX Directory List Utility (option 3.17)
275
R (repeat lines), Dialog Test (option 7) 361
R (repeat) line command
Breakpoints (option 7.8) 394
Command Table utility 178
Tables (option 7.4) 378, 380
RA (add to personal data set list) line command
z/OS UNIX Directory List Utility (option 3.17)
275
RACF, processing with
D - delete 113
R - rename 111
RDW parameter, DISPLAY command 71
Recfm field, Data Set List utility 141
RECFM keyword, SORT command 162
Record format field, Allocate New Data Set panel 107
Record length field, Allocate New Data Set panel 108
RefAdd command 153
Referred field, Data Set List utility 141
REFERRED keyword, SORT command 162
REFRESH command 159
REFRESH primary command
z/OS UNIX directory list
280
rename 419
rename action prompt workplace command 422
rename data set with expiration date 112
rename processing with RACF 111
rename workplace command 421
Rename workplace command 422
renaming data sets
with expiration date 112
REPEAT, Traces (option 7.7) 390, 391
repeatable items, syntax diagrams xxvi
reset action prompt workplace command 421
RESET command
Browse 80
Reset ISPF Statistics utility (option 3.5)
options
D - delete ISPF statistics 166
R - reset (create/update) ISPF statistics 166
results of resetting statistics 166
valid logical record lengths 163
RESET key
restriction 44
reset member statistics 100
Reset Mod Level option, Reset ISPF Statistics panel 165
RESET parameter, DISPLAY command 71
RESET primary command
z/OS UNIX directory list
280
Reset Seq Numbers option, Reset ISPF Statistics panel 165
reset statistics 415
restrictions on member expansion and member parts lists
all programming languages 314
assembler 314
COBOL 314
524  z/OS: z/OS ISPF User's Guide Vol II

## Page 563

restrictions on member expansion and member parts lists (continued)
FORTRAN 314
Pascal 314
PL/I 314
SCRIPT/VS 315
RESUME - Dialog Test (option 7) 361
return codes
Batch JCL generation 315, 342
member expansion 315
REVREF SuperC process statement 459
REXX error exits 354
REXX/370 compiler
batch processing 349
foreground processing 337
RFIND command
Data Set List utility (option 3.4) 157
table utility 227
using 77
RFIND primary command
z/OS UNIX directory list
279
RIGHT primary command
z/OS UNIX directory list
280
Rnn parameter 393
rules
for substituting slash (/) for data set name 142
for using the col-1 and col-2 operands 77
service call image for function trace entries 385
using the add row option 379
using the Dialog Test Breakpoints option 395
using the Modify Row option 377
Run method 277
S
S (information (short)) line command, Data Set List utility
(option 3.4) 152
S (invoke default) line command
z/OS UNIX Directory List Utility (option 3.17)
275
S (select) line command
Format Specification utility (option 3.11) 183
SCRIPT/VS (option 4.9) 328
S (shared) pool, Pool field 390
S (shared) variable, P (pool) field 369
S record format, Record Format field 107
SAVE command
Data Set List utility (option 3.4) 159
parameters
list-id 159
name 183
table utility 227
SAVE primary command
z/OS UNIX directory list
281
SCLM (option 10)
overview 2, 401
SCLM Primary Option Menu 401
SCLM Setting 123
SCLM Settings 165
screen format, specifying 30
SCRIPT/VS (option 4.9)
changing style options 328
SCRIPT/VS (option 4.9) (continued)
line commands
D (delete) 328
S (select) 328
selecting a formatting style 327
using the TSO/E Information Center Facility 329
scrollable fields, in Table Utility 223
Scrollable fields, in Table Utility 221
Search DS Name field, Extended Search-For Utility panel 210
search process options
ALLMEMS 437
ANYC 437
APNDLST 437
ASCII 438
COBOL 438
CPnnnnn 438
DPACMT 439
DPADCMT 439
DPBLKCL 439
DPCBCMT 439
DPCPCMT 439
DPFTCMT 439
DPMACMT 439
DPPLCMT 439
DPPSCMT 439
FINDALL 440
IDPFX 440
LMCSFC 440
LMTO 440, 488
LNFMTO 441
LONGLN 441
LPSF 441, 492
LTO 441, 492
MIXED 441
NOPRTCC 441
NOSEQ 441
NOSUMS 442
SEQ 442
XREF 444, 489
Search String field, Search-For Utility panel 204
search strings, multiple 206
Search-For command 154
Search-For utility (option 3.14)
applications 433
invoking directly 473
selecting members from a member list 207
SuperC program description 432
utility differences 431
Search-ForE command 154
SearchFor extended workplace command 422
SearchFor workplace command 422
Secondary quantity field, Allocate New Data Set panel 107
Select action, Action column 176
SELECT command
Format Specification utility (option 3.11) 183
SELECT service 355
SELECT SuperC process statement 461
select workplace command 422
SEQ parameter, Sequence Numbers field 188
Sequence field, table utility 228
Sequence Numbers field, SuperC Utility - Old Data Set Name
panel 188
sequential data set
Index  525

## Page 564

sequential data set (continued)
copy from z/OS UNIX file
253
copy to z/OS UNIX file 257
Serial, Volume 85
Session Manager
Command (option 6) 354
Foreground (option 4) 310
Member Parts List (option 4.12) 335
SCRIPT/VS 326
using 354
Set options for IMPORT format report 233
SETGID bit 266, 272
Settings option 27
Settings workplace command 422
SETUID bit 266, 272
SETVERB action
Action column 177
shared library status 266, 273
shared profile settings 56
shift-in characters
in Dialog Test variables 372
using the Value field 374
shift-out characters
in Dialog Test variables 372
using the Value field 374
short information command 422
shortcut keys 507
SHOWCMD command
Data Set List utility (option 3.4) 160
parameters
OFF 160
ON 160
SHR NOWRITE parameter 382
SHR WRITE parameter 382
SHRPROF system command 56
Size field
VTOC Information Display 147
SLIST SuperC process statement 462
smart action 426
Software Configuration and Library Manager (SCLM) 401
SORT command
Data Set List utility (option 3.4) 161
Format Specification utility (option 3.11) 183
keywords
%USED 162
BLKSZ 162
CREATED 162
DEVICE 162
DSORG 162
EXPIRES 162
LRECL 162
MESSAGE 162
NAME 162
RECFM 162
REFERRED 162
TRACKS 162
VOLUME 162
XT 162
parameters
field1 161
field2 161
NAME 183
TIME 183
SORT command (continued)
table utility 227
SORT primary command
z/OS UNIX directory list
281
Source Data Online field, Batch Selection panel 340
Source Data Packed field
Batch Selection panel 340
Foreground Print Options panel 313
Foreground Selection panel 310
Source field, COBOL Interactive Debug panel 331
Source Type field, Foreground Selection panel 311
Space display view, Initial View field 131
Space units field, Allocate New Data Set panel 106
Specify additional strings field, Search-For Utility panel 204
Specify Pack option for “To” Data Set field 122
split-screen mode
command line placement 29
SRCHFOR command, Data Set List utility (option 3.4) 162
SRCHFOR SuperC process statement 459
SRCHFORC SuperC process statement 459
standard Search-For utility, entering multiple search strings
206
standard utilities 431
Start Column field 180
Statements Dsn field
Extended Search-For Utility panel 211
SuperCE Utility panel 195
statistics
creating or resetting 166
STATS command 228
status area - ISPF Primary Option Menu 5
status area - Primary Option Menu
calendar view 10
Defining 15
function key view 9
No view selected 15
point-and-shoot view 14
session view 6
user view 13
sticky bit 266, 272
Storage class field, Allocate New Data Set panel 104
string parameter
FIND command 73
STRUCT command 229
Style field
SCRIPT/VS Formatting Style panel 328
SCRIPT/VS Processor panel 325
style, defined for SCRIPT/VS 325
SU primary command
z/OS UNIX directory list
282
SUBMIT command, BROWSE 69, 80
submit member 100
Submit workplace command 421
submitting a job stream 69, 80
suffix
add to file name 259
strip from file name 256
SUFFIX parameter, FIND command 75
summary of changes xxxv
SuperC
CSECT compare 473
526  z/OS: z/OS ISPF User's Guide Vol II

## Page 565

SuperC Activate/Create Profile
options
A - activate 202
C - create 202
D - default 202
SuperC alias members 472
SuperC CLIST interface 202
SuperC command 154
SuperC compare types
Byte 194
File 194
Line 194
Word 194
SuperC Compare utility
correcting false matches 471
filtering priority for input lines 469
find match example 470
invoking directly 473
partitioning and processing large files 471
SuperC comparison listing
| (change bar) 480
change bar (|) 480
column title line 478
D (deleted line) 479
DC (delete compose) 479
DEL= (delete TYPE code) 480
delete compose (DC) 479
delete moved (DM) 480
delete replace (DR) 479
deleted line (I) 479
DM (delete moved) 480
DMR= (delete-move-reformat TYPE code) 480
DMV= (delete-move TYPE code) 480
DR (delete replace) 479
I (inserted line) 479
IC (insert compose) 479
id column 478
ID column 478
ID column (listing file) 478
IM (insert moved) 479
IMR= (insert-move-reformat TYPE code) 480
IMV= (insert-move TYPE code) 480
INS= (insert TYPE code) 480
insert compose (IC) 479
insert moved (IM) 479
inserted line (I) 479
LEN column 478
LEN column (listing file) 478
MAT= (match TYPE code) 480
match compose (MC) 479
MC (match compose) 479
member summary section 477
N-LN# 478
N-LN# (listing file) 478
NARROW listing example 485
O-LN# 478
O-LN# (listing file) 478
overall summary section 477
page headings
compare date 478
compare time 478
new file ID 478
old file ID 478
page number 478
SuperC comparison listing (continued)
page headings (continued)
printer control character 478
program date 478, 487
program ID 478, 487
program version 478, 487
reformat new (RN) 479
reformat old (RO) 479
RFM= (reformat TYPE code) 480
RN (reformat new) 479
RO (reformat old) 479
RPL= (replace TYPE code) 480
scale 478
section title line 478
source line column 478
SOURCE LINE column 478
SOURCE LINE column (listing file) 478
TYPE column 478
TYPE column (listing file) 478
SuperC comparison process statements
.* 448
* 448
CHNGV 447
CMPBOFS 449
CMPCOLM 450
CMPCOLMN 450
CMPCOLMO 450
CMPLINE 451
CMPSECT 452
COLHEAD 454
DPLINE 455
DPLINEC 455
LNCT 458
LSTCOLM 458
NCHGT 447
NEXCLUDE 457
NFOCUS 457
NTITLE 462, 478
NY2AGE 463
NY2C 464
NY2D 464
NY2P 464
NY2Z 464
OCHGT 447
OEXCLUDE 457
OFOCUS 457
OTITLE 462, 478
OY2AGE 463
OY2C 464
OY2D 464
OY2P 464
OY2Z 464
REVREF 459
SELECT 461
SLIST 462
WORKSIZE 463
Y2PAST 466
SuperC comparison type
LINE 478
SuperC extended workplace command 422
SuperC listing type
DELTA 478
UPDCMS8 495
UPDCNTL 496
Index  527

## Page 566

SuperC listing type (continued)
UPDLDEL 499
UPDMVS8 500
UPDPDEL 501
UPDREV 493
UPDREV2 494
UPDSEQ0 501
UPDSUMO 502
SuperC listing types
CHNG 187, 195
Delta 187, 195
Long 187, 195
Nolist 187, 195
OVSUM 187, 195
SuperC listings 477
SuperC load module compares 472
SuperC process options
ALLMEMS 437
ANYC 437
APNDLST 437
APNDUPD 437
ASCII 438
CKPACKL 438
CNPML 438
COBOL 438
COVSUM 438
CPnnnnn 438
DLMDUP 438
DLREFM 438
DPACMT 439
DPADCMT 439
DPBLKCL 439
DPCBCMT 439
DPCPCMT 439
DPFTCMT 439
DPMACMT 439
DPPLCMT 439
DPPSCMT 439
FINDALL 440
FMSTOP 440
FMVLNS 440, 479, 480
GWCBL 440, 480
LOCS 441
LONGLN 441
NARROW 441, 485
NOPRTCC 441, 478, 487
NOSEQ 441
NOSUMS 442
REFMOVR 442
SDUPM 442
SEQ 442
SYSIN 442
UPDCMS8 442
UPDCNTL 442
UPDLDEL 442
UPDMVS8 443
UPDPDEL 443
UPDREV 443
UPDREV2 443
UPDSEQ0 444
UPDSUMO 444
VTITLE 444
WIDE 444
XWDCMP 444
SuperC process options (continued)
Y2DTONLY 444
SuperC process statements
+ (DPLINE operand) 456
+ (SRCHFOR operand) 461
+start_column (DPLINE operand) 456
+start_column (SRCHFOR operand) 461
B (COLHEAD keyword) 454
BTM (CMPBOFS keyword) 449
BTM (CMPLINE keyword) 451
BTM (CMPSECT keyword) 453
C (COLHEAD keyword) 454
D (COLHEAD keyword) 454
end_col (CMPSECT operand) 453
end_column (CMPCOLM operand) 450
end_position (NEXCLUDE operand) 457
end_position (NFOCUS operand) 458
end_position (OEXCLUDE operand) 457
end_position (OFOCUS operand) 458
fixed (Y2PAST operand) 467
hex_offset (CMPBOFS operand) 449
last_start_column (CMPLINE operand) 452
last_start_column (DPLINE operand) 456
last_start_column (LSTCOLM operand) 459
last_start_column (NCHGT operand) 448
last_start_column (OCHGT operand) 448
last_start_column (SRCHFOR operand) 460
line number (CMPLINE operand) 451
NBTM 449
NBTM (CMPBOFS keyword) 449
NBTM (CMPLINE keyword) 451
NBTM (CMPSECT keyword) 453
new_member (SELECT operand) 462
NTOP 449
NTOP (CMPBOFS keyword) 449
NTOP (CMPLINE keyword) 451
NTOP (CMPSECT keyword) 453
number (CHNGV operand) 447
number (LNCT operand) 458
number (LPSFV operand) 459
OBTM 449
OBTM (CMPBOFS keyword) 449
OBTM (CMPLINE keyword) 451
OBTM (CMPSECT keyword) 453
OFF (SLIST operand) 462
old_member (SELECT operand) 462
ON (SLIST operand) 462
OTOP 449
OTOP (CMPBOFS keyword) 449
OTOP (CMPLINE keyword) 451
OTOP (CMPSECT keyword) 453
output_string (NCHGT operand) 448
output_string (OCHGT operand) 448
P (COLHEAD keyword) 454
P (SRCHFOR operand) 460
RCVAL=number (REVREF operand) 459
REFID=name (REVREF operand) 459
S (SRCHFOR operand) 460
search_member (SELECT operand) 462
search_string (CMPLINE operand) 452
search_string (CMPSECT operand) 453
search_string (NCHGT operand) 448
search_string (OCHGT operand) 448
section ID (CMPSECT operand) 453
528  z/OS: z/OS ISPF User's Guide Vol II

## Page 567

SuperC process statements (continued)
sliding (Y2PAST operand) 467
start_column (CMPCOLM operand) 450
start_column (CMPLINE operand) 452
start_column (CMPSECT operand) 453
start_column (DPLINE operand) 456
start_column (LSTCOLM operand) 458
start_column (NCHGT operand) 448
start_column (OCHGT operand) 448
start_column (SRCHFOR operand) 460
start_position (NEXCLUDE operand) 457
start_position (NFOCUS operand) 458
start_position (OEXCLUDE operand) 457
start_position (OFOCUS operand) 458
string (SRCHFOR operand) 460
title_name (NTITLE operand) 463
title_name (OTITLE operand) 463
TOP (CMPBOFS keyword) 449
TOP (CMPLINE keyword) 451
TOP (CMPSECT keyword) 453
W (SRCHFOR operand) 460
Z (COLHEAD keyword) 454
SuperC program, requirements for 432
SuperC programming interface 202
SuperC reasons for differing comparison results 467
SuperC return codes
descriptions 468
empty input file error 468
error 468
error return codes 468
file attributes (inconsistent) 468
inconsistent file attributes 468
insufficient storage error 469
invalid sequence numbers 468
listing file error (disk full) 468
listing file error (read only) 468
listing file I/O error 468
no common members/files to compare 468
no data to compare error 468
normal completion 468
normal completion return codes 468
storage (insufficient) error 469
update file error (read only) 468
update file I/O error 468
warning 468
warning return codes 468
SuperC search listing
page headings
compare date 487
compare time 487
page number 487
printer control character 487
SuperC search process statements
.* 448
* 448
CMPCOLM 450
CMPLINE 451
CMPSECT 452
COLHEAD 454
DPLINE 455
DPLINEC 455
LNCT 458
LPSFV 459
LSTCOLM 458
SuperC search process statements (continued)
NCHGT 447
NTITLE 462
SELECT 461
SLIST 462
SRCHFOR 459
SRCHFORC 459
SuperC side-by-side listing 441
SuperC update data sets
UPDCMS8 495
UPDCNTL 496–498
UPDLDEL 499, 500
UPDMVS8 500
UPDPDEL 501
UPDREV 493, 494
UPDREV2 Revision File (2) 494
UPDSEQ0 501, 502
UPDSUMO 502–504
SuperC utility (option 3.12)
applications 433
new data set 184
old data set 186
printing a Search-For listing in batch mode 208
printing a SuperC listing in batch mode 189
process options 434
program description 432
specifying members on a member list 188
submit options
1 - generate output listing in DATA SET NAME 190
2 - generate output listing using completed / /
OUTDD DD 191
blank - generate output listing to SYSOUT CLASS
190
utility differences 431
SuperC utility, requirements for 432
SuperC workplace command 422
SuperC, DBCS considerations 184
SuperCE - Profile Manager panel
option A - Activate 202
option C - Create 202
option D - Defaults 202
SuperCE command 154
SuperCE utility (option 3.13)
applications 433
options
A - activate profiles and defaults 200
E - edit SuperCE statements data set 199
P - select process options 434
P - select SuperCE process options 199
S - Extended Search-For utility 209
process options
overview 434
SuperC program description 432
SuperCE utility 192
utility differences 431
symbolic debug data sets 332
symbolic link information, z/OS UNIX 267
syntax diagrams, how to read xxvi
SYSLIB field
Batch Linkage Editor (option 5.7) 347
Foreground Linkage Editor (option 4.7) 323
SYSLIN field
Batch Linkage Editor (option 5.7) 347
Foreground Linkage Editor (option 4.7) 323
Index  529

## Page 568

SYSOUT Class field
Batch Selection panel 340
Hardcopy Utility panel 168
SuperC - Submit Batch Jobs panel 190
system command table 174
system commands
DTEST 356
SYSTERM DD statement, parameters
member 343
prefix 343
T
T (truncation) column 176
T (variable types) field
K (key) variable
add row 379
delete row 377
display row 375
N (name) variable
add row 379
delete row 377
display row 375
S (save) variable
add row 379
delete row 377
display row 375
T record format, Allocate New Data Set panel 107
table
date created/modified 228
editing/browsing, See ISPF Table Utility
export to data set 224, 235
find string in 226
import from data set 234
insert blank row 226
processing if already open 235
saving changes 221, 227
size (virtual storage required) 229
sorting rows 227
status 229
structure, displaying 229
table input library 174
Table statistics, displaying 228
Table structure, displaying 229
Table Utility (option 3.16)
Browse command 219
CANCEL (table changes) command 223
Edit command 218, 219
EXPORT command 224
export report 232, 235
export to data set 235
extension variables 221
FEXPORT command 225
FILE command 225
find string in table 226
import from data set 234
INSERT command 226
Line command table support 238
options 231
output data set 230
overview 215
panel fields 216
performance 221
SAVE (table changes) command 227
Table Utility (option 3.16) (continued)
saving changes 221
scrollable fields 221, 223
scrolling 220
select table from DD list 219
select table from library 217
sorting rows 227
STATS command 228
table structure, displaying 229
tables already open 235
Table Utility Options panel 231
Tables (option 7.4)
1—display row 374
2—delete row options 375
3—modify row 376
4—add row 378
5—display structure 380
6—display status 381
add row line command 380
CANCEL add row primary command 380
CANCEL modify row primary command 378
D (delete) add row line command 380
D (delete) modify row line command 378
END add row primary command 380
END display row command 375
END modify row primary command 378
I (insert) add row line command 380
I (insert) modify row line command 378
LOCATE add row primary command 380
LOCATE display row command 375
LOCATE display structure command 381
LOCATE modify row primary command 378
R (repeat) add row line command 380
R (repeat) modify row line command 378
tape 105, 128
TERMIN command procedure statements 354
terminal characteristics, specifying 30
terminal data set, defined 344
terminal type, specifying 31
terminals
Batch Assembler H (option 5.1) 344
Batch Linkage Editor (option 5.7) 347
Batch VS COBOL II compiler (option 5.2) 344
Batch VS FORTRAN compiler (option 5.3) 345
Test field
Foreground VS COBOL II compiler (option 4.2) 320
testing
dialogs 355
TSO 355
usual test methods for dialog 355
text strings, use of 75
TIME parameter, SORT command 183
Total display view, Initial View field 131
trace output in ISPF log
function trace entries 385
trace header entries 384
variable trace entries 385
Traces (option 7.7)
1—function traces 389
2—variable traces 390
CANCEL Function Traces primary command 390
CANCEL Variable Traces primary command 391
D (delete) Function Traces line command 390
D (delete) Variable Traces line command 391
530  z/OS: z/OS ISPF User's Guide Vol II

## Page 569

Traces (option 7.7) (continued)
END Function Traces primary command 390
END Variable Traces primary command 391
I (insert) Function Traces line command 390
I (insert) Variable Traces line command 391
LOCATE Function Traces primary command 390
LOCATE Variable Traces primary command 391
R (repeat) Function Traces line command 390
R (repeat) Variable Traces line command 391
working with trace specifications 388
Tracks field
Data Set List utility 140
VTOC Information Display 147
TRACKS keyword, SORT command 162
trademarks 512
trigger statement errors 315
triggers
expansion trigger, definition 312
statement errors 315
user-defined 313
TRKS parameter, Space Units field 106
Trks/Cyls field, VTOC Information Display 147
TSO
CLISTs and REXX EXECs
Data Set List utility (option 3.4) 141
entering 353
variables 143, 145
commands
ACCOUNT 106
ALLOCATE 104
DELETE 156
TEST 355
TSO ACCOUNT 85
TSO command 416
TSO command for member 101
TSO workplace command 422
TTR data 92
Tutorial (option 7.T), displaying 397
U
U (uncatalog data set) line command, Data Set List utility
(option 3.4) 152
U (updating a format) line command, Format Specification
utility (option 3.11) 183
U record format, Record Format field 107
UA (user auditing) line command
z/OS UNIX Directory List Utility (option 3.17)
275
UIDs, switching 286, 307
uncatalog workplace command 422
Underline headings (in export report) 233
Unexclude Data Set command 153
Unexclude First Data Set command 154
Unexclude Last Data Set command 154
Unicode
FIND command 73
viewing in Browse 68, 70
Unit field, VTOC Information Display 147
unmovable data set, defined 140
UPD parameter, BROWSE Output field 196
Update DS Name field, SuperC Utility - Old Data Set Name
panel 186
Update Dsn field, SuperCE Utility panel 196
USASCII 75
Use EDIT as default to process selected table option 231
Use Edit to view the imported table 232
user interface
ISPF 507
TSO/E 507
user-defined trigger 313
UTF8 75
Utilities (option 3)
Command Table utility (option 3.9) 174
Data Set List utility (option 3.4) 126
Data Set utility (option 3.2) 101
Format Specification utility (option 3.11) 178
Hardcopy utility (option 3.6) 167
Library utility (option 3.1) 89
Move/Copy utility (option 3.3) 119
Outlist utility (option 3.8) 171
overview 2
Reset ISPF Statistics utility (option 3.5) 163
Search-For utility (option 3.14) 203
SuperC utility (option 3.12) 183
SuperCE utility (option 3.13) 192
Utility Selection Panel 89
utility differences
extended 431
standard 431
Utility Selection Panel 89
V
V (VDEFINE) variable, P (pool) field 369
V (view data set) line command, Data Set List utility (option
3.4) 149
V (view regular file) line command
z/OS UNIX Directory List Utility (option 3.17)
276
V record format, Record Format field 107
VA (view ASCII file) line command
z/OS UNIX Directory List Utility (option 3.17)
276
VA command, Data Set List utility (option 3.4) 163
Variables (option 7.3)
CANCEL primary command 370
creating new variables 370
D (delete) line command 370
DBCS data 372
deleting variables 371
END primary command 370
hexadecimal data 371
I (insert) line command 370
input errors 371
LOCATE primary command 370
manipulating variables 370, 371
primary commands 370
split-screen mode 371
test mode 371
usage notes 371, 372
variable life 371
variable value 371
working with dialog variables 367
variables, syntax diagrams xxvi
Verb column 175
VERT parameter, HEX command 78
View (option 1)
Index  531

## Page 570

View (option 1) (continued)
description 65
View Entry Panel 65
view action prompt workplace command 422
VIEW command, Browse 80
view output mode 185, 198, 205, 213
View workplace command 422
view, command table line command 178
viewing a member 101
Volume DATA field, VTOC Information Display 147
Volume display view, Initial View field 130
Volume field
Data Set List utility 139
Data Set List Utility panel 129
VOLUME keyword, SORT command 162
Volume Serial 85
Volume Serial field
Allocate New Data Set panel 105
volume, DASD 85
VS COBOL II compiler (option 4.2)
overview 319
VS COBOL II compiler (option 5.2) 344
VS COBOL II interactive debug (option 4.10) 330
VS COBOL II interactive debug (option 5.10) 348
VS command, Data Set List utility (option 3.4) 163
VS data set organization, DSORG field 140
VS FORTRAN compiler (option 4.3) 320
VS FORTRAN compiler (option 5.3) 345
VS Pascal compiler (option 4.6) 322
VS Pascal compiler (option 5.6) 346
VS-E 140
VSAM (*VSAM*) entry in Volume field 129
VSAM workplace command 421
VT command, Data Set List utility (option 3.4) 163
VTOC Data field, VTOC Information Display 147
VTOC summary workplace command 421
VU (view UTF8 file) line command
z/OS UNIX Directory List Utility (option 3.17)
277
VV command, Data Set List utility (option 3.4) 163
W
Warn if export data set exists 232
Warn if table exists in the output library 232
wildcard characters 128
Word parameter
Compare Type field 194
FIND command 73
word, defined for SuperC 194
Workplace (Option 11)
action bar choices
file 406
options 412
space 412
SuperC 413
test 414
view 411
actions that require prompt windows
ISPF command shell 417
move/copy 418
rename 419
reset statistics 415
TSO command 416
Workplace (Option 11) (continued)
commands 420
entry panels
data set view 404
library view 403
example scenario 427
ISPF Library field 404
ISPF referral lists 405
Object Name field 405
overview 403
specifying actions 406
Workplace Commands
= 422
ACTBAR action prompt 421
allocate 421
allocate action prompt 421
allocate SMS 422
Browse 421
catalog 421
command action prompt 421
compress 422
Copy 421
CUA attributes 421
delete 421
Delete 421
delete action prompt 421
DSLIST 421
Edit 421
edit action prompt 421
full information action prompt 421
global color change 421
information action prompt 421
ISPF command shell 421
ISPF command table 421
keylist 421
list action prompt 421
list view 421
locate 421
Member list 421
Move 421
NOACTBAR action prompt 421
Open 421
personal data set lists 421
personal library lists 421
point and shoot 421
Print 421
print action prompt 421
print data set 421
print data set index 422
Print data set list 421
Print VTOC information 421
rename 421
Rename 422
rename action prompt 422
reset action prompt 421
SearchFor 422
SearchFor extended 422
select 422
Settings 422
short information 422
Submit 421
SuperC 422
SuperC extended 422
TSO 422
532  z/OS: z/OS ISPF User's Guide Vol II

## Page 571

Workplace Commands (continued)
uncatalog 422
View 422
view action prompt 422
VSAM 421
VTOC summary 421
Workplace example 427
workplace function keys 422, 423
Workplace option description 2
workplace settings 423
workplace settings command 422
WORKSIZE SuperC process statement 463
WPSET command 422
WRITE parameter 382
Writer name 168
writing a list to a list data set
data set list 159
X
X (execute) line command
z/OS UNIX Directory List Utility (option 3.17)
277
X (print index listing) line command, Data Set List utility
(option 3.4) 153
XT field, Data Set list utility 140
XT keyword, SORT command 162
Y
Y2PAST SuperC process statement 466
YES parameter
Active field 389, 391, 393
BROWSE Output field 196
Confirm Delete Request field, select 131
Replace like-named members field, select 122
Select pack option for “To” data set field 122
Specify additional search strings field, select 204
Table available field 381
Table on disk field 383
YYYY.DDD format, Expiration Date field 109
YYYY/MM/DD format, Expiration Date field 109
Z
Z (compress data set) line command, Data Set List utility
(option 3.4) 152
z/OS system programmer applications 3
z/OS UNIX
execute command 277
file permissions 254, 259, 272
z/OS UNIX commands
z/OS UNIX directory list
282
z/OS UNIX Directory List Utility (option 3.17)
+ (scroll indicator) 286
= (repeat previous) line command 277
AA (auditor auditing) line command 252
action bar choices 244
audit information 266
B (browse) line command 253
bypass z/OS UNIX File Edit Options panel 283
C or CO (copy out) line command 253
z/OS UNIX Directory List Utility (option 3.17) (continued)
change column order and width 284
CI (copy in) line command 257
command time limit 283
confirm file delete option 242, 283
D (delete) line command 261
device data information 266
Display Directory List option 242
display permissions in octal format 283
E (edit) line command 261
EA (edit) line command 261
EDIT primary command 278
EU (edit) line command 261
extended attributes information 266
FIND primary command 279
FS (file system) line command 262
I (information) line command 264
L (List) line command 268
LEFT primary command 279
line command prefix characters 249
line commands 251
LOCATE primary command 279
MA (modify ACL) line command 268
MF (modify format) line command 270
MG(modify group) line command 271
MM (modify mode) line command 271
MO (modify owner) line command 272
mode fields information 265
MX (modify extended attributes) line command 273
N (new) line command 273
options 283
overview 239
owner information 265, 268
path name substitution character 283
path names 286
primary commands 278
Print Directory List option 251
R (rename) line command 275
RA (add to personal data set list) line command 275
REFRESH primary command 280
RESET primary command 280
restore default column arrangements 284
RFIND primary command 279
RIGHT primary command 280
run command under login shell 249
S (invoke default) line command 275
SAVE primary command 281
scrollable fields for path names 286
set and save default line commands 284
SORT primary command 281
sorting 244
SU primary command 282, 307
super-user mode 286, 307
switching UIDs 286, 307
symbolic link information 267
time zone value 240
timeout setting 283
UA (user auditing) line command 275
V (view regular file) line command 276
VA (view ASCII file) line command 276
view options 244
VU (view UTF8 file) line command 277
width of filename column 283
X (execute) line command 277
Index  533

## Page 572

z/OS UNIX Directory List Utility (option 3.17) (continued)
z/OS UNIX commands 282
z/OS user applications 3
ZDLBLKSZ variable 143
ZDLCAT variable 144
ZDLCATNM variable 144
ZDLCDATE variable 144
ZDLCMD variable 144
ZDLCONF variable 144
ZDLDEV variable 144
ZDLDSN variable 144
ZDLDSNTP variable 144
ZDLDSORG variable 144
ZDLDST variable 145
ZDLDSX variable 145
ZDLEDATE variable 144
ZDLEXT variable 144
ZDLEXTX variable 144
ZDLLCMD variable 144
ZDLLRECL variable 144
ZDLMIGR variable 144
ZDLMSG variable 145
ZDLMVOL variable 144
ZDLNDSN variable 145
ZDLOVF variable 145
ZDLRDATE variable 145
ZDLRECFM variable 145
ZDLREF variable 145
ZDLSIZE variable 145
ZDLSIZET variable 145
ZDLSIZEX variable 145
ZDLSIZTX variable 145
ZDLSPACU variable 145
ZDLUSED variable 145
ZDLVOL variable 145
ZPARM system variable
parenthesis added to 176
534  z/OS: z/OS ISPF User's Guide Vol II

## Page 573



## Page 574

IBM®
Product Number: 5655-ZOS
SC19-3628-60
