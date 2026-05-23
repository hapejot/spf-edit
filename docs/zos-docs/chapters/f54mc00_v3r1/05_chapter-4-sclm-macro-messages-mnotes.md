# Chapter 4. SCLM macro messages (MNOTEs)

Source file: f54mc00_v3r1.md
Start page: 887
Page span: 887-914

## Page 887

Chapter 4. SCLM macro messages (MNOTEs)
Macro messages are generated during project assembly.
The messages appear in this topic in alphabetical order. The Macro heading lists all macros pertaining to
the error or warning message. A macro appears on the list for one of these reasons:
• It issues the message.
• It must be corrected to remove the error or warning and prevent the message from being issued again.
MNOTE ERROR - A GROUP CAN NOT BE
SPECIFIED IF A WILDCARD(*)
BEEN USED IN THE GROUP
PARAMETER
Explanation
On the FLMNPRM macro, it is possible to specify
multiple groups; if the wildcard (*) has been specified,
no other values can be specified.
Macro
FLMNPRM
Project manager response
Modify the FLMNPRM macro to specify GROUP=* and
assemble the project definition again.
MNOTE ERROR - A LANGUAGE CAN NOT
BE SPECIFIED IF A WILDCARD(*)
BEEN USED IN THE TYPE
PARAMETER
Explanation
On the FLMNPRM macro, it is possible to specify
multiple languages; if the wildcard (*) has been
specified, no other values can be specified.
Macro
FLMNPRM
Project manager response
Modify the FLMNPRM macro to specify LANG=* and
assemble the project definition again.
MNOTE ERROR - A NAME must be
specified on the FLMAGRP macro
Explanation
No authorization code group name was specified on a
FLMAGRP macro.
Macro
FLMAGRP
Project manager response
Place the authorization code group name to be defined
in column 1, preceding the FLMAGRP macro, and
assemble the project definition again.
MNOTE ERROR - A NAME MUST BE
SPECIFIED ON THE FLMALTC
MACRO
Explanation
No alternate control group name was specified on a
FLMALTC macro.
Macro
FLMALTC
Project manager response
Place the alternate control group name to be defined
in column 1, preceding the FLMALTC macro, and
assemble the project definition again.
MNOTE ERROR - A TYPE CAN NOT BE
SPECIFIED IF A WILDCARD(*)
BEEN USED IN THE TYPE
PARAMETER
Explanation
On the FLMNPRM macro, it is possible to specify
multiple types; if the wildcard (*) has been specified,
no other values can be specified.
Macro
FLMNPRM
SCLM macro messages (MNOTEs)
© Copyright IBM Corp. 1980, 2024 867

## Page 888

Project manager response
Modify the FLMNPRM macro to specify TYPE=* and
assemble the project definition again.
MNOTE ERROR - ACCT AND EXPACCT
NAMES SAME IN FLMCNTRL AND
FLMALTC: aaaaaaaa
Explanation
The export account data set and the primary account
data set have the same names in the FLMCNTRL macro
and FLMALTC macro aaaaaaaa. This is not allowed.
Macro
FLMAEND
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - ACCT NAME AND ACCT2
NAME ARE THE SAME
Explanation
The name on the ACCT2 parameter must be different
from the name on the ACCT parameter.
Macro
FLMALTC, FLMCNTRL
Project manager response
Change the name on the ACCT2 parameter and
assemble the project definition again.
MNOTE ERROR - ACCT2 AND EXPACCT
NAMES SAME IN FLMCNTRL AND
FLMALTC: aaaaaaaa
Explanation
The export account data set and the primary account
data set have the same names in the FLMCNTRL macro
and FLMALTC macro aaaaaaaa. This is not allowed.
Macro
FLMAEND
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - ADMINID MUST BE
SPECIFIED IF MEMLOCK=Y
Explanation
If member level locking (MEMLOCK=Y) has been
specified on the FLMCNTRL macro, a SCLM
administrator userid must be specified by means of
the ADMINID parameter.
Macro
FLMCNTRL
Project manager response
Specify the SCLM administrator userid in the ADMINID
parameter on the FLMCNTRL macro and assemble the
project definition again.
Note: The userid specified will be able to specify
other SCLM administrators by means of the SCLM
administrator option (option A for the main menu).
MNOTE ERROR - ALL FLMSYSLBs for an
Include Set must be together
Explanation
The include set specified (INCLS keyword) matches
the name on a prior FLMSYSLB statement, but another
include set name was specified in between that one
and this statement. This can also be caused by leaving
the INCLS keyword off, implying the use of the default
set for one of these statements.
Macro
FLMSYSLB
Project manager response
Reorder the FLMSYSLB statement so that all the
specifications for an include set are together.
MNOTE ERROR - BACKUP=Y IS NOT
ALLOWED WITH PACKFILE=Y
Explanation
A type (FLMTYPE) cannot be specified as containing
the package backup information (PACKFILE=Y) and
that, during promotion, these types are to be backed
up (BACKUP=Y).
Macro
FLMTYPE
SCLM macro messages (MNOTEs)
868  z/OS: z/OS ISPF Messages and Codes

## Page 889

Project manager response
Modify the FLMTYPE macro to remove either the
PACKFILE=Y or the BACKUP=Y parameter and
assemble the project definition again.
MNOTE ERROR - BKGRP PARM ON
FLMGROUP HAS BEEN USED IN
A PREVIOUS FLMGROUP. BKGRP
NAME: xxxxxxxx FLMGROUP IN
ERROR: yyyyyyyy
Explanation
The backup group specified by the parameter BKGRP
parameter on the FLMGROUP macro is not unique.
Macro
FLMAEND
Project manager response
Modify FLMGROUP macros to ensure a unique BKGRP
parameter is specified for each group and assemble
the project definition again.
MNOTE ERROR - BKMBRLVL PARM
CANNOT BE SPECIFIED WITHOUT
BKGRP PARM SPECIFIED
Explanation
The member level restore (BKMBRLVL=Y) can only be
specified if the BKGRP parameter has been specified
on a FLMGROUP macro.
Macro
FLMGROUP
Project manager response
Modify the FLMGROUP macro to specify a BKGRP
parameter; also define the backup group using a
FLMGROUP macro and assemble the project definition
again.
MNOTE ERROR - BUFSIZE MUST BE
GREATER THAN 0
Explanation
The BUFSIZE parameter specified on the FLMLANGL
macro was 0. This parameter must be greater than 0.
Macro
FLMLANGL
Project manager response
Specify a BUFSIZE parameter greater than 0 on the
FLMLANGL macro for the language definition and
regenerate the project definition.
MNOTE ERROR - CATLG and VIO
specifications conflict
Explanation
Both CATLG=Y and VIO=Y were specified on the
FLMALLOC statement. However, a cataloged data set
cannot be on a VIO device.
Macro
FLMALLOC
Project manager response
If the processor requires the data set to be cataloged,
then remove the VIO specification (or code VIO=N,
which is implied by CATLG=Y).
MNOTE ERROR - CHECKSUM PARM MUST
BE "YES" OR "NO"
Macro
FLMATVER
Project manager response
Correct the value or remove the CHECKSUM
parameter, which amounts to specifying "YES", and
assemble the project definition again.
MNOTE ERROR - "COMPILE=SELECT"
MUST BE SPECIFIED FOR
"CALLMETH=ISPLNK".
Explanation
The ISPLNK callmeth requires that SELECT be
specified for the compile parameter. The program,
CLIST, or REXX exec to be run is specified in the
OPTIONS parameter using the ISPLINK PGM or CMD
keywords.
System programmer response
Specify SELECT in the COMPILE parameter and
the program, CLIST or REXX exec in the OPTIONS
parameter.
Macro
FLMTRNSL
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  869

## Page 890

