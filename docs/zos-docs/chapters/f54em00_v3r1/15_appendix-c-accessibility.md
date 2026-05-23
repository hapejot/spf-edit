# Appendix C. Accessibility

Source file: f54em00_v3r1.md
Start page: 465
Page span: 465-488

## Page 465

Appendix C. Accessibility
Accessible publications for this product are offered through IBM Documentation for z/OS (www.ibm.com/
docs/en/zos).
If you experience difficulty with the accessibility of any z/OS documentation see How to Send Feedback to
IBM to leave documentation feedback.
© Copyright IBM Corp. 1984, 2024 433

## Page 466

434  z/OS: z/OS ISPF Edit and Edit Macros

## Page 467

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
© Copyright IBM Corp. 1984, 2024 435

## Page 468

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
436  z/OS: z/OS ISPF Edit and Edit Macros

## Page 469

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
Notices  437

## Page 470

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
438  z/OS: z/OS ISPF Edit and Edit Macros

## Page 471

Index
Special Characters
! (exclamation point), for implicit edit macro 107
.ZCSR 59, 104
.ZDEST 104, 108
.ZFIRST 59, 104
.ZFRANGE 104, 108
.ZLAST 59, 104
.ZLRANGE 104, 108
( (column shift left), line command 138
) (column shift right), line command 140
& prefix for edit commands 13
&LASTCC variable 110
< (data shift left), line command 141
> (data shift right), line command 143
Numerics
3850 virtual volumes, accessing 6
A
A (after), line command 145
A operand, REXX TRACE statement 114
abbreviations for commands and other values 427
accessibility
contact IBM 433
ACCOUNT command 6
add a data set member 390
adding
a line 161, 355
edit macro command 87
models 72
adding data 269
after, line command 145
AK (after, multiple targets), line command 145
alias name, defining with edit macro 106
alias, assigning 220, 326
application-wide macros 25
ASCII
data, working with 51
linefeed character
LF macro command 350
LF primary command 51, 246
SOURCE primary command 51
ASCII, translating 282, 410
assignment statement
AUTOLIST 300
AUTONUM 301
AUTOSAVE 303
BLKSIZE 304
BOUNDS 305
CAPS 309
CHANGE COUNT 313
CURSOR 319
DATA_CHANGED 323
DATA_WIDTH 324
assignment statement (continued)
DATAID 325
DATASET 326
description 96
DISPLAY_COLS 330
DISPLAY_LINES 331
EXCLUDE_COUNTS 337
FIND_COUNTS 340
FLIP 341
FLOW_COUNTS 341
HEX 342
HIDE 239
how to use 98
IMACRO 348
LABEL 104, 351
LEVEL 353
LINE 354
LINE_AFTER 355
LINE_BEFORE 357
LINENUM 360
LRECL 364
MACRO_LEVEL 103, 366
MASKLINE 367
MEMBER 368
NOTES 373
NULLS 374
NUMBER 375
PACK 378
parentheses guidelines 98
PROFILE 383
RANGE_CMD 108, 384
RECFM 386
RECOVERY 387
reference section 295
RMACRO 109, 395
SCAN 95, 398
SEEK_COUNTS 402
STATS 411
summary 295
TABS 413
TABSLINE 415
USER_STATE 421
VERSION 422
XSTATUS 425
assistive technologies 433
attribute bytes, used with tabs 65
AUTOLIST
assignment statement 301
macro command 300
primary command 194
autolist mode
defined 19
querying the value 300
setting the value 194, 300
automatic generation of source listing 194, 300
automatic saving of data 197, 303
AUTONUM
Index  439

## Page 472

AUTONUM (continued)
assignment statement 301
macro command 301
primary command 19, 195
autonum mode 19
AUTOSAVE
assignment statement 303
macro command 303
primary command 19, 197
autosave mode, defined 19
B
B (before), line command 41, 148
batch processing, submitting data for 283, 412
batch processing, using edit macros in 102
batch, ending a macro 369
before, line command 148
beginning an edit session 4
BK (before, multiple targets), line command 148
BLKSIZE, assignment statement 304
block size, retrieving 304
boundaries
controlling 199, 305
default 24
definition line 24
setting 151
BOUNDS
assignment statement 305, 306
line command 151
macro command 305, 306
primary command 199
BROWSE
macro command 307
primary command 200
built-in command
disabling 220, 326
processing 201
built-in labels 59
BUILTIN
macro command 308
primary command 201
C
C (copy), line command
description 152
used with CREATE command 216
used with REPLACE command 271
CANCEL
macro command 308, 309
primary command 201
canceling edit changes 201, 308
CAPS
assignment statement 309, 310
DBCS data 203
macro command 309, 310
primary command 19, 202, 206
caps mode
defined 19
overview 20
querying the value 309
setting the value 202, 309
CHANGE
macro command
column-dependent data, defined 50
DBCS data 50
description 310, 312
EBCDIC data 50
RCHANGE command 385
saving and restoring values 421
primary command
column-dependent data, defined 50
DBCS data 50
description 44, 203, 205
EBCDIC data 50
qualifying search strings 54
specifying search strings 45
repeating 55
change a data string 310
CHANGE command, repeating 265
change count, retrieving 313
CHANGE_COUNTS, assignment statement 313
changed lines 22
changing a data string 203
changing data 44
changing models 75
character encoding 282, 410
character string
changing 203
finding 232, 338
how to use 46
specifying 45
characters
converting 202, 309
converting to lowercase 164
converting to uppercase 187
displaying hexadecimal 236, 342
clipboard, pasting lines from 261
CLIST CONTROL statements 114
CLIST edit macro statements 79, 87
CLIST WRITE statements 113
COBOL sequence field, defined 27
color in editor, changing 240
COLS
primary command 206
COLS, line command 155
column
shift left 138
shift right 140
column identification line, displaying 155
column limitations 54
column positions, referring to 106
column shifting
DBCS data 43
destructive 43
line command 43
columns
identifying 155
line command 155
query display 330
shift left 405
shift right 405
columns indicator line, displaying 206
command line 12, 191
command names, overriding 107
command procedure statements 88
440  z/OS: z/OS ISPF Edit and Edit Macros

