# Appendix B. Accessibility

Source file: f54dt00_v3r1.md
Start page: 525
Page span: 525-558

## Page 525

Appendix B. Accessibility
Accessible publications for this product are offered through IBM Documentation for z/OS (www.ibm.com/
docs/en/zos).
If you experience difficulty with the accessibility of any z/OS documentation see How to Send Feedback to
IBM to leave documentation feedback.
© Copyright IBM Corp. 1989, 2024 493

## Page 526

494  z/OS: z/OS ISPF DTL Guide

## Page 527

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
© Copyright IBM Corp. 1989, 2024 495

## Page 528

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
496  z/OS: z/OS ISPF DTL Guide

## Page 529

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
Notices  497

## Page 530

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
498  z/OS: z/OS ISPF DTL Guide

## Page 531

Index
Special Characters
% keyword
on ENTITY statement 172
% notation 12
<: notation
in doctype declaration 15
A
AB (action bar) tag
conditions of usage 180
defining an action bar 33
description 180
examples
adding HELP attribute 36
application panel markup 181
defining help panel 36
markup 35
using mnemonic selection 37
how to code 34
syntax 179
See also action bar
ABBREV attribute
on MSG tag 354
ABC (action bar choice) tag
conditions of usage 183
description 183
examples
defining help panel 36
markup 35
PDCVAR attribute 183
using mnemonic selection 37
syntax 182
ABSEPCHAR attribute on AB tag 180
ABSEPSTR attribute on AB tag 180
ACC1 attribute on PDC tag 392
ACC2 attribute on PDC tag 392
ACC3 attribute on PDC tag 392
accessibility
contact IBM 493
ACTBAR attribute
on PANEL tag 382
ACTBAR option 160
ACTION (action) tag
conditions of usage 188
description 188
examples
defining help panel 36
markup 35
PDC tags and SETVAR attribute 189
using mnemonic selection 37
for pull-down choice 35
syntax 184
ACTION attribute
on CMDACT tag 235
on KEYL tag 323
ACTION attribute (continued)
value ALIAS on CMDACT tag 235
value application-command on CMDACT tag 236
value BACKWARD on CMDACT tag 236
value CANCEL on CMDACT tag 236
value EXHELP on CMDACT tag 236
value EXIT on CMDACT tag 236
value FKA on CMDACT tag 236
value FORWARD on CMDACT tag 236
value HELP on CMDACT tag 236
value PANELID on CMDACT tag 236
value PASSTHRU on CMDACT tag 236
value RETRIEVE on CMDACT tag 236
value SELECT on CMDACT tag 235
value SETVERB on CMDACT tag 236
action bar
defining 33
description 5
providing help for 35
See also AB (action bar) tag
action bar (AB) tag
conditions of usage 180
description 180
examples 181
syntax 179
action bar choice 36
See also ABC (action bar choice) tag
action bar choice (ABC) tag
conditions of usage 183
description 183
examples 183
syntax 182
action message
defining 138
description 138
ADDPOP attribute on ACTION tag 187
ALARM attribute
on MSG tag 354
alerting users
using the ATTENTION tag 121
using the CAUTION tag 121
using the NT tag 121
using the WARNING tag 121
ALIAS command action
defining 145
description 145
ALIGN attribute
on CHOFLD tag 222
on DTAFLD tag 279
on LSTCOL tag 335
on LSTGRP tag 345
on REGION tag 406
using 77
ALPHA test
description 61
example 61
specifying on CHECKI tag 212
Index  499

## Page 532

ALPHAB test
description 65
example 65
specifying on CHECKI tag 215
alphabetic test
description 61
example 61
ALTDESCR attribute on CMD tag 233
APPLCMD attribute on ACTION tag 186
application command table
defining commands 143
defining with CMDTBL tag 143
overview 8
See also commands
application panel
action bar 5
bottom instruction 6
command area 6
defining a command area 48
defining a panel ID 28
defining a region 45
defining action bar and pull-downs 33
defining an area divider 44
defining common attributes and values 50
defining common elements 27
defining cursor placement 30
defining panel width and depth 29
defining the panel title 29
defining top and bottom instructions 37
defining with the PANEL tag 27
description of PANEL tag 383
examples of PANEL tag 384
function key area 6
layout 5
overview 5
panel body 6
panel title 5
providing help for 29
specifying a key mapping list 29
syntax for PANEL tag 376
top instruction 6
application-id
ISPx 143
option 158
using with CMDTBL tag 143
APPLID attribute
description 143
on CMDTBL tag 244
on HELP TAG 306
on HELPDEF TAG 312
on KEYL TAG 323
on PANDEF TAG 374
on PANEL TAG 380
APPTITLE attribute
on HELP TAG 306
on HELPDEF TAG 312
on PANDEF tag 374
on PANEL tag 381
AREA (area) tag
conditions of usage 192
defining panel text portion 38
description 191
examples 38, 193
syntax 189
area divider (DIVIDER) tag
conditions of usage 260
description 260
examples 260
syntax 258
area divider (DLDIV) tag
conditions of usage 266
description 266
examples 266
ASIS attribute
on PANEL tag 382
ASSIGNI (assignment list item) tag
conditions of usage 196
description 196
examples 196
syntax 195
ASSIGNL (assignment list) tag
conditions of usage 197
description 197
examples 197
syntax 196
assignment list (ASSIGNL) tag
conditions of usage 197
description 197
examples 197
syntax 196
assignment list item (ASSIGNI) tag
conditions of usage 196
description 196
examples 196
syntax 195
assistive technologies 493
ATTENTION (attention) tag
conditions of usage 198
description 198
syntax 198
attention message
description
format 124
ATTENTION tag
description 124
ATTN attribute
on ATTR tag 203
ATTR (attribute) tag
conditions of usage 203
description 203
examples 203
syntax 200
ATTRCHANGE attribute
description
data field 78
list field 94
on CHOFLD tag 223
on DTACOL tag 272
on DTAFLD tag 280
on LSTCOL tag 336
on LSTFLD tag 342
using 78, 94
ATTRCHAR attribute
description
data field 79
on ATTR tag 202
on DTAFLD tag 281
using 79
500  z/OS: z/OS ISPF DTL Guide

## Page 533

attribute
description 11
tag 11
attribute (ATTR) tag
conditions of usage 203
description 203
examples 203
syntax 200
attributes and values, coding 11
ATTRUSE attribute
on PANEL tag 381
AUTODMEM attribute
description
data field 79
on DTAFLD tag 282
using 79
AUTONRET attribute
on PANEL tag 383
AUTOSEL attribute
on CHOICE tag 230
AUTOTAB 87
AUTOTAB attribute
description 89, 94
on CHOFLD tag 222
on CHOICE tag 229
on CMDAREA tag 240
on DTACOL tag 272
on DTAFLD tag 278
on LSTCOL tag 335
on SELFLD tag 425
using 94
AUTOTCMD attribute
on PANEL tag 383
AUTOTYPE attribute
description
data field 79
on DTAFLD tag 281
using 79
AUTOVOL attribute
description
data field 79
on DTAFLD tag 281
using 79
B
BARRIER attribute on ACTION tag 187
BIT test
description 67
example 67
specifying on CHECKI tag 214
BMARGIN attribute
on PANDEF TAG 375
on PANEL TAG 382
BOTINST (bottom instruction) tag
conditions of usage 204
defining instruction text 37
description 204
examples 38, 205
syntax 204
bottom instruction
defining 37
description 6
bottom instruction (BOTINST) tag
bottom instruction (BOTINST) tag (continued)
conditions of usage 204
description 204
examples 205
syntax 204
BREAK attribute
on DL tag 262
on PARML tag 386
BREAK=ALL attribute in definition list 116
BREAK=FIT attribute in definition list 117
C
CAPS attribute
description
data field 79
input-only 94
input/output 94
on ATTR tag 202
on CMDAREA tag 241
on DTACOL tag 272
on DTAFLD tag 281
on LSTCOL tag 337
using 79
CASE attribute on KEYI tag 320
CAUTION (caution) tag
conditions of usage 206
description 125, 206
example 125
examples 207
syntax 205
CCSID attribute
on HELP tag 306, 357
on HELPDEF tag 312
on PANDEF tag 374
on PANEL tag 380
CDATA keyword
on ENTITY statement 172
character variables 54
CHARS test
specifying on CHECKI tag 212
CHDIV (choice divider) tag
conditions of usage 208
description 208
examples
solid and blank 208
syntax 207
check list
example 139
specifying message for 139
CHECKI (validity check item) tag
conditions of usage 217
description 217
examples 217
syntax 210
checking values within a numeric range, range test 61
CHECKL (validity check list) tag
conditions of usage 218
description 218
examples 218
syntax 218
using MSG attribute 139
CHECKVAR attribute
description 36
Index  501