MNOTE ERROR - FILEDAT IS ONLY VALID
WHEN IOTYPE=H
Explanation
The FILEDAT parameter on the FLMALLOC macro can
only be specified for a UNIX file (IOTYPE=H).
Macro
FLMALLOC
Project manager response
Either specify the file as UNIX (IOTYPE=H), or remove
the FILEDAT parameter and assemble the project
definition again.
MNOTE ERROR - CONCATENATED UNIX
FILES ARE NOT SUPPORTED
Explanation
Only one UNIX file is allowed when IOTYPE=H is
specified on the FLMALLOC macro.
Macro
FLMCPYLB
Project manager response
Delete all but one of the FLMCPYLB macros associated
with the FLMALLOC macro.
MNOTE ERROR - CONTROL MUST BE
SPECIFIED IF MEMLOCK=Y
Explanation
If member level locking (MEMLOCK=Y) has been
specified on the FLMCNTRL macro, a VSAM control
file must be specified by means of the CONTROL
parameter.
Macro
FLMCNTRL
Project manager response
Specify the VSAM control file in the CONTROL
parameter on the FLMCNTRL macro and assemble the
project definition again.
Note: You may need to have created the VSAM control
file for member level locking to work.
MNOTE ERROR - DDNAME MAY NOT BE
SPECIFIED FOR MULTIPLE ALLOC
Explanation
The same DDNAME value is specified on multiple
FLMALLOC macros for a translator. DDNAMEs must be
unique for each data set allocation for a translator.
Macro
FLMALLOC
Project manager response
Change the duplicate DDNAME value to a unique one
for the translator, and assemble the project definition
again.
MNOTE ERROR - DDNAME MUST BE
SPECIFIED IN FLMALLOC WHEN
PORDER IN FLMTRNSL IS 0 OR 1
Explanation
PORDER values of 0 or 1 on a FLMTRNSL macro
require a DDNAME value on the corresponding
FLMALLOC macro.
Macro
FLMALLOC, FLMTRNSL
Project manager response
Supply a DDNAME value, or change the PORDER value,
and assemble the project definition again.
MNOTE ERROR - DFLTTYP KEYWORD
MISMATCH: aaaaaaaa bbbbbbbb
Explanation
Programmer response
Examine the additional messages to determine where
the DFLTTYP keyword mismatch is located. The
inconsistency can be corrected by editing the language
definition and adjusting the FLMALLOC keyword values
for DFLTTYPE and/or use of the FLMTCOND macro.
Changes to a language definition require a reassembly
and link of the project definitions that use the language
definition.
Macro
FLMAEND
MNOTE ERROR - DINIT=Y NOT VALID FOR
IOTYPE
SCLM macro messages (MNOTEs)
870  z/OS: z/OS ISPF Messages and Codes

## Page 891

Explanation
The value specified on the DINIT conflicts with the
IOTYPE parameters of the FLMALLOC macro. DINIT=Y
requests the initialization of an output data set and is
only valid with IOTYPE values M, P, O, or W.
Macro
FLMALLOC
Project manager response
Remove the DINIT parameter, or change the IOTYPE
value and assemble the project definition again.
MNOTE ERROR - DSNAME VALUE > 44
CHARACTERS, IGNORED
Explanation
A value specified for the DSNAME parameter on the
FLMTRNSL macro exceeds the maximum length of
44 characters for a data set containing an SCLM-
controlled translator.
Macro
FLMCNTRL, FLMTRNSL
Project manager response
Re-specify a data set name of the correct length, or
add the EXLIBID parameter to the FLNTRNSL macro
if the translator defined is specified for an external
library and the EXLIBID parameter was omitted.
Assemble the project definition again.
MNOTE ERROR - DSNTYPE IS ONLY VALID
FOR IOTYPE=P
Explanation
The DSNTYPE parameter determines whether a
temporary partitioned data set is allocated as a PDS
or PDSE. This parameter is only valid when the value
for IOTYPE is P.
Macro
FLMALLOC
Project manager response
Remove the DSNTYPE parameter or change the
IOTYPE value to P, and reassemble and link the project
definition.
MNOTE ERROR - DUPLICATE
DECLARATION FOUND FOR
INCLUDE-SET
Explanation
There are duplicate FLMINCLS macros which specify
the same name within an SCLM language translator.
Macro
FLMINCLS
Project manager response
Remove one of the duplicate FLMINCLS macros and
assemble the project definition again.
MNOTE ERROR - DUPLICATE
DECLARATION FOUND FOR
LANGUAGE: "xxxxxxxx"
Explanation
Language xxxxxxxx is declared on multiple FLMLANGL
macros.
Macro
FLMAEND, FLMLANGL
Project manager response
Change one of the language names, and assemble the
project definition again.
MNOTE ERROR - DUPLICATE FLMNPRM
ROW EXISTS GROUP:xxxxxxxx
TYPE: yyyyyyyy LANG: zzzzzzzz
Explanation
The FLMNPRM macro has been invoked with the same
values for the GROUP, TYPE, and LANG. This causes a
duplicate row in the NOPROM table and is not allowed.
Macro
FLMNPRM
Project manager response
Modify the SCLM project to remove one of the
duplicate FLMNPRM macro entries and assemble the
project definition again.
MNOTE ERROR - DUPLICATE LABEL
SPECIFIED
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  871

## Page 892

Explanation
The label specified on the FLMTRNSL macro was
specified on multiple FLMTRNSL macros.
Macro
FLMTRNSL
Project manager response
Change the FLMTRNSL labels to ensure they are not
duplicated and assemble the project definition again.
MNOTE ERROR - ENCODE=Y ONLY VALID
FOR IOTYPE O OR P
Explanation
It is only possible to encode SCLM outputs if the
IOTYPE is either O or P.
Macro
FLMALLOC
Project manager response
Modify the FLMALLOC MACRO to specify an IOTYPE of
O or P.
MNOTE ERROR - ENDLESS LOOP FOR
GROUP: xxxxxxxx
Explanation
A promotion path of groups has created a loop. The
groups must promote from beginning to end in a linear
fashion.
Macro
FLMAEND, FLMGROUP
Project manager response
Check the various PROMOTE keywords on the
FLMGROUP macros, starting with the xxxxxxxx group
and moving up the promotion hierarchy. Change the
one that is causing the loop, and assemble the project
definition again.
MNOTE ERROR - EXPACCT IS REQUIRED
WHEN EXPXREF SPECIFIED
Explanation
An export VSAM accounting data set must be specified
on the EXPACCT parameter in order to use the
EXPXREF parameter.
Macro
FLMALTC, FLMCNTRL
Project manager response
Remove the EXPXREF parameter, or define an
EXPACCT data set, and assemble the project definition
again.
MNOTE ERROR - EXPACCT NAME AND
ACCT NAME ARE THE SAME
Explanation
The export account data set has the same name as the
primary account data set. This is not allowed.
Macro
FLMCNTRL
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - EXPACCT NAME AND
ACCT2 NAME ARE THE SAME
Explanation
The export account data set has the same name as the
primary account data set. This is not allowed.
Macro
FLMCNTRL
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - EXPXREF NAME AND
XREF NAME ARE THE SAME
Explanation
The export cross-reference data set has the same
name as the cross-reference data set. This is not
allowed.
Macro
FLMCNTRL
SCLM macro messages (MNOTEs)
872  z/OS: z/OS ISPF Messages and Codes

## Page 893

Project manager response
Change the name of the export cross-reference data
set and regenerate the project definition.
MNOTE ERROR - EXPACCT NAME IN
FLMALTC: aaaaaaaa SAME AS
ACCT NAME IN FLMALTC:
bbbbbbbb
Explanation
The export account data set in FLMALTC macro
aaaaaaaa has the same name as the primary account
data set in FLMALTC macro bbbbbbbb. This is not
allowed.
Macro
FLMAEND
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - EXPACCT NAME IN
FLMALTC: aaaaaaaa SAME AS
ACCT2 NAME IN FLMALTC:
bbbbbbbb
Explanation
The export account data set in FLMALTC macro
aaaaaaaa has the same name as the primary account
data set in FLMALTC macro bbbbbbbb. This is not
allowed.
Macro
FLMAEND
Project manager response
Change the name of the export account data set and
regenerate the project definition.
MNOTE ERROR - EXPXREF NAME IN
FLMALTC: aaaaaaaa SAME AS
XREF NAME IN FLMALTC:
bbbbbbbb
Explanation
The export cross-reference data set in FLMALTC macro
aaaaaaaa has the same name as the cross-reference
data set in FLMALTC macro bbbbbbbb. This is not
allowed.
Macro
FLMAEND
Project manager response
Change the name of the export cross-reference data
set and regenerate the project definition.
MNOTE ERROR - EXPXREF REQUIRED
WHEN XREF AND EXPACCT
SPECIFIED
Explanation
An export cross-reference VSAM data set must be
specified on the EXPXREF parameter if the XREF and
EXPACCT parameters are both specified.
Macro
FLMALTC, FLMCNTRL
Project manager response
Remove the XREF and/or EXPACCT parameters, or
define an EXPXREF data set and assemble the project
definition again.
MNOTE ERROR - FLMALLOC KEYREF
AND/OR DFLTTYP INCONSISTENT
Explanation
Inconsistent use of the KEYREF or DFLTTYP keywords
on FLMALLOC macros in conjunction with the use
of FLMTCOND macros was detected. Additional
messages follow.
Programmer response
Examine the additional messages to determine the
inconsistency. Correct the inconsistency by editing
the language definitions and adjusting the FLMALLOC
keyword values for KEYREF and DFLTTYPE and use
of the FLMTCOND macro. Changes to a language
definition require a reassembly and link of the project
definitions that use the language definition.
Macro
FLMAEND
MNOTE ERROR - FLMALLOC REFERENCES
AN UNDEFINED INCLUDE-
SET:xxxxxxxx LANGUAGE:yyyyyyyy
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  873

