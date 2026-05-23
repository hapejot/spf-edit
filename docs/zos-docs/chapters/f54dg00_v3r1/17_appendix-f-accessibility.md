# Appendix F. Accessibility

Source file: f54dg00_v3r1.md
Start page: 401
Page span: 401-424

## Page 401

Appendix F. Accessibility
Accessible publications for this product are offered through IBM Documentation for z/OS (www.ibm.com/
docs/en/zos).
If you experience difficulty with the accessibility of any z/OS documentation see How to Send Feedback to
IBM to leave documentation feedback.
© Copyright IBM Corp. 1980, 2025 373

## Page 402

374  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 403

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
© Copyright IBM Corp. 1980, 2025 375

## Page 404

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
376  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 405

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
Notices  377

## Page 406

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
378  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 407

Index
Special Characters
_ (underscore) character, default attribute 144
.ALARM control variable 246
.ATTR control variable
considerations 249
description 247
override conditions 168
using with table display panels 248
.ATTRCHAR control variable
description of 248
dynamic area override 124
override conditions 168
.AUTOSEL control variable 249
.CCSID section of message definition 270
.CSRPOS control variable 250
.CSRROW control variable 250
.CURSOR control variable
description 251
when not initialized or set to blank 251
.HELP control variable
description 252, 261, 267
.HHELP control variable
description 252
.KANA control variable in messages 267
.MSG control variable
description 253
in batch mode 34
panel user exit messages 218
.NRET control variable 253
.PFKEY control variable 254
.RESP control variable
description 254
in batch mode 33
.TRAIL control variable
description 255
example 101, 201
.TYPE keyword, message definition 268
.WINDOW keyword, message definition 268
.ZVARS control variable
description 255, 256
example 256
.ZVARS control variable, associating a PDC with a variable
name in )ABCINIT 136
‘’ (quotation marks), enclosing literals 96
)ABC section of panel definition 133
)ABC section, defining pull-down choice 135
)ABCINIT section of panel definition 137
)ABCPROC section of panel definition 137
)AREA section of panel definition 138
)ATTR section of panel definition 143
)BLANK file-tailoring control statement 278, 280
)BODY section of panel definition 169
)BODY statement, WINDOW keyword 94
)CCSID section of panel definition 175
)CM file-tailoring control statement 280
)DEFAULT skeleton control statement 280
)DO file-tailoring control statement 282
)DOT file-tailoring control statement 283
)ELSE file-tailoring control statement 284
)END section of panel definition 176
)END statement, required on panel definition 95
)ENDDO file-tailoring control statement 282
)ENDDOT file-tailoring control statement 283
)ENDREXX file-tailoring control statement 286
)ENDSEL file-tailoring control statement 288
)FIELD section of panel definition 176
)HELP section of panel definition 182
)IF file-tailoring control statement 284
)IM file-tailoring control statement 285
)INEXIT section of panel definition 183
)INIT section of panel definition 191
)ITERATE file-tailoring control statement 286
)LEAVE file-tailoring control statement 286
)LIST section of panel definition 192
)MODEL section of panel definition 192
)N comment statement 280
)NOP file-tailoring control statement 286
)PANEL statement KEYLIST parameter 192
)PNTS statement 194
)PROC section of panel definition 197
)REINIT section of panel definition 197
)REXX file-tailoring control statement 286
)SEL file-tailoring control statement 288
)SET file-tailoring control statement 289
)SETF file-tailoring control statement 289
)TB file-tailoring control statement 290
)TBA file-tailoring control statement 290
*REXX panel statement
SOURCELINE function 223
% sign
beginning a command procedure name with 12
default attribute character 144
+ sign
continuation character for literals 96
default attribute character 144
÷< operator on the IF statement 211
÷= operator on the IF statement 211
÷> operator on the IF statement 211
< operator on the IF statement 211
<= operator on the IF statement 211
= (equal sign) operator on the IF statement 211
> (greater than) operator on the IF statement 211
>= operator on the IF statement 211
Numerics
3278 Mod 5
batch mode 34
graphics interface mode 127
3290
batch mode 34
graphics interface mode 127
900-999 error return codes 22
Index  379

## Page 408

999 error return code 22
A
A, used to specify alternate tabbing 290
ABCINIT section of panel definition 137
ABCPROC section of panel definition 137
abend
description 24
diagnostic panels 336
ABEND
codes 337
accessibility
contact IBM 373
accessing table data 62
action bar choice initialization panel definition section
definition 137
action bar choice processing section of panel definition
definition 137
action bar choice section of panel definition
definition 133
action bars and pull-down choices 80
ADDPOP parameter on ISPSTART command 9
ADDPOP service 79, 80
ADDSOSI built-in function on assignment statement 205
alarm indicator message 371
ALARM keyword, message definition 268
ALPHA parameter on VER statement 232
ALPHAB parameter on VER Statement 232
alternate tabbing 290
APL keyboard character translations 311
APL2
multiple calls of 29
number of times invoked, system variable containing
361
using 27
workspace used as the function pool 30
application identifier, system variable 361
application keylist 79
application profile pool 53, 58
application profile pool extension name, system variable 362
application_id parameter on ISPSTART 9, 13
area section of panel definition
definition 138
AREA(DYNAMIC) parameter in )ATTR section 147
AREA(SCRL) parameter in )ATTR section 151
argument variables 62
array of variable lengths on panel user exit parameter 218
array of variable names on panel user exit parameter 218
ASIS parameter
in )BODY header statement 172
on VGET panel statement 242
on VPUT panel statement 243
with JUST keyword 155
aspect ratio system variable for PRINTG 370
assignment statement in panel definition 200
assistive technologies 373
attention exits (CLIST) 26
ATTN keyword in )ATTR section 151
ATTN statement 26
attribute characters
default 144
restriction 144
attribute section of panel definition
attribute section of panel definition (continued)
basic attribute types 163
CUA attribute types 166
default characters 144
definition 143
other attribute types 168
requirements for table display panel 116
authorized programs, invoking 25
authorized TSO commands, invoking 25
AUTOSEL (.AUTOSEL) control variable 249
AUTOSEL (auto-selection) 113
autoskip
description 161
graphic area 127
B
BACK tutorial command 261
background display execution 33
background panel processing 33
BARRIER keyword 99
batch display facility, using 33
batch environment
avoiding loops in batch 35
display error processing 35
log and list data sets 35
maximum number of panel displays 35
processing commands 34
terminal characteristics 34
TSO 31
batch execution
description 31
TSO error processing 33
TSO sample job 32
BATSCRD keyword on ISPSTART command 9, 34
BATSCRW keyword on ISPSTART command 9, 34
BDBCS keyword on ISPSTART command 9, 34
BDISPMAX keyword
and ZBDMAX system variable 361
on ISPSTART command 9, 35
BIT parameter on VER statement 232
BLANK file-tailoring control statement 280
blinking, specifying for HILITE keyword 154
body section of panel definition
controlling width of panel 170
defining 170
definition 169
formatting message field 171
requirements 116
requirements for table display panel 116
sample 174
Boolean operators on the IF statement 213
bottom-of-data marker
definition 113
system variable containing for table display, user
defined 370
BREDIMAX keyword on ISPSTART command 9, 34
BRIF service 76
BROWSE service 76
browse service scroll amount, system variable 369
browse services panel definition, scroll field location 90
built-in function on assignment statement 207
380  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 409

