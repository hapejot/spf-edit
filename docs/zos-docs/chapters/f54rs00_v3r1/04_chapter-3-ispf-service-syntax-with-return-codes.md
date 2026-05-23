# Chapter 3. ISPF service syntax with return codes

Source file: f54rs00_v3r1.md
Start page: 71
Page span: 71-162

## Page 71

Chapter 3. ISPF service syntax with return codes
Here are the ISPF services. The services are presented in alphabetical order. For each service, the
command procedure format is shown, followed by the PL/I call formats. For more complete information,
see the z/OS ISPF Services Guide.
Each service description consists of this information:
Format
The syntax used to code the service, showing both command invocation and call invocation.
Return codes
A description of the codes returned by the service. For all services, a return code of 12 or higher
implies a severe error. This error is usually a syntax error, but can be any severe error detected when
using the services.
The importance of parameter order, and using placeholders for
optional parameters
For several of the ISPF services, the syntax for call invocation (shown under "Call invocation format")
includes a number of optional parameters. These optional parameters are positional; that is:
• Each optional parameter you choose to specify must be specified in the sequence shown in the syntax
diagram.
• Where you specify a parameter without specifying one or more previous optional parameters, the
absence of each previous optional parameter must be indicated by a placeholder in the form of a blank
enclosed in single quotes followed by a comma.
For example, in the call invocation for the ADDPOP service shown here, the first and third optional
parameters have been specified (field-name and column respectively), whilst the second optional
parameter (in this case, row) has been omitted:
CALL ISPLINK ('ADDPOP  ', field-name, ' ', column);
When you do not specify an optional parameter, the default value (if any) for that parameter applies.
It is only necessary to include placeholders for unspecified intermediate parameters. That is, once
you have specified the last optional parameter you want, there is no need to specify placeholders
for subsequent optional parameters (if any); if there are default values for any subsequent optional
parameters, then they will apply.
Further examples:
All optional parameters specified:
CALL ISPLINK ('DISPLAY ', panel-name, message-id, cursor-field-name, cursor-
position, stack-buffer-name, ret-buffer-name, ret-length-name, message-
field-name);
First five optional parameters specified:
CALL ISPLINK ('DISPLAY ', panel-name, message-id, cursor-field-name, cursor-
position, stack-buffer-name);
First, third, and fourth optional parameters specified:
CALL ISPLINK ('DISPLAY ', panel-name, ' ', cursor-field-name, cursor-
position);
Third, fourth, and seventh optional parameters specified:
CALL ISPLINK ('DISPLAY ', ' ', ' ', cursor-field-name, cursor-position, ' ',
' ', ret-length-name);
© Copyright IBM Corp. 1989, 2024 45

## Page 72

Command format
This section describes the general format for ISPF services.
ISPEXEC command invocation
The general format for a command invocation is:
ISPEXEC service-name parameter
ISPEXEC parameter conventions
service-name
Alphabetic; up to 8 characters long.
parameter1
Positional parameter; required for some services.
parameter2 parameter3 …
Keyword parameters. They can take either of two forms:
keyword
   or
keyword(value)
The ISPLINK interface
For calls in PL/I or COBOL, the general call format for invoking ISPF services from functions by using
ISPLINK is:
CALL ISPLINK ( service-name , parameter
' ␣'
);
CALL ISPLINK parameters
These parameters are positional. They must appear in the order described for each service.
Parameters shown below the line are optional, but ISPF assumes default values for those parameters you
do not choose.
If you want to omit a parameter, you must still account for it by inserting a blank enclosed in single quotes
(' ') in its place. This is how you would omit parm2 from this sample call:
CALL ISPLINK (service-name, parm1, ' ', parm3);
If you need only the first few of a list of parameters, you must omit all other parameters to the right of the
last parameter you need. For example, if you are using a service that has five parameters, but you need to
use only the first three, code it like this:
CALL ISPLINK (service-name, parm1, parm2, parm3);
You must show the last parameter in the calling sequence with a ‘1’ as the high order bit in the last
entry of the address list. PL/I, COBOL, Pascal, and FORTRAN call statements automatically generate this
high-order bit. However, you must use the VL keyword in assembler call statements.
Command format
46  z/OS: z/OS ISPF Reference Summary

## Page 73

The ISPEXEC interface
You can use the command function form for service requests in a program function by using the call
format of ISPEXEC. Excluding calls in FORTRAN, Pascal, and APL2®, the general call format for invoking
ISPF services from program functions by using ISPEXEC is:
CALL  ISPEXEC (buf-len, buffer);
CALL ISPEXEC parameters
buf-len
Specifies a fullword fixed binary integer containing the length of the buffer.
buffer
Specifies a buffer containing the name of the service and its parameters just as they would appear in
an ISPEXEC invocation for a command invocation written in CLIST.
The maximum buffer size is 32767 bytes.
All services that are valid through ISPEXEC command invocation statements are valid through the CALL
ISPEXEC interface.
ADDPOP—start pop-up window mode
Command invocation format
ISPEXEC ADDPOP
POPLOC( field-name ) ROW( row)
COLUMN(  column)
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('ADDPOP␣␣' , field-name
'␣'
, row
'␣'
, column
'␣'
);
Return codes
 0
Normal completion.
12
An ADDPOP service call was issued before a panel was displayed or another ADDPOP service call was
issued before a panel was displayed for the previous ADDPOP call.
20
Severe error.
ADDPOP service
Chapter 3. ISPF service syntax with return codes  47

## Page 74

BRIF—Browse interface
Command invocation format
Command procedures cannot be used to invoke this service.
Call invocation format
CALL ISPLINK ('BRIF␣␣␣␣' , data-name
'␣'
, rec-format , rec-len
, read-routine , cmd-routine
'␣'
, dialog-data
'␣'
,
'ISRBROBA'
panel-name
'␣'
, format-name
'␣'
,
'NO␣'
'␣'
'YES'
, 'EXTEND␣␣'
'␣'
);
Read routine return codes
 0
Normal completion.
 4
Temporary end of file.
 8
Record requested beyond end of data. The relative record number of the last data record and a pointer
to the last data record are returned.
16
Read error. Browse data obtained up to the read error is formatted and displayed with an indication
that a read error was encountered.
20
Severe error. (The BRIF service terminates immediately with a return code of 20.)
Command routine return codes
 0
Normal completion.
 4
ISPF should process the requested function.
12
Command deferred; retain the command on the Command line. Browse data is redisplayed.
20
Severe error. (The BRIF service terminates immediately with a return code of 20.)
BRIF service return codes
 0
Normal completion.
BRIF service
48  z/OS: z/OS ISPF Reference Summary

## Page 75