## Page 894

Explanation
The Language yyyyyyyy contains a FLMALLOC MACRO
which has a reference to INCLUDE-SET xxxxxxx which
has not been defined.
Macro
FLMAEND
Project manager response
Modify the Language yyyyyyyy to specify the INCLUDE-
SET xxxxxxx and assemble the project definition again.
MNOTE ERROR - FLMCPYLB statement
missing after IOTYPE=A
or MALLOC=Y LANGUAGE
NAME: aaaaa TRANSLATOR
NUMBER:bbbbbb ALLOC NUMBER:
ccccc
Explanation
An FLMCPYLB macro is required after any FLMALLOC
macro with either IOTYPE=A or MALLOC=Y.
Macro
FLMAEND
Project manager response
Add the FLMCPYLB macro statement after the
FLMALLOC. Assemble the project definition again.
MNOTE ERROR - FLMINCLS FOUND
BEFORE AN FLMLANGL MACRO
Explanation
An FLMINCLS macro was found before an FLMLANGL
macro.
Macro
FLMINCLS
Project manager response
Move the FLMINCLS macro after the FLMLANGL macro
of the language.
MNOTE ERROR - FLMNPRM MACRO
PARAMETER WAS GREATER
THAN 8 CHARACTERS. XXXXXXX:
yyyyyyyyy
Explanation
The value yyyyyyyy specified for the parameter
XXXXXXXX on the FLMNPRM macro is greater than 8
characters.
Macro
FLMNPRM
Project manager response
Modify the value yyyyyyyy for the XXXXXXXX
parameter so that it is 8 characters or less and
assemble the project definition again.
MNOTE ERROR - FLMTCOND SPECIFIED
WITHOUT MATCHING FLMTRNSL
Explanation
An FLMTCOND macro precedes an FLMTRNSL macro
or more than the maximum FLMTCOND macros are
specified for an FLMTRNSL macro.
Programmer response
Examine the language definitions in the project
definition for an FLMTCOND macro that precedes
an FLMTRNSL macro or that is present with other
FLMTCOND macros following an FLMTRNSL macro.
Edit the language definition for correct use of the
FLMTCOND macro. Reassemble and link the project
definitions that use the language definition.
Macro
FLMTCOND
MNOTE ERROR - FLMTOPTS MUST FOLLOW
AN FLMTRNSL MACRO
Explanation
An FLMTOPTS macro precedes an FLMTRNSL macro.
Programmer response
Examine the language definitions in the project
definition for an FLMTOPTS macro that precedes an
FLMTRNSL macro. Edit the language definition for
correct use of the FLMTOPTS macro. Reassemble
and link the project definitions that use the language
definition.
Macro
FLMTOPTS
SCLM macro messages (MNOTEs)
874  z/OS: z/OS ISPF Messages and Codes

## Page 895

MNOTE ERROR - FLMTYPE EXTEND PARM
USED TO REFERENCE UNDEFINED
TYPE: xxxxxxxx
Explanation
Type name xxxxxxxx specified as the EXTEND
parameter of an FLMTYPE macro is not a valid type
defined in the project definition.
Macro
FLMAEND, FLMTYPE
Project manager response
Make sure all EXTEND type names are defined in
the project definition using FLMTYPE macros, and
assemble the project definition again.
MNOTE ERROR - FLMTYPE MUST NOT
REFERENCE ITSELF VIA® THE
EXTEND PARM
Explanation
The type name on the FLMTYPE macro must not
be used as the type name on the EXTEND keyword
parameter.
Macro
FLMTYPE
Project manager response
Use a different type name on the EXTEND keyword
parameter, and assemble the project definition again.
MNOTE ERROR - GROUP NAME MUST
ALSO BE DEFINED IN FLMGRP
MACRO
Explanation
The value of the GROUP parameter on the FLMATVER
macro is not a valid group defined in the project
definition.
Macro
FLMAEND, FLMATVER, FLMGROUP
Project manager response
Change the value of the GROUP parameter to a valid
group, or add the group with a FLMGROUP macro, and
assemble the project definition again.
MNOTE ERROR - GROUP: xxxxxxxx MUST
BE A KEY GROUP (DEVELOPMENT
GROUP)
Explanation
The development group xxxxxxxx does not have
the value of Y specified for the KEY parameter.
Development groups must be key.
Macro
FLMAEND, FLMGROUP
Project manager response
Change the value of the key parameter to Y, and
assemble the project definition again.
MNOTE ERROR - GROUP: xxxxxxxx NOT
DEFINED IN THE SCLM PROJECT
Explanation
The group xxxxxxxx specified on the FLMNPRM macro
is not defined in the SCLM project.
Macro
FLMXAEND
Project manager response
Modify the group xxxxxxxx on the FLMNPRM macro to
be a group specified in SCLM project and assemble the
project definition again.
MNOTE ERROR - UNIX FILES MUST BE
DEFINED UNDER IOTYPE=H
Explanation
The FILEDAT parameter on the FLMALLOC macro is
invalid. Valid values for the FILEDAT parameter are ' ',
TEXT, or BINARY.
Macro
FLMALLOC, FLMCPYLB
Project manager response
Modify the FILEDAT parameter on the FLMALLOC
macro to specify a valid value and assemble the
project definition again.
MNOTE ERROR - HIERARCHY ALLOCATION
FROM GROUP: xxxxxxxx EXCEEDS
MAXIMUM ALLOWED OF: 123
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  875

## Page 896

Explanation
The hierarchical view from group xxxxxxxx has more
than 123 groups. See z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference for more
information.
Macro
FLMAEND, FLMGROUP
Project manager response
Reduce the number of groups in the view from group
xxxxxxxx, and assemble the project definition again.
MNOTE ERROR - IF IOTYPE=L, PORDER IN
FLMTRNSL MUST BE 2 OR 3
Explanation
The value of PORDER in FLMTRNSL must be 2 or 3
when the IOTYPE value in FLMALLOC is L.
Macro
FLMALLOC, FLMTRNSL
Project manager response
Change the IOTYPE or PORDER value in error, and
assemble the project definition again.
MNOTE ERROR - IF XDEP IS SPECIFIED
ON SOME BUT NOT ALL FLMALTC
MACROS, IT MUST BE SUPPLIED
ON FLMCNTRL
Explanation
The cross-dependency data set was specified on one
of the FLMALTC but not all. You must specify it for all
the FLMALTC MACROS or on the FLMCNTRL MACRO
which will cause this value to be used on the FLMALTC
MACRO where it was not specified.
Macro
FLMALTC
Project manager response
Modify the FLMCNTRL macro to specify the cross-
dependency data set (XDEP parameter).
MNOTE ERROR - Illegal VOLume serial
specified (length)
Explanation
The volume serial does not have 6 characters.
Macro
FLMCPYLB,FLMSYSLB
Project manager response
Enter a valid volume serial.
MNOTE ERROR - INCLS PARAMETER NOT
VALID FOR THIS IOTYPE
Explanation
The FLMALLOC MACRO contains a INCLS parameter
which was specified for an IOTYPE other than 'I'.
Macro
FLMALLOC
Project manager response
Modify the FLMALLOC macro to remove the INCLS
parameter and assemble the project definition again.
MNOTE ERROR - INVALID OR
MISSING FUNCTION PARAMETER
SPECIFIED
Explanation
The specified value of the FUNCTN parameter is invalid
or blank. Allowable values are PARSE, VERIFY, BUILD,
COPY, or PURGE.
Macro
FLMTRNSL
Project manager response
Change the value to an allowable one, or remove the
FUNCTN parameter to use the default of PARSE, and
assemble the project definition again.
MNOTE ERROR - INVALID
OR UNSPECIFIED IOTYPE
PARAMETER
Explanation
The value specified on the IOTYPE parameter is
incorrect. IOTYPE parameter values are A, I, L, M, N,
O, P, S, U, and W.
SCLM macro messages (MNOTEs)
876  z/OS: z/OS ISPF Messages and Codes

## Page 897