C
call of ISPF 7, 8
CAPS keyword in panel )ATTR section 116, 145, 151
CCSID parameter of the GETMSG service 300
CCSID section of message definition
messages tagged 270
CCSID section of panel definition
definition 175
extended code page support 300
chain mode, explicit 102
char parameter
with PAD keyword 157
with PADC keyword 157
with PAS keyword 158
character compare on IF statement 212
character level attribute 124
character translations for APL, TEXT and Katakana
keyboards 311
CHINESES keyword on ISPSTART command 9, 15
CHINESET keyword on ISPSTART command 9, 15
CKBOX keyword in panel )ATTR section 151
CLEAR keyword on )MODEL statement in table display panel
117
CLIST
attention exits 26
invoking procedure from ISPSTART command 15
variables used in procedure 6
CLIST edit macros, running unnested 362
CM file-tailoring control statement 280
CMD
keyword
in )PROC section 99
in panel )BODY section 171
parameter on ISPSTART command 9
code page parameter for ISPSTART 13
coded character set identifier, system variable 364
CODEPAGE 13
COLOR keyword in panel )ATTR section 152
COMBO keyword in panel )ATTR section 144
command field
naming of 90
naming with the CMD keyword 171
panel )BODY section 170
position in panel definition 90
command field of a table display panel 113
command line placement, system variable 362
COMMAND parameter, in panel )PROC section 99
command procedure 55
command tables
and application IDs 13
definition of 2
ISPCMDS system command table 2
command verb after a SETVERB command table action,
system variable 365
commands
ISPF, in batch environment 33
processing in batch environment 34
reading syntax diagrams xvi
comment statements 95
comments, optional display 155
Common User Access (CUA)
description of ISPF support 79
dot leaders 92
Common User Access (CUA) (continued)
keyword values 167
compare character vs. numeric 212
compiled REXX 25, 216
COMPOUND variables 6
concatenation of variables 97
conditional padding of panel field 145
conditional substitution string 279
configuration utility (system variables) 359
CONFLICT parameter on SHRPROF command 18
CONT system variable on tutorial panels 262
contact
z/OS 373
continuation character for literals 96
continuation panel 262
control characters
in skeleton definition 277
control characters in skeleton definition 278
CONTROL NONDISPL in batch mode 33
CONTROL service 77
control variables
in panels 244
initialization 245
list of 244
when reset 245
conversion utility 79
CRASH 24
creating action bars 136
CRP of top row displayed in most recent table display,
system variable 370
CSRGRP(x) keyword in panel )ATTR section 153
CSRPOS (.CSRPOS) control variable 250
CUA guidelines, dot leaders 92
CUADYN 165
CUADYN keyword in panel )ATTR section 153
cursor placement, default 251
cursor position
system variable 362
D
DANISH keyword on ISPSTART command 9, 15
data records in skeleton definition
control characters 278
DATAMOD keyword in )ATTR section of dynamic panels 148
date and time information (system variables) 360
DBCS
batch mode 34
command and message fields 170
data validation 97
parameter on VER statement 232
replacement characters 160
specifying format 153
specifyingsearch argument format for table services 73
system variable containing terminal capability 366
variables
in messages and file skeletons 127
on panel definitions 274, 297
verifying string length (VER LEN) 236
DDL file name
system variable 361
DDLIST keyword in panel )ATTR section 144
ddname of file tailoring temporary file, system variable 364
debug tools 317
Index  381

## Page 410

DEFAULT
attribute or body section statement 144
skeleton control statement 280
default attribute characters 144
default keylist for DTL Help Panels 260
defining messages 265
delimiter
system variable 361
delimiters in verified variable 233
DELSOSI built-in function on assignment statement 205
DEPTH keyword in panel )ATTR section 153
determining table size 65
device name system variable for PRINTG 370
diagnosing ISPF abends 336
dialog
beginning with menu or function 5, 8
call by using application master menu 16
control 5
definition 1
development of 4
elements 1
example 66
function, languages used for coding 2
initiation 19
organization 5
return codes 21
running of 8
scope 20
termination 21
variables 6
writing
using display services 37
using file-tailoring services 73
using miscellaneous services 77
using PDF services 76
using table services 61
using variable services 52
dialog elements
description 4
test of 4
dialog function
creation of 4
description of 1
dialog, languages used for coding 2
example 66
function pools 54
naming 12
scope 20
Dialog Tag Language (DTL) 79
dialog variables
format of 59
ISPPRXVP processor 222
processing with panel REXX 222
dialog variables, list of 349
directive lines, optional display 155
display error processing in the batch environment 35
display message variations 270
display services
DBCS-related variables 127, 274, 297
in batch mode 33
displaying a pop-up window 80
DO
file-tailoring control statement 282
DOT file-tailoring control statement 283
DSNAME parameter on VER statement 232
DSNAMEF parameter on VER statement 232
DSNAMEFM parameter on VER statement 233
DSNAMEPQ parameter on VER statement 233
DSNAMEQ parameter on VER statement 233
DUMP keyword on ENVIRON command 335
dynamic area
character level attribute support 124
formatting panels 122
dynamic table expansion 40, 113
E
EBCDIC
parameter on VER statement 233
specifying format 153
EDIF service 76
EDIREC service 76
EDIT service 76
edit service panel definition, specifying location of scroll field
90
edit service scroll amount, system variable 369
EDREC service 76
elements of a dialog 1
ELSE file-tailoring control statement 284
ELSE statement in panel sections 209
ENBLDUMP parameter on ENVIRON command 332
end of displayed data specification 113
END section of panel definition
definition 176
ENDDO file-tailoring control statement 282
ENDDOT file-tailoring control statement 283
ENDREXX file-tailoring control statement 286
ENDSEL file-tailoring control statement 288
ENGLISH keyword on ISPSTART command 9, 15
entry point address on diagnostic panel 336
ENUM parameter on VER statement 233
ENVIRON system command 331
environment 1
environment description, system variable 361
EQ operator on the IF statement 211
error conditions for panel user exit 218
ERROR keyword on ENVIRON command 335
error message-id, system variable 371
error panel 35
error processing
SYSPRT file 20
TSO batch execution 33
when put into effect 20
error recovery panel at abend 337
error return codes from dialog to invoking application 22
ESTAE restrictions 31
EXCLPROF
parameter on ISPSTART command 14
EXCLPROF parameter 9
executable section of a dialog 183, 191, 197
executing APL2 functions 29
EXHELP 82, 257
exit data on panel user exit parameter 217
EXIT keyword in )PROC section 99, 101, 103
EXIT statement
panel REXX 223
EXIT statements 208
exits, CLIST attention 26
382  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 411