## Page 473

command scan mode, setting the value 398
command, PROFILE RESET 21
command, querying 384
commands
reading syntax diagrams xxi
commands, reversing last edit 286
compare command 314
COMPARE command examples 210
compare command return codes 316
compare command syntax 314
COMPARE command syntax 207
Compare, edit command 314
COMPARE, edit command 207
compress data 378
compressing data 260
CONLIST operand, CLIST CONTROL statement 114
contact
z/OS 433
control and display your profile 383
control edit recovery 266, 387
control null spaces 374
control version number 290, 422
CONTROL, ISPEXEC statement 111
controlling and displaying profile 262
controlling null spaces 257
controlling the edit boundaries 199, 305
controlling the edit environment 17
controlling the search for a data string 52
convert characters to lowercase 164
converting characters 202, 309
converting note lines to data 170
COPY
macro command 317
primary command
description 210, 212
how to use 41
copy a model into the current data set 369
copying a model into data set 249
copying data
into the current data set 41
lines of data 152
macro command 317
primary command 210
using edit macro 99
CREATE
macro command 318, 319
primary command
description 215
how to use 41
creating
a data set member 215, 318
data 41
new data 7
current member name, querying 368
cursor position
querying the value 319
setting the value 319
cursor values, saving and restoring 421
CURSOR, assignment statement
positioning cursor on command line 320
Cut and Save Lines 322
Cut Macro command 322
CUT Primary command 218
cutting and saving lines 218
D
D (delete) line command 156
data
adding 269
canceling changes 201, 308
changing 44, 203, 310
column-dependent, defined 50
compressing 260, 378
controlling the string search 52
converting data 187
copying 41, 210, 317
copying lines 152
creating 41
creating new 7
DBCS considerations 50
deleting 222, 328
description 205
EBCDIC considerations 50
editing existing 8
excluding 44, 230, 335
finding 44, 232, 338
inserting 349
managing 41
moving 41
packing 15
realigning, LF primary command 246
replacing 41, 269
retrieving the changed status 323
retrieving the ID 325
retrieving the width 324
saving automatically 197, 303
saving the current 276, 396
seek a data string 399
shift left 141, 406
shift right 143, 407
shifting 42, 44
sorting 280, 408
split a line 418
submitting for batch processing 283, 412
test flow a paragraph 418
data field, defined 375
data in controlled libraries, editing 15
data lines, referring to 105
data modes 20
data set
adding a member 390
copying a model into 249, 369
creating a member 215, 318
creating a new 7
editing a member 224, 333
editing existing 8
generating statistics 283, 411
moving a member 252, 371
password specification 6
renumbering lines automatically 267, 388
replacing a member 390
retrieving the current name 326
security 6
DATA_CHANGED, assignment statement 323
DATA_WIDTH, assignment statement 324
data-changed status, retrieving 323
DATAID, assignment statement 325
DATASET, assignment statement 326
Index  441

## Page 474