Macro
FLMALLOC
Project manager response
Correct the value, and assemble the project definition
again.
MNOTE ERROR - INVALID PATHDSP
KEYWORD
Explanation
The PATHDSP parameter on the FLMALLOC macro is
invalid. Valid values for the PATHDSP parameter are
KEEP or DELETE.
Macro
FLMALLOC
Project manager response
Modify the PATHDSP parameter on the FLMALLOC
macro to specify a valid value and assemble the
project definition again.
MNOTE ERROR - INVALID PATHMDE
KEYWORD
Explanation
The PATHMDE parameter on the FLMALLOC macro
is invalid. Valid values for the PATHMDE parameter
are SIRUSR, SIWUSR, SIXUSR, SIRWXU, SIRGRP,
SIWGRP, SIXGRP, SIRWXG, SIROTH, SIWOTH,
SIXOTH, SIRWXC, SISUID, or SISGID.
Macro
FLMALLOC
Project manager response
Modify the PATHOPTS parameter on the FLMALLOC
macro to specify a valid value and assemble the
project definition again.
MNOTE ERROR - INVALID PATHOPTS
KEYWORD
Explanation
The PATHOPTS parameter on the FLMALLOC macro is
invalid. Valid values for the PATHOPTS parameter are
ORDONLY, OWRONLY, ORDWR, OAPPEND, OCREAT,
OEXCL, NOCTTY, NONBLOCK, OSYNC, or OTRUNC.
Macro
FLMALLOC
Project manager response
Modify the PATHOPTS parameter on the FLMALLOC
macro to specify a valid value and assemble the
project definition again.
MNOTE ERROR - INVALID PROJECT
DEFINITION - FLMxxxxx NOT
SPECIFIED
Explanation
At least one group and type must be defined to
establish a valid SCLM project definition.
Macro
FLMAEND, FLMGROUP, FLMTYPE
Project manager response
Define one or more groups and types using the
FLMGROUP and FLMTYPE macros, and assemble the
project definition again.
MNOTE ERROR - INVALID SAMEAS
VALUE FOR INCLUDE-SET:
xxxxxxxx. REFERENCED INCLUDE-
SET HAS SAMEAS VALUE yyyyyyyy
LANG:zzzzzzzz.
Explanation
The SAMEAS value yyyyyyyy specified on the
FLMINCLS macro xxxxxxxx for the language translator
zzzzzzzz is invalid.
Macro
FLMAEND
Project manager response
Modify the FLMINCLS macro to specify a valid SAMEAS
value for the language zzzzzzzz and assemble the
project definition again.
MNOTE ERROR - INVALID SAMEAS
VALUE FOR INCLUDE-SET:
xxxxxxxx. REFERENCED INCLUDE-
SET UNDEFINED. LANG:yyyyyyyy
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  877

## Page 898

Explanation
The SAMEAS value specified in the SAMEAS parameter
on the FLMINCLS macro xxxxxxxx is not defined in
language translator yyyyyyyy.
Macro
FLMAEND
Project manager response
Modify the FLMINCLS macro to specify an include-
set in the SAMEAS parameter that can be found
in language translator yyyyyyyy, and assemble the
project definition again.
MNOTE ERROR - INVALID SCOPE:
aaaaaaaa FOR LANGUAGE:
bbbbbbbb
Explanation
The value specified on the SCOPE parameter
(aaaaaaaa) is incorrect. SCOPE parameter values are
LIMITED, NORMAL, SUBUNIT or EXTENDED.
Macro
FLMLANGL
Project manager response
Correct the value, and assemble the project definition
again.
MNOTE ERROR - INVALID VALUE FOR
"ACTION" PARAMETER
Explanation
A value other than APPEND or REPLACE was specified
for the ACTION keyword.
Programmer response
Change the value of the ACTION keyword to APPEND
or REPLACE in the FLMTOPTS macro. The FLMTOPTS
macro will follow an FLMTRNSL macro in a language
definition. Reassemble and link the project definitions
that use the language definition.
Macro
FLMTOPTS
MNOTE ERROR - INVALID VALUE FOR
FIELD "ACTION"
Explanation
A value other than RUN or SKIP was specified for the
ACTION keyword.
Programmer response
Change the value of the ACTION keyword to RUN
or SKIP in the FLMTCOND macro. The FLMTCOND
macro will follow an FLMTRNSL macro in a language
definition. Reassemble and link the project definitions
that use the language definition.
Macro
FLMTCOND
MNOTE ERROR - INVALID VALUE FOR
FIELD "INPLIST", DEFAULTED
Explanation
The value specified on the keyword parameter
INPLIST is incorrect. INPLIST parameter values are Y
or N. The default value of N was used.
Macro
FLMTRNSL
Project manager response
Correct the value, and assemble the project definition
again.
MNOTE ERROR - INVALID VALUE FOR THE
FIELD "KEY"
Explanation
The value specified on the keyword parameter KEY is
incorrect. KEY parameter values are Y or N.
Macro
FLMGROUP
Project manager response
Correct the value to Y or N, and assemble the project
definition again.
MNOTE ERROR - Invalid value for VIO
keyword, must be YES or NO
Explanation
The value specified on the keyword parameter VIO is
incorrect. VIO parameter values are YES, NO, Y, or N.
SCLM macro messages (MNOTEs)
878  z/OS: z/OS ISPF Messages and Codes

## Page 899

Macro
FLMALLOC
Project manager response
Correct the value and assemble the project definition
again.
MNOTE ERROR - INVALID VALUE
SPECIFIED FOR FIELD "WHEN"
Explanation
An FLMTCOND macro was specified with the WHEN
keyword that has an invalid value.
Programmer response
Examine the language definitions in the project
definition for an FLMTCOND macro that uses the
WHEN keyword. Edit the language definition for a
correct WHEN keyword value for the FLMTCOND
macro. Reassemble and link the project definitions
that use the language definition.
Macro
FLMTCOND
MNOTE ERROR - KEY GROUP > 123 FOR
GROUP: xxxxxxxx
Explanation
The hierarchical view from group xxxxxxxx has more
than 123 key groups. See z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference for more information.
Macro
FLMAEND, FLMGROUP
Project manager response
Reduce the number of key groups in the view from
group xxxxxxxx, and assemble the project definition
again.
MNOTE ERROR - KEYREF AND DFLTTYP
NOT FOUND: aaaaaaaa bbbbbbbb
Explanation
A language definition contains multiple translators
with the FLMTCOND macro and a WHEN clause
specified. An output allocation for a translator with
the DFLTTYP value aaaaaaaa does not match any of
the output allocations specified for the first translator
found in the language definition with FLMTCOND
and the WHEN clause specified. Additional messages
follow that identify the language definition and
translator that contains the unmatched allocation
statement.
Programmer response
Examine all of the messages to determine the keyword
mismatch. Edit the language definition and make the
KEYREF and DFLTTYP keyword values consistent or
change the use of FLMTCOND. Reassemble and link
the project definitions that use the language definition.
Macro
FLMAEND
MNOTE ERROR - KEYREF KEYWORD
NOT IN PREVIOUS FLMALLOC:
aaaaaaaa
Explanation:
Programmer response
Examine all of the messages to determine the keyword
mismatch. Edit the language definition and make
the KEYREF values consistent or change the use of
FLMTCOND for consistency. Reassemble and link the
project definitions that use the language definition.
Macro
FLMAEND
MNOTE ERROR - KEYREF VALUE NOT
VALID FOR IOTYPE VALUE
Explanation
When a value of S is specified for the IOTYPE
parameter, the KEYREF parameter must be entered
with SINC or INCL.
Macro
FLMALLOC
Project manager response
Change the IOTYPE or KEYREF values and assemble
the project definition again.
MNOTE ERROR - LABEL EXCEEDS 8
CHARACTERS
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  879

## Page 900

Explanation
The FLMTRNSL macro was specified with a label which
was greater than 8 characters in length.
Macro
FLMTRNSL
Project manager response
Modify the label for the build FLMTRNSL macro to be
8 characters or less in length and assemble the project
definition again.
MNOTE ERROR - LANG: xxxxxxxx NOT
DEFINED IN THE SCLM PROJECT
Explanation
The language xxxxxxxx specified on the FLMNPRM
macro is not defined in the SCLM project.
Macro
FLMXAEND
Project manager response
Modify the language xxxxxxxx on the FLMNPRM macro
to be a language specified in SCLM project and
assemble the project definition again.
MNOTE ERROR - LANGUAGE IS ONLY
VALID FOR IOTYPE=O OR P
Explanation
A language can be specified only on the LANG keyword
parameter for IOTYPE values O and P. Different
languages can only be assigned on build outputs.
Macro
FLMALLOC
Project manager response
Change the IOTYPE to O or P, and assemble the project
definition again.
MNOTE ERROR - LANGUAGE NAME
REQUIRED
Explanation
A user-specified pseudonym language name was
not specified on the LANG keyword parameter.
See "FLMLANGL macro" in the z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference for more information.
Macro
FLMLANGL
Project manager response
Add a user-specified pseudonym language name to
the LANG keyword parameter and assemble the
project definition again.
MNOTE ERROR - LANGUAGE NOT
SPECIFIED ON FLMxxxLB MACRO
Explanation
The first FLMCMPLB or FLMSYSLB macro must
have a language specified. See "FLMCMPLB macro"
or "FLMSYSLB macro" in the z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference for more information.
Macro
FLMCMPLB, FLMLANGL, FLMSYSLB
Project manager response
Add a language to the macro, which must correspond
to the correct FLMLANGL language, and assemble the
project definition again.
MNOTE ERROR - MALLOC=Y NOT
ALLOWED FOR IOTYPE VALUE
Explanation
When a value of Y is specified for the MALLOC
parameter, the IOTYPE parameter must be entered
with A or O.
Macro
FLMALLOC
Project manager response
Change the IOTYPE or MALLOC values and assemble
the project definition again.
MNOTE ERROR - MAXLINE MUST BE
GREATER THAN THE MINIMUM
VALUE OF 35
SCLM macro messages (MNOTEs)
880  z/OS: z/OS ISPF Messages and Codes