EXPAND keyword in panel )BODY section 170
expected-length operand (on VER LEN) 237
explicit chain mode 102
EXTEND parameter
in )ATTR section 147, 151
in graphic areas 149
Extended Code Page Support
base code pages 305
CCSIDs supported 303
description 299
ISPF-provided translate tables 308
messages tagged 300
panels tagged 300
translate load modules 300
Z variables 299
Extended Code Page Translate Tables Provided by ISPF 308
extended help 82, 257
extended highlighting availability, system variable 366
extension table 58
extension variables
clearing in model lines 117
F
FIELD keyword
in panel )FIELD section 177
field section of panel definition
definition 176
field-level help 82, 182, 257
field-type specification in panel )ATTR section 161
file tailoring temporary file name, system variable 364
file-tailoring services
example 75
skeleton files 73
writing dialogs 73
file-tailoring skeleton
control statement considerations 279
data record considerations 73, 278
DBCS considerations 297
debugging 324
defining 277
definition 3
sample 296
trace command (ISPFTTRC) 324
FILEID parameter on VER statement 235
fixed portion of a TBDISPL display 114
FORMAT keyword
in panel )ATTR section 145, 153
in panel )BODY section 170
formatting guidelines for panels 192
fragments, syntax diagrams xvi
FRENCH keyword on ISPSTART command 9, 15
function commands, definition 116
Function key set displayed, system variable 367
Function key settings, system variables 367
Function keys, system variable containing number of 367
function pool
using variables to communicate between functions 60
function, definition 1
G
GDDM
GDDM (continued)
in batch environment 33
interface to 126
GDDM service 77
GE keyword
in panel )ATTR section 154
GE operator on the IF statement 211
GERMAN keyword on ISPSTART command 9, 15
GETMSG service 78
GOTO statement in panel section 208, 209
graphic area, panel definition 148
GRPBOX 168
GT operator on the IF statement 211
H
help
extended 82, 257
field-level 82, 257
help for help 257
keys 83, 257
message 257, 258
panel 257, 258
reference phrase 83, 257
TUTOR command 257
tutorial 257
help for help command 257
help panel
name associated with error message 371
system variable containing name associated with error
message 372
with scrollable areas 140
See also tutorial
help section of panel definition
definition 182
HELP system command
entry to tutorial 261
on ABEND panels 337
HEX parameter on VER statement 235
HEX primary command 180
HIGH parameter with INTENS keyword 155
HILITE keyword in panel )ATTR section 154
I
IDATE parameter on VER statement 235
IF file-tailoring control statement 284
IF statement
basic IF 211
with Boolean operators 213
with VER constructs 212
with VSYM built-in function 213
IM file-tailoring control statement 285
IMAGE keyword 192
IN parameter used with CAPS keyword 151
INCLUDE parameter on VER Statement 235
IND keyword
in panel )FIELD section 177
index page, specifying for tutorials 262
INDEX tutorial command 261
INEXIT section of panel definition
definition 183
initialization of control variables 245
Index  383

## Page 412

initialization section of panel definition
definition 191
requirements for table display 119
initiating dialog execution 19
INPUT parameter used with TYPE keyword 162
INTENS keyword in panel )ATTR section 145
interpreted REXX 216
invoking
authorized commands 25
authorized programs 25
authorized TSO commands 25
TSO commands 25
invoking a dialog
from a selection panel 16
from the ISPF master application menu 16
the ISPSTART command 15
ISP@MSTR, ISPF Master Application Menu 103
ISP@PRIM on the ISPF Primary Option Menu 107
ISPCMDS system command table 2
ISPDPTRC (panel trace command) 317
ISPF
command 25
Common User Access support 79
default keylist 260
EDIF service 30
help panels 257
interface with APL2 30
overview 4
tutorial panels 257
variables 58
ISPF conversion utility 79
ISPF dialog variables
panel REXX 223
ISPF Services in Batch Mode 31
ISPFTTRC (file tailoring trace command) 324
ISPPREP preprocessed panel routine
batch environment 34
error conditions 132
examples 131
restrictions 129
return codes 132
using 128
ISPPRXVP dialog variable processor 222
ISPREXPX 219
ISPSTART command
description 7, 8
example 8
syntax 8
TSO 25
ISPTTDEF, using to specify translate tables 315
ISPTUTOR 261
ISRABEND debug tool 317
ISRCSECT debug tool 317
ISRFIND debug tool 317
ISRPOINT debug tool 317
ISRROUTE command 136
ISRTCB debug tool 317
ISRTEST debug tool 317
ISRVCALP panel REXX example 227
ITALIAN keyword on ISPSTART command 9, 15
ITERATE file-tailoring control statement 286
ITIME parameter on VER statement 236
J
JAPANESE keyword on ISPSTART command 9, 15
JDATE parameter on VER statement 236
JSON data, ZCLIENT
system variable 361
JSTD parameter on VER statement 236
JUST keyword in panel )ATTR section 116, 145, 155
justifying a panel field 155
K
KANA keyword
extended code page support 302
on panel )BODY section 170, 311
Katakana
keyboard character translations 311
terminal displaying messages 267
key assignment 79
keyboard
navigation 373
PF keys 373
shortcut keys 373
keylist
application 79
system 79
keylist defaults for DTL Help Panels 260
KEYLIST parameter on )PANEL statement 192
keylist utility 93
keys 260
keys help 83, 257
KEYS system command, batch environment 34
KEYSHELP 83, 257
keywords, syntax diagrams xvi
KOREAN keyword on ISPSTART command 9, 15
L
LANG(APL) parameter
in panel )PROC section 99
on ISPSTART command 9
languages used for coding functions 2
last visible line function (LVLINE) 205
LCOL keyword
in panel )FIELD section 178
LE operator on the IF statement 211
leading blanks in verified variable 233
LEAVE file-tailoring control statement 286
LEFT parameter used with JUST keyword 155
LEN keyword
in panel )FIELD section 177
LEN keyword on VER statement 236
LENGTH built-in function on assignment statement 204
LIBDEF service 78
library access services 76
LIND keyword
in panel )FIELD section 177
line display mode, automatic and nonautomatic entry into
line mode 12
list data set in a batch environment 35
LIST parameter on VER statement 237
list section of panel definition
definition 192
384  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 413

