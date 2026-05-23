# Chapter 5. SCLM services and macros

Source file: f54rs00_v3r1.md
Start page: 207
Page span: 207-250

## Page 207

Chapter 5. SCLM services and macros
This chapter shows the syntax and return codes for the SCLM services as well as the syntax for the SCLM
macros. For a complete description of the services and macros see the "SCLM Reference" section in the
z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference.
SCLM services
ACCTINFO—retrieve accounting information
Command invocation format
FLMCMD ACCTINFO,  project ,
prj_def
, group , type , member
,
user_info_table
,
include_table
,
change_code_table
,
ada_cu_table
,
SEARCH
FORWARD
MATCH
,
dd_msgs
Call invocation format
lastrc := FLMLNK('ACCTINFO',  sclm_id , , group , type , member , user_info_table
, include_table , change_code_table , ada_cu_table ,
SEARCH
FORWARD
MATCH
,$msg_array);
Return codes
 0
Normal completion. An account record exactly matching the specified criteria was found and the
information was stored successfully.
 8
Error completion. No account record was found for the specified member.
• If FORWARD was specified then there are no accounting records for the group which match or follow
the specified type and member name.
• If MATCH was specified then there is not an account record with the specified group, type and
member name.
• If SEARCH was specified then there are no matching account records found when searching up the
hierarchy starting from the specified group.
ACCTINFO service
© Copyright IBM Corp. 1989, 2024 181

## Page 208

12
Error completion. Refer to the messages for more information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. An invalid parameter list was passed to the requested service.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
AUTHCODE—set or retrieve an AUTHCODE
Command invocation format
FLMCMD AUTHCODE,  project ,
prj_def
, group , type , member
,
from_authcode
,
to_authcode
,
C
U
,
dd_authmsgs
,
dd_authrept
Call invocation format
lastrc := FLMLNK('AUTHCODE ', sclm_id , , group , type , member
, from_authcode , to_authcode ,
C
U
, dd_authmsgs , dd_authrept );
Return codes
 0
Normal completion. Authcode changed or reported successfully.
 2
Normal completion. Authcode not changed. One of these occurred:
• To_authcode = existing authcode (no change needed)
• From_authcode requested does not equal existing authcode (no change wanted)
• Member is not editable.
 4
Warning condition. Segment exists at a lower level with an authcode not equal to the "to_authcode"
which could overlay the current segment.
 8
Error condition. Invalid type, member, or mode parameter. See the dd_authmsgs for details.
AUTHCODE service
182  z/OS: z/OS ISPF Reference Summary

## Page 209

12
Severe error condition. Accounting record not found or severe error.
16
Severe error condition. One of these occurred:
• Not authorized to update "to_authcode", access_key mismatch, or not authorized to update data
set.
• Verification failed.
• Error updating accounting record.
• Invalid group.
SCLM might not produce messages because there was an error invoking the AUTHCODE module.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is not valid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
BUILD—build a member
Command invocation format
FLMCMD BUILD, project ,
prj_def
, group , type , member
,
userid
,
N
E
L
S
,
C
F
R
U
,
Y
N
,
Y
N
,
prefix_userid
,
dd_bldmsgs
,
dd_bldrept
,
dd_bldlist
,
dd_bldexit
BUILD service
Chapter 5. SCLM services and macros  183

## Page 210

Call invocation format
lastrc := FLMLNK('BUILD␢␢␢' , sclm_id , group , type , member
, userid
'␢'
,
N
E
L
S
, C
F
R
U
, Y
N
, Y
N
, prefix_userid
'␢'
, dd_bldmsgs , dd_bldrept , dd_bldlist , dd_bldexit );
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
12
Severe error condition. Messages are not produced. Error invoking the Build module.
16
Severe error condition. Messages are not produced. Unable to retrieve SCLM ID information.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced for one of these reasons:
• Invalid service requested
• Invalid parameter list for the requested service
• The version of the FLMLNK subroutines does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
DBACCT—retrieve accounting records for a member
Command invocation format
You cannot use command procedures to call this service.
DBACCT service
184  z/OS: z/OS ISPF Reference Summary

## Page 211

Call invocation format
lastrc := FLMLNK('DBACCT␢␢', sclm_id , group , type , member , found_group
,$acct_info ,$list_info ,$msg_array);
Return codes
 0
Normal completion.
 4
Warning condition. The accounting record could not be found.
 8