## Page 534

CHECKVAR attribute (continued)
example 37
on CHOICE tag 228
on PDC tag 391
CHKBOX attribute
description 88
on SELFLD tag 427
using 88
CHOFLD (choice data field) tag
conditions of usage 224
description 224
examples 224
syntax 219
CHOFLD (choice field) tag
using MSG attribute 139
choice 80, 85
See also selection field
CHOICE (selection choice) tag
conditions of usage 231
defining a selection field 80
description 230
examples 231
syntax 226
choice data field (CHOFLD) tag
conditions of usage 224
description 224
examples 224
syntax 219
choice divider (CHDIV) tag
conditions of usage 208
description 208
examples 208
syntax 207
CHOICECOLS attribute
description 87
on SELFLD tag 426
using 87
CHOICEDEPTH attribute
description 87
on SELFLD tag 426
using 87
CKBOX attribute
on ATTR tag 203
CLEAR attribute
description 94
on LSTCOL tag 335
CMD (command definition) tag
assigning function key 147
conditions of usage 234
description 233
examples 234
syntax 233
using to define a command 143
CMD attribute on KEYI tag 320
CMDACT (command action) tag
conditions of usage 237
description 237
examples 237
syntax 234
CMDAREA (command area) tag
conditions of usage 242
defining a command area 48
description 242
examples
CMDAREA (command area) tag (continued)
examples (continued)
defining 48
in an application panel markup 242
specifying the command prompt text 49
syntax 237
CMDLEN attribute
on CMDAREA tag 240
CMDLINE attribute
on PANEL tag 381
CMDLOC attribute
on CMDAREA tag 240
CMDTBL (command table) tag
assigning function key 147
conditions of usage 244
description 244
examples 245
syntax 243
using to define a command table 143
coding an action bar definition 34
COLOR attribute
description
data field 79
on ATTR tag 202
on DTAFLD tag 281
on HP tag 316
on LSTCOL tag 337
on NOTE tag 360
on NOTEL tag 363
on NT tag 366
on SELFLD tag 428
using 79, 88
COLSPACE attribute
description 94
on LSTCOL tag 336
COLTYPE attribute
description 94
on LSTCOL tag 336
column data 89
column, defining widths 89, 94
COLWIDTH attribute on LSTCOL tag 334
command 143
See also commands
command action (CMDACT) tag
conditions of usage 237
description 237
examples 237
syntax 234
using to define a command action 144
command area
defining 48
defining cursor placement 30
description 6
command area (CMDAREA) tag
conditions of usage 242
description 242
examples 242
syntax 237
command definition (CMD) tag
conditions of usage 234
description 233
examples 234
syntax 233
command table 143
502  z/OS: z/OS ISPF DTL Guide

## Page 535

See also application command table
command table (CMDTBL) tag
conditions of usage 244
description 244
examples 245
syntax 243
command-prompt-text attribute
on CMDAREA tag 241
commands
application command table 143
declaring variables for 53
defining a command area 48
defining application command table 143
defining with CMDTBL tag 143
description of application command table 8
reading syntax diagrams xxi
specifying command action 144
specifying with the RUN attribute 35
truncating 145
comment (COMMENT) tag
examples 246
syntax 245
COMMENT (comment) tag
examples 246
syntax 245
comment delimiter
description 16
example 16
using 16
comments
examples 16
including in generated panel 15
including in source files 16
COMPACT attribute
on BOTINST tag 204
on DL tag 262
on GRPHDR tag 301
on NOTEL tag 361
on OL tag 368
on P tag 370
on PARML tag 387
on PNLINST tag 396
on SL tag 434
on TOPINST tag 442
on UL tag 444
using in a note list 361
using in a simple list 109
using in an ordered list 113
using in an unordered list 111
compact for simple list 109
compact lists
note 108
ordered 113
simple 109
unordered 111
compiler options (COMPOPT) tag
syntax 247
COMPOPT (compiler options) tag
examples 249
syntax 247
compopt (COMPOPT) tag
examples 249
considerations, compatibility for AREA tag 192
contact
contact (continued)
z/OS 493
conversion utility
ACTBAR option 160
converting multiple source files 156
CUAATTR option 159
CUASUPP option 159
DBALIGN option 162
DBCS option 158
DISK option 158
DISPLAY option 160
DISPLAYW option 161
DSNCHK option 161
FORMAT option 160
GRAPHIC option 161
GUI option 160
help 156
installing 166
invocation panel input fields 152
invocation panel options 153
invocation panels 151
KANA option 158
KEYAPPL option 158
LISTING option 159
LISTREPL option 160
LOGREPL option 160
LSTVIEW option 159
MCOMMENT option 162
MERGESAREA option 160
messages 165
MSGEXPAND option 160
MSGSUPP option 159
NOACTBAR option 160
NOCUAATTR option 159
NOCUASUPP option 159
NODBALIGN option 162
NODBCS option 158
NODISPLAY option 160
NODISPLAYW option 161
NODSNCHK option 161
NOFORMAT option 160
NOGRAPHIC option 161
NOGUI option 160
NOKANA option 158
NOLISTING option 159
NOLISTREPL option 160
NOLOGREPL option 160
NOLSTVIEW option 159
NOMCOMMENT option 162
NOMERGESAREA option 160
NOMSGEXPAND option 160
NOMSGSUPP option 159
NOPANEL option 159
NOPLEB option 162
NOPREP option 159
NOREPLACE option 158
NOSCRIPT option 159
NOSTATS option 159
NOV3PADC option 162
NOVERSION option 160
NOZVARS option 161
overview 9
PANEL option 159
PLEB option 162
Index  503

## Page 536

conversion utility (continued)
PREP option 159
PROFDDN option 162
PROFILE option 162
REPLACE option 158
SCREEN option 158
SCRIPT option 159
STATS option 159
supporting keys help 148
syntax 156
using 151
V3PADC option 162
VERSION option 160
ZVARS option 161
converting DTL source files 151
COPYR (copyright) tag
examples 250
syntax 249
copyright
including in generated panel 15
copyright (COPYR) tag
examples 250
copyright (COPYRIGHT) tag
syntax 249
CSRGRP attribute
description
data field 78
on ATTR tag 203
on DTAFLD tag 280
on LSTCOL tag 336
on PS tag 399
on SELFLD tag 427
using 78, 88
CSRINDEX attribute
example 31
on PANEL tag 380
CSRPOS attribute
example 31
on PANEL tag 380
CUAATTR option 159
CUADYN attribute
on ATTR tag 203
CUASUPP option 159
CURSOR attribute
example 30
on PANEL tag 380
cursor field
on PANEL tag 380
cursor placement
ABC 30
characteristics 30
CHOICE 30
DTAFLD 30
in command area 30
in data field 30
in list field 30
in selection field 30
LSTCOL 30
SELFLD 30
CWIDTHS attribute
CWIDTHS attribute on SELFLD tag 426
description 87
using 87
D
DA (dynamic area) tag
conditions of usage 254
description 254
examples 255
syntax 250
using 43
data column
defining 89
example 89
data column (DTACOL) tag
conditions of usage 273
description 89, 273
examples 274
syntax 269
data field
attributes 77
defining a field prompt 71
defining alignment of data 77
defining an associated message 77, 87
defining cursor placement 30
defining data columns 89
defining help for 76
defining input/output 74
defining width 75
examples 74
providing descriptive text 75
tailoring 77, 86
data field (DTAFLD) tag
conditions of usage 282
description 282
examples 283
syntax 275
data field description (DTAFLDD) tag
conditions of usage 284
description 75, 284
example 75
examples 285
syntax 284
data set names
default used by conversion utility 167
DATAMOD attribute
on DA tag 252
DATAVAR attribute
description 53
on CHOFLD tag 221
on DTAFLD tag 277
on LSTCOL tag 334
on LSTVAR tag 348
DBALIGN attribute
description
data field 78
on DTAFLD tag 280
on SELFLD tag 428
using 78, 88
DBALIGN option 162
DBCS
option 158
restrictions for leading and trailing blanks 13
DBCS test
specifying on CHECKI tag 214
test
description 64
504  z/OS: z/OS ISPF DTL Guide

## Page 537