DBCS data
CHANGE command 50
column shifting 43
display boundary 4
hardware tabs 64, 65
SORT command 282, 409
TE (text entry) line command 63
TF (text flow) 61
TS (text split) line command 62
DDNames 87
debugging edit macros 113
debugging edit macros with ISREMSPY 116
DEFINE
edit macro command 90, 106
macro command 326
primary command 220
define tabs mode 413
defining
a name 220, 326
an alias for a command 106
an edit profile 18
defining macros
implicit 107
overriding command names 107
resetting definitions 106
scope of definitions 106
using an alias 106
defining tabs mode 285
DELETE
macro command 328
primary command 222
deleting
edit macro labels 105
labels 59
lines 156, 222
models 76
delimited string 45, 46
destination, specifying 108
destructive shift, defined 43
dialog development models 69
dialog service errors, debugging 113
dialog service requests 89
dialog variable name, defined 96
direction of the search 53
disabling a command 106
disabling a macro or built-in command 220, 326
display and control your profile 383
display boundary, DBCS data 4
display columns 330
display model notes 373
DISPLAY_COLS, assignment statement 330
DISPLAY_LINES, assignment statement 331
displaying an edit profile 18
displaying and controlling profile 262
displaying hexadecimal characters 236, 342
displaying model notes 256
displaying the Edit Settings panel 227
DOWN, macro command 331
duplicating lines 175
E
EBCDIC data 50
edit
edit (continued)
beginning a session 4
canceling changes 201, 308
column shifting 43
command reference section 191
command summary 12
considerations 14
controlling the boundaries 199, 305
controlling the environment 17
controlling the recovery 387
copying data 41
creating data 41
data display panel 8
displaying processed commands 13
editing data in controlled libraries 15
ending a session 11
entry panel 7
excluding lines 57
introduction to 3, 10
line command macros 14
line commands 12
macro command 14, 333
managing data 41
models 69
modes 19
moving data 41
number mode 27
option 2 4
primary command
description 224
syntax 224
primary commands, description 12
profiles 17
recovery, controlling 266
recursive 224, 333
replacing data 41
rules for entering line commands 135
selecting the editor 4
sequence number display 27
sequence number format 27
sequence numbers 26
shifting columns 43
shifting data 42, 44
splitting text 61
text entry 61
text flow 61
undisplayable characters 10
undoing edit interactions 66
word processing 61
EDIT
primary command
description 224
example 225
Edit - Entry panel 7
edit a member 333
Edit and View Settings panel 227
edit assignment statements
elements
keyphrase 96
overlays 97
value 96
how to use 98
manipulating data 99
Edit command errors, debugging 113
442  z/OS: z/OS ISPF Edit and Edit Macros

## Page 475

edit commands and PF key processing 13
edit compare command 314
edit COMPARE command 207
Edit data display panel 8
edit macro
alias name 106
assignment statements 87, 96
CLIST macro, differences from program macros 90
column positions, referring to 106
command procedure statements 88
command summary 14
commands 87
creating 87
data lines, referring to 105
defining 106
definition of 3
description 79
dialog service requests 89
identifying 364
implicit definition using an exclamation point 107
initial macro 24
introduction to 79
ISRBOX macro 119
ISRCHGS macro 126
ISRIMBED macro 121
ISRMASK macro 129
ISRMBRS macro 124
labels
description 103
editor-assigned 103
passing 105
referring to 105
using 104
levels 103
line command functions, how to perform 100
messages 102
naming 95
NOPROCESS operand 108
parameters 101
PROCESS command and operand 107
program macro
description 89
differences from CLIST macros 90
differences from REXX macros 90
parameter passing 90
running 94
writing 91
recovery macro 109
reference section 295
replacing built-in edit commands 107
resetting a command to previous status 106
return codes 109
REXX macro, differences from program macros 90
samples 119
testing
CLIST CONTROL statements 114
CLIST WRITE statements 113
description 113
experimenting with edit macro commands 115
return codes 111
REXX SAY statements 113
REXX TRACE statements 114
TSO commands 89
using 79
edit macro (continued)
variable substitution 95
variables 95
edit macros, debugging with ISREMSPY 116
Edit mode defaults 21
edit processing of PF keys 13
edit profile
autolist mode 194
autonum mode 195, 301
autosave mode 197, 303
boundary settings 151
caps mode 202
control and display 383
controlling and displaying 262
defaults 21
defining 18
definition of 17
displaying 18
initial macro 244, 348
lock 383
locking 262
modifying 19
naming 17
note mode 256
nulls mode 257
profile name 17
recovery macro 276
saving and restoring 421
specifying 6
tabs mode 285
types 17
Edit Profile Initialization, Site-wide 21
edit profile name, definition 17
edit profiles, locking 19
edit recovery
Edit Recovery panel 38
turning off 39
turning on 38
edit session, ending 229, 334
Edit Settings panel, displaying 227
editing a member 224
editing existing data 8
editor-assigned labels 59
editor, ISPF 3
EDITSET primary command 227
EDSET primary command 227
eliminating labels 59
END
macro command 334
primary command 229
end a macro 369
END command 197
end the edit session 334
ending an edit session 11, 229
entering text, text entry command 180
error codes for severe errors 110
error lines 22
EXCLUDE
macro command 335
primary command
description 44, 230, 231
qualifying search strings 54
specifying search strings 45
repeating 55
Index  443

## Page 476