Error condition. See the $msg_array parameter above for more details.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced for one of these reasons:
• Invalid service requested
• Invalid parameter list for the requested service
• The version of the FLMLNK subroutines does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
DBACCT service
Chapter 5. SCLM services and macros  185

## Page 212

DBUTIL—generate a tailored data set and report
Command invocation format
FLMCMD DBUTIL, project ,
prj_def
,
*
acct_group1
,
acct_group2
,
acct_group3
,
acct_group4
,
acct_group5
,
acct_group6
,
*
acct_type
,
*
acct_member
,
*
authcode
,
*
change_code
,
*
change_group
,
*
change_userid
,
*
language
,
YES
NO
,
ACCT
BMAP
*
,
*
IN
OUT
,
arch_group
,
arch_type
,
arch_member
,
NORMAL
EXTENDED
SUBUNIT
,
YES
NO
,
YES
NO
,
report_name
,
dd_msgs
,
dd_rept
,
dd_tailor
,
report_line
Call invocation format
You cannot use call procedures to start this service.
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
DBUTIL service
186  z/OS: z/OS ISPF Reference Summary

## Page 213

>8
Severe error condition. Messages are not produced.
DELETE—delete database components
Command invocation format
FLMCMD DELETE, project ,
prj_def
, group , type , member
, access_key ,
TEXT
ACCT
BMAP
Call invocation format
lastrc := FLMLNK('DELETE␣␣' ,sclm_id , group , type , member , access_key
, TEXT
ACCT
BMAP
,$msg_array );
Return codes
 0
Normal completion.
 4
Warning condition. The member, accounting record, or build map were not found.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced for one of these reasons:
• Invalid service requested
• Invalid parameter list for the requested service
• The version of the FLMLNK subroutines does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
DELETE service
Chapter 5. SCLM services and macros  187

## Page 214

DELGROUP—delete database components from group
Command invocation format
FLMCMD DELGROUP, project ,
prj_def
, group
*
, type
*
, member
*
, ACCT
BMAP
TEXT
OUTPUT
,
REPORT
EXECUTE
,
dd_list
,
dd_msgs
,
dd_rept
,
dd_exit
, Y
N
,
pack_days
Call invocation format
lastrc := FLMLNK('DELGROUP' , sclm_id , group
*
, type
*
, member
*
, ACCT
BMAP
TEXT
OUTPUT
, REPORT
EXECUTE
, dd_list
, dd_msgs , dd_rept , dd_exit , Y
N
, pack_days );
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
12
Severe error condition. SCLM does not produce messages because there was an error invoking the
DELGROUP module.
16
Severe error condition. SCLM does not produce messages because it was unable to retrieve SCLM ID
information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
DELGROUP service
188  z/OS: z/OS ISPF Reference Summary

## Page 215

32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module (for
future use).
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
DSALLOC—allocate data sets for group/type
Command invocation format
FLMCMD DSALLOC, project ,
prj_def
, first_group ,
P
A
, total_groups , type , ddname
Call invocation format
lastrc := FLMLNK('DSALLOC␢' , sclm_id , first_group , P
A
, total_groups
, type , ddname ,$msg_array );
Return codes
 0
Normal completion.
 4
Warning condition. The $msg_array parameter contains the warning message associated with this
condition. A warning occurs if the number of data sets allocated to ddname is less than the number
requested in the total_groups parameter.
 8
Error condition. The $msg_array parameter contains the error message associated with this condition.
20
Severe error condition. SCLM does not produce messages because the SCLM ID (sclm_id parameter)
is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services were not initialized.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• Invalid service requested
• Invalid parameter list for the requested service
• The version of the FLMLNK subroutines does not match the version of the SCLM services module.
DSALLOC service
Chapter 5. SCLM services and macros  189

## Page 216

34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
EDIT— edit a member of a controlled library
Command invocation format
FLMCMD EDIT, project ,
prj_def
, group1 ,
group2
,
group3
,
group4
, type , member ,
N
Y
,
imac
,
prof
,
Y
N
,
N
Y
,
N
Y
,
N
Y
,
authcode
,
chgcode
,
volser
,
dd_editmsgs
;
Call invocation format
lastrc := FLMLNK('EDIT',  sclm_id , group1 , group2 , group3 , group4 , type
, member , Y
N
, imac , prof , Y
N
, Y
N
, Y
N
, Y
N
, authcode
,  chgcode
, volser
, dd_editmsgs
);
Return codes
Possible return codes are:
 0
Normal completion.
 8
Error condition. See the dd_editmsgs for details.
EDIT service
190  z/OS: z/OS ISPF Reference Summary