LIST service 78
LISTBOX keyword in panel )ATTR section 144
LISTV parameter on VER Statement 237
LISTVX parameter on VER Statement 238
LISTX parameter on VER Statement 238
LMSG parameter on panel )BODY section 171
loading a panel user exit routine 216
loading a REXX panel exit 216
log data set
batch messages 34
in batch environment 35
LOG service 78
logical screens
system variable 362
logical screens, maximum
system variable 363
LOGO parameter on ISPSTART command 14
LOGOFF command 25
LOGON command 25
long error message text, system variable 371
loops, avoiding in batch 35
LOW parameter used with INTENS keyword 155
LT operator on the IF statement 211
LU name of TSO session, system variable 367
LVLINE built-in function on assignment statement 205
M
master application menu
example of definition 103
example of display 16
member lists scroll position, system variable 369
member lists, scrolling 369
menu
definition of primary option 101
entry to tutorial 261
example of a master application menu 103
example of primary option 112
special definition requirements 97, 98
use of ZPARENT to set next display 102
message alarm indicator 371
message definition
DBCS considerations 274, 297
description of 3
example of short and long 266
Katakana considerations 267
message ID 267
processing 265
syntax 266, 274
message field location 89
message fields in panel )BODY section 170
message help 257, 258, 267
message ID on panel user exit parameter 218
message library
description of 265
example 266
message text
long error 371
short error 371
system variable containing 371
message-id, system variable containing error 371
messages
display variations 270
in batch environment 34
miscellaneous services, used in writing dialogs 77
MIX parameter on VER statement 238
mixed characters, specifying format 153
MODE keyword 99, 101
model lines
clearing variables in 117
definition of 114
specified in a variable 118
model section of panel definition
definition 192
requirements for table display panel 117
model sets
description of 114
example 38
modeless message pop-ups 271
module name on diagnostic panel 336
movable pop-ups
manual movement 82
WINDOW command 81
MSG=value parameter on assignment statement 202
msgid keyword 267
multicultural support
common characters 299
GETMSG service 300
messages tagged with CCSID 270
TRANS service 300
N
NAME parameter on VER statement 238
name-list parameter
on VSYM panel statement 244
named variables 219
NAMEF parameter on VER statement 238
naming defined and implicit variables 56
naming restrictions for dialog functions 12
National Language Support 270
See also multicultural support
navigation
keyboard 373
NB parameter on VER statement 232
NE operator on the IF statement 211
negative number indicators 233
NEST keyword 99
nested CLISTS, attention exits 27
NESTMACS keyword on ISPSTART command 9
NEWAPPL, (application_id) parameter 9
NEWAPPL, (application-id) parameter 99
NEWPOOL parameter in )PROC section 99
NG operator on the IF statement 211
NL operator on the IF statement 211
NLS 270
See also multicultural support
NOCHECK parameter
example 101
in )PROC section 99
NOJUMP keyword in panel )ATTR section 156
NOKANA keyword in message definition 267
NOLOGO parameter on ISPSTART command 14
NON parameter used with INTENS keyword 155
NONBLANK parameter on VER statement 232
NOP file-tailoring control statement 286
NOPROMPT parameter on SHRPROF command 18
null system variable 361
Index  385

## Page 414

NULLS parameter used with PAD keyword 157
NUM parameter on VER statement 238
number of colors supported by the terminal type, system
variable 366
number of Function keys, system variable 367
number of variables on panel user exit parameter 218
numeric (extended) verification 233
numeric compare on IF statement 212
NUMERIC keyword in panel )ATTR section 156
Numeric Lock feature (with NUMERIC attribute keyword) 156
O
OFF parameter
with ATTN keyword 151
with CAPS keyword 151
with NOJUMP keyword 156
with NUMERIC keyword 156
with SKIP keyword 161
ON parameter
with ATTN keyword 151
with CAPS keyword 151
with NOJUMP keyword 156
with NUMERIC keyword 156
with SKIP keyword 161
ONEBYTE built-in function on assignment statement 206
online tutorial 260
OPT system variable 98
OPT(option) parameter on ISPSTART command 9
OUT parameter used with CAPS keyword 151
OUTLINE keyword
in panel )ATTR section 143, 145, 156
in panel )BODY section 170, 174
OUTPUT parameter used with TYPE keyword 162
P
PAD keyword in panel )ATTR section 145, 157
PADC keyword in panel )ATTR section 157
panel definition
)PNTS statement 194
attribute section
default characters 144
blanks 95
body section
sample 174
command field
description 89
specifying 170
comment statement 95
description 87, 89
design suggestions 91
dynamic areas 165
graphic areas 148
help and tutorial panels 260
initialization section
statement formats 199
line 1 content 90
line 2 content 90
line 3 content 90
location 89
menus 98
model section 117
panel definition (continued)
panel title, location 89
reinitialization section
statement formats 199
restrictions 94
sections 87
short message for TBDISPL operations 90
size 94
special requirements 97
specifying a message field 171
split-screen consideration 91
syntax rules 94
table display 112
tutorial and help panels 260
using )PANEL 192
panel help 257, 258
panel name on panel user exit parameter 217
PANEL parameter
in )PROC section 99
on ISPSTART command 9
panel redisplay 198
panel REXX
EXIT statement 223
ISPF dialog variables 223
ISRVCALP example panel 227
SOURCELINE function 223
panel section of panel definition
formatting panel 192
panel section on panel user exit parameter 217
panel trace command (ISPDPTRC) 317
panel user exit routine
description 214
how to invoke 217
how to load 216
parameters passed 217
return codes 218
panels
debug/trace 317
preprocessed 128
vertically scrollable 94
PANEXIT statement 214
PARM
keyword
in )PROC section 99
on preprocessed panels 128
parameter on ISPSTART command 9
parts of a dialog 1
PAS keyword in panel )ATTR section 158
passing control from program-coded to command-coded
function 5
PDF command 25
PDF service
library access 76
writingdialogs 76
pending END request 114
pending scroll request 114
pending selected rows 114
percent (%) sign, beginning a command procedure name
with 12
PF key, system variable 362
PFK built-in function on assignment statement 203
PGM keyword in )PROC section 99
PGM parameter on ISPSTART command 9
PICT parameter on VER statement 238
386  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 415

