# Chapter 7. Diagnostic Tools and Information

Source file: f54mc00_v3r1.md
Start page: 931
Page span: 931-960

## Page 931

Chapter 7. Diagnostic Tools and Information
This chapter covers the following topics:
• debugging tools
• The panel trace and file-tailoring trace utilities
• diagnostic information
• common problems that can occur when developing dialogs and using ISPF
ISPF debug tools
The following tools ship with ISPF as samples.
ISRABEND
A CLIST that provides a step-by-step explanation of how to diagnose an abend interactively. It uses
TSO TEST to gather the information that the IBM support organization normally requires.
ISRCSECT
A REXX exec used in conjunction with ISRTCB exec. It takes the entry point of a load module and
begins searching for a specific CSECT. If it finds one, the exec displays the CSECT's eye-catcher.
ISRFIND
A REXX exec that issues a LISTA STATUS and searches for a specified member or load module. Also,
the exec optionally calls AMBLIST to check the MODIFIED, FIXED, and PAGEABLE LPAs and checks
LPALIST and LNKLST (pointed to by system control blocks) for the specified load module. If invoked
under ISPF, the information is displayed via an ISPF table display (panel ISRFINDP) and allows the
user to BROWSE or EDIT the specified member.
ISRPOINT
A REXX exec used in conjunction with the ISRTCB exec. This exec uses the entry point address
obtained from ISRTCB and lists the CSECT eye-catchers associated with that load module.
ISRTCB
A REXX exec that emulates the TSO TEST command LISTMAP. It lists the TCBs and the load modules
(with their entry points) associated with each TCB, without using TSO TEST.
ISRTEST
A CLIST that uses TSO TEST to load the job pack area (JPA) and set breakpoints on entry to a specific
ISPF or PDF CSECT. This allows for the verification of the compilation date associated with the CSECT
with the most recent maintenance level for that version or release. Additionally, you can modify this
sample to set specific breakpoints within the CSECT to identify the failing instruction.
Panel trace command (ISPDPTRC)
The ISPDPTRC command traces the Dialog Manager panel processing that occurs within any screen in
the current ISPF session. You can trace both the execution of panel service calls (DISPLAY, TBDISPL, and
TBQUERY) and the processing that occurs within the Dialog Manager panel code, including the processing
of statements in the )ABCINIT, )ABCPROC, )INIT, )REINIT, and )PROC sections of the panel.
The output from the trace is written to a dynamically allocated VB (variable blocked) data set that has
a record length of 255. Where the ddname ISPDPTRC is preallocated, this data set is used, providing it
refers to a sequential, VB data set with a record length of at least 255.
The ISPDPTRC command starts the trace if it is not running. If the trace is already active, ISPDPTRC
allows you to stop and optionally to view or edit the trace output. ISPDPTRC must be executed while ISPF
is active.
The syntax of the command is:
ISPF debug tools
© Copyright IBM Corp. 1980, 2024 911

## Page 932

ISPDPTRC
END VIEW LIST QUIET
DSP
DISPLAY (
Both
None
In
Out
)
PNL
PANEL (
*
panel_name
panel_mask
)
READ(
Summary
None
Detail
)
SCR
SCREEN (
0
*
screen_id
)
SECT
SECTION (
*
All
None Init
NOInit
Reinit
NOReinit
Proc
NOProc
)
SVC
SERVICE (
Detail
None )
Where:
END
Terminates the trace if it is active. No attempt is made to edit or view the trace data set.
VIEW
Terminates the trace if it is active and views the trace data set. If an allocation for the DD ISPDPTRC is
present, this data set is viewed. SYSOUT data sets are not supported.
When VIEW is unable to locate the trace data set, it performs the LIST processing and displays the list
of panel trace data sets.
LIST
The panel trace command invokes the Data Set List Utility to display panel trace data sets.
Where the user's prefix is not blank, the data set list displayed is for data sets of the form:
prefix.**.ISPPNL.TRACE
Otherwise, the data set list displayed is for data sets of the form:
Panel trace command (ISPDPTRC)
912  z/OS: z/OS ISPF Messages and Codes

## Page 933

userid.**.ISPPNL.TRACE
QUIET
Prevents trace initialization and termination messages being displayed. Error messages continue to be
displayed on the screen.
DISPLAY
Controls the generation of trace records resembling the panel as displayed at the terminal. Only the
panel for the active screen is shown when a panel is being read into memory.
None
No trace records are produced during panel display processing.
In
Generates trace records showing the panel, including data entered after the user has pressed the
Enter key or a function key.
Out
Generates trace records showing the panel as it shown on the screen. Attribute bytes are also
represented in the screen display.
Both
Generates both the In and Out display traces. This is the default.
PANEL
Controls the generation of trace records based on the panel name.
*
Generate trace records for all panels. This is the default.
panel_name
Generates trace records only for the panel name as specified.
panel_mask
Generates trace records for panels matching panel_mask. The mask can contain % to represent a
single character or * to represent any number of characters.
Note: Panel service calls (DISPLAY, TBDISPL, and TBQUERY) continue to be traced for all panels,
regardless of the panel_name or panel_mask parameter specified.
READ
Controls the generation of trace records when a panel is being read into memory.
None
No trace records are produced during the read processing.
Summary
Generates summary information, including where the panel was loaded from (either an ISPPLIB or
LIBDEF data set), and the number of records read until the )END statement was detected. This is
the default setting.
Detail
Generates the same information as for the summary trace, but includes the panel source. Also
shows the return codes and panel source records inserted, modified, and deleted by a panel input
exit. Preprocessed panels can not be displayed.
SCREEN
Controls the generation of trace records based on the screen ID.
0
Generate trace records for the all logical screens. This is the default.
*
Generate trace records for the current screen ID.
screen_id
Generate trace records only for the logical screen ID as specified. The screen ID is a single
character in the range 1-9, A-W.
Panel trace command (ISPDPTRC)
Chapter 7. Diagnostic Tools and Information  913

## Page 934

SECTION
Controls the generation of trace records for the different panel logic sections. The default is all
sections.
* | All
Generates trace records for all sections. Either form of this parameter can only be specified by
itself, and not with any of the other SECTION parameter values.
None
Generates no trace records for any of the panel processing logic sections. This parameter can only
be specified by itself and not in conjunction with any of the other SECTION parameter values.
Init
Generates trace records for the )ABCINIT and )INIT sections.
Reinit
Generates trace records for the )REINIT section.
Proc
Generates trace records for the )ABCPROC and )PROC sections.
NOInit
Turns off the generation of trace records for the )ABCINIT and )INIT sections.
NOReinit
Turns off the generation of trace records for the )REINIT section.
NOProc
Turns off the generation of trace records for the )ABCPROC and )PROC sections.
SERVICE
Controls the generation of trace records for the panel processing service calls, namely DISPLAY,
TBDISPL and TBQUERY.
None
No trace records are produced during the service call processing.
Detail
Generates trace records for the DISPLAY, TBDISPL, and TBQUERY service calls, showing all the
parameters. A trace record is produced both before and after the call processing, with the post
record showing the return code from the service. This is the default setting.
Note:
1. Where neither the END nor VIEW parameters is provided, the panel trace is started if it is not already
active, otherwise the trace is stopped and where possible you are put into an edit session with the
trace output.
2. When the panel trace is already active, only the END and VIEW parameters have any effect on the
command. All other valid parameters are ignored. If invalid parameters are entered the command
terminates without starting to process the trace.
Trace format
Here are the details of the trace format, for the trace header, the display, and the processing trace.
Panel trace command (ISPDPTRC)
914  z/OS: z/OS ISPF Messages and Codes

## Page 935

Panel trace header
========= ISPF Panel Trace ==================== 2019.275 14:06:34 GMT ==========
     ZISPFOS: ISPF FOR z/OS 02.05.00          ZOS390RL: z/OS   02.05.00
     ISPDPTRC Command: ISPDPTRC
    Options in Effect: PANEL(*) SCREEN(0) SECTION(INIT REINIT PROC)
                       READ(SUMMARY) SERVICE(DETAIL) DISPLAY(BOTH)
     Physical Display: PRI=24x80  ALT=62x160
     ISPCDI   Version: ISPCDI 2019.218-BASE z25
     ISPDPA   Version: ISPDPA 2019.218-BASE z25
     ISPDPE   Version: ISPDPE 2019.218-BASE z25
     ISPDPL   Version: ISPDPL 2019.218-BASE z25
     ISPDPP   Version: ISPDPP 2019.218-BASE z25
     ISPDPPI  Version: ISPDPPI 2019.218-BASE z25
     ISPDPPK  Version: ISPDPPK 2019.218-BASE z25
     ISPDPPRX Version: ISPDPPRX 2019.218-BASE z25
     ISPDPR   Version: ISPDPR 2019.218-BASE z25
     ISPDPS   Version: ISPDPS 2019.218-BASE z25
     ISPDTD   Version: ISPDTD 2019.218-BASE z25
     ISPPNXRX Version: ISPPNXRX 2019.218-BASE z25
     ISPPQR   Version: ISPPQR 2019.218-BASE z25
     ISPDPTR0 Version: ISPDPTR0 2019.218-BASE z25