## Page 217

12
Severe error condition. SCLM does not produce messages because there was an error invoking the
edit module.
16
Verification error from a user exit routine.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized. See the SCLM Reference section in the z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference for information on initializing an SCLM services session.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of FLMLNK subroutine does not match the version of the SCLM services module.
END—end an SCLM services session
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('END␢␢␢␢␢',  appl_id , msg_line );
Return codes
 0
Normal completion.
 4
Warning condition. Unable to free an SCLM ID associated with the application ID.
 8
Error condition.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• Invalid service requested
• Invalid parameter list for the requested service
• The version of the FLMLNK subroutines does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
END service
Chapter 5. SCLM services and macros  191

## Page 218

ENDEC— encode and decode members
Command invocation format
FLMCMD ENDEC, project ,
prj_def
,
ENCODE
DECODE
,
in_ddname
,
in_group
,
in_type
,
in_member
,
out_ddname
,
out_group
,
out_type
,
out_member
, msgdd
Call invocation format
lastcc := FLMLNK('ENDEC␣␣␣', ', sclm_id ,
ENCODE
DECODE
,
in_ddname
,
in_group
,
in_type
,
in_member
,
out_ddname
,
out_group
,
out_type
,
out_member
,$msg_array );
Return codes
0
Normal completion. The encoding and decoding was performed.
4
Warning if ENCODE was specified, the input data set and member is already encoded. If DECODE was
specified, the input data set and member is already decoded.
12
Error completion. Refer to the messages for more information.
EXPORT—extract SCLM accounting information for a group
Command invocation format
FLMCMD EXPORT, project ,
prj_def
, group ,
N
Y
,
dd_msgs
,
dd_rept
ENDEC service
192  z/OS: z/OS ISPF Reference Summary

## Page 219

Call invocation format
lastrc := FLMLNK('EXPORT␢␢' , sclm_id , group , Y
N
, dd_msgs
, dd_rept );
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
12
Severe error condition. SCLM does not produce messages because there was an error invoking the
IMPORT module.
16
Severe error condition. SCLM does not produce messages because it was unable to retrieve SCLM ID
information.
20
Severe error condition. SCLM does not produced messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module (for
future use).
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
FREE—free database from its association with SCLM ID
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('FREE␢␢␢␢',sclm_id , msg_line );
Return codes
 0
Normal completion.
FREE service
Chapter 5. SCLM services and macros  193

## Page 220

8
Error condition.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
GETBLDMP—retrieve build map information
Command invocation format
FLMCMD GETBLDMP,  project ,
prj_def
, group , type , member
, bmap_table ,
dd_msgs
Call invocation format
lastrc := FLMLNK('GETBLDMP',  sclm_id , group , type , member , bmap_table
,$msg_array);
Return codes
 0
Normal completion. A build map record was found that exactly matched the specified criteria and the
information was stored successfully.
 4
Normal completion. A build map record was found at a higher level. The information was stored
successfully.
 8
Error completion. No account record was found for the specified member.
12
Error completion. Refer to the messages for more information.
GETBLDMP service
194  z/OS: z/OS ISPF Reference Summary

## Page 221

GETXDEP—return cross-dependency information
Command invocation format
FLMCMD GETXDEP, project , prj_def , group , type , member ,
xdep_table , scope,
dd_msgs
Call invocation format
Lastrc := FLMLNK('GETXDEP',  sclm_id , group , type , member ,
xdep_table , scope, $msg_array);
Return codes
4
The parent chain was truncated when the maximum nesting level was exceeded, or a circular
reference was detected. ZSFLIMIT is non-blank on truncated rows.
8
No parent data was found.
12
Cross-dependency database is not active.
16
Error updating ISPF table.
20
Error reading the Cross-dependency database.
IMPORT—import SCLM accounting information to current project
Command invocation format
FLMCMD IMPORT, project ,
prj_def
, group ,
' '
authcode
,
' '
change_code
,
' '
userid
,
C
U
R
,
dd_msgs
,
dd_rept
GETXDEP service
Chapter 5. SCLM services and macros  195

## Page 222

Call invocation format
lastrc := FLMLNK('IMPORT␢␢' , sclm_id , group ,
authcode
,
change_code
,
userid
, C
U
R
, dd_msgs , dd_rept );
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
12
Severe error condition. SCLM does not produce messages because there was an error invoking the
IMPORT module.
16
Severe error condition. SCLM does not produce messages because it was unable to retrieve SCLM ID
information.
20
Severe error condition. SCLM does not produced messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module (for
future use).
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
INIT—generate an SCLM ID for a database
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('INIT␢␢␢␢',  appl_id , project , prj_def ,sclm_id , msg_line );
INIT service
196  z/OS: z/OS ISPF Reference Summary