PICTCN parameter on VER statement 239
PNTS section of panel definition 194
Point-and-shoot section of panel definition 194
pools, variable
application profile 53
function 53
shared 53
pop-up window
ADDPOP service 80
movable 81
processing considerations 127
size 170
PORTUGUESE keyword on ISPSTART command 9, 15
POSITION, TBDISPL parameter 115
PQUERY
in batch environment 34
used with dynamic area 124
PQUERY service 78
prefix system variable 362
preprocessed panels
creating (ISPPREP) 128
definition 128
ISPPREP call 129
PARM keyword 128
SELECT service 128
Primary Option Menu 101
printer family type for PRINTG 370
processing section of panel definition
definition 197
requirements for table display 119
PROFILE parameter
on VGET panel statement 242
on VPUT panel statement 243
program status word on diagnostic panel 336
program_name parameter
on ISPSTART command 9
program-name parameter
in panel )PROC section 99
PROMPT parameter on SHRPROF command 18
protecting table resources 63
PSW on diagnostic panel 336
pull-down choice, defining within the )ABC section 135
Q
QUERY parameter on the ENVIRON command 336
quotation mark, enclosing literals 96
quote mark, enclosing literals 96
R
RADIO keyword in panel )ATTR section 159
RANGE parameter on VER statement 240
RCOL keyword
in panel )FIELD section 179
read-only profile pool extension variables 58
reason code on diagnostic panel 336
recovery termination manager at abend 338
redisplay of a panel 198
reference phrase help 83, 257
REFRESH statement in panel sections 220
register content at abend on diagnostic panel 336
reinitialization section of panel definition
reinitialization section of panel definition (continued)
definition 197
requirements for table display 119
relational operators (on VER LEN) 236
removing a pop-up window 80
removing variables from the shared or profile pool 57
REMPOP service 79, 80
REP keyword in panel )ATTR section 145, 160
repeatable items, syntax diagrams xvi
replacement characters 160
reset of control variables 245
RESET parameter on SHRPROF command 18
RETRY parameter on SHRPROF command 18
return codes
for panel user exit routine 218
from terminating dialog 21
return to function when scrolling 40
REVERSE parameter used with HILITE keyword 154
reverse video, specifying 154
REXCHK parameter on ENVIRON command 336
REXX edit macros, running unnested 362
REXX file-tailoring control statement 286
REXX panel exit
how to load 216
REXX panel statement
SOURCELINE function 223
REXX variables 223
RIGHT parameter used with JUST keyword 155
RIND keyword
in panel )FIELD section 178
ROWS keyword on )MODEL statement in table display panel
117
rows of a table, adding dynamically 40, 44
running a dialog 8
S
SCALE keyword
in panel )FIELD section 179
scope of a function 20
screen
logical number of 368
system variable containing 368
screen depth and width available for use by a dialog
system variable containing 368
screen depth and width available for use by a dialog, system
variable 368
screen depth on ISPSTART command for batch 9
screen name
system variable 363
screen width for batch mode on ISPSTART command 9
scroll amount
field of a TBDISPL display, definition of 114
for browse service, system variable containing 369
for edit service, system variable containing 369
for member lists, system variable containing 369
location 89
maximum for member lists 369
minimum for member lists 369
number of lines or columns 369
system variable containing 369
system variable containing field value 369
value default for dynamic areas and table display 369
SCROLL keyword
Index  387

## Page 416

SCROLL keyword (continued)
in panel )FIELD section 179
SCROLL parameter in )ATTR section 147
scroll position
for member lists, system variable containing 369
scrollable areas
definition, section of panel 138
in the )BODY section 151
vertically scrollable panels 94
with help panel 140
scrollable fields, primary commands 179
scrollable portion of a TBDISPL display 114
scrolling, expanding displayed table 41
SDWA reason code at abend 337
searching variable pools 53
SEL
file-tailoring control statement 288
system variable 98, 262
select field of a TBDISPL display 115
SELECT service
call 20
description 19
panel (VGET) 243
panel processing 99
passing control in a dialog 53
preprocessed panels 128
Selected Choice (SC) attribute 168
selected row, defined 115
selection panel, system variables 371
separator
system variable 361
system variable containing 363–365
services
to dialogs 1
to interactive applications 1
services description, SELECT 19
SET file-tailoring control statement 289
SETF file-tailoring control statement 289
SFIHDR keyword on )MODEL statement in table display
panel 117
SGERMAN keyword on ISPSTART command 9, 15
shadow variable 124
SHARED parameter
on VGET panel statement 242
on VPUT panel statement 243
shared pool 53
sharing variables among dialogs 57
shift-in character (DBCS) 153, 205
shift-out character (DBCS) 153, 205
short error message text, system variable 371
short message syntax 267
shortcut keys 373
SHRPROF
parameter on ISPSTART command 14
SHRPROF system command 17
SIND keyword
in panel )FIELD section 178
site command table prefix, system variable 363
skeleton
description of 3
skeleton definition
)REXX statement 286
assigning a value to a variable 289
comment statement 280
skeleton definition (continued)
control characters 277
control statements 277, 279
data records 277
defining 277
example 297
IF-THEN-ELSE statement 284
imbedding 285
imbedding blank lines 280
loop processing 286
null statement 286
SET with functions statement 289
specifying table processing 283
tab stop 290
SKIP
keyword in panel )ATTR section 145, 161
tutorial command 261
SMSG parameter on panel )BODY section 171
SOURCELINE function, and panel REXX 223
SPANISH keyword on ISPSTART command 9, 15
specifying DBCS search argument format 73
SPF command 25
SPLIT command, disabled in batch environment 34
split-screen in effect, system variable 368
SPLITV system command, disabled in batch environment 34
stacked commands, graphics interface mode restriction 127
START service 84
starting a dialog
methods 8
using the ISPSTART command 15
using the SELECT service 19
starting ISPF 7, 8
STDDATE parameter on VER statement 240
STDTIME parameter on VER statement 240
STEM variables 6
stepname of TSO logon, system variable 362
storing variables from a panel in shared and profile pools
(VPUT) 243
string of variable values on panel user exit parameter 218
substitution string, conditional 279
subtasking support 31
summary of changes xxv
SYMDEF parameter
on VGET panel statement 242
SYMNAMES parameter
on VGET panel statement 242
syntax diagrams, how to read xvi
syntax rules
message definition 267, 274
panel definition 94
skeleton definitions 277
System keylist 79
system symbolic variables 60
system variables
JSON data (ZCLIENT) 361
list of 359
used for communication between dialogs and ISPF 371
Z 361
ZACCTNUM 361
ZAMT 368
ZAPLCNT 361
ZAPPLID 361
ZAPPTTL 361
ZASPECT 370
388  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 417