================================================================================
TLD# Type  Panel   Section Cd  RC  Data
---- ---- -------- ------- -- ---  
------------------------------------------------------------------------------------>
Figure 2. Sample Panel Trace header
The trace header shows the following information:
1. Current date and time (GMT) when the trace was initialized
2. ISPF level information as found in dialog variable ZISPFOS
3. z/OS level information as found in dialog variable ZOS390RL
4. ISPDPTRC command with the invocation parameters
5. The options that are in effect for the current execution of the panel trace
6. Module level information for each of the modules associated with ISPF Panel Processing
The remainder of the trace is broken into a number of columns to show each trace record. The columns
are:
TLD#
The task or screen identifier from which the panel service is being invoked.
Type
The trace entry type. The valid types are:
DspI
Records are generated after a user has pressed the Enter key or a function key, and show the
data displayed on the ISPF panel at that time. Attribute bytes are also included in the display. The
generation of this type of trace record is controlled by the ISPDPTRC DISPLAY parameter.
DspO
Records are generated displaying an ISPF panel at the screen. Attribute bytes are also included
in the display. The generation of this type of trace record is controlled by the ISPDPTRC DISPLAY
parameter.
Err
Records are generated when a ISPF panel processing error occurs and ISPF issues an error
message. The records generated include both the short and long error messages.
InEx
Records generated when a panel record or return code is returned from a panel input exit.
PrcR
Records are generated during the processing of the panel logic sections,
including )INIT, )REINIT, )PROC, )ABCINIT and )ABCPROC. The data as displayed resembles that
Panel trace command (ISPDPTRC)
Chapter 7. Diagnostic Tools and Information  915

## Page 936

of the original panel, but may not be identical to it. Where an assignment statement includes
dialog variables or functions, an additional record is displayed showing the result of the
assignment. The generation of this type of trace records is controlled by the ISPDPTRC SECTION
parameter.
Read
Records are generated reading a panel into storage. The generation of this type of trace record is
controlled by the ISPDPTRC READ parameter. A summary trace does not show the panel source
records. The source of preprocessed panels can not be displayed.
RexR
Records that are generated when REXX processing is complete and control is being returned back
to the panel.
Rexx
Records that are generated when a *REXX statement is being processed.
Svc
Records are generated for calls to the ISPF Display Services and show all the call parameters. This
is limited to the DISPLAY, TBDISPL, and TBQUERY services. The generation of this type of trace
record is control by the ISPDPTRC SERVICE parameter.
SvcR
Records are generated returning from the ISPF Display services. The trace includes the return
code from the service.
Var
Records that are generated to show the ISPF variables and their values being passed to the Panel
Exit or Panel REXX command.
VarR
Records that are generated to show the ISPF variables and their values being passed back from
the Panel Exit or Panel REXX command.
Panel
The ISPF panel name associated with the trace record.
Section
The logic section associated with the PrcR type trace record.
Cd
The Condition value returned for IF and ELSE panel statements:
T
Indicates a True condition
F
Indicates a False condition
Note: A plus (+) character in this field indicates a record continuation.
RC
The Return Code, shown only for SvcR, PrcR, and InEx type trace records.
Data
Trace data for the particular trace entry. The trace data extends the full width of the output file and
will wrap if required.
Panel display
Figure 3 on page 917 shows the output and input trace generated for panel ISRUTIL. It includes a
scale line across the top and down the side of the panel, and includes panel size and cursor position
information. The input trace also gives an indication of the key or command entered.
Panel trace command (ISPDPTRC)
916  z/OS: z/OS ISPF Messages and Codes

## Page 937

TLD1 DspO                         0----+----1----+----2----+----3----+----4----+----5----+----6----+----7----+----8
TLD1 DspO ISRUTIL                 |   Menu  Help
TLD1 DspO ISRUTIL                 | ------------------------------------------------------------------------------
TLD1 DspO ISRUTIL                 |                            Utility Selection Panel
TLD1 DspO ISRUTIL                 | Option ===>&
TLD1 DspO ISRUTIL                 +
TLD1 DspO ISRUTIL                 | 1  Library     Compress or print data set.  Print index listing.  Print,
TLD1 DspO ISRUTIL                 |                  rename, delete, browse, edit or view members
TLD1 DspO ISRUTIL                 | 2  Data Set    Allocate, rename, delete, catalog, uncatalog, or display
TLD1 DspO ISRUTIL                 |                  information of an entire data set
TLD1 DspO ISRUTIL                 1 3  Move/Copy   Move, or copy members or data sets
TLD1 DspO ISRUTIL                 | 4  Dslist      Print or display (to process) list of data set names.
TLD1 DspO ISRUTIL                 |                  Print or display VTOC information
TLD1 DspO ISRUTIL                 | 5  Reset       Reset statistics for members of ISPF library
TLD1 DspO ISRUTIL                 | 6  Hardcopy    Initiate hardcopy output
TLD1 DspO ISRUTIL                 + 8  Outlist     Display, delete, or print held job output
TLD1 DspO ISRUTIL                 | 9  Commands    Create/change an application command table
TLD1 DspO ISRUTIL                 | 11 Format      Format definition for formatted data Edit/Browse
TLD1 DspO ISRUTIL                 | 12 SuperC      Compare data sets                             (Standard Dialog)
TLD1 DspO ISRUTIL                 | 13 SuperCE     Compare data sets Extended                    (Extended Dialog)
TLD1 DspO ISRUTIL                 2 14 Search-For  Search data sets for strings of data          (Standard Dialog)
TLD1 DspO ISRUTIL                 | 15 Search-ForE Search data sets for strings of data Extended (Extended Dialog)
TLD1 DspO ISRUTIL                 | 16 Tables      ISPF Table Utility
TLD1 DspO ISRUTIL                 | 17 Udlist      Print or display (to process) z/OS UNIX directory list
TLD1 DspO ISRUTIL  --------------- Screen=23x80  Cursor=4/14
TLD1 DspI                         0----+----1----+----2----+----3----+----4----+----5----+----6----+----7----+----8
TLD1 DspI ISRUTIL                 |   Menu  Help
TLD1 DspI ISRUTIL                 | ------------------------------------------------------------------------------
TLD1 DspI ISRUTIL                 | ISRUTIL                    Utility Selection Panel
TLD1 DspI ISRUTIL                 | Option ===>&4
TLD1 DspI ISRUTIL                 +
TLD1 DspI ISRUTIL                 | 1  Library     Compress or print data set.  Print index listing.  Print,
TLD1 DspI ISRUTIL                 |                  rename, delete, browse, edit or view members
TLD1 DspI ISRUTIL                 | 2  Data Set    Allocate, rename, delete, catalog, uncatalog, or display
TLD1 DspI ISRUTIL                 |                  information of an entire data set
TLD1 DspI ISRUTIL                 1 3  Move/Copy   Move, or copy members or data sets
TLD1 DspI ISRUTIL                 | 4  Dslist      Print or display (to process) list of data set names.
TLD1 DspI ISRUTIL                 |                  Print or display VTOC information
TLD1 DspI ISRUTIL                 | 5  Reset       Reset statistics for members of ISPF library
TLD1 DspI ISRUTIL                 | 6  Hardcopy    Initiate hardcopy output
TLD1 DspI ISRUTIL                 + 8  Outlist     Display, delete, or print held job output
TLD1 DspI ISRUTIL                 | 9  Commands    Create/change an application command table
TLD1 DspI ISRUTIL                 | 11 Format      Format definition for formatted data Edit/Browse
TLD1 DspI ISRUTIL                 | 12 SuperC      Compare data sets                             (Standard Dialog)
TLD1 DspI ISRUTIL                 | 13 SuperCE     Compare data sets Extended                    (Extended Dialog)
TLD1 DspI ISRUTIL                 2 14 Search-For  Search data sets for strings of data          (Standard Dialog)
TLD1 DspI ISRUTIL                 | 15 Search-ForE Search data sets for strings of data Extended (Extended Dialog)
TLD1 DspI ISRUTIL                 | 16 Tables      ISPF Table Utility
TLD1 DspI ISRUTIL                 | 17 Udlist      Print or display (to process) z/OS UNIX directory list
TLD1 DspI ISRUTIL  --------------- Screen=23x80  Cursor=4/15  Key=ENTER
Figure 3. Sample DISPLAY trace
Panel processing trace
Figure 4 on page 918 shows an example of the trace generated when processing the PROC section of
panel ISRUTIL after the number 4 was entered in the command field. Statements skipped as the result
of a "false" condition on an IF or ELSE statement are never displayed. In addition, the panel trace always
splits the value pairs for the TRANS functions into separate records, making the trace more readable.
The result of an assignment statement is only shown when the assignment statement includes a dialog
variable, an including panel control variable, or a panel function.
Panel REXX is not traced. This should be traced using normal REXX trace capabilities.
Panel trace command (ISPDPTRC)
Chapter 7. Diagnostic Tools and Information  917

## Page 938