12
No data to browse.
16
Unexpected return code received from a dialog-supplied routine; unable to continue. When an
unexpected return code is received, the BRIF service terminates immediately with a return code
of 16.
20
Severe error; unable to continue.
BROWSE—Browse a data set
Command invocation format
ISPEXEC BROWSE DATASET(  dsname)
VOLUME(  serial )
PASSWORD(  pswd-value ) PANEL( panel-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES) GEN( generation )
OR
ISPEXEC BROWSE DATAID( data-id )
MEMBER(  member-name )
GEN( generation )
PANEL( panel-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
OR
ISPEXEC BROWSE FILE( file-var )
PANEL( panel-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES) RECLEN( rec-len)
BROWSE service
Chapter 3. ISPF service syntax with return codes  49

## Page 76

Call invocation format
CALL ISPLINK ('BROWSE␣␣' , dsname
'␣'
, serial
'␣'
, pswd-value
'␣'
, panel-name
'␣'
, data-id
'␣'
, member-name
'␣'
, format-name
'␣'
,
'NO␣'
'␣'
'YES'
,
file-var
'␣'
, rec-len
'␣'
generation
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
12
Zero-length data; empty sequential data set or z/OS UNIX file, or zero-length member of a partitioned
data set.
13
The specified generation of the member was not found in the specified data sets.
14
Member not found.
15
A non-current generation was specified. None of the specified data sets are PDSE version 2 data sets
that are configured for member generations.
16
Either:
• No members matched the specified pattern.
• No members in the partitioned data set.
18
A VSAM data set was specified but the ISPF Configuration Table does not allow VSAM processing.
20
Severe error; unable to continue.
BROWSE service
50  z/OS: z/OS ISPF Reference Summary

## Page 77

CONTROL—set processing modes
Command invocation format
ISPEXEC CONTROL DISPLAY LOCK
LINE
START(  line-number )
SM
START(  line-number )
REFRESH
SAVE
RESTORE
ALLVALID
NONDISPL
ENTER
END NOSETMSG
ERRORS
CANCEL
RETURN
SPLIT ENABLE
DISABLE
NOCMD
SUBTASK PROTECT
CLEAR
TSOGUI QUERY
OFF
ON
REFLIST UPDATE
NOUPDATE
LE ON
OFF
PASSTHRU LRSCROLL PASQUERY
PASOFF
PASON
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CONTROL service
Chapter 3. ISPF service syntax with return codes  51

## Page 78

CALL ISPLINK ('CONTROL␣' ,
'DISPLAY␣' , 'LOCK␣␣␣␣'
, 'LINE␣␣␣␣'
, line-number
, 'SM␣␣␣␣␣␣'
, line-number
, 'REFRESH␣'
, 'SAVE␣␣␣␣'
, 'RESTORE␣'
, 'ALLVALID'
'NONDISPL'
, 'ENTER␣␣␣'
, '␣'
, 'END␣␣␣␣␣' , '␣' , 'NOSETMSG'
'ERRORS␣␣'
, 'CANCEL␣␣'
, 'RETURN␣␣'
'SPLIT␣␣␣' , 'ENABLE␣␣'
, 'DISABLE␣'
'NOCMD␣␣␣'
'SUBTASK␣' , 'PROTECT␣'
, 'CLEAR␣␣␣'
'TSOGUI␣␣' , 'QUERY␣␣␣'
, 'OFF␣␣␣␣␣'
, 'ON␣␣␣␣␣␣'
'REFLIST␣' , 'UPDATE␣␣'
, 'NOUPDATE'
'LE␣␣␣␣␣␣' , 'ON␣␣␣␣␣␣'
, 'OFF␣␣␣␣␣'
'PASSTHRU' , 'LRSCROLL' , '␣' , 'PASQUERY'
'PASOFF␣␣'
'PASON␣␣␣'
);
Return codes
 0
Normal completion.
 8
Split-screen mode already in effect. Applies only to a SPLIT DISABLE request. Split-screen mode
remains enabled.
20
Severe error.
CONTROL service
52  z/OS: z/OS ISPF Reference Summary

## Page 79

DIRLIST—directory list service
Command invocation format
ISPEXEC DIRLIST PATH( path-var )
CONFIRM(YES)
CONFIRM(NO)
CONFDRD(YES)
CONFDRD(NO)
PANEL( panel-name ) COLS( column-list )
FIXCOLS(YES)
FIXCOLS(NO)
LCMDS( line-command-list ) FROM( file-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('DIRLIST␣' , path-var ,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, panel-name
'␣'
, column-list
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, line-command-list
'␣'
, file-name
'␣'
);
Return codes
 0
Normal completion.
 8
Error building the directory list. The error condition is described in the ISPF system dialog variables.
12
A keyword value is incorrect.
20
A severe error occurred while processing the directory list.
DIRLIST service
Chapter 3. ISPF service syntax with return codes  53

## Page 80

DISPLAY—display panels and messages
Command invocation format
ISPEXEC DISPLAY
PANEL( panel-name ) MSG( message-id )
CURSOR(  cursor-field-name ) CSRPOS(  cursor-position )
COMMAND(  stack-buffer-name ) RETBUFFR(  ret-buffer-name )
RETLGTH(  ret-length-name ) MSGLOC(  message-field-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('DISPLAY␣' , panel-name
'␣'
, message-id
'␣'
, cursor-field-name
'␣'
, cursor-position
'␣'
, stack-buffer-name
'␣'
, ret-buffer-name
'␣'
, ret-length-name
'␣'
, message-field-name
'␣'
);
Return codes
 0
Normal completion.
For the COMMAND option, the ret-buffer-name is set to blanks and the ret-length-name is set to zero.
Passing an empty command chain buffer also results in a normal completion.
 4
One or more commands in the stack could not be found in the active set of command tables.
 8
User requested termination using the END or RETURN command. If CANCEL and EXIT are requested
from a panel displayed using the DISPLAY service call and the panel was defined with the dialog tag
language (DTL), the dialog manager returns the command in ZVERB and sets a return code of 8 from
the display screen.
12
The specified panel, message, message location field, or cursor field could not be found.
16
Truncation or translation error in storing defined variables.
DISPLAY service
54  z/OS: z/OS ISPF Reference Summary

## Page 81

20
Severe error.
DSINFO—data set information dialog
Command invocation format
ISPEXEC DSINFO DATASET(  dsname)
VOLUME(  serial )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('DSINFO␣␣' , dsname , serial
'␣'
);
Return codes
 0
Normal completion.
 8
User requested information unavailable. Dialog error variables (ZERRLM, and so on) contain further
information.
12
One of these:
• Internal Service Failure
• Error when using the OBTAIN macro to read the DSCB
• Error obtaining directory information
20
Severe error.
EDIF—Edit interface
Command invocation format
You cannot use command procedures to invoke this service.
DSINFO service
Chapter 3. ISPF service syntax with return codes  55

## Page 82

Call invocation format
CALL ISPLINK ('EDIF␣␣␣␣' , data-name
'␣'
, profile-name , rec-format
, rec-len , read-routine , write-routine , cmd-routine
'␣'
, dialog-data
'␣'
, edit-len
'␣'
, panel-name
'␣'
, macro-name
'␣'
, format-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, parm-var
'␣'
, tabname
'␣'
);
OR
CALL ISPLINK ('EDIF␣␣␣␣' , data-name
'␣'
,'␣' , rec-format
'␣'
, rec-len
'␣'
, read-routine , write-routine , cmd-routine
'␣'
,
dialog-data
'␣'
,'␣' ,'␣' ,'␣' ,'␣' ,'␣'
,'YES␣␣␣␣␣' ,'␣' , tabname
'␣'
);
Read routine return codes
 0
Normal completion.
 8
End of data records (no data record returned).
16
Read error. If a read error is encountered when the system builds the initial edit display, the EDIF
service terminates with a return code of 20. Otherwise, the edit data is redisplayed.
20
Severe error. (The EDIF service terminates immediately with a return code of 20.)
Write routine return codes
 0
Normal completion.
EDIF service
56  z/OS: z/OS ISPF Reference Summary

## Page 83

16
Output error, return to Edit mode.
20
Severe error. (The EDIF service terminates immediately with a return code of 20.)
Command routine return codes
 0
Normal completion.
 4
ISPF should process the requested function.
12
Command deferred; retain the command on the Command line. Edit data is redisplayed.
20
Severe error. (The EDIF service terminates immediately with a return code of 20.)
EDIF return codes
 0
Normal completion, data saved.
 4
Normal completion, data not saved.
16
Unexpected return code received from a dialog-supplied routine. When an unexpected return code is
received, the EDIF service terminates immediately with a return code of 16.
20
Severe error; unable to continue.
EDIREC—initialize edit recovery
Command invocation format
You cannot use command procedures to invoke this service.
Call invocation format
CALL ISPLINK ('EDIREC␣␣' , 'INIT␣␣␣␣' , command-name
'␣'
'QUERY␣␣␣'
'CANCEL␣␣'
'DEFER␣␣␣'
);
Return codes
 0
Normal completion.
• INIT - EDIF recovery table was successfully created.
• QUERY - Recovery is not pending.
 4
Normal completion.
• INIT - EDIF recovery table already exists for current application.
EDIREC service
Chapter 3. ISPF service syntax with return codes  57

## Page 84

• QUERY - Entry found in EDIF recovery table (recovery is pending).
20
Severe error; unable to continue.
EDIT—edit a data set
Command invocation format
ISPEXEC EDIT DATASET(  dsname)
VOLUME(  serial )
PASSWORD(  pswd-value ) PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
LOCK(NO)
LOCK(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES) WRAP
PRESERVE PARM( parm-var ) ASCII
UTF8
LINECMDS(  tabname ) GEN( generation )
OR
EDIT service
58  z/OS: z/OS ISPF Reference Summary

## Page 85

ISPEXEC EDIT DATAID( data-id )
MEMBER(  member-name )
GEN( generation )
PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
LOCK(NO)
LOCK(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES) WRAP
PRESERVE PARM( parm-var ) ASCII
UTF8
LINECMDS(  tabname )
OR
ISPEXEC EDIT FILE( file-var )
PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
LOCK(NO)
LOCK(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES) WRAP
PRESERVE PARM( parm-var ) RECLEN( rec-len) ASCII
UTF8
LINECMDS(  tabname )
EDIT service
Chapter 3. ISPF service syntax with return codes  59

## Page 86

Call invocation format
CALL ISPLINK ('EDIT␣␣␣␣' , dsname
'␣'
, serial
'␣'
, pswd-value
'␣'
, panel-name
'␣'
, macro-name
'␣'
, profile-name
'␣'
, data-id
'␣'
, member-name
'␣'
, format-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, ws-filename-buffer-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, 'WRAP␣␣␣␣'
'␣'
, 'PRESERVE'
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, parm-var
'␣'
, file-var
'␣'
, rec-len
'␣'
, 'ASCII␣␣␣'
'UTF8␣␣␣␣'
'␣'
, tabname
'␣'
, generation
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion; data was saved.
 4
Normal completion; data was not saved for one of these reasons.
• No data changes were made during the EDIT session.
• The CANCEL command was used to exit EDIT.
• Browse was substituted for EDIT because insufficient storage was available to read in the requested
data.
EDIT service
60  z/OS: z/OS ISPF Reference Summary

## Page 87

9
The specified generation of the member was not found in the specified data sets.
10
Member not found.
11
A non-current generation was specified. None of the specified data sets are PDSE version 2 data sets
that are configured for member generations.
12
YES was specified for the LOCK parameter or the ws-filename-buffer-name parameter was specified.
14
Member, sequential data set, or z/OS UNIX file in use.
16
Either:
• No members matched the specified pattern.
• No members in the partitioned data set.
18
A VSAM data set was specified but the ISPF Configuration Table does not allow VSAM processing.
20
Severe error; unable to continue.
EDREC—specify edit recovery handling
Command invocation format
ISPEXEC EDREC
INIT
CMD( command-name )
QUERY
PROCESS
PASSWORD(  pswd-value ) DATAID( data-id )
CANCEL
DEFER
Call invocation format
CALL ISPLINK ('EDREC␣␣␣'
,'INIT␣␣␣␣' , command-name
'␣'
,'QUERY␣␣␣'
,'PROCESS␣' , pswd-value
'␣'
, data-id
'␣'
,'CANCEL␣␣'
,'DEFER␣␣␣'
);
OR
EDREC service
Chapter 3. ISPF service syntax with return codes  61

## Page 88

CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal return.
INIT
Edit recovery table was successfully created.
QUERY
Recovery is not pending.
PROCESS
Recovery was completed and the data was saved.
 4
Normal return.
INIT
Edit recovery table already exists for current application.
QUERY
Entry found in edit recovery table; recovery is pending.
PROCESS
Recovery was completed, but user did not save data.
20
Severe error; unable to continue.
FTCLOSE—end file tailoring
Command invocation format
ISPEXEC FTCLOSE
NAME( member-name ) LIBRARY(  library )
NOREPL
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('FTCLOSE␣' , member-name
'␣'
, library
'␣'
, 'NOREPL␣␣'
'␣'
);
Return codes
 0
Normal completion.
FTCLOSE service
62  z/OS: z/OS ISPF Reference Summary

## Page 89

4
Member already exists in the output library and NOREPL was specified. The original member is
unchanged.
 8
File not open. FTOPEN was not used before FTCLOSE.
12
Output file in use. ENQ failed.
16
Skeleton library or output file not allocated.
20
Severe error.
FTERASE—erase file tailoring output
Command invocation format
ISPEXEC FTERASE member-name
LIBRARY(  library )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('FTERASE␣' , member-name , library
'␣'
);
Return codes
 0
Normal completion.
 8
File does not exist.
12
Output file in use; ENQ failed.
16
Alternate output library not allocated.
20
Severe error.
FTINCL—include a skeleton
Command invocation format
ISPEXEC FTINCL skel-name
NOFT EXT
FTERASE service
Chapter 3. ISPF service syntax with return codes  63

## Page 90

Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('FTINCL␣␣' , skel-name , 'NOFT␣␣␣␣'
'␣'
, 'EXT␣␣␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 8
Skeleton does not exist.
12
Skeleton in use; ENQ failed.
16
Data truncation occurred or skeleton library or output file not allocated.
20
Severe error.
FTOPEN—begin file tailoring
Command invocation format
ISPEXEC FTOPEN
TEMP
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('FTOPEN␣␣' , 'TEMP␣␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 8
File tailoring already in progress.
16
Skeleton library or output file not allocated.
12
Output file in use; ENQ failed.
FTOPEN service
64  z/OS: z/OS ISPF Reference Summary

## Page 91

20
Severe error.
GETMSG—get a message
Command invocation format
ISPEXEC GETMSG MSG( message-id )
SHORTMSG(  short-message-name )
LONGMSG(  long-message-name ) ALARM(  alarm-name )
HELP( help-name ) TYPE( type-name ) WINDOW(  window-name )
CCSID( ccsid-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('GETMSG␣␣' , message-id , short-message-name
'␣'
, long-message-name
'␣'
, alarm-name
'␣'
, help-name
'␣'
, type-name
'␣'
, window-name
'␣'
, ccsid-name
'␣'
);
Return codes
 0
Normal completion.
12
The specified message could not be found.
20
Severe error.
GRERROR—graphics error block service
Command invocation format
ISPEXEC  *This service does not apply to
          command or APL2 procedures*
GETMSG service
Chapter 3. ISPF service syntax with return codes  65

## Page 92

Call invocation format
CALL  ISPEXEC  *This service cannot be used
                with this interface*
CALL ISPLINK ('GRERROR␣' , error-record-pointer ,
call-format-descriptor-module-pointer   );
Return codes
 0
Normal completion
 8
ISPF/GDDM interface is not established
20
Severe error.
GRINIT—graphics initialization
Command invocation format
ISPEXEC  *This service does not apply to
          command or APL2 procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used
                with this interface*
OR
CALL ISPLINK ('GRINIT␣␣' , application-anchor-block , panel-name
'␣'
);
Return codes
 0
Normal completion.
 8
The specified panel does not contain a GRAPHIC area.
12
The specified panel could not be found.
20
Severe error.
GRTERM—graphics termination service
Command invocation format
ISPEXEC  *This service does not apply to
          command or APL2 procedures*
GRINIT service
66  z/OS: z/OS ISPF Reference Summary

## Page 93

Call invocation format
CALL  ISPEXEC  *This service cannot be used
                with this interface*
OR
CALL ISPLINK ('GRTERM␣␣');
Return codes
 0
Normal completion
20
Severe error.
LIBDEF—allocate application libraries
Command invocation format
Note: If none of the processing options COND, UNCOND, STACK, or STKADD is specified,
the processing option is set using the value in the ISPF configuration table keyword
DEFAULT_LIBDEF _PROCESSING_OPTION. If this keyword is not set in the ISPF configuration table, the
processing option is set to the default value UNCOND. Always specify a processing option for the LIBDEF
service to ensure that changes to the DEFAULT_LIBDEF _PROCESSING_OPTION value in the configuration
table do not cause unexpected changes to your dialog processing.
ISPEXEC LIBDEF lib-type
DATASET
EXCLDATA
LIBRARY
EXCLLIBR
ID( dataset-list )
ID( libname )
COND
UNCOND
STACK
STKADD
Call invocation format
Note: If none of the processing options COND, UNCOND, STACK, or STKADD is specified,
the processing option is set using the value in the ISPF configuration table keyword
DEFAULT_LIBDEF _PROCESSING_OPTION. If this keyword is not set in the ISPF configuration table, the
processing option is set to the default value UNCOND. Always specify a processing option for the LIBDEF
service to ensure that changes to the DEFAULT_LIBDEF _PROCESSING_OPTION value in the configuration
table do not cause unexpected changes to your dialog processing.
CALL ISPEXEC ( buf-len , buffer);
OR
LIBDEF service
Chapter 3. ISPF service syntax with return codes  67

## Page 94

CALL ISPLINK ('LIBDEF␣␣' , lib-type , '␣'
'DATASET␣'
'EXCLDATA'
'LIBRARY␣'
'EXCLLIBR'
, '␣'
dataset-list
libname
, '␣'
'COND␣␣␣␣'
'UNCOND␣␣'
'STACK␣␣␣'
'STKADD␣␣'
);
Return codes
 0
Normal completion.
 4
When removing the application library: Application library does not exist for this type.
When STKADD is specified: There is no existing stack.
 8
When COND is used: Application library already exists for this type.
12
ISPPROF was specified as the lib-type; invalid lib-type specified with EXCLDATA or EXCLLIBR.
16
A libname was not allocated, or the dataset-list contains an invalid MVS™ dsname.
20
Severe error.
LIST—write lines to the list data set
Command invocation format
ISPEXEC LIST BUFNAME(  dialog-variable-name ) LINELEN(  line-length )
PAGE
SINGLE
DOUBLE
TRIPLE
OVERSTRK CC
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
LIST service
68  z/OS: z/OS ISPF Reference Summary

## Page 95

CALL ISPLINK ('LIST␣␣␣␣' , dialog-variable-name , line-length
, 'PAGE␣␣␣␣'
'␣'
,
'SINGLE␣␣'
'␣'
'DOUBLE␣␣'
'TRIPLE␣␣'
, 'OVERSTRK'
'␣'
, 'CC␣␣␣␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 8
Maximum line length or data set LRECL exceeded; data has been truncated.
12
Specified dialog variable not found.
20
Severe error.
LMCLOSE—close a data set
Command invocation format
ISPEXEC LMCLOSE DATAID( data-id )
Call invocation format
CALL ISPLINK ('LMCLOSE␣', data-id );
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
Data set is not open.
10
No ISPF library or data set associated with the given data ID; that is, LMINIT has not been completed.
20
Severe error; unable to continue.
LMCLOSE service
Chapter 3. ISPF service syntax with return codes  69

## Page 96

LMCOMP—compresses a partitioned data set
Command invocation format
ISPEXEC LMCOMP DATAID( data-id )
Call invocation format
CALL ISPLINK ('LMCOMP␣␣',  data-id );
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 8
Library type is a PDSE and cannot be compressed
10
No data set associated with the given data ID.
12
One of these:
• Data set not partitioned
• Data set specified not allocated
• Data set is open
• Data set is not movable
• Data set must be allocated exclusively. Use ENQ(EXCLU) in LMINIT service.
• Concatenated libraries are not allowed for LMCOMP.
20
Severe error; unable to continue.
LMCOPY—copy members of a data set
Command invocation format
LMCOMP service
70  z/OS: z/OS ISPF Reference Summary

## Page 97

ISPEXEC LMCOPY FROMID( from-data-id )
FROMMEM(  from-member-name )
TODATAID(  to-data-id )
TOMEM(  to-member-name ) REPLACE
PACK TRUNC LOCK SCLMSET( YES
NO
)
ALIAS
NOALIAS
Call invocation format
CALL ISPLINK ('LMCOPY␣␣' , from-data-id , from-member-name
'␣'
, to-data-id , to-member-name
'␣'
, 'REPLACE'
'␣'
, 'PACK'
'␣'
, 'TRUNC'
'␣'
, 'LOCK␣␣␣␣'
'␣'
, '␣'
'YES␣␣␣␣␣'
'NO␣␣␣␣␣␣'
,
'ALIAS␣'
'␣'
'NOALIAS'
);
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Normal completion.
 4
Member not available, which indicates one of these situations:
• The "from" data set is empty.
• No members matched the specified pattern in the "from" data set.
 8
• The from-member-name was not found.
• The same name was specified for to-member-name and from-member-name.
10
No data set is associated with the given data ID.
12
One of these:
LMCOPY service
Chapter 3. ISPF service syntax with return codes  71

## Page 98

• A like-named member already exists in the "to" data set and the Replace option was not specified
• One or more members of the "to" data set are "in use", either by you or by another user, and could
not be copied
• Invalid data set organization
• Data set attribute invalid for copying or copying packed data
• Open error
• LOCK parameter is specified
16
Truncation error.
20
Severe error; unable to continue.
LMDDISP—data set display service
Command invocation format
ISPEXEC LMDDISP LISTID( dslist-id )
VIEW(VOLUME)
VIEW(SPACE)
VIEW(ATTRIB)
VIEW(TOTAL)
CONFIRM(YES)
CONFIRM(NO)
PANEL( panel-name )
CATALOG(NO)
CATALOG(YES)
TOTALS(NO)
TOTALS(YES)
STATUS(NO)
STATUS(YES)
EXDATE(NO)
EXDATE(YES)
REFLIST(NO)
REFLIST(YES)
Call invocation format
CALL ISPEXEC ( buf-len , buffer );
OR
LMDDISP service
72  z/OS: z/OS ISPF Reference Summary

## Page 99

CALL ISPLINK('LMDDISP␣' , dslist-id ,
'VOLUME␣␣'
'␣'
'SPACE␣␣␣'
'ATTRIB␣␣'
'TOTAL␣␣␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, panel-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
);
Return codes
 0
Normal completion.
 8
Error building data set list. The error condition is described in the ISPF system dialog variables.
10
A data set list does not exist for the list-id specified via keyword LISTID.
12
A keyword value is incorrect.
20
A severe error occurred while processing the data set list.
LMDFREE—free a data set list
Command invocation format
ISPEXEC LMDFREE LISTID( list-id )
Call invocation format
CALL ISPLINK ('LMDFREE␣', list-id);
OR
CALL ISPEXEC ( buf-len , buffer);
LMDFREE service
Chapter 3. ISPF service syntax with return codes  73

## Page 100

Return codes
 0
Normal completion.
 8
Free dslist ID failed. For more information about the error condition, see System variables used to
format error messagesin ISPF Services Guide.
10
No data set level or volume is associated with given dslist ID. LMDINIT has not been completed.
20
Severe error; unable to continue.
LMDINIT—initialize a data set list
Command invocation format
ISPEXEC LMDINIT LISTID( dslist-id-var )
LEVEL( dsname-level )
VOLUME(  volume-serial )
Call invocation format
CALL ISPLINK ('LMDINIT␣' , dslist-id-var , dsname-level
'␣'
, volume-serial
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion. LMDINIT returns a unique dslist ID in the variable specified in keyword LISTID.
 8
The dslist ID was not created; for more information about the error condition, see System variables
used to format error messagesin ISPF Services Guide.
12
A keyword value is incorrect.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMDINIT service
74  z/OS: z/OS ISPF Reference Summary

## Page 101

LMDLIST—list a data set
Command invocation format
ISPEXEC LMDLIST LISTID( dslist-id )
OPTION(LIST)
OPTION(FREE)
OPTION(SAVE)
OPTION(SAVEC)
OPTION(TOTALS)
DATASET(  dataset-var )
STATS(NO)
STATS(YES)
STATS(PRT)
GROUP(  group)
STATUS(NO)
STATUS(YES)
Call invocation format
CALL ISPLINK ('LMDLIST␣' , dslist-id ,
'LIST␣␣␣␣'
'␣'
'FREE␣␣␣␣'
'SAVE␣␣␣␣'
'SAVEC␣␣␣'
'TOTALS␣␣'
,
dataset-var
'␣'
,
'NO␣'
'␣'
'YES'
'PRT'
, group
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
One of these:
• LIST option - Normal completion. The name of next data set in the list is returned in the variable
specified in keyword DATASET. Data set statistics are returned, if requested.
LMDLIST service
Chapter 3. ISPF service syntax with return codes  75

## Page 102

• FREE option - Normal completion. The internal storage associated with the data set list has been
freed.
• SAVE option - Normal completion. The data set list has been successfully written to a data set.
The total number of tracks and datasets are returned to dialog variables in the function pool, if
requested.
• SAVEC option - Normal completion. The data set list has been successfully written to a data set.
The total number of tracks and datasets are returned to dialog variables in the function pool, if
requested.
• TOTALS option - Normal completion. No list has been written to a dataset. The total number of
tracks and datasets are returned into dialog variables in the function pool.
 4
One of these:
• No data sets matched specified search criteria (the values for keywords LEVEL and VOLUME on the
LMDINIT service).
• An incomplete VTOC list. An entry was found in the VTOC index but the volume was not available.
The name in the index has not been added to the data set list.
 8
End of data set list.
10
The data set list does not exist for dslist ID.
12
A keyword value is incorrect.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMERASE—erase a data set
Command invocation format
ISPEXEC LMERASE PROJECT(  project ) GROUP(  group) TYPE( type)
DATASET(  dataset )
PURGE(NO)
PURGE(YES) VOLUME(  volume) PASSWORD(  password )
LMERASE service
76  z/OS: z/OS ISPF Reference Summary

## Page 103

Call invocation format
CALL ISPLINK('LMERASE␣' , project
'␣'
, group
'␣'
, type
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, dataset
'␣'
, volume
'␣'
, password
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
One of these:
• Data set is not cataloged or other allocation failure.
• Data set delete failed.
• Data set name is an alias.
• Expiration date not expired and PURGE parameter omitted
• No data set specified as input
• PROJECT specified, but GROUP or TYPE not specified.
12
Expiration date not expired and PURGE(NO) specified.
20
Severe error; unable to continue.
LMFREE—free data set from its association with data ID
Command invocation format
ISPEXEC LMFREE DATAID( data-id )
Call invocation format
CALL ISPLINK('LMFREE␣␣',  data-id );
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
These return codes are possible:
LMFREE service
Chapter 3. ISPF service syntax with return codes  77

## Page 104

0
Normal completion.
 8
Free data ID failed; for more information about the error condition, see System variables used to
format error messagesin ISPF Services Guide.
10
No ISPF library or data set is associated with the given data ID; that is, LMINIT has not been
completed.
20
Severe error; unable to continue.
LMGET—read a logical record from a data set
Command invocation format
ISPEXEC LMGET DATAID( data-id ) MODE( MOVE
LOCATE
INVAR
MULTX
)
DATALOC(  dataloc-var ) DATALEN(  datalen-var ) MAXLEN( max-length )
Call invocation format
CALL ISPLINK ('LMGET␣␣␣',  data-id , 'MOVE␣␣␣␣'
'LOCATE␣␣'
'INVAR␣␣␣'
'MULTX␣␣␣'
, dataloc-var
, datalen-var , max-length );
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
End-of-data set condition; no message formatted.
10
No ISPF library or data set associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The data set is not open or is not open for input.
• An LMMFIND was not done for a partitioned data set.
• The parameter value is invalid.
LMGET service
78  z/OS: z/OS ISPF Reference Summary

## Page 105

16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMINIT—generate a data ID for a data set
Command invocation format
ISPEXEC LMINIT DATAID( data-id-var ) PROJECT options
DATASET(  dsname)
DDNAME(  ddname)
VOLUME(  serial ) PASSWORD(  password )
ENQ(SHR)
ENQ(EXCLU)
ENQ(SHRW)
ENQ(MOD)
ORG( org-var )
PROJECT options
PROJECT(  project ) GROUP1(  group1) TYPE( type)
GROUP2(  group2)
GROUP3(  group3) GROUP4(  group4)
Call invocation format
CALL ISPLINK ('LMINIT␣␣' , data-id-var
'␣' , '␣' , '␣' , '␣' , '␣' , '␣' , '␣' , ddname
'␣' , '␣' , '␣' , '␣' , '␣' , '␣' , , dsname , '␣'
, project, group1 , group2
'␣'
, group3
'␣'
, group4
'␣'
, type , '␣' , '␣'
, serial
'␣'
, password
'␣'
,
'SHR␣␣␣␣␣'
'␣'
'EXCLU␣␣␣'
'SHRW␣␣␣␣'
'MOD␣␣␣␣␣'
, org-var
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer );
LMINIT service
Chapter 3. ISPF service syntax with return codes  79

## Page 106

Return codes
These return codes are possible:
 0
Normal completion.
 8
Data ID not created; for more information about the error condition, see System variables used to
format error messagesin ISPF Services Guide.
12
The parameter value is invalid.
16
Truncation or translation error in accessing dialog variables.
20
Severe error; unable to continue.
For more information about dialog variables, see System variables used to format error messagesin
ISPF Services Guide.
Note: Data sets allocated with an XTIOT will return a "DDNAME Not Found" message and set RC=8 if
XTIOT support is not fully enabled.
Note: Data sets allocated with an XTIOT will return a "DDNAME Not Found" message and set RC=8 if
XTIOT support is not fully enabled.
LMMADD—add a member to a data set
Command invocation format
ISPEXEC LMMADD DATAID( data-id ) MEMBER(  member-name )
STATS(
NO
YES )
NOENQ
EXT(
NO
YES )
Call invocation format
CALL ISPLINK ('LMMADD␣␣' , data-id , member-name ,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, 'NOENQ'
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
LMMADD service
80  z/OS: z/OS ISPF Reference Summary

## Page 107

4
The directory already contains the specified name.
10
No ISPF library or MVS data set is associated with the given data ID; that is, LMINIT has not been
completed.
12
One of these:
• The data set is not open or is not open for output.
• The parameter value is invalid.
• The data set organization is invalid.
• The values for some member statistics are invalid.
14
No record has been written for the member to be added.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMDEL—delete members from a data set
Command invocation format
ISPEXEC LMMDEL DATAID( data-id ) MEMBER(  member-name )
NOENQ
Call invocation format
CALL ISPLINK('LMMDEL␣␣' , data-id , member-name , 'NOENQ␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
The member was not found.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The data set is not open or is not open for output.
• The parameter value is invalid.
• The data set organization is invalid.
20
Severe error; unable to continue.
LMMDEL service
Chapter 3. ISPF service syntax with return codes  81

## Page 108

LMMDISP—member list service: Display option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id )
OPTION(DISPLAY)
MEMBER(  pattern )
STATS(NO)
STATS(YES) PANEL( panel-name )
CURSOR(ZCMD)
CURSOR(ZLLCMD)
CURSOR(ZLUDATA)
TOP( top-row )
COMMANDS(S)
COMMANDS(ANY)
FIELD(1)
FIELD(9) ALLOWNEW
Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,
'DISPLAY␣'
'␣' , pattern
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, panel-name
'␣'
,
'ZCMD␣␣␣␣'
'␣'
'ZLLCMD␣␣'
'ZLUDATA␣'
,
top-row
'␣'
,'␣' ,'␣' ,
'S␣␣␣␣␣␣␣'
'␣'
'ANY␣␣␣␣␣'
,
1
'␣'
9
,
'ALLOWNEW'
'␣'
);
OR
LMMDISP service
82  z/OS: z/OS ISPF Reference Summary

## Page 109

CALL ISPLINK('LMMDISP␣' , data-id ,
'DISPLAY␣'
'␣' , pattern
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, panel-name
'␣'
,
'ZCMD␣␣␣␣'
'␣'
'ZLLCMD␣␣'
'ZLUDATA␣'
,
top-row
'␣'
,'␣' ,'␣' ,
'S␣␣␣␣␣␣␣'
'␣'
'ANY␣␣␣␣␣'
,
1
'␣'
9
,
'ALLOWNEW'
'␣'
);
Return codes
 0
One or more members were selected and/or a primary command not recognized by LMMDISP was
entered.
 4
The requested data sets were empty, or no members matched the specified pattern.
 8
END or RETURN was entered.
10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data set not open.
• Data set not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMDISP—member list service: GET option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id ) OPTION(GET)
STATS(NO)
STATS(YES)
LMMDISP service
Chapter 3. ISPF service syntax with return codes  83

## Page 110

Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,'GET␣␣␣␣␣' ,'␣' ,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Successful completion.
 8
No more selected members.
10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data set not open.
• Data set not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
• Member list has not been created.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMDISP—member list service: PUT option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id ) OPTION(PUT) MEMBER(  member-name )
ZLLCMD(  lcmd-value ) ZLUDATA(  udata-value )
Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,'PUT␣␣␣␣␣' , member-name ,'␣' ,'␣'
,'␣' ,'␣' , lcmd-value
'␣'
, udata-value
'␣'
);
OR
LMMDISP service
84  z/OS: z/OS ISPF Reference Summary

## Page 111

CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 8
A specified member does not exist in the member list.
10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data sets not open.
• Data sets not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
• Member list has not been created.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMDISP—member list service: ADD option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id ) OPTION(ADD) MEMBER(  member-name )
ZLLCMD(  lcmd-value ) ZLUDATA(  udata-value )
Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,'ADD␣␣␣␣␣' , member-name ,'␣' ,'␣'
,'␣' ,'␣' , lcmd-value
'␣'
, udata-value
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 8
The member already exists in the member list.
LMMDISP service
Chapter 3. ISPF service syntax with return codes  85

## Page 112

10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data sets not open.
• Data sets not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
• Member list has not been created.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMDISP—member list service: DELETE option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id ) OPTION(DELETE) MEMBER(  member-name )
Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,'DELETE␣␣' , member-name );
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 8
A specified member does not exist in the member list.
10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data sets not open.
• Data sets not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
• Member list has not been created.
16
A truncation or translation error occurred in accessing dialog variables.
LMMDISP service
86  z/OS: z/OS ISPF Reference Summary

## Page 113

20
Severe error; unable to continue.
LMMDISP—member list service: FREE option
Command invocation format
ISPEXEC LMMDISP DATAID( data-id ) OPTION(FREE)
Call invocation format
CALL ISPLINK('LMMDISP␣' , data-id ,'FREE␣␣␣␣');
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 8
No member list is associated with the given data ID.
10
No data set is associated with the given data ID; LMINIT has not been completed.
12
Indicates one of these conditions:
• Data sets not open.
• Data sets not partitioned.
• Invalid parameter value.
• Invalid data set organization.
• Invalid invocation syntax.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMFIND—find a library member
Command invocation format
ISPEXEC LMMFIND DATAID( data-id ) MEMBER(  member-name )
LOCK
LRECL( lrecl-var ) RECFM( recfm-var ) GROUP(  group-var )
STATS(NO)
STATS(YES) NOLLA
LMMDISP service
Chapter 3. ISPF service syntax with return codes  87

## Page 114

Call invocation format
CALL ISPLINK ('LMMFIND␣' , data-id , member-name , 'LOCK␣␣␣␣'
'␣'
, lrecl-var
'␣'
, recfm-var
'␣'
, group-var
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, 'NOLLA␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
Member not found.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• Data set is not open or is not open for input.
• A parameter value is invalid.
• Data set is not partitioned.
• LOCK parameter was specified.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMFIND service
88  z/OS: z/OS ISPF Reference Summary

## Page 115

LMMLIST—list a library's members
Command invocation format
ISPEXEC LMMLIST DATAID( data-id )
OPTION(LIST)
OPTION(FREE)
OPTION(SAVE)
MEMBER(  member-var )
STATS(NO)
STATS(YES) GROUP(  group)
PATTERN(  member-pattern ) LONG
Call invocation format
CALL ISPLINK ('LMMLIST␣' , data-id ,
'LIST'
'␣'
'FREE'
'SAVE'
, member-var
'␣'
,
'NO␣'
'␣'
'YES'
, group
'␣'
, member-pattern
'␣'
, 'LONG␣␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
One of these:
• LIST option - Normal completion. The member list is available and the next member in the list is
returned in the member-var parameter.
• FREE option - Normal completion. The member list is freed successfully.
• SAVE option - Normal completion. The member list is successfully written to a data set.
 4
Empty member list.
 8
One of these:
• LIST option - End of member list.
• FREE option - Member list does not exist.
LMMLIST service
Chapter 3. ISPF service syntax with return codes  89

## Page 116

• SAVE option - For a data ID, the LMMLIST service has been invoked with the SAVE option after being
invoked with LIST option, but before being invoked with the FREE option.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The data set is not open or is not partitioned.
• A parameter value is invalid.
• Member list was created using LMMDISP.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error; unable to continue.
LMMOVE—move members of a data set
Command invocation format
ISPEXEC LMMOVE FROMID( from-data-id )
FROMMEM(  from-member-name )
TODATAID(  to-data-id )
TOMEM(  to-member-name ) REPLACE
PACK TRUNC LOCK SCLMSET( Y
N
)
ALIAS
NOALIAS
Call invocation format
CALL ISPLINK ('LMMOVE␣␣' , from-data-id , from-member-name
'␣'
, to-data-id , to-member-name
'␣'
, 'REPLACE'
'␣'
, 'PACK'
'␣'
, 'TRUNC␣␣␣'
'␣'
, 'LOCK␣␣␣␣'
'␣'
, '␣'
'YES'
'NO␣'
,
'ALIAS␣␣'
'␣'
'NOALIAS'
);
OR
LMMOVE service
90  z/OS: z/OS ISPF Reference Summary

## Page 117

CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Successful completion.
 4
Either:
• "From" data set is empty.
• No member matched the pattern in the "from" data set.
 8
"From" member not found.
10
No data set is associated with given data ID.
12
One of these:
• A like-named member already exists in the “to” data set and the Replace option was not specified.
• One or more members of the 'TO' or 'FROM' data sets are "in use" by you or another user and could
not be moved.
• Invalid data set organization.
• Data set attribute invalid for packed data.
• Open error.
16
A truncation error occurred.
20
Severe error; unable to continue.
LMMREN—rename a data set member
Command invocation format
ISPEXEC LMMREN DATAID( data-id ) MEMBER(  old-member-name )
NEWNAME(  new-member-name )
NOENQ
Call invocation format
CALL ISPLINK('LMMREN␣␣' , data-id , old-member-name , new-member-name
, 'NOENQ␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
LMMREN service
Chapter 3. ISPF service syntax with return codes  91

## Page 118

Return codes
 0
Normal completion.
 4
Directory already contains the specified new name.
 8
Member not found.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The data set is not open or is not open for output.
• The parameter value is invalid.
• The data set organization is invalid.
20
Severe error; unable to continue.
LMMREP—replace a member of a data set
Command invocation format
ISPEXEC LMMREP DATAID( data-id ) MEMBER(  member-name )
STATS(NO)
STATS(YES)
NOENQ
EXT(NO)
EXT(YES)
Call invocation format
CALL ISPLINK('LMMREP␣␣' , data-id , member-name ,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, 'NOENQ␣␣␣'
'␣'
,
'NO␣␣␣␣␣␣
'␣'
'YES␣␣␣␣␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
LMMREP service
92  z/OS: z/OS ISPF Reference Summary

## Page 119

8
Member is added; it did not previously exist.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The data set is not open or is not open for output.
• The parameter value is invalid.
• The data set organization is invalid.
• Some member statistics have invalid values.
14
No record has been written for the member to be replaced.
16
Truncation or translation error in accessing dialog variables.
20
Severe error; unable to continue.
LMMSTATS—set and store ISPF statistics
Command invocation format
ISPEXEC LMMSTATS DATAID( data-id ) MEMBER(  member-name )
VERSION(  version-number ) MODLEVEL(  mod-level )
CREATED(  create-date ) MODDATE(  last-modified-date )
MODTIME(  last-modified-time ) CURSIZE(  current-size )
INITSIZE(  initial-size ) MODRECS(  records-modified ) USER( user-id )
DELETE CREATED4(4-char-year-create-date)
MODDATE4(4-char-year-last-modified-date)
SCLM(OFF)
SCLM(ON)
SCLM(ASIS)
NOLLA
EXT(NO)
EXT(YES) USER8( user-id8 )
ALIAS
NOALIAS
LMMSTATS service
Chapter 3. ISPF service syntax with return codes  93

## Page 120

Call invocation format
CALL ISPLINK ('LMMSTATS' , data-id , member-name , version-number
'␣'
, mod-level
'␣'
, create-date
'␣'
, last-modified-date
'␣'
, last-modified-time
'␣'
, current-size
'␣'
, initial-size
'␣'
, records-modified
'␣'
, user-id
'␣'
, 'DELETE'
'␣'
, 4-char-year-create-date
'␣'
, 4-char-year-last-modified-date
'␣'
,
'OFF␣␣␣␣␣'
'␣'
'ON␣␣␣␣␣␣'
'ASIS␣␣␣␣'
, 'NOLLA␣␣␣'
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, user-id8
'␣'
,
'ALIAS␣'
'␣'
'NOALIAS'
);
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Normal completion.
 4
Either:
• Data set is empty.
• No members matched the pattern.
 8
Member not found.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• Invalid parameter value.
• Data set is not partitioned.
• Data ID represents a concatenation of data sets.
• Data set is opened for output.
LMMSTATS service
94  z/OS: z/OS ISPF Reference Summary

## Page 121

• Data set name is an alias. And the NOALIAS parameter was specified.
20
Severe error; unable to continue.
LMOPEN—open a data set
Command invocation format
ISPEXEC LMOPEN DATAID( data-id )
OPTION(INPUT)
OPTION(OUTPUT) LRECL( lrecl-var )
RECFM( recfm-var ) ORG( org-var )
Call invocation format
CALL ISPLINK ('LMOPEN␣␣' , data-id ,
'INPUT␣'
'␣'
'OUTPUT'
, lrecl-var
'␣'
, recfm-var
'␣'
, org-var
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion.
 8
Data set could not be opened.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
One of these:
• The parameter value is invalid.
• Data set is already open.
• Cannot open concatenated data sets for output.
• Cannot open a data set allocated SHR for output.
16
Truncation or translation error in accessing dialog variables.
20
Severe error; unable to continue.
LMOPEN service
Chapter 3. ISPF service syntax with return codes  95

## Page 122

LMPRINT—print a partitioned or sequential data set
Command invocation format
ISPEXEC LMPRINT DATAID( data-id )
MEMBER(  member-name ) INDEX
FORMAT(YES)
FORMAT(NO) NOLLA
Call invocation format
CALL ISPLINK ('LMPRINT␣' , data-id , member-name
'␣'
, 'INDEX'
'␣'
,
'YES'
'␣'
'NO␣'
, 'NOLLA␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer );
Return codes
 0
Normal completion.
 4
Either:
• Data set is empty or contains an empty member.
• No members matched the pattern.
 8
Member not found.
10
No data set associated with given data ID.
12
Either:
• Invalid data set organization; must be partitioned or sequential.
• Invalid parameter.
20
Severe error; unable to continue.
LMPRINT service
96  z/OS: z/OS ISPF Reference Summary

## Page 123

LMPUT—write a logical record to a data set
Command invocation format
ISPEXEC LMPUT DATAID( data-id ) MODE( INVAR
MOVE
MULTX
)
DATALOC(  dataloc-var ) DATALEN(  data-length )
NOBSCAN
Call invocation format
CALL ISPLINK ('LMPUT␣␣␣',  data-id , 'INVAR␣␣␣'
'MOVE␣␣␣␣'
'MULTX␣␣␣'
, dataloc-var
, data-length ,'␣' , 'NOBSCAN'
'␣'
);
CALL ISPEXEC ( buf-len , buffer);
OR
Return codes
 0
Normal completion.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
12
Either:
• The data set is not open or is not open for output.
• The parameter value is invalid.
16
Truncation or translation error in accessing dialog variables.
20
Severe error; unable to continue.
LMPUT service
Chapter 3. ISPF service syntax with return codes  97

## Page 124

LMQUERY—give a dialog information about a data set
Command invocation format
ISPEXEC LMQUERY DATAID( data-id )
PROJECT(  proj-var )
GROUP1(  group1-var ) GROUP2(  group2-var )
GROUP3(  group3-var ) GROUP4(  group4-var ) TYPE( type-var )
DATASET(  dsn-var ) DDNAME(  ddn-var ) VOLUME(  serial-var )
ENQ( enq-var ) OPEN( open-var ) LRECL( lrecl-var )
RECFM( recfm-var ) DSORG( dsorg-var ) ALIAS(  alias-var )
PASSWORD(  password-var ) OVOLUME(  ovolume-var )
Call invocation format
CALL ISPLINK ('LMQUERY␣' , data-id , proj-var
'␣'
, group1-var
'␣'
, group2-var
'␣'
, group3-var
'␣'
, group4-var
'␣'
, type-var
'␣'
, dsn-var
'␣'
, ddn-var
'␣'
, serial-var
'␣'
, enq-var
'␣'
, open-var
'␣'
, lrecl-var
'␣'
, recfm-var
'␣'
, dsorg-var
'␣'
, alias-var
'␣'
, password-var
'␣'
, ovolume-var
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
LMQUERY service
98  z/OS: z/OS ISPF Reference Summary

## Page 125

Return codes
 0
Normal completion.
 4
No applicable information available for a specified keyword; blanks are returned.
10
No data set is associated with the given data ID; that is, LMINIT has not been completed.
16
Truncation or translation error in accessing dialog variables.
20
Severe error; unable to continue.
LMRENAME—rename an ISPF library
Command invocation format
ISPEXEC LMRENAME PROJECT(  project ) GROUP(  group) TYPE( type)
NEWPROJ(  new-project ) NEWGROUP(  new-group )
NEWTYPE(  new-type )
Call invocation format
CALL ISPLINK('LMRENAME' , project , group , type , new-project
'␣'
, new-group
'␣'
, new-type
'␣'
);
CALL ISPEXEC ( buf-len , buffer);
OR
Return codes
 0
Normal completion.
 4
New name already exists.
 8
One of these:
• Specified data set does not exist.
• Rename or catalog failed.
• Data set name is an alias.
12
The parameter value is invalid.
LMRENAME service
Chapter 3. ISPF service syntax with return codes  99

## Page 126

20
Severe error; unable to continue.
LOG—write a message to the log data set
Command invocation format
ISPEXEC LOG MSG( message-id )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('LOG␣␣␣␣␣', message-id );
Return codes
 0
Normal completion.
12
The message-id contains invalid syntax or was not found.
20
Severe error.
MEMLIST—member list dialog
Command invocation format
ISPEXEC MEMLIST DATAID( data-id )
MEMBER(  pattern )
CONFIRM(YES)
CONFIRM(NO)
PANEL( panel-name )
FIELD(9)
FIELD(1)
DEFAULT(S)
DEFAULT(  action)
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
LOG service
100  z/OS: z/OS ISPF Reference Summary

## Page 127

CALL ISPLINK ('MEMLIST␣' , data-id, , pattern
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, panel-name
'␣'
,
9
'␣'
1
,
'S␣␣␣␣␣␣␣'
'␣'
action
);
Return codes
 0
Normal completion.
8
The requested data set was empty or no members matched the specified pattern.
10
No data set is associated with the given data ID. LMINIT has not been completed.
12
Indicates one of these:
• Data set not partitioned.
• Parameter value not valid.
• Invocation syntax not valid.
16
A truncation or translation error occurred in accessing dialog variables.
20
Severe error.
PQUERY—obtain panel information
Command invocation format
ISPEXEC PQUERY PANEL( panel-name ) AREANAME(  area-name )
AREATYPE(  area-type-name ) WIDTH(  area-width-name )
DEPTH( area-depth-name ) ROW( row-number-name )
COLUMN(  column-number-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
PQUERY service
Chapter 3. ISPF service syntax with return codes  101

## Page 128

CALL ISPLINK ('PQUERY␣␣' , panel-name , area-name
, area-type-name
'␣'
, area-width-name
'␣'
, area-depth-name
'␣'
, row-number-name
'␣'
, column-number-name
'␣'
);
Return codes
 0
Normal completion
 8
The panel does not contain the specified area.
12
The specified panel cannot be found.
16
Not all are values returned because insufficient space was provided.
20
Severe error.
QBASELIB—query base library information
Command invocation format
ISPEXEC QBASELIB dd-name
ID( id-var )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('QBASELIB' , dd-name , id-var
'␣'
);
Return codes
 0
A DDNAME for the specified ddname exists and the requested information has been successfully
returned.
 4
The specified dd-name is not defined.
16
A dialog variable translation or truncation error has occurred.
QBASELIB service
102  z/OS: z/OS ISPF Reference Summary

## Page 129

20
A severe error has occurred.
QLIBDEF—query LIBDEF definition information
Command invocation format
ISPEXEC QLIBDEF lib-type
TYPE( type-var ) ID( id-var )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('QLIBDEF␣' , lib-type , type-var
'␣'
, id-var
'␣'
);
Return codes
 0
A LIBDEF definition for the specified lib-type exists and the requested information, if any, has been
successfully returned.
 4
The specified lib-type does not have an active LIBDEF definition.
12
An invalid lib-type value of ISPPROF has been specified.
16
A dialog variable translation or truncation error has occurred.
20
A severe error has occurred.
QTABOPEN—query open ISPF tables
Command invocation format
ISPEXEC QTABOPEN LIST(list-var)
Call invocation format
CALL ISPLINK ('QTABOPEN' , list-var );
Return codes
These return codes are possible:
 0
Normal completion.
 4
List incomplete. There was insufficient space to construct a valid variable name.
QLIBDEF service
Chapter 3. ISPF service syntax with return codes  103

## Page 130

12
Prefix too long. List-var must be 7 characters or less.
20
Severe error.
QUERYENQ—query system ENQ data
Command invocation format
ISPEXEC QUERYENQ TABLE(table-name) QNAME(qname) RNAME(rname)
REQ(pattern)
WAIT
LIMIT(limit) SAVE(list-id)
XSYS
Call invocation format
CALL ISPLINK ('QUERYENQ' , table-name , qname , rname , pattern
, 'WAIT␣␣␣␣'
'␣'
, limit , list-id , 'XSYS␣␣␣␣'
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Variables returned in each row of the table
Table 5. Variables Returned in Each Row of the Table
Name Size Description
ZENJOB 8 Job or address space name holding or requesting the ENQ
ZENQNAME 8 Qname portion of the ENQ
ZENRNAME 255 Rname portion of the ENQ
ZENDISP 5 SHARE or EXCLU
ZENHOLD 4 OWN or WAIT
ZENSCOPE 7 SYSTEM or SYSTEMS
ZENSTEP 7 STEP or blank
ZENGLOBL 6 GLOBAL or blank
ZENSYST 8 System name
ZENRESV 7 RESERVE or blank
Return codes
 0
Table returned or data set written, but XSYS parameter was not specified and the system is running in
STAR mode. The data returned may not reflect all ENQs on all systems.
 2
Table returned or data set written.
QUERYENQ service
104  z/OS: z/OS ISPF Reference Summary

## Page 131

4
Table returned but truncated due to limit.
 8
No ENQs satisfy the request.
10
No ENQs satisfy the request, but XSYS parameter was not specified and the system is running in STAR
mode. The data returned may not reflect all ENQs on all systems.
12
Table creation error, parameter or other termination error. See messages for more detail. This includes
services not available due to configuration table restrictions.
14
The SAVE data set is in use by another user.
20
Severe error, including TBADD error or data set creation errors.
REMPOP—remove a pop-up window
Command invocation format
ISPEXEC REMPOP
ALL
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('REMPOP␣␣' , 'ALL␣␣␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
16
A pop-up window does not exist at this select level.
20
Severe error.
REMPOP service
Chapter 3. ISPF service syntax with return codes  105

## Page 132

SELECT—select a panel or function
Command invocation format
ISPEXEC SELECT
PANEL( panel-name)
ADDPOP OPT( option)
CMD( command)
LANG( APL
CREX
) MODE( LINE
FSCR
) BARRIER NEST
PGM( program-name)
PARM( parameters) MODE( LINE
FSCR
)
NEWAPPL
( application-id) PASSLIB
NEWPOOL SUSPEND EXCLPROF
SCRNAME( screen_name)
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('SELECT␣␣' , length,
PANEL( panel-name)
ADDPOP OPT( option)
CMD( command)
LANG( APL
CREX
) MODE( LINE
FSCR
) BARRIER NEST
PGM( program-name)
PARM( parameters) MODE( LINE
FSCR
)
NEWAPPL
( application-id) PASSLIB
NEWPOOL SUSPEND EXCLPROF
SCRNAME( screen_name)
Return codes
These return codes are possible if a panel is specified:
 0
Normal completion. The END command was entered from the selected menu.
 4
Normal completion. The RETURN command was entered or the EXIT option was specified from the
selected menu or from some lower-level menu.
SELECT service
106  z/OS: z/OS ISPF Reference Summary

## Page 133

12
The specified panel could not be found.
16
Truncation error in storing the ZCMD or ZSEL variable.
20
Severe error.
Note:
1. A return code of 0 is returned when the SELECT service has been coded with no other parameters.
2. If a command or program is invoked by using SELECT, the return code from the command or program
is passed to the function that invoked SELECT.
SETMSG—set next message
Command invocation format
ISPEXEC SETMSG MSG( message-id )
COND
MSGLOC(  message-field-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('SETMSG␣␣' , message-id , 'COND␣␣␣␣'
'␣'
, message-field-name
'␣'
);
Return codes
 0
Normal completion.
 4
SETMSG with COND parameter issued and a SETMSG request was pending.
12
The specified message field name or message not be found.
20
Severe error.
SETMSG service
Chapter 3. ISPF service syntax with return codes  107

## Page 134

TBADD—add a row to a table
Command invocation format
ISPEXEC TBADD table-name
SAVE( name-list ) ORDER
MULT( number-of-rows )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBADD␣␣␣' , table-name , name-list
'␣'
, 'ORDER␣␣␣'
'␣'
, number-of-rows
'␣'
);
Return codes
 0
Normal completion.
 4
The number-of-rows parameter was specified but storage was obtained for only a single row.
 8
A row with the same key already exists; CRP set to TOP (zero). Returned only for tables with keys.
12
Table is not open.
16
Numeric convert error; see numeric restrictions for TBSORT. Returned only for sorted tables.
20
Severe error.
TBBOTTOM—set the row pointer to bottom
Command invocation format
ISPEXEC TBBOTTOM table-name
SAVENAME(  var-name )
ROWID( rowid-name ) NOREAD POSITION(  crp-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
TBADD service
108  z/OS: z/OS ISPF Reference Summary

## Page 135

OR
CALL ISPLINK ('TBBOTTOM' , table-name , var-name
'␣'
, rowid-name
'␣'
, 'NOREAD␣␣'
'␣'
, crp-name
'␣'
);
Return codes
 0
Normal completion.
 8
Table is empty; CRP set to TOP (zero).
12
Table is not open.
16
Variable value has been truncated or insufficient space provided to return all extension variable
names.
20
Severe error.
TBCLOSE—close and save a table
Command invocation format
ISPEXEC TBCLOSE table-name
REPLCOPY
NEWCOPY NAME( alt-name )
PAD( percentage) LIBRARY(  library )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBCLOSE␣' , table-name ,
'REPLCOPY'
'␣'
'NEWCOPY␣'
, alt-name
'␣'
, percentage
'␣'
, library
'␣'
);
Return codes
 0
Normal completion.
TBCLOSE service
Chapter 3. ISPF service syntax with return codes  109

## Page 136

12
Table is not open.
16
Alternate table output library was not allocated.
20
Severe error.
TBCREATE—create a new table
Command invocation format
ISPEXEC TBCREATE table-name
KEYS( key-name-list )
NAMES( name-list )
WRITE
NOWRITE REPLACE
LIBRARY(  library ) SHARE
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBCREATE' , table-name , key-name-list
'␣'
, name-list
'␣'
,
'WRITE␣␣␣'
'␣'
'NOWRITE␣'
, 'REPLACE␣'
'␣'
, library
'␣'
, 'SHARE␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 4
Normal completion—a duplicate table exists but REPLACE was specified.
 8
Either the table already exists and REPLACE was not specified, or REPLACE was specified and the
table is in SHARE mode.
12
Table in use; ENQ failed.
16
WRITE mode specified and alternate table input library not allocated. TBCREATE checks the input
library to determine if a duplicate table exists. See return code 8.
TBCREATE service
110  z/OS: z/OS ISPF Reference Summary

## Page 137

20
Severe error.
TBDELETE—delete a row from a table
Command invocation format
ISPEXEC TBDELETE table-name
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBDELETE' , table-name );
Return codes
 0
Normal completion.
 8
Keyed tables: The row specified by the value in key variables does not exist; CRP set to TOP (zero).
Non-keyed tables: CRP was at TOP (zero) and remains at TOP.
12
Table is not open.
20
Severe error.
TBDISPL—display table information
Command invocation format
ISPEXEC TBDISPL table-name
PANEL( panel-name ) MSG( message-id )
CURSOR(  field-name ) CSRROW(  table-row-number )
CSRPOS(  cursor-position )
AUTOSEL(YES)
AUTOSEL(NO) POSITION(  crp-name )
ROWID( rowid-name ) MSGLOC(  message-field-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
TBDELETE service
Chapter 3. ISPF service syntax with return codes  111

## Page 138

CALL ISPLINK ('TBDISPL␣' , table-name , panel-name
'␣'
, message-id
'␣'
, field-name
'␣'
, table-row-number
'␣'
, cursor-position
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, crp-name
'␣'
, rowid-name
'␣'
, message-field-name
'␣'
);
Return codes
 0
If the panel definition contains neither a )REINIT nor a )PROC section, the Enter key was pressed, or a
scroll command was entered. Any of these occurred:
• One row was selected in the scrollable part of the display. The CRP is set to point to that table row
and the row is retrieved. The input fields from the selected model set on the display are then stored
in the function pool.
• The user entered information into the fixed portion of the display.
• All of these:
– A scroll return to function has been specified (ZTDRET defined to UP, DOWN, or VERTICAL).
– More rows are needed to fill a scroll request.
– No selected rows remain to be processed.
If the panel definition contains a )REINIT or )PROC section, there is the additional possibility that the
user entered no information and just pressed the Enter key.
 4
The Enter key was pressed or a scroll command was entered. The first or both of these occurred:
• Two or more rows in the scrollable part of the display were selected. The CRP is set to the first
selected row and the row is retrieved. The input fields from the selected model set on the display
are then stored in the function pool.
• The user entered information into the fixed portion of the display.
• If scroll return to function has been specified, and two or more rows are selected for processing,
TBDISPL returns a return code 4 until all selected rows are processed. You process the request for
more rows to be added to the table only after all selected rows have been processed; that is, only
when ZTDSELS has a value of 0.
For subsequent TBDISPL requests with no panel name and no message-id, return code 4 is issued for
each request until one selected row remains to be accessed. For this last row, a return code of zero is
issued by TBDISPL, still specified with no panel name and no message-id. The variable ZTDSELS will
have a value of one.
 8
The END or RETURN command was entered. For panels created by the conversion utility, CANCEL and
EXIT commands also give return code 8. If CANCEL and EXIT is requested from a panel displayed
using TBDISPL service calls and the panel was defined with Dialog Tag Language (DTL), the dialog
manager returns the command in ZVERB and sets a return code of 8 from the display screen. The CRP
TBDISPL service
112  z/OS: z/OS ISPF Reference Summary

## Page 139

is set to the first of any selected rows in the scrollable part of the display. The input fields from the
selected model set on the display are then stored in the function pool.
If no rows were selected, the CRP is at the top (zero).
To process all selected rows when END or RETURN was entered, continue to issue TBDISPL requests
with no panel name or message-id specified until ZTDSELS is one.
If you enter the END command on a table display panel, a subsequent redisplay will result in a return
code of 8.
The user might have entered information into the fixed portion of the display.
12
The specified panel, message, cursor field, or message location field could not be found.
16
Truncation or translation error in storing defined variables.
20
Severe error.
TBEND—close a table without saving
Command invocation format
ISPEXEC TBEND table-name
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBEND␣␣␣' , table-name );
Return codes
 0
Normal completion.
12
Table is not open.
20
Severe error.
TBERASE—erase a table
Command invocation format
ISPEXEC TBERASE table-name
LIBRARY(  library )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
TBEND service
Chapter 3. ISPF service syntax with return codes  113

## Page 140

CALL ISPLINK ('TBERASE␣' , table-name , library
'␣'
);
Return codes
 0
Normal completion.
 8
Table does not exist in the output library.
12
Table in use; ENQ failed.
16
Table output library not allocated.
20
Severe error.
TBEXIST—determine whether a row exists in a table
Command invocation format
ISPEXEC TBEXIST table-name
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBEXIST␣' , table-name );
Return codes
 0
Normal completion; the CRP is positioned to the specified row.
 8
Keyed tables: the specified row does not exist; the CRP is set to TOP (zero).
Non-keyed tables: service not possible; the CRP is set to TOP.
12
Table is not open.
20
Severe error.
TBEXIST service
114  z/OS: z/OS ISPF Reference Summary

## Page 141

TBGET—retrieve a row from a table
Command invocation format
ISPEXEC TBGET table-name
SAVENAME(  var-name )
ROWID( rowid-name ) NOREAD POSITION(  crp-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBGET␣␣␣' , table-name , var-name
'␣'
, rowid-name
'␣'
, 'NOREAD␣␣'
'␣'
, crp-name
'␣'
);
Return codes
 0
Normal completion.
 8
Keyed tables: The row specified by the value in the key variables does not exist in any row after the
current row pointer, the CRP is set to TOP (ZERO).
Non-keyed tables: the CRP was at TOP and remains at TOP.
12
Table is not open.
16
Variable value has been truncated, or insufficient space was provided to return all extension variable
names.
20
Severe error.
TBMOD—modify a row in a table
Command invocation format
ISPEXEC TBMOD table-name
SAVE( name-list ) ORDER
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
TBGET service
Chapter 3. ISPF service syntax with return codes  115

## Page 142

CALL ISPLINK ('TBMOD␣␣␣' , table-name , name-list
'␣'
'ORDER␣␣␣'
'␣'
);
Return codes
 0
Normal completion. Keyed tables: Existing row updated. Non-keyed tables: New row added to table.
 8
Keys did not match; new row added to the table. Returned only for tables with keys.
12
Table is not open.
16
Numeric conversion error; see numeric restrictions for TBSORT. Returned only for sorted tables.
20
Severe error.
TBOPEN—open a table
Command invocation format
ISPEXEC TBOPEN table-name
WRITE
NOWRITE LIBRARY(  library )
SHARE
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBOPEN␣␣' , table-name ,
'WRITE␣␣␣'
'␣'
'NOWRITE␣'
, library
'␣'
, 'SHARE␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 8
Table does not exist.
TBOPEN service
116  z/OS: z/OS ISPF Reference Summary

## Page 143

12
ENQ failed; table was in use by another user or the current user.
16
Table input library was not allocated.
20
Severe error.
TBPUT—update a row in a table
Command invocation format
ISPEXEC TBPUT table-name
SAVE( name-list ) ORDER
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBPUT␣␣␣' , table-name , name-list
'␣'
, 'ORDER␣␣␣'
'␣'
);
Return codes
 0
Normal completion.
 8
Keyed tables: The key does not match that of the current row; CRP set to TOP (zero).
Non-keyed tables: CRP was at TOP and remains at TOP.
12
Table is not open.
16
For sorted tables: numeric conversion error; see numeric restrictions for TBSORT.
20
Severe error.
TBPUT service
Chapter 3. ISPF service syntax with return codes  117

## Page 144

TBQUERY—obtain table information
Command invocation format
ISPEXEC TBQUERY table-name
KEYS( key-name ) NAMES( var-name )
ROWNUM(  rownum-name ) KEYNUM(  keynum-name )
NAMENUM(  namenum-name ) POSITION(  crp-name )
SORTFLDS(  srt-name ) SARGLIST(  lst-name )
SARGCOND(  cond-name ) SARGDIR(  dir-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBQUERY␣' , table-name , key-name
'␣'
, var-name
'␣'
, rownum-name
'␣'
, keynum-name
'␣'
, namenum-name
'␣'
, crp-name
'␣'
, srt-name
'␣'
, lst-name
'␣'
, cond-name
'␣'
, dir-name
'␣'
);
Return codes
 0
Normal completion.
12
Table is not open.
16
Not all keys or names were returned because insufficient space was provided.
20
Severe error.
TBQUERY service
118  z/OS: z/OS ISPF Reference Summary

## Page 145

TBSARG—define a search argument
Command invocation format
ISPEXEC TBSARG table-name
ARGLIST(  name-list )
NEXT
PREVIOUS
NAMECOND(  name-cond-pairs )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBSARG␣␣' , table-name , name-list
'␣'
,
'NEXT␣␣␣␣'
'␣'
'PREVIOUS'
, name-cond-pairs
'␣'
);
Return codes
 0
Normal completion.
 8
All column variables are null, and the name-list parameter was not specified; no argument is
established.
12
Table is not open.
20
Severe error.
TBSAVE—save a table
Command invocation format
ISPEXEC TBSAVE table-name
REPLCOPY
NEWCOPY NAME( alt-name )
PAD( percentage) LIBRARY(  library )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
TBSARG service
Chapter 3. ISPF service syntax with return codes  119

## Page 146

OR
CALL ISPLINK ('TBSAVE␣␣' , table-name ,
'REPLCOPY'
'␣'
'NEWCOPY␣'
, alt-name
'␣'
, percentage
'␣'
, library
'␣'
);
Return codes
 0
Normal completion.
12
Table is not open.
16
Alternate table output library was not allocated.
20
Severe error.
TBSCAN—search a table
Command invocation format
ISPEXEC TBSCAN table-name
ARGLIST(  name-list )
SAVENAME(  var-name ) ROWID( rowid-name )
NEXT
PREVIOUS
NOREAD POSITION(  crp-name ) CONDLIST(  condition-value-list )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBSCAN␣␣' , table-name , name-list
'␣'
, var-name
'␣'
, rowid-name
'␣'
,
'NEXT␣␣␣␣'
'␣'
'PREVIOUS'
, 'NOREAD␣␣'
'␣'
, crp-name
'␣'
, condition-value-list
'␣'
);
TBSCAN service
120  z/OS: z/OS ISPF Reference Summary

## Page 147

Return codes
 0
Normal completion.
 8
Row does not exist, no match was found; CRP is set to TOP (zero). The rowid remains unchanged.
12
Table is not open.
16
Variable value has been truncated, or insufficient space is provided to return all extension variable
names.
20
Severe error.
TBSKIP—move the row pointer
Command invocation format
ISPEXEC TBSKIP table-name
NUMBER(  number ) SAVENAME(  var-name )
ROWID( rowid-name ) ROW( rowid ) NOREAD
POSITION(  crp-name )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBSKIP␣␣' , table-name , number
'␣'
, var-name
'␣'
, rowid-name
'␣'
, rowid
'␣'
, 'NOREAD␣␣'
'␣'
, crp-name
'␣'
);
 0
Normal completion.
 8
CRP would have gone beyond the number of rows in the table. This includes a table empty condition,
with CRP set to TOP (zero). The rowid remains unchanged.
12
Table is not open.
16
Variable value has been truncated, or insufficient space is provided to return all extension variable
names.
TBSKIP service
Chapter 3. ISPF service syntax with return codes  121

## Page 148

20
Severe error.
TBSORT—sort a table
Command invocation format
ISPEXEC TBSORT table-name FIELDS( sort-list )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBSORT␣␣' , table-name , sort-list );
Return codes
 0
Normal completion.
12
Table is not open.
16
Numeric convert error.
20
Severe error.
TBSORT service
122  z/OS: z/OS ISPF Reference Summary

## Page 149

TBSTATS—retrieve table statistics
Command invocation format
ISPEXEC TBSTATS table-name
CDATE( date-created-name )
CTIME( time-created-name ) UDATE( date-updated-name )
UTIME( time-updated-name ) USER( user-name )
ROWCREAT(  row-created-name ) ROWCURR(  rownum-name )
ROWUPD(  row-updated-name ) TABLEUPD(  table-updated-name )
SERVICE(  service-name ) RETCODE(  return-code-name )
STATUS1(  status1-name ) STATUS2(  status2-name )
STATUS3(  status3-name ) LIBRARY(  library )
VIRTSIZE(  virtual-storage-size-name ) CDATE4D(  date-created-name-4-digit )
UDATE4D(  date-updated-name-4-digit )
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
TBSTATS service
Chapter 3. ISPF service syntax with return codes  123

## Page 150

CALL ISPLINK ('TBSTATS␣' , table-name , date-created-name
'␣'
, time-created-name
'␣'
, date-updated-name
'␣'
, time-updated-name
'␣'
, user-name
'␣'
, row-created-name
'␣'
, rownum-name
'␣'
, row-updated-name
'␣'
, table-updated-name
'␣'
, service-name
'␣'
, return-code-name
'␣'
, status1-name
'␣'
, status2-name
'␣'
, status3-name
'␣'
, library
'␣'
, virtual-storage-size-name
'␣'
, date-created-name-4-digit
'␣'
, date-updated-name-4-digit
'␣'
);
Return codes
 0
Normal completion (returned even if the table does not exist).
16
Variable value has been truncated.
20
Severe error.
TBTOP—set the row pointer to the top
Command invocation format
ISPEXEC TBTOP table-name
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBTOP␣␣␣' , table-name );
TBTOP service
124  z/OS: z/OS ISPF Reference Summary

## Page 151

Return codes
 0
Normal completion.
12
Table is not open.
20
Severe error.
TBVCLEAR—clear table variables
Command invocation format
ISPEXEC TBVCLEAR table-name
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('TBVCLEAR' , table-name );
Return codes
 0
Normal completion.
12
Table is not open.
20
Severe error.
TRANS—translate data from one Coded Character Set Identifier
(CCSID) to another
Command invocation format
ISPEXEC TRANS FRMCCSID(  from-ccsid-number ) TOCCSID(  to-ccsid-number )
FROMVAR(  from-variable-name )
TOVAR(  to-variable-name )
LENGTH( data-length )
Call invocation format
CALL ISPEXEC ( buflen, buffer);
OR
TBVCLEAR service
Chapter 3. ISPF service syntax with return codes  125

## Page 152

CALL ISPLINK ('TRANS␣␣␣' , from-ccsid-number , to-ccsid-number , from-variable-name
, to-variable-name
'␣'
, data-length
'␣'
);
Return codes
 0
Service completed successfully.
 4
Translate tables do not support the requested "to ... from" combination. For a list of extended code
page translate tables provided by ISPF, see the z/OS ISPF Dialog Developer's Guide and Reference.
 8
From variable not found.
16
Variable services indicated a translation error or truncation occurred storing the translated data.
20
Severe error.
VCOPY—create a copy of a variable
Command invocation format
ISPEXEC  *This service does not apply to APL2 or command
          procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used with this interface*
OR
CALL ISPLINK ('VCOPY␣␣␣' , name-list , length-array , value-array
,
'LOCATE␣␣'
'␣'
'MOVE␣␣␣␣'
);
Return codes
 0
Normal completion.
 8
One or more variables do not exist.
12
Validation failed.
16
Truncation has occurred during data movement (move mode only).
20
Severe error.
VCOPY service
126  z/OS: z/OS ISPF Reference Summary

## Page 153

VDEFINE—define function variables
Command invocation format
ISPEXEC  *This service does not apply to APL2 or command
          procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used with this interface*
OR
CALL ISPLINK ('VDEFINE␣' , name-list , variable, format , length
, options-list
'␣'
, user-data
'␣'
, 'LFORMAT␣'
'␣'
);
Return codes
 0
Normal completion.
 8
Variable not found.
16
Data truncation occurred.
20
Severe error.
VDELETE—remove a definition of function variables
Command invocation format
ISPEXEC  *This service does not apply to APL2 or
          command procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used with this interface*
OR
CALL ISPLINK ('VDELETE␣', name-list
'*␣␣␣␣␣␣␣'
);
Return codes
 0
Normal completion.
 8
At least one variable not found.
VDEFINE service
Chapter 3. ISPF service syntax with return codes  127

## Page 154

20
Severe error.
VERASE—remove variables from shared and/or profile pool
Command invocation format
ISPEXEC VERASE name-list
ASIS
SHARED
PROFILE
BOTH
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('VERASE␣␣' , name-list ,
'ASIS␣␣␣␣'
'␣'
'SHARED␣␣'
'PROFILE␣'
'BOTH␣␣␣␣'
);
Return codes
 0
Normal completion.
 8
At least one variable not found.
20
Severe error.
VGET—retrieve variables from a pool or profile or system symbol
Command invocation format
ISPEXEC VGET name-list
ASIS
SHARED
PROFILE
SYMDEF
SYMNAMES(symname-list)
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
VERASE service
128  z/OS: z/OS ISPF Reference Summary

## Page 155

CALL ISPLINK ('VGET␣␣␣␣' , name-list ,
'ASIS␣␣␣␣'
'␣'
'SHARED␣␣'
'PROFILE␣'
'SYMDEF␣␣'
, symname-list
'␣'
);
Return codes
 0
Normal completion.
 8
Variable not found. If the SYMDEF parameter was specified: system symbol not found.
12
Validation failed.
16
Translation error or truncation occurred during data movement.
20
Severe error. If the SYMDEF parameter was specified: the number of symbol names in symname-list
exceeds the number of names in name-list.
VIEW—view a data set
Command invocation format
ISPEXEC VIEW DATASET(  dsname)
VOLUME(  serial )
PASSWORD(  pswd-value ) PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES)
CHGWARN(YES)
CHGWARN(NO) PARM( parm-var ) ASCII
UTF8
LINECMDS(  tabname ) GEN( generation )
OR
VIEW service
Chapter 3. ISPF service syntax with return codes  129

## Page 156

ISPEXEC VIEW DATAID( data-id )
MEMBER(  member-name )
GEN( generation )
PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES)
CHGWARN(YES)
CHGWARN(NO) PARM( parm-var ) ASCII
UTF8
LINECMDS(  tabname )
OR
ISPEXEC VIEW FILE( file-var )
PANEL( panel-name )
MACRO( macro-name ) PROFILE(  profile-name )
FORMAT(  format-name )
MIXED(NO)
MIXED(YES)
CONFIRM(YES)
CONFIRM(NO)
WS(NO)
WS(YES)
CHGWARN(YES)
CHGWARN(NO) PARM( parm-var ) ASCII
UTF8
LINECMDS(  tabname )
VIEW service
130  z/OS: z/OS ISPF Reference Summary

## Page 157

Call invocation format
CALL ISPLINK ('VIEW␣␣␣␣' , dsname
'␣'
, serial
'␣'
, pswd-value
'␣'
, panel-name
'␣'
, macro-name
'␣'
, profile-name
'␣'
, data-id
'␣'
, member-name
'␣'
, format-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
,
ws-filename-buffer-name
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, '␣'
'YES␣␣␣␣␣'
'NO␣␣␣␣␣␣'
, parm-var
'␣'
, file-var
'␣'
, 'ASCII␣␣␣'
'UTF8␣␣␣␣'
'␣'
, tabname
'␣'
, generation
'␣'
);
OR
CALL ISPEXEC ( buf-len , buffer);
Return codes
 0
Normal completion. Browse was substituted for VIEW if insufficient storage was available to read in
the requested data.
Note: Data can only be saved through the CREATE or REPLACE primary commands.
9
The specified generation of the member was not found in the specified data sets.
10
Member not found.
11
A non-current generation was specified. None of the specified data sets are PDSE version 2 data sets
that are configured for member generations.
12
VIEW has been disabled through the ISPF configuration table or the ws-filename-buffer-name
parameter was specified.
14
Member, sequential data set, or z/OS UNIX file in use.
VIEW service
Chapter 3. ISPF service syntax with return codes  131

## Page 158

16
Either:
• No members matched the specified pattern
• No members in the partitioned data set.
18
A VSAM data set was specified but the ISPF Configuration Table does not allow VSAM processing.
20
Severe error; unable to continue.
VIIF—view interface
Command invocation format
You cannot use command procedures to invoke this service.
Call invocation format
CALL ISPLINK ('VIIF␣␣␣␣' , data-name
'␣'
, profile-name , rec-format
, rec-len , read-routine , cmd-routine
'␣'
, dialog-data
'␣'
, edit-len
'␣'
, panel-name
'␣'
, macro-name
'␣'
, format-name
'␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
,
'NO␣␣␣␣␣␣'
'␣'
'YES␣␣␣␣␣'
, parm-var
'␣'
, write-routine
'␣'
,
'YES␣␣␣␣␣'
'␣'
'NO␣␣␣␣␣␣'
, tabname
'␣'
);
OR
VIIF service
132  z/OS: z/OS ISPF Reference Summary

## Page 159

CALL ISPLINK ('VIIF␣␣␣␣' , data-name
'␣'
,'␣' , rec-format
'␣'
,
rec-len
'␣'
, read-routine , cmd-routine
'␣'
, dialog-data
'␣'
,'␣' ,'␣' ,'␣' ,'␣' ,'␣' ,'YES␣␣␣␣␣' ,'␣' , write-routine
'␣'
,'␣'
, tabname
'␣'
);
Read routine return codes
 0
Normal completion.
 8
End of data records (no data record returned).
16
Read error. If a read error is encountered when the system builds the initial view display, the VIIF
service terminates with a return code of 20. Otherwise, the view data is redisplayed.
20
Severe error. (The VIIF service terminates immediately with a return code of 20.)
Command routine return codes
 0
Normal completion.
 4
ISPF should process the requested function.
12
Command deferred; retain the command on the Command line. View data is redisplayed.
20
Severe error. (The VIIF service terminates immediately with a return code of 20.)
VIIF return codes
 0
Normal completion, data not saved.
12
View has been disabled through the configuration table.
16
Unexpected return code received from a dialog-supplied routine. When an unexpected return code is
received, the VIIF service terminates immediately with a return code of 16.
20
Severe error; unable to continue.
VIIF service
Chapter 3. ISPF service syntax with return codes  133

## Page 160

VMASK—associate an edit mask with a dialog variable
Command invocation format
ISPEXEC  *This service does not apply to APL2 or command
          procedures*
 
Call invocation format
CALL ISPLINK ('VMASK ' , name-list
, 'FORMAT␣␣ ' , 'IDATE '
, 'STDDATE '
, 'ITIME '
, 'STDTIME '
, 'JDATE '
, 'JSTD '
, 'USER␣␣␣␣ ' , ' mask ', masklen
 '␣ '
)
Return codes
 0
Normal completion
 8
Variable not found
20
Severe error.
VPUT—update variables in the shared or profile pool
Command invocation format
ISPEXEC VPUT name-list
ASIS
SHARED
PROFILE
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
VMASK service
134  z/OS: z/OS ISPF Reference Summary

## Page 161

CALL ISPLINK ('VPUT␣␣␣␣' , name-list ,
'ASIS␣␣␣␣'
'␣'
'SHARED␣␣'
'PROFILE␣'
);
Return codes
 0
Normal completion.
 8
Variable not found.
16
Truncation occurred while copying variables to the application profile pool.
20
Severe error.
VREPLACE—replace a variable
Command invocation format
ISPEXEC  *This service does not apply to
          APL2 or command procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used with this interface*
OR
CALL ISPLINK ('VREPLACE' , name-list , lengths, values);
Return codes
 0
Normal completion.
16
Truncation has occurred during data movement.
20
Severe error.
VRESET—reset function variables
Command invocation format
ISPEXEC  *This service does not apply to
          APL or command procedures*
Call invocation format
CALL  ISPEXEC  *This service cannot be used with this interface*
VREPLACE service
Chapter 3. ISPF service syntax with return codes  135

## Page 162

OR
CALL ISPLINK ('VRESET␣␣');
Return codes
 0
Normal completion.
20
Severe error.
VSYM service—resolve system symbols
Command invocation format
ISPEXEC VSYM name-list
Call invocation format
CALL ISPEXEC ( buf-len , buffer);
OR
CALL ISPLINK ('VSYM␣␣␣␣' , name-list );
Return codes
 0
Normal completion.
 4
One or more symbol names not substituted (no corresponding system symbol was found).
 8
Variable not found in function pool.
12
Validation failed.
16
Truncation occurred resolving system symbols.
20
Severe error.
VSYM
136  z/OS: z/OS ISPF Reference Summary