exclude counts, querying the value 337
exclude status, reversing 234
EXCLUDE_COUNTS, assignment statement 337
excluded line limitations 54
excluded line messages
hiding 239
excluded lines
hiding 344
line status, set or query 425
redisplaying 58, 273
excluding a line 57, 188, 335
excluding data 44
explicit shifts, defined 42
extent of a search 53
F
F (show first line), line command 158
FIND
macro command
description 338, 339
RFIND command 275, 393
saving and restoring values 421
when to use instead of SEEK 401
primary command
description 44, 232, 233
qualifying search strings 54
specifying search strings 45
repeating 55
find counts, querying the value 340
FIND_COUNTS, assignment statement 340
finding a data string 232
finding a search string 338
finding data 44
finding lines, LOCATE primary command 247
finding models 75
first line, showing 158
flagged lines
changed lines 22
error lines 22
special lines 22
FLIP
assignment statement 341
definition 59
macro command 341
primary command 234
flow counts, querying the value 341
FLOW_COUNTS, assignment statement 341
Format Name field 7
formatted edit mode, defined 169
formatting input 367
fragments, syntax diagrams xxi
G
generate sequence numbers 375
generating data set statistics 283, 411
generating sequence numbers 258
guidelines for using the editor 14
H
Hardware Tab field, defined 65
hardware tabs
DBCS data 65
defining 64
description 63
fields, how to use 65
HEX
assignment statement 342
macro command 342
primary command 19, 236
hexadecimal characters
displaying 236, 342
format 19
mode 342
showing individual records in 159
string 45
HIDE
assignment statement 239
macro command 239, 344
primary command 239
hiding lines, EXCLUDE primary command 230
HILITE
macro command
description 344, 348
how to use 344
primary command
description 244
how to use 240
HILITE function description 28
HX (lowercase), line command 159
I
I (insert) line command 161
I operand, REXX TRACE statement 114
identify an edit macro 364
identifying columns 155
IMACRO
assignment statement 348
macro command 348
primary command 19, 244
implicit macro definition 107
implicit shifts, defined 42
initial macro, specifying 244, 348
initial macros
DEFINE commands used in 106
specifying in the EDIT service call 25
specifying on the Edit - Entry panel 25
starting 24
Initialization, Site-wide Edit Profile 21
INSERT, macro command 349
inserting
data 349
lines 161
interactive column numbers 106
introduction to edit macros 79
ISPEXEC 89
ISPF list data set 194, 300
ISPF, definition 3
ISRBLOCK, sample macro 431
ISRBOX, sample macro 119, 431
ISRCHGS, sample macro 126, 431
ISRCOUNT, sample macro 82, 431
ISRDASH, sample macro 79, 431
ISREDIT service 90
444  z/OS: z/OS ISPF Edit and Edit Macros

## Page 477

ISREDIT statements 87, 100
ISREMSPY 116
ISREMSPY, sample macro 431
ISRFLAG, sample macro 431
ISRIMBED, sample macro 121, 431
ISRMASK, sample macro 129, 431
ISRMBRS, sample macro 124, 431
ISRONLY, sample macro 431
ISRSEPC, sample macro 91, 431
ISRSEPP, sample macro 431
ISRSETLN, edit macro sample 398
ISRSLPLI, sample macro 91
ISRSLREX, sample macro 91, 431
ISRTDATA, sample macro 81, 431
ISRTDWRI, sample macro 113, 431
ISRTRYIT, sample macro 115, 431
K
keeping an edit command on the command line 13
keyboard
navigation 433
PF keys 433
shortcut keys 433
keyphrase, defined 96
keywords, syntax diagrams xxi
kinds of search strings 45
L
L (show last line), line command 163
L operand, REXX TRACE statement 114
LABEL
assignment statement
description 351
overview 104
querying the value 351
setting the value 351
labeled line, querying 360
labels
defined 59
deleting 59
editor-assigned 59
eliminating 59
in macro commands 59
specifying a range 60
labels in edit macros
deleting 105
description 103
editor-assigned 103
how to use 104
levels 103
nested macros 105
passing 105
referring to 105
languages for edit macros 79, 87
last line, showing 163
LC (lowercase), line command 164
left
scroll 352
shift columns 405
shift data 406
LEFT
LEFT (continued)
macro command 352
LEVEL
assignment statement 353
macro command 353
primary command 245
level number, specifying 245, 353
limiting the SORT command 281, 409
LINE
adding 357
assignment statement 354
querying the number 354
querying the value 354
setting the value 354
line command field
resetting 45
line command functions in edit macros 100
line command macros in edit 14
line commands
( (column shift left) 138
) (column shift right) 140
< (data shift left) 141
> (data shift right) 143
A, AK (after) 145
B (before) 148, 149
B, BK (before) 148
BOUNDS 151
C (copy) 152
COLS 155
D (delete) 156
description 135
F (show first line) 158
HX (lowercase) 159
I (insert) 161
L (show last line) 163
LC (lowercase) 164
M (move) 166
MASK 169
MD (make dataline) 170
O (overlay) 172
OK (overlay, multiple targets) 172
R (repeat) 175
rules for entering 135
S (show line) 58, 177
summary 136
TABS 179
TE (text entry) 61, 63, 180
TF (text flow) 61, 183
TS (text split) 61, 185
UC (uppercase) 187
usage 12
X (exclude) 55, 57, 188
line label
querying the value 351
setting the value 351
line number, ordinal 247
line numbers
restoring 273
line pointer
COPY macro command 317
CREATE macro command 318
CURSOR assignment statement 320
DELETE macro command 329
incomplete 319
Index  445