TLD1 PrcR ISRUTIL  PROC         0  &ZCMDWRK=&Z
TLD1 PrcR ISRUTIL  PROC       ->   &ZCMDWRK=''
TLD1 PrcR ISRUTIL  PROC    T    0  IF(&ZCMD ^= &Z)
TLD1 PrcR ISRUTIL  PROC         0    &ZCMDWRK=TRUNC(&ZCMD,'.')
TLD1 PrcR ISRUTIL  PROC       ->     &ZCMDWRK=4
TLD1 PrcR ISRUTIL  PROC         0    &ZTRAIL=.TRAIL
TLD1 PrcR ISRUTIL  PROC       ->     &ZTRAIL=''
TLD1 PrcR ISRUTIL  PROC    F    0    IF(&ZCMDWRK = &Z)
TLD1 PrcR ISRUTIL  PROC         0  &ZSEL=TRANS(TRUNC(&ZCMD,'.')
TLD1 PrcR ISRUTIL  PROC    +             1,'PGM(ISRUDA) PARM(ISRUDA1) SCRNAME(LIBUTIL)'
TLD1 PrcR ISRUTIL  PROC    +             2,'PGM(ISRUDA) PARM(ISRUDA2) SCRNAME(DSUTIL)'
TLD1 PrcR ISRUTIL  PROC    +             3,'PGM(ISRUMC) SCRNAME(MCOPY)'
TLD1 PrcR ISRUTIL  PROC    +             4,'PGM(ISRUDL) PARM(ISRUDLP) SCRNAME(DSLIST)'
TLD1 PrcR ISRUTIL  PROC    +             5,'PGM(ISRURS) SCRNAME(RESET)'
TLD1 PrcR ISRUTIL  PROC    +             6,'PGM(ISRUHC) SCRNAME(HARDCOPY)'
TLD1 PrcR ISRUTIL  PROC    +             8,'PGM(ISRUOLP) SCRNAME(OUTLIST)'
TLD1 PrcR ISRUTIL  PROC    +             9,'PANEL(ISPUCMA) ADDPOP SCRNAME(CMDTABLE)'
TLD1 PrcR ISRUTIL  PROC    +             11,'PGM(ISRFMT) SCRNAME(FORMAT)'
TLD1 PrcR ISRUTIL  PROC    +             12,'PGM(ISRSSM) SCRNAME(SUPERC)'
TLD1 PrcR ISRUTIL  PROC    +             13,'PGM(ISRSEPRM) SCRNAME(SUPERCE) NOCHECK'
TLD1 PrcR ISRUTIL  PROC    +             14,'PGM(ISRSFM) SCRNAME(SRCHFOR)'
TLD1 PrcR ISRUTIL  PROC    +             15,'PGM(ISRSEPRM) PARM(S4) SCRNAME(SRCHFORE) NOCHECK'
TLD1 PrcR ISRUTIL  PROC    +             16,'PGM(ISRUTABL) NEWPOOL SCRNAME(TBLUTIL)'
TLD1 PrcR ISRUTIL  PROC    +             17,'PGM(ISRUUDL) PARM(isruudlp) SCRNAME(UDLIST)'
TLD1 PrcR ISRUTIL  PROC    +             ' ',' '
TLD1 PrcR ISRUTIL  PROC    +             '*','?')
TLD1 PrcR ISRUTIL  PROC       ->   &ZSEL='PGM(ISRUDL) PARM(ISRUDLP) SCRNAME(DSLIST)'
Figure 4. Sample PROCESS trace
File tailoring trace command (ISPFTTRC)
The ISPFTTRC command traces the processing of file tailoring services that are invoked from any screen
within the current ISPF session. You can trace both the execution of file tailoring service calls (FTOPEN,
FTINCL, FTCLOSE, and FTERASE) and the processing that occurs within the file tailoring code and
processing of each statement.
The output from the trace is written to a dynamically allocated VB (variable blocked) data set that has
a record length of 255. Where the ddname ISPFTTRC is preallocated, this data set is used, providing it
refers to a sequential, VB data set with a record length of at least 255.
The ISPFTTRC command starts the trace if it is not running. If the trace is already active, ISPFTTRC allows
you to stop and optionally to view or edit the trace output. ISPFTTRC must be executed while ISPF is
active.
The syntax of the command is:
File tailoring trace command (ISPFTTRC)
918  z/OS: z/OS ISPF Messages and Codes

## Page 939

ISPFTTRC
END VIEW LIST QUIET
READ(
Detail
None
Summary
)
REC
RECORDS (
*
All
None Src
Source
Data
NOData
Cntl
NOCntl
NOSrc
NOSource
)
SCR
SCREEN (
0
*
screen_id
)
SVC
SERVICE (
Detail
None )
SKL
SKEL
SKELETON (
*
skel_name
skel_mask
)
TBV
TBVARS (
Detail
None )
Where:
END
Terminates the trace if it is active. No attempt is made to edit or view the trace data set.
VIEW
Terminates the trace if it is active and views the trace data set. If an allocation for the DD ISPFTTRC is
present, this data set is viewed. SYSOUT data sets are not supported.
File tailoring trace command (ISPFTTRC)
Chapter 7. Diagnostic Tools and Information  919

## Page 940

When VIEW is unable to locate the trace data set, it performs the LIST processing and displays the list
of panel trace data sets.
LIST
The file tailoring trace command invokes the Data Set List Utility to display file tailoring trace data
sets.
Where the user's prefix is not blank, the data set list displayed is for data sets of the form:
prefix.**.ISPFT.TRACE
Otherwise, the data set list displayed is for data sets of the form:
userid.**.ISPFT.TRACE
QUIET
Prevents trace initialization and termination messages being displayed. Error messages continue to be
displayed on the screen.
READ
Controls the generation of trace records when a skeleton member is being read into memory.
None
No trace records are produced during the read processing.
Summary
Generates summary information, including where the skeleton was loaded from (either an
ISPSLIB or LIBDEF data set), and the number of records read.
Detail
Generates the same information as for the summary trace, but includes the skeleton source. This
is the default setting.
RECORDS
Controls the generation of trace records during record processing of the skeleton member.
*, All
Generates trace records for all skeleton record processing. Either form of this parameter can only
be specified by itself, and not with any of the other RECORDS parameter values.
None
Generates no trace records for any of the skeleton record processing. This parameter can only be
specified by itself and not in conjunction with any of the other RECORDS parameter values.
Source
Generates trace records for the source skeleton record. This is performed before any processing is
done to determine if it is a data or control record.
Data
Generates trace records for the data records. This is performed after record processing has
completed.
Cntl
Generates trace records for the control statements. This is performed after record processing has
completed.
NOSource
Turns off the generation of trace records for the source skeleton records.
NOData
Turns off the generation of trace records for the data records.
NOCntl
Turns off the generation of trace records for the control statements.
SCREEN
Controls the generation of trace records based on the screen ID.
File tailoring trace command (ISPFTTRC)
920  z/OS: z/OS ISPF Messages and Codes

## Page 941

0
Generate trace records for the all logical screens. This is the default.
*
Generate trace records for the current screen ID.
screen_id
Generate trace records only for the logical screen ID as specified. The screen ID is a single
character in the range 1-9, A-W.
SERVICE
Controls the generation of trace records for the file tailoring service calls, namely OPEN, FTINCL,
FTCLOSE, and FTERASE.
None
No trace records are produced during the service call processing.
Detail
Generates trace records for the OPEN, FTINCL, FTCLOSE, and FTERASE service calls, showing all
the parameters. A trace record is produced both before and after the call processing, with the post
record showing the return code from the service. This is the default setting.
SKELETON
Controls the generation of trace records based on the skeleton name.
*, All
Generate trace records for all skeletons. This is the default.
skel_name
Generates trace records only for the skeleton name as specified.
skel_mask
Generates trace records for skeletons matching skel_mask. The mask can contain % to represent
a single character or * to represent any number of characters.
Note: File tailoring service calls (OPEN, FTINCL, FTCLOSE, and FTERASE) continue to be traced for all
skeleton processing, regardless of the skel_name or skel_mask parameter specified.
TBVARS
Used on a )DOT control word to display key variables and named variables on each iteration through
the table.
None
No trace records are produced during )DOT processing.
Detail
Generates trace records for the )DOT control word, displaying key variables and named table
variables on each iteration. Extension variables are not displayed. This is the default setting.
Note:
1. Where neither the END nor VIEW parameters are provided, the file tailoring trace is started if it is not
already active, otherwise the trace is stopped and where possible you are put into an edit session with
the trace output.
2. When the file tailoring trace is already active, only the END and VIEW parameters have any effect on
the command. All other valid parameters are ignored. If invalid parameters are entered the command
terminates without starting to process the trace.
Trace format
Here are the details of the trace format, for the trace header and the processing trace.
File tailoring trace command (ISPFTTRC)
Chapter 7. Diagnostic Tools and Information  921

## Page 942

File tailoring trace header
========= ISPF File Tailoring Trace =========== 2005.305 01:48:01 GMT ==========
     ZISPFOS: ISPF FOR z/OS 01.08.00          ZOS390RL: z/OS   01.05.00
     ISPFTTRC Command: ISPFTTRC
    Options in Effect: SKELETON(*) SCREEN(0) RECORDS(SOURCE CNTL DATA)
                       READ(DETAIL) SERVICE(DETAIL) TBVARS(DETAIL)
      ISPFICRX Version: ISPFICRX 05286-BASE z/18
      ISPFICWC Version: ISPFICWC 05286-BASE z/18
      ISPFICWD Version: ISPFICWD 05286-BASE z/18
      ISPFICWE Version: ISPFICWE 05286-BASE z/18
      ISPFICWL Version: ISPFICWL 05286-BASE z/18
      ISPFICWT Version: ISPFICWT 05286-BASE z/18
      ISPFICWX Version: ISPFICWX 05286-BASE z/18
      ISPFIEND Version: ISPFIEND 05286-BASE z/18
      ISPFIINT Version: ISPFIINT 05286-BASE z/18
      ISPFILBS Version: ISPFILBS 05284-BASE z/18
      ISPFITLR Version: ISPFITLR 05284-BASE z/18
      ISPFITR0 Version: ISPFITR0 05297-BASE z/18
      ISPFITRV Version: ISPFITRV 05286-BASE z/18