system variables (continued)
ZBDMAX 361
ZBDMXCNT 361
ZCFGCMPD 359
ZCFGCMPT 359
ZCFGKSRC 360
ZCFGLVL 360
ZCFGMOD 360
ZCLIENT 361
ZCMD 371
ZCOLORS 366
ZCONT 371
ZCS 361
ZCSDLL 361
ZCURFLD 372
ZCURINX 372
ZCURPOS 372
ZDATE 360
ZDATEF 360
ZDATEFD 360
ZDATESTD 360
ZDAY 360
ZDBCS 366
ZDECS 361
ZDEL 361
ZDEVNAM 370
ZDYNSCR 369
ZEDLMSG 361
ZEDSMSG 361
ZENTKTXT 361
ZENVIR 361
ZERRALRM 371
ZERRHM 371
ZERRLM 371
ZERRMSG 371
ZERRSM 371
ZERRTYPE 371
ZERRWIND 371
ZEURO 361
ZFAMPRT 370
ZFKA 366
ZGE 125, 366
ZGUI 361
ZHILITE 366
ZHINDEX 371
ZHTOP 371
ZIND 371
ZIPADD6 366
ZIPADDR 366
ZIPPORT 366
ZISPFOS 362
ZISPFRC 362
ZJ4DATE 360
ZJDATE 360
ZKEYHELP 362
ZKEYS 367
ZKLAPPL 367
ZKLNAME 367
ZKLTYPE 367
ZKLUSE 367
ZLANG 362
ZLOGNAME 371
ZLOGO 362
ZLOGON 362
system variables (continued)
ZLSTLPP 370
ZLSTNAME 371
ZLSTNUML 370
ZLSTTRUN 370
ZLUNAME 367
ZMLPS 362
ZMONTH 360
ZNESTMAC 362
ZOS390RL 362
ZPANELID 362
ZPARENT 371
ZPF01-24 367
ZPFCTL 367
ZPFFMT 367
ZPFKEY 362
ZPFLxx 367
ZPFSET 367
ZPFSHOW 367
ZPLACE 362
ZPREFIX 362
ZPRIKEYS 367
ZPRIM 371
ZPROFAPP 362
ZRXRC 223
ZSCBR 369
ZSCED 369
ZSCML 369
ZSCRCUR 362, 363
ZSCREEN 368
ZSCREENC 362
ZSCREEND 368
ZSCREENI 362
ZSCREENW 368
ZSCRMAX 363
ZSCRMAXD 368
ZSCRMAXW 368
ZSCRML 369
ZSCROLLA 369
ZSCROLLD 369
ZSCROLLN 369
ZSCROLNL 369
ZSCTPRE2 363
ZSCTPRE3 363
ZSCTPREF 363
ZSCTSRCH 363
ZSEL 371
ZSEQ 363
ZSM 363
ZSPLIT 368
ZSTART 363
ZSTDYEAR 360
ZSWPBR 368
ZSYSICON 363
ZSYSID 363
ZSYSNODE 364
ZSYSPLEX 364
ZSYSPROC 364
ZTDADD 370
ZTDAMT 370
ZTDLROWS 370
ZTDLTOP 370
ZTDMARK 370
ZTDMSG 370
Index  389

## Page 418

system variables (continued)
ZTDRET 370
ZTDROWS 370
ZTDSCRP 370
ZTDSELS 370
ZTDSIZE 370
ZTDSRID 370
ZTDTOP 370
ZTDVROWS 370
ZTEMPF 364
ZTEMPN 364
ZTERM 368
ZTERMCID 364
ZTERMCP 364
ZTERMCS 364
ZTHS 364
ZTIME 360
ZTIMEL 360
ZTS 364
ZTSICMD 364
ZTSSCMD 365
ZUCTPRE2 365
ZUCTPRE3 365
ZUCTPREF 365
ZUP 371
ZUSC 369
ZUSER 365
ZVERB 365
ZWINTTL 365
ZWSCDPG 365
ZWSCON 365
ZWSOPSYS 365
ZXSMAX 369
ZXSMIN 369
ZYEAR 360
SYSTSPRT file for error messages 33
T
tab stop in skeleton definition 290
tabbing
alternate 290
table
accessing data 62
adding rows dynamically 40
definition 3
dynamic expansion 113
temporary or permanent 61
when created or updated 3
table display (TBDISPL), terms related to 112
table display panel definition
attribute section 116
body section 116
example 120
example of multiple model lines 121
initialization section 119
message location 90
model line 38, 112
model section 117
scroll field location 90
short message area content 90
using the TBDISPL service 112
table rows
number of selected upon return from table display 370
table rows (continued)
number of system variable containing upon return from
table display 370
number of visible rows upon return from table display
370
system variable containing 370
table services
determining table size 65
example 64, 66
protecting resources 63
row operation 63
using 61, 62
tags, creating dialog elements 79
task abend code on diagnostic panel 336
TB file-tailoring control statement 290
TBA file-tailoring control statement 290
TBDISPL series 115
TBDISPL service
description 120
dynamically building the table 41
terms related to 112
writing dialogs 37
terminal data in batch mode 34
terminal type
specifying ISPTTDEF 315
system variable containing 368
terminating
a dialog 21
ISPF 7, 8
TERMSTAT parameter on ENVIRON command 335
TERMTRAC parameter on ENVIRON command 332
TEST
difference from TESTX 24
mode 23
parameter on ISPSTART command 9
testing dialog elements 4
TESTX
difference from TEST 24
mode 23
parameter on ISPSTART command 14
TEXT keyboard character translations 311
TEXT parameter used with TYPE keyword 162
time and date information (system variables) 360
TOC tutorial command 261
TOG statement 228
top-row-displayed indicator 43, 115, 370
trace
file-tailoring execution 324
panel execution 317
TRACE
difference from TEST and TRACEX 25
mode 24
parameter on ISPSTART command 9
TRACEX
difference from TEST and TRACE 25
mode 24
parameter on ISPSTART command 14
trademarks 378
trailing blanks in verified variable 233
TRANS built-in function on assignment statement
description 202
example 100, 252
example, nested 202, 203
translate tables, specifying 315
390  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 419