## Page 223

Return codes
 0
Normal completion.
 8
Error condition.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
LOCK—lock a member or assign an access key
Command invocation format
FLMCMD LOCK, project ,
prj_def
, group , type , member
,
authcode
,
access_key
,
userid
Call invocation format
lastrc := FLMLNK('LOCK␢␢␢␢' ,sclm_id , group , type , member
, authcode
'␢'
, access_key
'␢'
, userid
'␢'
, found_group
, max_prom_group ,$acct_info ,$list_info ,$msg_array);
Return codes
 0
Normal completion.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
LOCK service
Chapter 5. SCLM services and macros  197

## Page 224

• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
MIGRATE—create accounting information for selected members
Command invocation format
FLMCMD MIGRATE, project ,
prj_def
, group, type, member
,
authcode
,
language
,
change_code
,
C
U
F
,
dd_migmsgs
,
dd_miglist
,
dd_migrept
,
date
,
time
Call invocation format
lastrc:=FLMLNK('MIGRATE␣' , sclm_id , group , type , member , authcode
, language , change_code ,
C
U
F
,
dd_miglist
,
dd_migrept
,
date
,
time
);
Return codes
 0
Normal completion.
 4
Warning condition. See the SCLM messages for more information.
 8
Error condition. See the SCLM messages for more information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
MIGRATE service
198  z/OS: z/OS ISPF Reference Summary

## Page 225

32
Severe error condition. SCLM does not produce messages for one of these reasons:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
NEXTGRP—find the next group in a hierarchy
Command invocation format
FLMCMD NEXTGRP, project ,
prj_def
, group ,
dd_msgs
Call invocation format
lastrc := FLMLNK('NEXTGRP␣',sclm_id , group , dd_msgs);
Return codes
 0
Normal completion. NEXTGRP completed successfully. Variables are set.
 4
Warning condition. The group is already the top group. No variables are set.
 8
Error condition. Invalid project, prj_def, or group name.
12
Severe error condition. SCLM might not produce messages because there was an error invoking the
NEXTGRP module. For certain conditions messages are available.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
PARSE—parse a member for statistical and dependency information
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('PARSE␢␢␢' sclm_id , group , type , member , language
, Y
N
, ddname ,$stats_info ,$list_info ,$msg_array);
NEXTGRP service
Chapter 5. SCLM services and macros  199

## Page 226

Return codes
 0
Normal completion.
 4
Warning condition. A parser error occurred.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
PROMOTE—promote a member from one library to another
Command invocation format
FLMCMD PROMOTE, project ,
prj_def
, group , type , member
,
userid
,
N
E
S
,
C
U
R
,
dd_prommsgs
,
dd_promrept
,
dd_promexit
,
dd_copyerr
Call invocation format
lastrc := FLMLNK('PROMOTE␢' ,sclm_id , group, type, member , userid
'␢'
,
N
E
S
, C
U
R
, dd_prommsgs , dd_promrept , dd_promexit , dd_copyerr );
PROMOTE service
200  z/OS: z/OS ISPF Reference Summary

## Page 227

Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
12
Severe error condition. Messages are not produced. Error invoking the Promote module.
16
Severe error condition. Messages are not produced. Unable to retrieve SCLM ID information.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
RPTARCH—generate an SCLM architecture report
Command invocation format
FLMCMD RPTARCH, project ,
prj_def
, group , type , member
,
NONE
HL
LEC
CC
GEN
TOP SOURCE
, dd_rptmsgs , dd_rptrept
Call invocation format
You cannot use call procedures to start this service.
Return codes
 0
Normal completion.
RPTARCH service
Chapter 5. SCLM services and macros  201

## Page 228

4
Warning condition.
 8
Error condition.
16
Error condition. Unable to retrieve the SCLM table.
SAVE—lock, parse, and store a member
Command invocation format
FLMCMD SAVE, project ,
prj_def
, group, type, member ,
authcode
,
access_key
,
userid
,
language
,
Y
N
,
ddname
,
C
U
,
C
U
,
change_code
,
subproject
Call invocation format
lastrc := FLMLNK('SAVE␢␢␢␢' ,sclm_id , group, type, member , authcode , access_key
, userid
'␢'
, language , Y
N
, ddname , C
U
, C
U
, Y
N
,$list_info , max_prom_group ,$msg_array);
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
SAVE service
202  z/OS: z/OS ISPF Reference Summary