## Page 901

Explanation
The value specified on the MAXLINE parameter must
be greater than or equal to 35.
Macro
FLMCNTRL
Project manager response
Increase the value and assemble the project definition
again.
MNOTE ERROR - MEMBER IS ONLY VALID
FOR IOTYPE=P
Explanation
A value for the MEMBER parameter can be specified
only when the IOTYPE value is P. Members can
be written only to partitioned data sets, which are
allocated using an IOTYPE value of P.
Macro
FLMALLOC
Project manager response
Remove the MEMBER parameter, or change the
IOTYPE value to P, and assemble the project definition
again.
MNOTE ERROR - MISSING REQUIRED
PARAMETER: ACCT
Explanation
A VSAM accounting data set must be specified on the
ACCT parameter.
Macro
FLMALTC
Project manager response
Add the parameter and value, and assemble the
project definition again.
MNOTE ERROR - NEW FORMAT
PARAMETER USED WITH OLD
FORMAT EXIT USER EXIT:
aaaaaaaa
Explanation
The old format, aaaaaaaa, was used to specify the user
exit routine, but the new format was used to specify
one of the other parameters for the user exit. For
example, BLDEXT1 (old format) was used to specify
the user exit routine, but BLDNTFOP (new format) was
used to specify the options for the user exit.
Programmer response
Update the FLMCNTRL macro in your project definition
to use either all new format parameters OR all old
format parameters for a user exit, then assemble and
link the project definition again.
Macro
FLMCNTRL
MNOTE ERROR - NO GROUPS WERE
SPECIFIED ON THE FLMNPROM
MACRO
Explanation
No groups were specified on the GROUP parameter for
the FLMNPRM macro.
Macro
FLMNPRM
Project manager response
Modify the FLMNPRM macro to specify at least one
value for the GROUP parameter and assemble the
project definition again.
MNOTE ERROR - NO LANGUAGES WERE
SPECIFIED ON THE FLMNPROM
MACRO
Explanation
No languages were specified on the LANG parameter
for the FLMNPRM macro.
Macro
FLMNPRM
Project manager response
Modify the FLMNPRM macro to specify at least one
value for the LANG parameter and assemble the
project definition again.
MNOTE ERROR - NO TYPES WERE
SPECIFIED ON THE FLMNPROM
MACRO
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  881

## Page 902

Explanation
No types were specified on the TYPE parameter for the
FLMNPRM macro.
Macro
FLMNPRM
Project manager response
Modify the FLMNPRM macro to specify at least one
value for the TYPE parameter and assemble the
project definition again.
MNOTE ERROR - NOTGROUP AND
GROUP CAN NOT BE SPECIFIED
TOGETHER
Explanation
The GROUP and NOTGROUP keywords were both
specified in an FLMTOPTS macro.
Programmer response
Remove GROUP or NOTGROUP in the FLMTOPTS
macro. The FLMTOPTS macro will follow an FLMTRNSL
macro in a language definition. Reassemble and link
the project definitions that use the language definition.
Macro
FLMTOPTS
MNOTE ERROR - NOTGROUP SPECIFIED
WITH GROUP, NOTGROUP
IGNORED
Explanation
The NOTGROUP and GROUP keywords were both
specified in an FLMTCOND macro. The NOTGROUP
keyword value will be ignored and the macro will be
used as though only the GROUP keyword value was
specified.
Programmer response
Remove GROUP or NOTGROUP in the FLMTCOND
macro. The FLMTCOND macro will follow an
FLMTRNSL macro in a language definition. Reassemble
and link the project definitions that use the language
definition.
Macro
FLMTCOND
MNOTE ERROR - OLD AND NEW FORMATS
BOTH SPECIFIED FOR USER
EXIT. USER EXITaaaaaaaa USER
EXITbbbbbbbb
Explanation
Both the old format and new format were used to
specify a user exit routine, where aaaaaaaa is the old
format and bbbbbbbb is the new format. For example,
BLDNTF (new format) and BLDEXT1 (old format) both
appear as parameters on the FLMCNTRL macro in the
project definition.
Programmer response
Delete one of the user exit routine specifications used
in the project definition. Use only the new OR old
format. After the change is made, assemble and link
the project definition again.
Macro
FLMCNTRL
MNOTE ERROR - OLD FORMAT
PARAMETER USED WITH
NEW FORMAT EXIT. USER
EXITaaaaaaaa
Explanation
The new format, aaaaaaaa, was used to specify the
user exit routine, but the old format was used to
specify one of the other parameters for the user
exit. For example, BLDNTF (new format) was used to
specify the user exit routine, but BEXTIOP (old format)
was used to specify the options for the user exit.
Programmer response
Update the FLMCNTRL macro in your project definition
to use either all new format parameters OR all old
format parameters for the user exit, then assemble
and link the project definition again.
Macro
FLMCNTRL
MNOTE ERROR - ONE FLMTYPE NEEDS
THE PACKFILE PARM, FOR
PACKAGE BACKOUT TO WORK
Explanation
The BKGRP parameter has been specified on a
FLMGROUP macro; however, a type (FLMTYPE) must
be defined as being a packfile (PACKFILE=Y).
SCLM macro messages (MNOTEs)
882  z/OS: z/OS ISPF Messages and Codes

## Page 903

Macro
FLMAEND
Project manager response
Modify the SCLM project to specify a type as
PACKFILE=Y and assemble the project definition
again.
MNOTE ERROR - OPTIONS PARAMETER
REQUIRED
Explanation
The OPTIONS keyword must be specified.
Programmer response
Examine the language definitions in the project
definition for usage of the FLMTOPTS macro. Add
the OPTIONS keyword to the FLMTOPTS macros that
do not specify the OPTIONS keyword. Reassemble
and link the project definitions that use the language
definition.
Macro
FLMTOPTS
MNOTE ERROR - OUTPUT LANGUAGE NOT
DEFINED
Explanation
The language specified on the LANG parameter of an
FLMALLOC macro is not defined. All languages used as
values for the LANG parameter must be defined using
the FLMLANGL macro.
Macro
FLMAEND, FLMALLOC, FLMLANGL
Project manager response
Change the language specified for LANG, or define the
specified language using FLMLANGL, and assemble
the project definition again.
MNOTE ERROR - PATHDSP IS ONLY VALID
WHEN IOTYPE=H
Explanation
Specification of PATHDSP on the FLMALLOC macro is
only valid for UNIX files (IOTYPE=H).
Macro
FLMALLOC
Project manager response
Either specify the file as UNIX (IOTYPE=H), or remove
the PATHDSP parameter and assemble the project
definition again.
MNOTE ERROR - PATHMDE IS ONLY VALID
WHEN IOTYPE=H
Explanation
Specification of PATHMDE on the FLMALLOC macro is
only valid for UNIX files (IOTYPE=H).
Macro
FLMALLOC
Project manager response
Either specify the file as UNIX (IOTYPE=H), or remove
the PATHMDE parameter and assemble the project
definition again.
MNOTE ERROR - PATHOPTS IS ONLY
VALID WHEN IOTYPE=H
Explanation
Specification of PATHOPTS on the FLMALLOC macro is
only valid for UNIX files (IOTYPE=H).
Macro
FLMALLOC
Project manager response
Either specify the file as UNIX (IOTYPE=H), or remove
the PATHOPTS parameter and assemble the project
definition again.
MNOTE ERROR - PDSDATA VALUE MUST
BE "Y" FOR BUILD/PARSE
Explanation
The specified value on the PDSDATA parameter is
incorrect. The allowable value for build and parse
translators is Y.
Macro
FLMTRNSL
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  883

## Page 904