translation
common characters 299
GETMSG service 300
messages tagged with CCSID 270
TRANS service 300
TRUNC built-in function on assignment statement
description 201
example 100, 201
example, nested 202, 203
truncation, system variable containing list data set 370
TSO
batch environment 31
batch execution 32
command restrictions 25
invoking authorized commands 25
invoking commands 25
TSO command 25
TSO session LU name, system variable 367
TSOEXEC interface 25
TUTOR command 257
tutorial
call of 260
commands 261
defining panels 260
description 257
ending of 262
entry to 260
sample hierarchy of panels 263
specifying an index page 262
use 261
tutorial panels, system variables that contain information
about 371
TWOBYTE built-in function on assignment statement 206
TYPE keyword in panel )ATTR section 145
U
unavail specification in panel )ATTR section 163
underscore, specifying 154
UP tutorial scroll command 261
UPPER built-in function on assignment statement 204
UPPERENG keyword on ISPSTART command 9, 15
USCORE parameter used with HILITE keyword 154
used for communication between dialogs and ISPF 59
user exit for panel processing 214
user interface
ISPF 373
TSO/E 373
USER parameter used with PAD keyword 157
user-selection 116
userid, system variable 365
USERMOD parameter in )ATTR section 148
V
validation of DBCS data 97
value from scroll amount field, system variable 369
variable model lines 118
variable services
creating or deleting defined variables 55
summary 60
writing dialogs 52
variables
variables (continued)
assignment statement 200
COMPOUND 6
creating implicit 56
description of 6
dialog 53
dialog, format 59
in IF or ELSE statements 210
in message definition 273
in VER statements 230
maximum size 6
names too long for panel definition 256
naming 6
naming defined and implicit 56
on panels, restricted size 95
owned by ISPF 58
panel REXX 223
processing using panel user exit 216
read-only extension 58
removing from the shared or profile pool 57
saving across ISPF sessions 57
sharing among dialogs 57
STEM 6
storing from a panel to shared and profile pools (VPUT)
243
system variable charts 359
testing the value of 210
to function pool from shared or profile pools (VGET) 241
value test during panel processing 212
ZERRCSID 299
ZKEYHELP 83
ZTERMCID 299
ZTERMCP 299
ZTERMCS 299
Variables for ISPSTART parameters 10
variables, syntax diagrams xvi
VARS variable in table display panel 119
VCOPY service 60
VDEFINE service
writing dialogs 60
VDELETE service 60
VEDIT statement 229
VER statement in panel section
description 230
syntax 232
VERASE service 60
verifying variable content 232
VGET statement
in panel )INIT, )REINIT, or )PROC section 241
on DISPLAY panel 241
on SELECT panel 243
syntax 241
using 60
VIEW service 76
VIIF service 76
VMASK service 60
VPUT statement
example 244
in panel )INIT, )REINIT, or )PROC section 243
syntax 243
using 60
VREPLACE service 60
VRESET service 60
VSYM
Index  391

## Page 420

VSYM (continued)
statement 244
VSYM built-in function on assignment statement
example, nested 202, 203
VSYM statement
example 244
syntax 244
W
WAIT parameter on SHRPROF command 18
WIDTH keyword in panel )ATTR section 163
WIDTH keyword in panel )BODY section 170
WINDOW command 81
WINDOW keyword
defining pop-up windows 94
in panel )BODY section 170
window title variable 79, 80
writing dialogs
display services 37
file-tailoring services 73
miscellaneous services 77
PDF services 76
table services 61
variable services 52
Z
Z system variable 361
Z variables used for field name place-holders 256
ZACCTNUM system variable 361
ZAMT system variable 368
ZAPLCNT system variable 361
ZAPPLID system variable 361
ZAPPTTL system variable 361
ZASPECT system variable 370
ZBDMAX system variable
and BDISPMAX keyword 35
ZBDMXCNT system variable 361
ZC system variable 275, 297
ZCFGCMPD system variable 359
ZCFGCMPT system variable 359
ZCFGKSRC system variable 360
ZCFGLVL system variable 360
ZCFGMOD system variable 360
ZCLIENT system variable 361
ZCLRSFLD primary command 180
ZCMD 349
ZCMD system variable
example 100
on tutorial panels 262
processing
blank 101
invalid option 101
truncation 99
versus other names for command field 90
ZCOLORS system variable
in batch mode 34
ZCONT system variable 262, 264, 371
ZCS system variable 361
ZCSDLL system variable 361
ZCUNIT 356
ZCURFLD
ZCURFLD (continued)
general description 194
ZCURFLD system variable 372
ZCURINX
general description 194
ZCURINX system variable 372
ZCURPOS
general description 194
ZCURPOS system variable 372
ZCUSIZE 356
ZDATE system variable 360
ZDATEF system variable 360
ZDATEFD system variable 360
ZDATESTD system variable 360
ZDAY system variable 360
ZDBCS system variable
in batch mode 34
ZDECS system variable 361
ZDEL system variable 361
ZDEVNAM system variable 370
ZDLBLKSZ 349
ZDLCATNM 349
ZDLCDATE 349
ZDLDEV 349
ZDLDSNTP 349
ZDLDSORG 349
ZDLEDATE 349
ZDLEXT 349
ZDLEXTX 349
ZDLLRECL 349
ZDLMIGR 349
ZDLMVOL 349
ZDLOVF 349
ZDLRDATE 349
ZDLRECFM 350
ZDLSIZE 350
ZDLSIZEX 350
ZDLSPACU 350
ZDLUSED 350
ZDLVOL 350
ZDSN 350
ZDST 350
ZDYNSCR system variable 369
ZE system variable 275, 297
ZEDBDSN 350
ZEDILMSG 350
ZEDISMSG 350
ZEDMSGNO 350
ZEDROW 350
ZEDSAVE 350
ZEDTDSN 350
ZEDTMCMD 350
ZEDTMEM 350
ZEDTRD 350
ZEDUSER 351
ZEIBSDN 351
ZEIROW 351
ZEITDSN 351
ZEIUSER 351
ZENVIR system variable 32, 361
ZERRALRM 351
ZERRALRM system variable 371
ZERRHM 351
ZERRHM system variable 371
392  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 421