## Page 229

• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
SCLMINFO—return project information
Command invocation format
FLMCMD SCLMINFO,  project ,
prj_def
Call invocation format
lastrc := FLMLNK('SCLMINFO',  sclm_id );
Return codes
 0
Normal completion.
12
Error condition.
START—generate an application ID for a service session
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('START␢␢␢',  appl_id );
Return codes
 0
Normal completion.
12
Severe error condition. The maximum application ID limit was exceeded.
16
Severe error condition. An invalid version of the SCLM table was loaded.
20
Severe error condition. An invalid version of the multicultural support table was loaded.
24
Severe error condition. Unable to load the SCLM table.
28
Severe error condition. Unable to load the multicultural support table or the SCLM I/O load module.
32
Severe error condition. Messages are not produced. One of these is true:
SCLMINFO service
Chapter 5. SCLM services and macros  203

## Page 230

• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
STORE—store member information in an accounting record
Command invocation format
You cannot use command procedures to call this service.
Call invocation format
lastrc := FLMLNK('STORE␢␢␢' sclm_id , group, type, member , access_key
, language , userid
'␢'
, C
U
, Y
N
,$stats_info,$list_info
,$msg_array);
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
STORE service
204  z/OS: z/OS ISPF Reference Summary

## Page 231

UNLOCK—unlock a member in a development library
Command invocation format
FLMCMD UNLOCK, project ,
prj_def
, group , type , member
,
access_key
Call invocation format
lastrc := FLMLNK('UNLOCK␢␢' ,sclm_id , group , type , member
, access_key
'␢'
,$msg_array );
Return codes
 0
Normal completion.
 4
Warning condition.
 8
Error condition.
20
Severe error condition. Messages are not produced. Invalid SCLM ID.
24
Severe error condition. Messages are not produced. SCLM services have not been initialized.
32
Severe error condition. Messages are not produced. One of these is true:
• You requested an invalid service.
• You supplied an invalid parameter list for the requested service.
• The version of the FLMLNK subroutine does not match the version of the SCLM services module.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
VERDEL—delete version information
Command invocation format
FLMCMD VERDEL, project ,
prj_def
, group , type , member , date
, time ,
dd_msgs
,
longdate
UNLOCK service
Chapter 5. SCLM services and macros  205

## Page 232

Call invocation format
lastrc := FLMLNK('VERDEL ', sclm_id , , group , type , member , date , time
,$msg_array ,
longdate
);
Return codes
 0
Normal completion. The audit and version information were deleted.
 8
Error completion. No audit and version information was deleted. No audit record was found that
matches the specified criteria.
12
Error completion. Refer to the messages for more information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. An invalid parameter list was passed to the requested service.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
VERHIST—Retrieve Versioned Member Information
Command invocation format
FLMCMD VERHIST, project ,
prj_def
, group , type , member , date
, time , dd_report , Y
N
,
dd_msgs
,
longdate
Call invocation format
lastrc := FLMLNK('VERHIST␣' , sclm_id , , group , type , member , date , time
, dd_report , Y
N
,$msg_array
, longdate
);
VERHIST service
206  z/OS: z/OS ISPF Reference Summary

## Page 233

Return codes
Additional special services messages are written to the FLMMSGS ddname. See the "SCLM service
messages" section in z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference for
more information.
Other return codes might be produced by the FLMCMD or the FLMLNK processor. See the "SCLM service
return codes" section in z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference for
more information.
Possible return codes are:
0
Normal completion. An audit record exactly matching the specified criteria was found and the version
report was stored successfully.
8
Error completion. No audit record was found for the specified member.
12
Error completion. Refer to the messages for more information.
VERINFO—retrieve version information
Command invocation format
FLMCMD VERINFO,  project ,
prj_def
, group , type , member
,
date
,
time
,
user_info_table
,
include_table
,
change_code_table
,
ada_cu_table
,
FORWARD
BACKWARD
MATCH
,
dd_msgs
,
longdate
Call invocation format
lastrc := FLMLNK('VERINFO␣' , sclm_id , , group , type , member , date , time
, user_info_table , include_table , change_code_table , ada_cu_table
,
FORWARD
BACKWARD
MATCH
,$msg_array ,
longdate
);
Return codes
 0
Normal completion. An audit record exactly matching the specified criteria was found and the
information was stored successfully.
VERINFO service
Chapter 5. SCLM services and macros  207