Project manager response
Change the PDSDATA value to Y, and assemble the
project definition again.
MNOTE ERROR - PRIMARY GROUPS > 123
FOR GROUP: xxxxxxxx
Explanation
The hierarchical view from group xxxxxxxx has more
than 123 groups. See z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference for more
information.
Macro
FLMAEND, FLMGROUP
Project manager response
Reduce the number of groups in the view from group
xxxxxxxx, and assemble the project definition again.
MNOTE ERROR - PROBLEM IN FLMTRNSL
NUMBER: nnnn
Explanation
This error occurs when all of the FLMTRNSL macros
for BUILD translators in a language definition that
use FLMTCOND with the WHEN keyword do not
have identical KEYREF and DFLTTYP keywords on
the FLMALLOC macros. The FLMTRNSL number is
the ordinal of the FLMTRNSL macro in a language
definition. This is one of several messages that
describe the problem.
Programmer response
Examine all of the messages to determine the keyword
mismatch. Browse the project definition. The problem
was found in FLMTRNSL number nnnn in a language
definition. Edit the language definition and make the
KEYREF and DFLTTYP keyword values consistent.
Reassemble and link the project definitions that use
the language definition.
Macro
FLMAEND
MNOTE ERROR - PROBLEM IN LANGUAGE
DEFINITION NUMBER: nnnn
Explanation
This error occurs when all of the FLMTRNSL macros
for BUILD translators in a language definition that
use FLMTCOND with the WHEN keyword do not
have identical KEYREF and DFLTTYP keywords on the
FLMALLOC macros. The language definition number is
the language definition ordinal in the project definition.
This is one of several messages that describe the
problem.
Programmer response
Examine all of the messages to determine the keyword
mismatch. Browse the project definition. The problem
was found in language definition number nnnn in
the project definition. Edit the language definition
and make the KEYREF and DFLTTYP keyword values
consistent. Reassemble and link the project definitions
that use the language definition.
Macro
FLMAEND
MNOTE ERROR - PROMOTION GROUP IS
NOT THE BACKED UP GROUP.
BACKED UP GROUP: xxxxxxxx
BACKUP GROUP: yyyyyyyy
PROMOTION GROUP: zzzzzzzz.
Explanation
The promotion group zzzzzzzz specified in the
parameter PROMOTE on the FLMGROUP macro for
group xxxxxxxx is not the same as the backup group
yyyyyyyy specified on the BKGRP parameter on the
FLMGROUP macro for zzzzzzzz.
Macro
FLMAEND
Project manager response
Modify the FLMGROUP macros for xxxxxxxx and
zzzzzzzz so that the BKGRP parameter on zzzzzzzz and
the PROMOTE parameter on xxxxxxxx are the same
value. Once complete, assemble the project definition
again.
MNOTE ERROR - RECURSIVE ENTRY
CAUSED BY SPECIFIED
AUTHCODE GROUP
Explanation
An authorization code specified on the AC parameter is
also the authorization code group name on the same
FLMAGRP macro.
SCLM macro messages (MNOTEs)
884  z/OS: z/OS ISPF Messages and Codes

## Page 905

Macro
FLMAGRP
Project manager response
Change the authorization code or the group name, and
assemble the project definition again.
MNOTE ERROR - REUSEDAY IS NOT
ALLOWED WITHOUT PACKFILE=Y
Explanation
The REUSEDAY parameter should only be specified
on a type (FLMTYPE) that has been specified as
containing the package backup information.
Macro
FLMTYPE
Project manager response
Either move the REUSEDAY parameter to the
type specified as containing the package backup
information (PACKFILE=Y), or specify this type as
being a packfile (PACKFILE=Y) and assemble the
project definition again.
MNOTE ERROR - REUSEDAY PARM MUST
BE BETWEEN 0 and 9999
Explanation
The REUSEDAY parameter did not specify a value
between 0 and 9999.
Macro
FLMTYPE
Project manager response
Modify the REUSEDAY parameter to specify a value
between 0 and 9999, and assemble the project
definition again.
MNOTE ERROR - SAMEAS VALUE NOT
ALLOWED WITH TYPES VALUE
Explanation
It is not possible to specify the SAMEAS parameter
with the TYPES parameter on the FLMINCLS macro.
Macro
FLMINCLS
Project manager response
Remove either the SAMEAS parameter or the TYPES
parameter, and assemble the project definition again.
MNOTE ERROR - SEQNUM PARM MUST
BE "NONE", "COBOL" OR
"STANDARD"
Explanation
The sequence number parameter for versioning
(SEQNUM) must be either NONE, COBOL, or
STANDARD.
Macro
FLMATVER
Project manager response
Modify the SEQNUM parameter on the FLMATVER
macro to be a either NONE, COBOL, or STANDARD, and
assemble the project definition again.
MNOTE ERROR - SPECIFICATION OF
THIS MACRO EXCEEDS MAXIMUM
ALLOWED
Explanation
Each of these macros can be specified a limited
number of times in a project definition. In each case,
the maximum number is 32 000.
Macro
FLMALLOC, FLMATVER, FLMCPYLB, FLMEXLIB,
FLMGROUP, FLMLANGL, FLMTRNSL, FLMTYPE
Project manager response
Reduce the number of instances of the macro
producing the error, and assemble the project
definition again.
MNOTE ERROR - SPECIFIED AUTHCODE
ALREADY PROCESSED AS A
MEMBER
Explanation
The name of the authorization code group has already
been used as one of the authorization codes specified
on the AC parameter of a previous FLMAGRP macro.
Macro
FLMAGRP
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  885

## Page 906

Project manager response
Change the name of the group, or remove the bad
authorization code from the AC parameter of a
previous FLMAGRP macro, and assemble the project
definition again.
MNOTE ERROR - SPECIFIED AUTHCODE
GROUP CONTAINS DUPLICATE
MEMBERS
Explanation
The authorization code group name specified has
already been used as a group name on a previous
FLMAGRP macro.
Macro
FLMAGRP
Project manager response
Change the name of the authorization code group, and
assemble the project definition again.
MNOTE ERROR - SPECIFIED AUTHCODE
GROUP CONTAINS NO MEMBERS
Explanation
No authorization codes were specified on the AC
parameter. An authorization code group must contain
one or more authorization codes.
Macro
FLMAGRP
Project manager response
Add one or more authorization codes to the AC
parameter, or remove the FLMAGRP macro, and
assemble the project definition again.
MNOTE ERROR - SPECIFIED FLMALTC HAS
BEEN PREVIOUSLY DEFINED
Explanation
The alternate control name specified has already been
defined using an FLMALTC macro.
Macro
FLMALTC
Project manager response
Change one of the duplicate names, and assemble the
project definition again.
MNOTE ERROR - SPECIFIED GROUP HAS
BEEN PREVIOUSLY DEFINED
Explanation
The group name specified has already been defined
using an FLMGROUP macro.
Macro
FLMGROUP
Project manager response
Change one of the duplicate names, and assemble the
project definition again.
MNOTE ERROR - SPECIFIED LANGUAGE
WAS PREVIOUSLY DEFINED
Explanation
The language name specified was used on a
previous FLMCMPLB or FLMSYSLB macro. One unique
language can be used per FLMCMPLB or FLMSYSLB
concatenation. See "FLMCMPLB macro" or "FLMSYSLB
macro" in the z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference for more
information.
Macro
FLMCMPLB, FLMSYSLB
Project manager response
Change the language name and assemble the project
definition again.
MNOTE ERROR - SPECIFIED MACRO ONLY
VALID FOR BUILD TRANSLATORS
Explanation
A macro that is restricted to BUILD translators
was used with an FLMTRNSL that did not specify
FUNCTION=BUILD.
Programmer response
Examine the language definitions in the project
definition for usage of restricted macros with
an FLMTRNSL macro that does not specify
FUNCTION=BUILD. Edit the language definition for
SCLM macro messages (MNOTEs)
886  z/OS: z/OS ISPF Messages and Codes

## Page 907

correct use of the restricted macros. Reassemble
and link the project definitions that use the language
definition.
Macro
FLMTCOND, FLMTOPTS
MNOTE ERROR - SPECIFIED TYPE WAS
PREVIOUSLY DEFINED
Explanation
The type name has already been specified on a
previous FLMTYPE macro.
Macro
FLMTYPE
Project manager response
Change the type name, and assemble the project
definition again.
MNOTE ERROR - TASKLIB IS ONLY VALID
FOR BUILD TRANSLATORS
Explanation
TASKLIB can only be specified for build translators.
Macro
FLMTRNSL
Project manager response
Specify FUNCTN=BUILD on the FLMTRNSL macro
when using the TASKLIB keyword.
MNOTE ERROR - TASKLIB VALUE > 8
CHARACTERS, IGNORED
Explanation
The DDNAME specified for the TASKLIB keyword was
more than 8 characters.
Macro
FLMTRNSL
Project manager response
Change the DDNAME to one that has 8 characters or
less.
MNOTE ERROR - THE BKGRP DOES NOT
EXIST AS A FLMGROUP. BKGRP
NAME: xxxxxxxx
Explanation
The FLMGROUP macro being processed was
previously specified as a BKGRP on another
FLMGROUP macro. This current FLMGROUP macro
cannot specify the backup group (BKGRP) xxxxxxxx.
Macro
FLMAEND
Project manager response
Modify the FLMGROUP macro to remove the BKGRP
and assemble the project definition again.
MNOTE ERROR - THE FLMGROUP FOR A
BACKUP GROUP CANNOT ALSO
USE THE BKGRP KEYWORD.
BKGRP NAME: xxxxxxxx
Explanation
The backup group BKGRP=xxxxxxxx has been set up
on a FLMGROUP macro but a FLMGROUP macro has
not been set up for the group xxxxxxxx.
Macro
FLMAEND
Project manager response
Modify the project definition to specify a FLMGROUP
macro for xxxxxxxx and assemble the project definition
again.
MNOTE ERROR - THE xxxxxxxx PARM
VALUE MUST BE Y OR N
Explanation
A value other than Y or N was specified for the
parameter xxxxxxxx.
Macro
FLMTYPE
Project manager response
Modify the parameter xxxxxxxx to specify either Y or N,
and assemble the project definition again.
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  887

