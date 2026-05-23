# Chapter 3. SCLM messages

Source file: f54mc00_v3r1.md
Start page: 717
Page span: 717-886

## Page 717

Chapter 3. SCLM messages
This topic provides a complete listing and description of messages issued by the SCLM functions. Abend
codes are issued with associated error messages.
SCLM macro messages (MNOTEs) are listed in Chapter 4, “SCLM macro messages (MNOTEs),” on page
867.
FLMA000 PROJDEF error - SCLM received
an error attempting to load the
project definition.
Explanation
When attempting to maintain the list of SCLM
Administrators (Option A from the main menu)
SCLM received an error initializing the SCLM project
definition.
User response
Possible return codes are:
4
The specified project definition load module is
not RMODE(24). Generate the project definition
load module again and specify the RMODE(24)
parameter to the linkage editor.
8
An error occurred while attempting to obtain the
specified project definition or alternate project
definition. Verify the project or alternate project
definition.
12
The project definition is out of date. Reassemble
the project definition with new SCLM macros.
Submit the job again.
16
The project name specified does not match the
project name in the project definition. Verify that
the project name specified on the FLMABEG macro
and the project name in the project definition are
the same.
Another possible explanation is that SCLM is being
invoked with an alternate project definition name,
and this alternate project definition has the same
name as another alternate project definition name
that is being used, but has different contents.
This can occur only when two different primary
project names are being invoked with the same
alternate name from different sessions (such as
from a split screen). To fix the problem, cancel out
of one session and rename the alternate project
definition.
20
An attempt to open or close the project definition
failed. Browse the project definition data set
(project_id.PROJDEFS.LOAD). Select the member
whose name matches the alternate you are using,
or the project ID, if the alternate is blank. If the
member appears, close the Browse panel and
submit the job again.
24
The project definition data set could not be
allocated. Verify that the project definition data set
exists and is not allocated exclusively by another
user. For more information about allocating the
PROJDEFS data sets, see the topic about "Defining
the project environment" in z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference.
FLMA001 Allocation error - Error allocating
the SCLM control file specified in
the SCLM project definition.
Explanation
The VSAM data set specified on the CONTROL
parameter on the FLMCNTRL macro could not be
allocated.
User response
Check that the VSAM data set specified on the
CONTROL parameter on the FLMCNTRL macro exists
and is able to be accessed.
FLMA002 Option not available - The
SCLM control file has not been
specified in the project definition.
Maintaining administrator user IDs
is not possible.
Explanation
The VSAM data set specified on the CONTROL
parameter on the FLMCNTRL macro has not been
specified. The user will not be able to maintain SCLM
administrators or to transfer ownership of member
level locks until this data set is specified.
SCLM messages
© Copyright IBM Corp. 1980, 2024 697

## Page 718

User response
Specify a VSAM data set on the CONTROL parameter
on the FLMCNTRL macro to contain a list of the SCLM
administrators.
FLMA003 Insufficient storage - Allocation of
storage failed due to insufficient
virtual storage.
Explanation
Allocation of storage failed due to insufficient virtual
storage.
User response
Increase the region size of the TSO address spaces.
FLMA004 VSAM open failed - Opening of the
VSAM control file failed.
Explanation
An error was encountered when opening the VSAM
control file specified on the CONTROL parameter on
the FLMCNTRL macro.
User response
Determine if the VSAM data set is in use.
FLMA005 VSAM read error - Reading of the
VSAM control file failed.
Explanation
An error was encountered when reading a record
from the VSAM control file specified on the CONTROL
parameter on the FLMCNTRL macro.
User response
Determine if the VSAM data set is in use.
FLMA006 VSAM delete error - Deletion of
a record on the VSAM control file
failed.
Explanation
An error was encountered when deleting an SCLM
administrator from the VSAM control file specified on
the CONTROL parameter on the FLMCNTRL macro.
User response
Determine if the VSAM data set is in use.
FLMA007 VSAM write error - Writing of a
record to the VSAM control file
failed.
Explanation
An error was encountered when writing an SCLM
administrator to the VSAM control file specified on the
CONTROL parameter on the FLMCNTRL macro.
User response
Determine if the VSAM data set is in use.
FLMA008 aaaaaaaa added - SCLM
administrator aaaaaaaa added.
Explanation
An SCLM administrator was added to the VSAM control
file specified on the CONTROL parameter on the
FLMCNTRL macro.
User response
None.
FLMA009 Option not available - You must be
an SCLM administrator to use this
option.
Explanation
You must be an SCLM administrator to be able to
maintain the list of SCLM administrators.
User response
If you need this option, ask one of the existing SCLM
administrators to add you as an SCLM administrator.
FLMA010 Option not available - Member
level locking is not active, this
option is not available.
Explanation
Member level locking is not active. Therefore, the
'Maintain SCLM Administrator' option and 'Transfer
Ownership' line command are not available.
User response
To enable member level locking, see the topic about
"Defining the project environment" in z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
SCLM messages
698  z/OS: z/OS ISPF Messages and Codes

## Page 719

FLMP000 Package in use - The package
aaaaaaaa is currently being
updated by another user so you
cannot update or change the
status of members.
Explanation
The package you are attempting to to update is
currently being updated by another user.
User response
Check to see why the user is updating the package.
FLMP001 Package saved - The package
aaaaaaaa has been updated
to save the member selection
changes you have made.
Explanation
The package has been updated to save the member
selection changes you have made.
User response
None.
FLMP002 Invalid selection - The member
you have selected for member
recovery is not available for
recovery.
Explanation
The member you have selected for member recovery
is not available for recovery.
User response
None.
FLMP003 Restore Invalid - The
package either contains obsolete
references that have been
superseded or the target members
to be replaced have changed
since this package was created
or the package has already been
restored.
Explanation
The package cannot be restored for one of these
reasons:
• it contains references that have been superseded
• The target members to be replaced have changed
since this package was created
• The package has already been restored
User response
None.
FLMP004 Selection(s) ignored - You have
selected at least one member for
restore that does not have a status
of BACKEDUP.
Explanation
While attempting to restore members of a backed up
package you have selected at least one member that
does not have a status of BACKEDUP.
User response
Deselect any members that do not have a status of
BACKEDUP.
FLMP005 Restore ignored - You have
requested a member level restore
without selecting any members to
be restored.
Explanation
Restore ignored. You have requested a member level
restore without selecting any members to be restored.
User response
None.
FLMP006 Invalid command - The command
you have entered is invalid for this
function
Explanation
The line command you entered is invalid for restoring a
package backup member.
User response
Enter 'L' to locate a member or 'R' restore the member.
FLMP007 No matching packages - No
packages were found to contain
members matching the filter
values you have entered.
SCLM messages
Chapter 3. SCLM messages  699

## Page 720

Explanation
No packages were found that contain members
matching the filter values you entered.
User response
Modify the filter values.
FLMP008 Package Invalid - This package
member must first be processed
by the package backout migration
tool FLMPKFIX before it can be
processed.
Explanation
This package member must first be processed by the
package backout migration tool FLMPKFIX.
FLMP009 Display failed - Unable to display
panelid aaaaaaaa panel title
"bbbbbbbb".
FLMQ000 Data set unavailable - Data set
aaaaaaaa could not be found or is
not available for your use. Correct
the data set name and press Enter.
Explanation
The data set entered on the SCLM Sample Project
Create panel does not exist or is not available for
your use. The data set may be enqueued exclusively
to another user or you may not have authority to use
the data set.
User response
Correct the data set name or determine why the data
set is not available for your use.
FLMQ001 Allocation error - Error allocating
data set aaaaaaaa.. TSO
messages should have been
written to your screen describing
the error. If TSO messages
were not written to your
screen, check the setting of
WTPMSG in your TSO PROFILE.
Names of data sets created for
this project are in data set
bbbbbbbb..PROJDEFS.DATASETS.
Explanation
An error occurred creating a data set for the sample
SCLM project. TSO messages should have been written
to the screen describing the error.
User response
Correct the problem described in the TSO messages.
FLMQ002 Initialization error - Error
initializing VSAM file aaaaaaaa..
TSO messages should have been
written to your screen describing
the error. If TSO messages were
not written to your screen, check
the setting of WTPMSG in your TSO
PROFILE.
Explanation
An error occurred initializing the VSAM data base file
for the sample SCLM project. TSO messages should
have been written to the screen describing the error.
User response
Correct the problem described in the TSO messages.
FLMQ003 LMINIT error - Error for LMINIT
of data set aaaaaaaa.. LMINIT
Return Code = bbbbbbbb..
Explanation
An error occurred on the ISPF LMINIT service for the
data set listed in the message.
User response
Consult the ISPF Services Guide for an explanation
of the return code. Correct the problem and retry the
operation.
FLMQ004 LMCOPY error - Error for LMCOPY
of member aaaaaaaa from data
set bbbbbbbb.. LMCOPY Return
Code = cccccccc..
Explanation
An error occurred on the ISPF LMCOPY service for the
data set and member listed in the message.
User response
Consult the ISPF Services Guide for an explanation
of the return code. Correct the problem and retry the
operation.
FLMQ005 A fully qualified data set name
enclosed in quotes is required.
SCLM messages
700  z/OS: z/OS ISPF Messages and Codes

## Page 721

Explanation
You entered a data set name that is not fully qualified.
The SCLM Sample Project Create function requires
all data set names except the High Level Assembler
Location be fully qualified and enclosed in single
quotes.
User response
Correct the data set name.
FLMQ006 A fully qualified data set name
enclosed in quotes or LINKLIB
without quotes is required.
Explanation
You entered a data set name that is not fully qualified
and is not LINKLIB without quotes. The SCLM Sample
Project Create function requires the data set name
for the High Level Assembler Location be a fully
qualified data set name enclosed in quotes or the term
LINKLIB.
User response
Correct the entry.
FLMQ007 Assembly error - Error assembling
project definition
aaaaaaaa..PROJDEFS.SOURCE(aa
aaaaaa) with the ISPF Macros
Data set and
aaaaaaaa..PROJDEFS.SOURCE as
SYSLIB. The output is in
bbbbbbbb..
Explanation
Assembly of the project definition resulted in a
nonzero return code.
User response
Attempt to assemble the project definition member
using the ISPF Macros data set and the
project.PROJDEFS.SOURCE as SYSLIB. Use option 4.1
or batch. Correct the ISPF Macros data set name if it is
incorrect.
FLMQ008 Link error - Error linking project
definition
aaaaaaaa..PROJDEFS.OBJ(aaaaa
aaa). The output is in bbbbbbbb..
Explanation
Link Edit of the project definition resulted in a nonzero
return code.
User response
Attempt to link edit the project definition member
project.PROJDEFS.OBJ to project.PROJDEFS.LOAD.
Use option 4.7 or batch. Correct any errors you find.
FLMQ009 MIGRATE failed - Error migrating
members into project aaaaaaaa..
Error messages are contained in
aaaaaaaa..MIGRATE.MESSAGES.
Explanation
The SCLM MIGRATE of members into the project
failed.
User response
Correct the errors described by the messages in
project.FLMMSGS.
FLMQ010 BUILD failed - Error building the
project aaaaaaaa.. Messages are
in aaaaaaaa..BUILD.MESSAGES.
Listings are in
aaaaaaaa..BUILD.LISTINGS. The
BUILD report is in
aaaaaaaa..BUILD.REPORT.
Explanation
Build of the sample project failed.
User response
Correct the errors in the messages and listings
data sets. Complete the sample project using the
instructions in the SCLM Project Manager's Guide,
Preparing the Example Project Data, by completing the
BUILD and PROMOTE steps.
FLMQ011 PROMOTE failed - Error
promoting the sample
application from aaaaaaaa
to bbbbbbbb.. Messages are
in cccccccc..PROMOTE.MESSAGES.
Listings are in
cccccccc..PROMOTE.LISTINGS.
The PROMOTE report is in
cccccccc..PROMOTE.REPORT.
Explanation
Promote of the sample application failed.
SCLM messages
Chapter 3. SCLM messages  701

## Page 722

User response
Correct the errors in the messages and listings
data sets. Complete the sample project using the
instructions in the SCLM Project Manager's Guide,
Preparing the Example Project Data, by completing the
PROMOTE steps. You may have to repeat the BUILD
step.
FLMQ012 Project deleted - Project aaaaaaaa
was successfully deleted. All data
sets for the project were deleted.
Explanation
Deletion of the sample project was successful.
FLMQ013 Incomplete delete - Errors
occurred deleting data sets for
project aaaaaaaa.. TSO messages
should have been written to your
screen describing the errors. If
TSO messages were not written to
your screen, check the setting of
WTPMSG in your TSO PROFILE.
FLMQ014 Project not deleted - You did
not confirm deletion of project
aaaaaaaa.. Project aaaaaaaa was
not deleted.
FLMQ015 Project delete failed -
Errors occurred allocating
or reading the data set
aaaaaaaa..PROJDEFS.DATASETS
that contains the list of data sets
for project aaaaaaaa.. The project
cannot be deleted without this
data set.
Explanation
Data set aaaaaaaa..PROJDEFS.DATASETS contains
the list of data sets for the project. It is required to
delete the project.
User response
Correct the project name or determine why the
required data set is not available.
FLMQ016 Duplicate project - Data set
aaaaaaaa..PROJDEFS.DATASETS
already exists. This data set is
used to record the names of data
sets in the project.
Explanation
Data set aaaaaaaa..PROJDEFS.DATASETS used to
contain the list of data sets for the project already
exists. This project was already created or the data set
exists for some other purpose.
User response
Delete or rename the
aaaaaaaa..PROJDEFS.DATASETS data set.
FLMS001 The next group in the hierarchy is
aaaaaaaa.. It is bbbbbbbb..
Explanation
The NEXTGRP service determined the next group in
the hierarchy as requested. Whether the group is key
or non-key is also returned.
FLMS002 LMINIT failed - LMINIT for
messages ddname failed with
return code aaaaaaaa.. Make
sure ddname is properly allocated
before invoking service.
Explanation
LMINIT failed. The ddname may not be allocated.
User response
Make sure ddname is allocated with the correct
attributes before invoking service.
FLMS003 Browse failed - Attempt to browse
the messages data set failed.
If you have pre-allocated the
messages ddname make sure it is
correctly allocated before invoking
service.
Explanation
Browse failed for the messages data set.
User response
Make sure ddname is allocated with the correct
attributes before invoking service.
FLMS004 Group is top - The requested
group is already the top group in
the hierarchy. No next group is
available.
Explanation
The group is already the top group of the hierarchy.
SCLM messages
702  z/OS: z/OS ISPF Messages and Codes

## Page 723

FLMS005 Edit completed - The EDIT service
completed normally. No data
saved.
Explanation
The edit session ended without a request to save data.
FLMS006 aaaaaaaa allocated - The
DSALLOC service completed
successfully allocated aaaaaaaa..
Explanation
The DSALLOC service completed successfully.
FLMS007 Hierarchy requested - Group 2,
group 3, and group 4 must be
blank when Allocate Hierarchy is
selected.
Explanation
The DSALLOC service completed successfully.
FLMS008 Date required - If a time is
requested, the date is required.
Explanation
Both the date and time must be specified if either one
is specified.
User response
Specify the date or blank out the time.
FLMS009 Time required - If a date is
requested, the time is required.
Explanation
Both the date and time must be specified if either one
is specified.
User response
Specify the time or blank out the date.
FLMS100 Project invalid - SCLM is unable to
initialize project aaaaaaaa..
Explanation
The search for the project failed.
User response
Type the correct project name.
FLMS100A Project invalid - SCLM is unable
to initialize project aaaaaaaa or
alternate project bbbbbbbb..
Explanation
The search for the project or the alternate project
failed.
User response
Type the correct project name (and alternate, if one is
needed).
FLMS101 The authcode for aaaaaaaa
is changed from bbbbbbbb to
cccccccc..
Explanation
The AUTHCODE service changed the authorization
code for the requested member.
FLMS102 Authcode not changed - The
authcode did not need to be
changed. Either the authcode was
already as requested, or the
from authcode did not match, or
member was not editable.
Explanation
The AUTHCODE service did not need to change the
authorization code for the requested member.
FLMS103 Member at lower level - Member
exists at lower level with
a different authcode. Current
member may be overlayed during
promotion.
Explanation
The AUTHCODE service changed the authcode, but
there is a member at a lower group which has an
authcode which is not the same as the to authcode.
User response
Verify that the authcodes are correct at the lower level
and at the current level.
FLMS104 Group is top - The requested
group is already the top group in
the hierarchy. No next group is
available.
SCLM messages
Chapter 3. SCLM messages  703

## Page 724

Explanation
The group is already the top group of the hierarchy.
FLMS105 The authcodes for aaaaaaaa
are changed from bbbbbbbb to
cccccccc..
Explanation
The AUTHCODE service changed the authorization
codes for the requested members.
FLMS106 Authcodes not changed - At least
one authcode did not need to
be changed. Either the authcodes
were already as requested, or the
from authcodes did not match, or
members were not editable.
Explanation
The AUTHCODE service did not need to change the
authorization code for the requested members.
FLMS107 Authcodes displayed - The
authcode report for aaaaaaaa was
displayed.
Explanation
The AUTHCODE service either wrote the report to the
screen or displayed the data set allocated to the report
ddname.
FLMS108 The authcode for aaaaaaaa is
bbbbbbbb..
Explanation
The AUTHCODE service determined the authcode for
the member.
FLMS109 Authcode not changed - The
authcode did not need to be
changed. Either the authcode was
already as requested, or the
from authcode did not match, or
member was not editable.
Explanation
The AUTHCODE service did not need to change the
authorization code for the requested member.
FLMS200 aaaaaaaa failed - aaaaaaaa failed
with return code bbbbbbbb..
Explanation
The service received a failing return code.
FLMS201 aaaaaaaa locked - Member
aaaaaaaa successfully locked in
group bbbbbbbb type cccccccc..
The highest group it can be
promoted to is dddddddd.. It was
found in eeeeeeee..
Explanation
The LOCK service locked the member with the
requested parameters. The maximum promote group
and the found group are also returned.
FLMS202 aaaaaaaa unlocked - Member
aaaaaaaa successfully unlocked in
group bbbbbbbb type cccccccc..
Explanation
The LOCK service locked the member with the
requested parameters.
FLMS203 aaaaaaaa warning - aaaaaaaa
service complete with return code
bbbbbbbb..
Explanation
The service received a failing return code.
FLMS204 aaaaaaaa saved - Member
aaaaaaaa successfully saved in
group bbbbbbbb type cccccccc..
The highest group it can be
promoted to is dddddddd..
Explanation
The SAVE service locked the member with the
requested parameters. The maximum promote group
and the found group are also returned.
FLMS205 aaaaaaaa succeeded - aaaaaaaa
service complete with return code
0.
Explanation
The service completed successfully.
FLMS206 Table empty - aaaaaaaa service
completed with return code 0, but
at least one requested table was
empty.
Explanation
The service completed successfully. Either the user
data, change code, or include table was empty and did
not display.
SCLM messages
704  z/OS: z/OS ISPF Messages and Codes

## Page 725

FLMS207 No information deleted - No audit
record was found that matched
the specified criteria. No audit or
version information deleted.
Explanation
There was no audit record for the given data. Nothing
was deleted.
User response
Verify the SCLM library information, and use the
VERINFO service to verify the date and time. Make
sure the date and time are in the correct format.
FLMS208 aaaaaaaa not found - Project
aaaaaaaa is not found in data set
bbbbbbbb..PROJDEFS.LOAD.
Explanation
The project definition load library does not contain the
requested project.
User response
Verify the SCLM alternate project has been assembled
and link edited into the correct load library.
FLMS209 Project not found -
Project definition data set
aaaaaaaa..PROJDEFS.LOAD could
not be allocated.
Explanation
The project definition load library allocation failed.
User response
Verify the SCLM project name is correct. Verify that the
data set can be allocated.
FLMS210 User exits completed -
VERCC and/or CCVFY completed
successfully for member
aaaaaaaa in group bbbbbbbb type
cccccccc..
Explanation
VERCC , CCVFY exits were called for the requested
member. No locking, parsing, or storing of SCLM
accounting data occurred.
FLMS211 No user exits defined - No VERCC
or CCVFY exits were defined for
this project.
Explanation
The function executed successfully. VERCC and CCVFY
exits were not found.
FLMS300 aaaaaaaa invalid - Hundredths of
a second value is invalid.
Explanation
Hundredths of a second must be numeric.
User response
Enter the correct value.
FLMS301 aaaaaaaa invalid - Time must be
in bbbbbbbb format.
Explanation
Time must have the national time delimiter at
positions 3 and 6, and a decimal point at position 9.
All other characters must be numeric.
User response
Enter the time in the correct format.
FLMS302 aaaaaaaa not found - Member
aaaaaaaa is not found in
bbbbbbbb..cccccccc..dddddddd..
Explanation
The member must exist in the controlled library for the
SAVE service to work.
User response
Enter the correct member name, or to lock a member
that does not yet exist, use the LOCK service.
FLMS303 Type required - Type is required
for Search Type of Match or
Search.
Explanation
Match and Search must have type and member names.
User response
Enter a type name.
FLMS304 Member required - Member is
required for Search Type of Match
or Search.
SCLM messages
Chapter 3. SCLM messages  705

## Page 726

Explanation
Match and Search must have type and member names.
User response
Enter a member name.
FLMS305 Type required - Type is required
for Search Type of Backward or
Match.
Explanation
Match and Search must have type and member names.
User response
Enter a type name.
FLMS306 Member required - Member is
required for Search Type of
Backward or Match.
Explanation
Match and Search must have type and member names.
User response
Enter a member name.
FLMS307 Invalid value - Select one of the
available services.
Explanation
The user entered an invalid option.
User response
Specify one of the options listed.
FLMU000A ISPF Service Error - Return code:
aaaaaaaa from service: bbbbbbbb
for cccccccc.
FLMU000B ISPF Table Error - Return code:
aaaaaaaa from table service:
bbbbbbbb for table cccccccc.
FLMU000C Could not allocate table library.
Use the menu option 0 to set up
the library name.
FLMU000D Could not allocate table library.
RC aaaaaaaa received from the
bbbbbbbb service.
FLMU000E Error opening table aaaaaaaa. -
Table was not found. Run the
batch extract job to populate the
tables.
FLMU000F Could not open table aaaaaaaa. -
Table is in use.
FLMU001A Parameter error - Unknown
parameter: aaaaaaaa
FLMU001B Command error - Unknown
command: aaaaaaaa
FLMU003A Circular dependency - A circular
dependency has been detected.
FLMU004G End of chain - No dependent parts
found
FLMU008A Error determining account file
name(s)
FLMU008B LISTDSI rc=aaaaaaaa,
rs=bbbbbbbb - RC=aaaaaaaa,
RS=bbbbbbbb from LISTDSI for
cccccccc
FLMU008C Dataset not cataloged - aaaaaaaa
is not a cataloged dataset
FLMU008D DSINFO rc aaaaaaaa - RC
aaaaaaaa from DSINFO for
bbbbbbbb
FLMU008E Error accessing table library
aaaaaaaa
FLMU008F Canceled - Operation canceled
FLMU008G Invalid table dataset - Table
dataset does not exist or is blank
FLMU008H Extract tables invalid - The batch
extract job to populate the tables
did not complete successfully.
FLMU008I Extract tables not built - The batch
extract job needs to be run (option
1), to populate the tables.
FLMU008J Invalid project tables - The tables
that have been built are not
for the current project. Run the
Define SCLM project dependency
information tables job (option 1)
for this project.
FLMU008K Invalid alternate table - The tables
that have been built are not for the
current alternate project. Run the
Define SCLM project dependency
information tables job (option 1)
for this alternate project.
FLMU008L Table dataset open error - The
dataset containing the tables and
SCLM messages
706  z/OS: z/OS ISPF Messages and Codes

## Page 727

control member has received an
error during OPEN processing.
FLMU008M Specify table library - A SCLM
Explorer table library has not been
specified via option 0. Specify a
table library containing data built
for the current project.
FLMU009A Starting data extract...
FLMU009B Reading account file(s)....
FLMU009C Writing ISPF tables........
FLMU009E Starting dependency processing...
FLMU009F debug: aaaaaaaa - bbbbbbbb -
cccccccc - dddddddd
FLMU009G Error: aaaaaaaa from bbbbbbbb
cccccccc
FLMU009H Error: RC=aaaaaaaa,
REAS=bbbbbbbb from cccccccc
dddddddd
FLMU009S Acctfile read error:aaaaaaaa on
key bbbbbbbb
FLMU009Z Data extract ended.
FLM00000 MESSAGE ID aaaaa IS NOT
DEFINED
Explanation
An attempt to display a message failed because the
message ID does not exist.
User response
Contact the project manager.
System programmer response
Contact IBM support; this is an SCLM internal error.
FLM090 Unable to display panel - Unable
to display panel aaaaaaaa.
Explanation
The panel either does not exist in the linked panel
libraries, or the panel contains an error.
User response
Make sure the correct panel libraries are concatenated
to ISPPLIB. Use the TSO LISTALC command to display
concatenated libraries. If the panel exists in the library
concatenation, see your system programmer.
System programmer response
Determine if the panel has been altered at your site. If
not, contact IBM support.
FLM090A An error has occurred. Enter HELP
for a detailed description of the
error.
Explanation
This is an informational message.
FLM090B SCLM - Promote -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd)- RC=eeeeeeee,
MODE=ffffffff, gggggggg..
Explanation
This is an informational message.
User response
See the promote messages for return codes greater
than zero.
FLM090C SCLM - Build -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd)- RC=eeeeeeee,
MODE=ffffffff, gggggggg..
Explanation
This is an informational message.
User response
For return codes greater than zero, see the build
messages.
FLM090D Volume should be blank - Leave
VOLUME blank for command SUB
and LISTINGS destination data
set.
Explanation
Do not specify a volume name when submitting a
batch build in SCLM. SCLM uses the default volume.
User response
Blank out the volume name field.
FLM090F SCLM - Utility -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd) - Deleted eeeeeeee.
SCLM messages
Chapter 3. SCLM messages  707

## Page 728

Explanation
This is an informational message. SCLM completed the
request to delete member aaaaaaaa.
FLM090G SCLM - Saved -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd) - Member parsed and
stored.
Explanation
This is an informational message.
FLM090H SCLM - Utility -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd) - Auth code updated.
Explanation
This is an informational message.
FLM090I SCLM - Promote -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd)- MODE=eeeeeeee, ffffffff
job submitted.
Explanation
This is an informational message.
FLM090J SCLM - Build -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd)- MODE=eeeeeeee, ffffffff
job submitted.
Explanation
This is an informational message.
FLM090K Job submitted - Data set
aaaaaaaa submitted for printing
and deletion.
Explanation
This is an informational message.
FLM090L Data set deleted - SCLM deleted
data set aaaaaaaa without
printing it.
Explanation
This is an informational message.
FLM090M Vol aaaaaaaa not found - Volume
aaaaaaaa is not mounted or is not
authorized for your use.
Explanation
The volume could not be found or an error occurred
obtaining information about the volume.
User response
Request that the volume be mounted if it is not.
Otherwise, request the proper authorization.
FLM090N Vol aaaaaaaa not mounted -
Volume aaaaaaaa is not mounted
or is not authorized for your use.
Explanation
The volume could not be found or an error occurred
obtaining information about the volume.
User response
Request that the volume be mounted if it is not.
Otherwise, request the proper authorization.
FLM090O Browse failed - See SCLM systems
support or project administrator.
Explanation
This message is self explanatory.
FLM090P Data set kept - Data set aaaaaaaa
submitted for printing; data set
kept.
Explanation
This is an informational message.
FLM090Q Job submission error - Job
submission error. Invalid request.
Explanation
This is an informational message.
FLM090R Job submitted - SCLM submitted
the job request to the specified
output control.
Explanation
This is an informational message.
FLM090S Promote completed - The return
code from the Promote processor
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
SCLM messages
708  z/OS: z/OS ISPF Messages and Codes

## Page 729

Explanation
This is an informational message.
FLM090T Build completed - The return
code from the Build processor
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
Explanation
This is an informational message.
FLM090U Job queued - The job request is
queued for later submission as a
batch job.
Explanation
This is an informational message.
FLM090V Invalid option - The option you
entered is invalid.
Explanation
An invalid option was specified on the command line.
User response
Enter a valid option.
FLM090W Invalid value - You cannot select
PRINTER for EXECUTE.
Explanation
You can only request output to be routed to the printer
if you submit the job to batch.
User response
Either submit the job to batch, or route the output to
TERMINAL or data set.
FLM090X Invalid value - You cannot select
TERMINAL for SUBMIT.
Explanation
You can only request output to be routed to the
terminal if you execute the job in foreground.
User response
Either route the output to PRINTER or data set, or
execute the job in foreground.
FLM090Y Enter option - Enter one of the
listed options.
Explanation
No option was selected.
User response
Choose one of the options listed and press Enter.
FLM090Z Invalid option - Enter one of the
listed options.
Explanation
An invalid option was selected.
User response
Choose one of the options listed and press Enter.
FLM091 Arch report completed - The return
code from the Architecture Report
processor is aaaaaaaa.. Enter the
HELP command for a description
of common errors.
Explanation
This is an informational message.
FLM091B I/O error on Xref - An I/O error
occurred while retrieving a cross-
reference record.
Explanation
SCLM could not read a record from the VSAM cross-
reference file.
User response
Contact your project administrator.
Programmer response
Isolate the failure using IDCAMS after making sure the
cross-reference file exists as defined for this project.
FLM091C Xref not found - Cannot find
the cross-reference record for
selected Compilation Unit.
Explanation
This is an informational message.
User response
Save or migrate the member.
SCLM messages
Chapter 3. SCLM messages  709

## Page 730

FLM091D Xref mismatch - Mismatch of xref/
dependency information or xref/
code version.
Explanation
The member and dependency information are out of
synch.
User response
Parse the member and rebuild.
FLM091E Xref file missing - The project
definition does not include a
Cross-reference file.
Explanation
The cross-reference file as specified by the project
definition does not exist, or the project does not have a
cross-reference file defined to it.
System programmer response
If the file does not exist, allocate it; otherwise, define
the cross-reference file to the project.
User response
Contact your project administrator
FLM091F Requested accounting record is
not in the accounting file. Enter
HELP for a detailed description.
Explanation
SCLM could not find the accounting record in the
accounting VSAM file defined to this project.
User response
You can generate the accounting record for the
member using the SCLM migrate utility, or by editing
the member and issuing the SPROF command.
FLM091G Info/version mismatch - Mismatch
of Acct/seg dependency info or
acct rec/executing code version.
Explanation
The accounting segment dependency information or
accounting executing code version are mismatched.
User response
Parse and rebuild the member to correct.
FLM091H I/O error - An I/O error occurred
while reading from the accounting
file.
Explanation
A failure occurred either on OPEN or READ of the
accounting VSAM file.
System programmer response
Check that the accounting VSAM file for the project
exists. If it does, use IDCAMS or a similar tool to
isolate the I/O failure. If the accounting VSAM file is
unrecoverable, consider retrieving a backup, or use the
secondary accounting file if one was specified for the
project.
User response
Contact your project administrator.
FLM091I Database I/O error - An I/O
error occurred during a database
operation.
Explanation
SCLM could not read or update one of the VSAM files
defined to the project.
System programmer response
Check all VSAM files defined to the project using the
IDCAMS utility. If an alternate to the failing VSAM data
set exists, use it. Otherwise, consider using a previous
backup version.
User response
Contact your project administrator.
FLM091J Acct file I/O error - An I/O error
occurred while writing to the
accounting file.
Explanation
A failure occurred while writing to the accounting
VSAM file.
System programmer response
Verify that the accounting VSAM file for the project
exists. If it does, use IDCAMS or a similar tool to
isolate the I/O failure. If the accounting VSAM file is
unrecoverable, consider retrieving a backup or use the
secondary accounting file if one was specified for the
project.
SCLM messages
710  z/OS: z/OS ISPF Messages and Codes

## Page 731

User response
Contact your project administrator.
FLM091K Xref file I/O error - An I/O error
occurred while writing to the
cross-reference file.
Explanation
A failure occurred while writing to the cross-reference
VSAM file.
System programmer response
Determine if the cross-reference VSAM file for the
project exists. If it does, use IDCAMS or a similar
tool to isolate the I/O failure. If the cross-reference
VSAM file is unrecoverable, consider replacing it with a
previous backup version.
User response
Contact your project administrator.
FLM091L Acct. file I/O error - Error occurred
while updating the accounting file.
Return code = aaaaaaaa..
Explanation
A failure occurred on updating the accounting VSAM
file.
System programmer response
Check that the accounting VSAM file for the project
exists. If it does, use IDCAMS or a similar tool to
isolate the I/O failure. If the accounting VSAM file is
unrecoverable, consider retrieving a backup file, or use
the secondary accounting file, if one was specified for
the project.
User response
Contact your project administrator.
FLM091M Parsing error - Source contains
too many includes, compools, or
change codes.
Explanation
More information was found by the parser than the
project allows to be stored.
System programmer response
A restriction of the $list_info pointer parameter has
been exceeded. See ISPF Software Config ur ation 
and Library Manager (SCLM) Reference for more
information.
User response
If the member you are attempting to parse contains
many include directives, consider dividing the member
into two or more members. Otherwise, contact your
project administrator.
FLM091N Parser not specified - Language
definition does not include a
parser or parser data set name.
Explanation
Either SCLM could not locate the parser as specified in
the language definition, or a parser was not specified
in the language definition.
System programmer response
If the parser was correctly specified by name,
make sure the parser exists in one of the user's
concatenated libraries. Otherwise, specify the data set
where the parser can be found using the DSNAME=
parameter on the FLMTRNSL macro command.
User response
Contact your project administrator.
FLM091O Parsing error - Source contains
too many compilation unit
dependencies.
Explanation
The source member exceeded the space available
for associating a member's compilation unit
dependencies.
User response
Consider breaking the member into two or more
members.
FLM091P Authorization code warning. Enter
HELP for a detailed description.
Explanation
WARNING. Member exists at a lower group with an
authcode not equal to the new authcode which could
overlay the current member.
SCLM messages
Chapter 3. SCLM messages  711

## Page 732

FLM091Q Data set kept - SCLM kept data set
aaaaaaaa without printing it.
Explanation
This is an informational message.
FLM092A Default job information - The
cursor is placed at a default value.
Verify the value as correct.
Explanation
This message is self explanatory.
FLM092B Invalid job information - The
cursor is placed at the incorrect
value. Enter a correct value.
Explanation
The value at the cursor location is not valid.
User response
Update the value at the cursor location to be a valid
value.
FLM092C Batch - Submit - Job aaaaaaaa
submitted.
Explanation
This is an informational message.
FLM092D Error retrieving SCLM ID - An error
occurred while retrieving SCLM ID.
Explanation
This message is self explanatory.
FLM092E Invalid value - Enter one of the
listed values, YES or NO.
Explanation
This message is self explanatory.
FLM092F Enter required field - Enter CU
Qualifier Name or enter HELP for
more information.
Explanation
The CU Qualifier Name must be entered to proceed.
User response
Enter a CU Qualifier Name.
FLM092G Invalid option - Enter one of the
option numbers listed or enter
HELP.
Explanation
No option from the menu was selected.
User response
Select an option to proceed.
FLM092H Invalid option - Enter A, M, B, D, E,
V, C, P, U or blank. Enter the HELP
command for further information.
Explanation
The option entered is not a valid Library Utility
command.
User response
Choose one of the options listed or enter the HELP
command for additional information.
FLM092I Invalid command - Command
aaaaaaaa is undefined. Enter END
to cancel.
Explanation
An invalid command was entered on the command
line.
User response
Enter END to cancel and try again with a valid
command.
FLM092J Invalid command - Enter either
EXECUTE or SUBMIT for a valid
command or enter HELP.
Explanation
You are only allowed to enter EXECUTE, SUBMIT, or
END here.
User response
Choose a valid response.
FLM092K Invalid member name -
Enter a valid member name
which consists of alphanumeric
characters.
SCLM messages
712  z/OS: z/OS ISPF Messages and Codes

## Page 733

Explanation
An invalid member name was entered.
User response
Enter a valid member name.
FLM092L Enter option - Enter one of the
option numbers listed or enter
HELP.
Explanation
No option from the menu was selected.
User response
Select an option to proceed.
FLM092M Enter option - Enter A, B, D, M, U,
V or blank, or enter HELP for more
information.
Explanation
No option was entered.
FLM092N Enter command - Command
aaaaaaaa is undefined. Enter END
to cancel.
Explanation
The command that was entered is not valid on this
panel.
User response
Correct the command.
FLM092O Enter command - Enter either
EXECUTE or SUBMIT for a valid
command or enter HELP.
Explanation
Enter EXECUTE, SUBMIT, or END.
User response
Choose the proper command.
FLM092P Invalid member - Accounting
information for member not found.
Explanation
SCLM could not find the accounting record in the
accounting VSAM file defined to this project.
User response
You can generate the accounting record for the
member using the SCLM migrate utility, or by editing
the member and issuing the SPROF command.
FLM092Q Duplicate found - Duplicates are
not allowed.
Explanation
This message is self explanatory.
FLM092R Invalid value - Enter S, I, D, R or
press HELP for more information.
Explanation
This message is self explanatory.
FLM092S Enter required field - Enter the
data specified in the required
field.
Explanation
Enter a value for the field where the cursor is
positioned.
User response
Enter a valid value.
FLM092T Invalid promote to-group -
Promote to-group must be a group
in the project.
Explanation
The promote service was invoked with an invalid group
name for the promote-to-group.
User response
Correct the name of the promote-to-group and reissue
the command.
FLM092U Error opening table - Error opening
ISPF table.
Explanation
This message is self explanatory.
FLM092V Enter required field - Enter a valid
member name which consists of
alphanumeric characters.
Explanation
An invalid member name was entered.
SCLM messages
Chapter 3. SCLM messages  713

## Page 734

User response
Enter a valid member name.
FLM092W Member aaaaaaaa of bbbbbbbb -
Member aaaaaaaa of bbbbbbbb
FLM092X aaaaaaaa
Explanation
A single error occurred during SCLM processing.
User response
For further information about the message, an SCLM
message id appears at the beginning of the message.
The format of the message id is FLMxxxxx, where
xxxxx is a five digit number. The message is described
in ISPF Messages and Codes .
FLM092Y NOPROM update invalid -
The Accounting record you
are attempting to update
does not have an accounting
status of EDITABLE, NOPROM-
N or NOPROM-R. Updating the
accounting record is not possible.
Explanation
An attempt is being made to update a member with
accounting record which is not EDITABLE, NOPROM-N
or NOPROM-R. Only members which are EDITABLE
or if a member has previously set as being not
promotable are able to be updated.
FLM092Z REMOVE option invalid - Remove
(option 3) is invalid against a
member which has an EDITABLE
accounting status. Only members
which have previously been
specified as been not promotable
can use the remove option.
Explanation
An attempt is being made to update a member
with accounting record which is not NOPROM-N or
NOPROM-R. Only members which have previously
been set as being not promotable can use remove
(option 3).
FLM093A Member not updated - You used
the END or RETURN command to
terminate the request.
FLM093B Panel display error - An error was
encountered attempting to display
the panel FLMUSN#P.
FLM093C SCLM - Utility -
aaaaaaaa..bbbbbbbb..cccccccc(dd
dddddd) - accounting status
updated to eeeeeeee.
Explanation
This is an informational message.
FLM093D NOPROM update invalid - The
NOPROM line command cannot
be issued against an ARCHDEF
member. An Archdef member
is one with a language that
has ARCH=Y specified on the
FLMLANGL macro.
Explanation
An attempt is being made to update a ARCHDEF
member. This is not possible.
FLM093E Build Map Error - Insufficient
storage to review the build map.
User response
Increase the TSO region size.
FLM093F Build Map Error - Error
displaying panel FLMUSBRP. Error
message:aaaaaaaa
Explanation
This is an informational message.
User response
Determine why the panel FLMUSBRP failed to be
displayed.
FLM093G Build Map Error - Error calling
FLMTERSE to restore backup
member. RC=aaaaaaaa Error
text:bbbbbbbb
Explanation
This is an informational message.
User response
Determine there was an error calling FLMTERSE.
FLM093H Build Map Error - Error viewing
the temporary file containing the
NOPROM backup member. Error
message:aaaaaaaa
SCLM messages
714  z/OS: z/OS ISPF Messages and Codes

## Page 735

Explanation
This is an informational message.
User response
Determine there was an error calling FLMTERSE.
FLM093I Build Map Error - Error retrieving
data set information for aaaaaaaa
to allocate a temporary data set
to restore the NOPROM backup
member into.
User response
Determine why there was an error retrieving the data
set information.
FLM093J Build Map Error - Error allocating
temporary data set to restore the
NOPROM backup member into.
User response
Determine why there was an error retrieving the data
set information.
FLM093K No backup taken - The not
promoted member was not backed
up as a part of the promotion
process. Check the promotion
messages to see why this
occurred.
Explanation
This is an informational message.
User response
Determine why the not promoted member was not
backed up.
FLM093L Update not allowed - The SCLM
administrator has restricted the
use of the Change promote
processing using the FLMNPROM
macro. Please see the SCLM
administrator if this is a problem.
Explanation
The SCLM Administrator coded FLMNPROM macros
for the SCLM project definition. These macros tell
SCLM which groups, types and languages the N line
command in Library Utilities or utility of work can be
issued against.
FLM093M Noprom not allowed - Member
aaaaaaaa must exist at the
requested group bbbbbbbb to
change promote processing.
Explanation
You have requested a change of promote processing
at bbbbbbbb level, but SCLM found the member's
accounting information at a higher level.
User response
Either draw down the member to the group specified,
or specify the group the member accounting info exists
in on the Library Utility panel.
FLM290 aaa(24) - bbb(512)
Explanation
An error occurred in a non-SCLM service. The text of
this message contains the message returned by the
failing service.
User response
Refer to the text of the message to determine the
cause of the failure.
FLM290A Invalid macro parameter - Invalid
macro parameter aaa(80)..
Explanation
An invalid parameter was specified for an edit macro.
The long description contains the parameter in error.
User response
Correct the invalid parameter.
FLM290B Parser error. Enter HELP for a
detailed description of the error.
Explanation
The parse of the member that occurs while saving the
member was not successful.
System action
The member data is saved to the PDS data set and an
accounting record with LOCKOUT state is saved, but
accounting statistics and dependency information that
may have been returned by the parser are not saved.
SCLM messages
Chapter 3. SCLM messages  715

## Page 736

User response
Enter HELP to display a panel with any messages that
were generated by the parser.
Problem determination
Refer to the parser messages to determine the actions
to be taken to correct the problem.
FLM290C Member in use - Either you or
another user is updating member
aaaaaaaa..
Explanation
The member is currently being used in another
operation. The member may be in use by:
• Another user
• The other section of a split screen
• A recursive edit session.
User response
The user's action cannot be performed until the
current operation using the member has been
completed.
FLM290D Recursion unavailable - SCLM
View or Edit cannot be invoked
while the current SCLM edit
session is active. Use split screen
or end the current SCLM edit
session.
Explanation
An attempt was made to enter SCLM View or Edit while
an SCLM edit session was active.
User response
Return to the current edit session and end the edit, or
use the View or Edit primary commands. Alternatively,
use split screen to perform the intended operation.
FLM290E Save warnings. aaaaaaaa has
been stored. Enter HELP to display
the warning messages.
Explanation
The member was saved but warning messages were
generated while saving the member.
System action
The member aaaaaaaa was saved and the accounting
information was updated.
User response
Enter HELP to display the warning messages and
determine the actions to be taken based on the
messages displayed.
FLM290F aaaaaaaa macro error - Edit initial
macro requested is unknown or
returned a nonzero return code.
Explanation
The initial edit macro from the edit panel could not
be found or invoked or the macro returned a nonzero
return code.
User response
Change the field on the edit panel to specify a valid
edit macro name or leave the field blank.
FLM290G Group warning - Group aaaaaaaa
is not in the hierarchy view for
group bbbbbbbb..
Explanation
The hierarchy view for group bbbbbbbb is the list of
groups from bbbbbbbb to the top of the hierarchy as
defined in the project definition. This message is a
warning that the group aaaaaaaa is not in that list
of groups. Specifying groups out of the hierarchy view
may result in other error messages when you attempt
to edit a particular member.
User response
Verify that the groups entered on the edit panel are in
the desired order.
FLM290H aaaaaaaa saved - Member
aaaaaaaa has been parsed and
stored by SCLM.
Explanation
This is an informational message. The edit, parse,
and store of member aaaaaaaa were all completed
successfully.
FLM290I Invalid dev group - Group
aaaaaaaa is not the lowest group
in the hierarchy.
SCLM messages
716  z/OS: z/OS ISPF Messages and Codes

## Page 737

Explanation
The operation being requested may result in the
update of an SCLM-controlled member, but the
specified group is not a development group in the
project hierarchy. A development group is a group
which has no other groups promoting into it. This
operation can only be performed on members that are
in a development group.
User response
Specify a development group for the operation.
FLM290J Library not specified - You must
specify group aaaaaaaa on the
SCLM Edit Entry panel.
Explanation
The user requested to edit a member found in a group
that was not specified on the edit panel. This can occur
if:
• The user has changed the groups listed on the edit
panel.
• The hierarchy is more than 4 groups in depth.
SCLM uses the hierarchical view defined in the project
definition when searching for members' accounting
information. If the accounting information is found in
the hierarchical view but the group where it is found
is not specified on the edit panel, this message is
issued. The group aaaaaaaa is the group where the
accounting information was found.
User response
Update the groups listed on the edit panel to include
group aaaaaaaa.
FLM290K Invalid label - You must specify a
label parameter no longer than 6
characters.
Explanation
The label specified for CREATE, REPLACE, or MOVE
was longer than 6 characters in length. Edit labels
must be 6 characters or fewer in length.
User response
Enter the command specifying a valid edit label.
FLM290L Invalid group - This project does
NOT include group aaaaaaaa..
Explanation
The group, aaaaaaaa, specified by the user is not a
group defined in the project definition.
User response
Specify a group defined in the project definition.
FLM290M Invalid library order - Groups not
in hierarchical order. Enter HELP
to display group list.
Explanation
A group higher in the hierarchy cannot be specified
before a group lower in the hierarchy on the edit panel.
At least one of the groups specified on the edit panel is
not in hierarchical order.
User response
Specify the groups in hierarchical order.
Problem determination
Enter HELP to display up to 16 groups from the
hierarchy in hierarchical order, beginning with the
development group from the edit panel.
FLM290N Invalid member name - The
member name parameter is
limited to 8 characters.
Explanation
A member name was entered that is longer than 8
characters.
User response
Enter a valid member name.
FLM290O Authorization code aaaaaaaa is
invalid for group bbbbbbbb.. Enter
HELP to display an authorization
code list.
Explanation
Authorization code aaaaaaaa was not defined as
a valid authorization code for group bbbbbbbb in
the project definition. The authorization code was
specified on the SCLM edit panel.
SCLM messages
Chapter 3. SCLM messages  717

## Page 738

User response
Either specify a valid authorization code for the group,
or leave the authorization code field blank on the edit
panel.
Problem determination
Enter HELP to display a list of 30 valid authorization
codes for group bbbbbbbb.
FLM290P Multicultural support table error -
An error occurred while retrieving
the multicultural support ID; code
= aa.
Explanation
The NLS ID could not be retrieved when running a
macro. The error codes are:
4
The ID is not loaded.
8
The ID is not valid.
12
The ID is not initialized.
System action
The macro is not run.
System programmer response
Contact IBM support.
User response
Exit SCLM and ISPF then retry the operation. If the
problem persists, contact the system programmer.
FLM290Q Invalid parameter - Command
format is 'SCREATE|SREPLACE
member {label1 label2}'.
Explanation
An invalid parameter was specified on an SCREATE or
SREPLACE macro command.
User response
Make sure that a member name was specified. If
labels are specified, then both label1 and label2 must
be specified and the labels must have a "." (period) as
the first character.
FLM290S Promotion restricted - Member
can only be promoted to
group aaaaaaaa with auth. code
bbbbbbbb..
Explanation
The member being edited can only be promoted to
group aaaaaaaa using authorization code bbbbbbbb.
This is an informational message and no action needs
to be taken unless there will be a need to promote the
member past group aaaaaaaa.
User response
If the member needs to be promoted past group
aaaaaaaa, change the authorization code using the
SCLM library management utility.
FLM290T Command conflict - You entered
incomplete or conflicting line
commands.
Explanation
The current edit line commands are incomplete or
conflicting. For example, an incomplete line command
might have the beginning of a block to be copied
specified by "CC" but no end line specified. An
example of a conflicting edit line command is a block
copy where the target of the copy is within the block
being copied.
System action
The SCREATE, SMOVE, or SREPLACE operation is not
performed.
User response
Ensure that the line commands in the current edit
session are complete and do not conflict. The RESET
edit command can be used to reset the line commands
within the current edit session. Enter HELP when this
message is displayed to get more information on the
edit line commands.
FLM290U No members found - The hierarchy
contains no members, or no
members match the pattern.
Explanation
If a pattern was specified on the panel, then no
members were found in the hierarchy of groups from
the panel that match the pattern. If no pattern was
specified, then no members were found in any of the
groups.
SCLM messages
718  z/OS: z/OS ISPF Messages and Codes

## Page 739

User response
To browse or edit an existing member, ensure that the
groups, type, and pattern will find the desired member.
To edit a new member in an empty data set, specify
the member name on the edit entry panel.
FLM290V The project definition does not
include language aaaaaaaa..
Explanation
The language specified for the member being edited is
not defined to the project.
User response
Enter a language from the project definition. Enter
HELP to display a list of 30 languages from the project
definition.
FLM290W Member already exists - aaaaaaaa
exists at a higher group in the
hierarchy.
Explanation
The member aaaaaaaa exists at a higher group in
the hierarchy but does not exist at the group where
the edit is taking place. SCREATE requires that the
member not exist, and SREPLACE requires that either
the member not exist or that it exist at the group
where the edit is taking place.
User response
To use SCREATE to create a new SCLM member, make
sure that the member name being specified does
not exist. To use SREPLACE, ensure that either the
member does not exist or that the member is at the
group where the edit is taking place.
FLM290X Recovery failed - Edit recovery
failed for aaa(44)(bbbbbbbb).
Explanation
Edit recovery failed for member bbbbbbbb in data set
aaa(44).
User response
Review the Edit Recovery documentation for possible
reasons that edit recovery has failed.
FLM290Y You cannot use language
aaaaaaaa for editable members.
Explanation
Language aaaaaaaa is not defined as editable in the
project definition. Members that are edited must be
specified as editable in the project definition.
User response
Specify a language that was defined as editable in the
project definition. Enter HELP to display a selectable
list of editable languages from the project definition.
FLM290Z Invalid parameter - Command
format is 'SMOVE member {AFTER|
BEFORE label}'.
Explanation
An invalid parameter was specified on an SMOVE
macro command.
User response
Make sure that a member name was specified. If
AFTER or BEFORE was specified, then a label must be
specified and the label must have a "." (period) as the
first character. Enter HELP for more information on the
SMOVE macro command.
FLM291 Allocation error - SCLM cannot
allocate the libraries you
requested on the Edit or View
panel, or from the EDIT service.
Explanation
SCLM is unable to allocate or open the data set(s)
associated with the hierarchy specified on the Edit or
View panel.
System programmer response
Have the SCLM administrator ensure that the data
sets are properly allocated and that the users have
sufficient authority.
User response
Check that the data sets are not allocated exclusively
to another user.
FLM291A Change code unavailable - No
previous change code exists for
this member.
SCLM messages
Chapter 3. SCLM messages  719

## Page 740

Explanation
Either the member did not previously exist or the
accounting information for the member does not
contain any change codes.
User response
Specify a change code instead of entering "=" to
retrieve the previous change code.
FLM291B Invalid command - Command
aaa(65) is invalid.
Explanation
The command aaa(65) is not a valid command.
User response
Enter a valid change code or language and press Enter
to update the language or change code of the member.
Enter CANCEL to cancel the update. Enter a valid
command on the command line to have the command
processed.
FLM291C Command ignored - A RETURN
or EXIT command was ignored in
order to display the SCLM Edit
Profile panel.
Explanation
SCLM must have the language for a member in order to
save it. The RETURN or EXIT command was ignored in
order to obtain the language for the member.
User response
Enter a valid language for the project.
FLM291D This member cannot be promoted
using auth. code aaaaaaaa.
Explanation
Authorization code aaaaaaaa is not allowed for the
next group in the hierarchy. This member cannot be
promoted using this hierarchy unless the authorization
code is changed.
User response
If the member needs to be promoted using this
hierarchy, the authorization code needs to be changed
prior to promoting the member. The SCLM library
utility can be used to change the authorization code.
FLM291E aaaaaaaa replaced - Member
aaaaaaaa has been replaced,
parsed, and stored by SCLM.
Explanation
This is an informational message. Member aaaaaaaa
was replaced using the SREPLACE macro command.
FLM291F aaaaaaaa moved - Member
aaaaaaaa has been moved and
associated accounting information
deleted.
Explanation
This is an informational message. Member aaaaaaaa
was moved using the SMOVE macro command. The
new member has been created and the old member
deleted.
FLM291G aaaaaaaa created - Member
aaaaaaaa has been created,
parsed, and stored by SCLM.
Explanation
This is an informational message. Member aaaaaaaa
was created using the SCREATE macro command.
FLM291H Member is empty - Member
aaaaaaaa contains no data.
Explanation
View, SCREATE, or SREPLACE was attempted for an
empty member. These operations require a non-empty
member.
User response
Enter data into the member prior to using View,
SCREATE, or SREPLACE.
FLM291I Member was not found - The
specified member, aaaaaaaa, was
not found.
Explanation
Member aaaaaaaa was not found in the specified
hierarchy. Members must exist prior to viewing them.
User response
Specify the name of an existing member for View.
FLM291K Group(s) unallocated - Unallocated
groups specified. Press HELP for
LIB number information.
SCLM messages
720  z/OS: z/OS ISPF Messages and Codes

## Page 741

Explanation
At least one data set for a group in the hierarchy does
not exist although the data set(s) for group(s) above
it do exist. This will result in the LIB numbers not
matching the group numbers from the panel where the
hierarchy of groups was specified.
System programmer response
Have the SCLM administrator allocate all data sets if
the LIB numbers need to match the member's position
in the hierarchy.
User response
Do not use the LIB numbers to determine the position
of a member in the hierarchy. Enter HELP when the
message is displayed to receive more information.
FLM291L Selection Cancelled - CANCEL
requested. No language was
selected.
Explanation
Language selection was exited with the CANCEL
command. No language was selected.
User response
User must enter a valid language for the project.
FLM291M All groups unallocated - At least
one group must be allocated.
Explanation
At least one group must have an allocated data set in
order to perform this operation.
System programmer response
Have the SCLM administrator ensure that all needed
data sets are allocated.
User response
Contact the project administrator to allocate the
needed data sets.
FLM291N Group1 not cataloged - The data
set for the first group 'aaa(44)'
must be cataloged.
Explanation
Edit requires that the data set for the first group in the
hierarchy be allocated. The data set name is aaa(44).
System programmer response
Have the SCLM administrator ensure that development
groups in the hierarchy have the proper data sets
allocated.
User response
Contact the project administrator to allocate the data
set.
FLM291O Locate string too long - Locate
string must be less that or equal
to 10 characters.
Explanation
The string to be located was longer than 10 characters.
User response
Specify a valid locate string of 10 characters or less in
length.
FLM291P Locate string too long - Locate
string "name" must be less than or
equal to 110 characters.
Explanation
The name to be located was longer than a valid
compilation unit name.
User response
Specify a valid locate name of 110 characters or fewer
in length.
FLM291Q Type is too long - The type must
be fewer than or equal to 8
characters.
Explanation
The type name specified is longer than 8 characters.
User response
Either specify a type name pattern (that may be longer
than 8 characters), or specify a valid type name of 8
characters or fewer.
FLM291R Data set not cataloged - 'aaa(44)'
was not found in catalog.
Explanation
A non-SCLM controlled group was specified, but no
data set exists for the group.
SCLM messages
Chapter 3. SCLM messages  721

## Page 742

User response
Either specify an SCLM group name, or allocate the
data set aaa(44).
FLM291T SCLM internal error - Contact IBM
support for assistance.
Explanation
There was an error while allocating or opening a data
set for View.
System programmer response
Contact IBM support.
User response
Contact the project administrator.
FLM291U Data loss warning. aaaaaaaa has
been locked. Enter HELP to display
the warning messages.
Explanation
The accounting and PDS data for the member being
edited are not in sync.
System action
Edit continues.
User response
Enter HELP to display the panel with additional
messages.
FLM291V Group name is too long - The group
name must be fewer than or equal
to 8 characters.
Explanation
The group name is longer than 8 characters.
User response
Either specify a group name pattern (that may be
longer than 8 characters), or specify a valid group
name of 8 characters or fewer.
FLM291W Parameter required - A locate
string must be specified after the
LOCATE command.
Explanation
The locate string parameter is required for LOCATE
commands.
User response
Specify the string you want to LOCATE in the member
list.
FLM291X Invalid command - Command
aaa(65) is invalid.
Explanation
The command aaa(65) is not a valid command.
User response
Specify a valid command. Enter HELP when this
message is displayed for command information.
FLM291Y Invalid authcode - Commas are
not allowed in authorization
codes.
Explanation
The authorization code specified contains one or more
commas. The authorization code is not valid.
User response
Specify a valid authorization code. Contact the project
administrator to obtain valid authorization codes, if
necessary.
FLM291Z Invalid change code - Commas are
not allowed in change codes.
Explanation
The change code specified contains one or more
commas. The change code is not valid.
User response
Specify a change code that does not contain commas.
FLM292A Invalid data set name - Member
names are not allowed for
retrieval data sets.
Explanation
Member names are not allowed for non-SCLM
controlled retrieval data sets.
SCLM messages
722  z/OS: z/OS ISPF Messages and Codes

## Page 743

User response
Enter either a sequential data set name or a
partitioned data set name without a member name.
FLM292B Invalid sort field - The sort field
name must match a heading on the
panel.
Explanation
The field name specified on the sort command must
match one of the field headings displayed on the
panel.
User response
Specify a valid field heading from the panel.
FLM292C Invalid language - The language
you are locating cannot be more
than eight characters long.
Explanation
The language parameter of the locate command must
have eight or fewer characters.
User response
Specify a valid language on the locate parameter.
FLM292D Enter a language - Enter a
language to locate.
Explanation
The locate command requires a language parameter.
User response
Specify a valid language on the locate parameter.
FLM292E Language not found - The language
you are locating is not found.
Explanation
The language specified is not in the list of valid
languages.
User response
Verify that the language name was typed correctly. If
not, retype the command with the correct language. If
so, see your project administrator.
FLM292F Invalid command - Valid
commands are CANCEL, LOCATE,
and END.
Explanation
The command is not valid.
User response
Enter a valid command.
FLM292G Select a language - One and only
one language can be assigned to a
member.
Explanation
More than one language was selected from the list.
User response
Delete all but one S from the selection field.
FLM292H No languages - There are no
languages defined in the project.
See your project administrator.
Explanation
SCLM must have the languages defined for a project in
order to save members.
User response
See your project administrator.
Programmer response
Add a language definition for each programming
language in the project.
FLM292I Invalid parameter - The hierarchy
view value must be ON or OFF.
Explanation
The hierarchy view value specified on the HIER
command must be ON or OFF.
User response
Specify ON or OFF for the hierarchy view value.
FLM292J Invalid parameter - The member
name specified must be 8
characters or less.
Explanation
The member name parameter for a Library Utility
primary command must be less than or equal to 8
characters in length.
SCLM messages
Chapter 3. SCLM messages  723

## Page 744

User response
Specify a valid member name for the Library Utility
command.
FLM292K Parameter required - A member
name must be specified after the
Library Utility primary command.
Explanation
The member name parameter is required for these
Library Utility member list primary commands: A, B, M,
D, E, V, C, P or U.
User response
Specify a member name for the Library Utility
command.
FLM292L Archdef not current - The archdef
member has been saved but not
built. SCLM generated the member
list by parsing the archdef member
instead of using the associated
build map. This could cause poor
response times.
Explanation
Unit of work will normally use the build map to
generating a list of members to display. However since
the archdef member was updated the archdef member
was used to generate the list of members.
User response
Generate the archdef member to improve response
times when generating the member list in unit of work.
FLM292M Decode failed - An error occurred
during the decoding of a
member which belongs to the
SCLM group aaaaaaaa while
returning statistics for the dataset
bbbbbbbb.
Explanation
This message is self explanatory.
User response
Determine why SCLM was unable to retrieve statistics
for the mentioned data set.
FLM292N Encode failed - An error was
received trying to allocate a
temporary data set into which
SCLM will encode the member
associated with the SCLM group
aaaaaaaa.
Explanation
This message is self explanatory.
User response
Determine why SCLM was unable to allocate the
mentioned data set.
FLM292O Decode failed - An error was
received trying to decode the
member in the SCLM group
aaaaaaaa. The error message
returned from the decode routine
stated bbbbbbbb.
Explanation
This message is self explanatory.
User response
Determine why SCLM was unable to decode the
mentioned member.
FLM292P Invalid parameter - Command
format is 'SCOPY member {AFTER|
BEFORE label}'.
Explanation
An invalid parameter was specified on an SMOVE
macro command.
User response
Make sure that a member name was specified. If
AFTER or BEFORE was specified, then a label must be
specified and the label must have a "." (period) as the
first character. Enter HELP for more information on the
SCOPY macro command.
FLM292Q aaaaaaaa copied - Member
aaaaaaaa has been copied.
Explanation
This is an informational message. Member aaaaaaaa
was copied using the SCOPY macro command.
FLM292R Allocation Error - Error allocating
the temporary data set containing
the decoded member.
SCLM messages
724  z/OS: z/OS ISPF Messages and Codes

## Page 745

Explanation
This is an informational message. The Member was
encoded when saving the member the language was
modified. This new language has ENCODE=N, when
SCLM attempts to allocate the temporary data set
containing the decoded member it received an error.
FLM292S Allocation Error - Error allocating a
temporary SYSPRINT data set that
will be used to copy the member
from the temporary data set
containing the decoded member
into the development data set.
Explanation
This is an informational message. Error allocating a
temporary SYSPRINT data set that will be used to copy
the member from the temporary data set containing
the decoded member into the development data set.
FLM292T Invalid parameter - Command
format is 'SCOMPARE {dataset|
member|NEXT|SESSION} eXclude'
Explanation
An invalid parameter was specified on an SCOMPARE
macro command.
User response
Make sure that the parameter were specified correctly.
Enter HELP for more information on the SCOMPARE
macro command.
FLM292U Invalid type name - Name
aaaaaaaa contains an invalid
character or an * is not the last
character.
Explanation
Invalid type name.
User response
Correct the name.
FLM292V Invalid member name - Name
aaaaaaaa is invalid. Enter a valid
member name. The first and the
last characters can be an *, but **
is invalid.
Explanation
Invalid member name.
User response
Correct the name.
FLM292W Extraneous search string - String
aaaaaaaa is too long. Reduce
combined length of the search
strings at this screen. Additional
search data can be entered using
the Statements DSN.
Explanation
Total length of search strings is too high.
User response
Reduce total length of the search strings
FLM292X No members to display -
No members matching to the
specified pattern found.
Explanation
No members matching to the specified pattern found.
User response
Enter a different member pattern.
FLM292Y Table display error - Can not
display member list.
Explanation
Can not display member list.
User response
Notify IBM.
FLM292Z Invalid search-string - Matching
end quote around string missing.
Explanation
Matching end quote around string missing.
User response
Correct the search string.
FLM293 No members were selected - No
members were selected at any of
the displayed selection lists.
SCLM messages
Chapter 3. SCLM messages  725

## Page 746

Explanation
You did not make any selections or typed CANCEL at
each single member list screen.
User response
Start again
FLM293A Invalid embedded quote - A quote
may be used as a delimiter or
specified as two quotes.
Explanation
Invalid embedded quote
User response
Correct the search string.
FLM293B C invalid for string one -
Continuation operand valid on any
or all subsequent search strings.
Explanation
C invalid for string one
User response
Correct the search string.
FLM293C Null string invalid - A null string
is an unacceptable SUPERC search
argument.
Explanation
Incorrect search string
User response
Correct the search string.
FLM293D Invalid search-qualifier - Must
be WORD, PREFIX, SUFFIX or C
(continued).
Explanation
A search-qualifier Must be WORD, PREFIX, SUFFIX or
C (continued).
User response
Correct the search string.
FLM293E Invalid hex string - The hex string
has an invalid character within the
specified string.
Explanation
The hex string has an invalid character
User response
Correct the search string.
FLM293F Hex character not paired - The hex
string has an uneven pair of hex
characters.
Explanation
The hex string has an uneven pair of hex characters.
User response
Correct the search string.
FLM293G Invalid imbedded blank -
Imbedded blanks. must be
enclosed in quotes.
Explanation
Imbedded blanks. must be enclosed in quotes.
User response
Correct the search string.
FLM293H Stmts Dsn does not exist -
'aaa(56)' was not found in
catalog. Statements Dsn field
must be blank or specify an
existing sequential FIXED 80
LRECL dataset.
Explanation
Statements Dsn does not exist.
User response
Correct the Statements Dsn field
FLM293I Stmts Dsn invalid length - Length
must be 1 to 44 characters
including prefix.
Explanation
Length must be 1 44 characters including prefix.
SCLM messages
726  z/OS: z/OS ISPF Messages and Codes

## Page 747

User response
Correct the Statements Dsn field
FLM293J Stmts Dsn invalid attr. - 'aaa(44)'
has invalid attributes. Statements
Dsn field must be blank or specify
an existing sequential FIXED 80
LRECL dataset.
Explanation
Statements Dsn has invalid attributes.
User response
Correct the Statements Dsn field
FLM293K aaaaaaaa is in use. - Wait until
your running SCLM SEARCH Jobs
are finished and try again.
Explanation
Can not retrieve ZTEMPF variable.
User response
Try later
FLM293L Invalid group name - Name
aaaaaaaa contains an invalid
character or an * is not the last
character. No * allowed when
hierarchy search is selected.
Explanation
Invalid group name.
User response
Correct the name.
FLM293M Search string required - Search
string must be entered because
your Statements dataset does not
include any SRCHFOR.
Explanation
At least one Search string is required
User response
Enter at least one Search string.
FLM293N Invalid input - Additional Search
strings panel will not be displayed
because your Statements dataset
includes at least one SRCHFOR.
Explanation
Statements dataset includes at least one SRCHFOR.
No Search string will be used.
User response
De-select Additional search strings field.
FLM293O Member name required - Member
name is required for this
partitioned data set.
Explanation
Member name required.
User response
Enter member name.
FLM293P Invalid DSN - member - Member
name of data set name must be
1-8 chars and enclosed in ( ).
Explanation
Invalid member name.
User response
Correct member name.
FLM293Q Invalid member name - The
member name entered as part of
the data set name is invalid. Enter
up to 8 alphanumeric chars (first
must be alphabetic)
Explanation
Invalid member name.
User response
Correct member name.
FLM293R Invalid DSN - qualifier -
Each qualifier must be 1-8
alphanumeric characters, the first
alphabetic.
Explanation
Invalid DSN - qualifier
SCLM messages
Chapter 3. SCLM messages  727

## Page 748

User response
Correct invalid DSN qualifier
FLM293S Invalid DSN - syntax - Dsname
contains embedded blanks,
parentheses or apostrophes.
Reenter.
Explanation
Invalid DSN - syntax
User response
Correct invalid DSN.
FLM293T Member not found - Specified
member 'aaaaaaaa' not found in
data set.
Explanation
Member not found
User response
Specify an existing member
FLM293U Invalid DSN - syntax - Dsname
must not end with a period.
Reenter dsname.
Explanation
Member not found
User response
Specify an existing member
FLM293V GDG not allowed - No GDG is
allowed in this release.
Explanation
Statements DSN can not be a GDG in this release.
User response
Reenter.
FLM293W Invalid DSN - quotes - Data
set name contains unbalanced
apostrophes; reenter dsname.
Explanation
Statements DSN can not be a GDG in this release.
User response
Reenter.
FLM293Y Search entry error - You must
finish entry at your other SCLM
Search first.
Explanation
The file which contains info from the SCLM Utility entry
panel can not be allocated.
User response
Submit search at your other SCLM session.
FLM293Z Member list error - Error retrieving
member list.
Explanation
No members were selected
User response
Determine why SCLM was unable to retrieve member
list for the mentioned data set.
FLM294A Sub-project required - A RETURN
or EXIT command was ignored in
order to display the SCLM Edit
Profile panel.
Explanation
SCLM must have the sub-project for a member in order
to save it. The RETURN or EXIT command was ignored
in order to obtain the sub-project for the member.
User response
Enter a valid sub-project for the project.
FLM294B Sub-project invalid - The sub-
project aaaaaaaa. is not defined in
the SCLM Project bbbbbbbb
Explanation
Sub-project aaaaaaaa is not defined in the project
definition. Members that are edited must be specify a
valid sub-project.
User response
Specify a sub-project that was defined in the project
definition.
SCLM messages
728  z/OS: z/OS ISPF Messages and Codes

## Page 749

FLM294C Sub-project missing - This project
has subprojects defined but the
selected member has a blank
subproject.
Explanation
When sub-projects are defined all parts must be
assigned a valid sub-project .
User response
Assign a sub-project via EDIT (SPROF) or by the
MIGRATE or STORE services.
FLM294D Invalid parameter - The sub-
project parameter must not be
more than eight characters long.
Explanation
The sub-project parameter of the locate command
must have eight or fewer characters.
User response
Specify a valid sub-project on the locate parameter.
FLM294E Enter a sub-project - Enter a sub-
project to locate.
Explanation
The locate command requires a sub-project
parameter.
User response
Specify a valid sub-project on the locate parameter.
FLM294F Sub-project not found - The sub-
project you are locating is not
found.
Explanation
The sub-project specified is not in the list of valid sub-
projects.
User response
Verify that the sub-project name was typed correctly.
If not, retype the command with the correct sub-
project. If so, see your project administrator.
FLM294G Invalid command - Valid
commands are CANCEL, LOCATE,
and END.
Explanation
The command is not valid.
User response
Enter a valid command.
FLM294H Select a sub-project - One and
only one sub-project can be
assigned to a member.
Explanation
More than one sub-project was selected from the list.
User response
Delete all but one S from the selection field.
FLM294I Not Authorised - You are not
authorised to use sub-project
aaaaaaaa. for this member.
Explanation
The user is not authorized by the SCLM security
interface to use the selected sub-project for this
member.
User response
Select another sub-project for which the user is
authorised to use
FLM294J Invalid command - Command
aaa(65) is invalid.
Explanation
The command aaa(65) is not a valid command.
User response
Enter a valid sub-project and press Enter to update the
sub-project of the member. Enter CANCEL to cancel
the update. Enter a valid command on the command
line to have the command processed.
FLM294K Command ignored - A RETURN
or EXIT command was ignored in
order to display the SCLM Edit
Profile panel.
Explanation
SCLM must have the sub-project for a member in order
to save it. The RETURN or EXIT command was ignored
in order to obtain the sub-project for the member.
SCLM messages
Chapter 3. SCLM messages  729

## Page 750

User response
Enter a valid sub-project for the project.
FLM294L Selection Cancelled - CANCEL
requested. No sub-project was
selected.
Explanation
Sub-project selection was exited with the CANCEL
command. No sub-project was selected.
User response
User must enter a valid sub-project for the project.
FLM294M Sub-project changed - A new, valid
subproject has been assigned.
Explanation
Sub-project selection completed successfully.
User response
Just a notification.
FLM294N Not Authorised - You are not
authorised to access the service.
Explanation
User is not authorised to use the service.
User response
See your SCLM or Security administrator.
FLM294O Sub-project required - A non-
blank subproject must be
assigned.
Explanation
A non-blank subproject must be assigned when
Subproject security is active.
User response
See your SCLM or Security administrator.
FLM294P Sub-project not defined - The
subproject is not defined in this
project.
Explanation
A member's subproject must be defined in the active
project.
User response
See your SCLM or Security administrator.
FLM294Q Not Authorised - You are not
authorised to change sub-project
aaaaaaaa. for this member.
Explanation
The user is not authorized by the SCLM security
interface to change the sub-project for this member.
User response
See your SCLM or Security administrator.
FLM294R Not Authorised - You are not
authorised to access member
aaaaaaaa.
Explanation
The user is not authorized by the SCLM security
interface to access the member.
User response
See your SCLM or Security administrator.
FLM294S Not Authorised - You are not
authorised to access aaaaaaaa..
bbbbbbbb
Explanation
The user is not authorized by the SCLM security
interface to access the service.
User response
See your SCLM or Security administrator.
FLM294T Not Authorised - You are not
authorised to access aaaaaaaa..
Press PF3 to return to SCLM Main
menu.
Explanation
The user is not authorized by the SCLM security
interface to access the service.
User response
See your SCLM or Security administrator.
FLM294U Not Authorised - You are not
authorised to access the member
- subproject is blank.
SCLM messages
730  z/OS: z/OS ISPF Messages and Codes

## Page 751

Explanation
The user is not authorized by the SCLM security
interface to access the member.
User response
See your SCLM or Security administrator.
FLM294V Undefined Subproject - The
accounting information for this
versioned member is invalid.
The SCLM security interface is
unable to validate access to the
subproject.
Explanation
SCLM subproject security is active. The SCLM security
interface is unable to validate access to the subproject
because the accounting information for this versioned
member is invalid.
User response
See your SCLM or Security administrator.
FLM390 Invalid type - The project does
NOT include type aaaaaaaa..
Explanation
The type specified by the user is not a type defined in
the project definition.
User response
Specify a type defined in the project definition.
FLM390B Invalid selection - Use A, M, B, D,
E, V, C, P, U, T, N or W.
Explanation
The selection entered is not a valid Library Utility
command.
User response
Choose one of the selections listed, or enter the HELP
command for further information.
FLM390C View error - SCLM is unable
to view requested member
aaaaaaaa..
Explanation
An error occurred in attempting to view the member.
This message may be issued for any of these reasons:
• A zero-length member of a partitioned data set was
found.
• A specified member was not found.
• No members matched the specified pattern, or no
members exist in the partitioned data set.
• Severe error; unable to continue.
System programmer response
Contact IBM support.
User response
Verify that the member exists in the library
specified. If the problem persists, contact the project
administrator.
FLM390D Invalid selection - Text not
available for member aaaaaaaa..
Explanation
This message is self explanatory.
FLM390E Work completed - The Library
Utility completed with a return
code of aaaaaaaa..
Explanation
This is an informational message. The user has exited
the library utility member list.
FLM390F Empty type - Type aaaaaaaa
contains no members, accounting
records, or build maps.
Explanation
This message is self explanatory.
FLM390G Member list error - A severe error
occurred while retrieving member
list.
Explanation
SCLM was unable to read the directory for the data set
specified by the user.
System programmer response
Contact IBM support.
User response
Contact the project administrator.
SCLM messages
Chapter 3. SCLM messages  731

## Page 752

FLM390H Invalid group - The project does
NOT include group aaaaaaaa..
Explanation
The group specified by the user is not a group defined
in the project definition.
User response
Specify a group defined in the project definition.
FLM390I Invalid selection - Accounting
record not available for specified
member aaaaaaaa..
Explanation
This message is self explanatory.
FLM390J Invalid selection - Use S (only
valid selection).
Explanation
The selection entered is invalid. The only valid
selection is S.
User response
Enter an S in the Select field for each compilation unit
that you want to review.
FLM390K Unable to access Bmap - SCLM
cannot access build map record for
member aaaaaaaa..
Explanation
This message is self explanatory.
FLM390L Invalid selection - Build map
record not available for specified
member aaaaaaaa..
Explanation
This message is self explanatory.
FLM390M Unable to review - Data set
aaaaaaaa is in use by another
logical screen.
Explanation
The build map contents for the requested member are
currently in use by another logical screen.
User response
Exit the build map contents on the other logical
screen.
FLM390N Allocation failed - Data set
aaaaaaaa already exists.
Explanation
SCLM is attempting to create a temporary data set to
hold the build map contents. A data set with that name
already exists. SCLM was unable to delete the existing
data set.
User response
Delete the existing copy of the specified data set.
FLM390P Invalid selection - SCLM has
already deleted the requested
member aaaaaaaa..
Explanation
SCLM has already deleted the version and the audit
information you selected.
User response
Make another selection.
FLM390Q Invalid command - aaaaaaaa is
not a valid command.
Explanation
Primary commands, scroll commands or the LOCATE
command are the only commands that may be entered
on the command line of this panel. The LOCATE
command may be specified by L, LOC, or LOCATE
followed by the name of the member to be located.
User response
Specify a valid command on the command line.
FLM390R Invalid selection - The only valid
option is S.
Explanation
The selection entered is not a valid Search Utility
command.
User response
Choose one of the selections listed, or enter the HELP
command for further information.
SCLM messages
732  z/OS: z/OS ISPF Messages and Codes

## Page 753

FLM390T No records found - SCLM
cannot find any records for type
aaaaaaaa..
Explanation
No Ada intermediate records were found for the type
specified.
User response
Enter another type or * (asterisk) to view the Ada
intermediate records for all types.
FLM390U Invalid selection - Use D (only
valid selection).
Explanation
Delete is the only valid selection on this panel.
User response
Enter D in the selection field for the items to be
deleted.
FLM390W Verification error - Xref record
type, member, date, or time does
not match accounting record.
Explanation
The accounting and cross-reference records for the
member are out of synch.
User response
Edit and save or migrate the member. This will cause
new accounting and cross-reference information to be
generated for the member.
FLM390X Unable to allocate - SCLM
is unable to allocate data
set. DYNALLOC return code =
aaaaaaaa..
Explanation
This message is self explanatory.
FLM390Y Unable to delete member. Member
does not exist or the user is
not authorized to update it. Enter
HELP for a detailed description.
Explanation
An error occurred in deleting the text, accounting, or
build map for the member being moved or deleted.
User response
Enter the HELP command for additional information on
the cause of the error.
FLM391 Delete failed - Delete
of intermediate record was
unsuccessful.
Explanation
The intermediate code was successfully deleted, but
the delete of the intermediate accounting record
failed. This may have occurred for any of these
reasons:
• The record was not found.
• SCLM was unable to purge the record from the cross-
reference database.
• The VSAM cross-reference database was enqueued.
• The cross-reference database is not defined to the
project.
System programmer response
Verify that the VSAM database is operational and has
been defined to the project currently in use. Determine
whether the database has been enqueued.
User response
Contact the project administrator.
FLM391B Delete failed - Delete
of intermediate form was
unsuccessful. Browse aaaaaaaa..
Explanation
The delete of the intermediate code failed.
User response
Browse the file specified in the long message for
additional information on the cause of the failure.
FLM391C Invalid type - Specify a type other
then "*" to delete intermediate
form.
Explanation
A delete cannot be done when the type field on
the Sublibrary Management Utility entry panel is *
(asterisk).
SCLM messages
Chapter 3. SCLM messages  733

## Page 754

User response
Return to the Sublibrary Management Utility entry
panel and specify the desired type.
FLM391D Member not found - No text or
accounting information found for
the member specified.
Explanation
This message is self explanatory.
FLM391E Update not confirmed - You used
the END or RETURN command to
request termination.
Explanation
This is an informational message.
FLM391F Acct list I/O error - A severe
I/O error occurred while retrieving
accounting member list.
Explanation
This message may be issued for any of these reasons:
• The accounting information is not in synch with the
member information.
• An invalid group was specified.
• A severe error occurred in accessing the VSAM
database.
System programmer response
Verify that the VSAM database is operational.
User response
Exit SCLM and ISPF then retry the operation. If the
problem persists, contact the project administrator.
FLM391G Build map list I/O error - A severe
I/O error occurred while retrieving
build map list.
Explanation
This message may be issued for any of these reasons:
• The map information could not be decoded and is in
an invalid format.
• An invalid group was specified.
• A severe error occurred in accessing the VSAM
database.
System programmer response
Verify that the VSAM database is operational and has
not been corrupted.
User response
Exit SCLM and ISPF then retry the operation. If the
problem persists, contact the project administrator.
FLM391H Table error - An error occurred in
the TBCREATE routine; return code
= aaaaaaaa..
Explanation
An error occurred when SCLM attempted to create a
table to display the member list data.
System programmer response
Contact IBM support.
Programmer response
Contact the project administrator.
FLM391I Delete not confirmed - You used
the END or RETURN command to
request termination.
Explanation
This is an informational message.
FLM391J Invalid auth. code - You must
enter an authorization code; a
blank is not valid.
Explanation
A blank value was entered in the New Authorization
Code field.
User response
Enter a valid authorization code.
FLM391K Delete failed - SCLM cannot find
any records at group aaaaaaaa for
member bbbbbbbb..
Explanation
No text, accounting, or build map data was found for
the requested member. The member does not exist at
the group specified. When this message occurs while
in the member list, the member selected has been
deleted by another logical screen or user since the
Library Utility member list was built or an attempt was
SCLM messages
734  z/OS: z/OS ISPF Messages and Codes

## Page 755

made to delete a member that was not in the group
specified when the member list was built.
User response
Specify another member name. Exit and reenter the
member list in order to display a more current list of
members. Change the group name specified on the
Library Utility panel to the group in which the member
was found.
FLM391L Records not found - SCLM
cannot find any records at group
aaaaaaaa..
Explanation
No records were found at the specified group.
User response
Specify another group.
FLM391M Records not found - SCLM cannot
find any records for compilation
unit qualifier aaaaaaaa..
Explanation
No records were found for the specified CU Qualifier.
User response
Specify another Compilation Unit Qualifier.
FLM391N Compilation unit deleted - The
compilation unit you requested
has been deleted.
Explanation
This is an informational message.
FLM391O Display failed - SCLM cannot find
the panel, message, or cursor
field.
Explanation
SCLM could not find panel FLMUA.
User response
Verify that panel FLMUA exists in the panel library,
and that this library appears in your data set
concatenation.
FLM391P Enter required field - Enter unique
letters of member name followed
by * for all remaining letters.
Explanation
The member name field was blank.
User response
Enter a member name or a pattern in the member
field. An asterisk (*) entered in this field will process all
members of the specified group and type.
FLM391Q Invalid value - Select one of the
available utilities.
Explanation
The user entered an invalid option.
User response
Specify one of the options listed.
FLM391R Invalid value - Enter "/" for error
listings only; leave blank for all
listings.
Explanation
This message is self explanatory.
FLM391S Enter required field - You must
specify at least one group; all
others are optional.
Explanation
The group field was left blank. At least one group must
be specified.
User response
Enter a valid group in the group field.
FLM391T Invalid value - Enter the type
name you want or enter * for all
types defined.
Explanation
The type field was left blank.
User response
Enter a valid type in the type field. A pattern may be
used. An asterisk (*) entered in this field will process
all types defined to the project.
FLM391U DB Utility completed - The return
code from the DB Contents Utility
is aaaaaaaa..
SCLM messages
Chapter 3. SCLM messages  735

## Page 756

Explanation
This is an informational message.
User response
See Database Contents Utility messages for return
codes greater than zero.
FLM391V Select an option - You must select
at least one of the display options.
Explanation
This message is self explanatory.
FLM391W Select an option - You either have
to enter the complete library type
or the beginning of the type name
followed by *.
Explanation
This message is self explanatory.
FLM391X Search failed. - The SUPERC
Search failed with code
aaaaaaaa.. Codes below 100 are
documented SuperC error return
codes. Other codes are internal
processing errors which should be
reported to IBM service.
FLM391Y Search completed - The return
code from the Search Utility
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
Explanation
This is an informational message.
User response
See Search Utility messages for return codes greater
than zero.
FLM392A Invalid value - Use IN, OUT, or NOT
USED.
Explanation
The value specified for architecture control is invalid.
User response
Specify IN to select members controlled by the
specified architecture definition, or OUT to select
members not controlled by the architecture definition.
Specify NOT USED if no architecture definition is to be
used to select members.
FLM392B Invalid arch cutoff - If you specify
ARCH CUTOFF, you must specify
all ARCH fields.
Explanation
This message is self explanatory.
FLM392C Canceled by user - Your request
to use the additional selection
criteria panel is canceled.
Explanation
This is an informational message.
FLM392D Invalid value - Use YES or NO.
Explanation
This message is self explanatory.
FLM392E Canceled by user - Your request
to use the customization panel has
been canceled.
Explanation
This is an informational message.
FLM392F Invalid value - You cannot select
TERMINAL for both TAILORED
OUTPUT and REPORT.
Explanation
TERMINAL cannot be specified as the output
destination for both the tailored output and the report.
User response
Specify a different output destination for either the
tailored output or the report.
FLM392G Auth code update denied - You
cannot update the authorization
code in a non-editable acct.
record.
Explanation
An attempt is being made to update an authorization
code for a non-editable type. Authorization codes are
not valid for non-editable types.
SCLM messages
736  z/OS: z/OS ISPF Messages and Codes

## Page 757

User response
Update an authorization code for an editable type.
FLM392H Invalid value - You cannot select
NONE for both TAILORED OUTPUT
and REPORT.
Explanation
This message is self explanatory.
User response
Specify a value other than NONE for either the tailored
output or the report.
FLM392I Report name invalid - Commas are
not allowed in the tailored output
report name.
Explanation
Commas cannot be used in the tailored output report
name.
User response
Remove the commas from the report name.
FLM393A Migrate completed - The return
code from the Migration Utility
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
Explanation
This is an informational message.
User response
See the Migration Utility messages for return codes
greater than zero.
FLM393B SUBLIB manager completed -
The Sublibrary Management Utility
completed with return code =
aaaaaaaa..
Explanation
This is an informational message.
FLM393C Record not found - SCLM cannot
find any record for compilation
unit name aaaaaaaa..
Explanation
There is no intermediate accounting record for this
compilation unit.
User response
Regenerate the compilation unit using SCLM build.
FLM393D Update not allowed - SCLM cannot
find any record for member:
aaaaaaaa..
Explanation
This message is self explanatory.
FLM394A Export completed - The return
code from the Export Utility
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
Explanation
This is an informational message.
User response
See the Export Utility messages for return codes
greater than zero.
FLM395A Import completed - The return
code from the Import Utility
is aaaaaaaa.. Enter the HELP
command for a description of
common errors.
Explanation
This is an informational message.
User response
See the Import Utility messages for return codes
greater than zero.
FLM396A Record not found - Specified
record was not found in the audit
VSAM database.
Explanation
This is an informational message.
FLM396B Delete failed - An error occurred
while attempting to delete the
Version / Audit VSAM record
SCLM messages
Chapter 3. SCLM messages  737

## Page 758

Explanation
The Version / Audit VSAM record could not be deleted.
System programmer response
Verify that the VSAM database is operational and has
been defined to the project currently in use. Determine
whether the database has been enqueued.
User response
Contact the project administrator.
FLM396C Member not found - The member
was not found in the SCLM
hierarchy
Explanation
This is an informational message.
FLM396D Retrieval error - Versioned format
was not FULL or DELTA.
Explanation
This message is self explanatory.
FLM396E Retrieval error - An error occurred
while attempting to retrieve the
versioned member.
Explanation
The versioned member could not be retrieved.
Operator response
The accounting record might be corrupted. Try deleting
the accounting record of the member being retrieved
and recreating it by saving the member again. The
error can occur if the accounting record is corrupted
at a higher level of the hierarchy. In this case, you
might have to delete the accounting record at the
higher level and promote into that level to correct the
accounting error.
System programmer response
Assist the user in correcting the problem.
Programmer response
See the version messages for an explanation and
correct the problem if possible. Contact the project
administrator for assistance.
FLM396F Display failed - An error occurred
while attempting to display
accounting information.
Explanation
The accounting information could not be displayed.
System programmer response
Assist the user in correcting the problem.
Programmer response
See the version messages for an explanation and
correct the problem if possible. Contact the project
administrator for assistance.
FLM396G Compare Failed - The Compare
was unsuccessful. See the listing
data set for any messages.
Explanation
This is an informational message.
FLM396I No records selected - No records
were selected for the specified
choice.
Explanation
This is an informational message.
FLM396J Selection list error - A severe error
occurred while retrieving selection
list.
Explanation
The selection list could not be retrieved.
System programmer response
Assist the user in correcting the problem.
User response
See the version messages for an explanation, and
correct the problem if possible. Contact the project
administrator for assistance.
FLM396K Invalid selection - Use A, C, D, or
R.
Explanation
An invalid selection was typed.
SCLM messages
738  z/OS: z/OS ISPF Messages and Codes

## Page 759

User response
Type A, C, D, or R.
FLM396L Invalid selection - "C,H,R,V,X" are
not valid. No Version exists for
selected member.
Explanation
C, H, R, V, or X was entered, but there is no version for
this member.
User response
Specify C,H,R,V or X for a member which has a version.
FLM396M Invalid date - The Year, Month,
and/or Day is invalid for required
date format.
Explanation
An invalid date was entered. The year, month, or day is
invalid.
User response
Enter a valid date. Valid values are:
year
01-99
month
01-12
day
01-31
FLM396N Invalid selection - No Retrieval
Data Set Specified.
Explanation
Retrieve was selected, but no retrieval data set was
specified.
User response
Return to the Audit/Version panel and specify a
retrieval data set.
FLM396O Invalid dates - "FROM" date must
be less than or equal to "TO" date
Explanation
The FROM date is greater than the TO date.
User response
Specify a FROM date that is less than or equal to the
TO date.
FLM396P Invalid request - Auditing not
defined for Group aaaaaaaa Type
bbbbbbbb in Project cccccccc..
Explanation
Auditing is not defined for the specified group and type
in the project definition.
System programmer response
Enable auditing for the group and type, then
reassemble and link the project definition.
User response
Verify that the group and type are correct. If auditing
is needed for the group and type, contact the project
administrator.
FLM396Q Delete group completed - The
return code from the Delete
from Group Utility is aaaaaaaa..
Enter the HELP command for a
description of common errors.
Explanation
This is an informational message.
User response
See the Delete from Group Utility messages for return
codes greater than zero.
FLM396R Enter required field - Enter a valid
TYPE name or pattern.
Explanation
The TYPE field is a required field.
User response
Enter a name or pattern in the TYPE field.
FLM396S Enter required field - Enter a valid
MEMBER name or pattern.
Explanation
The MEMBER field is a required field.
SCLM messages
Chapter 3. SCLM messages  739

## Page 760

User response
Enter a name or pattern in the MEMBER field.
FLM396T Enter required field - Select one
of the listed values: Build map,
Account, Text or Output.
Explanation
The Delete Flag field is a required field. Entering a
value of Build map will delete all build map records
that match the pattern. Entering a value of Account
will delete all accounting records, cross-reference
records, intermediate records, and build map records
that match the pattern. Entering a value of Text
will delete everything that is deleted when Account
is specified plus any text members that match the
pattern. Entering a value of Output will delete all build
map records, intermediate records and code, and all
non-editable accounting records, their cross-reference
records, and associated text members that match the
pattern.
User response
Select either Build map, Account, Text, or Output.
FLM396U Enter required field - Enter one
of the listed values: EXECUTE or
REPORT.
Explanation
The DELETE MODE field is a required field.
User response
Select EXECUTE or REPORT for the DELETE MODE
field.
FLM396W Invalid value - Enter one of the
listed values: Build map, Account,
Text, or Output.
Explanation
The value entered for the DELETE FLAG field is invalid.
User response
Select Build map, Account, Text, or Output for the
Delete Flag field.
FLM396X Invalid value - Enter one of the
listed values: EXECUTE or REPORT
Explanation
The value entered for the DELETE MODE field is
invalid.
User response
Select EXECUTE or REPORT for the DELETE MODE
field.
FLM396Y VSAM I/O error - An error
occurred trying to access the
cross-reference database.
Explanation
The cross-reference data set could not be accessed for
one of these reasons:
• No cross-reference data set was defined in the
project definition.
• User does not have read access to the data set.
• The data set has the wrong key size.
• The data set has not been initialized.
System programmer response
Verify that the cross-reference data set is defined in
the project definition. If not, specify it and reassemble
and relink the project definition. If it does exist, verify
that the key size is correct and that the data set has
been initialized. Verify that the user has read access to
the data set.
Programmer response
Contact the project administrator.
FLM396Z Invalid value - Only a value of 3,
for Text, is valid when deleting a
Package.
Explanation
The value entered for the DELETE FLAG field is invalid.
User response
Select Text only, for the Delete Flag field.
FLM397A Invalid group - aaaaaaaa is not a
valid group name.
Explanation
The group name is not valid.
SCLM messages
740  z/OS: z/OS ISPF Messages and Codes

## Page 761

User response
Enter a valid group name.
FLM397B Total length too long - Combined
length of authorization codes plus
separators > 253.
Explanation
This message is self explanatory.
FLM397C Field should be blank - Field must
not contain data when an external
library is specified.
Explanation
This message is self explanatory.
FLM397D Project defn loaded -
aaaaaaaa..PROJDEF.LOAD(bbbbb
bbb) loaded successfully.
Explanation
This message is self explanatory.
FLM397E Project generated
- aaaaaaaa..bbbbbbbb..cccccccc.
(dddddddd.) generated
successfully.
Explanation
This message is self explanatory.
FLM397F Language generated
- aaaaaaaa..bbbbbbbb..cccccccc.
(dddddddd.) generated
successfully.
Explanation
This message is self explanatory.
FLM397G Invalid value - Enter a valid
command.
Explanation
This message is self explanatory.
FLM397H Invalid command - This command
is not valid if an external library is
specified.
Explanation
This message is self explanatory.
FLM397I Invalid request - Auditing not
defined for Group aaaaaaaa Type
bbbbbbbb in Proj Def cccccccc..
Explanation
Auditing is not defined for the specified group and type
in the alternate project definition.
System programmer response
Enable auditing for the group and type, then
reassemble and link the alternate project definition.
Programmer response
Verify that the group and type are correct. If auditing
is needed for the group and type, contact the project
administrator.
FLM398 Update not allowed - Member
must exist at the initial group
"aaaaaaaa." to update authcode.
Explanation
The member is not at the initial group specified.
User response
Draw down the member to the group specified, or
specify the group the member exists in on the Library
Utility panel.
FLM398A Delete completed - The return
code from the Delete command is
aaaaaaaa..
Explanation
This is an informational message.
FLM398B Update completed - The return
code from the Update command is
aaaaaaaa..
Explanation
This is an informational message.
FLM398C Acct record displayed - The return
code from the Browse Accounting
Record command is aaaaaaaa..
Explanation
This is an informational message.
SCLM messages
Chapter 3. SCLM messages  741

## Page 762

FLM398D Member viewed - The return code
from the View Text command is
aaaaaaaa..
Explanation
This is an informational message.
FLM398E Build map displayed - The return
code from the Browse Build Map
command is aaaaaaaa..
Explanation
This is an informational message.
FLM398F Equal not allowed - The equal
symbol "=" is not a supported
pattern symbol.
Explanation
An equal symbol was specified in the pattern. The
equal symbol is not valid for this function.
User response
Specify an * (asterisk) or a ¬ (NOT) as the pattern
symbol.
FLM398G Command required - Enter A, B, C,
D, E, M, or U.
Explanation
This message is self explanatory.
FLM398H Build completed - The return code
from the Build command was
aaaaaaaa..
Explanation
This is an informational message.
FLM398I Acct record not found - Accounting
record not found for member
aaaaaaaa..
Explanation
This message is self explanatory.
FLM398J Build map not found - Build Map
record not found for member
aaaaaaaa..
Explanation
No build map record was found for the member at the
specified group.
FLM398K Build not allowed - Accounting
record not found for group
aaaaaaaa member bbbbbbbb..
Explanation
This message is self explanatory.
FLM398L No data to display - There were no
aaaaaaaa. to display.
Explanation
There were no records of the type requested to
display. If text (T) was requested, either the data set
is empty or the data set does not exist. If accounting
records (A) were requested, there are no accounting
records to display. If build maps (M) were requested,
there were no build maps to display.
System programmer response
Assist the user in determining that the data set exists
and contains members. If the data set does not exist
but is needed, then allocate it.
Programmer response
Make sure the data set exists and that it contains
members if text was requested. Contact the project
administrator for assistance, if necessary.
FLM398M Edit completed - The return code
from the Edit command was
aaaaaaaa..
Explanation
This is an informational message.
FLM398N Library not specified - Specify "/"
for Hierarchy view on the Library
Utility Panel.
Explanation
The member you are attempting to edit has an
accounting record at a higher level than the group
specified on the Library Utility panel. The hierarchy
must be specified in order to do the drawdown
processing.
SCLM messages
742  z/OS: z/OS ISPF Messages and Codes

## Page 763

User response
Specify "/" for hierarchy view on the Library Utility
panel.
FLM398O Invalid option string - String
characters T, A, M. For example A,
TA, and MAT are valid strings.
Explanation
The option string you entered was not valid.
User response
Enter a valid combination of T (for text), A (for
accounting records), and M (for build maps).
FLM398P Processing terminated - You used
the END or RETURN command to
request termination.
Explanation
This is an informational message.
FLM398Q Accounting Record not found.
Enter HELP for a detailed
description.
Explanation
This message is self explanatory.
FLM398R Allocation failed - SCLM is unable
to allocate data set aaaaaaaa..
Explanation
Allocation failed for a data set in the hierarchy because
a type name was not found in the hierarchy, or a group
is undefined in the project definition.
System programmer response
Correct the project definition.
Programmer response
Contact the project administrator.
FLM398S Build failed - Build failed for
member aaaaaaaa..
Explanation
The build did not complete successfully.
User response
See the build messages for an explanation of the
failure.
FLM398T Enter processing mode - Enter
EX or SU on the command line
or select Execute or Submit in
the Process field to invoke this
function. If you enter a value in
the Process field, it will be saved
between sessions. Entering EX or
SU on the command line overrides
the Process field value.
Explanation
This function requires a processing mode.
Programmer response
Specify EX or SU on the command line, or select
Execute or Submit in the Process field.
FLM398U Version Compare failed. - The
Version Compare failed with code
aaaaaaaa. Codes below 100 are
documented SuperC error return
codes. Press HELP now to see
them. Code 122 indicates the
Retrieve/New data set was not
found. Create it and retry. Other
codes are internal processing
errors which should be reported to
IBM service.
FLM398V Promote failed - Promote failed for
member aaaaaaaa..
Explanation
The promote did not complete successfully.
User response
See the promote messages for an explanation of the
failure.
FLM398W Promote completed - The return
code from the Promote command
was aaaaaaaa..
Explanation
This is an informational message.
FLM398X Select a Version. - Please select
one and only one version to be
compared
SCLM messages
Chapter 3. SCLM messages  743

## Page 764

Explanation
This is an informational message.
FLM398Y Same Version - Compare is not
possible against the same version.
Please select a different Version.
Explanation
This message is self explanatory.
FLM398Z No dependency info - Dependency
record not found for member
aaaaaaaa..
Explanation
No dependency record was found for the member at
the specified group.
FLM399A Dependency inactive - No Cross-
Dependency database is defined
in this project.
Explanation
A dependency database is not defined in the project.
FLM399B Database error - An error occurred
accessing the Cross Dependency
database.
Explanation
An error occurred accessing the dependency database
FLM399C No subentries - Row can not be
expanded or collapsed.
Explanation
This message is self explanatory.
FLM399D List was truncated - The maximum
nesting level was reached. Entries
indicated by a > sign have parents
but they will not be displayed.
FLM399E Invalid selection - Use A, M, B, D,
E, V, C, P, U, S, T, N or X.
Explanation
The selection entered is not a valid Library Utility
command.
User response
Choose one of the selections listed, or enter the HELP
command for further information.
FLM399F Circular reference - This object
will not be expanded since it
refers to itself either directly or
indirectly.
Explanation
X cmd cant be used on this row
FLM399G Table error - RC > 8 from TBADD
Explanation
TBADD error occurred
FLM399H Not found - Member was not found
in the list
Explanation
TBSCAN had non-zero return code.
FLM490 Invalid environment - SCOMP can
only be run in an SCLM Edit
environment
Explanation
The SCOMP command is only valid when run in an
SCLM Edit environment.
FLM491 Allocation Error - Error allocating
retrieval data set aaa(44)
Explanation
SCLM could not allocate the temporary data set used
to retrieve the version selected.
System programmer response
The failure may be due to the naming convention of
the data set. If the name does not use your naming
convention the ISPF Data Set Name Change Exit can
be used to modify the name of the data set. The
naming convention for this data set is:
userid.EDITVER.TEMPnnnn
where userid is the user ID of the user requesting the
function, and nnnn is a number between 0000 and
9999.
User response
Contact your system programmer
FLM492 Allocation Error - Error allocating
SCLM messages data set
zdummyds,44
SCLM messages
744  z/OS: z/OS ISPF Messages and Codes

## Page 765

Explanation
SCLM could not allocate the temporary messages data
set used when calling the VERINFO and VERRECOV
services.
System programmer response
The failure may be due to the naming convention of
the data set. If the name does not use your naming
convention the ISPF Data Set Name Change Exit can
be used to modify the name of the data set. The
naming convention for this data set is:
userid.EDITVER.DUMMnnnn
where userid is the user ID of the user requesting the
function, and nnnn is a number between 0000 and
9999.
User response
Contact your system programmer
FLM493 Retrieval failed - The SCLM
VERINFO service returned a
zmodrc,4 when retrieving the
versions for zevermem,8
Explanation
SCLM could not retrieve all of the versions for the
member you are editing
System programmer response
Refer to the SCLM Guide and Reference for information
on the return codes from the VERINFO service.
User response
Contact your system programmer
FLM494 Invalid select character - Use the S
line command to select the version
to be compared.
Explanation
SCLM could not retrieve all of the versions for the
member you are editing
System programmer response
Refer to the SCLM Guide and Reference for information
on the return codes from the VERINFO service.
User response
Contact your system programmer
FLM495 No versions found - No versions of
member aaaaaaaa were found in
the SCLM versioning file.
Explanation
This is an informational message.
FLM496 Full source only - The only version
of member zevermem,8 that exists
is the full source version that
is identical to the copy you are
editing.
Explanation
This is an informational message.
FLM497 Select one version - Only one
version may be selected from the
list.
Explanation
This is an informational message.
FLM498A Version open error - Error opening
the Version member aaaaaaaa.
Explanation
This is an informational message.
FLM498B Version read error - Error reading
the Version member aaaaaaaa.
Explanation
This is an informational message.
FLM498C Version header error - The Version
member aaaaaaaa did not have a
header record as the first record in
the file.
Explanation
This is an informational message.
FLM498D Allocate error - Error allocating the
version history report dataset.
Explanation
This is an informational message.
FLM498E View error - Error viewing the
version history report.
SCLM messages
Chapter 3. SCLM messages  745

## Page 766

Explanation
This is an informational message.
FLM498F Write error - Error writing the
version history report.
Explanation
This is an informational message.
FLM498G Version not found - The requested
Version was not found in the
Version member aaaaaaaa
Explanation
This is an informational message.
FLM498H Archived Version - The requested
Version was found in an archive
data set that is not the current
Versioning data set, not all the
history is being displayed.
Explanation
This is an informational message.
FLM498I Retrieval Error - An error
was encountered attempting to
retrieve the data set statistics
for the data set aaaaaaaa Return
Code=bbbbbbbb
Explanation
This is an informational message.
FLM498J Allocation - An error was
encountered attempting to
allocate a temporary data set to
DECODE the member aaaaaaaa
into. Return Code=rr
Explanation
This is an informational message.
FLM498K Decoding Error - An error
was encountered attempting to
DECODE the version PDS member.
Return Code=aaaaaaaa Error
Message: bbbbbbbb
Explanation
This is an informational message.
FLM498L Open Error - An error
was encountered opening the
version PDS member. Return
Code=aaaaaaaa
Explanation
This is an informational message.
FLM498M Open Error - An error was
encountered opening the version
history report data set. Return
Code=aaaaaaaa
Explanation
This is an informational message.
FLM498N Open Error - An error was
encountered opening the member
aaaaaaaa in the data set
bbbbbbbb. Return Code=cccccccc
Explanation
This is an informational message.
FLM498O Member not found - The member
aaaaaaaa was not found in the
data set bbbbbbbb.
Explanation
The given member was not found in the data set
specified.
FLM498P Data Already Encoded - The
SAVE command found the data
was already encoded. To see the
decoded data please exit and re-
edit the member.
Explanation
SCLM detected that the data was encoded when
saving possibly by ISPF COPY command on an
encoded member. To view the decoded member exit
and re-edit the member.
FLM499 Retrieval failed - The SCLM
VERRECOV service returned a
zmodrc,4 when retrieving the
versions for zevermem,8
Explanation
SCLM could not retrieve all of the versions for the
member you are editing
SCLM messages
746  z/OS: z/OS ISPF Messages and Codes

## Page 767

System programmer response
Refer to the SCLM Guide and Reference for information
on the return codes from the VERRECOV service.
User response
Contact your system programmer
FLM600 Invalid function type - The
function type must be CMD, PGM
or PANEL
Explanation
The only valid function types are CMD, PGM, and
PANEL.
User response
Enter a valid function type.
FLM601 Enter all fields - All fields must be
specified for a line command entry
Explanation
All the fields in a row must be entered for a line
command entry.
User response
Reenter the line command including all the fields.
FLM601A User not Authorised - You are not
authorised to access this member.
Access is denied.
Explanation
The user is not authorized by the SCLM security
interface to access the selected member.
FLM602 Cmd table not found - The user
commands will not be available for
this session
Explanation
The user commands will not be available for this
session.
FLM603 Unexpected error - An SCLM
function aaaaaaaa returned an
unexpected error at Call bbbbbbbb
Explanation
An unexpected ISPF error was encountered.
FLM600A Cmd table not found - The
command table is not allocated
contact administrator
Explanation
The SCLM command tables are not allocated to the
ISPTLIB DDname.
FLM600B Invalid Selection - Use one of the
following aaaaaaaa
Explanation
Invalid line command.
User response
Enter one of the line commands listed on the UOW
Member List panel.
FLM600C Invalid Prefix - Enter a valid RACF
dataset prefix
Explanation
A data set prefix that is known to RACF must be
entered.
FLM600D Invalid Element - Work elements
flagged as invalid cannot be
processed
Explanation
Correct or remove the work element from the Unit of
Work.
FLM600E Not eligible for update - Work
elements Authcode can only be
updated at your user level.
Explanation
The work element must be in the development library
to be updated.
FLM600F Processing completed - The
selected action was successfully
completed.
Explanation
The selected action completed with return code 0.
FLM600G Processing failed - The selected
action did not complete
successfully.
SCLM messages
Chapter 3. SCLM messages  747

## Page 768

Explanation
The selected action completed with a nonzero return
code.
FLM600H Unit of Work Saved - The Unit of
Work member was automatically
saved.
Explanation
The UOW item was saved to reflect changes.
FLM600I Action Canceled - The selected
action was canceled.
Explanation
The user canceled the selection action.
FLM600J Job submitted - The Job was
successfully submitted.
Explanation
The background job has been submitted.
FLM600K Max recursions reached -
Selection canceled, the maximum
number of recursions was
exceeded.
Explanation
searched
FLM600L No build aaaaaaaa dataset - The
selected output dataset for build
aaaaaaaa does not exist
Explanation
Run the build command before viewing the output
data sets
FLM600M No promote aaaaaaaa dataset -
The selected output dataset for
promote aaaaaaaa does not exist
Explanation
Run the promote command before viewing the output
data sets
FLM600N No build map found - The selected
component does not have a build
map
Explanation
Run the build command to create the build map
FLM600O No Acct Info found - The selected
component does not have any
accounting information
Explanation
Either Migrate or edit and save the member in SCLM
FLM600P Enter Member and Type - To add
a new member you must enter the
member name and type
Explanation
Enter both the member and type parameters.
FLM600Q Member already exists - The
member and type you specified
already exists in this UOW
Explanation
You cannot create a duplicate entry in a Unit of Work
FLM600R Type Unknown - The type does not
exist in the project hierarchy
Explanation
Specify a valid data set type
FLM600S Option not available - The Transfer
option is only available to SCLM
administrators or the user who has
the member locked.
Explanation
The Transfer line command in SCLM option 3.1 is only
available to SCLM administrators or the user who has
the member locked.
User response
Ask either an SCLM administrator or the user who
has the member locked to transfer ownership of the
member to you.
FLM600T Transfer not possible - The
accounting record either doesn't
exist or exists at a higher
level. Transferring of ownership
for member level locking is not
possible.
Explanation
Transfer of ownership for member level locking is not
possible because the group at which you are issuing
SCLM messages
748  z/OS: z/OS ISPF Messages and Codes

## Page 769

the Transfer Ownership line command does not have
an accounting record.
User response
None.
FLM600U Transfer not possible - The
accounting record is not for a
development group. Transferring
of ownership for member level
locking is not possible.
Explanation
Transfer of ownership for member level locking is not
possible as the accounting record you are issuing the
line command against is not a development accounting
record. Transfer of ownership is only possible against a
development accounting record.
User response
None.
FLM00101 MEMBER NAME IS BLANK
Explanation
The Member field was left blank.
User response
Verify that the member parameter was specified and is
in the correct position.
FLM00102 SCLM INTERNAL ERROR
OCCURRED AT aaa CODE: bbb.
Explanation
An unexpected situation occurred during program
execution. The message identifies the name of the
SCLM routine that failed unexpectedly, and the return
code.
User response
Contact the project manager with aaa and bbb. This
information will be needed by IBM support.
Project manager response
Report this message (including the message ID, the
aaa text field, and the bbb text field) to IBM support.
FLM00201 PROCESSING BASED ON CHANGE
CODE.
Explanation
A build or promote by change code is in progress.
This message will be followed by a message indicating
whether the change codes processed will be included
(FLM00202) or excluded (FLM00203) and a list of the
change codes processed (FLM00204). This message is
provided for information only.
FLM00202 THE FOLLOWING CHANGE CODES
WILL BE INCLUDED:
Explanation
The change codes processed had a Y include flag
value. Members with the listed change code(s) will be
included from the build or promote group. Refer to
messages FLM00201 and FLM00204. This message is
provided for information only.
FLM00203 THE FOLLOWING CHANGE CODES
WILL BE EXCLUDED:
Explanation
The change code(s) processed had a 'N' include flag
value. Members with the listed change code(s) will
be excluded at the build or promote group. Refer to
messages FLM00201 and FLM00204. This message is
provided for information only.
FLM00204 aaaaaaaa
Explanation
Change code aaaaaaaa has been processed. Refer
to message FLM00201. This message is provided for
information only.
FLM01001 ERROR RETRIEVING
ACCOUNTING INFORMATION,
CODE: aaa TYPE: bbbbbbbb
MEMBER: cccccccc STARTING AT
GROUP: dddddddd
Explanation
No accounting information exists or could be retrieved
for member cccccccc within the hierarchy beginning at
group dddddddd.
User response
Possible return codes are:
4
SCLM did not find the member's accounting
information at this level in the hierarchy, but it did
find it at a higher level.
SCLM messages
Chapter 3. SCLM messages  749

## Page 770

8
SCLM did not find the member's accounting
information. Make sure member cccccccc exists
starting at group dddddddd type bbbbbbbb.
Register the member with SCLM using the SCLM
editor, migration utility, or the SAVE service. Run
the processor again.
When received during a build or promote by
change code, this message can indicate that a
member excluded at the build or promote group
based on change code does not exist at a higher
group. In this case, update the change codes
specified so that the required member is included.
12
SCLM successfully retrieved the member's
accounting and dependency information. However,
either the dependency information failed a
verification check or the accounting record version
does not match the member's version.
To determine the nature of the verification error,
use the library utility to browse the member's
accounting and dependency information.
To correct the problem, update the member by
using the SCLM editor, migration utility, or the
SAVE service.
16
The specified group was not found in the project
definition. This error can occur when you use
alternate project definitions or when you modify a
project definition. Contact the project manager.
20
A severe I/O error occurred. Contact the project
manager.
28
The type is not defined by the project definition
being used.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set(s)
starting at group dddddddd to determine the
problem.
28
Verify that the type specified is defined in the
project definition. If necessary, add the type to the
project definition and reassemble it. Submit the
job again.
FLM01002 ERROR UPDATING ACCOUNTING
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
An error occurred while attempting to write the
accounting information for member dddddddd.
User response
Possible return codes are:
4
An I/O error occurred while writing the
member's accounting information to the secondary
accounting data set. Because the primary
accounting data set was correctly updated, SCLM
will use the correct information for all references.
However, the two accounting data sets are no
longer identical. Contact the project manager.
8
The number of dependent members (compools,
included members, and compilation units)
referenced in the source member plus the
change codes and user data associated with the
dependent members exceeds the SCLM maximum
accounting record size of 32000 characters.
Consequently, the accounting information was not
written.
Change the member so that the number of
referenced dependents is decreased. Delete
unnecessary change codes and user data in the
accounting information.
12
SCLM internal error. Contact the project manager.
20
An I/O error occurred while writing the member's
accounting information to the primary accounting
data set. The failure to create accounting
information implies that SCLM will not be able to
track the member. Submit the job again and if the
error recurs, contact the project manager.
Project manager response
If the return code is:
4
An I/O error occurred while writing the
member's accounting information to the secondary
accounting data set. Run IDCAMS against the
secondary accounting data set. If it is damaged,
reallocate it and initialize it with data from the
primary accounting data set. Use the IDCAMS
SCLM messages
750  z/OS: z/OS ISPF Messages and Codes

## Page 771

REPRO service to copy the accounting data from
the primary accounting data set to the new
secondary accounting data set.
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
20
An I/O error occurred while writing the member's
accounting information to the primary accounting
data set. Run IDCAMS against the accounting data
set to determine the problem.
FLM01003 ERROR PURGING ACCOUNTING
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
A VSAM error occurred while attempting to purge
accounting information for member dddddddd.
User response
Possible return codes are:
4
The member's accounting record was not found.
The accounting record would have been deleted
anyway, so no action is required. However, to
determine why the accounting record was not
found, view the audit record if audit information
is available for the deleted record. Another SCLM
user running a concurrent job might have deleted
the record.
8
An I/O error occurred while attempting to
purge the accounting record from the secondary
accounting data set. The record was successfully
purged from the primary accounting data set,
but the primary and secondary accounting data
sets are no longer identical. Contact the project
manager.
12
Unable to purge the accounting record from the
primary accounting data set or an error occurred
during versioning. If a versioning error occurred,
investigate the accompanying versioning message.
Otherwise, submit the job again. If the problem
occurs again, contact the project manager.
16
Primary accounting VSAM data set enqueued.
Submit the job again after the data set is no longer
exclusively in use by another job.
20
An I/O error occurred while purging the accounting
record. Submit the job again. If the problem occurs
again, contact the project manager.
Project manager response
If the return code is:
8
The primary and secondary accounting data sets
are no longer identical. Reinitialize the secondary
data set from the primary data set.
12 or 20
Run IDCAMS against the accounting data set to
determine the problem.
FLM01004 ERROR RETRIEVING
ACCOUNTING INFORMATION,
CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc
MEMBER: dddddddd
Explanation
No accounting record exists or could be retrieved for
member dddddddd in group bbbbbbbb.
User response
Possible return codes are:
8
The member's accounting information was not
found. Introduce the member to SCLM using the
SCLM editor, migration utility, or SAVE service. Run
the processor again.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
For a return code of 20, run IDCAMS against the
accounting data set to determine the problem.
FLM01005 ERROR RETRIEVING
ACCOUNTING INFORMATION,
CODE: aaa GROUP: bbbbbbbb
SCLM messages
Chapter 3. SCLM messages  751

## Page 772

Explanation
An error occurred trying to access information from the
VSAM accounting data set for group bbbbbbbb.
User response
Possible return codes are:
8
An error occurred attempting to access the VSAM
data set for group bbbbbbbb, because group
bbbbbbbb could not be found. Verify that group
bbbbbbbb is valid and that the accounting data set
for that group has not become corrupted.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
20
A severe I/O error occurred, or the VSAM data
set for the specified group could not be opened.
Contact the project manager.
Project manager response
For a return code of 20, verify that the VSAM data set
for the specified group exists, or run IDCAMS against
the accounting data set to determine the problem.
FLM01006 ERROR RETRIEVING BUILD
MAP INFORMATION, CODE: aaa
GROUP: bbbbbbbb
Explanation
An error occurred trying to access information from the
VSAM accounting data set for group bbbbbbbb.
User response
Possible return codes are:
8
An error occurred attempting to access the VSAM
data set for group bbbbbbbb, because group
bbbbbbbb could not be found. Verify that group
bbbbbbbb is valid and that the accounting data set
for that group has not become corrupted.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
20
A severe I/O error occurred, or the VSAM data
set for the specified group could not be opened.
Contact the project manager.
Project manager response
For a return code of 20, verify that the VSAM data set
for the specified group exists, or run IDCAMS against
the accounting data set to determine the problem.
FLM01009 ERROR, UNABLE TO RESTORE
VERSIONED MEMBER FOR GROUP:
aaaaaaaa CODE: bbb
Explanation
An error occurred trying to restore a version of a
member for group aaaaaaaa, or while attempting to
update the accounting information for a member, or
an inconsistency in the accounting information was
detected.
User response
Possible return codes are:
4
An I/O error occurred while writing the
member's accounting information to the secondary
accounting data set. Because the primary
accounting data set was correctly updated, SCLM
will use the correct information for all references.
However, the two accounting data sets are no
longer identical. Contact the project manager.
8
The number of dependent members (compools,
included members, and/or compilation units)
referenced in the source member plus the
change codes and user data associated with the
dependent members exceeds the SCLM maximum
accounting record size of 32000 characters.
Consequently, the accounting information was not
written.
Change the member so that the number of
referenced dependents is decreased. Delete
unnecessary change codes and user data in the
accounting information.
12
SCLM internal error. Contact the project manager.
20
An I/O error occurred while writing the member's
accounting information to the primary accounting
SCLM messages
752  z/OS: z/OS ISPF Messages and Codes

## Page 773

data set. The failure to create accounting
information implies that SCLM will not be able to
track the member. Submit the job again and if the
error recurs, contact the project manager.
Project manager response
If the return code is:
4
An I/O error occurred while writing the
member's accounting information to the secondary
accounting data set. Run IDCAMS against the
secondary accounting data set. If it is damaged,
reallocate it and initialize it with data from the
primary accounting data set. Use the IDCAMS
REPRO service to copy the accounting data from
the primary accounting data set to the new
secondary accounting data set.
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
20
An I/O error occurred while writing the member's
accounting information to the primary accounting
data set. Run IDCAMS against the accounting data
set to determine the problem.
The following return codes indicate that the
accounting records are not synchronized with the
source code. An alternate project definition may be in
use that references a different VSAM cluster for the
accounting records:
112
Accounting information does not match
dependency information for the member.
120
The version of the accounting record does not
match the version of the code.
FLM01011 ERROR RETRIEVING
ACCOUNTING OR CROSS-
REFERENCE INFORMATION,
CODE: aaa ERROR GROUP:
bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
An error occurred while attempting to retrieve the
accounting or dependency information for member
dddddddd.
User response
Possible return codes are:
8
The member's accounting information was not
found. Introduce the member to SCLM using the
SCLM editor, migration utility, or SAVE service. Run
the processor again.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A severe I/O error occurred. Contact the project
manager.
24
SCLM could not find the accounting or the cross-
reference data set.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting or cross-
reference data set to determine the problem.
FLM01012 ERROR UPDATING ACCOUNTING
OR CROSS-REFERENCE DATA SET
INFORMATION, CODE: aaa ERROR
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
An error occurred while attempting to write the
accounting and dependency information for member
dddddddd.
User response
Possible return codes are:
SCLM messages
Chapter 3. SCLM messages  753

## Page 774

8
An I/O error occurred while writing the member's
accounting information and no attempt was made
to write the dependency information. Errors can
occur if SCLM attempts to reference this member.
Submit the job again, and if the error recurs,
contact the project manager.
12
An I/O error occurred while writing dependency
information for a compilation unit. Errors can occur
if SCLM attempts to reference this member. Submit
the job again, and if the error recurs, contact the
project manager.
Project manager response
For a return code of 20, run IDCAMS against the
accounting and cross-reference data sets to determine
the problem.
FLM01013 ERROR PURGING ACCOUNTING
OR CROSS-REFERENCE DATA SET
INFORMATION CODE: aaa ERROR
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
A VSAM error occurred while attempting to delete the
accounting or cross-reference information for member
dddddddd.
User response
Possible return codes are:
4
The member's accounting record was not found.
The accounting record would have been deleted
anyway, so no action is required. To determine
why the accounting record was not found, view
the audit record if audit information is available for
the deleted record. Another SCLM user running a
concurrent job might have deleted the record.
8
Unable to purge the cross-reference record or
an error occurred writing an audit record. The
accounting record is not deleted. If an error
occurred writing an audit record, investigate the
accompanying message.
20
Unable to purge the accounting record or an error
occurred during versioning. If a versioning error
occurred, investigate the accompanying versioning
message. Otherwise, submit the job again. If
the problem occurs again, contact the project
manager.
Project manager response
If the return code is:
8 or 20
Run IDCAMS against the VSAM data sets to
determine the problem.
FLM01020 CHANGE CODE PROCESSING
WARNING: MORE RECENT
CHANGE CODES EXIST FOR
MEMBER: aaaaaaaa
Explanation
The member has more than one change code. A
request has been made to build or promote by a
change code that is not the most recent. This is
potentially a problem when processing architecture
definitions that contain more than one member. The
results might not be what you would expect.
User response
If some of the parts for the most recent change code
have not been built because they were not requested
by this architecture definition, build more than one
change code in the architecture definition.
FLM01030 BUILD BY CHANGE CODE COPY
ERROR LMCOPY FAILED WITH
RETURN CODE: aaa FOR GROUP:
bbbbbbbb TYPE: cccccccc
Explanation
During change code processing, an attempt was made
to copy members from the build group to a temporary
data set. The LMCOPY service completed with return
code aaa.
User response
Try the build again. If it fails again, contact IBM
support.
FLM01072 ERROR UPDATING ACCOUNTING
INFORMATION FOR GROUP:
aaaaaaaa
TYPE: bbbbbbbb MEMBER:
cccccccc
MEMBER MUST EXIST
AT GROUP: dddddddd
Explanation
The VERIFY translator indicates that the accounting
information for the member must be updated, but the
member has not been drawn down to the group at
which the build or promote was invoked.
SCLM messages
754  z/OS: z/OS ISPF Messages and Codes

## Page 775

User response
Draw the member down to a development group,
rebuild and attempt the build or promote again. If the
problem recurs, contact the project manager.
Project manager response
Run IDCAMS against the data set(s) associated with
group dddddddd.
FLM01501 ERROR RETRIEVING BUILD MAP
INFORMATION CODE: aaa GROUP:
bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
No &bmap information could be retrieved for member
dddddddd.
User response
Possible return codes are:
8
The specified build map record does not exist.
Build the appropriate architecture member. Invoke
the processor again.
12
The format of the data retrieved was incorrect.
Delete the build map using the SCLM DELETE or
DELGROUP services, and build again to regenerate
it.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A large architecture definition may cause this error.
A severe I/O error occurred. Contact the project
manager.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
If an architecture definition member was being
built and the architecture definition member
is large (exceeding 800 statements) then split
the architecture definition member into two or
more high level architecture definition members.
These new architecture definition members may
then be referenced by a high level architecture
definition member. If this is not the problem, then
run IDCAMS against the accounting data set to
determine the problem.
FLM01502 ERROR UPDATING BUILD MAP
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
An error occurred during an attempt to write the build
map information for member dddddddd. The build
maps are stored in the accounting data set.
User response
Possible return codes are:
4
An I/O error occurred while attempting to
write the member's build map information to
the secondary accounting data set. Because
the primary accounting data set was correctly
updated, SCLM will use the correct information for
all references. However, the two accounting data
sets are no longer identical. Contact the project
manager.
8
The length of the &bmap exceeds the maximum
size (113,660 entries on each build map) allowed
by the accounting data set.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Possible return codes are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
20
A severe I/O error occurred. Run IDCAMS against
the accounting data set to determine the problem.
FLM01503 ERROR PURGING BUILD MAP
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
SCLM messages
Chapter 3. SCLM messages  755

## Page 776

Explanation
A VSAM error occurred while attempting to delete the
build map information for member dddddddd.
User response
Possible return codes are:
4
The member's accounting record was not found.
Determine why the accounting record was not
found.
8
An I/O error occurred while attempting to
purge the build map record from the secondary
accounting data set. The record was successfully
purged from the primary accounting data set, but
the primary and secondary accounting data sets
are no longer identical.
12
Unable to purge the build map record from the
primary accounting data set. Submit the job again.
If the problem occurs again, contact the project
manager.
16
Primary accounting VSAM data set enqueued.
Submit the job again after the data set is no longer
exclusively in use by another job.
20
An I/O error occurred while purging the build map
record. Submit the job again. If the problem occurs
again, contact the project manager.
Project manager response
If the return code is:
12 or 20
Run IDCAMS against the accounting data set to
determine the problem.
FLM02000 ERROR WRITING AUDITING
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
ACTION DATE: eeeeee
ACTION TIME: ffffffff
Explanation
An error occurred while attempting to write the
auditing information for member dddddddd.
User response
Possible return codes are:
8
The accounting record portion of the auditing
record exceeds the maximum amount of
dependent members (compools, included
members, and compilation units) allowed by SCLM.
Consequently, the auditing record was not written.
Change the member so that the number of
referenced dependents is decreased below the
maximum supported. Delete unnecessary change
codes and user data.
16
An I/O error occurred while attempting to write
the auditing record to the secondary auditing data
set. The primary auditing data set was correctly
updated, but the two auditing data sets are no
longer identical. Contact the project manager.
20
An I/O error occurred while writing the auditing
record to the primary auditing data set. Submit the
job again. If the problem occurs again, contact the
project manager.
Project manager response
If the return code is:
16
Run IDCAMS against the auditing data set to
determine the problem. If the secondary auditing
data set has been damaged, reallocate it and
initialize with data from the primary auditing data
set.
20
Run IDCAMS against the auditing data set to
determine the problem. If the primary auditing
data set has been damaged, reallocate it and
initialize with data from the secondary auditing
data set. If no secondary auditing data set exists,
initialize the primary auditing data set with a
backup copy of the primary auditing data set.
FLM02001 ERROR RETRIEVING AUDITING
INFORMATION, CODE: aaa TYPE:
bbbbbbbb
MEMBER: cccccccc
ACTION DATE: dddddd
ACTION TIME: eeeeeeee
Explanation
An error occurred while attempting to retrieve auditing
information for member cccccccc.
User response
Possible return codes are:
SCLM messages
756  z/OS: z/OS ISPF Messages and Codes

## Page 777

8
The member's audit record was not found.
Examine the information provided and determine if
any values need to be modified (such as the action
date). Submit the job again with the modified
values.
12
The member's audit record was successfully
retrieved, but the current version of the code does
not match the code version of the retrieved audit
record. Contact the project manager.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
If the return code is:
12
The version data set could have been tampered
with outside of SCLM or, an alternate project
definition specifies the same audit VSAM data set
but a different VERPDS to store member versions.
Correct the project definition in use.
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set to
determine the problem.
FLM02002 ERROR PURGING AUDITING
INFORMATION, CODE: aaa
GROUP: bbbbbbbb
TYPE: cccccccc
MEMBER: ddddddd
ACTION DATE: eeeeee
ACTION TIME: ffffffff
Explanation
An error occurred while attempting to purge auditing
information for member dddddddd.
User response
Possible return codes are:
4
The member's audit record was not found.
Examine the information provided and determine if
any values need to be modified (such as the action
date). Submit the job again with the modified
values.
8
an I/o error occurred while attempting to purge
the auditing record from the secondary auditing
data set. The record was successfully purged from
the primary auditing data set, but the primary
and secondary auditing data sets are no longer
identical.
12
An I/O error occurred while purging the auditing
record from the primary auditing data set. Submit
the job again. If the problem occurs again, contact
the project manager.
16
Primary audit VSAM data set enqueued. Submit
the job again after the data set is no longer
exclusively in use by another job.
20
An I/O error occurred while purging the auditing
record. Submit the job again. If the problem occurs
again, contact the project manager.
Project manager response
If the return code is:
12 or 20
Run IDCAMS against the auditing data set to
determine the problem.
FLM03001 ERROR RETRIEVING CROSS-
REFERENCE INFORMATION,
CODE: aaa CU NAME:
bbb(55)ccc(55)
CU TYPE: dddd
CU QUALIFIER: eeeeeeee
GROUP: ffffffff
Explanation
SCLM could not retrieve cross-reference information
for compilation unit bbb(55) ccc(55).
User response
Possible return codes are:
8
The accounting information for the compilation
unit was not found. Register the member with
SCLM using the SCLM editor, migration utility, or
the SAVE service. Run the processor again.
SCLM messages
Chapter 3. SCLM messages  757

## Page 778

12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of
the verification error, browse the accounting and
dependency information for the compilation unit
using the library utility. To correct the problem, edit
and save the member.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A severe I/O error occurred. Contact the project
manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set to
determine the problem.
24
Identify the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03002 ERROR UPDATING CROSS-
REFERENCE INFORMATION CODE:
aaa CU NAME: bbb(55) ccc(55)
CU TYPE: dddd
CU QUALIFIER: eeeeeeee
GROUP: ffffffff
Explanation
An error occurred while attempting to insert or update
information for compilation unit bbb(55) ccc(55) in the
cross-reference data set.
User response
Possible return codes are:
8
The length of the cross-reference information
exceeds the maximum size allowed by the cross-
reference data set. The maximum number of
combined compilation unit dependencies (upward
and downward) is 286. Reduce the number of
dependencies for the compilation unit.
12
SCLM internal error. Contact the project manager.
20
A severe I/O error occurred. Contact the project
manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
20
Run IDCAMS against the accounting data set to
determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03003 ERROR PURGING CROSS-
REFERENCE INFORMATION,
CODE: aaa CU NAME: bbb(55)
ccc(55)
CU TYPE: dddd
CU QUALIFIER: eeeeeeee
GROUP: ffffffff
Explanation
An I/O error occurred while attempting to delete
cross-reference information for compilation unit
bbb(55) ccc(55)
User response
Possible return codes are:
8
A severe I/O error occurred. Contact the project
manager.
16
The cross-reference data set is enqueued. Try the
job again later.
SCLM messages
758  z/OS: z/OS ISPF Messages and Codes

## Page 779

24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
8
A VSAM error occurred. Run IDCAMS against
the cross-reference data set to determine the
problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03021 ERROR ACCESSING ACCOUNTING
INFORMATION FOR DEPENDENT
COMPILATION UNIT, CU NAME:
aaa(55) bbb(55) CU TYPE:dddd
CU QUALIFIER: dddddddd CODE:
20
Explanation
An I/O error occurred while attempting to retrieve
accounting information for the dependent compilation
unit aaa(55) bbb(55).
User response
Submit the job again. If the problem recurs, contact
the project manager.
Project manager response
Run IDCAMS against the cross-reference data set to
determine the problem.
FLM03501 ERROR RETRIEVING
ACCOUNTING INFORMATION FOR
INTERMEDIATE FORM OF CU
NAME: aaa.(55) bbb.(55)
CU TYPE: ccc.
CU QUALIFIER: dddddddd
CODE: eee
STARTING WITH GROUP: ffffffff
TYPE: gggggggg MEMBER:
hhhhhhhh
Explanation
An error occurred while attempting to retrieve
accounting information for the specified intermediate
form, starting at group ffffffff and searching up through
the hierarchy.
User response
Possible return codes are:
8
The accounting information for the intermediate
form of the compilation unit was not found in any
group in the hierarchy defined, starting with group
ffffffff. This means that the compiled intermediate
form is missing or out of date. The member
containing the compilation unit needs to undergo
an SCLM build.
12
SCLM internal error. Contact the project manager.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A severe I/O error occurred. Contact the project
manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set to
determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03502 ERROR UPDATING ACCOUNTING
INFORMATION FOR
INTERMEDIATE FORM OF CU
NAME: aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
CODE: eee.
STARTING WITH GROUP: ffffffff
SCLM messages
Chapter 3. SCLM messages  759

## Page 780

TYPE: gggggggg MEMBER:
hhhhhhhh
Explanation
An error occurred while attempting to update
accounting information for the specified intermediate
form.
User response
Possible return codes are:
12
The record format of the member's intermediate
accounting data is incorrect for the current version
of SCLM. Contact the project manager.
20
An I/O error occurred while updating the
member’s intermediate accounting data. Contact
the project manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
12
Verify that the cross-reference data set is
compatible with the current release of SCLM.
20
Run IDCAMS against the cross-reference data set
to determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03503 ERROR PURGING ACCOUNTING
INFORMATION FOR
INTERMEDIATE FORM OF CU
NAME: aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd CODE:
eee
GROUP: ffffffff TYPE: gggggggg
MEMBER: hhhhhhhh
Explanation
An error occurred while attempting to purge
accounting information for intermediate form aaa(55)
bbb(55).
User response
Possible return codes are:
8
An I/O error occurred while purging. Submit the job
again and if the error recurs, contact the project
manager.
16
Target data set enqueued. Submit the job again
after the data set is no longer exclusively in use by
another job.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
8
Run IDCAMS against the cross-reference data set
to determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03504 ERROR RETRIEVING
ACCOUNTING INFORMATION FOR
INTERMEDIATE FORM OF CU
NAME: aaa (55) bbb (55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
CODE: eee
GROUP: ffffffff TYPE: gggggggg
MEMBER: hhhhhhhh
Explanation
An error occurred while attempting to retrieve
accounting information for intermediate form of
aaa(55) bbb(55).
User response
Possible return codes are:
8
The accounting information for the intermediate
form of the compilation unit was not found in
the specified group. This means that the compiled
intermediate form is missing or is out of date. The
member containing the compilation unit needs to
be rebuilt.
12
SCLM internal error. Contact the project manager.
SCLM messages
760  z/OS: z/OS ISPF Messages and Codes

## Page 781

20
An I/O error occurred while retrieving the
accounting information for the intermediate form
of the compilation unit. Contact the project
manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set to
determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM03505 ERROR PURGING INTERMEDIATE
FORM OF CU NAME: aaa(55)
bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
CODE: eee
GROUP: ffffffff TYPE: gggggggg
MEMBER: hhhhhhhh
Explanation
An error occurred while attempting to purge the
intermediate code for the specified intermediate form.
User response
Possible return codes are:
8
Unable to purge the intermediate code. Check
to see if the sublibrary is missing or a data
set security error occurred. Contact the project
manager.
16
The cross-reference data set was enqueued.
Submit the job again after the data set is no longer
exclusively in use by another job.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
8
Run IDCAMS against the cross-reference data set
to determine the problem.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. More
information on the FLMCNTRL Macro can be found
in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM04001 GROUP: aaaaaaaa IS NOT
DEFINED IN THE PROJECT
DEFINITION.
Explanation
Group aaaaaaaa is not defined to the project
definition.
User response
Verify that aaaaaaaa is the intended group. Verify that
the correct &libdef name was specified. Contact the
project manager.
Project manager response
Add the group to the project definition. More
information on defining the project environment can
be found in z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM04002 SPECIFIED GROUP: aaaaaaaa IS
NOT A DEVELOPMENT GROUP
Explanation
The specified group is not valid for the function
requested and must be defined to SCLM as a
development group. Contact the project manager for
a list of valid groups.
User response
Select a group that is defined in the project definition
as a development group.
FLM04003 TYPE: aaaaaaaa IS NOT DEFINED
IN THE PROJECT DEFINITION.
SCLM messages
Chapter 3. SCLM messages  761

## Page 782

Explanation
Type aaaaaaaa has not been defined in the current
project definition.
User response
Verify that aaaaaaaa is a type that is supposed to
contain SCLM data. If so, contact the project manager.
Project manager response
Add the type to the project definition.
FLM04005 AUTHORIZATION CODE: aaaaaaaa
IS NOT DEFINED TO GROUP:
bbbbbbbb
Explanation
Authorization code aaaaaaaa has not been defined
to SCLM as a valid authorization code for group
bbbbbbbb.
User response
Use an &authcode that has been defined to the
specified group. Contact the project manager for
a list of valid authorization codes. If the specified
authorization code is valid, contact the project
manager.
Project manager response
Check the project definition that defines the specified
group. The valid authorization codes for the group
are defined there. If authorization groups are used,
reference the FLMAGRP macros in the &libdef as
well. If the &authcode is valid, add it to the project
definition.
FLM04006 LANGUAGE: aaaaaaaa IS NOT
DEFINED IN THE PROJECT
DEFINITION
Explanation
Language aaaaaaaa is not defined in the project
definition used.
User response
Verify that the language of the member is defined in
the project definition. Specify a valid language and
submit the job again. Contact the project manager for
a list of valid languages.
Project manager response
Check the project definition for valid language names.
FLM04007 LANGUAGE: aaaaaaaa IS NOT
DEFINED FOR MEMBER: bbbbbbbb
TYPE: cccccccc
Explanation
Language, aaaaaaaa, is not defined in the project
definition used. If this message is received for an
existing member, the &libdef has probably changed
since the last time the source member was modified.
User response
Verify that the language of the member is defined
in the project definition. If it is not, specify a valid
language and submit the job again. Contact the project
manager for a list of valid languages.
Project manager response
Check the project definition for valid language names.
FLM04008 ACCOUNTING RECORD FOR
MEMBER: aaaaaaaa TYPE:
bbbbbbbb IS IN STATE: INITIAL
Explanation
The accounting record processing has not completed
for this member. This error can occur when there is an
active edit session for the member or when an active
edit session terminates abnormally. This error can
also occur when the LOCK or SAVE service terminates
abnormally.
User response
Use a new member name or have the owner of the
specified member free it. If you have an active edit
session for member aaaaaaaa, complete the session.
If you do not have an active edit session, use SCLM
option 3.1 to look at the accounting record for the
member. If the Accounting Status is not INITIAL,
accounting record processing has completed and the
error will not occur again. If the Accounting Status
is INITIAL, look at the Change User ID field in the
accounting record to identify the person who created
the record. Have the person who created the record
complete the requested processing or delete the
accounting record.
FLM04009 ACCOUNTING RECORD FOR
MEMBER: aaaaaaaa TYPE:
bbbbbbbb IS IN STATE: LOCKOUT
SCLM messages
762  z/OS: z/OS ISPF Messages and Codes

## Page 783

Explanation
The member has been locked.
User response
You must unlock the member before it can be edited.
Also, it must not exist in another development library
with an accounting record. You cannot lock a member
in one library if the member has been drawn down to
another library with an authorization code that allows
it to be promoted into a group that is part of the
first library's hierarchical view. To correct the problem,
change the authorization code of the existing member
so that it cannot be promoted, or promote the member
so that it can be drawn down.
FLM04010 GROUP CONTROLLED BY LIBRARY
aaaaaaaa.
Explanation
An attempt was made to access a group that is under
control of an external library. The failure occurred
because the group must be accessed via the external
library’s services.
User response
Specify a group that is under SCLM control.
FLM04011 DATA SET NAME: aaa(44) FOR
GROUP: bbbbbbbb TYPE: cccccccc
IS LONGER THAN ALLOWED.
LENGTH: ddd
Explanation
The sublibrary name created by concatenating the CU
qualifier suffix with the physical data set name is too
long.
User response
Contact the project manager.
Project manager response
Adjust the dsname parameter on the FLMGROUP
or FLMALTC macro for the group specified. The
parameter should be adjusted to reduce the size of the
data set name pattern to allow for the Ada sublibrary
suffix to be concatenated.
FLM04016 UNABLE TO PROCESS INCLUDES
FOR MEMBER. MEMBER:
aaaaaaaa TYPE: bbbbbbbb
Explanation
Member aaaaaaaa in Type bbbbbbbb had an
accounting error. SCLM is unable to process includes
for members with accounting errors.
User response
Correct the accounting error. For example, make sure
that the language and version are still valid for the
project.
FLM04029 PROJECT VSAM DATA SET
NOT DEFINED GROUP gggggggg,
DATABASE dddddddd, MACRO
mmmmmmmm
Explanation
A VSAM data set of type dddddddd is needed to
complete the operation requested by the user, but
there is no data set of that type defined for the group
gggggggg. The VSAM data sets for group gggggggg
were defined on the mmmmmmmm macro in the
project definition used by the user. The values for
mmmmmmmm will be either *FLMCNTRL if the VSAM
data sets from the FLMCNTRL macro were being used
or the name specified on FLMALTC macro referenced
by group gggggggg.
User response
Verify that the operation is being run against the
proper group and project definition.
Project manager response
Update the project definition to specify a VSAM data
set of type dddddddd for group gggggggg.
FLM04030 ERROR OPENING PROJECT VSAM
DATA SET GROUP: gggggggg,
DATABASE: ddddddddd, INTENT:
iiiiiiii, CODE: ccccc, MACRO:
mmmmmmmm, DATA SET:
nnnnnnnn.nnnnnnnnn.nnnnnnnn
Explanation
The VSAM data set nnnnnnnn.nnnnnnnnn.nnnnnnnn
could not be opened for iiiiiii access (where iiiiiiii
is either READ or UPDATE). The VSAM data set
was defined on the mmmmmmmm macro (where
mmmmmmmm is either *FLMCNTRL if the data set
was defined on the FLMCNTRL macro, or the name of
the FLMALTC macro referenced by group ggggggg).
The reason for the failure is indicated by the code:
SCLM messages
Chapter 3. SCLM messages  763

## Page 784

8
Unable to allocate the data set to a ddname
10
Unable to open the data set
12
Invalid key size for the data set
14
Unable to read from the data set
User response
Ensure that you have sufficient access to the VSAM
data sets in the project and that the correct project
definition is being used.
Project manager response
Check the data set nnnnnnnn.nnnnnnnnn.nnnnnnn to
ensure that it has been properly allocated, initialized
and protected and that the data set is not allocated
exclusively to another user of the system.
Check these items:
Code
Items to Check
8
• Check that the data set exists and the name
matches the name specified in the project
definition.
• Check that the data set is not allocated
exclusively to another user.
• Check that the device where the data set is
allocated is online.
10
• Check that the user has sufficient access to the
data set.
• Check that the data set is a valid VSAM data set.
• Check that the data set has been initialized.
12
Verify the key length of the data set with the key
length required by SCLM. More information on the
correct key lengths for the VSAM data sets can
be found in z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
14
Check that the data set has been initialized with at
least one VSAM record
FLM04031 IMPORT/EXPORT DATA
SET NAME DUPLICATES
PREVIOUSLY DEFINED DATA
SET NAME. DATA SET:
nnnnnnnn.nnnnnnnn.nnnnnnnn
MACRO: mmmmmmmm
DATABASE: dddddddd,
Explanation
The import/export data set
nnnnnnnn.nnnnnnnn.nnnnnnnn could not be used
because it duplicates a VSAM data set that
was defined for the dddddddd database on the
mmmmmmmm macro (where mmmmmmmm is either
FLMCNTRL if the data set was defined on the
FLMCNTRL macro, or the name of the FLMALTC
macro).
User response
Select a unique name for the import/export data set
name, specified in the mmmmmmmm macro.
FLM05001 EXISTING MEMBER’S
AUTHORIZATION CODE IS NOT
DEFINED TO THE GROUP GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc
ERROR GROUP: dddddddd
AUTHORIZATION CODE: eeeeeeee
Explanation
The member's authorization code, eeeeeeee, is not
defined to group aaaaaaaa. The member exists at a
higher level in the hierarchy, but it cannot be drawn
down because the authorization code assigned to it is
not allowed in group aaaaaaaa. You must change the
member's authorization code before drawing it down
to that group.
User response
It is possible that the function will succeed with a
different authorization code. Contact the &dbc for a
list of authorization codes that are valid for the group.
If none of the authorization codes defined to the
group work, try the same function at a different group.
Contact the project manager if all attempts fail.
Project manager response
The FLMGROUP macro lists the valid authorization
codes defined for this group in the project definition.
Do not attempt to add authorization codes to the
project definition unless you are familiar with the
risks associated with using authorization codes to
control SCLM operations as outlined in z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM05002 PREDECESSOR VERIFICATION
FAILED
SCLM messages
764  z/OS: z/OS ISPF Messages and Codes

## Page 785

INPUT GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
ERROR GROUP1: dddddddd
DATE: eeeeeeee TIME: ffffffff
ERROR GROUP2: gggggggg DATE:
hhhhhhhh TIME: iiiiiiii
Explanation
The version of the member in error group1 dddddddd
was not based on the member in error group2
gggggggg. During a promotion, this usually means that
a version of the member between these two groups
has been deleted. If the &authcode is being changed,
changes to the member in gggggggg will be lost if the
version in dddddddd is promoted.
The predecessor Date and Time fields in the
accounting information for the member in dddddddd
should contain the last modified Date and Time fields
for the next occurrence of the member within the
hierarchy.
For more information on specific contents of the
predecessor Date and Time fields, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
For the promote processor, if gggggggg is not the
group being promoted into, this message is a warning.
However, the promote processor, in conditional mode,
prevents the member in aaaaaaaa from replacing the
member in gggggggg. This problem can also occur if
the authorization codes are changed from other groups
in the project definition.
User response
For the promote processor, verify that the member in
aaaaaaaa contains all of the required changes present
in the member in gggggggg. If it does, and no other
promote verification errors are present, promote again
in unconditional mode. If other promote verification
errors are present, either correct the errors or use an
architecture member that controls as few members as
possible.
If you have tried to change the authorization code, and
the member is in a development group, verify that all
of the changes from the version in gggggggg have been
incorporated in aaaaaaaa. Then delete and recreate
the accounting information for the member using the
SCLM editor or the SAVE service. If aaaaaaaa is not
in a development group, the member must be drawn
down to a development group, and you must delete
the member in aaaaaaaa before using the procedure
outlined here. If you are not able to correct the
problem with the SCLM editor or SAVE service, contact
the project manager.
Project manager response
To locate and correct authorization code problems, see
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference for more information.
FLM05010 MEMBER LOCKED AT ANOTHER
GROUP INPUT GROUP: aaaaaaaa
TYPE: bbbbbbbb MEMBER:
cccccccc ERROR GROUP:
dddddddd AUTHORIZATION CODE:
eeeeeeee
Explanation
Member cccccccc has already been updated in
another hierarchy. The changes reside in error group
dddddddd, which is not in your hierarchy. You cannot
update the member because you would not be
working with the most current version of the member.
User response
Have the member promoted into a group that is in your
hierarchy (that is, one that appears on the SCLM Edit
- Entry panel). If the member cannot be promoted,
the member and its accounting information must be
deleted in error group dddddddd using the SCLM
library utility or the DELETE service. You can also
change the authorization code to restrict promotion.
FLM05020 ERROR ALLOCATING HIERARCHY
VIEW FOR TYPE: aaaaaaaa FROM
GROUP: bbbbbbbb CODE: ccc
Explanation
An error occurred during an attempt to allocate all
data sets that compose the hierarchical view for the
current SCLM function. The hierarchical view is for type
aaaaaaaa, and begins with the corresponding data set
at group bbbbbbbb
Possible return codes are:
4
A certain number of groups in the group hierarchy
was requested for allocation. The number of
requested groups exceeds the number of available
groups in the hierarchy starting with group
bbbbbbbb.
8
Type aaaaaaaa is not defined for the project.
The SCLM type aaaaaaaa might not be defined
in the project definition. The undefined type is
usually generated by SCLM when an asterisk (*)
is specified in the DFLTTYP parameter on an
FLMALLOC macro in the language definition of the
member being built.
SCLM messages
Chapter 3. SCLM messages  765

## Page 786

12
A group in the requested hierarchy view is not
defined for the project.
16
No data sets are physically allocated for this
hierarchical view.
20
An error occurred during an attempt to dynamically
allocate a data set in the hierarchy.
User response
For return codes:
4
No action is required. A hierarchical view
was created starting at group bbbbbbbb, and
continuing up through all the higher level defined
in the current project definition.
8
Determine if type aaaaaaaa has been incorrectly
specified as input. Determine if type aaaaaaaa
is not defined in the project definition (see the
project manager).
12
Determine if group bbbbbbbb has been incorrectly
specified as input. Determine if group bbbbbbbb
is not defined in the project definition (see the
project manager).
16
No data sets have been physically allocated
in the hierarchy for type aaaaaaaa starting at
group bbbbbbbb Determine if data sets should be
allocated, and allocate them if necessary. See the
project manager for assistance.
20
An error occurred during an attempt to dynamically
allocate a data set in the hierarchical view to a
ddname. Verify that data sets in the hierarchical
view are not allocated exclusively to another job,
and resubmit the job again.
Project manager response
Allocate the necessary data sets.
FLM05030 ERROR ALLOCATING BACKUP
DATASET FOR TYPE: aaaaaaaa
FROM GROUP: bbbbbbbb CODE:
ccc
Explanation
An error occurred during an attempt to allocate the
backup data set for the current promote.
Possible return codes are:
8
Type aaaaaaaa is not defined for the project.
12
Group bbbbbbbb is not defined for the project.
16
Data set project.bbbbbbbb.aaaaaaaa is not
physically allocated.
20
An error occurred during an attempt to dynamically
allocate the backup data set.
User response
8
Determine if type aaaaaaaa has been incorrectly
specified as input. Determine if type aaaaaaaa
is not defined in the project definition (see the
project manager).
12
Determine if group bbbbbbbb has been incorrectly
specified as input. Determine if group bbbbbbbb
is not defined in the project definition (see the
project manager).
16
Determine if data sets should be allocated,
and allocate them if necessary. See the project
manager for assistance.
20
Verify that data set is not allocated exclusively
to another job, and resubmit the job. If this does
not correct the error, see the project manager for
assistance.
Project manager response
8
Add an FLMTYPE macro for type aaaaaaaa.
12
Add an FLMGROUP macro for type bbbbbbbb.
16
Allocate data set project.bbbbbbbb.aaaaaaaa.
20
Check for dynamic allocation errors. Verify that the
region size is adequate.
FLM06501 TRANSLATOR RETURN CODE
FROM ===> aaa(16) ===> bbb
Explanation
This message identifies the return code received
from translator aaaaaaaaaaaaaaaa. If the return
code indicates success as defined on the FLMTRNSL
macro, all output is saved in the hierarchy and no
response is necessary. If the return code from the
SCLM messages
766  z/OS: z/OS ISPF Messages and Codes

## Page 787

translator did not meet the GOODRC specified for
the translator, SCLM saves translator output, such
as compiler listings, in the listings data set for the
processor if requested in the language definition.
User response
Use the listings data set to locate and correct all
errors identified by the translator. If the translator
is supplied with SCLM, check the return codes for
the translator. (See the SCLM Translators topic in
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference.) If the return code from
the translator is acceptable and the build function
indicates that the translator failed, contact the project
manager.
Project manager response
Change the GOODRC parameter of the FLMTRNSL
macro, which is defined in the project definition.
Note: SCLM provides some translators and parsers
for which return codes are documented in z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM06502 ERROR INVOKING TRANSLATOR:
aaaaaaaa, CODE: bbb
Explanation
SCLM could not invoke the aaaaaaaa translator.
The load module containing the translator might
be allocated exclusively to another job. There is a
possible error in the language definition that defines
the translator.
User response
If the translator has been used successfully in the past
and no changes were anticipated (for example, a new
compiler release), invoke the processor again. If the
translator is new or the problem recurs, contact the
project manager.
Project manager response
Verify that the parameters of the FLMTRNSL macro,
which are defined in the project definition, are correct.
More information on the FLMTRNSL macro can be
found in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM06503 PROBABLE SYSTEM/USER ABEND
FOR TRANSLATOR: aaaaaaaa
HEXADECIMAL VALUE OF RETURN
CODE: bbbbbbbb
Explanation
SCLM issues this message when an abend occurs for
translator aaaaaaaa.
User response
To determine the cause of the abend, look up the
hexadecimal return code in the z/OS MVS System
Codes. Check in the job log or system log for additional
messages that may provide more information. Use the
information provided in these messages to correct the
cause of the abend, and submit the job again.
FLM06504 NO TRANSLATORS DEFINED FOR
SCLM EXECUTION LANGUAGE:
aaaaaaaa.
Explanation
The translators defined for the language definition do
not contain any translators defined for SCLM. If any
translators are defined, they are defined for use in
external libraries.
User response
Execute the function against languages defined with
translators for SCLM execution.
Project manager response
Define translators for the languages that are
executable for SCLM.
FLM06510 ERROR ALLOCATING DATA SET:
aaa(44) FOR USER EXIT:
bbbbbbbb CODE: ccc
Explanation
An error occurred while attempting to allocate data set
aaa(44) for user exit bbbbbbbb. This data set should
contain the user exit routine to be invoked.
Possible return codes are:
12
SCLM internal error. Report this message, including
the message ID and all text fields, to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested, but the data
set was not partitioned.
28
The requested member could not be found.
SCLM messages
Chapter 3. SCLM messages  767

## Page 788

32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error code (in hexadecimal). If this is an
SMS error code (X'97xx') this will be followed by
SMS: ddddd; where ddddd is the IGD message
number associated with the error.
User response
Contact the project manager.
Project manager response
For return codes:
<65
Report this message (including the message ID
and all text fields) to IBM support.
X'xxxx'
SVC 99 error codes are described in the z/OS MVS
Programming: Authorized Assembler Services Guide
in the topic about the SVC 99 reason codes (or see
Appendix A of the ISPF Tutorial). For SMS error
codes, the IGD message can be located in the z/OS
MVS System Messages, Vol 8 (IEF-IGD). The listing
may have one or two leading zeros added to the
ddddd value.
FLM06511 ERROR INVOKING USER EXIT
ROUTINE: aaa(16), CODE: 32
Explanation
SCLM could not invoke the user exit. The load module
containing the user exit might be allocated exclusively
to another job. There is a possible error in the project
definition that defines the user exit.
User response
If the user exit has been used successfully in the past,
submit the job again. If the user exit is new or the
problem recurs, contact the project manager.
Project manager response
Verify that the user exit executes correctly outside of
SCLM. Verify that the user exit is defined correctly in
the project definition. For more information on user
exits, see z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM06512 VERIFICATION ERROR FROM
USER EXIT ROUTINE: aaa(16),
CODE: bbb
Explanation
The return code from user exit aaa(16) did not meet
the acceptable criteria specified for the user exit. The
output produced depends on the user exit routine.
User response
Review the local software configuration management
for information about the user exit.
Project manager response
For more information on user exits, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM06513 PROBABLE SYSTEM/USER ABEND
FOR USER EXIT ROUTINE: aaa(16)
HEXADECIMAL VALUE OF RETURN
CODE: bbbbbbbb
Explanation
SCLM issues this message when an ABEND (user exit
return code greater than 4096) occurs. SCLM also
provides the hexadecimal value of the user exit return
code.
User response
Contact the project manager.
Project manager response
Use the information provided in this message to
correct the cause of the ABEND. See z/OS MVS System
Codes for complete information on the return codes.
FLM07001 AUTHORITY CODE: aaa ON DATA
SET: bbb(44) RESULTED FROM
ATTEMPT TO UPDATE DATA. ATTR:
c MACRO RC: ddd EXIT RC: eee
EXIT REASON: fff
Explanation
An attempt was made to perform an SCLM function
without the proper authority. Users cannot update
SCLM control data, using SCLM functions, unless they
have the authority to update the data set to which the
control data is related.
DSNAME
Data set being accessed.
SCLM messages
768  z/OS: z/OS ISPF Messages and Codes

## Page 789

ATTR
R: READ, U: UPDATE, C: CONTROL, A: ALTER.
MACRO RC
For a code of 8, either the LOCATE macro failed to
find the data set, or the RACROUTE macro failed.
Otherwise, the MACRO RC contains the return code
from the LOCATE macro or the RACROUTE macro.
EXIT RC
For a MACRO RC of 8, this value contains the return
code from a data set security or the SAF router
exit routine. For RACF, this is the RACHECK return
code. Otherwise it is set to zero.
EXIT REASON
For a MACRO RC of 8, this value contains the
reason code from a data set security or the SAF
router exit routine. For RACF, this is the RACHECK
reason code. Otherwise it is set to zero.
Note: For the INIT service call using a program,
only the first line will appear, indicating the
user does not have READ access to the project
definition data set.
User response
Verify that you specified the correct group and type
for the function you are requesting. Also verify that
the requested data set exists. If the request was valid,
get update authority to the data set identified in the
message.
FLM07002 ERROR PERFORMING AN
ENQUEUE CODE: aaa QNAME:
bbbbbbbb
RNAME LENGTH: ccc
RNAME: ddd(60)
Explanation
The requested resource was enqueued by another
job. The enqueued resource, which is usually a data
set, is identified by RNAME. The code refers to the
return code from the enqueue macro. RNAME LENGTH
identifies the size of RNAME in bytes because RNAME
can contain trailing blanks. QNAME is the name of the
queue used for the enqueue operation.
User response
Try the job again later.
FLM07004 ERROR ALLOCATING A
TEMPORARY DATA SET CODE: aaa
DDNAME: bbbbbbbb
LRECL: cccccccc RECFM: dddddddd
NUMRECS: eeeeee DISP: ffff
DSNAME: ggg(44)
Explanation
An error occurred while attempting to allocate a
temporary data set.
Possible return codes (aaa) are:
4
Data set name omitted from input.
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error, reason code (in hexadecimal). Some
possible values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
Submit the job again. If the error recurs, contact the
project manager.
Project manager response
Verify that all data set names are correctly defined
and exist. For return codes of the form X'xxxx', refer
to the z/OS MVS Programming: Authorized Assembler
Services Guide for a description of the SVC 99 reason
codes (or see Appendix A of the ISPF tutorial). For SMS
error codes, the IGD message can be located in the
z/OS MVS System Messages, Vol 8 (IEF-IGD). The listing
SCLM messages
Chapter 3. SCLM messages  769

## Page 790

may have one or two leading zeros added to the ddddd
value. For additional assistance, contact IBM support.
FLM07005 ERROR RETRIEVING DIRECTORY
INFORMATION FOR TYPE:
aaaaaaaa CODE: bbb
Explanation
An error occurred during an attempt to retrieve the
directory information for a member in type aaaaaaaa.
Possible return codes are:
16
There are two possible reasons for this error:
• The hierarchy is not defined.
• Data sets required by the current operation are
not allocated.
20
A data set in type aaaaaaaa could not be opened,
possibly because:
• The hierarchy contains more than the maximum
allowed 123 extents
• A data set required by the current operation
could not be opened, possibly because the data
set is allocated exclusively to another job, or
because of a data set security protection error.
User response
Contact the project manager.
Project manager response
If the return code is:
16
A hierarchy error might have occurred.
Reassemble the project definition and make
certain that a return code of 0 is returned.
Determine if all data sets required by the current
operation are allocated. If not, allocate them, exit,
and reenter SCLM.
20
Verify that data sets in type aaaaaaaa are not
allocated exclusively to another job. Verify that the
user has the appropriate data set security access.
Check the number of extents used for all data sets
allocated to type aaaaaaaa in the hierarchy. If the
total number of extents used is greater than 123,
some of the data sets must be reallocated with a
larger block size.
FLM07006 ERROR ACCESSING MEMBER:
aaaaaaaa TYPE: bbbbbbbb,
ACCOUNTING GROUP: cccccccc
DATA SET NAME: ddd(44) CODE:
eee
Explanation
One of these has occurred:
• The type bbbbbbbb is not defined in the project
definition.
• The accounting record for the member exists, but
the corresponding data set for group cccccccc and
type bbbbbbbb does not.
• The data set ddd(44) might be allocated exclusively
to another job.
• The member aaaaaaaa does not exist in data set
ddd(44).
Possible return codes are:
8
The member is not registered with SCLM, the data
set ddd(44) does not exist, or member aaaaaaaa
does not exist within data set ddd(44).
16
SCLM cannot retrieve the directory information for
the member.
User response
If the return code is:
8
Verify that data set ddd(44) exists and contains
member aaaaaaaa. If group cccccccc is a
development group, you can use the SCLM Editor
to add the member into the data set. If the
member should not exist in group cccccccc, use
the SCLM Library Utility or the DELETE service
to remove the accounting information for the
member.
16
Check the input parameters and verify that the
type exists in the project definition. Verify that data
set ddd(44) is not allocated exclusively to another
job.
Project manager response
Determine if the data set should be allocated, and
allocate it if necessary. After reallocating the data set,
exit and reenter SCLM. Delete the accounting record
for the member at that group and type. Be aware that
an inconsistency in the hierarchy occurred because an
accounting record existed for a member that does not
exist.
FLM07007 ACCOUNTING INFORMATION IS
NOT ACCURATE FOR MEMBER:
aaaaaaaa TYPE: bbbbbbbb
SCLM messages
770  z/OS: z/OS ISPF Messages and Codes

## Page 791

ACCOUNTING GROUP: cccccccc
MEMBER GROUP: dddddddd
Explanation
One of these has occurred:
• Member aaaaaaaa is out of date. The accounting
information for the member does not match the
contents of the member. If neither group is a
development library, it is possible that the member
has been updated outside of SCLM control.
• The member exists in dddddddd and the accounting
record exists in cccccccc, but the data set
corresponding to cccccccc.bbbbbbbb does not exist.
It is also possible that all groups in the hierarchy (for
this type) are not of the same record format.
User response
If the member is editable, register the member with
SCLM using the SCLM editor, migration utility, or the
SAVE service. If the member is non-editable, delete
the member with the SCLM library utility or the
DELETE service and regenerate the member with the
SCLM build function.
Verify that the data set for cccccccc.bbbbbbbb is
allocated. If it is not, contact the project manager.
If the groups are not all the same record format, have
the project manager allocate all data sets with the
same record format
Project manager response
If the data set for cccccccc.bbbbbbbb is allocated
verify that all groups associated with the type are
of the same record format. If the data set is
not allocated, allocate it, exit, and reenter SCLM.
Delete the accounting record for the member at
cccccccc.bbbbbbbb. Be aware that an inconsistency in
the hierarchy occurred, because an accounting record
existed for a nonexistent member.
FLM07008 ERROR ACCESSING MEMBER:
aaaaaaaa GROUP: bbbbbbbb,
TYPE: cccccccc, DATA SET NAME:
ddd(44) CODE: eee
Explanation
Member aaaaaaaa could not be retrieved. Data set
ddd(44) might be empty, or could not be opened.
Possible return codes are:
32
Member aaaaaaaa could not be found in the data
set
36
Type cccccccc could not be found
User response
For return codes:
32
The member is not registered with SCLM, or was
deleted. Edit the member under SCLM, or use the
Migrate or Save service.
36
Contact the project manager for assistance.
Project manager response
Determine if the type is defined in the project
definition. If not, determine if it must be defined
and re-assemble the project definition if necessary.
Provide other assistance as needed.
FLM07009 ERROR ACCESSING MEMBER:
aaaaaaaa, FOR GROUP:
bbbbbbbb, TYPE: cccccccc, DATA
SET NAME: ddd(44) CODE: eee
Explanation
A possible I/O error occurred for data set ddd(44)
while attempting to access member aaaaaaaa, or data
set ddd(44) does not exist.
Possible return codes are:
8
Enqueue error for data set ddd(44).
12
Type cccccccc not found in hierarchy or the data
set is not allocated.
20
An error occurred during an attempt to allocate
data set ddd(44).
User response
Verify that the data set is not allocated exclusively to
another job. Contact the project manager.
Project manager response
For return code:
8
Verify that the data set is not allocated exclusively
to another job.
12
Determine if the type cccccccc is defined in the
project definition. If not, determine if the type
is required for the project and add it to the
SCLM messages
Chapter 3. SCLM messages  771

## Page 792

project definition if necessary. Determine if data
set ddd(44) exists. If it does not, allocate it, exit,
and reenter SCLM. Then attempt the operation
again.
20
Verify that the data set exists and that it is not
allocated exclusively to another job.
36
The unit was not specified
FLM07010 ERROR UPDATING DIRECTORY
INFORMATION AT GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc DATA SET
NAME: ddd(44) CODE: eee
Explanation
SCLM could not update the data set directory for this
member.
Possible return codes are:
4
Unable to update the directory information of a
data set allocated with RECFM=U.
8
Unable to get member name from the directory.
12
The file is not closed.
16
The file control block is NIL.
20-36
Indicates that an I/O error occurred.
24
This can also indicate that the target data set could
not be accessed.
41
BINDER STARTDialog failed
42
BINDER CREATEWorkmod failed
43
BINDER INCLUDE failed
44
BINDER SETOption failed
45
BINDER SAVEWorkmod failed
46
BINDER DELETEWorkmod failed
User response
Contact the project manager. For codes 41 through 46,
refer to the subsequent FLM07021 message and IEW
messages for information on the failure and corrective
response.
Project manager response
Contact IBM support. For codes 41 through 46,
refer to the subsequent FLM07021 message and IEW
messages for information on the failure and corrective
response.
FLM07011 ERROR ALLOCATING TEMPORARY
DATA SET FOR TRANSLATOR: aaa.
DATA SET NUMBER: bbb CODE: ccc
Explanation
An error occurred in allocating a temporary data set
for translator aaa. The data set number identifies
the relative position of the FLMALLOC macro used to
allocate the data set for that translator.
Possible return codes are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support..
X'xxxx'
SVC 99 error, reason code (in hexadecimal). Some
possible values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'0410'
Specified ddname unavailable.
X'0484'
Not enough space on any available pack.
X'1708'
Data set does not exist.
SCLM messages
772  z/OS: z/OS ISPF Messages and Codes

## Page 793

X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
This message indicates that an error occurred while
allocating a data set to be used by a translator.
If you receive a return code of 32 or X'xxxx', try the
operation again. If the problem recurs or you receive
any other return codes, contact the project manager.
Project manager response
If the return code is :
12
SCLM internal error. Contact IBM support.
Note: For the next set of codes, you should know that
the data set number corresponds to an FLMALLOC
macro associated with the specified translator (the
first FLMALLOC is data set 1, the second is data set
2 and so on).
16
Check FLMALLOC to ensure that the data set name
has been specified correctly.
20
Check FLMALLOC to ensure that all data set
attributes have been specified correctly.
24
Check FLMALLOC to ensure that a sequential data
set was not specified when a partitioned data set
was expected.
28
Check FLMALLOC to ensure that the correct
member name was specified.
32
Retry the operation. If the same problem occurs,
contact IBM support.
X'xxxx'
An error was received from SVC 99 during the
allocation. The SVC 99 reason code is specified
in the message (xxxx). Refer to the z/OS MVS
Programming: Authorized Assembler Services Guide
for a description of the SVC 99 reason codes or
see Appendix A of the ISPF Tutorial. For SMS error
codes, the IGD message can be located in the z/OS
MVS System Messages, Vol 8 (IEF-IGD). The listing
may have one or two leading zeros added to the
ddddd value. Some possible codes are:
X'0484'
Decrease the value specified for the RECNUM
parameter on the FLMALLOC macro and try
again.
X'1708'
If a user supplied data set naming convention
is used, try replacing the data set name for the
FLMCPYLB macro with variable @@FLMDSN.
FLM07012 ERROR SAVING MEMBER
aaaaaaaa TO GROUP: bbbbbbbb
TYPE: cccccccc, DATA SET NAME:
ddd(44) CODE: eee
Explanation
An error occurred during an attempt to copy member
aaaaaaaa to data set ddd(44). This message can be
preceded by an MVS system error message. Common
errors are:
D37
Primary space is full and secondary space is not
requested for data set ddd(44).
B37 or E37
The directory is full, or the maximum number of
extents (123) was exceeded, or the volume and
VTOC of data set ddd(44) are full and secondary
volumes are not available.
SVC99 error
RECFM of target data set not the same as RECFM
as of source data set.
RACF error
Error accessing data set ddd(44) due to a data set
security protection error.
Possible return codes:
12
A ddname is not allocated properly. A PDS was
allocated without a member name, a sequential
data set was allocated with a member name, or
some other allocation error occurred.
16
The output data set is full.
20
RACF failed or the input member was not found.
24
An input parameter was not valid.
28
A member entry could not be created because the
input member is an alias or has TTR notes.
32
Open failed or TSOLNK failed.
User response
Refer to the z/OS MVS System Messages manuals for
descriptions of the MVS error messages and how to
resolve them. If this message was not preceded by
an abend code, verify that the language definition in
SCLM messages
Chapter 3. SCLM messages  773

## Page 794

use does not contain a DDNAME that is used more
than once. Verify that the language definition does not
include member names on sequential data sets. Verify
that partitioned data sets have member names where
necessary.
Project manager response
Provide assistance as needed.
FLM07013 COPY FAILED AT GROUP:
aaaaaaaa, TYPE: bbbbbbbb, FOR
DATA SET NAME: ccc(44) CODE:
ddd
Explanation
An error occurred during an attempt to copy one or
more members to data set ccc(44).
Possible return codes are:
4
One or more members specified to be copied to
data set ccc(44) were not present in the source
data set(s), or the number of directory blocks
allocated for data set ccc(44) is inadequate.
8
The data set ccc(44) might be allocated exclusively
to another job.
12, 16
The data set ccc(44) does not exist or is not
allocated with the same attributes as data sets for
type bbbbbbbb.
28
Three possibilities exist:
1. IEBCOPY attention interrupt error
2. A member to copy could not be found
3. A member entry could not be created
User response
For return codes:
4
Perform these actions:
1. Determine if all members that must be copied
in the current operation are present in their
source data sets.
2. Reallocate data set ccc(44) with more directory
blocks.
8
Determine if the data set is allocated exclusively
to another job. When the data set is available
resubmit the job.
12, 16
Determine if the data set exists and is allocated
with the correct attributes. Contact the project
manager if needed.
28
Make sure that all members to copy exist.
Determine if the data set has free directory entries.
Project manager response
Provide assistance as needed.
FLM07014 COPY FAILED FOR GROUP:
aaaaaaaa, TYPE: bbbbbbbb, AT
DATA SET NAME: ccc(44) ABEND
CODE: ddd
Explanation
SCLM was unable to update data set ccc(44) because
of an ABEND during the copy operation.
Common ABEND codes and their meanings are:
D37
Primary space is full and secondary space is not
requested in the data set ccc(44).
B37 or E37
The directory is full; the maximum number of
extents (123) was exceeded; or the volume and
VTOC of the data set ccc(44) are full and secondary
volumes are not available.
User response
Check for MVS system error messages for detailed
information. Submit the job again after performing
these operations:
1. Compress the data set ccc(44) or reallocate it with
more space or directory blocks.
2. Verify that the volume and VTOC for the data set
ccc(44) are not full. Move the data set if they are
full.
FLM07015 ERROR ACCESSING DATA SET
aaa(44) FOR GROUP: bbbbbbbb
TYPE: cccccccc RETURN CODE ddd.
Explanation
SCLM was attempting to access the data set aaa(44)
for an operation at group bbbbbbbb and type cccccccc,
and a system error occurred.
User response
Contact the project manager.
SCLM messages
774  z/OS: z/OS ISPF Messages and Codes

## Page 795

Project manager response
The LOCATE/CAMLST macro is used to access the data
set. The return code ddd is the value returned from
that macro. For each return code only some of the
reasons are given here; for a complete list and for
additional information on LOCATE see z/OS DFSMSdfp
Advanced Services.
Possible return codes are:
12
A possible reason for this error is:
• An alias data set name was found
16
Data set does not exist at the lowest catalog index
level specified.
20
A syntax error exists in the data set name
24
Possible reasons for this error are:
• A permanent input/output error occurred.
• An unrecoverable error occurred.
28
An input parameter address to LOCATE cannot be
used
FLM07016 MULTIPLE FLMCPYLBS CANNOT
BE ALLOCATED WITH DISP: ddd
FOR TRANSLATOR: aaa DATA SET
NUMBER: bbb.
Explanation
The language definition contains an FLMALLOC with
a disposition parameter ddd requesting allocation of
multiple data sets. Multiple data set allocations must
use disposition SHR. The disposition parameter is
abbreviated as follows: S for SHR, M for MOD, O for
OLD, and N for NEW.
User response
Contact the project manager
Project manager response
Find the language definition that is causing the
problem. For this message, aaa represents the
occurrence of the FLMTRNSL macro in the language
definition and bbb represents the occurrence of
the FLMALLOC macro defined for that translator.
Either remove all but one FLMCPYLB macro for the
allocation or change the DISP parameter on the
FLMALLOC macro. Then reassemble and relink the
project definition.
FLM07020 DATA SET aaa(44) DOES NOT
EXIST FOR GROUP: bbbbbbbb
TYPE: cccccccc
Explanation
The data set specified by aaa(44) does not exist. This
data set is needed for an operation in group bbbbbbbb
and type cccccccc.
User response
Determine if data set aaa(44) should be allocated, and
allocate it if necessary. Contact the project manager if
you need the data set but are unable to allocate it.
Project manager response
Determine whether data set aaa(44) is needed for
the function that issued the error message. (More
information about function usage of data sets can be
found in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.) Allocate the data set if
necessary, or inform the user that the data set should
not be used for the current function.
FLM07021 BINDER RETURN CODE: aaa AND
REASON CODE: bbbbbbbb
Explanation
SCLM could not update the data set directory for this
member.
User response
Refer to the MVS Program Management: Advanced
Facilities publication, IEWBIND function reference for
return and reason codes.
Project manager response
Provide assistance as needed.
FLM07030 PURGE FAILED FOR GROUP:
aaaaaaaa TYPE: bbbbbbbb DATA
SET NAME: ccc(44) CODE: ddd
Explanation
An error occurred while purging members from the
specified group and type in data set ccc(44)
Possible return codes are:
8
The data set ccc(44) is allocated exclusively to
another job.
SCLM messages
Chapter 3. SCLM messages  775

## Page 796

12
An I/O error occurred while accessing the data set
ccc(44).
16
SCLM is unable to allocate data set ccc(44).
20
An SCLM internal error occurred or the user does
not have the proper security access to the data set.
User response
Verify that the data set ccc(44) exists and that it is not
allocated exclusively to another job. Ensure that you
have the proper security access to the data set. Submit
the job again.
Project manager response
If the return code is 20 contact SCLM Program
Support.
FLM07031 WARNING, UNABLE TO PURGE
MEMBER(S) BECAUSE MEMBER(S)
ARE MISSING FROM GROUP:
aaaaaaaa TYPE: bbbbbbbb
Explanation
One or more members in TYPE bbbbbbbb are missing.
Only accounting information exists in group aaaaaaaa.
The accounting information will be deleted.
User response
Verify that no members involved in the promotion
should have existed in GROUP aaaaaaaa.
FLM07032 ERROR BUILDING IOTYPE=S
INPUT DATA SET FOR
TRANSLATOR:aaaaaaaa DATA SET
NUMBER: bbb CODE: ccc
Explanation
SCLM was unable to build the input data set possibly
due to an out of space condition.
Possible return codes are:
16
Data set out of space.
20
Data set access failed.
User response
If the return code is:
16
Compress the data set and resubmit the job. If the
error persists, notify the project manager.
20
Contact the project manager.
Project manager response
If the return code is:
16
Increase the RECNUM size on the FLMALLOC
macro. The data set number identifies the relative
position of the FLMALLOC data set for that
translator.
20
Assign the appropriate data set access security
level.
FLM08500 ERROR PERFORMING
VERSIONING
Explanation
A processing error occurred while performing
versioning.
User response
Examine the error messages printed with the current
message to determine the cause of the problem.
FLM08501 ERROR COPYING MEMBER TO
VERSION DATA SET CODE aaa
GROUP bbbbbbbb TYPE cccccccc
MEMBER dddddddd VERSION
DATA SET eeeeeeee
Explanation
An error occurred while copying an SCLM member
to the versioning data set. The member was not
versioned.
Possible codes for aaa are:
8
The LRECL, DSORG, or RECFM of the version
PDS is invalid, a member within the version PDS
is corrupted, or a temporary version data set
could not be allocated. See message FLM39203,
FLM39222, or FLM39225.
Note: If this message is not accompanied by
message FLM39203 or message FLM39222, check
that the member has not been modified in the
versioning data set (VERPDS). If it has, create a
new member by copying the text you want to save
into the new member. Then cancel out of the edit
session. The version data set has been tampered
SCLM messages
776  z/OS: z/OS ISPF Messages and Codes

## Page 797

with. Point this out to your project manager, delete
all versions associated with the member, and then
recreate the member under its old name.
12
Versioning PDS could not be opened. See message
FLM39200.
16
Source data set could not be opened. See message
FLM39200.
20
Version PDS could not be opened. See message
FLM39225.
1xx
SuperCU not successful (100 + SuperCU return
code). See message FLM39220. See SuperCU
documentation for the meaning of the SuperCU
return code.
In the message, bbbbbbbb is the GROUP being
versioned, cccccccc is the TYPE being versioned,
dddddddd is the MEMBER being versioned, and
eeeeeeee is the fully qualified data set name of
versioning PDS.
User response
See the referenced message for each return code to
determine the action to be taken.
FLM08502 ERROR REBUILDING SOURCE
FROM VERSION, CODE: aaa
Explanation
The retrieval process for the specified version failed.
Possible codes for aaa are:
8
Version, member, or retrieval data set not found.
32
Error freeing versioning data sets.
48
Corrupt version data.
1nn
Error processing delta data. See message
FLM39220 for possible values of nn.
User response
Verify the data sets are allocated, cataloged, and
accessible to SCLM; otherwise, contact the project
manager.
Project manager response
Allocate, catalog, or make the data sets accessible to
SCLM.
FLM08503 GETMAIN ERROR IN VERSIONING
CODE: 004
Explanation
A storage error occurred attempting to create, delete,
or retrieve a version. The amount of storage retrieved
was insufficient to complete the version process.
User response
Verify the version data sets are allocated, cataloged,
and accessible to SCLM. Otherwise, contact the project
manager.
Project manager response
Verify the version data sets are allocated, cataloged,
and accessible to SCLM. Otherwise, contact IBM
support.
FLM08504 FREEMAIN ERROR IN
VERSIONING CODE: 004
Explanation
A storage error occurred attempting to create, delete,
or retrieve a version. Storage previously obtained for
the version process was not properly freed.
User response
Verify the version data sets are allocated, cataloged,
and accessible to SCLM. Otherwise, contact the project
manager.
Project manager response
Verify the version data sets are allocated, cataloged,
and accessible to SCLM. Otherwise, contact IBM
support.
FLM09002 THE aaaa REPORT WILL APPEAR
IN bbbb
Explanation
Data set bbbb will contain the report output. This
message is provided for information only.
FLM09004 THE aaaa MESSAGES WILL
APPEAR IN bbbb
SCLM messages
Chapter 3. SCLM messages  777

## Page 798

Explanation
Data set bbbb will contain the message output. This
message is provided for information only.
FLM09006 THE aaaa LISTING WILL APPEAR
IN bbbb
Explanation
Data set bbbb will contain the listing output. This
message is provided for information only.
FLM09008 RETURN CODE = aaaaaaaa
Explanation
The return code for this function is aaaaaaaa. This
message is provided for information only.
FLM20001 IF PARSER LISTINGS WERE
CREATED, THEY WILL APPEAR IN
DSN: aaa(44).
Explanation
Any listings generated by the parser will appear in data
set aaa(44).
User response
If parser listings were created, review the parser
listings for notification or error information and
respond accordingly. If parser listings were not
created but were expected, see your project manager.
Project manager response
If the dsname (DSN) value is blank and parser
listings should have been created, check the language
definition to ensure that the parser listings are
allocated with the PRINT=I or PRINT=Y option.
FLM20002 THE VERSION OF THE MEMBER
YOU ARE EDITING DOES NOT
HAVE AN ACCOUNTING RECORD
AT THE HIERARCHY LEVEL IN
WHICH IT WAS FOUND. ANOTHER
VERSION EXISTS IN THE SCLM-
CONTROLLED HIERARCHY.
Explanation
The member that you selected to edit exists in the
hierarchy in two different places. The group at which
the text of the member was found is not the same
as the group at which the first occurrence of the
accounting record was found. The version you are
editing might be out of date. Someone might have
copied a version of this member into a group in the
hierarchy without doing a MIGRATE or a SAVE.
User response
Verify that this is the correct version of the member
that you wish to edit. If not, cancel from the current
edit session, and use the Library Utility to delete the
back-level version; then edit the member again. You
should not promote the member without first checking
to see whether the two versions need to be manually
merged to prevent loss of data.
Project manager response
Check the project definition FLMALTC parameters. It is
possible that you have two groups using the same data
set to store members, and accounting information.
This could lead to data integrity problems. Either
change the promote path for one of the groups so that
they cannot affect each other, or change the dsname
specified on the FLMALTC macro for one of the groups.
FLM20003 MEMBER LEVEL LOCKING IS IN
FORCE AND USER: aaaaaaaa
HAS THE MEMBER LOCKED.
PLEASE CONTACT THE SCLM
ADMINISTRATOR TO UNLOCK THE
MEMBER: bbbbbbbb.
Explanation
The member that you selected to edit is owned by
another user. While the member is locked only the
user who last changed it or an SCLM administrator can
make any changes.
User response
Ask user aaaaaaaa or an SCLM administrator to
transfer ownership of the member to you using option
T on the SCLM Library Utility - Entry Panel, and then try
again.
FLM20004 ERROR CREATING THE ENCODING
TEMPORARY DATA SET. Return
Code=rr
Explanation
An error was encountered attempting to create a
temporary data set into which SCLM will encode.
User response
Determine why SCLM had a problem allocating a
temporary data set.
SCLM messages
778  z/OS: z/OS ISPF Messages and Codes

## Page 799

FLM20005 ERROR DECODING THE
MEMBER nnnnnnn, Return
Code=rr ERROR MSG:
xxxxxxxxxxxxxxxxxxxxxxxxxxx
Explanation
SCLM encountered a problem attempting to decode
the member nnnnnnn
User response
Use the error message to determine the why SCLM
was unable to decode the member nnnnnnnn.
FLM20006 ERROR ENCODING THE
MEMBER nnnnnnn, Return
Code=rr ERROR MSG:
xxxxxxxxxxxxxxxxxxxxxxxxxxx
Explanation
SCLM encountered a problem attempting to encode
the member nnnnnnn
User response
Use the error message to determine the why SCLM
was unable to encode the member nnnnnnnn.
FLM20007 WARNING MEMBER nnnnnnnn
WAS ALREADY ENCODED.
Explanation
SCLM attempted to encode the member nnnnnnn but
found that it was already encoded. This is a warning
message.
FLM20008 WARNING INPUT DSN/MEMBER
WAS ALREADY ENCODED. INPUT
MEMBER : nnnnnnn
Explanation
SCLM attempted to encode the member nnnnnnn but
found that it was already encoded.
FLM20009 WARNING INPUT DSN/MEMBER
WAS ALREADY DECODED. INPUT
MEMBER : nnnnnnn
Explanation
SCLM attempted to decode the member nnnnnnn but
found that it was already decoded.
FLM30001 CHANGE CODE DELETIONS NOT
SUPPORTED IN THIS UTILITY
Explanation
A delete change code request was specified from a
utility that does not support this request. The change
code was not deleted.
FLM30002 USER ENTRY DELETIONS NOT
SUPPORTED IN THIS UTILITY
Explanation
A delete user entry request was specified from a utility
that does not support this request. The user entry was
not deleted.
FLM32101 MIGRATION UTILITY INITIATED -
aaaaaaaa ON bbbbbbbb
Explanation
The migration process has started. aaaaaaaa is the
time of the message. bbbbbbbb is the date of the
message. This message is provided for information
only.
FLM32102 INVALID MIGRATE MODE: X
SPECIFIED. VALID VALUES
ARE C (CONDITIONAL), U
(UNCONDITIONAL), OR F
(FORCED).
Explanation
The Migrate mode parameter on the FLMCMD service
is not valid.
User response
Verify that the proper number of parameters have
been specified. Use one of these values, and try again.
C
Conditional (default)
U
Unconditional
F
Forced
FLM32201 UNABLE TO READ DIRECTORY FOR
DATA SET NAME: aaa(44) GROUP:
bbbbbbbb TYPE: cccccccc CODE:
ddd
Explanation
An error occurred while attempting to read the
directory of the data set aaa(44).
Possible return codes are:
SCLM messages
Chapter 3. SCLM messages  779

## Page 800

16
SCLM is unable to open the data set
20
An internal error occurred while attempting to read
the data set directory.
User response
Verify that the data set directory can be accessed by
using the SCLM editor to browse the data set. If you
cannot browse the data set, correct the problem and
resubmit the job. Possible problems are that the data
set is enqueued or the data set does not contain a
valid directory.
FLM32302 NO MEMBERS MATCHING
SELECTION CRITERIA
Explanation
The migration utility did not attempt to migrate any
members into SCLM control because there are no
members that match the input parameters supplied
for PROJECT, GROUP, TYPE, and MEMBER. SCLM could
not find anything to migrate.
User response
Verify that the PROJECT, GROUP, TYPE, and MEMBER
parameters specified are correct. Verify that the
information you expected to migrate is in the proper
data set according to the parameters specified.
FLM32303 NO MEMBERS MATCHING
SELECTION CRITERIA NEED
MIGRATION
Explanation
The migration utility did not attempt to migrate
any members into SCLM control because there are
no members that are not under SCLM control that
match the PROJECT, GROUP, TYPE, and MEMBER
parameters. Members are considered under SCLM
control if SCLM has accurate accounting information
for them.
User response
Verify that the members to be migrated are not already
under SCLM control and that they match the PROJECT,
GROUP, TYPE, and MEMBER parameters.
FLM32304 WARNING, A NEW ACCOUNTING
RECORD WILL BE GENERATED
FOR MEMBER: aaaaaaaa GROUP:
bbbbbbbb TYPE: cccccccc BASED
ON THE ACCOUNTING RECORD
AT GROUP: dddddddd. CHANGES
MAY NEED TO BE MERGED BEFORE
PROMOTING THE MEMBER.
Explanation
Migrate in forced mode will generate a new accounting
record for the member aaaaaaaa at group bbbbbbbb
and type cccccccc. The new accounting record will
be based on this one. If the authorization code was
changed by the migrate, ensure that any changes
were merged with those at group dddddddd before
promoting the member.
User response
Check to see that any changes are merged before
promoting the member.
FLM32310 USER DEFINED DDNAME:
aaaaaaaa FOR MIGRATION
MESSAGES IS NOT ALLOCATED
Explanation
The ddname specified for the migration messages was
not allocated. If the migration function is invoked using
the services, the ddname for the migration messages
is optional. If not specified, the migration report is
defaulted to the terminal. If the ddname is specified it
must be allocated.
User response
Verify that the user-supplied ddname for the migration
messages is allocated. Resubmit the job.
FLM32320 USER DEFINED DDNAME:
aaaaaaaa FOR MIGRATION
LISTING IS NOT ALLOCATED
Explanation
The ddname specified for the migration listing was not
allocated. If the migration function is invoked through
the services, the ddname for the migration listing
is optional. If not specified, the migration listing is
defaulted to the terminal. If a ddname is specified it
must be allocated.
User response
Verify that the user-supplied ddname for migration
listing is allocated. Resubmit the job.
FLM32401 MIGRATION UTILITY COMPLETED
Explanation
The migration utility finished processing.
SCLM messages
780  z/OS: z/OS ISPF Messages and Codes

## Page 801

User response
See the accompanying messages that appear with this
message on your screen for additional information
regarding the status of this report.
FLM32501 INVOKING MIGRATION UTILITY
Explanation
This message is provided for information only.
FLM32502 INVOKING EXPORT UTILITY
Explanation
This message is provided for information only.
FLM32503 INVOKING IMPORT UTILITY
Explanation
This message is provided for information only.
FLM32504 INVOKING DELETE GROUP
UTILITY
Explanation
This message is provided for information only.
FLM32600 DATE PARAMETER IS NOT IN A
VALID FORMAT FOR THE MIGRATE
UTILITY. DATE: aaaaaaaaaa
Explanation
The date must be in the National Language format and
have a 4-character year.
User response
Correct the date parameter on the Migrate service and
run the service again.
FLM32605 TIME PARAMETER IS NOT IN A
VALID FORMAT FOR THE MIGRATE
UTILITY. DATE: aaaaaaaaaa
Explanation
The time must be in the National Language format.
User response
Correct the time parameter on the Migrate service and
run the service again.
FLM32630 USER DEFINED DDNAME: aaa
FOR MIGRATE REPORT NOT
ALLOCATED
Explanation
The report ddname of the Migrate service must be
allocated before executing the Migrate service. The
ddname can be left blank to allocate the ddname to
the default output device (such as the terminal).
User response
Either allocate the data set before invoking the Migrate
service or set the ddname parameter to blank.
FLM32635 DATE REQUIRED IF TIME
REQUESTED
Explanation
The time parameter was requested, but the date
parameter was not.
User response
To migrate with a date and time, add the date
parameter to the service call. To migrate without a
date and time, remove the time parameter from the
service call.
FLM32640 TIME REQUIRED IF DATE
REQUESTED
Explanation
The date parameter was requested, but the time
parameter was not.
User response
To migrate with a date and time, add the time
parameter to the service call. To migrate without a
date and time, remove the date parameter from the
service call.
FLM33000 INVOKING SEARCH UTILITY
Explanation
This message is self explanatory.
User response
This message is for informational purposes.
FLM33001 SEARCH UTILITY INITIATED -
aaaaaaaa ON bbbbbbbbbb
Explanation
The search process has started. aaaaaaaa is the
time of the message. bbbbbbbbbb is the date of the
message.
SCLM messages
Chapter 3. SCLM messages  781

## Page 802

User response
This message is for informational purposes.
FLM33120 DDNAME: aaaaaaaa FOR SEARCH
MESSAGES IS NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
messages, is not allocated.
User response
Verify that aaaaaaaa ddname for the messages is
allocated. Submit the job again.
FLM33130 DDNAME: aaaaaaaa FOR SEARCH
REPORT IS NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
report, is not allocated.
User response
Verify that aaaaaaaa ddname for the messages is
allocated. Submit the job again.
FLM33164 INVALID RECORD FORMAT FOR
SUPERC DATA SET NAME:
aaaaaaaa GROUP: bbbbbbbb
TYPE: cccccccc RECFM: dddddddd
Explanation
DATA SET aaaaaaaa GROUP: bbbbbbbb TYPE:
cccccccc record format dddddddd is not valid. The
record format must have a type of Fixed, Variable, or
Undefined.
User response
Correct the record format.
FLM33165 NO OUTPUT WAS GENERATED BY
ISPF SUPERC UTILITY.
Explanation
The search utility finished processing but no output
was generated.
User response
See the accompanying messages that appear with
this message for additional information regarding the
status of this report.
FLM33166 ERROR RETRIEVING SEARCH
STRINGS
Explanation
The additional search strings were written to a
temporary data set. For some reason, the temporary
data set could not be accessed.
User response
Use the error messages to determine why SCLM was
unable to retrieve the specified search strings.
FLM33167 ERROR RETRIEVING SECTIONS
COUNT
Explanation
The count of valid types which exist for the given
project could not be retrieved.
User response
Determine why SCLM was unable to retrieve the count.
FLM33168 NO TEXT MEMBERS MATCHING
THE PATTERN: aaaaaaaa FOUND
IN GROUP: bbbbbbbb TYPE:
cccccccc
Explanation
There are no text members in the project that match
the input parameters supplied for group bbbbbbbb,
type cccccccc, and member pattern aaaaaaaa.
User response
Verify that the specified group, type, and member
pattern are correct.
FLM33169 ERROR RETRIEVING MEMBER
LIST GROUP: aaaaaaaa TYPE:
bbbbbbbb
Explanation
A severe error occurred while reading the directory
for the SCLM controlled dataset specified by GROUP:
aaaaaaaa TYPE: bbbbbbbb.
User response
Determine why SCLM was unable to retrieve the
member list.
SCLM messages
782  z/OS: z/OS ISPF Messages and Codes

## Page 803

FLM33170 ERROR RETRIEVING HIERARCHY
GROUP: aaaaaaaa TYPE:
bbbbbbbb
Explanation
A severe error occurred during an attempt to allocate
all data sets that compose the hierarchical view for
group aaaaaaaa, type bbbbbbbb.
User response
Contact IBM support.
FLM33171 NO MEMBERS WERE SELECTED
GROUP: aaaaaaaa TYPE:
bbbbbbbb.
Explanation
Group aaaaaaaa, type bbbbbbbb member selection
list was displayed at the SCLM Search Entry panel, but
no members were selected.
User response
This message is for informational purposes only. No
action is required.
FLM33172 NO ACCOUNT RECORDS
MATCHING THE PATTERN:
aaaaaaaa FOUND IN GROUP:
bbbbbbbb TYPE: cccccccc.
Explanation
There are no account members in the project that
match the input parameters supplied for group
bbbbbbbb, type cccccccc, and member pattern
aaaaaaaa.
User response
This message is for informational purposes only. No
action is required.
FLM33173 NON-EDITABLE LIBRARY GROUP:
aaaaaaaa TYPE: bbbbbbbb
Explanation
Group aaaaaaaa type bbbbbbbb will not be searched
if "Editable types only" field was selected at the SCLM
Search Entry panel.
User response
De-select the "Editable types only" if a search of non-
editable types is required.
FLM33174 ONE OR MORE DATASETS
ARE MISSING FROM THE
HIERARCHY GROUP: aaaaaaaa
TYPE: bbbbbbbb. NO SEARCH
WILL BE PERFORMED. DE-SELECT
SEARCH HIERARCHY FIELD AND
TRY AGAIN.
Explanation
SCLM can perform hierarchy search if a
data set is allocated for every possible
PROJECT.aaaaaaaa.bbbbbbbb combination in the
hierarchy.
User response
To search all levels in the hierarchy de-select the
"Search hierarchy" field and place an "*" in the group
field.
FLM33175 ERROR DELETING SCLM SEARCH
TEMPORARY INPUT DATASET
NAME: aaaaaaaa GROUP:
bbbbbbbb TYPE: cccccccc CODE:
rrrr.
Explanation
An error occurred while deleting SUPERC input dataset
aaaaaaa created by member encoding/decoding
routine. rrrr is the return code.
User response
Determine why SCLM was unable to delete the
temporary dataset.
FLM33177 SUPERC ERROR CODE = rr
Explanation
SuperC error - SuperC ended with a return code of rr.
User response
See the generated SUPERC listing for details.
FLM33178 ERROR ALLOCATING AN SCLM
SEARCH TEMPORARY INPUT
DATASET FOR GROUP: aaaaaaaa
TYPE: bbbbbbbb CODE: rr.
Explanation
An error occurred while allocating SUPERC input
dataset for group aaaaaaaa type bbbbbbbb. rr is the
return code.
SCLM messages
Chapter 3. SCLM messages  783

## Page 804

User response
Determine why SCLM had a problem allocating a
temporary data set.
FLM33179 ALL MEMBERS MATCHING THE
PATTERN aaaaaaaa IN GROUP:
bbbbbbbb TYPE: cccccccc FAILED
SUBPROJECT AUTHORISATION -
NO SEARCH WAS PERFORMED
Explanation
This message is self explanatory.
User response
Contact your SCLM administrator
FLM33181 ERROR DELETING SCLM SEARCH
TEMPORARY INPUT DATASET
NAME: aaaaaaaa CODE: rrrr.
Explanation
An error occurred while deleting SUPERC input dataset
aaaaaaa created by member encoding/decoding
routine. rrrr is the return code.
User response
Determine why SCLM was unable to delete the
temporary dataset.
FLM33198 SERIOUS ERROR OCCURRED IN
PROGRAM aaaaaaaa
Explanation
This message is self explanatory.
User response
Contact IBM support.
FLM33199 SEARCH UTILITY RETURN CODE =
rr
Explanation
The Search utility ended with a return code of rr.
User response
This message is for informational purposes only. No
action is required.
FLM33200 SEARCH UTILITY COMPLETED -
aaaaaaaa ON bbbbbbbbbb
Explanation
The search utility finished processing. aaaaaaaa is the
time of the message. bbbbbbbbbb is the date of the
message.
User response
See the accompanying messages that appear with
this message for additional information regarding the
status of this report.
FLM39002 VERSION / AUDIT RECORD
NOT FOUND, CODE: aaa
GROUP: bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd DATE: eeee
TIME: ffff
Explanation
The audit record was either not found at the level
specified on the SCLM Audit and Version Utility –
Entry Panel or the VSAM record could not be retrieved.
Possible return codes are:
8
Versioning record for member dddddddd could not
be found.
12
VSAM record could not be properly decoded.
16
Invalid group specified on the SCLM Audit and
Version Utility – Entry Panel.
20
I/O error reading the version data set.
User response
For return code:
8
Verify that the member exists and is under SCLM
control. If it is, contact the project manager.
16
Specify a valid group on the SCLM Audit and
Version Utility – Entry Panel.
20
Contact the project manager.
Project manager response
For return code:
8
Verify that the user has the proper access authority
for the versioning data set. Also verify that the
proper versioning data set is being accessed by the
project definition in use.
SCLM messages
784  z/OS: z/OS ISPF Messages and Codes

## Page 805

20
Run IDCAMS against the versioning data set to
determine the problem.
FLM39003 VERSION / AUDIT RECORD NOT
FOUND AT SPECIFIED LEVEL,
CODE: aaa GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd DATE: eeee TIME: ffff
Explanation
This is a warning message indicating that the audit
record was not found at the level specified on the
SCLM Audit and Version Utility – Entry Panel but was
found at a higher level. Possible return codes are:
4
Record was found at higher level
FLM39010 ERROR UPDATING DIRECTORY
INFORMATION FOR MEMBER:
aaaaaaaa DATA SET NAME:
bbb(44) CODE: ccc
Explanation
SCLM could not update the data set directory for this
member in the version retrieve to data set.
Possible return codes are:
4
Unable to update the directory information of a
data set allocated with RECFM=U.
8
Unable to get member name from the directory.
12
The file is not closed.
16
The file control block is NIL.
20-36
Indicates that an I/O error occurred.
24
This can also indicate that the target data set could
not be accessed.
User response
Contact the project manager.
Project manager response
Contact IBM support.
FLM39011 MEMBER DOES NOT EXIST IN
VERSION DATA SET, CODE: aaa
GROUP: bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd DATE: eeee
TIME: ffff PDS: gggggggggg
SYSTEM: hhhhhhhh
Explanation
The member dddddddd does not exist in the version
gggggggggg data set. Code aaa will always be 8.
User response
Contact the project manager.
Project manager response
Verify that the Version PDS file has not been restored
independently. The Version VSAM file and the Version
PDS file must stay in sync at all times. If the Version
PDS has not been restored or altered through manual
manipulation, then the file has been corrupted. Data
has been lost.
FLM39012 ERROR RETRIEVING SELECTION
LIST, CODE: aaa GROUP:
bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd DATE: eeee
TIME: ffff
Explanation
An error occurred while attempting to create the
versioning member selection list. Possible return
codes are:
8
Logical error occurred while reading the audit
database.
12
Physical I/O error occurred while reading the audit
database.
User response
Contact the project manager.
Project manager response
Verify the integrity of the audit VSAM database. If
problems with the database are discovered, reallocate
the database using IDCAMS.
FLM39114 RECORD LENGTH OF THE
VERSION RETRIEVAL DATA SET:
aaa(44) INCOMPATIBLE WITH
RECORD LENGTH OF SOURCE
DATA SET: bbb(44)
SCLM messages
Chapter 3. SCLM messages  785

## Page 806

Explanation
The retrieve to data set name specified on the SCLM
AUDIT AND VERSION UTILITY - ENTRY PANEL has a
logical record length that is smaller than the logical
record length of the original source data set of the
member that was versioned. The retrieval is not
allowed.
User response
Contact the project manager.
Project manager response
Change the name of the retrieve to sequential data set
to one that has an LRECL greater than or equal to the
LRECL of the source data set, or allocate a data set
that has an LRECL greater than or equal to the LRECL
of the source data set.
FLM39200 ERROR RETRIEVING STATISTICS
FOR DATA SET: aaa(44) CODE: bbb
Explanation
Error retrieving the attributes for the specified data
set, aaa(44). Possible values for CODE bbb, are:
4
Error freeing data set
8
Error freeing DCB attributes list or getting directory
block information
12
Could not obtain DCB attributes
16
Data set not found
20
Severe internal error.
User response
Verify that data set exists and is cataloged.
FLM39203 ERROR RETRIEVING SYSTEM
NAME FOR DDNAME: aaaaaaaa,
CODE: bbb
Explanation
Cannot find ddname aaaaaaaa in system catalog.
Possible values for CODE, bbb, are:
4
Ddname incorrect or not allocated for first
temporary version file
8
Ddname incorrect or not allocated for second
temporary version file
16
Ddname incorrect or not allocated for temporary
new version file
20
Ddname incorrect or not allocated for temporary
full source file
24
Ddname incorrect or not allocated for temporary
delta file
1xx
An LM service failed (called by FLMVSDSS).
Contact IBM support.
User response
Attempt the operation again. If the error persists,
contact your project manager.
Project manager response
Verify that the user is allowed to allocate temporary
data sets. This error could also be the result of DASD
problems on the system.
FLM39218 MATCHING DATE NOT FOUND IN
VERSION PDS
Explanation
A member with date/time specified on the version
selection panel could not be found in the version data
set. The version PDS is corrupted.
User response
Contact the project manager.
Project manager response
Ensure that the version data set is protected by some
data set security product. Verify that no one has edited
the version data set. Contact IBM support.
FLM39219 1ST RECORD IN VERSION PDS
IS NOT A HEADER/SEPARATOR
RECORD
Explanation
The first record of the version data is not a header
record.
SCLM messages
786  z/OS: z/OS ISPF Messages and Codes

## Page 807

User response
Contact your project manager.
Project manager response
The version data has been altered. Ensure that other
version data sets are protected by some data set
security product.
FLM39220 ERROR in SUPERCU. CODE: aaa
Explanation
An error was received from SUPERCU. Possible values
for CODE, aaa, are:
16
Error on temporary input data set
24
Error on temporary output data set
26
SVC 99 error
28
Output data set (ABEND E37)
33
Version data has been corrupted
34
Error hashing output file. It is likely that the version
data has been tampered with. The integrity of the
VERPDS data set is questionable.
48
Insufficient storage to perform compare.
49
SEQNUM=STD|COBOL is specified for the project
definition but the record is less than 8 (STD) or 6
(COBOL) characters in length. This means that the
member cannot possibly have NUMBERS ON.
User response
In the case of return code 49, either remove
SEQNUM=STD|COBOL or turn NUMBER ON when
editing the member.
Otherwise, contact the project manager.
Project manager response
In the case of return code 34, you may add the
CHECKSUM=NO parameter to the FLMATVER macro
definition for the affected version group and type,
reassemble the project definition, and retry the
retrieval of the version, thus overriding the checksum
verification failure. Note that the validity of the
retrieved version is not assured. This procedure is
recommended for emergency use only.
If the CODE does not help, contact IBM support.
FLM39222 INVALID LRECL, DSORG, OR
RECFM FOR VERSION PDS
Explanation
The version data set has been incorrectly allocated.
User response
Contact the project manager.
Project manager response
Check that the version PDS exists. Make sure that
the version PDS is allocated properly. For more
information about versioning partitioned data sets, see
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference.
FLM39225 ERROR ALLOCATING AN OLD
VERSION DATA SET CODE: aaa
Explanation
The member that contains the version does not exist
or is corrupt in the version PDS. This can occur only if
the version PDS has been corrupted manually. Possible
values for CODE, aaa, are:
24
Data set organization is not PO
28
Member not found.
32
Versioning data set could not be opened.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible reason code values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error
User response
This message indicates that an error occurred while
allocating a data set to be used by a translator.
SCLM messages
Chapter 3. SCLM messages  787

## Page 808

If you receive a return code of 32 or X'xxxx, try the
operation again. If the problem recurs or you receive
any other return codes, call the project manager.
Project manager response
Verify that the Version VSAM file or the version PDS
has not been restored independently. They MUST stay
in sync at all times. If one or the other has not been
restored, then the file has been corrupted through
manual manipulation. Data has been lost.
If the return code is:
X'xxxx'
An error was received from SVC 99 during the
allocation. The SVC 99 reason code is specified
in the message (xxxx). Refer to the z/OS MVS
Programming: Authorized Assembler Services Guide
for a description of the SVC 99 reason codes. For
SMS error codes, the IGD message can be located
in the z/OS MVS System Messages, Vol 8 (IEF-IGD).
The listing may have one or two leading zeros
added to the ddddd value.
FLM39226 ERROR ALLOCATING A
TEMPORARY VERSION DATA SET
CODE: aaa
Explanation
An error occurred while attempting to allocate a
temporary data set.
Possible return codes are:
4
Data set name omitted from input.
12
Parameter not valid.
16
Volume not available, or data set name or member
missing or not valid.
20
Invalid file attribute specified. This can be
something such as : expected numeric data not
numeric, space not designated as tracks, kilobytes,
blocks, etc, dsntype not specified as library, record
format not valid.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested data set could not be opened.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support..
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible reason code values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error
User response
Submit the job again. If the error recurs, contact the
project manager.
Project manager response
Verify that all data set names are correctly defined
and exist. Verify that the FLMALLOC for the temporary
data set is specified correctly. For return codes in
the form X'xxxx', refer to the z/OS MVS Programming:
Authorized Assembler Services Guide for a description
of the SVC 99 reason codes (or see Appendix A of the
ISPF Tutorial). For SMS error codes, the IGD message
can be located in the z/OS MVS System Messages,
Vol 8 (IEF-IGD). The listing may have one or two
leading zeros added to the ddddd value. For additional
assistance, contact IBM support.
FLM39228 ERROR ALLOCATING USER
RETRIEVAL DATA SET CODE: aaa
Explanation
The data set that is to contain the retrieved version
cannot be allocated. Possible values for CODE, aaa,
are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name, or not
authorized to update.
20
Invalid file attribute specified.
SCLM messages
788  z/OS: z/OS ISPF Messages and Codes

## Page 809

24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error
User response
Verify that the name is specified correctly on the
SCLM AUDIT AND VERSION UTILITY - ENTRY PANEL
under the heading SCLM retrieve group and type
or To other non-SCLM-controlled retrieve data set.
If it is specified correctly verify that the data set
is allocated and cataloged. For return codes in the
form X'xxxx', refer to the z/OS MVS Programming:
Authorized Assembler Services Guide for a description
of the SVC 99 reason codes (or see Appendix A of the
ISPF Tutorial). For SMS error codes, the IGD message
can be located in the z/OS MVS System Messages,
Vol 8 (IEF-IGD). The listing may have one or two
leading zeros added to the ddddd value. For additional
assistance, contact IBM support.
FLM39229 ERROR FREEING USER RETRIEVAL
DATA SET CODE: aaa
Explanation
This indicates an operating system problem. While the
data set was in use it became corrupted such that SVC
99 could not free it. While under exclusive control of
SCLM Versioning, some other program gained access
and renamed, erased, or otherwise defiled the retrieve
to data set. Possible values for CODE, aaa, are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
'X'xxxx
SVC 99 error reason code (in hexadecimal). If this
is an SMS error code (X'97xx'), this will be followed
by SMS: ddddd; where ddddd is the IGD message
number associated with the error.
User response
Contact the project manager.
Project manager response
For return codes of the form X'xxxx', refer to the
z/OS MVS Programming: Authorized Assembler Services
Guide for a description of the SVC 99 reason codes (or
see Appendix A of the ISPF Tutorial). For SMS error
codes, the IGD message can be located in the z/OS
MVS System Messages, Vol 8 (IEF-IGD). The listing may
have one or two leading zeros added to the ddddd
value. Call Operating system support.
FLM39231 RECORD LENGTH: aaaa FOUND IN
VERSION DATA SET
MEMBER: bbbbbbbb IS
INCOMPATIBLE WITH
RECORD LENGTH: cccc OF SOURCE
MEMBER
SOURCE MEMBER GROUP:
dddddddd SOURCE MEMBER
TYPE: eeeeeeee
VERSION DATA SET: ffffffff
**
Explanation
An error occurred while copying SCLM member
bbbbbbbb from the source data set to the versioning
data set. One of the records of this member in the
version data set fffffff had length aaaa, and this was
larger than record length cccc of the source data
set being added to the version. The source member
belongs to group ddddddd and was of type eeeeeeee.
The member in the version data set does contain the
new version, but the version might be damaged.
User response
Contact the project manager.
Project manager response
The VERPDS operand on either the FLMCNTRL or
FLMALTC macro resolves to the same versioning
partitioned data set for one of these cases:
SCLM messages
Chapter 3. SCLM messages  789

## Page 810

1. An FLMATVER macro with VERSION=YES and
TYPE=*
2. Multiple FLMATVER macros with VERSION=YES
and different types specified
If you specify a value of 2 or more for the VERCOUNT
parameter on the FLMCNTRL macro, you must specify
a separate VERPDS for each combination of group
and type that you intend to version. See FLMATVER
macro in z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference for more information.
Resolve by using the @@FLMGRP and @@FLMTYP
variables in the name of the VERPDS version
partitioned data set. Alternatively, use the FLMALTC
macro to specify a unique versioning partitioned data
set for each combination of group and type that you
intend to version. Then allocate all possible versioning
partitioned data sets based on groups and types being
versioned.
FLM39240 ERROR CREATING TEMPORARY
DATA SET TO DECODE THE
VERSION MEMBER. MEMBER :
nnnnnnn RETURN CODE : rr
Explanation
SCLM encountered an error attempting to create a
temporary data set to decode the member nnnnnnn
into.
User response
Determine why SCLM was unable to allocate the
temporary data set.
FLM39241 ERROR DECODING THE VERSION
MEMBER nnnnnnn, RETURN
CODE= rr. ERROR MSG:
xxxxxxxxxxxxxxx
Explanation
SCLM encountered a problem attempting to decode
the member nnnnnnn.
User response
Use the error message to determine the why SCLM
was unable to decode the member nnnnnnnn.
FLM39242 ERROR CREATING TEMPORARY
DATA SET TO DECODE THE
SOURCE. MEMBER : nnnnnnn
RETURN CODE : rr
Explanation
SCLM encountered an error attempting to create a
temporary data set to decode the member nnnnnnn
into.
User response
Determine why SCLM was unable to allocate the
temporary data set.
FLM39243 ERROR DECODING THE
SOURCE MEMBER nnnnnnn,
Return Code=rr. ERROR MSG:
xxxxxxxxxxxxxxx
Explanation
SCLM encountered a problem attempting to decode
the member nnnnnnn.
User response
Use the error message to determine the why SCLM
was unable to decode the member nnnnnnnn.
FLM39244 ERROR ENCODING THE
VERSION MEMBER nnnnnnn,
Return Code=rr. ERROR MSG:
xxxxxxxxxxxxxxx
Explanation
SCLM encountered a problem attempting to encode
the member nnnnnnn.
User response
Use the error message to determine the why SCLM
was unable to encode the member nnnnnnnn.
FLM40501 NO TRANSLATOR INVOKED FOR
LANGUAGE: aaaaaaaa
Explanation
No translator was invoked for language aaaaaaaa.
User response
If a translation was expected, contact the project
manager.
Project manager response
Examine the language definition to verify that a
translation was expected for language aaaaaaaa.
If the language contains translators, verify that at
SCLM messages
790  z/OS: z/OS ISPF Messages and Codes

## Page 811

least one translator specifies the FUNCTN=BUILD
parameter on the FLMTRNSL macro.
FLM40507 ERROR ALLOCATING DATA SET:
aaa(44) FOR TRANSLATOR:
bbbbbbbb DATA SET NUMBER: ccc
CODE: ddd
Explanation
An error occurred while attempting to allocate data
set aaa(44) for translator bbbbbbbb. The data set is
being allocated for the data set number ccc. The data
set number refers to the position of the FLMALLOC
statement being used to allocate this data set. For
example, if ccc=5, the error message is associated
with the fifth FLMALLOC statement for this translator.
Possible problem:
• More than one IOTYPE=I might have been specified
in the FLMALLOC list for a translator.
Possible return codes are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible reason code values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
This message indicates that an error occurred while
allocating a data set to be used by a translator.
If you receive a return code of 32 or X'xxxx', try the
operation again. If the problem recurs or you receive
any other return codes, call the project manager.
Project manager response
If the return code is:
12
SCLM internal error. Contact IBM support.
Note: For the next set of codes, you should know that
the data set number corresponds to an FLMALLOC
macro associated with the specified translator (the
first FLMALLOC is data set 1, the second is data set
2 and so on).
16
Check FLMALLOC to ensure that the data set name
has been specified correctly.
20
Check FLMALLOC to ensure that all data set
attributes have been specified correctly.
24
Check FLMALLOC to ensure that a sequential data
set was not specified when a partitioned data set
was expected.
28
Check FLMALLOC to ensure that the correct
member name was specified.
32
Retry the operation. If the same problem occurs,
contact IBM support.
X'xxxx'
An error was received from SVC 99 during the
allocation. The SVC 99 reason code is specified
in the message (xxxx). Refer to the z/OS MVS
Programming: Authorized Assembler Services Guide
for a description of the SVC 99 reason codes (or
see Appendix A of the ISPF tutorial). For SMS error
codes, the IGD message can be located in z/OS
MVS System Messages, Vol 8 (IEF-IGD). The listing
may have one or two leading zeros added to the
ddddd value.
FLM40510 ERROR ALLOCATING DATA SET:
aaa(44) FOR TRANSLATOR:
bbb(16) CODE: ccc
Explanation
An error occurred while attempting to allocate data set
aaa(44) for translator bbb(16). This data set should
contain the translator to be invoked.
SCLM messages
Chapter 3. SCLM messages  791

## Page 812

Possible return codes are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
Contact the project manager.
Project manager response
Verify that data set aaa(44) exists and is cataloged.
For return codes of the form X'xxxx', refer to the
z/OS MVS Programming: Authorized Assembler Services
Guide for a description of the SVC 99 reason codes (or
Appendix A of the ISPF Tutorial). For SMS error codes,
the IGD message can be located in z/OS MVS System
Messages, Vol 8 (IEF-IGD). The listing may have one or
two leading zeros added to the ddddd value.
FLM40516 MEMBER: aaaaaaaa TYPE:
bbbbbbbb WAS UPDATED DURING
THE BUILD
Explanation
The SCLM editor updated member aaaaaaaa during
build processing. SCLM does not save translator
output because it might have been created from the
previous version of the member.
User response
Submit the job again.
FLM40517 DUPLICATE KEYREF=aaaaaaaa
NOT ALLOWED FOR TRANSLATOR
OUTPUTS
Explanation
The translator invoked has two temporary output
data sets, allocated with either IOTYPE=O or P, both
targeted to the same output member (with the KEYREF
parameter). The build processor cannot copy multiple
output data sets produced by the translator to a single
targeted member.
User response
See the project manager.
Project manager response
Verify in the language definition that no two FLMALLOC
macro calls with either IOTYPE=O or P have the same
KEYREF value for a single translator. Also verify in the
language definition that two FLMTRNSL macros with
the FLMTCOND macro and the same KEYREF values
are not both executing in this particular situation.
FLM40519 NUMBER OF ALLOCATED DATA
SETS FOR HIERARCHY SEARCH AS
SPECIFIED IN DATA SET ccc FOR
TRANSLATOR ddd(16) HAS BEEN
EXCEEDED (aaa DATA SETS WERE
ALLOCATED, MAXIMUM ALLOWED
IS bbb)
Explanation
The number of data sets allocated for the translator
hierarchy search has exceeded the maximum value for
the system. This message is preceded by the call name
of the translator in question. The ddname allocated for
hierarchy search is specified by the FLMALLOC macro
with IOTYPE=I.
User response
For the translator in question, verify that all FLMALLOC
macros with IOTYPE=I do not exceed the system limit
SCLM messages
792  z/OS: z/OS ISPF Messages and Codes

## Page 813

for allocating data sets to a ddname. This error can be
caused by:
• Defining too many groups for the project
• Using the extended type option (Extend field on the
FLMTYPE macro)
• Specifying too many FLMCPYLBs for the ddname.
FLM41002 ERROR OCCURRED DURING
INITIALIZATION
Explanation
An error occurred during the initialization phase of the
build processor.
User response
See the message data set for all the messages related
to this error.
FLM42000 BUILD PROCESSOR INITIATED -
aaaaaaaa ON bbbbbbbb
Explanation
The build process has started. This message is
provided for information only.
FLM42004 INVALID INPUT PARAMETER
GROUPaaaaaaaa
TYPE bbbbbbbb
MEMBER cccccccc
USER ID dddddddd
BUILD MODE e
BUILD SCOPE f
ERROR LISTINGS ONLY g
REPORT REQUEST h
PREFIX USER ID iii(17)
Explanation
You specified an invalid input parameter to the build
processor. The values of the parameters are listed.
Only the first character is listed for build mode and
build scope.
Valid values for build mode are CONDITIONAL,
UNCONDITIONAL, REPORT, and FORCED. Valid values
for build scope are LIMITED, NORMAL, SUBUNIT, and
EXTENDED. Valid values for report request are Y and
N.
If the build processor was invoked through the SCLM
dialog panel, SCLM retrieves the user ID and prefix
user ID input parameters from the ISPF shared and
profile pools, respectively.
User response
Verify that all input parameters are specified correctly
and submit the job again. If the problem recurs,
contact IBM support.
FLM42100 USER DEFINED DDNAME:
aaaaaaaa FOR BUILD MESSAGE
NOT ALLOCATED
Explanation
The ddname aaaaaaaa, which was specified for
the build messages, was not allocated. If the build
function is called through the SCLM services, the
ddname for the build messages is optional. If not
specified, the build messages are defaulted to the
terminal. If a ddname is specified, it must be allocated.
User response
Verify that the user-supplied ddname for build
messages is allocated. Submit the job again.
FLM42104 USER DEFINED DDNAME:
aaaaaaaa FOR BUILD REPORT
NOT ALLOCATED
Explanation
The ddname aaaaaaaa, which was specified for the
build report, was not allocated. If the build function
is invoked through the SCLM services, the ddname
for the build report is optional. If not specified, the
build report is defaulted to the terminal. If a ddname is
specified, it must be allocated.
User response
Verify that the user-supplied ddname for build report
is allocated. Submit the job again.
FLM42106 USER DEFINED DDNAME:
aaaaaaaa FOR BUILD LISTING
NOT ALLOCATED
Explanation
The ddname aaaaaaaa, which was specified for the
build listing, was not allocated. If the build function
is invoked through the SCLM services, the ddname for
the build listing is optional. If not specified, the build
listing is defaulted to the terminal. If the ddname is
specified, it must be allocated.
User response
Verify that the user-supplied ddname for build listing is
allocated. Submit the job again.
SCLM messages
Chapter 3. SCLM messages  793

## Page 814

FLM42108 USER DEFINED DDNAME:
aaaaaaaa FOR USER EXIT DATA
SET NOT ALLOCATED
Explanation
The ddname aaaaaaaa, which was specified for the
user exit data set, was not allocated. If the build
function is invoked through the SCLM services, the
ddname for the user exit data set must be specified
if a user exit routine has been specified. Otherwise the
ddname is optional. If not specified, a user exit data
set is allocated to NULLFILE. If a ddname is specified,
it must be allocated.
User response
Verify that the user-supplied ddname for user exit file
is allocated. Submit the job again.
FLM42109 NO DDNAME SPECIFIED FOR
USER EXIT FILE
Explanation
A ddname was not specified to the build function for
the user exit file but the project definition specifies a
build user exit. (See the BLDEXT1 parameter on the
FLMCNTRL macro).
User response
Include the ddname to be used for the user exit file on
the FLMCMD or FLMLNK call to invoke build.
Project manager response
Remove the build user exit from the project definition
if it is not needed.
FLM43001 ERROR RETRIEVING
ACCOUNTING INFORMATION
CODE: aaa GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
(REFERENCED BY
MEMBER: eeeeeeee TYPE: ffffffff)
Explanation
No accounting information exists or could be retrieved
for member dddddddd. Member eeeeeeee is either an
architecture definition or a compilable source member.
Member dddddddd could have been referenced
directly in an architecture definition statement or via
a parsed dependency such as an include, compool, or
compilation unit.
Member dddddddd cannot be referenced directly by
member eeeeeeee. For example, member dddddddd
might be included by a member that is referenced with
a SINC statement in member eeeeeeee.
Possible return codes are:
8
The accounting information for the specified
member could not be found.
12
The accounting information for the specified
member is out of date.
16
The group is not defined in the project definition
being used.
20
A severe VSAM I/O error occurred.
24
An internal SCLM error occurred.
28
The type is not defined by the project definition
being used.
Possible problems:
• Member dddddddd has not been registered with
SCLM. Use the SCLM Library Utility to verify that an
accounting record exists for the member.
• Member dddddddd was excluded at the build group
because of change code processing and the member
could not be found at a higher group.
• Member was moved to another type after a previous
successful build.
• If member dddddddd is a build output, the scope of
the current build does not encompass the creation
of the output. It should be added to the architecture
definition you are building or it should be created
with another build before attempting the current
build again. This type of problem is likely to occur
when the LINK architecture definition statement is
used or when include and compool dependencies
are used to reference build outputs.
• The accounting information for member eeeeeeee
might not be accurate. This problem can be caused
by making changes to SCLM data sets outside the
SCLM environment. Alternatively, FLMSYSLB macros
could have been added or removed from the project
definition since the source members being built were
last parsed or migrated.
• A dynamic include (DYNI* in the build map) detected
by a previous build of some member in the scope
of this build has since been deleted from the
referenced library.
SCLM messages
794  z/OS: z/OS ISPF Messages and Codes

## Page 815

User response
• If the accounting information is not accurate for a
single member, use the SCLM Editor or Migrate Utility
to correct the problem. If FLMSYSLBs have been
changed, draw down and migrate all of the members
that are affected.
• If member dddddddd is a build output, build the
member that creates it before submitting this job
again.
• If the member is referenced by a dynamic include
that is being intentionally deleted, ensure that all
updates to other source members have been made
to remove that reference. Use Edit to create the
referenced member (content is unimportant, as long
as it can be successfully parsed). Rerun the build
and, on successful completion, examine the build
map(s) to ensure that the dynamic include no longer
exists. The member created for this purpose can
now be deleted.
If the return code is
8
Register both the specified member and the
referencing source member with SCLM using the
&sclm editor, SAVE service, or migration utility.
Submit the job again.
If member dddddddd was excluded based on
change code, update the change codes specified
so that the required member will be included in the
build.
12
Register the specified member with SCLM using
the SCLM editor, SAVE service, or migration utility.
Submit the job again.
Project manager response
If the return code is:
16
Verify that the specified group is defined in the
project definition. If necessary, add the group to
the project definition and reassemble it. Submit
the job again.
20
A VSAM error occurred. Run IDCAMS against the
accounting data set to determine the problem.
24
Contact IBM support.
28
Verify that the type specified is defined in the
project definition. If necessary, add the type to the
project definition and reassemble it. Submit the
job again.
FLM43002 Error retrieving member:
aaaaaaaa Type: bbbbbbbb
Explanation
Member aaaaaaaa in type bbbbbbbb could not be
allocated.
User response
Verify that the member exists and that the data set is
not allocated exclusively to another job.
FLM43003 Error processing Member:
aaaaaaaa Type: bbbbbbbb
Explanation
An error was encountered when processing member
aaaaaaaa in type bbbbbbbb. The member contains
an invalid statement. Message FLM44201 is issued to
indicate the invalid statement.
User response
Refer to the actions associated with message
FLM44201.
FLM43004 Copy loop detected member:
aaaaaaaa Type: bbbbbbbb
Explanation
The architecture member aaaaaaaa in type bbbbbbbb
is involved in a recursive copy loop. Message
FLM43005 will be issued for all members involved in
the loop.
User response
Remove one or more of the COPY statements in the
listed architecture members to eliminate the recursive
copy loop.
FLM43005 Referenced by member: aaaaaaaa
Type: bbbbbbbb
Explanation
The member aaaaaaaa in type bbbbbbbb referenced
another architecture member that was in error.
User response
Refer to the other messages to determine the cause of
the error.
FLM43007 LANGUAGE SCOPE: a FOR
MEMBER: bbbbbbbb TYPE:
SCLM messages
Chapter 3. SCLM messages  795

## Page 816

cccccccc CONFLICTS WITH BUILD
SCOPE SPECIFIED
Explanation
The scope a specified in project definition for member
bbbbbbbb is of greater range than the scope specified
on the Build panel. The first letter of the scope defined
in the project definition is listed.
User response
You can specify these four scopes (in ascending
order): LIMITED, NORMAL, SUBUNIT, and EXTENDED.
Verify that the range specified as input to the build
processor is of equal or greater range than the scope
specified in the project definition for the language of
the source member being built.
FLM43008 ERROR PROCESSING
DEPENDENCIES FOR MEMBER:
aaaaaaaa TYPE: bbbbbbbb
Explanation
Errors occurred while processing the dependencies for
the specified member. Other messages preceding this
one in the message data set provide more detail on the
exact errors that occurred.
User response
See the message data set for all the messages related
to this error.
FLM43109 NO ACCOUNTING INFORMATION
EXISTS FOR COMPILATION UNIT
(CU): CU NAME: aaa.(55) bbb.(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
(ACCOUNTING INFORMATION
EXISTS FOR INTERMEDIATE
FORM)
Explanation
Accounting information does not exist for the
compilation unit; however, accounting information
does exist for the associated intermediate form.
This error can be caused when you delete a source
member (using the library utility) but forget to delete
the intermediate forms produced by the compiler
for those compilation units contained in the deleted
source member.
User response
Delete the intermediate form from all groups in the
hierarchy used in the build, then resubmit the build.
FLM43111 SPECIFICATION MISSING FOR
COMPILATION UNIT: CU NAME:
aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
Explanation
The specified compilation unit has a dependency
on an implicit specification. Implicit specifications
are not allowed. For more information on implicit
specifications, see the description of the IMPSPEC
parameter on the FLMLANGL macro.
User response
Create a specification for the compilation unit.
Project manager response
Depending on the compiler being used, the IMPSPEC
parameter of the FLMLANGL macro for the language
might need to be set differently. If IMPSPEC
is changed, correct and reassemble the project
definition. Submit the job again.
FLM43119 VERIFICATION ERROR OCCURRED
FOR COMPILATION UNIT CU
NAME: aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
Explanation
Accounting information for compilation unit aaa(55)
bbb(55) does not match accounting information for the
member that contains the source for the compilation
unit. The member that contains the source for the
compilation unit is indicated in a succeeding message
in the message data set.
User response
Register the member with SCLM by using the SCLM
editor, the SAVE service, or the migration utility.
Submit the job again.
FLM43120 ERROR PROCESSING
DEPENDENCIES FOR
COMPILATION UNIT: CU NAME:
aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
SCLM messages
796  z/OS: z/OS ISPF Messages and Codes

## Page 817

Explanation
Errors occurred while processing the dependencies for
compilation unit aaa(55) bbb(55).
User response
See the message data set for messages related to this
error.
FLM44005 ERROR - A CIRCULAR
DEPENDENCY EXISTS IN THE
CURRENT BUILD TRACE BACK
OF DEPENDENCIES: MEMBER
aaaaaaaa TYPE bbbbbbbb
Explanation
A circular set of dependencies exists in the scope of
the current build. SCLM cannot complete the build
because of the circular dependencies. The message
contains a list of members and types containing the
circular dependency.
User response
Examine the list of specified members and remove the
circular dependency.
FLM44009 ERROR CHECKING FOR CIRCULAR
DEPENDENCIES
Explanation
This is an SCLM internal error checking for circular
dependencies.
User response
Notify project manager.
Project manager response
Contact IBM support.
FLM44031 WARNING, INTERMEDIATES NOT
GENERATED FOR COMPILATION
UNITS REFERENCED BY MEMBER:
aaaaaaaa TYPE: bbbbbbbb
LANGUAGE: cccccccc
Explanation
The member being built contains the source for
compilation units but no build translator is defined
for the language being used to build the member.
No intermediate members will be generated for the
compilation units contained in the source. A build map
will be created for the member and the build will
continue, but no outputs are generated.
The member name will be an architecture definition
if the source members are being included by SINC
statements; otherwise, the member name will be
the source member. If the member name is an
architecture definition then the language is the
language of the source member that is being used for
the build. If the member name is a source member, the
language is the language of that source member.
User response
Verify whether or not intermediates should be
generated for this build. If intermediates are to be
generated either change the language of the source
members to a language with a build translator or
inform the project manager.
Project manager response
If intermediates are to be produced for the language
then add a build translator to the language definition
for the project.
FLM44032 WARNING, “COMP” KEYWORD
NOT SPECIFIED FOR MEMBER:
aaaaaaaa TYPE: bbbbbbbb
Explanation
The message is a warning indicating that you have
not specified a COMP keyword for a JOVIAL compool.
This missing keyword (COMP) will result in the data
dictionary not being updated for the compool.
User response
Verify that the translator to be invoked for the
member contains an FLMALLOC macro with IOTYPE=O
and KEYREF=COMP. If the specified member is an
architecture member, add a COMP keyword.
FLM44035 FLMALLOC MACRO WITH
KEYREF=aaaaaaaa DOES NOT
EXIST FOR LANGUAGE bbbbbbbb
Explanation
The language definition for language bbbbbbbb
does not contain an FLMALLOC macro with KEYREF
aaaaaaaa. An architecture member contains the
keyword aaaaaaaa and controls invocation of the
translators for the language bbbbbbbb.
SCLM messages
Chapter 3. SCLM messages  797

## Page 818

User response
Verify that an FLMALLOC macro with a
KEYREF=aaaaaaaa parameter exists for the language;
otherwise, remove the keyword from the architecture
member.
FLM44036 THE aaaaaaaa KEYWORD WAS
SPECIFIED FOR MEMBER:
bbbbbbbb TYPE: cccccccc BUT
THERE WAS NO FLMALLOC MACRO
WITH A MATCHING KEYREF FOR
LANGUAGE dddddddd.
Explanation
This is an informational message. This message is
issued if there is a CREF or SREF in the architecture
definition or a DFLTCRF or DFLTSRF on the FLMLANGL
but no FLMALLOC referencing the CREF or SREF.
FLM44039 MULTIPLE "SINC" KEYWORDS
MUST REFERENCE THE SAME
TYPE SINCE COMPILATION UNIT
DEPENDENCIES ARE PRESENT
Explanation
Multiple SINC statements with different types were
specified in the architecture member in which
compilation dependencies existed for the members
specified on the SINC statement. SCLM requires that
all source members referenced with the SINC keyword
reside in the same type if any of the members contain
compilation units. The message that appears after
this message identifies the architecture member in
question.
User response
If you specify multiple source inputs with the SINC
keyword, verify that they reside in the same type.
FLM44050 ERROR, TYPE: aaaaaaaa
MEMBER: bbbbbbbb EXCEEDS
MAXIMUM INPUT LINES VALUE:
ccc, CURRENT STATEMENT
COUNT: ddd
Explanation
This message identifies that a single member exceeds
the SLOCLMT value specified in the FLMLANGL macro.
This member cannot be placed on any Input List for
this language.
User response
Modify the contents of the specified member to place
it within the limits of the SLOCLMT or contact the
project manager.
Project manager response
Modify the SLOCLMT value of the FLMLANGL macro in
the project definition.
FLM44101 ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb
NOT FOUND WITHIN SCOPE
OF ARCHITECTURE DEFINITION
BEING BUILT
Explanation
Member aaaaaaaa is being referenced during the
build; however, it was not predefined by build to be
within the scope of processing. This error can occur
if, for example, during the building of a system, a
subsystem of that system is rebuilt by another build or
promoted into the hierarchy (perhaps by another user).
The rebuilding of the subsystem can increase the
scope of the build for the system. The building of the
system might have proceeded too far to identify any
more members within the scope.
User response
Verify that no other builds or promotes are occurring
within your hierarchy and submit the job again. as per
RF comment 64505.
FLM44201 INVALID ARCHITECTURE
STATEMENT: aaa(80)
Explanation
The statement aaa(80) is not a valid architecture
statement. Refer to message FLM43003 to determine
the name of the architecture member containing the
invalid statement.
Either an undefined keyword was found in the
specified architecture member or there is a field
missing on an architecture definition statement in the
specified architecture member. For example:
  SINC MEMBER
is incorrect because the TYPE field is missing from the
statement.
  SINCX MEMBER TYPE
is incorrect because SINCX is not a valid architecture
definition statement.
SCLM messages
798  z/OS: z/OS ISPF Messages and Codes

## Page 819

SINC VERYLONGNAME SOURCE
is invalid because the member name is longer than 8
characters.
Correct the contents of the architecture member, and
submit the job again.
User response
Correct the invalid architecture statement.
FLM44202 ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb ALL
CCODE STATEMENTS MUST
CONTAIN THE SAME INCLUDE
FLAG VALUE
Explanation
Member aaaaaaaa in type bbbbbbbb contains CCODE
statements with conflicting include flag values. All
of the change codes specified within an architecture
definition must have the same include flag value.
User response
Update the CCODE statements in the architecture
definition so that all of the change codes are either
included (INCLUDE or default include flag) or excluded
(EXCLUDE include flag).
FLM44203 MEMBER: aaaaaaaa TYPE:
bbbbbbbb IS INCORRECTLY
REFERENCED BY MEMBER:
cccccccc TYPE: dddddddd
Explanation
An incorrect dependency exists when this reference
occurs. This error occurs when an LEC architecture
member references a member that does not produce
an output that matches the KREF keyword value. If no
KREF was coded, the default KREF keyword values are
OBJ and LOAD. Processing of member cccccccc cannot
continue.
User response
Verify that member aaaaaaaa in type bbbbbbbb
produces an output that matches the coded KREF
keyword, or the default KREF keywords object or
load. If the default KREF value is used, an object
module must be identified by the OBJ keyword. A load
module must be identified by the LOAD keyword. Other
outputs must be identified by the output keyword
coded for the KREF keyword.
FLM44204 MEMBER: aaaaaaaa TYPE:
bbbbbbbb CONTAINS MULTIPLE
cccccccc ARCHITECTURE
DEFINITION KEYWORDS
Explanation
Multiple architecture definition keywords have been
used in the architecture definition referenced. These
keywords cannot be used more than once in an
architecture definition: OUT1, OUT2, OUT3, OUT4,
OUT5, OUT6, OUT7, OUT8, OUT9, LOAD, LMAP, LIST,
OBJ, COMPOOL, CREF, SREF, and LKED.
User response
Examine the member aaaaaaaa in type bbbbbbbb
and remove the extraneous architecture definition
keywords referenced.
FLM44205 ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb
REFERENCES MEMBER: cccccccc
TYPE: dddddddd WITH KEYWORD:
eeeeeeee, USE: ffffffff INSTEAD
Explanation
Member cccccccc is referenced with the wrong
architecture definition keyword. The architecture
definition keyword INCL can only be used to reference
a member that contains an architecture definition.
The language of that architecture definition must be
defined with the ARCH=Y parameter on the FLMLANGL
macro. The referenced architecture definition must
contain valid architecture definition keywords.
The architecture definition keyword INCLD can only
be used to reference source members that can be
built by SCLM. The language of the member referenced
with architecture definition keyword INCLD must be
defined with the ARCH=N parameter on the FLMLANGL
macro.
User response
Verify that the correct language is assigned to
member cccccccc. If the language is correct, change
member aaaaaaaa to reference member cccccccc
with keyword ffffffff. If changing the keyword will
not produce the desired result, contact the project
manager.
Project manager response
Verify that the ARCH parm on the language definition
for the language assigned to member cccccccc
is correct. Refer to the description of the ARCH
parameter on the FLMLANGL macro in z/OS ISPF
SCLM messages
Chapter 3. SCLM messages  799

## Page 820

Software Config ur ation  and Library Manager Guide and
Reference for more information.
FLM44206 ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb MUST
NOT CONTAIN BOTH SINC AND
LOAD KEYWORDS
Explanation
Architecture member aaaaaaaa contains both a
SINC and a LOAD keyword. These keywords are
incompatible.
User response
If the purpose of the architecture definition is to
create a load module the SINC keyword should
probably be replaced with an INCLD keyword. If the
member referenced by the SINC keyword is correct,
the LOAD keyword must be changed to a different
output keyword (such as OBJ or OUTx). See z/OS ISPF
Software Config ur ation  and Library Manager Guide
and Reference for more information on architecture
definition keyword usage.
Project manager response
If the compile and linkedit translators are controlled
by the same language, use one of the OUTx
keywords in place of the LOAD keyword. Change the
KEYREF=LOAD parameter on one of the FLMALLOC
macros in the language definition to accomplish this.
For more information on the FLMALLOC macro see
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference.
FLM44207 NO KEYWORD: aaaaaaaa
SPECIFIED FOR ARCHITECTURE
MEMBER: bbbbbbbb TYPE:
cccccccc BUT ALLOCATION FOR
TRANSLATOR: ddd(16) DATASET:
eee SPECIFIES KEYREF=ffffffff
Explanation
The language definition being used for the build
of member bbbbbbbb in type cccccccc contains an
allocation referencing the ffffffff keyword but member
bbbbbbbb does not contain that keyword.
User response
Take one of these actions:
• Add the aaaaaaaa keyword to the bbbbbbbb
member.
• Change the language being used to build the
member to one that does not require the aaaaaaaa
keyword.
• Contact the project manager to remove the
reference to the ffffffff keyword from the language
definition.
Project manager response
Update the FLMALLOC macro referred to in the
message to remove KEYREF=ffffffff or have the user
perform one of the actions described under User
Response.
FLM44208 ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb MUST
NOT CONTAIN BOTH CCODE AND
COPY KEYWORDS
Explanation
Architecture definition aaaaaaaa in type bbbbbbbb
contains both CCODE and COPY statements. An
architecture definition cannot contain both.
User response
Create a new architecture definition to contain the
CCODE statement and an include (INCL) of the
architecture definition to contain the COPY statement.
FLM44231 INVALID REFERENCE TO LOAD
MODULE: aaaaaaaa TYPE:
bbbbbbbb
Explanation
The load module referenced with the LINK keyword is
a member that can be edited.
User response
For the architecture member that contains the error,
verify that the LINK keyword specifies a load module
and not the architecture member that creates the link
load module.
FLM44240 ERROR: TWO BUILD MAPS ARE
PRODUCING TYPE: aaaaaaaa
MEMBER: bbbbbbb BUILD MAP
1 TYPE: cccccccc MEMBER:
dddddddd KEYWORD: eeeee BUILD
MAP 2 TYPE: ffffffff MEMBER:
gggggggg KEYWORD: hhhhh
Explanation
Member bbbbbbbb is referenced as an output of two
different build maps within the scope of the Build or
SCLM messages
800  z/OS: z/OS ISPF Messages and Codes

## Page 821

by two different output keywords from the same build
map. This condition will cause the output of one of
the build maps to be overwritten by the output of
the other build map. If one of the build maps is for
a source member, member bbbbbbbb is defined as a
default output in the language definition for the source
member.
User response
If either dddddddd or gggggggg is an architecture
definition, change the reference to member bbbbbbbb
to resolve the conflict. The reference to member
bbbbbbbb might be in a member that is copied into
the architecture definition.
If both dddddddd and gggggggg are source members,
consider these actions:
1. Verify that the correct language was specified for
both source members.
2. Change the name of one of the source members.
3. Contact your project manager.
Project manager response
Use one of these techniques to resolve conflicting
outputs within language definitions:
1. Remove the DFLTTYP=aaaaaaaa parameter from
an FLMALLOC macro in one of the language
definitions to prevent the output from being
created by Build. If this is done, SCLM will no longer
create output for type aaaaaaaa when members of
this language are built.
2. Change the DFLTTYP=aaaaaaaa parameter to
point to a different type. If dddddddd and
gggggggg have the same language, consider using
a pattern in the DFLTTYP parameter. See z/OS ISPF
Software Config ur ation  and Library Manager Guide
and Reference for a description of the DFLTTYP
parameter.
3. Create a new language definition that takes one
of the actions listed. Change the name of the
language. Rebuild the project definition. Instruct
the user to change the language of one of the
source members.
FLM44241 ERROR: TWO ARCHITECTURES
ARE PRODUCING THE
INTERMEDIATE FORM OF: CU
NAME: aaa(55) bbb(55) CU
TYPE: cccc CU QUALIFIER:
dddddddd ARCHITECTURE #1
TYPE: eeeeeeee MEMBER: ffffffff
ARCHITECTURE #2 TYPE:
gggggggg MEMBER: hhhhhhhh
Explanation
The intermediate form of compilation unit aaa(55)
bbb(55) is an output of two different architecture
definitions within the scope of the build. This condition
will cause the output of one of the architecture
definitions to be overwritten by the output of the other
architecture definition.
This problem is generally caused when the source
member that contains compilation unit aaa(55)
bbb(55) is referenced with a SINC in one architecture
definition and a SINC or an INCLD in another
architecture definition. The problem could also occur
if the source member is referenced as an include by
one of the source members referenced with a SINC in
an architecture definition.
User response
Remove one of the references to the source member
that contains compilation unit aaa(55) bbb(55).
FLM44280 NO INPUT SPECIFIED
IN ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb
Explanation
The architecture definition aaaaaaaa in type
bbbbbbbb was identified as an LEC or CC ARCHDEF
but does not contain a keyword to identify the inputs.
User response
Modify the architecture definition to contain input
keywords such as INCL, INCLD, and SINC, and submit
the job again.
FLM44281 INVALID KEYWORD FOR
ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb
KEYWORD: cccccccc
Explanation
The architecture definition aaaaaaaa in type
bbbbbbbb contains the invalid keyword cccccccc.
User response
Modify the architecture definition to correct or
remove the invalid keyword. Refer to z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference for a list of valid keywords for HL, LEC, CC
and generic architecture members. Run the job again.
FLM44304 COMPOOL DEPENDENCY TYPE
WAS NOT SPECIFIED FOR
SCLM messages
Chapter 3. SCLM messages  801

## Page 822

MEMBER: aaaaaaaa TYPE:
bbbbbbbb
Explanation
Build processor could not find the CREF type for
member aaaaaaaa
User response
If the member is an architecture member then verify
that a CREF keyword exists. If the member is source,
verify that the language definition (FLMLANGL macro)
specifies the DFLTCRF parameter.
FLM44306 ERROR RETRIEVING
ACCOUNTING INFORMATION
CODE: aaa GROUP: bbbbbbbb
TYPE: cccccccc COMPOOL:
dddddddd
Explanation
No accounting information exists or could be retrieved
for the specified member within the hierarchy
beginning in group bbbbbbbb. The member in question
is a compool reference where type is defined either
by the CREF keyword in an architecture member or
the DFLTCRF parameter (FLMLANGL macro) in the
language definition.
User response
Possible return codes are:
8
The member’s accounting information was not
found. Build the member that creates the specified
compool.
12
The format of the member's accounting
information was incorrect. Delete the accounting
information for the specified compool and build
the member that creates the compool to
regenerate the accounting information.
Project manager response
If the return code is:
16
Verify that the specified group is in the project
definition. If not, add it. Correct and reassemble
the project definition.
20
A VSAM error occurred. Run IDCAMS against the
accounting data set to determine the problem.
FLM44307 ERROR REFERENCING COMPOOL:
aaaaaaaa TYPE: bbbbbbbb
Explanation
The referenced compool is a member that can be
edited. It must be a member that was created by the
build function.
User response
Delete the compool from the hierarchy. Rebuild the
member that created the compool member. Submit
the job again.
FLM44309 MEMBER: aaaaaaaa TYPE:
bbbbbbbb WAS FOUND AT
GROUP: cccccccc BUT IS BEING
CROSS REFERENCED AT GROUP:
dddddddd
Explanation
A reference was made to a compilation unit contained
in member aaaaaaaa in type bbbbbbbb at group
dddddddd. However, a more current version of the
member exists at group cccccccc. The member at
group cccccccc does not contain the compilation unit
nor does any other member in the hierarchy below
group dddddddd. This problem can occur when the
language of member aaaaaaaa is changed to one
that has no compilation units or uses a different
Ada sublibrary qualifier. Use the sublibrary utility to
purge the intermediate form. Check error messages
for additional information. If the intermediate form is
deleted outside of SCLM control, use the sublibrary
utility to delete accounting information for the
intermediate form.
User response
Change the language of member aaaaaaaa if
appropriate. Add a new member with the compilation
units contained in member aaaaaaaa at group
dddddddd. Remove the references to the compilation
units in member aaaaaaaa at group dddddddd.
FLM44311 ERROR PROCESSING
COMPILATION UNITS FOR
MEMBER: aaaaaaaa TYPE:
bbbbbbbb
Explanation
An error occurred during processing of the compilation
units for the specified member. Other messages are
generated that describe the exact errors that occurred.
SCLM messages
802  z/OS: z/OS ISPF Messages and Codes

## Page 823

User response
See the message data set for all the messages related
to this error.
FLM44315 PURGE ROUTINE FOR
INTERMEDIATE FORM IS BEING
INVOKED CU NAME: aaa.(55) bbb.
(55) CU TYPE:cccc
CU QUALIFIER: dddddddd
OLD RECORD - TYPE: eeeeeeee
MEMBER: ffffffff LANG: gggggggg
NEW RECORD - TYPE: hhhhhhhh
MEMBER: iiiiiiii LANG: jjjjjjjj
Explanation
The intermediate form is no longer valid. It was
previously created by another source member, or
the language of the source member was changed.
A routine is being invoked to purge the intermediate
form. This message is provided for information only.
FLM44319 UNABLE TO PURGE
INTERMEDIATE FORM CODE: aaaa
Explanation
The purge of the intermediate form was not
successful.
User response
Submit the job again. If the error recurs, contact the
project manager.
Project manager response
Verify that the sublibrary containing the intermediate
form is not corrupted.
FLM44320 ERROR, ACCOUNTING RECORD
DATA CHANGED DURING
CURRENT BUILD FOR TYPE:
aaaaaaaa MEMBER: bbbbbbbb
Explanation
During the course of the build a user modified the
source for member aaaaaaaa and type bbbbbbbb
outside of SCLM. The SCLM accounting information did
not match the new member.
User response
Use the SCLM editor to create valid accounting
information and restart the specified build. SCLM will
continue where it left off.
FLM44321 ERROR, NEW UPWARD
DEPENDENCIES FOUND THAT ARE
NOT IN THE CURRENT BUILD
FOR TYPE: aaaaaaaa MEMBER:
bbbbbbbb
Explanation
A user has modified member bbbbbbbb during the
build process. SCLM has attempted to allow the build
to continue, but has identified new dependencies that
are outside the scope of the current build.
User response
Restart the specified build, and SCLM will continue the
build process where it left off.
FLM44322 WARNING, TYPE: aaaaaaaa
MEMBER: bbbbbbbb WAS
MODIFIED DURING THE CURRENT
BUILD
Explanation
The user has modified member bbbbbbbb during the
build. However, the user modification did not affect
the current build process. This message is provided for
information only.
FLM44323 WARNING, MEMBER: aaaaaaaa IS
SEPARATE DEPENDENCY THAT IS
NOT IN THE CURRENT BUILD
Explanation
A user has modified member aaaaaaaa during the
build process. SCLM has attempted to allow the build
to continue but has identified new dependencies that
are outside the scope of the current build.
User response
Restart the specified build, and SCLM will continue the
build process where it left off.
FLM44324 ERROR PROCESSING
DYNAMIC INCLUDE GROUP:
aaaaaaaa TYPE:bbbbbbbb
MEMBER:cccccccc.
Explanation
A build translator returned member cccccccc in type
bbbbbbbb as a dynamic include dependency for
the member being built. This member could not be
found in the project hierarchy when searching up the
hierarchy from the group aaaaaaaa.
SCLM messages
Chapter 3. SCLM messages  803

## Page 824

User response
Determine if a statement in the source is causing the
include dependency to be generated. If it is and the
statement is incorrect then update the statement. If all
of the source statements are correct then contact the
project manager.
Project manager response
Check the build translator which is returning the list of
dynamic includes to ensure that it only returns include
names for members which exist in the hierarchy.
Dynamic includes can be returned by build translators
with @@FLMINC in the translator's options.
FLM44500 >> INVOKING aaaaaaaa
TRANSLATOR(S) FOR TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The aaaaaaaa translators (where aaaaaaaa can be
BUILD, COPY, PARSE, PURGE, or VERIFY) are being
invoked for member cccccccc. This member can be
either a source member or an architecture member.
This message is provided for information only.
FLM44501 REPORT: INVOKING aaaaaaaa
TRANSLATOR(S) FOR TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The translators aaaaaaaa would be invoked for
member cccccccc if the build mode were not report-
only.
FLM44502 >> INVOKING aaaaaaaa
TRANSLATOR(S) FOR INPUT LIST
LANGUAGE: bbbbbbbb.
Explanation
Translators are being invoked for input list language
bbbbbbbb. The members of the input list will be
specified by further messages. This message is
provided for information only.
FLM44503 >> INVOKING aaaaaaaa
TRANSLATOR(S) FOR LANGUAGE:
bbbbbbbb
Explanation
The aaaaaaaa translators are being invoked for data
related to members with language bbbbbbbb. This
message is provided for information only.
FLM44504 ERROR PRINTING TO BUILD
LISTING DATA SET FOR DATA
SET NUMBER aa IN TRANSLATOR:
bbb(16) CODE: ccc
Explanation
An error occurred during the printing of a translator
data set to the build listing data set. The file number
identifies the relative position of the FLMALLOC macro
used to allocate the data set for that translator.
Note: Only data sets allocated with IOTYPE=O, W, and
S can be printed to the build listing data set.
User response
Contact the project manager.
Project manager response
If the return code is:
12
The ddnames are not allocated properly. Verify
that the build listing data set and the translator
data set are allocated. The problem can be caused
by conflicting attributes between the two data
sets.
16
The build listing data set is full. Reallocate the data
set with more storage.
20
Data access failed or SCLM did not find the input
member. Verify that the type of access is allowed.
Verify that the translator data set still exists after
all translator steps in the language definition have
been completed. A user-created translator might
have purposely deallocated the data set.
ABEND
An ABEND can occur during the printing if the
translator data set is allocated smaller than the
total space required to hold all listings for the
build, or allocated with PRINT=Y on the FLMALLOC
macro and the data set is never opened by the
translator. In such cases, specify PRINT=I on the
FLMALLOC macro. This attribute forces the data
set to be opened before the translator is invoked,
and the data set will be targeted for printing to the
build listing data set.
FLM44505 >> INVOKING aaaaaaaa
TRANSLATOR(S) FOR TYPE:
bbbbbbbb
SCLM messages
804  z/OS: z/OS ISPF Messages and Codes

## Page 825

Explanation
The aaaaaaaa translators (where aaaaaaaa can be
COPY, PURGE, or VERIFY) are being invoked for the
external dependencies that have a type of bbbbbbbb.
This message is provided for information only.
FLM44506 ERROR SAVING DATA SET
NUMBER aaaaaaaa FOR
TRANSLATOR: bbbbbbbb TO
MEMBER: cccccccc TYPE:
dddddddd
Explanation
An error occurred while copying a translator data set
to member cccccccc. The data set number identifies
the relative position of the FLMALLOC macro used to
allocate the data set for that translator (The data set
is a sequential data set allocated with IOTYPE=O, or a
PDS data set allocated with IOTYPE=P and a member
name specified by the MEMBER parameter on the
FLMALLOC macro.).
User response
See the message data set for all the messages related
to this error.
FLM44507 ERROR SAVING DATA SET
NUMBER aa FOR TRANSLATOR:
bbb(16) TO MEMBER: cccccccc
TYPE: dddddddd
Explanation
An error occurred while copying a translator data set
to member cccccccc. The data set number identifies
the relative position of the FLMALLOC macro used
to allocate the data set for that translator. The data
set is a PDS allocated with IOTYPE=P. The messages
from IEBCOPY are written to all build listing for errors,
except 913 abends.
This error can be caused by failure to specify the DCBS
option on the FLMTRNSL macro. Refer to the IEBCOPY
messages in the build listing to determine the reason
for the failure. For more information on the FLMTRNSL
macro, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
Note: The IBM linkage editor requires that the DCBS
option parameter be passed in order for the SYSLMOD
block size to be used in creating load modules. If the
DCBS option is not specified, the linkage editor will
create load modules using the maximum record size
for the device type. Use the OPTIONS= variable on the
FLMTRNSL macro to pass the DCBS option. Failure to
do so can result in message FLM44507.
User response
See the message data set for all the messages related
to this error.
FLM44508 ERROR UPDATING STATISTICS
OR ACCOUNTING INFORMATION
FOR AN ALIAS OF LOAD
MEMBER :aaaaaaaa TYPE:
bbbbbbbb
Explanation
An error occurred while updating the statistics or
accounting information for an alias of load member
aaaaaaaa. The alias member was copied successfully,
but the SCLM control information could not be created
or updated for the alias member.
User response
See the message data set for all the messages related
to this error.
FLM44510 >> DELETING OUTPUT(S) FOR
TYPE: aaaaaaaa MEMBER:
bbbbbbbb
Explanation
The output member bbbbbbbb in type aaaaaaaa at
the specified build group has been deleted.
FLM44511 REPORT: DELETING OUTPUT(S)
FOR TYPE: aaaaaaaa MEMBER:
bbbbbbbb
Explanation
The output member bbbbbbbb in type aaaaaaaa at
the specified build group would have been deleted if
the build was run in a non-report mode.
FLM44512 TRANSLATOR ERROR FOR INPUT
LIST LANGUAGE: aaaaaaaa
Explanation
A translator error occurred for input list language
aaaaaaaa. The return code from the translator was
not considered acceptable. The acceptable codes are
specified on the FLMTRNSL macro with the GOODRC
parameter.
User response
Use the listings data set to locate and correct all errors
identified by the translator. If the return code from the
translator is acceptable and build indicated that the
translator failed, contact the project manager.
SCLM messages
Chapter 3. SCLM messages  805

## Page 826

Project manager response
Change the GOODRC parameter of the FLMTRNSL
macro, which is defined in the project definition.
FLM44513 TRANSLATOR ERROR FOR
MEMBER: aaaaaaaa TYPE:
bbbbbbbb
Explanation
A translator error occurred for member aaaaaaaa. The
return code from the translator was not considered
acceptable. The acceptable return codes are specified
on the FLMTRNSL macro with the GOODRC parameter.
User response
Use the listings data set to locate and correct all errors
identified by the translator. If the return code from the
translator is acceptable and build indicated that the
translator failed, contact the project manager.
FLM44514 TARGET OUTPUT MEMBER:
aaaaaaaa TYPE: bbbbbbbb IS
EDITABLE
Explanation
The build processor cannot copy the translator output
data set. Member aaaaaaaa was created by the SCLM
editor or registered with the migration utility or SAVE
service. The build processor only updates members
that were created through the build process (non-
editables).
User response
If the specified member is no longer to be used as an
editable component in the system, delete it from the
hierarchy. Otherwise, specify a new target member.
Submit the job again.
FLM44520 >> DELETING OBSOLETE
OUTPUT(S)
Explanation
SCLM has detected output from a previous build that
is out of date. The version of the output that exists
at a higher group in your hierarchical view is up to
date. Deleting the output is being performed rather
than regenerating the output to save time and space.
Only output found at the build group is considered for
deletion. Complete details on the output deleted can
be found in the build report.
This message is provided for information only.
FLM44521 REPORT: DELETING OBSOLETE
OUTPUT(S)
Explanation
SCLM has detected output from a previous build that
is out of date. The version of the output that exists at
a higher group in your hierarchical view is up to date.
The output would be deleted if the build mode were
not report-only. Only output found at the build group is
considered for deletion.
Complete details on the output deleted can be found
in the build report.
This message is provided for information only.
FLM44522 BUILD ERROR. BUILD MAP
CONTAINS A NOT PROMOTED
MEMBER (NOPROM) BUT BUILD
MAP HAS BEEN SPECIFIED AS
NEEDING TO BE REBUILT. BUILD
MAP: aaaaaaaa BMAP TYPE:
bbbbbbbb. EITHER DELETE THE
BUILD MAP IN ERROR AND
REBUILD OR PROMOTE THE
NOPROM MEMBER.
Explanation
BUILD MAP: aaaaaaaa BMAP TYPE: bbbbbbbb which
contains a NOPROM build map record was promoted.
The NOPROM member was left behind but the build
map was not rebuilt at the next level (NOPROM-N).
Now when attempting a build of the build map at
a level other than original build level containing the
NOPROM member causes the build map to be rebuilt.
This rebuild will generally be caused by members
other than NOPROM member being promoted but the
build map in error not being built and promoted.
User response
For this build map you need to determine if the
changes including the member causing the rebuild
need to be incorporated with your NOPROM member.
If this is the case, perform a build at the level
containing the NOPROM member and re-promote the
changes. If the changes including the member causing
the rebuild supercede the NOPROM changes, delete
the build map at the level getting the build error and
rebuild the build map.
FLM44523 NOPROM MEMBER HAS BEEN SET
AS UP TO DATE. BMAP MEMBER:
aaaaaaaa TYPE: bbbbbbbb.
NOPROM MEMBER: cccccccc TYPE:
dddddddd.
SCLM messages
806  z/OS: z/OS ISPF Messages and Codes

## Page 827

Explanation
BUILD MAP: aaaaaaaa BMAP TYPE: bbbbbbbb
contains a NOPROM build map record for NOPROM
MEMBER: cccccccc TYPE: dddddddd. The NOPROM
member was left behind but the build map was not
rebuilt at the next level (NOPROM-N). Now when
attempting a build of the build map at a level other
than original build level containing the NOPROM
member, SCLM has determined that the accounting
date/time does not match the NOPROM build record
for this NOPROM member. To stop a rebuild of the
member and remove the NOPROM member changes,
SCLM has set the NOPROM member as up to date for
this build map.
User response
This message is for informational purposes only. No
action is required.
FLM44600 >>>>> INPUT LIST CONTENTS
FOR LANGUAGE: aaaaaaaa
TRANSLATOR: bbb(16)
Explanation
The message is a header to the contents of an input
list. The members of the input list will be specified
by further messages. This message is provided for
information only.
FLM44601 TYPE aaaaaaaa MEMBER
bbbbbbbb ===> ccc
Explanation
This message identifies the return code for each
member in the input list. If the return code indicates
success as defined in the FLMTRNSL macro, all
outputs are being saved in the hierarchy, and no
response is necessary. If the return code from the
translator did not meet the MBRRC or GOODRC value
specified for the translator, SCLM saves translator
output, such as compiler listings, in the listings data
set for the processor if requested in the language
definition.
User response
Use the listings data set to locate and correct all errors
identified by the translator. If the return code from the
translator is acceptable and build indicated that the
translator failed, contact the project manager.
Project manager response
Change the MBRRC parameter of the FLMTRNSL
macro, which is defined in the project definition.
FLM44602 TYPE aaaaaaaa MEMBER
bbbbbbbb NOT BUILT
Explanation
This message identifies members of the current input
list that were not built. Members of the input list
will not be built when an error occurs when the
translator encounters an error processing an earlier
member of the Input List. This message is provided for
information only.
User response
Use the listings data set to locate and correct all errors
identified by the translator. If the return code from the
translator is acceptable and build indicated that the
translator failed, contact the project manager.
FLM44609 >>>>> TOTAL MEMBERS ===> aaa
TOTAL SLOC = bbb
Explanation
This message describes the total number of members
and the total source lines of code (SLOC) in the
preceding input list. This message is provided for
information only.
User response
Contact the project manager if you want more
members or SLOC per input list.
Project manager response
Change the MBRLMT parameter, the SLOCLMT
parameter, or both parameters of the FLMLANGL
macro for the desired translator. The FLMLANGL macro
is located in the project definition.
FLM45000 ERROR PROCESSING CURRENT
BUILD
Explanation
An error was encountered during the processing of the
current build.
User response
Examine the build messages file to determine the
cause of the error during the build.
FLM46000 BUILD PROCESSOR COMPLETED -
aaaaaaaa ON bbbbbbbb
SCLM messages
Chapter 3. SCLM messages  807

## Page 828

Explanation
The build processor completed.
User response
See the message data set for all the messages related
to the build.
FLM49000 INVOKING BUILD PROCESSOR
Explanation
This message is provided for information only.
FLM51000 PROMOTE PROCESSOR INITIATED
- aaaaaaaa ON bbbbbbbb
Explanation
This message is provided for information only.
FLM51001 BLANK USERID IS SPECIFIED
AS AN INPUT TO THE PROMOTE
PROCESSOR.
Explanation
The promote process has started. A blank user ID
was specified as an input parameter. If the processor
was invoked through the SCLM Promote panel, SCLM
retrieves the user ID from the ISPF variable pool.
User response
Verify that the user ID specified in the input parameter
is correct and non-blank. For more information
about the promote input parameters, see the topic
about the PROMOTE service in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM51002 INVALID SCOPE SPECIFIED.
Explanation
The promote scope specified is invalid. Valid promote
scopes are NORMAL, SUBUNIT, and EXTENDED.
User response
Verify that the input parameters specified for the
promote processor are correct.
FLM51003 INVALID PROMOTE MODE
SPECIFIED.
Explanation
The promote mode specified is invalid. Valid promote
modes are CONDITIONAL, UNCONDITIONAL, and
REPORT.
User response
Verify that the input parameters specified for the
promote processor are correct.
FLM51004 PROMOTE BYPASSED, GROUP:
aaaaaaaa IS TOP GROUP.
Explanation
Group aaaaaaaa has no group defined to promote to
in this project definition. The promote report is created
as if this were a report-only promote.
User response
Verify that the group specified as an input parameter
to the promote processor is the group containing the
data to be promoted. Also verify that you specified the
correct project definition as an input to the promote
processor.
FLM51006 SPECIFIED GROUP: aaaaaaaa IS
A PRIMARY NON-KEY GROUP
Explanation
Group aaaaaaaa is a primary non-key group.
Promoting from a primary non-key group is not
allowed. For a definition of primary non-key groups,
see z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
User response
Verify that the group and project definition specified
as inputs to the promote processor are correct. For
more information about primary non-key groups, see
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference.
FLM51007 PROMOTE GROUP DATA SET
NAMES NOT UNIQUE. GROUP 1:
aaaaaaaa GROUP 2: bbbbbbbb
DATA SET NAME: aaa(44)
Explanation
The two groups indicated have the same data set
name specified for them in the project definition.
Promote will not copy and/or purge data from the data
sets because both of the data sets have the same
SCLM messages
808  z/OS: z/OS ISPF Messages and Codes

## Page 829

name and the promote function could result in data
loss.
Project manager response
Check the project definition FLMALTC parameters on
the two groups specified. For one of the groups, either
change the promote path or the dsname specified on
the FLMALTC macro.
FLM51008 USER DEFINED DDNAME:
aaaaaaaa FOR PROMOTE
MESSAGES NOT ALLOCATED
Explanation
The ddname aaaaaaaa , which was specified for the
promote messages, was not allocated. If the promote
function is invoked through the SCLM services, the
ddname for the promote messages is optional. If the
ddname is not specified, the promote messages are
sent to the terminal by default. If the ddname is
specified, it must be allocated.
User response
Verify that the user-supplied ddname for promote
messages is allocated. Submit the job again.
FLM51009 USER DEFINED DDNAME:
aaaaaaaa FOR PROMOTE REPORT
NOT ALLOCATED
Explanation
DDname aaaaaaaa , which was specified for the
report, was not allocated. If the promote function
is invoked through SCLM services, the ddname for
the promote report is optional. If the ddname is not
specified, the promote report is sent to the terminal
by default. If the ddname is specified, it must be
allocated.
User response
Verify that the user-supplied ddname for promote
report is allocated. Submit the job again.
FLM51010 USER DEFINED DDNAME:
aaaaaaaa FOR COPY ERROR
MESSAGES NOT ALLOCATED
Explanation
Ddname aaaaaaaa , which was specified for the
promote copy error messages, was not allocated.
If the promote function is invoked through SCLM
services, the ddname for the copy error messages is
optional. If the ddname is not specified, the copy error
messages are sent to the terminal by default. If the
ddname is specified, it must be allocated.
User response
Verify that the user-supplied ddname for copy error
messages is allocated. Submit the job again.
FLM51011 USER DEFINED DDNAME:
aaaaaaaa FOR USER EXIT FILE
NOT ALLOCATED
Explanation
DDname aaaaaaaa, which was specified for the user
exit data set, was not allocated. If the promote
function is invoked through the SCLM services, the
ddname for the user exit data set is optional. If not
specified, a user exit data set is allocated to NULLFILE.
If the ddname is specified, it must be allocated.
User response
Verify that the user-supplied ddname for user exit data
set is allocated. Submit the job again.
FLM51103 NO KEY GROUP EXISTS BELOW
GROUP: aaaaaaaa
Explanation
Group aaaaaaaa is a non-key group and is defined as
one of the lowest groups in the hierarchy. No key group
is defined below group aaaaaaaa in the hierarchy.
User response
Contact the project manager.
Project manager response
The lowest groups of the project hierarchy (the
development groups) must be key groups. Modify the
project definition to make the lowest group key and
resubmit the job.
FLM52000 INITIATING VERIFICATION
PHASE - aaaaaaaa ON bbbbbbbb
Explanation
Indicates that the promote verification phase has been
initiated. In this phase, SCLM verifies all members
within the scope of the architecture definition. All
members must be up to date (for example, source
matches object) and must have correct accounting
information. This message is for information only.
FLM52001 VERIFICATION PHASE FOR
GROUP: aaaaaaaa
SCLM messages
Chapter 3. SCLM messages  809

## Page 830

Explanation
Indicates that the promote verification phase has
been initiated for group aaaaaaaa. This message is for
information only.
FLM52103 ERROR RETRIEVING BUILD MAP
INFORMATION, CODE: aaa TYPE:
bbbbbbbb
MEMBER: cccccccc
REFERENCED BY BUILD MAP AT
TYPE: dddddddd MEMBER:
eeeeeeee
Explanation
SCLM could not retrieve build map information for
member eeeeeeee.
User response
Possible return codes are:
8
Determine if the member and type specified are
correct. If the member and type are correct,
then build the architecture member used as input
for this promotion again. Otherwise, specify the
correct member and type and invoke the promote
function again. Submit the job again.
12
The format of the data retrieved was incorrect.
Delete the build map and build again to regenerate
it.
16
An invalid group was found in the project
definition. Contact the project manager.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
If the return code is:
16
Reassemble the project definition. Verify that
no errors occurred. Relink the project definition.
For more information about linking the project
definition, see z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
20
A VSAM error occurred. Run IDCAMS against the
accounting data set to determine the problem.
See z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM52104 ERROR PROCESSING BUILD MAP
FOR MEMBER: aaaaaaaa IN TYPE:
bbbbbbbb MEMBER REFERENCED
IN BUILD MAP: cccccccc TYPE OF
MEMBER REFERENCED IN BUILD
MAP: dddddddd BUILD MAP DATE:
eeeeee BUILD MAP TIME: ffffff
Explanation
An error occurred during an attempt to process an
undefined keyword in the Build Map for member
aaaaaaaa. The undefined keyword is associated with
member cccccccc, in type dddddddd, as referenced
in the Build Map. The Build Map might have been
generated by a prior version of SCLM.
User response
Rebuild member aaaaaaaa in type bbbbbbbb to
generate a new Build Map.
FLM52105 VERSION MISMATCH FOR
LANGUAGE: aaaaaaaa BUILD MAP
MEMBER: bbbbbbbb IN TYPE:
cccccccc LANGUAGE VERSION IN
BUILD MAP: dddddddd LANGUAGE
DEFINITION VERSION: eeeeeeee
Explanation
Since the last build, a new version of the translator for
the language was installed.
User response
Rebuild the member using the member and type
specified for the current promote operation. Once
the member has been successfully rebuilt, you can
promote it again.
FLM52106 ERROR PROCESSING BUILD MAP.
MEMBER: aaaaaaaa IN TYPE:
bbbbbbbb CONTAINS NOPROM-N
AND NOPROM-R ENTRIES. BUILD
MAP MUST NOT CONTAIN MIXED
NOPROM STATUS ENTRIES.
Explanation
This is an error message issued during the promote
verification. Member aaaaaaaa in type bbbbbbbb
contains both NOPROM-N and NOPROM-R members.
NOPROM-N and NOPROM-R are mutually exclusive.
User response
Ensure that all NOPROM members have the same
accounting status.
SCLM messages
810  z/OS: z/OS ISPF Messages and Codes

## Page 831

FLM52901 SCOPE: a SPECIFIED AS INPUT
IS INCOMPATIBLE WITH SCOPE:
b FOR LANGUAGE: cccccccc OF
MEMBER: dddddddd IN TYPE:
eeeeeeee
Explanation
Scope a requested for this promote has a smaller
range than the scope b specified in the project
definition for the language cccccccc of member
dddddddd. Promote accepts three values for scope:
NORMAL, SUBUNIT, and EXTENDED. NORMAL has the
smallest range; EXTENDED has the greatest range.
User response
Specify an equal or a larger range scope than the
scope of the member's language being promoted.
If a non-Ada source is being promoted, NORMAL is
usually sufficient for the promote scope. Otherwise,
EXTENDED scope is always compatible with the
languages. Verify that the architecture definition being
promoted has been built with the scope used as input
to the promote function.
FLM52904 THE SOURCE MEMBER: aaaaaaaa
IN TYPE: bbbbbbbb WAS
COMPILED WITHOUT THE
DEPENDENT COMPILATION UNIT:
CU NAME: ccc(55) ddd(55)
CU TYPE: eeeeeee
CU QUALIFIER: ffffffff
Explanation
The compilation unit ccc(55) ddd(55) was added since
the last build of source member cccccccc.
User response
Rebuild the architecture definition using the specified
scope. Resubmit the job.
FLM52905 ARCHITECTURE MEMBER:
aaaaaaaa IN TYPE: bbbbbbbb IS
NOT CURRENT THE DOWNWARD
DEPENDENCY COMPILATION
UNIT: CU NAME: ccc(55) ddd(55)
CU TYPE: eeee CU QUALIFIER:
ffffffff
OF MEMBER: gggggggg
TYPE: hhhhhhhh
HAS NOT BEEN BUILT.
Explanation
Compilation unit ccc(55) ddd(55) has never been built.
This error could occur for one of these reasons:
• The specified compilation unit was introduced to the
product after the architecture member was built.
• The specified member has never been built in
EXTENDED scope.
User response
Rebuild the specified architecture definition in
EXTENDED scope, and submit the job again.
FLM53005 BUILD MAP FOR MEMBER:
aaaaaaaa IN TYPE: bbbbbbbb
IS NOT CURRENT DATE/TIME
MISMATCH ON MEMBER: cccccccc
IN TYPE: dddddddd BUILD MAP
ENTRY DATE/TIME: eeeeeeee
ffffffff ACCOUNTING DATE/TIME:
gggggggg hhhhhhhh
Explanation
Member aaaaaaaa will not be promoted. The build
map for the member is not current. Possible causes:
• An input has changed since the last build.
• An output does not match the output produced by
the build.
• The build was not run with extended scope.
User response
Rebuild the member being promoted, then restart the
promote. If the build was not done with extended
scope, rerun the build using extended scope.
FLM53006 ARCHITECTURE MEMBER:
aaaaaaaa IN TYPE: bbbbbbbb
IS NOT CURRENT VERSION
MISMATCH ON MEMBER: cccccccc
IN TYPE: dddddddd BUILD MAP
VERSION: eeeeeeee ACCOUNTING
VERSION: ffffffff
Explanation
The version number of member or Build Map cccccccc
has changed since the last time the architecture
member was built. Also, the version number of
member cccccccc or Build Map eeeeeeee has changed
since the last time the architecture member was built.
User response
Rebuild the architecture member being promoted and
submit the job again.
FLM53106 PREDECESSOR VERIFICATION
FAILED
SCLM messages
Chapter 3. SCLM messages  811

## Page 832

INPUT GROUP: aaaaaaaa
TYPE:bbbbbbbb
MEMBER: cccccccc
ERROR GROUP1: dddddddd
DATE: eeeeeeee TIME: ffffffff
ERROR GROUP2: gggggggg
DATE: hhhhhhhh TIME: iiiiiiii
Explanation
The version of the member in dddddddd was not
based on the member in gggggggg. This error usually
means that a version of the member between the two
groups has been deleted.
The predecessor Date and Time fields in the
accounting information for the member in dddddddd
should contain the last modified Date and Time fields
for the next occurrence of the member within the
hierarchy.
The promote processor, in CONDITIONAL mode,
prevents the member in gggggggg from being replaced.
User response
Verify that the member contains all of the required
changes present in the member in gggggggg. If it does,
and no other promote verification errors are present,
promote again in UNCONDITIONAL mode.
If other promote verification errors are present, either
correct the errors or use an architecture member that
controls as few members as possible.
FLM53108 MEMBER: aaaaaaaa TYPE:
bbbbbbbb AT GROUP: cccccccc IS
NOT ELIGIBLE FOR PROMOTION
Explanation
One or more of the accounting information fields
for member aaaaaaaa has an invalid value, which
prevents SCLM from promoting the member. The fields
are:
• AUTHORIZATION CODE CHANGE
• ACCESS KEY
• ACCOUNTING RECORD TYPE.
If the Authorization Code Change field is not blank, an
attempt to change the &authcode of the member did
not complete successfully.
If the Access Key field is not blank, the member has
been reserved for use outside the project hierarchy or
blocked from promotion.
If the ACCOUNTING RECORD TYPE is INITIAL or
LOCKOUT, a lock has been placed on the member but
changes to the member have not been registered with
SCLM. The source for the member either does not exist
or does not match the accounting information.
User response
Use the SCLM library utility to review the contents of
the specified fields. If the Authorization Code Change
field is not blank, verify that the &authcode for the
member is correct. If it is, use the update capability
of the utility to reset the field. If the field should be
changed, use the utility to complete the change in
progress or assign a new authorization code.
If the ACCESS KEY is not blank, refer to local &scm
procedures to determine the cause of action based
on the values of the access key. If the access key is
eligible for removal, use the UNLOCK service to reset
the access key to blanks.
If the ACCOUNTING RECORD TYPE is initial or lockout
and the member is not present in the group you are
promoting from, delete the accounting information
using the library utility (or use an equivalent function
such as the UNLOCK service).
If the member exists, use the SCLM editor or SAVE
service to create correct accounting information.
Rebuild the architecture member being promoted after
the accounting information has been either deleted or
updated.
FLM53109 WARNING, PREDECESSOR
VERIFICATION FAILED INPUT
GROUP: aaaaaaaa
TYPE:bbbbbbbb
MEMBER: cccccccc
ERROR GROUP1: dddddddd
DATE: eeeeeeee TIME: ffffffff
ERROR GROUP2: gggggggg
DATE: hhhhhhhh TIME: iiiiiiii
Explanation
The version of the member in dddddddd was not
based on the member in gggggggg. This error usually
means that a version of the member between the two
groups has been deleted.
The predecessor date and time fields in the accounting
information for the member in dddddddd should
contain the last modified Date and Time fields for the
next occurrence of the member within the hierarchy.
This message is a warning. However, the promote
processor, in CONDITIONAL mode, prevents the
member from replacing the member in gggggggg.
This message occurs if the NOPROM member with
the authorization code that is not defined to the
target group cannot be promoted using one of the
authorization codes defined to the from group.
SCLM messages
812  z/OS: z/OS ISPF Messages and Codes

## Page 833

User response
Usually for this promote, no action is required.
An attempt to promote member cccccccc to group
gggggggg will fail in CONDITIONAL mode. However, if
member cccccccc has accounting status NOPROM-R,
a manual build must be performed at the target level
as the FLM53109 warning message ends the promote
processing and the NOPROM rebuild step following the
promote will not run. See the Software Config ur ation 
and Library Manager (SCLM) Guide and Reference for
more information about NOPROM.
FLM53901 ERROR RETRIEVING
ACCOUNTING INFORMATION FOR
INTERMEDIATE FORM OF: CU
NAME: aaa(55) bbb(55) CU TYPE:
cccc CU QUALIFIER: dddddddd
CODE: eee GROUP: ffffffff
Explanation
An error occurred while attempting to retrieve
accounting information for the specified intermediate
form. The error code associated with the error
message provides specifics regarding the nature of the
error.
User response
Possible return codes are:
8
The accounting information for the intermediate
form of the compilation unit was not found in
the specified group in the hierarchical view. The
compiled intermediate form might be missing or
out of date. Build the member containing the
compilation unit.
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
An I/O error occurred while retrieving the
accounting information for the intermediate form
of the compilation unit. Submit the job again. If the
error recurs, contact the project manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
A VSAM error occurred. Run IDCAMS against
the cross-reference data set to determine the
problem.
24
Identify the cross-reference data set on the
FLMCNTRL macro of the project definition. For
more information, about the FLMCNTRL macro,
see z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM53902 ARCHITECTURE MEMBER:
aaaaaaaa IN TYPE: bbbbbbbb
IS NOT CURRENT VERIFICATION
ERROR FOR COMPILATION UNIT:
CU NAME: ccc(55) ddd(55)
CU TYPE: eeee
CU QUALIFIER: ffffffff
BUILD MAP DATE/TIME:
gggggggg hhhhhhhh
ACCOUNTING DATE/TIME:
iiiiiiii jjjjjjjj
Explanation
A change has occurred since the last build of the
&alist being promoted. The output of the build does
not match the input. Build output for the specified
compilation unit was based on the Build Map date
and time indicated. The specified compilation unit has
since been updated but the updates have not been
built.
User response
Rebuild the &alist being promoted and submit the job
again.
FLM53903 WARNING, INTERMEDIATE
FORM AND ACCOUNTING
INFORMATION FOR THE
FOLLOWING COMPILATION UNIT
WILL BE PURGED FROM
GROUP: aaaaaaaa CU NAME:
bbb(55) ccc(55) CU TYPE: dddd
CU QUALIFIER: eeeeeeee FROM-
GROUP MEMBER: ffffffff TYPE:
gggggggg LANGUAGE: hhhhhhhh
ABOVE-GROUP MEMBER: iiiiiiii
TYPE: jjjjjjjj LANGUAGE: kkkkkkkk
SCLM messages
Chapter 3. SCLM messages  813

## Page 834

Explanation
The source for compilation unit bbb(55) ccc(55) was
moved to a different member. This move would cause
the intermediate form of the compilation unit to exist
in more than one sublibrary in the specified group
unless the intermediate form is purged. SCLM does not
allow multiple copies of a member's compilation unit
to exist in one group of the hierarchy; therefore, the
old compilation unit is purged.
User response
No action is necessary unless the promote fails to copy
the compilation unit identified. If the copy failed, the
group will not contain a copy of the compilation unit
until the promote completes successfully.
FLM53905 ERROR RETRIEVING
ACCOUNTING INFORMATION FOR
INTERMEDIATE FORM OF: CU
NAME: aaa(55) bbb(55) CU TYPE:
cccc CU QUALIFIER: dddddddd
CODE: eee GROUP: ffffffff
Explanation
An error occurred while attempting to retrieve
accounting information for the intermediate form
aaa(55) bbb(55).
User response
Possible return codes are:
8
The accounting information for the intermediate
form of the compilation unit was not found at
the specified group. This error indicates that the
compiled intermediate form is missing or out of
date. You need to build the member containing the
compilation unit.
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
An I/O error occurred retrieving the accounting
information for the intermediate form of the
compilation unit. Resubmit the job, and if the error
recurs, contact the project manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
A VSAM error occurred. Run IDCAMS against
the cross-reference data set to determine the
problem.
24
Identify the cross-reference data set on the
FLMCNTRL macro of the project definition. For
more information on the FLMCNTRL macro see
the z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM53906 ARCHITECTURE MEMBER:
aaaaaaaa IN TYPE: bbbbbbbb
IS NOT CURRENT. THE BUILD
MAP NO LONGER REFERENCES
COMPILATION UNIT: CU NAME:
aaa(55) bbb(55) CU TYPE: cccc CU
QUALIFIER: dddddddd
Explanation
The Build Map specified no longer contains a reference
to the compilation unit, although the compilation unit
still exists.
User response
If the compilation unit is outdated, then delete the
compilation unit using the Sublibrary Management
Utility (Option 3.2). Then perform another build at this
group in EXTENDED mode. Otherwise, attempt a build
in EXTENDED mode.
FLM55000 INITIATING COPY PHASE -
aaaaaaaa ON bbbbbbbb
Explanation
The promote verification phase was successful. The
copy phase has started. This message is provided for
information only.
FLM55100 STARTING PACKAGE BACKUP
PHASE - hh:mm:ss ON
yyyy/mm/dd
SCLM messages
814  z/OS: z/OS ISPF Messages and Codes

## Page 835

Explanation
Processing for package backup started at hh:mm:ss
ON yyyy/mm/dd.
FLM55110 BACKING UP TYPE: nnnnnnnn,
MEMBER: mmmmmmm, FROM:
ggggggg, TO: bbbbbbbb
Explanation
Package backup is backing up the member
mmmmmmmm in group gggggggg, type nnnnnnnn to
the backup group bbbbbbbb.
FLM55120 UNABLE TO BUILD PACKAGE
DETAILS MEMBER; ERROR
DURING ffffffff PDS: pppp.pppp
MEMBER: mmmmmmm LM
FUNCTION RC: rr
Explanation
SCLM was unable to build the package details member
due to an error running the LM function fffffffff against
the member mmmmmmm in the PDS pppp.pppp. The
return code from the LM function is rr.
User response
Refer to the ISPF services guide to determine why the
LM function failed with a return code rr.
FLM55125 INVALID RETURN CODE FROM
FLMLDATE ROUTINE PDS:
pppp.pppp MEMBER: mmmmmmm
LM FUNCTION RC: rr
Explanation
SCLM has encountered an error attempting to retrieve
the load module date from the member mmmmmmm
in the PDS pppp.pppp. The return code from the
FLMLDATE routine is rr.
User response
Contact IBM to determine the reason why the date in
the load module has been corrupted.
FLM55140 UNABLE TO PROCESS PACKAGE
DETAILS MEMBER; ERROR
DURING ffffffff PDS: pppp.pppp
MEMBER: mmmmmmm LM
FUNCTION RC: rr
Explanation
SCLM encountered an error processing the package
details member mmmmmmmm in the PDS pppp.pppp
while running the LM function fffffffff. The return code
from the LM function is rr.
User response
Refer to the ISPF Services Guide to determine why the
LM function failed with a return code rr.
FLM55150 ERROR CREATING PACKAGE
RESTORE VERSION OR AUDIT
RECORD CODE: rr ERROR GROUP:
ggggggg TYPE: tttttttt MEMBER:
mmmmmmm
Explanation
SCLM when creating a package encountered an error
attempting to create a Audit or Version VSAM record
for the member mmmmmmmm in group gggggggg with
a type of ttttttt. Return code from the VSAM write is rr.
User response
Check the promote message file to determine why
SCLM was getting errors writing the Version or audit
VSAM record.
FLM55160 ERROR DELETING PACKAGE
DETAILS FILE PDS: pppp.pppp
MEMBER: mmmmmmm FLMPKUTL
RC: rr
Explanation
SCLM encountered an error deleting the package
details member mmmmmmm in the PDS pppp.pppp.
The return code from the LM function is rr.
User response
Check to see that the package details member
mmmmmmm in the PDS pppp.pppp exists and is not
in use.
FLM55185 PACKAGE CANNOT BE RESTORED -
IN PENDING STATUS
Explanation
SCLM found the package backup member was in a
pending state as it is being processed by another user/
job.
User response
Determine who is restoring the package and
coordinate the processing with them.
SCLM messages
Chapter 3. SCLM messages  815

## Page 836

FLM55190 ENDING PACKAGE BACKUP
PHASE - hh:mm:ss ON
yyyy/mm/dd
Explanation
Processing for package backup ended at hh:mm:ss ON
yyyy/mm/dd.
FLM55195 INVALID PACKAGE DETAILS
FILE HEADER RECORD
PDS: pppp.pppp MEMBER:
mmmmmmmm PACKAGE STATUS:
ssssssss LOCAL RC: rr
Explanation
When reading package backup member
mmmmmmmm in the PDS pppp.pppp SCLM
encountered an invalid header record.
User response
Determine why the header record for package backup
member mmmmmmmm in the PDS pppp.pppp is
corrupted. This member SHOULD NOT be maintained
outside of SCLM.
FLM55196 PACKAGE REUSE IS IN EFFECT
Explanation
While performing package backup, SCLM determined
that package reused is in effect. Rather than
overwriting the package details member, SCLM will
update the existing package details with the members
being promoted.
FLM55197 DELETING EXISTING SAME-
NAMED PACKAGE
Explanation
As part of the package backup process, SCLM
determined that package reuse was not in effect
and hence the existing package details member will
be deleted prior to it being created as part of the
promotion process.
FLM55198 PACKAGE IN PENDING STATUS -
TO BE REUSE
Explanation
As part of the package backup process, SCLM
determined that package reuse was in effect but the
package was in a pending state. The package details
member will be updated with the promote package
details.
FLM55199 PACKAGE BACKOUT MIGRATION
HAS NOT BEEN RUN YET, PLEASE
RUN FLMBKFIX AGAINST: PDS:
pppp.pppp MEMBER: mmmmmm
PACKAGE STATUS: sssss (OR ALL
MEMBERS)'
Explanation
Changes were introduced in APAR OW56081 which
required the package backup members to be migrated
to a new format. This migration process has not
been run for the member mmmmmmmm in the PDS
pppp.pppp.
User response
Run FLMBKFIX against either the member
mmmmmmmm in the PDS pppp.pppp or all members
in the PDS.
FLM55201 ERROR OCCURRED DELETING
ACCOUNTING INFORMATION
FOR INTERMEDIATE FORM OF
DISCREPANCY ITEMS
Explanation
An error occurred while attempting to purge an
intermediate form or intermediate accounting record
in the "from" group. The intermediate form’s type or
member name at the "from" group does not match the
"to" group. Check that the source for a compilation unit
was not moved to a different member.
User response
See the message data set for all the messages related
to this error.
FLM55904 COPY OF INTERMEDIATE
FORM FAILED FOR LANGUAGE:
aaaaaaaa
Explanation
An error occurred while copying an intermediate form
of a compilation unit in language aaaaaaaa.
User response
See the message data set for all the messages related
to this error.
FLM55905 ERROR PURGING CROSS
REFERENCE INFORMATION FOR
EXTRA COMPILATION UNIT: CU
NAME: aaa(55) bbb(55)
CU TYPE: cccc
SCLM messages
816  z/OS: z/OS ISPF Messages and Codes

## Page 837

CU QUALIFIER: dddddddd CODE:
eee
GROUP: ffffffff TYPE: gggggggg
MEMBER: hhhhhhhh
Explanation
The promote processor deletes all the cross-reference
information for extra compilation units in group ffffffff
before it copies new text and accounting records
of all the members. An extra compilation unit is a
compilation unit that exists in the "to" group but does
not exist in the "from" group for a member existing in
both groups. This situation occurs when you modify a
member with an extra compilation in a development
library and then delete the extra compilation unit of
the member from the development library.
While deleting the cross-reference information from
the group for the compilation unit specified, an error
occurred and SCLM issued a return code.
User response
Possible return codes are:
8
A severe I/O error occurred. Contact the project
manager.
16
The cross-reference data set is enqueued. Try the
job again later.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
8
A VSAM error occurred. Run IDCAMS against
the cross-reference data set to determine the
problem.
24
Identify the cross-reference data set on the
FLMCNTRL macro of the project definition. For
more information on the FLMCNTRL macro, see
z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM57000 INITIATING PURGE PHASE -
aaaaaaaa ON bbbbbbbb
Explanation
The promote verification and copy phases have
completed successfully. The purge phase has started.
This message is provided for information only.
FLM57001 INITIATING PURGE FROM GROUP:
aaaaaaaa
Explanation
SCLM has started purging members beginning at group
aaaaaaaa. Purge processing completes at one group
before processing of the next group begins. This
message is provided for information only.
FLM57101 WARNING, ACCOUNTING
INFORMATION IS NOT CURRENT
FOR GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The accounting information for member cccccccc does
not match the contents of the member. It is possible
that the member has been updated outside of SCLM
control.
User response
Define the member to SCLM using the SCLM editor or
the SAVE service. If the member is not needed, delete
it using the SCLM library utility or the DELETE service.
FLM57102 WARNING, ACCOUNTING
INFORMATION EXISTS FOR
GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc.
THE CORRESPONDING DATA
SET: ddd(44) DOES NOT EXIST.
THE ACCOUNTING INFORMATION
WILL BE PURGED.
Explanation
SCLM has found accounting information for
aaaaaaaa.bbbbbbbb.cccccccc and the corresponding
data set ddd(44) does not exist. SCLM expected
to find the data set and could not. Because the
data set does not exist and there is no member
that corresponds to the accounting information, the
accounting information will be purged.
User response
This message is for informational purposes only. No
action is required.
FLM57105 ERROR UPDATING THE BUILD
MAP CONTAINING NOT
PROMOTED MEMBER. BMAP
NAME: aaaaaaaa BMAP TYPE:
bbbbbbbb RC=rr.
SCLM messages
Chapter 3. SCLM messages  817

## Page 838

Explanation
SCLM encountered an error attempting to update build
map record.
User response
Determine why SCLM was unable to update the build
map record.
FLM57106 WARNING, THE NOT PROMOTED
BACKUP DATA SET (NPROMBK)
WAS NOT SPECIFIED, THE NOT
PROMOTED MEMBERS WILL NOT
BE BACKED UP.
Explanation
The not promoted backup data set is not defined for
the project. Hence, when attempting to promote a
build map with a NOPROM build map record which
has an accounting status of NOPROM-N, SCLM will
not be able to backup the NOPROM member. If the
NOPROM member is changed after promotion and the
NOPROM member was not backed up, then it will not
be possible to recreate the build outputs that were
built/promoted based on this member.
User response
If a backup of the NOPROM member is required,
modify the SCLM project definition to add a NPROMBK
parameter on the FLMCNTRL macro to specify the
NOPROM backup data set name.
FLM57107 ERROR BACKING UP NOT
PROMOTED MEMBER. MEMBER:
aaaaaaaa TYPE: bbbbbbbb RC=rr
ERROR TEXT: xxxxxxxxxxxxxxx
Explanation
SCLM encountered an error attempting to create a
backup of the NOPROM MEMBER: aaaaaaaa TYPE:
bbbbbbbb during promotion.
User response
Use the error messages to determine why SCLM was
unable to backup the member aaaaaaaa.
FLM57108 ERROR RETRIEVING NOT
PROMOTED BACKUP COUNT FROM
CONTROL FILE RC=rr ERROR
MSG1 : xxxxxxxxxxxxxxx ERROR
MSG2 : yyyyyyyyyyyyyyy
Explanation
SCLM encountered an error attempting to retrieve not
promoted backup count from control file.
User response
Use the error messages to determine why SCLM was
unable to retrieve not promoted backup count from
the control file.
FLM57150 AUTOMATIC REBUILD
UNAVAILABLE FOR PROMOTE BY
CHANGE CODE
Explanation
User response
This message is for informational purposes only. No
action is required.
FLM57201 PURGE OF INTERMEDIATE FORM
FAILED FOR GROUP: aaaaaaaa
Explanation
Unable to purge intermediate form from the group.
User response
See the message data set for all the messages related
to this error.
FLM58000 PROMOTE PROCESSOR
COMPLETED aaaaaaaa ON
bbbbbbbb
Explanation
The promote processor completed.
User response
See the message data set for all the messages related
to the outcome of this promote.
FLM59001 INVOKING PROMOTE PROCESSOR
Explanation
This message is provided for information only.
FLM61007 DATABASE CONTENTS UTILITY
INITIATED - aaaaaaaa ON
bbbbbbbb
SCLM messages
818  z/OS: z/OS ISPF Messages and Codes

## Page 839

Explanation
Database utility processing has started. This message
is provided for information only.
FLM61008 NUMBER OF PAGES GENERATED
FOR THE REPORT - aaa(10)
Explanation
This message is provided for information only.
FLM61009 NUMBER OF PAGES GENERATED
FOR THE TAILORED OUTPUT -
aaa(10)
Explanation
This message is provided for information only.
FLM61011 NO aaaa RECORDS FOUND
Explanation
SCLM cannot find a list of records in the VSAM data
sets.
User response
Contact the project manager.
Project manager response
A VSAM error occurred while attempting to retrieve
a list of records from the VSAM data sets. The type
of record being retrieved is specified in the message
text. Refer to the Access Method Services for help in
determining the VSAM error.
FLM61012 VSAM I/O ERROR OCCURRED
WHILE ACCESSING THE
ACCOUNTING DATABASE
Explanation
There is a problem with the VSAM file that SCLM uses
to store its accounting information.
User response
Contact the project manager.
Project manager response
Refer to the Access Method Services for help in
determining the VSAM error.
FLM61015 ERROR RETRIEVING
ACCOUNTING OR CROSS-
REFERENCE INFORMATION CODE:
aaa ERROR GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
No accounting record exists or could be retrieved
for member dddddddd within the hierarchical view
beginning at group bbbbbbbb.
User response
Possible return codes are:
8
SCLM did not find the member’s accounting
information. Register the member with SCLM using
the edit function, migration utility, or the SAVE
service. Run the processor again.
12
The member’s accounting and dependency
information was retrieved successfully; however,
some of the dependency information failed
verification processing. To determine the nature
of the verification error, browse the member’s
accounting and dependency information by using
the library utility. The utility performs this
verification and displays the fields you want to
validate. Edit and then save the member to correct
the problem.
16
The specified group was not found in the
project definition. This error can occur when you
use alternate project definitions or when you
modify a project definition. Examine the project
definition for the missing group. Contact the
project manager.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
If the return code is:
16
Correct and reassemble the project definition.
Submit the job again, and verify that no errors
occurred.
20
Run IDCAMS against the accounting data set to
determine the problem.
FLM61020 NO MEMBERS MATCHING
SELECTION CRITERIA
SCLM messages
Chapter 3. SCLM messages  819

## Page 840

Explanation
SCLM could not find a match for project, group, type,
and member.
User response
Verify that the members matching the selection
criteria are under SCLM control.
FLM61021 DATABASE CONTENTS UTILITY
COMPLETED - aaaaaaaa ON
bbbbbbbb
Explanation
This message is provided for information only.
FLM61025 USER DEFINED DDNAME:
aaaaaaaa FOR DBUTIL REPORT
NOT ALLOCATED
Explanation
DDname aaaaaaaa, which was specified for the
DBUTIL report, was not allocated. If you invoke the
DBUTIL function using the services, the ddname for
the DBUTIL report is optional. If you do not specify the
ddname, the DBUTIL report is sent to the terminal by
default. If you specify a ddname, you must allocate it.
User response
Verify that the user-supplied ddname for DBUTIL
output is allocated. Submit the job again.
FLM61028 USER DEFINED DDNAME:
aaaaaaaa FOR TAILORED OUTPUT
NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
tailored output, was not allocated. If you invoke the
DBUTIL function using the services, the ddname for
the tailored output is optional. If you do not specify a
ddname, the tailored output is sent to the terminal by
default. If you specify a ddname, you must allocate it.
User response
Verify that the user-supplied ddname for the tailored
output is allocated. Submit the job again.
FLM61030 USER DEFINED DDNAME:
aaaaaaaa FOR DBUTIL MESSAGES
IS NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
messages, is not allocated. If you invoke the DBUTIL
function using the services, the ddname for the
messages is optional. If you do not specify a ddname,
the messages are sent to the terminal by default. If
you specify a ddname, you must allocate it.
User response
Verify that the user-supplied ddname for the messages
is allocated. Submit the job again.
FLM61035 TAILORED OUTPUT LINE LENGTH
EXCEEDS LIMIT
Explanation
The output line that is written to the tailored file
exceeded the 2048-character limit.
User response
Verify that the length of the lines being written as
output to the tailored file is not greater than 2048.
If it is greater than 2048, change your formatted
report line to contain SCLM variables that write 2048
characters or less to the tailored file.
FLM62000 ARCHITECTURE REPORT
PROCESSOR INITIATED -
aaaaaaaa ON bbbbbbbb
Explanation
Report processing has started. This message is
provided for information only.
FLM62001 STARTING ARCHITECTURE
MEMBER TYPE EXCEEDS CUTOFF
Explanation
The architecture report could not be generated
because the type of architecture member specified
exceeded the type of architecture definition given for
the cutoff of the report. The report cutoff should be
equal to or lower than the architecture member kind.
For information on architecture members, see z/OS
ISPF Software Config ur ation  and Library Manager
Guide and Reference.
User response
Specify a lower report cutoff and submit the job again.
FLM62002 ACCOUNTING INFORMATION FOR
MEMBER: aaaaaaaa TYPE:
SCLM messages
820  z/OS: z/OS ISPF Messages and Codes

## Page 841

bbbbbbbb IS IN INITIAL OR
LOCKOUT STATE.
Explanation
A lock has been placed on member aaaaaaaa but
changes to the member have not been registered with
SCLM. The source for the member either does not exist
or does not match the accounting information.
User response
If the member exists, use the SCLM editor or the
SCLM SAVE service to create the correct accounting
information.
FLM62008 INVALID CUTOFF PARAMETER:
aaa(24)
Explanation
The report cutoff for the architecture report is invalid.
User response
Verify that the report cutoff parameter specified is:
CC
For HL, LEC, and CC architecture members.
GEN
For HL, LEC, and generic architecture members.
HL
For HL architecture members.
LEC
For HL and LEC architecture members.
NONE
For all architecture members and source members
(no cutoff).
TOP SOURCE
For all top source members and all architecture
members.
FLM62010 REPORT DDNAME AND MESSAGE
DDNAME MUST BE DIFFERENT
Explanation
A function was invoked with the same ddname
specified for both the report file and the messages file.
User response
Specify different ddnames for the report file and the
messages file.
FLM62024 WARNING, MAXIMUM INDENTION
DEPTH EXCEEDED. REMAINDER
OF MEMBERS NOT INDENTED.
Explanation
The maximum indention depth has been exceeded
while processing members for the architecture report.
The architecture report continues; however, the
remainder of the members are not indented to indicate
included members.
FLM62025 USER DEFINED DDNAME:
aaaaaaaa FOR ARCHITECTURE
REPORT NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
architecture report, was not allocated. If you invoke
the architecture function using the services, the
ddname for the architecture report is optional. If
you do not specify a ddname, the architecture report
is sent to the terminal by default. If you specify a
ddname, you must allocate it.
User response
Verify that the user-supplied ddname for the
architecture report is allocated. Submit the job again.
FLM62030 USER DEFINED DDNAME:
aaaaaaaa FOR ARCHITECTURE
MESSAGES NOT ALLOCATED
Explanation
Ddname aaaaaaaa, which was specified for the
architecture messages, was not allocated. If you
invoke the architecture function using the services,
the ddname for the architecture messages is optional.
If you do not specify a ddname, the architecture
messages are sent to the terminal by default. If you
specify a ddname, you must allocate it.
User response
Verify that the user-supplied ddname for the
architecture messages is allocated. Submit the job
again.
FLM62104 INVALID STATEMENT IN
ARCHITECTURE MEMBER:
aaaaaaaa TYPE: bbbbbbbb
Explanation
The architecture member aaaaaaaa contains an
invalid statement. The architecture member might
contain keywords that are specific to both an LEC
and a CC (for example, an OBJ keyword and a LOAD
keyword). The architecture member might also have
two LMAP statements or a COPY keyword with an
SCLM messages
Chapter 3. SCLM messages  821

## Page 842

LMAP statement in the copied member. Any of these
errors can cause this message to occur.
User response
This error does not affect the architecture report.
Before attempting a build, check the specified
architecture member for any of the listed errors.
Modify the architecture member to correct the errors.
FLM62108 ERROR RETRIEVING
ACCOUNTING INFORMATION
CODE: aaa GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
No accounting information exists or could be retrieved
for member dddddddd.
User response
If the report was being run simply to view the high-
level architecture of the application, no response is
necessary. Otherwise, take the action recommended
for the return code received:
8
No accounting record exists for member
dddddddd. Register the member using the SCLM
editor, the migration utility, or the SAVE service.
Project manager response
If the return code is:
12
SCLM was unable to read the VSAM record
containing the member's accounting information.
Refer to the Access Method Services for help in
determining the VSAM error.
16
The specified group bbbbbbbb was not found in
the project definition. This error can occur when
an alternate project definition is used. The error
can also be caused by a change to the project
definition. Correct and reassemble the project
definition.
20
An I/O error occurred while attempting to read
an accounting record. This could also represent
an open error on the VSAM accounting data set.
Refer to the Access Method Services for help in
determining the VSAM error.
FLM62200 NUMBER OF VARIABLE EXCEEDS
THE MAXIMUM NUMBER OF
VARIABLES ALLOWED, REPORT
CONTINUES.
Explanation
SCLM cannot display all of the SCLM variables that the
user requested for the report.
User response
Check the variables specified and remove any
unnecessary ones, or break the report up into smaller
pieces.
FLM62900 ARCHITECTURE REPORT
PROCESSOR COMPLETED
Explanation
The architecture report processor finished executing.
User response
See the message data set for all the messages related
to the completion of the process.
FLM69005 INVOKING ARCHITECTURE
REPORT PROCESSOR
Explanation
This message is provided for information only.
FLM69010 INVOKING DATABASE CONTENTS
UTILITY
Explanation
This message is provided for information only.
FLM69015 THE REPORT WILL APPEAR IN
aaa(26)
Explanation
This message is provided for information only.
FLM69020 THE MESSAGES WILL APPEAR IN
aaa(26)
Explanation
This message is provided for information only.
FLM69025 THE TAILORED OUTPUT WILL
APPEAR IN aaa(26)
Explanation
This message is provided for information only.
SCLM messages
822  z/OS: z/OS ISPF Messages and Codes

## Page 843

FLM69030 DATABASE CONTENTS UTILITY
RETURN CODE = aaa.
Explanation
The Database Contents utility ended with a return
code of aaa. This message is provided for information
only.
FLM70002 PARAMETER STRING EXCEEDS
MAXIMUM SIZE ALLOWED FOR
TRANSLATOR aaaaaaaa
Explanation
The parameter string for the translator aaaaaaaa is
greater than the allowed maximum of 512 characters.
The parameter string is formed by concatenating the
values of the OPTIONS parameter of the FLMTRNSL
macro with the PARM and the PARMx architecture
member keywords. All SCLM variables in the resulting
string are then replaced with the variables’ values. Any
of the allowable sources for creation of the parameter
string could cause the parameter string size to be
exceeded.
User response
Reduce the size of the parameter string passed
through the PARM or the PARMx keyword in the
architecture definition if possible. Otherwise, contact
the project manager.
Project manager response
Reduce the size of the OPTIONS parameter string on
the FLMTRNSL macro for the specified translator.
FLM70003 SUBSTITUTION LIST EXCEEDS
MAXIMUM SIZE ALLOWED FOR
TRANSLATOR aaaaaaaa
Explanation
The ddname substitution list for translator aaaaaaaa
is greater than the maximum of 512 allowed.
Every FLMALLOC macro for the translator causes
an 8-character ddname to be put into the ddname
substitution list.
User response
Contact the project manager.
Project manager response
Either reduce the number of FLMALLOC macro
invocations for the specified translator or change the
PORDER parameter of the FLMTRNSL macro to 0 or
1 so that SCLM will not attempt to pass a ddname
substitution list.
FLM70101 INVALID COPYLIB NAME: aaa(44)
Explanation
The copylib name is too long.
User response
Contact the project manager.
Project manager response
The maximum size allowed is 44 characters. Reduce
the size of the copylib name to 44 characters or less to
allow for SCLM variable substitution if SCLM variables
are used.
FLM70102 UNSUPPORTED IOTYPE, UNABLE
TO ALLOCATE DATA
SET LANGUAGE aaaaaaaa
TRANSLATOR: bbbbbbbb DATA
SET: ccc IOTYPE: dddddddd
Explanation
An IOTYPE was specified in an FLMALLOC macro that
is not supported by the associated translator.
User response
Contact the project manager.
Project manager response
Correct the FLMALLOC statement in the language
definition for the specified language. See z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference for more information.
FLM70212 INVOKING COPY ROUTINE(S) FOR
LANGUAGE: aaaaaaaa
Explanation
Copy translator for language aaaaaaaa has been
called. This translator was defined in the language
definition. This message is provided for information
only.
FLM70214 INVOKING PURGE ROUTINE(S)
FOR LANGUAGE: aaaaaaaa.
Explanation
Purge routine for language aaaaaaaa has been called.
This program was defined in the language definition.
This message is provided for information only.
SCLM messages
Chapter 3. SCLM messages  823

## Page 844

FLM70216 INVOKING TRANSLATORS FOR
LANGUAGE: aaaaaaaa
Explanation
Translator for language aaaaaaaa has been called.
This message is provided for information only.
FLM70501 ERROR COPYING ALLOCATION
DATA SET aaa FOR TRANSLATOR
bbb TO LISTINGS DATA SET, CODE:
ccc
Explanation
One of these might have occurred:
• The listing data set was not allocated.
• The listing data set had an insufficient amount of
space allocated.
Possible return codes are:
12
There is no data to be copied to the listing data
set or the data has already been deleted by the
translator.
16
The output data set is full.
20
Data access failed due to:
• data set security protection
• Input member was not found.
24
The input parameter is invalid.
28
Data sets with a RECFM value of U cannot be
copied to the listings data set.
User response
If the return code is:
12
Verify that the translator really did create the file
and did not delete it. Try running the translator
outside SCLM and verify that the file is created.
16
Allocate a larger listings data set.
20
Verify that you have access authority to the listings
data set. Verify that the translator really did create
the file and did not delete it. Try running the
translator outside SCLM and verify that the file is
created.
24
Report this message (including the message ID
and all text fields) to IBM support.
28
Correct the record format or change the target data
set.
FLM70502 LISTINGS NOT COPIED BECAUSE
BLANK LISTINGS DDNAME
SPECIFIED
Explanation
SCLM did not copy listings to the listings data set
because you specified a blank ddname. Therefore, you
will not be able to see any listings that the translators
produced.
User response
Specify the listings ddname for the given function and
run the function again.
FLM70600 DATE/TIME VERIFICATION FAILED
FOR GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The VERIFY translator was invoked as part of Build
or Promote verification, and returned a date/time
indicating that the member cccccccc is out of date.
User response
Verify that the date is valid and submit the job again.
FLM70601 VERIFICATION TRANSLATOR
FAILED FOR LANGUAGE:
aaaaaaaa
Explanation
The return code from the verification translator for
language aaaaaaaa exceeded the allowable maximum
good return code.
User response
Determine the reason for the bad return code from the
verification translator and take appropriate action.
FLM70801 ERROR DEALLOCATING DATA SET
NUMBER: aaa FOR LANGUAGE:
cccccccc TRANSLATOR: bbb (16)
CODE: ddd
SCLM messages
824  z/OS: z/OS ISPF Messages and Codes

## Page 845

Explanation
An error occurred while deallocating a data set for
the translator bbb (16). The data set number aaa
identifies the allocated data set for that translator.
This message indicates that a data set for one of the
translators defined for the cccccccc language could not
be deallocated. Because the condition occurs during
cleanup, it can usually be treated as a warning. Verify
that the program has not deallocated the data sets
specified for the language.
Note: For the FREE and END services called using a
program, only the first line will appear.
Possible return codes are:
12
SCLM internal error. Report this message (including
the message ID and all text fields) to IBM support.
16
Missing or incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
was not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including message ID and all text
fields) to IBM support.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
Data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
Submit the job again. If the error recurs, report this
message (including the message ID and all text fields)
to IBM support.
FLM71002 ERROR INVOKING TRANSLATOR:
aaa(16)
Explanation
SCLM could not invoke translator aaa(16). The load
module containing the translator might be allocated
exclusively to another job, or there is an error in the
language definition that defines the translator.
User response
If the translator has been used successfully in the past
and changes were not anticipated (for example, a new
compiler release), invoke the processor again. If the
translator is new or the problem recurs, report this
message (including the message ID and all text fields)
to IBM support.
FLM71004 TRANSLATOR RETURN CODE
FROM ===> aaa(16) ===> bbbb
Explanation
The return code from the invoked translator did
not match the GOODRC parameter specified for the
translator. Translator output, such as compiler listings,
will be saved in the listings data set for the processor if
requested in the language definition.
User response
Use the listings data set to locate and correct all errors
identified by the translator. If the specified return code
is acceptable for the translator, contact the project
manager.
Project manager response
Change the GOODRC parameter of the FLMTRNSL
macro, which defines the specified return code, in the
project definition.
FLM71006 ERROR ALLOCATING DATA SET
FOR BUILD BY CHANGE CODE:
DSN: aaaaaaaa TYPE: bbb CODE:
ccc
Explanation
An error occurred while attempting to allocate
the indicated data set. The error occurred while
processing members to be included from type bbb.
Possible return codes are:
12
Internal error. Report this message (including the
message ID and all text fields) to IBM support.
SCLM messages
Chapter 3. SCLM messages  825

## Page 846

16
Incorrect data set name.
20
Invalid file attribute specified.
24
A member of a PDS was requested but the data set
is not partitioned.
28
The requested member could not be found.
32
The requested member was not available.
36
SCLM internal error (device unit missing). Report
this message (including the message ID and all
text fields) to IBM support.
X'xxxx'
SVC 99 error reason code (in hexadecimal). Some
possible reason code values are:
X'0210'
Requested data set unavailable. The data set
is allocated to another job and its usage
attributes conflict with this request.
X'1708'
The data set does not exist.
X'97xx'
SMS error code. This will be followed by SMS:
ddddd; where ddddd is the IGD message
number associated with the error.
User response
Submit the job again. If the error recurs, report this
message (including the message ID and all text fields)
to IBM support.
Project manager response
For reason codes of the form X'xxxx', refer to the
z/OS MVS Programming: Authorized Assembler Services
Guide for a description of the SVC 99 reason codes (or
see Appendix A of the ISPF Tutorial). For SMS error
codes, the IGD message can be located in z/OS MVS
System Messages, Vol 8 (IEF-IGD). The listing may have
one or two leading zeros added to the ddddd value.
Contact IBM support for assistance.
FLM71010 ERROR: FLMTCOND macro
keywords GROUP and NOTGROUP
specified together. FLMTCOND
follows FLMTRNSL with
COMPILE=aaaaaaaa
Explanation
aaaaaaaa is the COMPILE keyword value on the
FLMTRNSL associated with this FLMTCOND macro.
This error should be trapped by the FLMTCOND macro
and should not occur at runtime.
User response
See the project administrator for correction of the
project definition.
User response
Remove GROUP or NOTGROUP in the FLMTCOND
macro. The FLMTCOND macro will follow an
FLMTRNSL macro with COMPILE=aaaaaaaa in a
language definition. Reassemble and link the project
definitions that use the language definition.
FLM71011 ERROR: Previous translator
label not found. Translator
label=aaaaaaaa
Explanation
aaaaaaaa is the label in the FLMTCOND WHEN
keyword value that is not found on any previous
FLMTRNSL macros for build translators in the language
definition.
User response
See the project administrator for correction of the
project definition.
User response
Edit the language definition and adjust the FLMTRNSL
label and/or the label referenced by the WHEN
keyword. Reassemble and link the project definitions
that use the language definition.
FLM71012 ERROR: FLMTCOND macro
keyword WHEN syntax invalid.
FLMTCOND follows FLMTRNSL
with COMPILE=aaaaaaaa.
Explanation
aaaaaaaa is the value of the COMPILE keyword on the
FLMTRNSL associated with the FLMTCOND macro. The
value of the WHEN keyword does not contain valid
syntax.
User response
See the project administrator for correction of the
project definition.
SCLM messages
826  z/OS: z/OS ISPF Messages and Codes

## Page 847

User response
Edit the language definition and correct the WHEN
keyword value on the FLMTCOND macro following
the FLMTRNSL with COMPILE=aaaaaaaa. Reassemble
and link the project definitions that use the language
definition.
FLM72001 INVALID OR MISSING VALUE FOR
PARAMETER: aaaaaaaa
Explanation
The parameter aaaaaaaa has a invalid or missing
value. This message is issued by the FLMTXFER
translator.
User response
Contact the project manager.
Project manager response
Update the call to the FLMTXFER translator to ensure
that all required parameters are passed and that all
parameters have valid values.
FLM72002 NO DATA SET ALLOCATED TO DD
NAME: aaaaaaaa
Explanation
The ddname aaaaaaaa is not allocated, but is required
for the FLMTXFER translator.
User response
Contact the project manager.
Project manager response
Update the call to the FLMTXFER translator to ensure
that all required ddnames are allocated. The ddname
can be a ddname used directly by the FLMTXFER
translator, or it can be a ddname that was specified
in the input data passed in the FILES ddname.
FLM72003 INVALID TRANSFER TYPE: a LINE
NUMBER: bbbbb
Explanation
The transfer type a is not a valid transfer type. The
valid transfer types are A (ASCII), and B (BINARY). The
invalid transfer type was specified on line bbbbb of the
FILES DD input to the translator.
User response
Contact the project manager.
Project manager response
Ensure that the transfer types passed in the FILES
ddname to the FLMTXFER translator are valid.
FLM72004 NO TRANSFER SOURCE
SPECIFIED LINE NUMBER: aaaaa
Explanation
No source data set, ddname, or file was specified for
the transfer. Line aaaaa of the FILES DD input to the
translator is missing the source.
User response
Contact the project manager.
Project manager response
Ensure that the source of the transfer is specified on
each line in the FILES DD input to the FLMTXFER
translator.
FLM72005 INVALID SOURCE DATA SET
NAME: aaa(56) LINE NUMBER:
bbbbb
Explanation
The source data set name for the transfer is invalid.
Possible reasons for an invalid data set name are:
• The data set name is longer than 56 characters
• There is no ending quote on the data set name
• There are no characters between the beginning and
ending quotes.
User response
Contact the project manager.
Project manager response
Ensure that only valid data set names are being
specified in the FILES DD input to the FLMTXFER
translator.
FLM72008 INVALID DD NAME OR MEMBER
NAME: aaaaaaaa LINE NUMBER:
bbbbb
SCLM messages
Chapter 3. SCLM messages  827

## Page 848

Explanation
Line bbbbb contains an invalid specification for a
ddname or member name. Possible causes are:
• The member name is too long
• The ddname is too long
• a colon was specified before the member name
without a ddname being specified.
User response
Contact the project manager.
Project manager response
Update the call the FLMTXFER to ensure that valid
statements are specified in the FILES ddname input.
FLM72009 INVALID SYNTAX IN SOURCE
NAME: aaa(56) LINE NUMBER:
bbbbb
Explanation
Line bbbbb contains an invalid specification for the
transfer source. Possible causes are:
• specifying a data set name without the surrounding
quotes
• specifying a ddname without the required colon
• specifying an SCLM member without the required
period.
User response
Contact the project manager.
Project manager response
Update the call the FLMTXFER to ensure that valid
statements are specified in the FILES ddname input.
FLM72010 NO TRANSFER DESTINATION
SPECIFIED LINE NUMBER: aaaaa
Explanation
Line aaaaa contains no target specification for the
transfer.
User response
Contact the project manager.
Project manager response
Ensure that each statement in the FILES ddname input
contains a valid transfer destination.
FLM72011 TRANSFER MEMBER NOT IN
BUILD SCOPE MEMBER: aaaaaaaa
TYPE: bbbbbbbb
Explanation
Member aaaaaaaa in type bbbbbbbb is not in the
scope of the build and will not be transferred.
User response
Contact the project manager.
Project manager response
Ensure that each statement in the FILES ddname
input that specifies an SCLM member name uses
only members in the build scope. The FLMTBMAP
translator can be used to obtain a list of inputs and
outputs for a specific build.
FLM72012 UNABLE TO OBTAIN DATASET
NAME FOR DD NAME: aaaaaaaa
Explanation
The FLMTXFER translator could not obtain the data set
name for ddname aaaaaaaa.
User response
Contact the project manager.
Project manager response
Ensure that the ddname is being allocated to a
cataloged data set.
FLM72013 FILE TRANSFER FAILED, RETURN
CODE: aaaa, FROM FILE:
bbbbbbbb, TO FILE: cccccccc
Explanation
The ISPF FILEXFER service failed with a return code of
aaaa in the transfer of file bbbbbbbb to file cccccccc.
User response
Contact the project manager.
Project manager response
Refer to the ISPF FILEXFER service documentation
for an explanation of return code aaaa and additional
responses.
FLM75001 INVALID OR MISSING DATA FOR
INPUT PARAMETER aaaaaaaa
SCLM messages
828  z/OS: z/OS ISPF Messages and Codes

## Page 849

Explanation
In the Language Definition the input parameter
aaaaaaaa is missing or is not valid.
User response
Contact the project manager.
Project manager response
Verify that the Language Definition contains all the
input parameters, and that the parameters have the
proper values.
FLM75002 ERROR: aaaaaaaa TAG NOT
FOUND IN PROXY FILE
Explanation
The aaaaaaaa tag was not found in the proxy file.
User response
Verify that the CSP/370AD proxy contains a BUILD/
EBUILD tag section. Optionally, if you have EZE and
GROUP tags then they should have a corresponding
EEZE or EGROUP tag.
FLM75003 ERROR: aaaaaaaa NOT SPECIFIED
IN PROXY FILE
Explanation
The aaaaaaaa element was not found in the proxy file.
User response
Verify that the CSP/370AD proxy contains the
parameter MEMBER(name) for the CSP/370AD*
GENERATE command.
FLM75004 ERROR: COULD NOT OPEN
DDNAME = aaaaaaaa, PASCAL
RUN-TIME ERROR = bbb
Explanation
The ddname aaaaaaaa could not be opened.
User response
Verify that the DDNAME = aaaaaaaa exists in your
system. The codes for the PASCAL run-time error
bbb are described in the VS Pascal Application
Programming Guide A common error code is:
• 048 - Missing member in File: member_library
FLM75006 WARNING: THE FOLLOWING
PARAMETERS FOR THE
CSP/370AD GENERATE
COMMAND WILL BE
USED: SYSTEM(TARGET_SYSTEM),
MAPS(NONE), TABLES(NONE),
AND BATCH(N). THE
CORRESPONDING PARAMETERS
IN THE PROXY FILE WILL BE
IGNORED.
Explanation
SCLM will append the CSP/370AD GENERATE
command with a list of parameters that use specific
values. If the user specifies one of these parameters,
they will be ignored, and the SCLM specified ones
will be used instead. The value for 'target_system' in
SYSTEM(target_system) is obtained from the SYSTEM=
value of the OPTIONS parameter for the FLMTRNSL
macro. This is the FLMTRNSL macro for the CSP/
370AD Lista/Generate translator for the language
definition associated with the CSP/370AD proxy
member.
User response
Delete from the Proxy file any of the parameters
mentioned in the message.
FLM75007 END OF PROXY FILE REACHED
BEFORE THE TAG aaaaaaaa WAS
FOUND.
Explanation
The tags in the CSP/370AD proxy file must come
in pairs; for example, the :BUILD tag must have
an :EBUILD tag. This message indicates that a
corresponding end tag is missing.
User response
Ensure that the tags are paired, such as :BUILD
with :EBUILD.
FLM75008 TAG NOT VALID OR NESTED TAGS
FOUND IN PROXY FILE.
Explanation
Either a tag is not a valid one (probably due to
misspelling) or there are nested tags (which are not
allowed).
User response
Ensure that the tags are valid ones, and that there are
no nested tags.
SCLM messages
Chapter 3. SCLM messages  829

## Page 850

FLM75009 ERROR: BUFSIZE PARAMETER IN
FLMLANGL IS aaaa. IT SHOULD BE
AT LEAST bbbb.
Explanation
The BUFSIZE parameter is too small in the FLMLANGL
macro of the language definition associated with the
CSP/370AD proxy member being built.
User response
Contact the project manager.
Project manager response
Ensure that the value of BUFSIZE is at least bbbb.
Then reassemble the project definition.
FLM75010 ALLOCATION FAILED FOR
aaaaaaaa, DDNAME = bbbbbbbb,
DSNAME = cccccccc
Explanation
The ddname = bbbbbbbb could not be allocated to the
cccccccc DSNAME, while allocating aaaaaaaa or the
ddname could have been allocated to another dsname.
User response
Contact the project manager.
Project manager response
Verify that both the ddname and the dsname are valid.
Also verify that the dsname has not been allocated to
another DSNAME.
FLM75011 GROUP aaaaaaaa NOT FOUND IN
SCLM HIERARCHY TABLE
Explanation
The GROUP = aaaaaaaa does not exist in the SCLM
hierarchy table.
User response
Contact the project manager.
Project manager response
Verify that the GROUP is a valid one.
FLM75012 WARNING: NOT ALL MSLS IN MSL
CONTROL FILE WILL BE USED.
Explanation
SCLM concatenates the CSP/370AD MSLs described
in the MSL Control File (project.PROJDEFS.MSLCTRL)
according to the hierarchy defined in SCLM. The
Library MSLs are then appended at the end of this
concatenation of MSLs; if the total number of read/
only MSLs is greater than 5, this warning message
is issued (but SCLM does not truncate the MSLs
concatenation), because only the first 5 read/only
MSLs will be accepted by CSP/370AD.
User response
Contact the project manager.
Project manager response
Verify that the longest concatenation of MSLs does not
exceed 6.
FLM75013 TOO MANY CSP/370AD WORK
FILES IN MSL CONTROL FILE.
Explanation
There are more than 6 work files in the MSL Control
File (project.PROJDEFS.MSLCTRL).
User response
Contact the project manager.
Project manager response
Verify that there are no more than 6 work files in the
MSL Control File.
FLM75014 NUMBER OF MSL RECORDS IN
THE MSL CONTROL FILE LESS
THAN NUMBER OF GROUPS IN
THE PROJECT DEFINITION.
Explanation
In the MSL Control File (project.PROJDEFS.MSLCTRL)
the number of groups in the SCLM hierarchy should be
the same as the number of MSLs in CSP/370AD.
User response
Contact the project manager.
Project manager response
Verify that the groups in SCLM match
the MSLs specified in the MSL Control
File(project.PROJDEFS.MSLCTRL).
SCLM messages
830  z/OS: z/OS ISPF Messages and Codes

## Page 851

FLM75015 READ/WRITE MSL IS NOT
SPECIFIED.
Explanation
A valid Read/Write MSL was not specified for
the current SCLM group in the MSL Control File
(project.PROJDEFS.MSLCTRL).
User response
Contact the project manager.
Project manager response
Specify a Read/Write MSL.
FLM75016 THE FOLLOWING CSP/370AD
COMMAND WAS UNSUCCESSFUL:
aaaaaaaa
Explanation
The CSP command aaaaaaaa was unsuccessful.
User response
Verify the CSP/370AD log and output to determine the
cause of the problem. Then, take any required action
to fix the problem.
FLM80001 'END' RECORD NOT FOUND IN THE
"ACCOUNTING LIST INFO ARRAY"
Explanation
The accounting $list_info array has exceeded its buffer
size.
User response
Contact the project manager.
Project manager response
Increase the size of the accounting $list_info array
defined for the language on the FLMLANGL macro.
For more information on how to specify the size
of the accounting $list_info array, see the topic
about the FLMLANGL macro in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM80002 INVALID RECORD TYPE FOUND
IN THE "ACCOUNTING LIST INFO
ARRAY" RECORD TYPE: aaaaaaaa
Explanation
The record type is unknown.
User response
Contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained an invalid
record type. If a parser was used to create the array,
check that the passed values are correct. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM80003 INVALID COMPOOL NAME FOUND
IN THE "ACCOUNTING LIST INFO
ARRAY" RECORD KIND: aaaa
COMPOOL NAME: bbbbbbbb
Explanation
The accounting $list_info array contained an entry for
a compool with either an invalid or blank associated
compool name.
User response
If a parser was used, parse the members in question
again. If the same error occurs or a parser was not
used, contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained an invalid
or blank compool name. If a parser was used
to create the array, check that the passed values
are correct. For more information, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM80004 INVALID INCLUDE NAME FOUND
IN THE "ACCOUNTING LIST INFO
ARRAY" RECORD KIND: aaaa
INCLUDE NAME: bbbbbbbb
Explanation
The accounting $list_info array contained an entry for
an include with either an invalid or blank associated
include name.
SCLM messages
Chapter 3. SCLM messages  831

## Page 852

User response
If a parser was used, parse the members in question
again. If the same error occurs or a parser was not
used, contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained an invalid
or blank include name. If a parser was used to
create the array, check that the passed values
are correct. For more information, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM80005 INVALID CU DATA FOUND IN THE
"ACCOUNTING LIST INFO ARRAY"
RECORD KIND: aaaa
CU NAME: bbb(55) ccc(55)
CU TYPE: d GENERIC FLAG: e
DEPEND NAME: fff(55) ggg(55)
DEPEND CU TYPE: h
DEPENDENCY TYPE: i
Explanation
The accounting $list_info array contained invalid data
for a compilation unit (CU).
User response
If you used a parser, parse the members in question
again. If the same error occurs or you did not use a
parser, contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained the invalid
CU data. If a parser was used to create the array,
check that the passed values are correct. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM80006 AN EXTERNAL DEPENDENCY HAS
A BLANK NAME
Explanation
An external program is using the SCLM LIST_INFO
feature to return dependency information. The
LIST_INFO record contains a blank name.
User response
Determine the external program name and define a
valid dependency name.
FLM80007 INVALID INCLUDE SET NAME
FOUND IN THE "ACCOUNTING
LIST INFO ARRAY" RECORD KIND:
aaaa INCLUDE SET
NAME: bbbbbbbb
Explanation
The accounting $list_info array contained an include
record that referenced include set bbbbbbbb. This
include set is not defined in the language definition
of the member being saved.
User response
Notify the project manager of the problem.
Project manager response
Either update the language definition to include an
FLMINCLS macro for the include set, or update the
parser not to use include set bbbbbbbb in include
records.
FLM80008 INCONSISTENT DATE AND TIME
RETURNED FOR EXTERNAL
DEPENDENCY GROUP: aaaaaaaa
TYPE: bbbbbbbb NAME: c(43)
REFERENCED BY THE FOLLOWING
BUILD MAPS:
Explanation
More than one verify translator returned a date and
time for this external dependency. The dates and
times returned by the translators are not the same.
User response
Verify that the external dependency was not updated
between calls to the verify translators. If the external
dependency has changed, rebuild. If the external
dependency has not changed, contact the project
manager to determine why the translators are
returning different dates and times.
Project manager response
If different verify translators are returning information
for the same external dependency, verify that the
same date and time is being returned by each
translator. Refer to message FLM80009, which will
accompany this message, for the list of languages with
verify translators that are returning dates and times for
this external dependency.
FLM80009 aaaaaaaa bbbbbbbb AT GROUP:
cccccccc LANGUAGE: dddddddd
SCLM messages
832  z/OS: z/OS ISPF Messages and Codes

## Page 853

Explanation
This message lists the build maps referencing the
external dependency in message FLM80008. The build
map member name is aaaaaaa in type bbbbbbbb.
User response
Refer to message FLM80008.
Project manager response
Refer to message FLM80008.
FLM80010 CONFLICTING GENERIC FLAGS
FOUND FOR THE SAME CU IN THE
"ACCOUNTING LIST INFO ARRAY"
RECORD KIND: aaaa
CU NAME: bbb(55) ccc(55)
CU TYPE: d
Explanation
Dependencies for the same CU have different generic
flags. The generic flags must always be the same for
all dependencies within a CU.
User response
Contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained the invalid
CU data. If a parser was used to create the array,
check that the passed values are correct. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM80011 CONFLICTING DEPENDENCY
TYPE FLAGS FOUND IN THE
"ACCOUNTING LIST INFO ARRAY"
RECORD KIND: aaaa
CU NAME: bbb(55) ccc(55)
CU TYPE: d GENERIC FLAG: e
DEPEND NAME: fff(55) ggg(55)
DEPEND CU TYPE: h
Explanation
Different type flags exist for the same CU within the
accounting $list_info array. For example, the same CU
is specified as a SPEC in one instance and a BODY in
another.
User response
Contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contained the invalid
CU data. If a parser was used to create the array,
check that the passed values are correct. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM80012 ONLY ONE CU RECORD MAY BE
PRESENT IN THE "ACCOUNTING
LIST INFO ARRAY"
Explanation
If a CU record with a type of X exists in the accounting
$list_info array, it must be the only CU record in the
array.
User response
Contact the project manager.
Project manager response
Either the parser created or the user defined an
accounting $list_info array that contains the invalid
CU data. If a parser was used to create the array,
check that the passed values are correct. For more
information, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM80020 ERROR ALLOCATING THE CHANGE
CODE VERIFICATION ROUTINE
DATA SET DSNAME: aaa(44)
Explanation
Data set aaa(44) could not be allocated. The data set
might not exist or it might be allocated exclusively to
another user or job.
User response
Perform one of these actions, depending on which is
appropriate:
• Allocate the required data set and move the change
code verification routine into it.
• Free the data set so that it can be allocated in SHR
mode.
FLM80021 ERROR INVOKING THE CHANGE
CODE VERIFICATION ROUTINE
NAME: aaaaaaaa DSNAME:
bbb(44)
SCLM messages
Chapter 3. SCLM messages  833

## Page 854

Explanation
The change code verification routine aaaaaaaa could
not be invoked.
User response
Verify that the routine exists within data set bbb(44). If
the routine does not exist, move it into the proper data
set. If it does exist, report this message (including the
message ID and all text fields) to IBM support.
FLM80022 INVALID CHANGE CODE:
aaaaaaaa CHANGE CODE
VERIFICATION ROUTINE RETURN
CODE: bbb
Explanation
The change code verification routine completed with a
return code > 0.
User response
Contact the project manager.
Project manager response
Check your change code verification routine for return
code explanations. For more information on change
code verification routine specification, see the topic
about "User Exits" in z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
FLM80030 THE MAXIMUM SIZE OF THE
"ACCOUNTING LIST INFO ARRAY"
HAS BEEN EXCEEDED
Explanation
The accounting $list_info array has insufficient space
to contain the data specified. Dependency information,
user data records, and change code information must
fit into the array.
User response
If possible, eliminate unneeded user data or change
code information from the accounting record using the
SCLM library utility. If all of the information is required,
contact the project manager.
Project manager response
Increase the size of the accounting $list_info array
defined for the language on the FLMLANGL macro.
For more information on how to specify the size
of the accounting $list_info array, see the topic
about the FLMLANGL macro in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM80031 "ACCOUNTING LIST INFO ARRAY"
MUST ONLY CONTAIN CHANGE
CODE RECORDS RECORD TYPE:
aaaa
Explanation
An invalid record was found in the accounting
$list_info array. This message appears when the SAVE
service is invoked using the FLMLNK interface. The
SAVE service accepts only change code records in the
$list_info array; however, a record with type aaaa was
passed as input.
User response
If no change codes are to be added, set the $list_info
parameter to hex zeros. If change codes are to be
added, an END record must appear after the last
change code to terminate the list. Refer to the SAVE
service for more information. If you received this
message but you were not invoking the SAVE service
using FLMLNK, contact the project manager.
Project manager response
Report this message (including the message ID and all
text fields) to IBM support.
FLM80035 A NON-BLANK CHANGE CODE IS
REQUIRED
Explanation
The project definition indicates that change code
verification is in effect, but no change code was
specified.
User response
Provide a non-blank change code.
Project manager response
Ensure that a change code verification routine is
needed for the project. If so, inform all users that
they are required to enter change codes. In addition,
provide a list of valid values to your users.
FLM80500 ACCESS KEY INCORRECT ACCESS
KEY: aaa(16)
GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
SCLM messages
834  z/OS: z/OS ISPF Messages and Codes

## Page 855

Explanation
Access key aaa(16) was invalid. Member dddddddd is
locked out with an access key. The member can be
saved in the SCLM hierarchy only if the correct access
key is specified.
User response
If another user has the member checked out, wait until
it is checked in. Otherwise, specify the correct access
key for the member. You can check the value of the
access key by browsing the accounting record with the
SCLM Library Utility option (Option 3.1). If you wish
to delete member ddddddd, use the DELETE service
specifying the proper access key.
FLM81001 INVALID APPLICATION ID:
aaaaaaaa
Explanation
An INIT or END operation was attempted with an
invalid application ID (aaaaaaaa) specified.
User response
Make sure that the application ID passed back from
the START function is used in the INIT and END
functions.
FLM81201 INVALID PROJECT IDENTIFIER:
aaaaaaaa
Explanation
An invalid project identifier (aaaaaaaa) was passed to
an SCLM service. A valid project identifier is required
by the SCLM service requested.
User response
Supply a valid project identifier in the SCLM service
parameter list.
FLM81202 INVALID PROJECT DEFINITION
NAME: aaaaaaaa
Explanation
An invalid project definition name (aaaaaaaa) was
passed to an SCLM service. A valid project definition
name is required by the SCLM service requested.
User response
Supply a valid project definition name in the SCLM
service parameter list.
FLM81203 MAXIMUM SCLM ID LIMIT
EXCEEDED
Explanation
No more SCLM IDs are available at this time.
User response
Free some previously allocated SCLM IDs. To do this,
use the FREE service to free the SCLM IDs, and the
END service will open SCLM IDs associated with a
given application. You can also free SCLM IDs by
backing out of all active SCLM dialogs.
FLM81204 ERROR INITIALIZING THE
PROJECT DEFINITION, CODE: aaa
Explanation
The project definition could not be initialized.
User response
Possible return codes are:
4
The specified project definition load module is
not RMODE(24). Generate the project definition
load module again and specify the RMODE(24)
parameter to the linkage editor.
8
An error occurred while attempting to obtain the
specified project definition or alternate project
definition. Verify the project or alternate project
definition.
12
The project definition is out of date. Reassemble
the project definition with new SCLM macros.
Submit the job again. Also, see the section
"Running different versions of SCLM in multiple
partitions" in the SCLM Guide and Reference.
16
The project name specified does not match the
project name in the project definition. Verify
that the project name (on the FLMABEG macro)
specified and the project name in the project
definition are the same.
Another problem might be that SCLM is being
invoked with an alternate project definition name,
and this alternate project definition has the same
name but different contents as another alternate
project definition name that is being used. This can
occur only with two different project names and
when SCLM is being invoked from two different
sessions (such as with a split screen). To alleviate
SCLM messages
Chapter 3. SCLM messages  835

## Page 856

the problem, cancel out of one session and rename
the alternate project definition.
20
An attempt to open or close the project definition
failed. Browse the project definition data set
('project_id.PROJDEFS.LOAD'). Select the member
whose name matches the alternate you are using,
or the project ID, if the alternate is blank. If the
member appears, close the Browse panel and
submit the job again.
24
The project definition data set could not be
allocated. Verify that the project definition data set
exists and is not allocated exclusively by another
user. For more information about allocating the
PROJDEFS data sets, see z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference.
24
Project may not access the Account file. Check
SCLM security setup.
28
Data set security is active for this project, but you
do not have authority to update the accounting,
versioning, or cross reference VSAM database.
32
Subproject security is active for this project, but
there are no subprojects defined. The project
administrator should use the FLMPROJ macro
to define subprojects. The XFACILIT resources
should define access to the subprojects.
FLM81206 INVALID DATA SET NAME
PATTERN DETECTED, GROUP:
aaaaaaaa, CODE: bbb
Explanation
An invalid data set name pattern was detected for
group aaaaaaaa.
User response
Contact the project manager.
Project manager response
Possible return codes are:
8
An invalid data set name was generated. Possible
reasons for this failure are:
• The length of the data set name is greater than
44 characters.
• The SCLM variable @@FLMTYP is missing on the
DSNAME parameter being used by the specified
group. The variable @@FLMTYP must be used
with each dsname parameter. Determine which
FLMALTC or FLMCNTRL macro is being used by
the group and add @@FLMTYP to the pattern
specified by the DSNAME parameter.
FLM81302 ERROR PROCESSING
MEMBERS WITH ACCOUNTING
INFORMATION TYPE: INITIAL
Explanation
During cleanup activities, SCLM changes the members
that are in an INITIAL state to a LOCKOUT state.
If the initial accounting record cannot be found in
the database, or if an error is encountered while
attempting to update the accounting record, SCLM
issues this message. This error can be caused by
concurrent access of a member in the database (for
example, using split screens to work with a member).
User response
Use SCLM option 3.1 to browse the accounting record
and verify the state of the member. If an accounting
record is not present for the member and this result
was unexpected, submit the job again to correct the
problem. If an accounting record is present but its
accounting type is INITIAL, delete the record and
submit the job again.
FLM82002 MEMBER IS NON-EDITABLE
GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The specified member cannot be edited because it is
SCLM output. This member cannot be updated in this
way.
User response
SCLM does not allow you to edit non-editable
members. If you need to edit a non-editable member,
create a new member and copy the non-editable data
into the new member. You cannot copy the modified
data in the new member back into the original non-
editable member within SCLM.
FLM82003 MEMBER IS NOT LOCKED GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc
SCLM messages
836  z/OS: z/OS ISPF Messages and Codes

## Page 857

Explanation
The UNLOCK service was called but member cccccccc
was not locked. If this error occurred while you were
using the STORE service, no accounting information
was available. If this error occurred while you were
using the editor, the accounting information created at
the beginning of your edit session was lost.
User response
For the STORE service, verify that the LOCK service
completed successfully before calling the STORE
service. For the UNLOCK service, this message is
a warning and can be ignored. If you are in an
edit session, your data has been saved. However,
the accounting information is lost. Cancel this edit
session and edit the member again. Then issue the
SAVE command immediately to establish accurate
accounting information.
FLM82004 LANGUAGE: aaaaaaaa CANNOT
BE USED FOR EDITABLE MEMBERS
Explanation
The language aaaaaaaa specified to the PARSE
routine is not a valid language. Check the list of valid
languages in the project definition.
User response
Use a language that is in the project definition. If the
language that you need is not in the project definition,
contact the project manager.
Project manager response
Add the required language to the &libdef in the form
of a language definition and reassemble. For more
information on language definitions, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM82005 INPUT PARAMETER
"ERROR_LISTINGS_ONLY" MUST
BE 'Y' OR 'N'
ERROR_LISTINGS_ONLY: a
Explanation
An invalid parameter was passed to an SCLM service.
User response
Supply the valid parameter in the SCLM service
parameter list.
FLM82006 INPUT PARAMETER "VERIFY_CC"
MUST BE 'Y' OR 'N', VERIFY_CC: a
Explanation
An invalid parameter was passed to an SCLM service.
User response
Supply the valid parameter in the SCLM service
parameter list.
FLM82008 INPUT PARAMETER
"SUB_DRAWDOWN_MODE" MUST
BE 'C' OR 'U'
SUB_DRAWDOWN_MODE: a
Explanation
An invalid parameter was passed to an SCLM service.
User response
Supply the valid parameter in the SCLM service
parameter list.
FLM82203 THE MEMBER HAS ACCOUNTING
INFORMATION WITH TYPE:
EDITABLE AUTHORIZATION CODE
CANNOT BE UPDATED GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
The authorization code specified does not match the
authorization code already assigned to the member.
User response
Use the SCLM library utility to change the authorization
code.
FLM82301 EDITABLE MEMBER’S ACCESS KEY
IS BLANK GROUP: aaaaaaaa
TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
You tried to unlock a member that has an editable
accounting record and the access key was already
blank.
User response
If you want to unlock the member rather than just
reset the access key, use the DELETE service to delete
the member’s accounting record.
SCLM messages
Chapter 3. SCLM messages  837

## Page 858

FLM82401 ERROR PROCESSING SYSTEM
LIBRARIES FOR PARSING CODE:
aaa LANGUAGE: bbbbbbbb
ERROR DSNAME: ccc(44)
Explanation
SCLM was unable to allocate the system library
defined in the language definition for the language
bbbbbbbb.
User response
Check that the system libraries specified in the
language definition exist and are not allocated
exclusively. If one or more do not exist, contact the
project manager.
Project manager response
Remove the invalid system libraries from the language
definition and regenerate the project definition.
FLM82402 LANGUAGE IS DEFINED FOR
NON_EDITABLE MEMBERS ONLY
'LANGUAGE: bbbbbbbb
Explanation
You assigned a non-editable language to a member of
an editable type.
User response
Contact your project coordinator for the correct
language to assign to this member.
FLM82403 UNABLE TO ALLOCATE MACRO
LIBRARIES FOR PARSING, CODE:
aaa LANGUAGE bbbbbbbb
Explanation
SCLM was unable to allocate the macro libraries
associated with the parser for language bbbbbbbb.
User response
Verify that the user has access to the macro libraries
specified in the language definition.
12
Unable to read the directory for the system macro.
20
Unable to open one of the macro libraries.
Project manager response
Verify that the user has access to the macro libraries
defined for the language.
Possible return codes are:
12
Unable to read the directory for the system macro.
20
Unable to open one of the macro libraries.
FLM82501 EXISTING CU'S AUTHORIZATION
CODE NOT DEFINED TO GROUP
CU NAME: aaa(55) bbb(55) CU
TYPE:cccc
CU QUALIFIER: dddddddd
GROUP: eeeeeeee
ERROR GROUP: ffffffff
AUTHORIZATION CODE: gggggggg
Explanation
The authorization code aaa(55) bbb(55) is not defined
to the group. This implies that the CU is not authorized
to replace the version of the member in the error
group.
User response
It is possible that the function will succeed with a
different authorization code. Contact the &dbc for a
list of authorization codes that are valid for this group.
If none of the authorization codes defined to the
group work, try the same function at a different group.
Contact the &dbc if you need further assistance.
Project manager response
The list of valid authorization codes defined for group
can be found in the &libdef on the FLMGROUP macro.
Do not attempt to add authorization codes to the
project definition unless you are familiar with the
risks associated with using authorization codes to
control SCLM operations as outlined in z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM82502 INPUT PARAMETER
"$STATS_INFO" CANNOT BE NIL
Explanation
$STATS_INFO has not been initialized with data. The
SCLM service requested must have data in this record.
User response
Initialize $STATS_INFO and call the service again.
SCLM messages
838  z/OS: z/OS ISPF Messages and Codes

## Page 859

FLM82503 DUPLICATE CHANGE CODE
RECORDS FOUND IN THE
"ACCOUNTING LIST INFO ARRAY"
RECORD KIND: aaaa
CHANGE CODE: bbbbbbbb
Explanation
Change code bbbbbbbb was specified multiple times
within the same accounting $list_info array. A service
call using a user-specified parser had duplicate entries
for the change code.
User response
Remove duplicate entries for the change code in the
SCLM services parameter list and call the service
again.
Project manager response
Rewrite the involved parser to add logic that will
remove duplicate entries. For more information
on invoking user-defined parsers, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM82504 DUPLICATE COMPOOL RECORDS
FOUND IN THE "ACCOUNTING
LIST INFO ARRAY" RECORD KIND:
aaaa
COMPOOL NAME: bbbbbbbb
Explanation
Compool bbbbbbbb was specified multiple times
within the same accounting $list_info array. A service
call to a user-specified parser generated duplicate
entries for the compool.
User response
If the STORE service was called, remove duplicate
entries for the compool in the SCLM services
parameter list and call the service again. If the SAVE
service was called, or if data was passed to the STORE
service as a result of the PARSE service, contact the
project manager.
Project manager response
Rewrite the involved parser to add logic that will
remove duplicate entries. For more information on the
STORE service, see z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
FLM82505 DUPLICATE INCLUDE RECORDS
FOUND IN THE "ACCOUNTING
LIST INFO ARRAY" RECORD KIND:
aaaa
INCLUDE NAME: bbbbbbbb
Explanation
The include bbbbbbbb was specified multiple times
within the same accounting $list_info array. A service
call to a user-specified parser generated duplicate
entries for the include.
User response
If the STORE service was called, remove duplicate
entries for the include in the SCLM services parameter
list and call the service again. If the SAVE service was
called, or if data was passed to the STORE service
as a result of the PARSE service, contact the project
manager.
Project manager response
Rewrite the involved parser to add logic that will
remove duplicate entries. For more information on the
STORE service, see z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
FLM82506 DUPLICATE CU RECORDS FOUND
IN THE "ACCOUNTING LIST INFO
ARRAY" RECORD KIND: aaaa
CU NAME: bbb(55) ccc(55)
CU TYPE: d
GENERIC FLAG: e
DEPEND NAME: fff(55) ggg(55)
DEPEND CU TYPE: h
Explanation
The CU bbb(55) ccc(55) was specified multiple times
within the same accounting $list_info array. A service
call to a user-specified parser generated duplicate
entries for the CU.
User response
If the STORE service was called, remove duplicate
entries for the CU in the SCLM services parameter list
and call the service again. If the SAVE service was
called, or if data was passed to the STORE service
as a result of the PARSE service, contact the project
manager.
Project manager response
Rewrite the involved parser to add logic that will
remove duplicate entries. For more information on the
STORE service, see z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
SCLM messages
Chapter 3. SCLM messages  839

## Page 860

FLM82507 ERROR ALLOCATING THE
SPECIFIED MEMBER GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
Member cccccccc does not exist in the group
aaaaaaaa and type bbbbbbbb.
User response
Put the member in the hierarchy, or remove the
reference to the member source code or member lists.
FLM82508 CU LOCKED ELSEWHERE CU
NAME: aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
ERROR GROUP: eeeeeeee
TYPE: ffffffff MEMBER: gggggggg
ERROR AUTHORIZATION CODE:
hhhhhhhh
Explanation
CU aaa(55) bbb(55) has already been updated in
another hierarchical view or the CU is in the current
view but in another type. The changes reside in the
group specified in this message. This group is not in
your view of the hierarchy. You cannot update the
member because you would not be working with the
most current version of the member.
User response
Promote the member into a group that is in your
hierarchy (that is, one that appears on your SCLM Edit
- Entry panel). If the member cannot be promoted,
you must delete the member and its accounting
information in the error group using the SCLM library
utility or the DELETE service.
FLM82509 DRAWDOWN VERIFICATION
ERROR CODE: aaa CU NAME:
bbb(55) ccc(55)
CU TYPE: dddd
CU QUALIFIER: eeeeeeee
GROUP: ffffffff
AUTHORIZATION CODE: gggggggg
Explanation
One of these errors has occurred:
• Group ffffffff is an invalid SCLM group.
• An I/O error occurred in retrieving the XREF record
from the accounting database.
User response
Correct the group if it is incorrect. If the group if
correct, contact the project manager.
Project manager response
Verify that the cross-reference record in the
accounting database is correct.
FLM82511 CU DRAWN DOWN FROM
ANOTHER MEMBER CU NAME:
aaa(55) bbb(55)
CU TYPE: cccc
CU QUALIFIER: dddddddd
DRAWN DOWN FROM
GROUP: eeeeeeee
TYPE: ffffffff MEMBER: gggggggg
Explanation
SCLM is now tracking two sessions of compilation unit
aaa(55) bbb(55) in separate members. This message
is provided for information only.
FLM82601 INPUT PARAMETER
"PARSE_MODE" MUST BE 'C' OR
'U', PARSE_MODE: a
Explanation
An invalid parameter was passed to an SCLM service.
User response
Supply the valid parameter in the SCLM service
parameter list.
FLM82602 "LANGUAGE" CANNOT BE
DEFAULTED AN EDITABLE
ACCOUNTING RECORD DOES NOT
EXIST FOR THE MEMBER GROUP:
aaaaaaaa TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
A valid language has not been assigned to member
cccccccc. The language is obtained from SCLM
accounting information.
User response
Add a valid language to the service call or command.
FLM82603 WARNING: A PARSER
ERROR OCCURRED BUT AN
UNCONDITIONAL PARSE WAS
REQUESTED
SCLM messages
840  z/OS: z/OS ISPF Messages and Codes

## Page 861

Explanation
An error occurred while parsing the member but you
requested an unconditional parse. The SAVE service
continues and saves the statistical and dependency
information that the parser returned for the member.
User response
If possible, you should try to correct the parser
errors. A parser error can cause incorrect dependency
information to be saved for the member. If the parser
error cannot be corrected at this time, call the SAVE
service for this member at a later date to correct the
parser error.
FLM82604 ERROR: AN ERROR OCCURRED
ATTEMPTING TO ALLOCATE
THE TEMPORARY DATA SET
CONTAINING THE DECODED
MEMBER.
Explanation
SCLM encountered an error attempting to allocate the
temporary data set containing the decoded member.
User response
Determine why SCLM was unable to allocate the
temporary data set.
FLM82605 ERROR: AN ERROR OCCURRED
ATTEMPTING TO ALLOCATE A
TEMPORARY SYSPRINT DATA
SET TO BE USED TO COPY
THE DECODED MEMBER THE
DEVELOPMENT DATA SET.
Explanation
SCLM encountered an error attempting to allocate the
temporary SYSPRINT data set used to the decoded
member.
User response
Determine why SCLM was unable to allocate the
temporary data set.
FLM83100 UNABLE TO ALLOCATE THE
aaaa GROUPS REQUESTED. bbb.
GROUPS IN THE HIERARCHY
VIEW FROM cccccccc WILL BE
ALLOCATED
Explanation
The total_groups parameter passed to the DSALLOC
service was either less than zero or greater than the
total number of groups in the view of the hierarchy
starting from group cccccccc. The DSALLOC service will
allocate all of the groups in the hierarchical view. This
message will result in a warning condition.
User response
If allocating all groups in the hierarchy view is
acceptable, no action is necessary. If this result was
unexpected, verify that the first_group parameter
cccccccc was the value intended. If an alternate
project definition was used, verify that the intended
value was passed. If all parameters seem correct, use
SCLM Browse to identify the groups SCLM considers
part of the hierarchy view. This can be done by
specifying cccccccc on the SCLM main panel and
selecting option 1. If bbb is greater than 4, invoke
SCLM Browse successively using the group name that
appears at the end of the concatenation on the Browse
panel.
FLM84101 INPUT PARAMETER
"DELETE_FLAG" MUST BE 'BMAP',
'ACCT', OR 'TEXT' DELETE_FLAG:
aaaa
Explanation
An invalid parameter was passed to the DELETE
service.
User response
Supply the valid parameter in the SCLM service
parameter list. For more information, see the
topic about the DELETE service in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM84110 WARNING - BUILD MAP NOT
DELETED BECAUSE IT DOES NOT
EXIST
Explanation
The build map to be deleted does not exist.
FLM84111 WARNING - ACCOUNTING
RECORD NOT DELETED BECAUSE
IT DOES NOT EXIST
Explanation
The accounting record to be deleted does not exist.
SCLM messages
Chapter 3. SCLM messages  841

## Page 862

FLM84112 MEMBER: aaaaaaaa TYPE:
bbbbbbbb GROUP: cccccccc IS IN
USE BY YOU OR ANOTHER USER.
Explanation
Your attempt to delete member aaaaaaaa failed
because the member is in use by you or another user
(for example, the member is being edited.)
User response
Verify that no one is using member aaaaaaaa before
you attempt to delete it.
FLM84200 NEW AUTHORIZATION CODE IS
EQUAL TO OLD AUTHORIZATION
CODE. NO CHANGE IS REQUIRED
FOR: GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
The authorization code assigned to member cccccccc
is equal to the new authorization code.
User response
Verify that the new authorization code was specified
correctly.
FLM84204 WARNING, MEMBER: aaaaaaaa
COULD BE REPLACED BY MEMBER
AT GROUP: bbbbbbbb TYPE:
cccccccc WITH AUTHORIZATION
CODE: dddddddd
Explanation
The member that is being updated (aaaaaaaa) was
found at a lower group. The member at the lower
group is promotable, and, therefore, could replace
the member that is being updated. The update is
performed as requested.
User response
Report this situation to the project manager.
Project manager response
The authorization code for the lower group member
should be updated to match the higher group member
to avoid overlaying the update made in the higher
group.
FLM84300 INPUT PARAMETER "PRESERVE"
MUST BE 'Y' OR 'N' PRESERVE:
aaa
Explanation
The preserve parameter of the Edit service must be
Y to preserve the record length of variable-length
records, or N to remove trailing blanks. aaa is not a
valid value.
User response
Correct the preserve parameter.
FLM84305 INPUT PARAMETER "CONFIRM"
MUST BE 'Y' OR 'N' CONFIRM: aaa
Explanation
The confirm parameter of the Edit service must be Y
to confirm delete move, and replace operations within
edit, or N to process without confirmation. aaa is not a
valid value.
User response
Correct the confirm parameter.
FLM84310 INPUT PARAMETER "MIX" MUST
BE 'Y' OR 'N' MIX: aaa
Explanation
The mix parameter of the Edit service must be Y for
mixed mode, or N to edit without mixed mode. aaa is
not a valid value.
User response
Correct the mix parameter.
FLM84320 INPUT PARAMETER "ALL_HIER"
MUST BE ''N''', WHEN GROUP
2, GROUP 3, OR GROUP 4 IS
SPECIFIED. ALL_HIER: aaaa'
Explanation
To allocate the entire hierarchy, set the all_hier
parameter to 'Y'. To allocate only specific groups, put
any desired group names in the group 1, group 2,
group 3, and group 4 parameters, and set the all_hier
parameter to 'N'.
User response
Correct the all_hier parameter, or set group 2, group 3,
and group 4 to blanks.
FLM84325 INPUT PARAMETER "ALL_HIER"
MUST BE 'Y' OR 'N' ALL_HIER: aaa
SCLM messages
842  z/OS: z/OS ISPF Messages and Codes

## Page 863

Explanation
The all_hier parameter of the Edit service must be 'Y'
to allocate the entire hierarchy, or 'N' to allocate only
specified groups. aaa is not a valid value.
User response
Correct the all_hier parameter.
FLM84330 GROUP 1 MUST BE A
DEVELOPMENT GROUP GROUP1:
aaa
Explanation
aaa is not a development group. The group 1
parameter for the Edit service must be a development
group.
User response
Correct the group 1 parameter.
FLM84335 USER DEFINED DDNAME: aaa FOR
EDIT MESSAGES NOT ALLOCATED
Explanation
The messages ddname of the Edit service must
be allocated before executing the edit service. The
ddname can be left blank to allocate the ddname to
the default output device (such as the terminal).
User response
Either allocate the data set before invoking the Edit
service or set the ddname parameter to blank.
FLM84340 SCLM EDIT CANNOT BE INVOKED
WITHIN AN ACTIVE EDIT
SESSION. USE SPLIT SCREEN OR
END THE CURRENT SCLM EDIT
SESSION.
Explanation
SCLM edit service cannot be invoked from within an
SCLM edit session.
User response
Either close the current edit session or split the screen
and try again.
FLM84345 VOLUME IS NOT MOUNTED OR IS
NOT AUTHORIZED FOR YOUR USE.
VOLSER: aaa
Explanation
The parser volume is not mounted or you do not have
authority to use it.
User response
Verify that the volume is correct, and that you have
authority to access it.
FLM84350 AUTHCODE IS INVALID FOR
GROUP. AUTHCODE: aaaaaaaa
GROUP: bbbbbbbb
Explanation
The requested authcode aaaaaaaa is not available for
group bbbbbbbb.
User response
Enter a valid authorization code for the group, or leave
the authorization code blank to default.
FLM84355 VOLUME IS NOT MOUNTED OR IS
NOT AUTHORIZED FOR YOUR USE.
VOLSER: aaaaaa
Explanation
The requested parser volume could not be accessed.
User response
Verify that the volume is correct.
Project manager response
Verify that the volume is available and online.
FLM84500 FROM_AUTHCODE DOES NOT
MATCH CURRENT AUTHCODE.
FROM_AUTHCODE: aaaaaaaa
CURRENT AUTHCODE: bbbbbbbb
FOR MEMBER: cccccccc TYPE:
dddddddd
Explanation
The AUTHCODE service only updates a member's
authorization code if the FROM_AUTHCODE matches
the member's authcode, or if the FROM_AUTHCODE is
blank.
FLM84510 USER DEFINED DDNAME: aaa
FOR AUTHCODE MESSAGES NOT
ALLOCATED
SCLM messages
Chapter 3. SCLM messages  843

## Page 864

Explanation
The messages ddname of the AUTHCODE service must
be allocated before executing the AUTHCODE service.
The ddname can be left blank to allocate the ddname
to the default output device (such as the terminal).
User response
Either allocate the data set before invoking the
AUTHCODE service or set the ddname parameter to
blank.
FLM84515 USER DEFINED DDNAME: aaa
FOR AUTHCODE REPORT NOT
ALLOCATED
Explanation
The report ddname of the AUTHCODE service must be
allocated before executing the AUTHCODE service. The
ddname can be left blank to allocate the ddname to
the default output device (such as the terminal).
User response
Either allocate the data set before invoking the
AUTHCODE service or set the ddname parameter to
blank.
FLM84520 INPUT PARAMETER "MODE" MUST
BE "C" OR "U". MODE:a
Explanation
The mode must be C for conditional, U for
unconditional, or blank, which defaults to conditional.
User response
Change the mode to "C" or "U".
FLM84525 FROM_AUTHCODE MUST BE
BLANK IF TO_AUTHCODE IS
BLANK.
Explanation
If the FROM_AUTHCODE is given, then the
TO_AUTHCODE is required.
User response
To change all requested members with a given
FROM_AUTHCODE, use the TO_AUTHCODE parameter
to specify the authcode to change to. To get the
current authcode, leave both the FROM_AUTHCODE
and the TO_AUTHCODE blank.
FLM84530 FROM_AUTHCODE IS EQUAL TO
TO_AUTHCODE.
Explanation
The FROM_AUTHCODE and the TO_AUTHCODE must
not be the same.
User response
Update either the FROM_AUTHCODE or the
TO_AUTHCODE.
FLM84535 TO_AUTHCODE IS ALREADY SET
TO :aaaaaaaa FOR MEMBER:
bbbbbbbb TYPE: cccccccc
Explanation
The AUTHCODE is already set to the requested value.
FLM84540 THE MEMBER IS NOT EDITABLE.
MEMBER: aaaaaaaa TYPE:
bbbbbbbb
Explanation
Only editable members have authcodes.
FLM84545 TYPE OR MEMBER NAME PATTERN
IS INVALID. TYPE: aaaaaaaa
MEMBER: bbbbbbbb
Explanation
Either the type or member parameter is not a valid
name or pattern.
User response
Correct the parameter.
FLM84550 NO MEMBERS MATCH MEMBER OR
TYPE PATTERN. TYPE: aaaaaaaa
MEMBER: bbbbbbbb
Explanation
There are no members in the project and group that
match the type and member pattern.
FLM84555 INVALID GROUP NAME.
Explanation
The group does not exist in the project.
User response
Correct the group name.
SCLM messages
844  z/OS: z/OS ISPF Messages and Codes

## Page 865

FLM85000 aaaaaa UTILITY INITIATED -
bbbbbbbb ON cccccccc
Explanation
This message is provided for information only.
FLM85001 EXPORT ACCOUNTING DATA
SET CONTAINED UNEXPECTED
INFORMATION
Explanation
You specified N, meaning that the export data set does
not need to be purged because it is already empty.
However, data was found in the accounting export data
set defined for the export database.
User response
Submit the export again, specifying Y.
FLM85002 ERROR PURGING EXPORT DATA
SET
Explanation
A VSAM error occurred while attempting to delete
records from the export database.
User response
It is possible that the last export into this data
set contained information that depends on both
the accounting and cross-reference data sets, and
the current definition of the export database does
not contain a cross-reference data set. Delete and
reallocate the export accounting data set, the cross-
reference data set, or both. Submit the job again.
Project manager response
Run IDCAMS against the export data set to determine
the problem.
FLM85003 I/O ERROR RETRIEVING LIST OF
ACCOUNTING RECORDS. GROUP:
aaaaaaaa TYPE: bbbbbbbb
Explanation
An I/O error occurred while trying to retrieve a list of
existing SCLM accounting records.
User response
Submit the job again. If the problem recurs, contact
the project manager.
Project manager response
Run IDCAMS against the export data set to determine
the problem.
FLM85004 I/O ERROR RETRIEVING LIST OF
BUILD MAP RECORDS. GROUP:
aaaaaaaa TYPE: bbbbbbbb
Explanation
An I/O error occurred while trying to retrieve a list of
existing build map accounting records.
User response
Submit the job again. If the problem recurs, contact
the project manager.
Project manager response
Run IDCAMS against the export data set to determine
the problem.
FLM85005 I/O ERROR RETRIEVING LIST OF
INTERMEDIATE RECORDS.
Explanation
An I/O error occurred while trying to retrieve a list of
existing intermediate records.
User response
Submit the job again. If the problem recurs, contact
the project manager.
Project manager response
Run IDCAMS against the export data set to determine
the problem.
FLM85007 USER DEFINED DDNAME:
aaaaaaaa FOR IMPORT/EXPORT
MESSAGES NOT ALLOCATED
Explanation
The data set associated with ddname aaaaaaaa has
not been allocated.
User response
Allocate the data set and submit the job again, or
submit the job again without specifying a ddname for
the file to which messages will be written. If you omit
the ddname for the messages data set, the messages
will be written to the terminal.
SCLM messages
Chapter 3. SCLM messages  845

## Page 866

FLM85008 USER DEFINED DDNAME:
aaaaaaaa FOR IMPORT/EXPORT
REPORT NOT ALLOCATED.
Explanation
The data set associated with ddname aaaaaaaa has
not been allocated.
User response
Allocate the data set and submit the job again, or
submit the job again without specifying a ddname for
the data set to which reports will be written. If you
omit the ddname for the reports data set, the reports
will be written to the terminal.
FLM85009 INPUT PARAMETER: aaaaaaaa IS
INVALID. VALID VALUES ARE "Y"
OR "N".
Explanation
The value entered to indicate whether or not the
export data set should be purged before exporting is
invalid.
User response
Enter either Y or N for the purge option, and submit the
export job again.
FLM85010 MEMBER aaaaaaaa CONTAINS
NON-BLANK AUTHCODE CHANGE
FIELD. MEMBER NOT EXPORTED.
Explanation
Member aaaaaaaa was in the process of having the
authorization code changed when the tried to copy it.
User response
Submit the export job again.
FLM85011 SCLM DATABASE CONTAINED
INTERMEDIATE RECORDS. NO
CROSS-REFERENCE DATA SET
DEFINED FOR EXPORT.
Explanation
The regular SCLM VSAM database contained
intermediate and cross-reference records for the
specified group, but the export VSAM database does
not have a data set defined for intermediate and cross-
reference information.
User response
Define an export cross-reference database and submit
the export job again.
FLM85012 SCLM ACCOUNTING RECORDS
CONTAIN CROSS-REFERENCE
RECORDS. NO CROSS-REFERENCE
DATA SET DEFINED FOR EXPORT.
Explanation
The regular SCLM VSAM accounting database contains
links to cross-reference records, but the export VSAM
database does not have a data set defined for
intermediate and cross-reference information.
User response
Define an export cross-reference database and submit
the export job again.
FLM85013 ERROR EXPORTING
INTERMEDIATE RECORD FOR: CU
NAME: a(110) CU TYPE: bbbbbbbb
CU QUALIFIER: cccccccc CODE:
ddd TYPE: eeeeeeee MEMBER:
ffffffff TO EXPORT DATABASE.
Explanation
An error occurred when trying to write the
intermediate record for a(110) to the export database.
User response
Possible return codes are:
12
The record format of the member's intermediate
accounting data is incorrect for the current release
of SCLM. Contact the project manager.
20
An I/O error occurred while trying to write the
intermediate record to the export data set. Submit
the job again. If the error occurs again, contact the
project manager.
24
The cross-reference data set was not defined for
the export database. Contact the project manager.
Project manager response
If the return code is:
12
Verify that the cross-reference data set is
compatible with the current release of SCLM.
SCLM messages
846  z/OS: z/OS ISPF Messages and Codes

## Page 867

20
Run IDCAMS against the export cross-reference
data set to determine the problem.
24
Define the export cross-reference data set on
the FLMCNTRL macro of the project definition.
For more information on the FLMCNTRL macro
see z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM85014 ERROR RETRIEVING
INTERMEDIATE RECORD: CU
NAME: a(110) CU TYPE: bbbbbbbb
CU QUALIFIER: cccccccc CODE:
ddd GROUP: eeeeeeee MEMBER:
ffffffff FROM SCLM ACCOUNTING
DATABASE.
Explanation
An error occurred while attempting to retrieve the
accounting information for the intermediate record of
a(110).
User response
Possible return codes are:
8
The accounting information for the intermediate
record of the compilation unit was not found in
group eeeeeeee. This means that the compiled
intermediate record is missing or out of date.
Rebuild the member containing the compilation
unit.
12
SCLM internal error. Contact the project manager.
20
An I/O error occurred while retrieving the
accounting information for the intermediate form
of the compilation unit. Submit the job again. If the
error occurs again, contact the project manager.
24
The cross-reference data set was not defined in
the project definition. Contact the project manager.
Project manager response
If the return code is:
12
Contact SCLM Program Support.
20
A VSAM error occurred.
24
Define the cross-reference data set on the
FLMCNTRL macro of the project definition. For
more information on the FLMCNTRL macro, see
z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
FLM85015 ERROR RETRIEVING
ACCOUNTING OR CROSS-
REFERENCE INFORMATION.
CODE: aaa ERROR GROUP:
bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd FROM SCLM
ACCOUNTING DATABASE.
Explanation
An error occurred while attempting to retrieve a
member's accounting or dependency information.
User response
Possible return codes are:
8
The member's accounting information was not
found. Introduce the member to SCLM using the
SCLM editor, migration utility, or SAVE service. Run
the processor again.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
16
SCLM found an invalid group in the project
definition. Contact the project manager.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Run IDCAMS against the accounting data set to
determine the problem.
FLM85016 ERROR EXPORTING ACCOUNTING
OR CROSS-REFERENCE DATA SET
INFORMATION. CODE: aaa ERROR
GROUP: bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd TO EXPORT
DATABASE.
Explanation
An error occurred while attempting to write a
member's accounting and dependency information.
SCLM messages
Chapter 3. SCLM messages  847

## Page 868

User response
Possible return codes are:
8
An I/O error occurred while writing the member's
accounting information and no attempt was made
to write the dependency information. Errors can
occur if SCLM attempts to reference this member.
Submit the job again. If the error occurs again,
contact the project manager.
12
An I/O error occurred while writing dependency
information for a compilation unit. Errors can occur
if SCLM attempts to reference this member. Submit
the job again. If the error occurs again, contact the
project manager.
Project manager response
Run IDCAMS against the accounting and cross-
reference data sets to determine the problem.
FLM85017 ERROR RETRIEVING BUILD
MAP INFORMATION, CODE:
aaa GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
FROM SCLM ACCOUNTING
DATABASE.
Explanation
No build map record could be retrieved for the
member cccccccc.
User response
Possible return codes are:
8
The specified build map record does not exist.
Build the appropriate architecture member. Invoke
the processor again.
12
The format of the data retrieved was incorrect.
Delete the build map and build again to regenerate
it.
16
An invalid group was found in the project
definition. Contact the project manager.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
If the return code is:
16
Reassemble the project definition. Verify that
no errors occurred. Relink the project definition.
For more information, see z/OS ISPF Software
Config ur ation  and Library Manager Guide and
Reference.
20
A VSAM error occurred. Run IDCAMS against the
accounting data set to determine the problem.
FLM85018 ERROR EXPORTING BUILD MAP
INFORMATION, CODE: aaaa
GROUP: bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd TO EXPORT
DATABASE.
Explanation
An error occurred while attempting to insert or update
build map information in the accounting data set.
User response
Possible return codes are:
4
An I/O error occurred while writing the
member's accounting information to the secondary
accounting data set. Because the primary
accounting data set was correctly updated, SCLM
will use the correct information for all references.
However, the two accounting data sets are no
longer identical. Contact the project manager.
8
The length of the build map exceeds the maximum
size allowed by the accounting data set.
12
Contact SCLM Program Support.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Run IDCAMS against the accounting data set to
determine the problem. If the return code is 8, contact
SCLM Program Support.
FLM85020 ERROR PURGING INTERMEDIATE
INFORMATION FROM EXPORT
CROSS-REFERENCE DATA SET.
Explanation
An error occurred while attempting to purge
accounting records of intermediate form from the
export cross-reference VSAM data set.
SCLM messages
848  z/OS: z/OS ISPF Messages and Codes

## Page 869

User response
Possible reasons are:
1. An I/O error occurred while purging. Submit the
job again. Contact the project manager if the error
occurs again.
2. Target data set enqueued. Submit the job after the
data set is no longer exclusively in use by another
job.
3. The export cross-reference data set was not
defined in the project definition. Contact the project
manager.
Project manager response
Check the project definition. If an export cross-
reference data set is not defined, define one on the
FLMCNTRL macro of the project definition. For more
information on the FLMCNTRL macro, see z/OS ISPF
Software Config ur ation  and Library Manager Guide
and Reference. If the data set is defined to the
project definition, run IDCAMS against the export
cross-reference data set to determine the problem.
FLM85021 ERROR PURGING CROSS-
REFERENCE INFORMATION FROM
EXPORT CROSS-REFERENCE DATA
SET.
Explanation
An error occurred while attempting to purge cross-
reference records from the export cross-reference
VSAM data set.
User response
Possible reasons are:
1. An I/O error occurred while attempting to purge
the export cross-reference data set. Submit the
job again. Contact the project manager if the error
occurs again.
2. The target data set is enqueued. Submit the job
again after the data set is no longer exclusively in
use by another job.
3. The export cross-reference data set was not
defined in the project definition. Contact the project
manager.
Project manager response
Check the project definition. If the export cross-
reference data set is not defined, define one on the
FLMCNTRL macro of the project definition. For more
information on the FLMCNTRL macro, see z/OS ISPF
Software Config ur ation  and Library Manager Guide
and Reference. If the data set is defined to the
project definition, run IDCAMS against the export
cross-reference data set to determine the problem.
FLM85022 EXPORT CROSS-REFERENCE DATA
SET CONTAINED UNEXPECTED
INFORMATION.
Explanation
You specified N, meaning that the export data set does
not need to be purged because it is already empty.
However, data was found in the cross-reference data
set defined for the export database.
User response
Specify Y in the Purge> field, and submit the job again.
FLM85023 EXPORT - PURGE PHASE
INITIATED.
Explanation
This message is provided for information only.
FLM85024 EXPORT - EXPORT PHASE
INITIATED.
Explanation
This message is provided for information only.
FLM85025 WARNING - EXPORT DATABASE
CONTAINS NO RECORDS.
Explanation
No records exist in the export database. For export,
there were no records to be exported from the
specified group. For import there were no records in
the export database to be imported.
User response
For export, check to be sure the correct group was
specified and contained accounting information. For
import, make sure the correct export database was
defined in the project definition. Contact the project
manager for assistance.
Project manager response
For import, assist the user in determining that the
project definition contains the correct export database
and that records exist.
FLM85101 INPUT PARAMETER
"DELETE_FLAG" MUST BE 'BMAP',
'ACCT', 'TEXT' OR 'OUTPUT'.
SCLM messages
Chapter 3. SCLM messages  849

## Page 870

Explanation
The DELETE_FLAG value entered in the DELGROUP
input parameters is not valid. Valid values are: BMAP,
ACCT, TEXT or OUTPUT.
User response
Submit the delete group job with the correct input
value for DELETE_FLAG.
FLM85102 USER DEFINED DDNAME:
aaaaaaaa FOR DELETE GROUP
LISTING NOT ALLOCATED.
Explanation
The ddname specified for the delete group listing
was not allocated. If the delete group function is
invoked using the DELGROUP service, the ddname
for the delete group listing is optional. If you omit
the ddname, the delete group listing is written to
the terminal. If you specify a ddname, you must first
allocate it.
User response
Verify that the user-supplied ddname for the delete
group listing is allocated. Submit the job again.
FLM85103 USER DEFINED DDNAME:
aaaaaaaa FOR DELETE GROUP
REPORT NOT ALLOCATED.
Explanation
The ddname specified for the delete group report
was not allocated. If the delete group function is
invoked using the DELGROUP service, the ddname
for the delete group report is optional. If you omit
the ddname, the delete group report is written to
the terminal. If you specify a ddname, you must first
allocate it.
User response
Verify that the user-supplied ddname for the delete
group report is allocated. Submit the job again.
FLM85104 USER DEFINED DDNAME:
aaaaaaaa FOR DELETE GROUP
MESSAGES NOT ALLOCATED.
Explanation
The ddname specified for the delete group messages
was not allocated. If the delete group function is
invoked using the DELGROUP service, the ddname for
the delete group messages is optional. If you omit
the ddname, the delete group messages are written
to the terminal. If you specify a ddname, you must first
allocate it.
User response
Verify that the user-supplied ddname for the delete
group messages is allocated. Submit the job again.
FLM85105 USER DEFINED DDNAME:
aaaaaaaa FOR DELETE GROUP
USER EXITNOT ALLOCATED.
Explanation
The ddname specified for the delete group user exit
was not allocated. If the delete group function is
invoked using the DELGROUP service, the ddname for
the delete group user exit is optional. If you specify a
ddname, you must first allocate it.
User response
Verify that the user-supplied ddname for the delete
group user exit is allocated. Submit the job again.
FLM85107 aaaaa ACCOUNTING RECORDS
WERE DELETED.
Explanation
aaaaa accounting records were successfully deleted.
FLM85108 DELGROUP REPORT COMPLETED
-- NO RECORDS DELETED.
Explanation
This message is provided for information only.
FLM85109 aaaaa BUILD MAP RECORDS
WERE DELETED.
Explanation
aaaaa build map records were successfully deleted.
FLM85110 aaaaa INTERMEDIATE RECORDS
WERE DELETED.
Explanation
aaaaa intermediate records were successfully
deleted.
FLM85111 INPUT PARAMETER
"DELETE_MODE" MUST BE
'REPORT' OR 'EXECUTE'.
SCLM messages
850  z/OS: z/OS ISPF Messages and Codes

## Page 871

Explanation
The value entered to indicate whether or not the
delete group will actually delete data or only produce a
report is not valid.
User response
Enter either REPORT or EXECUTE for the delete_mode
parameter, and resubmit the delete group job.
FLM85202 INVALID IMPORT MODE
PARAMETER: aaaa
Explanation
The parameter specified for IMPORT MODE on the
FLMCMD service is not valid.
User response
Change the IMPORT MODE parameter to one of the
accepted values (C, U, or R). Verify that the correct
number of parameters have been specified and are in
the proper order.
FLM85208 EXPORT DATABASE CONTAINS
MULTIPLE GROUPS.
Explanation
The Import utility is attempting to retrieve SCLM
accounting information for a group from the Export
database, but the Export database has information for
more than one group.
User response
Run the Export utility again for the group you desire,
and specify YES as the REPLACE EXPORT DATA option.
This purges the Export database and writes the
desired information to use for the Import utility.
FLM85212 'INITIAL' ACCOUNTING RECORD
FOUND. TYPE: aaaaaaaa
MEMBER: bbbbbbbb
Explanation
An accounting record of type, INITIAL, was found in
the export database.
User response
Determine why an initial record has been exported.
If this is not a redundant accounting record, save the
record, submit the export job again, and then proceed
with the import operation.
FLM85213 ERROR RETRIEVING
ACCOUNTING RECORD LIST FROM
EXPORT DATABASE, CODE: aaa
Explanation
An error occurred while trying to retrieve the list of
accounting records from the export database.
User response
Possible return code is:
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Run IDCAMS against the accounting data set to
determine the problem.
FLM85214 NORMALIZATION FAILURE FOR
TYPE: aaaaaaaa, MEMBER:
bbbbbbbb, ACCOUNTING DATE:
cccccc, TIME: dddddd, PDS
DIRECTORY DATE: eeeeee, TIME:
ffffff
Explanation
The date and time stamps retrieved from the PDS
directory information for the member do not match
the accounting record Change Date and Change Time
fields.
User response
The member was edited after the export job was
executed.
FLM85215 ERROR RETRIEVING BUILD MAP
RECORD LIST FROM EXPORT
DATABASE, CODE: aaa
Explanation
An error occurred while trying to retrieve the list of
build map records from the export database.
User response
Possible return code is:
20
A severe I/O error occurred. Contact the project
manager.
SCLM messages
Chapter 3. SCLM messages  851

## Page 872

Project manager response
Run IDCAMS against the accounting data set to
determine the problem.
FLM85216 MEMBER: aaaaaaaa NOT FOUND
IN PDS DIRECTORY AT GROUP:
bbbbbbbb, TYPE: cccccccc, FOR
DATA SET NAME: ddd(44)
Explanation
The member does not exist in the data set ddd(44),
which is the specified target data set.
User response
Copy the source of the member into the data set
ddd(44) and resubmit the job again.
FLM85218 INVALID ACCOUNTING
INFORMATION FOR TYPE:
aaaaaaaa, MEMBER: bbbbbbbb,
AUTHORIZATION CHANGE CODE:
cccccccc
Explanation
An accounting record was found in the export
database that has a non-blank value in its
Authorization_Change_Code field.
User response
This field should be blank to verify the validity of the
authorization code and to allow the user to change its
value in the imported record.
FLM85219 AUTHORIZATION CODE: aaaa
IS NOT DEFINED TO GROUP:
bbbbbbbb
Explanation
Authorization code aaaa has not been defined to SCLM
as a valid authorization code for group bbbbbbbb.
User response
Use the authorization code that has been defined to
the specified group. Contact the project manager for
a list of valid authorization codes. If the specified
authorization code is valid, contact the project
manager.
Project manager response
Check the project definition that defines the specified
group. The valid authorization codes for the group
are defined there. If authorization codes are used,
reference the FLMAGRP macros in the project
definition, as well. If the authorization code is valid,
add it to the project definition.
FLM85220 CU QUALIFIER MISMATCH FOR
TYPE: aaaaaaaa, MEMBER:
bbbbbbbb, CU QUALIFIER:
cccccccc
Explanation
The CU qualifier found in the export accounting
record is not defined to the corresponding language
attributes.
User response
Contact the project manager.
Project manager response
Update the language definition so that the CU
qualifiers match.
FLM85222 ERROR RETRIEVING CROSS-
REFERENCE RECORD FOR CU-
NAME: aaa(55) bbb(55) CU-
TYPE: cccccccc CU-QUALIFIER:
dddddddd GROUP: eeeeeeee
Explanation
Severe I/O error occurred while retrieving cross-
reference record for the dependent compilation unit.
User response
Submit the job again. If the problem recurs, contact
the project manager.
Project manager response
Run IDCAMS against the cross-reference data set to
determine the problem.
FLM85224 CROSS-REFERENCE RECORD
ALREADY EXISTS IN THE
TARGET GROUP. CU NAME:
aaa(55) bbb(55) CU TYPE:
cccccccc CU QUALIFIER: dddddddd
GROUP: eeeeeeee TYPE: dddddddd
MEMBER: eeeeeeee
Explanation
The import operation attempted to introduce a cross-
reference record that already exists in the target
group.
SCLM messages
852  z/OS: z/OS ISPF Messages and Codes

## Page 873

User response
Delete the record and submit the job again.
FLM85226 DRAW DOWN VERIFICATION
FAILURE FOR A CU. TYPE:
aaaaaaaa MEMBER: bbbbbbbb
Explanation
A draw down verification of the accounting record
failed because of its dependent compilation unit.
User response
Have the member promoted into a group that is in your
hierarchy. If the member cannot be promoted, then
the member and its accounting information must be
deleted before submitting the import job again.
FLM85228 ERROR RETRIEVING BUILD MAP
INFORMATION FROM EXPORT
DATABASE, CODE: aaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
An error occurred while attempting to retrieve a build
map record from the export database.
User response
Possible return codes are:
8
Unsuccessful in decoding VSAM record.
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Run IDCAMS against the accounting data set to
determine the problem.
FLM85230 BUILD MAP RECORD ALREADY
EXISTS IN THE TARGET GROUP:
aaaaaaaa FOR TYPE: bbbbbbbb,
MEMBER: cccccccc
Explanation
The group aaaaaaaa, into which import tried to
introduce the records, already contains the build map
record for member cccccccc.
User response
A build map record should not exist in the specified
target group. Determine the reason for the existence
of the record. Delete the record and submit the import
job again.
FLM85232 ACCOUNTING RECORD ALREADY
EXISTS IN THE TARGET GROUP:
aaaaaaaa FOR TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
The group aaaaaaaa, into which import tried to
introduce the records, already contains the accounting
record for the member cccccccc.
User response
An accounting record should not exist in the specified
target group. Determine the reason for the existence
of the record. Delete the record and submit the import
job again.
FLM85235 ERROR OCCURRED RETRIEVING
INTERMEDIATE RECORD LIST
FROM EXPORT DATABASE, CODE:
aaa
Explanation
An error occurred while trying to retrieve the list of
intermediate records from the export database.
User response
Possible return code is:
20
A severe I/O error occurred. Contact the project
manager.
Project manager response
Verify that the export data set exists and is correctly
allocated.
FLM85238 ERROR RETRIEVING
INTERMEDIATE RECORD FROM
EXPORT DATABASE, CODE: aaa CU
NAME: aaa(55) bbb(55) CU TYPE:
cccccccc CU QUALIFIER: dddddddd
GROUP: eeeeeeee
Explanation
An error occurred while attempting to retrieve the
intermediate record from the export database.
User response
Possible return code is:
SCLM messages
Chapter 3. SCLM messages  853

## Page 874

20
An I/O error occurred while retrieving the
intermediate record from the export database.
Submit the job again. If the error recurs, contact
the project manager.
Project manager response
Verify that the export data set exists and is correctly
allocated.
FLM85240 INTERMEDIATE RECORD ALREADY
EXISTS IN THE TARGET GROUP:
aaaaaaaa CU NAME: bbb(55)
ccc(55) CU TYPE: dddddddd CU
QUALIFIER: eeeeeeee GROUP:
ffffffff
Explanation
The import operation attempted to introduce an
intermediate record that already exists in the target
group.
User response
If this is an error condition, delete the record and
submit the job again.
FLM85246 EDITABLE ACCOUNTING RECORD
WITH NON-EDITABLE LANGUAGE:
aaaaaaaa FOR TYPE: bbbbbbbb
MEMBER: cccccccc
Explanation
Import found an editable accounting record that has a
non-editable language in the export database.
User response
Contact the project manager.
Project manager response
Verify that the language of the member matches that
of its exported counterpart.
FLM85254 CROSS REFERENCE DB
NOT DEFINED IN PROJECT
DEFINITION.
Explanation
One or more intermediate records were found in
the export database; however, the cross-reference
database is not defined in the project definition.
User response
Contact the project manager.
Project manager response
Define an export cross-reference data set for the
project and regenerate the project definition.
FLM85260 ERROR RETRIEVING
ACCOUNTING RECORD FROM
EXPORT DATABASE, CODE: aaa
FOR TYPE: bbbbbbbb MEMBER:
cccccccc
Explanation
No accounting record exists or could be retrieved from
the export database for member cccccccc.
User response
Possible return codes are:
8
The accounting record was not found in the
requested group. Introduce the member to SCLM
using the SCLM editor, migration utility, or SAVE
service. Run the processor again.
12
The member's accounting and dependency
information was successfully retrieved; however,
some of the dependency information failed a
verification check. To determine the nature of the
verification error, browse the member's accounting
and dependency information using the SCLM
library utility. To correct the problem, edit and save
the member.
20
A severe I/O error occurred. Contact the project
manager.
24
The cross-reference data set was not found in the
project definition. Contact the project manager.
Project manager response
If the return code is:
20
A VSAM error occurred. Run IDCAMS against the
accounting database to determine the problem.
24
Identify the cross-reference data set on the
FLMCNTRL macro of the project definition. For
more information on the FLMCNTRL macro, see
z/OS ISPF Software Config ur ation  and Library
Manager Guide and Reference.
SCLM messages
854  z/OS: z/OS ISPF Messages and Codes

## Page 875

FLM85266 TYPE: aaaaaaaa MEMBER:
bbbbbbbb WAS EDITED WHILE
IMPORT HAS BEEN IN PROGRESS.
Explanation
This condition can occur if someone edits the member
after import completes verification on the member. If
this is the case, the change date/time of the import
record will not be the same as the change date/time of
the export record.
User response
Delete the accounting record, then copy the original
member and submit the import job again.
FLM85267 TYPE: aaaaaaaa IS NOT DEFINED
IN THE PROJECT DEFINITION FOR
MEMBER: bbbbbbbb
Explanation
The accounting record belongs to a type that is not
defined in the project definition.
User response
Contact the project manager.
Project manager response
Redefine the project definition if the type is to be
imported. Then submit the export and import jobs
again.
FLM85270 ACCOUNTING INFORMATION
MISMATCH FOUND. FOR TYPE:
aaaaaaaa, MEMBER: bbbbbbbb
Explanation
The export database contains an editable accounting
record and the SCLM project contains a non-editable
accounting record with the same member name in the
hierarchical view, or vice versa.
User response
Delete the member that is causing the mismatch, and
submit the import job again.
FLM85299 aaaaaaaa UTILITY COMPLETED -
aaaaaaaa ON bbbbbbbb.
Explanation
This message is provided for information only.
FLM85300 DATE PARAMETER OR LONGDATE
PARAMETER REQUIRED.
Explanation
The VERRECOV service and the VERDEL service
require a date. For a date with a 2-character year, use
the date parameter. For a date with a 4-character year,
use the longdate parameter.
User response
Use the date parameter to specify a date with a
2-character year, or use the longdate parameter to
specify a date with a 4-character year.
FLM85305 BOTH DATE PARAMETER AND
LONGDATE PARAMETER ENTERED.
DATE PARAMETER USED:
aaaaaaaa
Explanation
If both the date parameter and the longdate
parameter are entered, the date parameter is used.
FLM85306 ERROR NOPROM PARAMETER
MUST EITHER BE NOREBUILD,
REBUILD OR REMOVE.
PARAMETER xxxxxxxxxxxxxxx
WAS USED.
Explanation
This message is self explanatory.
User response
Supply the valid parameter in the SCLM service
parameter LIST.
FLM85307 ERROR THE MEMBER MUST
HAVE AN ACCOUNTING STATUS
OF EDITABLE, NOPROM-N OR
NOPROM-R TO INVOKE THE
NOPROM SERVICE TO BE ABLE
TO SET THE MEMBER AS NON-
PROMOTABLE.
Explanation
This message is self explanatory.
User response
Use SCLM option 3.1 to browse the accounting record
and verify the accounting status of the member.
SCLM messages
Chapter 3. SCLM messages  855

## Page 876

FLM85308 ERROR THE MEMBER MUST HAVE
AN ACCOUNTING STATUS OF
NOPROM-N OR NOPROM-R TO
INVOKE THE NOPROM SERVICE
TO SPECIFY THAT THE MEMBER IS
NOW PROMOTABLE.
Explanation
This message is self explanatory.
User response
Use SCLM option 3.1 to browse the accounting record
and verify the accounting status of the member.
FLM85309 ERROR THE NOPROM SERVICE
CAN NOT BE ISSUED AGAINST
A MEMBER WITH AN ARCHDEF
LANGUAGE (ARCH=Y ON
FLMLANGL MACRO).
Explanation
This message is self explanatory.
User response
This message is for informational purposes only. No
action is required.
FLM85310 ACCESS TO RUN THE
NOPROM SERVICE AGAINST
THE SPECIFIED MEMBER HAS
BEEN RESTRICTED USING
THE FLMNPROM MACRO.
PLEASE CONTACT THE SCLM
ADMINISTRATOR IF THIS IS A
PROBLEM.
Explanation
This message is self explanatory.
User response
This message is for informational purposes only. No
action is required.
FLM85500 ERROR RC AAAAAA READING
ACCOUNT RECORD FOR BBBBBB
Explanation
The XDEPUPDT utility could not read an Account
record. Processing is terminated.
FLM85501 ERROR RC AAAAAA READING
BUILDMAP RECORD FOR BBBBBB
Explanation
The XDEPUPDT utility could not read a Buildmap
record. Processing is terminated.
FLM85505 NO CROSS DEPENDENCY ENTRIES
SELECTED
Explanation
The XDEPUPDT service did not find any objects with
dependencies. No records will be written to the
database.
FLM85506 ERROR WRITING CROSS
DEPENDENCY RECORD KEY:
AAAAAAAA BBBBBBBB
CCCCCCCC CODE: EEEEEEEE
Explanation
The XDEPUPDT utility experienced an error writing a
database record.
User response
Check for preceding error messages.
FLM85508 PROCESSING COMPLETED
Explanation
The XDEPUPDT service completed successfully.
FLM85509 PROCESSING TERMINATED WITH
ERRORS
Explanation
The XDEPUPDT service did not complete successfully.
FLM85514 NO CROSS DEPENDENCY
INFORMATION FOUND FOR
GROUP:AAAAAAAA TYPE:
BBBBBBBB MEMBER:CCCCCCCC
Explanation
There was no record in the Cross-dependency
database for this member.
FLM85515 NO CROSS DEPENDENCY
DATABASE ACTIVE FOR PROJECT:
AAAAAAAA
Explanation
The project has not defined a cross dependency
database.
SCLM messages
856  z/OS: z/OS ISPF Messages and Codes

## Page 877

FLM87100 ERROR, PARAMETER STRING
MUST BE SHORTER THAN aaa
CHARS LONG
Explanation
The input parameter string exceeded the maximum
length.
User response
Shorten the input parameter string to a valid length.
FLM87103 RECURSIVE "FILE" COMMAND
INVOCATIONS ARE NOT ALLOWED
Explanation
A FILE command cannot be invoked within another
FILE.
User response
Remove the recursive occurrence of the FILE
command. The contents of the referenced data set can
be copied directly into the original data set if desired.
FLM87105 THE COMMAND IS NOT
SUPPORTED COMMAND: aaa(60)
Explanation
The command is not supported by this release of
SCLM.
User response
For a list of and descriptions of valid SCLM service
commands, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM87107 aaaaaaaa bbb(24) FOR MEMBER
cccccccc AT dddddddd, CODE: eee
Explanation
The FLMCMD command termination message
aaaaaaaa represents the service that was executed.
bbb(24) represents the completion status of the
command.
User response
For information on the return code, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM87109 aaaaaaaa bbb(24) FOR GROUP
cccccccc AT dddddddd, CODE: eee
Explanation
This service operates on an SCLM-controlled group
rather than a member. The FLMCMD command
termination message aaaaaaaa represents the service
that was executed. bbb(24) represents the completion
status of the command. cccccccc is the group
processed by the service. dddddddd is the time stamp
when the service ended. eee is the return code. This
return code is documented in the description for each
service.
User response
For information on the return codes for this service,
see the "SCLM Services" topic in the z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM87110 ERROR FOUND ON LINE aaaa OF
bbbbbbbb DATA SET
Explanation
Check to see if another error message was printed. If
it was, correct the error indicated by the other error
message first. If the error message FLM87110 is the
only error message printed, then an error was found
on line number aaaa of data set bbbbbbbb. If FLMCMD
FILE is used, the line number refers to the number of
prompts that have been displayed. The error occurred
in the command issued at the last prompt.
User response
The error should be corrected in the data set. For
more information about SCLM services, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM87115 DBUTIL aaa(9) AT bbbbbbbb,
CODE: ccc
Explanation
Completion message for DBUTIL. This message is
provided for information only.
User response
For information on the DBUTIL service, see the "SCLM
Services" topic in the z/OS ISPF Software Config ur ation 
and Library Manager Guide and Reference.
FLM87120 ERROR, aaa(24) PARAMETER IN
COLUMN bbb IS TOO LONG
SCLM messages
Chapter 3. SCLM messages  857

## Page 878

Explanation
The parameter aaa(24) is longer than the maximum
allowed.
User response
Shorten the parameter. For more details on the SCLM
services, see z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
FLM87125 ERROR, aaa(24) PARAMETER
MUST BE SPECIFIED
Explanation
The parameter aaa(24) has not been specified.
User response
Add the parameter to the SCLM service invocation.
For more details on the SCLM services, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM87130 INVALID VALUE IN COLUMN aaa
FOR bbb(24) PARAMETER
Explanation
The value for the bbb(24) parameter is invalid.
The column number (aaa(24)) identifies the starting
location of the invalid value with the command.
User response
Correct the value of the parameter in the command
data set. For more details on the SCLM services, see
z/OS ISPF Software Config ur ation  and Library Manager
Guide and Reference.
FLM87131 INVALID VALUE FOR SEARCH
INDICATOR PARAMETER: aaaa
VALUE MUST BE "SEARCH",
"FORWARD" OR "MATCH".'
Explanation
The ACCTINFO service search indicator parameter
is invalid. Valid values are SEARCH, FORWARD, and
MATCH.
User response
Correct the search indicator parameter and submit the
command again.
FLM87132 INVALID USER_HIER_VIEW (A)
SPECIFIED. VALUE MUST BE 'A',
'P', OR ' '.
Explanation
You have specified an invalid value for the parameter
that defines the type of hierarchy view to be allocated.
Valid values are A for all groups, P or blank for primary
(key) groups only.
User response
Specify A, P, or blank and resubmit.
FLM87133 EXTRANEOUS PARAMETER(S)
DETECTED IN SERVICE CALL
Explanation
An SCLM service was passed a parameter string that
contained more parameters than it requires.
User response
Remove the extra parameters from the SCLM
service invocation. For more details on the SCLM
services and their parameters, see z/OS ISPF
Software Config ur ation  and Library Manager Guide and
Reference.
FLM87134 NONNUMERIC CHARACTER
DETECTED IN THE
TOTALS_GROUPS PARAMETER OF
THE DSALLOC SERVICE
Explanation
A character other then 0-9 was found in the
totals_groups parameter of the DSALLOC service.
User response
The user should remove the nonnumeric character
from the DSALLOC parameter and resubmit the job.
FLM87135 THE DDNAME: aaaaaaaa IS
ALREADY IN USE
Explanation
The ddname aaaaaaaa is reserved for use by the
command processor.
User response
Change the ddname and submit the command again.
FLM87136 DATE PARAMETER IS NOT IN A
VALID FORMAT DATE: aaaa'
SCLM messages
858  z/OS: z/OS ISPF Messages and Codes

## Page 879

Explanation
The date parameter for the VERINFO, VERDEL, or
VERRECOV is not specified in the correct format for
your national language.
User response
Correct the date parameter and submit the command
again.
FLM87137 TIME PARAMETER IS NOT IN A
VALID FORMAT TIME: aaaa
Explanation
The time parameter for the VERINFO, VERDEL, or
VERRECOV is not specified in the correct format for
your national language.
User response
Correct the time parameter and submit the command
again.
FLM87138 INVALID VALUE FOR SEARCH
INDICATOR PARAMETER: aaaa
VALUE MUST BE "FORWARD",
"BACKWARD", OR "MATCH".'
Explanation
The VERINFO service search indicator parameter is
not valid. Valid values are FORWARD, BACKWARD, and
MATCH.
User response
Correct the search indicator parameter and submit the
command again.
FLM87140 THE DDNAME: aaaaaaaa HAS NOT
BEEN ALLOCATED
Explanation
The ddname aaaaaaaa specified was passed to
SCLM as a parameter, but the ddname has not been
allocated.
User response
Allocate the ddname and submit the command again.
FLM87150 THE COMMAND INVOCATION IS
TOO LONG
Explanation
The command invocation statement is longer than the
maximum 512 characters allowed.
User response
Edit the command and submit it again.
FLM87155 INVALID INPUT PARAMETER
FIRST OCCURRENCE ONLY -
aaaaaaaa
Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87160 INVALID INPUT PARAMETER DATA
TYPE - aaaaaaaa
Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87165 INVALID INPUT PARAMETER
ARCHITECTURE CONTROL -
aaaaaaaa
Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87167 INVALID INPUT PARAMETER
ARCHITECTURE SCOPE -
aaaaaaaa
Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87170 INVALID INPUT PARAMETER PAGE
HEADERS - aaaaaaaa
SCLM messages
Chapter 3. SCLM messages  859

## Page 880

Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87175 INVALID INPUT PARAMETER
SHOW TOTALS - aaaaaaaa
Explanation
Invalid data was entered as input in this field.
User response
Correct the invalid input parameter.
FLM87180 ERROR - ARCHITECTURE GROUP,
TYPE, AND MEMBER MUST BE
SPECIFIED IF IN OR OUT IS
SPECIFIED.
Explanation
Values must be specified for architecture group, type,
and member if IN or OUT is specified for the Database
Contents utility.
User response
Specify values for architecture group, type, and
member or specify "*" (asterisk) instead of IN or OUT
for architecture.
FLM87190 RECOVER TO DATA SET OR GROUP
AND TYPE MUST BE SPECIFIED
Explanation
A value must be specified for the data set to receive
the recovered version or values must be specified for
the group and type in the SCLM project to receive the
recovered member.
User response
Specify a value for the recovery data set or specify
values for the SCLM group and type to receive the
recovered version.
FLM87191 RECOVER TO DATA SET CAN
NOT BE SPECIFIED WITH TARGET
GROUP OR TYPE
Explanation
A data set for recovery of a version and a group and/or
type for recovery of a version may not be specified
together.
User response
Specify either a data set or a group and type, not both.
FLM87192 BOTH TARGET GROUP AND TYPE
MUST BE SPECIFIED
Explanation
Values were not specified for both the target group and
type in recovering a version of a member.
User response
Specify values for both the target group and type to
receive the version of a member.
FLM87193 UNABLE TO STORE DATA IN
REQUESTED TABLE: aaaaaaaa
Explanation
The VERINFO service encountered an error when
storing change codes, includes, user entries, or
compilation units from the accounting record into the
user defined table.
User response
Examine the user defined table that holds the values
from the VERINFO service. The table may not be
properly defined.
FLM87194 INVALID DATA SET NAME -
MEMBER NAMES ARE NOT
ALLOWED FOR RETRIEVAL DATA
SETS
Explanation
The VERRECOV service does not allow member names
to be specified for non-SCLM-controlled retrieval data
sets.
User response
Enter either a sequential data set name or a
partitioned data set name without a member name.
FLM87201 SCLM ID: aaaaaaaa IS NOT IN USE
SCLM messages
860  z/OS: z/OS ISPF Messages and Codes

## Page 881

Explanation
The SCLM ID corresponds to members that have been
locked but not freed by either the UNLOCK or the
STORE service. These will be converted from initial
state to lockout. This is just a warning message.
FLM87202 INVALID SCLM ID: aaaaaaaa
Explanation
The syntax of the SCLM ID is not valid. SCLM IDs are
generated in the format FLMddddd where d represents
a digit from 0-9.
User response
Check the SCLM ID specified for accuracy, or make
sure you used the INIT service to generate an SCLM ID
before you try to use the FREE service on it.
FLM87301 THE VERSION HISTORY REPORT
DDNAME MUST BE SPECIFIED
Explanation
When calling the VERHIST service you must specify
DDNAME of the data set where the Version History
Report will be written.
User response
Modify the VERHIST service call to specify a Version
History Report DDNAME.
FLM87304 THE VERSION HISTORY
PARAMETER VIEW REPORT MUST
BE EITHER Y OR N.
Explanation
When calling the VERHIST service the View History
report parameter must either Y or N.
User response
Modify the VERHIST service call to specify a View
History Report value of Y or N.
FLM87305 VERSION HISTORY REPORT
COMPLETED. Return Code=rr
Explanation
Informational message stating the return code from
the VERHIST service call.
FLM87306 ERROR RETRIEVING THE DATA
SET STATISTICS FOR VERSION
PDS DSN: mmmmm.mmmmmmm
RC=rr
Explanation
When running the VERHIST service SCLM encountered
an error attempting to retrieve the data set statistics
for the version PDS data set.
User response
Determine why SCLM was having errors retrieving the
statistics for the data set mmmmm.mmmmmmm
FLM87307 ERROR ALLOCATING TEMPORARY
DATA SET TO DECODE VERSION
PDS MEMBER INTO. RC=rr
Explanation
When running the VERHIST service SCLM encountered
an error attempting to create a temporary data set that
will be used to decode the member into.
User response
Determine why SCLM was unable to allocate the
temporary data set.
FLM87308 ERROR DECODING VERSION PDS
MEMBER. RC=rr ERROR MESSAGE:
xxxxxxxxxxxxx
Explanation
When running the VERHIST service SCLM encountered
a problem attempting to decode the member nnnnnnn.
User response
Use the error message to determine the why SCLM
was unable to encode the member nnnnnnnn.
FLM87309 ERROR ALLOCATING THE
VERSION PDS MEMBER RC=rr
Explanation
When running the VERHIST service SCLM encountered
a problem attempting allocate the version PDS
member.
User response
Determine why SCLM was unable to allocate the
version PDS member.
FLM87310 ERROR OPENING THE VERSION
PDS MEMBER RC=rr
SCLM messages
Chapter 3. SCLM messages  861

## Page 882

Explanation
When running the VERHIST service SCLM encountered
a problem attempting to open the version PDS
member.
User response
Determine why SCLM was unable to open the version
PDS member.
Verify the member has not been deleted from the
version PDS. If the member is present, verify using
the SCLM Audit and Version Utility that a version exists
for the date and time of the audit or version record
specified on the VERHIST command.
FLM87311 ERROR INSUFFICIENT STORAGE
TO PRODUCE VERSION HISTORY
REPORT.
Explanation
See above
FLM87312 ERROR READING THE VERSION
PDS MEMBER. RC=rr
Explanation
When running the VERHIST service SCLM encountered
a problem attempting to read the version PDS
member.
User response
Determine why SCLM was unable to read the version
PDS member.
FLM87313 ERROR THE VERSION PDS
MEMBER DID NOT HAVE A
HEADER RECORD AS THE FIRST
RECORD IN THE MEMBER.
PROCESSING TERMINATED.
Explanation
When running the VERHIST service SCLM found that
the first record in the version PDS member was not
a header record. This indicates that member was not
created using the SCLM versioning process.
User response
Determine why header record is not the first line in the
version PDS member.
FLM87314 ERROR THE VERSION PDS
MEMBER IS EMPTY
Explanation
When running the VERHIST service SCLM found that
the version PDS member was empty.
User response
None
FLM87315 ERROR THE SELECTED VERSION
WAS NOT FOUND IN THE VERSION
PDS MEMBER. PROCESSING IS
TERMINATED.
Explanation
When running the VERHIST service SCLM found that
the version date/time you specified was not found in
the version PDS member. Processing is terminated.
User response
None
FLM87318 ERROR VIEWING THE VERSION
HISTORY REPORT DATA SET. RC=rr
Explanation
SCLM encountered an error attempting view the
version history report written to the DDNAME specified
on the VERHIST service call.
User response
None.
FLM87320 ERROR WRITING THE VERSION
HISTORY REPORT RECORD. RC=rr
Explanation
When running the VERHIST Service SCLM encountered
an error attempting to write out a record to the version
history report.
User response
Determine why there were problems writing to the
data set specified via the report DDNAME parameter
on the VERHIST service.
FLM87401 ENDEC SERVICE PROCESSING
PARAMETER MUST EITHER BE
ENCODE OR DECODE.
SCLM messages
862  z/OS: z/OS ISPF Messages and Codes

## Page 883

Explanation
When calling the ENDEC service you must specify
either ENCODE or DECODE to determine if SCLM is to
encode or decode the input data set.
User response
None.
FLM87402 INPUT DDNAME OR GROUP/
TYPE PARAMETERS MUST BE
SPECIFIED.
Explanation
When calling the ENDEC service you must specify
either the input DDNAME or input group/type
parameter/s so SCLM can determine the input data set
that is to be encoded or decoded.
User response
Modify the ENDEC service to specify either the input
DDNAME or input group/type parameter/s.
FLM87403 INPUT GROUP AND/OR TYPE
PARAMETERS IGNORED AS THE
INPUT DDNAME PARAMETER WAS
SPECIFIED.
Explanation
Calling the ENDEC service you specified both the input
DDNAME and input group/type parameters. The input
data set specified via the input DDNAME will be used
by the ENDEC service.
User response
If the data set specified via the input DDNAME is
correct then no action is required. If the data set
specified via the input group/type is the one to be used
blank out the input DDNAME on the ENDEC service
call.
FLM87405 OUTPUT DDNAME OR GROUP/
TYPE PARAMETERS MUST BE
SPECIFIED.
Explanation
When calling the ENDEC service you must specify
either the output DDNAME or output group/type
parameter/s so SCLM can determine the output data
set that is to be encoded or decoded.
User response
Modify the ENDEC service to specify either the output
DDNAME or output group/type parameter/s.
FLM87406 OUTPUT GROUP AND/OR TYPE
PARAMETERS IGNORED AS THE
OUTPUT DDNAME PARAMETER
WAS SPECIFIED.
Explanation
Calling the ENDEC service you specified both the
output DDNAME and output group/type parameters.
The output data set specified via the output DDNAME
will be used by the ENDEC service.
User response
If the data set specified via the output DDNAME is
correct then no action is required. If the data set
specified via the output group/type is the one to be
used blank out the output DDNAME on the ENDEC
service call.
FLM87408 ERROR ALLOCATING THE DATA
SET mmmmm.mmmmmm GROUP:
aaaaaaa TYPE: bbbbbbb RC=rr
Explanation
SCLM received an error attempting to allocate the data
set mmmmm.mmmmmm.
User response
Determine why SCLM was unable to allocate the data
set mmmmm.mmmmmm.
FLM87409 INPUT MEMBER nnnnnnn DOES
NOT EXIST IN THE DATA SET
mmmmm.mmmmmm
Explanation
The member nnnnnnn was not found in the data set
mmmmm.mmmmmm
User response
None.
FLM87411 ERROR ALLOCATING THE DATA
SET mmmmm.mmmmmm
Explanation
SCLM received an error attempting to allocate the data
set mmmmm.mmmmmm.
SCLM messages
Chapter 3. SCLM messages  863

## Page 884

User response
Determine why SCLM was unable to allocate the data
set mmmmm.mmmmmm.
FLM89001 NO ACCOUNTING INFORMATION
FOUND IN HIERARCHY VIEW
FOR GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
No accounting information was found for the specified
member in the hierarchy view beginning at the
specified group.
User response
Verify selection parameters are correct and resubmit.
FLM89002 ERROR ACCESSING PROJECT
DATABASE FOR PROJECT:
aaaaaaaa
Explanation
An error occurred while attempting to access the
project database for the specified project.
FLM89004 INCOMPATIBLE ACCOUNT
INFORMATION TYPES. CURRENT
TYPE: a. SPECIFIED TYPE: b
Explanation
The member specified has a status (editable, non-
editable, and so forth) that is incompatible with the
status specified in the update accounting information.
User response
Enter compatible type and resubmit.
FLM89005 CANNOT CREATE NEW
ACCOUNTING RECORD OF
ACCOUNT INFO TYPE: a
Explanation
No accounting information exists for the specified
member. New accounting records can only have the
status INITIAL or NON-EDITABLE. The status specified
in the accounting information was other than INITIAL
or NON-EDITABLE.
User response
Enter valid type and resubmit.
FLM89006 ERROR UPDATING ISPF STATS
FOR PROJECT: aaaaaaaa GROUP:
bbbbbbbb TYPE: cccccccc
MEMBER: dddddddd
Explanation
The service encountered an unexpected error updating
the ISPF statistics of the member.
User response
Contact the System Administrator.
FLM89007 ERROR UPDATING PROJECT
DATABASE FOR PROJECT:
aaaaaaaa GROUP: bbbbbbbb
TYPE: cccccccc MEMBER:
dddddddd
Explanation
The service encountered an unexpected error updating
the SCLM project database containing the member’s
information.
User response
Contact project manager.
Project manager response
Run IDCAMS against the data set to determine
problem.
FLM89008 NO BUILD MAP INFORMATION
FOUND IN HIERARCHY VIEW
FOR GROUP: aaaaaaaa TYPE:
bbbbbbbb MEMBER: cccccccc
Explanation
No build map information was found for the specified
member in the hierarchy view beginning at the
specified group.
User response
Verify selection parameters are correct and resubmit.
FLM89009 LIBRARY ID: aaaaaaaa DOES
NOT PROMOTE TO SCLM GROUP:
bbbbbbbb
Explanation
The library indicated by the library id is not permitted
to promote to the specified SCLM development group.
SCLM messages
864  z/OS: z/OS ISPF Messages and Codes

## Page 885

User response
Reissue the command against an SCLM development
group to which this library can promote.
FLM89010 LANGUAGE: aaaaaaaa DOES
NOT CONTAIN EXTERNAL
TRANSLATORS
Explanation
An attempt to update an NON-EDITABLE accounting
record failed because the language specified in the
accounting information does not support external
translators.
User response
Place a valid language in the NON-EDITABLE
member’s accounting information and resubmit.
FLM89011 LANGUAGE: aaaaaaaa NOT
DEFINED FOR SCLM
Explanation
An attempt to update an accounting record failed
because the language specified in the accounting
information is not defined to SCLM.
User response
Enter a valid language in the accounting information
and resubmit.
Project manager response
Define the language specified to SCLM.
FLM89012 INVALID ACCOUNTING
INFORMATION KEYWORD: aaaa
Explanation
A keyword specified in the dynamic portion of the
accounting record is not valid. The list of valid
keywords is: INCL, CODE, and USER.
User response
Correct the invalid keyword in the dynamic portion of
the accounting record and resubmit.
FLM89013 INVALID BUILD MAP KEYREF:
aaaa
Explanation
A KEYREF specified in the dynamic portion of the build
map information is not valid.
User response
Correct the invalid keyword in the dynamic portion of
the build map and resubmit.
SCLM messages
Chapter 3. SCLM messages  865

## Page 886

SCLM messages
866  z/OS: z/OS ISPF Messages and Codes