## Page 234

8
Error completion. No audit record was found for the specified member.
• If FORWARD was specified then there are no audit records for the group which match or follow the
specified type, member, date and time.
• If BACKWARD was specified then there are no audit records for the group which match or precede
the specified type, member, date and time.
• If MATCH was specified then there is not an audit record with the specified group, type and member
name.
12
Error completion. Refer to the messages for more information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. An invalid parameter list was passed to the requested service.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
VERRECOV—recover a version
Command invocation format
FLMCMD VERRECOV,  project ,
prj_def
, group , type , member , date
, time ,
to_dataset
,
to_group
,
to_type
,
authcode
,
dd_msgs
,
longdate
Call invocation format
lastrc := FLMLNK('VERRECOV',  sclm_id , , group , type , member , date , time
, to_dataset , to_group , to_type , authcode ,$msg_array
,
longdate
);
Return codes
 0
Normal completion. The audit and version information were recovered.
VERRECOV service
208  z/OS: z/OS ISPF Reference Summary

## Page 235

8
Error completion. No audit and version information was recovered. No audit record was found that
matches the specified criteria.
10
Error completion. No audit and version information was recovered. The member could not be locked
with the specified authorization code.
12
Error completion. Refer to the messages for more information.
20
Severe error condition. SCLM does not produce messages because the SCLM ID is invalid.
24
Severe error condition. SCLM does not produce messages because SCLM services have not been
initialized.
32
Severe error condition. An invalid parameter list was passed to the requested service.
34
Severe error condition. An invalid service was requested.
36
Severe error condition. The version of the FLMLNK subroutine does not match the version of the SCLM
services module.
XDEPUPDT—Update Cross-dependency Information
Command invocation format
FLMCMD XDEPUPDT,  project , prj_def , *
group
,
dd_xdeprept
Call invocation format
Lastrc := FLMLNK('XDEPUPDT' , sclm_id *
group
, $msg_array);
SCLM macros
FLMABEG—define the project name of the project definition
Macro format
name FLMABEG
LOC=
BELOW
ABOVE
FLMAEND—last macro in the project definition
Macro format
FLMAEND
XDEPUPDT service
Chapter 5. SCLM services and macros  209

## Page 236

FLMAGRP—define a group of authorization codes
Macro format
name FLMAGRP AC=(
,
code )
FLMAGRP macro
210  z/OS: z/OS ISPF Reference Summary

## Page 237

FLMALLOC—define each DDname in the DDname substitution list for a
translator
Macro format
FLMALLOC IOTYPE= A
H
I
L
N
O
P
S
U
W
,BLKSIZE= block_size
,CATLG=
N
Y
,DDNAME= ddname
,DFLTMEM= default_member ,DFLTTYP= default_type
,DINIT=
N
Y
,DIRBLKS= directory_blocks
,DISP= OLD
SHR
MOD
NEW
,INCLS= include_set_name
,KEYREF= keyword_reference ,LANG= language
,LRECL= record_length
,MALLOC=
N
Y
,ALLCDEL=
N
Y
,MEMBER= member_name
,NOSAVRC= no_save_rc ,PATHOPT= uss_path_options
,PATHMDE= uss_path_mode ,PATHDSP= uss_path_disposition
,FILEDAT= uss_file_data
FLMALLOC macro
Chapter 5. SCLM services and macros  211

## Page 238

,PRINT=
N
Y
I
,RECFM= record_format
,RECNUM=  number_of_records ,VIO= Y
N
,ENCODE= Y
N
FLMALTC—specify alternate control information
Macro format
name FLMALTC ACCT= primary_accounting_data_set
,ACCT2= secondary_accounting_data_set ,DSNAME=  dataset_name
,EXPACCT=  export_account_data_set
,VERS= primary_audit_control_data_set
,VERS2= secondary_audit_control_data_set ,VERPDS= version_pds_name
,XDEP= xdep_data_set
FLMATVER—enable the audit and version utility
Macro format
FLMATVER GROUP= group
*
,TYPE= type
*
,SEQNUM= STANDARD
STD
COBOL
NONE
,VERSION= YES
NO
,VERCOUNT=  number_to_retain
,CHECKSUM=
YES
NO
FLMALTC macro
212  z/OS: z/OS ISPF Reference Summary

## Page 239