DBCS test (continued)
test (continued)
example 64
DD (definition description) tag
conditions of usage 256
description 114, 255
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
help panel markup 256
syntax 255
DDHD (definition description header) tag
conditions of usage 257
description 114, 257
examples 117, 258
syntax 257
declaring variables 53
default keylist, key mappings 147
defining help for key list 148
definition description (DD) tag
conditions of usage 256
description 255
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
help panel markup 256
syntax 255
definition description header (DDHD) tag
conditions of usage 257
description 257
examples 258
syntax 257
definition list (DL) tag
conditions of usage 263
description 263
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
default BREAK value of NONE 263
syntax 261
using to define a definition list 114
definition term (DT) tag
conditions of usage 268
description 268
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
help panel markup 268
syntax 267
definition term divider (DTDIV) tag
conditions 286
description 286
syntax 286
definition term header (DTHD) tag
conditions of usage 287
description 287
examples 288
syntax 287
definition term header divider (DTHDIV) tag
conditions 289
definition term header divider (DTHDIV) tag (continued)
description 289
syntax 288
definition term segment (DTSEG) tag
conditions 290
syntax 290
delimiter symbol 11
depth and width, defining with PANDEF tag 50
DEPTH attribute
defining application panel depth 29
description
data field 78
on AREA tag 190
on DA tag 252
on DTAFLD tag 280
on GA tag 296
on HELP tag 306
on HELPDEF tag 312
on PANDEF tag 374
on PANEL tag 379
on PS tag 399
on REGION tag 406
on SELFLD tag 425
using 78, 87
DESSKIP attribute
description
data field 79
on DTAFLD tag 281
using 79
DESTVAR attribute, on ASSIGNL tag 197
DESWIDTH attribute
description 89
on DTACOL tag 271
on DTAFLD tag 278
dialog element
creating 11
help panel 7
dialog elements
application command table 8
description 5
description of application panel 5
description of help panel 6
key mapping list 8
messages 7
types 5
variable classes 8
variables 8
Dialog Tag Language (DTL)
advantages 3
coding attributes and values 11
coding tag text 12
comments 15, 16
copyright statements 15
delimiters 11
document type declaration 15
entities 17
introduction 3
nesting tags 14
parameter entities 21
predefined symbols 23
relationship to CUA 3
results of converting 166
return codes 166
similarity to HTML 4
Index  505

## Page 538

Dialog Tag Language (DTL) (continued)
source file 11
syntax conventions 11
using 11
dialog variable 53
See also variables
DIR attribute
on AREA tag 191
on REGION tag 406
DISK option 158
DISP attribute
on MSG tag 354
DISPLAY attribute
description 77
input-only 94
input/output 94
on CHOFLD tag 222
on DTAFLD tag 279
on LSTCOL tag 337
using 77
DISPLAY option 160
DISPLAYW option 161
DISPLEN attribute
on SCRFLD tag 414
DIV attribute
on AREA tag 191
on DA tag 253
on GA tag 297
on GRPHDR tag 301
on LSTFLD tag 342
DIVEND attribute
on DL tag 262
on PARML tag 387
DIVIDER (area divider) tag
conditions of usage 260
defining a divider 44
description 260
examples
solid and blank 47, 260
TYPE attribute 44
within a horizontal REGION tag 47
within vertical REGION tag 45
syntax 258
within a REGION tag 47
DIVLOC attribute
on GRPHDR tag 301
DIVWIDTH attribute
on AREA tag 191
DL (definition list) tag
conditions of usage 263
description 114, 263
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
default BREAK value of NONE 263
syntax 261
using to define a definition list 114
DLDIV (area divider) tag
conditions of usage 266
description 266
examples
solid and blank 266
DLDIV tag
DLDIV tag (continued)
syntax 265
DM application, providing help for 130
DOCTYPE statement
declaring document type 15
description 171
document type declaration 15
parameters 171
syntax 171
document type declaration
description 171
types supported by DTL 15
double-byte characters, permitting usage through variables
54
DSNAME test
description 63
example 63
specifying on CHECKI tag 215
DSNAMEF test
description 63
example 63
specifying on CHECKI tag 215
DSNAMEFM test
description 63
example 63
specifying on CHECKI tag 215
DSNAMEPQ test
description 63
example 63
specifying on CHECKI tag 216
DSNAMEQ test
description 64
example 64
specifying on CHECKI tag 216
DSNCHK option 161
DT (definition term) tag
conditions of usage 268
description 114, 268
examples
basic 114
BREAK=ALL attribute 116
BREAK=FIT attribute 117
help panel markup 268
syntax 267
DTACOL (data column) tag
conditions of usage 273
description 273
examples 274
syntax 269
DTAFLD (data field) tag
conditions of usage 282
description 282
examples 283
syntax 275
using 74
using MSG attribute 139
See also data field
DTAFLDD (data field description) tag
conditions of usage 284
description 284
examples 285
syntax 284
See also data field
DTDIV (definition term divider) tag
506  z/OS: z/OS ISPF DTL Guide

## Page 539

DTDIV (definition term divider) tag (continued)
conditions 286
description 286
syntax 286
DTHD (definition term header) tag
conditions of usage 287
description 114, 287
examples 117, 288
syntax 287
DTHDIV (definition term header divider) tag
conditions 289
description 289
syntax 288
DTL
macros 177
results of converting 166
return codes 166
source files, converting 151
DTSEG (definition term segment) tag
conditions 290
syntax 290
dynamic area (DA) tag
conditions of usage 254
description 254
examples 255
syntax 250
using 43
E
EBCDIC test
description 65
example 65
specifying on CHECKI tag 215
embedding source files 22
emphasizing panel text
description 126
HP (Highlighted phrase) 126
restriction 126
RP (reference phrase) 126
end tag delimiters 11
ENDATTR attribute
on PANEL tag 381
entity
defining 17
description 17
examples
changing text 18
declaring a different name 20
declaring a file 19
declaring name and text string 17
naming conventions 18
specifying a name in text 18
using a text string in source file 19
using to embed external files 22
naming conventions 18
parameter 21
predefined 23
using to embed external files 22
entity declarations
conditions of usage 173
description 172, 173
example 174
entity definitions
entity definitions (continued)
example 175
ENTITY keyword
on ENTITY statement 172
ENTITY statement
parameters 172
syntax 172
entity-name
on ENTITY statement 172
entity-text
on ENTITY statement 173
ENTKEYTEXT attribute
on PANDEF TAG 375
on PANEL TAG 382
ENTWIDTH attribute
description 86, 89
on CHOFLD tag 222
on CMDAREA tag 240
on DTACOL tag 271
on DTAFLD tag 278
on SELFLD tag 424
using 86
ENUM 67
ENUM test
specifying on CHECKI tag 215
error messages 165
ERRORCHECK attribute
on PANEL tag 382
example (XMP) tag
conditions of usage 461
description 461
examples 105, 461
syntax 460
using to define an example 105
EXPAND attribute
description
data field 78
on CHOFLD tag 223
on DTAFLD tag 280
on HELP TAG 306
on HELPDEF TAG 312
on PANDEF TAG 375
on PANEL tag 381
using 78
EXTEND attribute
description 87
on AREA tag 191
on DA tag 252
on GA tag 296
on REGION tag 406
on SELFLD tag 425
using 87
F
FCHOICE attribute
description 87
on SELFLD tag 425
using 87
field
defining 71
defining a data field message 77, 87
defining a list field message 95
defining data field 74
Index  507

## Page 540

field (continued)
defining data field columns 89
defining list field 91
defining multiple-choice selection fields 81
defining selection fields 79
defining single-choice fields 80
interactive 71
providing additional information 129
types 71
field prompt
attributes 71
defining 71
example 71
specifying width 73
field-level help
on LSTCOL tag 334
on SELFLD tag 423
FIG (figure) tag
conditions of usage 292
description 292
examples 106, 293
syntax 291
using to define a figure 106
FIGCAP (figure caption) tag
conditions of usage 294
description 294
examples 107, 295
syntax 294
figure (FIG) tag
conditions of usage 292
description 292
examples 106, 293
syntax 291
using to define a figure 106
figure caption (FIGCAP) tag
conditions of usage 294
description 294
examples 107, 295
syntax 294
FileID test 63
FILEID test
specifying on CHECKI tag 216
files for installing the product 166
filespec
on ENTITY statement 173
FKA attribute on KEYI tag 320
FLDSPACE attribute
description 77, 89
on CHOFLD tag 222
on DTACOL tag 271
on DTAFLD tag 278
using 77
FLDSPOS attribute
on SCRFLD tag 416
FLDTYPE attribute
description
data field 79
on DTAFLD tag 281
on SELFLD tag 428
using 79, 88
FLDWIDTH attribute
description
data field 78
on DTAFLD tag 280
FLDWIDTH attribute (continued)
using 78
FMTWIDTH attribute
on GRPHDR tag 301
FORMAT attribute
description 95
on AREA tag 191
on ATTR tag 202
on CHDIV tag 208
on DA tag 253
on DIVIDER tag 259, 266
on DL tag 262
on GRPHDR tag 301
on LSTCOL tag 335
on MSG tag 354
on PARML tag 387
on PLDIV tag 394
on XLATL tag 459
FORMAT option 160
formatting panel text
Asian rules 13
English rules 13
fragments, syntax diagrams xxi
FRAME attribute on FIG tag 292
function key area
defining 147
description 6
G
GA (graphic area) tag
conditions of usage 297
description 297
examples 297
syntax 295
using 43
GAP attribute
on DIVIDER tag 259
on DLDIV tag 265
on PLDIV tag 394
GE attribute
on ATTR tag 202
generate (GENERATE) tag
conditions of usage 298
description 298
examples 299
syntax 298
GENERATE (generate) tag
conditions of usage 298
description 298
syntax 298
GENERATE (generate)tag
examples 299
generated panel comments
comments 15
generated panel statements
copyright 15
graphic area (GA) tag
conditions of usage 297
description 297
examples 297
syntax 295
using 43
GRAPHIC option 161
508  z/OS: z/OS ISPF DTL Guide