## Page 478

line pointer (continued)
INSERT macro command 349
invalid 318, 372
LABEL assignment statement 351
LINE assignment statement 354
LINE_AFTERassignment statement 356
LINE_BEFORE assignment statement 357
LOCATE macro command 362
MASKLINE assignment statement 367
MODEL macro command 370
MOVE macro command 371
referring to labels 105
SHIFT ( macro command 405
SHIFT ) macro command 406
SHIFT > macro command 407
TABSLINE assignment statement 415
TENTER macro command 416
TFLOW macro command 418
TSPLIT macro command 419
XSTATUS assignment statement 425
line pointer range
CREATE macro command 319, 322, 329, 363
DELETE macro command 329, 363
LOCATE macro command 363
SUBMIT macro command 412
line range 60
LINE_AFTER, assignment statement 355
LINE_BEFORE, assignment statement 357
LINE_STATUS 359
linefeed character
LF macro command 350
LF primary command 51, 52, 246
LINENUM, assignment statement 360
lines
adding 161
copying 152
deleting 156, 328
exclude status 425
excluded limitations 54
excluding 57, 230, 335
inserting 161
locating 247, 361
moving 166
numbering automatically 195
overlaying 172
query display 331
renumbering automatically 267, 388
repeating 175
showing 177
showing the first 158
showing the last 163
specifying ranges 59
splitting 62, 418
literal character string, defined 96
LOCATE
macro command
generic syntax 362
specific syntax 361
primary command
generic syntax 248
specific syntax 247
locate lines 361
locating lines, LOCATE primary command 247
lock your profile 383
locking an edit profile 19
locking your profile 262
logical record length, querying 364
logical tabs, description 63
lowercase, converting to 164
lptr
COPY macro command 317
CURSOR assignment statement 320
DELETE macro command 329
incomplete 319
INSERT macro command 349
invalid 318, 372
LABEL assignment statement 351
LINE assignment statement 354
LINE_AFTER assignment statement 356
LINE_BEFORE assignment statement 357
LOCATE macro command 362
MASKLINE assignment statement 367
MODEL macro command 370
MOVE macro command 371
referring to labels 105
SHIFT ( macro command 405
SHIFT ) macro command 406
SHIFT > macro command 407
TABSLINE assignment statement 415
TENTER macro command 416
TFLOW macro command 418
TSPLIT macro command 419
XSTATUS assignment statement 425
lptr-range
CREATE macro command 319, 322, 329, 363
DELETE macro command 329, 363
LOCATE macro command 363
LRECL, assignment statement 364
M
M (move), line command
description 166
used with CREATE command 216
used with REPLACE command 271
macro
ending in batch 369
specifying a recovery 276, 395
specifying an initial 244, 348
Macro command profile reset syntax 384
macro commands
abbreviations 427
assignment statements 96
AUTOLIST 300
AUTONUM 301
AUTOSAVE 303
BOUNDS 305
BROWSE 307
BUILTIN 308
CANCEL 308
CAPS 309
CHANGE 310
COPY 317
CREATE 318
CUT 322
DEFINE 326
DELETE 328
disabling 220, 326
446  z/OS: z/OS ISPF Edit and Edit Macros

## Page 479

macro commands (continued)
DOWN 331
EDIT 333
END 334
EXCLUDE 335
FIND 338
FLIP 341
HEX 342
HIDE 239
HILITE 344
identifying 220, 326
IMACRO 348
INSERT 349
introduction to 79
labels 59
LEFT 352
LEVEL 353
LF 350
LOCATE 361
MACRO 364
MEND 369
MODEL 369
MOVE 371
NONUMBER 372
NOTES 373
NULLS 374
NUMBER 375
PACK 378
PASTE 379
PROCESS 381
PROFILE 383
RCHANGE 265, 385
RECOVERY 387
reference section 295
RENUM 388
REPLACE 390
RESET 391
RFIND 275, 393
RIGHT 394
RMACRO 109, 395
SAVE 396
SCAN 398
SEEK 44, 399
SETUNDO 403
SHIFT ( 405
SHIFT ) 405
SHIFT < 406
SHIFT > 407
SORT 408
SOURCE 410
STATS 411
SUBMIT 412
summary 295
TABS 413
TENTER 61, 416
TFLOW 61, 418
TSPLIT 61, 418
UNNUMBER 419
UP 420
usage 14
VERSION 422
VIEW 423
macro definitions, resetting 106
macro nesting level
macro nesting level (continued)
querying 366
retrieving 103
MACRO_LEVEL, assignment statement 105, 366
MACRO, macro command 364
macros, sample 431
managing data 41
mask line, set or query 367
mask, defining 169
MASK, line command 169
MASKLINE, assignment statement
description 367, 368
overlays 97
using 98
MD (make dataline), line command 170
member name, querying 368
MEMBER, assignment statement 368
member, editing 224, 333
MEND, macro command 369
messages, displayed from edit macros 82, 102
mixed data, used with data strings 89
Mixed Mode field 7
model
adding 72
changing 72, 75
class, defined 69
copying into data set 249
copying into the current data set 369
deleting 72, 76
edit, defined 69
finding 72, 75
hierarchy 69
kinds 69
locating 75
logical name 69
macro command 369
name, defined 70
primary command 249
qualifier, defined 70
using 70
model notes, displaying 256, 373
model selection panels 71
modes, edit 19, 20
modification flag 247
modification level number, specifying 245, 353
modification level, description 26
modifying an edit profile 19
MOUNT authority 6
MOVE
macro command 371
primary command 41, 252
move a data set member 371
moving a data set member 252
moving a line of data in an edit macro 100
moving data into the current data set 41
moving lines 166
multiple parameters in an edit macro 101
N
name, defining 220, 326
naming edit macros 95
navigation
keyboard 433
Index  447