FLMCNTRL—specify project-specific control options
Macro format
FLMCNTRL
ACCT=
project .ACCOUNT.FILE
primary_account_data_set
,ACCT2= secondary_account_data_set ,EXPACCT=  export_account_data_set
,VERS= primary_audit_control_data_set
,VERS2= secondary_audit_control_data_set
,VSAMRLS=
NO
YES
,VERPDS= version_pds_name ,VERCOUNT=  number_to_retain
,DSNAME=  dataset_name_pattern
,DASDUNIT=
SYSALLDA
DASD_unit_name
,VIOUNIT=
VIO
VIO_unit_name
,MAXLINE=
60
max_line_count
,MAXVIO=
5000
max_vio_count
,OPTOVER=
N
Y
,MEMLOCK=
N
Y
,CONTROL=  control_data_set
,ADMINID=  administrator_userid ,VERCC= change_code_routine
,VERCCDS=  change_code_dataset
FLMCNTRL macro
Chapter 5. SCLM services and macros  213

## Page 240

,VERCCCM=
LINK
ATTACH
TSOLNK
ISPLNK
,VERCCOP=  change_code_options
,CCVFY= initial_change_code_exit_routine
,CCVFYDS=  initial_change_code_exit_dataset
,CCVFYCM=
LINK
ATTACH
TSOLNK
ISPLNK
,CCVFYOP=  initial_change_code_exit_options
,CCSAVE= save_change_code_exit_routine
,CCSAVDS=  save_change_code_exit_dataset
,CCSAVCM=
LINK
ATTACH
TSOLNK
ISPLNK
,CCSAVOP=  save_change_code_exit_options
,AVDVFY= verify_audit_version_delete_exit_routine
,AVDVFYDS=  verify_audit_version_delete_exit_dataset
,AVDVFYCM=
LINK
ATTACH
TSOLNK
ISPLNK
FLMCNTRL macro
214  z/OS: z/OS ISPF Reference Summary

## Page 241

,AVDVFYOP=  verify_audit_version_delete_exit_options
,AVDNTF=  notify_audit_version_delete_exit_routine
,AVDNTFDS=  notify_audit_version_delete_exit_dataset
,AVDNTFCM=
LINK
ATTACH
TSOLNK
ISPLNK
,AVDNTFOP=  notify_audit_version_delete_exit_options
,BLDINIT=  build_initial_user_exit_routine
,BLDINIDS=  build_initial_user_exit_dataset
,BLDINICM=
LINK
ATTACH
TSOLNK
ISPLNK
,BLDINIOP=  build_initial_user_exit_options
,BLDNTF= build_notify_user_exit_routine
,BLDNTFDS=  build_notify_user_exit_dataset
FLMCNTRL macro
Chapter 5. SCLM services and macros  215

## Page 242

,BLDNTFCM=
LINK
ATTACH
TSOLNK
ISPLNK
,BLDNTFOP=  build_notify_user_exit_options
,PRMINIT=  promote_initial_user_exit_routine
,PRMINIDS=  promote_initial_user_exit_dataset
,PRMINICM=
LINK
ATTACH
TSOLNK
ISPLNK
,PRMINIOP=  promote_initial_user_exit_options
,PRMVFY=  promote_verify_user_exit_routine
,PRMVFYDS=  promote_verify_user_exit_dataset
,PRMVFYCM=
LINK
ATTACH
TSOLNK
ISPLNK
,PRMVFYOP=  promote_verify_user_exit_options
,PRMCOPY=  promote_copy_user_exit_routine
FLMCNTRL macro
216  z/OS: z/OS ISPF Reference Summary

## Page 243

,PRMCPYDS=  promote_copy_user_exit_dataset
,PRMCPYCM=
LINK
ATTACH
TSOLNK
ISPLNK
,PRMCPYOP=  promote_copy_user_exit_options
,PRMPURGE=  promote_purge_user_exit_routine
,PRMPRGDS=  promote_purge_user_exit_dataset
,PRMPRGCM=
LINK
ATTACH
TSOLNK
ISPLNK
,PRMPRGOP=  promote_purge_user_exit_options
,DELINIT=  initial_delete_exit_routine ,DELINIDS=  initial_delete_exit_dataset
,DELINICM=
LINK
ATTACH
TSOLNK
ISPLNK
,DELINIOP=  initial_delete_exit_options
,DELVFY= verify_delete_exit_routine
FLMCNTRL macro
Chapter 5. SCLM services and macros  217

## Page 244