## Page 541

group header (GRPHDR) tag
conditions of usage 302
description 301
examples 302
syntax 300
GRPBOX attribute on REGION tag 406
GRPBXMAT attribute on REGION tag 407
GRPBXVAR attribute on REGION tag 407
GRPHDR (group header) tag
additional attributes for 96
conditions of usage 302
description 301
description of attributes 96
examples 302
syntax 300
GRPWIDTH attribute on REGION tag 406
GUI option 160
GUTTER attribute
on CHDIV tag 208
on DIVIDER tag 259
on DLDIV tag 266
on PLDIV tag 394
H
heading
in the information region 104
levels 104
heading (Hn) tag
conditions of usage 314
description 313
examples 104, 314
syntax 313
using to define a heading 104
HEADLINE attribute
on GRPHDR tag 301
on LSTGRP tag 345
help
defining a help pull-down 36
defining for a data field 76
defining help panels 130
field-level 423
for action bar 35
for application panels 29
for selection choice 85
for selection field 85
for the conversion utility 156
HELP (help panel) tag
conditions of usage 309
description 307
examples 309
syntax 303
HELP attribute
on ABC tag 182
on CHOFLD tag 221
on CHOICE tag 228
on CMDAREA tag 239
on DA tag 254
on DTAFLD tag 277
on HELP tag 305
on HELPDEF tag 311
on KEYL tag 323
on LSTCOL tag 334
on MSG tag 353
HELP attribute (continued)
on PANDEF tag 374
on PANEL tag 378
on PDC tag 391
on RP tag 412
on SELFLD tag 423
help default (HELPDEF) tag
conditions of usage 313
description 312
examples 313
syntax 311
help panel
defining for a data field 76
defining help panel text 131
function key area 7
layout 6
overview 6
panel body 7
panel title 7
title 7
types of help 6
using HELP attribute of PANEL tag 29
help panel (HELP) tag
conditions of usage 309
description 307
examples 309
syntax 303
help panel tag, using 130
help panels
defining 130
defining areas and regions 130
in sequence 134
scrollable 132
using the INFO tag with 130
help pull-down, defining 36
HELP tag 130
HELPDEF (help default) tag
conditions of usage 313
description 312
examples 313
syntax 311
HELPDEF attribute on HELP tag 305
HEX test
description 68
example 68
specifying on CHECKI tag 216
HIDE attribute on CHOICE tag 229
HIDEX attribute on CHOICE tag 230
highlighted phrase (HP) tag
conditions of usage 316
description 316
examples 316
syntax 315
HILITE attribute
description
data field 79
on ATTR tag 202
on DTAFLD tag 281
on HP tag 316
on LSTCOL tag 337
on NOTE tag 360
on NOTEL tag 363
on NT tag 366
on SELFLD tag 428
Index  509

## Page 542

HILITE attribute (continued)
using 79, 88
Hn (heading) tag
conditions of usage 314
description 313
examples 104, 314
syntax 313
using to define a heading 104
horizontal region 46
HP (Highlighted phrase) tag
conditions of usage 316
description 126, 316
examples 126, 316
restriction 126
syntax 315
I
ID attribute
on HELPDEF tag 311
on PANDEF tag 374
ID panel 28
IDATE attribute
value IDATE on CHECKI tag 216
IDATE test
description 68
example 68
IMAPCOL attribute
on HELP tag 307
on HELPDEF tag 312
on PANDEF tag 375
on PANEL tag 382
IMAPNAME attribute
description
data field 78
on CHOFLD tag 223
on CMDAREA tag 241
on DTAFLD tag 280
on HELP tag 307
on HELPDEF tag 312
on PANDEF tag 375
on PANEL tag 382
on PS tag 399
using 78
IMAPNAMEP attribute
description
data field 78
on CMDAREA tag 241
on DTAFLD tag 280
on PS tag 399
using 78
IMAPROW attribute
on HELP tag 307
on HELPDEF tag 312
on PANDEF tag 375
on PANEL tag 382
immediate-action, for pull-down choice 35
INCLUDE test
description 68
example 68
specifying on CHECKI tag 216
INDENT attribute
on AREA tag 190
on DL tag 262
INDENT attribute (continued)
on GRPHDR tag 301
on OL tag 368
on P tag 370
on PARML tag 387
on REGION tag 406
on SL tag 434
on UL tag 444
index value
on PANEL tag 380
INDVAL attribute
on SCRFLD tag 415
INDVAR attribute
on SCRFLD tag 415
INFO (information region) tag
conditions of usage 318
description 318
examples 101, 319
syntax 317
using to define an information region 101
information message
defining 138
description 138
providing for fields 129
information region
defining 101
defining text 102
definition lists 114
examples (XMP tag) 105
figure captions 107
figures 106
headings 104
list part 120
ordered lists 112
paragraphs 102
parameter lists 118
providing information for fields 129
simple lists 109
tags for text 101
unformatted text 104
unordered lists 110
information region (INFO) tag
conditions of usage 318
description 318
examples 101, 319
syntax 317
using to define an information region 101
INIT attribute
description
data field 78
on CHOFLD tag 223
on DTAFLD tag 280
on SELFLD tag 427
using 78, 87
initialization syntax, source-filespec 158
input/output data field 74
installing the conversion utility 166
instruction text 38
instructions, top and bottom 37
INTENS attribute
description
data field 79
on ATTR tag 202
on DTAFLD tag 281
510  z/OS: z/OS ISPF DTL Guide

## Page 543

INTENS attribute (continued)
on HP tag 316
on LSTCOL tag 337
on NOTE tag 360
on NOTEL tag 363
on NT tag 366
on SELFLD tag 428
using 79, 88
INTENSE attribute
on HP tag 316
on P tag 370
IPADDR4
description 69
example 69
IPADDR4 attribute
value IPADDR4 on CHECKI tag 217
IPADDR4 test
specifying on CHECKI tag 217
ISP application identifier 143
ISPCMDTB 143, 244
ISPDTLC
overview 9
using ? 156
using command 151
ISPKYLST
description 148
key mappings list 148
using 148
ISPx application identifier 143
item translate list
description 56
example 57
ITIME attribute
value ITIME on CHECKI tag 217
ITIME test
description 69
example 69
J
JDATE attribute
value JDATE on CHECKI tag 216
JDATE test
description 69
example 69
JSTD attribute
value JSTD on CHECKI tag 216
JSTD test
description 69
example 69
JUST attribute
on ATTR tag 202
K
KANA option 158
KEY attribute on KEYI tag 320
key item (KEYI) tag
conditions of usage 321
description 321
examples 321
syntax 319
using to define a key item 147
key list, defining help 148
key mapping list
defining 147
defining with PANDEF tag 50
overview 8
using KEYLIST attribute of PANEL tag 29
key mapping list (KEYL) tag
conditions of usage 323
description 323
examples 324
syntax 322
KEYAPPL option 158
keyboard
navigation 493
PF keys 493
shortcut keys 493
KEYI (key item) tag
conditions of usage 321
description 147, 321
examples 321
syntax 319
using to define a key item 147
KEYL (key mapping list) tag
conditions of usage 323
description 147, 323
examples 324
syntax 322
using 147
KEYLIST attribute
examples 29
on HELP TAG 306
on HELPDEF TAG 312
on PANDEF tag 374
on PANEL tag 379
KEYLTYPE attribute
on HELP TAG 306
on HELPDEF tag 312
on PANDEF tag 374
on PANEL tag 379
keys
assigning actions 147
default keylist 147
defining 148
defining key mapping lists 147
displaying 148
See also function key area
keywords, syntax diagrams xxi
L
LANG attribute on ACTION tag 187
LCOLDISP attribute
on SCRFLD tag 416
LCOLIND attribute
on SCRFLD tag 416
LEN test
description 67
example 67
specifying on CHECKI tag 215
LI (list item) tag
conditions of usage 326
description 325
examples
basic unordered list 110
Index  511