## Page 480

nested macros, starting 103
nesting level, querying 366
NOCONLIST operand, CLIST CONTROL statement 115
NOLIST operand, CLIST CONTROL statement 115
non-destructive shifting, defined 44
NONUMBER
macro command 372
primary command 256
NOPROCESS 108
normal, defined for stats mode 25
NOSYMLIST operand, CLIST CONTROL statement 115
note lines, converting to data 170
note mode
description of 19
querying the value 373
setting the value 256, 373
NOTES
assignment statement 373
macro command 373
primary command 19, 256
notes, displaying model 256, 373
null spaces, controlling 257, 374
NULLS
assignment statement 374
macro command 374
primary command 20, 257
nulls mode
description of 20
querying the value 374
setting the value 257, 374
NUMBER
assignment statement 375
macro command 375
primary command
description 20, 258
DISPLAY operand 27
number mode
defined 20
description 20, 258
initializing 27
setting, edit 26
turning off 256, 372
used with RENUM command 267, 388
number, specifying the modification level 245, 353
numbering lines automatically 195, 301
numbers
controlling version 290, 422
generating sequence 258, 375
modification level 26
remove sequence 419
removing sequence 289
sequence 26
turning off number mode 256, 372
O
O (overlay), line command 172
O operand, REXX TRACE statement 114
OK (overlay, multiple targets), line command 172
ordinal line number 247
overlaying lines 172
overlays, guidelines on how to perform 97
overriding, built-in edit commands 107
P
PACK
assignment statement 378
macro command 378
primary command 20, 260
pack mode 20, 260
packing data, edit 15
panel
excluding lines 188
process the 381
resetting the 391
set up for text entry 416
panel data, resetting 273
panel values, saving and restoring 421
panels
Edit data display 8
Edit Entry 5, 226
edit profile display 18, 264
Edit Recovery 38
model selection 71
parameters in an edit macro 101
passing labels 105
passing parameters to an edit macro
description 101
multiple 101
processing an Edit command 90
program macros 90
password protection 6
Paste Lines 379
Paste Macro command 379
PASTE primary command 261
pasting lines 261
pathnames, specifying for z/OS UNIX files 15
PDF, defined 3
PF key processing in edit 13
PF keys, scroll commands 10
picture string 46
power typing, defined 63
prepare display for data insertion 349
PRESERVE command 11, 262
PRESERVE macro 380
primary commands
abbreviations 427
AUTOLIST 19, 194
AUTONUM 19, 195
AUTOSAVE 19, 197
BOUNDS 199
BROWSE 200
BUILTIN 201
CANCEL 201
CAPS 19, 202
CHANGE 44, 203
COPY 41, 210
CREATE 41, 215
CUT 218
DEFINE 220
DELETE 222
displaying after processing 13
EDIT 224
END 229
EXCLUDE 44, 230
FIND 44, 232
FLIP 59, 234
448  z/OS: z/OS ISPF Edit and Edit Macros

## Page 481