,DELVFYDS=  verify_delete_exit_dataset
,DELVFYCM=
LINK
ATTACH
TSOLNK
ISPLNK
,DELVFYOP=  verify_delete_exit_options ,DELNTF= notify_delete_exit_routine
,DELNTFDS=  notify_delete_exit_dataset
,DELNTFCM=
LINK
ATTACH
TSOLNK
ISPLNK
,DELNTFOP=  notify_delete_exit_options
,XDEP= xdep_data_set
project .XDEP.FILE
,XDEPDYN= Y
N
FLMCPYLB—identify additional data sets to be concatenated to a DDname
Macro format
FLMCPYLB
dataset_name
pathname
NULLFILE
,VOL= volser
FLMGROUP—define one group in the project definition
Macro format
name FLMGROUP AC=(
,
code )
,ALTC= group_control_options
,BKGRP= group_name
,BKMBRLVL=
N
Y
,KEY=
Y
N
,PROMOTE=  next_group
FLMCPYLB macro
218  z/OS: z/OS ISPF Reference Summary

## Page 245

FLMINCLS—associate include-sets with types in the project hierarchy
Macro format
name FLMINCLS
SAMEAS= flmincls_name_
TYPES=(  list_of_types ) ,CROSLANG=
Y
N
FLMLANGL—define a language to SCLM
Macro format
FLMLANGL LANG= language
,ALCSYSLIB=
N
Y
,ARCH=
N
Y
100
,BUFSIZE=  buffer_size
,CANEDIT=
Y
N
,CHKSYSLB=
PARSE
BUILD
IGNORE
,COMPOOL=
N
Y
,DEPPRCS=
Y
N
,DFLTCRF=  default_CREF_reference ,DFLTSRF=  default_source_reference
,SCOPE=
NORMAL
LIMITED
SUBUNIT
EXTENDED
,VERSION=  language_version
,LANGDESC=  language_description
,MBRLMT=
0
,ENCODE=
N
Y
FLMINCLS macro
Chapter 5. SCLM services and macros  219

## Page 246

FLMLRBLD—rebuild members with a particular language after promotion
Macro format
FLMLRBLD
GROUP= group_list
FLMPROJ—define a subproject to an SCLM project/alternate
Macro format
name FLMPROJ
subproj_desc
FLMNPROM—specify which SCLM editable elements may or may not be
marked as non-promotable
Macro format
FLMNPROM GROUP= (
,
group )
*
,
TYPE= (
,
type )
*
, LANG= (
,
lang )
*
, NPROM= YES
NO
FLMSYSLB—define a set of data sets for a language containing project
macros or included members
Macro format
language
FLMSYSLB dataset_name
,INCLS= include_set_name
,VOL= volser
FLMLRBLD macro
220  z/OS: z/OS ISPF Reference Summary

## Page 247

FLMTCOND—select build translators based on group and return codes
Macro format
FLMTCOND
GROUP= group_list
NOTGROUP=  group_list
,WHEN= relations_list
,ACTION=
RUN
SKIP
FLMTOPTS—select the options based on group
Macro format
FLMTOPTS OPTIONS= options_list
,GROUP= group_list
NOTGROUP=  group_list
,ACTION=
APPEND
REPLACE
FLMTCOND macro
Chapter 5. SCLM services and macros  221

## Page 248

FLMTRNSL—define once for each translator to be invoked for a language
Macro format
translator_ label
FLMTRNSL CALLNAM=' call_name '
,FUNCTN=
PARSE
VERIFY
BUILD
COPY
PURGE
,COMPILE=  translator_name
,DSNAME=  translator_dataset_name
,GOODRC=
0
good_return_code
,NOSVEXT=
0
no_save_external_rc
,OPTFLAG=
Y
N
,OPTIONS=  option_list ,PARMKWD=  parameter_keyword
,PDSDATA= N
Y ,PORDER=
1
0
2
3
,VERSION=  translator_version
,CALLMETH=
ATTACH
LINK
TSOLNK
ISPLINK
,TASKLIB=  translator_ddname
,INPLIST=
N
Y
,MBRRC= maximum_good_return_code
Note: See the "SCLM Reference" section in the z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference for information about the two translators FLMTPRE and FLMTPST.
FLMTRNSL macro
222  z/OS: z/OS ISPF Reference Summary

## Page 249

FLMTYPE—define one FLMTYPE in the project definition
Macro format
name FLMTYPE
EXTEND= extended_type
,BACKUP=
N
Y
,ISAPACK=
N
Y
,PACKFILE=
N
Y
,REUSEDAY=  number_of_days
FLMTYPE macro
Chapter 5. SCLM services and macros  223

## Page 250

FLMTYPE macro
224  z/OS: z/OS ISPF Reference Summary