## Page 544

LI (list item) tag (continued)
examples (continued)
help panel with unordered list 326
list part (LP) tag 120
nested unordered 111
nesting ordered list 112
note list 108
paragraph nested in a list 119
simple list 109
syntax 325
LINDVAL attribute
on SCRFLD tag 415
LINDVAR attribute
on SCRFLD tag 415
LINE attribute
description 95
on LSTCOL tag 335
on LSTVAR tag 348
LINES (lines) tag
conditions of usage 328
description 327
examples 105, 328
syntax 327
using to define unformatted text 104
list column
defining width 94
truncating 94
list column (LSTCOL) tag
conditions of usage 338
description 337
examples 338
syntax 332
list field
additional attributes for 94
auto-tab attribute 94
defining 91
defining alignment of data 94
defining an associated message 95
defining cursor placement 30
defining required input for 95
description 91
example 91
specifying help for 95
tailoring 94
list field (LSTFLD) tag
conditions of usage 343
description 343
examples 344
syntax 341
list group (LSTGRP) tag
conditions of usage 346
description 346
example 347
syntax 345
list item (LI) tag
conditions of usage 326
description 325
examples
basic 109
COMPACT attribute on simple list 109
compact ordered list 113
help panel with unordered list 326
syntax 325
list part (LP) tag
list part (LP) tag (continued)
conditions of usage 331
description 331
examples 120, 332
syntax 330
using to define a list part 120
list variable (LSTVAR) tag
conditions of usage 349
description 348
example 349
syntax 348
LISTDEPTH attribute
description 88
on SELFLD tag 428
using 88
LISTING option 159
LISTREF attribute
description 88
on SELFLD tag 427
using 88
LISTREPL option 160
lists
definition 114
list part 120
nesting lists within lists 120
note 108
ordered 112
parameter 118
simple 109
types 108
unordered 110
LISTTYPE attribute
description 88
on SELFLD tag 427
using 88
LISTV test
description 66
example 66
specifying on CHECKI tag 215
LISTVX test
description 66
example 66
specifying on CHECKI tag 215
LIT (literal) tag
conditions of usage 329
description 329
examples 330
syntax 329
literal (LIT) tag
conditions of usage 329
description 329
examples 330
syntax 329
LMSG attribute
on PANEL tag 382
LOCATION attribute
on MSG tag 354
on REGION tag 407
LOGREPL option 160
LP (list part) tag
conditions of usage 331
description 331
examples 120, 332
syntax 330
512  z/OS: z/OS ISPF DTL Guide

## Page 545

LP (list part) tag (continued)
using to define a list part 120
LSTCOL (list column) tag
additional attributes for 94
conditions of usage 338
description 91, 337
description of attributes 94
examples 338
syntax 332
using MSG attribute 139
LSTFLD (list field) tag
conditions of usage 343
description 91, 343
examples 344
syntax 341
LSTGRP (list group) tag
conditions of usage 346
description 91, 346
example 347
syntax 345
LSTVAR (list variable) tag
conditions of usage 349
description 91, 348
example 349
syntax 348
LSTVIEW option 159
LVLINE attribute
on DA tag 252
on GA tag 297
M
M (mnemonic) tag
conditions of usage 351
description 351
example 352
parameters 351
syntax 351
macros
DTL 177
MARGIND attribute on AREA tag 190
MARGINW attribute
example 39
on AREA tag 190
markup declarations
comments 16
defining entities and parameter entities 17
document type declaration 15
entity declarations 17
types supported by DTL 15
markup language
advantages 4
description 3
markup, coding 11
MATCH attribute
description 36
example 37
on CHOICE tag 229
on PDC tag 392
MCOMMENT option 162
MENU attribute
on PANEL tag 380
MERGESAREA attribute
on HELP TAG 306
MERGESAREA attribute (continued)
on HELPDEF TAG 312
on PANDEF TAG 375
on PANEL tag 382
MERGESAREA option 160
message
declaring variables for 53
defining 137
defining for a data field 77, 87
defining for a list field 95
example 8
for check list 139
member 137
specifying a variable in text 141
specifying type 138
types 137
message (MSG) tag
conditions of usage 355
description 355
examples
MSG SUFFIX attribute 137
syntax 352
using to define a message 137
message identifier 138
message member 137
message member (MSGMBR) tag
conditions of usage 357
description 357
examples
basic 137
defining a message member 357
specifying type 138
syntax 356
using to define a message member 137
messages
assigning for check list 139
assigning for data field 139
assigning for failing specified translation 139
assigning for failing validity check 139
assigning for list column 139
conversion utility 165
defining 137
description 137
error 165
overview 7
warning 165
MIX test
description 65
example 65
specifying on CHECKI tag 216
MIXC attribute on CMDACT tag 235
MNEMGEN attribute on AB tag 180
mnemonic (M) tag
conditions of usage 351
description 351
example 352
parameters 351
syntax 351
mnemonic choice selection
from pull-downs and action bars 36
support of 36
MODE attribute on ACTION tag 187
MSG (message) tag
conditions of usage 355
Index  513

## Page 546

MSG (message) tag (continued)
description 355
examples
MSG SUFFIX attribute 137
syntax 352
using to define a message 137
MSG attribute
on CHECKL tag 218
on DTAFLD tag 278
on LSTCOL tag 334
on SELFLD tag 425
on VARCLASS tag 447
on XLATL tag 459
using 77, 87
MSGEXPAND option 160
MSGLINE attribute
on HELP tag 307
on PANEL tag 381
MSGMBR (message member) tag
conditions of usage 357
description 357
examples
basic 137
defining a message member 357
specifying type 138
syntax 356
using to define a message member 137
MSGSUPP option 159
MSGTYPE attribute on MSG tag 353
multicultural support 4, 24
multiple-choice selection field
defining 81
discussion 81
example 81
MVS naming conventions 166
N
NAME attribute
identifying variables 53
on CHOICE tag 227
on CMD tag 233
on CMDAREA tag 240
on DA tag 252
on DTAFLD tag 277
on GA tag 296
on HELP tag 304
on KEYL tag 323
on MSGMBR tag 357
on PANEL tag 378
on SELFLD tag 423
on VARCLASS tag 446
on VARDCL tag 449
rules for variable names 179
using with the PANEL tag 28
NAME test
description 64
example 64
specifying on CHECKI tag 214
NAMEF test
description 64
example 64
specifying on CHECKI tag 214
naming conventions for MVS 166
National Language Support 4
See also multicultural support
navigation
keyboard 493
NEST attribute on ACTION tag 187
nesting
lists within lists 120
ordered lists 112, 113
simple list 109
tags within lists 119
unordered lists 111
NEWAPPL attribute on ACTION tag 187
NEWPOOL attribute on ACTION tag 187
NEWWINDOW attribute on ACTION tag 187
NLS 4
See also multicultural support
NOACTBAR option 160
NOCHECK attribute on ACTION tag 187
NOCUAATTR option 159
NOCUASUPP option 159
NODBALIGN option 162
NODBCS option 158
NODISPLAY option 160
NODISPLAYW option 161
NODSNCHK option 161
NOENDATTR attribute
description 78, 95
on CHOFLD tag 222
on DIVIDER tag 259
on DTAFLD tag 279
on LSTCOL tag 336
using 78
NOFORMAT option 160
NOGRAPHIC option 161
NOGUI option 160
NOINIT attribute
on CMDAREA tag 239
NOJUMP attribute
description
data field 79
on CMDAREA tag 241
on DTAFLD tag 281
using 79
NOKANA option 158
NOLISTING option 159
NOLISTREPL option 160
NOLOGREPL option 160
NOLSTVIEW option 159
NOMATCH attribute on CHOICE tag 229
NOMCOMMENT option 162
NOMERGESAREA option 160
NOMSGEXPAND option 160
NOMSGSUPP option 159
NOPANEL option 159
NOPLEB option 162
NOPREP option 159
NOREPLACE option 158
NOSCRIPT option 159
NOSEL attribute
description 88
on SELFLD tag 428
using 88
NOSKIP attribute
on DL tag 262
514  z/OS: z/OS ISPF DTL Guide

## Page 547