================================================================================
TLD# Type Skeleton  Rec#   IM IF DO TB  Cd RC Data
---- ---- -------- ------  -- -- -- --  -- -- ----------------------------- … -->
Figure 5. Sample file  tailoring trace header
The trace header shows the following information:
1. Current date and time (GMT) when the trace was initialized
2. ISPF level information as found in dialog variable ZISPFOS
3. z/OS level information as found in dialog variable ZOS390RL
4. ISPFTTRC command with the invocation parameters
5. The options that are in effect for the current execution of the file tailoring trace
6. Module level information for each of the modules associated with file tailoring and skeleton processing
The remainder of the trace is broken into a number of columns to show each trace record. The columns
are:
TLD#
The task or screen identifier from which the file tailoring is being invoked.
Type
The trace entry type. The valid types are:
CtlR
Records are generated when record processing has completed and the record was determined
to be a control statement. The generation of CtlR trace records is controlled by the ISPFTTRC
RECORDS parameter.
DatR
Records are generated when record processing has completed and the record was determined to
be a data record. The generation of DatR trace records is controlled by the ISPFTTRC RECORDS
parameter.
Err
Records are generated when a file tailoring processing error occurs and ISPF issues an error
message. The generated records include both the short and long error messages.
FncI
Records are generated when a built-in function has been identified and is ready to be evaluated.
FncR
Records are generated when a built-in function has been evaluated.
File tailoring trace command (ISPFTTRC)
922  z/OS: z/OS ISPF Messages and Codes

## Page 943

NoFT
Records are generated after the point where the NOFT parameter is specified on the FTINCL
service call, or the point where the NT option is specified on the )IM control statement. The
generation of NoFT trace records is controlled by the ISPFTTRC RECORDS parameter.
Read
Records are generated reading a skeleton into storage. The generation of Read trace records is
controlled by the ISPFTTRC READ parameter. A summary trace does not show the skeleton source
records.
RexR
Records are generated when REXX processing is complete and control is being returned back to
the file tailoring.
Rexx
Records are generated when a )REXX control statement is being processed.
Src
Records are generated when a skeleton record is selected for processing. The generation of Src
trace records is controlled by the ISPFTTRC RECORDS parameter.
Svc
Records are generated for calls to the ISPF file tailoring services and show all the call parameters.
This is limited to the FTOPEN, FTINCL, FTCLOSE, and FTERASE services. The generation of Svc
trace records is controlled by the ISPFTTRC SERVICE parameter.
SvcR
Records are generated returning from the ISPF file tailoring services. The trace includes the return
code from the service. The FTCLOSE return trace entry includes an additional record showing the
number of records written to the file tailoring output data set.
Var
Records that are generated to show the ISPF variables and their values being passed to the file
tailoring REXX command.
VarR
Records that are generated to show the ISPF variables and their values being passed back from
the file tailoring REXX command.
Skeleton
The ISPF skeleton name associated with the trace record.
Record
Display the record number associated with the trace entry type. For Read, Src, and CtlR the input
record number from the skeleton member is displayed. (For control statements that are continued
over more than one line this is always the record number associated with the first line of the control
statement.) For DatR and NoFT, the output record number is displayed. This field is blank for all other
record types.
IM
The current imbed level. The skeleton name specified on the FTINCL service is always level 1.
IF
The current IF or SEL level. This field is blank if no )IF or )SEL statement is being processed.
DO
The current DO level. This field is blank if no )DO structure is being processed.
TB
The current Table level. This field is blank if no )DOT structure is being processed.
Cd
The Condition value returned for the following skeleton control statements:
• )IF, )SEL, )UNTIL, or )WHILE statement
T
Indicates a True condition
File tailoring trace command (ISPFTTRC)
Chapter 7. Diagnostic Tools and Information  923

## Page 944

F
Indicates a False condition
• )ENDDO, or )ENDDOT statement
X
Indicates the corresponding )DO or )DOT control statement is terminating. In other words, the
exit condition has been met.
• )IM statement with OPT parameter
X
Imbed member was not found. File tailoring processing will continue.
Note: A plus (+) character in this field indicates a record continuation.
RC
The Return Code, shown only for SvcR, DatR, and CtlR trace entries.
Data
Trace data for the particular trace entry. The trace data extends the full width of the output file and
will wrap if required.
File tailoring processing trace
TLD# Type Skeleton  Rec#   IM IF DO TB  Cd RC Data
---- ---- -------- ------  -- -- -- --  -- -- ----------------------------- … -->
TLD1 Svc                                      FTOPEN  TEMP
--------------------------------------------- DD=ISP14484  DSN=LSACKV1.SPFTEMP1.CNTL
TLD1 SvcR                                   0 FTOPEN  TEMP
TLD1 Svc                                      FTINCL  SKREX1A  EXT
--------------------------------------------- DD=ISPSLIB   DSN=LSACKV2.ISPSLIB
TLD1 Read SKREX1A       1                     >>1A>>START>> REXX >>
TLD1 Read SKREX1A       2                     )SET VARLIST = &STR(VAR1  VAR2,VAR3 )
TLD1 Read SKREX1A       3                     )SET VAR1 = SAY
TLD1 Read SKREX1A       4                     )SET VAR2 = HI
TLD1 Read SKREX1A       5                     )SET VAR3 = &STR(TO REXX  )
TLD1 Read SKREX1A       6                     )SET VAR4 = &STR(:)
TLD1 Read SKREX1A       7                     )REXX &VARLIST VAR4
TLD1 Read SKREX1A       8                      SAY VAR1 VAR2 VAR3 VAR4
TLD1 Read SKREX1A       9                      VAR3 = 'from rexx to you'
TLD1 Read SKREX1A      10                     )ENDREXX
TLD1 Read SKREX1A      11                     >>1A>>-END-<< &VAR1 &VAR2 &VAR3
TLD1 Read SKREX1A  -------------------------- Total Records=11
TLD1 Src  SKREX1A       1   1                 >>1A>>START>> REXX >>
TLD1 DatR SKREX1A       1   1               0 >1A>START> REXX >
TLD1 Src  SKREX1A       2   1                 )SET VARLIST = &STR(VAR1  VAR2,VAR3 )
TLD1 FncI SKREX1A       2   1                 &STR(VAR1  VAR2,VAR3 )
TLD1 FncR SKREX1A       2   1               0 = VAR1  VAR2,VAR3
TLD1 FncR SKREX1A                              +000060  0000000B 00000003 800000…
     ⋮
TLD1 CtlR SKREX1A      10   1               0 )ENDREXX
TLD1 RexR SKREX1A                             ZFTXRC(2)=0
TLD1 RexR SKREX1A                             ZFTXMSG(8)=
TLD1 RexR SKREX1A                             VAR1(3)=SAY
TLD1 RexR SKREX1A                             VAR2(2)=HI
TLD1 RexR SKREX1A                             VAR3(9)=from rexx
TLD1 RexR SKREX1A                             VAR4(1)=:
TLD1 Src  SKREX1A      11   1                 >>1A>>-END-<< &VAR1 &VAR2 &VAR3
TLD1 DatR SKREX1A       2   1               0 >1A>-END-< SAY HI from rexx
TLD1 SvcR                                   0 FTINCL  SKREX1A  EXT
TLD1 Svc                                      FTCLOSE
--------------------------------------------- DD=ISP09474  DSN=LSACKV1.SPFTEMP1.CNTL
TLD1 SvcR                                   0 FTCLOSE
TLD1 SvcR          -------------------------- Total Records=2
Figure 6. Sample file  tailoring process trace
Diagnostic information
This section is intended to help you gather information to diagnose ISPF problems.
Diagnostic information
924  z/OS: z/OS ISPF Messages and Codes

## Page 945

Using the ENVIRON system command
ISPF provides the ENVIRON command to assist you in gathering data that can be helpful in diagnosing
problems, thus reducing service time. The ISPF session does not have to be running in any ISPF TEST/
TRACE mode when you use the ENVIRON command.
The ENVIRON command can help you to:
• Produce system abend dumps when not running in ISPF TEST mode (ENBLDUMP parameter).
• Trace the TPUT, TGET, and PUTLINE buffers and obtain dump information for TPUT and TGET errors
(TERMTRAC parameter).
• Gather terminal status information (TERMSTAT parameter).
• Gather Rexx diagnostic information.
You can display a panel (Figure 7 on page 925) for selecting command options by entering the ENVIRON
command with no parameters, or display the panel through the use of the Environ settings... choice from
the Environ pull-down on the ISPF Settings panel. This panel includes the current values of the ENVIRON
command parameters (ENBLDUMP and TERMTRAC) and the ddname, if any, allocated for a dump data
set. The values can be changed by entering new values directly on the panel.
   Log/List  Function keys  Colors  Environ  Identifier  Help