primary commands (continued)
HEX 19, 236
HIDE 239
HILITE 240
IMACRO 19, 244
LEVEL 245
LF 51, 52, 246
LOCATE 247
MODEL 249
MOVE 41, 252
NONUMBER 256
NOTES 19, 256
NULLS 20, 257
NUMBER 20, 258
PACK 20, 260
PASTE 261
PROFILE 19, 262
RECOVERY 20, 266
reference section 191
RENUM 267
REPLACE 41, 269
RESET 59, 273
RMACRO 276
SAVE 276
SETUNDO 20, 278
SORT 280
SOURCE 51, 282
STATS 20, 283
SUBMIT 283
summary 191
TABS 20, 285
UNDO 286
UNNUMBER 289
usage 12
VERSION 290
VIEW 292
PROCESS command and operand 107
PROCESS, macro command
description 382
used with RANGE_CMD assignment statement 385
processing built-in commands 201, 308
PROFILE
assignment statement 383
macro command
description 383
profile control syntax 383
profile lock syntax 383
primary command
description 19, 263
display or define a profile 18
profile control syntax 263
profile lock syntax 263
profile defaults 21
PROFILE RESET command 21
Profile Reset syntax 263
Profile reset syntax, macro command 384
profile, edit
autolist mode 194, 369
autonum mode 195, 301
autosave mode 197, 303
boundaries 199
boundary settings 151
caps mode 202
control and display 383
profile, edit (continued)
controlling and displaying 262
defining 18
description 17
displaying 18
initial macro 244, 348
lock 383
locking 19, 262
modifying 19
note mode 256
nullsmode 257
recovery macro 276
saving and restoring 421
tabs mode 285
types 17
program macros
differences from CLISTs 90
differences from REXX EXECs 90
how to write 91
implicit definition 107
passing parameters 90
running 94
Q
qualifying the search string 54
query
a line 354
autolist mode 300
autonum mode 301
autosave mode 303
block size 304
caps mode 309
change count 313
command entered 384
current member name 368
cursor position 319
data ID 325
data set name 326
data width 324
data-changed status 323
display columns 330
display lines 331
edit boundaries 305
edit profile 383
exclude counts 337
exclude status for a line 425
find counts 340
flow counts 341
hexadecimal mode 342
initial macro 348
line label 351
line number 360
logical record length 364
macro nesting level 366
mask line 367
modification level number 353
note mode 373
nulls mode 374
number mode 375
pack mode 378
record format 386
recovery mode 387
seek counts 402
Index  449

## Page 482

query (continued)
tabs line 415
tabs mode 413
version number 422
Query Source and Change Information for a Line in a Data
Set, LINE_STATUS 359
Query Volume Information 424
R
R (repeat) line command 175
R operand, REXX TRACE statement 114
range
specifying 108
using labels to specify 60
RANGE_CMD, assignment statement
description 108, 384
used with the PROCESS command 385
RC variable 110
RCHANGE, macro command
description 265, 385
used to repeat CHANGE command 55
realigning data, LF primary command 246
RECFM, assignment statement 386
record format, query 386
recovery
controlling edit 387
edit 38
macro 109, 395
macro, saving the name of 276
mode 20, 387
of data after system failure 266
RECOVERY
assignment statement 387
macro command 387
primary command 20, 266
recursive editing, defined 224, 333
redisplaying excluded lines 58
referring to column positions 106
referring to data lines 105
reformatting a paragraph 183
regular expression 46
regular expressions 48
relative line number of cursor, setting or retrieving 319
relative line numbers 106
remove sequence numbers 419
removing lines 222, 328
removing sequence numbers 289
RENUM
macro command 388
primary command 267
RENUMBER primary command, DISPLAY operand 27
renumbering lines automatically 267, 388
repeatable items, syntax diagrams xxi
repeating a change 265, 385
repeating a search
RCHANGE command, Edit 55
RFIND command, Edit 55
repeating lines 175
REPLACE
macro command 390
primary command
description 269, 270
how to use 41
replace a data set member 390
replacing
data 41, 269
lines 99
RESET
macro command 391
primary command 273
RESET command, PROFILE 21
reset the data display 391
resetting macro definitions 106
resetting the data panel 273
resetting the line command field 45
retrieving the change count 313
retrieving the data ID 325
retrieving the data set name 326
retrieving the data width 324
retrieving the data-changed status 323
return codes
&LASTCC variable 110
0 to 20 109
above 20 110
ISPF editor 110
RC variable 110
reversing exclude status of data 234
reversing last data change 286
REXX edit macro statements 79, 87
REXX SAY statements, using to debug edit macros 113
REXX TRACE statements, using to debug edit macros 114
RFIND command
description 275, 393
used to repeat FIND and EXCLUDE commands 55
RIGHT
macro command 394
scroll 394
RMACRO
assignment statement
description 395
overview 109
macro command 395
primary command
description 276
overview 109
S
S (show line), line command
description 177
redisplaying excluded lines 58
S operand, REXX TRACE statement 114
sample edit macros 119
SAVE
macro command 396
primary command 276
save data automatically 303
save the current data 396
SAVE_LENGTH command 397
saving and restoring
CHANGE macro command values 421
cursor and panel values 421
edit profile 421
FIND macro command values 421
saving current data 276
saving data automatically 197
SCAN
450  z/OS: z/OS ISPF Edit and Edit Macros

## Page 483