NOSKIP attribute (continued)
on FIG tag 292
on LINES tag 327
on NOTE tag 359
on NOTEL tag 362
on NT tag 365
on OL tag 368
on SL tag 434
on UL tag 444
on XMP tag 460
NOSTATS option 159
NOTE (note) tag
conditions of usage 360
description 360
examples 360
syntax 358
note (NT) tag
conditions of usage 366
description 366
examples 367
syntax 364
note list (NOTEL) tag
conditions of usage 363
description 363
examples 364
syntax 361
NOTE tag
description 121
example 122
NOTEL (note list) tag
conditions of usage 363
description 121, 363
examples 122, 364
syntax 361
NOV3PADC option 162
NOVERSION option 160
NOZVARS option 161
NT (note) tag
conditions of usage 366
description 121, 366
example 123
examples 367
syntax 364
NUM test
description 67
example 67
specifying on CHECKI tag 214
NUMERIC attribute
on ATTR tag 202
numeric variables
converting 56
description 56
uses 56
O
OFFSET attribute
on P tag 370
OL (ordered list) tag
conditions of usage 369
description 368
examples
COMPACT attribute 113
list part (LP) tag 120
OL (ordered list) tag (continued)
examples (continued)
nested with paragraph 369
nesting 112
paragraph nested in a list 119
syntax 367
using to define an ordered list 112
OPT attribute on ACTION tag 187
ordered list (OL) tag
conditions of usage 369
description 368
examples
COMPACT attribute 113
list part (LP) tag 120
nested with paragraph 369
nesting 112
paragraph nested in a list 119
syntax 367
using to define an ordered list 112
OUTLINE attribute
description
data field 78
DTACOL tag 89
LSTCOL tag 95
selection field 87
on ATTR tag 202
on CHOFLD tag 223
on CHOICE tag 229
on CMDAREA tag 239
on DTACOL tag 272
on DTAFLD tag 279
on LSTCOL tag 336
on PANDEF tag 375
on PANEL tag 381
on SELFLD tag 426
using 78, 87
output data field 74
overriding variable classes 70
P
P (paragraph) tag
conditions of usage 371
description 371
examples
basic 102
defining information region width 371
formatting of 103
nested in an ordered list 119
syntax 370
using to define a paragraph 102
PAD attribute
description
data field 78
DTACOL tag 89
LSTCOL tag 95
selection field 87
on ATTR tag 202
on CHOFLD tag 222
on CHOICE tag 229
on CMDAREA tag 239
on DTACOL tag 271
on DTAFLD tag 279
on LSTCOL tag 336
Index  515

## Page 548

PAD attribute (continued)
on PANDEF tag 374
on PANEL tag 381
on SELFLD tag 426
using 78, 87
PADC attribute
description
data field 78
DTACOL tag 89
LSTCOL tag 95
selection field 87
on ATTR tag 202
on CHOFLD tag 222
on CHOICE tag 229
on CMDAREA tag 239
on DTACOL tag 271
on DTAFLD tag 279
on LSTCOL tag 336
on PANDEF tag 375
on PANEL tag 381
on SELFLD tag 426
using 78, 87
PANDEF (panel default) tag
conditions of usage 375
defining panel defaults 50
description 375
examples
overriding a value 52
referring to default definitions 376
shared panel dimensions only 51
shared panel values 51
syntax 372
PANDEF attribute
on PANEL tag 378
panel
declaring variables for 53
defining fields 71
defining with the PANEL tag 27
PANEL (panel) tag
conditions of usage 383
defining a panel ID 28
defining an application panel 27
defining cursor placement 30
defining panel NAME value 28
defining the panel title 29
defining the panel width and depth 29
description 383
examples
CURSOR attribute 30
HELP attribute 30
KEYLIST attribute 29, 384
start and end tags 28
WIDTH and DEPTH attributes 29
specifying a key mapping list 29
specifying a KEYLIST attribute 29
specifying associated help panel 29
syntax 376
panel body 6
panel default (PANDEF) tag
conditions of usage 375
description 375
examples 376
syntax 372
panel defaults 50
panel instruction (PNLINST) tag
conditions of usage 396
description 396
examples 397
syntax 396
PANEL option 159
panel region, defining 45
panel title
defining 29
description 5
for help panels 7
panel-title-text attribute
on PANEL tag 383
panels
converting multiple 165
PANELSTMT attribute
on PANEL tag 382
paragraph (P) tag
conditions of usage 371
description 371
examples
basic 102
defining information region width 371
formatting of 103
nested in an ordered list 119
syntax 370
using to define a paragraph 102
parameter description (PD) tag
conditions of usage 389
description 389
examples 118, 389
syntax 388
parameter entity
description 21
examples 21
naming conventions 22
syntax 172
parameter list (PARML) tag
conditions of usage 387
description 387
examples 388
syntax 386
using to define a parameter list 118
parameter list divider (PLDIV) tag
syntax 394
parameter term (PT) tag
conditions of usage 401
description 401
examples 118, 402
syntax 400
parameter term divider (PTDIV) tag
conditions of usage 402
description 402
examples 403
syntax 402
parameter term segment (PTSEG) tag
conditions 404
syntax 404
PARM attribute
on ACTION tag 186
on KEYI tag 320
parm list divider (PLDIV) tag
conditions of usage 395
description 394
516  z/OS: z/OS ISPF DTL Guide

## Page 549

parm list divider (PLDIV) tag (continued)
examples 395
PARML (parameter list) tag
conditions of usage 387
description 387
examples 118, 388
syntax 386
using to define a parameter list 118
PAS attribute
description
data field 78
on ATTR tag 202
on CHOFLD tag 223
on DTAFLD tag 280
on LSTCOL tag 336
using 78
PASSLIB attribute on ACTION tag 187
PASSTHRU 145
PD (parameter description) tag
conditions of usage 389
description 389
examples 118, 389
syntax 388
PDC (pull-down choice) tag
conditions of usage 392
description 392
examples
basic 392
defining help panel 36
markup 35
using mnemonic selection 37
syntax 390
PDCVAR attribute on ABC tag 183
PDSEP (pull-down separator) tag
syntax 394
phrase-to-be-highlighted attribute
on HP tag 316
PICT test
description 65
example 65
specifying on CHECKI tag 214
PICTCN test
description 66
example 66
specifying on CHECKI tag 214
PLACE attribute
description
data field 79
on CMDAREA tag 241
on DTAFLD tag 280
on PS tag 399
using 79
PLDIV (parm list divider) tag
conditions of usage 395
description 394
examples
solid and blank 395
syntax 394
PLEB option 162
PMTFMT attribute
description
data field 78
on DTACOL tag 272
on DTAFLD tag 279
PMTFMT attribute (continued)
using 78
PMTLOC attribute
description 89
on CMDAREA tag 239
on DTAFLD tag 279
on SELFLD tag 423
using 71
PMTSKIP attribute
description
data field 79
on DTAFLD tag 281
on SELFLD tag 428
using 79, 88
PMTTEXT attribute
on CMDAREA tag 240
PMTWIDTH attribute
description 89
on DTACOL tag 271
on DTAFLD tag 278
on SELFLD tag 424
using 71
PNLINST (panel instruction) tag
conditions of usage 396
description 396
examples 397
syntax 396
point-and-shoot (PS) tag
conditions of usage 399
description 399
example 400
syntax 398
pop-up window, displaying messages on 137
POSITION attribute
description 95
on LSTCOL tag 335
position value
on PANEL tag 380
predefined entities 23
predetermined tag attributes 11
PREP option 159
preselected pull-down choice 36
PRIME attribute
on PANEL tag 380
PROFDDN option 162
PROFILE option 162
prompt 71
See also field prompt
prompt-width, specifying for data field 73
PS (point-and-shoot) tag
conditions of usage 399
description 399
example 400
syntax 398
PSBUTTON attribute
on CMDAREA tag 241
PSVAL attribute
description
data field 78
on CHOFLD tag 223
on CMDAREA tag 241
on DTAFLD tag 280
using 78
PSVAR attribute
Index  517

## Page 550

