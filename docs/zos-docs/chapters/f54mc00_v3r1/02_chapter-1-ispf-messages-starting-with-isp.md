# Chapter 1. ISPF messages starting with ISP

Source file: f54mc00_v3r1.md
Start page: 21
Page span: 21-336

## Page 21

Chapter 1. ISPF messages starting with ISP
ISPA001 Allocation error message - ISPF
system data set allocation error -
press Enter to continue.
Explanation
This is an informational message. This message
precedes further messages for which ISPF system
data received the allocation error.
User response
Press Enter.
ISPA002 List allocate err msg - List file
allocation error - ISPF will operate
without a list data set.
Explanation
A list data set allocation error occurred; ISPF will
operate without a list data set.
System programmer response
Diagnose the allocation error. Contact IBM support.
User response
Continue to use ISPF. Contact your system
programmer if message recurs.
ISPA003 Log allocate err msg - Log file
allocation error - ISPF will operate
without a log data set.
Explanation
A log data set allocation error occurred; ISPF will
operate without a log data set.
System programmer response
Diagnose the allocation error. Contact IBM support.
User response
Continue to use ISPF. Contact your system
programmer if message recurs.
ISPA004 LISTX allocate err msg -
Temporary listing data set cannot
be allocated.
Explanation
The ISPF temporary listing data set cannot be
allocated.
System programmer response
Diagnose the allocation error using the appropriate
IBM documentation. Contact IBM support.
User response
Note the error message and text. Contact your system
programmer.
ISPA005 CNTLX allocate err msg -
Temporary control card data set
cannot be allocated.
Explanation
The ISPF temporary ISPCTLx or ISPWRKx data set
cannot be allocated.
System programmer response
Diagnose the allocation error using the appropriate
IBM documentation. Contact IBM support.
User response
Note the error message and text. Contact your system
programmer.
ISPA006 EDITX allocate err msg - Edit
backup data set cannot be
allocated.
Explanation
This message is self explanatory.
ISPA007 Open error message - Error trying
to open 'aaaaaaaa'.
Explanation
An error occurred while trying to open aaaaaaaa.
System programmer response
Diagnose the open error for aaaaaaaa using the
appropriate IBM documentation. Contact IBM support.
ISPF messages starting with ISP
© Copyright IBM Corp. 1980, 2024 1

## Page 22

User response
Note the message and text. Contact your system
programmer.
ISPA107 Open error message - Error trying
to open data set.
Explanation
An error occurred while trying to open the ISPF system
data set.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPA008 Invalid temp ds-qual. - The
temporary data set qualifier is
invalid. The log, list and temporary
cntl data sets will be allocated
without it.
Explanation
The temporary data set qualifier, specified in the
configuration utility, did not adhere to the qualifier
naming convention.
System programmer response
Correct the qualifier in the configuration utility.
User response
Contact your system programmer.
ISPA108 Error from DAIR routine - An error
was encountered while running
the TSO IKJDAIR routine.
Explanation
An error was encountered while running the TSO
IKJDAIR routine.
System programmer response
Diagnose the DAIR error codes. Contact IBM support.
User response
Note the error message. Contact your system
programmer.
ISPA111 Invalid prefix length - Data set
prefix length specified in exit must
be greater than or equal to 1 and
less than or equal to 27.
Explanation
An invalid prefix length has been specified by EXIT 16:
Log, List, and Temporary Data Set Allocation Exit.
User response
The data set prefix length specified in EXIT 16 must be
greater than or equal to 1 and less than or equal to 26.
Refer to EXITS in the ISPF Planning and Customizing
for information on EXIT 16.
ISPA300 APL2® interface error - Invalid
APL2 interface chain manager
request: aaaaaaaa.
Explanation
An invalid request was made to the storage chain
manager.
System programmer response
This could be either a system error or an ISPF internal
error. If you do not have mixed levels of ISPF code,
contact IBM support.
User response
Contact the responsible programmer.
Programmer response
A request other than GET, FREE, or FREEALL was made
to the storage chain manager. This could also be an
ISPF internal error. If you feel that your program is not
at fault, contact the system programmer.
ISPA301 APL2 interface error - Unable to
free APL2 environment chain.
Explanation
There was an error while ISPF was attempting to free
storage.
System programmer response
This is a possible system or ISPF internal error. If you
do not have mixed levels of ISPF code, contact IBM
support.
ISPF messages starting with ISP
2  z/OS: z/OS ISPF Messages and Codes

## Page 23

User response
Contact your system programmer.
ISPA302 APL2 interface error - Unable to
get block for APL2 environment
chain.
Explanation
ISPF was unable to obtain additional storage.
User response
Contact your system programmer.
Programmer response
Verify that the user's region size is adequate for
running the program that failed. If the region size is
adequate, contact IBM support.
ISPA310 APL2 interface error - Invalid
APL2 driver request: aaaaaaaa.
Explanation
The APL2 driver request was not a valid function.
System programmer response
This is a possible system error or ISPF internal error.
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
Contact the responsible programmer.
Programmer response
If the program function was valid, this may be a
system error or an ISPF internal error. Contact your
system programmer.
ISPA311 APL2 interface error - APL2 driver
is unable to aaaaaaaa storage.
Explanation
There was an error in either obtaining or freeing
storage.
User response
Contact the system programmer.
Programmer response
This is a possible system error or an ISPF internal
error. If the error occurred during a GET command,
verify that the user region size is adequate for running
the program. If the region size is adequate or the
error occurred during a FREE command, contact IBM
support.
ISPA320 APL2 interface error - Invalid
APL2 environment manager
request: aaaaaaaa, direction:
bbbbbbbb.
Explanation
The program request is not a valid ISPF APL2 request.
System programmer response
This is a possible system or internal ISPF error. If you
do not have mixed levels of ISPF code, contact IBM
support.
User response
Contact the responsible programmer.
Programmer response
If the request is a valid program request, should
contact the system programmer.
ISPA321 APL2 interface error - APL2
environment manager is unable to
aaaaaaaa storage.
Explanation
There was an error in an ISPF storage GET or FREE
command.
System programmer response
This is a possible system error or ISPF internal error. If
the error is a GET command, verify that the user region
size is adequate for running the program. If the region
size is adequate or the error is a FREE, contact IBM
support.
User response
Contact your system programmer.
ISPA322 ISPF/APL2 link error - The SELECT
service request to initiate APL2
requires keyword: lang(APL).
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  3

## Page 24

Explanation
There was an invalid SELECT service request;
lang(APL) was omitted.
User response
Contact the responsible programmer.
Programmer response
Correct the SELECT statement so that it includes
lang(APL).
ISPA323 ISPF/APL2 link ended - The APL2
side has ended, but there are ISPF
service requests outstanding.
Explanation
APL has ended prematurely.
User response
Contact the responsible programmer.
Programmer response
Ensure that APL does not end until the ISPF service
requests have completed. For additional information,
read "Interface between ISPF and APL2" in z/OS ISPF
Dialog Developer's Guide and Reference.
ISPA324 Invalid APL2 selection -
Outstanding APL2 workspace
requests for ISPF services must be
terminated.
Explanation
The APL program has failed.
User response
Contact the responsible programmer.
Programmer response
Cleanup mode was active when an APL select
was initiated. For more information, read "Interface
between ISPF and APL2" in z/OS ISPF Dialog
Developer's Guide and Reference.
ISPA325 aaaaaaaa variable request -
Outstanding APL2 workspace
requests for ISPF services must be
terminated.
Explanation
The APL program failed.
User response
Contact the responsible programmer.
Programmer response
Cleanup mode was active when an APL variable
service was initiated. For more information, read
"Interface between ISPF and APL2" in z/OS ISPF
Dialog Developer's Guide and Reference .
ISPA326 ISPF/APL2 link damaged -
APL2 request has ended, but
there are ISPF service requests
outstanding.
Explanation
The APL program failed.
User response
Contact the responsible programmer.
Programmer response
The APL request was ended prematurely. For more
information read, "Interface between ISPF and APL2"
in z/OS ISPF Dialog Developer's Guide and Reference .
ISPA330 APL2 workspace error - Function:
aaaaaaaa, variable: bbbbbbbb,
return code: cccccccc.
Explanation
The variable service shown was unsuccessful.
User response
Contact the responsible programmer.
Programmer response
Verify that the program is coded correctly for use of
the variable shown in the message.
ISPA331 APL2 workspace error - Function:
list all variables, return code:
aaaaaaaa.
Explanation
An error in the list all variables function caused a
program failure.
ISPF messages starting with ISP
4  z/OS: z/OS ISPF Messages and Codes

## Page 25

System programmer response
This is a possible system or ISPF internal error. If you
do not have mixed levels of ISPF code, contact IBM
support.
User response
Contact the responsible programmer.
Programmer response
Verify that the APL program is coded correctly for
this function. If the list all variables function is coded
correctly, contact the system programmer.
ISPA332 APL2 interface error - APL2
variable services exit routine is
unable to aaaaaaaa storage.
Explanation
The program was unsuccessful because of a storage
management problem.
System programmer response
This is a possible system or ISPF internal error. If you
do not have mixed levels of ISPF code, contact IBM
support.
User response
Contact your system programmer.
ISPA333 Data truncation occurred - Data for
APL2 variable list is too long.
Explanation
The program was unsuccessful because of variable list
truncation.
User response
Contact the responsible programmer.
ISPA334 APL2 variable error - The value
for variable aaaaaaaa is not a
character scalar or vector.
Explanation
The program was unsuccessful because of an incorrect
variable value.
User response
Contact the responsible programmer.
Programmer response
Correct the variable value.
ISPA335 APL2 interface error - Invalid
auxiliary processor variable
services request: aaaaaaaa.
Explanation
The request name listed was not recognized.
System programmer response
This is a possible system or internal error. If you do not
have mixed levels of ISPF code, contact IBM support.
User response
Contact the responsible programmer.
Programmer response
Correct the request. If the request is valid, this could
be an internal error; contact the system programmer.
ISPA336 Invalid variable name - aaaaaaaa
is not a valid variable name for
both ISPF and APL2.
Explanation
The variable name is invalid.
User response
Contact the responsible programmer.
Programmer response
Correct the variable name.
ISPA337 APL2 function limit - An ISPF
service may retrieve up to 64k
bytes of APL2 workspace data.
Explanation
Storage for the program has been exhausted.
User response
Contact the responsible programmer.
Programmer response
Correct the program to prevent the storage error.
ISPA400 ISPAPTT dialog error - No
parameter input was passed.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  5

## Page 26

Explanation
The parameter did not include all of the necessary
inputs.
User response
Contact the responsible programmer.
Programmer response
Correct the parameters for the terminal type.
ISPA401 ISPAPTT dialog error - Unexpected
return code aaaaaaaa from
service bbbbbbbb.
Explanation
The APL program was unsuccessful.
System programmer response
This is a possible system or internal error. If you do not
have mixed levels of ISPF code, contact IBM support.
User response
Contact the responsible programmer.
Programmer response
The service shown failed with the return code shown.
Verify that the service is coded correctly. If it is
correct, contact the system programmer.
ISPC001 ISPCNT error: RC =aaaaaaaa -
Open failure, member does not
exist in data set.
Explanation
The SPF PARMS member specified does not exist in
the SPF PARMS data set
User response
Verify that the spelling of the SPF PARMS member
is correct. Ensure that the member exists in the SPF
PARMS data set allocated to ddname ISPPARM. Refer
to ISPF Planning and Customizing for more information
on the ISPCNT program.
ISPC002 ISPCNT error: RC =aaaaaaaa - The
input data set is a partitioned data
set. Specify a member name.
Explanation
An existing member of the SPF PARMS partitioned
data set must be specified.
User response
Verify that the name of the SPF PARMS data set is
correct and that an existing member is specified. Refer
to ISPF Planning and Customizing for more information
on the ISPCNT program.
ISPC003 ISPCNT error: RC =aaaaaaaa -
SPF parm data must have the
keyword TKV in the 3-5 or 4-6
position.
Explanation
The specified member is not a valid PARMS data set
if the keyword TKV is missing or not in the correct
location.
User response
Verify that the specified member exists in the SPF
PARMS data set. Browse the specified member to
see if the TKV keyword is missing or in the wrong
position. Refer to ISPF Planning and Customizing for
more information on the ISPCNT program.
ISPC004 ISPCNT error: RC =aaaaaaaa -
Open failure, allocate ISPPARM
ddname to a SPF parms member.
Explanation
The OPEN FAILURE is issued for not having allocated
the SPF PARMS data set to the ddname ISPPARM prior
to invoking ISPCNT.
User response
Allocate the fully-qualified SPF PARMS data set to
the ddname ISPPARM. Refer to ISPF Planning and
Customizing for more information on the ISPCNT
program.
ISPC010 ISPC010W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc=dddddddd" attribute
specification is not valid
when the "eeeeeeee" conversion
option has been specified.
"cccccccc=dddddddd" will not be
used.
ISPF messages starting with ISP
6  z/OS: z/OS ISPF Messages and Codes

## Page 27

Explanation
The eeeeeeee conversion option is in conflict with the
use of attribute cccccccc specified for value dddddddd.
The cccccccc attribute specification is ignored.
User response
Change the attribute value or change the conversion
utility option.
ISPC011 ISPC011W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Previous "cccccccc" tag attribute
was missing or not valid.
"dddddddd" tag will be ignored.
Explanation
The previous tag cccccccc has a missing or invalid
attribute. The current dddddddd tag cannot be
processed and will be ignored.
User response
Review the ISPDTLC log to determine the problem with
the cccccccc tag. Correct the problem and rerun the
conversion utility.
ISPC012 ISPC012W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc attribute has already
been defined for the dddddddd
tag. cccccccc = "eeeeeeee" will be
ignored and the first setting will be
used.
Explanation
The cccccccc attribute has been previously defined
on the current dddddddd tag. Duplicate attribute
specifications are ignored.
User response
Provide only 1 specification for attribute cccccccc.
ISPC013 ISPC013W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc=dddddddd" attribute is
not defined for the "eeeeeeee" tag
and will be ignored.
Explanation
The cccccccc attribute is not valid for the eeeeeeee tag.
User response
Remove the cccccccc attribute specification from the
DTL source file.
ISPC013A ISPC013AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" attribute is not defined
for the "dddddddd" tag and will be
ignored.
Explanation
The cccccccc attribute is not valid for the dddddddd
tag.
User response
Remove the cccccccc attribute specification from the
DTL source file.
ISPC014 ISPC014W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc attribute has already
been defined for the dddddddd
tag. This duplicate attribute
specification will be ignored.
Explanation
The cccccccc attribute has been previously defined
on the current dddddddd tag. Duplicate attribute
specifications are ignored.
User response
Provide only 1 specification for attribute cccccccc.
ISPC014A ISPC014AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc=dddddddd" attribute
has already been defined. This
duplicate attribute specification
will be ignored.
Explanation
The cccccccc=dddddddd attribute has been previously
defined on another tag. Duplicate attribute
specifications are ignored.
User response
Provide only 1 specification for attribute
cccccccc=dddddddd.
ISPC015 ISPC015W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The cccccccc attribute is not
valid when the dddddddd
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  7

## Page 28

attribute is also specified.
"cccccccc=eeeeeeee" will not be
used.
Explanation
There is a conflict in the specification of attributes
cccccccc and dddddddd. The dddddddd attribute will
be used. The cccccccc attribute is ignored.
User response
Remove the cccccccc attribute specification from the
DTL source file.
ISPC015A ISPC015AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc attribute is not valid
when the dddddddd attribute is
also specified. cccccccc will not be
used.
Explanation
There is a conflict in the specification of attributes
cccccccc and dddddddd. The dddddddd attribute will
be used. The cccccccc attribute is ignored.
User response
Remove the cccccccc attribute specification from the
DTL source file.
ISPC016 ISPC016E: Error. Line aaaaaaaa of
file "bbbbbbbb". The cccccccc tag
can only be coded within a text
string. The previous tag has not
been properly closed.
Explanation
The cccccccc tag can be used only within the text part
of the tag syntax following the '>' close tag delimiter.
User response
Make sure that the previous tag has been closed with
the ">" tag delimiter. Correct the DTL source and rerun
the conversion utility.
ISPC017 ISPC017W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc="dddddddd" attribute
value is not supported in the
conversion process. The default
value "eeeeeeee" will be used.
Explanation
The invalid value dddddddd has been specified for the
cccccccc attribute. The conversion utility will use the
default value eeeeeeee.
User response
Correct the DTL source and rerun the conversion
utility.
ISPC018 ISPC018E: Error. Line aaaaaaaa
of file "bbbbbbbb". A required tag
was not coded under the cccccccc
tag. One of these tags is required
to be coded under the cccccccc
tag: "dddddddd".
Explanation
The cccccccc tag requires the use of at least one of the
nested tags dddddddd.
User response
Correct the DTL source and rerun the conversion
utility.
ISPC019 ISPC019E: Error. Line aaaaaaaa of
file "bbbbbbbb". Required cccccccc
attribute was not specified or was
not valid on the dddddddd tag.
Explanation
The dddddddd tag requires the specification of
attribute cccccccc tags dddddddd.
User response
Correct the DTL source and rerun the conversion
utility.
ISPC020 ISPC020W: Warning. ISPDTLC
Release aaaaaaaa. for bbbbbbbb.
is being run on ISPF Release
cccccccc. for dddddddd.. Results
are not predictable.
Explanation
This message is self-explanatory.
User response
Review the file allocations to make sure that ISPDTLC
is being run from the SYSEXEC or SYSPROC data set
for the correct ISPF release.
ISPF messages starting with ISP
8  z/OS: z/OS ISPF Messages and Codes

## Page 29

ISPC020A ISPC020AI: ISPF Dialog Tag
Language Conversion Utility for
ISPF aaaaaaaa.. Current APAR
level: bbbbbbbb.. Current PTF
number: cccccccc..
Explanation
This is an informational message. This is second log
record for batch mode.
ISPC021 ISPC021I: aaaaaaaa ISPF
bbbbbbbb Dialog Tag Language
Conversion Utility
Explanation
This is an informational message. This is first log
record.
ISPC022 ISPC022I: Converting source file
"aaaaaaaa"...
Explanation
This is an informational message.
ISPC023 ISPC023W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate source file name
"cccccccc" ignored during DTLLST
processing.
Explanation
This is an informational message.
ISPC024 ISPC024I: Duplicate source file
name "aaaaaaaa" ignored during
member selection processing.
Explanation
This is an informational message.
ISPC025 ISPC025I: Profile processing
has been changed. The entries
for 'DTLMIN' will be ignored.
Conversion Utility messages are
now part of the standard ISPF
message library. Please remove all
'DTLMIN' entries from your profile.
Explanation
This is an informational message.
ISPC026 ISPC026I: Profile processing
has been changed. The entries
for 'DTLNLS' will be ignored.
The multicultural support literals
are now provided through an
ISPF facility. Please remove all
'DTLNLS' entries from your profile.
Explanation
This is an informational message.
ISPC027 ISPC027I: Source file name
"aaaaaaaa" ignored (not found)
during member selection
processing.
Explanation
This is an informational message.
ISPC028 ISPC028I: aaaaaaaa message(s)
have been suppressed.
Explanation
This is an informational message.
ISPC028A ISPC028AI: Total of aaaaaaaa
message(s) have been suppressed.
Explanation
This is an informational message.
ISPC029 aaaaaaaa warning(s) and
bbbbbbbb error(s) found.
Explanation
This is an informational message.
ISPC029A ISPC029AI: Total of aaaaaaaa
warning(s) and bbbbbbbb error(s)
found.
Explanation
This is an informational message.
ISPC030 ISPC030W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc end-tag does not end
any open tag and is ignored. The
current open-tag is dddddddd..
Explanation
The end tag cccccccc is not matched to any open tag.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  9

## Page 30

User response
Check tag syntax, especially from tag dddddddd
through the error line containing cccccccc. Correct the
DTL source and rerun the conversion utility.
ISPC031 ISPC031E: Error. Line aaaaaaaa
of file "bbbbbbbb". cccccccc has
missing end-tag and was ended by
dddddddd tag.
Explanation
The cccccccc tag requires an end tag. The dddddddd
tag has been found in the DTL source before the
required end-tag for cccccccc.
User response
Check tag nesting, especially from tag cccccccc
through the error line containing dddddddd. Correct
the DTL source and rerun the conversion utility.
ISPC032 ISPC032E: Error. Line aaaaaaaa
of file "bbbbbbbb". cccccccc tag
ended prematurely by the end of
the file.
Explanation
The DTL source file does not have an end-tag for
cccccccc.
User response
Make sure the DTL source file includes an end-tag for
cccccccc. Review the source file for an open quoted
string. Correct the DTL source file and rerun the
conversion utility.
ISPC034 ISPC034W: Warning. The text
from a COMMENT tag with a TYPE
value of ATTR, CCSID, or PANEL
was not added to the generated
panel. Move the COMMENT tag
to a higher location in the panel
definition.
Explanation
A COMMENT tag with a TYPE value of ATTR, CCSID,
or PANEL was encountered in the panel definition. The
tag is in a location that keeps the comment text from
being added to the generated panel.
User response
Move the COMMENT tag to a higher location in the
panel definition and recompile the panel.
ISPC035 ISPC035W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
No declaration found for ENTITY
"cccccccc". The variable cannot be
substituted. (ENTITY definitions
are case sensitive. Verify that the
entity-name definition matches
the entity reference.)
Explanation
Variable substitution cannot be completed for variable
cccccccc.
User response
If the entity is defined, make sure that the entity-name
reference is the same case as the entity declaration. If
the variable cccccccc is a runtime substitution variable,
you can use the predefined entity "&amp". You can
code the variable in the tag source as "&amp;variable"
to make "&variable" appear in the panel. Correct the
DTL source file and rerun the conversion utility.
ISPC036 ISPC036W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
During SOURCE tag processing,
no ENTITY declaration found for
"cccccccc". The variable cannot
be substituted. If "cccccccc"
is a TSO command or other
panel logic value, this message
may be removed by coding
"%amp;dddddddd". If "cccccccc"
is a variable to be substituted,
review the DTL source file.
(ENTITY definitions are case
sensitive. Verify that the entity-
name definition matches the entity
reference.)
Explanation
Variable substitution cannot be completed for variable
cccccccc.
User response
If the entity is defined, make sure that the entity-name
reference is the same case as the entity declaration.
If the variable cccccccc is a TSO command, the coding
can be specified as "%amp;S3." to avoid this message.
Correct the DTL source file and rerun the conversion
utility.
ISPC039 ISPC039E: Error. Line aaaaaaaa
of file "bbbbbbbb". cccccccc ended
ISPF messages starting with ISP
10  z/OS: z/OS ISPF Messages and Codes

## Page 31

prematurely by dddddddd start-
tag.
Explanation
A tag nesting error has caused the dddddddd tag to
end processing of the open cccccccc tag.
User response
Correct the DTL source file and rerun the conversion
utility.
ISPC040 ISPC040E: Error. Line aaaaaaaa
of file "bbbbbbbb". cccccccc tag
cannot be coded after previously
coded dddddddd tag.
Explanation
A tag was found that is not allowed after the previously
coded tag.
Programmer response
Verify the valid nesting conditions and update the DTL
source file.
ISPC041 ISPC041W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag cannot be coded after
previously coded dddddddd tag.
cccccccc and its nested tags will be
ignored.
Explanation
A tag was found that is not allowed after the previously
coded tag. The tag, and any tags nested within, are
ignored during the continuation of the conversion.
Programmer response
Verify the valid nesting conditions and update the DTL
source file.
ISPC042 ISPC042E: Error. Line aaaaaaaa of
file "bbbbbbbb". Document type is
unknown.
Explanation
The document-type declaration (DOCTYPE) was not
found in the file to be converted.
Programmer response
Update the DTL source file to include the !DOCTYPE
DM SYSTEM document-type declaration and reconvert
the file.
ISPC043 ISPC043E: Error. Line aaaaaaaa
of file "bbbbbbbb". Multiple
DOCTYPE records.
Explanation
More than one DOCTYPE record was found within a
single GML source file.
Programmer response
Update the DTL source file so it contains a single
DOCTYPE record.
ISPC044 ISPC044W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc and its nested tags will be
ignored in the conversion process.
Explanation
The cccccccc tag coded is not supported by ISPF. The
tag and all tags nested within will be ignored by the
conversion utility.
Programmer response
No response is required.
ISPC045 ISPC045E: Error. Line aaaaaaaa of
file "bbbbbbbb". Unable to format
data. Reason code was "cccccccc".
Explanation
Examine the line indicated to determine the formatting
issue.
ISPC046 ISPC046W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Multiple nested REGION tags with
the DIR=HORIZ attribute are not
allowed. All tags nested within
this REGION tag will be syntax
checked and then ignored during
panel formatting.
Explanation
This message is self explanatory.
ISPC047 ISPC047W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
use of the DEPTH attribute to
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  11

## Page 32

create a scrollable area on the
cccccccc tag is not valid because
DEPTH has been specified on
an enclosing tag. The DEPTH
attribute is ignored. ISPF does not
support nested scrollable areas.
Explanation
This message is self-explanatory.
User response
Remove the DEPTH attribute from the cccccccc tag.
ISPC050F Fixed length output panel libraries
must have a record length of 80,
132, or 160 bytes.
Explanation
The record length of the specified panel library is not
supported by ISPF.
User response
Use a panel library that has one of the specified record
lengths.
ISPC050V Variable length output panel
libraries should have a record
length of 84, 136, or 164 bytes.
The minimum record length is 84
and the maximum is 164.
Explanation
The record length of the specified panel library is not
supported by ISPF.
User response
Use a panel library that has one of the specified record
lengths.
ISPC051F Fixed length output message
libraries must have a minimum
record length of 80 bytes.
Explanation
The record length of the specified message library is
not supported by ISPF.
User response
Use a message library with a record length of 80.
ISPC051V Variable length output message
libraries must have a minimum
record length of 84 bytes.
Explanation
The record length of the specified message library is
not supported by ISPF.
User response
Use a message library with a record length of 84.
ISPC052F Fixed length output log files must
have a minimum record length of
80 bytes.
Explanation
The record length of the specified log file is not
supported by ISPF.
User response
Use a log file with a record length of 80.
ISPC052V Variable length output log files
must have a minimum record
length of 84 bytes.
Explanation
The record length of the specified log file is not
supported by ISPF.
User response
Use a log file with a record length of 84.
ISPC053F Fixed length output list files must
have a minimum record length of
80 bytes.
Explanation
The record length of the specified list file is not
supported by ISPF.
User response
Use a list file with a record length of 80.
ISPC053V Variable length output list files
must have a minimum record
length of 84 bytes.
Explanation
The record length of the specified list file is not
supported by ISPF.
ISPF messages starting with ISP
12  z/OS: z/OS ISPF Messages and Codes

## Page 33

User response
Use a list file with a record length of 84.
ISPC054F Fixed length output script files
must have a minimum record
length of 80 bytes.
Explanation
The record length of the specified script file is not
supported by ISPF.
User response
Use a script library with a record length of 80.
ISPC054V Variable length output script files
must have a minimum record
length of 84 bytes.
Explanation
The record length of the specified script file is not
supported by ISPF.
User response
Use a script library with a record length of 84.
ISPC055 Tables files must have a record
length of 80 bytes.
Explanation
The record length of the specified tables file is not
supported by ISPF.
User response
Use a tables library with a record length of 80.
ISPC059 Record format "aaaaaaaa" is not
valid for file "bbbbbbbb".
Explanation
The record format of the specified file is not supported
by ISPF.
User response
Table file format must be Fixed. DTL source, panels,
messages, and script file format may be either Fixed
or Variable. Log file and List file format may be either
Fixed or Variable, with or without print control.
ISPC060 Press Enter to start conversion.
Explanation
This message is self-explanatory.
User response
Press Enter to continue the conversion process.
ISPC061 Verify panel settings and press
Enter to start conversion.
Explanation
This message is self-explanatory.
User response
Press Enter to continue the conversion process.
ISPC062 Verify panel settings and press
Enter to submit conversion job.
Explanation
This message is self-explanatory.
User response
Press Enter to continue the conversion process.
ISPC067 Select code not valid - Use "S"
or "/" to select, "E" to Edit,
"V" to View, "B" to Browse, or
primary command DESELECT to
deselect all entries and erase all
line command notation entries.
Explanation
An invalid line command was entered on a member
list.
User response
Enter a valid line command for the member, or enter
RENEW to clear all line commands.
ISPC068A Select members to be converted.
(Enter "S *" to select all
members.)
Explanation
Select members from the ISPF member selection list
by placing an asterisk (*) in front of each member to
convert. If all the members are to be converted, enter
"S *" on the command line and press the Enter key.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  13

## Page 34

User response
Make appropriate selection of members to be
converted.
ISPC068B Select members to be converted
or END to start conversion
processing.
Explanation
Select members to be converted by placing an "S" in
front of the member or members to convert. When all
members to convert have been selected, enter END
and press the Enter key, or press the F3 key to start
the conversion process.
User response
Make additional selections, cancel conversion process,
or enter END to start the conversion process.
ISPC068C Only members from the first
16 data sets are included in
this list. Select members to be
converted. (Enter "S *" to select
all members.)
Explanation
Select members from the ISPF member selection list
by placing an asterisk (*) in front of each member to
convert. If all the members are to be converted, enter
"S *" on the command line and press the Enter key.
User response
Make appropriate selection of members to be
converted.
ISPC069A No members match the specified
pattern or the input data set(s) are
empty.
Explanation
The input DTL Source GML data set(s) were empty,
or no members in the data set(s) match the pattern
specified.
User response
Check the data sets or the member pattern or both.
ISPC069B No members have been selected.
Explanation
No members have been selected for conversion.
User response
To convert a DTL source file, enter the member name
and invocation options and press the Enter key.
ISPC069C DTL conversion in process for
"aaaaaaaa".
Explanation
This is an informational message. Conversion of the
member is currently in progress.
ISPC071 You may select either the option
for KANA or NOKANA, or you may
leave both options blank.
Explanation
KANA and NOKANA are optional selections. KANA and
NOKANA are mutually exclusive options, only one can
be chosen.
User response
Leave both the KANA and NOKANA options blank, or
select either the KANA or NOKANA option and press
ENTER to continue the conversion process.
ISPC072 Select option for "Display
converted panels" or "Display
converted panels in a window",
but not both.
Explanation
"Display converted panels" and "Display converted
panels in a window" are mutually exclusive options.
Only one can be chosen.
User response
Select only one of the "Display converted panels" or
"Display converted panels in a window" options and
restart the conversion process.
ISPC073 Select option for KANA or
NOKANA, but not both.
Explanation
KANA and NOKANA are mutually exclusive options,
only one can be chosen.
User response
Select only one of the KANA or NOKANA options and
restart the conversion process.
ISPF messages starting with ISP
14  z/OS: z/OS ISPF Messages and Codes

## Page 35

ISPC074 Select option for Generate List
file or Generate List file with
substitution, but not both.
Explanation
Generate List file and Generate List file with
substitution are mutually exclusive options, only one
can be chosen.
User response
Select option for Generate List file or Generate List file
with substitution, and restart the conversion process.
ISPC075 The aaaaaaaa language requires
that the DBCS option be selected.
Either select the DBCS option or
specify a non-DBCS language.
Explanation
The DBCS option is not selected but the current
language is a DBCS language. The language and DBCS
option selection are in conflict.
User response
Either select the DBCS option or specify a non-DBCS
language.
ISPC076 Enter up to 4 positions for
application id. First position must
be alpha.
Explanation
The value entered for the Keylist Application ID
was invalid. Valid values are 0-4 positions; the first
position, if used, must be alphabetic.
User response
Enter a valid Keylist Application ID.
ISPC077 The KANA option is valid only with
the JAPANESE language selection.
Explanation
KANA is only valid when the JAPANESE language is
used for the conversion.
User response
Select the JAPANESE language, or deselect the KANA
option, and restart the conversion process.
ISPC078 The DBCS option is valid only for
JAPANESE, CHINESES, CHINESET
or KOREAN language selection.
Explanation
DBCS is only valid when the selected language
used for the conversion is JAPANESE, CHINESES,
CHINESET, or KOREAN.
User response
Select one of the valid languages if the DBCS option is
needed, or deselect the DBCS option and restart the
conversion process.
ISPC079 Choose a language from the ISPF
supported languages. Use the Help
option for a list of languages.
Explanation
The language specified is not one of the valid ISPF
supported languages. Valid languages are:
English
German
Swiss German
Danish
Spanish
Portuguese
French
Italian
Japanese
Korean
Chinese (Simplified)
Chinese (Traditional)
User response
Specify a valid ISPF language and restart the
conversion process.
ISPC080 Enter a valid DTL source file
member name.
Explanation
This message is self explanatory.
ISPC081 Enter a valid DTL source library
name.
Explanation
The DTL source library name specified was invalid.
For example, the library name may not conform to
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  15

## Page 36

valid naming conventions, or unmatched quotes were
detected.
User response
Enter a valid DTL source library name and restart the
conversion process.
ISPC082 Enter a valid output panel library
name.
Explanation
The output panel library name specified was invalid.
For example, a library name may not conform to
valid naming conventions, or unmatched quotes were
detected.
User response
Enter a valid output panel library name and restart the
conversion process.
ISPC083 Enter a valid output message
library name.
Explanation
The output message library name specified was
invalid. For example, a library name may not conform
to valid naming conventions, or unmatched quotes
were detected.
User response
Enter a valid output message library name and restart
the conversion process.
ISPC084 Enter a valid output log file name.
Explanation
The output log file name specified was invalid.
For example, a log file name may not conform to
valid naming conventions, or unmatched quotes were
detected.
User response
Enter a valid output log file name and restart the
conversion process.
ISPC086 Enter a valid DTL source filename.
Explanation
The DTL source member list can not be displayed.
This may occur when LMINIT or LMOPEN has not
completed successfully.
User response
Exit from the conversion utility. Enter ISPDTLC and
restart the conversion process.
ISPC091 Enter a valid log file member
name.
Explanation
The log file member name entered contains invalid
characters, or does not conform to valid TSO member
naming conventions.
User response
Enter a valid member name for the log file.
ISPC092 Enter a valid output list file name.
Explanation
The output list file name specified was invalid.
For example, a list file name may not conform to
valid naming conventions, or unmatched quotes were
detected.
User response
Enter a valid output list file name and restart the
conversion process.
ISPC093 Enter a valid list file member
name.
Explanation
The list file member name entered contains invalid
characters, or does not conform to valid TSO member
naming conventions.
User response
Enter a valid member name for the list file.
ISPC094 Enter a valid output SCRIPT
library name.
Explanation
The SCRIPT library name specified was invalid. For
example, a library name may not conform to valid
naming conventions, or unmatched quotes were
detected.
User response
Enter a valid output SCRIPT library name and restart
the conversion process.
ISPF messages starting with ISP
16  z/OS: z/OS ISPF Messages and Codes

## Page 37

ISPC095 Enter a valid TABLES library name.
Explanation
The TABLES library name specified was invalid. For
example, a library name may not conform to valid
naming conventions, or unmatched quotes were
detected.
User response
Enter a valid TABLES library name and restart the
conversion process.
ISPC100 ISPC100E: Error. Line aaaaaaaa
of file "bbbbbbbb". The DBCS
control byte "SO" has been
detected in your source file, but
the DBCS option has not been
specified. The conversion results
are unpredictable. The conversion
utility has automatically enabled
the DBCS option to allow
formatting of your source file to
continue. Reconvert your source
file specifying the DBCS option.
Explanation
The conversion utility encountered a DBCS control
shift-out byte when the DBCS option was not specified
for the conversion. The DBCS option has been enabled
at this point, but text formatting may be unpredictable.
User response
Restart the conversion process with the DBCS option
specified.
ISPC101 ISPC101E: Error. Line aaaaaaaa
of file "bbbbbbbb". DBCS SO byte
encountered before required DBCS
SI byte.
Explanation
A DBCS shift-out byte was encountered in the GML
source file prior to a DBCS shift-in byte.
Programmer response
Correct the GML source file to contain matched shift-in
and shift-out bytes and restart the conversion process.
ISPC102 ISPC102W: Warning. Line
aaaaaaaa of file "bbbbbbbb". At
least 4 byte field widths are
required to format DBCS strings.
There is not enough room to
format the "cccccccc" on the
dddddddd tag.
Explanation
The field width for the DBCS field must be 4 bytes
or greater to accommodate the shift-in/shift-out bytes
along with the double-byte character.
Programmer response
Update the GML source file to allow at least 4 bytes for
the field width and restart the conversion process.
ISPC103 ISPC103E: Error. Line aaaaaaaa of
file "bbbbbbbb". Cannot end string
without DBCS SI byte.
Explanation
A string containing a DBCS shift-out byte ended before
the matching DBCS shift-in byte was found.
User response
Check the GML source file and match DBCS shift-
in and shift-out bytes, then restart the conversion
process.
ISPC104 ISPC104E: Error. Line aaaaaaaa of
file "bbbbbbbb". Cannot end DBCS
string on odd byte boundary.
Explanation
A DBCS string was encountered with an odd number of
bytes, which is not acceptable for a pure DBCS string.
User response
Update the GML source file for the correct DBCS data
and restart the conversion process.
ISPC105 ISPC105E: Error. Line aaaaaaaa
of file "bbbbbbbb". DBCS SI byte
encountered without matching
beginning DBCS SO byte.
Explanation
A DBCS shift-in byte was encountered in the GML
source file without a matching DBCS shift-out byte.
Programmer response
Correct the GML source file to contain matched shift-in
and shift-out bytes and restart the conversion process.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  17

## Page 38

ISPC106 ISPC106W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
room to format data.
Explanation
No room was left on the panel to format the given
data.
User response
Check the GML source file for spacing requirements for
depth and width, then restart the conversion process.
ISPC107 ISPC107W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
VARCLASS type "cccccccc" has no
meaning for dddddddd entry fields
and is ignored by the conversion
utility.
Explanation
A VARCLASS tag was specified with a TYPE that does
not have meaning when specified for the entry field.
User response
Update the GML source file and restart the conversion
process.
ISPC108 ISPC108E: Error. The specified or
default language "aaaaaaaa" has
been changed to "bbbbbbbb" to
allow formatting of your source file
to continue. Reconvert your source
file specifying a DBCS supported
language.
Explanation
The language specified for the conversion does not
support DBCS. The formatting of the source file
requires DBCS, and the language has been changed
to a DBCS-supported language.
User response
Check that your GML source file is correct and restart
the conversion process specifying a DBCS-supported
language.
ISPC109 ISPC109W: Warning. The use of
the "aaaaaaaa" language requires
that the DBCS option be specified
for the conversion. The conversion
utility has automatically enabled
the DBCS option.
Explanation
This is an informational message. The language
specified for the conversion requires the DBCS option.
The DBCS option has been automatically enabled for
the conversion.
ISPC110 ISPC110W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
String constant exceeds 253
characters and will be truncated.
Explanation
This message is self-explanatory.
ISPC112 ISPC112W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
key value "cccccccc" is not valid.
Valid values are F1-F24 only.
Explanation
The KEY= value on the KEYI tag was coded incorrectly.
Valid values are F1-F24.
Programmer response
Update the GML to reflect a valid KEY value on the
KEYI tag, and restart the conversion process.
ISPC114 ISPC114W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Odd length DBCS substring
encountered. Beginning of string
padded with a single-byte blank.
Explanation
DBCS substring must contain an even length. A single-
byte blank has been added at the beginning of the
string to correct the length.
Programmer response
Verify the DBCS substring.
ISPC115 ISPC115W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
keys were defined for "cccccccc".
Explanation
No KEYI (Key Item) tag was found for processing
within the KEYL (Key List) tag.
Programmer response
Update the KEYL tag to include at least one KEYI.
ISPF messages starting with ISP
18  z/OS: z/OS ISPF Messages and Codes

## Page 39

ISPC120 ISPC120E: Error. ISPF Keylist was
not created. Possible reasons may
be that the conversion tool was
not run as a dialog on ISPF or ISPF
was not active.
Explanation
An invalid return code was received from the SELECT
service.
User response
Contact the responsible programmer.
ISPC121 ISPC121W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Key Description exceeds allowable
length of cccccccc characters.
Explanation
The maximum key description allowed by the Dialog
Tag Language is 64. The first 8 bytes of description are
used by ISPF.
User response
Reduce the length of the key description to 64.
ISPC122 ISPC122W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The internal-command-name
"cccccccc" is not valid. When
dddddddd, eeeeeeee must be
"ffffffff".
Explanation
This message is self explanatory.
ISPC123 ISPC123W: Warning. Line
aaaaaaaa of file
"bbbbbbbb". Internal-command-
name "cccccccc" is not valid for
the "dddddddd" function key.
Explanation
When the HELP command is specified in a keylist, it
must be assigned to the F1 or F13 key.
When the EXIT command is specified in a keylist, it
must be assigned to the F3 or F15 key.
When the CANCEL command is specified in a keylist, it
must be assigned to the F12 or F24 key.
User response
Use the appropriate key for the HELP, EXIT, or CANCEL
command.
ISPC126 ISPC126W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate "cccccccc" key was
encountered and will be ignored.
Explanation
Only 1 definition of the cccccccc key is allowed in each
keylist.
User response
Remove the duplicate definition for the cccccccc key.
ISPC127 ISPC127W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The key value "cccccccc" is not
supported by the conversion utility
as a valid key assignment. Valid
values are F1-F24 only.
Explanation
This message is self-explanatory.
User response
Use a key value in the F1-F24 range.
ISPC128 ISPC128E: Error. Line aaaaaaaa
of file "bbbbbbbb". Key
list Application ID required.
"cccccccc" must be specified as an
invocation option in order to write
"dddddddd".
Explanation
No application ID is available to identify the keylist.
User response
Specify the application ID on the invocation panel, or
use the KEYLAPPL=xxxx keyword from the invocation
syntax to specify the application ID for this keylist.
ISPC129A ISPC129AE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code of
"12" (Attributes are not valid) from
the SELECT service attempting to
create an ISPF Keylist.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  19

## Page 40

Explanation
The keylist update program has detected an invalid
key attribute.
User response
Provide a valid value for the FKA attribute of the KEYI
tag.
ISPC129B ISPC129BE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "16" (Key defined is not
valid) from the SELECT service
attempting to create an ISPF
Keylist.
Explanation
The keylist update program has detected an invalid
key name.
User response
Provide a valid value for the KEY attribute of the KEYI
tag.
ISPC129C ISPC129CE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "cccccccc" (Severe Error) from
the SELECT service attempting to
create an ISPF Keylist.
Explanation
The keylist update program has detected a severe
error while attempting to update the keylist.
System programmer response
If you are not running mixed levels of ISPF code, you
may need to contact IBM support.
User response
If the error continues, contact your system
programmer.
ISPC129D ISPC129DE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "24" (Syntax Error) from the
SELECT service attempting to
create an ISPF Keylist.
Explanation
This message is self explanatory.
ISPC129E ISPC129EE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code of
"28" (Keylist entry not found) from
the SELECT service attempting to
create an ISPF Keylist.
Explanation
This message is self explanatory.
ISPC129F ISPC129FE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "32" (Keylist table open) from
the SELECT service attempting to
create an ISPF Keylist. (The keylist
update program received a return
code of "12" from the TBOPEN
service.)
Explanation
The keylist update program received a return
code of 12 from the TBOPEN service. The keylist
update cannot be performed on the currently active
application ID.
User response
ISPDTLC must be run with a different application ID.
ISPC129G ISPC129GE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "36" (ISPTABL file not
allocated) from the SELECT
service attempting to create an
ISPF Keylist. (The keylist update
program received a return code of
"16" from the TBCLOSE service.)
Explanation
The keylist update program received a return code of
16 from the TBCLOSE service. The output table library
is not allocated.
User response
Verify that the ISPTABL file has been allocated before
running ISPDTLC.
ISPC130 ISPC130W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Unknown numeric value found
processing DTL source string
"cccccccc". Check for sequence
numbers in the DTL source file.
Explanation
During source file processing, an 8 character numeric
value string was found. This value might be a file
ISPF messages starting with ISP
20  z/OS: z/OS ISPF Messages and Codes

## Page 41

sequence number. DTL source file records can not
contain sequence numbers.
User response
Check the GML source file. Remove any sequence
numbers and restart the conversion process.
ISPC131 ISPC131W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Text
string "cccccccc" found in source
file where tag is expected. Text
string is ignored.
Explanation
While scanning the source file for the next tag, an
extraneous text string was found, possibly an unclosed
tag or a missing end-tag.
User response
Check the GML source file and restart the conversion
process.
ISPC132 ISPC132W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Undefined "cccccccc" start-tag.
"cccccccc" is ignored.
Explanation
The start-tag specified is not valid.
Programmer response
Update the GML to contain a valid start-tag, and restart
the conversion process.
ISPC133 ISPC133E: Error. Line aaaaaaaa
of file "bbbbbbbb". The first
character in keyword "cccccccc"
is not valid. "cccccccc" must start
with characters "A-Z" and the ">"
must be used as the closing tag
delimiter. One possible cause is
that a previous tag was not closed
with the ">" delimiter.
Explanation
A keyword was detected with invalid syntax. The
keyword must begin with an alphabetic character and
must contain a closing tag delimiter of ">". This error
could occur if the previous tag did not contain the
closing tag delimiter.
Programmer response
Update the GML to contain a valid keyword and restart
the conversion process.
ISPC134 ISPC134E: Error. Line aaaaaaaa
of file "bbbbbbbb". Pending quote
was not closed and caused all
records in the file to be read in an
attempt to find the closing quote.
Explanation
A single quote was encountered and the closing quote
was not found in the rest of the file.
Programmer response
Update the GML to contain matching opening and
closing quotes and restart the conversion process.
ISPC135 ISPC135W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Unmatched quotes in entity value:
cccccccc.. Panel formatting may be
affected.
Explanation
A single('') or double(") quote was encountered within
cccccccc that is the same kind of quote character
found immediately before or after the symbolic
variable.
Programmer response
Update the GML to contain matching opening and
closing quotes and restart the conversion process.
ISPC136 ISPC136E: Error. Line aaaaaaaa
of file "bbbbbbbb". Incorrect tag
syntax with multiple close tag
delimiters found on the cccccccc
tag.
Explanation
Multiple close tag delimiters found when processing
the cccccccc tag.
User response
Check the GML source file and restart the conversion
process.
ISPC137 ISPC137E: Error. Line aaaaaaaa
of file "bbbbbbbb". Incorrect tag
syntax. The cccccccc tag must
follow the preceding tag with no
intervening blanks or text. Only
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  21

## Page 42

one cccccccc tag is allowed within
a LI, LP or P tag.
Explanation
This message is self-explanatory.
User response
If any blanks precede the CAUTION or WARNING tag,
remove the blanks and reconvert the DTL source file.
If multiple CAUTION or WARNING tags are present,
remove the multiple occurrence of the tag.
ISPC140 ISPC140W: The DBCS option is
required when KANA has been
specified. The conversion utility
has automatically enabled the
DBCS option.
Explanation
The KANA keyword has been specified without
specification of the DBCS option. DBCS is required
when processing with KANA. The conversion utility has
automatically enabled the DBCS option.
User response
If KANA is not to be used, restart the conversion
process without the KANA and DBCS options.
ISPC141 ISPC141W: The JAPANESE
language is required when KANA
has been specified. The conversion
utility has automatically set the
language to "JAPANESE".
Explanation
The KANA keyword has been specified without
specification of the JAPANESE language. The
conversion utility requires the use of the JAPANESE
language when processing KANA. The conversion
utility has automatically set the language specification
to JAPANESE.
User response
If KANA is not to be used, restart the conversion
process without the KANA and JAPANESE options.
ISPC142 ISPC142W: The DBCS option has
been specified, but either no
language has been specified or the
language selected is not a DBCS
language. The conversion utility
has automatically set the language
to "JAPANESE".
Explanation
The DBCS keyword has been specified. However,
either no language was selected, or the language
specified does not support DBCS. The conversion
utility has automatically set the language specification
to JAPANESE.
User response
If DBCS is not to be used, restart the conversion
process without the DBCS and JAPANESE options.
ISPC149 ISPC149E: Error. Line aaaaaaaa
of file "bbbbbbbb". Line contains
DBCS data which is not valid. Hex
value of data string ="cccccccc".
Explanation
Line contains DBCS data which is not valid. Check for
proper use of shift out and shift in control bytes, and
make sure the DBCS data is an even number of bytes.
User response
Correct the DTL source file and rerun the conversion
process.
ISPC150 ISPC150W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
When "cccccccc", "dddddddd" on
the eeeeeeee tag does not conform
to the CUA Architecture definition
and will therefore not be CUA
compliant.
Explanation
This is an informational message.
ISPC151 ISPC151W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
use of cccccccc="dddddddd" on
the eeeeeeee tag does not conform
to the CUA Architecture definition
and will therefore not be CUA
compliant.
Explanation
This is an informational message.
ISPC160 Enter "F" for fixed length panels or
"V" for variable length panels.
Explanation
This message is self explanatory.
ISPF messages starting with ISP
22  z/OS: z/OS ISPF Messages and Codes

## Page 43

User response
Enter either "F" or "V" and restart the conversion
process.
ISPC161 Enter a valid panel record length
(80, 132, or 160).
Explanation
This message is self explanatory.
User response
Enter one of the lengths listed and restart the
conversion process.
ISPC162 Enter a valid file name for an
output panel MACLIB.
Explanation
This message is self explanatory.
User response
Enter a valid file name and restart the conversion
process.
ISPC163 Enter a valid file name for an
output message MACLIB.
Explanation
This message is self explanatory.
User response
Enter a valid file name and restart the conversion
process.
ISPC164 Enter a valid file name for an
output log file.
Explanation
This message is self explanatory.
User response
Enter a valid file name and restart the conversion
process.
ISPC165 Enter a valid file name for an
output list file.
Explanation
This message is self explanatory.
ISPC166 Enter a valid file name for an
output SCRIPT MACLIB.
Explanation
This message is self explanatory.
ISPC201 ISPC201E: Error. Line aaaaaaaa
of file "bbbbbbbb". No help panel
title specified after Help Tag.
Explanation
A Help panel tag requires a panel title.
User response
Update the DTL source file to add a panel title and
rerun the conversion utility.
ISPC202 ISPC202W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Panel title has been truncated.
Explanation
The panel title is too long for the specified panel width.
User response
Reduce the length of the title text or increase the
specified panel width, as appropriate.
ISPC203 ISPC203W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Panel title cannot be centered
within the available panel width.
Explanation
This is an informational message.
ISPC204 ISPC204W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" maximum
length is eeeeeeee character(s).
cccccccc will be truncated to
"ffffffff". Multiple output panels
are being created to simulate
tutorial scrolling. The truncated
name will be the name of the first
panel created. Subsequent panels
will have an eighth character
suffix of 0-1 and A-Z for a
maximum of 37 total panels.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  23

## Page 44

Explanation
Help panel text has exceeded the number of lines
available for one help panel. Because the panel does
not have a scrollable area defined, the default is to
create multiple help panels.
User response
Update the DTL source. If the panel depth is less than
22 lines or the width is less than 76 bytes, and the
help text will fit within these limits, increase the depth
to a maximum of 22 lines, or increase the width to a
maximum of 76 bytes.
If the text exceeds these limits, you can add an
<AREA depth=n> tag to the DTL source (where n is the
number of display lines for the scrollable area). This
will cause all of the help text to be formatted in one
panel.
If multiple panels are desired, change the name
specified on the <HELP> tag to a seven character
name to eliminate this message.
ISPC205 ISPC205E: Error. Line aaaaaaaa
of file "bbbbbbbb". Unable to
create any more cccccccc panels
dynamically to simulate scrolling.
The maximum of 37 panels has
been exceeded.
Explanation
Help panel text has exceeded the number of panels
that can be created by the conversion utility.
User response
Increase the width or depth (or both) of the help
panels to allow more text on each panel, convert the
DTL source to generate a scrollable panel, or divide the
help text into multiple HELP tags.
ISPC230 Enter the seven character FMID for
the SMP/E USERMOD.
Explanation
This is an informational message.
ISPC231 Enter DDDEF name - Enter the
DDDEF name to be used as the
SYSLIB for the keyword source in
the SMP/E USERMOD.
Explanation
This is an informational message.
ISPC232 Enter DDDEF name - Enter the
DDDEF name to be used as the
SYSLIB for the load modules in the
SMP/E USERMOD.
Explanation
This is an informational message.
ISPC233 Enter DDDEF name - Enter the
DDDEF name to be used as the
DISTLIB for the keyword source in
the SMP/E USERMOD.
Explanation
This is an informational message.
ISPC234 Error Building USERMOD - An
error occurred building the SMP/E
USERMOD. Correct errors reported
and retry the function.
Explanation
This is an informational message.
ISPC240 ISPC240W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Lines that are wider than the
current INFO width have been
truncated.
Explanation
This is an informational message.
ISPC241 ISPC241W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Lines that are longer than the
current PANEL record length have
been truncated.
Explanation
This is an informational message.
ISPC242 ISPC242W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Lines that are wider than the
current PANEL width have been
truncated.
Explanation
This is an informational message.
ISPC243 ISPC243W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Extraneous text "cccccccc" found
ISPF messages starting with ISP
24  z/OS: z/OS ISPF Messages and Codes

## Page 45

during the format process of
"dddddddd" tag. Text is ignored.
Explanation
Text was present following the close tag delimiter on
the dddddddd tag. This tag does not allow a tag text
field.
User response
Remove the invalid text string.
ISPC244 ISPC244W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
text found during the format
process for the "cccccccc" tag.
Panel formatting may be affected.
Explanation
No text was provided for the cccccccc tag. This tag
should have a text field for proper panel formatting.
User response
Place text between the cccccccc tag and the cccccccc
end-tag.
ISPC245 ISPC245W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Panel width too small to include
LSTCOL DATAVAR=cccccccc..
cccccccc will not be written to the
panel.
Explanation
There is insufficient room for the cccccccc table
column.
User response
Adjust table column width specifications.
ISPC246 ISPC246W: Warning. Panel width
too small to include "aaaaaaaa".
Explanation
Formatting of panel fields extends beyond the
available panel width. Data on the right side of the
panel is not formatted.
User response
Adjust horizontal field formatting to permit formatting
within the available panel width.
ISPC247 ISPC247W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Panel width too small to include
LSTCOL text "cccccccc".
Explanation
This is an informational message.
ISPC248 ISPC248W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Unable to format a divider line
for the LSTFLD tag because all 8
available model lines have been
used by nested LSTCOL tags.
Explanation
This is an informational message.
ISPC249 ISPC249W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
LSTCOL tag "column-heading"
extends beyond right panel
boundary and will be truncated to
"cccccccc".
Explanation
This is an informational message.
ISPC250 Keyword file saved - The updated
keyword file was successfully
saved to aaaaaaaa..
Explanation
This is an informational message.
ISPC251 Keyword file verified - Keyword
file aaaaaaaa was verified and is
correct.
Explanation
This is an informational message.
ISPC252 Keyword file errors - Errors
were found during verification of
keyword file aaaaaaaa and were
previously displayed.
Explanation
The keyword verification function found errors in the
keyword file being processed. A listing showing the
errors was previously displayed to the user.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  25

## Page 46

User response
Correct the errors in the keyword file and reprocess it.
ISPC253 Load module built - The
configuration table load module
and VSAM load module were
successfully saved to aaaaaaaa..
Explanation
The specified keyword file contained keywords for
both the configuration table and the VSAM Edit/
View/Browse support. Both load modules have been
successfully built.
ISPC254 Load module build error - An error
was encountered creating load
module aaaaaaaa from keyword
file bbbbbbbb..
Explanation
An error was encountered converting the specified file
to a load module. An assembler or link edit listing
should have been displayed to indicate the error.
User response
If the error can be corrected by a change to the
keyword file make the change and reprocess. If the
problem cannot be corrected, contact your system
programmer.
ISPC255 Conversion successful -
Conversion of assembler file
aaaaaaaa to keyword file
bbbbbbbb was successful.
Explanation
This is an informational message.
ISPC256 Conversion error - Conversion of
assembler source file aaaaaaaa to
keyword file bbbbbbbb failed.
Explanation
An error was encountered converting the specified
assembler source file to the keyword file.
User response
Verify the input assembler file is an ISPF configuration
table assembler member (member ISPCNFIG from
SAMPLIB).
ISPC257 Member name not allowed -
Member name is not allowed on
the load and object data sets.
Specify the member name in the
Configuration Member and VSAM
Member fields.
Explanation
A data set name string that included a member name
was specified for either the "Output Configuration
Table Load Module Data Set" or "Object data set" field
of the Build Configuration Table Load Module panel.
Member names for these data set should be specified
in the "Configuration Member" and "VSAM member"
fields if you wish to use names other than the defaults
of ISPCFIGU and ISPCFIGV.
ISPC258 Assemble failed - The assemble
of the configuration table source
failed, see the assembly listing
below for details.
Explanation
The assembler of the generated assembler used to
create the ISPF Configuration Table load module
failed. The user is placed in View on the assembly
listing.
User response
Contact IBM service.
ISPC259 Link Edit failed - The link edit
of the configuration table failed,
see the link edit listing below for
details.
Explanation
The link edit to create the ISPF Configuration Table
load module failed. The user is placed in View on the
link edit listing.
User response
Contact IBM service.
ISPC260 Rename needed - The
configuration load modules were
successfully saved to aaaaaaaa.,
but using names bbbbbbbb. and
cccccccc.. These members must be
named ISPCFIGU and ISPCFIGV
to be used as ISPF configuration
modules.
ISPF messages starting with ISP
26  z/OS: z/OS ISPF Messages and Codes

## Page 47

Explanation
The configuration load module and VSAM load module
were successfully built using the names specified
in the "Configuration member" and "VSAM member"
fields. These load module members must be renamed
to ISPCFIGU and ISPCFIGV and placed in a load library
accessible to ISPF for them to be used by ISPF.
ISPC261 Allocation error - An error was
encountered allocating Keyword
data set aaaaaaaa..
Explanation
The TSO ALLOCATE command file failed when
attempting to allocate the specified keyword file to a
DDNAME.
User response
Verify the data set is not in use by another user.
ISPC262 EXECIO error - An error was
encountered reading Keyword
data set aaaaaaaa..
Explanation
An EXECIO error was received attempting to read the
specified keyword file.
User response
Check if the data set is the correct data set and
readable by other functions.
ISPC263 Allocation error - An error
was encountered allocating the
temporary assembler data set.
Explanation
ISPF was unable to create the temporary SYSIN data
set that will be used in building the configuration table
load module.
User response
Contact your system programmer.
ISPC264 Rename needed - The
configuration load module was
successfully saved to aaaaaaaa.,
but using name bbbbbbbb..
This member must be named
ISPCFIGU to be used as an ISPF
configuration module.
Explanation
The configuration load module was successfully
built using the name specified in the "Configuration
member" field. This load module member must be
renamed to ISPCFIGU and placed in a load library
accessible to ISPF for it to be used by ISPF.
ISPC265 Load module built - The
configuration table load module
was successfully saved to
aaaaaaaa..
Explanation
This is an informational message.
ISPC266 Allocation error - An error
was encountered allocating the
temporary object data set.
Explanation
ISPF was unable to create the temporary SYSLIN data
set that will be used in building the configuration table
load module.
User response
Contact your system programmer.
ISPC267 Entered required field - Enter the
selection for the type of build you
want: 1 to build the Configuration
Table into a load module. 2 to
build the Configuration Table into
an SMP/E USERMOD.
Explanation
The "Select build type" field must not be blank.
User response
Enter the selection for the type of build you want: 1 to
build the Configuration Table into a load module. 2 to
build the Configuration Table into an SMP/E USERMOD.
ISPC268 Incorrect attributes - The keyword
data set must be a partitioned
data set with a record length of at
least 251 for fixed data sets and
255 for variable data sets.
Explanation
This is an informational message.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  27

## Page 48

ISPC269 Allocation error - An error was
encountered allocating assembler
input data set aaaaaaaa..
Explanation
The TSO ALLOCATE command file failed when
attempting to allocate the specified assembler data
set file to a DDNAME.
User response
Verify the data set is not in use by another user.
ISPC270 EXECIO error - An error was
encountered reading assembler
input data set aaaaaaaa..
Explanation
An EXECIO error was received attempting to read the
specified assembler input file.
User response
Check if the data set is the correct data set and
readable by other functions.
ISPC271 Keyword file saved - Updated
keyword file has been saved to
aaaaaaaa and verified as correct.
Explanation
This is an informational message.
ISPC272 Verification failed - Updated
keyword file has been saved to
aaaaaaaa but failed verification.
Verification messages were
previously displayed.
Explanation
This is an informational message.
ISPC273 Keyword file saved - Updated
keyword file has been saved to
aaaaaaaa but not verified.
Explanation
The edited keyword file has been successfully
updated, but the verification step was bypassed as
requested.
ISPC274 Conversion error - Conversion of
configuration table to keyword file
aaaaaaaa failed.
Explanation
An error was encountered converting the specified
configuration table load module to the keyword file.
ISPC275 Function required - The function
letter of B(rowse) E(dit) or V(iew)
is required.
Explanation
When specifying a data set or pattern to be restricted
from the ISPF VSAM Edit/View/Browse, you must
specify to which function the data set name or pattern
applies.
ISPC276 Defaults loaded - Keyword file
aaaaaaaa was not found, default
values have been set for all
configuration options.
Explanation
This is an informational message.
ISPC277 Keyword file loaded - Defaults
have been initialized from
keyword file aaaaaaaa..
Explanation
This is an informational message.
ISPC278 Block size value of aaaaaaaa is
not a multiple of bbbbbbbb..
Explanation
This is an informational message.
ISPC279 Block size value of aaaaaaaa is
not a multiple of record length
value bbbbbbbb..
Explanation
This is an informational message.
ISPC280 Assemble failed - The assemble
of the configuration table VSAM
module source failed, see the
assembly listing below for details.
Explanation
The assembler of the generated assembler used to
create the ISPF Configuration Table VSAM load module
failed. The user is placed in View on the assembly
listing.
ISPF messages starting with ISP
28  z/OS: z/OS ISPF Messages and Codes

## Page 49

User response
Contact IBM service.
ISPC281 Link Edit failed - The link edit
of the configuration table VSAM
module failed, see the link edit
listing below for details.
Explanation
The link edit to create the ISPF Configuration Table
VSAM load module failed. The user is placed in View
on the link edit listing.
User response
Contact IBM service.
ISPC282 Invalid data set name - The data
set name specified is invalid. Enter
the data set name and member for
the keyword file in the separate
fields provided.
Explanation
The data set name entered is syntactically incorrect.
User response
Correct the data set name.
ISPC283 File Tailoring Failed - The
aaaaaaaa service returned a code
of bbbbbbbb processing skeleton
ISPCSKEL.
Explanation
ISPF skeleton ISPCSKEL is used to create the
assembler source that is built into the configuration
table load module. The file tailoring process failed.
User response
Use the service name and return code in the message
to diagnose the error and take corrective action.
ISPC284 Invalid data set name - Remove
the quotes from the data set name
or pattern entered.
Explanation
The specified name must be an unquoted data set
name or pattern.
User response
Correct the data set name or pattern entered.
ISPC285 Invalid combination - At least one
data set must be specified for each
restriction field selected.
Explanation
Both a data set name or pattern and a VSAM restriction
field must be selected. It is not valid to specify only
one or the other.
ISPC286 Input file empty - The input
data set or member specified for
conversion is empty.
Explanation
This is an informational message.
ISPC287 Invalid input file - The input
file specified is not an ISPF
Configuration Table assembler
source module, no ISRCONFG
CSECT found.
Explanation
The source file specified as input to the Conversion
option does not appear to be an ISPF Configuration
Table assembler module. ISPF looks for the string
'ISRCONFG CSECT' on the first line not containing an
asterisk in column 1 and the string was not found in
the file specified.
User response
Specify the correct input file.
ISPC288 Invalid member name - The
same member name cannot
be specified for both the
Configuration member and the
VSAM member, ISPCFIGV cannot
be specified for the Configuration
member, and ISPCFIGU cannot be
specified for the VSAM member.
Explanation
One of these errors has occurred:
• The same member name has been specified for both
the Configuration member and VSAM member fields.
• ISPCFIGV was specified for the Configuration
member
• ISPCFIGU was specified for the VSAM member
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  29

## Page 50

This could result in ISPF overlaying the Configuration
member with the VSAM member.
User response
Specify a different name for one of the members.
ISPC289 Write error - An error was
encountered writing the keyword
file to the output data set. Data set
may be full, compress the library
and retry the function.
Explanation
A nonzero return code was returned by EXECIO when
writing the keyword file to the specified output data
set. The most common reason for this is the data set
being out of space.
User response
Compress the data set and retry the function. If the
write still fails, contact you systems programmer.
ISPC290 GIMDTS error - An error
was encountered with GIMDTS
converting the keyword source file
to the SMP/E output data set. See
the following GIMDTS listing.
Explanation
When building the ISPF Configuration table into an
SMP/E USERMOD an error was encountered with
GIMDTS converting the keyword source file to the
SMP/E output data set. See the generated GIMDTS
listing for details.
ISPC291 IEBCOPY error - An error
was encountered with IEBCOPY
unloading the load module
aaaaaaaa. to a temporary
sequential file. See the IEBCOPY
listing.
Explanation
When building the ISPF Configuration table into an
SMP/E USERMOD an error was encountered with
IEBCOPY unloading the load module for either the
configuration table, module ISPCFIGU, or the VSAM
restrictions table, module ISPCFIGV to a temporary
sequential file. See the generated IEBCOPY listing for
details
ISPC292 GIMDTS error - An error
was encountered with GIMDTS
converting the IEBCOPY unloaded
load module aaaaaaaa. to the
SMP/E output data set. See the
GIMDTS listing.'
Explanation
When building the ISPF Configuration table into an
SMP/E USERMOD an error was encountered with
GIMDTS converting the IEBCOPY unloaded data set to
the SMP/E output data set. See the generated GIMDTS
listing for details.
ISPC293 EXECIO error - An error was
encountered writing the USERMOD
to the output data set. Data set
may be full, compress the library
and retry the function.
Explanation
When building the ISPF configuration table into an
SMP/E USERMOD an EXECIO error was encountered
writing the USERMOD to the output data set.
ISPC294 USERMOD built - The SMP/E
USERMOD was successfully saved
to aaaaaaaa..
Explanation
The ISPF configuration table was successfully built
into an SMP/E USERMOD.
ISPC295 Enter DDDEF name - Enter the
DDDEF name to be used as the
DISTLIB for the load modules in
the SMP/E USERMOD.
Explanation
This is an informational message.
ISPC296 Enter the seven character name
of a prior USERMOD to be
superseded by this SMP/E
USERMOD. The fields must be
filled from top to bottom.
Explanation
This is an informational message.
ISPC297 Invalid data set name - The data
set name specified is invalid. The
SMP/E data set name must not
contain a member name.
ISPF messages starting with ISP
30  z/OS: z/OS ISPF Messages and Codes

## Page 51

Explanation
The data set name entered is syntactically incorrect. It
must not contain a member name.
User response
Correct the data set name.
ISPC298 Incorrect attributes - The SMP/E
data set must be a partitioned
data set with a record length of 80
and a record format of FB.
Explanation
This is an informational message.
ISPC299 Enter a seven character identifier
for the SMP/E USERMOD.
Explanation
This is an informational message.
ISPC300 Use a different name. ISPCFIGU is
currently being used by ISPF and
a copy from a different data set
cannot be loaded.
Explanation
This message is self-explanatory.
ISPC301 ISPC301W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate MSG tag SUFFIX found.
This MSG tag will be ignored.
Explanation
Message SUFFIX must be unique for each message
within a MSGMBR.
User response
Remove the duplicate suffix specification and rerun
the conversion utility.
ISPC302 ISPC302W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Message text is longer than
cccccccc characters and will be
truncated.
Explanation
The message text provided exceeds the ISPF
maximum message length.
User response
Reduce the length of the message text and rerun the
conversion utility.
ISPC303 ISPC303W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
MSGMBR has no messages. No
message file will be written.
Explanation
No valid MSG tags were formatted within the current
MSGMBR tag.
User response
Correct reported problems or include MSG tag(s)
within the MSGMBR tag, as appropriate, and rerun the
conversion utility.
ISPC304 ISPC304W: Warning. Line
aaaaaaaa of file "bbbbbbbb". MSG
tag contains no text. Message
cannot be created.
Explanation
MSG tag must include the message text.
User response
Add the required message text and rerun the
conversion utility.
ISPC305 ISPC305W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Conflict on VARSUB tag between
attributes "VAR" and "FIELD".
"FIELD" cannot be specified if
"VAR" is used. "FIELD" is ignored
for this conversion.
Explanation
DTL does not allow both VAR and FIELD attributes on
the same VARSUB tag. FIELD is not supported by ISPF.
User response
Remove the FIELD attribute and rerun the conversion
utility.
ISPC307 ISPC307W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The SUFFIX value "cccccccc" is
not valid because the resulting
message number would be more
than 8 characters in length. This
MSG tag will be ignored.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  31

## Page 52

Explanation
A combination of a 7 character MSGMBR name and a 2
character SUFFIX is invalid.
User response
Change either the MSGMBR name or the SUFFIX and
rerun the conversion utility.
ISPC309 ISPC309W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is a conflict in the use
of "WIDTH=cccccccc" on the
MSGMBR tag and "FORMAT=ASIS"
on the MSG tag. The message may
not display as intended.
Explanation
This is an informational message.
ISPC310 ISPC310W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The first line of the message
created by MSG tag formatting
is longer than 80 characters.
Message cannot be created.
Explanation
A combination of short message and message
attributes for the first line of a message has exceeded
the width of the message file.
User response
Remove the short message specification or reduce the
length of variable names used for message attributes
HELP, MSGTYPE, and LOCATION, as appropriate.
ISPC311 ISPC311W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
NAME on MSGMBR tag is not valid.
No message file will be written.
Explanation
The name provided does not follow the ISPF message
member name format.
User response
Change the name to match the ISPF name format and
rerun the conversion utility.
ISPC320 Invalid Application ID - Enter 1
to 4 alphanumeric characters (first
cannot be numeric). Alternatively,
use the special form '*m:n' to use
from character 'm' to 'n' of the
system name (ZSYSID)
Explanation
A valid Application ID consists of 1 to 4 alphanumeric
characters. The first character must be alphabetic (not
numeric).
Alternatively, use the special form '*m:n' to obtain the
application ID from the system name (ZSYSID) when
ISPF is initialized. 'm' and 'n' are the start and end
positions used to sub-string the system name.
The default start position(m) is 1 and the default end
position(n) is the start position + 3, to a maximum
value of 8.
User response
Specify a valid application ID.
ISPC321 Invalid Application ID - The
start position for obtaining the
application ID from the system
name (ZSYSID) must be numeric
and in the range 1 to 8.
Explanation
The specification of the start position for the special
form of the application ID is invalid. The start position,
if specified, must be in numeric and in the range 1 to 8.
The default start position is 1.
User response
Correct the start position
ISPC322 Invalid Application ID - The
colon(:) before the end position
has been omitted
Explanation
The specification of the end position for the special
form of the application ID must be preceded by a
colon(:)
User response
Insert a colon(:) before the end position.
ISPC323 Invalid Application ID - The
end position for obtaining the
application ID from the system
name (ZSYSID) must be numeric
and in the range 1 to 8. It must
also be greater than or equal to
ISPF messages starting with ISP
32  z/OS: z/OS ISPF Messages and Codes

## Page 53

the start position, and not more
than start position + 3
Explanation
The specification of the end position for the special
form of the application ID is invalid. The end position,
if specified, must be preceded by a colon(:) and be in
range 1 to 8. It must also be greater than or equal to
the start position and not more than the start position
plus 3.
The default end position is the start position + 3 to a
maximum value of 8.
User response
Correct the end position.
ISPC330 Invalid Qualifier - Enter up to
8 alphanumeric characters (first
cannot be numeric), or enter
a qualifier containing system
symbolic variables.
Explanation
The Temporary Data Set Qualifier consists of either
1. a valid name of 1 to 8 alphanumeric characters
and the first character must be alphabetic (not
numeric), or
2. a valid name consisting of one or more system
symbolic variables (eg: &SYSNAME) that resolve to
a valid name of alphanumeric characters and the
first characters must be alphabetic (not numeric).
Where the resolved qualifier exceeds 8 characters,
it will be truncated to 8 characters.
User response
Specify a valid data set qualifier
Refer to z/OS MVS Initialization and Tuning Reference
for details on using system symbolic variables
ISPC331 Invalid Qualifier - Characters
not associated with a system
symbolic variable must be valid
alphanumeric characters (first
cannot be numeric).
Explanation
Any text characters used around any system symbolic
variables must be valid alphabetic characters and the
first must be alphabetic (not numeric).
User response
Specify valid alphanumeric characters around any
system symbolic variables.
ISPC332 Symbol Name Invalid - Enter
up to 8 alphanumeric characters,
following the '&' sign (first cannot
be numeric).
Explanation
A valid symbol name consists of an '&' sign followed by
1 to 8 alphanumeric characters and the first character
must be alphabetic (not numeric).
A symbol name may optionally include a start position
and length to substring the symbolic variable, and may
include a terminating period (eg: &SYSNAME(1:4). to
use the first 4 characters of &SYSNAME).
User response
Specify a valid symbol name
Refer to z/OS MVS Initialization and Tuning Reference
for details on using system symbolic variables
ISPC333 Start Position Invalid - The Start
Position for Symbol aaaaaaaa
must be non-zero numeric value
in the range -bbbbbbbb to
+bbbbbbbb..
Explanation
The starting substring position for a system symbolic
variable must be numeric and in the range from 1 to 1
more than the length of the system symbolic variable
name, to a maximum value of 8. The number may
be negative to indicate the indicate a start position
relative to the end of the symbolic variable.
User response
Specify a valid start position
Refer to z/OS MVS Initialization and Tuning Reference
for details on using system symbolic variables
ISPC334 Substring Length Invalid - The
Substring Length for Symbol
aaaaaaaa must be a positive
numeric value in the range 1 to
bbbbbbbb..
Explanation
The substring length for a system symbolic variable
must be a positive (non-zero) numeric value. The Start
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  33

## Page 54

position plus the length must not exceed the length of
the system symbolic variable.
User response
Specify a valid substring length
Refer to z/OS MVS Initialization and Tuning Reference
for details on using system symbolic variables
ISPC335 Substring Parms Missing - Specify
both substring start position
and length separated by ':' and
enclosed in '()'.
Explanation
One or more substring parameters for a system
symbolic symbol is missing. Specify the substring start
position and length separated by a ':'
eg: &SYSNAME(1:4)
User response
Specify both the substring start position and length in
brackets and separated by a ':' or omit the parameter
completely to use the entire symbol value.
Refer to z/OS MVS Initialization and Tuning Reference
for details on using system symbolic variables
ISPC400 ISPC400W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
External-command-name must be
one word. It cannot contain any
blanks. The external-command-
name is ignored.
Explanation
This message is self-explanatory.
User response
Enter a valid command name and rerun the conversion
utility.
ISPC401 ISPC401W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
RUN option for the ACTION tag
was specified without a command.
The ACTION tag is ignored.
Explanation
This message is self-explanatory.
User response
Add a command name to the RUN attribute on the
ACTION tag and rerun the conversion utility.
ISPC402 ISPC402W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Internal-command-name must be
equal to the external-command-
name in order to allow command
truncation when the T tag is
used. There will be no minimum
command name in this command.
Explanation
This message is self-explanatory.
User response
Correct the DTL source so the internal-command-
name and external-command-name match, and rerun
the conversion utility.
ISPC403 ISPC403W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Command table was not written
because the required "APPLID"
keyword was not specified.
Explanation
This message is self-explanatory.
User response
Specify the APPLID attribute on the CMDTBL tag
ISPC404 ISPC404W: Warning. Line
aaaaaaaa of file "bbbbbbbb". An
error occurred while processing
a command table. Processing of
command table has been canceled
after a return code of cccccccc
from table service "dddddddd".
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for explanations of
table services return codes.
ISPC405 ISPC405E: Error. ISPF Command
Table was not created. An
error occurred while opening a
command table. Refer to your
ISPF messages starting with ISP
34  z/OS: z/OS ISPF Messages and Codes

## Page 55

ISPF documentation on TBOPEN
for a Return Code of "aaaaaaaa".
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for explanations of
table services return codes.
ISPC406 ISPC406E: Error. ISPF Command
Table was not created. Possible
reasons may be that the
conversion tool was not run as a
dialog on ISPF or ISPF was not
active.
Explanation
This message is self-explanatory.
User response
Rerun the conversion utility from the ISPF command
line.
ISPC407A ISPC407AE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "16" (ISPTABL file not
allocated) from the TBCLOSE
service attempting to create an
ISPF Command Table.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for explanations of
table services return codes.
ISPC407B ISPC407BE: Error. Line aaaaaaaa
of file "bbbbbbbb". Return code
of "cccccccc" (Severe Error) from
the TBCLOSE service attempting to
create an ISPF Command Table.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for explanations of
table services return codes.
ISPC408 ISPC408E: Error. Line aaaaaaaa
of file "bbbbbbbb". The dynamic
area identifier "cccccccc" is longer
than the dddddddd characters of
dynamic area width.
Explanation
This message is self-explanatory.
User response
Increase the width of the dynamic area to at least the
number of characters shown in the area identifier.
ISPC409 ISPC409E: Error. Line aaaaaaaa
of file "bbbbbbbb". The graphic
area identifier "cccccccc" is longer
than the dddddddd characters of
graphic area width.
Explanation
This message is self-explanatory.
User response
Increase the width of the graphic area to at least the
number of characters shown in the area identifier.
ISPC410 ISPC410W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The ALIAS for command cccccccc
was found after the command.
The ALIAS entry must precede
the command it references. The
ALIAS will be unavailable to ISPF.
Please update your tag source file
to move the ALIAS before the
referenced command.
Explanation
This message is self-explanatory.
User response
Move the ALIAS reference in the DTL source as
indicated and rerun the conversion utility.
ISPC411 ISPC411W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc is longer than dddddddd
characters, will be truncated.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  35

## Page 56

User response
Adjust cccccccc to be equal to dddddddd characters
and rerun the conversion utility.
ISPC412 ISPC412W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
LSTGRP heading must include at
least one field from the first model
line to be displayed. This heading
cannot be formatted and is being
reset to blanks.
Explanation
This message is self-explanatory.
User response
At least one LSTCOL tag from the first model line
should be included within the LSTGRP tag.
ISPC413 ISPC413W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
External command string cannot
be less than 2 characters in length
and will be set to " ".
Explanation
This message is self-explanatory.
User response
Correct the command name to be a minimum of 2
characters in length and rerun the conversion utility.
ISPC414 ISPC414W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Only
one keyword from this list can be
specified on an ACTION tag: RUN,
CLASS, SETVAR, TOGVAR.
Explanation
This message is self-explanatory.
User response
Correct the DTL source to specify only one of the
attributes listed and rerun the conversion utility.
ISPC415 ISPC415W: Warning. Line
aaaaaaaa of file "bbbbbbbb". ISPF
will support a Command Action
up to 240 characters in length.
The Command Action will be
truncated.
Explanation
This message is self-explanatory.
User response
Correct the command action to the stated length limit
and rerun the conversion utility.
ISPC416 ISPC416W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
ISPF does not support the
"ACTION=cccccccc" coded on the
dddddddd tag. The Command
Action will be set to "NOP".
Explanation
This message is self-explanatory.
User response
Replace the invalid action and rerun the conversion
utility.
ISPC417 ISPC417E: Error. Line aaaaaaaa
of file "bbbbbbbb". Additional
parameters must follow cccccccc
and will affect processing.
Explanation
This message is self-explanatory.
User response
Add the additional command parameters and rerun
the conversion utility.
ISPC418 ISPC418W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
parameters are valid following
cccccccc..
Explanation
This message is self-explanatory.
User response
Remove the indicated invalid parameters and rerun the
conversion utility.
ISPC419 ISPC419W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
commands were specified within
the CMDTBL tag. No command
table will be written.
ISPF messages starting with ISP
36  z/OS: z/OS ISPF Messages and Codes

## Page 57

Explanation
This message is self-explanatory.
User response
Add the required CMD tag definitions and rerun the
conversion utility.
ISPC420 ISPC420W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
valid CMDACT ACTION was not
specified for cccccccc.
Explanation
This message is self-explanatory.
User response
Provide a valid command action and rerun the
conversion utility.
ISPC421 ISPC421W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" exceeds the maximum
length of dddddddd text
characters that can be placed on
the current panel command line.
"cccccccc" will be truncated to fit
on the command line.
Explanation
This message is self-explanatory.
User response
Shorten the command prompt text or increase the
panel width, as appropriate, and rerun the conversion
utility.
ISPC422 ISPC422W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
This panel contains a )MODEL
line generated by the LSTFLD
and LSTCOL tags but not a
required CMDAREA command line
to accompany it. A command line
is being added to the panel.
Explanation
This message is self-explanatory.
User response
Provide a CMDAREA tag to eliminate this message.
ISPC423 ISPC423W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
requested SCROLL amount field
will not fit on the command line
for this panel. The SCROLL amount
field is ignored.
Explanation
This message is self-explanatory.
User response
Shorten the command prompt text or increase the
panel width, as appropriate, and rerun the conversion
utility.
ISPC423A ISPC423AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
point-and-shoot text as specified
by the PSBUTTON attribute will
not fit on the command line for this
panel. The point-and-shoot text is
not added to the command line.
Explanation
This message is self-explanatory.
User response
Shorten the command prompt text, shorten the
point-and-shoot text or increase the panel width, as
appropriate, and rerun the conversion utility.
ISPC424 ISPC424E: Error. Line aaaaaaaa
of file "bbbbbbbb". An error
occurred during the completion
of processing for command table
"cccccccc". Processing of the
command table has been canceled
after a return code of dddddddd
from ISPF service "eeeeeeee".
A temporary command table
member "ffffffff" has been created
on table file "gggggggg".
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for explanations of
table services return codes.
ISPC425 ISPC425W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" tag field name
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  37

## Page 58

"dddddddd" exceeds the
maximum field width of eeeeeeee
characters available for the name.
The NOZVARS option is ignored for
field name "dddddddd".
Explanation
This message is self-explanatory.
User response
Shorten the field name to be less than or equal to the
field width and rerun the conversion utility.
ISPC426 ISPC426E: Error. Line aaaaaaaa of
file "bbbbbbbb". "LINE=cccccccc"
is not valid because a previous
variable model line definition
exists for this line.
Explanation
This message is self-explanatory.
User response
Specify a different line number for this variable model
line definition and rerun the conversion utility.
ISPC427 ISPC427E: Error. Line aaaaaaaa of
file "bbbbbbbb". "LINE=cccccccc"
is not valid because a standard
model line definition exists for this
line.
Explanation
This message is self-explanatory.
User response
Specify a different line number for this variable model
line definition and rerun the conversion utility.
ISPC430 ISPC430W: Warning. Line
aaaaaaaa of file "bbbbbbbb". This
is a selection panel and requires
a command line, but no CMDAREA
tag was present. A command line
is being added to the panel.
Explanation
This message is self-explanatory.
User response
Provide a CMDAREA tag to eliminate this message.
ISPC431 ISPC431W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
default name "ZCMD" was used
to generate ")INIT" and ")PROC"
section panel statements. These
panel statements are not correct
because the CMDAREA tag name
has been specified as "cccccccc".
Move the CMDAREA tag so that it
is placed before the SELFLD tag
that defines the menu choices and
reconvert the panel.
Explanation
The CMDAREA tag was placed in the DTL source file
following the SELFLD tag. The specified CMDAREA
name "cccccccc" was not available when the panel
statements were generated.
User response
Move the CMDAREA tag so that it is placed before
the SELFLD tag that defines the menu choices. The
specified name "cccccccc" will be used in the panel
statements.
ISPC432 ISPC432W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
ACTION tags following an ACTION
tag with the RUN attribute
specified are not valid and will be
ignored.
Explanation
This message is self-explanatory.
User response
Place the ACTION tag with the RUN attribute after the
other ACTION tags for this PDC or CHOICE tag and
rerun the conversion utility.
ISPC433 ISPC433W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
TYPE=cccccccc on the SELFLD tag
is not valid on a panel when the
MENU keyword is not specified on
the PANEL tag. This tag will be
formatted as TYPE=SINGLE.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
38  z/OS: z/OS ISPF Messages and Codes

## Page 59

User response
Add the MENU attribute to the PANEL tag and rerun
the conversion utility.
ISPC434 ISPC434W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Only
one occurrence of a SELFLD tag
with a TYPE attribute value of
MENU, MODEL or TUTOR can be
specified on a panel. This tag will
be formatted as TYPE=SINGLE.
Explanation
This message is self-explanatory.
User response
Place each set of option menu selections on a
separate panel.
ISPC435 ISPC435W: Warning. Line
aaaaaaaa of file "bbbbbbbb". If
an ACTION tag within a SELFLD
tag defined as TYPE=TUTOR is
coded with the TYPE attribute, the
TYPE value must be specified as
PANEL. The ACTION tag attribute
TYPE=cccccccc is ignored.
Explanation
This message is self-explanatory.
User response
Place each set of option menu selections on a
separate panel.
ISPC436 ISPC436W: Warning. Line
aaaaaaaa of file "bbbbbbbb". If
an ACTION tag within a SELFLD
tag is coded as TYPE=VAR, the
RUN attribute must be specified as
"%varname". The ACTION tag will
be processed as TYPE=CMD.
Explanation
This message is self-explanatory.
User response
Place each set of option menu selections on a
separate panel.
ISPC500 ISPC500W: Warning. Multicultural
support language literals could
not be obtained by ISPF for the
requested language. The values
for the current session language
will be used.
Explanation
This message is self-explanatory.
System programmer response
Make sure that all multicultural support modules have
been made available to ISPF. Verify that the user
is running with enough virtual storage to allow ISPF
to load the requested language module. If the error
persists, contact IBM support.
User response
Contact your system programmer.
ISPC501 ISPC501W: Warning. Multicultural
support language literals could
not be obtained by ISPF for the
requested language. The default
English literals will be used.
Explanation
This message is self-explanatory.
System programmer response
Make sure that all multicultural support modules have
been made available to ISPF. Verify that the user
is running with enough virtual storage to allow ISPF
to load the requested language module. If the error
persists, contact IBM support.
User response
Contact your system programmer.
ISPC502 ISPC502W: Warning. ISPDTLC
received a nonzero return code
from EXECIO while processing
'aaaaaaaa'. Refer to the
'bbbbbbbb' documentation for an
EXECIO Return code = 'cccccccc'.
Explanation
This message is self-explanatory.
User response
Correct the condition causing return code cccccccc.
ISPC503 ISPC503W: Warning. Data will
not replace the 'aaaaaaaa'
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  39

## Page 60

file because NOREPLACE was
specified on invocation.
Explanation
This message is self-explanatory.
User response
To replace the data on the aaaaaaaa file, change the
invocation option to REPLACE or select the Replace
Panel/Message/Script option on the invocation panel.
ISPC503A ISPC503AW: Warning. Data will
not replace the 'aaaaaaaa'
file because NOLOGREPL was
specified on invocation.
Explanation
This message is self-explanatory.
User response
To replace the data on the aaaaaaaa file, change the
invocation option to LOGREPL or select the Replace
Log File Members option on the invocation panel.
ISPC503B ISPC503BW: Warning. Data will
not replace the 'aaaaaaaa'
file because NOLISTREPL was
specified on invocation.
Explanation
This message is self-explanatory.
User response
To replace the data on the aaaaaaaa file, change the
invocation option to LISTREPL or select the Replace
List File Members option on the invocation panel.
ISPC504 ISPC504E: Error. Log file output
is directed to the ISPF log file.
The ISPF log file is not allocated.
Change ISPF Option 0 to allocate
the log file or provide an ISPDTLC
log file.
Explanation
The ISPF log file is not allocated and cannot be used
by ISPDTLC.
User response
If the user intends to direct the log file to the ISPF
log and no ISPF log can be allocated, then the Option
0 (ISPF Settings) update is required to allow ISPF to
allocate the log file. Alternatively, the user can create
a private log file and provide this log file name to
ISPDTLC. If the ISPDTLC command syntax invocation
is being used, the log file name is provided in the
ISPDTLC profile. If the interactive ISPDTLC invocation
panel is being used, then the log file name is entered
on the panel.
ISPC505 ISPC505E: Error. File was not
preprocessed by ISPF. Possible
reasons may be the Conversion
was not successful resulting
in no generated output for
preprocessing, or the ISPF
Preprocessing Utility was unable
to successfully preprocess a
generated file. Preprocessor input
is on work file aaaaaaaa.. Refer
to the ISPF log file for more
information about the ISPPREP
error.
Explanation
A nonzero return code was returned from ISPPREP.
User response
Correct the error described in the ISPF log.
ISPC506 ISPC506E: Error. File 'aaaaaaaa'
was not found or was empty.
Explanation
This message is self-explanatory.
User response
Provide a file that contains DTL source records.
ISPC507 ISPC507E: Error. Data cannot be
written to disk because a name
conflict exists between input
aaaaaaaa file "bbbbbbbb" and
output file cccccccc "dddddddd".
Explanation
The input file dddddddd will be overwritten.
User response
Provide a unique output member name.
The input and output file names may be the same,
but the member name for the output file must be
different than the member name for the input file to
ISPF messages starting with ISP
40  z/OS: z/OS ISPF Messages and Codes

## Page 61

avoid overwriting the source data. cccccccc identifies
the output file type where the conflict was found.
ISPC508 ISPC508W: Warning. Data cannot
be written to the file name
'aaaaaaaa'. There is a name
conflict with active ISPF files.
Explanation
The output name specified for the ISPDTLC log or list
file conflicts with the ISPF log or list name.
User response
Provide a unique output log or list name.
ISPC508A ISPC508AE: Error. You cannot
specify the same file name for
both the LOG and LIST files.
Explanation
The output file names specified for the LOG and LIST
file are the same.
User response
Provide a unique output log or list name.
The input and output file names may be the same, but
the member name for the LOG file must be different
than the member name for the LIST file.
ISPC509 ISPC509E: Error. Incorrect syntax:
'aaaaaaaa'.
Explanation
ISPDTLC parameters provided as command invocation
syntax are in error.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the command syntax.
ISPC510 ISPC510E: Error. Incorrect syntax:
Option must be "aaaaaaaa" or
"bbbbbbbb" but not both.
Explanation
This message is self-explanatory.
User response
Specify either option aaaaaaaa or option bbbbbbbb.
ISPC511 ISPC511E: Error. Incorrect syntax:
aaaaaaaa="bbbbbbbb". Keylist
Application ID must be from 1 to
4 characters in length.
Explanation
This message is self-explanatory.
User response
Provide a 1 to 4 character application ID.
ISPC512 ISPC512E: Error. Incorrect syntax:
aaaaaaaa="bbbbbbbb" contains
character(s) which are not
valid. The first character of
the Keylist Application ID
must be 'cccccccc'or 'dddddddd'.
Characters 2 - 4, if entered,
must be 'cccccccc', 'dddddddd' or
'eeeeeeee'.
Explanation
This message is self-explanatory.
User response
Provide a correct 1 to 4 character application ID.
ISPC513 ISPC513W: Warning. Conversion
option 'aaaaaaaa' is valid only
when running ISPDTLC under ISPF
Option 7 (Dialog Test). 'aaaaaaaa'
changed to 'bbbbbbbb'.
Explanation
Option aaaaaaaa is valid only for ISPF test mode
processing and is changed to bbbbbbbb.
User response
Remove the specification of option aaaaaaaa unless
you are running in ISPF test mode.
ISPC514 ISPC514W: Warning. Invocation
option "aaaaaaaa" has been
removed.
Explanation
Invocation option "aaaaaaaa" has been removed from
ISPDTLC.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  41

## Page 62

User response
Remove the specification of option "aaaaaaaa" from
the command syntax invocation.
ISPC515 ISPC515W: Warning. Option is
not valid: 'aaaaaaaa' changed to
'bbbbbbbb'.
Explanation
Option aaaaaaaa is not valid for batch processing and
is changed to bbbbbbbb.
User response
Remove the specification of option aaaaaaaa from the
batch invocation.
ISPC516 ISPC516W: Warning. ISPF option
0 log file disposition is set to
"aaaaaaaa". Log file messages
cannot be written to the ISPF log.
The "DISK" option is ignored.
Explanation
This message is self-explanatory.
User response
Change the log file disposition or place log messages
on an ISPDTLC log file.
ISPC517 ISPC517E: Error. Unable to
gain exclusive allocation of
"aaaaaaaa". The file "aaaaaaaa"
will not be written.
Explanation
This message is self-explanatory.
User response
Use a private output file instead of file "aaaaaaaa".
ISPC518 ISPC518E: Error. Conversion
Utility message number aaaaaaaa
is missing in message file:
'bbbbbbbb'.
Explanation
This message is self-explanatory.
System programmer response
Make sure that all messages provided for ISPF have
been properly installed. If the error persists, contact
IBM support.
User response
Contact your system programmer.
ISPC519 ISPC519E: Error. Line aaaaaaaa of
file "bbbbbbbb". The ISPF LIBDEF
service for "cccccccc" has ended
with return code: "dddddddd".
"eeeeeeee" will not be updated.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Services Guide for a description of
LIBDEF return codes.
ISPC521 ISPPREP is being called to
preprocess aaaaaaaa panel(s)
from the work file bbbbbbbb to the
panel file cccccccc..
Explanation
This is an informational message.
ISPC522 ISPC522W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
panel with the "cccccccc" option
specified cannot be preprocessed.
Panel dddddddd will be saved in
ISPF source format.
Explanation
This message is self-explanatory.
User response
If ISPDTLC was invoked with command syntax, specify
the NOPREP conversion option. If the invocation was
from the interactive panel, deselect the Preprocess
Panel Output option.
ISPC523 ISPC523W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ISPF LIBDEF service for "cccccccc"
has ended with return code:
"dddddddd". Panel "eeeeeeee" will
not be displayed.
ISPF messages starting with ISP
42  z/OS: z/OS ISPF Messages and Codes

## Page 63

Explanation
The ISPDTLC DISPLAY option has attempted a LIBDEF
service in order to display panel eeeeeeee. The LIBDEF
service ended with return code dddddddd. Panel
eeeeeeee cannot be displayed.
User response
Refer to z/OS ISPF Services Guide for a description of
LIBDEF return codes.
ISPC524 ISPC524W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ISPF DISPLAY service has ended
with return code: "cccccccc". The
error text is: "dddddddd". Panel
"eeeeeeee" will be displayed using
the VIEW service.
Explanation
The ISPDTLC DISPLAY option has encountered an
error trying to display panel eeeeeeee. The DISPLAY
service ended with return code dddddddd. The
DISPLAY service error description is: "dddddddd".
User response
Refer to z/OS ISPF Services Guide for a description of
DISPLAY return codes.
ISPC525 The "aaaaaaaa" file cannot be
saved because no aaaaaaaa file
records have been created.
Explanation
This message is self-explanatory.
User response
Issue the SAVELOG, SAVELIST, or SAVEALL commands
only when a log or list file member is to be written to a
partitioned data set.
ISPC526 The "aaaaaaaa" file was specified
as a sequential format file. All
records have previously been
written.
Explanation
This message is self-explanatory.
User response
Issue the SAVELOG, SAVELIST, or SAVEALL commands
only when a log or list file member is to be written to a
partitioned data set.
ISPC527 The "aaaaaaaa" file was defaulted
to the ISPF aaaaaaaa file. All
records have previously been
written.
Explanation
The option to write the ISPDTLC log or list file to
disk has been selected, but no log or list name was
provided. ISPDTLC has placed the log or list output on
the ISPF log or list file, respectively.
User response
Issue the SAVELOG, SAVELIST, or SAVEALL commands
only when a log or list file member is to be written to a
partitioned data set.
ISPC528 There are no records available to
be saved to the aaaaaaaa file.
Explanation
No log or list records have been created since the
last SAVELOG, SAVELIST, or SAVEALL command was
issued.
User response
Issue the SAVELOG, SAVELIST, or SAVEALL commands
only when a log or list file member is to be written to a
partitioned data set.
ISPC529 The output library "aaaaaaaa"
is being updated with member
"bbbbbbbb".
Explanation
This is an informational message. A pending log or list
file is being written to the specified output file.
ISPC530 ISPC530W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
record length for output panels is
not for fixed length records of 80
bytes. The output MACLIB cannot
be used. The panel will be written
to "cccccccc".
Explanation
This message is self explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  43

## Page 64

ISPC533 Member processing is supported
only for filetype 'MACLIB'.
Explanation
This message is self explanatory.
ISPC534 Table for gml MACLIB members
not created.
Explanation
This message is self explanatory.
ISPC535 Gml MACLIB member not added to
member table.
Explanation
This message is self explanatory.
ISPC536 Selection code entered is not
valid.
Explanation
This message is self explanatory.
ISPC537 No members match the specified
pattern.
Explanation
This message is self explanatory.
ISPC560 ISPC560W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is a conflict in the use
of "cccccccc" with "dddddddd".
"dddddddd" is being reset to
"eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change either the cccccccc or the dddddddd value to
remove the conflict.
ISPC561 ISPC561W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" tag and its attributes
have been removed from the tag
language. cccccccc will be ignored
for this conversion and will be
syntax checked only. The cccccccc
tag should be removed from the
tag source as it is no longer
required.
Explanation
This message is self-explanatory.
User response
Remove the obsolete tag from the DTL source file.
ISPC562 ISPC562W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc=dddddddd" is not
defined. Default values will be set.
Explanation
This message is self-explanatory.
User response
Provide the missing attribute.
ISPC563 ISPC563W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" tag and its attributes
will be ignored during the
conversion process and syntax
checked only. Text following the
"cccccccc" tag will appear as
normal text.
Explanation
This is an informational message. The cccccccc tag is
not supported by ISPF.
ISPC564 ISPC564W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is a conflict in the use of
"cccccccc" with "dddddddd". There
is not enough space to format
the text "dddddddd". Reduce the
text to eeeeeeee bytes, or increase
ffffffff. to gggggggg..
Explanation
This message is self-explanatory.
ISPC565 ISPC565W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Tag
"cccccccc" and its attributes will
be ignored in the conversion
process.
ISPF messages starting with ISP
44  z/OS: z/OS ISPF Messages and Codes

## Page 65

Explanation
This is an informational message. The cccccccc tag is
not supported by ISPF.
ISPC566 ISPC566W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Keyword "cccccccc=dddddddd"
found on the eeeeeeee tag will be
ignored in the conversion process
but will be syntax checked.
Explanation
This is an informational message. The attribute
cccccccc is not supported by ISPF.
ISPC568 ISPC568W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Comment ending delimiter
"cccccccc" should be changed to
"-->".
Explanation
This message is self-explanatory.
ISPC569 ISPC569W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc attribute is missing for
the dddddddd tag. The conversion
process will use a default value.
Please update the tag source file.
Explanation
This message is self-explanatory.
User response
Add the cccccccc attribute definition to the dddddddd
tag.
ISPC570 ISPC570W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Keyword "cccccccc" found on the
dddddddd tag will be ignored in
the conversion process but will be
syntax checked.
Explanation
This is an informational message. The cccccccc
attribute cannot be used and will be ignored.
ISPC571 ISPC571W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
attribute value "cccccccc" is not
valid when the dddddddd tag is
found in this nesting context.
Explanation
The cccccccc attribute cannot be used and will be
ignored.
User response
Remove the cccccccc attribute on this use of the
dddddddd tag.
ISPC572 ISPC572W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
attribute "PMTWIDTH=cccccccc"
does not provide enough space to
format the ISPF prompt indicator
"===>". "PMTFMT=ISPF" will be
ignored.
Explanation
The combination of the length of the prompt text plus
the length of the ISPF prompt indicator will not fit
within the prompt width specified.
User response
Increase the PMTWIDTH attribute value.
ISPC573 ISPC573W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is not enough space to
format: "cccccccc".
Explanation
The combination of the length of the current text plus
the width of the previously formatted portions of the
panel will not fit within the current region, area, or
panel width specified.
User response
Increase the appropriate WIDTH attribute value, if
possible, or adjust the width of previously formatted
parts of the panel to provide additional space for the
current text.
ISPC574 ISPC574W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is a conflict in the use of
"cccccccc" with "dddddddd". There
is not enough space to format the
text "eeeeeeee".
Explanation
This message is self-explanatory.
ISPC577 ISPC577W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  45

## Page 66

cccccccc="dddddddd" maximum
length is eeeeeeee character(s).
cccccccc will be truncated to
"ffffffff".
Explanation
This message is self-explanatory.
User response
Update the DTL source to the appropriate maximum
length.
ISPC578 ISPC578W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" minimum
length is eeeeeeee character(s).
cccccccc will be set to "ffffffff".
Explanation
This message is self-explanatory.
User response
Update the DTL source to the appropriate minimum
length.
ISPC579 ISPC579W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" contains
character(s) which are not valid.
cccccccc will be set to "eeeeeeee".
Explanation
• For panel attributes, pad characters, or expand
characters, the dddddddd character has been
previously used and another character should be
specified.
• For CHECKI tag processing of the PICT attribute,
the dddddddd character(s) are invalid for the "VER
(xxx,PICT,string)" panel statement.
• For MSGMBR tag processing of the NAME attribute,
the dddddddd character(s) do not follow the rules
for message member names.
• For MSG tag processing of the SUFFIX attribute, the
dddddddd character(s) are invalid or the SUFFIX is
more than two characters in length.
• For processing of the MSG attribute, the dddddddd
character(s) do not follow the rules for message
names.
• For other NAME (or %varname) validation
processing, the dddddddd character(s) do not follow
the rules for variable names.
User response
Correct the dddddddd character(s).
ISPC580 ISPC580E: Error. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
integer.
Explanation
This message is self-explanatory.
User response
Change dddddddd to an integer value.
ISPC581 ISPC581W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
integer. cccccccc will be set to
"eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change dddddddd to an integer value.
ISPC583 ISPC583W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
integer eeeeeeee.. cccccccc will be
set to "eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change dddddddd to an integer value.
ISPC584 ISPC584E: Error. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
integer within a range of eeeeeeee
to ffffffff..
Explanation
This message is self-explanatory.
User response
Change the value dddddddd to be within the specified
range.
ISPF messages starting with ISP
46  z/OS: z/OS ISPF Messages and Codes

## Page 67

ISPC585 ISPC585W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
integer within a range of eeeeeeee
to ffffffff.. cccccccc will be set to
"gggggggg".
Explanation
This message is self-explanatory.
User response
Change the value dddddddd to be within the specified
range.
ISPC586 ISPC586E: Error. Line aaaaaaaa
of file "bbbbbbbb". Incorrect
value "cccccccc" specified for
dddddddd="cccccccc".
Explanation
This message is self-explanatory.
User response
Change the cccccccc value to a valid choice.
ISPC587 ISPC587W: Warning. Line
aaaaaaaa of file
"bbbbbbbb". Incorrect value
"cccccccc" specified for
dddddddd="cccccccc". dddddddd
will be set to default "eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change the cccccccc value to a valid choice.
ISPC588 ISPC588W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate field names are
not permitted by ISPF.
cccccccc="dddddddd" has already
been defined as a panel field
name. cccccccc will be set to
"eeeeeeee".
Explanation
This message is self-explanatory.
User response
Choose a different dddddddd value for the cccccccc
attribute.
ISPC589 ISPC589W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate field names are not
permitted by ISPF. cccccccc has
already been defined as a panel
help field name. cccccccc will be
ignored.
Explanation
This message is self-explanatory.
User response
Choose a different name for this panel help reference.
ISPC600 ISPC600E: Error. Line aaaaaaaa
of file "bbbbbbbb". Multiple
Action Bars not allowed in panel
definition.
Explanation
Only one AB tag group is allowed in a panel definition.
User response
Remove the multiple AB tag(s).
ISPC601 ISPC601E: Error. Line aaaaaaaa
of file "bbbbbbbb". No Action Bar
Choices defined within AB Tag.
Explanation
A minimum of one ABC tag must be specified within
the AB tag.
User response
Add ABC tag definitions to the AB tag.
ISPC602 ISPC602W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag "dddddddd" attribute
value cannot be used without a
"eeeeeeee" attribute value.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  47

## Page 68

User response
Remove the dddddddd attribute or add an eeeeeeee
attribute, as appropriate.
ISPC603 ISPC603W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
RUN option for the ACTION tag can
only be nested within the PDC tag
or within the CHOICE tag when a
TYPE of MENU, MODEL or TUTOR
has been specified on the SELFLD
tag.
Explanation
This message is self-explanatory.
User response
Remove the RUN option for this ACTION tag.
ISPC604 ISPC604W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Required cccccccc "dddddddd"
was not provided and may affect
processing.
Explanation
The cccccccc tag text described as dddddddd is
missing.
User response
Provide the appropriate description.
ISPC605A ISPC605AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ABC tag limit of "cccccccc" has
been exceeded. Remaining action
bar choices will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the extra ABC tags from the DTL source.
ISPC605B ISPC605BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PDC tag limit of "cccccccc" has
been exceeded. Remaining pull-
down choices will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the extra PDC tags from the DTL source.
ISPC605C ISPC605CW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ASSIGNI tag limit of "cccccccc"
has been exceeded. Remaining
ASSIGNI tags will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the extra ASSIGNI tags from the DTL source.
ISPC605D ISPC605DW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
XLATI tag limit of "cccccccc" has
been exceeded. Remaining XLATI
tags will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the extra ASSIGNI tags from the DTL source.
ISPC606 ISPC606W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Cursor Position "cccccccc" exceeds
the length (dddddddd) of the
"eeeeeeee" field on the ffffffff tag.
The conversion utility will not
use the Cursor Position. ISPF will
default the position to 1.
Explanation
This message is self-explanatory.
User response
Change the cursor position to a value within the length
of the eeeeeeee field.
ISPC607 ISPC607W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
panel has been defined with either
a Width > 80 and/or a Depth
> 24 and will require a device
larger than 24x80 for display of
the panel.
ISPF messages starting with ISP
48  z/OS: z/OS ISPF Messages and Codes

## Page 69

Explanation
This message is self-explanatory.
ISPC608 ISPC608W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
text encountered following the
cccccccc tag.
Explanation
This message is self-explanatory.
User response
Add a description to the cccccccc tag definition
following the close tag delimiter.
ISPC609 ISPC609W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
text encountered following the
TOPINST tag.
Explanation
This message is self-explanatory.
User response
Add a description to be placed at the top of the panel.
ISPC610 ISPC610W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc "dddddddd" larger
than the maximum allowed.
"dddddddd" will be truncated to
eeeeeeee bytes.
Explanation
dddddddd is longer than the maximum allowed length
of eeeeeeee.
User response
Reduce the length of dddddddd to eeeeeeee.
ISPC611 ISPC611W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cursor field "cccccccc" specified on
the PANEL tag was not matched to
any field found on the panel.
Explanation
This message is self-explanatory.
User response
Correct the field name specified on the PANEL tag.
ISPC612 ISPC612W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cursor row will be set to "cccccccc"
as requested on the dddddddd
tag. "cccccccc" may not be a valid
value. The Cursor Index will be
checked by ISPF for valid position.
Explanation
This message is self-explanatory.
ISPC613 ISPC613W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
attribute "cccccccc=dddddddd"
will be ignored by the conversion
utility because the CURSOR
attribute has not been specified.
Explanation
This message is self-explanatory.
User response
Add the CURSOR attribute to the PANEL definition.
ISPC614 ISPC614W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The cursor position "cccccccc"
specified on the PANEL tag was
not set for any field found on the
panel.
Explanation
This message is self-explanatory.
ISPC615 ISPC615W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The cursor index to table row
"cccccccc" specified on the PANEL
tag was not set for any field found
on the panel.
Explanation
This message is self-explanatory.
ISPC616 ISPC616W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate Action Bar Choice
description is not allowed. ABC
will not be included on Action Bar.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  49

## Page 70

User response
Provide a unique action bar choice description.
ISPC617 ISPC617W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
No pull-down choice(s) found
for Action Bar Choice being
processed. ABC will not be
included on Action Bar.
Explanation
This message is self-explanatory.
User response
Provide at least one pull-down choice for each ABC
tag.
ISPC618 ISPC618W: Warning. Line
aaaaaaaa of file
"bbbbbbbb". Undefined value
"cccccccc" specified for
"dddddddd=cccccccc". cccccccc
has not been defined in a
dddddddd tag.
Explanation
This message is self-explanatory.
User response
Provide an cccccccc definition using an dddddddd tag.
ISPC618A ISPC618AW: Warning.
Line aaaaaaaa of file
"bbbbbbbb". Undefined value
"cccccccc" specified for
"dddddddd=cccccccc". "cccccccc"
has not been defined in a eeeeeeee
tag.
Explanation
This message is self-explanatory.
User response
Provide an cccccccc definition using an eeeeeeee
tag. For a CHOICE tag, cccccccc can be either a
system generated selection choice number or a value
specified using the SELCHAR attribute.
ISPC619 ISPC619W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate cccccccc ID
"dddddddd" specified for
"eeeeeeee=dddddddd". Only one
cccccccc entry is accepted for each
ID name. This entry is rejected
and the first one processed will be
used.
Explanation
This message is self-explanatory.
User response
Remove the duplicate cccccccc ID.
ISPC620 ISPC620E: Error. Line aaaaaaaa of
file "bbbbbbbb". The format for the
DOCTYPE statement is not valid.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC621 ISPC621E: Error. Line aaaaaaaa
of file "bbbbbbbb". Parentheses or
brackets are required for enclosing
ENTITY definition(s).
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC622 ISPC622E: Error. Line aaaaaaaa
of file "bbbbbbbb". No ending
comment delimiter (-->) found
during ENTITY processing of
source file record "cccccccc".
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPF messages starting with ISP
50  z/OS: z/OS ISPF Messages and Codes

## Page 71

ISPC623 ISPC623E: Error. Line aaaaaaaa of
file "bbbbbbbb". ENTITY Symbol
name expected.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC624 ISPC624E: Error. Line aaaaaaaa
of file "bbbbbbbb". Illegal ENTITY
symbol value encountered.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC625 ISPC625E: Error. Line aaaaaaaa
of file "bbbbbbbb". Each ENTITY
definition must be enclosed within
either single(') or double(") quotes
and must be ended with '>'. Check
for missing or mismatched quote
delimiters and verify that each
ENTITY is ended with '>'.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC626 ISPC626E: Error. Line aaaaaaaa of
file "bbbbbbbb". No <!ENTITY or
<:ENTITY definition encountered
or ENTITY definition(s) not ended
with a close parenthesis or close
bracket character.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC627 ISPC627E: Error. Line aaaaaaaa of
file "bbbbbbbb". Recursive use of
ENTITY file "cccccccc".
Explanation
This message is self-explanatory.
User response
Review the ENTITY file includes and remove the
recursive use of file "cccccccc".
ISPC628 ISPC628W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate ENTITY name "cccccccc"
found. This entry will be ignored
and the first use of name
"cccccccc" for value "dddddddd"
will be used.
Explanation
This message is self-explanatory.
User response
Remove the duplicate ENTITY name.
ISPC629 ISPC629W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
CDATA keyword cannot be used
with a parameter entity definition
and will be ignored.
Explanation
This message is self-explanatory.
User response
Refer to z/OS ISPF Dialog Tag Language Guide and
Reference for an explanation of the Document Type
and Entity declarations.
ISPC630 ISPC630W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ENTITY "cccccccc" will override
the system defined entity value
dddddddd with the new value
eeeeeeee..
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  51

## Page 72

Explanation
This message is self-explanatory.
ISPC631 ISPC631W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The maximum number of single
choices allowed within the limit
of "ENTWIDTH=cccccccc" has
been exceeded. The remaining
sequentially numbered CHOICE
tags will be ignored.
Explanation
This message is self-explanatory.
User response
Increase the ENTWIDTH value, if possible, or remove
the excess CHOICE tags.
ISPC631A ISPC631AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The maximum number of Tutorial
choices allowed within the ISPF
limit of 100 has been exceeded.
The remaining CHOICE tags will be
ignored.
Explanation
This message is self-explanatory.
User response
The ISPF limit of 100 Tutorial choices has been
exceeded. Additional Tutorial choices must be placed
on a different panel. Remove the excess CHOICE tags
from the current DTL source file.
ISPC632 ISPC632W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" will not
fit within the remaining space
available of eeeeeeee character(s).
cccccccc will be set to "ffffffff".
Explanation
This message is self-explanatory.
User response
Reduce the cccccccc value to eeeeeeee.
ISPC633 ISPC633W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute dddddddd
is not valid unless the eeeeeeee
attribute is also specified.
dddddddd=ffffffff will not be used.
Explanation
This message is self-explanatory.
User response
Remove the dddddddd attribute or include the
eeeeeeee attribute as part of the cccccccc tag
definition.
ISPC634 ISPC634W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute "dddddddd"
is not valid unless the "eeeeeeee"
attribute is also specified.
"dddddddd" will not be used.
Explanation
This message is self-explanatory.
User response
Remove the dddddddd attribute or include the
eeeeeeee attribute as part of the cccccccc tag
definition.
ISPC635 ISPC635W: Warning. The
aaaaaaaa option is not valid
unless the bbbbbbbb option is also
specified. aaaaaaaa will not be
used.
Explanation
This message is self-explanatory.
User response
Remove the aaaaaaaa option or add the bbbbbbbb
option.
ISPC636 ISPC636W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc=dddddddd" attribute
specification on the "eeeeeeee"
tag conflicts with the "ffffffff"
conversion option. The attribute
value is accepted but may cause
inconsistent results when the
panel is displayed.
ISPF messages starting with ISP
52  z/OS: z/OS ISPF Messages and Codes

## Page 73

Explanation
The ffffffff conversion option is in conflict with the use
of attribute cccccccc specified for value dddddddd on
the eeeeeeee tag.
User response
To eliminate the message either change the attribute
value or change the conversion utility option.
ISPC637 ISPC637W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc=dddddddd" attribute
specification on the "eeeeeeee"
tag is a reserved keyword. The
attribute value cannot be used and
is reset to blank.
Explanation
The specified dddddddd attribute value is a reserved
keyword.
User response
Specify a different value for the cccccccc attribute.
ISPC638 ISPC638W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute dddddddd
is not valid unless the eeeeeeee
conversion option is also specified.
dddddddd=ffffffff will not be used.
Explanation
This message is self explanatory.
User response
Remove the dddddddd attribute or include the
eeeeeeee conversion option on the ISPDTLC interactive
panel or as part of the ISPDTLC command invocation
syntax.
ISPC639 ISPC639W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute "dddddddd"
is not valid unless the "eeeeeeee"
conversion option is also specified.
"dddddddd" will not be used.
Explanation
This message is self explanatory.
User response
Remove the dddddddd attribute or include the
eeeeeeee conversion option on the ISPDTLC interactive
panel or as part of the ISPDTLC command invocation
syntax.
ISPC640 ISPC640W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
PDSEP tag has been found in the
DTL source before the first PDC tag
has been processed. The PDSEP
tag is ignored.
Explanation
PDSEP tags are valid only between PDC tags. A
separator cannot be placed before the first pull-down
choice.
User response
Remove the PDSEP tag which is coded before the first
PDC tag for this action bar item.
ISPC641 ISPC641W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
PDSEP tag has been found in the
DTL source after the last PDC tag
has been processed. The PDSEP
tag is ignored.
Explanation
PDSEP tags are valid only between PDC tags. A
separator cannot be placed after the last pull-down
choice.
User response
Remove the PDSEP tag which is coded after the last
PDC tag for this action bar item.
ISPC642 ISPC642W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Multiple PDSEP tags have been
found in the DTL source between
PDC tags. The first PDSEP tag is
accepted. Additional PDSEP tags
are ignored.
Explanation
Only one PDSEP tag is valid between PDC tags.
User response
Remove the multiple PDSEP tags which are coded
between PDC tags.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  53

## Page 74

ISPC643 ISPC643W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The combined specification of
accelerator key values for ACC1,
ACC2, and ACC3 exceed the
implementation limit of 30 bytes.
The accelerator key description
"cccccccc" will be discarded.
Explanation
The accelerator key description is limited to 20 bytes.
User response
Revise the selection of ACCn values to reduce the
number of bytes in the accelerator key description.
ISPC644 ISPC644W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" is not a valid single
accelerator key. Valid values are:
INSERT, DELETE, BACKSPACE, and
F1 through F12. The accelerator
key description "cccccccc" will be
discarded.
Explanation
CTRL, SHIFT, ALT, A-Z and 0-9 cannot be used as a
single accelerator key.
User response
Revise the accelerator key selection to use only valid
key combinations.
ISPC645 ISPC645W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" is a duplicate of a
previously specified accelerator
key. The accelerator key
description "cccccccc" will be
discarded.
Explanation
Do not specify the same accelerator key value for
attributes ACC1, ACC2, and ACC3.
User response
Revise the accelerator key attributes ACC1, ACC2, and
ACC3 to remove duplicate key specifications.
ISPC646 ISPC646W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" is not a valid
accelerator key combination.
The accelerator key description
"cccccccc" will be discarded.
Explanation
Combinations of keys such as SHIFT and A-Z or 0-9
are not valid.
User response
Revise the accelerator key selection to use only valid
key combinations.
ISPC647 ISPC647W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" attribute has been
specified without the previous
"dddddddd" attribute(s). The
accelerator key description will be
created using the available "ACCn"
attributes in numerical order.
Explanation
The ACC3 attribute was present without ACC1 or
ACC2, or the ACC2 attribute was present without
ACC1. The accelerator will be created using the
available attributes in numerical order.
User response
Specify accelerator key selection in numerical order.
ISPC648 ISPC648W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"cccccccc" is a duplicate of a
previously specified accelerator
key combination. The accelerator
key description "cccccccc" will be
discarded.
Explanation
The accelerator key combination cannot be duplicated
within the panel.
User response
Revise the accelerator key specifications to remove
the duplicate accelerator key combinations.
ISPC700 ISPC700E: Error. Line aaaaaaaa of
file "bbbbbbbb". Displayable lines
have been exceeded. Panel will
not be saved.
ISPF messages starting with ISP
54  z/OS: z/OS ISPF Messages and Codes

## Page 75

Explanation
More lines have been formatted for the panel body
than will fit within the specified panel depth.
User response
• Increase the specified panel depth, if possible.
• Remove some of the fields or text included in this
panel.
ISPC701 ISPC701W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Panel width has been exceeded.
cccccccc field data will be
truncated.
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC702 ISPC702W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag was specified without
the NAME or DATAVAR keyword
and will cause the absence of the
field on the panel.
Explanation
The field could not be formatted because no field
identifier was provided.
User response
Add the DATAVAR attribute to the DTL source file.
ISPC703A ISPC703AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Entry width "cccccccc" will not fit
within the Panel width and will be
changed to "dddddddd".
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC703B ISPC703BW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Prompt width "cccccccc" will not
fit within the Panel width and will
be changed to "dddddddd".
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC703C ISPC703CW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Choice width "cccccccc" will not fit
within the Panel width and will be
changed to "dddddddd".
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC703D ISPC703DW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Description width "cccccccc" will
not fit within the Panel width and
will be changed to "dddddddd".
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC704A ISPC704AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag entry width value is
"0" and may affect the display of
the entry field.
Explanation
There is insufficient space for the entry field.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  55

## Page 76

User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC704B ISPC704BW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag prompt width value is
"0" and may affect the display of
the prompt field.
Explanation
There is insufficient space for the field prompt.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC704C ISPC704CW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag description width
value is "0" and may affect the
display of the description field.
Explanation
There is insufficient space for the field description.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC705A ISPC705AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"SELWIDTH" value of the SELFLD
tag is not large enough to contain
the entry field.
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC705B ISPC705BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"SELWIDTH" value of the SELFLD
tag is not large enough to contain
the choice field.
Explanation
This message is self-explanatory.
User response
• Increase the panel width, if possible.
• Review the generated ISPF panel source result and
adjust field widths or spacing as necessary.
ISPC706 ISPC706W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Both
ENTWIDTH and COLWIDTH were
specified on the LSTCOL tag. The
value of "cccccccc" will be used as
the column width.
Explanation
This message is self-explanatory.
User response
Change the ENTWIDTH attribute to COLWIDTH.
ISPC707 ISPC707W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Conflict between "REQUIRED=NO"
with a "MSG" specified.
Explanation
The MSG attribute is valid only when REQUIRED=YES.
User response
Remove the MSG attribute from the DTL source.
ISPC708 ISPC708W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Conflict between "REQUIRED"
keyword value or "MSG" keyword
value specified for an output field.
These keywords should not be
used with USAGE=OUT.
Explanation
The REQUIRED and MSG attributes are not valid for an
output field.
User response
Remove the REQUIRED and/or MSG attributes from
the DTL source.
ISPF messages starting with ISP
56  z/OS: z/OS ISPF Messages and Codes

## Page 77

ISPC710 ISPC710W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag attribute "dddddddd"
default is set to a width of
"eeeeeeee" and may affect the
display of fields.
Explanation
The conversion utility has changed the indicated width.
User response
The conversion utility has set a width in one of these
contexts:
• A DTAFLD defined without an ENTWIDTH attribute,
either on the DTAFLD or DTACOL tag, that has a
length specified on an associated VARCLASS tag.
• A DTAFLD tag for which no entry width can be
determined is set to zero.
• A DTAFLD tag defined without a PMTWIDTH attribute
is set to the length of the prompt text, or to zero if no
space is available to format the prompt.
• A DTAFLD tag defined without a DESWIDTH attribute
is set to the length of the description text on the
associated DTAFLD tag, or to zero if no space is
available to format the description.
• A DTAFLD or LSTCOL tag that has an associated
VARCLASS specified as one of the VEDIT keywords
is forced to the defined display length.
• A SELFLD defined without an SELWIDTH attribute,
either on the SELFLD or DTACOL tag, is set to the
remaining available width.
• A SELFLD defined with an SELWIDTH value larger
than the remaining available width is set to the
remaining available width.
• A SELFLD tag defined without a PMTWIDTH attribute
is set to the length of the prompt text, or to zero if no
space is available to format the prompt.
• A VARCLASS tag with a TYPE of VMASK specified
with a length greater than 20 is reset to 20.
ISPC711 ISPC711W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc tag "dddddddd" attribute
value is less than the ISPF defined
minimum allowable width of 20
characters for a scrollable area.
The scrollable area will not be
formatted.
Explanation
The minimum width for a scrollable area is 20.
User response
Set the width of the scrollable area to 20 or more.
ISPC712 ISPC712E: Error. Line aaaaaaaa
of file "bbbbbbbb". Keyword
"NAME" must be specified with
"TYPE=SINGLE".
Explanation
The NAME attribute is required on a SELFLD tag with
"TYPE=SINGLE".
User response
Add the NAME attribute to the SELFLD tag.
ISPC713 ISPC713W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc attribute will be
ignored in context of SELFLD
TYPE=dddddddd..
Explanation
This message is self explanatory.
User response
Remove the indicated attribute from the SELFLD tag.
ISPC714 ISPC714E: Error. Line aaaaaaaa
of file "bbbbbbbb". NAME attribute
must be specified on CHOICE tag if
SELFLD TYPE=MULTI.
Explanation
This message is self-explanatory.
User response
Review the CHOICE tags defined within the SELFLD
and make sure each has a NAME attribute specified.
ISPC715A ISPC715AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate "CHECKVAR" name
"cccccccc" found on a CHOICE
tag. The "cccccccc" variable name
cannot be repeated for more
than one CHOICE if SELFLD
TYPE=MULTI. This name will
be accepted but results are
unpredictable.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  57

## Page 78

User response
For a multi-choice selection list, each CHECKVAR
name should be unique.
ISPC715B ISPC715BW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Duplicate "MATCH" value
"cccccccc" used on a CHOICE
tag. The "cccccccc" value cannot
be repeated for more than one
CHOICE if SELFLD TYPE=SINGLE.
This value will be accepted but
results are unpredictable.
Explanation
This message is self-explanatory.
ISPC716 ISPC716W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Inconsistent CHECKVAR value
found on CHOICE tag. The
CHECKVAR variable name must be
the same for all CHOICE tags if
SELFLD TYPE=SINGLE. This name
will be accepted but results are
unpredictable.
Explanation
This message is self-explanatory.
User response
For a single choice selection list, each CHECKVAR
name should be unique.
ISPC717 ISPC717W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Inconsistent use of MENU keyword
on the PANEL tag. No selection
choice list was created on panel
"cccccccc".
Explanation
A PANEL specified with MENU keyword should include
a SELFLD tag specified as TYPE=MENU.
User response
Remove the MENU attribute from the PANEL tag, or
add an option menu selection list with the SELFLD tag.
ISPC718 ISPC718E: Error. Line aaaaaaaa
of file "bbbbbbbb". More than
one "cccccccc" tag found within
"dddddddd". Only 1 "cccccccc"
tag can be used within each
"dddddddd" tag.
Explanation
The "cccccccc" tag can only be specified one time
within the "dddddddd" tag.
User response
Update the DTL source to remove the multiple use of
the "cccccccc" tag.
ISPC719A ISPC719AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Inconsistent use of Mnemonics
within "cccccccc" entries. CUA
requires that all "cccccccc" entries
must have a Mnemonic or none
can.
Explanation
This message is self-explanatory.
User response
ISPF supports the Mnemonic tag only within the ABC
and PDC tags.
If this message is the result of providing a Mnemonic
tag within a CHOICE tag, consider removing the tag
from the DTL source file.
If this message relates to ABC or PDC tag processing,
consider adding a Mnemonic tag to the ABC or PDC tag
identified within the message.
ISPC719B ISPC719BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
duplicate mnemonic value has
been found during processing
of a "cccccccc" tag. Only one
mnemonic may be specified
for each "cccccccc". Mnemonic
characters used within a set of
tags (ABC, PDC or CHOICE) must
be unique.
Explanation
This message is self-explanatory.
User response
Remove the duplicate use of the M (Mnemonic) tag
coded on the "cccccccc" tag.
ISPC719C ISPC719CW: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
ISPF messages starting with ISP
58  z/OS: z/OS ISPF Messages and Codes

## Page 79

DBCS mnemonic value has been
found within a "cccccccc" tag.
Mnemonic characters must be
single byte format. The specified
mnemonic character will not be
used.
Explanation
This message is self-explanatory.
User response
Specify a single byte character for the mnemonic.
ISPC719D ISPC719DW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
mnemonic "cccccccc" specified for
the "dddddddd" tag description is
not valid and will not be used. The
mnemonic character must be in
the range "A-Z", "a-z", or "0-9".
Explanation
The mnemonic character must be in the range A-Z,
a-z, or 0-9.
User response
Change the M tag to specify a valid character.
ISPC719E ISPC719EW: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
unique mnemonic could not be
generated for the "cccccccc" tag
description "dddddddd". Use the
Mnemonic tag to provide a unique
mnemonic character.
Explanation
This message is self explanatory.
ISPC719F ISPC719FW: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
duplicate mnemonic value has
been found during processing
of a "cccccccc" tag. The
conversion utility will attempt to
automatically select a mnemonic
character to replace the specified
value. Review the generated panel
and if the resulting mnemonic
selection is unsatisfactory, add
Mnemonic tags to remove the
duplicate condition.
Explanation
This message is self-explanatory.
User response
Add additional Mnemonic tag(s) as required to prevent
the automatic mnemonic character selection from
generating a mnemonic which conflicts with your
choice for the current item.
ISPC720 ISPC720W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
tag language has been changed
for the cccccccc tag. The attribute
"dddddddd" has been replaced by
"eeeeeeee". The conversion utility
will use the value of "dddddddd"
as "eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change the DTL source value dddddddd to eeeeeeee.
ISPC721 ISPC721W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The tag language has been
changed for the cccccccc tag.
The attribute value "dddddddd"
has been replaced by "eeeeeeee".
The conversion utility will accept
the entry "ffffffff=dddddddd".
However, you should change your
source to use the new attribute
value ("ffffffff=eeeeeeee").
Explanation
This message is self-explanatory.
User response
Change the DTL source value "dddddddd" to
"eeeeeeee".
ISPC722 ISPC722W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The tag language has been
changed for the cccccccc tag. The
attribute value "dddddddd" has
been removed from the cccccccc
tag and will be ignored in this
conversion. You should update
your source file to remove the
"eeeeeeee=dddddddd" entry.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  59

## Page 80

Explanation
This message is self-explanatory.
User response
Change the DTL source to remove the entry
"eeeeeeee=dddddddd".
ISPC723 ISPC723W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
tag language has been changed
for the cccccccc tag. The attribute
"dddddddd" has been removed
from the cccccccc tag and will
be ignored in this conversion.
You should remove the dddddddd
reference from your tag source.
Explanation
This message is self-explanatory.
User response
Change the DTL source to remove the entry
"dddddddd".
ISPC729 ISPC729W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" attribute has been
specified on a "dddddddd" tag but
the attribute "eeeeeeee" was not
specified (or is not valid). The
"cccccccc" attribute will be reset
to blank.
Explanation
The use of cccccccc depends on the specification of
eeeeeeee.
User response
Either add the eeeeeeee attribute to the DTL source
or correct other related coding to make the eeeeeeee
attribute valid.
ISPC730 ISPC730W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
required attribute "DATAVAR" is
missing from the DTAFLD tag. The
"NAME" value will become the
panel field name. You should add
the "DATAVAR" attribute to your
tag source. (It is recommended
that you code both attributes with
the same value.)
Explanation
This message is self-explanatory.
User response
Add the DATAVAR attribute using the same name value
specified for the NAME attribute.
ISPC731 ISPC731W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Both "DATAVAR" and "NAME" are
specified on the DTAFLD tag, but
they have different values. The
"DATAVAR" value will become
the panel field name. It is
recommended that you code both
attributes with the same value.
Explanation
This message is self-explanatory.
User response
Change the NAME attribute to the same value as the
DATAVAR attribute.
ISPC732 ISPC732W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"AUTOTAB=YES" attribute is in
conflict with "USAGE" on the
cccccccc tag. AUTOTAB will be
changed to "NO".
Explanation
This message is self-explanatory.
User response
Change AUTOTAB=YES to AUTOTAB=NO, or remove
the AUTOTAB attribute from the DTL source.
ISPC733 ISPC733W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" attribute is not valid
without a valid "DEPTH" value.
The "cccccccc" attribute will be
ignored.
Explanation
This message is self-explanatory.
User response
Remove the cccccccc attribute from the DTL source.
ISPF messages starting with ISP
60  z/OS: z/OS ISPF Messages and Codes

## Page 81

ISPC734 ISPC734W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"EXTEND" attribute has already
been specified for this panel
or "EXTEND" is not valid in
this tag nesting configuration.
The "EXTEND" attribute will be
ignored.
Explanation
This message is self-explanatory.
User response
Remove the EXTEND attribute from the DTL source.
ISPC735 ISPC735W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"SCROLL" attribute has already
been specified for this panel either
on an enclosing AREA tag or on
another DA tag. The "SCROLL"
attribute for this DA tag will be
ignored.
Explanation
This message is self-explanatory.
User response
Remove the SCROLL attribute from the DTL source.
ISPC736 ISPC736E: Error. Line aaaaaaaa
of file "bbbbbbbb". The "cccccccc"
attribute has been specified for
this panel and is in conflict with
table display. The panel will not be
saved.
Explanation
This message is self-explanatory.
User response
Remove the cccccccc attribute from the DTL source.
ISPC737 ISPC737W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Both
"PAD" and "PADC" attributes have
been specified on the "cccccccc"
tag. The value of "PADC" will be
used.
Explanation
This message is self-explanatory.
User response
Remove either the PAD or PADC attribute from the DTL
source.
ISPC738A ISPC738AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "EXTEND=ON" attribute
specification is not valid on a
"cccccccc" tag within a horizontal
region. The "EXTEND" attribute
will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the EXTEND attribute from the DTL source.
ISPC738B ISPC738BW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "EXTEND=ON" attribute
specification is not valid on
a "cccccccc" tag formatted
horizontally. The "EXTEND"
attribute will be ignored.
Explanation
This message is self-explanatory.
User response
Remove the EXTEND attribute from the DTL source.
ISPC738C ISPC738CW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "EXTEND=FORCE" attribute
specification is coded on a
"cccccccc" tag within a horizontal
region. The resulting panel will
not display unless the extendable
portion of the panel is below all
other fields or text on the panel.
Explanation
This message is self-explanatory.
User response
Verify the panel by displaying it using ISPF Dialog Test.
ISPC738D ISPC738DW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "EXTEND=FORCE" attribute
specification is coded on
a "cccccccc" tag formatted
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  61

## Page 82

horizontally. The resulting panel
will not display unless the
extendable portion of the panel is
below all other fields or text on the
panel.
Explanation
This message is self-explanatory.
User response
Verify the panel by displaying it using ISPF Dialog Test.
ISPC739 ISPC739W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" attribute has been
specified on a previous tag.
The "cccccccc" attribute value
"dddddddd" will be reset to blank.
Explanation
This message is self-explanatory.
User response
Remove the cccccccc attribute from the DTL source or
change the dddddddd value to remove the conflict.
ISPC740 ISPC740W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"cccccccc" tag "PMTWIDTH" value
is too small to allow for leader
dots.
Explanation
This message is self-explanatory.
User response
Increase the value for prompt width so that a
minimum of two leader dots can be formatted
following the prompt text.
ISPC741 ISPC741W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
A DIVIDER within a horizontal
region with GUTTER=1 must
specify TYPE=NONE. TYPE is reset
to NONE.
Explanation
A divider which specifies TYPE=SOLID requires a
minimum of two spaces to provide for the divider
attribute byte and the divider character.
User response
Either increase the GUTTER value to 2 or change the
TYPE value to NONE.
ISPC742 ISPC742W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The NOENDATTR attribute on the
DIVIDER tag is valid only when
the GUTTER value is 2 or more.
NOENDATTR is ignored.
Explanation
When NOENDATTR is specified, the ending attribute
for the divider is not included. The starting attribute for
a divider is always used. A divider must be more than
one character in width to use NOENDATTR.
User response
Either increase the GUTTER value or remove the
NOENDATTR attribute from the DIVIDER tag.
ISPC743 ISPC743W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
DEPTH="cccccccc" will exceed the
remaining panel depth due to
prompt text formatting. DEPTH
will be reset to "dddddddd".
Explanation
The prompt text provided has been formatted on more
than one line causing the remaining available panel
depth to be reduced.
User response
Either increase the prompt width to format the prompt
on one line or change the panel depth to compensate
for the multiple line prompt.
ISPC744 ISPC744W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
There is insufficient remaining
depth to format a scrollable area.
Explanation
A scrollable area requires a minimum of two panel
body lines.
User response
If possible, increase the panel depth.
ISPC745 ISPC745W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
"CHOICECOLS=cccccccc" value of
ISPF messages starting with ISP
62  z/OS: z/OS ISPF Messages and Codes

## Page 83

the SELFLD tag does not provide
enough columns to sub-divide all
of the CHOICE tags. The remaining
CHOICE tags will be placed in
the last column available for the
current SELFLD tag.
Explanation
This message is self-explanatory.
User response
Increase the specified number of choice columns.
ISPC746 ISPC746E: Error. Line aaaaaaaa
of file "bbbbbbbb". The
"CHOICECOLS=cccccccc" value of
the SELFLD tag specified more
scrollable columns than the
"CHOICEDEPTH=dddddddd" value
could create from the number
of CHOICE tags processed. This
condition results in a mismatch
between defined scrollable areas
in the panel )BODY section
and )AREA sections generated.
The panel will not be saved.
Explanation
This message is self-explanatory.
User response
The source file can to changed to either:
• Reduce the choice depth so that the available
choices are distributed to all of the specified choice
columns.
• Reduce the choice columns so that at least one
choice is formatted for each choice column.
ISPC747 ISPC747W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "CHOICECOLS=cccccccc"
value of the SELFLD tag
specified more columns than the
"CHOICEDEPTH=dddddddd" value
could create from the number of
CHOICE tags processed.
Explanation
This message is self-explanatory.
User response
The source file can to changed to either:
• Reduce the choice depth so that the available
choices are distributed to all of the specified choice
columns.
• Reduce the choice columns so that at least one
choice is formatted for each choice column.
ISPC748 ISPC748W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
panel line "cccccccc" contains
EXPAND characters which cannot
operate due to horizontal region
formatting.
Explanation
The expand processing of display cannot operate
when attribute bytes are found on a line following the
expand characters. Panel lines formatted as part of
horizontal regions contain attribute bytes for limiting
the region width and for alignment of horizontal data.
User response
Remove the EXPAND characters from line cccccccc or
revise the panel to use only vertical region formatting.
ISPC749 ISPC749W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The NOENDATTR attribute on the
DIVIDER tag is valid only when
the DIVIDER tag is placed within
a horizontal region. NOENDATTR is
ignored.
Explanation
The NOENDATTR attribute only applies to DIVIDER
tag formatting within a horizontal region. NOENDATTR
does not apply in vertical regions or when DIVIDER is
used outside of the REGION tag.
User response
Remove the NOENDATTR attribute from this use of the
DIVIDER tag.
ISPC749A ISPC749AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The "cccccccc" attribute on the
DIVIDER tag is valid only when
the DIVIDER tag is placed within
a vertical region. "cccccccc" is
ignored.
Explanation
The "cccccccc" attribute only applies to DIVIDER tag
formatting within a vertical region.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  63

## Page 84

User response
Remove the NOENDATTR attribute from this use of the
DIVIDER tag.
ISPC750 ISPC750W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The special ENTITY "cccccccc"
requires dddddddd bytes of space.
The DTL source file has specified
"PMTWIDTH" as eeeeeeee bytes.
Explanation
This message is self-explanatory.
User response
Increase the value for prompt width to dddddddd to
provide adequate prompt formatting space.
ISPC751 ISPC751W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The ENTITY name "cccccccc"
maximum length is dddddddd
byte(s). "cccccccc" will be
truncated to "eeeeeeee".
Explanation
ENTITY names must follow these rules for name
length:
• Parameter entities: 1-7.
• System entities: 1-8.
• Other entities: 1-17.
User response
Correct the ENTITY name to follow the listed rules.
ISPC752 ISPC752W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ENTITY name "cccccccc" minimum
length is dddddddd character(s).
cccccccc will be set to blank.
Explanation
ENTITY names must follow these rules for name
length:
• Parameter entities: 1-7.
• System entities: 1-8.
• Other entities: 1-17.
User response
Correct the ENTITY name to follow the listed rules.
ISPC753 ISPC753W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ENTITY name "cccccccc" contains
character(s) which are not valid.
cccccccc will be set to blank.
Parameter and System entity
names must be from 1 to 8
bytes. Other entity names can be
from 1 to 17 bytes. One or more
underscore ("_") bytes must be
included in names longer than 8
bytes.
Explanation
ENTITY names must follow these rules:
• Length.
– Parameter entities: 1-8.
– System entities: 1-8.
– Other entities: 1-17.
• The first character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, can be A-Z, a-z, 0-9, @,
#, or $.
• When an 'other' entity name is longer than 8 bytes,
one or more of the remaining characters must be an
underscore ("_").
• Entity-names are case-sensitive.
User response
Correct the ENTITY name to follow the listed rules.
ISPC760 ISPC760W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Unable to create a new cursor
group for the specification of
"CSRGRP=YES". CSRGRP is reset
to "NO".
Explanation
The total number of cursor groups is limited to 99.
Each specification of CSRGRP=YES causes ISPDTLC to
create a new cursor group. All of the available cursor
group numbers from 1 to 99 have been used and no
additional group numbers are available.
User response
Reduce the number of occurrences of CSRGRP=YES or
combine fields currently specified as separate cursor
groups.
ISPC761 ISPC761W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
ISPF messages starting with ISP
64  z/OS: z/OS ISPF Messages and Codes

## Page 85

")LIST" section entries referenced
by "LISTREF=cccccccc" are not
found. The LISTREF attribute is
ignored. Panel formatting may be
affected.
Explanation
The ")LIST" section name specified by the LISTREF
attribute must be created by a previous SELFLD tag
which includes CHOICE tags to define the ")LIST"
section entries.
User response
Correct the LISTREF specification to refer to a
previously defined SELFLD tag group, or include
CHOICE tags within the current SELFLD tag to define
a new ")LIST" section.
ISPC762 ISPC762W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
")LIST" section name specified by
"LISTREF=cccccccc" is a duplicate
of a previous name. "cccccccc"
is changed to "dddddddd". Panel
formatting may be affected.
Explanation
The ")LIST" section name specified by the LISTREF
attribute on a SELFLD tag which includes CHOICE tags
is a duplicate of a previously defined ")LIST" section
name.
User response
Correct the LISTREF specification to specify a unique
name, or remove the LISTREF attribute from the
current SELFLD tag.
ISPC763 ISPC763W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
CHOICE text "cccccccc" is too long
to display as one line. ISPF does
not support multiple lines of panel
text for "LISTTYPE=dddddddd".
The LISTTYPE attribute is ignored.
This SELFLD tag group will be
formatted as a numbered single-
choice list.
Explanation
Multiple lines of text for a CHOICE tag are not
supported. The LISTTYPE attribute on the associated
SELFLD tag is ignored. A numbered single-choice
selection list is created by default.
User response
Reduce the CHOICE text length to fit within the
available SELFLD width, or increase the SELFLD width
to allow for longer CHOICE text.
ISPC764 ISPC764W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The CHOICE text formatting
width of "cccccccc" exceeds
the ISPF maximum for
"LISTTYPE=dddddddd". The
formatting width is reset to "99".
Explanation
The CHOICE text formatting width is calculated based
on the SELWIDTH value, allowing for the entry width
and required panel attribute bytes.
User response
No action is necessary. To eliminate this message,
reduce the SELWIDTH attribute value on the enclosing
SELFLD tag.
ISPC767A ISPC767AW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
A CHOICE tag that specified
the UNAVAIL attribute has also
specified the RP (Reference
Phrase) tag as part of the
CHOICE tag text. ISPF does not
support Reference Phrases on
unavailable choices. The UNAVAIL
(and UNAVAILMAT) attribute
specifications are ignored.
Explanation
ISPF does not support Reference Phrases on
unavailable choices. The conversion utility will format
the Reference Phrase and will ignore the UNAVAIL
(and UNAVAILMAT) attribute specifications.
User response
If unavailable choice support is required for this
CHOICE, remove the RP (Reference Phrase) tag
specification from the choice text and reconvert the
panel. If unavailable choice support is not required,
remove the UNAVAIL (and UNAVAILMAT) attributes
from this CHOICE tag.
ISPC767B ISPC767BW: Warning. Line
aaaaaaaa of file "bbbbbbbb".
A CHOICE tag that specified
the UNAVAIL attribute has also
specified the HP (Emphasized
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  65

## Page 86

Text) tag as part of the
CHOICE tag text. ISPF does
not support Emphasized Text on
unavailable choices. The UNAVAIL
(and UNAVAILMAT) attribute
specifications are ignored.
Explanation
ISPF does not support Emphasized Text on unavailable
choices. The conversion utility will format the
Emphasized Text and will ignore the UNAVAIL (and
UNAVAILMAT) attribute specifications.
User response
If unavailable choice support is required for this
CHOICE, remove the HP (Emphasized Text) tag
specification from the choice text and reconvert the
panel. If unavailable choice support is not required,
remove the UNAVAIL (and UNAVAILMAT) attributes
from this CHOICE tag.
ISPC768 ISPC768W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
CHOICE tag that is used to build
a COMBO list cannot have user
defined "cccccccc" tag text. The
"cccccccc" tag is ignored.
Explanation
The conversion utility automatically generates Point-
and-Shoot entries for CHOICEs that are part of a
COMBO list.
User response
Remove any Emphasized Text (HP), Reference Phrase
(RP), or Point-and-Shoot (PS) tag definitions within the
CHOICE tag.
ISPC769 ISPC769W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
number of SELFLD tags containing
unavailable choices has exceeded
the maximum of 36. Remaining
SELFLD tags will use the "ZS#"
prefix for variable names used for
unavailable choice text fields. If
duplicate field names result, the
panel cannot be displayed.
Explanation
The number of SELFLD tags containing unavailable
choices has exceeded the maximum that can be
successfully created by the conversion utility.
User response
Revise the panel to limit the number of SELFLD tags
with unavailable choices to 36.
ISPC770 ISPC770W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag text "dddddddd" is
not valid when the TYPE=NONE
attribute is either specified or
defaulted. The text is ignored.
Explanation
Text on the cccccccc tag is in conflict with the
TYPE=NONE attribute which is defined as creating a
blank divider line.
User response
Remove the text from the cccccccc tag, or change the
TYPE attribute.
ISPC771 ISPC771W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag text "dddddddd" is
not valid unless the FORMAT
attribute is specified. The text is
ignored.
Explanation
Text on the cccccccc tag is not accepted unless the
FORMAT attribute is also specified.
User response
Remove the text from the cccccccc tag, or add the
FORMAT attribute.
ISPC772 ISPC772W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag text "dddddddd" is
not valid within a horizontal
region. The text is ignored.
Explanation
Text on the cccccccc tag is not accepted within a
horizontal region.
User response
Remove the text from the cccccccc tag.
ISPC773 ISPC773W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The CHDIV tag is not supported
for selection lists that specify
"cccccccc". The tag is syntax
ISPF messages starting with ISP
66  z/OS: z/OS ISPF Messages and Codes

## Page 87

checked but will be ignored during
panel formatting.
Explanation
The LISTTYPE=COMBO specification creates a single
input field in the panel )BODY section. Divider lines
can only be used within choice lists formatted in
the panel )BODY section. The CHOICEDIR=HORIZ
specification formats the choice numbers in sequence
from left to right, top-to-bottom. Divider lines can only
be used within choice lists formatted vertically (top-to-
bottom, left-to-right).
User response
Remove the CHDIV tag from the selection list.
ISPC774 ISPC774W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute "dddddddd"
is not valid when the cccccccc
tag is used with the eeeeeeee
tag. "dddddddd=ffffffff" will not be
used.
Explanation
The dddddddd attribute of the cccccccc tag is
restricted. The dddddddd attribute is not valid with the
eeeeeeee tag.
User response
Remove the dddddddd attribute from the cccccccc tag
definition.
ISPC775A ISPC775AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute "dddddddd"
is not valid when the cccccccc tag
is used within a horizontal region.
"dddddddd" will not be used.
Explanation
The dddddddd attribute of the cccccccc tag is
restricted. The dddddddd attribute is not valid when
the cccccccc tag is used within a horizontal region.
User response
Remove the dddddddd attribute from the cccccccc tag
definition.
ISPC775B ISPC775BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute "dddddddd"
is not valid when the cccccccc tag
is used within a scrollable area.
"dddddddd" will not be used.
Explanation
The dddddddd attribute of the cccccccc tag is
restricted. The dddddddd attribute is not valid when
the cccccccc tag is used within a scrollable area.
User response
Remove the dddddddd attribute from the cccccccc tag
definition.
ISPC776 ISPC776W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The definition for
"IMAPNAME=cccccccc" specifying
a blank text string is not valid.
Image "cccccccc" will not be used.
Explanation
Each image definition must map to a unique text
string.
User response
Remove the image definition attributes from the tag
definition, or change the PS tag to enclose a non-blank
text string.
ISPC777 ISPC777W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The definition for
"IMAPNAME=cccccccc" specifying
the text "dddddddd" is a duplicate
of a previously defined image text
string. Image "cccccccc" will not
be used.
Explanation
Each image definition must map to a unique text
string.
User response
Remove the image definition attributes from the tag
definition, or change the text enclosed by the PS tag.
ISPC778 ISPC778W: Warning. There is a
conflict in the use of "aaaaaaaa"
as a panel data set with
"bbbbbbbb". "bbbbbbbb" is being
reset to "cccccccc".
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  67

## Page 88

Explanation
The converted panel can be displayed only after it has
been written to the output panel file.
User response
Either provide a valid panel file name in place of
"aaaaaaaa" or remove the selection for the "Display
converted panels" or "Display converted panels in a
window" option. conflict.
ISPC779 ISPC779W: Warning. The
invocation option NODSNCHK has
been changed to DSNCHK because
an EXECIO error occurred.
Explanation
DSNCHK will cause a full validation of all input and
output file names to assist in correcting the EXECIO
error.
User response
Use the NODSNCHK option again after the EXECIO
error is resolved.
ISPC780 ISPC780W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute dddddddd is
not valid unless the cccccccc tag
is nested within a eeeeeeee tag.
dddddddd=ffffffff will not be used.
Explanation
This message is self-explanatory.
User response
Remove the dddddddd attribute.
ISPC781 ISPC781W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc tag attribute dddddddd
is not valid when the eeeeeeee
attribute is also specified.
dddddddd=ffffffff will not be used.
Explanation
This message is self-explanatory.
User response
Remove either the dddddddd attribute or the eeeeeeee
attribute.
ISPC782 ISPC782W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The column is not wide enough
to display both the fields
associated with the cccccccc
tag attributes dddddddd and
eeeeeeee. dddddddd=ffffffff and
eeeeeeee=gggggggg will not be
used.
Explanation
This message is self-explanatory.
User response
Either remove the dddddddd and eeeeeeee attributes
or make the column wider.
ISPC783 ISPC783W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The column is not wide enough
to display the field associated
with the cccccccc tag attribute
dddddddd. dddddddd=eeeeeeee
will not be used.
Explanation
This message is self-explanatory.
User response
Either remove the dddddddd attribute or make the
column wider.
ISPC784 ISPC784W: Warning. Line
aaaaaaaa of file "bbbbbbbb". Only
a single SCRFLD tag can be nested
in a cccccccc tag. This SCRFLD tag
definition will not be used.
Explanation
This message is self-explanatory.
User response
Remove the extra SCRFLD tag definition.
ISPC785 ISPC785W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The SCRFLD tag attribute
cccccccc cannot be specified
for a field defined immediately
within a horizontal region.
cccccccc=dddddddd will not be
used.
ISPF messages starting with ISP
68  z/OS: z/OS ISPF Messages and Codes

## Page 89

Explanation
This message is self-explanatory.
User response
Either remove the cccccccc attribute or define the field
immediately within a vertical region.
ISPC786 ISPC786W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The variable cccccccc specified
with the SCRFLD tag attribute
LCOLIND will not be displayed
for the dddddddd field because
LCOLIND=cccccccc has already
been specified on an SCRFLD
tag nested in another LSTCOL or
DTAFLD tag.
Explanation
This message is self-explanatory.
User response
Either change the variable name on the LCOLIND
attribute or remove LCOLIND=cccccccc from the
SCRFLD tag.
ISPC787 ISPC787W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Region width is too small to
include indicator "cccccccc" on the
same line as the DTAFLD field.
FLDSPOS is set to BELOW.
Explanation
This message is self-explanatory.
User response
Increase the width of the region or set FLDSPOS to
BELOW on the SCRFLD tag.
ISPC801 ISPC801E: Error. Line aaaaaaaa
of file "bbbbbbbb". cccccccc and
dddddddd tags were not matched,
or the number of cccccccc tags
does not match the number of
entries specified by the TSIZE
attribute.
Explanation
This message is issued during processing of a DL
(definition list) or PARML (parameter list) tag set when
the nested term and description tags are not matched.
When TSIZE is specified as "TSIZE='s1 s2..sn'", this
message is also issued if the number of nested term
tags does not match the number of entries specified
by TSIZE.
User response
Make sure that each cccccccc tag has a matching
dddddddd tag. When TSIZE is specified as "TSIZE='s1
s2..sn'", make sure that a term tag is coded for each
entry defined by TSIZE.
ISPC801A ISPC801AE: Error. Line aaaaaaaa
of file "bbbbbbbb". The number
of cccccccc tags does not match
the number of entries specified by
the TSIZE attribute. Check for a
missing "cccccccc" tag.
Explanation
This message is issued during processing of a DL
(definition list) or PARML (parameter list) tag set when
the nested term and description tags are not matched.
When TSIZE is specified as "TSIZE='s1 s2..sn'", this
message is issued if the number of nested term tags
does not match the number of entries specified by
TSIZE.
User response
When TSIZE is specified as "TSIZE='s1 s2..sn'", make
sure that a term tag is coded for each entry defined by
TSIZE.
ISPC801B ISPC801BE: Error. Line aaaaaaaa
of file "bbbbbbbb". The number of
cccccccc tags does not match the
number of entries specified by the
TSIZE attribute. Check for an extra
"cccccccc" tag.
Explanation
This message is issued during processing of a DL
(definition list) or PARML (parameter list) tag set when
the nested term and description tags are not matched.
When TSIZE is specified as "TSIZE='s1 s2..sn'", this
message is issued if the number of nested term tags
does not match the number of entries specified by
TSIZE.
User response
When TSIZE is specified as "TSIZE='s1 s2..sn'", make
sure that a term tag is coded for each entry defined by
TSIZE.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  69

## Page 90

ISPC801C ISPC801CE: Error. Line aaaaaaaa
of file "bbbbbbbb". A description
tag is missing in a multiple
term group. Check for a missing
"cccccccc" tag.
Explanation
This message is issued during processing of a DL
(definition list) or PARML (parameter list) tag set when
the nested term and description tags are not matched.
When TSIZE is specified as "TSIZE='s1 s2..sn'", this
message is issued if cccccccc (description) tag is
missing.
User response
Make sure that each term group has a matching
cccccccc tag.
ISPC802 ISPC802E: Error. Line aaaaaaaa of
file "bbbbbbbb". Processing for tag
cccccccc has generated attribute
"dddddddd" which is outside of
the range of attribute values
available to ISPDTLC. Panel will
not be saved.
Explanation
Scrollable areas, dynamic areas, graphic areas, and
ISPDTLC generated expand characters use characters
in the range of X'00'through X'2f'. The panel in process
requires more characters than are available.
User response
Reduce the number of special formatting areas on the
panel.
ISPC803 ISPC803E: Error. The list nesting
level exceeds the maximum
allowed (aaaaaaaa) for the
bbbbbbbb tag.
Explanation
ISPDTLC allows nesting of UL, SL, and OL to a
maximum of 10 levels. The current tag source exceeds
the maximum nesting allowed.
User response
Reduce the nested tags to a maximum of 10 levels.
ISPC804 ISPC804E: Error. Line aaaaaaaa
of file "bbbbbbbb". The maximum
number (cccccccc) of panel
attributes has been exceeded.
Panel will not be saved.
Explanation
ISPF allows a maximum of cccccccc attributes on this
panel.
User response
Reduce the number of requested attributes to a
maximum of cccccccc.
The number of requested attributes includes attribute
override entries. These are .ATTR entries that are
added by the Dialog Tag Language compiler for
attributes that are specified on CHOFLD, DTACOL,
DTAFLD, LSTCOL, and LSTFLD tags. If the same
set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Add the
parameter ATTRCHANGE=YES to the tags to cause
the compiler to instead add a single entry in the
panel )ATTR section for each unique set of attributes
specified. The entry for a set of attributes is then
shared by all tags that specify that set of attributes.
ISPC805 ISPC805W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Multiple LSTFLD tag (and nested
tags) will be syntax checked only
and will be discarded by the
conversion utility.
Explanation
ISPDTLC allows only one LSTFLD tag group on a panel.
The first group found will be used; remaining groups
will be discarded.
User response
Provide only one LSTFLD tag group for each panel.
ISPC806 ISPC806E: Error. Line aaaaaaaa of
file "bbbbbbbb". Conflict between
attribute being generated for the
"cccccccc" tag and a previously
defined attribute. Panel will not be
saved.
Explanation
There is a duplicate of the current attribute specified
on an ATTR or DA tag, or as the EXPAND characters for
the panel.
ISPF messages starting with ISP
70  z/OS: z/OS ISPF Messages and Codes

## Page 91

User response
Review the use of the ATTRCHAR attribute (ATTR
tag), USERMOD and DATAMOD attributes (DA tag) and
panel expand characters. Choose values above X'2F'
for these items.
ISPC807 ISPC807E: Error. Line aaaaaaaa of
file "bbbbbbbb". Conflict between
the "cccccccc" attribute specified
on the "dddddddd" tag and either
a previously defined attribute or a
reserved attribute. Panel will not
be saved.
Explanation
ISPDTLC uses characters in the range of X'00'through
X'2f'as panel attribute bytes. An attribute specified in
the DTL source on the "dddddddd" tag is either:
• in conflict with an attribute required by ISPDTLC,
• or a duplicate of an attribute previously specified.
User response
1. If the message is issued during conversion of
an ISPF product panel (FLMxxxxx, ISPxxxxx, or
ISRxxxxx), specify the keylist application id as ISR
and rerun the conversion.
2. In other cases, make one of these changes and
rerun the conversion.
• Change the indicated attribute to a different
character.
• Specify ATTRUSE=YES or ATTRUSE=ALL on the
PANEL tag.
ISPC808 ISPC808E: Error. Line aaaaaaaa
of file "bbbbbbbb". A scrollable
area which contains a LSTFLD tag
grouping cannot contain any other
tags. Panel will not be saved.
Explanation
This message is self explanatory.
ISPC809 ISPC809E: Error. Line aaaaaaaa
of file "bbbbbbbb". A scrollable
area has been defined without any
lines. A null )AREA section is not
valid for ISPF. Panel will not be
saved.
Explanation
This message is self-explanatory.
User response
Review the log for other errors that may have caused
data for the scrollable area to be discarded.
ISPC810 ISPC810E: Error. The maximum
number of current files (aaaaaaaa)
has been exceeded.
Explanation
ISPDTLC keeps multiple input files in memory to
minimize file I/O for file embed processing. The limit
of aaaaaaaa files has been exceeded.
User response
Revise the structure of the DTL source files to reduce
the number of required active embed files. (This might
be achieved by limiting the number of nested embed
files.)
ISPC811 ISPC811W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The CHOFLD tag can not
be used within CHOICE text
for "LISTTYPE=cccccccc". The
LISTTYPE attribute is ignored.
This SELFLD tag group will be
formatted as a numbered single-
choice list.
Explanation
Data fields can not be used within CHOICE text
formatted within a )LIST panel section. The LISTTYPE
attribute on the associated SELFLD tag is ignored. A
numbered single-choice selection list is created by
default.
User response
Modify the DTL source in one of these ways:
• Remove the CHOFLD tag from the current CHOICE
text
• Change the LISTTYPE to RADIO
• Remove the LISTTYPE attribute.
ISPC851 ISPC851W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
help panel has been defined with
either a Width > 76 and/or a Depth
> 22 and will require a device
larger than 24x80 for display of
the panel. The width must allow
for 4 bytes and the depth must
allow 2 lines for pop-up borders on
help panels.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  71

## Page 92

Explanation
This message is self-explanatory.
ISPC852 ISPC852W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
When a panel is defined with
width = "cccccccc", the panel
depth must be less than
"dddddddd". Panel depth is reset
to "eeeeeeee".
Explanation
This message is self-explanatory.
ISPC853 ISPC853W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
When a panel is defined with
width = "cccccccc", the maximum
panel depth must be 3 lines less
than the terminal screen size to
prevent an ISPF display error.
Explanation
This message is self-explanatory.
ISPC854 ISPC854W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The AUTOTYPE=cccccccc definition
is not valid unless you have
also used AUTOTYPE to define
"dddddddd". AUTOTYPE=cccccccc
will be ignored.
Explanation
This message is self-explanatory.
User response
Add missing AUTOTYPE definitions for PROJECT,
GROUP1 and TYPE.
ISPC856 ISPC856W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
panel width "cccccccc" will not
fit within the output file record
length of "dddddddd". The WIDTH
of panels being converted with the
NOPREP option must be less than
or equal to the record length of the
output panel file.
Explanation
This message is self-explanatory.
User response
Reduce the value of the WIDTH attribute or specify an
output panel file with a larger record length.
ISPC857 ISPC857W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
panel width "cccccccc" will not
fit within the specified output file
record length of "dddddddd". The
WIDTH of panels being converted
with the NOPREP option must be
less than or equal to the specified
record length of the output panel
file.
Explanation
This message is self explanatory.
ISPC859 ISPC859W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
LSTFLD tag (and nested LSTCOL
tags) will be formatted at the
bottom of the panel for use by
ISPF table display.
Explanation
This message is self-explanatory.
User response
This message is issued when the LSTFLD tag group is
followed by other panel tags.
ISPC865 ISPC865W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
comment "cccccccc" does not
conform to DTL rules for the use
of the '-' character. This comment
is accepted for the conversion.
Please update the tag source file
to DTL comment syntax standards.
Explanation
The rules for DTL comments allow only single "-"
(dash) characters within the comment text. Multiple
contiguous "-" (dash) characters indicate comment
continuation text.
User response
Revise the comment to use only single "-" (dash)
characters within the comment text.
ISPC866 ISPC866W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
GML source "cccccccc" does not
ISPF messages starting with ISP
72  z/OS: z/OS ISPF Messages and Codes

## Page 93

conform to DTL rules for comment
continuation. This syntax is
accepted for the conversion.
Please update the tag source file
to DTL comment syntax standards.
Explanation
Each line of a continued comment should begin with
the characters "--". ISPDTLC treats all lines following
an open comment delimiter ("") is encountered.
User response
Correct the comment syntax by placing the comment
continuation characters "--" at the start of each
comment continuation line.
ISPC868 ISPC868E: Error. Line aaaaaaaa
of file "bbbbbbbb". The use of the
"cccccccc" tag is limited to the first
99 scrollable areas. Panel will not
be saved.
Explanation
The naming structure for Reference Phrases is
ZRPxxyyy, and the naming structure for point-and-
shoot entries is ZPSxxyyy, where xx is 00 for the panel
body, or 01 - 99 for the number of the scrollable area,
and yyy is the number of the entries within area xx.
The maximum number of scrollable areas that can
contain Reference Phrases or point-and-shoot entries
is 99.
User response
Remove any RP or PS tags found in the scrollable areas
that are beyond the defined limit.
ISPC869 ISPC869E: Error. Line aaaaaaaa of
file "bbbbbbbb". The maximum of
999 uses of the "cccccccc" tag for a
dddddddd panel section has been
exceeded. Panel will not be saved.
Explanation
The naming structure for Reference Phrases is
ZRPxxyyy, and the naming structure for point-and-
shoot entries is ZPSxxyyy, where xx is 00 for the panel
body, or 01 - 99 for the number of the scrollable area,
and yyy is the number of the Reference Phrase or
point-and-shoot entry within area xx. The maximum
number of entries within any area xx is 999.
User response
Remove any RP or PS tags found in the dddddddd
section that are beyond the defined limit.
ISPC890A ISPC890AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM1 attribute must be specified
to provide the mask-character for
the PICTCN verify statement.
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. DTL does not allow characters C, c, A, a, N, n, X,
x, 9, left parenthesis, right parenthesis, comma, single
quote (') or double quote (") as the mask-character.
User response
Provide the PARM1 attribute with a valid mask-
character.
ISPC890B ISPC890BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM2 attribute must be specified
to provide the field-mask for the
PICTCN verify statement.
Explanation
The PICTCN verify statement requires a field-mask to
describe the format of the field to be verified. The
field-mask is a combination of character constants and
the mask-character specified with attribute PARM1.
User response
Provide the PARM2 attribute with a valid field format.
ISPC890C ISPC890CW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM3 attribute must be specified
to provide the verification string
for the PICTCN verify statement.
Explanation
The PICTCN verify statement requires a verification
string to specify the verification for the field to be
verified. The verification string is a combination of the
character constants as specified with PARM2 and the
picture string characters C, A, N, X and 9.
User response
Provide the PARM3 attribute with a valid field format.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  73

## Page 94

ISPC891 ISPC891W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM1 attribute containing the
mask-character must be only 1
character.
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. DTL does not allow characters C, c, A, a, N, n, X,
x, 9, left parenthesis, right parenthesis, comma, single
quote (') or double quote (") as the mask-character.
User response
Provide the PARM1 attribute with a valid mask-
character.
ISPC892A ISPC892AW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM1 attribute contains the
reserved picture string character
"cccccccc". DTL does not allow
characters C, c, A, a, N, n, X, x, or
9 as the mask-character.
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. DTL does not allow characters C, c, A, a, N, n, X,
x, 9, left parenthesis, right parenthesis, comma, single
quote (') or double quote (") as the mask-character.
User response
Provide the PARM1 attribute with a valid mask-
character.
ISPC892B ISPC892BW: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM1 attribute contains the DTL
restricted character "cccccccc".
DTL does not allow characters
left parenthesis, right parenthesis,
comma, single quote (') or double
quote (") as the mask-character.
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. DTL does not allow characters C, c, A, a, N, n, X,
x, 9, left parenthesis, right parenthesis, comma, single
quote (') or double quote (") as the mask-character.
User response
Provide the PARM1 attribute with a valid mask-
character.
ISPC893 ISPC893W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
PARM1 supplied mask-character
"cccccccc" is not found as part
of the PARM2 supplied field-mask
"dddddddd".
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. The mask-character must appear within the
field-mask.
User response
Provide the PARM2 attribute with a valid field-mask.
ISPC894 ISPC894W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The field-mask position cccccccc
(character "dddddddd") cannot
be a reserved picture string
character.
Explanation
The PICTCN verify statement requires a mask-
character to represent a picture character in the field-
mask. The mask-character must appear within the
field-mask in those positions to be verified by a picture
string character. Picture string characters C, c, A, a, N,
n, X, x, and 9 are not valid within the field-mask.
User response
Provide the PARM2 attribute with a valid field-mask.
ISPC895 ISPC895W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
field-mask "cccccccc" length is not
equal to the verification string
"s4" length.
Explanation
The PICTCN verify statement requires the field-mask
to represent each character in the verification string.
The length of the field-mask must be equal to the
length of the verification string.
ISPF messages starting with ISP
74  z/OS: z/OS ISPF Messages and Codes

## Page 95

User response
Provide the PARM3 attribute with a valid verification
string that matches the length of the field-mask.
ISPC896 ISPC896W: Warning. Line
aaaaaaaa of file "bbbbbbbb". No
picture string characters found in
verification string "cccccccc".
Explanation
The PICTCN verify statement requires that the
verification string contain at least one of the picture
string characters.
User response
Provide the PARM3 attribute with a valid verification
string that contains at least one picture string
character.
ISPC897 ISPC897W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Position "cccccccc" (character
"dddddddd") of field-mask
"eeeeeeee" does not correspond to
a valid picture string character in
verification string "ffffffff".
Explanation
The PICTCN verify statement requires that each mask-
character within the field-mask correspond to a valid
picture string character in the relative position of the
verification string.
User response
Provide the PARM3 attribute with a valid verification
string that contains a picture string character for each
mask-character found within the field-mask.
ISPC898 ISPC898W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The constant in position cccccccc
(character "dddddddd") of field-
mask "eeeeeeee" does not
correspond to the constant in the
relative position of verification
string "ffffffff".
Explanation
The PICTCN verify statement requires that each
character constant within the field-mask correspond
to the relative position of the verification string.
User response
Provide the PARM3 attribute with a valid verification
string that contains a matching character constant for
each character constant found within the field-mask.
ISPC901 ISPC901W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Horizontal AREA formatting has
reached the panel width. Tags
following this AREA will be
formatted below the current
AREA(s), beginning at the left edge
of the panel.
Explanation
An AREA tag specifying DIR=HORIZ is in process,
and the width of the AREA matches or exceeds the
PANEL width. ISPDTLC will format the current AREA
as specified. The next AREA or other tag will continue
formatting at the left edge of the panel.
User response
Review the WIDTH and DIR attributes on the AREA
tags. The combined width of horizontally formatted
AREAs cannot exceed the PANEL width.
ISPC908 ISPC908W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
CHECKI tag syntax error. CHECKI
tag will be discarded.
Explanation
This message is self-explanatory.
User response
Refer to previous log messages for more information
about the syntax errors.
ISPC909 ISPC909W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
CHECKI can be specified only
once. First specification will be
used.
Explanation
This message is self-explanatory.
User response
ISPDTLC supports only one CHECKI tag within each
CHECKL definition.
ISPC910 ISPC910W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  75

## Page 96

Variable by the same name
"cccccccc" was already defined.
Explanation
This message is self-explanatory.
User response
Each VARCLASS name and VARDCL name must be
unique.
ISPC911 ISPC911W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The VARCLASS attribute cccccccc
requires a dddddddd value. The
conversion process will continue
but defaults may be affected.
Please update your tag source file
to contain a valid dddddddd value.
Explanation
This message is self-explanatory.
User response
Add the missing information to the TYPE=cccccccc
attribute.
ISPC912 ISPC912W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc is a required attribute
when TYPE=dddddddd.. CHECKI
tag will be ignored.
Explanation
The required attribute cccccccc is not present.
User response
Add the required attribute information.
ISPC913 ISPC913W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc attribute value does
not correspond with dddddddd..
CHECKI tag will be ignored.
Explanation
The PARM2 data is not valid for TYPE=CHARS.
User response
Specify one of the listed character strings as
documented in z/OS ISPF Dialog Tag Language Guide
and Reference.
ISPC914 ISPC914W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
PARM1 is larger than PARM2.
CHECKI tag will be ignored.
Explanation
PARM2 specifies the high bound of a RANGE check
and must be larger than the low bound specified by
PARM1.
User response
Adjust the range values so that PARM2 is larger than
PARM1.
ISPC915 ISPC915W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Attribute ignored, not needed for
TYPE=cccccccc..
Explanation
This message is self-explanatory.
User response
Remove the extra attribute and value.
ISPC916 ISPC916W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
cccccccc="dddddddd" attribute
value is not supported in the
conversion process. Attribute
cccccccc will be set to the default
"eeeeeeee".
Explanation
This message is self-explanatory.
User response
Change dddddddd to a valid value.
ISPC918 ISPC918W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc keyword was not
specified. cccccccc will be set to
dddddddd..
Explanation
This message is self-explanatory.
User response
Provide the cccccccc attribute with an appropriate
value.
ISPF messages starting with ISP
76  z/OS: z/OS ISPF Messages and Codes

## Page 97

ISPC919 ISPC919W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
NAME = "cccccccc" keyword has
not been defined by the dddddddd
tag.
Explanation
A VARDCL tag has referenced an undefined VARCLASS
tag.
User response
Provide the missing VARCLASS definition.
ISPC920 ISPC920W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Double quote within CHECKI
TYPE=values will be treated as
character data and not as an item
enclosure.
Explanation
This message is self explanatory.
ISPC921 ISPC921W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
More than 100 items found
in PARM2 attribute value for
"TYPE=VALUES" on the CHECKI
tag. First 100 items will be used.
Explanation
This message is self-explanatory.
User response
Reduce the number of values to a maximum of 100.
ISPC922 ISPC922W: Warning. Line
aaaaaaaa of file "bbbbbbbb". The
attributes PARM2 and PARM3
contain duplicate values on the
CHECKI tag. The PARM3 value is
ignored.
Explanation
TYPE=INCLUDE on the CHECKI tag must have a
different value for PARM3 than the value provided for
PARM2.
User response
Provide unique PARM2 and PARM3 values.
ISPC930 ISPC930W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
VARLIST tag has been found
within the PANEL tag. The
VARLIST found within the PANEL
tag will be accepted for this
conversion. You should update
your tag source file to change the
VARLIST tag to occur outside the
PANEL tag. VARLIST within PANEL
is not a valid coding structure.
Explanation
This message is self-explanatory.
User response
Move the VARLIST tag group outside of the PANEL
definition.
ISPC931 ISPC931W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
VARDCL tag has been found both
under the scope of !DOCTYPE and
within the PANEL tag for variable
"cccccccc". The VARDCL found
within the PANEL tag will be
ignored. You should update your
tag source file to remove the
VARDCL for "cccccccc" located
within the PANEL tag as this is not
a valid coding structure.
Explanation
This message is self-explanatory.
User response
Remove the VARLIST tag group within the PANEL
definition.
ISPC932 ISPC932W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
A VARDCL tag has been found
within the PANEL tag for variable
"cccccccc". The VARDCL(s) found
within the PANEL tag will be
accepted for this conversion. You
should update your tag source file
to change all VARDCL(s) to occur
outside the PANEL tag. VARDCL
within PANEL is not a valid coding
structure.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  77

## Page 98

User response
Move the VARLIST tag group outside of the PANEL
definition.
ISPC934 ISPC934W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"TYPE=NUMERIC" has been
specified both on the CHECKI
tag and the VARCLASS tag. The
entry on the CHECKI tag will
be ignored for this conversion.
"TYPE=NUMERIC" should be
removed from the CHECKI tag.
Explanation
This message is self-explanatory.
User response
Remove the TYPE=NUMERIC coding from the CHECKI
tag.
ISPC935 ISPC935W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"USAGE" has been specified on
the XLATL tag. This attribute
has been removed from the tag
language. "USAGE" should be
determined by the referencing
tag (explicit or implied). "USAGE"
should be removed from the XLATL
tag.
Explanation
This message is self-explanatory.
User response
Remove the USAGE coding from the XLATL tag.
ISPC936 ISPC936W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
"VARDCL" was not found for
variable cccccccc specified on
the dddddddd tag. Each variable
should be specified by a "VARDCL"
tag.
Explanation
This message is self-explanatory.
User response
Add a VARDCL tag for each panel variable.
ISPC937 ISPC937W: Warning. Line
aaaaaaaa of file "bbbbbbbb". A
VARLIST tag has been found both
outside and within the PANEL
tag. The conversion utility does
not support a mixed occurrence
of VARLIST. The VARLIST found
within the PANEL tag will be
ignored for this conversion. You
should update your tag source file
to remove the VARLIST tags found
within the PANEL tag.
Explanation
This message is self-explanatory.
User response
Remove the VARLIST tag group within the PANEL
definition.
ISPC938 ISPC938W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
A VARDCL tag has been found
within the PANEL tag for variable
"cccccccc". The VARDCL found
within the PANEL tag will be
ignored because a VARLIST was
found outside the PANEL tag. You
should update your tag source file
to remove the complete VARLIST/
VARDCL tag group located within
the PANEL tag.
Explanation
This message is self-explanatory.
User response
Remove the VARLIST tag group within the PANEL
definition.
ISPC939 ISPC939W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
The VARCLASS attribute specified
on a VARDCL tag found within
the PANEL tag will be ignored.
VARDCL within the panel tag is
supported for prior release level
attributes only. The VARCLASS
attribute is valid only on VARDCL
tags found outside the PANEL
tag. You should update your
tag source file to remove the
complete VARLIST/VARDCL tag
ISPF messages starting with ISP
78  z/OS: z/OS ISPF Messages and Codes

## Page 99

group located within the PANEL
tag.
Explanation
This message is self-explanatory.
User response
Remove the VARLIST tag group within the PANEL
definition.
ISPC940 ISPC940E: Error. Line aaaaaaaa
of file "bbbbbbbb". The VARCLASS
tag to define "cccccccc" must
precede the VARDCL tag which
references "cccccccc".
Explanation
This message is self-explanatory.
User response
Place the VARCLASS tag for cccccccc before the
VARDCL tag which references it.
ISPC941 ISPC941E: Error. Line aaaaaaaa
of file "bbbbbbbb". All VARCLASS
tags must precede any VARLIST
tag in the source file.
Explanation
This message is self-explanatory.
User response
Place all VARCLASS tags before any VARLIST tag
group.
ISPC943 ISPC943W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
Length="cccccccc" must be an
even number for "TYPE=DBCS".
"cccccccc" has been changed to
"dddddddd".
Explanation
This message is self-explanatory.
User response
Change the cccccccc value to an even number.
ISPC944 ISPC944W: Warning. Line
aaaaaaaa of file "bbbbbbbb".
cccccccc="dddddddd" must be an
even number for field "eeeeeeee"
which is specified as DBCS.
"dddddddd" has been changed to
"ffffffff".
Explanation
This message is self-explanatory.
User response
Change the dddddddd value to an even number.
ISPC970 ISPC970W: Warning. ISPDTLC
received a nonzero return code
from EXECIO while processing
'aaaaaaaa'. Refer to the
'bbbbbbbb' documentation for an
explanation of EXECIO return code
'cccccccc'.
Explanation
This message is self-explanatory.
User response
Correct the condition causing return code cccccccc.
ISPC971 ISPC971E: Error. Check file
allocation for 'aaaaaaaa.'.
Explanation
The ISPDTLC EXECIO write routine has encountered
an error on data set 'aaaaaaaa'. This message will
be followed by message ISPC972 if space allocated is
equal to space used. This message will be followed by
message ISPC973 if directory blocks allocated is equal
to directory blocks used.
User response
Reallocate the data set name with more space or with
additional directory blocks.
ISPC972 ISPC972W: Warning. File space
allocated = file space used.
Explanation
This message follows message ISPC971 if space
allocated is equal to space used.
User response
Reallocate the data set name with more space.
ISPC973 ISPC973W: Warning. Directory
blocks allocated = directory blocks
used.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  79

## Page 100

Explanation
This message follows message ISPC971 if directory
blocks allocated is equal to directory blocks used.
User response
Reallocate the data set name with additional directory
blocks.
ISPC974 ISPC974W: Warning. Data has
been truncated during DISKW
operation.
Explanation
The length of the output record exceeds the LRECL of
the output file.
User response
Verify that the record length of the file is correct.
ISPC975 ISPC975W: Warning. End of file
reached before the specified
number of records have been read.
Explanation
This message is self-explanatory.
User response
If message recurs, notify IBM service.
ISPC976 ISPC976E: Error. EXECIO
completed unsuccessfully.
Explanation
This message is self-explanatory. Review any
immediately previous messages for additional
information.
User response
If message recurs, notify IBM service.
ISPC978 ISPC978W: Warning. ISPDTLC
received a nonzero return code
from ISPF service 'aaaaaaaa'
while processing 'bbbbbbbb'.
The ISPF error message is
'cccccccc'. Please refer to the
ISPF documentation for an
explanation of aaaaaaaa return
code 'dddddddd'.
Explanation
This message is self-explanatory.
User response
Refer to the ISPF documentation for a more complete
description of aaaaaaaa return codes.
ISPC979 ISPC979W: Warning. Refer to
the aaaaaaaa. documentation for
a more complete description of
EXECIO return codes.
Explanation
This message is self-explanatory.
User response
Refer to the aaaaaaaa. documentation for a more
complete description of EXECIO return codes.
ISPC980A ISPC980AE: Error. Do not specify
a member name for the
aaaaaaaa file sequential data set
"bbbbbbbb".
Explanation
A sequential data set name has been entered with a
member name on the ISPDTLC invocation panel for the
log file or list file.
User response
Correct the data set name by removing the member
name.
ISPC980B ISPC980BE: Error. Do not specify
a member name for the
aaaaaaaa file partitioned data set
"bbbbbbbb".
Explanation
A partitioned data set name has been entered with a
member name on the ISPDTLC invocation panel.
User response
Correct the data set name by removing the member
name.
ISPC981 ISPC981E: Error. The aaaaaaaa
file "bbbbbbbb" is a partitioned
data set and must be specified
with a member name. You may
use an asterisk (*) as the member
name to specify that the aaaaaaaa
ISPF messages starting with ISP
80  z/OS: z/OS ISPF Messages and Codes

## Page 101

member name is the same as the
input GML member name.
Explanation
A log or list partitioned data set provided in the
ISPDTLC profile must be specified with a member
name. The member name can be an asterisk (*) to
specify that the aaaaaaaa member name is the same
as the input GML member name.
User response
Correct the profile entry by providing a member
name for data set bbbbbbbb referenced on DDNAME
DTLaaaaaaaa.
ISPC981A ISPC981AI: Notify. The aaaaaaaa
file "bbbbbbbb" is a partitioned
data set specified without a
member name. An asterisk (*) is
assumed as the member name
to specify that the aaaaaaaa
member name is the same as the
input GML member name.
Explanation
A log or list partitioned data set provided in the
ISPDTLC profile may be specified with a member
name. If no member name is provided, an asterisk
(*) is assumed to specify that the aaaaaaaa member
name is the same as the input GML member name.
User response
No action is required. You can eliminate this message
by updating the profile entry to provide a member
name for data set bbbbbbbb referenced on DDNAME
DTLaaaaaaaa.
ISPC982 ISPC982E: Error. The aaaaaaaa
file "bbbbbbbb" is not available.
The TSO function SYSDSN has
returned the message: "cccccccc".
Explanation
This message is self-explanatory.
User response
Correct the specified file name.
ISPC983 ISPC983E: Error. "aaaaaaaa"
profile not found.
Explanation
The ISPDTLC profile aaaaaaaa was not found.
User response
Specify a correct profile data set name.
ISPC984 ISPC984E: Error. Bad aaaaaaaa
structure for "bbbbbbbb".
Explanation
This message is self-explanatory.
User response
The response to this message depends on the error
information in the message as returned to ISPDTLC
from TSO.
ISPC985 ISPC985E: Error. Bad profile
structure. DDNAME "aaaaaaaa" in
profile "bbbbbbbb" is not valid.
Explanation
This message is self-explanatory.
User response
Correct the DDNAME values within profile data set
"bbbbbbbb".
ISPC986 ISPC986E: Error. The aaaaaaaa
file "bbbbbbbb" record format
"cccccccc" is not valid. This file
must have a "dddddddd" format.
Explanation
All referenced aaaaaaaa files must have the same
record format.
User response
Change the record format for file bbbbbbbb as
indicated.
ISPC987 ISPC987E: Error. The allocation for
file "aaaaaaaa" has failed. The
TSO ALLOC function has returned
the message: "bbbbbbbb".
Explanation
The TSO allocation for file aaaaaaaa failed.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  81

## Page 102

User response
Make sure the aaaaaaaa file is available. If the
allocation failure is caused by a previous error, you
may have to log off from TSO to correct the problem.
ISPC988 ISPC988E: Error. The "SCRIPT"
option has been specified, but no
SCRIPT output file name has been
provided.
Explanation
This message is self-explanatory.
User response
Provide a file for SCRIPT output.
ISPC989 ISPC989E: Error. Incorrect syntax:
Option "aaaaaaaa" conflicts with
option "bbbbbbbb". You cannot
specify both options for the
same invocation of the conversion
utility.
Explanation
This message is self explanatory.
User response
Specify only one of the listed options.
ISPC989A ISPC989AE: Error. Incorrect
syntax: Option "aaaaaaaa"
conflicts with option "bbbbbbbb".
You cannot specify both options on
the same COMPOPT tag.
Explanation
This message is self explanatory.
User response
Specify only one of the listed options.
ISPC990 ISPC990E: Error. Incorrect syntax:
Option must be "aaaaaaaa" or
"bbbbbbbb" but not both.
Explanation
This message is self-explanatory.
User response
Remove either option aaaaaaaa or option bbbbbbbb
from the invocation syntax.
ISPC991 ISPC991E: Error. Incorrect syntax:
aaaaaaaa="bbbbbbbb". The value
for aaaaaaaa must be from 1 to
cccccccc characters in length.
Explanation
This message is self-explanatory.
User response
Change bbbbbbbb to a maximum of cccccccc
characters.
ISPC992 ISPC992E: Error. Incorrect syntax:
aaaaaaaa="bbbbbbbb" contains
character(s) which are not valid.
The first character of "bbbbbbbb"
must be "A-Z" or "#, $, @".
Remaining characters must be "A-
Z", "#, $, @" or "0-9".
Explanation
This message is self-explanatory.
User response
Remove the invalid characters from the invocation
syntax.
ISPC993 ISPC993E: Error. Incorrect syntax:
aaaaaaaa="bbbbbbbb" contains
character(s) which are not valid.
All characters must be "A-Z", "#,
$, @" or "0-9".
Explanation
This message is self explanatory.
ISPC994 ISPC994E: Error. Option is not
valid: "aaaaaaaa".
Explanation
The invocation syntax aaaaaaaa is not valid.
User response
Remove the invalid value aaaaaaaa from the
invocation syntax.
ISPC995 ISPC995E: Error. Undefined error.
Explanation
This message is self explanatory.
ISPF messages starting with ISP
82  z/OS: z/OS ISPF Messages and Codes

## Page 103

ISPC996 ISPC996E: Error. Duplicate option:
"aaaaaaaa".
Explanation
The invocation syntax aaaaaaaa is a duplicate value.
User response
Remove the duplicate value aaaaaaaa from the
invocation syntax.
ISPC997 ISPC997E: Error. Option is not
valid: "aaaaaaaa=bbbbbbbb".
Explanation
The invocation syntax aaaaaaaa=bbbbbbbb is invalid.
User response
Remove the invalid value aaaaaaaa=bbbbbbbb from
the invocation syntax.
ISPC998 ISPC998E: Error. The ISPDTLC
invocation syntax is: "ISPDTLC
source-filespec ( invocation
options". The "(" is required to
identify the invocation options.
Explanation
This message is self-explanatory.
User response
The invocation syntax requires that the 'source-
filespec' and invocation options be separated by the
character "(". Respecify the invocation command,
including the "(" delimiter.
ISPC999 ISPC999E: Conversion terminated
due to severe error.
Explanation
This message is self-explanatory.
User response
This message follows one or more other errors in the
log. Refer to the other error messages for the problem
explanation.
ISPD001 Allocation failed - DAIR RC =
aaaaaaaa dec, DARC = bbbbbbbb
hex.
Explanation
A TSO allocation failed. The Dynamic Allocation
Interface Return Code (DAIR) and the Dynamic
Allocation Return Code have been supplied. For more
information on these return codes, see the System
Programmer response.
System programmer response
The DAIR and DARC codes are listed in the ISPF
tutorial. Enter HELP to get the first tutorial panel, then
enter INDEX on the command line. Enter D on the
command line to get to the topics screen, and select
D1 to get the listing of the DAIR and DARC codes.
User response
Contact your system programmer.
ISPD002 Data set not found - 'aaaaaaaa'
not on volume 'bbbbbbbb'.
Explanation
The data set specified was not found on the volume
specified.
System programmer response
Verify the data set name and the volume. If the data
set name is valid, determine it's volume location.
User response
Verify the data set name, and the volume. Contact your
system programmer.
ISPD003 Catalog or VTOC error -
I/O, insufficient storage, or
unrecoverable error with volume
aaaaaaaa.
Explanation
The return code from TSO DAIR indicates one of
the three problems listed. Contact your system
programmer with the information in this error
message.
System programmer response
If you are unable to determine the problem with the
listed volume, contact IBM support.
User response
Contact your system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  83

## Page 104

ISPD004 Volume 'aaaaaaaa' not avail -
Volume not mounted or not
authorized for your use.
Explanation
The volume you requested is not mounted, or you are
not authorized to use this volume.
System programmer response
Determine why this user is unable to use the volume.
User response
Contact your system programmer.
ISPD005 Data set in use - Data set
'aaaaaaaa' in use by another user,
try later.
Explanation
Another user has the use of this data set. Try to access
this data set later.
User response
Try to access the data set at a later date.
ISPD006 VTOC full on vol 'aaaaaaaa' - No
room in volume table of contents
for more data sets.
Explanation
The Volume Table of Contents (VTOC) on this volume is
full. Another data set cannot be added to the VTOC.
System programmer response
Correct the DASD space problem.
User response
Provide this information to the system programmer.
ISPD007 No space on vol 'aaaaaaaa' -
Insufficient space for data set
'bbbbbbbb'.
Explanation
The volume specified has insufficient space for the
data set specified.
System programmer response
Correct the DASD space problem.
User response
Contact your system programmer.
ISPD008 Invalid index structure - Dsname
'aaaaaaaa' index conflict.
Explanation
The TSO DAIR routine has reported a problem with this
data set.
System programmer response
Correct the DASD index problem.
User response
Contact your system programmer.
ISPD009 DDNAME not found - DDNAME
'aaaaaaaa' not allocated.
Explanation
The ddname has not been allocated.
User response
Allocate the ddname or contact the responsible
programmer.
ISPD010 File name in use - File name
(DDNAME) 'aaaaaaaa' is allocated
to another data set.
Explanation
The ddname is not available for your use at this time.
System programmer response
Correct the allocations.
User response
Contact the responsible programmer.
ISPD011 Syntax error in DDNAME -
File name (DDNAME) 'aaaaaaaa'
contains invalid special character.
Explanation
An invalid special character was found in the DDNAME.
User response
Correct the syntax of the ddname.
ISPF messages starting with ISP
84  z/OS: z/OS ISPF Messages and Codes

## Page 105

ISPD012 Syntax error member name
- Member name 'aaaaaaaa'
contains invalid special character.
Explanation
The member name specified contains an invalid
special character.
User response
Correct the syntax of the member name.
ISPD013 Invalid data set name - Syntax
error in data set name 'aaaaaaaa'.
Explanation
This message is self-explanatory.
User response
Correct the syntax of the data set name.
ISPD014 Data set not cataloged -
'aaaaaaaa' was not found in
catalog.
Explanation
The data set was not found in the catalog.
User response
Enter a valid data set name.
ISPD015 Dd dynams exhausted - Data set
'aaaaaaaa' cannot be allocated.
Explanation
You will be unable to allocate additional ddnames.
User response
Contact your system programmer.
ISPD016 Duplicate data set name - Data set
'aaaaaaaa' already exists.
Explanation
You cannot create a data set with this name.
User response
Create your data set using another name.
ISPD017 Data set not partitioned - Member
'aaaaaaaa' was specified for data
set 'bbbbbbbb'.
Explanation
This data set is not a partitioned data set.
User response
Store this member in a partitioned data set or store it
as a sequential data set.
ISPD018 Multivolume data set - Data set
'aaaaaaaa' occupies more than
one volume.
Explanation
You do not have multivolume support.
User response
Contact your system programmer.
ISPD019 Catalog error - Already cataloged,
VSAM protected, or other -
'aaaaaaaa'.
Explanation
Catalog attempt was unsuccessful.
User response
Use another name or contact the system programmer.
ISPD020 Data set already open - Data set
'aaaaaaaa' is already in use on
your behalf.
Explanation
You are already using the data set and you cannot
open it again.
User response
Complete the other task that uses the data set, and
then this task can use it.
ISPD021 Allocation denied - Data set
'aaaaaaaa' - request denied by
installation exit.
Explanation
Your installation will not allow you to use this data set.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  85

## Page 106

User response
Contact your system programmer.
ISPD022 Allocation failed - DAIR RC
= 'aaaaaaaa' dec, CTRC =
'bbbbbbbb' hex.
Explanation
Allocation failed, the return codes listed will be
required to determine why the allocation failed.
System programmer response
The DAIR and DARC codes are listed in the ISPF
tutorial. Enter HELP to reach the first tutorial panel,
then enter INDEX on the command line. Enter D on the
command line to get to the topics screen. Select D1 for
the listing of the DAIR and DARC codes.
User response
Contact your system programmer.
ISPD023 Tape not supported - ISPF does
not support data sets on tape.
Explanation
Allocation failed for a data set on tape. ISPF does not
support data sets on tape.
ISPD101 Invalid panel name - 'aaaaaaaa' -
8 char limit
Explanation
The panel name must be 8 characters or fewer.
Programmer response
Use a shorter panel name.
ISPD102 Invalid PGM name - 'aaaaaaaa' - 8
char limit
Explanation
The program name must be 8 characters or fewer.
Programmer response
Retry with a shorter program name.
ISPD103 Invalid keyword given -
'aaaaaaaa' invalid
Explanation
A keyword name is invalid for one of these reasons:
• The keyword name is longer than 10 characters.
• The keyword name is not one of the valid keyword
names.
• The MODE keyword is not used with PGM or CMD.
Programmer response
Correct the keyword and try again.
ISPD104 Invalid parameter given -
'aaaaaaaa' invalid
Explanation
A keyword is invalid for one of these reasons:
• A value is given in parentheses with no name before
it.
• The keyword does not have an expected value.
• The keyword is longer than 10 characters.
• The value of the MODE keyword is not FSCR or LINE.
Programmer response
Correct the keyword and try again.
ISPD105 Insufficient storage - Unable to
process command.
Explanation
A GETMAIN command for main storage failed while
analyzing ISPF command syntax, or while creating
internal file control tables. The command was not
processed.
Programmer response
Try running the program in a larger region.
ISPD106 Conflicting parameters - CMD
conflicts with another parameter.
Explanation
A keyword was found that is inconsistent with the CMD
keyword. For example, PGM and CMD cannot both be
used in the same command.
Programmer response
Check the usage of the CMD keyword, and eliminate
the conflict.
ISPF messages starting with ISP
86  z/OS: z/OS ISPF Messages and Codes

## Page 107

ISPD107 Conflicting parameters - PGM
conflicts with another parameter
Explanation
A keyword was found which is inconsistent with the
PGM keyword. For example, PGM and PANEL cannot
both be used in the same command.
Programmer response
Check the usage of the PGM keyword, and eliminate
the conflict.
ISPD108 Conflicting parameters - PANEL
conflicts with another parameter.
Explanation
A keyword was found that is inconsistent with the
PANEL keyword. For example, the PANEL and PARM
keywords cannot both be used in the same command.
Programmer response
Check the usage of the PANEL keyword and remove
the conflict.
ISPD109 Invalid command - Parentheses
are not paired correctly.
Explanation
Each left parenthesis must be matched with a
right parenthesis. The command has a left or right
parenthesis that is unpaired.
Programmer response
Correct the syntax to make sure parentheses are
paired correctly.
ISPD110 CMD abended - 'aaaaaaaa'
terminated abnormally.
Explanation
The ISPF command aaaaaaaa has abnormally
terminated. If this was a system abend, message
ISPG075 was issued with the system abend code. If
this was a user abend, message ISPG073 was issued
with the user abend code.
Programmer response
Review the other messages that explain the abend.
ISPD111 CMD did not complete -
'aaaaaaaa' was terminated with
an ATTENTION.
Explanation
The command or CLIST did not complete because an
ATTENTION was signalled. This probably happened
because the PA1 key was pressed.
User response
No response is required.
ISPD112 Invalid PGM name - Link to
'aaaaaaaa' failed.
Explanation
An MVS™ LINK to the program named aaaaaaaa has
failed. The reason could be:
• The program was not found.
• The program was not authorized.
Programmer response
Check the name and authorization of aaaaaaaa for
errors.
ISPD113 Invalid panel - 'aaaaaaaa' is not a
valid 'selection' panel.
Explanation
The panel that was asked to display is not a valid menu
panel. This is probably because it does not have a
ZSEL variable.
User response
Be sure that the panel name is that of a menu panel.
Programmer response
If necessary, add a ZSEL variable to the panel
definition.
ISPD114 Invalid NEWAPPL ID - aaaaaaaa
is not a valid NEWAPPL ID. Length
exceeds 4 characters.
Explanation
The NEWAPPL keyword requires a 1 to 4 character
application ID. The supplied ID was greater than 4
characters.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  87

## Page 108

Programmer response
Retry the command with a 1 to 4 character NEWAPPL
keyword value.
ISPD115 Option conflict - An OPT keyword
conflicts with a previously
specified option.
Explanation
The OPT keyword specifies an option on the first
menu. This keyword cannot be used more than once.
Programmer response
Retry the command with only a single OPT keyword.
ISPD116 Option too long - Maximum option
length supported is 24 characters.
Explanation
The OPTION value has a length limit and this limit has
been exceeded.
Programmer response
Use a shorter OPTION length.
ISPD117 The initially invoked CLIST ended
with a return code = aaaaaaaa.
Explanation
A CLIST or REXX procedure was selected at ISPSTART
time that ended with a return code that was not 0 or 4.
These are the only valid values.
Programmer response
If you wish to return a value to the calling program or
procedure, consider using the ZISPFRC variable. See
z/OS ISPF Dialog Developer's Guide and Reference for
details.
ISPD118 The initially invoked module ended
with a return code = aaaaaaaa.
Explanation
A program module was selected at ISPSTART time that
set a return code value other than 0 or 4. These are the
only valid values.
Programmer response
If you wish to set a return code for the program or
procedure that invoked ISPSTART, consider using the
ZISPFRC variable to return this value. See z/OS ISPF
Dialog Developer's Guide and Reference for details.
ISPD119 Required parm missing - A select
parameter requires one of PGM,
CMD, or PANEL.
Explanation
A required SELECT parameter was omitted. If the OPT
parameter was specified, PANEL was omitted. If the
LANG parameter was specified, CMD was omitted. If
the PARM parameter was specified, PGM was omitted.
Programmer response
Add the required parameter and retry the request.
ISPD120 Invalid CLIST name - 'aaaaaaaa' -
name greater than 8 characters.
Explanation
The CLIST name specified in the CMD parameter
cannot be longer than 8 characters.
Programmer response
Correct the CLIST name so that it is fewer than or
equal to 8 characters.
ISPD121 Specified option invalid -
'aaaaaaaa' - contains invalid opt
selection.
Explanation
The SELECT service request used an OPT keyword but
the keyword value was invalid for the panel selected.
For example, on panel XYZ, options 1 to 5 are valid and
OPT(6) was specified.
Programmer response
Change the panel name or OPT value on the SELECT
request.
ISPD122 Command not allowed -
'aaaaaaaa' command not allowed
within ISPEXEC select service
request.
Explanation
A nested command, probably ISPEXEC, was found
within an ISPEXEC command request.
Programmer response
Avoid the use of nested ISPEXEC commands.
ISPF messages starting with ISP
88  z/OS: z/OS ISPF Messages and Codes

## Page 109

ISPD123 Invalid command - Command
'aaaaaaaa' not found or contains
invalid syntax.
Explanation
The command was not found, was otherwise not
executable, or the exit routine returned an invalid
return code.
Programmer response
The command was not found in the assumed library.
ISPD124 Invalid command - Command
name 'aaaaaaaa' contains invalid
syntax.
Explanation
The command name is invalid. Command names must
start with an alphabetic character or @, #, or $. The
remaining characters must be alphanumeric. In MVS,
the remaining characters (but not the initial character)
can be X'C0'(left brace).
Programmer response
Correct the command name syntax to conform to
these rules.
ISPD126 Invalid command - Commands
such as LOGON, LOGOFF, SPF,
ISPF, etc. are not permitted.
Explanation
Special commands that may harm the ISPF
environment are not permitted. These commands
include LOGON, LOGOFF, SPF, ISPF, test authorized
commands, and commands invoking an authorized
program. CLISTs cannot invoke these commands
either.
Programmer response
Remove the unauthorized command and continue.
ISPD128 Invalid command - The entry
for this command in the ISPF
TSO Command Table (ISPTCM)
indicates it cannot be run when
ISPF has been invoked from a web
client.
Explanation
Commands that cause problems or fail when ISPF is
running for a web client can be defined in the ISPF TSO
Command Table (ISPTCM) as being ineligible to run in
this environment.
System programmer response
Check the ISPF TSO Command Table (ISPTCM)
definition to see if this command is defined as being
ineligible for the web client environment.
ISPD130 Recursion error - Recursive use of
an ISPF function ( aaaaaaaa ) is
not allowed.
Explanation
A non-recursive ISPF function was already active and
an attempt was made to link to another program. This
is not allowed.
Programmer response
Correct the application so that ISPF is not entered
recursively.
ISPD131 PDF subs load error - Unable to
load the subroutines for the PDF
component of ISPF.
Explanation
This message is self explanatory.
ISPD132 Insufficient storage - Insufficient
storage to load the subroutines for
the PDF component of ISPF.
Explanation
Not enough main storage is available to load PDF.
Programmer response
Increase ISPF's region size as necessary.
ISPD133 Invalid application ID - Appl ID
'aaaaaaaa' is reserved for system
use by the dialog manager.
Explanation
Any application ID that starts with "ISP" and whose
fourth character is non-blank is an invalid ID.
Programmer response
Use an application ID that does not violate the rule in
the message explanation.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  89

## Page 110

ISPD134 Invalid application ID - Appl ID
'aaaaaaaa' starts with or contains
an invalid character.
Explanation
The application ID must start with an alphabetic
character or with @, #, or $ and the remaining
characters must be alphanumeric. The MVS X'C0'(left
brace) character may be used in any character except
the first.
Programmer response
Correct the application ID to conform to the rules in
the message explanation.
ISPD135 Select string truncated -
Substituted select string exceeded
allowable length of 32767.
Explanation
The ISPF command text length exceeded 32 767 bytes
after substitution of variables.
System action
The command is not executed.
Programmer response
Correct the substituted variables so the command text
is less than 32 767 bytes after substitution.
ISPD136 Language parm invalid -
'aaaaaaaa' specified in LANG
parameter is not valid.
Explanation
The value of the LANG parameter must be APL,
COBOL, PL1, or PLI. The value specified was not valid.
Programmer response
Change the command to use one of the allowed LANG
values.
ISPD137 Language parameter error -
Syntax error in language
parameter value 'aaaaaaaa'.
Explanation
The language parameter value has some kind of
syntax error. There may be extra characters after the
language name, embedded blanks in the name, or an
invalid delimiter.
Programmer response
Correct the language parameter to remove the syntax
error.
ISPD139 Language size error - Language
size contains nonnumeric
characters or too many digits.
Explanation
The language size parameter has a nonnumeric
character or has more than 8 digits.
Programmer response
Correct the language size parameter.
ISPD140 Enter option - Enter one of the
listed options.
Explanation
No option value was entered on a menu panel. This
applies to all menu panels except the primary option
menu.
User response
Key one of the options highlighted on the menu panel
and press Enter.
ISPD141 Invalid option - The option that
was entered was not valid.
Explanation
The option entered was not one of the options defined
in the menu panel. The menu panel definition replaced
an invalid option selection with a "?" character that
caused this message to display.
User response
Enter a valid option value for this menu.
ISPD142 Nesting limit exceeded - The
nesting limit for dialog EXECs in
user area has been exceeded.
Explanation
This message is self explanatory.
ISPD143 EXEC not found - The EXEC named
'aaaaaaaa' was not found in the
search order.
ISPF messages starting with ISP
90  z/OS: z/OS ISPF Messages and Codes

## Page 111

Explanation
This message is self explanatory.
ISPD144 Panel definition error - 100 panels
processed. No return key stop
found. ZPARENT causing loop.
Explanation
After executing the RETURN command, ISPF tries to
display the primary option menu in the hierarchy or
explicit panel chain. If more than 100 panels are in
the chain, ISPF assumes that there is an error in the
specification of ZPARENT that is causing an infinite
loop in the panel chain. Processing stops.
Programmer response
Look for errors in the specification of ZPARENT in
panel definitions or in the application.
ISPD145 Invalid logo panel name -
'aaaaaaaa' - name greater than 8
characters.
Explanation
The LOGO keyword on the ISPSTART command
specifies a panel name whose length is greater than
8 characters. No more than 8 are allowed.
Programmer response
Correct the LOGO keyword value to use a shorter panel
name.
ISPD146 Keyword conflict - BARRIER and
NEST are only allowed with the
CMD keyword.
Explanation
Use BARRIER and NEST only in a TSO environment.
Programmer response
Remove the BARRIER or NEST keyword unless a CMD
function is being invoked. These keywords are only
valid when used with the CMD parameter.
ISPD147 Wrong environment - BARRIER
and NEST keywords are only valid
in the TSO environment.
Explanation
Use BARRIER and NEST only in a TSO environment.
Programmer response
Only use BARRIER and NEST in a TSO environment.
ISPD148 BLDL error - Error processing
LIBDEF search for program
aaaaaaaa. BLDL return code =
bbbbbbbb.
Explanation
A LIBDEF was specified for ISPLLIB and BLDL
encountered an error trying to locate a program while
invoking the SELECT service. A return code of 8 would
normally indicate an I/O error.
User response
Refer to the appropriate system documentation for an
explanation of the BLDL macro return codes.
ISPD150 Invocation error - System error
encountered invoking authorized
command 'aaaaaaaa'.
Explanation
ISPF cannot invoke an authorized command, such as
LOGON. It is also possible that the TSO command start
exit routine rejected the command.
Programmer response
Avoid usage of authorized commands in ISPF.
ISPD151 Language qualifier error - The only
value valid for the APL language is
START.
Explanation
Some value other than START was specified for the
APL language qualifier. Or START was followed by
something else, which is invalid.
Programmer response
Correct the syntax of the APL language qualifier.
ISPD152 Invalid PGM name - Load of PGM
'aaaaaaaa' failed.
Explanation
This message is self explanatory.
ISPD153 PASSLIB spec invalid - NEWAPPL
must be specified to specify
PASSLIB.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  91

## Page 112

Explanation
The PASSLIB keyword was used but NEWAPPL was
omitted.
Programmer response
Correct the syntax and retry the command.
ISPD154 Storage release error -
Error occurred when releasing
application library blocks.
Explanation
A severe error occurred while doing FREEMAIN of
LIBDEF control blocks.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPD155 Deallocation failure - Deallocation
failure - check allocated data sets.
Explanation
The deallocation of LIBDEF data sets failed during the
termination of ISPEXEC.
User response
Contact your system programmer.
Programmer response
Contact IBM support.
ISPD156 Authorization failure - Request
rejected by exit routine for select
service start.
Explanation
The exit routine rejected the request because it lacked
authority to use the exit.
Programmer response
Correct the authorization failure.
ISPD157 Severe error - Select service start
exit routine returned invalid code
aaaaaaaa.
Explanation
The select service start exit routine returned an
unexpected return code, which is shown in the
message.
Programmer response
Examine the exit routine to see where the unexpected
return code was generated.
ISPD158 Invocation error - System error
encountered invoking command
'aaaaaaaa'.
Explanation
Routine IKJTBLS abended while starting this
command.
Programmer response
Examine the abend information to determine cause of
abend.
ISPD170 Conflicting parameters - DCSS
conflicts with another parameter.
Explanation
This message is self explanatory.
ISPD171 Storage error - A storage error
occurred in the SELECT DCSS
service.
Explanation
This message is self explanatory.
ISPD172 DCSS name too long - The segment
name 'aaaaaaaa' is greater than 8
characters.
Explanation
This message is self explanatory.
ISPD173 Wrong environment - The SELECT
DCSS service is only valid in a CMS
environment.
Explanation
This message is self explanatory.
ISPD174 Conflicting parameters - DCSS or
PGM was specified more than
once.
ISPF messages starting with ISP
92  z/OS: z/OS ISPF Messages and Codes

## Page 113

Explanation
This message is self explanatory.
ISPD175 IKJADTAB interface error - RC
= 'aaaaaaaa' from TSO routine
IKJADTAB, function = 'bbbbbbbb'.
Explanation
During NEWAPPL processing of the SELECT service,
TSO routine IKJADTAB was invoked to set up or free
an ALTLIB table for the new select level. IKJADTAB
set return code aaaaaaaa, which is an internal error.
The service function bbbbbbbb is what was passed to
IKJADTAB.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPD176 Interface error - RC = 'aaaaaaaa'
from TSO routine IRXSTK, function
= DROPTERM.
Explanation
During NEWAPPL processing for the SELECT service,
TSO routine IRXSTK is called to remove a barrier
(DROPTERM) from the REXX data stack. IRXSTK set
return code aaaaaaaa, which is an internal error.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPD177 Interface error - RC = 'aaaaaaaa'
from TSO routine IRXSTK, function
= MARKTERM.
Explanation
During NEWAPPL processing for the SELECT service,
routine IRXSTK is called to place a barrier
(MARKTERM) on the REXX data stack. IRXSTK set
return code aaaaaaaa, which is an internal error.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPD178 Invalid screen name - A screen
name must be more than one
character, and all alphanumerics,
and cannot be LIST, PREV, or
NEXT.
Explanation
A screen name must contain from 2 to 8 alphanumeric
characters. It cannot be LIST, PREV, or NEXT.
Programmer response
Correct the screen name specified on the SELECT
service.
ISPD180 Invalid control parm - An
attention interrupt has already
been established for this session.
Explanation
This message is self explanatory.
ISPD181 Invalid control parm - There is no
current attention to CANCEL.
Explanation
This message is self explanatory.
ISPD182 Invalid environment - This service
does not exist in this environment.
Explanation
This message is self explanatory.
ISPD183 Invalid interface - This service can
only be invoked from the module
interface.
Explanation
This message is self explanatory.
ISPD187 Key display forced off - Current
application does not allow ISPF to
display function keys.
Explanation
This message is self explanatory.
ISPD188 Key display forced on - Current
application does not allow the
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  93

## Page 114

function key display to be
removed.
Explanation
This message is self explanatory.
ISPD189 Mismatched quotes - The string
contains an uneven number of
quotes.
Explanation
Unmatching quotes were found in a parenthesized
keyword value for an ISPF service request.
Programmer response
Correct the command syntax so there are no
unmatched quotation marks.
ISPD190 Invalid CONTROL parm -
'aaaaaaaa' is invalid as request
type parameter to the CONTROL
service.
Explanation
This message is self explanatory.
ISPD191 Invalid CONTROL parm -
'aaaaaaaa' is invalid option for
'CONTROL errors' service request.
Explanation
This message is self explanatory.
ISPD192 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for 'CONTROL display' service
request.
Explanation
This message is self explanatory.
ISPD193 Invalid CONTROL parm - The
start line-number is invalid
for CONTROL DISPLAY service
request.
Explanation
This message is self explanatory.
ISPD194 Missing CONTROL parm - A
required parameter is missing
from the CONTROL service
request.
Explanation
This message is self explanatory.
ISPD195 Missing CONTROL parm - A
required parameter missing
from CONTROL DISPLAY service
request.
Explanation
This message is self explanatory.
ISPD196 Invalid CONTROL parm -
'aaaaaaaa' is invalid option for
CONTROL ABEND service request.
Explanation
This message is self explanatory.
ISPD197 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL CRETURN service
request.
Explanation
This message is self explanatory.
ISPD198 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL NONDISPL service
request.
Explanation
This message is self explanatory.
ISPD201 Panel name is invalid - 'aaaaaaaa'
- name greater than 8 characters.
Explanation
The panel name must be 8 characters or fewer.
Programmer response
Use a shorter panel name.
ISPD202 PGM name is invalid - 'aaaaaaaa' -
name greater than 8 characters.
Explanation
The program name must be 8 characters or fewer.
Programmer response
Use a shorter program name.
ISPF messages starting with ISP
94  z/OS: z/OS ISPF Messages and Codes

## Page 115

ISPD203 Invalid keyword - 'aaaaaaaa'
contains unrecognized keyword.
Explanation
A keyword name is invalid for one of these reasons:
• The name is longer than 10 characters.
• The name is not one of the valid keyword names.
• The MODE keyword is not used with PGM or CMD.
Programmer response
Correct the keyword and try again.
ISPD204 Invalid parameter - 'aaaaaaaa'
contains unrecognized parameter.
Explanation
A keyword is invalid for one of these reasons:
• A value was given in parentheses with no name
before it.
• The keyword did not have an expected value.
• The keyword was longer than 10 characters.
• The value of the MODE keyword was not FSCR or
LINE.
Programmer response
Correct the keyword and try again.
ISPD205 Insufficient storage - Unable to
continue processing.
Explanation
A GETMAIN for main storage failed while analyzing
ISPF command syntax or while creating internal file
control tables. The command was not executed.
Programmer response
Try running the application in a larger region. For
further assistance, contact IBM support.
ISPD206 Conflicting parameters - CMD
conflicts with another keyword.
Explanation
A keyword was found that is inconsistent with the CMD
keyword. For example, PGM and CMD cannot both be
used in the same command.
Programmer response
Check the usage of the CMD keyword and eliminate
the conflict.
ISPD207 Conflicting parameters - PGM
conflicts with another keyword.
Explanation
A keyword was found that is inconsistent with the PGM
keyword. For example, PGM and PANEL cannot both be
used in the same command.
Programmer response
Check the usage of the PGM keyword and eliminate
the conflict.
ISPD208 Conflicting parameters - PANEL
conflicts with another keyword.
Explanation
A keyword was found that is inconsistent with the
PANEL keyword. For example, the PANEL and PARM
keywords cannot both be used in the same command.
Programmer response
Check the usage of the PANEL keyword and remove
the conflict.
ISPD209 Incorrect parameters -
Parentheses are not paired
correctly.
Explanation
Each left parenthesis must be matched with a
right parenthesis. The command had a left or right
parenthesis that was unpaired.
Programmer response
Correct the syntax to make sure parentheses are
paired correctly.
ISPD210 CMD abended - 'aaaaaaaa'
terminated abnormally.
Explanation
The aaaaaaaa command has abnormally terminated.
Programmer response
Use the provided error message and log information
to determine the cause of the abend. Consider using
the ENVIRON command and the Dialog Test facility to
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  95

## Page 116

aid in problem determination. See z/OS ISPF Dialog
Developer's Guide and Reference for further help.
ISPD211 CMD did not complete -
'aaaaaaaa' was terminated with
an ATTENTION.
Explanation
The command or CLIST did not complete because an
ATTENTION was received, probably because the PA1
key was pressed.
User response
No response is required.
ISPD212 Invalid PGM name - Link to
'aaaaaaaa' failed, abend code =
x'bbbbbbbb'.
Explanation
An MVS LINK to the program named aaaaaaaa has
failed. The reason could be:
• The program was not found.
• The program was not authorized.
Programmer response
Check the name and authorization of aaaaaaaa for
errors.
ISPD213 Invalid panel - 'aaaaaaaa' is not a
valid 'selection' panel.
Explanation
The panel you asked to display is not a valid menu
panel. This is probably because it does not have a
ZSEL variable.
User response
Be sure that the panel name is that of a menu panel.
Programmer response
If necessary, add a ZSEL variable to the panel
definition.
ISPD214 Invalid NEWAPPL ID - 'aaaaaaaa'
is not a valid 'NEWAPPL ID'.
Length exceeds 4 characters.
Explanation
The NEWAPPL keyword requires a 1 to 4 character
application ID. The supplied ID was greater than 4
characters.
Programmer response
Retry the command with a 1 to 4 character NEWAPPL
keyword value.
ISPD215 Option conflict - An OPT keyword
conflicts with a previously
specified option.
Explanation
The OPT keyword specifies an option on the first
menu. This keyword cannot be used more than once.
Programmer response
Retry the command with only a single OPT keyword.
ISPD216 Option too long - Maximum option
length supported is 24 characters.
Explanation
The OPTION value has a length limit and this limit has
been exceeded.
Programmer response
Use a shorter OPTION length.
ISPD217 Initial CLIST RC > 4 - The initially
invoked CLIST ended with a return
code = aaaaaaaa.
Explanation
A CLIST or REXX procedure was selected ending with
a return code that was not 0 or 4. These are the only
valid values.
Programmer response
If you wish to return a value to the calling program or
procedure, consider using the ZISPFRC variable. See
z/OS ISPF Dialog Developer's Guide and Reference for
details.
ISPD218 Initial program RC > 4 - The
initially invoked module ended
with a return code = aaaaaaaa.
ISPF messages starting with ISP
96  z/OS: z/OS ISPF Messages and Codes

## Page 117

Explanation
A program module was selected with a return code
value other than 0 or 4. These are the only valid
values.
Programmer response
If you wish to set a return code for the program or
procedure, consider using the ZISPFRC variable to
return this value. See z/OS ISPF Dialog Developer's
Guide and Reference for details.
ISPD219 Required parm missing - A select
parameter requires one of PGM,
CMD, or PANEL.
Explanation
A required SELECT parameter was omitted. If the OPT
parameter was specified, then PANEL was omitted.
If the LANG parameter was specified, then CMD was
omitted. If the PARM parameter was specified, then
PGM was omitted.
Programmer response
Add the required parameter and retry the request.
ISPD220 CLIST name is invalid - 'aaaaaaaa'
- name greater than 8 characters.
Explanation
The CLIST name specified in the CMD parameter
cannot be longer than 8 characters.
Programmer response
Correct the CLIST name so that it is less than or equal
to 8 characters.
ISPD221 Specified option invalid -
'aaaaaaaa' - contains invalid OPT
selection.
Explanation
The selected option is invalid.
User response
Contact the responsible programmer.
Programmer response
Correct the option selected with the SELECT service.
ISPD222 Command not allowed -
'aaaaaaaa' not allowed from
command line or nested in SELECT
CMD request.
Explanation
A nested command, probably ISPEXEC, was found
within an ISPEXEC command request.
Programmer response
Avoid the use of nested ISPEXEC commands.
ISPD223 Invalid command - Command
'aaaaaaaa' not found or contains
invalid syntax.
Explanation
The command was not found, was otherwise not
executable, or the exit routine returned an invalid
return code (0, 4, or 16).
Programmer response
Verify that the command is in the assumed library,
that it is executable, and that any user exit routine for
commands is returning a valid return code.
ISPD224 Invalid command - Command
name 'aaaaaaaa' contains invalid
syntax.
Explanation
The command name is invalid. Command names must
start with an alphabetic character, or @, #, or $. The
remaining characters must be alphanumeric. In MVS,
the remaining characters (but not the initial character)
can be X'C0'(left brace).
Programmer response
Correct the command name syntax to conform to
these rules.
ISPD226 Invalid command - Commands
such as LOGON, LOGOFF, SPF,
ISPF, etc. are not permitted.
Explanation
Special commands that may harm the ISPF
environment are not permitted. These commands
include LOGON, LOGOFF, SPF, ISPF, test authorized
commands, and commands invoking an authorized
program. CLISTs cannot invoke these commands
either.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  97

## Page 118

Programmer response
Remove the unauthorized command and continue.
ISPD228 Invalid command - The entry
for this command in the ISPF
TSO Command Table (ISPTCM)
indicates it cannot be run when
ISPF has been invoked from a web
client.
Explanation
Commands that cause problems or fail when ISPF is
running for a web client can be defined in the ISPF TSO
Command Table (ISPTCM) as being ineligible to run in
this environment.
System programmer response
Check the ISPF TSO Command Table (ISPTCM)
definition to see if this command is defined as being
ineligible for the web client environment.
ISPD230 Recursion error - Recursive use of
an ISPF function is not allowed.
Explanation
A non-recursive ISPF function was already active and
an attempt was made to link to another program. This
is not allowed.
Programmer response
Correct the application so that ISPF is not entered
recursively.
ISPD231 PDF subs load error - Unable to
load the subroutines for the PDF
component of ISPF.
Explanation
This message is self explanatory.
ISPD232 Insufficient storage - Insufficient
storage to load the subroutines for
the PDF component of ISPF.
Explanation
Not enough main storage was available to load PDF.
Programmer response
Increase the region size for ISPF as necessary.
ISPD233 Invalid application ID -
Application ID 'aaaaaaaa' is
reserved for system use by the
dialog manager.
Explanation
Any application ID that starts with ISP and whose
fourth character is non-blank is an invalid ID.
Programmer response
Use an application ID that does not violate the rule in
the message explanation.
ISPD234 Invalid application ID -
Application ID 'aaaaaaaa' starts
with or contains an invalid
character.
Explanation
The application ID must start with an alphabetic
character or with @, #, or $ and the remaining
characters must be alphanumeric. The MVS X'C0'(left
brace) character may be used in any character position
except the first.
Programmer response
Correct the application ID to conform to the rules in
the message explanation.
ISPD235 Select string truncated -
Substituted select string exceeded
allowable length of 32767.
Explanation
The ISPF command text length exceeded 32 767 bytes
after substitution of variables.
Programmer response
Correct the substituted variables so the command text
is less than 32 767 bytes after substitution.
ISPD236 LANG parm invalid - 'aaaaaaaa'
specified in LANG parameter is not
valid.
Explanation
The value of the LANG parameter must be APL,
COBOL, PL1, or PLI. The value specified was not valid.
Programmer response
Change the command to use one of the allowed LANG
values.
ISPF messages starting with ISP
98  z/OS: z/OS ISPF Messages and Codes

## Page 119

ISPD237 Language parameter error -
Syntax error in language
parameter value 'aaaaaaaa'.
Explanation
The language parameter value had a syntax error.
There may be extra characters after the language
name, embedded blanks in the name, or an invalid
delimiter.
Programmer response
Correct the language parameter to remove the syntax
error.
ISPD239 Language size error - Language
size contains nonnumeric
characters or too many digits.
Explanation
The language size parameter has a nonnumeric
character or has more than 8 digits.
Programmer response
Correct the language size parameter.
ISPD240 Enter option - Enter one of the
listed options.
Explanation
No option values were entered on a menu panel. This
applies to all menu panels except the primary option
menu.
User response
Key one of the options highlighted on the menu panel
and press Enter.
ISPD241 Invalid option - The option that
was entered was not valid.
Explanation
The option entered was not one of the options defined
in the menu panel. The menu panel definition replaced
an invalid option selection with a "?" character which
caused this message to display.
User response
Enter a valid option value for this menu.
ISPD242 Nesting limit exceeded - The
nesting limit for dialog EXECs in
user area has been exceeded.
Explanation
This message is self explanatory.
ISPD243 EXEC not found - The EXEC named
'aaaaaaaa' was not found in the
search order.
Explanation
This message is self explanatory.
ISPD244 Panel definition error - 100 panels
processed. No return key stop
found. ZPARENT causing loop.
Explanation
After executing the RETURN command, ISPF tries to
display the primary option menu in the hierarchy or
explicit panel chain. If more than 100 panels are in
the chain, ISPF assumes that there is an error in the
specification of ZPARENT that is causing an infinite
loop in the panel chain. Processing stops.
Programmer response
Look for errors in the specification of ZPARENT in
panel definitions or in the application.
ISPD245 Invalid logo panel name -
'aaaaaaaa' - name greater than 8
characters.
Explanation
The LOGO keyword on the ISPSTART command
specifies a panel name with a length greater than 8
characters. No more than 8 characters are allowed.
Programmer response
Correct the LOGO keyword value to use a shorter panel
name.
ISPD246 Keyword conflict - BARRIER and
NEST are only allowed with the
CMD keyword.
Explanation
The BARRIER and NEST keywords are only valid when
coded with the CMD keyword.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  99

## Page 120

Programmer response
Remove the BARRIER or NEST keyword unless a CMD
function is being invoked. These keywords are only
valid when used with the CMD parameter.
ISPD247 Wrong environment - BARRIER
and NEST keywords are only valid
in the TSO environment.
Explanation
Use BARRIER and NEST only in a TSO environment.
Programmer response
Only use BARRIER and NEST in a TSO environment.
ISPD248 BLDL error - Error processing
LIBDEF search for program
aaaaaaaa. BLDL return code =
bbbbbbbb..
Explanation
A LIBDEF was specified for ISPLLIB, and BLDL
encountered an error trying to locate a program while
invoking the SELECT service. A return code of 8
normally indicates an I/O error.
System programmer response
Refer to the appropriate system documentation for an
explanation of the BLDL macro return codes.
User response
Contact your system programmer.
ISPD250 Invocation error - System error
encountered invoking authorized
command 'aaaaaaaa'.
Explanation
ISPF cannot invoke an authorized command, such as
LOGON. It is also possible that the TSO command start
exit routine rejected the command.
Programmer response
Avoid usage of authorized commands in ISPF.
ISPD251 Language qualifier error - The only
value valid for the APL language is
START.
Explanation
Some value other than START was specified for the
APL language qualifier. Or START was followed by
something else, which is invalid.
Programmer response
Correct the syntax of the APL language qualifier.
ISPD252 Invalid PGM name - Load of PGM
'aaaaaaaa' failed.
Explanation
This message is self explanatory.
ISPD253 PASSLIB spec invalid - NEWAPPL
must be specified to specify
PASSLIB.
Explanation
The PASSLIB keyword was used but NEWAPPL was
omitted.
Programmer response
Correct the syntax and retry the command.
ISPD254 Storage release error -
Error occurred when releasing
application library blocks.
Explanation
A severe error occurred while processing a FREEMAIN
of LIBDEF control blocks.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPD255 Deallocation failure - Deallocation
failure - check allocated data sets.
Explanation
Deallocation of LIBDEF data sets failed during
termination of ISPEXEC.
System programmer response
Contact IBM support.
ISPF messages starting with ISP
100  z/OS: z/OS ISPF Messages and Codes

## Page 121

User response
Contact your system programmer.
ISPD256 Authorization failure - Request
rejected by exit routine for select
service start.
Explanation
The exit routine rejected the request because it lacked
authority to use the exit.
Programmer response
Correct the authorization failure.
ISPD257 Severe error - Select service start
exit routine returned invalid code
aaaaaaaa
Explanation
The select service start exit routine returned an
unexpected return code, which is shown in the
message.
Programmer response
Examine the exit routine to see where the unexpected
return code was generated.
ISPD258 Invocation error - System error
encountered invoking command
'aaaaaaaa'.
Explanation
Routine IKJTBLS abended while starting this
command.
Programmer response
Examine the abend information to determine the
cause of the abend.
ISPD270 Conflicting parameters - DCSS
conflicts with another parameter.
Explanation
This message is self explanatory.
ISPD271 Storage error - A storage error
occurred in the SELECT DCSS
service.
Explanation
This message is self explanatory.
ISPD272 DCSS name too long. - The
segment name 'aaaaaaaa' is
greater than 8 characters.
Explanation
This message is self explanatory.
ISPD273 Wrong environment - The SELECT
DCSS service is only valid in a CMS
environment.
Explanation
This message is self explanatory.
ISPD274 Conflicting parameters - DCSS or
PGM was specified more than
once.
Explanation
This message is self explanatory.
ISPD275 IKJADTAB interface error - RC
= 'aaaaaaaa' from TSO routine
IKJADTAB, function = 'bbbbbbbb'.
Explanation
During NEWAPPL processing of the SELECT service,
TSO routine IKJADTAB was invoked to set up or free
an ALTLIB table for the new select level. IKJADTAB
set return code aaaaaaaa which is an internal error.
The service function bbbbbbbb is what was passed to
IKJADTAB.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPD276 Interface error - RC = 'aaaaaaaa'
from TSO routine IRXSTK, function
= 'dropterm'
Explanation
During NEWAPPL processing for the SELECT service,
TSO routine IRXSTK is called to remove a barrier
('DROPTERM') from the REXX data stack. IRXSTK set
return code aaaaaaaa which is an internal error.
System programmer response
Contact IBM support.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  101

## Page 122

User response
Contact your system programmer.
ISPD277 Interface error - RC = 'aaaaaaaa'
from TSO routine IRXSTK, function
= 'markterm'
Explanation
During NEWAPPL processing for the SELECT service,
routine IRXSTK is called to place a barrier
('MARKTERM') on the REXX data stack. IRXSTK set
return code aaaaaaaa which is an internal error.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPD278 Invalid screen name - A screen
name must be more than one
character, satisfy the rules for
a member name, and cannot be
LIST, PREV or NEXT.
Explanation
A screen name must contain from 2 to 8 alphanumeric
characters and satisfy the rules for a member name. It
cannot be LIST, PREV or NEXT.
Programmer response
Correct the screen name specified on the SELECT
service.
ISPD280 Invalid CONTROL parm - An
attention interrupt has already
been established for this session.
Explanation
This message is self explanatory.
ISPD281 Invalid CONTROL parm - There is
no current attention to CANCEL.
Explanation
This message is self explanatory.
ISPD282 Invalid environment - This service
does not exist in this environment.
Explanation
This message is self explanatory.
ISPD283 Invalid interface - This service can
only be invoked from the module
interface.
Explanation
This message is self explanatory.
ISPD284 Invalid CONTROL parm -
'aaaaaaaa' is invalid option for
CONTROL ATTN service request.
Explanation
This message is self explanatory.
ISPD285 Missing CONTROL parm - A
required parameter missing from
CONTROL ATTN service request.
Explanation
This message is self explanatory.
ISPD286 Invalid PFSHOW parm - Valid
PFSHOW parameters are ON, OFF,
TAILOR, or blank.
Explanation
An incorrect parameter was specified on the PFSHOW
command. Correct parameters are ON, OFF, TAILOR,
or no parameter.
User response
Correct the parameter and retry the command.
ISPD287 Key display forced off - Current
application does not allow ISPF to
display function keys.
Explanation
The application that is running has set the ZPFCTL
system variable so that you cannot use PFSHOW to
view the function key area.
User response
Try to use the application to do what you want. For
example, the application may allow you to select a
Help function on the screen rather than by pressing a
function key.
ISPD288 Key display forced on - Current
application does not allow
ISPF messages starting with ISP
102  z/OS: z/OS ISPF Messages and Codes

## Page 123

function key display to be
removed.
Explanation
The application that is running has set the ZPFCTL
system variable so that you cannot use PFSHOW to
remove the function key area.
User response
This application is dependent on function keys.
Contact your system programmer for application
information.
ISPD289 Mismatched quotes - The string
contains an uneven number of
quotes
Explanation
Unmatched quotes were found in a parenthesized
keyword value for an ISPF service request.
Programmer response
Correct the keyword value so there are no unmatched
quotation marks.
ISPD290 Invalid CONTROL parm -
'aaaaaaaa' is invalid as request
type parameter to the CONTROL
service.
Explanation
The CONTROL service has been called with a
parameter that is not valid. The valid request
type parameters for the CONTROL service in MVS
are DISPLAY, NONDISPL, ERRORS, SPLIT, NOCMD,
SUBTASK, TSOGUI, REFLIST, LE, and PASSTHRU.
Programmer response
Correct the CONTROL parameter and retry.
ISPD291 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL ERRORS service
request.
Explanation
The CONTROL ERRORS service has been called with an
option that is not valid. The valid options are CANCEL
and RETURN. CANCEL is the default.
Programmer response
Specify a valid option for CONTROL ERRORS.
ISPD292 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL DISPLAY service
request.
Explanation
The CONTROL DISPLAY service request has been
called with an option that is not valid. The valid options
are LOCK, LINE, SM, REFRESH, SAVE, RESTORE, and
ALLVALID.
Programmer response
Change aaaaaaaa to one of the options listed.
ISPD293 Invalid CONTROL parm - The
start line-number is invalid
for CONTROL DISPLAY service
request.
Explanation
The starting line number specified with the CONTROL
DISPLAY service request is invalid, probably because it
is a negative number.
Programmer response
Specify a valid start line number.
ISPD294 Missing CONTROL parm - A
required parameter is missing
from the CONTROL service
request.
Explanation
The CONTROL service request requires a parameter
but none was specified. Valid parameters are DISPLAY,
NONDISPL, ERRORS, SPLIT, NOCMD, SUBTASK,
TSOGUI, REFLIST, LE, and PASSTHRU. Most of these
parameters require other parameters which are
described in z/OS ISPF Services Guide.
Programmer response
Supply a valid parameter for the CONTROL request.
ISPD295 Missing CONTROL parm - A
required parameter missing
from CONTROL DISPLAY service
request.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  103

## Page 124

Explanation
The CONTROL DISPLAY service request requires
another parameter. Valid parameters are LOCK, LINE,
SM, REFRESH, SAVE, RESTORE, and ALLVALID. One of
these must be specified.
Programmer response
Supply a valid parameter for CONTROL DISPLAY.
ISPD296 Invalid CONTROL parm -
'aaaaaaaa' is invalid option for
CONTROL ABEND service request.
Explanation
In CMS, the CONTROL ABEND service request requires
another parameter, either ON or OFF. Another value
aaaaaaaa was specified, which is incorrect.
Programmer response
Change aaaaaaaa to ON or OFF.
ISPD297 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL CRETURN service
request.
Explanation
In CMS, the CONTROL RETURN service request
requires another parameter, either CSTART or CSTOP.
An invalid value aaaaaaaa was used instead.
Programmer response
Change aaaaaaaa to CSTART or CSTOP.
ISPD298 Invalid CONTROL parm -
'aaaaaaaa' is invalid option
for CONTROL NONDISPL service
request.
Explanation
The CONTROL NONDISPL service request requires
another parameter, which can be either ENTER or END.
An incorrect value aaaaaaaa was used instead.
Programmer response
Change aaaaaaaa to ENTER or END.
ISPD299 SPLIT not supported - The ISPF
batch environment does not
support SPLIT.
Explanation
The CONTROL SPLIT ENABLE service request is not
allowed while running in the ISPF batch display
environment.
Programmer response
Avoid using this request if the application is to be run
in batch.
ISPD301 Invalid FKA parm - Valid FKA
parameters are ON, OFF, SHORT or
blank.
Explanation
The parameter for the FKA command must be ON,
OFF, SHORT, or blank. Something else was used, which
is incorrect.
User response
Correct the parameter to the FKA command and retry.
ISPD304 Missing CONTROL parm - A
required parameter is missing
from the CONTROL PASSTHRU
service request.
Explanation
The CONTROL PASSTHRU service request is missing a
required parameter. The valid parameters are:
• LRSCROLL PASQUERY
• LRSCROLL PASON
• LRSCROLL PASOFF
System action:
The CONTROL PASSTHRU service request is not
processed.
Programmer response:
Supply a valid parameter for the CONTROL PASSTHRU
service request.
ISPD305 Invalid CONTROL parm -
'aaaaaaaa' is not a valid
parameter for the CONTROL
PASSTHRU service request.
Explanation
The CONTROL PASSTHRU service request includes a
parameter that is not valid. The valid parameters are:
• LRSCROLL PASQUERY
• LRSCROLL PASON
• LRSCROLL PASOFF
ISPF messages starting with ISP
104  z/OS: z/OS ISPF Messages and Codes

## Page 125

System action:
The CONTROL PASSTHRU service request is not
processed.
Programmer response:
Supply a valid parameter for the CONTROL PASSTHRU
service request.
ISPD801 TBOPEN failure - Return code
aaaaaaaa from TBOPEN for table
"bbbbbbbb". (1+)
Explanation
The ISPPUP panel update utility could not open the
input table using the TBOPEN service due to a severe
error. The return code value and the table name are
given in the message. Processing is terminated.
System programmer response
Use the return code value to determine the reason for
the severe error.
ISPD802 Table not found - Table
"aaaaaaaa" not found in ISPTLIB,
or ISPTLIB not allocated. (1+)
Explanation
The input table for the ISPPUP panel update utility was
not found. This could be because the table name was
misspelled, or the table library was not allocated. The
return code from TBOPEN is 8 or 16. Processing is
terminated.
System programmer response
Make sure the desired table is in the table library and
that the correct library is allocated.
ISPD803 ISPPLIB error - The panel input
library ISPPLIB is not open.
(aaaaaaaa+)
Explanation
The panel being updated for the ISPPUP panel update
utility could not be found because the ISPPLIB panel
library is not open. The modified panel output library
(ISPPMOD) was searched and the panel was not there.
Processing is terminated.
System programmer response
Make sure that the proper panel input library is
allocated before running ISPPUP.
ISPD804 Panel not found - Panel
"aaaaaaaa" not found in
panel input library "ISPPLIB".
(bbbbbbbb)
Explanation
The ISPPUP panel update utility could not find the
input panel aaaaaaaa in the panel input library
ISPPLIB. Processing continues with the next panel.
System programmer response
Correct the name of the input panel, or make sure it is
in a data set allocated to ISPPLIB.
ISPD805 BLDL error - I/O error during
BLDL for ddname "aaaaaaaa".
(bbbbbbbb)
Explanation
The panel update utility, ISPPUP, had a severe error
from the BLDL request when trying to find a panel
member in the ISPPLIB or ISPPMOD library. The return
code in the message indicates a severe error for BLDL.
Processing is terminated.
System programmer response
Analyze the reason why the BLDL failed. Contact IBM
support if further help is needed.
ISPD806 aaaaaaaa error - The panel output
library "aaaaaaaa" has not been
allocated. (1+)
Explanation
The ISPPUP panel update utility could not continue
because the ISPPMOD panel output library was not
allocated. Processing is terminated.
System programmer response
Make sure the correct library is allocated to ISPPMOD
before running ISPPUP.
ISPD807 No asterisk found - The "ZSEL
=" in panel "aaaaaaaa" has no
asterisk translation. (bbbbbbbb)
Explanation
While the selection panel update utility, ISPPUP, was
updating panel aaaaaaaa, no asterisk (*) translation
was found in the TRANS statement. Processing
continues with the next panel.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  105

## Page 126

System programmer response
It is desirable to have an asterisk selection in all
selection panel TRANS statements.
ISPD808 Model section found - Panel
"aaaaaaaa" is an invalid selection
panel (model section found).
(bbbbbbbb)
Explanation
A )MODEL section should be used only in table
display panels. The selection panel update utility
(ISPPUP) found a )MODEL section in a selection panel.
Processing continues with the next panel.
System programmer response
Check to make sure this panel is a selection panel.
ISPD809 GETMAIN error - Return
code aaaaaaaa from common
subroutine CSM. (1+)
Explanation
While the panel update utility (ISPPUP) was initializing,
it attempted to get storage for a panel buffer or
internal control blocks. The GETMAIN failed and
processing has terminated.
System programmer response
Check to make sure adequate main storage is
allocated to the job.
ISPD810 Read error - I/O error
reading panel "aaaaaaaa" from
"bbbbbbbb". (cccccccc)
Explanation
An I/O error occurred in the Select Panel Update
Utility, ISPPUP, while reading panel definition
aaaaaaaa from the library file defined by bbbbbbbb.
Processing has terminated.
System programmer response
Determine the reason for the I/O error, if possible.
Contact IBM support for further help if needed.
ISPD811 Panel too large - Definition for
panel "aaaaaaaa" will exceed
bbbbbbbb lines. (cccccccc)
Explanation
The Selection Panel Update utility cannot handle a
selection panel that has more than bbbbbbbb lines.
Processing continues with the next panel.
System programmer response
Remove unnecessary lines from the panel and run the
utility again.
ISPD812 Body size error - Body of panel
"aaaaaaaa" is empty or larger
than bbbbbbbb lines. (cccccccc)
Explanation
The Selection Panel Update utility, ISPPUP, was
processing panel aaaaaaaa which has either zero
lines or more than bbbbbbbb lines in the body section.
Processing continues with the next panel.
System programmer response
Correct the body section of the panel so that it can be
handled by the utility, or update the panel by hand.
ISPD813 Missing )END stmt - End of file
found before )END statement for
panel "aaaaaaaa". (bbbbbbbb)
Explanation
The Selection Panel Update utility, ISPPUP, reached
the end of the current panel, aaaaaaaa, unexpectedly.
The utility expects an )END statement to complete the
panel definition. Processing continues with the next
panel.
System programmer response
Correct the incomplete panel so that it contains
an )END statement.
ISPD814 No PROC section - "aaaaaaaa"
is an invalid selection panel (no
PROC section found). (bbbbbbbb)
Explanation
The Selection Panel Update utility, ISPPUP, did not find
a )PROC section in panel aaaaaaaa. Selection panels
must have a )PROC section. Processing continues with
the next panel.
System programmer response
Correct the panel so that it has a )PROC section.
ISPF messages starting with ISP
106  z/OS: z/OS ISPF Messages and Codes

## Page 127

ISPD815 No ZSEL assignment - No ZSEL
assignment found in PROC section
of panel "aaaaaaaa". (bbbbbbbb)
Explanation
The Selection Panel Update utility, ISPPUP, did not find
any statement in the )PROC section of panel aaaaaaaa
that assigns a value to ZSEL. Processing continues
with the next panel.
System programmer response
Ensure that all panels processed by ISPPUP have ZSEL
assignment statements.
ISPD816 Invalid option - Existing option
"aaaaaaaa" in panel "bbbbbbbb"
is too long. (cccccccc)
Explanation
The Selection Panel Update utility, ISPPUP, has found
an option value in an input panel (panel being
updated) that is too long. Processing continues with
the next panel.
System programmer response
Make sure the option value is no more than 4
characters.
ISPD817 Long select string - Select string
specified for panel "aaaaaaaa" is
too long. (bbbbbbbb)
Explanation
The Selection Panel Update utility, ISPPUP, tried to
add the selection string text to the )PROC section of a
panel but the text would not fit into the logical record
(80 characters). The limit on selection string text is 30
characters. Processing continues with the next panel.
System programmer response
Shorten the selection panel text.
ISPD818 Duplicate option - Option
"aaaaaaaa" already exists in
panel "bbbbbbbb". (cccccccc)
Explanation
This message is self explanatory.
ISPD819 Invalid option - Option
"aaaaaaaa" for "bbbbbbbb" has
blanks, commas, or periods.
(cccccccc)
Explanation
The Selection Panel Update utility, ISPPUP, found
the OPTION field in the input table to have a
faulty value. The field is faulty because it contains
an embedded blank, comma, or period character.
Processing continues with the next panel.
System programmer response
Correct the option field in the input table that drives
the ISPPUP.
ISPD820 Invalid keyword - "aaaaaaaa" for
"bbbbbbbb" has blanks, commas,
or periods. (cccccccc)
Explanation
This message is self explanatory.
ISPD821 Write error - I/O error writing
panel "aaaaaaaa" to "bbbbbbbb".
(cccccccc+)
Explanation
This message is self explanatory.
ISPD822 STOW error - Rc aaaaaaaa,
reason code bbbbbbbb for member
"cccccccc". (dddddddd+)
Explanation
This message is self explanatory.
ISPD823 FREEMAIN error - Return code
aaaaaaaa from common sub CSM.
(all rows processed.)
Explanation
This message is self explanatory.
ISPD824 Cdsn failure - Return code
aaaaaaaa from common sub CDSN
for "bbbbbbbb". (1+)
Explanation
This message is self explanatory.
ISPD825 Open error - Return code
aaaaaaaa from common sub CDO
for "bbbbbbbb". (1+)
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  107

## Page 128

Explanation
This message is self explanatory.
ISPD826 Invalid data - Invalid data in row
aaaaaaaa of table "bbbbbbbb".
row ignored. (aaaaaaaa)
Explanation
This message is self explanatory.
ISPD827 Invalid text count - aaaaaaaa text
lines specified; must be between 0
and bbbbbbbb..(cccccccc)
Explanation
This message is self explanatory.
ISPD828 Missing keyword - Keyflag set but
no keyword given in row aaaaaaaa
of table "bbbbbbbb".(aaaaaaaa)
Explanation
This message is self explanatory.
ISPD829 Blank panel name - Panel
name in row aaaaaaaa of table
"bbbbbbbb" is blank. (aaaaaaaa)
Explanation
This message is self explanatory.
ISPD830 aaaaaaaa error - The sequential
input library "aaaaaaaa" has not
been allocated. (1+)
Explanation
This message is self explanatory.
ISPD831 aaaaaaaa error - The sequential
output library "aaaaaaaa" has not
been allocated. (1+)
Explanation
This message is self explanatory.
ISPD832 Seq write error - I/O error
writing panel "aaaaaaaa" data
to"bbbbbbbb". (cccccccc+)
Explanation
This message is self explanatory.
ISPD833 Keyword required - Keyword
string required for file input was
not found. (aaaaaaaa)
Explanation
This message is self explanatory.
ISPD834 No panel name keyword - Input
file does not start with PANEL
keyword.
Explanation
This message is self explanatory.
ISPD835 Table failure - Create of temporary
table failed.
Explanation
This message is self explanatory.
ISPD836 DMSCSL invocation error - Routine
DMSCSL return code = 'aaaaaaaa'.
Contact your system programmer.
Explanation
This message is self explanatory.
ISPE000 Invalid return code (aaaaaaaa)
received from ISPF reserve exit.
Explanation
The Reserve installation exit has returned a return
code other than 0 or 16.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the Reserve installation exit, then
determine the reason the exit is returning a return
code other than 0.
User response
Contact your system programmer.
ISPE001 ISPF reserve exit, RC=16, resource
not available.
Explanation
The requested resource is not available. This is
determined by the Reserve installation exit.
ISPF messages starting with ISP
108  z/OS: z/OS ISPF Messages and Codes

## Page 129

System programmer response
See ISPF Planning and Customizing to determine
which exit is the Reserve installation exit, then
determine which resource is not available.
User response
Contact your system programmer.
ISPE002 Severe error - Display service exit
routine returned invalid return
code aaaaaaaa.
Explanation
The Display Service installation exit routine has
returned a return code other than 0 or 4.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the DISPLAY installation exit, then
determine why the exit returned a return code other
than 0 or 4.
User response
Contact your system programmer.
ISPE003 Authorization failure - Command
'aaaaaaaa' rejected by exit routine
for TSO command start.
Explanation
A TSO command has been rejected by the TSO
Command installation exit.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the TSO Command installation exit then
determine why the exit rejected the command.
User response
Contact your system programmer for a list of
commands that are not allowed by your TSO
Command installation exit.
ISPE004 TSO command error - Exit routine
for command 'aaaaaaaa' returned
invalid code bbbbbbbb.
Explanation
The TSO Command installation exit has returned a
return code other than 0, 4, or 16.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the TSO Command installation exit, then
determine why the exit returned an invalid return code.
User response
Contact your system programmer.
ISPE005 Resource not available - Reserve
exit, RC=16, resource not
available.
Explanation
The requested resource is not available, as determined
by the Reserve installation exit.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the Reserve installation exit, then
determine the resource that is not available.
User response
Contact your system programmer.
ISPE006 Invalid return code - Invalid return
code received from reserve exit -
valid RC=0,16.
Explanation
The Reserve installation exit has returned a return
code other than 0 or 16.
System programmer response
See ISPF Planning and Customizing to determine
which exit is the Reserve installation exit, then
determine the reason for the exit returning a return
code other than 0 or 16.
User response
Contact your system programmer.
ISPE007 Severe error - Panel input exit
routine aaaaaaaa returned invalid
return code bbbbbbbb for panel
cccccccc.
Explanation
The panel input exit routine has returned a return code
other than 0, 2, 4, or 8. The return code could have
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  109

## Page 130

been 20 for a severe error or some other return code
that was not recognized by ISPF.
User response
Contact your system programmer.
ISPE101 ENVIRON parm error - aaaaaaaa
is an invalid parameter for
ENVIRON command
Explanation
This message is self explanatory.
ISPE102 ENVIRON aaaaaaaa error -
bbbbbbbb is an invalid parameter
for ENVIRON aaaaaaaa command.
Explanation
The second parameter used with the ENVIRON
command is invalid.
User response
Enter one of the correct parameters as shown here:
ENVIRON
ENBLDUMP
ON
OFF
TERMTRAC
ON
ERROR
DUMP
OFF
TERMSTAT
QUERY
ISPE103 Ddname not specified - A ddname
must be specified for ENVIRON
TERMTRAC aaaaaaaa.
Explanation
For TERMTRAC ON, ERROR, or DUMP, a ddname is
required for the output of the terminal tracing.
User response
Enter a ddname using the ENVIRON panel selection.
ISPE104 TERMTRAC not activated - Storage
could not be obtained for the
TERMTRAC buffer.
Explanation
Insufficient storage is available to obtain a buffer for
TERMTRAC.
Programmer response
Allocate additional storage and retry the TERMTRAC
command.
ISPE105 TERMTRAC activated - TERMTRAC
has been activated.
Explanation
Terminal tracing is active. Output will be written to the
requested ddname.
ISPE106 TERMTRAC not active - ENVIRON
TERMTRAC dump requested when
TERMTRAC is not active.
Explanation
TERMTRAC with DUMP parameter cannot be executed
until TERMTRAC has been activated using the
ENVIRON TERMTRAC ON command.
Programmer response
Start TERMTRAC by issuing the ENVIRON TERMTRAC
command. See this example.
ENVIRON TERMTRAC ON
ISPE107 TERMTRAC turned off - TERMTRAC
has been turned off.
Explanation
This is an informational message.
ISPE108 ENVIRON ENBLDUMP on -
ENVIRON ENBLDUMP has been
turned on.
Explanation
This is an informational message.
ISPE109 ENVIRON ENBLDUMP off -
ENVIRON ENBLDUMP has been
turned off.
Explanation
This is an informational message.
ISPE110 Invalid ENBLDUMP value - An
ENBLDUMP value of ON or OFF
must be specified.
ISPF messages starting with ISP
110  z/OS: z/OS ISPF Messages and Codes

## Page 131

Explanation
To enable a dump for subtask abend when not in
ISPF TEST mode, set ENBLDUMP to ON. To disable the
dump, set ENBLDUMP to OFF.
User response
Enter one of the correct values.
ISPE111 Invalid TERMTRAC value - A
TERMTRAC value of ON, OFF, or
ERROR must be specified.
Explanation
To turn terminal tracing on, specify ON. To turn
terminal tracing off, specify OFF. To turn terminal
tracing on and initiate an MVS SNAP dump if a TPUT
or TGET error occurs, specify ERROR.
User response
Enter one of the correct values.
ISPE112 Invalid ddname - Ddname must be
alphanumeric with first character
alphabetic.
Explanation
The ddname must follow the given naming
conventions.
User response
Enter an alphanumeric ddname with an alphabetic first
character.
ISPE113 Invalid TERMSTAT value - YES or
QUERY must be specified to invoke
ENVIRON TERMSTAT.
Explanation
The value entered for TERMSTAT is invalid.
User response
Enter one of the correct values.
Programmer response
Enter YES or QUERY to invoke ENVIRON TERMSTAT.
ISPE114 aaaaaaaa = bbbbbbbb
Explanation
This is an informational message.
ISPE115 aaaaaaaa = bbbbbbbb - x'cccccccc'
Explanation
This is an informational message.
ISPE116 aaaaaaaa = x'bbbbbbbb'
Explanation
This is an informational message.
ISPE117 aaaaaaaa
Explanation
This is an informational message.
ISPE118 ENVIRON parm error - The
parameter is longer than 8
characters.
Explanation
A parameter entered with the ENVIRON command
exceeds 8 characters.
User response
Enter the ENVIRON command using correct
parameters.
ISPE200 Command not allowed - Recursive
entry to ISPF is not permitted.
Explanation
A second ISPF session may not be started while an
ISPF session is active.
User response
Use the current ISPF session, or use ISPF split screen
to start another ISPF session.
ISPE203 Parameter missing - A parameter
is required with the TSO
command.
Explanation
It is invalid to specify TSO without following it with a
valid TSO command.
User response
Enter a valid TSO command after the word TSO.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  111

## Page 132

ISPE208 Command not allowed - This
command is not intended to be
executed from option 6.
Explanation
Commands containing the ISPF NONDISPL keyword
cannot be executed from option 6.
User response
Correct the command and then reenter.
ISPE300 SHRPROF unavailable - The
SHRPROF command is unavailable
when the Shared Profile support is
inactive.
Explanation
This message is self explanatory.
ISPE301 SHRPROF parm error - A
parameter is longer than 8
characters.
Explanation
A parameter entered with the SHRPROF command
exceeds 8 characters.
User response
Enter the SHRPROF command using correct
parameters.
ISPE302 SHRPROF parm error - Too
many parameters specified on the
SHRPROF command
Explanation
Too many parameters have been specified on the
SHRPROF command.
User response
Enter the SHRPROF command using correct
parameters as shown in this list: SHRPROF RESET
WAIT [n] RETRY [n] PROMPT | NOPROMPT CONFLICT
SYSTEM | ISPF | APPLID | REFLIST | EDIT | BATCH |
OTHER [Keep | Discard | Prompt]
ISPE303 SHRPROF aaaaaaaa error - Too
many parameters specified on the
SHRPROF aaaaaaaa command.
Explanation
Too many parameters have been specified on the
SHRPROF command.
User response
Enter the SHRPROF command using correct
parameters as shown in this list: SHRPROF RESET
WAIT [n] RETRY [n] PROMPT | NOPROMPT CONFLICT
SYSTEM | ISPF | APPLID | REFLIST | EDIT | BATCH |
OTHER [Keep | Discard | Prompt]
ISPE304 SHRPROF WAIT error - aaaaaaaa
must be numeric and in the range
0 to 9999.
Explanation
The second parameter used with the SHRPROF
command must be numeric and within the range 0 to
9999 inclusive.
User response
Enter the correct parameter or leave blank to default
to the configuration default.
ISPE305 SHRPROF RETRY error - aaaaaaaa
must be numeric and in the range
0 to 99.
Explanation
The second parameter used with the SHRPROF
command must be numeric and within the range 0 to
99 inclusive.
User response
Enter the correct parameter or leave blank to default
to the configuration default.
ISPE310 SHRPROF aaaaaaaa error -
Required second parameter
missing for SHRPROF aaaaaaaa
command.
Explanation
The second parameter used with the SHRPROF
command is missing.
User response
Enter one of the correct parameters as shown in this
list: SHRPROF RESET WAIT [n] RETRY [n] PROMPT
| NOPROMPT CONFLICT SYSTEM | ISPF | APPLID |
ISPF messages starting with ISP
112  z/OS: z/OS ISPF Messages and Codes

## Page 133

REFLIST | EDIT | BATCH | OTHER [Keep | Discard |
Prompt]
ISPE311 SHRPROF aaaaaaaa error -
bbbbbbbb is an invalid parameter
for SHRPROF aaaaaaaa command.
Explanation
The second parameter used with the SHRPROF
command is invalid.
User response
Enter one of the correct parameters as shown in this
list: SHRPROF RESET WAIT [n] RETRY [n] PROMPT
| NOPROMPT CONFLICT SYSTEM | ISPF | APPLID |
REFLIST | EDIT | BATCH | OTHER [Keep | Discard |
Prompt]
ISPE312 SHRPROF aaaaaaaa error -
bbbbbbbb is an invalid parameter
for SHRPROF aaaaaaaa cccccccc
command.
Explanation
The third parameter used with the SHRPROF
aaaaaaaa cccccccc command is invalid.
User response
For SHRPROF CONFLICT BATCH, the third parameter
should be either KEEP or DISCARD.
ISPE313 SHRPROF aaaaaaaa error -
bbbbbbbb is an invalid parameter
for SHRPROF aaaaaaaa cccccccc
command.
Explanation
The third parameter used with the SHRPROF
aaaaaaaa cccccccc command is invalid.
User response
For SHRPROF CONFLICT, the third parameter should
be either KEEP, DISCARD, or PROMPT.
ISPE314 SHRPROF parm error - aaaaaaaa
is an invalid parameter for
SHRPROF command
Explanation
The first parameter used with the SHRPROF command
is invalid.
User response
Enter one of the correct parameters as shown in this
list: SHRPROF RESET WAIT [n] RETRY [n] PROMPT
| NOPROMPT CONFLICT SYSTEM | ISPF | APPLID
| REFLIST | EDIT | BATCH | OTHER [Keep|Discard|
Prompt]
ISPE320 SHRPROF Settings reset - Shared
Profile settings have been reset to
ISPF configuration defaults.
Explanation
This is an informational message.
ISPE321 SHRPROF Canceled - Shared
Profile settings have been
canceled
Explanation
This is an informational message.
ISPE322 SHRPROF Settings Updated -
Shared Profile settings have been
updated
Explanation
This is an informational message.
ISPE323 SHRPROF aaaaaaaa set to
bbbbbbbb
Explanation
This is an informational message.
ISPE324 SHRPROF aaaaaaaa reset to
bbbbbbbb
Explanation
This is an informational message.
ISPE325 SHRPROF aaaaaaaa bbbbbbbb set
to cccccccc
Explanation
This is an informational message.
ISPE326 SHRPROF aaaaaaaa bbbbbbbb
reset to cccccccc
Explanation
This is an informational message.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  113

## Page 134

ISPE327 SHRPROF aaaaaaaa set -
SHRPROF aaaaaaaa has been set.
Explanation
This is an informational message.
ISPF101 Router service error - Unknown
file tailoring service.
Explanation
An incorrect File Tailoring service request was used.
Valid requests are FTOPEN, FTINCL, FTERASE, and
FTCLOSE.
Programmer response
Correct the request to use one of the valid names
shown.
ISPF102 No work file - Work file not open.
Explanation
During file tailoring, the output is written to a
temporary data set. For some reason, the temporary
data set could not be opened.
Programmer response
Check the log for other errors that explain the open
failure.
ISPF103 No imbed file - Imbed file
'aaaaaaaa' does not exist,
bbbbbbbb record-cccccccc.
Explanation
During file tailoring, an imbed statement named an
input file, aaaaaaaa, that was not found. The imbed
statement was in bbbbbbbb. The record number of the
imbed statement is cccccccc.
Programmer response
Correct the skeleton file name in the imbed statement,
or create the skeleton file named in the imbed
statement.
ISPF104 )SEL error - )ENDSEL has no
matching )SEL, aaaaaaaa record-
bbbbbbbb.
Explanation
In file tailoring, an )ENDSEL was found before
a matching )SEL statement. The control file is
aaaaaaaa, the error is found in record number
bbbbbbbb.
Programmer response
Correct the control statements so that each )ENDSEL
has a matching )SEL.
ISPF105 Output overflow - Line to be
written greater than data set
LRECL (aaaaaaaa), bbbbbbbb
record-cccccccc.
Explanation
During file tailoring, the substitution of variables has
created a record that is greater than the logical
record length (aaaaaaaa) of the output file. The name
bbbbbbbb and record number (cccccccc) of the input
file are also given.
Programmer response
Increase the logical record length of the output file, or
restructure the input file so that output record is not
exceeded due to variable substitution.
ISPF106 Control word error - Invalid
control word, aaaaaaaa record-
bbbbbbbb.
Explanation
An unexpected control word was found in the file
tailoring control file. The file name is aaaaaaaa; the
error was found at record bbbbbbbb in the file.
Programmer response
Correct the control word error in the control file.
ISPF107 Control word error - Invalid
control word parameter, aaaaaaaa
record-bbbbbbbb.
Explanation
One of the parameters in a file tailoring control
statement is invalid. The control file is aaaaaaaa;
record number bbbbbbbb is where the error was
found.
Programmer response
Correct parameter in the file tailoring control
statement.
ISPF108 Variable name error - Invalid
variable name, aaaaaaaa record-
bbbbbbbb.
ISPF messages starting with ISP
114  z/OS: z/OS ISPF Messages and Codes

## Page 135

Explanation
A file tailoring variable name is invalid, perhaps
because it is too long, or because it contains only a
single ampersand character. The tailoring input file is
aaaaaaaa; the record number is bbbbbbbb.
Programmer response
Correct the variable in record bbbbbbbb of control file
aaaaaaaa.
ISPF109 )IM error - Exceeds maximum )IM
level of 15, aaaaaaaa record-
bbbbbbbb.
Explanation
The file tailoring control file aaaaaaaa has more than
fifteen levels of imbeds (imbed within imbed). Only
fifteen are allowed. The error was discovered at record
bbbbbbbb of the control file.
Programmer response
Correct the control file so that no more than 15 imbed
levels are used.
ISPF110 aaaaaaaa. error - Exceeds
maximum )IF and )SEL levels of
32, bbbbbbbb record-cccccccc.
Explanation
In file tailoring, up to 32 levels of nesting are allowed
when doing )IF or )SEL within an )IF or )SEL. This
limit was exceeded in record cccccccc of member
bbbbbbbb.
Programmer response
Correct the file tailoring dialog so that only 32 nesting
levels are used.
ISPF111 Record missing - Continuation
card missing, aaaaaaaa record-
bbbbbbbb.
Explanation
The end of file was reached on a data file or imbedded
file, but the last record was marked as "continued"
because it had a "?" at the end. It is assumed that the
continuation record is missing and processing stops.
The error was detected at record bbbbbbbb in member
aaaaaaaa.
Programmer response
Remove the "?" character from the last record,
or supply the missing record to complete the
continuation.
ISPF112 Substitution error - Invalid cond.
sub. string, aaaaaaaa record-
bbbbbbbb.
Explanation
The conditional substitution string in record bbbbbbbb
of member aaaaaaaa has incorrect syntax.
Programmer response
See ISPF Dialog Developer's Guide and Reference
for help with the syntax. Correct the syntax of the
conditional substitution string.
ISPF113 ENQUEUE error - Error occurred on
ENQUEUE for file 'aaaaaaaa' RC-
bbbbbbbb.
Explanation
This message is self explanatory.
ISPF114 )IM error - )IM ends in wrong )SEL,
aaaaaaaa record-bbbbbbbb.
Explanation
This is an internal error in ISPF. The stack pointers
for )SEL and )IM have become inconsistent and
processing cannot continue.
Programmer response
Contact IBM support.
ISPF115 )IM error - )IM ends in wrong )DOT,
aaaaaaaa record-bbbbbbbb.
Explanation
This is an internal error in ISPF. The stack pointers
for )DOT and )IM have become inconsistent and
processing cannot continue.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  115

## Page 136

ISPF116 )SEL error - )SEL has no
matching )ENDSEL, aaaaaaaa
record-bbbbbbbb.
Explanation
A )SEL was in progress and the end of file was reached
on either the main skeleton file or an imbedded file.
ISPF was expecting to see the matching )ENDSEL
before the end of the imbed or main file and didn't
find it. The member name is aaaaaaaa and bbbbbbbb
is the record number where the error was detected.
Programmer response
Examine the file and provide the matching )ENDSEL.
ISPF117 )SEL error - )SEL ends in
wrong )DOT, aaaaaaaa record-
bbbbbbbb.
Explanation
This is an internal error in ISPF. The internal
stack pointers for )IF/)SEL and )DOT have become
inconsistent and processing cannot continue.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF118 Too many tables - Table stack is
full, aaaaaaaa record-bbbbbbbb.
Explanation
More than 4 levels of )DOT nesting was attempted
during file tailoring. Only 4 levels are allowed. The
error was detected in member aaaaaaaa while
processing record number bbbbbbbb.
Programmer response
Change the file tailoring skeleton so that no more than
4 nesting levels are used.
ISPF119 )DOT error - )ENDDOT has no
matching )DOT, aaaaaaaa record-
bbbbbbbb.
Explanation
A )ENDDOT statement was found in the file tailoring
input and there is no corresponding )DOT. )ENDDOT
must be preceded by a )DOT. The error was discovered
in member aaaaaaaa at record number bbbbbbbb.
Programmer response
Correct the file so that )ENDDOT is preceded by )DOT.
ISPF120 )DOT error - )DOT has no
matching )ENDDOT, aaaaaaaa
record-bbbbbbbb.
Explanation
File tailoring reached the end of the current imbed
or input file without finding an )ENDDOT to match a
previous )DOT. The error is in member aaaaaaaa at
record bbbbbbbb.
Programmer response
Provide an )ENDDOT statement in the same input file
or imbed to match the previous )DOT.
ISPF121 )DOT error - )DOT ends in
wrong aaaaaaaa., bbbbbbbb
record-cccccccc
Explanation
This is an internal error in ISPF. The internal pointers
for )DOT and )IF/)SEL have become inconsistent and
processing cannot continue. The error was detected in
member bbbbbbbb at record number cccccccc.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF122 Service error - RC aaaaaaaa
from bbbbbbbb, cccccccc record-
dddddddd
Explanation
An error occurred in table processing for file tailoring,
related to )DOT or )ENDDOT. The table service is
bbbbbbbb and the return code from the service is
aaaaaaaa. The error was discovered while processing
record dddddddd of member cccccccc.
Programmer response
Use the return code value to help determine the nature
of the problem.
ISPF messages starting with ISP
116  z/OS: z/OS ISPF Messages and Codes

## Page 137

ISPF123 Invalid operator - Invalid
relational operator, aaaaaaaa
record-bbbbbbbb
Explanation
A relational operator in the file tailoring )SEL statement
is invalid, or the relational expression is incomplete.
Relational operators are things like (=) or GE. See
z/OS ISPF Dialog Developer's Guide and Reference for
a description of relational expressions. The error was
found in record bbbbbbbb of member aaaaaaaa.
Programmer response
Correct the relational operator in the )SEL statement.
ISPF124 Invalid operator - Invalid
Boolean operator, aaaaaaaa
record-bbbbbbbb
Explanation
A Boolean operator was expected in the file
tailoring )SEL statement. What was found was neither
"or" nor "and". The error was found at record
bbbbbbbb in member aaaaaaaa.
Programmer response
Correct the Boolean operator in the )SEL statement.
ISPF125 Invalid operation - Invalid
operation code in aaaaaaaa,
bbbbbbbb record-cccccccc
Explanation
An arithmetic operator other than "+" (plus) or "-"
(minus) was found in the )DO, )SET, or )SETF statement
of a file tailoring file. The error is found in record
cccccccc of member bbbbbbbb.
Programmer response
Correct the )DO, )SET, or )SETF statement to use a
valid arithmetic operator.
ISPF126 )DOT error - Table being
processed via )DOT, aaaaaaaa
record-bbbbbbbb.
Explanation
In file tailoring, the same table is being processed
recursively via )DOT. The same table cannot be used
more than once in nested )DOT statements. The
error was discovered at record bbbbbbbb in member
aaaaaaaa.
Programmer response
Correct the nested )DOT statements so that no table
name is used more than once.
ISPF127 )DEFAULT error - Invalid character
in )DEFAULT, aaaaaaaa record-
bbbbbbbb.
Explanation
More than 7 characters were specified in the )DEFAULT
statement for file tailoring. Exactly 7 must be
specified. The error was found in record bbbbbbbb of
member aaaaaaaa.
Programmer response
Correct the )DEFAULT statement so that no more than
7 characters are specified.
ISPF128 )DEFAULT error - Character
missing in )DEFAULT, aaaaaaaa
record-bbbbbbbb.
Explanation
Fewer than 7 characters were specified in a )DEFAULT
statement for file tailoring. Exactly 7 characters must
be specified. The error was found in record bbbbbbbb
of member aaaaaaaa.
Programmer response
Correct the )DEFAULT statement so that exactly 7
characters are specified.
ISPF129 DEQUEUE error - Error occurred on
DEQUEUE for file 'aaaaaaaa' RC-
bbbbbbbb.
Explanation
This message is self explanatory.
ISPF130 GETMAIN error - Module
aaaaaaaa received RC-bbbbbbbb
from ISPCSM.
Explanation
An unexpected error return code appeared while
getting or freeing storage. The storage routine is
ISPCSM, the code it returned is bbbbbbbb, and
calling module is aaaaaaaa. The error occurred during
file tailoring. If the return code is 4, a conditional
GETMAIN was unsuccessful. If 8, then an invalid
request code was used, or an error was returned from
GETMAIN or FREEMAIN.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  117

## Page 138

User response
The region size may not be large enough. Check your
region size, then contact IBM support.
ISPF131 Build TFD error - Module
aaaaaaaa received RC-bbbbbbbb
from ISPCBTFD.
Explanation
An unexpected internal error occurred in ISPF while
trying to build an internal control block for file
tailoring. The return code and the module name that
received the return code are given in the message.
System programmer response
Contact IBM support.
Programmer response
If the message shows module ISPFITLR received a
return code of 8 from ISPCBTFD, ensure that all these
conditions are true:
• Both the SISPSLIB and SISPSxxx (where xxx is the
language, such as ENU) data sets are allocated to
DDNAME ISPSLIB.
• For the data sets allocated to DDNAME ISPSLIB,
the data set with the largest blocksize is first in the
concatenation.
• There is not a mix of record format VB and FB data
sets allocated to DDNAME ISPSLIB.
ISPF132 Data set name error - Module
aaaaaaaa received RC-bbbbbbbb
from ISPCDSN.
Explanation
An internal error occurred in ISPF during file tailoring.
Module aaaaaaaa called ISPCDSN to get a data set
name but ISPCDSN set return code bbbbbbbb, which
was unexpected. If the return code is 4, ISPCDSN
found no DD statement or the DDNAME in the DCB was
blank.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPF133 DEQUEUE error - Module
aaaaaaaa received RC-bbbbbbbb
from ISPCDQ.
Explanation
An internal error occurred in ISPF while trying to
dequeue a data set or member during file tailoring.
Module ISPCDQ, which is a dequeue service module,
returned unexpected return code bbbbbbbb to module
aaaaaaaa. If the return code is 4, DEQ was unable
to remove the task from wait state. If 8, DEQ got a
Resource Not Found return code. If 12, there is an
invalid input parameter.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPF134 FIND error - Module aaaaaaaa
received RC-bbbbbbbb from
ISPCFI.
Explanation
An internal error occurred in ISPF during file tailoring.
Module aaaaaaaa called ISPCFI to FIND a Partitioned
Data Set member and received an unexpected return
code, bbbbbbbb. If the return code is 4, then a
member was not found. If the return code is 8, an I/O
error occurred during BLDL.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPF135 I/O error - Unrecoverable I/O error
on aaaaaaaa.
Explanation
An input I/O error occurred during file tailoring. The
error may have occurred on the main skeleton file, on
a )DOT file, an )IM file, or while reading a temporary
file. aaaaaaaa. is the member name that was being
processed.
User response
Check the input files for obvious incompatibilities with
the ISPF recommended values, such as wrong record
length. Also make sure that variable and fixed record
types have not been mixed in the concatenation of
input data sets.
ISPF messages starting with ISP
118  z/OS: z/OS ISPF Messages and Codes

## Page 139

ISPF136 I/O error - Unrecoverable I/O error
on output data set.
Explanation
An I/O error occurred writing the output data set for
file tailoring.
User response
Check the output data set for possible
incompatibilities, such as wrong data set organization.
If further help is needed, call IBM support.
ISPF137 Ftopen error - Required files not
open
Explanation
This message is self explanatory.
ISPF138 STOW error - Module aaaaaaaa
received decimal return code
bbbbbbbb, decimal reason code
cccccccc from the STOW macro.
Explanation
An internal error occurred in ISPF while doing file
tailoring. An unexpected error code was returned from
the common STOW module (ISPCST). The module
that received the code is aaaaaaaa; the value of
bbbbbbbb is the decimal return code, the value of
cccccccc is the decimal reason code returned from the
MVS STOW macro. Possible STOW errors are directory
out of space or I/O error. For a more complete list
of possible error conditions refer to the appropriate
system documentation on the STOW macro.
System programmer response
Refer to the appropriate system documentation on the
STOW macro.
User response
Contact your system programmer.
ISPF139 Output error - Output file is not a
PDS.
Explanation
One of these occurred:
• An FTCLOSE was attempted with a specified member
name and the output file was not a partitioned data
set,
• An FTCLOSE was attempted on a data set that was
not a library, or
• An FTERASE was attempted on a data set that was
not partitioned.
Programmer response
Correct the error.
ISPF140 Data set not allocated - aaaaaaaa
data set is not allocated.
Explanation
The input data set or an imbed data set for file tailoring
was not allocated. The ddname is aaaaaaaa.
Programmer response
Allocate the input data sets and retry file tailoring.
ISPF141 ENQUEUE failed - Data set
aaaaaaaa in use, ENQUEUE failed.
Explanation
During file tailoring the input, output, or a temporary
data set could not be enqueued. aaaaaaaa is either
the ddname or data set name that could not be
enqueued. The data set is in use by another task, and
file tailoring cannot continue.
Programmer response
Using aaaaaaaa as a guide, resolve the conflicting use
of the data set.
ISPF142 Variable truncation - Variable
truncation has occurred.
Explanation
During file tailoring, a file tailoring variable was
replaced with a value that was truncated.
Programmer response
Analyze the variables to see which one may have been
truncated. Look at the data files to see if there are
lines that can be shortened to provide extra room for
expansion.
ISPF143 STAE error - Module aaaaaaaa
received RC-bbbbbbbb from STAE.
Explanation
An internal error occurred in ISPF while file tailoring
was trying to establish or cancel an STAE exit. The
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  119

## Page 140

return code from the STAE macro is bbbbbbbb.
aaaaaaaa is the name of the module where the STAE
macro was called.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF144 System abend - Module
aaaaaaaa intercepted system
abend bbbbbbbb.
Explanation
A system abend occurred during file tailoring. The
system abend code is bbbbbbbb and the STAE exit
routine which generated this message is in module
aaaaaaaa. Data sets have been closed and dequeued.
Programmer response
Use the abend code to help discover the reason for the
abend. Contact IBM support if further help is needed.
ISPF145 Output data conflict - Generated
data in last output column, last
input column cannot be moved.
Explanation
Column 72 in the skeleton data file already has data
in it, and there is generated data that would replace it.
This existing data might be a sequence number which
cannot be moved.
Programmer response
Avoid generating data in column 72 of the skeleton if
the file contains sequence numbers.
ISPF147 FSSTATE error - RC-aaaaaaaa
received from FSSTATE.
Explanation
This message is self explanatory.
ISPF151 Invalid tab position - Tab
(aaaaaaaa) out of range (1-255),
bbbbbbbb record-cccccccc.
Explanation
One of the tab values in the file tailoring )TB or )TBA
statement is not in the correct range of 1 through 255.
The incorrect value is aaaaaaaa, and the error was
discovered in record cccccccc of member bbbbbbbb.
Programmer response
Correct the )TB or )TBA statement to use tab values in
the range 1 to 255.
ISPF152 Invalid tab position - Tab
(aaaaaaaa) must exceed previous
tab, bbbbbbbb rec-cccccccc.
Explanation
The numeric tab positions in the )TB or )TBA
statement for file tailoring are not in increasing
sequence. aaaaaaaa is the value of the offending
tab value; it was found in record cccccccc of member
bbbbbbbb.
Programmer response
Correct the )TB or )TBA statement so each tab value is
higher than the previous one.
ISPF153 Too many tab stops - Tab control
word has more than 16 tab stops,
aaaaaaaa rec-bbbbbbbb.
Explanation
The )TB or )TBA statement in file tailoring has more
than 16 tab stops specified. No more than 16 are
allowed. The error was found in record bbbbbbbb of
member aaaaaaaa.
Programmer response
Correct the )TB or )TBA statement so no more than 16
tab stops are specified.
ISPF154 Invalid LRECL - The limit of input/
output data set logical record
length is 255.
Explanation
File tailoring data sets cannot have a logical record
length greater than 255. This applies to both input and
output data sets.
Programmer response
Check logical record length of input and output data
sets for file tailoring, including temporary data sets.
Make sure none has a logical record length greater
than 255.
ISPF messages starting with ISP
120  z/OS: z/OS ISPF Messages and Codes

## Page 141

ISPF155 STOW password error - File
tailoring does not support
password protected members.
Explanation
FTCLOSE or FTERASE is being attempted on a file
tailoring output data set member and the member is
password protected. File tailoring output to password
protected members is not supported.
Programmer response
Select an output data set for file tailoring that is not
password protected.
ISPF156 Output overflow - Line to be
written greater than data set
LRECL aaaaaaaa.
Explanation
During a file tailoring FTCLOSE, the last non-blank
character of the tailored output data would not fit
within the logical record length of the output data set.
The data set logical record length is aaaaaaaa.
Programmer response
Increase the logical record length of the output data
set or ensure that data will fit within the output data
set logical record.
ISPF170 Service error - Status2 aaaaaaaa
returned from bbbbbbbb, cccccccc
record-dddddddd
Explanation
This is an internal error in ISPF. A bbbbbbbb service
returned an unexpected value of aaaaaaaa for
status2. Processing cannot continue. The table in error
is used in member cccccccc at record dddddddd.
System programmer response
Contact IBM support.
Programmer response
Use the status2 value to help determine the nature of
the problem.
ISPF171 )DOT SCAN error - Incomplete
SCAN parameter on )DOT,
aaaaaaaa record-bbbbbbbb
Explanation
This is an internal error in ISPF. The internal pointers
for )DOT and )SEL have become inconsistent and
processing cannot continue. The error was detected
in member aaaaaaaa at record number bbbbbbbb.
Programmer response
Correct the SCAN parameter of the )DOT statement.
ISPF172 )DOT SCAN error - Too many SCAN
parameters on )DOT statement,
aaaaaaaa record-bbbbbbbb
Explanation
A maximum of 50 name-cond-pairs can be specified
for the SCAN parameter on the )DOT statement. The
error was found in record bbbbbbbb of member
aaaaaaaa.
Programmer response
Reduce the number of name-cond-pairs on the SCAN
parameter of the )DOT statement.
ISPF173 )DOT SCAN error - Invalid SCAN
variable name on )DOT statement,
aaaaaaaa record-bbbbbbbb
Explanation
A variable name on the SCAN parameter is invalid.
The error was found in record bbbbbbbb of member
aaaaaaaa.
Programmer response
Correct the variable names on the SCAN parameter of
the )DOT statement.
ISPF174 )DOT SCAN error - Invalid
SCAN condition value on )DOT
statement, aaaaaaaa record-
bbbbbbbb
Explanation
A condition operator on the SCAN parameter is invalid.
The error was found in record bbbbbbbb of member
aaaaaaaa.
Programmer response
Correct the condition values on the SCAN parameter of
the )DOT statement.
ISPF175 Too many DOs - DO stack is full,
aaaaaaaa record-bbbbbbbb.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  121

## Page 142

Explanation
More than 16 levels of )DO nesting was attempted
during file tailoring. Only 16 levels are allowed.
The error was detected in member aaaaaaaa while
processing record number bbbbbbbb.
Programmer response
Change the file tailoring skeleton so that no more than
16 nesting levels are used.
ISPF176 )DO error - )DO has no
matching )ENDDO, aaaaaaaa
record-bbbbbbbb.
Explanation
A )DO was in progress and the end of file was reached
on either the main skeleton file or an imbedded
file. ISPF was expecting to see the matching )ENDDO
before the end of the imbed or main file and didn't find
it. The member name is aaaaaaaa and bbbbbbbb is
the record number where the error was detected.
Programmer response
Examine the file and provide the matching )ENDDO.
ISPF177 )DO error - )ENDDO has no
matching )DO, aaaaaaaa record-
bbbbbbbb.
Explanation
In file tailoring, an )ENDDO was found before a
matching )DO statement. The control file is aaaaaaaa,
the error is found in record number bbbbbbbb.
Programmer response
Correct the control statements so that each )ENDDO
has a matching )DO.
ISPF178 )SEL error - )SEL ends in
wrong )DO, aaaaaaaa record-
bbbbbbbb.
Explanation
This is an internal error in ISPF. The internal stack
pointers for )SEL and )DOT have become inconsistent
and processing cannot continue.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF180 )DO error - )DO ends in
wrong )IF or )SEL, aaaaaaaa
record-bbbbbbbb
Explanation
This is an internal error in ISPF. The internal pointers
for )DO and )IF/)SEL have become inconsistent and
processing cannot continue. The error was detected
in member aaaaaaaa at record number bbbbbbbb.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF181 )DO error - )DO ends in
wrong )DOT, aaaaaaaa record-
bbbbbbbb
Explanation
This is an internal error in ISPF. The internal pointers
for )DO and )DOT have become inconsistent and
processing cannot continue. The error was detected
in member aaaaaaaa at record number bbbbbbbb.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF182 aaaaaaaa error - aaaaaaaa
outside a )DO structure, bbbbbbbb
record-cccccccc
Explanation
This is an logic error in ISPF skeleton. The aaaaaaaa
control statement can only be used within an
active )DO structure. The error was detected in
member bbbbbbbb at record number cccccccc.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF messages starting with ISP
122  z/OS: z/OS ISPF Messages and Codes

## Page 143

ISPF183 )DO aaaaaaaa error - )DO
aaaaaaaa internal logic error,
bbbbbbbb record-cccccccc
Explanation
This is an internal error in ISPF. The internal pointers
for )DO have become inconsistent and processing
cannot continue. The error was detected in member
bbbbbbbb at record number cccccccc.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF184 Invalid )DO syntax - Invalid )DO
Control variable name, aaaaaaaa
record-bbbbbbbb
Explanation
The control variable name specified on a )DO control
statement is too long or contains invalid characters.
The error was detected in member aaaaaaaa at
record number bbbbbbbb.
Programmer response
Correct the )DO control statement
ISPF185 Invalid )DO syntax - Invalid
numeric value in )DO control
statement, aaaaaaaa record-
bbbbbbbb
Explanation
The )DO control statement contains an invalid
numeric value. Numeric values must be in the
range -2147483647 to 2147483646. The error was
detected in member aaaaaaaa at record number
bbbbbbbb.
Programmer response
Correct the )DO control statement
ISPF186 Invalid )DO syntax - Syntax of )DO
control statement is invalid,
aaaaaaaa record-bbbbbbbb
Explanation
The syntax of the )DO control statement is incorrect.
The error was detected in member aaaaaaaa at
record number bbbbbbbb.
Programmer response
Correct the )DO control statement
ISPF187 Invalid )DO syntax - )DO control
statement contains duplicate
aaaaaaaa keyword, bbbbbbbb
record-cccccccc
Explanation
The syntax of the )DO control statement is incorrect.
Duplicate aaaaaaaa keyword specified. The error was
detected in member bbbbbbbb at record number
cccccccc.
Programmer response
Correct the )DO control statement
ISPF188 Invalid )DO syntax - )DO control
statement contains UNTIL and
WHILE keywords, aaaaaaaa
record-bbbbbbbb
Explanation
The syntax of the )DO control statement is incorrect.
Only one UNTIL or WHILE keyword permitted on
a )DO control statement, but not both. The error was
detected in member aaaaaaaa at record number
bbbbbbbb.
Programmer response
Correct the )DO control statement
ISPF189 Invalid )DO aaaaaaaa syntax -
Syntax of )DO aaaaaaaa control
statement is invalid, bbbbbbbb
record-cccccccc
Explanation
The syntax of the )DO aaaaaaaa control statement
is incorrect. The error was detected in member
bbbbbbbb at record number cccccccc.
Programmer response
Correct the )DO control statement
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  123

## Page 144

ISPF190 )ELSE error - )ELSE has no
matching )IF, aaaaaaaa record-
bbbbbbbb
Explanation
An )ELSE was found that does not have a
corresponding )IF. The member name is aaaaaaaa
and bbbbbbbb is the record number where the error
was detected.
Programmer response
Examine the file and provide the matching )IF.
ISPF191 )IF error - )IF missing
THEN keyword, aaaaaaaa record-
bbbbbbbb
Explanation
An )IF statement does not include the THEN keyword.
The THEN keyword is a required parameter on the )IF
control statement. The member name is aaaaaaaa
and bbbbbbbb is the record number where the error
was detected.
Programmer response
Examine the file and provide the THEN keyword on
the )IF.
ISPF192 )IF error - )IF has no statement,
aaaaaaaa record-bbbbbbbb.
Explanation
An )IF was in progress and the end of file was reached
on either the main skeleton file or an imbedded file.
ISPF was expecting to see a statement for the )IF
before the end of the imbed or main file and didn't
find it. The member name is aaaaaaaa and bbbbbbbb
is the record number where the error was detected.
Programmer response
Examine the file and provide the required )IF
statement.
ISPF193 )ELSE error - )ELSE has no
statement, aaaaaaaa record-
bbbbbbbb.
Explanation
An )ELSE was in progress and the end of file was
reached on either the main skeleton file or an
imbedded file. ISPF was expecting to see a statement
for the )ELSE before the end of the imbed or main file
and didn't find it. The member name is aaaaaaaa and
bbbbbbbb is the record number where the error was
detected.
Programmer response
Examine the file and provide the required )ELSE
statement.
ISPF194 aaaaaaaa error - aaaaaaaa
outside a )DOT structure,
bbbbbbbb record-cccccccc
Explanation
This is an logic error in ISPF skeleton. The aaaaaaaa
control statement specifying the DOT parameter can
only be used within an active )DOT structure. The error
was detected in member bbbbbbbb at record number
cccccccc.
Programmer response
Examine the file and correct the )LEAVE statement.
ISPF195 Mismatched Parentheses -
Parentheses are not paired
correctly, aaaaaaaa record-
bbbbbbbb
Explanation
Each left parenthesis in the )SETF control statement
must be matched with a right parenthesis. The
statement has a left or right parenthesis that
was unpaired. The error was detected in member
aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and correct the syntax to make sure
parentheses are paired correctly.
ISPF196 )ENDREXX error - )ENDREXX has
no matching )REXX, aaaaaaaa
record-bbbbbbbb.
Explanation
In file tailoring, an )ENDREXX was found before a
matching )REXX statement. The )ENDREXX statement
is only required where inline rexx statements are
included in the skeleton. The error was detected in
member aaaaaaaa at record number bbbbbbbb.
Programmer response
Correct the control statements so that each )ENDREXX
has a matching )REXX.
ISPF messages starting with ISP
124  z/OS: z/OS ISPF Messages and Codes

## Page 145

ISPF197 )REXX error - )REXX has no
matching )ENDREXX, aaaaaaaa
record-bbbbbbbb.
Explanation
A )REXX was in progress and the end of file was
reached on either the main skeleton file or an
imbedded file. ISPF was expecting to see the
matching )ENDREXX before the end of the imbed or
main file and didn't find it. The error was detected in
member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and provide the matching )ENDREXX
ISPF198 System abend - REXX processing
has generated a system
abend aaaaaaaa, intercepted
by bbbbbbbb, cccccccc record-
dddddddd.
Explanation
A system abend occurred processing a REXX exec
during file tailoring. The system abend code is
aaaaaaaa and the STAE exit routine which generated
this message is in module bbbbbbbb. The error
was detected in member cccccccc at record number
dddddddd. File tailoring is terminated.
Programmer response
Use the abend code to help discover the reason for the
abend processing the REXX exec. Contact IBM support
if further help is needed.
ISPF199 ESTAE error - Module aaaaaaaa
received RC-bbbbbbbb from
ESTAE.
Explanation
An internal error occurred in ISPF while file tailoring
was trying to establish or cancel an ESTAE exit. The
return code from the ESTAE macro is bbbbbbbb.
aaaaaaaa is the name of the module where the STAE
macro was called.
System programmer response
Contact IBM support.
Programmer response
Contact your system programmer.
ISPF200 Invalid aaaaaaaa Syntax -
Function statement syntax
not recognised on aaaaaaaa
statement, bbbbbbbb record-
cccccccc
Explanation
The syntax of a function statement was found to be
invalid. This includes mismatched parentheses. The
error was detected in member bbbbbbbb at record
number cccccccc.
Programmer response
Examine the file and correct the statement syntax.
ISPF201 Invalid Function - Unsupported
function call, aaaaaaaa record-
bbbbbbbb
Explanation
A function call is unsupported. The error was detected
in member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and correct the function call.
ISPF202 Invalid Function Syntax - Too
many subparameters specified
for aaaaaaaa function, bbbbbbbb
record-cccccccc.
Explanation
A function call contains too many subparameters. The
error was detected in member bbbbbbbb at record
number cccccccc.
Programmer response
Examine the file and correct the function call syntax.
ISPF203 aaaaaaaa Parm Missing - The
bbbbbbbb function is missing the
required aaaaaaaa parameter,
cccccccc record-dddddddd.
Explanation
The syntax of a function call is incomplete. One or
more required subparameters are missing, or contain a
null value. The error was detected in member cccccccc
at record number dddddddd.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  125

## Page 146

Programmer response
Examine the file and provide the missing
subparameter(s).
ISPF204 Invalid aaaaaaaa Value - The
bbbbbbbb function contains an
invalid aaaaaaaa parameter value,
cccccccc record-dddddddd.
Explanation
A subparameter value on a function call is invalid.
The error was detected in member cccccccc at record
number dddddddd.
Programmer response
Examine the file and correct the function
subparameters.
ISPF205 Symbol name error - Invalid
aaaaaaaa. symbol name,
bbbbbbbb record-cccccccc.
Explanation
A symbol name is invalid, or contains a null value. The
error was detected in member bbbbbbbb at record
number cccccccc.
Programmer response
Examine the file and correct the symbol name.
ISPF206 Invalid aaaaaaaa Value - The
bbbbbbbb function contains an
invalid aaaaaaaa parameter that
is Out of Range, cccccccc record-
dddddddd.
Explanation
A subparameter contains an numeric value that is
not within the valid range for the function. The error
was detected in member cccccccc at record number
dddddddd.
Programmer response
Examine the file and correct the subparameter value.
ISPF210 Invalid Expression - Expression
contains too many levels
of nesting, aaaaaaaa record-
bbbbbbbb
Explanation
The expression was found to contain more than 32
levels of nested parentheses. The error was detected
in member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and simplify the expression.
ISPF211 Invalid Expression - Length
of expression exceeds 255
characters, aaaaaaaa record-
bbbbbbbb
Explanation
The length of an expression was found to exceed
255 characters. The error was detected in member
aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and reduce the overall length of the
expression.
ISPF212 Mismatched parentheses -
Parentheses are not paired
correctly, aaaaaaaa record-
bbbbbbbb
Explanation
A mismatch in the parentheses has been detected.
The error was detected in member aaaaaaaa at
record number bbbbbbbb.
Programmer response
Examine the file and correct the syntax to ensure
parentheses are paired correctly.
ISPF213 Invalid numeric - Expression
contains an invalid numeric value,
aaaaaaaa record-bbbbbbbb.
Explanation
An expression contains an invalid numeric value.
Numeric values must be an integer in the range
-2147483647 to 2147483646. The error was
detected in member aaaaaaaa at record number
bbbbbbbb.
Programmer response
Examine the file and correct the syntax of the
expression.
ISPF messages starting with ISP
126  z/OS: z/OS ISPF Messages and Codes

## Page 147

ISPF214 Invalid Operator - Expression
contains an invalid operator,
aaaaaaaa record-bbbbbbbb
Explanation
Expression contains an invalid operator. Valid
operators are '+', '-', '*', '/', '**', and '/ /'. The error
was detected in member aaaaaaaa at record number
bbbbbbbb.
Programmer response
Examine the file and correct the syntax of the
expression.
ISPF215 Invalid Expression - Expression
contains invalid characters,
aaaaaaaa record-bbbbbbbb
Explanation
The expression contains an invalid or unexpected
syntax. The error was detected in member aaaaaaaa
at record number bbbbbbbb.
Programmer response
Examine the file and correct the syntax of the
expression.
ISPF216 Divide by Zero - An expression
has attempted to divide by zero,
aaaaaaaa record-bbbbbbbb
Explanation
An attempt was made to divide by zero. The error
was detected in member aaaaaaaa at record number
bbbbbbbb.
Programmer response
Examine the file and correct the syntax of the
expression.
ISPF217 Arithmetic Overflow - An
expression has resulted in an
overflow condition, aaaaaaaa
record-bbbbbbbb
Explanation
An intermediate or final result of an expression
exceeds the allowed number range, resulting in
an arithmetic overflow. The error was detected in
member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and modify the expression to ensure
that the overflow condition does not occur.
ISPF220 Invalid Dialog Variable -
Dialog Variable name for REXX
processing is invalid, aaaaaaaa
record-bbbbbbbb
Explanation
A dialog variable specified on a )REXX statement is
invalid, perhaps because it is too long, or because it
contains invalid characters. The error was detected in
member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the file and correct the dialog variable name
ISPF221 REXX not found - Unable to locate
REXX routine aaaaaaaa required
for File Tailoring, bbbbbbbb
record-cccccccc
Explanation
A File Tailoring )REXX statement specifies the name of
an external routine that can not be located. The error
was detected in member bbbbbbbb at record number
cccccccc.
Programmer response
Examine the skeleton and check the name of the file
tailoring REXX routine. Ensure the REXX routine is
available in your SYSPROC or SYSEXEC allocation.
ISPF222 REXX-defined error - File Tailoring
REXX routine-defined error.
ZFTXRC=aaaaaaaa..
Explanation
A File Tailoring Rexx routine set a return code of 8
in variable ZFTXRC but did not store a Message ID to
describe the failure in variable ZFTXMSG. This generic
message is provided by ISPF. File Tailoring processing
continues.
Programmer response
If appropriate, provide a meaningful message for the
user.
ISPF223 REXX Termination - File Tailoring
terminated by REXX routine
setting ZFTXRC=aaaaaaaa,
bbbbbbbb record-cccccccc.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  127

## Page 148

Explanation
A File Tailoring REXX routine set a return in the
dialog variable ZFTXRC that was not 0 or 8. This
generic message is provided by ISPF. File Tailoring
processing is terminated with a severe error. The error
was detected in member bbbbbbbb at record number
cccccccc.
Programmer response
Check the REXX routine to determine the cause
of error. If appropriate, provide a meaningful error
message for the user.
ISPF224 REXX Failure - Severe error
encountered processing a File
Tailoring Rexx routine, aaaaaaaa
record-bbbbbbbb, cccccccc return
code-dddddddd
Explanation
A severe error was encountered while executing a
File Tailoring Rexx routine. The error was detected in
member aaaaaaaa at record number bbbbbbbb.
Programmer response
Examine the REXX routine to determine the cause of
the REXX routine failure.
ISPF300 Trace complete - The Trace
output has been written to
SYSOUT allocated to the DDname
ISPFTTRC.
Explanation
The output to the File Tailoring Trace has been written
to a SYSOUT class allocated to the DDname ISPFTTRC
and can not be viewed by the ISPFTTRC command.
ISPF301 Parameter Invalid - Parameter
#aaaaaaaa is invalid. Valid
parameters are: END, VIEW,
LIST, QUIET, DISPLAY(),
READ(), SKELETON(), SCREEN(),
SERVICE(), and DEBUG.
Explanation
A parameter specified for the ISPDPTRC command
is invalid. Valid parameters are: END, VIEW, LIST,
QUIET, DISPLAY(), READ(), SKELETON(), SCREEN(),
SERVICE(), and DEBUG.
User response
Correct the command parameters.
ISPF302 Invalid skeleton name - The
skeleton name specified for the
SKELETON() parameter is invalid.
Explanation
The skeleton name specified must be either a valid
member name or member name pattern.
User response
Correct the supplied skeleton name.
ISPF303 aaaaaaaa value missing - The
value for aaaaaaaa parameter was
omitted.
Explanation
This message is self explanatory.
User response
Correct the command subparameters.
ISPF304 Invalid parameter value - The
RECORDS() parameter values are
invalid. Valid values are either
'*', NONE, or a combination of:
DATA, CNTL, SOURCE, or NODATA,
NOCNTL, NOSOURCE.
Explanation
The values specified for the RECORDS() parameter
are invalid. Valid values are either '*', NONE, or a
combination of: DATA, CNTL, SOURCE, or NODATA,
NOCNTL, NOSOURCE.
User response
Correct the parameter value.
ISPF305 Invalid parameter value - The
SCREEN() parameter value is
invalid. Valid values are: *
(current), 0 (all), or a screen id in
the range 1-9, A-W.
Explanation
The value specified for the SCREEN() parameter is
invalid. Valid values are: * (current), 0 (all), or a screen
id in the range 1-9, A-W.
ISPF messages starting with ISP
128  z/OS: z/OS ISPF Messages and Codes

## Page 149

User response
Correct the parameter value.
ISPF306 Invalid parameter value - The
SERVICE() parameter value is
invalid. Valid values are: NONE,
DETAIL.
Explanation
The value specified for the SERVICE() parameter is
invalid. Valid values are: NONE, DETAIL.
User response
Correct the parameter value.
ISPF307 Invalid parameter value - The
READ() parameter value is
invalid. Valid values are: NONE,
SUMMARY, DETAIL.
Explanation
The value specified for the READ() parameter is invalid.
Valid values are: NONE, SUMMARY, DETAIL.
User response
Correct the parameter value.
ISPF308 Invalid parameter value - The
TBVARS() parameter value is
invalid. Valid values are: NONE,
DETAIL.
Explanation
The value specified for the TBVARS() parameter is
invalid. Valid values are: NONE, DETAIL.
User response
Correct the parameter value.
ISPF310 LIST substituted - File tailoring
trace data set 'aaaaaaaa' not
found. List of file tailoring trace
data sets displayed
Explanation
ISPFTTRC attempted to VIEW a trace data set that
could not be found. A data set list of possible file
tailoring trace data sets was displayed
ISPF311 No trace data sets found - No ISPF
file tailoring trace data set names
were found matching 'aaaaaaaa'
Explanation
This is an informational message.
ISPG001 Invalid option number - Select
an option and enter the option
number or code.
Explanation
This message is self explanatory.
ISPG002 Enter project name - Project name
is missing or invalid.
Explanation
The Project name field is blank or invalid in the ISPF
library data set input fields.
User response
A valid Project name must be entered for the ISPF
library. An ISPF library is a cataloged partitioned data
set with a three-level data set name in this format:
'project.group.type'.
ISPG003 Enter group name - Group name is
missing or invalid.
Explanation
The Group name field is blank or invalid in the ISPF
library data set input fields.
User response
A valid Group name must be entered for the ISPF
library. An ISPF library is a cataloged partitioned data
set with a three-level data set name in this format:
'project.group.type'.
ISPG004 Enter type qualifier - Type is
missing or invalid.
Explanation
The Type name field is blank or invalid in the ISPF
library data set input fields.
User response
A valid Type name must be entered for the ISPF
library. An ISPF library is a cataloged partitioned data
set with a three-level data set name in this format:
'project.group.type'
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  129

## Page 150

ISPG005 Enter member name - The
specified data set is partitioned. A
member name is required.
Explanation
This message is self explanatory.
ISPG006 Enter volume serial - A volume
serial is needed for this operation.
Explanation
This message is self explanatory.
ISPG007 Vol serial not allowed - Use
"Other data set name" field when
specifying volume serial.
Explanation
The Volume Serial input field invalidly contains a value
when not specifying Other Data Set Name field.
User response
When using the ISPF Library input fields for entering
the data set name, do not put any value in the Volume
Serial input field. Only when using the Other Data Set
Name input field may the Volume Serial input field
contain a valid value.
ISPG008 Invalid required parm - Select
the required parameter from the
options shown.
Explanation
This message is self explanatory.
ISPG009 Enter data set name - Use
standard TSO format for data set
name.
Explanation
This message is self explanatory.
ISPG010 Data set is open - Data set is
currently being used by ISPF.
Explanation
The specified data set is open and currently being
used by ISPF.
User response
This message is issued when one of the data sets
LIST, LOG, TEMPLIST, TEMPCNTL, or EDIT RECOVERY
is specified and ISPF is currently using the data set.
The LOG and LIST data sets are controlled by the LOG
and LIST commands. These two commands can keep
the current data set and continue with a new data set
so that keep data set can be used. The TEMPLIST,
TEMPCNTL, and EDIT RECOVERY data sets need to be
saved and closed in order to be used.
ISPG011 Jump function disabled - The
NOJUMP attribute keyword has
disabled the jump function for this
field.
Explanation
The field has the jump function disabled. It was
disabled by the attribute keyword NOJUMP(ON).
Programmer response
To enable the jump function for this field the attribute
keyword NOJUMP(ON) should be set to NOJUMP(OFF)
or remove the NOJUMP attribute keyword from the
attribute type.
ISPG012 Panelid area warning - Panelid
area is formatted by SYSNAME,
USERID, PANELID and SCRNAME
commands - in that order. There is
insufficient space to display all the
requested information. Current
values are SYSNAME(aaaaaaaa),
USERID(bbbbbbbb),
PANELID(cccccccc) and
SCRNAME(dddddddd).
Explanation
More than two of SYSNAME, USERID, PANELID and
SCRNAME commands are active. The panelid area
is only 17 bytes wide and may not display all the
requested information.
User response
Set SYSNAME, USERID, PANELID or SCRNAME
commands to OFF until the desired information is
displayed in the panelid area.
Programmer response
None.
ISPG020 I/O error during write - Unable to
write data.
Explanation
This message is self explanatory.
ISPF messages starting with ISP
130  z/OS: z/OS ISPF Messages and Codes

## Page 151

ISPG021 Member not found - The requested
member was not found in the PDS
directory(s).
Explanation
This message is self explanatory.
ISPG024 Invalid scroll amount - Valid: M
(MAX), P (PAGE), H (HALF), C
(CSR), D (DATA) or aaaaaaaa to
bbbbbbbb
Explanation
An invalid scroll amount has been specified.
User response
Enter one of the valid responses listed in the Help
message, either - M (MAX), P (PAGE), H (HALF),
C (CSR), D (DATA), or a value in between the
minimum and maximum scroll amount defined in the
configuration table.
ISPG026 No top panel defined - The ZHTOP
dialog variable is not set to a panel
name.
Explanation
The dialog variable ZHTOP does not contain the name
of the first tutorial panel.
User response
The variable ZHTOP must contain the name of the first
tutorial panel. The variable can be set in the beginning
of the application to ensure that the user can always
display the tutorial panel regardless of how the tutorial
was entered. ZHTOP can also be set on the primary
option menu.
ISPG027 Option not implemented - Try
another option.
Explanation
This message is self explanatory.
ISPG028 Invalid selection - Valid codes are
B, S, U, T or I, or a number if
selection list present.
Explanation
An invalid code or selection number entered on a
tutorial panel.
User response
You can view the tutorial sequentially by leaving the
command/option field blank and repeatedly pressing
the Enter key.
Alternatively, you can select topics from lists displayed
on many of the tutorial pages. For example, enter
OPTION ===> 3
to select topic number three.
You can also enter one of these commands on any
tutorial page:
BACK or B
to back up to the previously viewed page.
SKIP or S
to skip the current topic and go on to the next
topic.
UP or U
to display a higher level list of topics.
TOC or T
to display the table of contents.
INDEX or I
to display the tutorial index.
ISPG029 No index panel defined - The
ZHINDEX dialog variable is not set
to a panel name.
Explanation
The dialog variable ZHINDEX does not contain the
name of the first INDEX panel.
User response
The variable ZHINDEX must contain the name of the
first INDEX panel. The variable can be set in the
beginning of the application to ensure that the user
can always display the INDEX panel regardless of how
tutorial was entered. ZHINDEX can also be set on the
primary option menu.
ISPG030 Invalid member name - Member
name specified contains invalid
characters.
Explanation
This message is self explanatory.
ISPG031 No space in directory - PDS
directory is full, allocate more
directory blocks.
Explanation
This message is self explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  131

## Page 152

ISPG032 I/O error during STOW -
Permanent I/O error occurred
updating directory block.
Explanation
This message is self explanatory.
ISPG033 Invalid command - Valid
commands are SELECT (s) and
LOCATE (l).
Explanation
This message is self explanatory.
ISPG034 I/O error during STOW -
Permanent I/O error occurred
writing "EOF mark" after member.
Explanation
This message is self explanatory.
ISPG035 Invalid select code - Type 's' for
select in front of the member
desired.
Explanation
This message is self explanatory.
ISPG036 Compress data set - STOW error.
STOW rc was r15=x'10' r0 was not
1 or 2. See sys programmer
Explanation
This message is self explanatory.
ISPG037 I/O error - I/O error while reading
PDS directory.
Explanation
This message is self explanatory.
ISPG038 I/O error - I/O error return from
BLDL (reading PDS directory).
Explanation
A permanent I/O error was detected when the system
attempted to search the directory of the specified data
set.
System programmer response
Restore the specified data set from backup, if possible.
Contact IBM support if further assistance is required.
User response
Ensure that the specified data set is the correct data
set. Note message number and text, then contact your
system programmer.
ISPG039 Specify member name - Command
specified requires a member
name.
Explanation
This message is self explanatory.
ISPG040 Invalid DSORG - Data set
organization must be partitioned
or sequential.
Explanation
The data set organization was entered incorrectly.
User response
Correct the data set organization.
ISPG041 Interface error - Rc = 'aaaaaaaa'
from TSO routine IKJADTAB,
function = ENDTABLE
Explanation
An error occurred on return from link to IKJADTAB.
System programmer response
Determine what the user was doing when the error
occurred, then contact IBM support.
User response
Contact your system programmer.
ISPG044 Concatenation failed - Unable
to concatenate the specified
libraries.
Explanation
Concatenation failed, ISPCDAIR did not return with
RC=0.
System programmer response
If the user's action was correct, contact IBM support.
User response
Contact your system programmer.
ISPF messages starting with ISP
132  z/OS: z/OS ISPF Messages and Codes

## Page 153

ISPG045 STAE macro error - Unable to
obtain storage for STAE macro,
subpool 0 full.
Explanation
Storage has been exhausted, there was a previous
error that could have used all of the available storage.
System programmer response
Verify that the task is not looping and that the storage
is sufficient to execute the failing function.
User response
Verify that your region size is large enough to run the
task that just failed. If you still have a problem, contact
the system programmer.
ISPG046 IKJTBLS interface error -
Authorized table name 'aaaaaaaa'
not recognized.
Explanation
An error has occurred between the ISPF and TSO
interface.
User response
Contact your system programmer.
Programmer response
Verify that the program has been properly authorized.
ISPG047 IRXTERMA interface error - ISPF
received return code 'aaaaaaaa'
from TSO routine IRXTERMA
Explanation
An error occurred while initializing the REXX
environment.
System programmer response
If the user's action was correct, contact IBM support.
User response
Contact your system programmer.
ISPG048 TSO interface error - - Abend
'aaaaaaaa' dec, reason code
'bbbbbbbb' in TSO routine IKJTBLS
Explanation
An abend occurred while searching for a program
name in the authorization tables.
System programmer response
Verify that the user has the ability to run authorized
programs. Contact IBM support to determine why
IKJTBLS returned an error code of 20.
User response
Contact your system programmer.
ISPG049 TSO interface error - - Abend
'aaaaaaaa' hex, reason code
'bbbbbbbb' in TSO routine IKJTBLS
Explanation
An abend while searching for a program name in the
authorization tables.
System programmer response
Verify that the user has the ability to run authorized
programs. Contact IBM support to determine why
IKJTBLS returned an error code of 20.
User response
Contact your system programmer.
ISPG050 aaaaaaaa is not active - The
command or function key entered
is not defined.
Explanation
Either an invalid command was entered or the function
key is not defined.
User response
Correct the command.
ISPG051 aaaaaaaa command error - The
command or function key entered
is defined incorrectly.
Explanation
The command or function key action was
unsuccessful.
User response
Correct the command.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  133

## Page 154

ISPG052 Invalid PANELID parm - Valid
panelid parameters are ON, OFF,
or blank.
Explanation
The PANELID command should be followed by ON,
OFF, or nothing.
User response
Correct the PANELID command.
ISPG053 Command is not active - The
command or function key entered
is not defined.
Explanation
The command entered or the function entered is not
defined.
User response
Correct the command.
ISPG054 Command parm is missing -
The entered command requires a
parameter.
Explanation
Additional data must follow this command.
User response
Enter a complete command. If additional information
is needed, use the ISPF documentation that defines
the command.
ISPG055 Invalid MSGID parm - Valid MSGID
parameters are ON, OFF or blank.
Explanation
The MSGID command requires the additional
information shown.
User response
Correct the command.
ISPG056 The message ID of the last
message was aaaaaaaa.
Explanation
This is an informational message.
ISPG057 Line command error - A command
is already in the line command
field.
Explanation
There is a line command conflict. A command is
already in the line command field.
User response
Correct the line commands.
ISPG058 Line command error - The line with
the cursor does not contain a line
command field.
Explanation
There is not a field designated to accept line
commands in the line that contains the cursor.
User response
Correct the attempted use of the line command.
ISPG059 Data truncated - Command input
string larger than command input
buffer for this panel.
Explanation
The command string entered is too large and will be
truncated.
User response
Correct the command input string.
ISPG060 Invalid command name - A
command entered or contained in
a CLIST has invalid syntax.
Explanation
The command entered or used within a CLIST has
invalid syntax.
User response
Contact the responsible programmer.
Programmer response
Correct the CLIST syntax at the failing line.
ISPG061 Invalid command name - The
command entered or contained in
a CLIST is null (all blanks).
ISPF messages starting with ISP
134  z/OS: z/OS ISPF Messages and Codes

## Page 155

Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Correct the CLIST by entering a valid command name.
ISPG062 Invalid command - Unable
to process command. IKJSCAN
return code = aaaaaaaa.
Explanation
The command entered is unauthorized.
User response
Enter the correct command name. If the command is
correct, contact the responsible programmer.
Programmer response
Verify that the command name is an authorized
command.
ISPG063 Command rejected - Command
rejected - 'aaaaaaaa' command
not supported under ISPF.
Explanation
This command name cannot be used while using ISPF.
User response
Enter the name correctly, or contact the responsible
programmer.
Programmer response
Correct the command name to avoid conflict with ISPF.
ISPG063A Command rejected - Command
rejected - 'aaaaaaaa' command
not supported when ISPF is
invoked from a web client.
Explanation
The entry for this command in the ISPF TSO Command
Table (ISPTCM) indicates this command cannot be run
when ISPF has been invoked from a web client.
System programmer response
Check the entry for this command in the ISPF TSO
Command Table (ISPTCM).
User response
Ensure you have entered the correct command
name. If ask your system administrator whether this
command should be valid to run when ISPF is invoked
from a web client.
ISPG064 Command abend - User abend
aaaaaaaa dec occurred processing
command 'bbbbbbbb'.
Explanation
The subtask abended during execution of the
command.
User response
Contact the responsible programmer.
Programmer response
Determine why the subtask abended. Contact IBM
support, if necessary.
ISPG065 Attach error - Unable to attach
command 'aaaaaaaa'. Attach
return code=bbbbbbbb.
Explanation
The command cannot be executed.
User response
Contact the responsible programmer.
Programmer response
Check the ATTACH macro documentation to determine
why the ATTACH command failed.
ISPG066 Command abend - Abend
aaaaaaaa hex occurred processing
command 'bbbbbbbb'.
Explanation
The command cannot be processed.
User response
Contact the responsible programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  135

## Page 156

Programmer response
If you are unable to determine why the abend
occurred, contact IBM support.
ISPG067 PUTGET error - PUTGET return
code aaaaaaaa hex. Notify your
system programmer.
Explanation
A system failure occurred.
System programmer response
An unexpected return code was received while
executing a command. Refer to the PUTGET
documentation for additional information about this
return code.
User response
Contact your system programmer.
ISPG068 Member name too long - Member
name must be less than or equal to
8 characters.
Explanation
The member name of the data set exceeds the 8
character limit.
User response
Correct the member name.
ISPG069 No members in data set - The PDS
specified contains no members.
Explanation
This is an informational message.
ISPG070 IKJEFTSR interface error -
Authorized command 'aaaaaaaa'.
Return code = bbbbbbbb.. Reason
code = cccccccc..
Explanation
The command that was processing has failed.
User response
Contact the responsible programmer.
Programmer response
IKJEFTSR, the TSO/E Service Routine, has returned
the listed return and reason codes. Use these codes to
determine the reason for the failure.
ISPG071 IKJEFTSR interface error -
Authorized program 'aaaaaaaa'.
Return code = bbbbbbbb.. Reason
code = cccccccc..
Explanation
The authorized program that was processing has
failed.
User response
Contact the responsible programmer.
Programmer response
IKJEFTSR, the TSO/E Service Routine, has returned
the listed return and reason codes. Use these codes to
determine the reason for the failure.
ISPG072 Attention termination - Authorized
program 'aaaaaaaa' was
terminated by an attention.
Explanation
If an attention key was pressed, this is an
informational message only.
System programmer response
If the user did not press an attention key, contact IBM
support.
User response
If the attention key was not pressed, contact the
system programmer.
ISPG073 Command abended - User abend
aaaaaaaa dec occurred processing
authorized command 'bbbbbbbb'.
Explanation
The command being processed has failed.
User response
Report this message to the responsible programmer.
ISPF messages starting with ISP
136  z/OS: z/OS ISPF Messages and Codes

## Page 157

Programmer response
IKJEFTSR, the TSO/E Service Routine, has returned
the user abend code specified to assist in problem
determination.
ISPG074 BLDL error message - Error
processing 'aaaaaaaa' command.
BLDL return code was greater than
4.
Explanation
The BLDL failed while running the dialog.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPG075 Command abended - Abend
aaaaaaaa hex occurred processing
authorized command 'bbbbbbbb'.
Explanation
A system failure occurred during execution of the
command.
System programmer response
A subtask dump may help to determine where the
failure occurred.
User response
Contact your system programmer.
ISPG076 Attention message - Command
terminated due to attention.
Explanation
If an attention was entered, this message is
informational only.
System programmer response
Contact IBM support.
User response
If an attention was not entered, contact the system
programmer.
ISPG077 Abend 806 - Command 'aaaaaaaa'
abended with code 806000. Load
module not found.
Explanation
This is an informational message.
User response
Contact the responsible programmer.
Programmer response
Verify that both your concatenations and the LIBDEF
concatenations are correct.
ISPG078 Program abend - User abend
aaaaaaaa dec occurred processing
authorized program 'bbbbbbbb'.
Explanation
The program was unsuccessful.
User response
Contact the responsible programmer.
Programmer response
Use the abend code returned from IKJEFTSR (the
TSO/E Service Routine) to determine the reason for the
abend.
ISPG079 Program abend - Abend
aaaaaaaa hex occurred processing
authorized program 'bbbbbbbb'.
Explanation
The program that was processing has failed.
User response
Contact the responsible programmer.
Programmer response
A subtask dump may be necessary to determine
the reason for the failure. Contact IBM support if
additional assistance is needed.
ISPG080 Inconsistent data sets - Data set
organizations are not the same.
Explanation
An incorrect data set name may have been entered.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  137

## Page 158

User response
Verify that all data set names are entered correctly.
If the data set names are correct, contact the
responsible programmer.
Programmer response
Correct the DSORG of the data sets entered.
Sequential and partitioned data sets cannot be mixed.
ISPG081 Data set not partitioned - Member
name specified, but the data set is
not a partitioned data set.
Explanation
You have specified a member name for a non-
partitioned data set.
User response
Enter the correct data set name.
ISPG082 Data set not partitioned - Only
partitioned data sets may be
concatenated.
Explanation
A non-partitioned data set has been entered in the
concatenation sequence.
User response
Enter the correct data set names, or delete any data
sets that are not partitioned.
ISPG083 Deconcatenation failed - Unable
to deconcatenate the specified
libraries.
Explanation
One of the libraries cannot be deconcatenated; it is
probably still in use.
User response
End the job that is using one of the libraries.
ISPG084 Reallocation failed - DAIR RC =
aaaaaaaa dec, DARC = bbbbbbbb
hex, dsn = 'cccccccc'
Explanation
The data set could not be reallocated as old.
User response
Use the return codes shown to determine why the
operation failed. Contact the system programmer, if
necessary.
Here are the DAIR return codes:
CODE
MEANING
0
DAIR completed successfully. Secondary error
code in DARC field.
4
Invalid parameter list passed to DAIR.
8
Catalog Management error. Error code in CTRC
field.
12
Dynamic Allocation error. Error code in DARC field.
16
No TIOT entries were available for use.
20
The DDNAME requested in unavailable.
24
The DSNAME requested is a member of a
concatenated group.
28
DDNAME or DSNAME not allocated, or ATTR list
name not found.
32
DISP=NEW specified for previously permanently
allocated data set.
36
Catalog information routine error.
40
More index blocks exist than the program provided
room for.
44
DISP=OLD, MOD, or SHR for data set previously
allocated for delete.
48
Reserved.
52
Request denied by Installation Exit.
ISPG085 Deallocation failed - DAIR RC =
aaaaaaaa dec, DARC = bbbbbbbb
hex, dsn = 'cccccccc'
Explanation
The deallocation of the data set failed.
ISPF messages starting with ISP
138  z/OS: z/OS ISPF Messages and Codes

## Page 159

User response
See message ISPG084.
ISPG086 Delete data set failed - 'aaaaaaaa'
is open, cannot be deleted.
Explanation
The data set is in use and cannot be deleted.
User response
End the task that is using the data set.
ISPG087 Data set in use - The data set is
currently in use by you.
Explanation
The data set is being used in another task.
User response
End the other task before starting this task.
ISPG088 Dsn ALLOC check failed - Unable
to check if data set is already
allocated.
Explanation
The allocation check failed.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPG090 Invalid data set name - Data
set name contains unbalanced
apostrophes.
Explanation
The data set name entered contains unbalanced
apostrophes.
User response
Reenter the data set name with the correct
apostrophes.
ISPG091 Invalid data set name - Data set
name must be 1-44 characters in
length counting prefix (if any).
Explanation
The data set name you entered had an invalid length.
It must be no more than 44 characters including the
TSO prefix, if used.
User response
Check the data set name, and reenter it correctly.
ISPG092 Invalid member name - Member
name must be 1-8 characters and
enclosed in parentheses.
Explanation
The member name that was entered was invalid. The
member name must be 1 to 8 characters, enclosed in
parentheses.
User response
Reenter the member name using the correct syntax.
ISPG093 Invalid data set name -
Embedded blanks, parentheses, or
apostrophes in data set name are
invalid.
Explanation
The data set name that was entered contained blanks,
apostrophes, or parentheses, which are invalid in a
data set name.
User response
Reenter the data set name following the correct
syntax.
ISPG094 Data set not cataloged -
Generation data set not found.
Check catalog of generation group.
Explanation
The data set specified was not found.
User response
Check the data set name entered.
ISPG095 Inconsistent block size - The first
library must have the largest block
size.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  139

## Page 160

Explanation
The first library of the concatenation must have
the largest block size of the libraries within the
concatenation.
System programmer response
Update the concatenation so that the largest block size
data is first in the concatenation.
ISPG096 Inconsistent record size - All
libraries must have the same
record length.
Explanation
All libraries within a concatenation must have the
same record length.
System programmer response
Update the concatenation and library allocations to
have consistent record lengths.
ISPG097 Inconsistent record type - All
libraries must be the same record
type.
Explanation
All libraries within a concatenation must be defined
with the same record type.
Programmer response
Review the libraries within the concatenation for the
correct record type.
ISPG098 Inconsistent record size - The
first library must have the largest
record length.
Explanation
The first library within a concatenation must be
defined with the largest record length.
Programmer response
Review the libraries within the concatenation for the
correct record length and concatenation sequence.
ISPG099 Not generation data set - Data set
is not a generation data set.
Explanation
Data set specified is not a generation data set.
Programmer response
Check the data set name specified.
ISPG100 Invalid data set org - Data set is
not sequential or partitioned.
Explanation
The data set referenced does not have an organization
of either sequential or partitioned. The data set
organization must be either sequential or partitioned.
System programmer response
Update the data set referenced, or reallocate the data
set so that it is either sequential or partitioned.
User response
Contact your system programmer.
ISPG101 STAE failed - Unable to open data
set. Possibly insufficient storage
for STAE.
Explanation
There is insufficient storage available to open the data
set.
User response
Make sure you have sufficient storage and retry the
function.
ISPG102 Invalid DSORG - Data set specified
is a PDS, which is not allowed for
this function.
Explanation
The data set referenced for this function is a
partitioned data set (PDS). A PDS is not allowed for
this function.
System programmer response
Update the function so that it references a data set
with the correct organization.
User response
Contact your system programmer.
ISPG103 Invalid record format - S type
partitioned data sets are not
supported by ISPF.
ISPF messages starting with ISP
140  z/OS: z/OS ISPF Messages and Codes

## Page 161

Explanation
The data set referenced is an S type partitioned data
set, which is not supported by ISPF.
User response
Select a data set that is not an S type partitioned data
set.
ISPG104 Invalid DSORG - Data set is
sequential, which is not allowed
for this function
Explanation
The data set referenced for this function is a
sequential data set. Sequential data sets are not
allowed for this function.
System programmer response
Update the function so that it references a data set
with the correct organization.
User response
Contact your system programmer.
ISPG105 Invalid block size - Block size of
data set must not be zero.
Explanation
The block size of the data set specified must not be
equal to zero.
System programmer response
Specify a data set with a valid block size, or reallocate
the data set so that it does not have a block size of
zero.
User response
Contact your system programmer.
ISPG106 Invalid record length - Record
length exceeds maximum
('aaaaaaaa') allowed for this
function.
Explanation
The data set specified contains an invalid record
length for this function.
System programmer response
Specify a data set with a valid record length for this
function, or reallocate the data set.
User response
Contact your system programmer.
ISPG107 Invalid record format - Fixed
length records not supported for
this function.
Explanation
The data set specified contains an invalid record
format for this function. The function does not support
fixed length records.
System programmer response
Specify a data set with a valid record format, or
reallocate the data set referenced.
User response
Contact your system programmer.
ISPG108 Invalid record length - LRECL must
be 0 or equal to block size for
unblocked RECFM=F.
Explanation
The record length for an unblocked RECFM=F data set
must be either zero or equal to the block size.
System programmer response
Reallocate the data set so that it has a valid record
length for the block size specified.
User response
Contact your system programmer.
ISPG109 Invalid block size - Data set block
size is not a multiple of LRECL for
RECFM=FB.
Explanation
The block size of a fixed block data set must be a
multiple of the record length.
System programmer response
Reallocate the data set so that is has a valid block size
for the record format specified.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  141

## Page 162

User response
Contact your system programmer.
ISPG110 Invalid record format - Variable
length records invalid for this
function.
Explanation
The data set specified for this function contains an
invalid record format. The function does not support
variable length records.
System programmer response
Either specify a valid data set for the function or
reallocate the data set with a valid record format.
User response
Contact your system programmer.
ISPG111 Invalid block size - Data set block
size must be > 8 for RECFM=V.
Explanation
The data set specified for the function contains an
invalid block size. The data set block size must be
greater than 8 for a data set with a RECFM=V.
System programmer response
Update the data set allocation to a valid block size.
User response
Contact your system programmer.
ISPG112 Invalid record length - Data set
LRECL must be > 4 for RECFM=V.
Explanation
The data set specified for the function contains an
invalid record length. The data set record length must
be greater than 4 for a data set with a RECFM=V.
System programmer response
Update the data set allocation to a valid record length.
User response
Contact your system programmer.
ISPG113 Invalid record length - LRECL and
block size are inconsistent for
RECFM=V data set.
Explanation
The record length and block size specified are
inconsistent for data sets with RECFM=V.
System programmer response
Update the data set allocation so that it has a valid
block size and LRECL.
User response
Contact your system programmer.
ISPG114 Invalid record format - RECFM=U
is not allowed for this function.
Explanation
Data sets that have RECFM=U are not allowed for the
specified function.
System programmer response
Update the data set allocation so that it has a valid
record format.
User response
Contact your system programmer.
ISPG115 Invalid record format - Data set
has record format not supported
by ISPF. Must be F, V, or U.
Explanation
The data set specified has a record format that is not
supported by ISPF and its services.
System programmer response
Specify a valid record format for the data set. Valid
formats are F, V, and U.
User response
Contact your system programmer.
ISPG116 Open failed - Unable to open data
set. DCB open flag is not set.
Explanation
The data set could not be opened.
System programmer response
Contact IBM support.
ISPF messages starting with ISP
142  z/OS: z/OS ISPF Messages and Codes

## Page 163

User response
Contact your system programmer.
ISPG117 Open failed - Open for data set
'aaaaaaaa' abended with abend
code 'bbbbbbbb'.
Explanation
The data set could not be opened.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPG118 Obtain failed - Unable to open
data set. Obtain returned nonzero
return code.
Explanation
The DSCB for this data set could not be found, and the
data set could not be opened.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPG119 Invalid record format - ISPF does
not support spanned records.
Explanation
The data set referenced uses spanned records and so
is not supported by ISPF.
User response
Use a data set that does not contain spanned records.
ISPG120 Empty data set or member - Empty
PDS member or sequential data
set has been requested.
Explanation
The partitioned data set member or sequential data
set requested is empty.
User response
Verify the data set or member specified.
ISPG121 I/O error - An I/O error was
encountered reading the first
record requested.
Explanation
The data set cannot be read.
System programmer response
If this error is not caused by a defective DASD, contact
IBM support.
User response
Contact your system programmer.
ISPG122 Insufficient storage - Not enough
storage is available for browse to
proceed.
Explanation
Storage has been exhausted.
System programmer response
Verify that the user has sufficient storage to meet the
minimum ISPF requirements. Contact IBM support if
the storage is sufficient.
User response
Contact your system programmer.
ISPG123 Invalid type of data set - Data set
has nonzero key. This data set is
not supported by ISPF.
Explanation
The data set specified is not a type that is supported
by ISPF.
User response
Contact your system programmer.
ISPG124 Invalid SYSNAME parm - Valid
sysname parameters are ON, OFF,
or blank.
Explanation
The SYSNAME command should be followed by ON,
OFF, or nothing.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  143

## Page 164

User response
Contact your system programmer.
ISPG125 Invalid USERID parm - Valid
USERID parameters are ON, OFF,
or blank.
Explanation
The USERID command should be followed by ON, OFF,
or nothing.
User response
Contact your system programmer.
ISPG129 SWAP LIST not available - The
SWAP LIST command is not
supported when ISPF is invoked
from a web client.
Explanation
ISPF does not support the display of the Swap List
panel for a web client.
User response
Use other methods including other forms of the SWAP
command to swap logical screens.
ISPG130 Reserve failed - Abend occurred
during reserve processing. Unable
to open data set.
Explanation
ISPF was unable to reserve and open the data set for
your use.
System programmer response
Contact IBM support.
User response
If the error continues, contact the system programmer.
ISPG131 Unable to open data set - The dd
that should be in the tiot cannot be
found.
Explanation
The data set could not be opened.
System programmer response
Contact IBM support.
User response
If the error continues, contact the system programmer.
ISPG132 Authorization failed - You may not
use this protected data set. Open
913 abend.
Explanation
You are not authorized to use this data set.
System programmer response
Authorize the user, if required.
User response
Contact your system programmer.
ISPG133 Insufficient storage - Not enough
storage for buffer. Log on with
larger SIZE parameter.
Explanation
OPEN tried to obtain a buffer to contain the I/O
requested. Not enough storage was available for the
GETMAIN.
ISPG140 Error generating command - Proc
contains invalid statement type.
Explanation
This message is self explanatory.
ISPG142 Error generating command -
Unable to FIND proc member in
ISPSLIB library.
Explanation
This message is self explanatory.
ISPG143 I/O error - I/O error FINDing proc.
Notify system programmer.
Explanation
This message is self explanatory.
ISPG144 Error generating command - Proc
contains command continuation
before COMMAND statement.
Explanation
This message is self explanatory.
ISPF messages starting with ISP
144  z/OS: z/OS ISPF Messages and Codes

## Page 165

ISPG145 I/O error - I/O error reading proc.
Notify system programmer.
Explanation
This message is self explanatory.
ISPG146 Error generating command -
Command generated exceeds
command buffer length.
Explanation
This message is self explanatory.
ISPG149 Specify required parm - The cursor
is positioned at the blank or
invalid parameter.
Explanation
This message is self explanatory.
ISPG150 Error generating command -
Invalid keyword in PROC.
Explanation
This message is self explanatory.
ISPG151 Error generating command -
Invalid data set name on PROC
"ALLOC" statement.
Explanation
This message is self explanatory.
ISPG152 Error generating command -
PROC contains more than one
"COMMAND" statement.
Explanation
This message is self explanatory.
ISPG153 Error generating command -
PROC contains invalid dsname on
"FREEDSN" statement.
Explanation
This message is self explanatory.
ISPG155 Error generating command -
PROC contains invalid ddname on
"ALLOC" statement.
Explanation
This message is self explanatory.
ISPG156 Error generating command -
PROC contains too many "ALLOC"
statements. Max is 10.
Explanation
This message is self explanatory.
ISPG157 Open error - Unable to open
input data set. "SCANLIB" PROC
statement may be in error.
Explanation
This message is self explanatory.
ISPG158 Input member not found - The
member specified cannot be found
in the input data set(s).
Explanation
This message is self explanatory.
ISPG159 Error generating command -
PROC contains invalid dsname on
"SCANLIB" statement.
Explanation
This message is self explanatory.
ISPG162 Error generating command - Menu
contains action statement with
invalid name or "key".
Explanation
This message is self explanatory.
ISPG166 I/O error - I/O error finding input
data set member. Data set should
be a PDS.
Explanation
This message is self explanatory.
ISPG200 GDDM load failed. - Attempt to
load GDDM was not successful.
Explanation
The attempt to load GDDM was not successful. One
possible reason is that the GDDM libraries were not
found within the user's concatenations.
System programmer response
Check the allocations for the proper GDDM libraries.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  145

## Page 166

User response
Contact your system programmer.
ISPG201 Device not supported. - The ISPF/
GDDM interface does not support
the current device.
Explanation
This is an informational message. The current device
is not supported while the ISPF/GDDM interface is
active. For example, GDDM is not available on devices
with a primary width different from the alternate
width, such as a 3278 model 5.
ISPG202 SPLIT not supported. - The ISPF/
GDDM interface does not support
SPLIT on the current device.
Explanation
This is an informational message. The use of SPLIT
is not supported on the current device while the
ISPF/GDDM interface is active. For example, on 3290
terminals, the vertical split function is disabled. Panels
are displayed with a larger-size character set, and the
partition jump key is not functional.
ISPG205 GRINIT failure - GDDM cannot be
initialized when running in Batch
mode on a client that is using the
JSON API.
Explanation
GDDM cannot be initialized because the user is
running in batch mode on a client that is using the
JSON API.
System programmer response
The user should reinvoke ISPF outside of a batch
environment in order to run this GDDM application.
User response
Contact your system programmer.
ISPG208 ISPG208 GDDM error - function =
Return code =
Explanation
GDDM had an error. The GDDM Function and Return
code is posted to assist the user.
User response
Contact the Application Programmer
Programmer response
The reported function should be examined for proper
GDDM usage.
ISPG209 ISPG209 GDDM error message -
Explanation
GDDM has detected an error and posted an error
message.
System programmer response
GDDM documentation should be used to determine
the source of the error.
User response
Contact your system programmer.
ISPG210 Invalid SWAPBAR parm - Valid
SWAPBAR parameters are ON,
OFF, /, or blank.
Explanation
The SWAPBAR command should be followed by ON,
OFF, /, or nothing.
ISPG211 Invalid value - Valid values are / or
blank.
Explanation
The Show SWAPBAR divider line value can be / or
nothing.
ISPG212 Invalid value - Valid values are 'S',
'C', 'D' or blank.
Explanation
The update action requested must be one of S, C, D or
nothing.
ISPG213 Invalid value - Valid values are: 0
or 1
Explanation
An invalid value was supplied for the variable. Valid
values for the variable are 0 and 1
User response
Enter either 0 or 1 into the variable.
ISPG214 Invalid value - Valid values are 'N',
'B', 'R', 'U' or blank.
ISPF messages starting with ISP
146  z/OS: z/OS ISPF Messages and Codes

## Page 167

Explanation
The hilite requested must be one of N, B, R, U or
nothing.
ISPH001 TRANS parameter error -
"FRMCCSID(aaaaaaaa)" is not the
required 5 numeric digits.
Explanation
The TRANS service required from-CCSID-number
parameter FRMCCSID(aaaaaaaa) must be composed
of 5 numeric digits.
User response
Ensure that the correct 5-digit decimal number that
specifies the CCSID to be used has FRMCCSID
parameter. Refer to TRANS service in z/OS ISPF
Services Guide for more information.
ISPH002 TRANS parameter error -
"TOCCSID(aaaaaaaa)" is not the
required 5 numeric digits.
Explanation
The TRANS service required to-CCSID-number
parameter TOCCSID(aaaaaaaa) must be composed of
5 numeric digits.
User response
Ensure that the correct 5-digit decimal number that
specifies the CCSID to be used has the TOCCSID
parameter. Refer to TRANS service in z/OS ISPF
Services Guide for more information.
ISPH003 TRANS parameter error -
"LENGTH(aaaaaaaa)" must be a
numeric value from 0 to 32767.
Explanation
The TRANS service LENGTH parameter contains an
integer value greater than 32 767.
User response
Enter a correct value for the LENGTH parameter that
is an integer value between 0 and 32 767. Refer to
TRANS service in z/OS ISPF Services Guide for more
information.
ISPH004 CCSID TRANSlate error - Translate
tables do not support "aaaaaaaa"
to "bbbbbbbb", (cccccccc).
Explanation
The translate tables do not support or are not available
to support from CCSID aaaaaaaa to bbbbbbbb.
System programmer response
Ensure that the correct CCSID translate tables exist.
Contact IBM support.
User response
Contact your system programmer.
ISPI001 ISPI001 Invalid screen size. The
width cannot exceed 160 and the
depth cannot exceed 62
Explanation
The terminal is configured for a screen size that is not
within the ISPF supported configuration.
System programmer response
Correct the terminal configuration screen size.
User response
Contact your system programmer.
ISPI002 ISPI002 Following file could not
be opened:
Explanation
A DDNAME is posted with this message and a file
within that DDNAME is probably entered incorrectly
and is not found.
System programmer response
Check for correct allocations
User response
Check the user specified allocations. If correct,
contact the system programmer for assistance with
the Logon proc.
ISPI003 ISPI003 Following file was not
preallocated:
Explanation
The DDNAME that follows this message had an invalid
data set name or a name was missing.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  147

## Page 168

User response
Check your allocations and if correct contact the
system programmer.
ISPI004 ISPI004 Unidentified ISPF system
error specified.
Explanation
During initialization an error was encountered. It was
not one of the common initialization errors.
System programmer response
Determine why ISPF can't initialize.
User response
Contact your system programmer.
Problem determination
Does any logon proc work? What is different about the
one that fails.
ISPI005 ISPI005 User character
translation discontinued
Explanation
Control Characters for Line Deleting and Character
Deleting has been turned off.
ISPI006 ISPI006 Following file has invalid
data set characteristics
Explanation
The DDNAME that follows this message contains a
data set that has invalid characteristics.
User response
Check the allocations for the DDNAME in the message.
ISPI007 ISPI007 ISPF command not
allowed. You are already under
ISPF
Explanation
You can not execute the ISPF command when ISPF is
already running.
User response
Correct the command.
ISPI009 ISPI009 Following required
module for selected language
could not be loaded:
Explanation
The translate table module could not be loaded.
User response
Assure the module can be loaded and that there is
enough storage to load the module.
ISPI013 The GWI parameter is not
supported in the foreground
environment.
Explanation
The GWI parameter was used when starting ISPF in
the foreground environment. This is not supported.
User response
Do not specify the GWI parameter.
ISPI021 ISPI021 Unrecoverable error in
initialization of
Explanation
A problem exists with the initialization of the function
listed with this message.
System programmer response
Possible bad data set in the allocations shown in the
message. If initialization continues to fail, contact IBM
for assistance.
User response
Contact your system programmer.
ISPI022 ISPI022 ISPF cannot continue.
ISPF abending.
Explanation
A severe error occurred when ISPF was initializing.
This message follows ISPI021 and can be eliminated
when the problem described by the first message is
corrected.
System programmer response
Correct the problem described by ISPI021.
ISPF messages starting with ISP
148  z/OS: z/OS ISPF Messages and Codes

## Page 169

User response
Contact your system programmer.
ISPI023 ISPI023 ENVIRON TERMTRAC
definition OFF, buffer could not be
obtained
Explanation
Storage was not available for creating the buffer and
thus ENVIRON TERMTRAC is not available.
System programmer response
Determine why storage is not available.
User response
Contact your system programmer.
ISPI024 ISPI024 TSO module IRXECUSP
could not be loaded
Explanation
REXX could not be initialized.
System programmer response
Determine if IRXECUSP exists and why the load fails.
Contact IBM support for further assistance.
User response
Contact your system programmer.
ISPI025 ISPI025 TSO routine IRXINIT
severe error - REXX environment
could not be initialized.
Explanation
A severe error occurred while initializing the REXX
environment.
System programmer response
Determine why the REXX environment can not be
initialized. Contact IBM support for further assistance.
User response
Contact your system programmer.
ISPI026 ISPI026 Attempt made to exceed
total lines in display.
Explanation
ISPF display processing has detected an attempt
to exceed the total number of lines in the display.
Proceeding could result in destructive processing.
Instead ISPF issues this message along with abend
code 999.
System programmer response
Contact IBM for assistance.
User response
Contact your system programmer.
ISPJ001 Load error - 'aaaaaaaa' load
module not found.
Explanation
The translation table load module aaaaaaaa for the
parameter used when invoking ISPTTDEF was not
found.
System programmer response
Ensure that the translate table load module exists and
that the load module is correctly allocated. Refer to
z/OS ISPF Dialog Developer's Guide and Reference for
related information on invoking ISPTTDEF.
User response
Check the parameter for correct spelling and that
the terminal type is listed on the option 0.1 panel.
If the parameter is correct, contact your system
programmer.
ISPJ002 Load error - 'aaaaaaaa' translate
table load module not found or in
error
Explanation
The translation table load module aaaaaaaa specified
for a terminal type of OTHER was not found or
validation failed when checking that the module
consisted of translation tables.
System programmer response
Ensure that the translate table load module exists and
that the load module is correctly allocated.
User response
Check the value entered for correct spelling. If the
value is correct, contact your system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  149

## Page 170

ISPL010 Invalid variable name - LIST
service variable name, 'aaaaaaaa',
is syntactically incorrect.
Explanation
Specified variable name aaaaaaaa is syntactically
incorrect.
User response
Enter a syntactically correct variable name. Dialog
variables are referred to symbolically by a name
composed of 1 to 8 characters (6, for FORTRAN).
Alphanumeric characters A-Z, 0-9, #, $, or @ can be
used in the name, but the first character cannot be
numeric. APL variable names cannot contain #, $, or
@.
ISPL011 Variable not found - LIST service
dialog variable, 'aaaaaaaa', does
not exist.
Explanation
The specified LIST service dialog variable aaaaaaaa
was not found.
User response
Check the specified dialog variable name for incorrect
spelling, and verify that the dialog variable name
exists.
ISPL012 Invalid line length - Minimum LIST
service line length is one when CC
is specified.
Explanation
The specified LINELEN value for the LIST service is
incorrect.
User response
Verify that the LINELEN value contains 1 byte for the
carriage control character when using the LIST service
with CC specified.
ISPL013 Invalid variable value - LIST
service variable, 'aaaaaaaa',
cannot be null when CC is
specified.
Explanation
The specified LIST service dialog variable aaaaaaaa
contains a null value and CC option has been specified
for the LIST service.
User response
Ensure that the specified dialog variable contains a
value other than null when specifying the LIST service
with the CC option.
ISPL014 Invalid line length - LIST service
line length must be greater than or
equal to 0.
Explanation
The LINELEN value for the LIST service contains a
value less than zero.
User response
Ensure that the line length is greater than zero. The
LINELEN value specifies the length of each line in
the buffer being passed to ISPF. ISPF truncates these
lines if the line length specified is greater than the
truncation value in system variable ZLSTTRUN. The
line length must have an unsigned integer value and,
for a call, must be a full word fixed integer.
ISPL015 LIST data set error - Severe error
occurred when allocating/opening
the LIST data set.
Explanation
An error occurred when allocating or opening the LIST
data set.
System programmer response
Verify the correct allocation of the LIST data set.
Contact IBM support for further assistance.
User response
Notify system programmer of the message and the
return code.
ISPL016 LIST not available - Error
previously encountered when
allocating the LIST data set.
Explanation
A previous allocation error exists for the LIST data set.
System programmer response
Verify the correct allocation of the LIST data set.
Contact IBM support for further assistance.
ISPF messages starting with ISP
150  z/OS: z/OS ISPF Messages and Codes

## Page 171

User response
Contact your system programmer.
ISPL017 LIST not available - Error
previously encountered when
opening/writing to the LIST data
set.
Explanation
A previous error exists for opening or writing to the
LIST data set.
System programmer response
A previous OPEN/PUT DCB abend exists and needs to
be corrected. Verify the correct allocation and status
of the LIST data set. Contact IBM support for further
assistance.
User response
Contact your system programmer.
ISPL018 LIST data set error - Severe error
occurred when writing to the LIST
data set.
Explanation
A severe error occurred when attempting to WRITE to
the LIST data set.
System programmer response
The LIST data set may be closed or a DCB abend may
have occurred against the LIST data set. Ensure that
the LIST data set allocation is correct. Contact IBM
support for further assistance.
User response
Contact your system programmer.
ISPL030 Command not allowed - aaaaaaaa
not allowed while aaaaaaaa
disposition or termination panel is
active.
Explanation
The LOG/LIST command cannot be issued on the
active LOG/LIST disposition panel, or when the
TERMINATION panel is active.
User response
The user must exit from the panel in order to perform
the specified command.
ISPL031 Data set not active - The aaaaaaaa
data set is not allocated or has not
been used this session.
Explanation
The LOG or LIST data set is not allocated, or has not
been used this session.
User response
ISPF will allocate the LOG or LIST data set when
log or list information is first generated during a
session, or first generated after the data sets have
been processed. The user can preallocate the LOG or
LIST data sets prior to invoking ISPF. More information
on ISPF log and list data sets can be found in ISPF
User's Guide.
ISPL032 Command not active - The
aaaaaaaa data set cannot be
processed when it has been
preallocated.
Explanation
The LOG or LIST commands are not active if the LOG or
LIST data sets are preallocated.
User response
The user may preallocate both the LOG and LIST data
sets; however, the LOG and LIST commands cannot
be issued to process a preallocated LOG or LIST data
set. Refer to ISPF Planning and Customizing for further
information on preallocated data sets.
ISPL033 aaaaaaaa job not submitted -
aaaaaaaa print job cannot be
submitted while file tailoring is
active.
Explanation
The LOG or LIST print job cannot be submitted while
file tailoring is active.
User response
Reissue LOG or LIST print command when file tailoring
is complete.
ISPL034 aaaaaaaa data set in use -
aaaaaaaa data set cannot be
processed while data set is in use.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  151

## Page 172

Explanation
The LOG data set is currently in use and cannot be
processed.
User response
The LOG cannot be processed when using option 7.5
Dialog Test to display the LOG data set. Exit option 7.5
to process the LOG data set.
ISPL035 Invalid aaaaaaaa command -
Valid aaaaaaaa parameters are
PRINT, KEEP, DELETE, or blank.
Explanation
An invalid LOG or LIST command parameter was
issued.
User response
Enter a valid LOG or LIST command parameter. Valid
parameters are PRINT, KEEP, DELETE, or BLANK.
ISPL036 Invalid command - First parameter
to ISPLLP must be LOG or LIST.
Explanation
An invalid ISPLLP command parameter was issued.
User response
The first parameters to ISPLLP must be either LOG or
LIST.
ISPL101 LIBDEF display error - The
ISPLIBD utility detected an
unexpected decimal return code
aaaaaaaa from the "bbbbbbbb"
service.
Explanation
The dialog utility, ISPLIBD, received an error from the
interface routine between dialog modules and ISPF.
System programmer response
Check the return codes to determine the error.
User response
Contact your system programmer.
ISPL102 Enter aaaaaaaa parameter - Enter
a specific library name for the
aaaaaaaa command.
Explanation
You must enter a library name when issuing the
LOCATE command for the LIBDEF display utility.
User response
Enter the name of the library you wish to locate. The
library name must be 1 to 8 characters in length and
cannot be ISPPROF.
ISPL103 Invalid library name - "aaaaaaaa"
is not a valid LIBDEF library name.
The library name must be 1 to
8 characters long and cannot be
ISPPROF.
Explanation
The LOCATE command of the LIBDEF Display Utility
requires a valid LIBDEF library name of 1 to 8
characters in length, and it cannot be ISPPROF.
User response
Reenter the LOCATE command with a valid LIBDEF
library name.
ISPL104 Invalid command - "aaaaaaaa"
is not a valid command for the
ISPLIBD utility.
Explanation
An invalid command was entered. The LIBDEF Display
Utility supports the LOCATE command, and two
abbreviations, LOC and L. The LOCATE command is
used to locate a specific library name. For example,
LOCATE ISPPLIB locates the LIBDEF definition for
ISPPLIB.
User response
Reenter the command using LOCATE, LOC, or L and the
library name.
ISPL105 Library name invalid - "aaaaaaaa"
is not a valid LIBDEF library name.
The library name must be 1 to
8 characters long and cannot be
ISPPROF. The value is ignored.
Explanation
The ISPLIBD command of the LIBDEF Display Utility
requires a valid LIBDEF library name of 1 to 8
characters in length, and it cannot be ISPPROF.
ISPF messages starting with ISP
152  z/OS: z/OS ISPF Messages and Codes

## Page 173

User response
Reenter the ISPLIBD command with a valid LIBDEF
library name.
ISPL106 ISPLIBD recursion - The ISPLIBD
command may not be issued
recursively within the same logical
screen.
Explanation
You entered the ISPLIBD command more than once
within the same logical screen. The ISPLIBD command
may not be issued more than once within the same
logical screen.
User response
Issue the ISPLIBD command only once per logical
screen.
ISPLO999 Licensed Materials - Property
of IBM 5694-A01 Copyright
IBM Corp. 1980, 2013. All
rights reserved. US Government
Users Restricted Rights -
Use, duplication or disclosure
restricted by GSA ADP Schedule
Contract with IBM Corp.
Explanation
This is an informational message.
ISPN101 Storage release error -
Error occurred when releasing
application library blocks.
Explanation
An error occurred trying to FREEMAIN the skeleton
library block with the ddname, ISPSLIB.
System programmer response
Diagnose the FREEMAIN error that occurred when
releasing storage for the skeleton library block,
ISPSLIB. Contact IBM support.
User response
Contact your system programmer.
ISPN102 Storage obtain error - Error
occurred when obtaining storage
for application library blocks.
Explanation
An error occurred when obtaining storage for
application library blocks.
System programmer response
Diagnose the GETMAIN error which occurred when
obtaining storage. Contact IBM support.
User response
Contact your system programmer.
ISPN103 Invalid LIBDEF type - ISPPROF is
not a valid LIBDEF type.
Explanation
An application-level definition for ISPPROF, the ISPF
profile library, is not permitted, because ISPPROF
contains user-related data.
User response
ISPPROF is an invalid LIBDEF type. Enter a valid
LIBDEF type such as the ISPF ddname libraries:
ISPMLIB, ISPPLIB, ISPSLIB, ISPTLIB, ISPTABL,
ISPFILE, ISPLLIB, or a valid generic library name.
ISPN104 Invalid dsname list - Data set
name(s) incorrectly specified.
Explanation
The data set name or names are incorrectly specified.
User response
Correct spelling and statement format errors.
ISPN105 Multiple ddnames - Only one
ddname may be specified when
LIBRARY is specified.
Explanation
The LIBRARY keyword on a LIBDEF service request
associates a allocated ddname with an ISPF lib-type.
This is a one-to-one correspondence, thus only one
ddname can be specified using the LIBRARY keyword
on a LIBDEF service request.
User response
Remove all but one of the specified ddnames from the
LIBDEF statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  153

## Page 174

ISPN106 Ddname not allocated - Ddname
'aaaaaaaa' specified on LIBDEF
service not allocated.
Explanation
Ddname aaaaaaaa is not allocated.
User response
Ensure correct spelling of the ddname. Allocate the
specified ddname prior to entering ISPF.
ISPN107 Member name specified - Data set
member specification is invalid.
Explanation
The LIBDEF service provides a data set-list parameter.
It does not support specifying member names of data
sets. You can specify a single data set or a list of data
sets.
User response
Remove the member name specification from the data
set name specified on the LIBDEF service statement.
ISPN108 Invalid dsname framing - Dsname
specification contains inconsistent
framing characters.
Explanation
The data set-list parameter on the LIBDEF service has
been coded incorrectly. The data set name should be
specified with a starting and ending single quote. For
example:
DATASET('PROJECT.LEVEL.USER1')
A list of data set names must be separated by blanks
or commas. For example:
DATASET('PROJECT.LEVEL.USER1','PROJECT.LEVEL.USE
R2')
User response
Correct the quotation marks for the specified data set
name.
ISPN109 Invalid dsname length - Data set
name specification cannot exceed
44 characters.
Explanation
The data set name specified is longer than 44
characters. Data set names must conform to TSO data
set naming conventions.
System programmer response
Rename the specified data set if the name exceeds 44
characters. Contact IBM support.
User response
Verify the spelling of the specified data set name.
If the spelling is correct, contact your system
programmer.
ISPN110 Invalid library ddname -
EXCLDATA or EXCLLIBR must be
specified with ISPLLIB.
Explanation
EXCLDATA or EXCLLIBR keyword must be specified
with ISPLLIB library type.
User response
Only ISPLLIB library type may be used for the
EXCLDATA or EXCLLIBR LIBDEF keywords.
ISPN111 Deallocation failure - Deallocation
failure - check allocated data sets.
Explanation
This message is self explanatory.
ISPN112 Invalid volume serials - Volume
serials operand incorrectly
specified.
Explanation
An invalid volume serial number was specified on the
ISPPREP command.
User response
Correct the specified volume serial number.
ISPN113 Too many parameters - Too
many parameters specified on the
LIBDEF service.
Explanation
Too many parameters are specified on the LIBDEF
service.
ISPF messages starting with ISP
154  z/OS: z/OS ISPF Messages and Codes

## Page 175

User response
Correct the specified parameters for the ISPPREP
service call or the LIBDEF service call, or both.
ISPN114 Authorization failure -
Authorization failure from LIBDEF
exit point.
Explanation
There is an authorization failure from user LIBDEF
service exit point.
System programmer response
Correct the authorization failure for LIBDEF service
exit.
User response
Contact your system programmer.
ISPN115 Severe error - Severe error from
LIBDEF exit point.
Explanation
A severe error was returned from the user's LIBDEF
service exit.
System programmer response
Correct the LIBDEF service exit error.
User response
Contact your system programmer.
ISPN116 Severe error - Maximum number
of 15 data set names has been
exceeded by 'aaaaaaaa' when
using LIBDEF STKADD parameter.
Explanation
A severe error was returned from the LIBDEF service.
The number of data sets exceeded the maximum of
15 when issuing the LIBDEF DATASET service with the
STKADD parameter.
User response
Contact your system programmer.
Programmer response
Reduce the number of data set names in the list
on the LIBDEF DATASET service using the STKADD
parameter. Reduce the data set names by the
"exceeded by" number supplied in the message.
ISPN117 Severe error - The LIBDEF STKADD
parameter is invalid when used
with the EXCLDATA, LIBRARY, or
EXCLLIBR parameters. STKADD
is only valid with the DATASET
parameter.
Explanation
A severe error was returned from the LIBDEF service.
The parameter STKADD is for use with the DATASET
parameter only.
User response
Contact your system programmer.
Programmer response
Correct the LIBDEF service call that incorrectly used
the STKADD parameter with the parameter LIBRARY,
EXCLDATA, or EXCLLIBR parameter.
ISPO001 Selection panel error -
"aaaaaaaa." is invalid parameter
for program ISPOPT on panel
"bbbbbbbb".
Explanation
There is an invalid parameter, aaaaaaaa, for program
ISPOPT on panel bbbbbbbb.
User response
Contact the responsible programmer.
Programmer response
Correct the parameter on the selection panel.
ISPO002 Inconsistent parameters - 24
function keys are valid only for
3278, 3278CF, 3278KN, 3290
terminal types.
Explanation
This message is self explanatory.
ISPO003 Invalid pad character - Input
field pad character and command
delimiter may not be the same.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  155

## Page 176

Explanation
The input field pad character will be used to fill
input fields on a panel. It must be different than the
command delimiter and it cannot be a-z, A-Z, 0-9 or /,
except N and B can be used to indicate nulls and
blanks respectively.
User response
Enter a character that is different than the command
delimiter character.
ISPO004 Invalid value - Press Help for
tutorial page giving valid options.
Explanation
Valid values for the number of PFKEYS are 12 or 24.
For more information about PFKEYS review the Help
information.
User response
Enter a valid value after reviewing the Help
information.
ISPO005 Inconsistent parameters - Part
may not be specified with 3278T.
Explanation
The screen format of PART (format using hardware
partitions) cannot be used for terminal type 3278T.
The screen format of PART is valid only for the 3290
terminal type. Formats valid for 3278T terminal types
are: STD and MAX.
User response
Enter a screen format other than Part for terminal type
3278T.
ISPO006 Display format invalid - Display
format must be NO, SHORT, or
LONG.
Explanation
For a keylist, the display format options are: 1.)LONG -
indicates that the key label should be displayed in the
function key area when the FKA command is toggled
to the first cycle after OFF. 2.)SHORT - indicates that
the key label should be displayed in the function key
area when the FKA command is toggled to the first
or second cycle after OFF. 3.) NO - indicates that the
key label should never be displayed in the function key
area.
User response
Enter one of the correct Display Format values: NO,
SHORT, or LONG.
ISPO007 F key value required - A function
key must be assigned a value in
order to have a display format.
Explanation
When working with keylists, if a display format or a
label is specified a definition must also be specified.
Any definition is valid.
User response
Assign a value to the function key.
ISPO008 F key value required - A function
key must be assigned a value in
order to have a label.
Explanation
When defining or changing keylists, if a label or a
display format is specified for the function key, the
function key must have a definition. Any definition is
valid.
User response
Assign a value (definition) to the function key.
ISPO009 Invalid selection - Enter a keylist
name or select an existing keylist.
Explanation
You must enter a new keylist name or an existing
keylist name and then select the action.
User response
Enter or select a keylist, then select an action.
ISPO010 Keylist does not exist - The keylist
selected does not exist; therefore,
it cannot be changed.
Explanation
The selected keylist does not exist for this application
ID
User response
Create a new keylist for this application ID, or select
the application ID that contains the required keylist.
ISPF messages starting with ISP
156  z/OS: z/OS ISPF Messages and Codes

## Page 177

ISPO011 Keylist already exists - The keylist
name entered already exists;
therefore, it cannot be created.
Explanation
When creating a keylist, you have entered a keylist that
already exists. To create a keylist, enter the name of a
keylist that is not in the displayed list.
User response
Enter a valid keylist name that does not already exist.
ISPO012 Keylist not saved - The keylist
changes were not saved or the
keylist was not created.
Explanation
Either the changes to the keylist were not saved, or the
keylist was not created.
User response
Either select the Save action to save the keylist
changes, or create a new keylist.
ISPO013 Keylist not selected - Enter a
keylist name or select one from
the list and select an action.
Explanation
To create, change, delete, or browse a keylist from
the Keylist Utility, enter the keylist name or select
a keylist name from the displayed list. Next, select
create, change, delete, or browse from the Functions
pull-down.
ISPO014 Keylist does not exist - The keylist
selected does not exist; therefore,
it cannot be browsed.
Explanation
The keylist does not exist for this application ID.
User response
Either create the required keylist, or select the
application ID under which the keylist exists.
ISPO015 Keylist in use - The selected
keylist is currently being updated
by you.
Explanation
You have selected a keylist that you are currently
updating.
User response
Finish updating the keylist and save it before
performing another action on the specified keylist.
ISPO016 Delete Warning - This keylist
belongs to a currently running
application. If this keylist is
deleted, a dialog error can occur
if a current application panel
references this keylist.
Explanation
A keylist for a currently running application cannot be
deleted.
User response
To delete the keylist, press Enter on the confirmation
panel. If you do not want to delete the keylist, press
the Cancel key.
ISPO017 Keylist tables in use - The
internal tables used by keylist are
presently in use.
Explanation
The KEYLIST command may not be issued from the
Keylist Utility. However, you can split the screen and
issue the KEYLIST command on the other screen.
User response
The KEYLIST command may not be issued while
actions are being performed on keylist from a previous
KEYLIST command. Splitting the screen allows the
user to access the KEYLIST command on the other
screen.
ISPO018 Invalid Action - The action
selected is not defined for the
keylist utility.
Explanation
An invalid action was selected. Select one of the
valid actions displayed on the informational line of the
keylist panel.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  157

## Page 178

User response
Enter a valid selection from the action bar or enter N,
E, V, or D as the selection character to perform a New,
Edit, View, or Delete action.
ISPO019 Keylist saved - The keylist changes
were saved or the keylist was
created.
Explanation
This is an informational message.
ISPO020 Invalid keylist name - The
specified keylist name is invalid.
Explanation
The keylist name that was specified is invalid.
User response
Enter a valid keylist name.
ISPO021 Updates not allowed - No
modifications will be accepted
while editing or browsing a keylist.
Explanation
No modifications to the profile will be accepted while
editing or browsing a keylist. Updates are only allowed
if private keylists are enabled in the function keys
settings. This can be determined by going to ISPF
option 0 - settings, selecting the function keys action
bar, and seeing whether the choice 'Use private and
shared' or the choice 'Use only shared' has an '*' in
front of it, designating it as the active option.
User response
Finish editing or browsing the keylist before updating
the profile. Change the function keys settings (ISPF
option 0, function keys action bar) to both private and
shared if a modified version of a keylist is wanted. This
will enable a private version of a keylist to be created
when modifications are made.
ISPO022 Input command error - Command
entered is not recognized.
Explanation
When updating a command table from the Command
Table Utility, you entered a command which was not
recognized.
User response
Enter a valid command. Enter HELP for a list of valid
commands, or refer to ISPF User's Guide .
ISPO023 Keylist not altered - The keylist
was not altered, therefore it is not
saved.
Explanation
This is an informational message. The keylist was not
altered.
ISPO024 Keylist not deleted - The keylist
was not found so it cannot be
deleted.
Explanation
The keylist was not found under this application ID.
User response
Select the appropriate application ID from which to
delete the keylist.
ISPO025 Keylist not deleted - The keylist
selected is not a private copy of
the keylist.
Explanation
The keylist selected is a shared keylist, not a private
copy, and cannot be deleted using KEYLIST.
User response
Only private copies of the keylist can be deleted using
KEYLIST.
ISPO026 Profile not created - The profile
table could not be created and
is required for keylist. ****
aaaaaaaa ****
Explanation
The Keylist Utility could not create a profile table,
ISPPROF. A profile table is necessary when working
with keylists.
System programmer response
Determine the reason the profile was not created.
Contact IBM support.
ISPF messages starting with ISP
158  z/OS: z/OS ISPF Messages and Codes

## Page 179

User response
Note message ID and text. Contact your system
programmer.
ISPO027 Keylist not saved - Internal error
code aaaaaaaa resulted in the
keylist not being saved. ****
bbbbbbbb ****
Explanation
ISPF's attempt to save the keylist resulted in an error
and the keylist was not saved.
System programmer response
Contact IBM support.
User response
Note the error message and text. Contact your system
programmer.
ISPO028 Invalid Help name - The specified
keylist Help panel name is invalid.
Explanation
The specified keylist Help panel name is invalid.
System programmer response
Check and correct the GML source code as needed.
Contact IBM support.
User response
Check the GML source code for the ISPDTLC generated
keylist, and correct the Help panel name. Contact your
system programmer.
ISPO029 Keylist deleted - The selected
keylist was deleted.
Explanation
This is an informational message. A request was made
to delete a keylist and the delete was performed.
ISPO030 Invalid color - Enter valid color of:
BLUE, RED, PINK, GREEN, TURQ,
YELLOW, or WHITE.
Explanation
An invalid COLOR was entered.
User response
Enter one of the correct COLOR responses as listed in
the message.
ISPO031 Invalid intensity - Enter valid
intensity value of: LOW or HIGH.
Explanation
An invalid INTENSITY value was entered.
User response
Enter one of the correct INTENSITY responses as
listed in the message.
ISPO032 Invalid highlighting - Enter
valid highlight value of: BLINK,
REVERSE, NONE, or USCORE.
Explanation
An invalid HIGHLIGHT value was entered.
User response
Enter one of the correct HIGHLIGHT responses as
listed in the message.
ISPO033 Invalid ISPFVAR parm - The
ISPFVAR command only accepts
the ABTAB, PSTAB, JUMP, EDPRT,
SESM, LMSG, and SPLTLINE
parameters. The values may
be set by specifying (ON) or
(OFF) immediately after the
parameter. For example, ISPFVAR
ABTAB(ON).
Explanation
The ISPFVAR command was entered with an invalid
parameter.
User response
The only values accepted are ABTAB PSTAB, JUMP,
EDPRT, SESM, LMSG, and SPLTLINE. If the user
specifies a value of (ON) or (OFF), the option is turned
on or off. If no value is specified, the current return
code is set to 0 if the value is off and 1 if the value is
on.
ISPO034 Keylist not deleted - The keylist
was not deleted due to user's
request.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  159

## Page 180

Explanation
This is an informational message. On the confirmation
dialog prompt, the user requested that the keylist not
be deleted. That request was honored.
ISPO035 Select a file action - Select a file
action bar item to set a default
"/" action. The default action will
remain set until a different file
action is selected, the keylist view
is changed, or keylist utility is
terminated.
Explanation
A default "/" was entered, but no file action was
selected.
User response
Select a file action bar item, to set a default action to
perform.
ISPO901 Enter required field - Enter
required field at the cursor
position.
Explanation
The Terminal type field was left blank.
User response
Enter a valid choice for terminal type.
ISPO902 Valid input field pad values are
anything but A-Z, 0-9 and /,
except N which indicates nulls
and B which indicates blanks. It
must also be different than the
command delimiter.
Explanation
An invalid pad value was entered.
User response
Enter one of the correct values shown.
ISPO903 Valid command delimiters are
anything but A-Z and 0-9 and = or .
Explanation
An invalid command delimiter has been entered.
User response
Enter one of the values listed.
ISPO904 Invalid value - You must select
'Panel display CUA mode' on a
client that is using the JSON API.
Explanation
You must select Panel display CUA mode on a client
that is using the JSON API.
User response
Enter a / in the Panel display CUA mode field.
ISPO912 Enter required field - Enter user ID
default at the cursor position.
Explanation
The user ID default was left blank.
User response
Enter a valid choice for the user ID default.
ISPO913 Valid values: FBA or VBA
Explanation
An invalid value has been entered.
User response
Enter a correct value as shown.
ISPO914 Valid values: 1 to 999 for FBA and
5 to 999 for VBA
Explanation
An invalid value has been entered.
User response
Enter a correct value as shown.
ISPO915 Valid values: 80 to 160
Explanation
An invalid value has been entered.
User response
Enter a correct value as shown.
ISPF messages starting with ISP
160  z/OS: z/OS ISPF Messages and Codes

## Page 181

ISPO916 Changing the "Always show split
line" setting has no effect in
partition mode.
Explanation
The user changed a setting that has no effect in
partition mode.
ISPO919 Valid number of keys are: 12 or 24
Explanation
An invalid value has been entered.
User response
Enter a correct value as shown.
ISPO926 Enter required field - Enter
system name default at the cursor
position.
Explanation
The system name default was left blank.
User response
Enter a valid choice for the system name default.
ISPO930 Enter required field - Enter
message identifier default at the
cursor position.
Explanation
The message identifier default was left blank.
User response
Enter a valid choice for the message identifier default.
ISPO931 Enter required field - Enter panel
identifier default at the cursor
position.
Explanation
The panel identifier default was left blank.
User response
Enter a valid choice for the panel identifier default.
ISPO932 Enter required field - Enter
screen name default at the cursor
position.
Explanation
The screen name default was left blank.
User response
Enter a valid choice for the screen name default.
ISPO936 Terminal type warning - The
terminal type you have selected
may be incompatible with the
current ISPF language setting and
can lead to unpredictable results.
Explanation
The terminal type selected might not apply to your
installation. To assure correct operation of ISPF, the
terminal type must be correct as it determines the
character set for your terminal.
User response
Make sure the terminal type is correct.
ISPP000 ISPF Log continued - - - Session #
aaaaaaaa
----------------------------------------
---------------
Explanation
This is an informational message. This message is
placed at the beginning of a continued ISPF log file
when the session number is available.
ISPP001 Start of ISPF Log - - - - Session #
aaaaaaaa
----------------------------------------
---------------
Explanation
This is an informational message. This message is
placed at the beginning of a new ISPF log file when
the session number is available.
ISPP002 End of ISPF Log - - - - - Session #
aaaaaaaa
----------------------------------------
---------------
Explanation
This is an informational message. This message is
placed at the end of the ISPF log file when the ISPF
session is ended.
ISPP003 ISPF Log continued - - - Session #
not available
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  161

## Page 182

----------------------------------------
------
Explanation
This is an informational message. This message is
placed at the beginning of a continued ISPF log file
when the session number is not available.
ISPP004 ***** Dialog Error *****
- Application(aaaaaaaa);
Function bbbbbbbb (cccccccc);
Service(dddddddd)
Explanation
This is an informational message. This message is
placed in the ISPF log file when a dialog error occurs.
ISPP005 Line from panel: - aaaaaaaa
Explanation
This is an informational message. This message is
added to the ISPF log file when a dialog error is found
in the panel syntax. 'aaaaaaaa' contains the panel line
where the error is found.
ISPP006 Line from cmd: - aaaaaaaa
Explanation
This is an informational message. This message is
added to the ISPF log file when a dialog error pertains
to the command syntax. 'aaaaaaaa' contains the line
where the error is found.
ISPP007 Line from skeleton: - aaaaaaaa
Explanation
This is an informational message. This message is
added to the ISPF log file when a dialog error occurs in
skeleton processing. 'aaaaaaaa' contains the skeleton
line where the error is found.
ISPP008 Trace: Command line - 2ND LEVEL
MSG NOT USED!!!, INSERTED
DIRECTLY IN TLD BY SEP
Explanation
This is an informational message.
ISPP009 Start of ISPF Log - - - - Session #
not available
----------------------------------------
------
Explanation
This is an informational message. This message is
placed at the beginning of a new ISPF log file when
the session number is not available.
ISPP010 Value out of range - Enter 0 or a
numeric value between aaaaaaaa
and bbbbbbbb
Explanation
The entered value is neither 0 nor in the range of valid
values.
User response
Enter an appropriate value in the field pointed to by
the cursor.
ISPP011 ISPF subtask abend - Completion
code = aaaaaaaa (No other
information available.)
Explanation
The subtask failed.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPP012 ISPF subtask abend - VS
aaaaaaaa ISPF bbbbbbbb System
abend code = cccccccc (HEX) PSW
dddddddd eeeeeeee
Explanation
The task failed.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPP013 Name aaaaaaaa EPA bbbbbbbb
ISPSUBS EPA cccccccc ISPTSI
addr dddddddd
Explanation
This is an informational message.
ISPF messages starting with ISP
162  z/OS: z/OS ISPF Messages and Codes

## Page 183

ISPP014 Registers at - R0 aaaaaaaa
R1 bbbbbbbb R2 cccccccc R3
dddddddd
Explanation
This is an informational message.
ISPP015 Entry to - R4 aaaaaaaa
R5 bbbbbbbb R6 cccccccc R7
dddddddd
Explanation
This is an informational message.
ISPP016 Abend - R8 aaaaaaaa R9
bbbbbbbb R10 cccccccc R11
dddddddd
Explanation
This is an informational message.
ISPP017 R12 aaaaaaaa R13 bbbbbbbb R14
cccccccc R15 dddddddd
Explanation
This is an informational message.
ISPP018 Reason code associated with
abend = aaaaaaaa
Explanation
This is an informational message.
ISPP019 ISPF subtask abend - VS
aaaaaaaa ISPF bbbbbbbb User
abend code = cccccccc (DECIMAL)
PSW dddddddd eeeeeeee
Explanation
This is an informational message.
ISPP020 aaaaaaaa data set kept -
bbbbbbbb cccccccc has been kept.
Explanation
This is an informational message.
ISPP021 aaaaaaaa data set deleted -
bbbbbbbb cccccccc has been
deleted.
Explanation
This is an informational message.
ISPP022 aaaaaaaa print job submitted -
bbbbbbbb cccccccc will be printed/
deleted by job dddddddd.
Explanation
This is an informational message.
ISPP022B aaaaaaaa print job submitted -
bbbbbbbb cccccccc will be printed
and kept by job dddddddd.
Explanation
This is an informational message.
ISPP023 aaaaaaaa data set printed -
bbbbbbbb cccccccc has been
queued to dddddddd and deleted.
Explanation
This is an informational message.
ISPP023B aaaaaaaa data set printed -
bbbbbbbb cccccccc has been
queued to dddddddd and kept.
Explanation
This is an informational message.
ISPP024 aaaaaaaa print job error -
bbbbbbbb cccccccc has been kept
because of submit error.
Explanation
This message is self-explanatory.
ISPP025 aaaaaaaa not printed. - ISPP025
- ISPF Print RC = bbbbbbbb.
See ISPF Messages and Codes.
cccccccc dddddddd has been kept.
Explanation
Possible return codes are:
2
The user canceled local print.
4
ISPF issued the TSO PRINTDS command, but the
user pressed ATTENTION during PRINTDS.
6
Local print has been disabled by the system
programmer. Your system programmer has set
the LOCALPRT field in ISRCNFIG to blank, which
disables local print.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  163

## Page 184

8
ISPF issued the TSO PRINTDS command, but TSO
PRINTDS abended. Further information may be
available from TSO PRINTDS by setting WTPMSG
and MSGID in your TSO profile.
12
TSO PRINTDS had a return code greater than zero.
Further information may be available from TSO
PRINTDS by setting WTPMSG and MSGID in your
TSO profile.
16
Unexpected RC from local print.
System programmer response
Verify that the local print function is set up correctly.
User response
Contact your system programmer.
ISPP026 aaaaaaaa bbbbbbbb has been
kept.
Explanation
This is an informational message.
ISPP027 aaaaaaaa bbbbbbbb was
preallocated (no free was done).
Explanation
This is an informational message.
ISPP028 Print utility exit - The print
utility exit produced a return code
of aaaaaaaa when processing
bbbbbbbb cccccccc.
Explanation
This is an informational message.
ISPP028B Print utility exit - The print utility
exit produced a return code of
aaaaaaaa on request for batch job
bbbbbbbb.
Explanation
This is an informational message.
ISPP031 Specify required field - Specify
either the batch sysout class or the
printer ID or the writer-name.
Explanation
In order for ISPF to determine the destination of the
output, the batch sysout class OR the printer ID OR the
writer-name is required.
User response
Specify either the batch sysout class or the printer ID
or the writer-name.
ISPP032 Duplicate specification - For Batch
output, specify the batch sysout
class. For local output, specify
either the printer ID or the writer-
name.
Explanation
In order for ISPF to determine the destination of the
output, the batch sysout class OR the (printer ID or
writer-name) is required.
User response
Specify either the batch sysout class or the (printer ID
or writer-name), not both.
ISPP033 Profile sharing active - The
additional qualifier for ISPF
temporary data sets, by default
set to ISP&aaaaaaaa by the
Profile Sharing facility, might
prevent the reallocation of the
same Log/List data sets in the next
session.
Explanation
In order to keep the temporary data sets, including
the Log and List data sets unique, ISPF will insert the
additional temporary data set qualifier, specified in the
ISPF Configuration Utility, in the data set name. By
default, this qualifier is set to ISP&&aaaaaaaa when
the Profile Sharing facility is active and will be different
for each TSO session.
User response
Either disable the Profile Sharing facility or make sure
the additional qualifier allows for reallocation of the
same Log/LIST datasets in the next session.
ISPP034 Check qualifier - The Multi-Logon
Profile Sharing feature is enabled
and an additional qualifier for
ISPF temporary data sets was
specified. Make sure that this
qualifier is unique within the
ISPF messages starting with ISP
164  z/OS: z/OS ISPF Messages and Codes

## Page 185

sysplex, else unpredictable results
might occur.
Explanation
In order to keep the temporary data sets, including
the Log/List and trace data sets unique when Profile
Sharing is active, an additional qualifier which is
unique within the sysplex should be used. By default,
this qualifier is set to ISP&&&SEQ which will be
different for each TSO session.
User response
Specify an additional temporary data set qualifier
which is unique within the sysplex or use the default
ISP&&&SEQ.
ISPP040 .ALARM not set - ALARM keyword
specified on short message but it
was not set.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP041 .HELP has no panel name - The
HELP keyword was specified on
short message but with no panel
name.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP043 No message ID was found - The
message ID was blank.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP044 Member not found - The member
that should contain the message
was not found.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP045 Invalid message ID - The last
character of the message ID was
invalid.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP046 Invalid message ID - The
message ID contains more than 5
characters.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  165

## Page 186

User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP047 Invalid member name - The
member name does not end in two
numerics.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP048 I/O error or end-of-file - An
I/O error or end-of-file was
encountered before message was
found.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP049 Leading quote missing - The
leading quote was missing from
either the long or short message.
Explanation
This message is self-explanatory.
User response
Contact the responsible programmer.
Programmer response
Refer to z/OS ISPF Dialog Developer's Guide and
Reference for a description of ISPF message syntax
and keywords.
ISPP100 Panel 'aaaaaaaa' error - Panel not
found.
Explanation
The requested panel is not found in the ISPPLIB
libraries. When modified or copied panels from prior
ISPF releases are used with a newer ISPF release, an
unpredictable error may occur.
Programmer response
Verify that:
• The panel name is spelled correctly.
• The expected libraries are allocated.
• The requested panel is in the expected library.
Only those panels shipped with a particular release
should be modified and used under that release.
This is because processing of the panel as defined
in the )PROC sections may have changed significantly
between releases.
ISPP101 Panel 'aaaaaaaa' error - Old style
menu format found.
Explanation
This message is self explanatory.
ISPP102 Panel 'aaaaaaaa' error - I/0 error
or other BLDL error in retrieving
panel.
Explanation
An I/0 error from BLDL occurred while attempting to
find the panel name.
System programmer response
Ensure that the data set containing the panel is usable.
Contact IBM support for additional assistance.
User response
Retry retrieving the panel. If another BLDL error
occurs, consult your system programmer.
ISPP103 Panel 'aaaaaaaa' error - Attribute
character found has already been
defined.
ISPF messages starting with ISP
166  z/OS: z/OS ISPF Messages and Codes

## Page 187

Explanation
A specific attribute character has been defined more
than once in the attribute section.
User response
Contact the responsible programmer.
Programmer response
Examine the panel attribute section and determine
which attribute character is defined more than once.
Ensure that each attribute character is defined only
once.
ISPP104 Panel 'aaaaaaaa' error - More than
bbbbbbbb attribute characters are
defined.
Explanation
The number of attribute characters defined exceeds
the maximum allowed, 127. This limit includes the
three default characters, attribute overrides, and
TBDISPL dual defaults. For action bar panels or panels
with scrollable areas, you can specify a maximum of
110 attribute characters. This is because ISPF uses
some attribute characters internally for action bar
panels or panels with scrollable areas.
Programmer response
Limit the number of defined attribute characters to a
maximum of 127.
ISPP105 Panel 'aaaaaaaa' error - DEFAULT
keyword out of order in )ATTR
section of panel.
Explanation
The DEFAULT keyword is not specified correctly on
the )ATTR section heading.
Programmer response
Check the DEFAULT keyword on the )ATTR header
statement. The )ATTR header statement allows only
one DEFAULT keyword.
ISPP106 Panel 'aaaaaaaa' error - Invalid
attribute keyword value.
Explanation
The keyword value specified for an attribute keyword
is not one of the valid keyword values. For example,
the attribute keyword INTENS has three valid keyword
values, HIGH, LOW, and NON.
Programmer response
Check the defined attribute keyword values, and
determine if they are valid for the specified keyword.
ISPP107 Panel 'aaaaaaaa' error - Invalid
attribute keyword value type (not
a literal or dialog var).
Explanation
The panel )ATTR section contains an invalid attribute
keyword value. The keyword value entered is not a
literal or a dialog variable.
Programmer response
Check the attribute keyword values for an invalid
value.
ISPP108 Panel 'aaaaaaaa' error - Right
parenthesis not found where
expected for keyword.
Explanation
A closing parenthesis (right parenthesis) is missing
from the keyword value of an attribute.
Programmer response
Check for a missing right parenthesis on attribute
keyword values in the attribute section.
ISPP109 Panel 'aaaaaaaa' error - Invalid
keyword found within the )ATTR
section.
Explanation
A keyword that was not valid was found in the )ATTR
section of a panel definition.
The valid keywords for the )ATTR section are:
AREA, ATTN, CAPS, COLOR, DATAMOD,
EXTEND, FORMAT, HILITE, INTENS,
JUST, NUMERIC, OUTLINE, PAD, PADC,
REP, SCROLL, SKIP, TYPE, USERMOD
Programmer response
Check the )ATTR section for invalid or missing
keywords.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  167

## Page 188

ISPP110 Panel 'aaaaaaaa' error - Attribute
keyword found before an attribute
character.
Explanation
Each statement in the panel attribute section, )ATTR,
must begin with a single character or 2-digit
hexadecimal code. The remainder of the statement
can contain keyword parameters.
Programmer response
Look for a keyword preceding the attribute.
For example, this statement produces this error:
PAD(' ') @ TYPE(INPUT) INTENS(HIGH)
The keyword, PAD(' '), should not precede the single
character attribute, @.
ISPP111 Panel 'aaaaaaaa' error - Illegal
characters or statement type
found in )ATTR section.
Explanation
An attribute statement contains illegal characters.
For example, this statement produces this error:
@ TYPE(INPUT) INTENS(HIGH))
The keyword parameters should be enclosed in
single parentheses. In the previous example, the
keyword parameter HIGH is followed by a double right
parenthesis.
Programmer response
Check the panel attribute section statements for illegal
characters or invalid keywords.
ISPP112 Panel 'aaaaaaaa' error - 3 distinct,
legal attribute characters not
found with DEFAULT keyword
Explanation
The DEFAULT keyword on the )ATTR or )BODY header
statement must specify exactly three distinct, legal
attribute characters.
Programmer response
Check the DEFAULT keyword characters on either or
both of the )ATTR and )BODY header statements for
missing or illegal characters.
ISPP113 Unable to get save area -
Additional storage needed is
unavailable for display processing.
Explanation
An attempt by ISPF to get storage for display
processing failed.
User response
Contact your system programmer.
ISPP114 Panel 'aaaaaaaa' error - REFRESH
is not permitted for a graphic area.
Explanation
The contents of fields within a graphic area cannot
be retrieved prior to a redisplay. On redisplay, the
variables within the graphic area are displayed as the
user last saw them.
Programmer response
The REFRESH statement is not valid for fields within
the graphic area. Remove the REFRESH statement, or
refresh only those fields outside the graphic area.
ISPP115 Panel 'aaaaaaaa' error -
Ampersand (&), blank, or null is
invalid as an attribute character.
Explanation
The attribute section contains one of the invalid
attributes.
Programmer response
Choose special (non-alphanumeric) characters for
attribute characters so they do not conflict with the
panel text. An ampersand (&), blank (hexadecimal
40), shift-out (hexadecimal 0E), shift-in (hexadecimal
OF), or null (hexadecimal 00) cannot be used as an
attribute character.
ISPP116 Panel 'aaaaaaaa' error - Unable
to process redisplay request, last
thing displayed not a panel.
Explanation
A valid previous panel does not exist, thus ISPF is
unable to process the redisplay request.
Programmer response
Check the previous panel definition for possible errors.
ISPF messages starting with ISP
168  z/OS: z/OS ISPF Messages and Codes

## Page 189

ISPP117 Panel 'aaaaaaaa' error - The
maximum of bbbbbbbb nested
panel definitions has been
exceeded.
Explanation
This message is self explanatory.
ISPP118 Panel 'aaaaaaaa' error - Invalid
cursor field/area specified as a
parameter.
Explanation
An invalid field or area was specified for cursor
position.
Programmer response
Check for an invalid field specified for cursor position.
For example, you cannot set the cursor to a dynamic
area name in the panel body.
ISPP119 Panel 'aaaaaaaa' error -
Attempting to redisplay a panel
that was never successfully
created.
Explanation
The panel contained an error that prevented ISPF from
creating the panel image. You cannot redisplay a panel
that was never created.
Programmer response
Correct the error that prevented ISPF from creating
the panel.
ISPP120 Panel 'aaaaaaaa' error - )BODY
section of panel not found when
expected.
Explanation
The panel definition sections must be used in this
order:
)CCSID
)PANEL
)ATTR
)ABC
)ABCINIT
)ABCPROC
)BODY
)MODEL
)AREA
)INIT
)REINIT
)PROC
)HELP
)END
Programmer response
Check the order in which you use the panel definition
sections. Ensure that they match the order described
in the explanation.
ISPP121 Panel 'aaaaaaaa' error - Panel
definition too large, greater than
screen size.
Explanation
The panel defined by the )BODY definition section
contains more lines than can be displayed on your
current terminal screen.
Programmer response
Reduce the number of lines in the )BODY definition
section to the maximum screen size. For example, if
your maximum terminal screen size is 25 lines, reduce
the number of lines in the )BODY definition section to
25 lines.
ISPP122 Panel 'aaaaaaaa' error - Invalid
statement or keyword on )BODY
heading statement.
Explanation
The )BODY header statement of a panel definition
contains a keyword or statement that is not valid. The
valid keywords for the )BODY header statement are:
ASIS
CMD
DEFAULT
EXPAND
FORMAT
KANA
LMSG
OUTLINE
SMSG
WIDTH
WINDOW
Programmer response
Check the )BODY header statement for an invalid
keyword or statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  169

## Page 190

ISPP123 Panel 'aaaaaaaa' error - The
command field length is greater
than 255.
Explanation
The defined command field length on the panel is
greater than 255 characters. The maximum command
field length is 255 characters.
Programmer response
Redefine the command field length on the panel to be
less than or equal to 255 characters.
ISPP124 Panel 'aaaaaaaa' error - Panel
record longer than given (or
defaulted) screen width to be
used.
Explanation
The length of a panel record exceeds the defined
screen width.
Programmer response
Adjust any panel records that exceed the screen width.
ISPP125 Panel 'aaaaaaaa' error - Duplicate
panel field (or area) name defined.
Explanation
The panel contains two fields or areas with the same
name.
Programmer response
Ensure that all the defined panel fields and areas have
unique variable names.
ISPP126 Panel 'aaaaaaaa' error - Panel
field name (or area name) exceeds
maximum length of bbbbbbbb..
Explanation
A panel field (or area name) is longer than the
maximum length allowed for panel field names (or
area names).
Programmer response
Ensure that the panel field names (or area names) do
not exceed the maximum length allowed.
ISPP127 Panel 'aaaaaaaa' error - Field
or area name missing following
identifying attribute character.
Explanation
The panel contains a defined area or field that does
not have a name following the attribute character for
that area or field.
Programmer response
Check for defined areas or fields with missing field
or area names following the identifying attribute
character.
ISPP128 Panel 'aaaaaaaa' error - Shadow
variable name must follow area
name.
Explanation
If a dynamic area is to contain character attributes, a
shadow variable must be defined. The shadow variable
is associated with the dynamic area by placing the
shadow variable name after the dynamic area name in
the panel definition.
Programmer response
Define a shadow variable, placing it after the dynamic
area name. The two names must be separated by a
comma only, and the shadow variable name must be
followed by a blank.
ISPP129 Panel 'aaaaaaaa' error - String
of CLEAR name values exceeds
maximum size of 255 characters.
Explanation
The )MODEL header statement keyword, CLEAR,
identifies the dialog variable names that are to be
cleared to blank before each row of the table is read.
This string of variable names, exceeds the maximum
size of 255 characters.
Programmer response
Ensure that the string of variable names does not
exceed the maximum size of 255 characters.
ISPP130 Panel 'aaaaaaaa' error - A panel
section is out of order or has
already been defined.
ISPF messages starting with ISP
170  z/OS: z/OS ISPF Messages and Codes

## Page 191

Explanation
The panel definition sections must be used in this
order:
1. )CCSID
2. )PANEL
3. )ATTR
4. )ABC
5. )ABCINIT
6. )ABCPROC
7. )BODY
8. )MODEL
9. )AREA
10. )INIT
11. )REINIT
12. )PROC
13. )HELP
14. )END
Duplicate panel sections are not allowed.
Programmer response
Verify the panel definitions, checking for out-of-order
panel sections or duplicate names.
ISPP131 Panel 'aaaaaaaa' error -
Required )BODY records not found
following heading.
Explanation
A panel must have at least one line in the )BODY
section.
Programmer response
Check the )BODY section of your panel and verify that
the )BODY section has at least one line (record).
ISPP132 Panel 'aaaaaaaa' error - Invalid
keyword found in )INIT, )REINIT,
or )PROC section.
Explanation
An invalid keyword was found in the )INIT, )REINIT,
or )PROC sections of the panel.
Programmer response
Check that valid keywords are being used
appropriately in the )INIT, )REINIT, and )PROC
sections of the panel.
ISPP133 Panel 'aaaaaaaa' error -
Invalid statement found
within )INIT, )REINIT, or )PROC
section.
Explanation
The )INIT, )REINIT, or )PROC section contains an
invalid statement. The valid statements for these
panel sections are:
• Assignment
• EXIT
• GOTO
• IF/ELSE
• REFRESH
• TOG
• VER
• VEDIT
• VGET
• VPUT
• PANEXIT
• *REXX/*ENDREXX
Programmer response
Verify that the panel statements in the )INIT, )REINIT,
or )PROC section are valid statements as shown in the
list shown.
ISPP134 Panel 'aaaaaaaa' error - Invalid
statement or keyword found on
section heading.
Explanation
One of section headings on the panel contained an
invalid statement or keyword.
Programmer response
Verify panel section heading statements for valid
keywords.
ISPP135 Panel 'aaaaaaaa' error - Invalid
statement type. Name may need
to be enclosed in apostrophes.
Explanation
This message is self explanatory.
ISPP136 Panel 'aaaaaaaa' error -
Something other than an "="
operator found in an assignment
statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  171

## Page 192

Explanation
An assignment statement in the )INIT, )REINIT,
or )PROC section contains something other than an "="
operator.
Programmer response
Correct the assignment statement's operator.
ISPP137 Panel 'aaaaaaaa' error - Table
display panel, duplicate )MODEL
statement found.
Explanation
Only one )MODEL section is allowed per panel.
Programmer response
Remove the duplicate )MODEL section from the panel
definition.
ISPP138 Panel 'aaaaaaaa' error - Invalid
"clear" keyword value (not a literal
or dialog variable).
Explanation
The header CLEAR keyword value in the )MODEL
section must be literal or a dialog variable.
Programmer response
Verify that the section header CLEAR keyword values
in the )MODEL section are literals or dialog variables.
ISPP139 Panel 'aaaaaaaa' error -
Something other than CLEAR or
ROWS keyword found on )MODEL
header statement.
Explanation
The )MODEL section header contains an invalid
keyword.
Programmer response
Use only the CLEAR or ROWS keywords on the )MODEL
header statement.
ISPP140 Panel 'aaaaaaaa' error - Built-
in function in the assignment
statement is not recognized.
Explanation
A built-in function defined in an assignment statement
is in error. Only the 4 built-in functions listed here are
valid:
• TRUNC (truncate)
• TRANS (translate)
• PFK (PF key)
• LVLINE (last visible line).
Programmer response
Change the built-in function in error to one of the 4
valid built-in functions.
ISPP141 Panel 'aaaaaaaa' error - Invalid
assignment statement.
Explanation
The )INIT, )REINIT, or )PROC section contains an
invalid assignment statement.
Programmer response
Verify that all assignment statements follow the
assignment statement format: variable = value
ISPP142 Panel 'aaaaaaaa' error - Invalid
statement type.
Explanation
This message is self explanatory.
ISPP143 Panel 'aaaaaaaa' error - Invalid
statement type.
Explanation
This message is self explanatory.
ISPP144 Panel 'aaaaaaaa' error - The max
of bbbbbbbb levels of nested built-
in functions has been exceeded.
Explanation
The TRUNC and TRANS built-in functions exceed the
maximum level (2) of nesting.
Programmer response
The TRUNC and TRANS built-in functions allow only
2 levels of nesting. Limit the TRANS and TRUNC
statements to 2 levels of nesting.
ISPF messages starting with ISP
172  z/OS: z/OS ISPF Messages and Codes

## Page 193

ISPP145 Panel 'aaaaaaaa' error - Invalid
nested keyword used within a
built-in function.
Explanation
An invalid keyword was used within a nested built-
in function. For example, in this statement the PFK
built-in function keyword is invalid within the built-in
function, TRANS.
&ESER =
TRUNC(PFK(1),H)
Programmer response
Remove the invalid nested keyword used within a
built-in function.
ISPP146 Panel 'aaaaaaaa' error -
Invalid built-in function variable
specified.
Explanation
The variable used within the built-in function is invalid.
Programmer response
Check for invalid variables within the TRUNC and
TRANS built-in functions.
ISPP147 RESTORE request error - A
CONTROL RESTORE service
request was issued and either
there was no matching CONTROL
SAVE request or the matching
CONTROL SAVE request was
issued before the first panel was
displayed.
Explanation
For each CONTROL service Restore request, there
should be a matching CONTROL service Save request
that is issued after the first panel is displayed.
Programmer response
Verify that the CONTROL service Restore requests
have matching Save requests that are issued after the
first panel is displayed.
ISPP148 Panel 'aaaaaaaa' error - At least
one of the CLEAR names listed is
not a panel field name.
Explanation
The keyword, CLEAR(var-name, varname …) within
the )MODEL section contains a variable name that is
not a panel field name.
Programmer response
Correct the )MODEL section's keyword, CLEAR, to
contain variable names that are panel fields.
ISPP149 Panel 'aaaaaaaa' error - The
REFRESH statement is invalid
within the )INIT section.
Explanation
The panel section statement, REFRESH, is not valid
in the )INIT section. The REFRESH statement can
appear within the )PROC or )REINIT section of a panel
definition.
Programmer response
Remove the REFRESH statement from the )INIT
section. Place the REFRESH statement in the
appropriate section, )PROC or )REINIT.
ISPP150 Panel 'aaaaaaaa' error - No )END
found before reaching end of file.
Explanation
No )END statement was found on the last line of the
panel definition. An )END statement is required as the
last line of each panel definition. ISPF ignores any data
that appears on lines following the )END statement.
Programmer response
Update the panel definition to contain an )END
statement on the last line.
ISPP151 Panel 'aaaaaaaa' error - I/O error
in retrieving panel (CDG).
Explanation
An attempt to retrieve the panel to read a logical
record of the panel has failed.
System programmer response
Contact IBM support.
User response
Try to display the panel again. If this fails, contact your
system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  173

## Page 194

ISPP152 Panel 'aaaaaaaa' error - Invalid
CDG return code, system error.
Explanation
An error occurred while trying to read a panel record.
System programmer response
Contact IBM support.
User response
Try to display the panel again. If this fails, contact your
system programmer.
ISPP153 Panel 'aaaaaaaa' error - Invalid
quoted string, end quote or
continuation ("+") missing.
Explanation
The ending quotation mark is missing from a quoted
string, or a continuation character, "+" is missing.
Programmer response
Add the missing ending quotation mark or continuation
character (+) to the statement in error.
ISPP154 Panel 'aaaaaaaa' error - Illegal
use of "¬=" operator.
Explanation
This message is self explanatory.
ISPP155 Panel 'aaaaaaaa' error - A dialog
variable name must have a
minimum length of bbbbbbbb.
and must not exceed a maximum
length of cccccccc. characters.
Explanation
A dialog variable name must follow minimum or
maximum length as specified in this message.
Programmer response
Change the dialog variable name to meet the minimum
or maximum length.
ISPP156 Panel 'aaaaaaaa' error -
Unrecognized keyword.
Explanation
An invalid panel keyword has been detected.
Programmer response
Check the panel definition for invalid or misspelled
keywords and correct the keyword.
ISPP157 Panel 'aaaaaaaa' error -
Unrecognized control variable.
Explanation
An invalid panel control variable was found. The ISPF
panel control variables are:
.ALARM
.ATTR
.ATTRCHAR
.AUTOSEL
.CSRPOS
.CSRROW
.CURSOR
.HELP
.HHELP
.MSG
.NRET
.PFKEY
.RESP
.TRAIL
.ZVARS
Programmer response
Ensure all the panel control variables used are valid.
ISPP158 Panel 'aaaaaaaa' error - Invalid
field/area name assigned to
the .CURSOR control variable.
Explanation
The .CURSOR control variable is set to an invalid panel
field name or to a scrollable area name.
Programmer response
Set the .CURSOR control variable to valid field/area
name. Its value must be a character string that
matches a field name or a DYNAMIC or GRAPHIC
area name in the panel body. Its value cannot be a
character that matches a scrollable AREA name, but
it can be a character string that matches a field name
within the scrollable area.
ISPP159 Panel 'aaaaaaaa' error - Invalid
response value assigned to
the .RESP control variable.
ISPF messages starting with ISP
174  z/OS: z/OS ISPF Messages and Codes

## Page 195

Explanation
An invalid response value was assigned to the .RESP
panel control variable. Valid values are END and
ENTER.
Programmer response
Set the .RESP control variable to the appropriate valid
value, END or ENTER.
ISPP160 Panel 'aaaaaaaa' error - Invalid
variable name, (may need to be
enclosed in apostrophes)
Explanation
This message is self explanatory.
ISPP161 Panel 'aaaaaaaa' error - Invalid
variable name, (may need to be
enclosed in apostrophes).
Explanation
This message is self explanatory.
ISPP162 Panel 'aaaaaaaa' error - Invalid
variable name, (may need to be
enclosed in apostrophes)
Explanation
This message is self explanatory.
ISPP163 Panel 'aaaaaaaa' error - Invalid
TRANS value (not a literal or dialog
variable).
Explanation
The TRANS value must be a literal or a dialog variable.
Programmer response
Correct the appropriate TRANS value to a literal or a
dialog variable.
ISPP164 Panel 'aaaaaaaa' error - Invalid
TRANS msg= value (not a literal or
dialog variable).
Explanation
The MSG=value option specified on the TRANS built-
in function statement has an invalid value for the
message ID.
Programmer response
Correct the MSG=value option on the TRANS built-in
function to a valid dialog variable or literal.
ISPP165 Panel 'aaaaaaaa' error - TRUNC
value is not a numeric or single
character.
Explanation
The TRUNC built-in function value must be either a
numeric quantity indicating the length of the truncated
result or any special character indicating truncation at
the first occurrence of that character.
Programmer response
Correct the TRUNC built-in function value to either
the appropriate numeric quantity or to the appropriate
special character needed to achieve the expected
truncation results.
ISPP166 Panel 'aaaaaaaa' error - Invalid
TRUNC value (not a literal or dialog
variable).
Explanation
The value used in the TRUNC built-in function is
invalid. The value must be a literal or a dialog variable.
Programmer response
Correct the value in the TRUNC built-in function to be a
literal or a dialog variable.
ISPP167 Panel 'aaaaaaaa' error - Invalid IF
keyword value (not a literal, dialog
or control variable).
Explanation
The IF statement keyword value is invalid. The IF
statement keyword values must be literals, dialog
variables, or control variables.
Programmer response
Check the IF statements keyword values and verify
that they are literals, dialog variables, or control
variables.
ISPP168 Panel 'aaaaaaaa' error - Invalid IF
keyword operator found.
Explanation
The operator used in the IF statements basic value
test expression is invalid. Valid operators are:
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  175

## Page 196

=
EQ
¬=
NE
<
LT
>
GT
¬>
NG
¬<
NL
>=
GE
<=
LE
Character symbol operators must be specified in
uppercase.
If character symbols are used (for example, EQ or NE),
use a blank to delimit them. Blanks cannot appear
inside a compound operator (for example, >=, ¬<) or
character symbol.
Programmer response
Examine the IF statement for invalid syntax.
ISPP169 Panel 'aaaaaaaa' error - Invalid IF
logical expression (not a dialog or
control variable or VER statement
construct).
Explanation
The IF statement syntax is invalid. The IF statement
may contain dialog variables, control variables, or the
VER statement construct.
Programmer response
Examine the IF statement for invalid syntax.
ISPP170 Panel 'aaaaaaaa' error - Invalid
ATTR keyword value (dialog var)
found while initializing panel.
Explanation
The dialog variable used for an )ATTR section keyword
value is invalid. The dialog variable is substituted after
processing the )INIT section.
Programmer response
Check the dialog variables used in the )ATTR section
for keyword values. Ensure that the dialog variables
are set to valid values.
ISPP171 Panel 'aaaaaaaa' error - Invalid
value for TYPE keyword found
while initializing panel.
Explanation
A dialog variable was used for a TYPE keyword
value and that TYPE keyword value must be coded
explicitly. These TYPE values must be coded explicitly.
It is invalid to assign any of these values to dialog
variables. The TYPE values are:
AB
NT
Programmer response
Verify that the TYPE values in the list shown have been
coded explicitly and not as dialog variables.
ISPP172 Panel 'aaaaaaaa' error - Invalid
TRUNC value (dialog var) found
while processing panel.
Explanation
The TRUNC built-in function value (a dialog variable) is
neither a numeric quantity indicating the length of the
truncated result, nor any special character indicating
truncation at the first occurrence of that character.
Programmer response
Check that the dialog variable for the TRUNC value is
a numeric quantity or a special character, whichever is
appropriate.
ISPP173 Panel 'aaaaaaaa' error - More than
bbbbbbbb entries specified for a
tutorial TRANS function.
Explanation
The maximum number of entries allowed is 100.
Programmer response
Limit the number of entries specified for a tutorial
TRANS function to 100.
ISPP174 Panel 'aaaaaaaa' error - Invalid
value for a verify PICT string found
while processing panel.
ISPF messages starting with ISP
176  z/OS: z/OS ISPF Messages and Codes

## Page 197

Explanation
A PICT string in a VER statement in the )INIT, )REINIT,
or )PROC section of a panel definition is incorrect.
The valid syntax is VER (xxx,PICT,string), where xxx is
a variable. The string parameter can be composed of
any of these:
C
Any character
A
Any alphabetic character (A-Z, #, @, $)
N
Any numeric character (0-9)
9
Any numeric character (same as "N")
X
Any hexadecimal character (0-9, A-F)
In addition, the string can contain any special
characters (except #, @, $) or lowercase alphabetic
characters.
For example: VER(xxx, PICT,'A/NNN') The value of
the variable must start with an alphabetic character
followed by a slash, followed by 3 numeric characters.
Programmer response
Correct the invalid value for the verify PICT string.
ISPP175 Panel 'aaaaaaaa' error - Invalid
value for a verify RANGE limit
found while processing panel.
Explanation
A RANGE limit in a VER statement in
the )INIT, )REINIT, or )PROC section of a panel
definition is not valid.
The valid syntax is:
VER (xxx,RANGE,lowerlimit,upperlimit)
where:
xxx is a variable
"Lowerlimit" and "upperlimit" must be numeric
characters (0-9) of no more than 16 digits each.
Negative values can be specified.
Programmer response
Correct the invalid value for a verify RANGE limit.
ISPP176 Panel 'aaaaaaaa' error -
Invalid .CURSOR value found
while processing panel.
Explanation
The control variable .CURSOR value must be a
character string that matches a field name, a DYNAMIC
or GRAPHIC area name, or a field name within a
scrollable AREA. It cannot be a character string that
matches a scrollable AREA name.
Programmer response
Verify that .CURSOR is set to a valid value.
ISPP177 Panel 'aaaaaaaa' error -
Invalid .RESP value found while
processing panel.
Explanation
An invalid response (.RESP) value was found when
processing the panel. Valid values are ENTER or END.
Programmer response
Set the value of the .RESP control variable to ENTER or
END.
ISPP178 Panel 'aaaaaaaa' error -
Conversion or truncation problem
found while attempting to store
variable 'bbbbbbbb'.
Explanation
A conversion or truncation problem was found when
attempting to store the variable. The format or length
of the variable value may be in error.
Programmer response
Verify the length and format of the variable.
ISPP179 Panel 'aaaaaaaa' error - Shadow
variable name exceeds maximum
length of bbbbbbbb.
Explanation
The shadow variable name exceeds the maximum
length specified.
Programmer response
Shorten the shadow variable name to the maximum
length allowed.
ISPP180 Panel 'aaaaaaaa' error -
Verification TYPE unrecognized.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  177

## Page 198

Explanation
The VER (verify) statement keyword is invalid. Valid
keywords are:
• ALPHA
• ALPHAB
• NUM
• INCLUDE
• ENUM
• HEX
• BIT
• LEN
• FILEID
• PICT
• NAME
• DSNAME
• RANGE
• LIST
• LISTV
• DBCS
• EBCDIC
• MIX
Programmer response
Verify that the keyword in the VER statement is one of
the valid keywords.
ISPP181 Panel 'aaaaaaaa' error - Invalid
VERIFY value (not a literal or
dialog variable).
Explanation
The keyword value in the VER (verify) statement is
invalid. This value must be a literal or dialog variable.
Programmer response
Correct the invalid keyword value in the VER
statement.
ISPP182 Panel 'aaaaaaaa' error - Invalid
VERIFY msg= value (not a literal or
dialog variable).
Explanation
The value in the VER (verify) statement msg=value
parameter must be a literal or dialog variable.
Programmer response
Correct the value to be a literal or a dialog variable.
ISPP183 Panel 'aaaaaaaa' error - Incorrect
# of VERIFY values for verification
type indicated.
Explanation
An incorrect number of verify values is specified for
the verification type that is indicated.
Programmer response
Check the syntax for the VER statement that uses your
keyword type for verification.
ISPP184 Panel 'aaaaaaaa' error - Invalid
VERIFY picture string given.
Explanation
A PICT string in a VER statement in the )INIT, )REINIT,
or )PROC section of a panel definition is incorrect.
The valid syntax is VER (xxx,PICT,string), where xxx
is a variable and the "string" parameter can be
composed of any of these:
C
Any character
A
Any alphabetic character (A-Z, #, @, $)
N
Any numeric character (0-9)
9
Any numeric character (same as "N")
X
Any hexadecimal character (0-9, A-F)
In addition, the string can contain any special
characters (except #, @, or $) or lowercase alphabetic
characters.
Example:
VER(xxx, PICT,'A/NNN')
The value of the variable must start with an alphabetic
character followed by a slash, followed by 3 numeric
characters.
Programmer response
Correct the VER statement picture string.
ISPP185 Panel 'aaaaaaaa' error - Invalid
VERIFY range value given.
ISPF messages starting with ISP
178  z/OS: z/OS ISPF Messages and Codes

## Page 199

Explanation
A RANGE limit in a VER statement in
the )INIT, )REINIT, or )PROC section of a panel
definition is not valid.
The valid syntax is:
VER (xxx,RANGE,lowerlimit,upperlimit)
where:
xxx is a variable
"lowerlimit" and "upperlimit" are numeric
characters (0-9) of no more than 16 digits each.
Negative values can be specified.
Programmer response
Correct the RANGE limit, following the guidelines.
ISPP186 Panel 'aaaaaaaa' error - Invalid
VERIFY type given (second
parameter must be a literal).
Explanation
The VER statement second parameter, NONBLANK,
is a literal. The literal, NB, may be used as an
abbreviation of NONBLANK.
Programmer response
Update the second parameter of the VER statement to
either NONBLANK or NB.
ISPP187 Panel 'aaaaaaaa' error - Invalid
VERIFY variable (must be a dialog
variable).
Explanation
The first parameter of the VER statement inside the
parentheses specifies the name of the variable to
be checked. This first parameter must be a dialog
variable.
Programmer response
Check the first parameter specified and verify that this
parameter is a dialog variable.
ISPP188 Panel 'aaaaaaaa' error - Maximum
of 100 list values exceeded on VER
statement.
Explanation
In the VER statement construct
"LIST,value1,value2,...", the list of values cannot
exceed 100.
Programmer response
Limit your list of values in the VER statement to fewer
than or equal to 100.
ISPP189 Panel 'aaaaaaaa' error - Control
variable compare value too long,
maximum length is 8 bytes.
Explanation
An error was found in the IF statement processing
comparing a control variable to a value that exceeds
the maximum length of 8 bytes.
Programmer response
Verify that the IF statement processing is comparing
the control variable to a valid value.
ISPP190 Enter required field - Enter
required field at the cursor
position.
Explanation
A VER statement containing the NONBLANK keyword
failed. The field is blank.
User response
Enter the appropriate data into the field at the cursor
position.
ISPP190A Missing field definition - The field
definition 'aaaaaaaa' is missing
in the panel )BODY section. When
the field definition is missing, the
cursor position is unpredictable.
Explanation
A VER statement containing the NONBLANK keyword
failed. The field is not defined in the panel )BODY
section.
Programmer response
If the field is not defined in the panel )BODY section,
define the field in the panel )BODY section.
ISPP191 Must be alphabetic - Enter
alphabetic characters (A-Z, a-z, #,
$, or @).
Explanation
This is an ISPF-supplied message displayed as the
result of an alphabetic verification failure.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  179

## Page 200

User response
Enter the appropriate data into the field pointed to by
the cursor.
ISPP192 Must be hexadecimal - Enter
hexadecimal characters (0-9, A-F,
a-f).
Explanation
This is an ISPF-supplied message displayed as the
result of a hexadecimal verification failure.
User response
Enter the appropriate data into the field pointed to by
the cursor.
ISPP193 Must be numeric - Enter numeric
characters (0-9).
Explanation
This is an ISPF-supplied message displayed as the
result of a numeric verification failure.
User response
Enter the appropriate data into the field pointed to by
the cursor.
ISPP194 Value out of range - Enter a
numeric value between aaaaaaaa.
and bbbbbbbb.
Explanation
This is an ISPF-supplied message displayed as the
result of a RANGE,lower limit,upper limit verification
failure.
User response
Enter the appropriate data into the field pointed to by
the cursor.
ISPP195 Invalid value - Enter one of the
listed values ('aaaaaaaa').
Explanation
This is an ISPF-supplied message displayed as the
result of a LIST,value1,value2,... verification failure.
User response
Enter one of the listed values into the field pointed to
by the cursor.
Programmer response
If the listchoices occupy more than 72 bytes, provide
help via a customized message using the MSG keyword
on the VER statement.
ISPP195A Invalid value - Enter one of the
listed values ('aaaaaaaa') .
Explanation
Used instead of ISPP195 when the listed values are
not appropriate.
User response
Enter one of the listed values into the field pointed to
by the cursor.
ISPP196 Must be a bit string - Enter all '0's
or '1's.
Explanation
This is an ISPF-supplied message displayed as the
result of a BIT verification failure.
User response
Enter a binary value into the field pointed to by the
cursor.
ISPP197 Invalid fileid - Variable must
contain a fileid (CMS syntax), valid
with a LISTFILE command.
Explanation
This is an ISPF-supplied message displayed as the
result of a FILEID verification failure. The file name
and type, if given, must be 1 to 8 alphanumeric
characters, including A-Z, 0-9, $, #, @, +, - (hyphen), :
(colon), and _ (underscore). The filemode must be a
single letter (A-Z), optionally followed by a single digit
(0-9). In addition, one or more fields of the file ID can
be an asterisk (*) or a string of characters followed by
an asterisk.
User response
Enter a valid file ID in CMS syntax.
ISPP198 Must be numeric - Enter a numeric
value between 'aaaaaaaa' and
'bbbbbbbb'.
Explanation
This is an ISPF-supplied message displayed as the
result of a RANGE verification failure.
ISPF messages starting with ISP
180  z/OS: z/OS ISPF Messages and Codes

## Page 201

User response
Enter a valid numeric value within the range specified.
ISPP199 Invalid value - Field has failed
verification. Reenter or press the
End key.
Explanation
This is an ISPF-supplied message displayed as the
result of a verification failure.
User response
Reenter data or press the END key.
ISPP200 Invalid name - Enter up to 8
alphanumeric chars (first must be
alphabetic).
Explanation
The variable must contain a valid name, following
the rules of member names, using up to eight
alphanumeric characters (A-Z, #, $, @, 0-9). The first
character must be alphabetic (A-Z, #, $, @).
User response
Reenter the name, following the rules for member
names.
Programmer response
None
ISPP200A Invalid member name - The
member name entered as part of
the data set name is invalid. Enter
up to 8 alphanumeric chars (first
must be alphabetic) or valid GDG
suffix.
Explanation
The variable must contain a valid name, following
the rules of member names, using up to eight
alphanumeric characters (A-Z, #, $, @, 0-9). The first
character must be alphabetic (A-Z, #, $, @). Examples
of valid Generation Data Group suffixes are:
xxxxxx.yyyy.zzzz(-1)
or
xxxxxx.yyyy.zzzz(+2)
User response
Reenter the member name, following the rules for
member names.
Programmer response
None
ISPP201 Invalid name - Enter a name of 1
to 8 alphanumeric characters.
Explanation
A verification failed on the VER statement with the
NAME keyword. The variable being verified does not
follow the rule of member names, using up to 8
alphanumeric characters (A-Z, #, $, @, 0-9). The first
character must be alphabetic (A-Z, #, $, @).
User response
Reenter a valid member name, following the rule
of member names, using up to 8 alphanumeric
characters (A-Z, #, $, @, 0-9). The first character must
be alphabetic (A-Z, #, $, @).
Programmer response
Check the variable in the VER statement with the
keyword NAME. The variable must contain a valid
member name following the rule of member names,
using up to 8 alphanumeric characters (A-Z, #, $, @,
0-9). The first character must be alphabetic (A-Z, #, $,
@).
ISPP202 Invalid sign - Press the Help key
for additional information.
Explanation
The correct sign notation was not used. Sign notation
for a number is not required, but if you do use it,
remember these rules:
• Indicate a negative number by a leading minus
sign, -695; a trailing minus sign, 695-; or a number
enclosed in parentheses, (695).
• If the number is negative, you cannot use blanks
between the sign indicator and the number unless
the numerical delimiter is a blank.
• Indicate a positive number by a leading plus sign,
+695; or a number with no sign, 695.
User response
Contact the responsible programmer.
Programmer response
Correct the sign notation. Refer to the discussion in
ISPF Dialog Developer's Guide and Reference under
the VER statement's keyword, ENUM, for correct sign
notation.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  181

## Page 202

ISPP203 Invalid delimiter - Press the Help
key for additional information.
Explanation
The delimiter notation entered within a numerical
field is incorrect. You do not have to use delimiters,
but once you do, you must continue using them
throughout the field.
User response
Contact the responsible programmer.
Programmer response
Correct the delimiter notation within the numerical
field. Follow these rules when using delimiters:
• Proceeding from the left of the field, specify three
digits between delimiters.
7999,345,789.
is a valid number since there are three
digits between delimiters after finding the first
delimiter.
98,88,765
is an incorrectly-specified number since there
are only two digits between delimiters.
• If you use a decimal, you cannot use any other
delimiter to the right of the decimal.
345,789.0005
is correct.
45,879.531,20
is incorrect.
ISPP204 Invalid decimal - Press the Help
key for additional information.
Explanation
The decimal notation that was entered is incorrect.
Enter only one decimal indicator in a field. If numerical
delimiters were also used, there must be three digits
between the delimiters and the decimal.
User response
Contact the responsible programmer.
Programmer response
Correct the decimal notation. Follow these rules when
entering decimal notation:
999,765.800
is correct.
999,7658.00
is incorrect (4 digits between the numerical
indicator and the decimal).
ISPP205 Panel 'aaaaaaaa' error - Invalid
VERIFY length operator.
Explanation
The length of the variable (number of characters)
must satisfy the condition expressed by the relational
operator and expected length. You have used an
invalid relational operator.
User response
Contact the responsible programmer.
Programmer response
Correct the invalid relational operator. Valid relational
operators are:
=
or EQ
¬=
or NE
<
or LT
>
or GT
¬>
or NG
¬<
or NL
>=
or GE
<=
or LE
These character symbols must be expressed in
uppercase.
ISPP206 Panel 'aaaaaaaa' error - VERIFY
length must be numeric.
Explanation
The expected-length operand on the VER statement
is invalid. The expected-length operand should be
a positive number having a maximum of 5 numeric
characters. If the expected-length operand exceeds
the maximum of 5 numeric characters, ISPF truncates
the operand to 5. The expected-length operand can be
expressed as a literal or as a dialog variable containing
the value.
ISPF messages starting with ISP
182  z/OS: z/OS ISPF Messages and Codes

## Page 203

User response
Contact the responsible programmer.
Programmer response
Correct the expected-length operand to be a positive
number that has a maximum of 5 numeric characters.
ISPP207 Panel 'aaaaaaaa' error - Invalid
VERIFY length operator found
while processing panel.
Explanation
The value of the dialog variable used in the VER
statement as the relational operator is invalid.
Programmer response
Correct the value of the dialog variable used in the VER
statement as the relational operator to one of these
valid relational operators:
=
or EQ
¬=
or NE
<
or LT
>
or GT
¬>
or NG
¬<
or NL
>=
or GE
<=
or LE
These character symbols must be expressed in
uppercase.
ISPP208 Panel 'aaaaaaaa' error - Invalid
VERIFY numeric length found
while processing panel.
Explanation
The expected-length operand on the VER statement
is invalid. The expected-length operand should be
a positive number having a maximum of 5 numeric
characters. If the expected-length operand exceeds
the maximum of 5 numeric characters, ISPF truncates
the operand to 5. The expected-length operand can be
expressed as a literal or as a dialog variable containing
the value.
Programmer response
Correct the value of the dialog variable used in the VER
statement as the expected-length operand. This dialog
variable's value must be a positive number having a
maximum of 5 numeric characters.
ISPP209 Invalid length - The length of
the data must be aaaaaaaa.
bbbbbbbb..
Explanation
The length of the variable (number of characters) does
not satisfy the condition expressed by the relational
operator and expected length in the VER statement
using the LEN keyword.
User response
Contact the responsible programmer.
Programmer response
Correct the length of the variable to satisfy the
condition expressed by the relational operator and
expected length in the VER statement, or adjust the
expected length in the VER statement.
ISPP210 Invalid DSN - quotes - Data
set name contains unbalanced
apostrophes; reenter dsname.
Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The quotation
marks enclosing the data set name are unbalanced.
User response
Check the quotation marks enclosing the data set
name. The quotation marks should be balanced. For
example,
'xxxxx.yyy.zzz'
has balanced quotation marks and
'xxxxx.yyy.zzz
has unbalanced quotation marks. Reenter the data set
name, enclosing it in balanced quotation marks.
ISPP211 Invalid DSN - member - Member
name of data set name must be
1-8 chars and enclosed in ( ).
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  183

## Page 204

Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The member
name of the data set must be 1-8 characters and
enclosed in parentheses.
User response
Check that the member name of the data set contains
1-8 characters and is enclosed in parentheses and
reenter.
Programmer response
Verify that the member name assigned to the variable
in the VER statement complies with the data set
member name length of 1-8 characters and is
enclosed in parentheses.
ISPP212 Invalid DSN - length - Dsname
must be 1-44 chars in length,
including prefix. Reenter dsname.
Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The data set
name must be 1-44 characters in length, including
prefix.
User response
Reenter the data set name. The data set name must be
1-44 characters in length, including prefix.
Programmer response
Verify that the data set name assigned to the variable
in the VER statement complies with the data set name
length of 1-44 characters.
ISPP213 Invalid DSN - syntax - Dsname
must not end with a period.
Reenter dsname.
Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The data set
name must not end with a period.
User response
Reenter the data set name. Do not end the data set
name with a period.
Programmer response
Verify that the data set name assigned to the variable
in the VER statement does not end with a period.
ISPP214 Invalid DSN - syntax - Dsname
contains embedded blanks,
parentheses or apostrophes.
Reenter.
Explanation
The VER statement with the DSNAME keyword
has detected an invalid TSO data set name. The
data set name must not contain embedded blanks,
parentheses, or apostrophes.
User response
Reenter the data set name. The data set name
must not contain embedded blanks, parentheses, or
apostrophes.
Programmer response
Verify that the data set name assigned to the variable
in the VER statement does not contain embedded
blanks, parentheses, or apostrophes.
ISPP215 Invalid DSN - qualifier - A data set
name qualifier must contain from
1-8 characters. Reenter.
Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The data set
name qualifiers must contain from 1-8 characters.
User response
Reenter the data set name. The data set name
qualifiers must contain from 1-8 characters.
Programmer response
Verify that the data set name assigned to the variable
in the VER statement does not contain qualifiers
greater than 8 characters or less than one character.
ISPP216 Invalid DSN - qualifier -
Each qualifier must be 1-8
alphanumeric characters, the first
alphabetic.
Explanation
The VER statement with the DSNAME keyword has
detected an invalid TSO data set name. The data set
ISPF messages starting with ISP
184  z/OS: z/OS ISPF Messages and Codes

## Page 205

name qualifiers must contain from 1-8 characters and
the first character of each qualifier must be alphabetic.
User response
Reenter the data set name. The data set name
qualifiers must contain from 1-8 characters and the
first character of each qualifier must be alphabetic.
Programmer response
Verify that the data set name assigned to the variable
in the VER statement does not contain qualifiers
greater than eight characters or less than one
character. Also, verify that the first character of each
qualifier is alphabetic.
ISPP217 Invalid number - A valid number
contains at least one digit (0-9).
Explanation
A VER statement expected a valid number. A valid
number contains at least one digit (0-9).
User response
Enter a valid number.
ISPP218 Invalid name - Enter up to
8 alphanumeric characters (first
cannot be numeric).
Explanation
A VER statement expected a valid name of 1 to 8
alphanumeric characters and the first character must
be alphabetic (not numeric).
User response
Use a valid variable naming convention.
ISPP219 DSN too long - The field
"aaaaaaaa" does not have enough
space defined for ISPF to add
a closing quote and/or closing
parenthesis to the data set name.
Explanation
The VER statement with the DSNAMEQ or DSNAMEPQ
keyword resulted in the variable value being longer
than the length specified on the VDEFINE for the
variable in question.
User response
Contact the responsible programmer.
Programmer response
Ensure that the correct length was specified on
the VDEFINE service. Keep in mind when using the
VDEFINE service that the panel VERIFY statement
keywords DSNAMEQ and DSNAMEPQ add the closing
quotation mark if the starting quotation mark exists,
and DSNAMEPQ adds the closing member name
parenthesis if the starting member name parenthesis
exists.
ISPP220 Invalid string - Character number
'aaaaaaaa' must be alphabetic (A-
Z, a-z, #, $, or @).
Explanation
The VER statement with the PICT or PICTCN keyword,
has found characters within the variable that do
not match the corresponding type of character in
the picture string. In this case, the picture string
characters are alphabetic.
User response
Reenter the character string data using alphabetic
characters (A-Z, a-z, #, $, @).
Programmer response
Verify that the data assigned to the variable in the VER
statement with the PICT or PICTCN keyword contains
alphabetic characters (A-Z, a-z, $, #, @).
ISPP221 Invalid string - Character number
'aaaaaaaa' must be numeric (0-9).
Explanation
The VER statement with the PICT or PICTCN keyword
has found characters within the variable that do
not match the corresponding type of character in
the picture string. In this case, the picture string
characters are numeric.
User response
Reenter the character string data using numeric
characters (0-9).
Programmer response
Verify that the data assigned to the variable in the VER
statement with the PICT or PICTCN keyword contains
numeric characters (0-9).
ISPP222 Invalid string - Character number
'aaaaaaaa' must be hexadecimal
(0-9, A-F, a-f).
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  185

## Page 206

Explanation
The VER statement with the PICT or PICTCN keyword
has found characters within the variable that do
not match the corresponding type of character in
the picture string. In this case, the picture string
characters are hexadecimal.
User response
Reenter the character string data using hexadecimal
characters (0-9, A-F, a-f).
Programmer response
Verify that the data assigned to the variable in the VER
statement with the PICT or PICTCN keyword contains
hexadecimal characters (0-9, A-F, a-f).
ISPP223 Invalid string - Character number
'aaaaaaaa' must be 'bbbbbbbb'.
Explanation
The VER statement with the PICT or PICTCN keyword
has found a character within the variable that does
not match the corresponding type of character in the
picture string. For example, this VER statement,
VER(&variable,PICT,'A/NNN'),
requires that the variable's value start with an
alphabetic character followed by a slash, followed by
3 numeric characters. If the variable value was B1123,
you would receive the error message:
"CHARACTER NUMBER '2' MUST BE '/'".
User response
Reenter the character string data using characters that
match the corresponding type of characters in the
picture string.
Programmer response
Verify that the data assigned to the variable in the
VER statement with the PICT or PICTCN keyword
contains characters that match the corresponding type
of characters in the picture string.
ISPP224 Invalid string - aaaaaaaa
characters are required in the
format, 'bbbbbbbb'.
Explanation
The VER statement with the PICT or PICTCN keyword
has found a character within the variable that does
not match the corresponding format of characters in
the picture string. For example, this VER statement,
VER(&variable,PICT,'A/NN'), requires that the variable
start with an alphabetic character, followed by a slash,
followed by 2 numeric characters. If the variable was
"B/123", you would receive this error message:
"4" CHARACTERS ARE REQUIRED IN THE FORMAT,
A/NN.
User response
Reenter the character string data using characters
that match the format and the corresponding type of
characters in the picture string.
Programmer response
Verify that the data assigned to the variable in the
VER statement with the PICT or PICTCN keyword
contains characters that match the corresponding type
of characters in the picture string and the picture
string format.
ISPP225 Panel 'aaaaaaaa' error - Specify
1 compare value for IF when
operator is 'bbbbbbbb'.
Explanation
The operator you specified does not support
comparison against multiple values. You can specify
comparison against up to 255 values for the EQ (=) and
NE (¬=) operators. For the remaining operators, you
can specify comparison against only one value.
User response
Contact the responsible programmer.
Programmer response
Specify only one compare value for the IF statement
when using the operator specified in the error
message.
ISPP226 Panel 'aaaaaaaa' error - An ELSE
in column 'bbbbbbbb' did not
match a previous IF statement.
Explanation
The ELSE statement is not column-aligned with the
matching IF statement.
User response
Contact the responsible programmer.
ISPF messages starting with ISP
186  z/OS: z/OS ISPF Messages and Codes

## Page 207

Programmer response
Column-align the ELSE statement with the matching IF
statement.
ISPP227 Panel 'aaaaaaaa' error - Specify
255 or less IF compare values for
operator 'bbbbbbbb'.
Explanation
More than 255 compare values are specified for the IF
statement with the operator EQ (=) or NE (¬=).
User response
Contact the responsible programmer.
Programmer response
Limit the compare values to 255 values for the IF
statement with the operator EQ (=) or NE (¬=).
ISPP228 Panel 'aaaaaaaa' error - MSG=
parameter is illegal in IF
statement VER logical expression.
Explanation
The IF statement conditional-expression supports
the VER statement construct coded without the
MSG=parameter.
User response
Contact the responsible programmer.
Programmer response
Remove the MSG=parameter from the IF statement's
VER statement construct.
ISPP229 Panel 'aaaaaaaa' error - The IF
statement has an illegal format.
More than 255 logical expressions
have been specified.
Explanation
The IF statement supports Boolean operators and
allows you to combine up to 255 logical expressions.
You have exceeded the limit of 255 expressions.
User response
Contact the responsible programmer.
Programmer response
Limit the number of logical expressions specified on
the IF statement to 255.
ISPP230 Panel 'aaaaaaaa' error - Scroll
input field not second input field
or not 4 chars long.
Explanation
This message is self explanatory.
ISPP231 Panel 'aaaaaaaa' error - Command
input field not first input field or
less than 8 chars long.
Explanation
This message is self explanatory.
ISPP232 Invalid length - Command chain
length exceeds the allowable
maximum of 255.
Explanation
The command chain length is larger than the
maximum of 255 bytes.
User response
Limit the command chain length to the maximum of
255 bytes.
ISPP233 Invalid variable name - Variable
'aaaaaaaa' could not be retrieved
by ISPDVCGT.
Explanation
The name of the variable which contains the command
stack could not be retrieved from the ISPF variable
pool.
User response
Contact the responsible programmer.
Programmer response
Ensure that the specified variable is spelled correctly.
Check the command chain in the specified user
variable. Contact IBM support.
ISPP234 Panel 'aaaaaaaa' error - The IF
statement has an illegal format. A
Boolean operator is specified with
no following logical expression.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  187

## Page 208

Explanation
The IF statement has a Boolean operator but no
following logical expression.
User response
Contact the responsible programmer.
Programmer response
Redefine the IF statement following IF statement
syntax:
IF (logical-expression Boolean operator logical-
expression)
ISPP235 Panel 'aaaaaaaa' error - Invalid
LISTV value given for VERIFY
statement (must be a dialog
variable).
Explanation
The LISTV value is not a dialog variable. The keyword
LISTV on a VER statement allows the use of a dialog
variable containing a list of values to be used for the
variable field verification.
User response
Contact the responsible programmer.
Programmer response
Place the list of values to be used for variable field
verification into a dialog variable. This example verifies
an area code field. In the )INIT section, the LISTV
value &varlist is set to these area codes:
&varlist = '919 805 312'.
In the )PROC section, this VER statement is coded:
VER (&areacode,NONBLANK,LISTV,&varlist,MSG=NSL01
1)
ISPP236 Panel 'aaaaaaaa' error - Invalid
syntax on IF statement. Right
parenthesis or Boolean operator is
missing.
Explanation
The IF statement syntax is invalid. A closing right
parenthesis or Boolean operator is missing.
User response
Contact the responsible programmer.
Programmer response
Check the IF statement syntax for the missing right
parenthesis or Boolean operator.
ISPP237 Panel 'aaaaaaaa' error - No
*ENDREXX statement found to
terminate inline Rexx.
Explanation
The *REXX statement is used in a panel procedure
to run inline Rexx but the corresponding *ENDREXX
statement to terminate the inline Rexx is missing.
User response
Contact the responsible programmer.
Programmer response
When using the *REXX statement to code inline Rexx
in a panel procedure ensure there is a corresponding
*ENDREXX statement to terminate the inline Rexx.
ISPP238 Panel 'aaaaaaaa' error - Default
keyword must contain only 3
characters or a right parenthesis is
missing.
Explanation
There may be more than 3 characters defined or a
closing parenthesis (right parenthesis) is missing from
the DEFAULT keyword of the ATTR section.
Programmer response
Check for too many values or a a missing right
parenthesis on the DEFAULT keyword values in the
attribute section.
ISPP240 Panel 'aaaaaaaa' error - .ZVARS
value, with "Z" replacement
names, not found when expected.
Explanation
In the body and area sections of a panel definition and
in the model lines for a table display panel, the name
of an input or output field can be represented by the
single character Z. This serves as a placeholder; the
actual name of the field is defined in the initialization
section of the panel definition and assigned to the
control variable, .ZVARS. This error message indicates
that the actual name of a field has not been defined
and assigned to the control variable .ZVARS.
ISPF messages starting with ISP
188  z/OS: z/OS ISPF Messages and Codes

## Page 209

Programmer response
Define and assign the actual name of the field
(represented by the Z placeholder) to the control
variable .ZVARS in the panel initialization section.
ISPP241 Panel 'aaaaaaaa' error - Number
of .ZVARS field names does not
equal number of "Z" in/out fields.
Explanation
There is a one-to-one correspondence between the
placeholder Z variables and the corresponding field
names for the Z variables defined and assigned in the
initialization section to the control variable .ZVAR. For
example, if the )BODY section contains 3 Z variables,
the )INIT section must define and assign 3 field names
to the control variable .ZVAR.
User response
Contact the responsible programmer.
Programmer response
VERIFY that there is a one-to-one correspondence
between the Z variables in the )BODY section and
the field names defined and assigned to the control
variable, .ZVAR, in the )INIT section.
ISPP242 Panel 'aaaaaaaa' error -
The .ZVARS value contains invalid
variable name (1-8 alphanumeric
chars)
Explanation
The field name defined and assigned to the control
variable, .ZVAR, is invalid. Dialog variable names
can be composed of 1-8 characters. Alphanumeric
characters A-Z, 0-9, #, $, or @ can be used in the
name, but the first character cannot be numeric.
User response
Contact the responsible programmer.
Programmer response
Follow the naming convention described here for
variable names.
ISPP243 Panel 'aaaaaaaa' error - A
"Z" variable replacement name
within .ZVARS is a duplicate field
name.
Explanation
This message indicates that there is a duplicate
field name defined and assigned to the control
variable .ZVARS.
User response
Contact the responsible programmer.
Programmer response
Check the .ZVAR assignment statement for duplicate
names and ensure all names in the list are unique.
ISPP244 Panel 'aaaaaaaa' error - The
constructed "bbbbbbbb" string
exceeds 255 characters.
Explanation
The constructed VPUT or VGET string has exceeded
the VPUT or VGET string length of 255 characters.
User response
Contact the responsible programmer.
Programmer response
Limit the constructed string to the maximum of 255
characters.
ISPP245 Panel 'aaaaaaaa' error - Invalid
"bbbbbbbb" variable name (not a
literal or dialog variable).
Explanation
The VPUT or VGET statement contained an invalid
variable name or a missing right parenthesis.
User response
Contact the responsible programmer.
Programmer response
VERIFY the VGET or VPUT statement contains valid
variable names and VERIFY that there is no missing
right parenthesis.
ISPP246 Panel 'aaaaaaaa' error -
Unexpected ")" delimiter found in
bbbbbbbb. statement.
Explanation
A closing right parenthesis was found in an
unexpected position in a VGET or VPUT statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  189

## Page 210

User response
Contact the responsible programmer.
Programmer response
Check the VPUT or VGET statement for the misplaced
right parenthesis.
ISPP247 Panel 'aaaaaaaa' error -
"bbbbbbbb" pool TYPE or copy
destination is not a literal value.
Explanation
The variable pool TYPE or copy destination on the
VGET or VPUT statement is not a literal value.
User response
Contact the responsible programmer.
Programmer response
The variable pool TYPE or copy destination on the
VGET or VPUT statement must be a literal value. Valid
literal values are: ASIS, SHARED, or PROFILE.
ISPP248 Panel 'aaaaaaaa' error - .ZVARS
name list begins with "(" but does
not end with ")".
Explanation
If a list of names is assigned to .ZVARS, the name list
must be enclosed in parentheses. The .ZVARS name
list is missing a right parenthesis.
User response
Contact the responsible programmer.
Programmer response
Supply the missing right parenthesis to the .ZVARS
statement name list.
ISPP249 Panel 'aaaaaaaa' error - Invalid
VEDIT variable name
Explanation
The name of the dialog variable specified in the VEDIT
statement is invalid.
User response
Contact the responsible programmer.
Programmer response
VERIFY the name of the dialog variable specified in the
VEDIT statement follows variable naming conventions.
A name is composed of 1 to 8 alphanumeric
characters (A-Z, 0-9, #, $, or @). The first character
of the variable name cannot be numeric.
ISPP250 Panel 'aaaaaaaa' error - Invalid
ROWS keyword value (not a literal
or dialog variable).
Explanation
Table display panels contain a )MODEL section with
the optional keyword ROWS(value). The value for the
ROWS keyword can be either a dialog variable or a
literal but the ROWS keyword value found is neither a
dialog variable nor a literal.
Programmer response
Correct the ROWS keyword value to be a literal or a
dialog variable.
ISPP251 Panel 'aaaaaaaa' error - Invalid
ROWS keyword value given on
model line (not SCAN or ALL)
Explanation
The )MODEL statement supports the optional keyword
ROWS(value), where value can be ALL, SCAN, or a
dialog variable that is assigned the value ALL or SCAN.
User response
Contact the responsible programmer.
Programmer response
Change the ROWS keyword value to SCAN or ALL.
ISPP252 Panel 'aaaaaaaa' error - Invalid
ROWS keyword value (dialog var)
found while initializing panel.
Explanation
The dialog variable used as the value for ROWS
keyword on the )MODEL statement is set to an invalid
value. A valid value for the ROWS keyword is SCAN or
ALL.
User response
Contact the responsible programmer.
ISPF messages starting with ISP
190  z/OS: z/OS ISPF Messages and Codes

## Page 211

Programmer response
Change the ROWS keyword value to SCAN or ALL.
ISPP253 Panel 'aaaaaaaa' error - SKIP(ON)
or ATTN(ON) attributes are not
valid for input fields.
Explanation
The attribute section of the panel contains an
attribute character with an attribute TYPE of input
and the attribute keyword SKIP(ON) or ATTN(ON). The
attribute keyword SKIP is valid only for text fields or
output (protected) fields. The attribute keyword ATTN
is valid only for text fields.
User response
Contact the responsible programmer.
Programmer response
Remove the attribute keyword SKIP(ON) or ATTN(ON)
from the attribute defined with the TYPE value of
input.
ISPP254 Panel 'aaaaaaaa' error - ATTN(ON)
attribute is valid for only protected
text fields.
Explanation
The attribute ATTN(ON) is valid only on fields with the
attribute TYPE of protected text.
User response
Contact the responsible programmer.
Programmer response
Remove the attribute keyword ATTN(ON) from the
attribute where attribute TYPE is not protected text.
ISPP255 Panel 'aaaaaaaa' error - A blank
or null designator character must
immediately follow an attention-
select attribute.
Explanation
The ATTN(ON) keyword specifies that the field can be
selected by using a cursor select key. This attribute is
valid only for fields with the TYPE protected text. The
panel designer must provide an adequate number of
blank characters before and after the single character
or 2-digit hexadecimal code that defines this TYPE of
field, as required by 3270 hardware.
User response
Contact the responsible programmer.
Programmer response
Provide an adequate number of blank characters
before and after the single character or 2-digit
hexadecimal code that defines this TYPE of field, as
required by 3270 hardware.
ISPP256 Panel 'aaaaaaaa' error -
NUMERIC(ON) attribute is valid
only for unprotected fields.
Explanation
The attribute keyword NUMERIC(ON) is valid only for
fields with the TYPE of unprotected input.
User response
Contact the responsible programmer.
Programmer response
Remove the NUMERIC(ON) attribute keyword from the
field that does not have the TYPE of unprotected input.
The NUMERIC(ON) keyword is valid only for fields with
the TYPE of unprotected input.
ISPP257 Panel 'aaaaaaaa' error - Cannot
specify both SKIP(ON) and
NUMERIC(ON) attributes.
Explanation
NUMERIC(ON) and SKIP(ON) attribute keywords
cannot be specified for the same field.
User response
Contact the responsible programmer.
Programmer response
Use either the NUMERIC(ON) attribute keyword or the
SKIP(ON) attribute keyword but not both for the same
field.
ISPP258 Panel 'aaaaaaaa' error - The
attribute keyword, CKBOX, is valid
on these type of input fields:
INPUT, DATAIN, CEF, LEF, NEF, and
EE. It is also valid on output field
type, SC.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  191

## Page 212

Explanation
The attribute keyword, CKBOX, was specified on an
invalid attribute type. CKBOX(ON|OFF) is valid on
these attribute types: INPUT, DATAIN, CEF, LEF, NEF,
and EE. CKBOX(ON|OFF) is also valid on output field
type, SC, to allow for an unavailable selection field.
User response
Contact the responsible programmer.
Programmer response
The CKBOX attribute keyword allows a one character
input field followed by an output field to be processed
as a check box on a client that is using the JSON
API. To code for check boxes, code the keyword,
CKBOX(ON), on an input field type. The valid input field
types are: INPUT, DATAIN, CEF, LEF, NEF, and EE. For
providing the option of unavailable selection fields, the
output field type, SC, accepts the CKBOX(ON) keyword
used in conjunction with the UNAVAIL(ON) keyword.
ISPP259 Panel 'aaaaaaaa' error - The
attribute keyword, PAS, is valid
only for input or output field types.
Explanation
The attribute keyword, PAS(ON/OFF), was specified
on a text field. The attribute keyword PAS(ON/OFF) is
valid for input and output fields only.
User response
Contact the responsible programmer.
Programmer response
Verify that the panel definition has the PAS attribute
keyword defined on input or output field types only.
The PAS attribute keyword can be defined in the )ATTR
section or used in attribute override statements.
ISPP260 Panel 'aaaaaaaa' error - Invalid
alternate field keyword value (not
a literal or dialog variable).
Explanation
The )BODY statement allows you to specify alternate
locations for the system-defined (default) fields for
messages and the command field. The keywords are:
CMD(field-name)
SMSG(field-name)
LMSG(field-name)
The field-name can be a literal or a dialog variable.
This error indicates an invalid keyword value (not a
literal or a dialog variable) has been coded for one of
the three keywords.
Programmer response
Change the keyword value to a literal or a dialog
variable.
ISPP261 Panel 'aaaaaaaa' error - Invalid
alternate command field name
given (must be a panel field
name).
Explanation
The )BODY statement keyword CMD has an invalid
field-name value. The field-name value must be a
panel field name.
Programmer response
Correct the CMD field name value to a valid panel field
name. The field-name value can be a literal or a dialog
variable.
ISPP262 Panel 'aaaaaaaa' error - Invalid
alternate short message field
name given (must be a panel
field).
Explanation
The )BODY statement keyword SMSG has an invalid
field-name value. The field-name value must be a
panel field name.
Programmer response
Correct the SMSG field name value to a valid panel
field name. The field-name value can be a literal or a
dialog variable.
ISPP263 Panel 'aaaaaaaa' error - Invalid
alternate long message field name
given (must be a panel field).
Explanation
The )BODY statement keyword LMSG has an invalid
field-name value. The field-name value must be a
panel field name.
Programmer response
Correct the LMSG field name value to a valid panel
field name. The field-name value can be a literal or a
dialog variable.
ISPF messages starting with ISP
192  z/OS: z/OS ISPF Messages and Codes

## Page 213

ISPP264 Panel 'aaaaaaaa' error - Alternate
command or message field name
given is invalid (no panel fields).
Explanation
The )BODY statement keyword CMD, SMSG, or LMSG
has an invalid field-name value because there are no
fields in the panel.
Programmer response
If the panel is to remain as is with no field names,
remove the CMD, SMSG, or LMSG keyword from
the )BODY statement. Otherwise, add a field to the
panel and set the corresponding field name value to
the panel field name you created.
ISPP265 Panel 'aaaaaaaa' error -
There must be exactly two
EXPAND characters, enclosed in
parentheses.
Explanation
The )BODY statement keyword EXPAND(xy) contains
the repetition delimiter characters. You must specify
exactly 2 delimiter characters; the starting delimiter
character (x) and the ending delimiter character
(y). The starting and ending delimiter can be the
same character. The delimiter characters cannot be
specified with a dialog variable.
Programmer response
Correct the starting and ending delimiter characters in
the EXPAND keyword of the )BODY statement.
ISPP266 Panel 'aaaaaaaa' error - The
alternate command field name
given is not an input field.
Explanation
The CMD keyword field name of the )BODY statement
identifies the panel field (variable name) to be treated
as the command field. The field name it identifies must
be TYPE(INPUT).
Programmer response
Correct the CMD keyword field name to identify a
panel field which has the TYPE(INPUT).
ISPP267 Panel 'aaaaaaaa' error - The
alternate short or long message
field name given is not an output
field.
Explanation
The LMSG or SMSG keyword field name of the )BODY
statement identifies the panel field (variable name)
to be treated as the long message field or the short
message field. The field the LMSG or SMSG identifies
must be TYPE(OUTPUT).
Programmer response
Correct the LMSG keyword field name or SMSG
keyword field name to identify a panel field which has
the TYPE(OUTPUT).
ISPP270 Panel 'aaaaaaaa' error - Invalid
double digit hex character given
for attribute character or keyword
code.
Explanation
An invalid 2-digit hexadecimal code has been given for
an attribute character or keyword code.
Programmer response
Choose special (non-alphanumeric) characters for
attribute characters that will not conflict with the
panel text. An ampersand (&), blank (hexadecimal
40), shift-out (hexadecimal 0E), shift-in (hexadecimal
0F), or null (hexadecimal 00) cannot be used as an
attribute character.
ISPP271 Panel 'aaaaaaaa' error - The area-
related keyword value must be
explicitly stated (not a dialog
variable)
Explanation
The area related keywords EXTEND(ON|OFF),
SCROLL(ON|OFF) do not allow keyword values to be
specified as a dialog variable.
Programmer response
Do not specify a dialog variable as the value for the
area keywords EXTEND and SCROLL. Valid values for
each keyword are ON or OFF. The default for each
keyword is OFF.
ISPP272 Panel 'aaaaaaaa' error - .ALARM
value was not YES or NO or blank.
Explanation
The control variable .ALARM is set to an invalid value.
Valid values are YES, NO, blank, or null.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  193

## Page 214

Programmer response
Set the .ALARM value to one of the four valid values:
YES, NO, blank, or null.
ISPP273 Panel 'aaaaaaaa' error - Multiple
graphic areas have been defined
on the panel, only 1 allowed.
Explanation
ISPF supports only one graphic area in a panel
definition.
Programmer response
Limit the number of defined graphic areas to one per
panel.
ISPP274 Panel 'aaaaaaaa' error -
Overlapping AREAS have been
defined on the panel.
Explanation
Several AREAS have been defined that overlap on the
panel.
Programmer response
Ensure that the areas defined on the panel do not
overlap.
ISPP275 Panel 'aaaaaaaa' error - Multiple
extend/scroll areas defined on the
panel, only 1 allowed.
Explanation
You can define only one area in the )BODY section
with EXTEND(ON). This rule applies to scrollable areas,
dynamic areas, and graphic areas. For example, if two
scrollable areas are defined in the )BODY section, only
one can be defined with EXTEND(ON).
Programmer response
Define only one area in the )BODY section with the
EXTEND(ON) keyword.
ISPP276 Panel 'aaaaaaaa' error - SCROLL
keyword is valid for only dynamic
areas.
Explanation
The keyword SCROLL is valid only for dynamic areas.
It specifies that the dynamic area can be treated as a
scrollable area.
Programmer response
Use the SCROLL keyword only on dynamic areas that
are defined by the keyword AREA(DYNAMIC).
ISPP277 Panel 'aaaaaaaa' error - First line
of an area must identify the left
and right boundaries.
Explanation
Scrollable, graphic, and dynamic areas require that the
area's attribute (defined in the attribute section) must
designate the right and left boundaries of the area's
first line.
Programmer response
Use the areas attribute character (defined in the
attribute section) to designate the left and right
boundaries of the area's first line.
ISPP278 Panel 'aaaaaaaa' error - A panel
field overlaps the boundary of a
dynamic or scrollable area.
Explanation
A panel field cannot overlap the boundary of a
dynamic or scrollable area.
Programmer response
Ensure that panel fields do not overlap the attribute-
defined boundary of dynamic or scrollable areas.
ISPP279 Panel 'aaaaaaaa' error - Area
cannot be extended, other fields/
areas would be extended also.
Explanation
An invalid field or area extension was entered.
Other fields could be duplicated or areas could
be inadvertently extended if this extension is
implemented.
User response
Contact the responsible programmer.
Programmer response
Ensure that field and area panels are defined properly.
Contact IBM support.
ISPP280 Panel 'aaaaaaaa' error - AREA
attribute keyword given for
something other than AREA
definition.
ISPF messages starting with ISP
194  z/OS: z/OS ISPF Messages and Codes

## Page 215

Explanation
An area-specific keyword was given for an attribute
other than the area attribute. For example,
EXTEND(ON) was used on something other than the
graphic, dynamic, or scrollable area definition. Another
example, SCROLL, USERMOD, or DATAMOD keyword
was used on something other than a dynamic or
graphic area definition.
Programmer response
Verify that your attribute keywords in the )ATTR
sections are used with the correct attribute definitions.
ISPP281 Panel 'aaaaaaaa' error - Invalid
attribute keyword given as part of
an AREA definition.
Explanation
An invalid attribute keyword was given as part
of an area definition. For example, the attribute
keyword JUST(LEFT) is invalid on the dynamic area
(AREA(DYNAMIC)) definition.
Programmer response
Verify that all the attribute keywords used on the
scrollable, dynamic, or graphic area definitions are
valid for that specific area definition.
ISPP282 Panel 'aaaaaaaa' error - A panel
field overlaps the boundary of a
dynamic or scrollable area.
Explanation
A panel field may not be defined to overlap a dynamic
or scrollable area.
Programmer response
Ensure that no panel fields overlap a dynamic or
scrollable area.
ISPP283 Panel 'aaaaaaaa' error - .CSRPOS
or .CSRROW value was not a valid
numeric value.
Explanation
The .CSRPOS and .CSRROW value must be an integer.
Programmer response
Verify that the .CSRPOS and .CSRROW values are
integers.
ISPP284 Panel 'aaaaaaaa' error -
Invalid .CSRPOS or .CSRROW
value found while processing
panel.
Explanation
The .CSRPOS and .CSRROW value must be an integer.
Programmer response
Verify that .CSRPOS and .CSRROW values are integers.
ISPP285 Panel 'aaaaaaaa' error - Invalid
LVLINE keyword value type (not a
literal or dialog variable).
Explanation
The LVLINE keyword value must be a literal or a dialog
variable. The line number of the final visible line within
a dynamic or graphic area on a screen is available
through the use of the LVLINE built-in function on an
assignment statement in the )INIT, )REINIT, or )PROC
section panel. The value parameter of the LVLINE
function is the name of the graphic or dynamic area.
Programmer response
Assign the LVLINE keyword value a literal or a dialog
variable that is (or contains the name of) the graphic or
dynamic area.
ISPP286 Panel 'aaaaaaaa' error - The area
referenced by the LVLINE function
is not a dynamic or graphic area on
the panel.
Explanation
The LVLINE built-in function provides the line number
of the last visible line within a graphic or dynamic area
on the currently displayed panel. The value parameter
of the LVLINE function is the name of the graphic or
dynamic area. This function is valid for dynamic or
graphic areas only.
Programmer response
Verify that the value of the LVLINE function is a literal
that is the name of the graphic or dynamic area, or a
dialog variable that contains the name of the graphic
or dynamic area.
ISPP287 Panel 'aaaaaaaa' error - Invalid
LVLINE area name found while
processing panel.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  195

## Page 216

Explanation
The LVLINE built-in function provides the line number
of the last visible line within a graphic or dynamic area
on the currently displayed panel. The value parameter
of the LVLINE function is the name of the graphic or
dynamic area. This function is valid for dynamic or
graphic areas only.
Programmer response
Verify that the value of the LVLINE function is a literal
that is the name of the graphic or dynamic area or a
dialog variable that contains the name of the graphic
or dynamic area.
ISPP288 Panel 'aaaaaaaa' error - Invalid
WIDTH keyword value type (not a
literal or dialog variable).
Explanation
An invalid WIDTH keyword value has been found.
The WIDTH keyword's value on the )BODY header
statement can be a literal or a dialog variable. The
WIDTH value must be numeric, not less than 80 or
greater than the width of the terminal on which the
panel is to be displayed.
Programmer response
Verify that the WIDTH keyword value on the )BODY
header statement is a literal or a dialog variable.
ISPP289 Panel 'aaaaaaaa' error - Invalid
WIDTH value, (must be numeric
chars, >= 80, <= screen width).
Explanation
An invalid WIDTH keyword value has been found. The
value of the WIDTH keyword on the )BODY header
statement can be a literal or a dialog variable. The
WIDTH value must be numeric, not less than 80 or
greater than the width of the terminal on which the
panel is to be displayed.
Programmer response
Verify that the WIDTH keyword value is numeric and
not less than 80 or greater than the width of the
terminal on which the panel is to be displayed.
ISPP290 Panel 'aaaaaaaa' error - The
EXPAND designation is improperly
used within the panel record.
Explanation
Within the panel body, the expand delimiters have
been used improperly. The format of the EXPAND
keyword on the )BODY statement is: EXPAND(xy).
The value, xy, represents the repetition delimiter
characters. Once defined, these delimiters can be
used on any line within the panel body to enclose
a single character that is to be repeated to expand
the line to the required width. A missing right or left
delimiter around the single character to be expanded
could cause this error.
Programmer response
Verify that the expand character in the panel body
record is enclosed by both a starting and an ending
delimiter.
ISPP291 Panel 'aaaaaaaa' error - EXPAND
keyword value may not be blank,
"&", or an attribute character.
Explanation
The format of the EXPAND keyword on the )BODY
statement is: EXPAND(xy). The value, xy, represents
the repetition delimiter characters. The delimiter
characters cannot be blanks, dialog variables (&xx), or
attribute characters.
Programmer response
Verify that the delimiter characters are not blanks,
dialog variables, or attribute characters.
ISPP292 Panel 'aaaaaaaa' error - Invalid
field/area name specified on
REFRESH statement. It must be
less than or equal to 8 characters.
Explanation
The field or area name specified on the REFRESH
statement had an invalid length. The field or area
name must be fewer than or equal to 8 characters.
Programmer response
Correct the field or area name to be fewer than or
equal to 8 characters.
ISPP293 Panel 'aaaaaaaa' error - REFRESH
is specified for a field or area that
is not on the panel.
ISPF messages starting with ISP
196  z/OS: z/OS ISPF Messages and Codes

## Page 217

Explanation
ISPF searched the fields in the panel body to find
a match for the field area name specified on the
REFRESH statement and could not find a match.
Programmer response
Verify that the field or area name specified on the
REFRESH statement in the )PROC or )REINIT section
is spelled correctly or is actually the name of a field or
area as defined in the panel body.
ISPP294 Panel 'aaaaaaaa' error - The .ATTR
or .ATTRCHAR argument must be
enclosed in parentheses.
Explanation
An invalid format for the .ATTR or .ATTRCHAR
statement was found. The format is: .ATTR(field)
or .ATTRCHAR(char).
Programmer response
Enclose the .ATTR or .ATTRCHAR argument in
parentheses.
ISPP295 Panel 'aaaaaaaa' error - .ATTR
argument must be a literal, a
dialog variable, or .CURSOR.
Explanation
The .ATTR argument must be a literal, a dialog
variable, or the control variable .CURSOR.
Programmer response
Correct the .ATTR argument to be either a literal, a
dialog variable, or the control variable .CURSOR.
ISPP296 Panel 'aaaaaaaa' error - .ATTR is
specified for a field that is not on
the panel.
Explanation
The .ATTR argument specified does not match any
field on the panel.
Programmer response
Verify that the .ATTR argument is set to a valid field on
the panel.
ISPP297 Panel 'aaaaaaaa' error - .ATTR
or .ATTRCHAR may appear only on
left side of assignment stmt.
Explanation
The .ATTR or .ATTRCHAR keyword may appear only
on the left side of the assignment statement. For
example: .ATTR(field1) = 'COLOR(RED)'
Programmer response
Verify that the .ATTR or .ATTRCHAR keywords appear
only on the left side of the assignment statement.
ISPP298 Panel 'aaaaaaaa' error
- .ATTRCHAR argument must be a
literal or a dialog variable.
Explanation
An invalid .ATTRCHAR argument was used.
The .ATTRCHAR argument must be a dialog variable or
a literal.
Programmer response
Verify that all .ATTRCHAR arguments are either dialog
variables or literals.
ISPP299 Panel 'aaaaaaaa' error
- .ATTRCHAR is specified for an
invalid attribute character.
Explanation
The .ATTRCHAR argument specified is not an attribute
character defined on the panel.
Programmer response
Ensure that all .ATTRCHAR arguments specified are
attribute characters defined in the )ATTR section of the
panel.
ISPP300 Panel 'aaaaaaaa' error - ATTR
override keyword value not given,
or not enclosed in parentheses.
Explanation
The ATTRIBUTE statement is entered incorrectly.
Programmer response
Correct the TYPE or VALUE on the ATTRIBUTE
statement.
ISPP301 Panel 'aaaaaaaa' error - A value
is not specified for one of the
attribute override keywords.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  197

## Page 218

Explanation
An attribute override statement is missing a keyword
required for that field type.
Programmer response
Check the panel for attribute overrides and ensure that
all field types have the required keywords.
ISPP302 Panel 'aaaaaaaa' error - Invalid
attribute keyword or value is
specified for attribute override.
Explanation
The attribute override keyword is invalid, or the value
used with the keyword is invalid.
Programmer response
Check the attribute override statements on the panel.
Ensure that all field types have valid keywords, and all
values following the keywords are valid.
ISPP303 Panel 'aaaaaaaa' error - Attribute
override may not be specified for
area delimiter keywords.
Explanation
The panel has improperly used one of these in an
attribute override:
• Area
• Rep
• Extend
• Scroll
• Usermod
• Datamod.
Programmer response
Correct the panel so that it does not use any of the
previous list in an attribute override.
ISPP304 Panel 'aaaaaaaa' error - Duplicate
keywords were specified on an
ATTRIBUTE override statement.
Explanation
An attribute override statement has used the same
keyword twice.
Programmer response
Correct the attribute override statement in your panel.
ISPP305 Panel 'aaaaaaaa' error - Invalid
attribute TYPE change attempted
in an ATTR override statement.
Explanation
There is an invalid type attribute override.
Programmer response
For more information about the .ATTR and .ATTRCHAR
override conditions, see z/OS ISPF Dialog Developer's
Guide and Reference
ISPP306 Panel 'aaaaaaaa' error
- .ATTRCHAR defines a new
character that is not DATAIN or
DATAOUT.
Explanation
A new attribute character must be used on DATAIN or
DATAOUT field.
Programmer response
Do not use a new attribute character unless it is for a
DATAIN or DATAOUT field.
ISPP307 Panel 'aaaaaaaa' error - Invalid
override of cmd field or scroll amt
field to TYPE(OUTPUT)
Explanation
TYPE(OUTPUT) cannot be used to override the
command field or the scroll amount field.
Programmer response
Correct the override on the panel.
ISPP308 Panel 'aaaaaaaa' error - Invalid
PAD character specified in .ATTR
or .ATTRCHAR statement.
Explanation
An .ATTR or .ATTRCHAR statement used an invalid
PAD character.
Programmer response
Correct the .ATTR or .ATTRCHAR statement in the
panel.
ISPP309 Panel 'aaaaaaaa' error - .AUTOSEL
value was not YES or NO or blank.
ISPF messages starting with ISP
198  z/OS: z/OS ISPF Messages and Codes

## Page 219

Explanation
Set .AUTOSEL to YES to retrieve the CSRROW, even if
the user did not explicitly select the row. Set .AUTOSEL
to NO if the row is to be retrieved only if the user
explicitly selects the row.
Programmer response
Correct the .AUTOSEL statement in the panel.
ISPP310 Panel 'aaaaaaaa' error - Maximum
allowable number of attributes
exceeded by attribute override.
Explanation
Attribute overrides plus attributes exceed 255.
Programmer response
Correct the panel.
ISPP311 Panel 'aaaaaaaa' error - String
of attribute override keywords
exceeds maximum size of 255
characters.
Explanation
The string of keywords on a .ATTR or .ATTRCHAR
statement exceeds 255 characters.
Programmer response
Correct the panel.
ISPP312 Panel 'aaaaaaaa' error -
"Scrollable" or "extendable" area
not allowed in a table display
panel.
Explanation
The SCROLL and EXTEND keywords are not valid when
defining an area in a TBDISPL panel.
Programmer response
Correct the panel.
ISPP313 Panel 'aaaaaaaa' error - An AREA
definition is not permitted within
the ")MODEL" section.
Explanation
Any attribute except those associated with dynamic,
graphic, or scrollable areas (AREA, EXTEND, SCROLL,
USERMOD, and DATAMOD) can be used with any fields
in the model lines.
Programmer response
Correct the panel )MODEL section.
ISPP314 Panel 'aaaaaaaa' error - Invalid
to specify both PAD and PADC
attributes for one field.
Explanation
You cannot specify both a pad character (PAD) and a
conditional pad character (PADC) for the same field.
Programmer response
Correct the attribute in the panel.
ISPP315 Invalid MODEL definition - A
variable model line may not be
defined as a variable itself.
Explanation
If a variable begins in column 1 of any model line,
the value of that variable defines the model line. The
variable can contain any character string that is a valid
panel definition model line, except that the variable
cannot define a variable model line.
Programmer response
Correct the model line in the panel.
ISPP316 Model line not defined - The
variable model line has not been
initialized before the display.
Explanation
All model line variables must be initialized before the
table display service is called with a nonblank panel
name.
Programmer response
Define a model line in the panel.
ISPP317 Invalid model definition - The
variable model is the only
information allowed coded on the
line.
Explanation
For variable model lines, the variable must be the only
information on the model line.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  199

## Page 220

Programmer response
Correct the model line on the panel.
ISPP318 Invalid model definition - The
variable model line width is
greater than the panel width.
Explanation
The value of the variable is greater than the panel
width.
Programmer response
Correct the model section of the panel.
ISPP319 Panel "aaaaaaaa" error - The
command field may not be located
within the )MODEL section.
Explanation
The command field is identified as either the first input
field in the panel, or by the CMD keyword of the )BODY
statement. If the command field is within the )MODEL
section, an error occurs.
Programmer response
Correct the )MODEL section by removing the command
field.
ISPP320 Panel 'aaaaaaaa' error - Panel
control variable bbbbbbbb;PFKEY
cannot be set by a dialog.
Explanation
bbbbbbbb;PFKEY cannot be set to a value, it is read
only.
Programmer response
Remove the panel statement that attempts to set
bbbbbbbb;PFKEY.
ISPP321 Panel 'aaaaaaaa' error - The INIT,
REINIT, AREA, or PROC section is
too long.
Explanation
There is a potential 64K internal table overflow
because of the number of statements in the panel.
Programmer response
Either reduce the number of statements in the panel or
use more than one panel.
ISPP322 Panel 'aaaaaaaa' error - A panel
must have at least one line in
the )BODY section.
Explanation
The body section defines the format of the panel as
seen by the user. A valid panel must have at least one
line in the )BODY section.
Programmer response
Add at least one panel line into the )BODY section.
ISPP323 Panel 'aaaaaaaa' error - More than
126 pairs have been specified for
the TRANS function.
Explanation
TRANS logic has exceeded the maximum pairs
allowed.
Programmer response
The TRANS function of the panel must use fewer than
126 pairs.
ISPP324 Panel 'aaaaaaaa' error - A panel
cannot be preprocessed when
WIDTH is specified as dialog
variable.
Explanation
The preprocessor will not preprocess panels that use a
dialog variable with the WIDTH keyword.
Programmer response
See "Restrictions for using ISPPREP" in z/OS ISPF
Dialog Developer's Guide and Reference.
ISPP325 Panel 'aaaaaaaa' error - A panel
cannot be preprocessed with
areas defined as extendable.
Explanation
The specification of EXTEND(ON) will cause ISPPREP
to bypass the panel during preprocessing.
Programmer response
Determine if EXTEND(ON) is needed. If it is needed,
the panel cannot be preprocessed.
ISPP326 Panel 'aaaaaaaa' error - A panel
cannot be preprocessed when
ISPF messages starting with ISP
200  z/OS: z/OS ISPF Messages and Codes

## Page 221

model lines are specified as
variables.
Explanation
Model lines cannot be defined as variables if the
panels are to be preprocessed.
Programmer response
If the panel must be preprocessed, the model lines
must not be specified as variables.
ISPP327 Panel 'aaaaaaaa' error - Variable
name or name-list not coded for
'bbbbbbbb' statement.
Explanation
The panel statement requires additional data.
Programmer response
Complete the panel statement by adding a variable or
a name-list.
ISPP328 Panel 'aaaaaaaa' error - Required
keylist 'bbbbbbbb' in applid
'cccccccc' was not found.
Explanation
The applid, cccccccc, is either a single applid, cccc, or
a choice of a 2 applids, cccc or ISP, in the form of cccc/
ISP.
The keylist bbbbbbbb is not found in the ISPTLIB
concatenation for application cccc or ISP (if applid is
of the form cccc/ISP).
User response
Verify that the ISPTLIB concatenation has a keylist
member for the application. For example, if cccccccc
is XXX/ISP, verify that members XXXKEYS or ISPKEYS
are in ddname ISPTLIB. Change your ISPTLIB
concatenation if necessary.
Verify the keylist table has a row with bbbbbbbb in
variable KEYLISTN.
Programmer response
Create a keylist named bbbbbbbb in ccccKEYS (or
ISPKEYS if the applid is of the form cccc/ISP) or
change panel aaaaaaaa to use an existing keylist.
ISPP329 Panel 'aaaaaaaa' error - LVLINE
function can only be used in
an )INIT, )REINIT or )PROC panel
section.
Explanation
The LVLINE built-in function was coded in an invalid
section of the panel.
Programmer response
Delete or move the LVLINE function from the section.
ISPP330 BDISPMAX exceeded - aaaaaaaa
displays exceeded in batch mode
on panel bbbbbbbb.
Explanation
Either the maximum number of displays set in variable
BDISPMAX on the ISPSTART command, or the default
value of 100 has been exceeded.
Programmer response
Verify that the batch job is not looping and, if not,
increase the value of BDISPMAX. See "Avoiding panel
loop conditions in the Batch environment" in z/OS ISPF
Dialog Developer's Guide and Reference .
ISPP331 BREDIMAX exceeded - aaaaaaaa
bbbbbbbb;msg redisplays
exceeded in batch mode.
Explanation
The Batch redisplay value in BREDIMAX on the
ISPSTART command or the default value of 2 has been
exceeded.
Programmer response
Verify that the Batch job is not looping on a redisplay
and, if not, increase the value of BREDIMAX. See
"Message processing in the Batch environment" in
z/OS ISPF Dialog Developer's Guide and Reference .
ISPP332 Panel aaaaaaaa message - In
batch, this message was displayed
on the panel.
Explanation
A panel was displayed while in batch mode. This
message was displayed.
Programmer response
This is an informational message. The message
following this one may contain important information.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  201

## Page 222

Refer to that message number in this documentation
for additional information, if needed.
ISPP333 Panel 'aaaaaaaa' error - A panel
cannot be preprocessed when an
INEXIT section is specified.
Explanation
The preprocessor will not preprocess panels that
specify an INEXIT section.
Programmer response
See "Restrictions for using ISPPREP" in z/OS ISPF
Dialog Developer's Guide and Reference.
ISPP334 Panel 'aaaaaaaa' error -
Unable to locate REXX routine
bbbbbbbb required for panel REXX
processing.
Explanation
A panel *REXX statement specifies the name of an
external routine that can not be located.
Programmer response
Examine the panel and check the name of the panel
REXX routine. Ensure the REXX routine is available in
your SYSPROC or SYSEXEC allocation.
ISPP335 Rexx-defined failure - Panel Rexx
routine-defined failure.
Explanation
The panel Rexx routine set a return code of 8 in
variable ZRXRC but did not store a MSGID to describe
the failure in variable ZRXMSG. This generic message
is provided by ISPF.
Programmer response
If appropriate, provide a meaningful error message for
the user.
ISPP336 Panel 'aaaaaaaa' error - Severe
error encountered during panel
Rexx routine.
Explanation
A severe error was encountered while executing the
panel Rexx routine.
Programmer response
The return code set in variable ZRXRC by the panel
Rexx routine was not 0 or 8. The return code could
have been 20 for severe error or some other return
code that was not recognized by ISPF. Verify that your
Rexx routine set valid data in ZRXRC. If the data is
valid, contact IBM support for assistance.
ISPP337 REXX failure - Severe error
encountered processing a panel
REXX routine.
Explanation
A severe error was encountered while executing the
panel Rexx routine.
Programmer response
Examine the REXX routine to determine the cause of
the REXX routine failure.
ISPP338 Panel 'aaaaaaaa' error - Invalid
dialog variable name encountered
on *REXX panel statement.
Explanation
A dialog variable name specified on a *REXX panel
statement is invalid. Either it is too long or contains
invalid characters.
Programmer response
Ensure that the dialog variable names to be passed
to the REXX routine are valid and do not exceed the
maximum length allowed.
ISPP340 Panel 'aaaaaaaa' error - Panel exit
address must be dialog variable.
Explanation
The panel exit cannot be processed because the
address is not a variable.
Programmer response
Correct the panel exit to include a variable for exit-add.
See "Invoking the panel exit user routine" in z/OS ISPF
Dialog Developer's Guide and Reference.
ISPP341 Panel 'aaaaaaaa' error - Panel
exit message ID must be literal or
dialog variable.
ISPF messages starting with ISP
202  z/OS: z/OS ISPF Messages and Codes

## Page 223

Explanation
If the MSG keyword of the PANEXIT statement is
specified, the message id must be either a literal value
or a dialog variable.
Programmer response
Correct the panel exit routine, MSGID must be a dialog
variable or a literal value.
ISPP342 Panel 'aaaaaaaa' error - Panel exit
type must be a literal.
Explanation
The exit type must be PGM or LOAD.
Programmer response
Use the literal PGM or LOAD as the exit type.
ISPP343 Panel 'aaaaaaaa' error - Valid
panel exit type is PGM or LOAD.
Explanation
This message is self-explanatory.
Programmer response
Use PGM or LOAD for exit type.
ISPP344 Panel 'aaaaaaaa' error - Invalid
dialog variable name length on
panel exit statement.
Explanation
The variable name length on the panel exit statement
exceeds 8 characters.
Programmer response
Use a variable name with a length not exceeding 8
characters.
ISPP345 Exit-defined failure - Panel exit
routine exit-defined failure.
Explanation
The exit routine returned RC=8 but did not provide a
MSGID to describe the failure. This generic message is
provided by ISPF.
Programmer response
If appropriate, provide a meaningful error message for
the user.
ISPP346 Panel 'aaaaaaaa' error - Severe
error encountered during panel
exit routine.
Explanation
A severe error was encountered while executing the
panel exit routine.
Programmer response
The return code from exit processing was not 0 or 8.
The return code could have been 20 for severe error
or some other return code that was not recognized by
ISPF. Verify that your exit returned valid data. If the
data is valid, contact IBM support for assistance.
ISPP347 Panel 'aaaaaaaa' error - Dialog
variable value length changed by
panel exit.
Explanation
The panel exit has changed the variable length from
the length that was set on the panel exit statement.
Programmer response
Your panel exit cannot change the variable length that
was used on the panel exit statement.
ISPP348 Panel 'aaaaaaaa' error - Exit
data or message field defined
incorrectly for panel exit.
Explanation
The panel exit statement is incorrectly formatted.
Programmer response
Reformat the panel exit statement using the format
defined under "Invoking the Panel User Exit Routine"
in z/OS ISPF Dialog Developer's Guide and Reference.
ISPP349 Panel 'aaaaaaaa' error - Invalid
panel exit address or panel exit
name specified.
Explanation
The panel exit statement has invalid syntax.
Programmer response
The panel exit-add or exit-mod entry in the panel exit
statement does not conform to the syntax defined in
"Invoking the Panel User Exit Routine" in z/OS ISPF
Dialog Developer's Guide and Reference .
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  203

## Page 224

ISPP350 Panel 'aaaaaaaa' error - Invalid
WIDTH keyword value type (not a
literal or dialog var).
Explanation
This message is self explanatory.
ISPP351 Panel 'aaaaaaaa' error - Invalid
window width value (must be
numeric, >= 8, less than screen
width).
Explanation
The value specified for the width on the WINDOW
keyword on the )BODY section panel statement is
invalidtf68 must be a numeric value that is greater
than or equal to 8 and less than the screen width.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the width coded in the panel definition.
ISPP352 Panel 'aaaaaaaa' error - Invalid
window depth value (must be
numeric, > 0, less than screen
depth).
Explanation
The value specified for the depth on the WINDOW
keyword on the )BODY section panel statement is
invalid. It must be a numeric value that is greater than
0 and less than the screen depth.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the depth coded in the panel definition.
ISPP353 Panel 'aaaaaaaa' error - Panel
record longer than specified (or
defaulted) window width.
Explanation
A record in the panel definition is longer than the width
specified on the WINDOW keyword on the )BODY
section panel statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Make sure all of the panel definition statements are no
longer than the value specified (or defaulted) for the
width of the WINDOW keyword on the )BODY section
panel statement. A common cause for this error is data
past column 72; scroll right to see the data.
ISPP354 Panel 'aaaaaaaa' error - Number
of body records > specified (or
defaulted) window depth.
Explanation
The number of records in the )BODY section is greater
than the depth specified on the WINDOW keyword on
the )BODY section panel statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You can either delete some of the records in
the )BODY section or create a scrollable area out of
the information in the )BODY section.
ISPP355 Panel 'aaaaaaaa' error - Panel
field exceeds the specified (or
defaulted) window width.
ISPF messages starting with ISP
204  z/OS: z/OS ISPF Messages and Codes

## Page 225

Explanation
This message is self explanatory.
ISPP356 Panel 'aaaaaaaa' error - Panel in
window cannot exceed the current
physical screen width.
Explanation
The panel you are attempting to display is wider than
the panel currently being displayed in the pop-up or is
wider than the physical width of the screen.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify either the width of the widest panel to be
displayed or the width of the widest physical display
as the window width.
ISPP357 Panel 'aaaaaaaa' error - Number
of pull-down choices is greater
than screen depth - 2.
Explanation
The number of pull-down choices defined in the panel
is too large to display on the screen. The number must
be less than or equal to the depth of the screen, less 2.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
You might be able to make the panel display if you
change to a display device that has more lines per
screen.
Programmer response
Reduce the number of pull-down choices for the action
bar in which they appear.
ISPP358 Panel 'aaaaaaaa' error - Panel exit
name to load must be literal or
dialog variable.
Explanation
The name of the panel exit is not valid. The panel exit
name, specified on the PANEXIT statement, must be a
dialog variable or a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a dialog variable or literal after LOAD on the
PANEXIT statement.
ISPP359 Panel 'aaaaaaaa' error - Load of
panel exit routine failed.
Explanation
ISPF attempted to load the panel exit named, but
received an error and could not continue.
User response
There was a programming error running the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
It is possible that you do not have all of the data sets
allocated that the application expects. In this case,
contact your system administrator.
Programmer response
It is possible that you are not allocating all of the data
sets that are required for your application. If you think
you should be able to load the exit, then you might be
specifying the name of the module incorrectly on the
PANEXIT statement.
ISPP360 Panel 'aaaaaaaa' error - VER
statement syntax error on LISTV
keyword parameter. Specification
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  205

## Page 226

of the varlist subparameter is
invalid or missing.
Explanation
The LISTV keyword was specified on a VER statement,
but the variable that defines the list of values (the
"varlist") is missing or specified incorrectly.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correctly specify the "varlist". The varlist is the name
of a variable, preceded with an ampersand, that
contains a list of values that will be compared to the
value contained in the verify variable.
ISPP361 Panel 'aaaaaaaa' error -
Specification error found for
VER statement LISTV keyword
parameter. The dialog variable
specified in the varlist
subparameter is undefined or
cannot be accessed.
Explanation
The dialog variable specified as the varlist on the
LISTV keyword in the VER panel statement is
undefined or cannot be accessed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the specification. Perhaps the variable has not
been defined to the dialog using VDEFINE.
ISPP362 Panel 'aaaaaaaa' error -
Specification error found for
VER statement LISTV keyword
parameter. The dialog variable
specified in the varlist
subparameter contains invalid
data.
Explanation
The dialog variable specified as the "varlist" on the
LISTV keyword in the VER panel statement is has
invalid data.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the definition of the varlist.
ISPP363 Panel 'aaaaaaaa' error -
Specification error found for
VER statement LISTV keyword
parameter. The dialog variable
specified in the varlist
subparameter contains more than
100 values.
Explanation
Too many values were specified for a "varlist".
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Reduce the number of values in the varlist to 100 or
less.
ISPP364 Panel 'aaaaaaaa' error - VER
statement syntax error on
INCLUDE keyword parameter.
Specification of the IMBLK and
VALUEN subparameters are out of
order.
ISPF messages starting with ISP
206  z/OS: z/OS ISPF Messages and Codes

## Page 227

Explanation
The IMBLK optional positional subparameter must be
specified before the VALUEN subparameters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the error by specifying IMBLK before ALPHA,
ALPHAB, or NUM on the VER statement.
ISPP365 Panel 'aaaaaaaa' error - VER
statement syntax error on
INCLUDE keyword parameter.
Subparameter 'value1' must be
specified.
Explanation
ALPHA, ALPHAB, or NUM were not specified after
INCLUDE (or IMBLK) on the VER statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the error by specifying at least one of these
values: ALPHA, ALPHAB, or NUM.
ISPP366 Panel 'aaaaaaaa' error - VER
statement syntax error on
INCLUDE keyword parameter. An
invalid value has been specified
for 'value1' and/or 'value2'.
Explanation
The only valid values for INCLUDE are ALPAH,
ALPHAB, and NUM.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the error by specifying ALPAH, ALPHAB, and/or
NUM.
ISPP367 Panel 'aaaaaaaa' error - VER
statement syntax error on
INCLUDE keyword parameter.
More than two values have been
specified.
Explanation
Only two values may be specified on the INCLUDE
keyword on the VER statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the error by specifying only two of the valid
values: ALPHA, ALPHAB, or NUM.
ISPP368 Panel 'aaaaaaaa' error - ALPHAB
keyword invalid on VER statement.
No pointer to translate table.
Explanation
No translate table exists for the lowercase letters.
System programmer response
Ensure that the translate tables for your language have
been properly installed for ISPF.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  207

## Page 228

to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPP370 Must be alphabetic - Enter
alphabetic characters only (A-Z or
a-z).
Explanation
Only alphabetic characters are accepted in the field
where the error occurred. Letters A-Z and a-z are
accepted, but blanks are not allowed.
User response
Enter a response with valid characters.
ISPP371 Invalid chars entered - Enter
numeric and/or alphabetic
characters (0-9, A-Z, a-z, #, $, or
@).
Explanation
Only alphabetic and selected special characters are
accepted in the field where the error occurred. Letters
A-Z, a-z, #, $, and @ are accepted, but blanks are not
allowed.
User response
Enter a response with valid characters.
ISPP372 Invalid chars entered - Enter
numeric and/or alphabetic
characters only (0-9, A-Z, or a-z).
Explanation
Only alphabetic and numeric characters are accepted
in the field where the error occurred. Letters A-Z,
a-z, and digits 0-9 are accepted, but blanks are not
allowed.
User response
Enter a response with valid characters.
ISPP373 Must be alphabetic - Enter
alphabetic characters (A-Z, a-z, #,
$, or @). The value entered may
also contain blank characters.
Explanation
Only alphabetic characters are accepted in the field
where the error occurred. Letters A-Z, a-z, #, $, and @
are accepted as well as blanks.
User response
Enter a response with valid characters.
ISPP374 Must be alphabetic - Enter
alphabetic characters only (A-Z or
a-z). The value entered may also
contain blank characters.
Explanation
Only alphabetic characters are accepted in the field
where the error occurred. Letters A-Z and a-z are
accepted as well as blanks.
User response
Enter a response with valid characters.
ISPP375 Must be numeric - Enter numeric
characters (0-9). The value
entered may also contain blank
characters.
Explanation
Only numeric characters are accepted in the field
where the error occurred.
User response
Enter a response with valid characters.
ISPP376 Invalid chars entered - Enter
numeric and/or alphabetic
characters (0-9, A-Z, a-z, #, $, or
@). The value entered may also
contain blank characters.
Explanation
Only alphabetic and selected special characters are
accepted in the field where the error occurred. Letters
A-Z, a-z, #, $, and @ are accepted as well as blanks.
User response
Enter a response with valid characters.
ISPP377 Invalid chars entered - Enter
numeric and/or alphabetic
characters only (0-9, A-Z, or a-
z). The value entered may also
contain blank characters.
Explanation
Only alphabetic and numeric characters are accepted
in the field where the error occurred. Letters A-Z, a-z,
#, $, @, and digits 0-9 are accepted as well as blanks.
ISPF messages starting with ISP
208  z/OS: z/OS ISPF Messages and Codes

## Page 229

User response
Enter a response with valid characters.
ISPP378 Unavailable choice - An
unavailable choice (one of
'aaaaaaaa') was selected. Enter
one of the available values.
Explanation
An unavailable choice was selected. Enter one of
the available choices. ISPF can format 72 bytes of
unavailable choices.
User response
Enter one of the available choices. The unavailable
choices cannot be entered.
Programmer response
If the unavailable choices occupy more than 72 bytes,
provide help via a customized message using the MSG
keyword on the VER statement.
ISPP380 Panel 'aaaaaaaa' error - COLOR,
INTENS, or HILITE keywords
cannot be specified on CUA panel
element attributes.
Explanation
Either COLOR, INTENS, or HILITE was specified on an
attribute assignment that has a CUA panel element as
a TYPE. This is not allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Either remove the COLOR, INTENS, or HILITE keyword,
or do not use a CUA panel element for the TYPE.
ISPP381 Panel 'aaaaaaaa' error - The
CAPS keyword value cannot be
overridden on the CUA panel
element attributes AB or RP.
Explanation
Action bar and Reference phrase CUA panel element
attributes cannot have the CAPS keyword specified in
the )ATTR section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Remove the CAPS keyword.
ISPP382 Panel 'aaaaaaaa' error - TYPE(EE)
valid for overrides on input fields
only.
Explanation
The dialog attempted to override an attribute with an
Error Emphasis (EE) CUA panel element TYPE. This is
only valid when the field being overridden is an input
field.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Do not override the attribute with TYPE(EE).
ISPP383 Panel 'aaaaaaaa' error - Invalid
label on GOTO statement.
Explanation
The label for a GOTO statement is syntactically
incorrect. Labels can only be 1 to 8 characters
long and must start with A-Z or a-z. The remaining
characters can be A-Z, a-z, or 0-9.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  209

## Page 230

mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a correct label.
ISPP384 Panel 'aaaaaaaa' error - GOTO
label 'bbbbbbbb' not found.
Explanation
The label specified on a GOTO statement was not
found in the section in which the GOTO is specified.
A label must be followed by a colon.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the label on the GOTO or the label itself.
ISPP385 Panel 'aaaaaaaa' error - TYPE(RP)
attribute valid only on a panel
displayed by ISPF help.
Explanation
Reference Phrase CUA panel element attributes are
only allowed on panels being displayed by ISPF Help
(Tutorial).
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Do not use TYPE(RP) in panels that are not displayed
by ISPF Help.
ISPP386 Panel 'aaaaaaaa' error - TYPE(RP)
attribute usage requires )HELP
section definition.
Explanation
If a reference phrase attribute is used in the )BODY
section, an accompanying statement in the )HELP
section must exist to tell which panel is to be
displayed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Write a corresponding reference phrase statement in
the )HELP section for each reference phrase field in
the )BODY section.
ISPP387 Panel 'aaaaaaaa' error - The
RADIO keyword is only valid on
CEF or SACR fields.
Explanation
The RADIO keyword is only valid on CEF or SACR
fields, if the RADIO keyword is used for any other
attribute types this error message will be displayed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Only use the RADIO keyword for CEF or SACR attribute
types.
ISPP388 Panel 'aaaaaaaa' error - The
'bbbbbbbb' keyword only allows
numbers in the range 1-99.
ISPF messages starting with ISP
210  z/OS: z/OS ISPF Messages and Codes

## Page 231

Explanation
The CSRGRP keyword only allows numbers in the
range 1-99, a CSRGRP value was used that was not
in the range 1-99.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Change the CSRGRP value to be in the valid range from
1-99.
ISPP390 Panel 'aaaaaaaa' error -
Invalid )PANEL statement syntax.
Keyword found not the KEYLIST
keyword.
Explanation
The only keyword that is valid after the )PANEL
statement is KEYLIST. Anything else is an error.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct or remove the existing keyword on the )PANEL
statement.
ISPP391 Panel 'aaaaaaaa' error -
Illegal )PANEL statement keylist-
name parameter (must be a
literal).
Explanation
The keylist-name on the KEYLIST keyword on
the )PANEL statement must be a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the keylist-name.
ISPP392 Panel 'aaaaaaaa' error -
Invalid )PANEL statement syntax.
The keylist-name parameter is
missing.
Explanation
The keylist-name on the KEYLIST keyword on
the )PANEL statement was not specified. It is required.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a keylist-name on the KEYLIST keyword, or
remove the KEYLIST keyword and its parameters to
use the default keylist.
ISPP393 Panel 'aaaaaaaa' error - A
keylist-name value must be 1-8
characters. The first character
must be A-Z or a-z, and any
remaining characters can be A-Z,
a-z, or 0-9.
Explanation
The keylist-name specified on the KEYLIST keyword on
the )PANEL statement has an invalid character.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  211

## Page 232

will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a valid keylist-name on the KEYLIST keyword.
ISPP394 Panel 'aaaaaaaa' error
- Invalid )PANEL statement
syntax. Missing expected right
parenthesis.
Explanation
The KEYLIST keyword requires a closing parenthesis
that was not found.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a closing parenthesis.
ISPP395 Panel 'aaaaaaaa' error -
Illegal )PANEL statement keylist-
applid parameter (must be a
literal).
Explanation
The keylist-applid on the KEYLIST keyword on
the )PANEL statement must be a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the keylist-applid.
ISPP396 Panel 'aaaaaaaa' error - A
keylist-applid value must be 1-4
characters. The first character
must be A-Z or a-z, and any
remaining characters can be A-Z,
a-z, or 0-9.
Explanation
The keylist-applid specified on the KEYLIST keyword
on the )PANEL statement has an invalid character.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a valid keylist-applid on the KEYLIST keyword.
ISPP397 Panel 'aaaaaaaa' error -
Invalid )PANEL statement syntax.
Valid keywords are KEYLIST and
IMAGE.
Explanation
The only keywords that are valid after the )PANEL
statement are KEYLIST and IMAGE. Anything else is
an error.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct or remove the existing keyword on the )PANEL
statement.
ISPP398 Panel 'aaaaaaaa' error -
A )PANEL section statement error
has occurred. The IMAGE(image-
name) keyword value is greater
than 8 characters. The image-
name must be less than or equal to
8 characters and follow TSO data
set member naming conventions.
ISPF messages starting with ISP
212  z/OS: z/OS ISPF Messages and Codes

## Page 233

Explanation
The image-name is greater than 8 characters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the length of the image-name to be less than
or equal to 8 characters.
ISPP399 Panel 'aaaaaaaa' error -
Invalid row or column
value for )PANEL IMAGE(image-
name,row,col) keyword. The row
and column values can be dialog
variables, or numeric(0-9).
Explanation
The row or column specified is not a dialog variable, or
is not numeric.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the row and column to dialog variables, or
numeric values.
ISPP400 Panel 'aaaaaaaa' error - Invalid or
missing keyword on )ABC section
heading.
Explanation
The only keyword allowed on the )ABC panel
statement is DESC and it is required.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct or add the DESC keyword and its parameters
to the )ABC panel statement.
ISPP401 Panel 'aaaaaaaa' error - )ABCINIT
section heading statement has an
invalid format.
Explanation
The )ABCINIT panel statement does not accept any
keywords or parameters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Only specify )ABCINIT on the panel statement.
ISPP402 Panel 'aaaaaaaa' error
- )ABCPROC section heading
statement has an invalid format.
Explanation
The )ABCPROC panel statement does not accept any
keywords or parameters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Only specify )ABCPROC on the panel statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  213

## Page 234

ISPP403 Panel 'aaaaaaaa' error - Invalid
statement or keyword found
within )ABC section.
Explanation
A statement in the )ABC section was not a valid panel
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the panel statement.
ISPP404 Panel 'aaaaaaaa' error - Invalid
statement or keyword found
within )ABCINIT section.
Explanation
A statement or keyword in the )ABCINIT section was
not a valid panel statement or keyword.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the panel statement.
ISPP405 Panel 'aaaaaaaa' error - Invalid
statement or keyword found
within )ABCPROC section.
Explanation
A statement or keyword in the )ABCPROC section was
not a valid panel statement or keyword.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the panel statement.
ISPP406 Panel 'aaaaaaaa' error - Invalid
value found for DESC keyword in
section )ABC heading.
Explanation
The value specified for the description text in the DESC
keyword on the )ABC was not a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the text on the DESC keyword.
ISPP407 Panel 'aaaaaaaa' error - Action
bar field text in )BODY section
not found in matching )ABC
descriptive text keyword.
Explanation
The text of the action bar in the )BODY section did not
match the text in the DESC keyword on the )ABC panel
statement. They must match exactly.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the text either in the )BODY section or in
the )ABC section.
ISPF messages starting with ISP
214  z/OS: z/OS ISPF Messages and Codes

## Page 235

ISPP408 Panel 'aaaaaaaa' error - Action
bar text field requires leading
blank in )BODY section.
Explanation
At least one blank must exist between the action bar
attribute character and the text of the action bar in
the )BODY section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add at least one blank between the action bar
attribute character and the text of the action bar.
ISPP409 Panel 'aaaaaaaa' error - )ABC
text description keyword value
does not match description value
on )ABC heading.
Explanation
The text of the action bar in the )BODY section did not
match the text in the DESC keyword on the )ABC panel
statement. They must match exactly.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the text in either the )BODY section or in
the )ABC section.
ISPP410 Panel 'aaaaaaaa' error - An )ABC
section must contain at least one
statement.
Explanation
An )ABC section must contain at least an ACTION
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add at least an ACTION statement to the )ABC section.
ISPP411 Panel 'aaaaaaaa' error -
An )ABCINIT section must contain
at least one statement.
Explanation
There must be at least one statement in the )ABCINIT
section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add a statement to the )ABCINIT section.
ISPP412 Panel 'aaaaaaaa' error -
An )ABCPROC section must
contain at least one statement.
Explanation
There must be at least one statement in the )ABCPROC
section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add a statement to the )ABCINIT section.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  215

## Page 236

ISPP413 Panel 'aaaaaaaa' error - Invalid
value for PDC name keyword
found within )ABC section.
Explanation
The keyword specified on the PDC statement was not
DESC. DESC is the only keyword accepted.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the PDC statement.
ISPP414 Panel 'aaaaaaaa' error - Invalid
value for PDC descriptive
text keyword found within )ABC
section.
Explanation
The value specified in parentheses following the DESC
keyword is invalid. It must be no longer than 64
characters and, if the value contains blanks or special
characters, it must be enclosed in single quotes.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the PDC statement.
ISPP415 Panel 'aaaaaaaa' error - Invalid
RUN() value found on ACTION
statement.
Explanation
The value specified in the parentheses on the RUN
keyword of the ACTION statement in the )ABC section
is invalid. The command name must be 2 to 8
characters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ACTION statement.
ISPP416 Panel 'aaaaaaaa' error - Invalid
PARM() value found on ACTION
statement.
Explanation
The value specified in parentheses on the PARM
keyword of the ACTION statement in the )ABC section
is invalid. If parameters contain special characters or
blanks, they must be enclosed in quotes.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ACTION statement.
ISPP417 Panel 'aaaaaaaa' error -
Maximum number of )ABC sections
exceeded.
Explanation
A maximum of 40 action bars are allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
216  z/OS: z/OS ISPF Messages and Codes

## Page 237

Programmer response
Limit the panel to 40 action bars.
ISPP418 Panel 'aaaaaaaa' error - The
action bar pull-down area is too
long.
Explanation
Too many pull-downs have been defined to display the
panel. The maximum number of pull-down choices is
the depth of the screen less 2.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Reduce the number of pull-down choices for the action
bar.
ISPP419 Panel 'aaaaaaaa' error - Multiple
ACTION statements found after a
PDC statement.
Explanation
Only one action statement is allowed per PDC
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify only one ACTION statement per PDC
statement.
ISPP420 Panel 'aaaaaaaa' error - Invalid
'ACC' keyword value - text must
be less than bbbbbbbb characters,
and contain only 3 keys. If there
are blanks between the key values
and the plus sign (+), then the ACC
value text string must be enclosed
in single quotes.
Explanation
The accelerator text specified as the ACC keyword
value in the PDC statement is longer than the
maximum allowed length, more than three keys were
specified, or a string with intervening blanks is not
enclosed in single quotes.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the accelerator text value in the ACC keyword
on the PDC statement.
ISPP421 Cursor not on choice - The cursor
was not positioned on an action
bar choice.
Explanation
The cursor was not on an action bar choice when the
Enter key was pressed. This usually occurs when the
cursor is in the first row of the first column of the
action bar or after the last action bar choice.
User response
Move the cursor to the text of an action bar choice and
press Enter.
ISPP422 Select a choice - Select a choice by
entering a value or positioning the
cursor on the choice.
Explanation
The cursor was not on a pull-down choice when the
Enter key was pressed. The cursor was probably on
the pull-down menu border.
User response
Move the cursor to a pull-down selection and press
Enter.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  217

## Page 238

ISPP423 Conflicting selection - Choice
number entered and cursor
positioned on different choice.
Explanation
A pull-down choice was selected both by the selection
field and by positioning the cursor on a pull-down
choice. You can only select a pull-down choice using
one of these methods at a time.
User response
Select a pull-down choice by either entering a number
in the select field or by positioning the cursor and
pressing Enter, not both.
ISPP424 Invalid value - Enter one of the
listed choices.
Explanation
This message is self explanatory.
ISPP425 Choice value error - The pull-down
choice value length is not between
1 and 8 characters.
Explanation
The selection for the pull-down choice was invalid. It
cannot be more than 8 digits.
User response
Enter a valid pull-down choice selection.
ISPP426 Choice value error - The pull-down
choice value must be numeric.
Explanation
A nonnumeric value was entered for the pull-down
choice. The value must be a number.
User response
Enter a valid pull-down choice selection.
ISPP427 Choice value error - The pull-
down choice value must equal an
available choice number.
Explanation
The only values that are acceptable for a pull-down
choice are those listed in the pull-down menu.
User response
Enter a valid pull-down choice selection.
ISPP428 Panel 'aaaaaaaa' error - Invalid
UNAVAIL variable name found on
PDC statement.
Explanation
The UNAVAIL variable name is not a valid dialog
variable name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt to
continue running the dialog, or you can choose to not
override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the dialog variable name for the UNAVAIL
keyword.
ISPP429 Panel 'aaaaaaaa' error - Invalid
'MNEM' keyword value - must be
between 1 and bbbbbbbb..
Explanation
The MNEM keyword value must be a position within
the action bar or pull-down choice text.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value for the mnemonic position in the
MNEM keyword.
ISPP430 Panel 'aaaaaaaa' error - Invalid
or missing parameter on TOG
statement.
Explanation
A syntax error was found on the TOG statement.
ISPF messages starting with ISP
218  z/OS: z/OS ISPF Messages and Codes

## Page 239

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement.
ISPP431 Panel 'aaaaaaaa' error - Too many
tokens given on TOG statement.
Explanation
Too many parameters were specified on the TOG
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement.
ISPP432 Panel 'aaaaaaaa' error - No
dialog variable specified on TOG
statement.
Explanation
The third parameter, the dialog variable, was omitted
from the TOG statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement.
ISPP433 Panel 'aaaaaaaa' error - No mode
specified on TOG statement.
Explanation
The first parameter, the mode, was omitted from the
TOG statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement. The mode, field name, and
dialog variable are all required parameters.
ISPP434 Panel 'aaaaaaaa' error - No field
specified on TOG statement.
Explanation
The second parameter, the field, was omitted from the
TOG statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement. The mode, field name, and
dialog variable are all required parameters.
ISPP435 Panel 'aaaaaaaa' error - Field
specified on TOG statement is not
a valid field.
Explanation
The field specified on the TOG statement does not
exist in the panel definition.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  219

## Page 240

mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement by specifying a field that
exists.
ISPP436 Panel 'aaaaaaaa' error - Mode
must be S or M.
Explanation
The only valid values for the mode are S (single) and M
(multiple).
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the TOG statement by specifying S or M for the
mode.
ISPP437 Panel 'aaaaaaaa' error - Multiple
choice TOG valid only in )PROC
section.
Explanation
The multiple choice (M) mode of the TOG statement is
only allowed in the )PROC section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Either use the S mode of the TOG command or move
the TOG statement to the )PROC section.
ISPP438 Panel 'aaaaaaaa' error - TOG
statement valid only in )PROC
or )ABCPROC section.
Explanation
The TOG statement was found in a section other than
the )PROC or )ABCPROC sections. It is only valid in
these two sections.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Move the TOG statement to the )PROC section or
an )ABCPROC section.
ISPP439 Panel 'aaaaaaaa' error - Invalid
value found on the PDSEP
keyword. Valid values are ON and
OFF.
Explanation
The PDSEP keyword on the PDC statement must be
followed with a valid value of ON or OFF.
Programmer response
Code a valid value on the PDSEP keyword.
ISPP440 Panel 'aaaaaaaa' error - Invalid
FIELD() value in )HELP section.
Explanation
The FIELD keyword requires the name of a field on the
panel. The field value must be a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
220  z/OS: z/OS ISPF Messages and Codes

## Page 241

Programmer response
Specify a literal that is the name of a field on the panel
within the parentheses of the FIELD keyword.
ISPP441 Panel 'aaaaaaaa' error - Invalid
PANEL() value in )HELP section.
Explanation
The PANEL keyword requires the name of a panel to be
displayed. The panel value must be a literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a literal that is the name of a panel within the
parentheses of the PANEL keyword.
ISPP442 Panel 'aaaaaaaa' error - Exceeded
maximum number of entries
in )HELP section.
Explanation
More than the maximum of 25 field level helps have
been coded.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Reduce the number of field level helps coded in
the )HELP section.
ISPP443 Missing field - Field level help
requested for missing field.
Explanation
This message is self explanatory.
ISPP444 Panel 'aaaaaaaa' error - Duplicate
field name found in )HELP section.
Explanation
A field was specified more than once in the )HELP
section. Only one field level help per field is allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Delete the duplicate FIELD keywords.
ISPP445 Panel 'aaaaaaaa' error - Missing
FIELD() keyword in )HELP section.
Explanation
The FIELD keyword is required for field level help
statements.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add the FIELD keyword along with the field name to
the field level help statement.
ISPP446 Panel 'aaaaaaaa' error - Missing
PANEL() keyword in )HELP section.
Explanation
The PANEL keyword, required on the field level Help
statement, was not found.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  221

## Page 242

to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add the PANEL keyword with the name of the panel to
be displayed on the field level Help statement.
ISPP447 Panel 'aaaaaaaa' error - Invalid
keyword found in )HELP section.
Explanation
The only valid keywords on a field level Help statement
are FIELD and PANEL or MSG or PASSTHRU. PANEL,
MSG, and PASSTHRU are mutually exclusive.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the field level Help statement.
ISPP448 Panel 'aaaaaaaa' error - Invalid
EXITADDR() value in )HELP
section.
Explanation
This message is self explanatory.
ISPP449 Panel 'aaaaaaaa' error -
Invalid EXITDATA() value in )HELP
section.
Explanation
This message is self explanatory.
ISPP450 Panel 'aaaaaaaa' error - )ABCINIT
section of panel not found when
expected.
Explanation
The )ABCINIT section should appear after the )ABC
section and before the )ABCPROC section. It was out
of order.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Move the )ABCINIT section to the correct location.
ISPP451 Panel 'aaaaaaaa' error
- )ABCPROC section of panel not
found when expected.
Explanation
The )ABCPROC section should appear after
the )ABCINIT section and before a new the )ABC
section. It was out of order.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Move the )ABCPROC section to the correct location.
ISPP452 Panel 'aaaaaaaa' error - Missing
descriptive text keyword on PDC
statement.
Explanation
The DESC keyword is required for the PDC statement.
It was not found.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
222  z/OS: z/OS ISPF Messages and Codes

## Page 243

Programmer response
Add a DESC keyword and text to the PDC statement.
ISPP453 Panel 'aaaaaaaa' error - Missing
RUN() keyword on ACTION
statement.
Explanation
The RUN keyword is required on the ACTION
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add the RUN keyword to the ACTION statement.
ISPP454 Panel 'aaaaaaaa' error - Duplicate
action bar field text found
in )BODY section.
Explanation
The text for the action bar coded in the )BODY section
is repeated. Each action bar coded in the )BODY
section must be unique for the panel.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Remove the duplicate action bar coded in the )BODY
section, or correct the text to match an action bar
choice.
ISPP455 Panel 'aaaaaaaa' error - Invalid
accelerator key, bbbbbbbb, found
in the ACC value. Valid values are
Ctrl, Shift, Alt, Backspace, Insert,
Delete, F1-F12, A-Z, a-z, and 0-9.
Explanation
The text for an accelerator key with the ACC text
value string is invalid. Valid keys are Ctrl, Shift, Alt,
Backspace, Insert, Delete, F1 - F12, A-Z, a-z, and 0-9.
Key combinations must be separated by a plus sign
with no embedded blanks.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a valid accelerator key within the ACC text
string.
ISPP456 Panel 'aaaaaaaa' error - Invalid
single accelerator key, bbbbbbbb,
found in the ACC value. This key
must be used in combination with
some other valid key(s).
Explanation
These single keys cannot be assigned as an
accelerator key: Ctrl, Shift, Alt, A-Z, a-z, and 0-9.
These must be used in combination with some other
key(s).
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Add an additional accelerator key(s) within the ACC
text string.
ISPP457 Panel 'aaaaaaaa' error - Invalid
key combination found in the ACC
value. A single character cannot
be used in combination with the
"SHIFT" key.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  223

## Page 244

Explanation
A single character 'A-Z', 'a-z', or '0-9' cannot be used in
combination with the "SHIFT" key.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the key combination within the ACC text string.
ISPP458 Panel 'aaaaaaaa' error - Invalid
key combination found in the ACC
value. If two keys are specified,
one key must be Ctrl, Alt, or Shift,
and the other must be Insert,
Delete, Backspace, F1-F12, A-Z,
a-z, or 0-9. If three keys are
specified, two keys must be Ctrl,
Alt, or Shift, and the other must
be Insert, Delete, Backspace, F1-
F12, A-Z, a-z, or 0-9. No two keys
within the ACC text string can
contain the same value.
Explanation
An invalid key combination has been found in the ACC
value text string. If two keys are specified, one key
must be Ctrl, Alt, or Shift, and the other must be
Insert, Delete, Backspace, F1-F12, A-Z, a-z, or 0-9. If
three keys are specified, two keys must be Ctrl, Alt, or
Shift, and the other must be Insert, Delete, Backspace,
F1-F12, A-Z, a-z, or 0-9. No two keys within the ACC
text string can contain the same value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the key combination within the ACC text string.
ISPP459 Panel 'aaaaaaaa' error - Invalid
key combination found in the
ACC value. A single number '0-9'
cannot be used in any combination
with the "SHIFT" key.
Explanation
A single number '0-9' cannot be used in any
combination with the "SHIFT" key.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the key combination within the ACC text string.
ISPP460 Panel 'aaaaaaaa' error - Two
consecutive field names without
panel name in )HELP section.
Explanation
The FIELD keyword appeared twice without an
intervening PANEL keyword being specified. This is not
allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a PANEL keyword between the two FIELD
keywords, or remove one of the FIELD keywords.
ISPP461 Panel 'aaaaaaaa' error - Two
consecutive panel names without
field name in )HELP section.
Explanation
The PANEL keyword appeared twice without an
intervening FIELD keyword specified. This is not
allowed.
ISPF messages starting with ISP
224  z/OS: z/OS ISPF Messages and Codes

## Page 245

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a FIELD keyword between the two PANEL
keywords, or remove one of the PANEL keywords.
ISPP462 Panel 'aaaaaaaa' error - Two
consecutive MSG keywords
without a FIELD keyword in
the )HELP section.
Explanation
The MSG keyword appeared twice without an
intervening FIELD keyword specified. This is not
allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a FIELD keyword between the two MSG
keywords, or remove one of the MSG keywords.
ISPP463 Panel 'aaaaaaaa' error - Two
consecutive PASSTHRU keywords
without a field name in the )HELP
section.
Explanation
The PASSTHRU keyword appeared twice without an
intervening FIELD keyword specified. This is not
allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a FIELD keyword between the two PASSTHRU
keywords, or remove one of the PASSTHRU keywords.
ISPP464 Panel 'aaaaaaaa' error - A
group box attribute character
cannot be overridden with the
bbbbbbbb;ATTRCHAR statement.
Explanation
The width and depth of a group box attribute cannot
be overridden with the bbbbbbbb;ATTRCHAR control
variable. To override the width and depth of a group
box, use the bbbbbbbb;ATTR(field), where "field" is the
dialog variable name for the group box as specified in
the )BODY section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Check the )INIT, )PROC, and )REINIT sections for
invalid use of an bbbbbbbb;ATTRCHAR control
statement which overrides a group box type attribute.
ISPP465 Panel 'aaaaaaaa' error - An invalid
keyword following the application
id was found on the KEYLIST
keyword of the )PANEL statement.
The only valid keyword is SHARED.
Explanation
The only valid parameter specified after the
application id on the KEYLIST keyword on the )PANEL
statement is SHARED. Anything other than SHARED
will result in this error.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  225

## Page 246

will end and you will be returned to the initial panel for
the application.
Programmer response
Either specify the SHARED keyword or do not.
ISPP466 Panel 'aaaaaaaa' error - A
LISTBOX, DDLIST, or COMBO name
is invalid. A LISTBOX, DDLIST,
or COMBO name is composed
of 1-8 characters. Alphanumeric
characters A-Z, 0-9, #, $, OR @
can be used in the name, but the
first character cannot be numeric.
Explanation
The LISTBOX, DDLIST or COMBO name either exceeds
the 8 character limit, contains an invalid character, or
starts with a numeric character.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Verify your LISTBOX, DDLIST, or COMBO names are
composed of 1-8 characters. Alphanumeric characters
A-Z, 0-9, #, $, or @ can be used in the name, but the
first character cannot be numeric.
ISPP467 Panel 'aaaaaaaa' error - The name
value for the LISTBOX, DDLIST, or
COMBO attribute section keyword
is valid only on input type fields.
Explanation
The name value for LISTBOX, DDLIST or COMBO
keyword specified in the )ATTR section has been
coded on an output or text attribute type. The name
value for the LISTBOC, DDLIST or COMBO keyword is
valid only on input type fields.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Check the panel attribute section for LISTBOX(name),
DDLIST(name) or COMBO(name) specified on a text or
output attribute type. LISTBOX(name), DDLIST(name),
or COMBO(name) is not valid on text or output
attribute types. The valid keyword values for LISTBOX
or DDLIST on text or output attribute types are
LISTBOX(ON/OFF) or DDLIST(ON/OFF).
ISPP468 Panel 'aaaaaaaa' error - The
WIDTH and DEPTH keywords only
allow numbers in the range 0-99.
Explanation
The WIDTH and DEPTH keywords for group boxes, list
boxes and drop-down lists only allow numbers in the
range 0-99, a WIDTH or DEPTH value was used that
was not in the range 0-99.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Verify the WIDTH or DEPTH value on any group boxes,
list boxes, or drop down lists are within the valid range
from 0-99. If a WIDTH or DEPTH value is out of this
range, correct it.
ISPP470 Panel 'aaaaaaaa' error - )CCSID
value must be 5 numeric digits.
Explanation
The number specified on the NUMBER keyword on
the )CCSID section statement must be five numeric
digits, even if they are preceding zeros.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
226  z/OS: z/OS ISPF Messages and Codes

## Page 247

Programmer response
Correct the )CCSID statement.
ISPP471 Panel 'aaaaaaaa' error - Illegal
token in )CCSID section.
Explanation
The only valid keyword on the )CCSID statement is
NUMBER. A keyword other than NUMBER was found.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the )CCSID statement.
ISPP472 Panel 'aaaaaaaa' error - NUMBER
keyword does not exist in
the )CCSID section.
Explanation
The NUMBER keyword is required on the )CCSID
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the )CCSID statement.
ISPP473 Variable not found - Variable
specified for VEDIT not found on
panel.
Explanation
A variable that exists on the panel must be specified
on the VEDIT statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the VEDIT statement by adding a variable
name that exists on the panel.
ISPP474 Panel 'aaaaaaaa' error - The
name parameter is missing from
the )LIST statement.
Explanation
The name parameter is required on the )LIST
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The name parameter on the )LIST statement
is required and should match the appropriate
DDLIST(name), LISTBOX(name) or COMBO(name).
ISPP480 Panel 'aaaaaaaa' error -
A ')PNTS' section statement
contains two FIELD() keywords or
the statement's VAR() or VAL()
keyword is missing.
Explanation
The ')PNTS' statement contains invalid keywords.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  227

## Page 248

Programmer response
Correct the ')PNTS' section statement, ensuring that
the statement contains only one of each keyword
FIELD(), VAR() and VAL(). The keywords must be
in this order: FIELD(field-name) VAR(variable-name)
VAL(value).
ISPP481 Panel 'aaaaaaaa' error - A ')PNTS'
section statement contains two
VAR() keywords or the statement's
VAL() keyword is missing.
Explanation
The ')PNTS' statement contains an invalid number of
VAR() keywords or the VAL() keyword is missing.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' section statement, ensuring that
the statement contains only one of each keyword
FIELD(), VAR() and VAL(). The keywords must be
in this order: FIELD(field-name) VAR(variable-name)
VAL(value).
ISPP482 Panel 'aaaaaaaa' error - Invalid
VAL(value) found in the ')PNTS'
section statement.
Explanation
The VAL() keyword contains an invalid value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the VAL() keyword for
the ')PNTS' section statement. Check for the correct
value and the correct spelling.
ISPP483 Panel 'aaaaaaaa' error - Invalid
FIELD() value found in the ')PNTS'
section statement.
Explanation
The FIELD() keyword contains an invalid value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the FIELD() keyword for
the ')PNTS' section statement. Check for the correct
value and the correct spelling.
ISPP484 Panel 'aaaaaaaa' error - Duplicate
FIELD(name) found in 'PNTS'
section.
Explanation
The ')PNTS' statement contains a valid FIELD(), VAR()
and VAL() keyword followed by a duplication of the
FIELD() keyword.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' statement, ensuring that the
statement contains only one of each keyword FIELD(),
VAR() and VAL() per statement.
ISPP485 Panel 'aaaaaaaa' error - Invalid
VAR() value in ')PNTS' section.
Explanation
The value for the VAR() keyword is invalid.
ISPF messages starting with ISP
228  z/OS: z/OS ISPF Messages and Codes

## Page 249

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The value for the VAR() keyword is incorrect. Ensure
that the variable name conforms to ISPF variable
naming conventions.
ISPP486 Panel 'aaaaaaaa' error - Invalid
VAL() value in ')PNTS' section.
Explanation
The value for the VAL() keyword is invalid.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The value for the VAL() keyword is incorrect. The value
must be a dialog variable or a literal.
ISPP487 Panel 'aaaaaaaa' error - Invalid
keyword found in ')PNTS' section.
Explanation
One of the keywords used in the ')PNTS' statement is
incorrect.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' statement, ensuring that the
statement contains only one of each keyword
FIELD(), VAR(), VAL(), BITMAP(), BITMAPD(),
TEXT(), PLACE(), DEPTH() per statement. The
keywords must be specified in this order:
FIELD(field-name) VAR(variable-name) VAL(value)
DEPTH(n) BITMAP(bitmap-name) BITMAPD(bitmap-
name) TEXT(text) PLACE(n).
ISPP488 Panel 'aaaaaaaa' error - No
FIELD() field-name found in
')PNTS' section.
Explanation
A )PNTS section entry is missing a FIELD() field-name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' statement, ensuring that the
statement contains only one of each keyword FIELD(),
VAR() and VAL() per statement. The keywords
must be specified in this order: FIELD(field-name)
VAR(variable-name) VAL(value). Verify that the FIELD
keyword has a field-name specified.
ISPP489 Panel 'aaaaaaaa' error - A ')PNTS'
section statement contains two
VAL() keywords.
Explanation
The point-and-shoot section of a panel definition can
only have one VAL keyword.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  229

## Page 250

Programmer response
Correct the ')PNTS' statement, ensuring that the
statement contains only one of each keyword FIELD(),
VAR() and VAL() per statement. The keywords
must be specified in this order: FIELD(field-name)
VAR(variable-name) VAL(value).
ISPP490 Panel 'aaaaaaaa' error - No VAR()
variable-name found in ')PNTS'
section.
Explanation
The VAR keyword variable-name was not found or was
blank.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' section entry to contain each of
these keywords in the order given here: FIELD(field-
name) VAR(variable-name) VAL(value). Ensure that the
variable-name is supplied.
ISPP491 Panel 'aaaaaaaa' error - Keywords
in ')PNTS' section missing or out of
order.
Explanation
The keywords in the ')PNTS' section must be in
this order: FIELD(field-name) VAR(variable-name)
VAL(value).
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' section entry to contain each of
these keywords in the order given here: FIELD(field-
name) VAR(variable-name) VAL(value).
ISPP492 Panel 'aaaaaaaa' error - Invalid
VAL() value found on ')PNTS'
section entry.
Explanation
The length of the VAL(value) found in the )PNTS
section is too large. If the value is a literal, the length
must be less than 255.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' section entry to contain the
acceptable length for the value of the VAL keyword.
ISPP493 Panel 'aaaaaaaa' error - Keywords
in ')LIST' section missing or out of
order.
Explanation
For drop-down lists and list boxes the keywords in
the ')LIST' section must be in this order: VAL(value)
CHOICE(choice). For combination boxes only the
CHOICE(choice) keyword is used.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
If working with list boxes or drop-down lists,
correct the ')LIST' section entry to contain each of
these keywords in the order given here: VAL(value)
CHOICE(choice). If working with combination boxes,
correct the ')LIST' section entry to contain this
keyword: CHOICE(choice).
ISPP494 Panel 'aaaaaaaa' error - Invalid
CHOICE() value found in the ')LIST'
section statement.
ISPF messages starting with ISP
230  z/OS: z/OS ISPF Messages and Codes

## Page 251

Explanation
The CHOICE() keyword contains an invalid choice
value. The choice value can be a dialog variable or a
literal. The error occurred while scanning the choice
value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the CHOICE() keyword
for the ')LIST' section statement. Check for the correct
value and the correct spelling. The choice value can be
a dialog variable or a literal.
ISPP495 Panel 'aaaaaaaa' error - The
list-name specified on this )LIST
section is a duplicate of
another )LIST section's list-name.
Each )LIST section must have a
unique list-name.
Explanation
The panel contains two )LIST sections which have
the same list-name. Each )LIST section specified must
have a unique list-name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Check the )LIST sections for duplicate list-names.
When the duplicate list-names are found, rename one
the list-names to a unique list-name.
ISPP496 Panel 'aaaaaaaa' error - A list-
name specified on this )LIST
section has no corresponding list-
name defined in the )ATTR section.
Explanation
The list-name on the )LIST section statement must
have a corresponding name (either DDLIST(name),
LISTBOX(name), or COMBO(name) defined.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Check the )LIST sections for list-names that have
no corresponding name defined with the LISTBOX,
DDLIST, or COMBO keywords. Verify the spelling of
the names on the keywords and on the )LIST section
heading.
ISPP497 Panel 'aaaaaaaa' error - The value
in the )PNTS section parameter
VAR(value) is a variable whose
length exceeds the limitation
of a leading ampersand plus 7
characters.
Explanation
The variable value for the )PNTS section VAR(value)
exceeds the ding limitation of a leading ampersand
plus 7 characters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
When the value of the )PNTS section parameter
VAR(value) is a g name variable it must consist of a
leading ampersand plus 7 characters. The g variable
specified exceeded the 7 character limit. Change the
variable name to a variable name containing a leading
ampersand and no more than 7 characters.
ISPP498 Panel 'aaaaaaaa' error - The value
in the )PNTS section parameter
VAL(value) is a variable whose
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  231

## Page 252

length exceeds the limitation
of a leading ampersand plus 7
characters.
Explanation
The variable value for the )PNTS section VAL(value)
exceeds the limitation of a leading ampersand plus 7
characters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
When the value of the )PNTS section parameter
VAL(value) is a g name variable it must consist of a
leading ampersand plus 7 characters. The g variable
specified exceeded the 7 character limit. Change the
variable name to a variable name containing a leading
ampersand and no more than 7 characters.
ISPP500 Exit address error - The panel help
exit address is zero.
Explanation
This message is self explanatory.
ISPP501 Exit variable error - The panel
help exit address variable is not
defined.
Explanation
This message is self explanatory.
ISPP502 Exit variable error - An error
occurred retrieving the panel help
exit address variable.
Explanation
This message is self explanatory.
ISPP503 Exit variable error - The panel help
exit data variable is not defined.
Explanation
This message is self explanatory.
ISPP504 Exit variable error - An error
occurred retrieving the panel help
exit data variable.
Explanation
This message is self explanatory.
ISPP505 Keys Help not available -
Application has not set system
variable ZKEYHELP.
Explanation
No help panel was defined for the current keylist.
Programmer response
Either identify a keys help panel in the keylist
definition or set variable ZKEYHELP to an appropriate
help panel.
ISPP506 Extended Help panel is not
defined for the application panel.
Explanation
EXHELP was requested, but no extended help panel
has been defined for the application panel displayed.
Programmer response
Define a help panel with the .help variable if a help
panel is desired.
ISPP507 Extended Help panel is currently
displayed.
Explanation
EXHELP was requested, but the user was already in
extended Help.
User response
Do not request EXHELP once you are in extended Help.
ISPP508 Keys Help panel is currently
displayed.
Explanation
KEYSHELP was requested while the KEYSHELP panel
was displayed.
User response
Do not issue the KEYSHELP command while in a Help
panel for the current keylist.
ISPF messages starting with ISP
232  z/OS: z/OS ISPF Messages and Codes

## Page 253

ISPP509 RP help not defined - )HELP
section definition for reference
phrase aaaaaaaa missing.
Explanation
No Help panel was defined for a reference phrase field.
Programmer response
Make sure an entry is coded in the )HELP section for
the reference phrase field in question.
ISPP510 Panel 'aaaaaaaa' error - The
DEPTH keyword in a )PNTS section
statement is missing or out-of-
order.
Explanation
The DEPTH keyword must be coded before the TEXT,
PLACE, IMAGE, and IMAGEP keywords and after the
FIELD VAR and VAL keywords in a )PNTS section
statement.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' section statement, ensuring that
the DEPTH keyword immediately follows the VAL()
keyword.
ISPP511 Panel 'aaaaaaaa' error - The
image-name specified with the
IMAGE or IMAGEP keyword
contains an invalid character, or is
greater than 8 characters.
Explanation
An image-name must be 1-8 characters. The first
character must be A-Z or a-z, and any remaining
characters can be A-Z, a-z, or 0-9.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a valid image-name for the IMAGE keyword.
An image-name must be 1-8 characters. The first
character must be A-Z or a-z, and any remaining
characters can be A-Z, a-z, or 0-9.
ISPP512 Panel 'aaaaaaaa' error -
Invalid )PNTS statement syntax.
The image-name parameter is
missing or defined as blank for the
IMAGEP keyword.
Explanation
The image-name for the IMAGEP keyword on the panel
statement was not specified, or is blank. An image-
name is required.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify an image-name for the IMAGEP keyword, or
remove the IMAGEP keyword.
ISPP513 Panel 'aaaaaaaa' error - Invalid
panel statement syntax. The
image-name is not defined
for the IMAGE keyword on
a )PANEL statement or on a )PNTS
statement.
Explanation
The image-name for the IMAGE keyword on a )PANEL
or )PNTS statement was not specified. An image-name
is required.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  233

## Page 254

will end and you will be returned to the initial panel for
the application.
Programmer response
Specify an image-name on the IMAGE keyword.
ISPP514 Panel 'aaaaaaaa' error -
Invalid )PNTS statement syntax.
The depth-value is incorrect on the
DEPTH keyword. Valid values are 0
- 62 or a dialog variable containing
one of these values.
Explanation
The value for the DEPTH() keyword is not a value from
0 - 62 or a dialog variable containing one of these
values.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The value for the DEPTH() keyword is incorrect. Supply
a correct numeric value or a dialog variable set to a
correct numeric value.
ISPP516 Panel 'aaaaaaaa' error -
Invalid )PNTS statement syntax.
The place-value is incorrect on
the PLACE keyword. Valid values
are: A (above) B (below), L (left),
R (right) or a dialog variable
containing one of these values.
Explanation
The value for the PLACE() keyword is incorrect.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The value for the PLACE() keyword is incorrect. Valid
values are: A (above), B (below) L (left) R (right) or a
dialog variable containing one of these values.
ISPP518 Panel 'aaaaaaaa' error - No text
found following the TEXT keyword.
Explanation
In the )PNTS section the TEXT keyword was used but
no text was supplied.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Supply the text required by the TEXT keyword. It must
match the text for that particular point and shoot text
field. Text of more than one word must be enclosed
within single quotes. Text containing variables must
allow room for expansion, if needed.
ISPP519 Panel 'aaaaaaaa' error - A ')PNTS'
section statement contains a
point-and-shoot text field with
only the DEPTH keyword specified.
If the DEPTH keyword is coded,
the TEXT keyword must also be
coded.
Explanation
If the DEPTH keyword is used on a point-and-shoot
text field, the TEXT keyword must also be coded.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')PNTS' statement, ensuring that the
statement contains both a DEPTH and a TEXT
ISPF messages starting with ISP
234  z/OS: z/OS ISPF Messages and Codes

## Page 255

keyword, or contains neither the DEPTH or the TEXT
keyword.
ISPP520 PICTCN syntax error - VER
statement syntax error on PICTCN
keyword parameter. The field-
mask parameter does not contain
the mask-character specified.
Explanation
The PICTCN keyword was specified on a VER
statement with a mask-character, but that mask-
character was not found in the field-mask parameter.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Change the field-mask parameter to contain the mask-
character specified in the VERify statement.
ISPP521 DSN syntax error - Verification
failure using keyword DSNAMEF.
Consecutive asterisks (*) are not
allowed.
Explanation
The DSNAMEF keyword verification failed because
consecutive asterisks (*) were found in the data set
name qualifier.
User response
Ensure each data set name qualifier contains no more
than one asterisk.
ISPP522 Name syntax error - Verification
failure using keyword NAMEF.
Consecutive asterisks (*) are not
allowed.
Explanation
The NAMEF keyword verification failed because
consecutive asterisks (*) were found in the member
name.
User response
Ensure that the member name contains only one
asterisk.
ISPP523 Panel 'aaaaaaaa' error - Invalid
value for a verify PICTCN string
found while processing panel.
Explanation
A PICTCN string in a VER statement in
the )INIT, )REINIT, or )PROC section of a panel
definition is incorrect.
The valid syntax is VER (xxx,PICTCN,mask-char,field-
mask,string) where xxx is a variable. The string
parameter can be composed of your defined constants
and any of these values:
C
Any character
A
Any alphabetic character (A-Z, a-z, #, @, $)
N
Any numeric character (0-9)
9
Any numeric character (same as "N")
X
Any hexadecimal character (0-9, A-F, a-f)
For example: VER(xxx,
PICTCN,'*','V**R**M**','VNNRNNMNN') The value of the
variable must start with the constant V followed by
2 numeric characters, the constant R followed by 2
numeric characters, the constant M followed by 2
numeric characters.
Programmer response
Correct the invalid value for the verify PICTCN string.
ISPP524 Panel 'aaaaaaaa' error - Invalid
value for a verify PICTCN string
found while processing panel.
Explanation
A PICTCN string in a VER statement in
the )INIT, )REINIT, or )PROC section of a panel
definition is incorrect.
The valid syntax is VER (xxx,PICTCN,mask-char,field-
mask,string) where xxx is a variable. The string
parameter can be composed of constants and any of
these values:
C
Any character
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  235

## Page 256

A
Any alphabetic character (A-Z, #, @, $)
N
Any numeric character (0-9)
9
Any numeric character (same as "N")
X
Any hexadecimal character (0-9, A-F)
For example: VER(xxx,
PICTCN,'*','V**R**M**','VNNRNNMNN') The value of the
variable must start with the constant V followed by
2 numeric characters, the constant R followed by 2
numeric characters, the constant M followed by 2
numeric characters.
Programmer response
Correct the invalid value for the verify PICTCN string.
ISPP525 PICTCN mask-char error - VER
statement syntax error on PICTCN
mask-char parameter. The mask-
character parameter cannot be
one of the picture string
characters (C,A,N,9,X, or c,a,n,x)
Explanation
The PICTCN mask-character used is invalid. It cannot
be one of these picture string characters (C,A,N,9,X, or
c,a,n,x).
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Change the mask-char parameter to contain a valid
mask-character for the VERify PICTCN statement.
ISPP526 VERify JSTD error - VER statement
failed for Julian standard date
(JSTD) when verifying the year.
The value for the year must be
numeric. The date is expressed
in a 4-digit year (YYYY) and a
3-digit day (DDD). The format is
YYYY.DDD. Valid values for YYYY
are 0000-9999. Valid values of
DDD are 001-365 or 001-366 if the
year is a leap year.
Explanation
One of the first four values entered was not a numeric.
The first four values of the date must be numeric. The
date is expressed in a 4-digit year (YYYY) and a 3-digit
day (DDD). The format is YYYY.DDD.
User response
Enter the Julian standard date in the correct
numeric format YYYY.DDD. Valid values for YYYY are
0000-9999. VALID values for DDD are 001-365 or
001-366 if the year is a leap year.
ISPP527 VERify JSTD error - VER statement
failed for Julian standard date
(JSTD) when verifying the day.
The value for the day must be
numeric. The date is expressed
in a 4-digit year (YYYY) and a
3-digit day (DDD). The format is
YYYY.DDD. Valid values for YYYY
are 0000-9999. Valid values of
DDD are 001-365 or 001-366 if the
year is a leap year.
Explanation
One of the values for day was not a numeric. The 3-
digit day must be numeric. The date is expressed in a
4-digit year (YYYY) and a 3-digit day (DDD). The format
is YYYY.DDD.
User response
Enter the Julian standard date in the correct
numeric format YYYY.DDD. Valid values for YYYY are
0000-9999. VALID values for DDD are 001-365 or
001-366 if the year is a leap year.
ISPP528 VERify JSTD error - VER statement
failed for Julian standard date
(JSTD) when verifying the day.
The day was outside of the range
001-365 or 001-366 for leap years.
Explanation
The day was not within the range 001-365 or 001-366
for leap years. Enter a day within the range and in the
format DDD.
User response
Enter the day within the range and in the format DDD.
ISPF messages starting with ISP
236  z/OS: z/OS ISPF Messages and Codes

## Page 257

ISPP529 VERify error - VER statement
failed for the national language
date delimiter character.
Explanation
The date delimiter character does not match the
national language date delimiter character. For the
U.S., the date delimiter is a slash (/).
User response
Enter the correct national language date delimiter
character.
ISPP529A VERify error - VER statement
failed for Julian date or Julian
standard date delimiter. A period
should be used as the delimiter.
For example: YY.DDD is the Julian
date format and YYYY.DDD is the
Julian standard date format.
Explanation
The date delimiter character is not a period (.). The
format for Julian date is YY.DDD. The format for Julian
standard date is YYYY.DDD.
User response
Use the period (.) as the date delimiter character in the
Julian date or Julian standard date.
ISPP530 Verify IDATE error - VER statement
failed for IDATE. The year, day,
or month is not numeric. The
date must be expressed as a
2-digit year (YY), 2-digit month
(MM) and 2-digit day (DD). The
year, month and day 2-digit values
are separated by the national
language date delimiter character.
Valid year values are 00-99, valid
month values are 01-12, and valid
day values are 01-31.
Explanation
The date was not numeric. Enter numeric data for the
2-digit year (00-99), the 2-digit month (01-12), and
the 2-digit day (01-31). The format for the U.S. is
YY/MM/DD.
User response
Enter numeric data for the 2-digit year (00-99), the
2-digit month (01-12), and the 2-digit day (01-31).
ISPP531 Verify IDATE error - VER statement
failed for IDATE. The 2-digit month
is not within the range 01-12.
The date must be expressed in
a 2-digit year (YY), 2-digit month
(MM) and 2-digit day (DD). The
year, month and day 2-digit values
are separated by a delimiting
character.
Explanation
The month was not within the range 01-12. Enter
numeric data for the 2-digit month within the range
01-12.
User response
Enter the month as a 2-digit month within the range
01-12.
ISPP532 Verify IDATE error - VER statement
failed for IDATE. The 2-digit year
is not within the range 00-99.
The date must be expressed in
a 2-digit year (YY), 2-digit month
(MM) and 2-digit day (DD). The
year, month and day 2-digit values
are separated by a delimiting
character.
Explanation
The year was not within the range 00-99. Enter
numeric data for the 2-digit year within the range
00-99. The date must be expressed in a 2-digit year
(YY), 2-digit month (MM) and 2-digit day (DD). The
year, month and day 2-digit values are separated by a
delimiting character.
User response
Enter the year as a 2-digit year within the range 00-99.
ISPP533 Verify IDATE error - VER statement
failed for IDATE. The 2-digit day
entered is not within the 01-31
range for the months of January,
March, May, July, August, October
or December, or the 2-digit day
entered is not within the 01-30
range for the months of April,
June, September or November.
Explanation
The day entered was not a valid day of the month. The
2 digit day must fall within the 01-31 range for the
months of January, March, May, July, August, October,
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  237

## Page 258

and December, or within the 01-30 range for the
months of April, June, September, and November.
User response
Enter the day as a 2-digit day within the ranges for the
appropriate months. Refer to the explanation section.
ISPP533A Verify IDATE error - VER statement
failed for IDATE. The 2-digit day
specified exceeds the days in the
month of February. February has a
range of 01-29 in leap years and a
range of 01-28 in non-leap years.
Explanation
The day specified exceeds the number of days in the
month of February. In non-leap years the range is
01-28 and in leap years the range is 01-29.
User response
Enter a valid 2-digit day for February. In non-leap
years the range is 01-28 and in leap years the range
is 01-29.
ISPP534 Verify JDATE error - VER
statement failed for Julian date
(JDATE) day. The day was outside
of the range 001-365 or 001-366
for leap years.
Explanation
The day was not within the range 001-365 or 001-366
for leap years. Enter a day within the range and in the
format DDD.
User response
Enter the day in the DDD format within the range
001-365 for non-leap years and 001-366 for leap
years.
ISPP535 Verify STDDATE error - VER
statement failed for standard
date (STDDATE). The 4-digit year
(YYYY) or 2-digit month (MM) or
2-digit day (DD) was not numeric.
Valid year values are 0000-9999,
valid month values are 01-12, and
valid day values are 01-31.
Explanation
The STDDATE format represents a date expressed in a
4-digit year (YYYY), a 2-digit month (MM), and a 2-digit
day (DD). For the U.S. the format is YYYY/MM/DD. Valid
year values are 0000-9999, valid month values are
01-12, and valid day values are 01-31.
User response
Enter the date as a 4-digit year, 2-digit month, and
2-digit day as numeric values. Valid year values are
0000-9999, valid month values are 01-12, and valid
day values are 01-31.
ISPP536 Verify IPADDR4 error - VER
statement failed for IPADDR4.
Two consecutive dot delimiters
were found in the IP version
4 address. The format for a
valid IP version 4 address
in dotted decimal notation is:
nnn.nnn.nnn.nnn. For example,
6.9.97.2 or 9.37.198.44 are in
valid IP version 4 address
notation.
Explanation
The invalid IP version 4 address contains consecutive
dot delimiters. The format for a valid IP version
4 address consists of a decimal value representing
each of the 4 bytes (octets) that make up the
address and is represented in the dotted decimal
notation: nnn.nnn.nnn.nnn For example, 6.9.97.2 or
9.37.198.44 are in valid IP version 4 address notation.
User response
Enter the IP version 4 address using the dotted
decimal notation.
ISPP537 Verify IPADDR4 error - VER
statement failed for IPADDR4. One
of the 4 bytes (octets) that make
up the IP version 4 address is
greater than 3 decimal digits. The
format is: nnn.nnn.nnn.nnn
Explanation
The invalid IP version 4 address contains a byte
(octet) consisting of more than 3 decimal digits.
The IP version 4 address is represented in dotted
decimal notation with a decimal value in each of the 4
bytes (octets) that make up the address. For example
9.69.97.2 or 9.37.198.44
User response
Enter the IP version 4 address using the dotted
decimal notation.
ISPF messages starting with ISP
238  z/OS: z/OS ISPF Messages and Codes

## Page 259

ISPP538 Verify IPADDR4 error - VER
statement failed for IPADDR4. The
IP version 4 address is not in the
correct dotted decimal notation.
Either 1) Too few or too many dot
delimiters were found or 2) The
address started or ended with a
dot delimiter. The correct format
is: nnn.nnn.nnn.nnn
Explanation
The invalid IP version 4 address has either too few
(less than 3) or too many (more than 3) dot delimiters
OR the address started or ended with a dot delimiter.
This is a valid example of dotted decimal notation for
an IP version 4 address: 9.67.198.44
User response
Enter the IP version 4 address using the dotted
decimal notation.
ISPP539 Verify IPADDR4 error - VER
statement failed for IPADDR4. The
first byte (octet) of the IP version
4 address is not in the range of
0-223 decimal.
Explanation
The first byte (octet) of the IP version 4 address
exceeds the decimal range of 0-223.
User response
Enter an IP version 4 address in which the first byte
(octet) falls within the 0-223 decimal range.
ISPP539A Verify IPADDR4 error - VER
statement failed for IPADDR4. The
second, third or fourth byte (octet)
of the IP version 4 address is not
in the range of 0-255 decimal.
Explanation
Either the second, third or fourth byte (octet) of the IP
version 4 address exceeds the decimal range of 0-255.
User response
Enter an IP version 4 address in which the second,
third and fourth bytes (octets) of the IP address fall
within the 0-255 decimal range.
ISPP539B Verify IPADDR4 error - VER
statement failed for IPADDR4. The
IP version 4 address contains an
invalid digit or delimiter. Valid
digits can range from 0-9, and a
dot (.) is the only valid delimiter.
The format is: nnn.nnn.nnn.nnn
Explanation
The IP version 4 address contains either an invalid
decimal digit or an invalid delimiter. Valid digits can
range from 0-9 and the delimiter must be a dot (.). The
format for the IP version 4 address in dotted decimal
notation is: nnn.nnn.nnn.nnn
User response
Enter a valid IP version 4 address in dotted decimal
notation with the digits ranging from 0-9 and the dot
delimiter. For example, 9.27.1.73
ISPP540 Verify STDDATE error - VER
statement failed for STDDATE. The
2-digit month was not within the
range of 01-12.
Explanation
The month entered was not a valid month. A valid
2-digit month falls in the range of 01-12.
User response
Enter a valid 2-digit month in the range of 01-12.
ISPP541 Verify STDDATE error - VER
statement failed for STDDATE.
The 2-digit day entered is not
within the 01-31 range for the
months of January, March, May,
July, August, October or December
OR the 2-digit day entered is not
within the 01-30 range for the
months of April, June, September
or November.
Explanation
The day entered was not a valid month. The 2 digit
day must fall within the 01-31 range for the months
of January, March, May, July, August, October, and
December, or within the 01-30 range for the months
of April, June, September, and November.
User response
Enter the day as a 2-digit day within the ranges for the
appropriate months. Refer to the explanation section.
ISPP542 Verify STDDATE error - VER
statement failed for STDDATE. The
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  239

## Page 260

2-digit day specified exceeds the
days in the month of February.
February has a range of 01-29 in
leap years and a range of 01-28 in
non-leap years.
Explanation
The day specified exceeds the number of days in the
month of February. In non-leap years the range is
01-28 and in leap years the range is 01-29.
User response
Enter a valid 2-digit day for February. In non-leap
years the range is 01-28 and in leap years the range
is 01-29.
ISPP543 Verify JDATE error - VER
statement failed for JDATE. The
year was not expressed as a
2-digit year. The date must be
expressed as 2-digit year in the
range of 00-99 and 3-digit day of
the year in the range of 001-365
or 001-366 for a leap year. The
format is YY.DDD with a period as
the delimiter.
Explanation
The format for JDATE is YY.DDD. Enter numeric data
for the 2-digit year within the range 00-99. The date
must be expressed as a 2-digit year (YY) and 3-digit
day of the year (DDD). The year and day of the year
values are separated by the delimiting character, a
period (.).
User response
Enter the year as a 2-digit year within the range 00-99
and a 3-digit day of the year within the range 001-365
or 001-366 for leap years.
ISPP544 Verify JDATE error - VER
statement failed for JDATE. The
day of the year was not expressed
as a 3-digit day of the year (DDD).
The date must be expressed as 2-
digit year in the range of 00-99
and 3-digit day of the year in the
range of 001-365 or 001-366 for
a leap year. The format is YY.DDD
with a period as the delimiter.
Explanation
The format for JDATE is YY.DDD. Enter numeric data
for the 3-digit day of the year within the range
001-365 or 001-366 for leap years. The date must be
expressed as a 2-digit year (YY) and 3-digit day of the
year (DDD). The year and day of the year values are
separated by the delimiting character, a period (.).
User response
Enter the year as a 2-digit year within the range 00-99
and a 3-digit day of the year within the range 001-365
or 001-366 for leap years.
ISPP545 Verify ITIME error - VER statement
failed for International time
(ITIME). The national language
time delimiter is invalid or position
3 does not contain the national
language time delimiter.
Explanation
The national language time delimiter is invalid for the
national language or is not in position 3.
User response
Enter the correct national language time delimiter in
the third position. For the U.S. the time delimiter is a
colon (:).
ISPP546 Verify ITIME error - VER statement
failed for International time
(ITIME). The hour or minute value
is not numeric. Valid values are
00-23 for hour and 00-59 for
minute.
Explanation
The hour or minute specified was not numeric. The
value range for hour is 00-23 and for minute is 00-59.
User response
Enter valid values for hour in the range of 00-23 and
for minute in the range of 00-59.
ISPP547 Verify ITIME error - VER statement
failed for international time
(ITIME). The hour or minute
entered is outside of the valid
range. The value range for hour is
00-23 and for minute is 00-59.
Explanation
The value entered for hour or minute falls outside of
the valid range for hour or minute. The value range for
hour is 00-23 and for minute is 00-59.
ISPF messages starting with ISP
240  z/OS: z/OS ISPF Messages and Codes

## Page 261

User response
Enter valid values for hour in the range of 00-23 and
for minute in the range of 00-59.
ISPP548 Verify STDTIME error - VER
statement failed for standard time
(STDTIME). The national language
time delimiter is invalid or position
3 and 6 do not contain the national
language time delimiter.
Explanation
The national language time delimiter is invalid for the
national language or is not in character position 3 and
position 6.
User response
Enter the correct national language time delimiter in
positions 3 and 6. For the U.S. the time delimiter is a
colon (:).
ISPP549 Verify STDTIME error - VER
statement failed for standard time
(STDTIME). The hour, minute, or
second value is not numeric.
Valid values are 00-23 for hour,
00-59 for minute, and 00-59 for
second. For the U.S., the format is
HH:MM:SS.
Explanation
The hour, minute, or second specified was not
numeric. Valid values for hour fall in the range of
00-23. Valid values for minute and second in the range
of 00-59.
User response
Enter valid values for hour in the range of 00-23 and
for minute and second in the range of 00-59.
ISPP549A Verify STDTIME error - VER
statement failed for standard time
(STDTIME). The hour, minute, or
second value is not within the
valid range. Valid ranges are 00-23
for hour and 00-59 for minute and
second. For the U.S., the format is
HH:MM:SS.
Explanation
The hour, minute, or second specified is outside of the
valid range. The valid ranges are 00-23 for hour, 00-59
for minute and second.
User response
Enter valid values for hour in the range of 00-23 and
for minute and second in the range of 00-59.
ISPP550 Panel 'aaaaaaaa' error - An invalid
keyword was found in the ')FIELD'
section.
Explanation
The ')FIELD' statement contains invalid keywords.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the ')FIELD' statement, ensuring that
the statement contains a FIELD keyword
and optionally any of these keywords:
LEN,LCOL,RCOL,IND,LIND,RIND and SCROLL
ISPP551 Panel 'aaaaaaaa' error - Invalid
FIELD() value found in the ')FIELD'
statement.
Explanation
The FIELD() keyword contains an invalid value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the FIELD() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP552 Panel 'aaaaaaaa' error - Invalid
LEN() value found in the ')FIELD'
statement.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  241

## Page 262

Explanation
The LEN() keyword contains an invalid value.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the LEN() to be a value
between 1 and 32767 or a valid field name.
ISPP553 Panel 'aaaaaaaa' error - Invalid
RCOL() value found in the ')FIELD'
statement.
Explanation
The RCOL() keyword contains an invalid value - it
should be a valid dialog variable name that is referred
to only once in the )FIELD section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the RCOL() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP554 Panel 'aaaaaaaa' error - Invalid
LCOL() value found in the ')FIELD'
statement.
Explanation
The LCOL() keyword contains an invalid value - it
should be a valid dialog variable name that is referred
to only as an LCOL value in the )FIELD section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the LCOL() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP555 Panel 'aaaaaaaa' error - Invalid
IND() value found in the ')FIELD'
statement.
Explanation
The IND() keyword contains an invalid value. Either
the first parameter specifies an invalid dialog variable
name or the second parameter does not specify 2 non-
blank bytes as a literal enclosed in quotes.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the IND() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP556 Panel 'aaaaaaaa' error - Invalid
keyword found in the ')FIELD'
statement.
Explanation
The ')FIELD' statement contains invalid keywords.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
242  z/OS: z/OS ISPF Messages and Codes

## Page 263

Programmer response
Correct the keyword for the ')FIELD' statement. Check
for the correct value and the correct spelling.
ISPP557 Panel 'aaaaaaaa' error - Invalid
LIND() value found in the ')FIELD'
statement.
Explanation
The LIND() keyword contains an invalid value. Either
the first parameter specifies an invalid dialog variable
name or the second parameter does not specify a non-
blank 1 byte literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the LIND() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP558 Panel 'aaaaaaaa' error - Invalid
RIND() value found in the ')FIELD'
statement.
Explanation
The RIND() keyword contains an invalid value. Either
the first parameter specifies an invalid dialog variable
name or the second parameter does not specify a non-
blank 1 byte literal.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the RIND() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP559 Panel 'aaaaaaaa' error - Invalid
SCROLL() value found in the
')FIELD' statement.
Explanation
The SCROLL() keyword contains an invalid value - it
should be either a valid variable name that is referred
to only on the SCROLL keyword in the )FIELD section
or the fixed values "ON" or "OFF".
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the SCROLL() keyword
for the ')FIELD' statement. Check for the correct value
and the correct spelling.
ISPP560 Panel 'aaaaaaaa' error - Invalid
field name specified on the
LENGTH() built-in function
Explanation
The LENGTH() keyword contains an invalid field name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the LENGTH() built-in
function.
ISPP561 Panel 'aaaaaaaa' error - Invalid
field name specified on the
UPPER() built-in function
Explanation
The UPPER() keyword contains an invalid field name.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  243

## Page 264

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the UPPER() built-in
function.
ISPP562 End of field - You have scrolled to
the end of the current field.
Explanation
You cannot scroll any further right because you are
positioned at the end of the field.
ISPP563 Start of field - You are positioned
at the start of the current field.
Explanation
You cannot scroll any further left because you are
positioned at the start of the field.
ISPP564 Input truncated - You have
typed data beyond the end of
the field or you have replaced
DBCS characters with single byte
character resulting in truncation of
input.
Explanation
When DBCS characters are displayed in a scrollable
field the displayed field may contain inserted shift-out
and shift-in characters. Hence you can enter more
single byte characters than the editable characters
displayed. The input will be truncated to the editable
portion of the variable and the truncated input will be
redisplayed ignoring any scroll commands.
Programmer response
None.
ISPP564A Input truncated - After converting
input data from UTF-8 to EBCDIC
the number of bytes to be stored
in a field exceeded the length
of the field. The input data has
been truncated at the length of the
field.
Explanation
UTF-8 characters may be converted to DBCS
characters when UTF-8 data is converted to EBCDIC.
So a single UTF-8 character entered in the browser
may occupy 2 bytes when converted to EBCDIC.
In addition the converted EBCDIC data will include
shift-out and shift-in characters to delimit DBCS sub-
strings. Hence data entered into a field in the browser
can result in more data than can be be accommodated
by the field after conversion from UTF-8 to EBCDIC.
When this occurs ISPF truncates the EBCDIC data at
the length of the input field.
Programmer response
None.
ISPP565 Invalid command - The command
you have entered is not valid for
the expand field function.
Explanation
An invalid command was specified to the expand field
function.
User response
Clear the command or enter a valid EXPAND window
primary command. See help.
ISPP566 Invalid parameter - ON/OFF are
the only valid parameters HEX
command parameters.
Explanation
An invalid parameter was specified on invocation of
the expand hex command.
User response
Change the command to specify the proper
parameters for the HEX command and reissue the
command.
Programmer response
Change the macro to specify the proper parameters for
the hex command.
ISPP567 Panel 'aaaaaaaa' error - Invalid
SIND() value found in the ')FIELD'
statement.
ISPF messages starting with ISP
244  z/OS: z/OS ISPF Messages and Codes

## Page 265

Explanation
The SIND() keyword contains an invalid value. Either
the first parameter specifies an invalid dialog variable
name or the second parameter does not specify 3 non-
blank bytes as a literal enclosed in quotes.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the SIND() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP568 Panel 'aaaaaaaa' error - Invalid
SCALE() value found in the ')FIELD'
statement.
Explanation
The SCALE() keyword contains an invalid value - it
should be a valid dialog variable name that is referred
to only once in the )FIELD section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the SCALE() keyword for
the ')FIELD' statement. Check for the correct value and
the correct spelling.
ISPP569 Panel 'aaaaaaaa' error - Invalid
field name specified on the VSYM()
built-in function
Explanation
The VSYM() keyword contains an invalid field name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the value contained in the VSYM() built-in
function.
ISPP600 No window to move - There is no
active pop-up window to move.
Explanation
The WINDOW command was entered, but no pop-up
window was displayed in the logical screen from which
the command was entered.
User response
Use the WINDOW command only when a pop-up
window is displayed.
ISPP601 Cannot move window - Pop-up
window cannot be moved while
pull-down is displayed.
Explanation
The WINDOW command was entered while a pull-
down menu was displayed. ISPF does not allow a pop-
up window to be repositioned while a pull-down menu
is displayed.
User response
Close the pull-down window and then reposition the
window.
ISPP606 Enter an application name at the
cursor position. Any application
name and parameters that are
valid for the ISPF START command
are valid in this field. If additional
space is needed, press the Expand
PF key and a pop-up window will
be displayed containing a longer
input field.
Explanation
The user has selected the 'Start a new application'
option, but the Application Name input field is blank.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  245

## Page 266

User response
Enter an application name at the cursor position.
ISPP607 The Expand PF key is not active
unless the cursor is in the
Application Name input field.
Explanation
The user has pressed the Expand PF key, but the
cursor must first be in the Application Name input
field.
User response
Put the cursor in the Application Name input field and
then press the Expand PF key to display a pop-up
window containing a longer input field.
ISPP608 No more screens - There are no
more screens available. You have
the maximum number of available
screens.
Explanation
There is a maximum of 32 logical screens in ISPF,
or less if your installation has specified less. You
must close a screen before attempting another SPLIT
command.
User response
You must close a screen first and then issue the SPLIT
command.
ISPP609 Not enough storage - There is
not enough storage to safely
start another screen. The SPLIT
command is nullified.
Explanation
ISPF requires more storage than is available in your
region to perform the SPLIT request. Increasing your
user region will provide additional storage.
User response
Log on to TSO with a larger region size.
ISPP610 Panel 'aaaaaaaa' error - Width of
line in the )AREA section is wider
than allowed in the )BODY section
definition.
Explanation
The line shown in the error box is longer than the line
in the )BODY section that marks the scrollable area.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Either shorten the line in the )AREA section, or
increase the width of the definition of the scrollable
area in the panel definition.
ISPP611 Panel 'aaaaaaaa' error - A
scrollable area cannot be defined
within an )AREA section.
Explanation
An attempt was made to define a scrollable area within
a scrollable area. This is not allowed.
User response
There is a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A scrollable area cannot have another scrollable area
defined within it. You may move all but one of the
scrollable areas to the )BODY section, but be aware
that you can only define one area in the )BODY section
with EXTEND(ON). This includes scrollable areas,
dynamic areas, and graphic areas.
ISPP612 Panel 'aaaaaaaa' error - A graphic
area cannot be defined within
an )AREA section.
Explanation
An attempt was made to define a graphic area within a
scrollable area. This is not allowed.
ISPF messages starting with ISP
246  z/OS: z/OS ISPF Messages and Codes

## Page 267

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A scrollable area cannot have a graphic area defined
within it. Move either the scrollable areas or the
graphic area to the )BODY section.
ISPP613 Panel 'aaaaaaaa' error - An action
bar cannot be defined within
an )AREA section.
Explanation
An attempt was made to define an action bar within a
scrollable area. This is not allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A scrollable area cannot have an action bar defined
within it. Move the action bar to the )BODY section.
ISPP614 Panel 'aaaaaaaa' error - An
area defined with SCROLL(ON) or
EXTEND(ON) cannot be defined
within an )AREA section.
Explanation
An attempt was made to define an area with
SCROLL(ON) or EXTEND(ON) within a scrollable area.
This is not allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A panel with a scrollable area cannot have another
area within it that has EXTEND(ON) or SCROLL(ON)
specified. Change the EXTEND or SCROLL value to OFF.
ISPP615 Panel 'aaaaaaaa' error - )AREA
section defined out of
order, It must appear
after the )BODY section and
before the )INIT, )REINIT, )PROC,
or )HELP sections if coded.
Explanation
An attempt was made to define a scrollable area, but
the )AREA section was specified out of order in the
panel definition.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Move the )AREA section after the )BODY section and
before the )INIT, )REINIT, )PROC, or )HELP sections if
they are coded in the panel definition.
ISPP616 Panel 'aaaaaaaa' error - Invalid
keyword for the )AREA section.
Explanation
This message is self explanatory.
ISPP617 Panel 'aaaaaaaa' error - )AREA
section is not defined for an area
section defined in the )BODY or an
empty )AREA section was defined.
Explanation
The )BODY section contains a field with an attribute
that indicates the field is scrollable, but no )AREA
section exists or it has no lines defined in it.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  247

## Page 268

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You must code an )AREA section for the field in
the )BODY section that has the attribute AREA(SCRL).
That area must contain at least one line.
ISPP618 Panel 'aaaaaaaa' error - An )AREA
section is defined without a
section defined in the panel )BODY
section.
Explanation
The )BODY section contains a field with an attribute
that indicates the field is scrollable, but no )AREA
section exists with that field's name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You must code an )AREA section for the field in
the )BODY section that has the attribute AREA(SCRL).
ISPP619 Panel 'aaaaaaaa' error - Invalid
keyword for AREA of type SCRL.
Explanation
This message is self explanatory.
ISPP620 Panel 'aaaaaaaa' error - )MODEL
cannot be defined in an )AREA
that has other information already
defined.
Explanation
A )MODEL statement cannot be specified in a
scrollable area.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A )MODEL section cannot be defined in an )AREA
section. You could make the )MODEL section part of
the )BODY section or use uniquely named variables
instead of table variables and put them in the
scrollable area.
ISPP621 Panel 'aaaaaaaa' error - Area
name must be 1 to 8 characters in
length.
Explanation
The name specified on the )AREA section panel
statement is too long.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The name on the )AREA section must exist and must
be 8 characters or fewer in length.
ISPP622 Panel 'aaaaaaaa' error - Scrollable
area name must be specified
correctly
Explanation
This message is self explanatory.
ISPP623 Panel 'aaaaaaaa' error - The depth
must be specified as a number
Explanation
The depth specified on the )AREA section panel
statement contains a nonnumeric character.
ISPF messages starting with ISP
248  z/OS: z/OS ISPF Messages and Codes

## Page 269

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
The depth of the scrollable area specified on the
DEPTH() parameter on the )AREA section panel
statement must be numeric.
ISPP624 Panel 'aaaaaaaa' error - )MODEL
not allowed in )AREA section
Explanation
A )MODEL statement cannot be specified in a
scrollable area.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A )MODEL section cannot be defined in an )AREA
section. Make the )MODEL section part of the )BODY
section or use uniquely named variables instead of
table variables and put them in the scrollable area.
ISPP625 Panel 'aaaaaaaa' error - Scrollable
area must be defined at least
20 wide in the )BODY section
definition.
Explanation
The scrollable area as defined is too narrow.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
A scrollable area must be at least 20 characters
wide, defined by the starting and ending scrollable
attribute characters. The width includes the attribute
characters.
ISPP626 Panel 'aaaaaaaa' error - Depth
defined on the area section
must be less than or equal to
the number of lines defined in
the )AREA section.
Explanation
For a scrollable area with EXTEND(ON) specified
on the scrollable attribute, the depth specified on
the DEPTH() parameter of the )AREA section panel
statement must be less than or equal to the number
of lines defined in the )AREA section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the specification in the DEPTH() parameter or
change the number of lines in the )AREA section.
ISPP627 Panel 'aaaaaaaa' error - Depth
defined on the area section must
be equal to or greater than
the number of lines defined for
the scrollable area in the )BODY
section.
Explanation
The depth specified on the DEPTH() parameter must
be equal to or greater than the number of lines defined
for the scrollable area in the )BODY section.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  249

## Page 270

Programmer response
You must either change the number of lines defined
in the panel section or change the depth specified
in the DEPTH() parameter on the )AREA section
panel statement. This number specifies the minimum
number of lines in the scrollable area (not including
the scroll indicator). It must be greater than or equal to
the number of lines defined for the area in the )BODY
section and less than or equal to the number of lines in
the )AREA section.
ISPP628 Panel 'aaaaaaaa' error - Variable
cannot be used for depth
specification.
Explanation
A variable was specified on the DEPTH() parameter
of the )AREA section panel statement. This is not
allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You must code a number for the depth of the scrollable
area. This number specifies the minimum number of
lines in the scrollable area (not including the scroll
indicator). It must be greater than or equal to the
number of lines defined for the area in the )BODY
section and less than or equal to the number of lines in
the )AREA section.
ISPP629 Panel 'aaaaaaaa' error -
EXTEND(OFF) scrollable area must
be a depth of at least 2 lines in
the )BODY section definition.
Explanation
When EXTEND(OFF) is specified for a scrollable area,
the scrollable area defined in the )BODY section must
have at least 2 lines. The scroll indicator takes one line
and you must have at least one line of data to scroll.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You must add a line to the scrollable area defined
in the )BODY section or change the attribute of the
scrollable area to EXTEND(ON).
ISPP650 Panel 'aaaaaaaa' error - Depth
specified will not fit on the screen.
Explanation
The depth specified on the DEPTH() parameter, added
to the depth of the rest of the panel, is too large to fit
on the display.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Try changing to a display device that has more lines
per screen.
Programmer response
Either reduce the depth specified on the DEPTH()
parameter of the )AREA section panel statement
or change the number of non-scrollable area lines
defined in the )BODY section.
ISPP651 The cursor must be within a
scrollable area to perform the
requested scroll.
Explanation
The cursor was not in a scrollable area when a
command to scroll the screen was received.
User response
Place the cursor in the scrollable area you wish to
scroll before pressing the scroll function key.
ISPP652 End of data - There is no additional
information below this line.
ISPF messages starting with ISP
250  z/OS: z/OS ISPF Messages and Codes

## Page 271

Explanation
This is an informational message. You have scrolled to
the end of the scrollable information.
ISPP653 Top of data - There is no additional
information above this line.
Explanation
This is an informational message. You have scrolled to
the beginning of the scrollable information.
ISPP654 The scrollable area must be visible
to perform the requested scroll.
Explanation
There is a scrollable area defined on the panel, but it
does not fit on the display.
User response
There was a programming error defining the panel that
you attempted to display. You might be able to make
the scrollable area visible by using a display device
with more lines per screen.
Programmer response
Too many lines have been defined outside the
scrollable area so that the scrollable area does not
appear on the panel. Reduce the number of non-
scrollable lines in the )BODY section or move some of
them to the scrollable area.
ISPP655 End of data - There is no additional
information below this line. Press
Enter or RIGHT again to display
the next panel, if one has been
defined.
Explanation
This is an informational message. You have scrolled to
the end of the scrollable information.
ISPP656 Top of data - There is no additional
information above this line. Press
LEFT again to display the previous
panel, if one has been defined.
Explanation
This is an informational message. You have scrolled to
the beginning of the scrollable information.
ISPP657 Panel 'aaaaaaaa' error - A graphic
area cannot be defined within a
group box.
Explanation
This is an informational message. Graphic areas are
not supported within group boxes.
Programmer response
There are three options: remove the group box
definition from the panel, remove the graphic area
from the group box, code the graphic area outside of
the group box.
ISPP658 Invalid screen name - A screen
name must be more than one
character, and all alphanumerics,
and cannot be LIST, PREV or NEXT.
Explanation
The screen name entered was invalid.
User response
Specify a valid screen name.
ISPP659 Any application name and
parameters that are valid for the
ISPF START command are valid in
this field. If additional space is
needed, press the Expand PF key
while the cursor is in this field and
a pop-up window will be displayed
containing a longer input field.
Explanation
This message is field level help text for the Application
Name input field.
ISPP700 Panel 'aaaaaaaa' error - The exit
name on the )INEXIT statement is
invalid. An exit name is composed
of 1-8 characters. Alphanumeric
characters A-Z, 0-9, #, $, OR @
can be used in the name, but the
first character cannot be numeric.
Explanation
The exit name on the )INEXIT statement either
exceeds the 8 character limit, contains an invalid
character, or starts with a numeric character.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  251

## Page 272

to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Verify your INEXIT name is composed of 1-8
characters. Alphanumeric characters A-Z, 0-9, #, $,
or @ can be used in the name, but the first character
cannot be numeric.
ISPP701 Panel 'aaaaaaaa' error - Illegal
token in )INEXIT section.
Explanation
An invalid keyword was found on the )INEXIT
statement. The only valid keywords on the )INEXIT
statement are LOAD, PGM, CACHE, and the exit name.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Correct the )INEXIT statement.
ISPP702 Panel 'aaaaaaaa' error - Panel
input exit type is invalid - must be
PGM or LOAD.
Explanation
The exit type specified on the )INEXIT statement must
be PGM or LOAD.
User response
Use the literal PGM or LOAD as the panel input exit
type.
Programmer response
Correct the )INEXIT statement.
ISPP703 Panel 'aaaaaaaa' error - The exit
name parameter is missing from
the )INEXIT statement.
Explanation
The exit name parameter is required on the )INEXIT
statement when an exit type of LOAD is specified.
User response
Provide the name of the panel input exit when an exit
type of LOAD is specified on the )INEXIT statement.
Programmer response
Correct the )INEXIT statement.
ISPP704 Panel 'aaaaaaaa' error - Load of
panel input exit routine failed.
Explanation
The exit name parameter is required on the )INEXIT
statement.
User response
Ensure the name of the panel input exit on the )INEXIT
statement is correct.
It is possible that you do not have all of
the load module data sets allocated that the
application expects. In this case, contact your system
administrator.
Programmer response
If necessary, correct the )INEXIT statement or ensure
all required data sets are allocated.
ISPP705 Panel 'aaaaaaaa' error - The
exit address variable parameter
is missing from the )INEXIT
statement.
Explanation
The exit address variable parameter is required on
the )INEXIT statement when an exit type of PGM is
specified.
User response
Provide the name of a variable containing the address
of the exit when an exit type of PGM is specified on
the )INEXIT statement.
Programmer response
Correct the )INEXIT statement.
ISPP706 Panel 'aaaaaaaa' error - The
name of the exit address
ISPF messages starting with ISP
252  z/OS: z/OS ISPF Messages and Codes

## Page 273

variable specified on the )INEXIT
statement is invalid.
Explanation
The dialog variable name specified on the )INEXIT
section statement is invalid. Either it is too long or
contains invalid characters.
Programmer response
Ensure that the name of the dialog variable containing
the address of the exit is specified on the )INEXIT
statement with valid characters and does not exceed
the maximum length allowed.
ISPP707 Panel 'aaaaaaaa' error - The input
exit address variable contains an
invalid address value.
Explanation
The dialog variable specified on the )INEXIT section
statement contains an invalid address value.
Programmer response
Ensure that the dialog variable specified on
the )INEXIT statement contains the address of a panel
input exit routine.
ISPP900 Panel 'aaaaaaaa' error - Invalid
ADDSOSI value (not a dialog
variable).
Explanation
Only a dialog variable may be specified in the
ADDSOSI built-in function.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify a dialog variable for the parameter to the
ADDSOSI built-in function.
ISPP901 Panel 'aaaaaaaa' error - Invalid
DELSOSI value (not a literal or
dialog variable).
Explanation
Something other than a literal or a dialog variable
was specified as a parameter on the DELSOSI built-in
function; for example, omitting the parameter to the
function.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify only a literal or a dialog variable on the
DELSOSI built-in function.
ISPP902 Panel 'aaaaaaaa' error - Invalid
ONEBYTE value (not a dialog
variable).
Explanation
Something other than a literal or a dialog variable
was specified as a parameter on the ONEBYTE built-in
function; for example, omitting the parameter to the
function.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify only a literal or a dialog variable on the
ONEBYTE built-in function.
ISPP903 Panel 'aaaaaaaa' error - Invalid
TWOBYTE value (not a dialog
variable).
Explanation
Something other than a literal or a dialog variable was
specified as a parameter on the TWOBYTE built-in
function; for example, omitting the parameter to the
function.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  253

## Page 274

User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Specify only a literal or a dialog variable on the
TWOBYTE built-in function.
ISPP904 Panel 'aaaaaaaa' error - x'0E' or
x'0F' is invalid as an attribute
character.
Explanation
A shift-in or shift-out character was used as an
attribute character on the panel being displayed. They
are not valid attribute characters on display devices
that support the shift-in and shift-out characters.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Use any character other than X'00', X'0E', X'0F',
X'40'(blank), or ampersand as an attribute character.
ISPP910 Must be DBCS - Enter DBCS
characters.
Explanation
The field for which you are entering data requires
DBCS (double-byte character set) characters.
User response
Enter only double-byte characters in the field.
ISPP911 Must be mixed - Enter correct
mixed form characters.
Explanation
The field for which you are entering data accepts
both DBCS (double-byte character set) characters and
EBCDIC (single-byte) characters. You must, therefore,
enclose DBCS character strings in shift-out and shift-in
characters.
User response
Enter the double-byte characters strings in the field
with shift-out and shift-in characters surrounding
them.
ISPP912 Must be EBCDIC - Enter EBCDIC
characters.
Explanation
The field for which you are entering data accepts only
EBCDIC (single-byte) characters.
User response
Enter only single-byte characters in the field.
ISPP913 ADDSOSI error - Invalid ADDSOSI
value (dialog var) found while
INIT/PROC panel.
Explanation
The variable name specified in the ADDSOSI built-in
function resolved to a value in the )INIT section or
the )PROC section that cannot have shift-out and shift-
in characters added.
Programmer response
Verify the panel logic to ensure that valid data for the
ADDSOSI function is being passed. An example of data
that could result in this error is a solitary shift-out or
shift-in character or unbalanced shift-out and shift-in
characters in a DBCS or mixed string.
ISPP914 DELSOSI error - Invalid DELSOSI
value (dialog var) found while
INIT/PROC panel.
Explanation
The literal or variable name specified in the DELSOSI
built-in function resolved to a value in the )INIT
section or the )PROC section that cannot have shift-out
and shift-in characters added.
ISPF messages starting with ISP
254  z/OS: z/OS ISPF Messages and Codes

## Page 275

Programmer response
Verify the panel logic to ensure that valid data for the
DELSOSI function is being passed. An example of data
that could result in this error is data that does not
contain a shift-out/shift-in character pair.
ISPP915 TWOBYTE error - Invalid
TWOBYTE value (dialog var) found
while INIT/PROC panel.
Explanation
The literal or variable name specified in the TWOBYTE
built-in function resolved to a value in the )INIT
section or the )PROC section that cannot have shift-out
and shift-in characters added.
Programmer response
Verify the panel logic to ensure that valid data for the
TWOBYTE function is being passed. An example of
data that could result in this error is a solitary shift-out
or shift-in character or unbalanced shift-out and shift-
in characters in a DBCS or mixed string.
ISPP916 ONEBYTE error - Invalid ONEBYTE
value (dialog var) found while
INIT/PROC panel.
Explanation
The literal or variable name specified in the ONEBYTE
built-in function resolved to a value in the )INIT
section or the )PROC section that cannot have shift-out
and shift-in characters added.
Programmer response
Verify the panel logic to ensure that valid data for
the ONEBYTE function is being passed. An example of
data that could result in this error is a solitary shift-out
or shift-in character or unbalanced shift-out and shift-
in characters in a DBCS or mixed string.
ISPP917 Panel 'aaaaaaaa' error -
Invalid attribute FORMAT change
attempted in an attribute override
statement.
Explanation
The attribute override attempted to change the format
from EBDCID to DBCS or vice versa. This is not
allowed.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
You cannot change the FORMAT in an attribute
override statement.
ISPP918 Panel 'aaaaaaaa' error - Exceeds
maximum number of attributes
allowed (127).
Explanation
Only 127 attributes are allowed in the definition of a
panel.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Reduce the number of attributes being used on the
panel.
ISPP920 Panel 'aaaaaaaa' error - The
alternate command field name
given is a DBCS format field.
Explanation
The alternate command field (CMD() on the )BODY
section panel statement) cannot have a format of
DBCS.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  255

## Page 276

Programmer response
Change the FORMAT specification of the attribute
being used to define the alternate command line field.
A FORMAT(MIX) specification is recommended since
FORMAT(EBCDIC) does not display DBCS characters
properly.
ISPP921 Panel 'aaaaaaaa' error - The
alternate short or long message
field name given is DBCS format.
Explanation
The alternate short or long message field cannot have
a format of DBCS.
User response
There was a programming error defining the panel that
you attempted to display. If you are running in TEST
mode, you can either override the error and attempt
to continue running the dialog, or you can choose not
to override the error. If you select the latter, the dialog
will end and you will be returned to the initial panel for
the application.
Programmer response
Change the FORMAT specification of the attribute
being used to define the short or long message field.
A FORMAT(MIX) specification is recommended since
FORMAT(EBCDIC) does not display DBCS characters
properly.
ISPP940 Trace complete - The Trace
output has been written to
SYSOUT allocated to the DDname
ISPDPTRC.
Explanation
The output to the Panel Trace has been written to a
SYSOUT class allocated to the DDname ISPDPTRC and
can not be viewed by the ISPDPTRC command.
ISPP941 Parameter Invalid - Parameter
#aaaaaaaa is invalid. Valid
parameters are: END, VIEW,
LIST, QUIET, DISPLAY(), PANEL(),
READ(), SCREEN(), SECTION(),
SERVICE(), and DEBUG.
Explanation
A parameter specified for the ISPDPTRC command is
invalid. Valid parameters are: END, VIEW, LIST, QUIET,
DISPLAY(), PANEL(), READ(), SCREEN(), SECTION(),
SERVICE(), and DEBUG.
User response
Correct the command parameters.
ISPP942 Invalid panel name - The panel
name specified for the PANEL()
parameter is invalid.
Explanation
The panel name specified must be either a valid
member name or member name pattern.
User response
Correct the supplied panel name.
ISPP943 aaaaaaaa value missing - The
value for aaaaaaaa parameter was
omitted.
Explanation
This message is self explanatory.
User response
Correct the command subparameters.
ISPP944 Invalid parameter value - The
DISPLAY parameter value is
invalid. Valid values are: NONE,
IN, OUT, BOTH.
Explanation
The value specified for the DISPLAY() parameter is
invalid. Valid values are: NONE, IN, OUT, BOTH.
User response
Correct the parameter value.
ISPP945 Invalid parameter value - The
SCREEN() parameter value is
invalid. Valid values are: *
(current), 0 (all), or a screen id in
the range 1-9, A-W.
Explanation
The value specified for the SCREEN() parameter is
invalid. Valid values are: * (current), 0 (all), or a screen
id in the range 1-9, A-W.
User response
Correct the parameter value.
ISPF messages starting with ISP
256  z/OS: z/OS ISPF Messages and Codes

## Page 277

ISPP946 Invalid parameter value - The
SECTION() parameter values are
invalid. Valid values are either
'*', NONE, or a combination of:
INIT, REINIT, PROC, or NOINIT,
NOREINIT, NOPROC.
Explanation
The values specified for the SECTION() parameter
are invalid. Valid values are either '*', NONE, or
a combination of: INIT, REINIT, PROC, or NOINIT,
NOREINIT, NOPROC.
User response
Correct the parameter value.
ISPP947 Invalid parameter value - The
SERVICE() parameter value is
invalid. Valid values are: NONE,
DETAIL.
Explanation
The value specified for the SERVICE() parameter is
invalid. Valid values are: NONE, DETAIL.
User response
Correct the parameter value.
ISPP948 Invalid parameter value - The
READ() parameter value is
invalid. Valid values are: NONE,
SUMMARY, DETAIL.
Explanation
The value specified for the READ() parameter is invalid.
Valid values are: NONE, SUMMARY, DETAIL.
User response
Correct the parameter value.
ISPP950 LIST substituted - Panel trace data
set 'aaaaaaaa' not found. List of
panel trace data sets displayed
Explanation
ISPDPTRC attempted to VIEW a trace data set that
could not be found. A data set list of possible panel
trace data sets was displayed
ISPP951 No trace data sets found - No ISPF
panel trace data set names were
found matching 'aaaaaaaa'
Explanation
This is an informational message.
ISPP952 Must be numeric - Enter a positive
or negative number
Explanation
The generation number must be a positive or negative
numeric value.
User response
Specify a valid generation number.
ISPR000 CONTROL service error -
An unexpected error received
from the CONTROL service
(RC=aaaaaaaa).
Explanation
The CONTROL service executed with a return code of
aaaaaaaa.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR001 Invalid specification - From and
To data sets cannot be the same
when all members specified.
Explanation
ISPPREP does not allow the same data set name
to be specified for both input and output when the
"*"(asterisk) notation is used to process all members.
User response
Change the name of the output data set.
ISPR002 Parameters missing - The
invocation parameters are
required for batch execution.
Explanation
A batch execution has been requested, but no
invocation parameters were specified.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  257

## Page 278

User response
Correct the invocation syntax to specify the required
parameters.
ISPR003 Variable service error - An
unexpected error received from
variable services (RC=aaaaaaaa).
Explanation
One of the variable services executed with a return
code of aaaaaaaa.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR004 Display service error - An
unexpected error received
from the display service
(RC=aaaaaaaa).
Explanation
The DISPLAY service executed with a return code of
aaaaaaaa.
User response
If the error continues, contact your system
programmer.
ISPR006 Member list error - An internal
member list service error
(RC=aaaaaaaa).
Explanation
The member list function executed with a return code
of aaaaaaaa.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR008 LIBDEF service error - An
unexpected error was received
from the LIBDEF service
(RC=aaaaaaaa).
Explanation
The LIBDEF service executed with a return code of
aaaaaaaa.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR009 FIND service error - An
unexpected error received from
the FIND service (RC=aaaaaaaa).
Explanation
The find function executed with a return code of
aaaaaaaa.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR010 ENQUEUE service error - An
unexpected error was received
from the ENQUEUE service
(RC=aaaaaaaa).
Explanation
This message is self-explanatory.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR011 Utility - ISPPREP - Panel
'aaaaaaaa' not converted. See
previous log message.
ISPF messages starting with ISP
258  z/OS: z/OS ISPF Messages and Codes

## Page 279

Explanation
The panel aaaaaaaa could not be converted. The
previous log message refers to the specific problem
encountered.
User response
Correct the reported problem and reprocess the panel.
ISPR012 Panel not processed - Panel
'aaaaaaaa' not converted. See
ISPF log for explanation.
Explanation
The panel aaaaaaaa could not be converted. Refer
to the ISPF log for another message with the specific
problem encountered.
User response
Correct the reported problem and reprocess the panel.
ISPR013 Utility - ISPPREP - Panel
'aaaaaaaa' in use by you or
another user.
Explanation
ISPPREP cannot process panel aaaaaaaa because it is
already in use by you or another user.
User response
Free the aaaaaaaa panel from use and reprocess it.
ISPR014 Panel in use - Panel 'aaaaaaaa' in
use by you or another user.
Explanation
ISPPREP cannot process panel aaaaaaaa because it is
already in use by you or another user.
User response
Free the aaaaaaaa panel from use and reprocess it.
ISPR015 DEQUEUE service error - An
unexpected error was received
from the DEQUEUE service
(RC=aaaaaaaa).
Explanation
This message is self-explanatory.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR016 STOW service error - An
unexpected error was received
from the STOW service
(RC=aaaaaaaa).
Explanation
This message is self-explanatory.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR017 I/O error on output -
An unexpected error was
received from the put service
(RC=aaaaaaaa).
Explanation
This message is self-explanatory.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR018 Output service error - An
unexpected error was received
from the output service
(RC=aaaaaaaa).
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  259

## Page 280

System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
If the error continues, contact your system
programmer.
ISPR019 Enter new member name -
Member must be renamed when
From and To data sets are the
same.
Explanation
ISPPREP does not allow the output member name to
be the same as the input member name when the
output data set name is the same as the input data set
name.
User response
Specify a different member name for the output
member.
ISPR020 Conversion of aaaaaaaa panels
in progress - bbbbbbbb panels
processed.
Explanation
This is an informational message. This is a status
message issued every 50 panels when more than 50
panels are processed in a single ISPPREP cycle.
ISPR021 Utility - ISPPREP - Panel
'aaaaaaaa' already exists in
output data set.
Explanation
The output panel aaaaaaaa already exists in the
output data set, and member replace was not
specified.
User response
Specify member replace, or choose a different output
panel name.
ISPR022 Panel not processed - Panel
'aaaaaaaa' already exists in
output data set.
Explanation
The output panel aaaaaaaa already exists in the
output data set, and member replace was not
specified.
User response
Specify member replace, or choose a different output
panel name.
ISPR023 Conversion of aaaaaaaa panels in
progress.
Explanation
This is an informational message. This is a status
message issued at the start of an ISPPREP processing
cycle.
ISPR024 Utility - ISPPREP - Data set
"aaaaaaaa" contains no members.
Explanation
The data set aaaaaaaa(44) specified as the input to
ISPPREP has no members.
User response
Verify the data set name.
ISPR025 No members in data set - Data set
"aaaaaaaa" contains no members.
Explanation
The data set aaaaaaaa(44) specified as the input to
ISPPREP has no members.
User response
Verify the data set name.
ISPR026 Utility - ISPPREP - Panel
'aaaaaaaa' was not found in the
PDS directory.
Explanation
ISPPREP could not find member aaaaaaaa in the
input data set directory.
ISPR027 Member not found - Panel
'aaaaaaaa' was not found in the
PDS directory.
ISPF messages starting with ISP
260  z/OS: z/OS ISPF Messages and Codes

## Page 281

Explanation
ISPPREP could not find member aaaaaaaa in the
input data set directory.
ISPR028 Invalid output LRECL - ISPPREP
output data set has an invalid
LRECL, valid LRECL is between 80
160 for FB and 84 and 164 for VB.
Explanation
The record length of the output data set is not
supported by ISPF.
User response
Use an output data set with a record length between
80 and 160 for fixed blocked data sets, and 84 and
164 for variable blocked data sets.
ISPR030 Utility - preprocess - Panel
'aaaaaaaa' has been successfully
converted.
Explanation
This is an informational message.
ISPR031 Panel aaaaaaaa converted - Panel
'aaaaaaaa' has been successfully
converted.
Explanation
This is an informational message.
ISPR032 Utility - preprocess - aaaaaaaa
panels have been successfully
converted.
Explanation
This is an informational message.
ISPR033 Panels converted - aaaaaaaa
panels have been successfully
converted.
Explanation
This is an informational message.
ISPR034 Utility - preprocess - Unable
to convert aaaaaaaa panels.
Successfully converted bbbbbbbb
panels.
Explanation
Some of the panels could not be converted.
User response
Refer to previous ISPF log messages for information
about the conversion failure for each panel not
converted.
ISPR035 Panels not converted - Unable
to convert aaaaaaaa panels.
Successfully converted bbbbbbbb
panels.
Explanation
Some of the panels could not be converted.
User response
Refer to the ISPF log messages for information about
the conversion failure for each panel not converted.
ISPR037 From data set aaaaaaaa
Explanation
This is an informational message. This is a part of the
ISPF log messages written at the end of the ISPPREP
conversion cycle.
ISPR038 to data set aaaaaaaa
Explanation
This is an informational message. This is a part of
the ISPF log messages written at the end of the
ISPPREP conversion cycle. This message is issued
when either all members of the input data set have
been converted, or a single member is converted and
the output member name is the same as the input
member name.
ISPR039 to data set aaaaaaaa, newname =
bbbbbbbb
Explanation
This is an informational message. This is a part of the
ISPF log messages written at the end of the ISPPREP
conversion cycle. This message is issued in place of
message ISPR038 when a single member is converted
and the output member name is different from the
input member name.
ISPR040 Input parameter error - Required
keyword parameter INPAN is
missing.
Explanation
This message is self-explanatory.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  261

## Page 282

User response
Provide the INPAN keyword and input data set name.
ISPR041 Input parameter error - The INPAN
and OUTPAN data set names must
be unique.
Explanation
This message is self-explanatory.
User response
Specify a different data set name for input and output.
ISPR042 Input parameter error - Input
string "aaaaaaaa" contains syntax
error or invalid keyword.
Explanation
This message is self-explanatory.
User response
Correct the syntax "aaaaaaaa" and reprocess.
ISPR043 Input parameter error - Input
string "aaaaaaaa" contains
duplicate or conflicting keyword.
Explanation
This message is self-explanatory.
User response
Correct the syntax "aaaaaaaa" and reprocess.
ISPR044 Input parameter error - Output
member name is invalid when
entire input library selected.
Explanation
This message is self-explanatory.
User response
Remove the output member name.
ISPR045 Input parameter error - Required
keyword parameter OUTPAN is
missing.
Explanation
This message is self-explanatory.
User response
Provide the OUTPAN keyword and the output data set
name.
ISPR050 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing panel bbbbbbbb.. The
panel output library cccccccc is
full.
Explanation
A system abend D37 occurred while processing the
indicated panel.
Programmer response
Increase the size of the output panel library and
compress if necessary.
Problem determination
Ensure that the TSO profile is set to WTPMSG and
MSGID so that the system abend messages are seen
for this error and additional information can then be
obtained from system documentation.
ISPR051 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing panel bbbbbbbb.. The
panel output library cccccccc or
the panel output volume is full.
Explanation
A system abend B37 or E37 was encountered while
processing the panel output library.
System programmer response
Point to a different pack, or make space available on
the existing pack if the pack is currently full.
Programmer response
Refer to system documentation on these abend codes.
The data set may be out of extents and need a larger
allocation. Also the pack may be full, leaving no room
to write additional records.
Problem determination
Ensure that the TSO profile is set to WTPMSG and
MSGID so that the system abend messages are seen
for this error. Additional information can then be
obtained from system documentation.
ISPF messages starting with ISP
262  z/OS: z/OS ISPF Messages and Codes

## Page 283

ISPR053 ESTAE error - aaaaaaaa received
return code bbbbbbbb from ESTAE.
Explanation
An error occurred on the ESTAE macro.
System programmer response
Refer to the appropriate system documentation for the
ESTAE macro to check the return code issued by the
ESTAE macro.
User response
Contact your system programmer.
ISPR054 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing panel bbbbbbbb.
Explanation
A system abend occurred while processing a panel.
System programmer response
If the abend code indicates insufficient space in the
panel output data set, try either deleting unneeded
members or compress the data set or both. If
necessary, contact IBM support.
User response
Refer to your system documentation to discover
the cause of the problem, or contact your system
programmer.
ISPS001 Terminal output error - ** ISPF
screen output error - code =
aaaaaaaa **
Explanation
There was an error in the screen display data stream.
System programmer response
See z/OS ISPF Dialog Developer's Guide and Reference
for a listing of the screen errors.
User response
If the error continues, contact the system programmer.
ISPS002 Terminal input error - ** ISPF
screen input error - code =
aaaaaaaa **
Explanation
There was an error in the screen display data stream.
System programmer response
See z/OS ISPF Dialog Developer's Guide and Reference
for a listing of the screen errors.
User response
If the error continues, contact the system programmer.
ISPS003 Trace - TPUT - - Tldaaaaaaaa
length=bbbbbbbb option=cccccccc
last 4 bytes=dddddddd
Explanation
This is an informational message.
ISPS004 Trace - TGET - - Tldaaaaaaaa
length=bbbbbbbb option=cccccccc
Explanation
This is an informational message.
ISPS006 Terminal TPUT error - ** ISPF
screen output error - code =
aaaaaaaa - TPUT RC = bbbbbbbb
**
Explanation
There was an error in the screen display data stream.
System programmer response
See z/OS ISPF Dialog Developer's Guide and Reference
for a listing of the screen errors.
User response
If the error continues, contact the system programmer.
ISPS007 Terminal TGET error - ** ISPF
screen input error - code =
aaaaaaaa - TGET RC = bbbbbbbb
**
Explanation
There was an error in the screen display data stream.
System programmer response
See z/OS ISPF Dialog Developer's Guide and Reference
for a listing of the screen errors.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  263

## Page 284

User response
If the error continues, contact the system programmer.
ISPS008 Data: hex=aaaaaaaa,
EBCDIC='bbbbbbbb'
Explanation
This is an informational message.
ISPS009 Tutorial test line - aaaaaaaa Cur
panel = bbbbbbbb Prev panel =
cccccccc Last msg = dddddddd
Explanation
This is an informational message.
ISPS011 32K not available for attach of
PMD - ** Logical screen request
failed - insufficient storage.**
Explanation
Storage obtain failed.
System programmer response
Ensure that the user's region size satisfies the ISPF
minimum storage requirement.
User response
Contact your system programmer.
ISPS012 User action for ISPS011 - ** Log
on with larger SIZE parameter **
Explanation
This is an informational message.
ISPS013 Attach of PMD failed - ** Logical
screen request failed - ATTACH
RC= aaaaaaaa **
Explanation
The task could not be processed.
User response
Contact your system programmer.
Programmer response
The ATTACH macro failed, contact IBM support.
ISPS014 Immediate PMD termination after
ATTACH - ** Logical screen
request failed - abend aaaaaaaa
**
Explanation
The task could not be processed.
System programmer response
If the abend code is an ISPF user abend code, assume
that the failure is caused by the user environment. If
the abend is a system abend, take a subtask dump to
determine the reason for the abend.
User response
Contact your system programmer.
ISPS015 User action for ISPS013 and
ISPS014 - ** Contact your
system programmer or dialog
developer.**
Explanation
This is an informational message.
ISPS016 No abend recovery till P.O.M PROC
section processed ok - No dialog
error recovery until a primary
option menu is fully processed.
Explanation
This is an informational message. ISPF error recovery
is not available until you have processed an Enter on a
primary option menu.
ISPS017 Error before error-free primary
option menu known to DM - A
dialog error has occurred before
the dialog manager is initialized.
Explanation
This is an informational message. ISPF error recovery
is not available until you have processed an Enter on a
primary option menu.
User response
Contact the responsible programmer.
Programmer response
Determine why the dialog fails. You may code a
primary option menu to enable ISPF error recovery.
ISPF messages starting with ISP
264  z/OS: z/OS ISPF Messages and Codes

## Page 285

ISPS018 Invalid ZISPFRC value
('aaaaaaaa'), nonnumeric
character found.
Explanation
The value in ZISPFRC must contain a numeric value.
User response
Contact the responsible programmer.
Programmer response
Ensure that your dialog returns a numeric value to
ZISPFRC.
ISPS019 Invalid ZISPFRC value
('aaaaaaaa'), out of allowable
range.
Explanation
The value returned to ZISPFRC exceeds 16777215.
User response
Contact the responsible programmer.
Programmer response
Correct the value returned to ZISPFRC.
ISPS100 Invalid service name - 'aaaaaaaa'
exceeds the allowable length of 8.
Explanation
The user invoked an ISPF service and the service
name's length exceeded the 8 character limit.
User response
Contact the responsible programmer.
Programmer response
Correct the ISPF service name.
ISPS101 Service name missing - No dialog
service request name found.
Explanation
ISPF does not know what service to execute.
User response
Contact the responsible programmer.
Programmer response
Correct the dialog to provide an ISPF service name.
ISPS102 Invalid service name - 'aaaaaaaa'
is not a recognized dialog service
name.
Explanation
The ISPF service name is not valid.
User response
Contact the responsible programmer.
Programmer response
Correct the dialog to provide a valid ISPF service
name.
ISPS103 Too many parameters -
Unexpected parameters were
found on the dialog service
statement.
Explanation
The user invoked an ISPF service with too many
parameters.
User response
Contact the responsible programmer.
Programmer response
Correct the parameters passed with the ISPF service.
ISPS104 Error in subfield - An error was
encountered in the subfield of the
'aaaaaaaa' keyword.
Explanation
An ISPF service was invoked and an error was found in
the service keyword's numeric token.
User response
Contact the responsible programmer.
Programmer response
Correct the use of the ISPF service, the keyword (as
displayed in the message), and the accompanying
subfield.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  265

## Page 286

ISPS105 Invalid keyword - 'aaaaaaaa' is
not a valid keyword for this
service.
Explanation
An invalid keyword was used when invoking an ISPF
service.
User response
Contact the responsible programmer.
Programmer response
Enter a keyword that is valid for this ISPF service.
ISPS106 Subfield missing - 'aaaaaaaa'
requires a subfield. None found.
Explanation
A keyword on an ISPF service call requires a subfield,
but none was found.
User response
Contact the responsible programmer.
Programmer response
Provide the required subfield.
ISPS107 Required parm missing - Not all
required parameters were found
for the aaaaaaaa service.
Explanation
The ISPF service cannot run until all the required
parameters are provided.
User response
Contact the responsible programmer.
Programmer response
Provide the parameters required for this service.
If one of the parameters is an address, it might be that
the first byte is x'40', which ISPF interprets as a blank.
Turn the high order bit to ON for address parameters to
avoid this issue.
ISPS108 Invalid length - Parameter
'aaaaaaaa' exceeds the allowable
length.
Explanation
The ISPF service cannot be run because the parameter
exceeds the length allowed for that service.
User response
Contact the responsible programmer.
Programmer response
Correct the length of the parameter.
ISPS109 Unexpected list found - A list of
names was found where a list was
not expected.
Explanation
The ISPF service cannot be run.
User response
Contact the responsible programmer.
Programmer response
Correct the syntax of the ISPF service that was being
run.
ISPS110 Too many items in list - The
number of names in the list
exceeds aaaaaaaa.
Explanation
An ISPF service was invoked with a name-list
parameter and the number of names in the list
exceeds the value given in the message.
User response
Contact the responsible programmer.
Programmer response
Correct the syntax of the ISPF service.
ISPS111 Service string too long - Exceeds
maximum length of 32767 bytes
after variable substitution.
Explanation
The string to invoke the ISPF service exceeded the
maximum buffer size of 32767 bytes after variable
substitution.
ISPF messages starting with ISP
266  z/OS: z/OS ISPF Messages and Codes

## Page 287

User response
Contact the responsible programmer.
Programmer response
Correct the syntax of the ISPF service.
ISPS112 Var substitution error - Severe
error while resolving symbolic
variables in ISPEXEC statement.
Explanation
A variable on the ISPEXEC statement could not be
resolved.
User response
Contact the responsible programmer.
Programmer response
Correct the ISPEXEC statement in the dialog.
ISPS113 Invalid name list - A name list
must begin with a left parenthesis.
Explanation
An ISPF service's name-list parameter is missing a left
parenthesis. If the ISPF service's name-list parameter
consists of more than one name, it must be enclosed
in parentheses.
User response
Contact the responsible programmer.
Programmer response
Correct the ISPEXEC statement in error.
ISPS114 Conflicting keywords - Keyword
'aaaaaaaa' conflicts with a
previously specified keyword.
Explanation
The ISPF service could not execute because of an
error in the keyword coding.
User response
Contact the responsible programmer.
Programmer response
Correct the ISPEXEC statement in error.
ISPS115 Invalid keyword - 'aaaaaaaa'
is not valid in the specified
parameter position.
Explanation
The ISPF service could not be executed because of a
syntax problem.
User response
Contact the responsible programmer.
Programmer response
Correct the ISPEXEC statement in error.
ISPS116 PDF services unavailable -
Insufficient storage to load the
PDF component of ISPF.
Explanation
An error occurred attempting to load the PDF
component of ISPF. The error indicates insufficient
storage for the PDF component.
System programmer response
Verify that the user's region size meets the minimum
requirements for ISPF. If the problem continues,
contact IBM support.
User response
Contact the system programmer.
ISPS117 Invalid service request - The
'aaaaaaaa' service is not allowed
via the ISPEXEC interface.
Explanation
There are several services not available when you use
the ISPEXEC interface, but are available when you
use the ISPLINK interface. The service given in the
message is one that is not allowed with the ISPEXEC
interface.
User response
Contact the responsible programmer.
Programmer response
Correct the dialog to use only valid ISPF services via
the ISPEXEC interface.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  267

## Page 288

ISPS118L SERVICE NOT INVOKED - A VALID
ISPF ENVIRONMENT DOES NOT
EXIST.
Explanation
ISPS118L is issued when ISPLINK does not find the
ISPTASK TCB present on the TCBOTC chain. This can
happen if ISPLINK is called from native TSO and ISPF
was never started. This can also occur if ISPLINK is
called from an authorized program even though ISPF is
active.
User response
Contact the responsible programmer.
Programmer response
Make sure ISPF is active prior to issuing ISPLINK
requests. If the error is a result of a call from an
authorized program then remove this call to ISPLINK
from the authorized program. Authorized programs
cannot make ISPF service requests.
ISPS118S SERVICE NOT INVOKED - A VALID
ISPF ENVIRONMENT DOES NOT
EXIST.
Explanation
ISPF will find the ISPTASK TCB and check to see if the
requesting task is at the same task level as ISPTASK.
If the application is not at the same level ISPF will
attempt to POST the applications ECB. If there is
not an ECB that ISPF knows about then message
ISPS118S is issued. For ISPF to know about the ECB
the SELECT CMD service is required to set up the
POST/WAIT interface. Additionally the SELECT CMD
must be against a command processor.
User response
Contact the responsible programmer.
Programmer response
Ensure that the ISPF SELECT services commands are
issued correctly.
ISPS119 INVALID SERVICE - 'aaaaaaaa'
service not supported in the batch
environment.
Explanation
The service listed in the message is not supported in
the batch environment.
User response
Contact the responsible programmer.
Programmer response
Correct the erroneous ISPEXEC statement.
ISPS190 No ADDPOP specified - The FRAME
or BKGRND keyword was specified
before an ADDPOP keyword was
found on a SELECT statement.
Explanation
You must code an ADDPOP keyword before specifying
FRAME or BKGRND.
Programmer response
Correct the SELECT service parameters.
ISPS191 Invalid frame type - An invalid
FRAME value was specified on a
SELECT service. The only valid
values are STD, FIX or DLG.
Explanation
The FRAME keyword only accepts these values:
FRAME(STD), FRAME(FIX), FRAME(DLG).
Programmer response
Correct the SELECT service parameters.
ISPS192 Invalid background - An invalid
BKGRND value was specified on
a SELECT service. The only valid
values are STD or DLG.
Explanation
The BKGRND keyword only accepts these values:
BKGRND(STD), BKGRND(DLG).
Programmer response
Correct the SELECT service parameters.
ISPS193 Conflicting parameters - WSCMD
conflicts with another keyword.
Explanation
A keyword was found that is inconsistent with the
WSCMD keyword. For example, WSCMD and PANEL
cannot both be used in the same command.
ISPF messages starting with ISP
268  z/OS: z/OS ISPF Messages and Codes

## Page 289

Programmer response
Check the usage of the WSCMD keyword and eliminate
the conflict.
ISPS200 SNAP macro error - SNAP macro
error - aaaaaaaa ddname is not
allocated.
Explanation
The ddname must be allocated before the SNAP macro
can execute.
User response
The ddname selected for use during execution of the
SNAP macro must be allocated.
ISPS201 SNAP macro error - Error from
SNAP macro, return code =
aaaaaaaa
Explanation
The SNAP macro was not executed successfully.
System programmer response
If the failure continues, contact IBM support.
User response
If the failure continues, contact the system
programmer.
ISPS202 SNAP dump generated - A SNAP
dump has been generated.
Explanation
The MVS SNAP macro has been issued and has
produced a dump of the internal ISPF terminal trace
buffer.
ISPS203 SNAP macro error - SNAP macro
error - no ddname has been
specified for the SNAP macro.
Explanation
A ddname must be specified to allow a successful
execution of the SNAP macro.
User response
See the ENVIRON command in z/OS ISPF Dialog
Developer's Guide and Reference for an explanation of
how to specify the ddname.
ISPS204 SNAP dump in progress - A
SNAP dump is currently being
processed.
Explanation
This is an informational message.
ISPS290 Conflicting parameters - WSCMD
conflicts with another keyword.
Explanation
A keyword was found that is inconsistent with the
WSCMD keyword. For example, WSCMD and PANEL
cannot both be used in the same command.
Programmer response
Check the usage of the WSCMD keyword and eliminate
the conflict.
ISPS291 WSCMD string too long - The
parameter specified on SELECT
WSCMD is too long.
Explanation
The text in the WSCMD parameter on the SELECT
service is longer than the 255 maximum allowed.
Programmer response
The text may fit if placed in a variable and the SELECT
WSCMDV . parameter is used.
ISPS292 WSCMDV variable error - The
variable specified in the SELECT
WSCMDV is either not defined or
its contents are blank, or longer
than 255 characters.
Explanation
ISPF tried to resolve the variable specified in the
SELECT WSCMDV service and encountered an error.
The variable may not be defined. If the variable is
defined, its contents may be blank or longer than the
255 character maximum allowed.
Programmer response
Verify the contents of the variable and correct it.
ISPS300 Name too long - Variable name
supplied to the QTABOPEN service
must be less than 8 characters in
length
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  269

## Page 290

Explanation
QTABOPEN service returns a list of variables and
needs at least one character length to return some
data.
Programmer response
Provide a variable name that is shorter than 8
characters.
ISPS301 List truncated - More data
was available to return by
the QTABOPEN service, however
no more space is available to
construct a valid variable name.
aaaaaaaa instances of the list
variable have been created.
Explanation
More data was available to return by the QTABOPEN
service, however no more space is available to
construct a valid variable name.
Programmer response
Shorten the name of the variable passed to QTABOPEN
to allow more space for the suffix to be constructed.
ISPS310L COMMAND NOT INVOKED - A
VALID ISPF ENVIRONMENT DOES
NOT EXIST.
Explanation
ISPS318L is issued when an ISPF Command does not
find the ISPTASK TCB present on the TCBOTC chain.
This can happen if the ISPF Command is called from
native TSO and ISPF was never started. This can also
occur if the command is called from an authorized
program even though ISPF is active.
User response
Contact the responsible programmer.
Programmer response
Make sure ISPF is active prior to issuing the ISPF
command. If the error is a result of a call from
an authorized program then remove this call to the
command from the authorized program. Authorized
programs cannot make ISPF service requests.
ISPT001 Internal table error - aaaaaaaa
detected an invalid DTA or DTB
control block.
Explanation
ISPF detected an invalid table control block.
System programmer response
An internal table control block is invalid. This could
be a possible storage overlay problem. If this fails
in a specific application, check that application for
destructive move statements or other possible overlay
problems. If the failure still continues, force a dump
and contact IBM support.
User response
Contact the responsible programmer.
ISPT002 Internal table error - aaaaaaaa
detected an invalid service
request or RDA.
Explanation
ISPF detected an invalid table control block.
System programmer response
An internal table control block is invalid. This could
be a possible storage overlay problem. If this fails
in a specific application, check that application for
destructive move statements or other possible overlay
problems. If failure still continues, force a dump and
contact IBM support.
User response
Contact the responsible programmer.
ISPT003 Internal table error - Row data
area not large enough to contain
the updated row.
Explanation
The RDA control block is not large enough for the row
being modified.
System programmer response
Try restoring the table from a backup and retry the
application. If the problem still persists, contact IBM
support.
User response
Contact the responsible programmer.
ISPF messages starting with ISP
270  z/OS: z/OS ISPF Messages and Codes

## Page 291

ISPT004 Internal table error - aaaaaaaa
detected error during SAWA
construction.
Explanation
An error was encountered building an internal table
control block.
System programmer response
Try restoring the table from a backup and retry the
application. If the problem still persists, contact IBM
support.
User response
Contact the responsible programmer.
ISPT005 Internal table error - aaaaaaaa
detected an invalid name value
work area.
Explanation
An error was encountered with an internal table
control block.
System programmer response
Try restoring the table from a backup and retry the
application. If the problem still persists, contact IBM
support.
User response
Contact the responsible programmer.
ISPT006 Variable services error - aaaaaaaa
received return code bbbbbbbb
from ISPDVCGT.
Explanation
An error occurred while trying to retrieve variable for a
table row.
System programmer response
This error should not occur. It could be caused by a
storage overlay situation. Contact IBM support.
User response
Contact your system programmer.
ISPT007 Router service error - aaaaaaaa is
unknown table service.
Explanation
An invalid internal table service request was issued.
System programmer response
ISPF issued an invalid table service request through
an internal interface. This should never occur. Obtain a
dump at the time of the error and contact IBM support.
User response
Contact your system programmer.
ISPT008 GETMAIN error - aaaaaaaa
received return code bbbbbbbb
from GETMAIN.
Explanation
The GETMAIN macro failed.
System programmer response
Refer to the appropriate system documentation on the
GETMAIN macro for the return code information.
User response
Contact your system programmer.
ISPT009 FREEMAIN error - aaaaaaaa
received return code bbbbbbbb
from FREEMAIN.
Explanation
The FREEMAIN macro failed.
System programmer response
Refer to the appropriate system documentation on the
FREEMAIN macro for the return code information.
User response
Contact your system programmer.
ISPT010 Internal table error - aaaaaaaa
detected an invalid name list work
area.
Explanation
An error was encountered building an internal table
control block.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  271

## Page 292

System programmer response
Try restoring the table from a backup and then retry
the application. If the problem persists, contact IBM
support.
User response
Contact your system programmer.
ISPT011 DEQUEUE error - aaaaaaaa
received return code bbbbbbbb
from DEQUEUE.
Explanation
An error occurred on the DEQUEUE macro.
System programmer response
Refer to the appropriate system documentation on the
DEQUEUE macro for the return code information.
User response
Contact your system programmer.
ISPT012 ENQUEUE error - aaaaaaaa
received return code bbbbbbbb
from ENQUEUE.
Explanation
An error occurred on the ENQUEUE macro.
System programmer response
Refer to the appropriate system documentation on the
ENQUEUE macro for the return code information.
User response
Contact your system programmer.
ISPT013 Internal table error - aaaaaaaa
received return code bbbbbbbb
from TFD build.
Explanation
An error occurred while trying to build an internal
control block.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPT014 Error on RDJFCB - aaaaaaaa
received return code bbbbbbbb
from RDJFCB.
Explanation
An error occurred on the RDJFCB macro.
System programmer response
Refer to the appropriate system documentation on the
RDJFCB macro for the return code information.
User response
Contact your system programmer.
ISPT015 BLDL/FIND error - aaaaaaaa
received return code bbbbbbbb
from BLDL/FIND.
Explanation
An error occurred on the BLDL/FIND macro
System programmer response
Refer to the appropriate system documentation on the
BLDL/FIND macro for the return code information.
User response
Contact your system programmer.
ISPT016 I/O error on PUT - aaaaaaaa
received I/O error indication from
common PUT.
Explanation
An I/O error occurred while trying to write a table
member.
System programmer response
Check the ISPTABL, ISPPROF and any other
appropriate table output libraries for these possible
causes:
• The table output library must not have concatenated
libraries.
• A real I/O error may have occurred. Try to browse
the table output library to see if an I/O error occurs
outside of table services.
• The table library allocation may have bad DCB
parameters. The table library must be a partitioned
data set with fixed block 80 byte records.
ISPF messages starting with ISP
272  z/OS: z/OS ISPF Messages and Codes

## Page 293

User response
Contact your system programmer.
ISPT017 I/O error on GET - aaaaaaaa
received I/O error indication from
common GET.
Explanation
An I/O error occurred while trying to read a table
member.
System programmer response
Check the ISPTLIB (or appropriate table input library)
for these possible causes:
• The table input library may have inconsistent DCB
information for the concatenated data sets.
• A real I/O error may have occurred. Try to browse
the table input library to see if an I/O error occurs
outside of table services.
User response
Contact your system programmer.
ISPT018 STOW error - aaaaaaaa received
decimal return code bbbbbbbb,
decimal reason code cccccccc from
the STOW macro.
Explanation
An error occurred on the STOW macro when updating
a partitioned data set directory or PDSE directory.
System programmer response
Refer to the appropriate system documentation on the
STOW macro for the return code and reason code
information.
User response
Contact your system programmer.
ISPT019 Internal table error - aaaaaaaa
received return code bbbbbbbb
from table delete.
Explanation
An error occurred while removing the DTB control
block from the active chain.
System programmer response
A possible storage overlay occurred. Examine any user
programs for possible storage overlay problems and
rerun the dialog. If the problem persists, contact IBM
support.
User response
Contact your system programmer.
ISPT020 Internal table error - Table
information record (TIR) cannot fit
into block.
Explanation
This message is self explanatory.
ISPT021 Parameter conflict - TBSKIP
service request specifies row
number and row ID.
Explanation
Mutually exclusive parameters for the TBSKIP service
were issued.
System programmer response
The parameter that causes the error is not an
external parameter. This error message should only be
generated as a result of an internal call that is in error.
Determine the steps necessary to recreate the failure
and contact IBM support.
User response
Contact your system programmer.
ISPT022 Unexpected EOF - Unexpected
end-of-file received while reading
table member.
Explanation
The end of file was reached unexpectedly, while
reading a table member. More records should have
been in the table.
System programmer response
The table is unusable and needs to be restored from a
backup.
User response
Contact your system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  273

## Page 294

ISPT023 WRITE/NOWRITE conflict -
TBOPEN for table aaaaaaaa
specifies conflicting WRITE/
NOWRITE option.
Explanation
A table was being opened with the SHARE parameter,
but the WRITE/NOWRITE option conflicted with the
option already in use by the other screen.
User response
Contact the responsible programmer.
Programmer response
If the SHARE parameter is used with TBOPEN, the
same WRITE/NOWRITE option must be used for each
screen that shares the table. Ensure that dialogs that
plan to share a table across split screens all use
WRITE or all use NOWRITE for the same shared table.
ISPT024 Error on input/output -
Unexpected return code aaaaaaaa
from CDG/CDP. RC=13 implies
disk full.
Explanation
An unexpected error occurred during I/O to a table
member.
System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPT025 Library format error - First table
record read does not have TIR: as
identifier.
Explanation
The table member being read is missing a required
header.
System programmer response
The table member is not usable. Restore the table
member from a backup copy.
User response
Contact your system programmer.
ISPT026 ESTAE error - aaaaaaaa received
return code bbbbbbbb from ESTAE.
Explanation
An error occurred on the ESTAE macro.
System programmer response
Refer to the appropriate system documentation for the
ESTAE macro to check the return code issued by the
ESTAE macro.
User response
Contact your system programmer.
ISPT027 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing table bbbbbbbb.
Explanation
A system abend occurred while processing a table.
System programmer response
If the abend code indicates insufficient space in the
table output data set, try either deleting unneeded
members or compress the data set or both. If
necessary, contact IBM support.
User response
Refer to your system documentation to discover
the cause of the problem, or contact your system
programmer.
ISPT028 Internal table error - aaaaaaaa
detected an invalid scan argument
work area.
Explanation
An error occurred while trying to clean up an internal
ISPF control block during the processing of a TBSAVE,
TBEND, TBCLOSE, or TBCREATE (with REPLACE)
service.
System programmer response
This is either a storage overlay problem caused by
a user program, or an internal ISPF problem. If
necessary, contact IBM support.
User response
Contact your system programmer.
ISPF messages starting with ISP
274  z/OS: z/OS ISPF Messages and Codes

## Page 295

ISPT029 Table not in write mode - TBSAVE
issued for table aaaaaaaa that is
not in write mode.
Explanation
A TBSAVE was issued, but the NOWRITE parameter
was used for the TBOPEN or TBCREATE that opened
the table.
User response
Contact the responsible programmer.
Programmer response
Ensure that a TBSAVE is not issued when the table is
not open in write mode.
ISPT030 Error opening aaaaaaaa - Error
occurred attempting to open input
table library.
Explanation
An error occurred during the open for the input table
library.
System programmer response
Check the table input library allocation for possible
problems with the allocation. Ensure that a valid DCB
is used if the code supplies its own DCB macro for
a library pointed to by the LIBRARY parameter on
the TBOPEN or TBCREATE, or pointed to by a LIBDEF
service.
User response
Contact your system programmer.
ISPT031 Error opening aaaaaaaa - Error
occurred attempting to open
output table library.
Explanation
An error occurred during the open for the table output
library.
System programmer response
Check the table output library allocation for possible
problems with the allocation. Check the ISPTABL
allocation for a valid DCB. If a LIBDEF is used,
check the DCB associated with the LIBDEF. Also, the
LIBRARY parameter could be used on the TBCREATE
or TBOPEN to point to a different DDNAME. If so, then
check that allocation.
User response
Contact your system programmer.
ISPT032 No scan arguments - No
arguments established for
TBSCAN of table aaaaaaaa.
Explanation
No scan argument has been established for the
TBSCAN service.
User response
Contact the responsible programmer.
Programmer response
Ensure that either a TBSARG has been issued for the
table, or that an ARGLIST parameter is used with
the TBSCAN service. The ARGLIST parameter on the
TBSCAN service is optional, but if it is omitted then an
argument must have been established by a previous
TBSARG service.
ISPT033 Table already open - aaaaaaaa
issued for table bbbbbbbb that is
already open.
Explanation
An attempt was made to open a table that is already
open.
User response
Contact the responsible programmer.
Programmer response
Logic may be missing to close a table in error
conditions. If the open error is expected then use
CONTROL ERRORS RETURN to mask the error from the
user.
ISPT034 Table is not open - aaaaaaaa
issued for table bbbbbbbb that is
not open.
Explanation
A table service was issued for a table that was not
open.
User response
Contact the responsible programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  275

## Page 296

Programmer response
Verify that the table is open before any table services
are issued.
ISPT035 Library not allocated - aaaaaaaa
library is not allocated.
Explanation
A LIBRARY parameter was supplied for a table service
such as TBOPEN, but the referenced ddname was not
allocated.
User response
Contact the responsible programmer.
Programmer response
Ensure that any ddnames referenced by the LIBRARY
parameter on a table service have been allocated.
ISPT036 Table in use - aaaaaaaa issued
for table bbbbbbbb that is in use,
ENQUEUE failed.
Explanation
The table has an existing enqueue against it already.
Another user may be updating the table at the same
time.
System programmer response
The RNAME used for the enqueue is the first data set
name in the ISPTLIB concatenation sequence padded
to 44 characters with blanks, followed by the table
name for 8 characters padded with blanks. Since the
enqueue is done on the input library at open time, it
may be desirable to put the data set pointed to by
the table output library (normally ISPTABL) as the first
data set in the ISPTLIB concatenation sequence.
User response
Try again later or contact your system programmer.
ISPT037 Library format error -
Invalid record: Table=aaaaaaaa
TTRN=bbbbbbbb Offset=cccccccc.
Explanation
An invalid record was detected while reading the table.
System programmer response
The table is not usable. Try to restore the table
member from a backup copy. If necessary, contact
IBM support.
User response
Contact your system programmer.
ISPT038 Table structure error - aaaaaaaa
for table bbbbbbbb detected an
invalid internal table structure.
Explanation
An invalid table structure was detected by ISPF.
System programmer response
Restore the table member from a backup copy if
necessary. (See programmer response.)
User response
Contact the responsible programmer.
Programmer response
A TBCREATE that specifies the same field in both
the KEY and NAMES parameters will cause this error.
Ensure that all variable names are specified for either
the KEY or NAMES parameter, but not both. If this
does not resolve the problem, the table is not usable
and should be restored from a backup copy.
ISPT039 PAD value invalid - PAD value
specified on aaaaaaaa command
is invalid.
Explanation
The value specified in the PAD parameter is invalid.
User response
Contact the responsible programmer.
Programmer response
This error is probably caused by a bad value in the
storage pointed to by the TBCLOSE parameters. The
PAD parameter on a program call should be a fullword
fixed binary integer.
ISPT040 File not found. - File "aaaaaaaa"
was not found
ISPF messages starting with ISP
276  z/OS: z/OS ISPF Messages and Codes

## Page 297

Explanation
This message is self explanatory.
ISPT041 File not accessed. - File
"aaaaaaaa" is not accessed
Explanation
This message is self explanatory.
ISPT042 FSSTATE error - Return code
aaaaaaaa received from FSSTATE.
Explanation
This message is self explanatory.
ISPT043 VMSPF error - Code x'aaaaaaaa'
from VMSPF virtual machine for
file bbbbbbbb.
Explanation
This message is self explanatory.
ISPT044 ENQUEUE error - Error occurred
on ENQUEUE for file 'aaaaaaaa' -
rc=bbbbbbbb.
Explanation
This message is self explanatory.
ISPT045 DEQUEUE error - Error occurred
on DEQUEUE for file 'aaaaaaaa' -
rc=bbbbbbbb.
Explanation
This message is self explanatory.
ISPT046 Filemode not supported -
Filemode of '*' is not supported for
output file "aaaaaaaa."
Explanation
This message is self explanatory.
ISPT047 Library allocation error - Error
occurred during the allocation of
library aaaaaaaa.
Explanation
This message is self explanatory.
ISPT048 Read-only disk - Write failed for
aaaaaaaa, disk read only.
Explanation
This message is self explanatory.
ISPT050 Table panel unavailable - Unable
to process reinvocation. Enter
HELP for more information.
Explanation
The TBDISPL service was called without the panel
name and the request could not be processed.
User response
Contact the responsible programmer.
Programmer response
The TBDISPL request could not be processed for one
of these reasons:
• TBDISPL has never been called with the panel-name
parameter specified during the session for this
application. To correct this, make sure that the first
call to TBDISPL in the application uses the panel
name.
• The last panel displayed was by a service other
than TBDISPL and the CONTROL service was not
used to save and restore the panel environment.
The CONTROL service should be used to save
and restore the panel environment when nesting
display requests under the TBDISPL command. Use
of the ADDPOP service performs the equivalent
of a CONTROL DISPLAY SAVE prior to creating
the pop-up window. The REMPOP service performs
the equivalent of a CONTROL DISPLAY RESTORE
after removing the current pop-up window. A dialog
should not issue its own CONTROL DISPLAY SAVE/
RESTORE around an ADDPOP/REMPOP sequence.
ISPT051 Panel "aaaaaaaa" error - The body
of a TBDISPL panel must have a
command field.
Explanation
No command field was coded on the TBDISPL panel.
User response
Contact the responsible programmer.
Programmer response
Ensure that the TBDISPL panel has a command field
coded in the )BODY section of the panel.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  277

## Page 298

ISPT052 Panel "aaaaaaaa" error - The
model line(s) must contain 1 or
more input/output fields.
Explanation
No input or output fields were defined in the )model
section.
User response
Contact the responsible programmer.
Programmer response
Correct the TBDISPL panel in question to have at least
1 input/output field defined in the model line(s).
ISPT053 Invalid VARS value -
Vars="aaaaaaaa" is invalid or
missing.
Explanation
The TBDISPL panel has an invalid or missing VARS
variable. If Z variables are used as name placeholders
in the model line(s), a .ZVARS or VARS variable must
be defined so that the real variable names can be
associated with the name placeholders. If neither
variable is defined, this error occurs. This error can
also occur if the VARS variable is null or has no names
within a name list.
User response
Contact the responsible programmer.
Programmer response
Correct the TBDISPL panel to use .ZVARS to define
the name list associated with the Z variables defined.
VARS can be used due to compatibility with older
releases of ISPF, but .ZVARS is recommended.
ISPT054 Invalid VARS value - A "Z" variable
replacement name within is a
duplicate field name.
Explanation
A duplicate name exists within the name list for the
VARS variable value.
User response
Contact the responsible programmer.
Programmer response
Correct the TBDISPL panel so it does not specify
duplicate names within the name list for the VARS
variable value. If possible, use the .ZVARS control
variable instead of the VARS variable.
ISPT056 Invalid VARS value - The # of field
names does not match # of "Z"
in/out fields on model line(s)
Explanation
The number of names in the name list for the
VARS variable value does not match the number
of Z variables. There should be a one to one
correspondence between the field names in the VARS
variable value and the Z variable placeholders.
User response
Contact the responsible programmer.
Programmer response
Correct the TBDISPL panel so that the VARS
variable value match the Z variables. If possible,
use the .ZVARS control variable instead of the VARS
variable.
ISPT057 Panel "aaaaaaaa" error - Unable
to fit at least 1 table entry on
screen, due to panel format.
Explanation
This message is self explanatory.
ISPT058 Panel "aaaaaaaa" error - Table
display panel must have ")MODEL"
and 1-8 model lines
Explanation
The TBDISPL panel is either missing model lines or has
more than 8 model lines specified.
User response
Contact the responsible programmer.
Programmer response
Correct the TBDISPL panel to have at least 1 and no
more than 8 model lines.
ISPT060 Table display loop - No more
selections to process (End key
pressed or panel never given)
ISPF messages starting with ISP
278  z/OS: z/OS ISPF Messages and Codes

## Page 299

Explanation
A TBDISPL service was issued with no panel name, but
no selections remained to be processed.
User response
Contact the responsible programmer.
Programmer response
This error is a protection against a loop and should not
normally occur. Check the dialog to see if CONTROL
ERRORS RETURN is active. If so, check all error
conditions that might cause this loop.
Problem determination
Dialog Test can be used to set breakpoints on the
TBDISPL as well as any services within the TBDISPL
loop. The return codes can then be checked from the
Breakpoint panel. Also, the Variables selection can be
chosen from the Breakpoint panel to examine error
conditions for return codes of 12 and higher. Variables
ZERRMSG, ZERRSM, and ZERRLM would be the main
variables to examine for error information.
ISPT061 Table "aaaaaaaa" error - Attempt
to process a table row that no
longer exists.
Explanation
The row selected for processing in a TBDISPL panel
no longer exists. This could happen while processing
a table display with multiple rows selected. An error
could occur on a selected row which causes the table
display panel to be redisplayed before all the selected
rows have been processed. The display will reflect the
table as it was prior to any updates. If a user reselects
a row that was deleted, this error will occur.
User response
Do not try to reprocess a deleted row.
ISPT062 Table display error - Scroll return
variable ZTDSCRP does not exist
in the function pool.
Explanation
Scrolling was attempted, but variable ZTDSCRP does
not exist in the function pool.
User response
Contact the responsible programmer.
Programmer response
Make sure that variable ZTDSCRP is defined in the
function pool if the dialog uses dynamic table display
processing.
ISPT063 Table display error - The value
of scroll return variable ZTDSCRP,
'aaaaaaaa', is invalid.
Explanation
The value of variable ZTDSCRP is invalid. This variable
must be numeric.
User response
Contact the responsible programmer.
Programmer response
Ensure that a numeric value is in variable ZTDSCRP. A
length of zero or a value of all blanks is invalid.
ISPT064 DMSCSL invocation error -
aaaaaaaa received csl return code
'bbbbbbbb' from cdsn.
Explanation
This message is self explanatory.
ISPT065 DMSCSL invocation error -
aaaaaaaa received csl return code
'bbbbbbbb' from CFI.
Explanation
This message is self explanatory.
ISPT066 Invalid parameter - AUTOSEL
parameter was not YES, NO, or
blank. It was "aaaaaaaa".
Explanation
An invalid AUTOSEL value was specified on the
TBDISPL service.
User response
Contact the responsible programmer.
Programmer response
Ensure that the AUTOSEL parameter has a value of
YES, NO, or blank. There should also be at least 1
trailing blank after the YES or NO value.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  279

## Page 300

ISPT067 SFS directory full - SFS directory's
space limit exceeded a rollback
has occurred.
Explanation
This message is self explanatory.
ISPT068 No authority for file - Not
authorized to write to file on
directory accessed as "aaaaaaaa"
Explanation
This message is self explanatory.
ISPT070 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing table bbbbbbbb.. The
table output library cccccccc is
full.
Explanation
A system abend D37 occurred while processing the
indicated table.
Programmer response
Increase the size of the output table library and
compress if necessary. If the abend continues to
occur, check for a user error, such as a loop writing
to the table output library.
Problem determination
In the case of a possible user error (such as a loop),
use Dialog Test to trace the table services such as
TBADD, TBOPEN, and TBCLOSE. The dialog could be in
a loop on these services causing the output library to
grow larger until a space abend terminates the loop.
Also, ensure that the TSO profile is set to WTPMSG and
MSGID so that the system abend messages are seen
for this error and additional information can then be
obtained from system documentation.
ISPT071 System abend aaaaaaaa - Abend
aaaaaaaa encountered while
processing table bbbbbbbb.. The
table output library cccccccc or the
table output volume is full.
Explanation
A system abend B37 or E37 was encountered while
processing the table output library.
System programmer response
Point to a different pack, or make space available on
the existing pack if the pack is currently full.
Programmer response
Refer to system documentation on these abend codes.
Make sure the dialog is not in a loop causing the out
of space condition. The data set may be out of extents
and need a larger allocation. Also the pack may be full,
leaving no room to write additional records.
Problem determination
In the case of a possible user error (such as a loop),
use Dialog Test to trace the table services, such as
TBADD, TBOPEN, and TBCLOSE. The dialog could be in
a loop on these services causing the output library to
grow larger until a space abend terminates the loop.
Also, ensure that the TSO profile is set to WTPMSG
and MSGID so that the system abend messages are
seen for this error. Additional information can then be
obtained from system documentation.
ISPT072 Directory full - aaaaaaaa received
return code bbbbbbbb from STOW.
The table output library cccccccc
directory is full.
Explanation
The table output library cccccccc directory is full.
Programmer response
Increase the number of directory blocks for the table
output library.
Problem determination
If the problem persists after increasing the number
of directory blocks, use Dialog Test to trace TBOPEN,
TBCREATE, TBSAVE, and TBCLOSE. A loop may be
occurring on these services causing the directory full
condition.
ISPT073 Cannot process table - The table
cannot be processed by the ISPF
Table Utility because it contains
a variable named aaaaaaaa..
Variable names with a prefix of
'ZTD' or 'ZTB' are reserved for use
by the ISPF table processing and
the ISPF Table Utility.
ISPF messages starting with ISP
280  z/OS: z/OS ISPF Messages and Codes

## Page 301

Explanation
The table variables with names commencing with
'ZTD' or 'ZTB' could conflict with variables used
internally by the ISPF Table Utility. Attempting to
use the ISPF Table Utility to process the table could
cause either the utility to fail or the table data to be
corrupted.
Programmer response
Rename any table variables with a name commencing
with 'ZTD' or 'ZTB'.
ISPT074 Cannot process table - The table
cannot be processed by the ISPF
Table Utility because it contains
an extension variable named
aaaaaaaa. in row bbbbbbbb..
Variable names with a prefix of
'ZTD' or 'ZTB' are reserved for use
by the ISPF table processing and
the ISPF Table Utility.
Explanation
The table variables with names commencing with
'ZTD' or 'ZTB' could conflict with variables used
internally by the ISPF Table Utility. Attempting to
use the ISPF Table Utility to process the table could
cause either the utility to fail or the table data to be
corrupted.
Programmer response
Rename any table variables with a name commencing
with 'ZTB'.
ISPT080 aaaaaaaa Conflict -
bbbbbbbb(cccccccc) - Table
processing conflict.
Explanation
An conflict has been detected in an ISPF profile table
that has been updated since it was last accessed by
the ISPF session. ISPF configuration options define
the default action to be taken when a conflict is
detected. The action can be Keep, Delete, or Prompt.
ISPT081 aaaaaaaa - Table last accessed
bbbbbbbb cccccccc., last modified
dddddddd eeeeeeee..
Explanation
This message is written to the ISPF log data set in
conjunction with message ISPT080. Refer to message
ISPT080 for an explanation.
ISPT082E Edit Profile
ISPT082I ISPF Profile
ISPT082O Other Profile Table
ISPT082P Application Profile
ISPT082R Reference List
ISPT082S System Profile
ISPT083 Data set in use - Data set
'aaaaaaaa' in use by another user,
try later or enter HELP for a list
of jobs and users allocated to
'aaaaaaaa'.
Explanation
ISPF received a non zero return code attempting to
enqueue the data set aaaaaaaa indicating the data
set is currently allocated to another job or user and is,
therefore, unavailable to satisfy your request.
User response
If the data set is in use by another job, wait for that
job to finish, or have the submitter of the job cancel
it, then retry your request. If the data set is in use
by another user, have that user free it, then retry the
request.
ISPT101 Storage release error - Error
encountered during storage
release by TBSORT service.
Explanation
An error occurred on the FREEMAIN macro trying to
release storage obtained by TBSORT.
System programmer response
Contact IBM support.
ISPT102 No storage available - Insufficient
storage available to perform
TBSORT service.
Explanation
The GETMAIN macro could not obtain the necessary
storage to perform the TBSORT.
System programmer response
Ensure that the user's region is sufficient to run the
failing dialog. If the error continues, contact IBM
support.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  281

## Page 302

User response
Contact your system programmer.
ISPT103 Invalid sort key - The name
'aaaaaaaa' is not a column name
of this table.
Explanation
The specified name is not a column name within the
table being sorted.
Programmer response
Specify a field name that is either a NAME or KEY field
in the table.
ISPT104 Invalid direction code - The given
sort direction code of 'aaaaaaaa'
must be an A or a D.
Explanation
The sort direction code must be either A (ascending) or
D (descending).
Programmer response
Supply an A or D for the sort direction code.
ISPT105 Invalid sort type 'aaaaaaaa' - The
valid table sort types are B, C, N,
or Y.
Explanation
Only B for binary, C for character, Y for year, owed or N
for numeric are all as valid sort types.
Programmer response
Enter B, C, N, or Y for the sort type.
ISPT106 Value too large - The sort field
'aaaaaaaa' was too large to
convert to binary.
Explanation
The sort field was too large for a numeric type sort.
The largest value that can be sorted with type numeric
is plus or minus 2 147 483 647.
Programmer response
Ensure that none of the variable values being
sorted with type numeric exceed plus or minus
2 147 483 647.
ISPT107 Invalid sort digit - The digit
'aaaaaaaa' found in the sort field
cannot be converted.
Explanation
A numeric type sort was specified and a value was
found that was not numeric.
Programmer response
Ensure that all variable values for the field being
sorted are valid numeric (0-9) values if a numeric type
sort is specified.
ISPT108 Sort value too large - A string
greater than 16 characters is too
long to convert.
Explanation
The numeric value being sorted cannot exceed a total
of 16 characters. This length includes any plus or
minus signs, blanks, or decimal points.
Programmer response
Ensure that no variable values for the field being
sorted exceed a total of 16 characters if a numeric
type sort is specified.
ISPT109 Invalid scan operator - The given
operator 'aaaaaaaa' is not a valid
scan operator.
Explanation
An invalid operator was specified in the condition
value list for the CONDLIST parameter of the TBSCAN
service.
Programmer response
Ensure that a valid operator is used in the condition
value list. The valid operators are EQ, NE, LE, LT, GE,
and GT.
ISPT200 Invalid name-op pair - The given
name 'aaaaaaaa' is not a valid
search argument.
Explanation
The given name does not match the name of a key
field, name field, or name of an extension variable for
the table.
ISPF messages starting with ISP
282  z/OS: z/OS ISPF Messages and Codes

## Page 303

Programmer response
Specify a valid name in the TBSARG NAMECOND
parameter.
ISPT201 Too many operators - More
operators than arguments were
specified
Explanation
The condition-value-list for the CONDLIST parameter
has more operators than there are names in the name-
list of the ARGLIST parameter.
Programmer response
Compare the names in the ARGLIST parameter to the
operators in the CONDLIST parameter. The CONDLIST
operators correspond one-to-one with the names in
the ARGLIST parameter. Extra operators cause the
error. If there are fewer values in the CONDLIST than
there are names, EQ is used as the default for the
remaining names in the ARGLIST.
ISPT202 Invalid number of rows - The
number of rows must be a positive
number less than 32768
Explanation
The number of rows specified in the MULT parameter
of the TBADD service is greater than or equal to
32 768.
Programmer response
Specify a number for the MULT parameter that is less
than 32 768.
ISPT203 Table row too long - aaaaaaaa
was requested for a row bbbbbbbb
bytes long. Maximum length is
65536.
Explanation
The requested table service request would cause the
table row to exceed 65 536 bytes.
Programmer response
Decrease the size of the table row.
Problem determination
The length of a table row can be computed as follows:
Row size = 22 + 4a + b + 9c
where:
a
Total number of variables in the row, including
extensions
b
Total length of variable data in the row
c
Total number of extension variables in the row
ISPT204 Invalid LIBDEF usage - A LIBDEF
definition for ISPTABL cannot be
used for table input processing
Explanation
A TBOPEN or TBCREATE was issued that used a
LIBDEF definition for ISPTABL. ISPTABL is intended for
output only.
Programmer response
Do not use the ISPTABL LIBDEF definition for TBOPEN
or TBCREATE. Use LIBDEF for ISPTLIB instead.
ISPT205 Invalid LIBDEF usage - A LIBDEF
definition for ISPTLIB cannot be
used for table output processing
Explanation
A TBSAVE, TBERASE, or TBCLOSE was issued that
used a LIBDEF definition for ISPTLIB. ISPTLIB is
intended for input only.
Programmer response
Do not use the ISPTLIB libdef definition for table
output processing. Use LIBDEF for ISPTABL instead.
ISPT206 File recall failed - File 'aaaaaaaa'
is migrated and implicit recall
failed.
Explanation
An FSREAD was issued for a table member in VM that
resulted in a return code of 50 or 51. These return
codes indicate that a file is migrated and recall failed.
ISPT207 Invalid year offset - The year
offset must be a value from 1 to
7.
Explanation
The year sort only accepts a variable value with a
maximum value of 8 characters where part of the
value is a 2 digit year. Hence the offset to the
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  283

## Page 304

beginning of the year can only be a value from 1 to
7.
Programmer response
Specify a valid offset for the year parameter.
ISPU330 Invalid command table - The
command table has keys, and
therefore cannot be processed.
Explanation
The command table being read by the Command Table
utility has variables defined as keys.
System programmer response
Check the ISPTLIB concatenation for the table being
read. The table name is xxxxCMDS, where xxxx is the
application ID entered by the user.
ISPF command tables should not have keys. This
condition could occur if users created their own
command tables with the TBCREATE service. The
Display Status selection of Tables under Dialog Test
can be used to get status information on the table,
which may be helpful in determining how the invalid
table was created.
ISPU331 Input command error - Command
entered is not recognized.
Explanation
An invalid command was entered.
User response
Enter a valid command, or erase the invalid command
from the command line.
ISPU332 Invalid appl ID - Application IDs
ISP and ISR are not allowed.
Explanation
This message is self explanatory.
ISPU333 Appl ID too long - Application IDs
have a maximum length of four
characters.
Explanation
A user cannot enter more than 4 characters for the
application ID field.
System programmer response
This error requires a user to have modified the
supplied panel, ISPUCMA. The supplied panel has only
4 physical positions available. Check the panel source
for ISPUCMA to see how the user could be entering
more than 4 characters.
User response
Enter no more than 4 characters for the application ID.
ISPU334 Invalid line command - D, I, R,
and E are the only valid line
commands.
Explanation
An invalid line command was entered.
User response
Either correct the invalid line command, or blank it out.
ISPU335 Starter line substituted - To delete
a table, delete all lines and enter
END command.
Explanation
This message is issued if all lines in a command table
are deleted. A dummy line is added to allow input if
more commands are to be entered.
User response
Enter END to delete the table, or add new entries as
needed.
ISPU336 Verb too short - Command table
verbs must be at least two
characters long.
Explanation
The command verb must be from 2 to 8 characters
long, and must begin with an alphabetic character.
User response
Enter a valid name for the verb.
ISPU337 Invalid verb - Command table
verbs must begin with an
alphabetic character.
Explanation
The command verb must be from 2 to 8 characters
long, and must begin with an alphabetic character.
ISPF messages starting with ISP
284  z/OS: z/OS ISPF Messages and Codes

## Page 305

User response
Enter a valid name for the verb.
ISPU338 Invalid T or Trunc value -
Command table truncation values
must be within the length of the
verb.
Explanation
A truncation value was specified that is greater than
the length of the command verb.
User response
Use a truncation value that is less than or equal to the
verb length.
ISPU339 Invalid action field - Incorrect
data entered for the action field.
Explanation
An invalid ACTION value was specified for the
command verb.
User response
Enter a valid ACTION value. See z/OS ISPF Dialog
Developer's Guide and Reference for information on the
valid actions allowed.
ISPU339A Severe error - A severe error
occurred on the TBOPEN for the
command table.
Explanation
A severe error occurred while processing the TBOPEN
for the command table.
System programmer response
Check the ISPTLIB allocation for a possible allocation
error.
User response
Contact your system programmer.
ISPU340 Invalid appl ID - Enter up to 4
alphanumeric characters. The first
must be alphabetic.
Explanation
An invalid value was entered for the application ID
field. From 1 to 4 characters can be entered with
the first position being alphabetic and the remaining
positions alphanumeric.
User response
Enter a valid value.
ISPU341 Table in use - The command table
specified is already open and
cannot be opened again.
Explanation
You have specified a command table that has already
been specified for the Command Table Utility, and you
have not exited from the utility yet. You may have
specified the command table from another screen, or
you may have hidden the utility from view by going to
another function using the action bars.
User response
Close the other invocation of the Command Table
Utility for that command table.
ISPU342 Invalid line command - V is the
only valid line command.
Explanation
An invalid line command was entered. D, I, E, and R
are only valid on a table that can be updated.
User response
Either correct the invalid line command, or blank it out.
ISPU343 Entry updated - The change to
command was updated, but has
not been saved in the command
table yet. You must exit (save)
from the Command Table Utility to
make the change permanent.
Explanation
An entry in the command table was changed. It will be
made permanent if the user exits from the Command
Table Utility or will be ignored if the user cancels from
the Command Table Utility.
ISPU344 Entry unchanged - The change to
command was ignored.
Explanation
An entry in the command table made, but canceled.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  285

## Page 306

ISPV000 Invalid variable name - Variable
'aaaaaaaa' in list 'bbbbbbbb' is
syntactically incorrect.
Explanation
The dialog specified an invalid variable name.
User response
Contact the responsible programmer.
Programmer response
Ensure that proper ISPF naming conventions are
followed for variable names. A dialog variable name
is composed of 1 to 8 characters (6, for FORTRAN).
Alphanumeric characters A-Z, 0-9, #, $, or @ can be
used in the name, but the first character cannot be
numeric. APL variable names cannot contain #, $, or
@.
ISPV001 Variable not modifiable - Variable
'aaaaaaaa' is reserved by the
system.
Explanation
An attempt was made to update a non-modifiable
variable.
User response
Contact the responsible programmer.
Programmer response
Correct the dialog so it does not update the reserved
ISPF variable.
ISPV002 Insufficient storage - Not enough
main storage was available to
satisfy request.
Explanation
Variable services needed more storage to process
variable requests, but the system was unable to obtain
more storage.
User response
Log on with a larger region size.
Programmer response
Make sure that any user programs are managing
storage properly. Check VDEFINE requests for
incorrect length values. Also, a loop on VDEFINE
requests without a corresponding VDELETE could
cause storage to grow.
ISPV003 Invalid variable blocks - Pool
initialization must be matched
with pool termination.
Explanation
This message is self explanatory.
ISPV004 Descriptor already used - Variable
'aaaaaaaa' in list 'bbbbbbbb' is
already defined.
Explanation
An invalid variable control block was encountered.
The variable control block chain had a duplicate entry
which would have caused a loop.
User response
Contact the responsible programmer.
Programmer response
Check the VDEFINE in error for possible coding
problems. Also, ensure that the use of Control Errors
Return did not allow a previous error condition to be
improperly handled or to go undetected. Use Dialog
Test to trace ISPF services and set breakpoints on the
appropriate services, such as VDEFINE and CONTROL.
For further help, contact IBM support.
ISPV005 Destructive move - Source and
target overlap destructively. No
data was moved.
Explanation
An update of a variable would cause a destructive
move. The target data overlaps the source data.
User response
Contact the responsible programmer.
Programmer response
Check for a VDEFINE with an incorrect length or an
incorrect variable storage pointer. Use Dialog Test to
trace the VDEFINE service.
ISPV006 Data truncation occurred - Data
for aaaaaaaa format variable
"bbbbbbbb" was too long.
ISPF messages starting with ISP
286  z/OS: z/OS ISPF Messages and Codes

## Page 307

Explanation
The variable value being updated is longer than the
length specified on the VDEFINE for the variable in
question.
User response
Contact the responsible programmer.
Programmer response
Ensure that the correct length was specified on
the VDEFINE service. The length should have been
specified as a fullword binary value. If this variable
is being updated from a panel, make sure that the
field length on the panel is not larger than the length
specified on the VDEFINE service. Dialog Test can be
used to trace the variable and the VDEFINE service.
ISPV007 Invalid data characters - Data
for aaaaaaaa format variable
"bbbbbbbb" was not aaaaaaaa
characters.
Explanation
Invalid data was supplied for the variable being
updated. The data was not valid for the format
specified on the VDEFINE for the variable. For
example, a value other than 0 or 1 was supplied for a
variable with a format of BIT specified on the VDEFINE
service.
User response
Contact the responsible programmer.
Programmer response
Ensure that the data being used to update a variable is
valid for the format specified on the VDEFINE for that
variable.
ISPV008 Invalid data length - Length of
"aaaaaaaa" is bbbbbbbb - valid
lengths are 0 to 32767.
Explanation
An invalid length was encountered on a VREPLACE or
VCOPY service. The length must be between 0 and
32 767.
User response
Contact the responsible programmer.
Programmer response
Ensure that a valid length is specified on the VCOPY
or VREPLACE service. Use Dialog Test to trace or set
breakpoints on the VCOPY and VREPLACE service.
ISPV009 Invalid variable name - Variable
'aaaaaaaa' is syntactically
incorrect.
Explanation
The dialog specified an invalid variable name.
User response
Contact the responsible programmer.
Programmer response
Ensure that correct ISPF naming conventions are
followed for variable names. A dialog variable name
is composed of 1 to 8 characters (6, for FORTRAN).
Alphanumeric characters A-Z, 0-9, #, $, or @ can be
used in the name, but the first character cannot be
numeric. APL variable names cannot contain #, $, or
@.
ISPV010 Profile not loaded - Profile table
'aaaaaaaa' not read. Table service
RC=bbbbbbbb.
Explanation
An error occurred while trying to open a profile table
for a new application.
System programmer response
Check out the ISPTLIB allocations for possible
problems. Possible return codes are:
8
The profile table was not found. Check the ISPTLIB
concatenation for allocation errors. ISPTLIB will
be used for default profile tables when the profile
table is not found in ISPPROF.
12
An enqueue error occurred. Check the ISPPROF
and ISPTLIB allocations for contention problems.
20
A severe error was encountered. The user may
have a bad profile table. Restore the table or
delete the profile in question and retry.
User response
Contact your system programmer.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  287

## Page 308

ISPV011 Profile table not found - Unable
to access aaaaaaaa table for
variable bbbbbbbb.
Explanation
The variable cannot be accessed because the profile
table is not found.
System programmer response
The active profile table can no longer be located.
Check to make sure there are no prior errors causing
the profile table to be closed prematurely. Check ISPF
log for additional messages. Use dialog test to trace all
ISPF services to get an idea of where the problem may
be.
User response
Contact your system programmer.
ISPV012 Invalid VDEFINE keyword -
'aaaaaaaa' is not valid. Use COPY,
NOBSCAN, or LIST.
Explanation
An options-list parameter other than COPY, NOBSCAN,
or LIST was specified.
Programmer response
Specify a valid options-list parameter.
ISPV013 Variable exit error - Exit
routine at aaaaaaaa gave error
code bbbbbbbb for cccccccc of
dddddddd.
Explanation
A variable service exit encountered an unexpected
return code.
System programmer response
Check to see if a user defined exit is active for the
VDEFINE service. If so, contact the owner of the exit
for additional information. Dialog Test can be used to
trace or breakpoint the VDEFINE service to see if a
VDEFINE exit has been specified. If the variable exit is
an ISPF exit, contact IBM support.
User response
Contact your system programmer.
ISPV014 Required parm missing - User-
data block is required with USER
format VDEFINEs.
Explanation
A VDEFINE format of USER was specified, but no user-
data value was supplied.
Programmer response
Correct the VDEFINE service in error.
ISPV015 Profile full - Profile variable
'aaaaaaaa' not saved. 'bbbbbbbb'
table is full.
Explanation
The profile variable cannot be added because it will
cause the profile table row to exceed 64K.
System programmer response
Make sure the variable being added does not have
a length definition problem. The table could be fine,
but the variable being added is too large. If the table
is very full, the VERASE service should be used to
clean up the profile by deleting variables that are not
needed. Use dialog trace to trace the variable being
added. An incorrect length on a VDEFINE or a loop on
VPUT could cause this.
User response
Contact your system programmer.
ISPV016 R/O profile not loaded - R/O profile
table 'aaaaaaaa' not read. Table
service RC=bbbbbbbb.
Explanation
Variable ZPROFAPP specified a read-only extension
profile existed; however, the R/O table could not be
read.
System programmer response
Check the ISPTLIB allocation. These return codes may
apply:
8
The table is not found. Either variable ZPROFAPP
has a bad value or the ISPTLIB allocation does not
point to the data set that contains this profile table.
20
A severe error occurred. The table may need to be
restored from a backup copy.
ISPF messages starting with ISP
288  z/OS: z/OS ISPF Messages and Codes

## Page 309

User response
Contact your system programmer.
ISPV017 Invalid format - Invalid VDEFINE
format 'aaaaaaaa' found for
variable 'bbbbbbbb'.
Explanation
The format parameter supplied on the VDEFINE
service was not a valid value.
User response
Contact the responsible programmer.
Programmer response
Use a valid format with the VDEFINE service.
ISPV020 Variable access invalid - A valid
CLIST environment does not exist.
Explanation
ISPF tried to access a variable through TSO, but a
CLIST environment did not exist.
System programmer response
Check for any code that could affect the ECT control
block. Contact IBM support, if necessary.
User response
Contact the responsible programmer.
Programmer response
This error is a result of a zero ECTIOWA value in
the ECT. Check the dialog for any code that could
affect the ECT control block. Contact the system
programmer, if necessary.
ISPV021 Invalid aaaaaaaa request -
Variable name 'bbbbbbbb' is not a
valid REXX variable name.
Explanation
An invalid variable name was passed to IKJCT441 to
be processed as a result of an ISPF service request
while in a REXX environment.
User response
Contact the responsible programmer.
Programmer response
Check the variable in question for valid syntax. Use
the REXX trace command or Dialog Test to trace ISPF
services.
ISPV022 Invalid variable name - Variable
name 'aaaaaaaa' is syntactically
incorrect.
Explanation
This message is self explanatory.
ISPV023 Data has been truncated - The
maximum length of CLIST variable
data is 32767 characters.
Explanation
ISPF tried to retrieve a CLIST variable using IKJCT441
but the length of the variable was greater than 32 767.
User response
Contact the responsible programmer.
Programmer response
Ensure that CLIST and REXX variables do not exceed
32 767 bytes in length.
ISPV024 Invalid SET request - CLIST
variable 'aaaaaaaa' is a system
variable. Not modifiable.
Explanation
ISPF was used to attempt to modify a TSO system
variable.
User response
Contact the responsible programmer.
Programmer response
Do not use ISPF services to modify TSO system
variables.
ISPV025 Invalid SET request - CLIST
variable 'aaaaaaaa' not updated
because it is a label element.
Explanation
ISPF attempted to update a variable that is a label
element. This is not allowed through the IKJCT441
interface.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  289

## Page 310

User response
Contact the responsible programmer.
Programmer response
Do not code variables that are label elements as
variables that can be updated by ISPF. For example,
do not use a variable that is a CLIST label element as
an input field in an ISPF panel.
ISPV026 Invalid GET request - CLIST
variable 'aaaaaaaa' requires
evaluation. Not supported.
Explanation
The variable is a CLIST built-in function, such as &STR,
that requires evaluation.
User response
Contact the responsible programmer.
Programmer response
Do not use CLIST built-in function variables within an
ISPF service call such as VGET.
ISPV027 Invalid GET request - CLIST
variable 'aaaaaaaa' not retrieved
because it is a label element.
Explanation
The variable is a CLIST label and was not retrieved.
User response
Contact the responsible programmer.
Programmer response
Do not use CLIST variables defined as labels as ISPF
variables as well.
ISPV028 IKJCT441 interface error - The
caller is not in a CLIST or REXX
environment. Return code = 40.
This could be a result of not having
the function pool flag set in the
ISPTCM table for the command
being executed.
Explanation
ISPF called IKJCT441 to process a CLIST/REXX
variable and got a return code of 40. This normally
means that there was an invalid environment.
System programmer response
The probable cause of this error is that a command
was invoked with the SELECT service and the
command was in the ISPTCM table with a flag byte
X'02'instead of X'42'. Any command processor that is
in the ISPTCM table that uses ISPF services should
have the function pool flag set.
User response
Contact your system programmer.
ISPV029 IKJCT441 interface error - Severe
error accessing CLIST variables.
Return code = 'aaaaaaaa'.
Explanation
A severe error occurred while processing CLIST
variables. Refer to the appropriate TSO documentation
for an explanation of the IKJCT441 error code.
System programmer response
Contact IBM support, if necessary.
ISPV030 Too Many Symbol Names - VGET
service specifies more symbol
names than dialog variables.
Explanation
There are more symbol names than Dialog variables
specified for the VGET SYMDEF service.
User response
Contact the responsible programmer.
Programmer response
Correct the number of symbol names and dialog
variables specified for the VGET service.
ISPV040 Invalid PACK(N) - N value of
PACK(N) must be between 0 and
18.
Explanation
The value of N must be greater than or equal to 0 and
less than or equal to 18.
Programmer response
Ensure a valid value is specified with the PACK
keyword.
ISPF messages starting with ISP
290  z/OS: z/OS ISPF Messages and Codes

## Page 311

ISPV041 Incompatible pack length -
PACK(N) format specified requires
a length greater than was
specified.
Explanation
The length specified on the VDEFINE service was not
large enough to to contain the number of digits to the
right of the decimal point as specified by the N value.
Programmer response
Ensure that the length specified with the VDEFINE
service is large enough to contain the packed number.
The value of N cannot be greater than twice the length
minus one.
ISPV042 Invalid VDEFINE syntax - A format
of "*" must have an "*" in the
corresponding name position in
the VDEFINE parameters.
Explanation
Using the LIST option, an "*" (asterisk) was found in
the format field array and the corresponding entry in
the variable name array was not an "*" (asterisk) also.
Programmer response
Make sure that the format field array and name list
array are paired properly.
ISPV043 Invalid VDEFINE syntax - An "*"
format must be specified with at
least one other valid format and
the list option specified.
Explanation
A value of "*" (asterisk) is only valid with the LIST
option. Also, there must be at least one valid format
other than "*" (asterisk) in the format array.
Programmer response
Do not code all values in the format array as "*"
(asterisk).
ISPV044 Invalid format data - Data
contains invalid hex character for
packed format
Explanation
The data for a format of PACK must be a numeric value
(0-9).
Programmer response
Ensure that valid numeric data is supplied for a
variable with a format of PACK.
ISPV045 Invalid data length - Length of
"aaaaaaaa" is bbbbbbbb - valid
lengths are 4 and 8.
Explanation
A variable with a format of FLOAT must have a length
of either 4 or 8 bytes.
Programmer response
Make sure that the length supplied on the VDEFINE
service has a value of either 4 or 8 when the format
value is FLOAT.
ISPV046 Invalid data length - Length of
"aaaaaaaa" is bbbbbbbb - valid
lengths are 1 to 10.
Explanation
A variable with a format of PACK must have a length
from 1 to 10 bytes.
Programmer response
Make sure that the length supplied on the VDEFINE
service has a value from 1 to 10 when the format value
is PACK.
ISPV047 Invalid data length - Length of
"aaaaaaaa" is bbbbbbbb - valid
lengths are 2 to 32767.
Explanation
A variable with a format of BINSTR must have a length
greater than 1 and less than 32 768.
Programmer response
Ensure that the length specified on the VDEFINE for a
variable with a format of BINSTR is greater than 1 and
less than 32 768.
ISPV048 Invalid data string - Variable
"aaaaaaaa" must contain a null
terminator.
Explanation
The last byte of a variable that has a format of BINSTR
must be a null terminator (X'00').
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  291

## Page 312

Programmer response
Ensure that the length supplied on the VDEFINE
service for a BINSTR format variable allows for a
null terminator (binary zero) to be placed in the last
position of the variable value. ISPF will store up to
"length - 1" of significant data and then place a null
terminator in the last position.
ISPV050 Invalid mask format - Format
"aaaaaaaa" is not a valid mask
format.
Explanation
An invalid mask format was specified on the VMASK
service.
User response
Contact the responsible programmer.
Programmer response
Check the VMASK service for valid mask formats.
ISPV051 Invalid mask length - Invalid
length "aaaaaaaa" specified for
the VMASK service
Explanation
An invalid length was specified for a USER format on
the VMASK service.
User response
Contact the responsible programmer.
Programmer response
The length of the mask must be a fullword binary
integer greater than 0 and less than or equal to 20.
ISPV052 Invalid variable mask - The
specified mask contains invalid
format characters.
Explanation
An invalid mask symbol was used in the mask
parameter of the VMASK service with a USER type of
mask.
User response
Contact the responsible programmer.
Programmer response
Correct the mask so it contains valid mask symbols as
described by the VMASK service in z/OS ISPF Services
Guide .
ISPV053 Invalid variable data - The
specified variable contains invalid
data for the specified variable
mask.
Explanation
The variable value had data that did not match the
mask pattern. For example, if the mask pattern was
AAA and numeric data was supplied, this error would
occur.
User response
Correct the variable value to fit the mask pattern.
ISPV054 Invalid variable length - The
specified variable value is an
invalid length.
Explanation
The length of the variable value does not match the
length of the mask pattern.
User response
Correct the variable value to match the mask pattern.
ISPV055 Variable not masked - The
specified variable in the VEDIT is
not a masked variable.
Explanation
A VEDIT was specified for a variable that did not have
a VMASK associated with it.
User response
Contact the responsible programmer.
Programmer response
Make sure that any variable that has a VEDIT defined
also has a VMASK associated with it.
ISPV056 Invalid variable format - The
specified variable's VDEFINE
format is invalid for the VMASK
service.
ISPF messages starting with ISP
292  z/OS: z/OS ISPF Messages and Codes

## Page 313

Explanation
An invalid VDEFINE format was specified for the
variable associated with VMASK. The VMASK service
only supports a format of CHAR, FIXED, or PACK.
User response
Contact the responsible programmer.
Programmer response
Correct the code so it either uses the VDEFINE service
with one of the supported formats for the VMASK
service, or does not use the VMASK.
ISPV057 Invalid user mask - A mask should
contain at least one of these
characters: A, 9, H, N, or X.
Explanation
The specified mask must contain at least one of the
symbols A, 9, H, N, or X.
User response
Contact the responsible programmer.
Programmer response
Correct the code so it supplies a valid user mask.
ISPV058 Variable not VDEFINEd - The
specified variable has not been
VDEFINEd.
Explanation
A VDEFINE has not been issued for the variable
associated with the VMASK.
User response
Contact the responsible programmer.
Programmer response
Ensure that a VDEFINE is done for any variable that
has a VMASK associated with it.
ISPV059 Invalid variable mask - "B" can
only be embedded. Blanks cannot
begin or end a variable mask.
Explanation
A VMASK was issued with a B at the beginning or end
of the mask pattern.
User response
Contact the responsible programmer.
Programmer response
Correct the code so it does not specify a mask pattern
with a B in the first or last position of the mask.
ISPV200 Bad service code - The service
code passed to ISPDVCLX was not
a valid one.
Explanation
This message is self explanatory.
ISPV201 Truncation occurred - A variable
was truncated on return from the
EXECCOMM interface.
Explanation
This message is self explanatory.
ISPV202 Bad variable name - A bad
variable name was requested from
the EXECCOMM interface.
Explanation
This message is self explanatory.
ISPV203 Bad variable value - The value of
the variable was too long.
Explanation
This message is self explanatory.
ISPV204 Bad EXECCOMM func. code -
The function code passed to
EXECCOMM was invalid.
Explanation
This message is self explanatory.
ISPV300 Trace complete - The Trace
output is has been written to
SYSOUT allocated to the DDname
ISPVCALL.
Explanation
The output to the ISPVCALL Trace has been written
to a SYSOUT class allocated to the DDname ISPVCALL
and can not be viewed by the ISPVCALL command.
ISPV301 LIST substituted - ISPVCALL trace
data set 'aaaaaaaa' not found.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  293

## Page 314

List of ISPVCALL trace data sets
displayed
Explanation
ISPVCALL attempted to VIEW a trace data set that
could not be found. A data set list of possible
ISPVCALL trace data sets was displayed
ISPV302 No trace data sets found - No
ISPVCALL trace data set names
were found matching 'aaaaaaaa'
Explanation
This is an informational message.
ISPW001 ADDPOP service error - ADDPOP
service issued before first panel
display.
Explanation
At least one panel must have been displayed before an
ADDPOP can be issued.
Programmer response
ISPF normally displays panel ISPBLANK as a dummy
panel prior to an ADDPOP if no previous panel
exists. Make sure that this panel is in the ISPPLIB
concatenation sequence to ensure that ADDPOP will
work if the first panel to be displayed by ISPF is in a
pop-up window.
ISPW002 ADDPOP service error -
Consecutive ADDPOP services
without a panel display.
Explanation
Two consecutive ADDPOP services have been issued
without an intervening display.
Programmer response
Make sure that a panel display is done in between
ADDPOP requests.
ISPW003 REMPOP service error - There are
no active pop-up windows at the
current select level.
Explanation
A REMPOP service was issued, but there were no pop-
ups active.
Programmer response
Ensure that all POPUP and REMPOP services are
paired properly. Use dialog test to trace the ADDPOP
and REMPOP services.
Problem determination
Use dialog test to trace the ADDPOP and REMPOP
services.
ISPW004 Windowing error - Request
to resume windowing however
windowing not suspended.
Explanation
This message is self explanatory.
ISPW005 Windowing error - Unable to
ADDPOP due to previous panel
display error.
Explanation
The ADDPOP request failed due to a previous display
request error. The display request error could be
something like a panel not found or an error in panel
logic. This error will not normally be seen unless the
user is using Control Errors Return in order to see and
handle errors personally.
Programmer response
Correct the error on the display request so that the
ADDPOP request will be honored.
ISPW008 Windowing error - MSGLOC field
"aaaaaaaa" is not defined on the
panel.
Explanation
A MSGLOC field name was specified, but the field is
not defined on the panel.
Programmer response
Specify a field name that exists on the panel.
ISPW009 Internal error - Service request to
module ISPWIN specified invalid
name.
Explanation
This is an internal service call and should not fail.
ISPF messages starting with ISP
294  z/OS: z/OS ISPF Messages and Codes

## Page 315

System programmer response
Contact IBM support.
User response
Contact your system programmer.
ISPWB000 Client requested ISPF session
initialization Userid: aaaaaaaa
ASIDX: bbbb Message Queue:
cccccccccc CCSID: ddddd
Explanation
This message is issued when a request is received to
start an ISPF session on behalf of a client.
User response
For more information see ISPWB000 in “Abend codes
and information” on page 935.
ISPWB001 Request received from client
to force termination. Userid:
aaaaaaaa ASIDX: bbbb
Explanation
This operator message is issued when ISPF receives
a request from a client to force the termination of the
ISPF session for a user.
User response
For more information see ISPWB001 in “Abend codes
and information” on page 935.
ISPWB002 Call to BPX1QSN to send a
message to the queue failed.
Return code: 'aaaa'X Reason code:
'bbbb'X
Explanation
This operator message is issued when a call to z/OS
UNIX service BPX1QSN to send panel JSON to the
client via a z/OS UNIX message queue fails.
User response
For more information see ISPWB002 in “Abend codes
and information” on page 935.
ISPWB003 Call to BPX1QRC to read a
message from the queue failed.
Return code: 'aaaa'X Reason code:
'bbbb'X
Explanation
This operator message is issued when a call to z/OS
UNIX service BPX1QRC to receive response JSON from
the client via a z/OS UNIX message queue fails.
User response
For more information see ISPWB003 in “Abend codes
and information” on page 935.
ISPWB004 Call to BPX1QRC returned a
message of length zero. Return
code: 'aaaa'X Reason code:
'bbbb'X
Explanation
This operator message is issued when a call to z/OS
UNIX service BPX1QRC to receive response JSON from
the client returns a message with a length of zero.
User response
For more information see ISPWB004 in “Abend codes
and information” on page 935.
ISPYB011 Command not recognized - Valid
line commands are: I (insert), D
(delete), R (repeat).
Explanation
These line commands are valid: Inn to insert nn lines,
Dnn to delete nn lines, and Rnn to repeat nn lines.
User response
Enter one of these valid line commands: I (insert), D
(delete), R (repeat).
ISPYB012 Invalid service name - 'aaaaaaaa'
is not a valid ISPF service.
Explanation
Breakpoints are set before or after ISPF services.
A valid ISPF service name is required to set a
breakpoint.
User response
Enter a valid ISPF service as defined in z/OS ISPF
Services Guide.
ISPYB013 Invalid WHEN condition - WHEN
not recognized, specify BEFORE,
AFTER, or Rnn
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  295

## Page 316

Explanation
The WHEN condition tells Dialog Test at what point
to interrupt dialog execution. The valid conditions are:
BEFORE to stop before the service receives control,
AFTER to stop after the service completes, Rnn to stop
after the service completes if the return code is the
integer nn, and (blank) to stop both before and after
the service.
ISPYB014 Invalid function name - The
function name specified does not
follow ISPF naming convention.
Explanation
The function name must follow ISPF naming
conventions.
User response
Enter a valid program or command name.
ISPYB015 Invalid active status - Active
status type not recognized.
Specify YES or NO
Explanation
The active status type field specifies whether the
breakpoint is active or not.
User response
To activate the breakpoint, specify YES. To deactivate
the breakpoint, specify NO.
ISPYB017 Invalid command - 'aaaaaaaa' is
not a valid breakpoints command.
Explanation
Valid commands are CANCEL, RESUME, QUAL, and
LOCATE.
User response
Enter a valid breakpoints command.
ISPYB018 'aaaaaaaa' not found - The service
name does not exist in the
breakpoints list.
Explanation
The LOCATE command was unable to locate the
requested service.
User response
Enter a valid service name.
ISPYB019 'aaaaaaaa' found - Service name
was located, and appears on the
first line.
Explanation
This is an informational message.
ISPYB021 aaaaaaaa failed - bbbbbbbb
request is not recognized by
aaaaaaaa service routine.
Explanation
There was a failure in the breakpoint routines.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB022 aaaaaaaa failed - Breakpoint not
set, bbbbbbbb is not a valid dialog
management service name.
Explanation
There was a failure in the breakpoint routines.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB023 ISPYBI failed - aaaaaaaa returned
rc(bbbbbbbb) request(cccccccc
failed during setup)
Explanation
There was a failure in the breakpoint routines.
ISPF messages starting with ISP
296  z/OS: z/OS ISPF Messages and Codes

## Page 317

System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB024 ISPYBI failed - aaaaaaaa returned
rc(bbbbbbbb) request(cccccccc
failed during create table)
Explanation
There was an error in creating the breakpoint table.
System programmer response
An internal TBCREATE failed. If the failure continues,
contact IBM support.
Programmer response
If the error continues, contact the system programmer.
ISPYB025 ISPYBI failed - aaaaaaaa returned
rc(bbbbbbbb) request(cccccccc
failed during initialize table)
Explanation
The breakpoint routines failed while initializing the
table.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB026 ISPYBI failed - aaaaaaaa returned
rc(bbbbbbbb) request(cccccccc
failed during edit control)
Explanation
The breakpoint routines failed while the tables were
being initialized for editing.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB027 ISPYBI failed - aaaaaaaa returned
rc(bbbbbbbb) request(cccccccc
failed during build chain)
Explanation
There was a failure while building the breakpoint
chain.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB028 aaaaaaaa failed - Breakpoint not
set, function 'bbbbbbbb' is not a
valid function name.
Explanation
There was a failure in the breakpoint routines.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB029 aaaaaaaa failed - Breakpoint not
set, 'bbbbbbbb' is not a valid
WHEN condition.
Explanation
There was failure in the breakpoint routines.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  297

## Page 318

System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYB031 Service name not entered - The
service name is required to
establish a breakpoint.
Explanation
ISPF breakpoints occur at ISPF service invocation
or exit. A service name is required to establish a
breakpoint.
Programmer response
Enter a valid ISPF service name.
ISPYB032 Command conflict - 'aaaaaaaa' is
within range of the D command.
Correct either one.
Explanation
The second line command would be deleted because
it is within the range of the D (DELETE) command.
Programmer response
Correct the conflicting line commands.
ISPYB033 Too many parameters - No
parameters are defined for this
command.
Explanation
Parameters are not allowed with this command.
Programmer response
The data following the command must be removed
from the command line.
ISPYB034 Enter service name - Enter the
name of the service to be located.
Explanation
The LOCATE command requires the ISPF service name
that you want to locate.
Programmer response
Enter an ISPF service with the LOCATE command.
ISPYB035 Too many parameters - A service
name is the only valid parameter
for the LOCATE command.
Explanation
There should not be any data following the ISPF
service name when a LOCATE command is entered.
Programmer response
Clear the command line of the data that follows the
LOCATE command and the ISPF service name.
ISPYB036 Invalid service name - A service
name must not be longer than 8
characters.
Explanation
The ISPF service name following the LOCATE
command is longer than allowed.
Programmer response
Correct the ISPF service name used with the LOCATE
command.
ISPYB037 Test mode not active - You must be
running an application from Dialog
Test, but not in Dialog Test to issue
the DTEST command.
Explanation
The DTEST command was issued while not under
Dialog Test or while in Dialog Test, but not running an
application (Dialog Test option 1).
User response
The DTEST command is only valid while running
an application invoked from option 1 in Dialog Test
(Functions).
ISPYB038 Invalid parameter - You specified
an invalid parameter on the
DTEST command. The only valid
parameters are 1 through 8.
Explanation
The user entered an invalid parameter on the DTEST
command. The valid parameters are a number 1
through 8 corresponding to the Dialog Test options.
ISPF messages starting with ISP
298  z/OS: z/OS ISPF Messages and Codes

## Page 319

User response
The user should specify a valid parameter.
ISPYB039 Switch to Test failed - ISPF was
not able to switch from the user's
environment to the Dialog Test
environment.
Explanation
When running the DTEST command, ISPF needs to
switch from the user's environment to the Dialog Test
environment. This switch failed and therefore ISPF
cannot continue processing the DTEST command.
Programmer response
Try establishing the test environment again and issue
the DTEST command again or try logging onto TSO
with a large region size and then try establishing the
test environment again and issue the DTEST command
again. If problem persists, contact IBM service.
ISPYF010 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc
request(dddddddd).
Explanation
The dialog failed; the message defines the failing
function.
System programmer response
If you do not have mixed code, contact IBM support.
User response
Contact the responsible programmer.
Programmer response
This is either a system error or an ISPF internal error. If
the error continues, contact the system programmer.
ISPYF011 Enter function name - The name
of a panel, command, or program
must be entered.
Explanation
This message is self-explanatory.
Programmer response
Enter a function such as a panel, command, or
program.
ISPYF012 Panel select complete - Select of
the requested panel is complete.
Explanation
This is an informational message.
ISPYF013 Function rc = aaaaaaaa - The
specified function completed with
a return code of aaaaaaaa.
Explanation
This is an informational message.
ISPYF014 Invalid value - Enter YES or NO at
the cursor position.
Explanation
When NEWAPPL is selected, the PASSLIB field must
have an entry.
Programmer response
Enter YES or NO in the field that contains the cursor.
ISPYF015 Enter one name only - More than
one name was entered for a panel,
command or program.
Explanation
This panel allows execution of a panel, a command,
or a program. Only one name can be entered for
processing.
Programmer response
Enter only one panel, command, or program name for
processing.
ISPYF016 Language invalid - Language must
be APL, CREX, or blank.
Explanation
When entering a command for processing, set the
LANG field to APL, CREX, or blank.
Programmer response
Correct the LANG field.
ISPYF017 No parms specified - Enter YES or
NO for extended plist only if PARM
specified.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  299

## Page 320

Explanation
This message is self explanatory.
ISPYF018 MODE invalid - MODE must be
either LINE, FSCR, or blank.
Explanation
Enter LINE for line mode, FSCR for full screen mode, or
blank.
Programmer response
Correct the entry for the MODE field.
ISPYF019 Conflicting values - Selecting
PASSLIB is valid only when
NEWAPPL is also specified.
Explanation
This message is self-explanatory.
Programmer response
Enter the correct value for the PASSLIB and NEWAPPL
fields.
ISPYL011 ISPYLI failed - ISPYXEM returned
rc(aaaaaaaa) request(entry
failed).
Explanation
There was an error on an internal ISPF call.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
User response
Contact your system programmer.
ISPYL012 ISPYLI failed - ISPYLS returned
rc(aaaaaaaa), more details
precede this message in the log.
Explanation
There was a failure in the browse log routine.
System programmer response
If you have preallocated your log data set, verify that it
is allocated correctly. A preallocated log data set with
a disposition of new or old cannot be browsed. If the
disposition is correct, this error could be caused by
mixed levels of ISPF code. If you do not have mixed
levels of ISPF code, contact IBM support.
User response
Contact your system programmer.
ISPYL013 Log data set empty - Log data
is empty. Browse cannot be
performed.
Explanation
This is an informational message. There are no entries
in the log.
ISPYL014 I/O error - An I/O error was
encountered reading the first
record requested.
Explanation
There was an error reading the log data set.
System programmer response
This may be a DASD problem. Attempt to delete the
log data set and then try to recreate the log entry. If
the error continues, contact IBM support.
User response
If the error continues, contact the system programmer.
ISPYL015 Insufficient storage - Not enough
storage is available for browse to
proceed.
Explanation
There was an error while attempting to browse the log
data set.
System programmer response
If the region size satisfies the ISPF requirements,
contact IBM support.
User response
Terminate ISPF and allocate more region storage
before running ISPF again. If the error still occurs,
contact the system programmer.
ISPYL016 Invalid browse panel - Panel
ISPYLP1 is invalid. Contact the
system programmer.
ISPF messages starting with ISP
300  z/OS: z/OS ISPF Messages and Codes

## Page 321

Explanation
The ISPYLP1 panel being referenced cannot be
used by the BROWSE service. Contact your system
programmer.
System programmer response
Correct the problem with ISPYLP1. If you cannot
determine the error in the panel, contact IBM support.
ISPYL017 Recursion error - Recursive use
of ISPF functions (in this case
browse) is not allowed.
Explanation
BROWSE is already active, you cannot activate
BROWSE again.
User response
End one of the BROWSE sessions.
ISPYL018 Browse severe error - Severe error
occurred in module ISRCBR.
Explanation
Browse has returned an RC=20. Contact your system
programmer.
System programmer response
If BROWSE functions correctly using the BROWSE
option, contact IBM support.
ISPYP011 Parameter missing - Either panel
name or message must be
entered.
Explanation
This panel requires either a panel name or message
ID.
Programmer response
Enter a panel name or message ID.
ISPYP012 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc
request(dddddddd)
Explanation
There was an internal ISPF error. Contact the system
programmer.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
ISPYP013 .RESP=ENTER reset - The
panel contained a .RESP=ENTER
statement. The value has been
ignored for this display request.
Explanation
This is an informational message. When displaying a
panel from option 2 of Dialog Test, a .RESP=ENTER
statement will cause an infinite loop. To avoid this
looping situation, ISPF will ignore the .RESP=ENTER
for the display from this option.
ISPYP014 .RESP=ENTER changed - The
panel contained a .RESP=ENTER
statement. This value was
overridden in order to exit this
display request.
Explanation
This is an informational message. When displaying a
panel from option 2 of Dialog Test, a .RESP=ENTER
statement will cause an infinite loop. To avoid this
looping situation, ISPF will alter a .RESP=ENTER in
the )PROC or )REINIT section to .RESP=END in order
to exit the display request.
ISPYR011 Command not recognized - Valid
line commands are: I (insert), D
(delete), R (repeat).
Explanation
An invalid line command was entered.
Programmer response
Enter one of the correct line commands.
ISPYR012 Command conflict - 'aaaaaaaa' is
within range of the D command.
Correct either one.
Explanation
There is a line command that is within the range of the
D command. The delete routine would delete the line
when the DELETE completed.
Programmer response
Correct the line commands.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  301

## Page 322

ISPYR013 Invalid active status - Active
status type not recognized, specify
YES or NO.
Explanation
The Active field must be YES or NO.
Programmer response
Correct the Active field status to YES or NO.
ISPYR014 Too many parameters - No
parameters are defined for this
command.
Explanation
Parameters are allowed with this command.
Programmer response
Correct the command. Parameters are only allowed
with the LOCATE command.
ISPYR015 Invalid command - 'aaaaaaaa' is
not a valid trace command.
Explanation
The command used is not valid for the trace function.
Programmer response
Use the HELP command if you are not sure what
commands are valid for the trace function.
ISPYR016 Invalid function name - The
function name specified does not
follow ISPF naming convention.
Explanation
The function name entered is invalid.
Programmer response
Enter a correct function name.
ISPYR021 aaaaaaaa failed - bbbbbbbb
request is not recognized by
aaaaaaaa service routine.
Explanation
This message is self explanatory.
ISPYR022 ISPYRFI failed - aaaaaaaa
returned rc (bbbbbbbb) request
(cccccccc failed during setup).
Explanation
The function failed to initialize properly.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is a system error or an ISPF internal error. If the
error continues, contact the system programmer.
ISPYR023 ISPYRFI failed - aaaaaaaa
returned rc (bbbbbbbb) request
(cccccccc failed during create
table).
Explanation
The table used for the trace could not be created.
System programmer response
Verify that the TBCREATE works outside of the dialog
test option. If the TBCREATE only fails in dialog test,
and you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
If the error continues, contact the system programmer.
ISPYR024 ISPYRFI failed - aaaaaaaa
returned rc (bbbbbbbb) request
(cccccccc failed during initialize
table).
Explanation
The trace function did not initialize.
System programmer response
This could be a system error or an ISPF internal error.
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
If the problem continues, contact the system
programmer.
ISPYR025 ISPYRFI failed - aaaaaaaa
returned rc (bbbbbbbb) request
(cccccccc failed during edit
control).
ISPF messages starting with ISP
302  z/OS: z/OS ISPF Messages and Codes

## Page 323

Explanation
The trace routines failed while the tables were being
initialized for editing.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYR026 ISPYRFI failed - aaaaaaaa
returned rc (bbbbbbbb) request
(cccccccc failed during build
chain).
Explanation
There was a failure while building the breakpoint
chain.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
This is possibly an internal error caused by either a
system error or mixed levels of ISPF code. If the error
continues, contact the system programmer.
ISPYR031 Function name required - Enter
a function name or ALL for all
functions.
Explanation
A function name is required.
Programmer response
Either enter a function name or enter HELP for
additional information.
ISPYR032 Invalid service name - 'aaaaaaaa'
is not a valid dialog management
service.
Explanation
The service name entered is invalid.
Programmer response
Enter a valid ISPF service name.
ISPYR033 Invalid service name - A service
name must not be longer than 8
characters.
Explanation
The service name exceeds 8 characters.
Programmer response
Enter a valid ISPF service name.
ISPYR034 'aaaaaaaa' not found - The
function name does not exist in
the trace list.
Explanation
The function name entered with the LOCATE command
does not exist.
Programmer response
Enter another function name with the LOCATE
command.
ISPYR035 'aaaaaaaa' found - Function name
was located, and appears on the
first line.
Explanation
This is an informational message.
ISPYR037 Enter function name - Enter the
name of the function to be located.
Explanation
A function name must be entered with the LOCATE
command.
Programmer response
Enter a function name to be used with the LOCATE
command.
ISPYR038 Too many parameters - A function
name is the only valid parameter
for the LOCATE command.
Explanation
Data was entered beyond the function name when the
LOCATE command was entered.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  303

## Page 324

Programmer response
Remove any data following the function name.
ISPYR039 Invalid function name - A function
name must not be longer than 8
characters.
Explanation
The function name exceeds 8 characters.
Programmer response
Enter a valid function name with the LOCATE
command.
ISPYR041 Variable name required - Enter
a variable name or ALL for all
variables.
Explanation
The TRACE panel requires a variable name or ALL for
tracing.
Programmer response
Enter ALL or a variable name in the variable field.
ISPYR042 Enter variable name - Enter the
name of the variable to be located.
Explanation
No variable name was entered with the LOCATE
command.
Programmer response
Enter a variable name with the LOCATE command.
ISPYR043 Too many parameters - A variable
name is the only valid parameter
for the LOCATE command.
Explanation
The LOCATE command does not allow data after the
variable name.
Programmer response
Remove the data following the variable name.
ISPYR044 Invalid variable name - A variable
name must not be longer than 8
characters.
Explanation
The variable name exceeds 8 characters.
Programmer response
Correct the variable name.
ISPYR045 Invalid operation - Valid
operations are GET, PUT, CHG, or
ALL.
Explanation
Valid entries for the Operation field are GET, PUT, CHG,
or ALL.
Programmer response
Correct the entry in the Operation field.
ISPYR046 Invalid pool specified - Enter F
(function), S (shared), P (profile).
Explanation
The entry in the Pool field must be F, S, or P.
Programmer response
Correct the entry in the Pool field.
ISPYR047 'aaaaaaaa' not found - The
variable name does not exist in the
trace list.
Explanation
The variable was not found in the trace list.
Programmer response
Enter another variable name with the LOCATE
command.
ISPYR048 'aaaaaaaa' found - Variable name
was located, and appears on the
first line.
Explanation
This is an informational message.
ISPYR049 Invalid variable syntax - Variable
name must be alphanumeric and
the first character cannot be
numeric.
ISPF messages starting with ISP
304  z/OS: z/OS ISPF Messages and Codes

## Page 325

Explanation
There is a syntax violation in the variable name.
Programmer response
Correct the variable name.
ISPYS011 Enter dialog service - The desired
dialog service and its parameters
must be entered.
Explanation
You must enter an ISPF service and any required
parameters.
Programmer response
Enter an ISPF service and any required parameters.
ISPYS012 Service rc = aaaaaaaa - The
specified dialog service completed
with a return code of aaaaaaaa.
Explanation
This is an informational message.
ISPYS013 CONTROL not allowed - The
dialog CONTROL service cannot be
invoked from this panel.
Explanation
This message is self-explanatory.
Programmer response
Do not attempt to invoke ISPF CONTROL service from
this panel.
ISPYS014 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc
request (dddddddd).
Explanation
The requested function has failed with a severe error.
Programmer response
Verify that the requested function will work correctly
outside of Dialog Test before contacting IBM support.
ISPYS015 Data will be saved - Data entered
on this screen will be saved upon
exit.
Explanation
This is an informational message. Data that the user
enters on this panel will be saved when the user exits
the panel.
ISPYS016 Data will be cleared - Data entered
on this screen will not be saved
upon exit.
Explanation
This is an informational message. Data that the user
enters on this panel will not be saved when the user
exits the panel.
ISPYT010 Table is not open - Function
requires an open table, open via
option 6.
Explanation
The table must be open using Dialog Test option 6
before you can perform the requested function against
the table.
User response
Contact the responsible programmer.
Programmer response
Use Dialog Test option 6 and the TBOPEN service to
open the table.
ISPYT011 Table not found - The specified
table could not be found.
Explanation
The table could not be found in the ISPTLIB
allocations.
User response
Contact the responsible programmer.
Programmer response
Correct the allocations to ISPTLIB, or create the table.
ISPYT012 Invalid command - 'aaaaaaaa' is
not a valid command for this table
operation.
Explanation
The command entered is invalid for this panel.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  305

## Page 326

User response
Contact the responsible programmer.
Programmer response
Enter a valid command, such as LOCATE.
ISPYT013 'aaaaaaaa' not found - The key or
name 'bbbbbbbb' was not found in
the table.
Explanation
This is an informational message.
ISPYT014 'aaaaaaaa' was found - The key or
name 'bbbbbbbb' was found in the
table.
Explanation
This is an informational message.
ISPYT015 Table in use - The specified table
is in use by another user, it cannot
be opened.
Explanation
This table is being used by another user and cannot be
used by you now.
Programmer response
The other user must close the table before you can use
it.
ISPYT016 Library not allocated - The input
table library is not allocated.
Explanation
The library that contains the table has not been
allocated.
User response
Contact the responsible programmer.
Programmer response
If the table is required, the library must be allocated.
ISPYT017 Enter table name - The name of
the table is required.
Explanation
You must enter a table name.
User response
Contact the responsible programmer.
Programmer response
Enter a table name to continue.
ISPYT018 Enter option - A valid option must
be entered.
Explanation
One of the listed options must be entered.
Programmer response
Enter one of the numerical options that is listed.
ISPYT019 Invalid option - Your selection is
not valid.
Explanation
You must enter one of the numeric options that is
listed.
Programmer response
Enter one of the numeric options that is listed.
ISPYT020 Name too long - Variable names
are eight characters maximum.
Explanation
The variable name is too long.
Programmer response
Correct the variable name.
ISPYT021 Row not added - Blank row
was not added, operation was
canceled.
Explanation
This is an informational message. The operation was
canceled; the blank row was not added.
ISPYT022 Invalid hex data - Enter valid hex
characters in multiples of two.
Explanation
Hexadecimal characters must be entered in multiples
of two.
ISPF messages starting with ISP
306  z/OS: z/OS ISPF Messages and Codes

## Page 327

Programmer response
Correct the hexadecimal entry.
ISPYT024 Table aaaaaaaa opened - Table
aaaaaaaa opened in bbbbbbbb
mode by dialog test.
Explanation
This is an informational message.
ISPYT025 Table aaaaaaaa closed - Table
aaaaaaaa closed by dialog test.
Explanation
This is an informational message.
ISPYT031 No argument specified - A search
argument is necessary for the
LOCATE command.
Explanation
Before the LOCATE command can be run, a value to be
searched for must be entered.
Programmer response
Enter a value following the LOCATE command.
ISPYT032 Row number invalid - Row number
must be greater than zero.
Explanation
This message is self-explanatory.
ISPYT033 'aaaaaaaa' was found - Variable
'aaaaaaaa' was found in the row.
Explanation
This is an informational message.
ISPYT034 'aaaaaaaa' not found - Variable
'aaaaaaaa' was not found in the
row.
Explanation
This is an informational message.
ISPYT035 Command conflict - A line
command is within the range of
the D command. Correct either
one.
Explanation
There is a line command that would not be processed
because that line would be deleted when the DELETE
line command is completed.
Programmer response
Correct the conflicting line commands.
ISPYT036 Invalid name - Variable name
characters/syntax is not valid.
Explanation
The variable name syntax is in error.
Programmer response
Correct the variable name.
ISPYT037 Too many parameters - No
parameters are defined for the
'CANCEL' command.
Explanation
The CANCEL command does not allow parameters.
Programmer response
Remove the data following the CANCEL command.
ISPYT038 Too many parameters - Only one
argument may be entered with the
LOCATE command.
Explanation
Only one name can be used with the LOCATE
command.
Programmer response
Correct the LOCATE command.
ISPYT039 Use TOP - Use TOP for adding row
1.
Explanation
Enter TOP to add a new row as the first row.
Programmer response
Enter TOP for row number.
ISPYT040 Key/name protected - Key and
name variables may not be
deleted.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  307

## Page 328

Explanation
The DELETE command is not allowed on key and name
variables.
Programmer response
Do not attempt to delete protected variables.
ISPYT041 Row exceeds table size - The
specified row number is greater
than the table size.
Explanation
The row number entered exceeds the number of rows
in the table.
Programmer response
Display the structure to determine the number of rows,
or enter a correct row number.
ISPYT042 Invalid row identity - The table
row must be specified as a
number.
Explanation
This message is self-explanatory.
ISPYT043 Row not identified - Enter a search
argument or a row number.
Explanation
This option requires a row number or a search
argument.
Programmer response
Enter a row number or search argument.
ISPYT044 Function invalid - The function
code is not valid.
Explanation
The function code is invalid.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
If a valid option was entered, this could be an internal
error. Contact your system programmer.
ISPYT045 Row not found - No row matches
the search argument.
Explanation
This message is self-explanatory.
ISPYT046 Row found - A row matching the
search argument was found.
Explanation
This is an informational message.
ISPYT047 Row not deleted - The specified
row was not deleted from the
table.
Explanation
This is an informational message. END was entered,
and the DELETE command was ignored.
ISPYT048 Row aaaaaaaa deleted - Row
aaaaaaaa was deleted from the
table.
Explanation
This is an informational message.
ISPYT049 Row not deleted - No match
searching keyed table, or CRP was
at top.
Explanation
The row could not be deleted because no match was
found.
Programmer response
Enter another search argument for the DELETE
command.
ISPYT050 Key/name not modifiable - Key
and name variables may not be
changed.
Explanation
The key and names variable names cannot be
changed.
Programmer response
Do not attempt to change key or names variable
names.
ISPF messages starting with ISP
308  z/OS: z/OS ISPF Messages and Codes

## Page 329

ISPYT051 Keys are not unique - Another row
in the table has the same key
values.
Explanation
You cannot have two key variables with the same
value.
Programmer response
Provide another value for the key variable.
ISPYT052 Row modified - The specified row
has been modified.
Explanation
This is an informational message.
ISPYT053 Row added - A new row has been
added following the current row.
Explanation
This is an informational message.
ISPYT054 Operation canceled - No rows have
been added or modified.
Explanation
This is an informational message.
ISPYT055 Row not added - A row has not
been added to the table.
Explanation
This is an informational message.
ISPYT056 Table is at top - Display, modify
and CANCEL are not valid when
the table is at top.
Explanation
This message is self-explanatory.
Programmer response
Use a row number when the table is at top.
ISPYT057 Invalid line command - Valid
table commands are I (insert), R
(repeat), and D (delete).
Explanation
An invalid line command was used.
Programmer response
Use only valid line commands.
ISPYT058 Top is not valid - Top is only valid
for adding a row.
Explanation
This message is self-explanatory.
ISPYT059 New row added - The keys did not
match an existing row.
Explanation
This message is self explanatory.
ISPYT060 Enter WRITE or NOWRITE - Save
option is specified as WRITE or
NOWRITE.
Explanation
This message is self-explanatory.
User response
Enter WRITE or NOWRITE.
ISPYT061 Enter YES or NO - Replace option
is specified as YES or NO.
Explanation
This message is self explanatory.
ISPYT062 Table not created - The specified
table was not created due to error.
Explanation
This message is self explanatory.
ISPYT063 Table created - The table was
created. If WRITE, it is closed.
Explanation
This message is self explanatory.
ISPYT064 Enter names - The names of keys
or names is required.
Explanation
This message is self explanatory.
ISPYT065 Table create canceled - The table
create function was canceled.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  309

## Page 330

Explanation
This message is self explanatory.
ISPYT066 Invalid command - Only the
CANCEL command is valid now.
Explanation
This message is self explanatory.
ISPYT067 Enter table name - Table name is a
required field.
Explanation
This message is self explanatory.
ISPYT068 Use TOP - Use TOP for adding row
1.
Explanation
This message is self explanatory.
ISPYT069 Save/name not deleted - Only save
names can be deleted.
Explanation
This message is self explanatory.
ISPYT070 Windowed panel - The panel or
panels for the selected option will
appear in a pop-up window.
Explanation
This is an informational message. The user selected a
pull-down menu that changed the display of an option
from a full screen display to a windowed display.
ISPYT071 Full screen panel - The panel or
panels for the selected option will
appear in a full screen display.
Explanation
This is an informational message. The user selected a
pull-down menu that changed the display of an option
from a windowed display to a full screen display.
ISPYT072 Retrieved - The command has
been retrieved.
Explanation
This is an informational message. The command in the
retrieve area of the panel has been retrieved to the
command area.
ISPYV000 No LOCATE string - In order to use
the RFIND command, you must
have previously issued a LOCATE
command.
ISPYV001 Invalid pool specified - Enter F, I,
S, or P.
Explanation
An incorrect variable pool was entered.
Programmer response
Enter one of the correct variable pools.
ISPYV002 Command not recognized - The
valid line commands are I (insert)
and D (delete).
Explanation
An incorrect line command was entered.
Programmer response
Enter one of the correct line commands.
ISPYV003 Duplicate variable - Variables
within each pool must have unique
names.
Explanation
The variable name already exists within the pool.
Variable names must be unique.
Programmer response
Enter a unique variable name.
ISPYV004 Invalid command - The command
entered is not a valid variables
command. Valid commands are
LOCATE variable, SORT, SORT
NAME and SORT VALUE. Valid
abbreviations are L, LOC, SORT
N, SORT V, and SORT VAL. The
default SORT command sorts the
list by variable pool and variable
name.
Explanation
An invalid command was entered.
Programmer response
Enter a valid command, or enter HELP for a list of valid
commands.
ISPF messages starting with ISP
310  z/OS: z/OS ISPF Messages and Codes

## Page 331

ISPYV005 No argument specified - A search
argument is necessary for the
LOCATE command.
Explanation
The LOCATE command requires that you enter the data
to use for the search.
Programmer response
Enter a search argument with the LOCATE command.
ISPYV006 Variable name found - The
specified variable appears at the
top of the display.
Explanation
This is an informational message.
ISPYV007 Bottom of Data Reached - The
specified variable was not found in
the list of variables. Press RFIND
to continue searching from the top
of the list.
Explanation
The specified variable does not exist in the variables
list.
Programmer response
Enter another variable, or enter HELP for additional
information.
ISPYV008 Non-modifiable variable - This is
a system variable and may not be
altered/deleted by the user.
Explanation
This is an informational message. Type Z system
variables may not be altered or deleted.
ISPYV009 Variable name required - A
variable name is required if data
is entered for pool or value.
Explanation
Variable pool information or a variable value was
entered, but the variable name was not entered.
Programmer response
Enter a variable name.
ISPYV011 Invalid hex data - Enter valid hex
characters in multiples of two.
Explanation
Hexadecimal characters must be entered in multiples
of two.
Programmer response
Correct the hexadecimal character entry.
ISPYV012 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc
request (dddddddd).
Explanation
The service and request did not complete processing.
System programmer response
This could be an internal error or a system error. If you
do not have mixed levels of ISPF code, contact IBM
support.
Programmer response
If the error continues, contact the system programmer.
ISPYV013 Invalid variable syntax - Variable
name must be alphanumeric and
the first character cannot be
numeric.
Explanation
The variable name violates variable name syntax.
Programmer response
Enter a valid variable name.
ISPYV014 Delete not allowed - The indicated
line may not be deleted.
Explanation
The DELETE command is not valid for this line.
Programmer response
Do not attempt to delete variables that are not
modifiable.
ISPYV015 Delete not allowed - Multiple
DELETE line command not
allowed.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  311

## Page 332

Explanation
The DELETE line command is limited to one line for
each D.
Programmer response
Enter a D for each line to be deleted.
ISPYV016 Sort completed - The table was
sorted by variable name within
profile pools.
ISPYV017 The specified variable name does
not exist in the list. The list
has been positioned at the next
nearest match. Press RFIND to
locate the nearest match in the
next variable pool.
ISPYV301 Internal test error - Unable to
aaaaaaaa storage for variable
bbbbbbbb.
Explanation
A storage problem prevented the function from ending.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
If the problem continues, contact the system
programmer.
ISPYV303 Invalid variable name - Variable
name aaaaaaaa is syntactically
incorrect.
Explanation
The variable name failed the ISPF syntax check.
Programmer response
Correct the variable name.
ISPYV304 Invalid function request - Request
aaaaaaaa is undefined.
Explanation
The request is not valid in ISPF, and did not complete
processing.
System programmer response
This is a system or internal error. If you do not have
mixed levels of ISPF code, contact IBM support.
Programmer response
If the problem continues, contact the system
programmer.
ISPYV305 Invalid pool name - Variable pool
aaaaaaaa is undefined.
Explanation
The variable pool that was entered is invalid.
Programmer response
Enter a valid variable pool, or enter HELP for additional
information.
ISPYV306 Invalid hex data - Variable
(aaaaaaaa) value (bbbbbbbb).
Explanation
The hexadecimal data could not be converted.
Programmer response
Correct the hexadecimal data. If the error continues,
contact IBM support.
ISPYV307 End quote missing - Variable
(aaaaaaaa) value (bbbbbbbb).
Explanation
This message is self-explanatory.
ISPYX001 Test severe error - Details precede
this message in the ISPF log.
Explanation
The function failed to complete.
Programmer response
Check the ISPF log for additional information about
this error.
ISPYX002 Recursive Dialog Test - Dialog Test
may only be entered once in each
screen.
ISPF messages starting with ISP
312  z/OS: z/OS ISPF Messages and Codes

## Page 333

Explanation
You attempted to enter Dialog Test for a second time
in one screen. You may only enter Dialog Test once per
screen.
User response
Either split the screen and enter Dialog Test from the
new screen or back out of the panels being displayed
in the current screen until you reach Dialog Test again.
ISPYX200 Dialog trace -----------
- Application(aaaaaaaa.)
Function(bbbbbbbb.)
Screen(cccccccc.)
Explanation
This is an informational message.
ISPYX201 aaaaaaaa bbbbbbbb
cccccccc;cccccccc;cccccccc;
dddddddd - eeeeeeee
Explanation
This is an informational message.
ISPYX202 aaaaaaaa
Explanation
This is an informational message.
ISPYX203 ..Return code (aaaaaaaa.) -
bbbbbbbb
Explanation
This is an informational message.
ISPYX211 aaaaaaaa Pool (bbbbbbbb.)
cccccccc - dddddddd
Explanation
This is an informational message.
ISPYX212 ..aaaaaaaa by bbbbbbbb - cccccccc
Explanation
This is an informational message.
ISPYX213 ..aaaaaaaa - bbbbbbbb
Explanation
This is an informational message.
ISPYX301 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc.
Request (dddddddd).
Explanation
The function did not complete.
System programmer response
This is a system error or internal error. Verify that
the user's region is large enough for ISPF minimum
requirements. If the error continues, contact IBM
support.
Programmer response
If the error continues, contact the system programmer.
ISPYX302 aaaaaaaa failed - bbbbbbbb
returned return code cccccccc.
Request (Get test common area).
Explanation
There is an error in storage management. The function
was not completed.
System programmer response
This is a system error or internal error. Verify that the
user's region size is large enough for ISPF minimum
requirements. If the error continues, contact IBM
support.
Programmer response
If the error continues, contact the system programmer.
ISPYX321 Dialog Test recursion - Invocation
of the Dialog Test option may not
occur while in that option.
Explanation
This message is self-explanatory.
ISPYX322 Not in Dialog Test - The Dialog Test
option must be used to select a
Test suboption.
Explanation
You cannot directly call the Dialog Test suboptions.
The Dialog Test option must be entered before Test
suboptions can be selected.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  313

## Page 334

Programmer response
Do not attempt to select Test suboptions outside of the
Dialog Test option.
ISPYX323 Not in Test mode - A Dialog Test
suboption may not be selected in
user mode.
Explanation
A Dialog Test suboption was selected while in user
mode. User mode is entered when a breakpoint
returns control to the user.
Programmer response
Do not attempt to enter Dialog Test mode or select
Test suboptions while in user mode.
ISPYX351 Internal test error - Invalid test
chain manager aaaaaaaa request
bbbbbbbb.
Explanation
There was an error while the test chain manager was
processing.
System programmer response
A system or internal error has occurred. Contact IBM
support.
Programmer response
If the error continues, contact the system programmer.
ISPYX352 Internal test error - Unable to free
test chain aaaaaaaa.
Explanation
An error occurred while the test chain manager was
attempting to free some event areas.
System programmer response
A system or internal error has occurred. Contact IBM
support.
Programmer response
If the error continues, contact the system programmer.
ISPYX353 Internal test error - Unable to get
block for test chain aaaaaaaa.
Explanation
An error has occurred while attempting to get storage
for the test chain manager.
System programmer response
If the region size meets ISPF minimum requirements,
contact IBM support.
Programmer response
If the error continues, contact the system programmer.
ISPYX361 Internal test error - Invalid test
trace routine caller aaaaaaaa.
Explanation
The function was not completed because of an ISPF
internal error.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
Contact your system programmer.
ISPYX371 Internal test error - Invalid test
main routine caller aaaaaaaa.
Explanation
The function was not completed because of an ISPF
internal error.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
Programmer response
Contact your system programmer.
ISPYX381 Internal test error - Invalid test
environment manager request
aaaaaaaa.
Explanation
The function could not be completed.
System programmer response
If you do not have mixed levels of ISPF code, contact
IBM support.
ISPF messages starting with ISP
314  z/OS: z/OS ISPF Messages and Codes

## Page 335

Programmer response
This may be an ISPF internal error. Contact your
system programmer.
ISPYX401 aaaaaaaa has no effect - Select an
option and press the Enter key to
leave this panel.
Explanation
This message is self-explanatory.
ISPZ000 aaaaaaaa - bbbbbbbb
Explanation
This is an informational message.
ISPZ001 Invalid command - aaaaaaaa is
undefined.
Explanation
An invalid command was entered on the panel.
User response
Enter a command that is valid for the corresponding
panel.
ISPZ002 Invalid command - aaaaaaaa
bbbbbbbb is undefined.
Explanation
The command entered is not defined.
User response
Enter a command that is valid for the corresponding
panel.
Programmer response
Ensure that the proper commands are in the command
table under the correct application ID.
ISPZZ100 Row aaaaaaaa of bbbbbbbb
Explanation
This is an informational message. This is the text used
for the top-row-displayed indicator on table display
panels with CUA Mode set to OFF.
ISPZZ101 aaaaaaaa
Explanation
This is an informational message. This is a null
message used for the top-row-displayed indicator
on table display panels when no rows are being
displayed.
ISPZZ102 Row aaaaaaaa to bbbbbbbb of
cccccccc
Explanation
This is an informational message. This is the text used
for the top-row-displayed indicator on table display
panels with CUA Mode set to ON and ROWS set to ALL
in the panel model section.
ISPZZ103 Row aaaaaaaa from bbbbbbbb
Explanation
This is an informational message. This is the text used
for the top-row-displayed indicator on table display
panels with CUA Mode set to ON and Rows set to
SCAN in the panel model section.
ISPF messages starting with ISP
Chapter 1. ISPF messages starting with ISP  315

## Page 336

ISPF messages starting with ISP
316  z/OS: z/OS ISPF Messages and Codes