## Page 908

MNOTE ERROR - TYPE NAME MUST ALSO
BE DEFINED IN FLMTYPE MACRO
Explanation
The value of the TYPE parameter on the FLMATVER
macro is not a valid type defined in the project
definition.
Macro
FLMAEND, FLMATVER, FLMTYPE
Project manager response
Change the value of the TYPE parameter to a valid
type, or add the type with an FLMTYPE macro, and
assemble the project definition again.
MNOTE ERROR - TYPE: xxxxxxxx NOT
DEFINED IN THE SCLM PROJECT
Explanation
The type xxxxxxxx specified on the FLMNPRM macro is
not defined in the SCLM project.
Macro
FLMXAEND
Project manager response
Modify the type xxxxxxxx on the FLMNPRM macro to
be a type specified in SCLM project and assemble the
project definition again.
MNOTE ERROR - UNABLE TO FIND
FLMALTC: xxxxxxxx
Explanation
The alternate control name specified on the ALTC
parameter of the FLMGROUP macro is not defined.
Macro
FLMAEND, FLMALTC, FLMGROUP
Project manager response
Define the alternate control name using FLMALTC, or
correct the ALTC parameter in error, and assemble the
project definition again.
MNOTE ERROR - UNABLE TO FIND
PROMOTE GROUP: xxxxxxxx
Explanation
The group named xxxxxxxxx was specified on
the PROMOTE parameter of an FLMGROUP macro.
However, that group name is not a valid group.
Macro
FLMAEND, FLMGROUP
Project manager response
Change the group name on the PROMOTE parameter,
or add the group using an FLMGROUP macro, and
assemble the project definition again.
MNOTE ERROR - UNDEFINED TYPE
REFERENCED BY THE INCLUDE-
SET TYPE:xxxxxxxx
Explanation
The TYPE parameter on the FLMINCLS macro
references a type xxxxxxxx which is not defined in the
SCLM project.
Macro
FLMINCLS
Project manager response
Modify the FLMINCLS macro to specify a type found in
the SCLM project definition and assemble the project
definition again.
MNOTE ERROR - VALUE FOR "PDSDATA"
MUST MATCH FOR ALL
TRANSLATORS OF THE SAME
TYPE WITHIN A LANGUAGE
Explanation
Multiple translators of any function type (such as
PARSE, BUILD and COPY) for the same language must
all specify the same PDSDATA value.
Macro
FLMTRNSL
Project manager response
Change the PDSDATA values for the translators, and
assemble the project definition again.
MNOTE ERROR - VERCOUNT MUST BE
ZERO OR >= 2
SCLM macro messages (MNOTEs)
888  z/OS: z/OS ISPF Messages and Codes

## Page 909

Explanation
The VERCOUNT parameter on the FLMCNTRL or
FLMATVER macros must either be '0' or greater than
or equal to '2'.
Macro
FLMCNTRL, FLMATVER
Project manager response
Modify the VERCOUNT on the FLMCNTRL or FLMATVER
macro to be a either '0' or greater than or equal to '2',
and assemble the project definition again.
MNOTE ERROR - VERS IS REQUIRED
WHEN xxxxxxxx SPECIFIED
Explanation
The VERS parameter must be specified when xxxxxxxx
is VERS2 or VERPDS. VERS specifies a VSAM audit
control data set name, which must be present when
specifying a secondary VSAM audit control data set
(VERS2), or version data PDS (VERPDS).
Macro
FLMALTC, FLMCNTRL
Project manager response
Add a VERS parameter and value, and assemble the
project definition again.
MNOTE ERROR - VERS NAME AND VERS2
NAME ARE THE SAME
Explanation
The name on the VERS2 parameter must be different
from the name on the VERS parameter.
Macro
FLMALTC, FLMCNTRL
Project manager response
Change the name on the VERS2 parameter and
assemble the project definition again.
MNOTE ERROR - "VERS" REQUIRED IN
FLMALTC FOR VERSION/AUDIT
FLMALTC: xxxxxxxx
Explanation
Audit and/or version control has been enabled using
an FLMATVER macro, but no VSAM audit control
data set has been specified on the FLMALTC macro
associated with the group being audited/versioned.
The FLMALTC alternate control definition in error is
named xxxxxxxx.
Macro
FLMAEND, FLMALTC, FLMATVER
Project manager response
Add a VERS parameter and data set name to the
FLMALTC macro in error, or remove the FLMATVER
macro, and assemble the project definition again.
MNOTE ERROR - "VERS" REQUIRED IN
FLMCNTRL FOR VERSION/AUDIT
Explanation
Audit and/or version control has been enabled using a
FLMATVER macro, but no VSAM audit control data set
has been specified on the FLMCNTRL macro.
Macro
FLMAEND, FLMATVER, FLMCNTRL
Project manager response
Add a VERS parameter and data set name to the
FLMCNTRL macro, or remove the FLMATVER macro,
and assemble the project definition again.
MNOTE ERROR - VOLume keyword is not
accepted with NULLFILE specified
Explanation
The data set name specified on an FLMCPYLB
statement is NULLFILE (indicating a dummy data set
allocation) along with the VOL=xxxxxx keyword. A
volume specification for a dummy data set is not
allowed.
Macro
FLMCPYLB
Project manager response
Remove VOL=xxxxxx specification and assemble the
project definition again.
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  889

## Page 910

MNOTE ERROR - XDEPUPD parm value
must be Y or N
Explanation
The XDEPUPD parameter indicates whether the cross-
dependency data set is to be kept in sync with the
accounting data set. This parameter must be either Y
or N.
Macro
FLMCNTRL
Project manager response
Modify the XDEPUPD parameter to be either Y or N.
MNOTE ERROR - XREF AND EXPXREF
NAMES SAME IN FLMCNTRL,
FLMALTC: aaaaaaaa
Explanation
The export cross-reference data set and the cross-
reference data set have the same names in the
FLMCNTRL macro and FLMALTC macro aaaaaaaa.
This is not allowed.
Macro
FLMAEND
Project manager response
Change the name of the export cross-reference data
set and regenerate the project definition.
MNOTE ERROR - XREF IS REQUIRED
WHEN EXPXREF SPECIFIED
Explanation
A VSAM cross reference data set must be specified
on the XREF parameter in order to use the EXPXREF
parameter.
Macro
FLMALTC, FLMCNTRL
Project manager response
Remove the EXPXREF parameter, or define an XREF
data set, and assemble the project definition again.
MNOTE ERROR - xxxxxxxx PARAMETER IS
REQUIRED
Explanation
The xxxxxxxx parameter must be specified with an
acceptable value. Versioning must be enabled for
specific groups and types. See "FLMATVER macro"
in the z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference for more information.
Macro
FLMATVER
Project manager response
Add the missing parameter and a corresponding value
and assemble the project definition again.
MNOTE WARNING - DASDUNIT NAME > 8
CHARS, TRUNCATED
Explanation
A value specified for the DASDUNIT parameter
exceeded the maximum length of eight characters.
The name specified was truncated to 8 characters.
Macro
FLMCNTRL
Project manager response
If the truncated value is unacceptable, change it to the
correct length, and reassemble the project definition.
MNOTE WARNING - EXTENDED TYPE
NAME > 8 CHARS, TRUNCATED
Explanation
The type name specified on the EXTEND keyword
parameter is too long. Type names can only a
maximum of 8 characters. The name specified was
truncated to 8 characters.
Macro
FLMTYPE
Project manager response
If the truncated type name is unacceptable, change
it to the correct length, and assemble the project
definition again.
MNOTE WARNING - FIELD "DISP" IS
IGNORED FOR IOTYPES L, N, AND
U
SCLM macro messages (MNOTEs)
890  z/OS: z/OS ISPF Messages and Codes

## Page 911