PSVAR attribute (continued)
description
data field 78
on CHOFLD tag 223
on CMDAREA tag 241
on DTAFLD tag 279
using 78
PT (parameter term) tag
conditions of usage 401
description 401
examples 118, 402
syntax 400
PTDIV (parameter term divider) tag
conditions of usage 402
description 402
examples 403
syntax 402
PTSEG (parameter term segment) tag
conditions 404
syntax 404
pull-down choice
(PDC) tag
conditions of usage 392
description 392
examples 392
syntax 390
actions 35
defining 33
example 36
preselected 36
providing help for 35
pull-down separator (PDSEP) tag
syntax 394
pull-down, defining 33
R
RANGE test
checking values within a numeric range 61
description 61
specifying on CHECKI tag 212
RCOLDISP attribute
on SCRFLD tag 416
RCOLIND attribute
on SCRFLD tag 416
reference phrase (RP) tag
conditions of usage 412
description 412
examples 412
syntax 411
REFRESH attribute
description 88
on SELFLD tag 427
using 88
REGION (region) tag
conditions of usage 407
defining a region 45
description 407
examples
DIR attribute 45
horizontal and vertical 47, 408
syntax 405
repeatable items, syntax diagrams xxi
REPLACE keyword
REPLACE keyword (continued)
on ENTITY statement 173
REPLACE option 158
REQUIRED attribute
description
data field 77
LSTCOL tag 95
selection field 86
on CHOFLD tag 221
on DTACOL tag 272
on DTAFLD tag 278
on LSTCOL tag 334
on SELFLD tag 425
using 77, 86
RESULT attribute on ASSIGNI tag 195
return codes, results of converting with DTL 166
RINDVAL attribute
on SCRFLD tag 415
RINDVAR attribute
on SCRFLD tag 415
risk (attention statement) 124
risk (warning statement) 124
ROWS attribute
on LSTFLD tag 342
RP (Reference phrase) tag
conditions of usage 412
description 127, 412
example 127
examples 412
restriction 127
syntax 411
rules
for variable names 179
formatting Asian panel text 13
formatting English panel text 13
RULES attribute
on LSTFLD tag 341
RUN attribute
example 144
on ACTION tag 186
specifying a command 35, 144
S
SCALE attribute
on SCRFLD tag 416
SCRCAPS attribute
on CMDAREA tag 241
on DA tag 253
on LSTFLD tag 342
SCREEN option 158
SCRFLD (scrollable field) tag
conditions of usage 418
description 417
examples 418
syntax 413
SCRIPT option 159
SCRNAME attribute on ACTION tag 187
SCROLL attribute
on DA tag 252
on SCRFLD tag 416
scrollable fields
defining 97
SCRFLD tag 413
518  z/OS: z/OS ISPF DTL Guide

## Page 551

SCROLLTAB attribute
on CMDAREA tag 240
on DA tag 253
on LSTFLD tag 342
SCROLLVAR attribute
on CMDAREA tag 240
on DA tag 253
on LSTFLD tag 342
SCRVHELP attribute
on CMDAREA tag 240
on DA tag 253
on LSTFLD tag 342
SELCHAR attribute on CHOICE tag 229
SELCHECK parameter 89
SELDEFAULT attribute
description 88
on SELFLD tag 428
using 88
selection choice (CHOICE) tag
conditions of usage 231
description 230
examples 231
syntax 226
selection choice, defining space for 85
selection field
attributes 86
defining 79
defining a field prompt 71
defining cursor placement 30
defining space for choice 85
help for 85
using the CHOICE tag 80
selection field (SELFLD) tag
conditions of usage 429
description 429
examples 430
syntax 421
selection list, defining cursor placement 31
selection width, defining 86
SELFLD (selection field) tag
conditions of usage 429
description 429
examples 430
syntax 421
using MSG attribute 139
SELFMT attribute
description 88
on SELFLD tag 427
using 88
SELMSG attribute
description 87
on SELFLD tag 426
using 87
SELMSGU attribute
description 87
on SELFLD tag 427
using 87
SELWIDTH attribute
defining space for 85
description 85, 89
on DTACOL tag 271
on SELFLD tag 424
using 85
SETVAR attribute on ACTION tag 187
SETVERB 145
SHADOW attribute
on DA tag 253
shortcut keys 493
simple list (SL) tag
conditions of usage 434
description 434
examples
basic 109
compact and nested 435
COMPACT attribute 109
syntax 433
using to define a simple list 109
SINDVAL attribute
on SCRFLD tag 415
SINDVAR attribute
on SCRFLD tag 415
single-choice selection field
defining 80
discussion 80
example 80
SKIP attribute
on ATTR tag 202
on PARML tag 387
SL (simple list) tag
conditions of usage 434
description 434
examples
basic 109
compact and nested 435
COMPACT attribute 109
syntax 433
using to define a simple list 109
SMSG attribute
on MSG tag 354
on PANEL tag 382
SORT attribute on CMDTBL tag 244
source (SOURCE) tag
example 437
SOURCE (source) tag
example 437
SOURCE (Source) tag
conditions of usage 436
description 436
syntax 435
source file
defining dialog elements 11
defining entities and parameter entities 17
DOCTYPE declaration 171
embedding 22
frequently used words (entities) 17
including comments 15, 16
including copyright statements 15
source-filespec, for system 158
SPACE attribute
on LI tag 325
on OL tag 368
on P tag 370
on SL tag 434
on UL tag 444
SPACE keyword
on ENTITY statement 173
specifying
a list of values to match value of user input 62
Index  519

## Page 552

specifying (continued)
a list of valuesx to match value of user input 62
calling the conversion utility 158
source-filespec 158
SPLIT attribute
on DL tag 262
on PARML tag 387
start tag delimiters 11
STATS option 159
STDDATE attribute
value STDDATE on CHECKI tag 216
STDDATE test
description 68
example 68
STDTIME attribute
value STDTIME on CHECKI tag 217
STDTIME test
description 69
example 69
STRIP attribute
on GRPHDR tag 301
SUBSTITUTE attribute
on GENERATE tag 298
SUFFIX attribute on MSG tag 353
summary of changes xxix
SUSPEND attribute on ACTION tag 187
syntax diagrams, how to read xxi
SYSTEM keyword
on ENTITY statement 173
T
T (truncation) tag
conditions of usage 438
description 437
examples 145, 438
syntax 437
using to define command truncation 145
tag
attributes 11
coding text 12
delimiters 11
description 11
nesting 14
rules for coding values 12
text 12
values 11
tags
AB 179
ABC 182
ACTION 184
ASSIGNI 195
ASSIGNL 196
ATTENTION 121, 198
ATTR 200
BOTINST 204
CAUTION 121
CHDIV 207
CHECKI 210
CHECKL 218
CHOFLD 219
CHOICE 226
CMD 233
CMDACT 234
tags (continued)
CMDAREA 237
CMDTBL 243
COMMENT 245
COMPOPT 247
COPYRIGHT 249
DA 250
DD 255
DDHD 257
DIVIDER 258
DL (definition list) 261
DLDIV 265
DT 267
DTACOL 269
DTAFLD 275
DTAFLDD 284
DTDIV 286
DTHD 287
DTHDIV 288
DTSEG 290
FIG 291
FIGCAP 294
for information region 101
GA 295
GENERATE 298
GRPHDR 300
HELP 303
HELPDEF 311
Hn 313
HP 315
INFO 317
KEYI 319
KEYL 322
LI 325
LINES 327
LIT 329
LP 330
LSTCOL 332
LSTFLD 341
LSTGRP 345
LSTVAR 348
M 351
mnemonic 351
MSG 352
MSGMBR 356
nesting 120
NOTE 358
NOTEL 361
NT 121, 364
OL 367
P 370
PANDEF 372
PANEL 376
PARML 386
PD 388
PDC 390
PDSEP 394
PLDIV 394
PNLINST 396
PS 398
PT 400
PTDIV 402
PTSEG 404
REGION 405
520  z/OS: z/OS ISPF DTL Guide

## Page 553

tags (continued)
RP 411
SCRFLD 413
SELFLD 421
SL 433
SOURCE 435
T 437
TEXTLINE 439
TEXTSEG 439
TOPINST 441
UL 443
VARCLASS 445
VARDCL 449
VARLIST 450
VARSUB 452
WARNING 121, 454
XLATI 456
XLATL 458
XMP 460
text
adding to a list 120
defining for information region 102
help 131
providing attention to user 124
providing caution 125
providing notes to user 121
providing warning to user 124
static 101
tag 12
tags used to define 101
using the example tag 105
using the figure tag 106
using the heading tags 104
using the LINES tag 104
using the paragraph tag 102
TEXT attribute
description 95
on AREA tag 191
on LSTCOL tag 335
on NOTE tag 360
on NOTEL tag 363
on NT tag 366
on OL tag 368
on SL tag 434
on UL tag 444
text line (TEXTLINE) tag
conditions of usage 439
description 439
examples 439
syntax 439
text segment (TEXTSEG) tag
conditions of usage 440
description 440
examples 440
syntax 439
TEXTFMT attribute
description 96
on LSTCOL tag 336
TEXTLEN attribute
description 96
on LSTCOL tag 336
TEXTLINE (text line) tag
conditions of usage 439
description 439
TEXTLINE (text line) tag (continued)
syntax 439
TEXTLINE (text line)tag
examples 439
TEXTLOC attribute
description 95
on LSTCOL tag 335
TEXTSEG (text segment) tag
conditions of usage 440
description 440
syntax 439
TEXTSEG(text segment)tag
examples 440
TEXTSKIP attribute
description 96
on LSTCOL tag 336
title, panel 29
TITLINE attribute
on PANEL tag 381
TMARGIN attribute
on PANDEF TAG 375
on PANEL TAG 382
TOGVAR attribute on ACTION tag 188
top instruction
defining 37
description 6
top instruction (TOPINST) tag
conditions of usage 442
description 442
examples 442
syntax 441
TOPINST (top instruction) tag
conditions of usage 442
defining instruction text 37
description 442
examples 38, 442
syntax 441
trademarks 498
TRAIL attribute
description 87
on SELFLD tag 426
using 87
translate item (XLATI) tag
conditions of usage 457
description 457
examples 457
syntax 456
translate list (XLATL) tag
conditions of usage 459
description 459
examples 459
syntax 458
translate lists
defining 56
types 56
translation
defining items for 56
example 57
TRUNC attribute
on CHOICE tag 230
on XLATL tag 459
truncation (T) tag
conditions of usage 438
description 437
Index  521