SCAN (continued)
assignment statement 398
macro command 398
SCAN assignment statement 95
scope of macro definitions 106
scroll
down 331
left 352
right 394
up 420
using PF keys 10
search
controlling 52
DBCS search string, delimiting 45
extent 53
qualifying 54
starting point and direction 53
search string, finding 275
search strings
character 45
delimited 45
finding 338
hexadecimal 45
picture 45
simple 45
security, data set 6
seek a data string 399
seek counts, query 402
SEEK_COUNTS, assignment statement 402
SEEK, macro command
description 44, 399, 401
when to use instead of FIND 339
sequence numbers
display 27
format 27
generating 258, 375
initializing 27
setting, edit 26
set
a line 354
autolist mode 300
autonum mode 301
autosave mode 303
caps mode 309
command scan mode 398
cursor position 319
edit boundaries 199, 305
edit profile 383
exclude status for a line 425
hexadecimal mode 342
initial macro 348
line label 351
mask line 367
modification level number 353
note mode 373
nulls mode 257, 374
number mode 375
pack mode 378
recovery mode 387
tabs line 415
tabs mode 285, 413
version number 422
setting
mask 169
setting the edit boundaries 199, 305
SETUNDO
macro command 403
primary command 66, 278
SETUNDO command 278
SHIFT (, macro command 405
SHIFT ), macro command 405
SHIFT <, macro command 406
SHIFT >, macro command 407
shift columns
left 405
right 405
shift data
left 406
right 407
shifting data
edit
columns 43
explicit 42
implicit 42
non-destructive 44
shortcut keys 433
showing first line 158
showing last line 163
showing lines 177
SI characters, delimiting a search 45
simple editing 10
simple string 45, 46
Site-wide Edit Profile Initialization 21
site-wide macro 14
SO characters, delimiting a search 45
software tab field, defined 180
software tabs
defining 64
description 63
fields, how to use 180
SORT
macro command
DBCS data 409
description 408, 409
limiting 409
without operands 409
primary command
DBCS data 282
description 280, 281
limiting 281
without operands 281
sorting data 280, 408
source listing, create 194, 300
spaces, controlling null 257, 374
special lines 22
specify a recovery macro 109, 395
specifying
an initial macro 14, 24, 348
the level number 353
specifying a recovery macro 276
split screen, searching within 54
splitting a line of text 185
splitting lines 62
splitting text 61
standard sequence field, defined 27
starting point of a search 53
statistics
creation and maintenance of 25
Index  451

## Page 484

statistics (continued)
generating for a data set 283, 411
STATS
assignment statement 411
macro command 411
primary command 20, 283
stats mode 20, 25
strings, kinds of search
character 45
delimited 45
hexadecimal 45
picture 45
simple 45
SUBMIT
macro command 412
primary command 283
submit data for batch processing 412
submitting data for batch processing 283
summary of changes xxix
SYMLIST operand, CLIST CONTROL statement 114
syntax diagrams, how to read xxi
Syntax, macro command profile reset 384
syntax, Profile Reset 263
T
TABS
assignment statement 413
controlling and querying 64, 413
line command
defining hardware tabs 64
defining software tabs 64
description 179
limiting hardware tab columns 65
using software tab fields 180
macro command 413
primary command 20, 285
tabs line
querying the value 415
setting the value 415
tabs mode
description 20, 64
setting the value 285, 413
TABSLINE, assignment statement 415
TE (text entry), line command
DBCS data, using a DBCS terminal 63
description 63, 180, 181
example 181
syntax 180
template (overlay)
definition 98
how to design 98
TENTER, macro command 416
text entry
in word processing 61
setting up the panel 416
TE line command 180
text flow 61
text flowing a paragraph 183, 418
text split a line 418
TF (text flow), line command
DBCS data, using a DBCS terminal 61
description 61, 183, 184
TFLOW, macro command 418
trademarks 438
trailing blanks, saving 262
TS (text split), line command
DBCS data 62
description 185
TSO commands in edit macros 89
TSPLIT, macro command 418
turn off number mode 372
turning off number mode 256
U
UC (uppercase), line command 187
undisplayable characters 10
UNDO
primary command 286
SETUNDO requirement 403
with SETUNDO macro 278
undoing edit interactions
description 286
how to use 66
UNDO primary command 286
UNDOSIZE 66
UNIX
specifying pathnames 15
UNIX files
copying and moving data 41
creating and replacing data 41
UNNUMBER
macro command 419
primary command 289
UP, macro command 420
uppercase, converting data to 187
user interface
ISPF 433
TSO/E 433
USER_STATE, assignment statement 421
using the ISPF editor 3
UTF-8
data, working with 52
linefeed character
LF primary command 52
V
value portion of an edit macro statement 96
variable substitution, controlling 95
variables in edit macros 95
variables, syntax diagrams xxi
verifying parameters 108
VERSION
assignment statement 422
macro command 422
primary command 290
version number
controlling 290, 422
description 26
VIEW
macro command 423
primary command 292
VOLUME assignment statement 424
Volume Information 424
452  z/OS: z/OS ISPF Edit and Edit Macros

## Page 485

W
writing program macros 89, 91
X
X (exclude), line command
using 55, 57
XSTATUS, assignment statement 425
Z
z/OS UNIX
specifying pathnames 15
z/OS UNIX files
copying and moving data 41
creating and replacing data 41
ZDEFAULT edit profile 21
ZEDILMSG dialog variable 111
ZEDISMSG dialog variable 111
ZEDITCMD variable 102
ZEDLMSG 102
ZEDMSGNO dialog variable 111
ZEDSAVE variable 323
ZEDSMSG 102
ZUSERMAC variable 25
Index  453

## Page 486

454  z/OS: z/OS ISPF Edit and Edit Macros

## Page 487



## Page 488

IBM®
Product Number: 5655-ZOS
SC19-3621-60