┌─────────────────────────────── ISPF Settings ───────────────────────────────┐
│                       ISPF ENVIRON Command Settings                         │
│                                                                             │
│ Enter "/" to select option                                                  │
│    Enable a dump for a subtask abend when not in ISPF TEST mode             │
│                                                                             │
│ Terminal Tracing (TERMTRAC)                                                 │
│   Enable . . . 3  1. Enable terminal tracing (ON)                           │
│                   2. Enable terminal tracing when a terminal error is       │
│                      encountered (ERROR)                                    │
│                   3. Disable terminal tracing (OFF)                         │
│   DDNAME . . . ISPSNAP   (DDNAME for TERMTRAC ON, ERROR, or DUMP.)          │
│                                                                             │
│ Terminal Status (TERMSTAT)                                                  │
│   Enable . . . 3  1. Yes, invoke TERMSTAT immediately                       │
│                   2. Query terminal information                             │
│                   3. No                                                     │
│                                                                             │
│ Rexx ENVBLK check (REXCHK)                                                  │
│   Enable . . . 3  1. ON, check Rexx ENVBLK pointer                          │
│                   2. Dump, dump if bad ENVBLK pointer                       │
│                   3. OFF                                                    │
│ Command ===>                                                                │
│  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
│  F9=Swap       F12=Cancel                                                   │
⋘─────────────────────────────────────────────────────────────────────────────┘
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
F10=Actions  F12=Cancel
Figure 7. ENVIRON Settings Panel (ISPENVA)
You can issue the ENVIRON command at any time during an ISPF session.
ENVIRON command syntax and parameter descriptions
The general syntax for the ENVIRON command is:
Diagnostic information
Chapter 7. Diagnostic Tools and Information  925

## Page 946

ENVIRON
ENBLDUMP
ON
OFF TERMTRAC
ON
ERROR
DUMP
OFF
TERMSTAT
QUERY REXCHK
OFF
ON
DUMP
The parameter descriptions for the ENVIRON command are as follows:
ENBLDUMP
Specifying the ENBLDUMP parameter enables ISPF to produce an abend dump if a subtask
abnormally terminates when ISPF is not running in TEST mode. The ENBLDUMP parameter does
not apply to attached commands. Before a dump is taken you must allocate either the SYSUDUMP,
SYSMDUMP, or SYSABEND ddname. For more information about these data sets, refer to z/OS MVS
Diagnosis: Tools and Service Aids.
The default value for the ENBLDUMP parameter is ON. ENVIRON ENBLDUMP ON specifies to ISPF that
a dump is to be generated for the subtask that abended.
Issuing ENVIRON ENBLDUMP OFF cancels the effect of the ON status.
The ENBLDUMP parameter value is preserved across ISPF sessions in the ISPSPROF profile.
With ENBLDUMP active, even when ISPF is not running in TEST mode, abnormal termination of a
subtask results in a dump being taken and control being returned to TSO. ISPF execution is not
resumed.
When running in ISPF TEST mode, issuing ENVIRON ENBLDUMP has no effect on dump processing.
TERMTRAC
Specifying the TERMTRAC parameter allows you to trace all terminal input and output data (TPUT,
TGET, PUTLINE) during an ISPF session. The TERMTRAC parameter also allows you to turn on in-core
tracing and cause ISPF to produce a SNAP dump if the TPUT or TGET service results in an error. ISPF
does not have to be running in TSO TEST mode.
Note: The ENVIRON TERMTRAC buffer does not include:
• The TPUT/TGET instructions issued to query the terminal:
– At ISPF initialization
– By the ENVIRON TERMSTAT command
• The TPUT instruction issued to clear the screen at ISPF termination
• Under certain severe ISPF error conditions, the TPUT instruction issued to display a severe error line
message
Before issuing the ENVIRON TERMTRAC DUMP command you must have first issued the ENVIRON
TERMTRAC ON or ENVIRON TERMTRAC ERROR command.
Before using the TERMTRAC option, you must define to ISPF the ddname for the data set to be
used for the SNAP macro, which ISPF invokes to provide data stream dumps. The ddname can be
defined by specifying it on the panel displayed as a result of either issuing the ENVIRON command
with no parameters, or selecting the "Environ settings" choice from the Environ pull-down on the
ISPF Settings panel. You must follow the data set characteristics guidelines defined by MVS for the
SNAP macro. See z/OS MVS Programming: Assembler Services Guide for DCB information that can be
specified for the SNAP ddname.
The terminal data stream buffer used for ENVIRON TERMTRAC data collection is not reset to zeros.
Diagnostic information
926  z/OS: z/OS ISPF Messages and Codes

## Page 947

Subparameters define terminal data tracing as follows:
• ENVIRON TERMTRAC ON
Activates TPUT, TGET, and PUTLINE buffer tracing of the terminal data stream. All data is retained
in a 24K buffer provided by ISPF. No buffer entry is fragmented. If an entry will not fit into the
remaining buffer space, ISPF issues a SNAP to capture the buffer data. The next trace entry is stored
at the top of the buffer, regardless of the status of the SNAP execution.
Messages are displayed to the user only for errors during SNAP execution. No messages are
displayed during dumps taken as a result of the data buffer filling.
Because ENVIRON TERMTRAC ON causes a SNAP dump to be taken each time the buffer fills, the
ddname that you allocate for the SNAP macro should have a disposition of MOD. This assures that
no trace data is lost.
The layout of the terminal data buffer for all SNAP dumps is:
1 TPUT/TGET/PUTLINE BUFFER TRACE
  2 Header of 8 bytes initialized to
    TERMTRAC
  2 4-byte pointer to where the next entry
    is to be placed
  2 Reserved (20 bytes, for 32-byte boundary
    alignment)
  2 TPUT/TGET/PUTLINE DATA (*)
    3 8-byte TPUT/TGET/PUTLINE identifier
    3 4-byte pointer to previous entry
    3 Information specific to the terminal
      type identifier.
The TPUT/TGET identifiers and specific information for each is as follows. Each buffer entry is
aligned on a 32-byte boundary.
TGET
Before issuing TGET SVC. 4-byte pointer to previous entry. General purpose registers 0, 1, and
15:
   R0  = input data area size
   R1  = input data area pointer
   R15 = TGET option byte
TGETR
Return from TGET SVC. 4-byte pointer to previous entry. General purpose registers 1 and 15:
   R1  = input data length
   R15 = TGET return code
4-byte length of data stream.
Data stream.
TPUT
Before issuing edit TPUT macro. 4-byte pointer to previous entry. General purpose registers 0, 1,
and 15:
   R0  = output data area
   R1  = output data area pointer
   R15 = TPUT option byte
4-byte length of data stream.
Data stream.
TPUTR
Return from edit TPUT macro. 4-byte pointer to previous entry. General purpose register 15:
   R15 = TPUT return code
Diagnostic information
Chapter 7. Diagnostic Tools and Information  927

## Page 948

TPUTNE
before issuing the noedit TPUT macro. 4-byte pointer to previous entry. General purpose
registers 0, 1, and 15:
   R1  = address of plist
   R15 = TPUT option byte
16-byte noedit plist:
   Reserved (2 bytes)
   2-byte length of data stream
   Code (1 byte)
   3-byte addr of data stream
   Reserved (8 bytes)
Data stream.
TPUTNER
Return from noedit TPUT macro. 4-byte pointer to previous entry. General purpose register 15:
   R15 = TPUT return code
PUTLINE
Before issuing the PUTLINE macro. 4-byte pointer to previous entry 12-byte PUTLINE parameter
block:
   Control flags (2 bytes)
   2-byte TPUT options field
   4-byte address of message
   4-byte address of format-only line
125-byte message description:
   2-byte message length
   2-byte message offset
   121-byte message
Actions that occur as a result of issuing the ENVIRON TERMTRAC command when ENVIRON
TERMTRAC ON is already in effect are listed by command subparameter below:
ON
ENVIRON TERMTRAC ON continues to function normally.
OFF
Tracing is turned off and ISPF issues a SNAP macro. If ENVIRON TERMTRAC tracing is requested
again, the next entry is written at the top of the buffer, regardless of whether the prior SNAP was
successful.
ERROR
Changes the setting of the command to ENVIRON TERMTRAC ERROR. Tracing continues, with
the next buffer entry being written after the last entry written by the ENVIRON TERMTRAC ON
setting.
DUMP
The ENVIRON TERMTRAC ON condition continues. In addition, ISPF issues a SNAP macro and, if
the SNAP is successful, the next trace entry is written at the top of the buffer. If the SNAP fails,
the next entry is written after the last entry before the SNAP.
• ENVIRON TERMTRAC ERROR
Initiates tracing of the TPUT, TGET, and PUTLINE buffers. In addition, it causes ISPF to initiate a
SNAP dump if a TPUT or TGET error occurs. The dump includes the storage trace buffer, the current
TCB, all system control program information, and all problem program information. The SNAP macro
definition provides more specific information about the areas dumped when all system control
program and problem program information is requested.
ISPF issues the SNAP macro on the first occurrence of a TPUT failure. ISPF makes three consecutive
attempts to correct a TPUT error.
Diagnostic information
928  z/OS: z/OS ISPF Messages and Codes

## Page 949

