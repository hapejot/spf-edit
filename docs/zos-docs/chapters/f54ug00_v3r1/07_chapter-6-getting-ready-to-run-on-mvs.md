# Chapter 6. Getting ready to run on MVS

Source file: f54ug00_v3r1.md
Start page: 167
Page span: 167-172

## Page 167

Chapter 6. Getting ready to run on MVS
This topic helps you prepare to use ISPF data-element libraries. ISPF data elements include such things
as panels and messages. Libraries to be accessed during processing of a dialog must be defined to ISPF.
This topic describes the kinds of data-element libraries required when ISPF is active. It also describes
and provides examples of how to define library data sets to ISPF, both before starting a session, and
dynamically during a session.
Setting up ISPF libraries
To set up libraries for developing and testing dialogs, follow these steps:
1. Set up the panel, message, skeleton, table, and program libraries for the application. Allocate new
partitioned data sets.
2. Create a CLIST or REXX command procedure that contains the necessary ALLOCATE statements to
allocate the libraries. Concatenate the application libraries ahead of the libraries required by ISPF, as
described in “Library concatenation” on page 78.
Note: You can use the LIBDEF service to dynamically allocate libraries instead of allocating them
before invoking ISPF. For more information see the z/OS ISPF Services Guide.
3. Create the panels, messages, and skeletons by editing directly into the application libraries.
4. Create the dialog functions and ensure that the load modules are in libraries accessible to ISPF.
Functions coded as program modules must be link-edited. When a function is link-edited, the ISPLINK
subroutine must be included (explicitly or by automatic call) in the load module. ISPLINK is distributed
in load module format and can be placed in a system library for automatic call during link-edit.
5. Invoke the application. To do this, add an ISPSTART command to the command procedure created in
step 2. The ISPSTART command should start the application using the appropriate PANEL, CMD, or
PGM parameter. Users can start the application by using this command procedure or by selecting the
application from the master menu or another menu.
Allocating required ISPF libraries
The libraries described in Table 19 on page 139 are partitioned data sets required for operation of ISPF in
the MVS/TSO environment:
Table 19. Required Partitioned Data Sets
DDNAME Description RECFM LRECL BLKSIZE
ISPPLIB Panel Library FB 80 3120
ISPMLIB Message Library FB 80 3120
ISPSLIB Skeleton Library FB 80 3120
ISPTLIB Table Input Library FB 80 3120
ISPPROF User Profile Library FB 80 (see note)
SYSPROC Command Procedures Lib FB 80 3120
Note: The block size must be a multiple of 80. You can specify BLKSIZE=0 to use a system determined
block size.
The panel, message, skeleton, and table input libraries are distributed with ISPF. As distributed, the
libraries have the characteristics listed. These libraries can be reblocked by the installation to a larger
block size. In addition, the panel, message, and skeleton libraries can be copied into a variable record
Getting ready to run on MVS
© Copyright IBM Corp. 1980, 2024 139

## Page 168

format. The maximum length records supported are 160 for panels, 80 for messages, and 255 for
skeletons. If data sets having unequal record lengths are to be concatenated, the record format must be
variable. If you have preprocessed any panels in the panel library, they must be reprocessed using the
ISPPREP utility after changing the panel library's record size or record format. Preprocessed panels will
not function correctly if copied directly to a data set with a different record size or format.
Table 20 on page 140 contains the LRECL limits which are enforced during ISPF initialization:
Table 20. LRECL Limits during ISPF Initialization
DDNAME Description RECFM Minimum
LRECL
Maximum
LRECL
ISPPLIB Panel Library FB VB 80 84 160 164
ISPMLIB Message Library FB VB 80 84 80 84
ISPSLIB Skeleton Library FB VB 80 84 255 259
The VB libraries require the LRECL to contain 4 extra bytes for the record descriptor word.
Note: Use of the BUFNO parameter on allocation of ISPF libraries is not supported.
Problems can occur when using file tailoring services together with other services (EDIT, COPY, ...) that
result in modifying the data set members in the ISPSLIB concatenation. ISPSLIB is the input skeleton
library, and it is assumed to be a static library. FTINCL obtains existing DCB/DEB information based on the
last OPEN done against ISPSLIB by ISPF.
It is recommended that applications that use file tailoring and that also modify members of ISPSLIB use
the LIBDEF service for ISPSLIB to point to the application's skeleton library. Additionally, the application
should check for any changes to the data set information (DCB/DEB) before invoking file tailoring services.
If there has been a change, then the application should issue a NULL LIBDEF for ISPSLIB and then
re-issue the original LIBDEF for ISPSLIB. This forces the ISPSLIB library to close and then re-open.
ISPF assumes that ISPSLIB is a static library. When you make allocations, consider limiting the possibility
of extents by allocating the skeleton with the largest optimal block size.
There is a separate profile library for each end user. Its contents are dynamically generated and updated
while ISPF is running. There is also a unique profile library for each national language version.
The recommended data set names for these libraries are shown here. Check with your ISPF system
administrator to determine if these are the actual data set names used at your installation.
DDNAME
DSNAME
ISPPLIB
ISP.SISPPxxx
ISPMLIB
ISP.SISPMxxx
ISPSLIB
ISP.SISPSxxx ISP.SISPSLIB
ISPTLIB
ISP.SISPTxxx
ISPPROF
User-selected. Unique for each national language used.
SYSPROC
ISP.SISPEXEC ISP.SISPCLIB
xxx is a placeholder that represents the specific language you are using:
xxx
Language
Getting ready to run on MVS
140  z/OS: z/OS ISPF User's Guide Vol I