## Page 554

truncation (T) tag (continued)
examples 145, 438
syntax 437
using to define command truncation 145
TSIZE attribute
description 88
on DL tag 262
on PARML tag 386
on SELFLD tag 427
using 88
TUTOR attribute
on HELP tag 306
on PANEL tag 380
tutor-choice fields 84
TYPE attribute
description 79
on ACTION tag 187
on ATTR tag 202
on CHDIV tag 208
on CHECKI tag 212
on COMMENT tag 246
on DIVIDER tag 259
on DLDIV tag 265
on HP tag 316
on NOTE tag 359
on NOTEL tag 363
on NT tag 365
on PANEL tag 381
on PLDIV tag 394
on SELFLD tag 423
on SOURCE tag 436
on VARCLASS tag 446
using 79
value %varname on VARCLASS tag 447
value ALPHA on CHECKI tag 212
value ALPHAB on CHECKI tag 215
value ANY on VARCLASS tag 446
value BIT on CHECKI tag 214
value CHAR on VARCLASS tag 446
value CHARS on CHECKI tag 212
value DBCS on CHECKI tag 214
value DBCS on VARCLASS tag 446
value DSNAME on CHECKI tag 215
value DSNAMEF on CHECKI tag 215
value DSNAMEFM on CHECKI tag 215
value DSNAMEPQ on CHECKI tag 216
value DSNAMEQ on CHECKI tag 216
value EBCDIC on CHECKI tag 215
value EBCDIC on VARCLASS tag 447
value ENUM on CHECKI tag 215
value FILEID on CHECKI tag 216
value HEX on CHECKI tag 216
value IDATE on VARCLASS tag 447
value INCLUDE on CHECKI tag 216
value IPADDR4 on CHECKI tag 217
value ITIME on VARCLASS tag 447
value JDATE on VARCLASS tag 447
value JSTD on VARCLASS tag 447
value LEN on CHECKI tag 215
value LISTV on CHECKI tag 215
value LISTVX on CHECKI tag 215
value MIX on CHECKI tag 216
value MIXED on VARCLASS tag 446
value NAME on CHECKI tag 214
TYPE attribute (continued)
value NAMEF on CHECKI tag 214
value NUM on CHECKI tag 214
value NUMERIC on VARCLASS tag 447
value PICT on CHECKI tag 214
value PICTCN on CHECKI tag 214
value RANGE on CHECKI tag 212
value STDDATE on VARCLASS tag 447
value STDTIME on VARCLASS tag 447
value VALUES on CHECKI tag 213
value VALUESX on CHECKI tag 213
value VMASK on VARCLASS tag 447
U
UL (unordered list) tag
conditions of usage 444
description 444
examples
basic 110
basic and nested 445
nested 111
syntax 443
using to define an unordered list 110
UNAVAIL attribute
on CHOICE tag 230
on PDC tag 391
UNAVAILMAT attribute on CHOICE tag 230
unformatted text
using the example tag 105
using the figure tag 106
using the LINES tag 104
unordered list (UL) tag
conditions of usage 444
description 444
examples
basic 110
basic and nested 445
nested 111
syntax 443
using to define an unordered list 110
upper translate list
example 56
translating a value to uppercase 56
USAGE attribute
description 91
on CHOFLD tag 221
on DTAFLD tag 277
on LSTCOL tag 334
user interface
ISPF 493
TSO/E 493
USERMOD attribute
on DA tag 252
V
V3PADC option 162
validity check item (CHECKI) tag
conditions of usage 217
description 217
examples 217
syntax 210
522  z/OS: z/OS ISPF DTL Guide

## Page 555

validity check list (CHECKL) tag
conditions of usage 218
description 218
examples 139, 218
syntax 218
validity checks
description 60
types 60
using 60
using CHECKL and CHECKI tags 60
value
description 11
tag 11
VALUE attribute
on ACTION tag 187
on ASSIGNI tag 195
on PS tag 399
on XLATI tag 456
VALUE1 attribute on ACTION tag 188
VALUE2 attribute on ACTION tag 188
VALUES test
description 62
example 62
specifying on CHECKI tag 213
VALUESX test
description 62
example 62
specifying on CHECKI tag 213
VAR attribute
on PS tag 399
on VARSUB tag 452
VARCLASS (variable class) tag
conditions of usage 448
description 448
examples 448
syntax 445
using MSG attribute 139
using to define a variable class 54
VARCLASS attribute
description 77, 89
input-only 96
input/output 96
on CHOFLD tag 221
on DTAFLD tag 277
on LSTCOL tag 334
on VARDCL tag 449
using 77
VARDCL (variable declaration) tag
conditions of usage 450
description 449
examples 53, 450
syntax 449
VARDCL attribute
description
data field 79
on CMDAREA tag 241
on DTACOL tag 272
on DTAFLD tag 282
on LSTCOL tag 337
on LSTFLD tag 343
on SELFLD tag 428
using 79
variable class
associating translate list 56
variable class (continued)
defining 54
defining character variables 54
defining translate list 458
description 53
overriding 70
overriding the data variable 96
overview 8
variable class (VARCLASS) tag
conditions of usage 448
description 448
examples 448
syntax 445
variable data
aligning 77, 94
description 94
variable declaration (VARDCL) tag
conditions of usage 450
description 449
examples 450
syntax 449
variable list
defining 53
variable list (VARLIST) tag
conditions of usage 451
description 451
examples 451
syntax 450
variable names
as a tag value, assigning 12
rules 179
rules for %varname as a tag value 179
variable substitution (VARSUB) tag
conditions of usage 453
description 453
examples 141, 453
syntax 452
using to substitute a variable 141
variable translate list
defining 56
relation to VARCLASS definition 56
types 56
variable validation 56
variables
character 54
declaring 53
defining a variable class 54
in data fields 74
item translating 57
numeric 56
overview 8
rules for naming 179
specifying in message text 141
translate list 56
translating 56, 57
validating 56
validity checks 60
variables, syntax diagrams xxi
VARLIST (variable list) tag
conditions of usage 451
description 451
examples 53, 451
syntax 450
using 53
Index  523

## Page 556

VARSUB (variable substitution) tag
conditions of usage 453
description 453
examples 453
syntax 452
VERIFY attribute
description 88
on SELFLD tag 427
using 88
VERSION option 160
vertical region 45
W
WARNING (warning) tag
conditions of usage 455
description 454
examples 199, 455
syntax 454
warning message
defining 138
description
format 124
information messages 138
ISPF conversion utility messages 165
example 124
WARNING tag
description 124
example 124
width and depth, defining with PANDEF tag 50
WIDTH attribute
defining application panel width 29
on AREA tag 191
on DA tag 253
on FIG tag 292
on GA tag 296
on GRPHDR tag 301
on HELP tag 305
on HELPDEF tag 311
on INFO TAG 318
on MSGMBR tag 357
on PANDEF tag 374
on PANEL tag 379
on REGION tag 406
window
layout 5
WINDOW attribute
on PANDEF tag 374
on PANEL tag 380
WINTITLE attribute
on HELP TAG 306
on HELPDEF TAG 312
on PANDEF tag 374
on PANEL tag 381
word-wrapping
data field width 75
on DESWIDTH attribute 75
X
XLATI (translate item) tag
conditions of usage 457
description 457
XLATI (translate item) tag (continued)
examples 457
syntax 456
XLATL (translate list) tag
conditions of usage 459
description 459
examples 459
syntax 458
using MSG attribute 139
XMP (example) tag
conditions of usage 461
description 461
examples 105, 461
syntax 460
using to define an example 105
Z
ZCMD system variable
for PASSTHRU 145
for SETVERB 145
ZCONT attribute
on PANEL tag 382
ZGUI attribute
description 88
on SELFLD tag 427
using 88
ZKEYHELP, using 148
ZUP attribute
on PANEL tag 382
ZVARS option 161
ZVERB system variable for SETVERB 145
524  z/OS: z/OS ISPF DTL Guide

## Page 557



## Page 558

IBM®
Product Number: 5655-ZOS
SC19-3620-60