Before using this option, you must have defined the ddname for the SNAP macro as described
earlier in this topic under TERMTRAC.
Actions that occur as a result of issuing the ENVIRON TERMTRAC command when ENVIRON
TERMTRAC ERROR is already in effect are listed by command subparameter below:
ON
Changes the setting of the command to ENVIRON TERMTRAC ON. Tracing continues, with the
next buffer entry being written after the last entry written by the ENVIRON TERMTRAC ON
setting.
ERROR
ENVIRON TERMTRAC ERROR continues to function normally, with the next trace entry written
after the last ERROR trace entry.
OFF
The setting for ENVIRON TERMTRAC is set to OFF. If ENVIRON TERMTRAC tracing is requested
again, the next entry is written at the top of the buffer, regardless of whether the prior SNAP was
successful.
DUMP
The ENVIRON TERMTRAC ERROR condition continues. In addition, ISPF issues a SNAP macro
and, if the SNAP is successful, the next trace entry is written at the top of the buffer. If the SNAP
fails, the next entry is written after the last entry before the SNAP.
• ENVIRON TERMTRAC DUMP
Causes ISPF to immediately issue a SNAP macro, but only if ENVIRON TERMTRAC ON or ENVIRON
TERMTRAC ERROR is active. The resulting dump includes the storage trace buffer, the current TCB,
all system control program information, and all problem program information. The SNAP macro
definition provides more specific information about the areas dumped when all system control
program and problem program information is requested.
Note:
1. This command execution does not turn off terminal data stream tracing if it is active at the time.
2. The next entry is written to the top of the terminal data buffer if the SNAP was successful;
otherwise, tracing continues immediately after the last trace buffer entry.
• ENVIRON TERMTRAC OFF
Resets active ENVIRON TERMTRAC ON and ENVIRON TERMTRAC ERROR commands. If ENVIRON
TERMTRAC is active, ISPF issues a SNAP macro.
The TERMTRAC parameter value is preserved across ISPF sessions in the ISPSPROF profile. The
ddname specified for TERMTRAC on the ENVIRON option panel is also saved across sessions.
TERMSTAT
Specifying the TERMSTAT option of the ENVIRON command allows you to collect information about
the characteristics of the terminal you are using and the line to which it is attached. The information is
returned to your terminal by using line mode, and is written to the ISPF log data set.
The description below of the information returned from an ENVIRON TERMSTAT request is divided
into three parts:
• A list of terminal characteristics as defined in ISPF variables. In other words, this list defines what
ISPF thinks your terminal characteristics are.
• A list of terminal characteristics as defined within TSO.
• A list of structured fields that apply only to terminals with extended data stream (EDS) capability.
If you issue ENVIRON TERMSTAT (without the QUERY parameter) ISPF unconditionally returns
information from lists A and B (below). In addition, if your terminal is connected to a port that
supports extended data streams, ISPF returns information from list C (below).
If your terminal is one that supports extended data streams, such as an IBM 3279, but is connected
to a non-EDS port, you can issue ENVIRON TERMSTAT QUERY to force ISPF to return information from
Diagnostic information
Chapter 7. Diagnostic Tools and Information  929

## Page 950

list C. Be aware that if you issue ENVIRON TERMSTAT QUERY, and your terminal is not a type that
supports extended data streams, such as the IBM 3277, you will receive an ORDER STREAM CHECK
error.
Information returned as a result of issuing the ENVIRON TERMSTAT command is as follows:
List A – Terminal Characteristics as Defined Within ISPF
   14-bit terminal addressing mode (ON or OFF)
   16-bit terminal addressing mode (ON or OFF)
   Color mode (ON or OFF)
   Highlighting mode (ON or OFF)
   DBCS mode (ON or OFF)
   Primary screen size (length, width, total bytes)
   Alternate screen size (length, width, total bytes)
   Partition screen size (length, width, total bytes)
   ISPF terminal buffer data (TSB ptr., TSB size,
    TPP addr.)
List B – Terminal Characteristics as Defined Within TSO
   Return code from GTTERM
   Primary screen information (rows, columns)
   Alternate screen information (rows, columns)
   Screen attribute value
   Character set (ASCII or EBCDIC)
   Extended data streams or non-EDS support
   Return code from GTSIZE
   GTSIZE information (rows, columns)
   Access method being used (VTAM*)
List C – Terminals Supporting EDS (structured fields)
   Usable areas
   Partitions
   Character sets
   Color
   Highlighting
   Reply modes
   PC 3270
   Implicit partition
   Input control
   Field rule
• ENVIRON TERMSTAT QUERY
The QUERY parameter allows you to request terminal data related to extended data stream
capability, even though your terminal is connected to a port that does not support extended data
streams.
REXCHK
ENVIRON REXCHK should only be used at the request of IBM service personnel.
Abend panels provide diagnostic information
When ISPF processing ends abnormally, diagnostic panels are available for displaying:
• Task abend code
• Reason code
• Module name
• Entry point address
• Program-Status Word (PSW)
• Register content at the time of the abend
This information is used in logged abend messages. A tutorial panel displays a list of the common abend
codes.
Diagnostic information
930  z/OS: z/OS ISPF Messages and Codes

## Page 951

On abnormal ISPF termination, the Error Recovery panel shown in Figure 8 on page 931 indicates the
abend code and reason code.
                                 Error Recovery
 Command ===>
      * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
      * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
      * *               ISPF processor ended abnormally               * *
      * *                                                             * *
      * *          System abend code        0C1                       * *
      * *                       Reason code 01                        * *
      * *                                                             * *
      * *                                                             * *
      * *                                                             * *
      * *  Note: The ABEND and REASON codes displayed above are       * *
      * *        HEXADECIMAL values for "SYSTEM" abends and DECIMAL   * *
      * *        values for "USER" abends.                            * *
      * *                                                             * *
      * *  Enter HELP command for list of common ABEND codes.         * *
      * *  Press ENTER key for additional DIAGNOSTIC information.     * *
      * *  Enter END command to display primary option menu.          * *
      * *                                                             * *
      * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
      * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
Figure 8. Error Recovery Panel (ISPPRS1)
If the SDWA (System Diagnostic Work Area) Reason Code is not supplied, that is, the SDWA reason code
flag bit is OFF, the Reason Code panel field is blank. If the abend code documentation indicates that the
reason code is in a particular register, see the contents of that register, which can be displayed on the
Additional Diagnostic Information panel as shown in Figure 9 on page 931.
If you enter HELP, ISPF displays a list of the common abend codes. To return to the Error Recovery panel,
enter END from the Common ABEND panel.
If you press Enter from the Error Recovery panel, the Additional Diagnostic Information panel is
displayed. Figure 9 on page 931 shows sample data where the SDWA extension is installed. The format
for the register content is slightly different if the SDWA extension is not present.
                       Additional Diagnostic Information
 Command ===>
                                                                    More:     +
               System abend code        = 0C1
                            Reason code = 01
          ISPF Release Level : 5.7.0000
          Module name  . . . : ASMTEST
          Entry point address  0000D488
          PSW  . . . . . . . : 078D1000  0000D4BC
          Register content:
          R0  00000000 - 16308E22 R1  00000000 - 00048EA4
          R2  00000000 - 0000D4D0 R3  00000000 - 00048AC0
          R4  00000000 - 00048AAC R5  00000000 - FFFFFFFF
          R6  00000000 - 00000000 R7  00000000 - 00000001
          R8  00000000 - 00000000 R9  00000000 - 00039060
          R10 00000000 - 00048AA8 R11 00000000 - 00000000
          R12 00000000 - 0000D488 R13 00000000 - 0000D4D0
          R14 00000000 - 80FCC860 R15 00000000 - 0000D488
Figure 9. Additional Diagnostic Information panel (ISPPRS3)
Entry point, PSW, and register values are in hexadecimal. Abend code and reason code are in hexadecimal
for system abends and in decimal for user abends. Meanings for the entries on the Additional Diagnostic
Information panel are:
Diagnostic information
Chapter 7. Diagnostic Tools and Information  931

## Page 952

Abend code
Abend completion code, identified on the panel as "user" or "system".
Reason code
Component reason code or return code associated with the abend.
ISPF Release Level
ISPF version/release/modification level.
Module Name
Name of abending program or *NOT SPECIFIED* if no name is available.
Entry Point Address
Entry point address of abending program.
PSW
Program-Status Word at time of error.
Register content
General Purpose register content at time of error.
If the Recovery Termination Manager (RTM) could not get storage for the System Diagnostic Work Area
(SDWA) or an error occurred within the error routine, all fields on this panel will contain 0's, with the
exception of the abend code and ISPF release level. Those fields will contain the correct data.
You can enter the HELP command from this panel as well to display the list of common abend codes.
Information associated with an abend is available from the ISPF log file.
Press the END function key to return to the primary option menu.
ISPF statistics entry in a PDS directory
A valid ISPF directory can consist of fifteen halfwords of user data with bytes 29-30 blank, or twenty half
words of user data with bit 3 of byte 3 set on. Shown here is the format of the information that ISPF
writes to the PDS directory to maintain statistics for a member. If you suspect the statistics data has been
corrupted, you can compare the existing entry against these formats to help in problem determination.
Byte
Description and Format
1
Version number, in hexadecimal format. Value is between X'01' and X'99'.
2
Modification level, in hexadecimal format. Value is between X'00' and X'99'.
3
Flags:
Bit 1
SCLM indicator. SCLM uses this to determine whether the member and any related SCLM
information are still in sync.
• ON means the member was last edited by SCLM, the Software Configuration and Library
Manager.
• OFF means the member was somehow processed outside SCLM.
Bit 2
Reserved.
Bit 3
Indicates ISPF extended statistics exist.
Bit 4-7
Reserved for future ISPF use.
Bit 8
Reserved.
Diagnostic information
932  z/OS: z/OS ISPF Messages and Codes