ZERRLM 351
ZERRLM system variable 371
ZERRMSG 351
ZERRMSG system variable
for panel user exit messages 219
ZERRSM 351
ZERRSM system variable 371
ZERRTYPE system variable 371
ZERRWIND system variable 371
ZEURO system variable 361
ZEXPAND primary command 179
ZFAMPRT system variable 370
ZFKA system variable 366
ZGE system variable 125, 366
ZGEN 351
ZGENH 351
ZGRPLVL 351
ZGRPNME 351
ZGUI system variable 361
ZHIAUTO 351
ZHICOLOR 352
ZHICURSR 352
ZHIFIND 352
ZHILANG 352
ZHILITE system variable
in batch mode 34
ZHINDEX system variable
example 107
specifying top indexed panel 262
ZHIPAREN 352
ZHTOP system variable
example 107
specifying top tutorial panel 262
ZICFPRT 356
ZIND system variable
using on tutorial panels 262
ZIPADD6 system variable 366
ZIPADDR system variable 366
ZIPPORT system variable 366
ZISPFOS system variable 362
ZISPFRC system variable
description 21
example of using 23
return codes 362
ZJ4DATE system variable 360
ZJDATE system variable 360
ZKEYHELP system variable 83, 362
ZKEYS system variable 367
ZKLAPPL system variable 367
ZKLNAME system variable 367
ZKLTYPE system variable 367
ZKLUSE system variable 367
ZLAC 352
ZLALIAS 352
ZLAMODE 352
ZLANG system variable 362
ZLATTR 352
ZLC4DATE 353
ZLCDATE 353
ZLCNORC 353
ZLGENS 353
ZLGMAX 353
ZLGNEW 353
ZLGOLD 353
ZLGSAV 353
ZLINORC 353
ZLLIB 353
ZLM4DATE 354
ZLMACENT 354
ZLMDATE 354
ZLMEMBER 354
ZLMNORC 350, 354
ZLMOD 354, 356
ZLMSEC 354
ZLMTIME 354, 356
ZLOGNAME system variable 371
ZLOGO system variable 362
ZLOGON system variable 362
ZLPDSUDA 354
ZLRMODE 354
ZLSIZE 355
ZLSSI 354
ZLSTLPP system variable 370
ZLSTNAME system variable 371
ZLSTNUML system variable 370
ZLSTTRUN system variable 370
ZLTTR 355
ZLUNAME system variable 367
ZLUSER 355
ZLUSER8 355
ZLVERS 355
ZMLCOLS 355
ZMLCR 355
ZMLPS system variable 362
ZMLTR 355
ZMONTH system variable 360
ZMSRTFLD 355
ZNESTMAC system variable 362
ZPARENT system variable 102, 371
ZPDFREL 356
ZPF01-24 system variables 367
ZPFCTL system variable 367
ZPFFMT system variable 367
ZPFKEY system variable 362
ZPFSET system variable 367
ZPFSHOW system variable 367
ZPLACE system variable 362
ZPREFIX system variable 362
ZPRIKEYS system variable 367
ZPRIM system variable
example 102, 107
ignored in explicit chain mode 103
using 103
ZPROFAPP system variable 362
ZRXMSG system variable 223
ZRXMSGsystem variables
ZRXMSG 223
ZRXRC system variable 223
ZSCBR system variable 369
ZSCED system variable 369
ZSCLM 356
ZSCML system variable 369
ZSCRCUR system variable 362, 363
ZSCREEN system variable 368
ZSCREENC system variable 362
ZSCREEND system variable
in batch environment 34
ZSCREENI system variable 362
Index  393

## Page 422

ZSCREENW system variable
in batch environment 34
ZSCRMAX system variable 363
ZSCRMAXD system variable
in batch environment 34
panel definition 94
ZSCRMAXW system variable
in batch environment 34
panel definition 94
ZSCRML system variable 369
ZSCROLLA system variable 117, 369
ZSCROLLD system variable 117, 369
ZSCROLLN system variable 117, 369
ZSCROLNL system variable 117, 369
ZSCTPRE2 system variable 363
ZSCTPRE3 system variable 363
ZSCTPREF system variable 363
ZSCTSRCH system variable 363
ZSEL system variable
contains result of truncating ZCMD 98
example 100
on menus 98
on tutorial panels 262
parameters and keywords used with 99
restriction for 262
ZSEQ system variable 363
ZSESS 356
ZSM system variable 363
ZSPLIT system variable 368
ZSTDYEAR system variable 360
ZSWIND 357
ZSWPBR system variable 368
ZSYSICON system variable 363
ZSYSID system variable 363
ZSYSNODE system variable 364
ZSYSPLEX system variable 364
ZSYSPROC system variable 364
ZTDADD function variable
using 42
ZTDADD system variable 370
ZTDAMT function variable
using 42
ZTDAMT system variable 370
ZTDAMTL function variable
using 42
ZTDLROWS function variable
using 43
ZTDLROWS system variable 370
ZTDLTOP function variable
using 42, 43
ZTDLTOP system variable 370
ZTDMARK system variable 113, 370
ZTDMSG system variable 370
ZTDRET function variable
using 40
ZTDRET system variable 370
ZTDROWS system variable 370
ZTDSCRP function variable
using 42
ZTDSCRP system variable 370
ZTDSELS system variable
description 40
example 40
ZTDSIZE function variable
ZTDSIZE function variable (continued)
using 43
ZTDSIZE system variable 370
ZTDSRID function variable
using 42
ZTDSRID system variable 370
ZTDTOP system variable 370
ZTDVROWS system variable 370
ZTEMPF system variable 364
ZTEMPN system variable 364
ZTERM system variable 368
ZTERM, mapped to APL2 terminals 28
ZTERMCID system variable 364
ZTERMCP system variable 364
ZTERMCS system variable 364
ZTHS system variable 364
ZTIME system variable 360
ZTIMEL system variable 360
ZTS system variable 364
ZTSICMD system variable 364
ZTSSCMD system variable 365
ZUCTPRE2 system variable 365
ZUCTPRE3 system variable 365
ZUCTPREF system variable 365
ZUP system variable
on tutorial panels 262
ZUSC system variable 369
ZUSER system variable 365
ZUSERMAC 356
ZVERB system variable 117, 365
ZWINTTL 80
ZWINTTL system variable 365
ZWSCDPG system variable 365
ZWSCON system variable 365
ZWSOPSYS system variable 365
ZXSMAX system variable 369
ZXSMIN system variable 369
ZYEAR system variable 360
394  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 423



## Page 424

IBM®
Product Number: 5655-ZOS
SC19-3619-60