## Page 169

ENU
US English
JPN
Japanese
ENP
Uppercase English.
You should concatenate application libraries for panels, messages, skeletons, and tables ahead of the
corresponding ISPF libraries using the ddnames shown. The application libraries must have the same
data set characteristics as the required libraries, as described. For example, assume that application XYZ
uses these partitioned data sets for panels, messages, skeletons, and tables:
XYZ.PANELS
XYZ.MSGS
XYZ.SKELS
XYZ.TABLES
You would issue these allocations:
//ISPPLIB  DD DSN=XYZ.PANELS,DISP=SHR
//         DD DSN=ISP.SISPPxxx,DISP=SHR
//ISPMLIB  DD DSN=XYZ.MSGS,DISP=SHR
//         DD DSN=ISP.SISPMxxx,DISP=SHR
//ISPSLIB  DD DSN=XYZ.SKELS,DISP=SHR
//         DD DSN=ISP.SISPSxxx,DISP=SHR
//         DD DSN=ISP.SISPSLIB,DISP=SHR
//ISPTLIB  DD DSN=XYZ.TABLES,DISP=SHR
//         DD DSN=ISP.SISPTENU,DISP=SHR
//ISPPROF  DD DSN=USERAA.ISPF.PROFILE,DISP=OLD
//SYSPROC  DD DSN=ISP.SISPEXEC,DISP=SHR
//         DD DSN=ISP.SISPCLIB,DISP=SHR
These allocations must be performed before you start ISPF. They can be done in the user's TSO LOGON
procedure by using DD statements, as shown, or in a CLIST or REXX command procedure by using the
corresponding TSO ALLOCATE commands.
Allocating optional ISPF table libraries
The data set described in Table 21 on page 141 is optional. You must allocate it only if an application uses
table services.
Table 21. Table Data Sets
DDNAME Description RECFM LRECL BLKSIZE
ISPTABL Table Output Library FB 80 (See note)
Note: The block size must be a multiple of 80. You can specify BLKSIZE=0 to use a system determined
block size.
The table output library must be a partitioned data set. The ISPTABL ddname that defines the table output
library can specify the same data set as the table input library, ddname ISPTLIB. The first data set in the
ISPTLIB concatenation should be the same as the data set used for ISPTABL. This ensures predictable
behavior of dialogs that use table services without specifying the LIBRARY keyword. The output and input
data sets must be the same if the updated version of a table is to be reprocessed by the same dialog that
updated it.
You must allocate the table output library to ddname ISPTABL before using table services. ISPF includes
ENQ logic to prevent simultaneous updates. ISPTABL must not specify a concatenated sequence of data
sets. It is possible to have the dialog dynamically allocate ISPTABL and free it upon completion. However,
Getting ready to run on MVS
Chapter 6. Getting ready to run on MVS  141

## Page 170