## Page 953

4
The seconds portion of the time last modified, in packed decimal format.
5-8
Creation date:
Byte 5
Century indicator. X'00' = 1900. X'01' = 2000.
Byte 6-8
Julian date, in packed decimal format
9-12
Date last modified:
Byte 9
Century indicator. X'00' = 1900. X'01' = 2000.
Byte 10-12
Julian date, in packed decimal format
13-14
Time last modified, in packed format:
Byte 13
Hours, in packed decimal format
Byte 14
Minutes, in packed decimal format
15-16
Current number of lines, in hexadecimal format
17-18
Initial number of lines, in hexadecimal format
19-20
Number of modified lines, in hexadecimal format
21-28
Userid, in character format
29-40
Varies according to whether bit 3 of byte 3 is set off or on:
Bit 3 of byte 3 set off
Bytes 29-30 set to blank.
Bit 3 of byte 3 set on
Bytes 29-40 set to:
29-32
Current number of lines.
33-36
Initial number of lines.
37-40
Number of modified lines.
Common problems using ISPF
This section contains some common error messages that may be encountered while using ISPF. Error
resolutions and explanations are also included.
Messages
• IKJ56500I COMMAND NOT FOUND
Common problems using ISPF
Chapter 7. Diagnostic Tools and Information  933

## Page 954

If a command processor exists only in LPA, there must be an entry in the ISPTCM for the command
processor. See z/OS ISPF Planning and Customizing for more details on customizing the ISPF TSO
command table.
• IKJ56861I FILE ddname NOT FREED, DATA SET IS OPEN
If the LIBRARY parameter is used with a table service, the user is not able to free the ddname
for the table library pointed to by the LIBRARY parameter. ISPF keeps this library open until a new
ddname is used in the LIBRARY parameter with another table service. ISPF functions in this manner for
performance reasons.
Issuing a table service with a LIBRARY parameter containing a ddname that does not exist causes
the previous library to be closed and therefore allows the user to free the previous ddname. Use of
CONTROL ERRORS RETURN may be used to guard against a severe error as a result of a ddname not
existing.
For example:
    ALLOC FILE(DD1) DATASET('USERID.YOUR.TABLES') SHR
    ISPEXEC TBOPEN MYLIB LIBRARY(DD1)
    .
    .                                   /*ISPF services against your table*/
    .
    ISPEXEC TBCLOSE MYLIB LIBRARY(DD1)
    ISPEXEC CONTROL ERRORS RETURN
    ISPEXEC TBOPEN JUNK LIBRARY(DDJUNK) /*nonexistent table in a  */
                                        /*nonexistent library     */
    ISPEXEC CONTROL ERRORS CANCEL
    FREE F(DD1)
• ISPP150 Panel 'name' error–At least one of the CLEAR names listed is not a panel field name.
or:
ISPP121 Panel 'name' error–Panel definition too large, greater than screen size.
when entering KEYLIST, when requesting field-level help in ISPF panels, or when displaying panels
created using DTL.
These messages are often caused by having a GML library in the ISPPLIB concatenation or by having
GML source code in the panel library. Check your ISPPLIB concatenation to make sure that the ISPF-
supplied GML library is not concatenated first. The ISPF-supplied GML library should not be in any of the
ISPF library concatenations. Make sure that the libraries in your ISPPLIB concatenation do not contain
GML source code.
• ISPT036 Table in use–'table service' issued for table 'table name' that is in use, ENQUEUE failed.
This message frequently occurs when batch jobs that use ISPF services run concurrently. This occurs
because most batch jobs allocate a new profile each time they run. ISPF issues a TBOPEN against
ISPPROF DD card for member ISPSPROF. The TBOPEN fails since ISPPROF does not contain this
member. ISPF then issues a TBOPEN against ISPTLIB to copy the default ISPSPROF from ISPTLIB to
ISPPROF.
If the first data set in the ISPTLIB concatenation sequence is the same for two batch jobs running
concurrently, message ISPT036 is issued. To ensure that this condition does not occur, the first data set
in the ISPTLIB concatenation should be user unique. For example, 'sysuid..ISPPROF' would be a user
unique data set, which could be used as the first data set concatenated to the ISPTLIB DD.
For the same reasons, this problem can also occur when two users log on to ISPF for the first time if
they have the same data set concatenated first in the ISPTLIB concatenation.
• ISPT016, ISPT017, and other I/O Errors
ISPF has various messages that reference I/O errors on either GET or PUT (READ and WRITE macros)
such as message ISPT017. These errors are typically caused by concatenation problems on one of the
ISPF libraries.
Common problems using ISPF
934  z/OS: z/OS ISPF Messages and Codes

## Page 955

Allocating data sets that do not have consistent DCB parameters in ISPF library concatenations often
causes these messages. Also, ISPTABL, ISPFILE, and ISPPROF are used for output and therefore must
have only a single data set allocated to their ddnames.
– For I/O errors during panel services, check your ISPPLIB concatenation for inconsistent DCBs.
– For I/O errors during file tailoring services, check your ISPSLIB concatenation for inconsistent DCBs
and make sure that only one data set is allocated to ddname ISPFILE.
– For I/O errors during table services, check your ISPTLIB concatenation for inconsistent DCBs and
make sure that only one data set is allocated to ddname ISPTABL.
I/O error messages cannot be issued when there is a problem with the ISPMLIB concatenation since
messages cannot be located due to the I/O error. Message CMG999 occurs when there is an I/O error
due to an ISPMLIB concatenation problem.
• CMG999
CMG999 is issued with an appropriate description of the error condition for any problem with accessing
a message. See z/OS ISPF Dialog Developer's Guide and Reference for further information on how to
define a message.
Unexpected output
• ISPF services do not pick up updated copies of messages or panels.
When not in TEST mode, the most recently accessed panel and message definitions are retained in
virtual storage for performance reasons. If you have modified a panel or message file, using TEST mode
ensures that the latest copy of each message or panel is accessed. See z/OS ISPF Services Guide for
more information on executing ISPF in TEST mode.
• ISPF commands such as WINDOW, COLOR, CUAATTR, EXIT, CANCEL, ACTIONS, KEYSHELP, KEYLIST,
EXHELP, FKA, and ISPDTLC are not recognized as valid commands, or function keys defined as these
commands do not function properly.
The user issuing these commands or pressing the function keys defined as these commands has a
private copy of ISPCMDS in the ISPTLIB concatenation. The user's private copy of ISPCMDS is missing
some or all of the new commands supplied in the new command table, ISPCMDS.
Users experiencing this problem should either replace their private copy of ISPCMDS with the ISPF-
supplied copy, or update their private ISPCMDS with the missing commands.
• ISPF commands such as INT, SWITCH, TSOGUI, WS, WSCON and WSDISCON are not recognized as
valid commands, or function keys defined as these commands do not function properly.
The user issuing these commands or pressing the function keys defined as these commands has a
private copy of ISPCMDS in the ISPTLIB concatenation. The user's private copy of ISPCMDS contains
these deleted commands.
Users experiencing this problem should either replace their private copy of ISPCMDS with the ISPF-
supplied copy, or update their private ISPCMDS to delete these commands.
Abend codes and information
ISPF controller and processor task abends are controlled by STAE and STAI exit routines and by ISPF
execution modes set using the ISPSTART TEST parameters.
Under normal conditions (that is, when processor and controller dumps have not been requested by
specifying the ISPSTART TEST command):
• When a processor task abends:
– No dump is taken.
– The controller reattaches the processor main drive (ISPPMD).
– The primary option menu is redisplayed for that logical screen.
Abend codes and information
Chapter 7. Diagnostic Tools and Information  935

## Page 956

• When the controller task abends:
– ISPF terminates with *** ISPF MAIN TASK ABEND *** message.
– Control returns to TSO.
– Pressing Enter causes a dump to be taken if a dump data set has been allocated.
The controller and processor tasks issue the ABEND system service and allow dumps under certain
situations. The ISPF modules that issue ABENDs and their associated codes and reasons are listed below:
Abend code 0C1 in various common ISPF subroutines
In several ISPF modules, an invalid operation code of (X'00') is executed to force an abend at the
point that an unexpected condition occurs. Contact IBM support if this condition occurs within an
ISPF module.
Abend code 0C4 in ISPDVCGT, ISPDVCPT, or ISPDVCFD
These abends are often caused by mismatched VDEFINE and VDELETE services in a user's program.
The VDEFINE service gives ISPF addressability to user storage. This storage is used by variable
services any time the variable that has been established by the VDEFINE service is referenced. If this
storage is released back to the system, an 0C4 abend may occur depending on whether the storage is
still accessible. Here are two common scenarios that often show these abends:
• A program establishes a variable in a called subroutine using the VDEFINE service and subsequently
uses an ISPF service that references this variable in another routine. If the called subroutine
was dynamically loaded and therefore released its storage, an 0C4 abend could occur when the
subroutine references a VDEFINEd variable.
• A program establishes a variable in a called subroutine using the VDEFINE service and then calls
another program without using the SELECT service. Then the called program VDEFINEs a variable
with the same name, but does not VDELETE it on exit. If the calling program references that variable
after the called program returns control to it, an 0C4 abend can occur. Since a VDELETE has not
been done, ISPF services still reference the variable VDEFINEd by the called program.
If the program intent is to use the same variable in the main and called routines, the variable should
be VDEFINEd only in the main routine. If the program intent to isolate a variable to be used only in the
routine in which it is VDEFINEd, then the program should also VDELETE the variable before it ends. To
diagnose whether the user application has this problem, a function trace on VDEFINE, VDELETE, and
the SELECT services (Option 7.7.1) is very helpful.
Abend codes 111 or 222
To produce these abends, the user must be in test mode and request processor dumps by entering
one of the following commands on the ISPF command line. With exception of the user completion
code, both commands function in the same manner.
ABEND
Terminates ISPF with user completion code 111.
CRASH
Terminates ISPF with user completion code 222.
Abend code 908
ZISPFRC value was not valid
Abend code 920
ISPSTART command syntax was not valid
Abend code 950
An ISPF session running on behalf of a z/OS client had to be abnormally terminated. One of the
following write-to-operator (WTO) messages will accompany the user abend:
ISPWB000
Client requested ISPF session initialization
Userid: aaaaaaa ASIDX: bbbb
Message Queue: cccccccccc CCSID: ddddd
Abend codes and information
936  z/OS: z/OS ISPF Messages and Codes