Explanation
A nonblank value was specified for the DISP
parameter on an FLMALLOC macro with IOTYPE=L, N,
or U. Because these IOTYPEs do not allocate a data
set, disposition has no meaning. The value specified is
ignored.
Macro
FLMALLOC
Project manager response
Change the IOTYPE on the macro to allocate a data
set or remove the DISP parameter from the macro.
Reassemble and link the project definition.
MNOTE WARNING - FLMALTC Macro
Name: "xxx(8)" NEVER
REFERENCED BY AN FLMGROUP
MACRO
Explanation
The alternate control named xxxxxxxx was not used as
an ALTC value in any FLMGROUP macro. That alternate
control will not be used.
Macro
FLMAEND, FLMALTC, FLMGROUP
Project manager response
Correct the FLMALTC macro, or the ALTC parameter
on the FLMGROUP macro, and assemble the project
definition again.
MNOTE WARNING - FLMALTC NAME
GREATER THAN 8 CHARS,
TRUNCATED
Explanation
The name on the FLMALTC macro is too long. The
name is truncated to 8 characters.
Macro
FLMALTC
Project manager response
Use a name with 8 or fewer characters on the
FLMALTC macro statement. Assemble the project
definition again.
MNOTE WARNING - FLMSYSLB LIBRARIES
FOR LANGUAGE: xxxxxxxx
EXCEEDS MAXIMUM OF
123. REMAINING LIBRARIES
IGNORED. INCLUDE-SET: zzzzzzzz.
Explanation
The INCLUDE-SET: xxxxxxxx was specified with more
than 123 libraries.
Macro
FLMAEND, FLMXAEND
Project manager response
Modify the INCLUDE-SET to specify 123 libraries or
less and assemble the project definition again.
MNOTE WARNING - FLMxxxLB LIBRARIES
FOR LANGUAGE: xxxxxxxx EXCEED
MAXIMUM OF: 123. REMAINING
LIBRARIES IGNORED.
Explanation
The number of libraries concatenated using
FLMCMPLB or FLMSYSLB for language xxxxxxxx is too
high. Only the first 123 libraries will be concatenated.
Macro
FLMAEND, FLMCMPLB, FLMSYSLB
Project manager response
If the result of ignoring some libraries is unacceptable,
reduce the number of concatenated libraries, and
assemble the project definition again.
MNOTE WARNING - INVALID VALUE FOR
FIELD "CALLMETH"
Explanation
The specified value for the CALLMETH parameter
is incorrect. Allowable values are ATTACH, LINK,
TSOLNK, and ISPLNK.
Macro
FLMCNTRL, FLMTRNSL
Project manager response
Correct the value and assemble the project definition
again.
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  891

## Page 912

MNOTE WARNING - INVALID VALUE FOR
FIELD "KEYREF", IGNORED
Explanation
The value specified on the keyword parameter KEYREF
is incorrect. It must be a valid build map or
architecture definition keyword. The keyword and the
specified value were ignored.
Macro
FLMALLOC
Project manager response
Check the values in z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference. Change the
value to an acceptable one, and assemble the project
definition again.
MNOTE WARNING - INVALID VALUE FOR
FIELD "xxxxxxxx", DEFAULTED
Explanation
The value specified on the keyword parameter
xxxxxxxx is incorrect. The default value was used.
Macro
FLMALLOC, FLMATVER, FLMCNTRL, FLMLANGL,
FLMTRNSL
Project manager response
Check the values for the macro keyword in the
topic about "SCLM Macros" in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference. Change the value to an acceptable one, and
assemble the project definition again.
MNOTE WARNING - IOTYPE INVALID FOR
TRANSLATOR FUNCTION
Explanation
The value specified on the IOTYPE parameter is not
valid for the FUNCTN parameter specified in the
FLMTRNSL macro.
Macro
FLMALLOC
Project manager response
Change the IOTYPE specified to one that is valid for
the translator function and reassemble the project
definition.
MNOTE WARNING - LABEL INVALID
FOR xxxxxxxx. TRANSLATOR
DEFAULTED TO BLANK.
Explanation
The build FLMTRNSL macro (FUNCTN=BUILD) was
specified with a label. This label was defaulted to
blanks.
Macro
FLMTRNSL
Project manager response
Remove the invalid label from the build FLMTRNSL
macro.
MNOTE WARNING - LANGDESC GREATER
THAN 40 CHARS, TRUNCATED
Explanation
The language description specified by parameter
LANGDESC on the FLMLANGL macros is greater
than 40 characters. The LANGDESC parameter was
truncated to 40 characters.
Macro
FLMLANGL
Project manager response
Modify the LANGDESC to be 40 characters or less and
assemble the project definition again.
MNOTE WARNING - LANGUAGE NAME
GREATER THAN 8 CHARS,
TRUNCATED
Explanation
The user-specified pseudonym language name on the
LANG keyword parameter is too long. Language names
can be up to 8 characters. The name specified was
truncated to 8 characters.
Macro
FLMLANGL
SCLM macro messages (MNOTEs)
892  z/OS: z/OS ISPF Messages and Codes

## Page 913

Project manager response
If the truncated language name is unacceptable,
change it to the correct length, and assemble the
project definition again.
MNOTE WARNING - LANGUAGE: xxxxxxxx
NOT DEFINED. FLMxxxLB
LIBRARIES IGNORED.
Explanation
Language xxxxxxxx specified on a FLMCMPLB or
FLMSYSLB library concatenation is not defined by
any FLMLANGL macro. The library concatenation
associated with language xxxxxxxx was ignored.
Macro
FLMAEND, FLMLANGL, FLMCMPLB, FLMSYSLB
Project manager response
Define the language using FLMLANGL, or correct the
language on the FLMCMPLB or FLMSYSLB macro, and
assemble the project definition again.
MNOTE WARNING - MAXVIO MUST
BE LESS THAN 2147483648,
DEFAULTED TO 5000
Explanation
The value specified on the MAXVIO parameter is too
high. The maximum value is 2 147 483 647. The
default value of 5000 was used.
Macro
FLMCNTRL
Project manager response
If the default value is not acceptable, reduce the
specified MAXVIO value, and assemble the project
definition again.
MNOTE WARNING - MULTIPLE BRANCHES
DETECTED FOR NON-KEY GROUP:
xxxxxxxx
Explanation
Non-key group xxxxxxxxx has multiple groups
promoting into it. Any group with more than one
lower group promoting into it should be key. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
Macro
FLMAEND, FLMGROUP
Project manager response
Change the group structure to remove the condition
if it is unacceptable. Then assemble the project
definition again.
MNOTE WARNING - NO COMMAS
ALLOWED IN AUTHCODE xxxxxxxx
AUTHCODE IGNORED
Explanation
Authorization code xxxxxxxx contains commas.
Commas are not allowed in authorization codes. The
authorization code will be ignored.
Macro
FLMAGRP, FLMGROUP
Project manager response
Remove the invalid authorization code and reassemble
and relink the project definition.
MNOTE WARNING - SPECIFIED GROUP
CONTAINS NO AUTHCODES
Explanation
No authorization codes or groups were specified on
the AC parameter. The AC parameter can be omitted
but doing so prevents members from being edited in
the group. In addition, no editable members can be
promoted into or out of the group.
Macro
FLMGROUP
Project manager response
If omitting the authorization codes or groups is too
restrictive, add some values to the AC parameter, and
assemble the project definition again.
MNOTE WARNING - TASKLIB IS IGNORED
UNLESS CALLMETH IS ATTACH
Explanation
TASKLIB is only valid when the CALLMETH is ATTACH.
The TASKLIB keyword will be ignored when the
CALLMETH is LINK or TSOLNK.
SCLM macro messages (MNOTEs)
Chapter 4. SCLM macro messages (MNOTEs)  893

## Page 914

Macro
FLMTRNSL
Project manager response
Specify ATTACH for the CALLMETH keyword.
MNOTE WARNING - VIOUNIT NAME > 8
CHARS, TRUNCATED
Explanation
A value specified for the VIOUNIT parameter exceeded
the maximum length of 8 characters. The name
specified was truncated to 8 characters.
Macro
FLMCNTRL
Project manager response
If the truncated value is unacceptable, change it to
the correct length, and assemble the project definition
again.
MNOTE WARNING - xxxxxxxx NAME
GREATER THAN 8 CHARACTERS,
TRUNCATED
Explanation
The specified group, type, alternate control (ALTC)
or version name xxxxxxxx is too long. Group, type,
alternate control or version names can only be 1-8
characters. The name specified was truncated to 8
characters.
Macro
FLMALTC, FLMGROUP, FLMLANGL, FLMTSEXT,
FLMTYPE
Project manager response
If the truncated name is unacceptable, change the
name to the correct length, and assemble the project
definition again.
MNOTE ERROR - INVALID MODULE
LOCATION "LOC=xxxxx" MUST BE
EITHER ABOVE OR BELOW
Explanation
The module location specified by the parameter LOC
on the FLMABEG macro is incorrect. It must be either
ABOVE or BELOW.
Macro
FLMABEG
Project manager response
Modify the LOC parameter on the FLMABEG macro
to specify a valid value and assemble the project
definition again.
SCLM macro messages (MNOTEs)
894  z/OS: z/OS ISPF Messages and Codes