in an environment in which multiple dialogs can be executing, it is more practical to permanently allocate
ISPTABL. ISPTABL should be allocated with DISP=SHR, even though it specifies an output data set. An
application can use the ISPTABL allocation if it already exists or use a LIBDEF for ISPTABL so that only
this application is affected.
For more information about table locking and allocation, see the "Protecting Table Resources" topic in
z/OS ISPF Dialog Developer's Guide and Reference.
Allocating optional file tailoring ISPF libraries
The data set described in Table 22 on page 142 is optional. You must allocate it only if an application uses
file-tailoring services.
Table 22. File-Tailoring Data Sets
DDNAME Description RECFM LRECL BLKSIZE
ISPFILE File-Tailoring Output FB/VB 255 max.
File-tailoring output can be written to a temporary sequential data set provided by ISPF. The temporary
data set is allocated automatically, so there is no need for the dialog to allocate a data set. The fully
qualified name of the temporary data set is available in system variable ZTEMPF. The ddname is available
in ZTEMPN. This temporary data set always uses 80-character fixed-length records.
If the temporary data set is not used, file-tailoring output can be written to either a partitioned or a
sequential data set. Both fixed-length and variable-length records are permitted. The maximum logical
record length is 255 bytes. A data set must be allocated to ddname ISPFILE before starting file-tailoring
services. The dialog can dynamically allocate the output library, and can free it upon completion.
For a sequential data set, ISPFILE must be allocated with DISP=OLD. For a partitioned data set, ISPFILE
can be allocated with DISP=SHR. ISPFILE must not specify a concatenated sequence of data sets.
Allocating CLIST, REXX, and program libraries
Dialog functions that are coded as CLIST or REXX command procedures can be in a procedure library that
has been allocated to ddname SYSPROC before starting ISPF. A REXX command procedure can also be
allocated to the SYSEXEC ddname. The SYSEXEC ddname is described in z/OS TSO/E REXX User's Guide.
You must link-edit dialog functions that have been coded as programs. The load module can reside in a
step library, a system link library (such as SYS1.LINKLIB), or the link pack area. Alternatively, it can be
in a partitioned data set (RECFM=U) allocated to ddname ISPLLIB(DISP=SHR). This library (the ISPF Link
Library) can be used for testing new dialogs that contain program-coded functions. If used, it must be
allocated prior to starting ISPF. ISPLLIB can specify a concatenated sequence of partitioned data sets.
ISPLLIB is used as a task library when fetching load modules. It is searched before the system link
libraries and the link pack area. If ISPF product modules are kept in a step library and a task library
(ISPLLIB) is used, the data sets containing the ISPF product modules should be included in the ISPLLIB
concatenation sequence as well as the step library concatenation. If a program is to be used in split-
screen mode it should be linked as reentrant or nonreusable.
If you are using the z/OS UNIX Directory List Utility, the Language Environment® run-time library data
sets SCEERUN and SCEERUN2 must be in STEPLIB or LNKLST. The modules in these data sets are not
searched for in ISPLLIB.
Allocating DBCS libraries
DBCS users can use alternate message, panel, and skeleton libraries. To do so, the DBCS versions of the
libraries must have been allocated using these ddnames:
ISPMALT
Alternate message library
Getting ready to run on MVS
142  z/OS: z/OS ISPF User's Guide Vol I

## Page 171

ISPPALT
Alternate panel library
ISPSALT
Alternate skeleton library.
You can allocate these libraries when you allocate the distributed ISPF libraries. If the alternate libraries
are allocated and the terminal has DBCS capability, ISPF uses the alternate libraries. If either of these two
conditions is not satisfied, ISPF uses the distributed libraries.
Selecting the National Language for ISPF sessions
An ISPF session can be run in any installation-supported national language. Before starting ISPF with the
ISPSTART command, a user must perform the necessary allocations. For example, command procedure
ISPFE might be issued for an English session.
The same set of ddnames (ISPPLIB, ISPMLIB, ISPSLIB, ISPTLIB, and ISPPROF) must be allocated
regardless of the command procedure used. At logon time, the necessary allocations for the national
language at an installation can be performed by a CLIST or REXX logon procedure.
The language in which a session runs reflects the value (not always the full language name) in the
read-only system variable ZLANG, which is available to dialogs running under ISPF. The default value
for session languages is specified when ISPF is installed, and is discussed in z/OS ISPF Planning and
Customizing. You can override the default session language with an alternate language keyword on the
ISPSTART command. See the z/OS ISPF Dialog Developer's Guide and Reference for the exact syntax to
use.
By specifying a default session language, the installation can ensure that both ISPF initialization
messages and the normal session messages are in the default language. Even if you override the session
default language with an alternate language using an ISPSTART language keyword, some of the first
initialization messages, issued before the command scan, are in the default session language. However,
any messages issued after processing of the ISPSTART parameters are in the language specified by the
keyword.
If the terminal does not support DBCS, and the default language (or the alternate language selected by
the language keyword) requires DBCS, ISPF uses English as the session language.
In cases where the session language requires DBCS, certain messages are always issued in English. These
messages are:
• ISPF Main task abend.
• ISPF Subtask abend.
• The following required module for the selected language could not be loaded.
• ISPF command not allowed. You are already under ISPF.
• Invalid environment; TSO/E version 2.1 or later required.
Getting ready to run on MVS
Chapter 6. Getting ready to run on MVS  143

## Page 172

Getting ready to run on MVS
144  z/OS: z/OS ISPF User's Guide Vol I