## Page 957

This message does not indicate a problem but is issued when a request is received to start
an ISPF session on behalf of a client. This informational message shows the user ID that the
address space will run under, the ID of the TSO address space, the ID of the z/OS UNIX message
queue used to exchange messages between TSO/ISPF and the client, and the CCSID used when
converting message between EBCDIC and Unicode.
ISPWB001
Request received from client to force termination. The ISPF session is abnormally terminated.
Userid: aaaaaaa ASIDX: bbbb
This operator message is issued when ISPF receives a request from a client to force the
termination of the ISPF session for a user. The message shows the TSO ID of the user and the ID
of the TSO address space.
ISPWB002
Call to BPX1QSN to send a message to the queue failed. Return code: 'aaaa'X Reason
code: 'bbbb'X
The ISPF session is abnormally terminated.
This operator message is issued when a call to z/OS UNIX service BPX1QSN to send panel JSON
to the client via a z/OS UNIX message queue fails. The message shows the return and reason code
from BPX1QSN.
ISPWB003
Call to BPX1QRC to read a message from the queue failed. Return code: 'aaaa'X Reason
code: 'bbbb'X
The ISPF session is abnormally terminated.
This operator message is issued when a call to z/OS UNIX service BPX1QRC to receive response
JSON from the client via a z/OS UNIX message queue fails. The message shows the return and
reason code from BPX1QRC.
ISPWB004
Call to BPX1QRC returned a message of length zero. Return code: 'aaaa'X Reason code:
'bbbb'X
The ISPF session is abnormally terminated.
This operator message is issued when a call to z/OS UNIX service BPX1QRC to receive response
JSON from the client returns a message with a length of zero. The message shows the return and
reason code from BPX1QRC.
Abend code 988
Invalid TSO environment. See z/OS ISPF Planning and Customizing for the proper TSO version.
Abend code 990
An error occurred running in batch mode. If ZISPFRC has not been set previously, and ISPF
encounters a severe error that terminates the product, then 990 is set.
Abend code 995
Configuration table is not compatible with current ISPF release. Configuration table must be release
4.8 or later.
Abend code 996 (or X'3E5')
ISPF was not able to load the terminal translate table during initialization. Check that the load module
defined in the configuration table is available in the ISPLLIB or MVS load library search concatenation.
The value is stored in the user's profile data set, so a reset may be required to load the correct value.
Abend code 997 (or X'3E5')
A TPUT returned a return code other than 0 or 8. A message is displayed and an attempt is made to
redisplay the full screen. If the redisplay fails twice, this abend is issued.
Abend codes and information
Chapter 7. Diagnostic Tools and Information  937

## Page 958

Abend code 998 (or X'3E6')
An ISPF severe error that occurs while not in CONTROL ERRORS RETURN mode and before ISPF is
fully initialized. ISPF is considered to be fully initialized when the Enter key on the primary option
menu has been processed without a severe error occurring.
Abend code 999 (or X'3E7')
This abend is issued for the following reasons:
• No function pool is established for a command processor.
For example, a command processor that uses ISPF services is invoked using option 6 or SELECT
CMD, but the command processor does not have a function pool. The user needs to have an entry
for the command processor in the ISPTCM with the X'40' flag set on. The X'40' flag indicates
that the command requires a function pool. See z/OS ISPF Planning and Customizing for more
information on customizing the ISPTCM.
• An error occurs while another error is already being processed.
ISPF issues the abend code 999 in this case to protect against an infinite loop.
• An error occurred during ISPF initialization.
For example:
– An I/O error occurred due to ISPF library allocations such as ISPSLIB, ISPPLIB, ISPMLIB, and so
forth, containing inconsistent or incorrect DCB attributes.
– An ISPF library allocation does not contain the required ISPF libraries in its concatenation. For
example, the ISPMLIB contains user product libraries but not ISPF libraries.
See the ISPF log for more information.
Terminal I/O error codes
Below is a list of terminal I/O error codes that you may see while using ISPF.
• ISPF screen output error code
41
TPUT return code not equal to 0 or 8
• ISPF screen input error code
21
TGET return code other than 0, 4, or 8.
22
Input stream size greater than input buffer size or 0.
23
Unknown attention identifier (AID).
24
Invalid input AID.
25
Input stream size invalid for input AID.
26
Input cursor location not within physical screen.
28
First byte of input buffer field not an SBA (invalid input data).
31
Byte preceding the physical screen field is past the end of the physical screen (input data from
invalid screen position).
32
Byte preceding the physical screen field is not an input attribute (input data from invalid screen
position).
Terminal I/O error codes
938  z/OS: z/OS ISPF Messages and Codes

## Page 959

33
Physical screen field not defined on panel (input data from invalid screen position).
51
Physical screen field attribute not found in logical screen.
52
Byte preceding logical screen field is not an input attribute.
55
Physical screen size is greater than corresponding logical screen size.
Note:
1. The physical screen size is determined by ISPF during initialization.
2. The input buffer size is a variable based on the physical screen size.
3. The logical screen is the same size as the physical screen, and is the size that the processor task uses
for screen I/O. When the 3290 is running in 62 X 160 partition mode, the SPLITV command makes the
logical screen width equal to 80. When a 3278 mod 5 is running in standard mode, the logical screen
size is 24 X 80.
4. Only part of the logical screen appears on the physical screen when ISPF is running in split-screen
mode. When the 3290 is running in 62 X 160 partition mode, the entire logical screen may be visible,
depending on the position of the horizontal split line.
5. An input buffer field extends from an SBA to either the next SBA or the end of the input buffer.
6. A physical screen field extends from the location indicated in the input buffer SBA to the location of the
next attribute byte in the physical screen.
Register linkage conventions
ISPF uses standard linkage conventions:
• SELECT PGM(program-name)
REGISTER
CONTENTS
1
Points to the address of the parameter data (from the PARM keyword) field (half-word length)
followed by the data
2 - 12
Not used
13
72-byte save area
14
Return address
15
Entry address / Return code on exit
• ISPF EXITS / Call to ISPLINK
REGISTER
CONTENTS
1
On entry, points to a parameter list; each address in the list in turn points to a parameter. On return
to the caller of ISPLINK, the user's parameter list starts at the second parameter. ISPF has inserted
a parameter in front of the user's parameters for ISPF use.
2 - 12
Not used
Register linkage conventions
Chapter 7. Diagnostic Tools and Information  939

## Page 960

13
72-byte save area
14
Return address
15
Entry address / Return code on exit
• SELECT CMD(cmdname) where cmdname is a program that is attached as a command processor by
ISPF:
REGISTER
CONTENTS
1
Points to a CPPL (Command Processor Parameter List) which is a list of four addresses that point
respectively to: Command buffer, UPT, PSCB, ECT. See the TSO programming services manual for
descriptions of these parameters.
2 - 12
Not used
13
72-byte save area
14
Not applicable
15
Return code on exit
Usually when an abend occurs within ISPF code, register 12 points to the entry point of the abending
CSECT.
Obtaining message IDs
In order to obtain the message ID associated with an error message in ISPF, you need to be in ISPF TEST
mode.
ISPF is in TEST mode if:
• ISPF is invoked with the TEST, TESTX, TRACE, or TRACEX parameter specified on the ISPSTART, PDF, or
ISPF command, or
• "Restore TEST/TRACE options" is not selected in option 0 and you go into option 7, Dialog Test, at some
point in your current ISPF session.
If you are not in TEST mode, split the screen, enter option 7, Dialog Test, and swap back to the screen
containing the error.
You can use the either of the following methods to get the message ID:
• Enter print on the panel displaying the error message. The message ID, along with the displayed
message text and screen output, appears in the LIST data set. The LIST data set can be printed using
the LIST command.
• With the short message displayed:
1. Press the function key assigned to Help (default is F1) or type help on the command line. This
displays the long message text for the error.
2. Press the function key assigned to Help or type help on the command line once more to display the
Tutorial panel associated with the error. The bottom lines of the Tutorial panel contain fields that list
the current panel name, the previous panel name, and the message ID. The value following LAST
MSG= is the message ID associated with the error.
Obtaining message IDs
940  z/OS: z/OS ISPF Messages and Codes
